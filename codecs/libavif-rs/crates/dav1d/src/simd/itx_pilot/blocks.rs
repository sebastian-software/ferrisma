// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `inv_txfm_add_c` and `inv_txfm_add_wht_wht_4x4_c` of
//! `upstream/dav1d/src/itx_tmpl.c` on vectors.
//!
//! The C transforms the rows of the coefficient block, rounds and clips
//! them into `tmp` (row-major), transforms the columns of `tmp` and adds
//! them to the pixels. Here a vector holds one value of several rows (first
//! pass) or columns (second pass):
//!
//! - first pass: coefficient `(x, y)` is at `coeff[y + x * sh]`, so the
//!   values of rows `y0..y0 + L` at column `x` are contiguous; the 1-D
//!   transform runs across the `w` vectors of a group of rows.
//! - between the passes each `L`x`L` block of the result is transposed and
//!   stored into `tmp` as the C lays it out;
//! - second pass: the values of columns `x0..x0 + L` at row `y` of `tmp`
//!   are contiguous; the 1-D transform runs across the `h` vectors.
//!
//! `L` is 8 (`i32x8`), or 4 (`i32x4`) where the block is 4 wide or 4 tall.

use fearless_simd::prelude::*;
use fearless_simd::{i16x8, i32x4, i32x8, u16x8, u8x16};

use super::transforms::{each, transform, wht4, Clip, Kind, Lanes};

/// A pixel of the DSP table: `u8` at 8 bpc, `u16` above.
pub trait Pixel: Copy + 'static {
    fn to_i32(self) -> i32;
    /// Stores `V::LEN` clipped values (`0..=bitdepth_max`) as pixels.
    fn store<S: Simd, V: Lanes<S>>(values: V, pixels: &mut [Self]);

    /// `iclip_pixel(dst + residual)` for `V::LEN` pixels, where `residual`
    /// is a second-pass output `(c + 8) >> 4`.
    fn add<S: Simd, V: Lanes<S>>(simd: S, pixels: &mut [Self], residual: V, pixel_max: i32);

    /// `iclip_pixel(dst + dc)` for a row, `dc` the DC-only value.
    fn add_dc<S: Simd>(simd: S, row: &mut [Self], dc: i32, pixel_max: i32);
}

// Both run in 16-bit lanes up to 10 bits, which is exact there: the second
// pass clips to the range of `int16_t` at 8 and 10 bits, and its largest
// unclipped output (the 32-point identity, four times a clipped value) is
// below 2^17, so `|residual|` is below 2^13; the DC-only value of a
// coefficient within `cf_max` (recon_tmpl.c) is below 2^12. At 12 bits the
// clip is 2^17 and the residual can reach 2^15: 32-bit lanes there.

impl Pixel for u8 {
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn store<S: Simd, V: Lanes<S>>(values: V, pixels: &mut [Self]) {
        values.store_u8(pixels);
    }

    #[inline(always)]
    fn add<S: Simd, V: Lanes<S>>(simd: S, pixels: &mut [Self], residual: V, _pixel_max: i32) {
        add_u8(simd, pixels, residual.to_i16x8());
    }

    #[inline(always)]
    fn add_dc<S: Simd>(simd: S, row: &mut [Self], dc: i32, _pixel_max: i32) {
        let dc = i16x8::splat(simd, dc as i16);
        match row.len() {
            4 => add_u8(simd, &mut row[..4], dc),
            8 => add_u8(simd, &mut row[..8], dc),
            _ => {
                for pixels in row.chunks_exact_mut(16) {
                    let (low, high) = u8x16::from_slice(simd, pixels).widen();
                    let clip = |wide: u16x8<S>| -> u16x8<S> { (wide.bitcast::<i16x8<S>>() + dc).max(0).min(255).bitcast() };
                    clip(low).narrow(clip(high)).store_slice(pixels);
                }
            }
        }
    }
}

/// `iclip_pixel(dst + residual)` for up to 8 pixels of 8 bits.
#[inline(always)]
fn add_u8<S: Simd>(simd: S, pixels: &mut [u8], residual: i16x8<S>) {
    let mut bytes = [0; 16];
    bytes[..pixels.len()].copy_from_slice(pixels);
    let (wide, _) = u8x16::from_slice(simd, &bytes).widen();
    let sum: u16x8<S> = (wide.bitcast::<i16x8<S>>() + residual).max(0).min(255).bitcast();
    pixels.copy_from_slice(&sum.narrow(sum).as_slice()[..pixels.len()]);
}

impl Pixel for u16 {
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn store<S: Simd, V: Lanes<S>>(values: V, pixels: &mut [Self]) {
        values.store_u16(pixels);
    }

    #[inline(always)]
    fn add<S: Simd, V: Lanes<S>>(simd: S, pixels: &mut [Self], residual: V, pixel_max: i32) {
        if pixel_max <= 1023 {
            add_u16(simd, pixels, residual.to_i16x8(), pixel_max);
        } else {
            add_to_pixels(simd, pixels, residual, pixel_max);
        }
    }

    #[inline(always)]
    fn add_dc<S: Simd>(simd: S, row: &mut [Self], dc: i32, pixel_max: i32) {
        if pixel_max > 1023 {
            return add_dc_wide(simd, row, dc, pixel_max);
        }
        let dc = i16x8::splat(simd, dc as i16);
        if row.len() == 4 {
            add_u16(simd, &mut row[..4], dc, pixel_max);
        } else {
            for pixels in row.chunks_exact_mut(8) {
                add_u16(simd, pixels, dc, pixel_max);
            }
        }
    }
}

/// `iclip_pixel(dst + dc)` for a row, in 32-bit lanes.
#[inline(always)]
fn add_dc_wide<S: Simd, P: Pixel>(simd: S, row: &mut [P], dc: i32, pixel_max: i32) {
    if row.len() == 4 {
        add_to_pixels(simd, row, i32x4::splat(simd, dc), pixel_max);
    } else {
        for pixels in row.chunks_exact_mut(8) {
            add_to_pixels(simd, pixels, i32x8::splat(simd, dc), pixel_max);
        }
    }
}

/// `iclip_pixel(dst + residual)` for up to 8 pixels of 10 bits.
#[inline(always)]
fn add_u16<S: Simd>(simd: S, pixels: &mut [u16], residual: i16x8<S>, pixel_max: i32) {
    let mut values = [0; 8];
    values[..pixels.len()].copy_from_slice(pixels);
    let sum = (u16x8::from_slice(simd, &values).bitcast::<i16x8<S>>() + residual).max(0).min(pixel_max as i16);
    pixels.copy_from_slice(&sum.bitcast::<u16x8<S>>().as_slice()[..pixels.len()]);
}

/// A coefficient of the DSP table (`coef`): `i16` at 8 bpc, `i32` above.
pub trait Coef: Copy + 'static {
    const ZERO: Self;
    fn to_i32(self) -> i32;
}

impl Coef for i16 {
    const ZERO: Self = 0;
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
}

impl Coef for i32 {
    const ZERO: Self = 0;
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self
    }
}

/// What the bit depth decides in `inv_txfm_add_c`.
#[derive(Clone, Copy)]
pub struct Depth {
    /// `row_clip_min`, `row_clip_max`: the clipping of the first pass.
    pub row_clip: (i32, i32),
    /// `col_clip_min`, `col_clip_max`: of the intermediate and the second pass.
    pub col_clip: (i32, i32),
    /// `bitdepth_max`, the upper bound of `iclip_pixel`.
    pub pixel_max: i32,
}

impl Depth {
    /// `BITDEPTH == 8`: both clips are the range of `int16_t`.
    pub fn eight() -> Self {
        let int16 = (i16::MIN.into(), i16::MAX.into());
        Depth { row_clip: int16, col_clip: int16, pixel_max: 255 }
    }

    /// `BITDEPTH == 16` with `bitdepth_max`.
    pub fn high(bitdepth_max: i32) -> Self {
        let row_min = ((!bitdepth_max as u32) << 7) as i32;
        let col_min = ((!bitdepth_max as u32) << 5) as i32;
        Depth { row_clip: (row_min, !row_min), col_clip: (col_min, !col_min), pixel_max: bitdepth_max }
    }
}

/// `enum TxfmType` values the driver tests for (src/levels.h).
pub const DCT_DCT: usize = 0;

/// `dav1d_tx1d_types` (itx_1d.c): the first (row) and second (column)
/// 1-D transform of each `enum TxfmType` but WHT.
pub const TX1D_TYPES: [[Kind; 2]; 16] = {
    use Kind::*;
    [
        [Dct, Dct],
        [Adst, Dct],
        [Dct, Adst],
        [Adst, Adst],
        [FlipAdst, Dct],
        [Dct, FlipAdst],
        [FlipAdst, FlipAdst],
        [Adst, FlipAdst],
        [FlipAdst, Adst],
        [Identity, Identity],
        [Dct, Identity],
        [Identity, Dct],
        [Adst, Identity],
        [Identity, Adst],
        [FlipAdst, Identity],
        [Identity, FlipAdst],
    ]
};

/// The `enum TxfmType` that the C kernel in DSP-table slot `slot` passes to
/// `inv_txfm_add_c`: the C assigns to slot `DCT_ADST` the kernel of
/// `ADST_DCT` and vice versa, and so for every pair of types that are
/// transposes of each other (`assign_itx_all_fn16`, `assign_itx_all_fn84`).
pub const fn kernel_type(slot: usize) -> usize {
    // ADST_DCT 1, DCT_ADST 2, FLIPADST_DCT 4, DCT_FLIPADST 5, ADST_FLIPADST 7,
    // FLIPADST_ADST 8, V_DCT 10, H_DCT 11, V_ADST 12, H_ADST 13,
    // V_FLIPADST 14, H_FLIPADST 15.
    match slot {
        1 => 2,
        2 => 1,
        4 => 5,
        5 => 4,
        7 => 8,
        8 => 7,
        10 => 11,
        11 => 10,
        12 => 13,
        13 => 12,
        14 => 15,
        15 => 14,
        _ => slot,
    }
}

/// The `shift` of each block size (the `inv_txfm_fn*` lines of itx_tmpl.c).
pub const fn shift(w: usize, h: usize) -> u32 {
    match (w, h) {
        (4, 4) | (4, 8) | (8, 4) => 0,
        (4, 16) | (8, 8) | (8, 16) | (16, 4) | (16, 8) | (16, 32) | (32, 16) | (32, 64) | (64, 32) => 1,
        _ => 2,
    }
}

/// For each `eob` of a block size, the last row and the last column that a
/// coefficient at scan position `0..=eob` can be in.
pub struct ScanExtent {
    /// `dav1d_last_nonzero_col_from_eob` (scan.c; the C's "col" is the row
    /// of its first pass).
    pub rows: Box<[u8]>,
    /// The same for the columns (the first pass's inputs).
    pub columns: Box<[u8]>,
}

/// The block's rows, its coefficients and how much of them is coded.
pub struct Block<'a, P, C, const W: usize, const H: usize> {
    /// Rows `0..H` of the destination, `W` pixels each.
    pub rows: [&'a mut [P]; H],
    /// `min(W, 32) * min(H, 32)` coefficients, column after column.
    pub coeff: &'a mut [C],
    pub eob: usize,
    /// The scan's extent for the block size.
    pub extent: &'static ScanExtent,
}

/// The rows and columns of the coefficient block that can hold a nonzero
/// coefficient: the rows are `last_nonzero_col + 1` of `inv_txfm_add_c`;
/// the columns come from the scan as well where the C takes the rows from
/// it, and are all `sw` otherwise.
pub fn coded_extent(tx_type: usize, w: usize, sh: usize, eob: usize, extent: &ScanExtent) -> (usize, usize) {
    use Kind::Identity;
    let [first, second] = TX1D_TYPES[tx_type];
    let sw = w.min(32);
    if second == Identity && first != Identity {
        ((sh - 1).min(eob) + 1, sw)
    } else if first == Identity && second != Identity {
        ((eob >> w.trailing_zeros()) + 1, sw)
    } else {
        (usize::from(extent.rows[eob]) + 1, usize::from(extent.columns[eob]) + 1)
    }
}

/// `inv_txfm_add_c` for a `W`x`H` block of `tx_type` (`enum TxfmType`, not
/// WHT); `SW`, `SH` are `min(W, 32)`, `min(H, 32)`.
pub fn inverse_transform_add<
    S: Simd,
    P: Pixel,
    C: Coef,
    const W: usize,
    const H: usize,
    const SW: usize,
    const SH: usize,
>(
    simd: S,
    block: Block<'_, P, C, W, H>,
    tx_type: usize,
    depth: Depth,
) {
    let Block { mut rows, coeff, eob, extent } = block;
    let pass = Pass { kinds: TX1D_TYPES[tx_type], rect2: W == 2 * H || H == 2 * W, shift: shift(W, H), depth };
    // The whole block in one region compiled with the level's features.
    simd.vectorize(
        #[inline(always)]
        || {
            if tx_type == DCT_DCT && eob == 0 {
                return dc_only(simd, &mut rows, coeff, pass);
            }
            // Where only the first 8 or 16 rows and columns of coefficients
            // are coded, the passes get that bound as a constant, and the
            // compiler drops the zero inputs from the transforms (as the C's
            // tx64 variants and the assembly's eob paths do by hand). Not for
            // the 64-point sizes: rare, and each bound is another copy of the
            // passes.
            let (coded_rows, coded_columns) = coded_extent(tx_type, W, SH, eob, extent);
            let within = |bound: usize| {
                W <= 32 && H <= 32 && coded_rows <= bound && coded_columns <= bound && (SW > bound || SH > bound)
            };
            if within(8) {
                passes::<_, _, _, W, H, 8, 8>(simd, &mut rows, coeff, coded_rows, pass);
            } else if within(16) {
                passes::<_, _, _, W, H, 16, 16>(simd, &mut rows, coeff, coded_rows, pass);
            } else {
                passes::<_, _, _, W, H, SW, SH>(simd, &mut rows, coeff, coded_rows, pass);
            }
        },
    )
}

/// What the two passes of a block share.
#[derive(Clone, Copy)]
struct Pass {
    /// The first (row) and the second (column) 1-D transform.
    kinds: [Kind; 2],
    /// `is_rect2`: the coefficients are scaled by 181 / 256 first.
    rect2: bool,
    shift: u32,
    depth: Depth,
}

/// Both passes, for a block whose coefficients outside the first `NX`
/// columns and `NY` rows are zero.
#[inline(always)]
fn passes<S: Simd, P: Pixel, C: Coef, const W: usize, const H: usize, const NX: usize, const NY: usize>(
    simd: S,
    rows: &mut [&mut [P]; H],
    coeff: &mut [C],
    coded_rows: usize,
    pass: Pass,
) {
    let sh = H.min(32);
    let mut tmp = [[0; W]; NY];
    if W.min(sh) == 4 {
        first_pass::<_, i32x4<S>, _, W, NX>(simd, coeff, sh, coded_rows, pass, &mut tmp);
    } else {
        first_pass::<_, i32x8<S>, _, W, NX>(simd, coeff, sh, coded_rows, pass, &mut tmp);
    }
    if W == 4 {
        second_pass::<_, i32x4<S>, _, W, H>(simd, &tmp[..NY.min(sh)], pass, rows);
    } else {
        second_pass::<_, i32x8<S>, _, W, H>(simd, &tmp[..NY.min(sh)], pass, rows);
    }
}

/// The rows of the coefficient block: transformed, rounded, clipped and
/// stored into `tmp` (row `y` at `tmp[y]`), the coefficients read set to 0.
/// Rows from `coded_rows` up and columns from `NX` up hold no nonzero
/// coefficient; their part of `tmp` stays 0, as in the C.
#[inline(always)]
fn first_pass<S: Simd, V: Lanes<S>, C: Coef, const W: usize, const NX: usize>(
    simd: S,
    coeff: &mut [C],
    sh: usize,
    coded_rows: usize,
    pass: Pass,
    tmp: &mut [[i32; W]],
) {
    let lanes = V::LEN;
    let nx = NX.min(W.min(32));
    let (row_min, row_max) = pass.depth.row_clip;
    let row_clip = Clip { min: V::splat(simd, row_min), max: V::splat(simd, row_max) };
    let (col_min, col_max) = pass.depth.col_clip;
    let (shift, rnd) = (pass.shift, (1 << pass.shift) >> 1);
    for y0 in (0..coded_rows).step_by(lanes) {
        // Input x of rows y0..y0 + L; from `nx` up zero.
        let mut input = [V::splat(simd, 0); W];
        for (x, c) in input[..nx].iter_mut().enumerate() {
            let column = &mut coeff[x * sh + y0..][..lanes];
            *c = load(simd, column, C::to_i32);
            column.fill(C::ZERO);
            if pass.rect2 {
                *c = (*c * 181 + 128) >> 8;
            }
        }
        let out = transform(pass.kinds[0], input, row_clip);
        let mut out = each(out, |v| ((v + rnd) >> shift).max(col_min).min(col_max));
        for (x0, square) in out.chunks_exact_mut(lanes).enumerate() {
            V::transpose(square);
            for (y, row) in square.iter().enumerate() {
                row.store_slice(&mut tmp[y0 + y][x0 * lanes..][..lanes]);
            }
        }
    }
}

/// The columns of `tmp` (rows from `tmp.len()` up are zero) transformed and
/// added to the pixels: `iclip_pixel(dst + ((c + 8) >> 4))`.
#[inline(always)]
fn second_pass<S: Simd, V: Lanes<S>, P: Pixel, const W: usize, const H: usize>(
    simd: S,
    tmp: &[[i32; W]],
    pass: Pass,
    rows: &mut [&mut [P]; H],
) {
    let lanes = V::LEN;
    let (col_min, col_max) = pass.depth.col_clip;
    let col_clip = Clip { min: V::splat(simd, col_min), max: V::splat(simd, col_max) };
    for x0 in (0..W).step_by(lanes) {
        let mut input = [V::splat(simd, 0); H];
        for (c, row) in input.iter_mut().zip(tmp) {
            *c = V::from_slice(simd, &row[x0..][..lanes]);
        }
        let out = transform(pass.kinds[1], input, col_clip);
        for (row, c) in rows.iter_mut().zip(out) {
            P::add(simd, &mut row[x0..][..lanes], (c + 8) >> 4, pass.depth.pixel_max);
        }
    }
}

/// `V::LEN` values converted to `i32` lanes.
#[inline(always)]
fn load<S: Simd, V: Lanes<S>, T: Copy>(simd: S, values: &[T], to_i32: impl Fn(T) -> i32) -> V {
    let mut v = V::splat(simd, 0);
    for (lane, &value) in v.as_mut_slice().iter_mut().zip(values) {
        *lane = to_i32(value);
    }
    v
}

/// `dst = iclip_pixel(dst + residual)` for `V::LEN` pixels.
#[inline(always)]
fn add_to_pixels<S: Simd, V: Lanes<S>, P: Pixel>(simd: S, pixels: &mut [P], residual: V, pixel_max: i32) {
    let sum = (load::<S, V, P>(simd, pixels, P::to_i32) + residual).max(0).min(pixel_max);
    P::store(sum, pixels);
}

/// The C's path for a DCT_DCT block with only the DC coefficient
/// (`eob < has_dconly`): one value added to every pixel, unclipped.
#[inline(always)]
fn dc_only<S: Simd, P: Pixel, C: Coef, const H: usize>(simd: S, rows: &mut [&mut [P]; H], coeff: &mut [C], pass: Pass) {
    let shift = pass.shift;
    let rnd = (1 << shift) >> 1;
    let mut dc = coeff[0].to_i32();
    coeff[0] = C::ZERO;
    if pass.rect2 {
        dc = (dc * 181 + 128) >> 8;
    }
    dc = (dc * 181 + 128) >> 8;
    dc = (dc + rnd) >> shift;
    dc = (dc * 181 + 128 + 2048) >> 12;
    for row in rows.iter_mut() {
        P::add_dc(simd, row, dc, pass.depth.pixel_max);
    }
}

/// `inv_txfm_add_wht_wht_4x4_c`: the lossless 4x4 Walsh-Hadamard transform,
/// no rounding and no clipping before the pixels.
pub fn wht_add<S: Simd, P: Pixel, C: Coef>(simd: S, rows: &mut [&mut [P]; 4], coeff: &mut [C], pixel_max: i32) {
    simd.vectorize(
        #[inline(always)]
        || {
            // Input x of rows 0..4 is coeff[x * 4..][..4], shifted by 2.
            let mut input = [i32x4::splat(simd, 0); 4];
            for (x, c) in input.iter_mut().enumerate() {
                *c = load::<_, i32x4<S>, _>(simd, &coeff[x * 4..][..4], C::to_i32) >> 2;
            }
            coeff[..16].fill(C::ZERO);
            let mut tmp = wht4(input);
            i32x4::transpose(&mut tmp);
            for (row, c) in rows.iter_mut().zip(wht4(tmp)) {
                add_to_pixels(simd, row, c, pixel_max);
            }
        },
    )
}
