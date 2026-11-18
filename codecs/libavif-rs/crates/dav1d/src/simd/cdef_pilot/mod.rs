// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Experiment (branch `v1/dav1d-cdef-pilot`, not for `main`): CDEF written
//! from the algorithm instead of transliterated from dav1d's assembly.
//!
//! Three variants of the same kernels, each meant to give the output of
//! `upstream/dav1d/src/cdef_tmpl.c` (and so of the original's assembly,
//! which checkasm holds to the C):
//!
//! - `v1_autovec`: plain Rust on fixed-size arrays, written so that LLVM
//!   vectorizes it; no intrinsics, no `unsafe`.
//! - `v2_fearless`: the same algorithm on the vector types of the
//!   `fearless_simd` crate (portable across NEON and AVX2).
//! - `v3_neon` / `v3_avx2`: the same algorithm with `core::arch`
//!   intrinsics, one implementation per architecture.
//!
//! This module holds what the three share: the DSP-table boundary (the
//! only raw-pointer code), the padding of the C `padding` function, the
//! parameters derived from the strengths, and the direction table.
//!
//! The switch: the module exists only when the crate is built with
//! `--cfg dav1d_cdef_pilot` (via `RUSTFLAGS`), so a normal build is the
//! shipped port, unchanged. In such a build the environment variable
//! `DAV1D_CDEF_PILOT=v1|v2|v3` picks a variant when the DSP table is
//! initialized (after the shipped init has run); unset, the shipped kernels
//! stay. With the CPU flags masked below NEON / AVX2 the C kernels stay as
//! before, so the scalar gate is unaffected.
//!
//! The numerics, argued once: `cdef_tmpl.c` computes in `int`. The variants
//! compute in 16-bit lanes, which is exact here:
//! - the padded block holds pixels (0..=4095) or the C's `INT16_MIN` marker;
//! - `|p - px|` is at most 32768 + 4095 and fits `u16` (`i16::abs_diff`);
//! - `constrain` is at most the strength (240 at 12 bits);
//! - the tap-weighted sum is at most 12 * 240 + 12 * 64 = 3648 in
//!   magnitude, so `i16` holds it and the rounding step;
//! - the C's `umin` sees the marker as 0xFFFF8000 and `imax` as -32768,
//!   i.e. the marker never wins: `u16` minimum and `i16` maximum do the same.
//!
//! The direction search squares partial sums of at most 8 values in
//! -128..=127 and weights them by `840 / n` for `n` pixels (AV1 spec 7.15.2,
//! `Div_Table`); the largest cost is below 2^30, so `u32` sums never wrap,
//! and a sum of `square * weight` per partial equals the C's grouping.

use core::ffi::{c_int, c_uint};

pub mod v1_autovec;
pub mod v2_fearless;
#[cfg(target_arch = "aarch64")]
pub mod v3_neon;
#[cfg(target_arch = "x86_64")]
pub mod v3_avx2;

/// `enum CdefEdgeFlags` (src/cdef.h).
pub const HAVE_LEFT: u32 = 1 << 0;
pub const HAVE_RIGHT: u32 = 1 << 1;
pub const HAVE_TOP: u32 = 1 << 2;
pub const HAVE_BOTTOM: u32 = 1 << 3;

/// The C's fill value for pixels outside the frame (`fill` in cdef_tmpl.c).
pub const PAD: i16 = i16::MIN;

/// The two taps (row, column) of each direction, from the block's pixel
/// outward: AV1 spec `Cdef_Directions`, dav1d's `dav1d_cdef_directions`
/// (rows of 12 there).
pub const DIRECTIONS: [[(isize, isize); 2]; 8] = [
    [(-1, 1), (-2, 2)],
    [(0, 1), (-1, 2)],
    [(0, 1), (0, 2)],
    [(0, 1), (1, 2)],
    [(1, 1), (2, 2)],
    [(1, 0), (2, 1)],
    [(1, 0), (2, 0)],
    [(1, 0), (2, -1)],
];

/// The weight of a squared partial sum of `n` pixels in the direction
/// search, `840 / n` (AV1 spec `Div_Table`), per partial-sum index.
pub const DIAGONAL_WEIGHTS: [u32; 15] = [840, 420, 280, 210, 168, 140, 120, 105, 120, 140, 168, 210, 280, 420, 840];
pub const ALT_WEIGHTS: [u32; 11] = [420, 210, 140, 105, 105, 105, 105, 105, 140, 210, 420];
pub const LINE_WEIGHT: u32 = 105;

/// A pixel type of the DSP table: `u8` at 8 bpc, `u16` above.
pub trait Pixel: Copy + Default + 'static {
    fn to_i16(self) -> i16;
    /// The C's conversion from `int` to `pixel` (truncating).
    fn from_i16(value: i16) -> Self;
}

impl Pixel for u8 {
    #[inline]
    fn to_i16(self) -> i16 {
        self.into()
    }
    #[inline]
    fn from_i16(value: i16) -> Self {
        value as u8
    }
}

impl Pixel for u16 {
    #[inline]
    fn to_i16(self) -> i16 {
        self as i16
    }
    #[inline]
    fn from_i16(value: i16) -> Self {
        value as u16
    }
}

/// The block and two pixels around it, as the C `padding` builds `tmp`:
/// `rows[y + 2][x + 2]` for block rows `-2..h+2` and columns `-2..w+2`,
/// `PAD` where the frame has no pixel.
pub struct Padded {
    pub rows: [[i16; 12]; 12],
}

impl Padded {
    /// `N` values of block row `y` from block column `x` on.
    #[inline]
    pub fn at<const N: usize>(&self, y: isize, x: isize) -> &[i16; N] {
        let row = &self.rows[(y + 2) as usize];
        row[(x + 2) as usize..][..N].try_into().unwrap()
    }

    /// The pixels of rows `y..` at offset `(dy, dx)`, `W` per row, as one
    /// array of `L` lanes (`L / W` rows).
    #[inline]
    pub fn lanes<const W: usize, const L: usize>(&self, y: usize, (dy, dx): (isize, isize)) -> [i16; L] {
        let mut lanes = [0; L];
        for (r, chunk) in lanes.chunks_exact_mut(W).enumerate() {
            chunk.copy_from_slice(self.at::<W>((y + r) as isize + dy, dx));
        }
        lanes
    }
}

/// The strengths and what the C derives from them.
#[derive(Clone, Copy)]
pub struct Params {
    pub dir: usize,
    pub pri_strength: u16,
    pub sec_strength: u16,
    /// Primary taps of the inner and the outer pixel.
    pub pri_taps: [i16; 2],
    pub pri_shift: u32,
    pub sec_shift: u32,
}

impl Params {
    pub fn new(pri_strength: c_int, sec_strength: c_int, dir: c_int, damping: c_int, bitdepth_min_8: u32) -> Self {
        let ulog2 = |v: c_int| (v as u32).ilog2() as c_int;
        let pri_tap = 4 - ((pri_strength >> bitdepth_min_8) & 1) as i16;
        Params {
            dir: dir as usize,
            pri_strength: pri_strength as u16,
            sec_strength: sec_strength as u16,
            // "if pri_tap_k == 4 then it becomes 2 else it remains 3"
            pri_taps: [pri_tap, (pri_tap & 3) | 2],
            pri_shift: if pri_strength != 0 { (damping - ulog2(pri_strength)).max(0) as u32 } else { 0 },
            sec_shift: if sec_strength != 0 { (damping - ulog2(sec_strength)) as u32 } else { 0 },
        }
    }

    /// The primary tap pair `k` (0 inner, 1 outer).
    #[inline]
    pub fn primary(&self, k: usize) -> (isize, isize) {
        DIRECTIONS[self.dir][k]
    }

    /// The two secondary tap pairs `k`: directions `dir + 2` and `dir - 2`.
    #[inline]
    pub fn secondary(&self, k: usize) -> [(isize, isize); 2] {
        [DIRECTIONS[(self.dir + 2) & 7][k], DIRECTIONS[(self.dir + 6) & 7][k]]
    }
}

/// The negated offset: the tap on the other side of the pixel.
#[inline]
pub fn opposite((dy, dx): (isize, isize)) -> (isize, isize) {
    (-dy, -dx)
}

/// The rounding of the C: `(sum - (sum < 0) + 8) >> 4`.
#[inline]
pub fn round_sum(sum: i16) -> i16 {
    (sum - (sum < 0) as i16 + 8) >> 4
}

/// The filtered block, rows `0..h`, columns `0..w`.
pub type Filtered = [[i16; 8]; 8];

/// The direction search's input: `(pixel >> (bitdepth - 8)) - 128`.
pub type DirBlock = [[i16; 8]; 8];

/// Best direction and its variance from the eight costs (C: the first
/// maximum wins).
pub fn best_direction(cost: [u32; 8]) -> (c_int, c_uint) {
    let mut best_dir = 0;
    for n in 1..8 {
        if cost[n] > cost[best_dir] {
            best_dir = n;
        }
    }
    (best_dir as c_int, (cost[best_dir] - cost[best_dir ^ 4]) >> 10)
}

/// One implementation of the kernels.
pub trait Variant {
    /// # Safety
    /// On x86-64 the CPU has AVX2 (the variant is installed only then).
    unsafe fn filter<const W: usize, const H: usize>(input: &Padded, params: &Params) -> Filtered;
    /// # Safety
    /// As for `filter`.
    unsafe fn find_dir(block: &DirBlock) -> (c_int, c_uint);
}

// --- the DSP-table boundary ----------------------------------------------------

/// The C `padding` (cdef_tmpl.c): reads the block's pre-filter
/// neighborhood. Every read is one the C makes: block rows `0..h` at
/// columns `0..w` (and `w..w+2` with a right edge), `left[0..h]` with a
/// left edge, the two rows at `top` and at `bottom` with those edges, at
/// columns `-2..w+2` as the left and right edges allow.
///
/// # Safety
/// The original's contract of a `cdef_fn` (`cdef_apply_tmpl.c`): those
/// pixels are readable; `stride` is in bytes.
#[inline]
unsafe fn read_neighborhood<P: Pixel>(
    dst: *const P,
    stride: isize,
    left: *const [P; 2],
    top: *const P,
    bottom: *const P,
    w: usize,
    h: usize,
    edges: u32,
) -> Padded {
    let mut padded = Padded { rows: [[PAD; 12]; 12] };
    let stride = stride / core::mem::size_of::<P>() as isize;
    // The padded columns that come from the frame: `x0..x1` (`-2..w+2` at
    // full edges).
    let x0 = if edges & HAVE_LEFT != 0 { 0 } else { 2 };
    let x1 = w + if edges & HAVE_RIGHT != 0 { 4 } else { 2 };
    let row = |base: *const P, y: isize, from: usize| {
        // SAFETY: a row the contract makes readable, columns `from - 2..x1 - 2`.
        unsafe { core::slice::from_raw_parts(base.offset(y * stride + from as isize - 2), x1 - from) }
    };
    let mut copy = |y: usize, from: usize, src: &[P]| {
        for (d, s) in padded.rows[y][from..x1].iter_mut().zip(src) {
            *d = s.to_i16();
        }
    };
    if edges & HAVE_TOP != 0 {
        copy(0, x0, row(top, 0, x0));
        copy(1, x0, row(top, 1, x0));
    }
    for y in 0..h {
        copy(y + 2, 2, row(dst, y as isize, 2));
    }
    if edges & HAVE_BOTTOM != 0 {
        copy(h + 2, x0, row(bottom, 0, x0));
        copy(h + 3, x0, row(bottom, 1, x0));
    }
    if edges & HAVE_LEFT != 0 {
        // SAFETY: `left[0..h]` is readable with a left edge.
        let left = unsafe { core::slice::from_raw_parts(left, h) };
        for (row, pair) in padded.rows[2..h + 2].iter_mut().zip(left) {
            row[0] = pair[0].to_i16();
            row[1] = pair[1].to_i16();
        }
    }
    padded
}

/// Writes rows `0..h`, columns `0..w` of the filtered block to `dst`.
///
/// # Safety
/// Those pixels are writable and not otherwise borrowed.
#[inline]
unsafe fn write_block<P: Pixel>(dst: *mut P, stride: isize, block: &Filtered, w: usize, h: usize) {
    let stride = stride / core::mem::size_of::<P>() as isize;
    for (y, values) in block[..h].iter().enumerate() {
        // SAFETY: row `y` of the block, `w` pixels.
        let row = unsafe { core::slice::from_raw_parts_mut(dst.offset(y as isize * stride), w) };
        for (d, &v) in row.iter_mut().zip(values) {
            *d = P::from_i16(v);
        }
    }
}

/// A `cdef_fn` at 8 bpc for variant `V` and block size `W`x`H`.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
unsafe extern "C" fn filter_8bpc<V: Variant, const W: usize, const H: usize>(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    let input = read_neighborhood(dst, stride, left, top, bottom, W, H, edges);
    let params = Params::new(pri_strength, sec_strength, dir, damping, 0);
    let output = V::filter::<W, H>(&input, &params);
    write_block(dst, stride, &output, W, H);
}

/// A `cdef_fn` at 16 bpc.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
unsafe extern "C" fn filter_16bpc<V: Variant, const W: usize, const H: usize>(
    dst: *mut u16,
    stride: isize,
    left: *const [u16; 2],
    top: *const u16,
    bottom: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
    bitdepth_max: c_int,
) {
    let input = read_neighborhood(dst, stride, left, top, bottom, W, H, edges);
    let bitdepth_min_8 = (bitdepth_max as u32 + 1).ilog2() - 8;
    let params = Params::new(pri_strength, sec_strength, dir, damping, bitdepth_min_8);
    let output = V::filter::<W, H>(&input, &params);
    write_block(dst, stride, &output, W, H);
}

/// Reads the 8x8 block of the direction search.
///
/// # Safety
/// 8 rows of 8 pixels at `img` with `stride` bytes are readable.
#[inline]
unsafe fn read_dir_block<P: Pixel>(img: *const P, stride: isize, bitdepth_min_8: u32) -> DirBlock {
    let stride = stride / core::mem::size_of::<P>() as isize;
    core::array::from_fn(|y| {
        // SAFETY: row `y` of the block.
        let row = unsafe { core::slice::from_raw_parts(img.offset(y as isize * stride), 8) };
        core::array::from_fn(|x| (row[x].to_i16() >> bitdepth_min_8) - 128)
    })
}

/// A `cdef_dir_fn` at 8 bpc.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
unsafe extern "C" fn dir_8bpc<V: Variant>(img: *const u8, stride: isize, var: *mut c_uint) -> c_int {
    let (dir, variance) = V::find_dir(&read_dir_block(img, stride, 0));
    *var = variance;
    dir
}

/// A `cdef_dir_fn` at 16 bpc.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
unsafe extern "C" fn dir_16bpc<V: Variant>(img: *const u16, stride: isize, var: *mut c_uint, bitdepth_max: c_int) -> c_int {
    let bitdepth_min_8 = (bitdepth_max as u32 + 1).ilog2() - 8;
    let (dir, variance) = V::find_dir(&read_dir_block(img, stride, bitdepth_min_8));
    *var = variance;
    dir
}

// --- the switch ---------------------------------------------------------------

#[cfg(target_arch = "aarch64")]
use super::arm::cdef::{Dav1dCdefDSPContext, Dav1dCdefDSPContext16};
#[cfg(target_arch = "x86_64")]
use super::x86::cdef::{Dav1dCdefDSPContext, Dav1dCdefDSPContext16};

/// The variant `DAV1D_CDEF_PILOT` names, if any.
fn selected() -> Option<&'static str> {
    match std::env::var("DAV1D_CDEF_PILOT").ok()?.as_str() {
        "v1" => Some("v1"),
        "v2" => Some("v2"),
        "v3" => Some("v3"),
        other => panic!("DAV1D_CDEF_PILOT={other}: expected v1, v2 or v3"),
    }
}

fn install8<V: Variant>(c: &mut Dav1dCdefDSPContext) {
    c.dir = Some(dir_8bpc::<V>);
    c.fb[0] = Some(filter_8bpc::<V, 8, 8>);
    c.fb[1] = Some(filter_8bpc::<V, 4, 8>);
    c.fb[2] = Some(filter_8bpc::<V, 4, 4>);
}

fn install16<V: Variant>(c: &mut Dav1dCdefDSPContext16) {
    c.dir = Some(dir_16bpc::<V>);
    c.fb[0] = Some(filter_16bpc::<V, 8, 8>);
    c.fb[1] = Some(filter_16bpc::<V, 4, 8>);
    c.fb[2] = Some(filter_16bpc::<V, 4, 4>);
}

#[cfg(target_arch = "aarch64")]
type V3 = v3_neon::Neon;
#[cfg(target_arch = "x86_64")]
type V3 = v3_avx2::Avx2;

/// Called at the end of the shipped `cdef_dsp_init_*_8bpc`, only where it
/// installed its SIMD kernels (NEON, AVX2): replaces them by the selected
/// variant.
pub fn install_8bpc(c: &mut Dav1dCdefDSPContext) {
    match selected() {
        Some("v1") => install8::<v1_autovec::AutoVec>(c),
        Some("v2") if v2_fearless::available() => install8::<v2_fearless::Fearless>(c),
        Some("v3") => install8::<V3>(c),
        _ => {}
    }
}

/// As `install_8bpc`, at 16 bpc.
pub fn install_16bpc(c: &mut Dav1dCdefDSPContext16) {
    match selected() {
        Some("v1") => install16::<v1_autovec::AutoVec>(c),
        Some("v2") if v2_fearless::available() => install16::<v2_fearless::Fearless>(c),
        Some("v3") => install16::<V3>(c),
        _ => {}
    }
}

#[cfg(test)]
mod tests;
