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
use fearless_simd::{i32x4, i32x8};

use super::transforms::{each, transform, wht4, Clip, Kind, Lanes};

/// A pixel of the DSP table: `u8` at 8 bpc, `u16` above.
pub trait Pixel: Copy + 'static {
    fn to_i32(self) -> i32;
    /// Stores `V::LEN` clipped values (`0..=bitdepth_max`) as pixels.
    fn store<S: Simd, V: Lanes<S>>(values: V, pixels: &mut [Self]);
}

impl Pixel for u8 {
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn store<S: Simd, V: Lanes<S>>(values: V, pixels: &mut [Self]) {
        values.store_u8(pixels);
    }
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

/// The block's rows, its coefficients and how much of them is coded.
pub struct Block<'a, P, C, const W: usize, const H: usize> {
    /// Rows `0..H` of the destination, `W` pixels each.
    pub rows: [&'a mut [P]; H],
    /// `min(W, 32) * min(H, 32)` coefficients, column after column.
    pub coeff: &'a mut [C],
    pub eob: usize,
    /// `dav1d_last_nonzero_col_from_eob` of the block size.
    pub last_nonzero_rows: &'static [u8],
}

/// `last_nonzero_col + 1` of `inv_txfm_add_c`: the rows of the first pass
/// that can hold a nonzero coefficient.
pub fn coded_rows(tx_type: usize, w: usize, sh: usize, eob: usize, last_nonzero_rows: &[u8]) -> usize {
    use Kind::Identity;
    let [first, second] = TX1D_TYPES[tx_type];
    let last = if second == Identity && first != Identity {
        (sh - 1).min(eob)
    } else if first == Identity && second != Identity {
        eob >> w.trailing_zeros()
    } else {
        last_nonzero_rows[eob].into()
    };
    last + 1
}

/// `inv_txfm_add_c` for a `W`x`H` block of `tx_type` (`enum TxfmType`, not
/// WHT); `SH` is `min(H, 32)`.
pub fn inverse_transform_add<S: Simd, P: Pixel, C: Coef, const W: usize, const H: usize, const SH: usize>(
    simd: S,
    block: Block<'_, P, C, W, H>,
    tx_type: usize,
    depth: Depth,
) {
    let [first, second] = TX1D_TYPES[tx_type];
    let rect2 = W == 2 * H || H == 2 * W;
    let shift = shift(W, H);
    let Block { mut rows, coeff, eob, last_nonzero_rows } = block;
    // The whole block in one region compiled with the level's features.
    simd.vectorize(
        #[inline(always)]
        || {
            if tx_type == DCT_DCT && eob == 0 {
                return dc_only(simd, &mut rows, coeff, rect2, shift, depth);
            }
            let coded_rows = coded_rows(tx_type, W, SH, eob, last_nonzero_rows);
            let mut tmp = [[0; W]; SH];
            if W.min(SH) == 4 {
                first_pass::<_, i32x4<S>, _, W>(simd, coeff, SH, coded_rows, first, rect2, shift, depth, &mut tmp);
            } else {
                first_pass::<_, i32x8<S>, _, W>(simd, coeff, SH, coded_rows, first, rect2, shift, depth, &mut tmp);
            }
            if W == 4 {
                second_pass::<_, i32x4<S>, _, W, H>(simd, &tmp, second, depth, &mut rows);
            } else {
                second_pass::<_, i32x8<S>, _, W, H>(simd, &tmp, second, depth, &mut rows);
            }
        },
    )
}

/// The rows of the coefficient block: transformed, rounded, clipped and
/// stored into `tmp` (row `y` at `tmp[y]`), the coefficients read set to 0.
/// Rows from `coded_rows` up hold no nonzero coefficient; their part of
/// `tmp` stays 0, as in the C.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn first_pass<S: Simd, V: Lanes<S>, C: Coef, const W: usize>(
    simd: S,
    coeff: &mut [C],
    sh: usize,
    coded_rows: usize,
    kind: Kind,
    rect2: bool,
    shift: u32,
    depth: Depth,
    tmp: &mut [[i32; W]],
) {
    let lanes = V::LEN;
    let sw = W.min(32);
    let row_clip = Clip { min: V::splat(simd, depth.row_clip.0), max: V::splat(simd, depth.row_clip.1) };
    let (col_min, col_max) = depth.col_clip;
    let rnd = (1 << shift) >> 1;
    for y0 in (0..coded_rows).step_by(lanes) {
        // Input x of rows y0..y0 + L; columns from 32 up are zero.
        let mut input = [V::splat(simd, 0); W];
        for (x, c) in input[..sw].iter_mut().enumerate() {
            let column = &mut coeff[x * sh + y0..][..lanes];
            *c = load(simd, column, C::to_i32);
            column.fill(C::ZERO);
            if rect2 {
                *c = (*c * 181 + 128) >> 8;
            }
        }
        let out = transform(kind, input, row_clip);
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
    kind: Kind,
    depth: Depth,
    rows: &mut [&mut [P]; H],
) {
    let lanes = V::LEN;
    let col_clip = Clip { min: V::splat(simd, depth.col_clip.0), max: V::splat(simd, depth.col_clip.1) };
    for x0 in (0..W).step_by(lanes) {
        let mut input = [V::splat(simd, 0); H];
        for (c, row) in input.iter_mut().zip(tmp) {
            *c = V::from_slice(simd, &row[x0..][..lanes]);
        }
        let out = transform(kind, input, col_clip);
        for (row, c) in rows.iter_mut().zip(out) {
            add_to_pixels(simd, &mut row[x0..][..lanes], (c + 8) >> 4, depth.pixel_max);
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
fn dc_only<S: Simd, P: Pixel, C: Coef, const H: usize>(
    simd: S,
    rows: &mut [&mut [P]; H],
    coeff: &mut [C],
    rect2: bool,
    shift: u32,
    depth: Depth,
) {
    let rnd = (1 << shift) >> 1;
    let mut dc = coeff[0].to_i32();
    coeff[0] = C::ZERO;
    if rect2 {
        dc = (dc * 181 + 128) >> 8;
    }
    dc = (dc * 181 + 128) >> 8;
    dc = (dc + rnd) >> shift;
    dc = (dc * 181 + 128 + 2048) >> 12;
    if rows[0].len() == 4 {
        add_dc::<_, i32x4<S>, _>(simd, rows, dc, depth.pixel_max);
    } else {
        add_dc::<_, i32x8<S>, _>(simd, rows, dc, depth.pixel_max);
    }
}

#[inline(always)]
fn add_dc<S: Simd, V: Lanes<S>, P: Pixel>(simd: S, rows: &mut [&mut [P]], dc: i32, pixel_max: i32) {
    let dc = V::splat(simd, dc);
    for row in rows.iter_mut() {
        for pixels in row.chunks_exact_mut(V::LEN) {
            add_to_pixels(simd, pixels, dc, pixel_max);
        }
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
