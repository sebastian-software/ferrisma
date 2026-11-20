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
//! # The lanes
//!
//! A pass's vectors are as wide as the level's natural vector, as far as the
//! pass has independent rows or columns to fill them (`Coef`):
//!
//! - at 8 bpc the stored values are `i16` lanes of the level's own width,
//!   `Simd::i16s`: 8 on NEON, 16 on AVX2 (`Level`). A pass with 16 or more
//!   rows (first pass: the rows that can hold a nonzero coefficient) or
//!   columns (second pass) takes 16 lanes on AVX2, one register; a pass
//!   with 8 takes `i16x8`. The wide values of those lanes are `i32x16`, two
//!   AVX2 registers, as `i32x8` is two NEON registers.
//! - above 8 bpc the stored values need 32 bits: `i32x8` (one AVX2
//!   register, two NEON registers).
//! - a pass over 4 rows or columns, and every pass of a block 4 wide (its
//!   transpose is 4x4), takes `i32x4`: `fearless_simd` has no 4-lane `i16`
//!   vector.
//!
//! The transposes work on squares of `L`x`L` values. With 16 lanes, the
//! 16x16 transpose is four 8x8 ones (`transforms::transpose_i16x16`); the
//! first pass of an 8-wide block with 16 rows fills the left half of the
//! square, the right half is zero and drops out, and the low 8 lanes of each
//! row are stored. So NEON runs the code of the 8-lane layout unchanged,
//! and AVX2 the same code with 16 lanes, from one source.
//!
//! Between the passes `tmp` holds the coefficient type: `int16_t` at 8 bpc,
//! where the C clips the intermediate to its range.
//!
//! On x86-64 the 4x4, 4x8 and 8x4 blocks run both passes in one region
//! (`passes`).

use fearless_simd::prelude::*;
use fearless_simd::{i16x16, i16x8, i32x4, i32x8, u16x16, u16x8, u8x16};

use super::transforms::{transform, tx1d, wht4, Int32, Kind, Lanes, Tx1d};

/// A SIMD level whose natural `i16` vector (`Simd::i16s`) holds the stored
/// values of the 8 bpc passes: `i16x8` on NEON, `i16x16` on AVX2.
pub trait Level: Simd<i16s: Memory<Self, i16>> {}

impl<S: Simd<i16s: Memory<S, i16>>> Level for S {}

/// A pixel of the DSP table: `u8` at 8 bpc, `u16` above.
pub trait Pixel: Copy + 'static {
    fn to_i32(self) -> i32;
    /// Stores `V::LEN` clipped values (`0..=bitdepth_max`) as pixels.
    fn store<S: Simd, V: Int32<S>>(values: V, pixels: &mut [Self]);

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
// The DC-only path takes 16 pixels at a time in rows of 16 and more.

impl Pixel for u8 {
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn store<S: Simd, V: Int32<S>>(values: V, pixels: &mut [Self]) {
        values.store_u8(pixels);
    }

    #[inline(always)]
    fn add<S: Simd, V: Lanes<S>>(simd: S, pixels: &mut [Self], residual: V, _pixel_max: i32) {
        residual.residual().add_u8(simd, pixels);
    }

    #[inline(always)]
    fn add_dc<S: Simd>(simd: S, row: &mut [Self], dc: i32, _pixel_max: i32) {
        match row.len() {
            // Constant lengths: the copies of `add_u8` are then plain moves.
            4 => i16x8::splat(simd, dc as i16).add_u8(simd, &mut row[..4]),
            8 => i16x8::splat(simd, dc as i16).add_u8(simd, &mut row[..8]),
            _ => {
                for pixels in row.chunks_exact_mut(16) {
                    i16x16::splat(simd, dc as i16).add_u8(simd, pixels);
                }
            }
        }
    }
}

impl Pixel for u16 {
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
    #[inline(always)]
    fn store<S: Simd, V: Int32<S>>(values: V, pixels: &mut [Self]) {
        values.store_u16(pixels);
    }

    #[inline(always)]
    fn add<S: Simd, V: Lanes<S>>(simd: S, pixels: &mut [Self], residual: V, pixel_max: i32) {
        if pixel_max <= 1023 {
            residual.residual().add_u16(simd, pixels, pixel_max);
        } else {
            add_to_pixels(simd, pixels, residual.wide(), pixel_max);
        }
    }

    #[inline(always)]
    fn add_dc<S: Simd>(simd: S, row: &mut [Self], dc: i32, pixel_max: i32) {
        if pixel_max > 1023 {
            return add_dc_wide(simd, row, dc, pixel_max);
        }
        match row.len() {
            4 => i16x8::splat(simd, dc as i16).add_u16(simd, &mut row[..4], pixel_max),
            8 => i16x8::splat(simd, dc as i16).add_u16(simd, &mut row[..8], pixel_max),
            _ => {
                for pixels in row.chunks_exact_mut(16) {
                    i16x16::splat(simd, dc as i16).add_u16(simd, pixels, pixel_max);
                }
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

/// A residual in `i16` lanes, added to the pixels where 16 bits are exact
/// (8 and 10 bits, see above).
pub trait Residual<S: Simd>: SimdInt<S, Element = i16> {
    /// `iclip_pixel(dst + self)` for the `pixels.len()` (at most `LEN`)
    /// pixels of 8 bits.
    fn add_u8(self, simd: S, pixels: &mut [u8]);

    /// The same for pixels of 10 bits.
    fn add_u16(self, simd: S, pixels: &mut [u16], pixel_max: i32);
}

impl<S: Simd> Residual<S> for i16x8<S> {
    #[inline(always)]
    fn add_u8(self, simd: S, pixels: &mut [u8]) {
        let mut bytes = [0; 16];
        bytes[..pixels.len()].copy_from_slice(pixels);
        let (wide, _) = u8x16::from_slice(simd, &bytes).widen();
        let sum: u16x8<S> = (wide.bitcast::<i16x8<S>>() + self).max(0).min(255).bitcast();
        pixels.copy_from_slice(&sum.narrow(sum).as_slice()[..pixels.len()]);
    }

    #[inline(always)]
    fn add_u16(self, simd: S, pixels: &mut [u16], pixel_max: i32) {
        let mut values = [0; 8];
        values[..pixels.len()].copy_from_slice(pixels);
        let sum = (u16x8::from_slice(simd, &values).bitcast::<i16x8<S>>() + self).max(0).min(pixel_max as i16);
        pixels.copy_from_slice(&sum.bitcast::<u16x8<S>>().as_slice()[..pixels.len()]);
    }
}

/// 16 pixels at a time.
impl<S: Simd> Residual<S> for i16x16<S> {
    #[inline(always)]
    fn add_u8(self, simd: S, pixels: &mut [u8]) {
        let (low, high) = u8x16::from_slice(simd, pixels).widen();
        let sum: u16x16<S> = (low.combine(high).bitcast::<i16x16<S>>() + self).max(0).min(255).bitcast();
        let (low, high) = sum.split();
        low.relaxed_narrow(high).store_slice(pixels);
    }

    #[inline(always)]
    fn add_u16(self, simd: S, pixels: &mut [u16], pixel_max: i32) {
        let sum = (u16x16::from_slice(simd, pixels).bitcast::<i16x16<S>>() + self).max(0).min(pixel_max as i16);
        sum.bitcast::<u16x16<S>>().store_slice(pixels);
    }
}

/// A coefficient of the DSP table (`coef`): `i16` at 8 bpc, `i32` above.
/// Also the element of `tmp` between the passes.
pub trait Coef: Copy + 'static {
    const ZERO: Self;
    fn to_i32(self) -> i32;

    /// The lanes of the passes with 16 rows or columns and more: the level's
    /// natural width (module documentation).
    type Native<S: Level>: Memory<S, Self>;
    /// The lanes of the passes with 8.
    type Lanes8<S: Simd>: Memory<S, Self>;
    /// The lanes of the passes with 4 (32 bits at every depth).
    type Lanes4<S: Simd>: Memory<S, Self>;

    /// `depth`, as a constant where the coefficient type fixes it (8 bpc),
    /// so that the shared passes compile with the constant ranges.
    fn depth(depth: Depth) -> Depth;
}

impl Coef for i16 {
    const ZERO: Self = 0;
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self.into()
    }
    type Native<S: Level> = S::i16s;
    type Lanes8<S: Simd> = i16x8<S>;
    type Lanes4<S: Simd> = i32x4<S>;
    #[inline(always)]
    fn depth(_: Depth) -> Depth {
        Depth::eight()
    }
}

impl Coef for i32 {
    const ZERO: Self = 0;
    #[inline(always)]
    fn to_i32(self) -> i32 {
        self
    }
    type Native<S: Level> = i32x8<S>;
    type Lanes8<S: Simd> = i32x8<S>;
    type Lanes4<S: Simd> = i32x4<S>;
    #[inline(always)]
    fn depth(depth: Depth) -> Depth {
        depth
    }
}

/// Lanes loaded from and stored to coefficients or `tmp` (elements `C`).
pub trait Memory<S: Simd, C>: Lanes<S> {
    fn load_from(simd: S, values: &[C]) -> Self;
    /// Stores the first `values.len()` lanes: all, or (16 lanes, an 8-wide
    /// block's transpose) the low 8.
    fn store_to(self, values: &mut [C]);
}

impl<S: Simd> Memory<S, i16> for i16x8<S> {
    #[inline(always)]
    fn load_from(simd: S, values: &[i16]) -> Self {
        Self::from_slice(simd, values)
    }
    #[inline(always)]
    fn store_to(self, values: &mut [i16]) {
        self.store_slice(values);
    }
}

impl<S: Simd> Memory<S, i16> for i16x16<S> {
    #[inline(always)]
    fn load_from(simd: S, values: &[i16]) -> Self {
        Self::from_slice(simd, values)
    }
    #[inline(always)]
    fn store_to(self, values: &mut [i16]) {
        if values.len() == 8 {
            self.split().0.store_slice(values);
        } else {
            self.store_slice(values);
        }
    }
}

impl<S: Simd> Memory<S, i32> for i32x8<S> {
    #[inline(always)]
    fn load_from(simd: S, values: &[i32]) -> Self {
        Self::from_slice(simd, values)
    }
    #[inline(always)]
    fn store_to(self, values: &mut [i32]) {
        self.store_slice(values);
    }
}

impl<S: Simd> Memory<S, i32> for i32x4<S> {
    #[inline(always)]
    fn load_from(simd: S, values: &[i32]) -> Self {
        Self::from_slice(simd, values)
    }
    #[inline(always)]
    fn store_to(self, values: &mut [i32]) {
        self.store_slice(values);
    }
}

/// The 4-lane passes at 8 bpc: `int16_t` coefficients and `tmp` in 32-bit
/// lanes.
impl<S: Simd> Memory<S, i16> for i32x4<S> {
    #[inline(always)]
    fn load_from(simd: S, values: &[i16]) -> Self {
        let mut eight = [0; 8];
        eight[..4].copy_from_slice(values);
        i16x8::from_slice(simd, &eight).widen().0
    }
    /// The values are within `int16_t` (clipped to it between the passes).
    #[inline(always)]
    fn store_to(self, values: &mut [i16]) {
        values.copy_from_slice(&self.relaxed_narrow(self).as_slice()[..4]);
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

/// `is_rect2` of a block size: the coefficients are scaled by 181 / 256
/// first.
pub const fn is_rect2(w: usize, h: usize) -> bool {
    w == 2 * h || h == 2 * w
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

/// `inv_txfm_add_c` for a `W`x`H` block in DSP-table slot `SLOT` (not
/// WHT); `SW`, `SH` are `min(W, 32)`, `min(H, 32)`.
pub fn inverse_transform_add<
    S: Level,
    P: Pixel,
    C: Coef,
    const W: usize,
    const H: usize,
    const SW: usize,
    const SH: usize,
    const SLOT: usize,
>(
    simd: S,
    block: Block<'_, P, C, W, H>,
    depth: Depth,
) {
    let tx_type = const { kernel_type(SLOT) };
    let Block { mut rows, coeff, eob, extent } = block;
    let pass = Pass { kinds: TX1D_TYPES[tx_type], rect2: is_rect2(W, H), shift: shift(W, H), depth };
    if tx_type == DCT_DCT && eob == 0 {
        return dc_only::<_, _, _, W, H>(simd, &mut rows, coeff, pass);
    }
    // Where only the first 8 or 16 rows and columns of coefficients are
    // coded, the passes get that bound as a constant, and the compiler drops
    // the zero inputs from the transforms (as the C's tx64 variants and the
    // assembly's eob paths do by hand). Not for the 64-point sizes: rare, and
    // each bound is another copy of the passes.
    let (coded_rows, coded_columns) = coded_extent(tx_type, W, SH, eob, extent);
    let within =
        |bound: usize| W <= 32 && H <= 32 && coded_rows <= bound && coded_columns <= bound && (SW > bound || SH > bound);
    if within(8) {
        passes::<_, _, _, W, H, 8, 8>(simd, &mut rows, coeff, coded_rows, pass);
    } else if within(16) {
        passes::<_, _, _, W, H, 16, 16>(simd, &mut rows, coeff, coded_rows, pass);
    } else {
        passes::<_, _, _, W, H, SW, SH>(simd, &mut rows, coeff, coded_rows, pass);
    }
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
///
/// On x86-64 the 4x4, 4x8 and 8x4 blocks run both passes in one region,
/// so that `tmp` stays in registers and no pass is a call of a region of
/// its own (with the AVX2 features: not inlined): for these blocks the two
/// calls cost about as much as the transforms (4x4 DCT_DCT 359 -> 300
/// instructions). The region is shared by the kernels of the block size (it
/// takes the transforms as values). On AArch64 the same single region cost
/// 12% more instructions on these blocks than the two pass functions, so
/// they keep those.
#[inline(always)]
fn passes<S: Level, P: Pixel, C: Coef, const W: usize, const H: usize, const NX: usize, const NY: usize>(
    simd: S,
    rows: &mut [&mut [P]; H],
    coeff: &mut [C],
    coded_rows: usize,
    pass: Pass,
) {
    use tx1d::*;
    let sh = H.min(32);
    let mut tmp = [[C::ZERO; W]; NY];
    if const { W * H <= 32 && cfg!(target_arch = "x86_64") } {
        simd.vectorize(
            #[inline(always)]
            || {
                let pass = Pass { depth: C::depth(pass.depth), ..pass };
                match pass.kinds[0] {
                    Kind::Dct => first_pass_lanes::<_, _, Dct, W, H, NX, NY>(simd, coeff, coded_rows, pass, &mut tmp),
                    Kind::Adst => first_pass_lanes::<_, _, Adst, W, H, NX, NY>(simd, coeff, coded_rows, pass, &mut tmp),
                    Kind::FlipAdst => first_pass_lanes::<_, _, FlipAdst, W, H, NX, NY>(simd, coeff, coded_rows, pass, &mut tmp),
                    Kind::Identity => first_pass_lanes::<_, _, Identity, W, H, NX, NY>(simd, coeff, coded_rows, pass, &mut tmp),
                }
                let tmp = tmp.as_flattened();
                match pass.kinds[1] {
                    Kind::Dct => second_pass_lanes::<_, _, _, Dct, W, H>(simd, tmp, pass.depth, rows),
                    Kind::Adst => second_pass_lanes::<_, _, _, Adst, W, H>(simd, tmp, pass.depth, rows),
                    Kind::FlipAdst => second_pass_lanes::<_, _, _, FlipAdst, W, H>(simd, tmp, pass.depth, rows),
                    Kind::Identity => second_pass_lanes::<_, _, _, Identity, W, H>(simd, tmp, pass.depth, rows),
                }
            },
        );
        return;
    }
    let tmp_rows = &mut tmp;
    match pass.kinds[0] {
        Kind::Dct => first_pass_shared::<_, _, Dct, W, H, NX, NY>(simd, coeff, coded_rows, pass, tmp_rows),
        Kind::Adst => first_pass_shared::<_, _, Adst, W, H, NX, NY>(simd, coeff, coded_rows, pass, tmp_rows),
        Kind::FlipAdst => first_pass_shared::<_, _, FlipAdst, W, H, NX, NY>(simd, coeff, coded_rows, pass, tmp_rows),
        Kind::Identity => first_pass_shared::<_, _, Identity, W, H, NX, NY>(simd, coeff, coded_rows, pass, tmp_rows),
    }
    let tmp = tmp[..NY.min(sh)].as_flattened();
    match pass.kinds[1] {
        Kind::Dct => second_pass_shared::<_, _, _, Dct, W, H>(simd, tmp, pass.depth, rows),
        Kind::Adst => second_pass_shared::<_, _, _, Adst, W, H>(simd, tmp, pass.depth, rows),
        Kind::FlipAdst => second_pass_shared::<_, _, _, FlipAdst, W, H>(simd, tmp, pass.depth, rows),
        Kind::Identity => second_pass_shared::<_, _, _, Identity, W, H>(simd, tmp, pass.depth, rows),
    }
}

/// The rows of the coefficient block: transformed with `T`, rounded,
/// clipped and stored into `tmp` (row `y` at `tmp[y]`), the coefficients
/// read set to 0. Rows from `coded_rows` up and columns from `NX` up hold no
/// nonzero coefficient; their part of `tmp` stays 0, as in the C.
///
/// One region with the level's features per row length `W`, lanes `V`,
/// bound `NX` and transform, shared by the kernels of every block size with
/// that row length: the block's height reaches it only as values (`sh`, and
/// `rect2` and `shift` in `pass`). Inlined into each kernel, or one copy per
/// block size, the passes made the library large and its build slow. On
/// x86-64 the region is a function of its own (it carries the AVX2
/// features), so it is shared as it is; on AArch64 the compiler inlines it,
/// and this function is kept out of line instead (for code size and build
/// time only).
#[cfg_attr(target_arch = "aarch64", inline(never))]
fn first_pass<S: Level, V: Memory<S, C>, C: Coef, T: Tx1d, const W: usize, const NX: usize>(
    simd: S,
    coeff: &mut [C],
    sh: usize,
    coded_rows: usize,
    pass: Pass,
    tmp: &mut [[C; W]],
) {
    simd.vectorize(
        #[inline(always)]
        || {
            // Inside the region: on x86-64 the region is a function of its
            // own, and a constant computed outside it would reach it only as
            // a value in memory.
            let pass = Pass { depth: C::depth(pass.depth), ..pass };
            rows_pass::<_, V, _, T, W, NX>(simd, coeff, sh, coded_rows, pass, tmp);
        },
    )
}

/// The lanes of the first pass of a `W`x`H` block whose rows from `NY` up
/// are zero (module documentation): 4 where the block is 4 wide or tall, the
/// natural width where `NY` fills it, else 8.
macro_rules! first_pass_lanes_of {
    ($pass:ident, $S:ident, $C:ident, $T:ident, $W:expr, $H:expr, $NX:expr, $NY:expr, $($arg:expr),*) => {
        if $W.min($H) == 4 {
            $pass::<_, $C::Lanes4<$S>, _, $T, $W, $NX>($($arg),*)
        } else if $NY.min($H) >= $C::Native::<$S>::LEN {
            $pass::<_, $C::Native<$S>, _, $T, $W, $NX>($($arg),*)
        } else {
            $pass::<_, $C::Lanes8<$S>, _, $T, $W, $NX>($($arg),*)
        }
    };
}

/// `first_pass` of a `W`x`H` block, on the lanes of the block.
#[inline(always)]
fn first_pass_shared<S: Level, C: Coef, T: Tx1d, const W: usize, const H: usize, const NX: usize, const NY: usize>(
    simd: S,
    coeff: &mut [C],
    coded_rows: usize,
    pass: Pass,
    tmp: &mut [[C; W]],
) {
    first_pass_lanes_of!(first_pass, S, C, T, W, H, NX, NY, simd, coeff, H.min(32), coded_rows, pass, tmp)
}

/// The rows of `first_pass`, inline (the x86-64 region of the small blocks).
#[inline(always)]
fn first_pass_lanes<S: Level, C: Coef, T: Tx1d, const W: usize, const H: usize, const NX: usize, const NY: usize>(
    simd: S,
    coeff: &mut [C],
    coded_rows: usize,
    pass: Pass,
    tmp: &mut [[C; W]],
) {
    first_pass_lanes_of!(rows_pass, S, C, T, W, H, NX, NY, simd, coeff, H.min(32), coded_rows, pass, tmp)
}

/// `first_pass` on lanes `V`, for a block of `sh` coefficient rows (`pass`:
/// its `rect2` and `shift`, and the depth as the coefficient type fixes it).
#[inline(always)]
fn rows_pass<S: Simd, V: Memory<S, C>, C: Coef, T: Tx1d, const W: usize, const NX: usize>(
    simd: S,
    coeff: &mut [C],
    sh: usize,
    coded_rows: usize,
    pass: Pass,
    tmp: &mut [[C; W]],
) {
    let depth = pass.depth;
    let lanes = V::LEN;
    let nx = NX.min(W.min(32));
    let row_clip = V::range(simd, depth.row_clip);
    let col_clip = V::range(simd, depth.col_clip);
    let shift = pass.shift;
    let rnd = (1 << shift) >> 1;
    for y0 in (0..coded_rows).step_by(lanes) {
        // Input x of rows y0..y0 + L; from `nx` up zero.
        let mut input = [V::splat(simd, Default::default()); W];
        for (x, c) in input[..nx].iter_mut().enumerate() {
            let column = &mut coeff[x * sh + y0..][..lanes];
            *c = V::load_from(simd, column);
            column.fill(C::ZERO);
            if pass.rect2 {
                *c = V::stored((c.wide() * 181 + 128) >> 8);
            }
        }
        // iclip((tmp[i] + rnd) >> shift, col_clip_min, col_clip_max)
        let mut out = transform(
            T::KIND,
            input,
            row_clip,
            #[inline(always)]
            |v: V| v.round_shift(shift).clip_stored(col_clip),
            #[inline(always)]
            |v| V::clip((v + rnd) >> shift, col_clip),
        );
        if W >= lanes {
            for (x0, square) in out.chunks_exact_mut(lanes).enumerate() {
                V::transpose(square);
                for (y, row) in square.iter().enumerate() {
                    row.store_to(&mut tmp[y0 + y][x0 * lanes..][..lanes]);
                }
            }
        } else {
            // 16 lanes, 8 columns: the left half of a square of zeros.
            let mut square = [V::splat(simd, Default::default()); 16];
            let square = &mut square[..lanes];
            square[..W].copy_from_slice(&out);
            V::transpose(square);
            for (y, row) in square.iter().enumerate() {
                row.store_to(&mut tmp[y0 + y]);
            }
        }
    }
}

/// The columns of `tmp` (`w` values per row; rows from the end of `tmp` up
/// are zero) transformed with `T` and added to the pixels:
/// `iclip_pixel(dst + ((c + 8) >> 4))`. One region per column length `H`,
/// lanes `V` and transform, shared by the kernels of every block size with
/// that height (the width reaches it as `w`), as `first_pass` is.
#[cfg_attr(target_arch = "aarch64", inline(never))]
fn second_pass<S: Level, V: Memory<S, C>, C: Coef, P: Pixel, T: Tx1d, const H: usize>(
    simd: S,
    tmp: &[C],
    w: usize,
    depth: Depth,
    rows: &mut [&mut [P]; H],
) {
    simd.vectorize(
        #[inline(always)]
        || {
            // Inside the region (as in `first_pass`).
            columns_pass::<_, V, _, _, T, H>(simd, tmp, w, C::depth(depth), rows);
        },
    )
}

/// The lanes of the second pass of a block `W` wide: 4 where it is 4 wide,
/// the natural width where its `W` columns fill it, else 8.
macro_rules! second_pass_lanes_of {
    ($pass:ident, $S:ident, $C:ident, $T:ident, $W:expr, $H:expr, $($arg:expr),*) => {
        if $W == 4 {
            $pass::<_, $C::Lanes4<$S>, _, _, $T, $H>($($arg),*)
        } else if $W >= $C::Native::<$S>::LEN {
            $pass::<_, $C::Native<$S>, _, _, $T, $H>($($arg),*)
        } else {
            $pass::<_, $C::Lanes8<$S>, _, _, $T, $H>($($arg),*)
        }
    };
}

/// `second_pass` of a `W`x`H` block, on the lanes of the block.
#[inline(always)]
fn second_pass_shared<S: Level, C: Coef, P: Pixel, T: Tx1d, const W: usize, const H: usize>(
    simd: S,
    tmp: &[C],
    depth: Depth,
    rows: &mut [&mut [P]; H],
) {
    second_pass_lanes_of!(second_pass, S, C, T, W, H, simd, tmp, W, depth, rows)
}

/// The columns of `second_pass`, inline (the x86-64 region of the small
/// blocks).
#[inline(always)]
fn second_pass_lanes<S: Level, C: Coef, P: Pixel, T: Tx1d, const W: usize, const H: usize>(
    simd: S,
    tmp: &[C],
    depth: Depth,
    rows: &mut [&mut [P]; H],
) {
    second_pass_lanes_of!(columns_pass, S, C, T, W, H, simd, tmp, W, depth, rows)
}

/// `second_pass` on lanes `V`.
#[inline(always)]
fn columns_pass<S: Simd, V: Memory<S, C>, C: Coef, P: Pixel, T: Tx1d, const H: usize>(
    simd: S,
    tmp: &[C],
    w: usize,
    depth: Depth,
    rows: &mut [&mut [P]; H],
) {
    let lanes = V::LEN;
    let col_clip = V::range(simd, depth.col_clip);
    for x0 in (0..w).step_by(lanes) {
        let mut input = [V::splat(simd, Default::default()); H];
        for (c, row) in input.iter_mut().zip(tmp.chunks_exact(w)) {
            *c = V::load_from(simd, &row[x0..][..lanes]);
        }
        // (c + 8) >> 4
        let residual = transform(
            T::KIND,
            input,
            col_clip,
            #[inline(always)]
            |v: V| v.round_shift(4),
            #[inline(always)]
            |v| V::stored((v + 8) >> 4),
        );
        for (row, r) in rows.iter_mut().zip(residual) {
            P::add(simd, &mut row[x0..][..lanes], r, depth.pixel_max);
        }
    }
}

/// `V::LEN` values converted to `i32` lanes.
#[inline(always)]
fn load<S: Simd, V: Int32<S>, T: Copy>(simd: S, values: &[T], to_i32: impl Fn(T) -> i32) -> V {
    let mut v = V::splat(simd, 0);
    for (lane, &value) in v.as_mut_slice().iter_mut().zip(values) {
        *lane = to_i32(value);
    }
    v
}

/// `dst = iclip_pixel(dst + residual)` for `V::LEN` pixels.
#[inline(always)]
fn add_to_pixels<S: Simd, V: Int32<S>, P: Pixel>(simd: S, pixels: &mut [P], residual: V, pixel_max: i32) {
    let sum = (load::<S, V, P>(simd, pixels, P::to_i32) + residual).max(0).min(pixel_max);
    P::store(sum, pixels);
}

/// The C's path for a DCT_DCT block with only the DC coefficient
/// (`eob < has_dconly`): one value added to every pixel, unclipped.
#[inline(always)]
fn dc_only<S: Simd, P: Pixel, C: Coef, const W: usize, const H: usize>(
    simd: S,
    rows: &mut [&mut [P]; H],
    coeff: &mut [C],
    pass: Pass,
) {
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
    // One region per block size, with the width as a constant and the value
    // moved in (not read through a reference on each row).
    let pixel_max = pass.depth.pixel_max;
    simd.vectorize(
        #[inline(always)]
        move || {
            for row in rows.iter_mut() {
                P::add_dc(simd, &mut row[..W], dc, pixel_max);
            }
        },
    )
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
            Lanes::transpose(&mut tmp);
            for (row, c) in rows.iter_mut().zip(wht4(tmp)) {
                add_to_pixels(simd, row, c, pixel_max);
            }
        },
    )
}
