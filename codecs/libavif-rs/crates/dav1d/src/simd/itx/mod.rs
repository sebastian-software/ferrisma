// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! The inverse transforms written from the algorithm (ADR 0062, "Kernels
//! written from the algorithm") with the portable vector types of
//! `fearless_simd`, as CDEF (`simd/cdef/`), one implementation for NEON and
//! for x86-64 AVX2, at 8, 10 and 12 bits.
//!
//! - `transforms`: the 1-D transforms of `itx_1d.c` (DCT 4..64, ADST 4..16,
//!   flipped ADST, identity, WHT) on vectors, one row or column per lane;
//! - `blocks`: `inv_txfm_add_c` of `itx_tmpl.c` (two passes, the transpose
//!   between them, rounding and clipping as the C does, the C's DC-only path)
//!   and the 4x4 WHT;
//! - this module: the DSP-table boundary (the only raw-pointer code) and the
//!   kernels' table, from which the init mirrors fill their slots.
//!
//! The numerics: the C computes in `int`; the vectors compute the same
//! expressions, so every value is the C's (including the C's rearranged
//! products, which keep 12-bit input free of overflow). At 10 and 12 bits
//! every lane is 32 bits wide. At 8 bits the passes keep the values the C
//! has clipped (and the inputs) in 16-bit lanes and widen to 32 bits only
//! where the C forms a value beyond `int16_t` before it clips it: the
//! products, the sums they multiply, the unclipped outputs of the ADST and
//! the identities. The documentation of `transforms` derives the ranges that
//! make this exact for every `int16_t` coefficient, conformant or not; the
//! clips to `int16_t` are then the saturating 16-bit operations. The 16-bit
//! vectors have the level's natural width (`Simd::i16s`): 8 lanes on NEON,
//! 16 on AVX2, where a pass has that many rows or columns; the layout and
//! its transposes are in the documentation of `blocks`. The passes with 4
//! lanes (blocks 4 wide or tall) and the WHT stay on 32 bits. The final
//! addition to the pixels runs in 16-bit lanes, where that is exact
//! (`blocks::Pixel`).
//!
//! So the kernels give the C's pixels for every input. The original's
//! assembly gives them where the intermediates stay within the ranges of a
//! conformant stream (itx_1d.c: SIMD may overflow beyond them); with
//! coefficients over the whole `int16_t` range at 8 bpc about half of the
//! calls differ between the original's C and its NEON or AVX2 kernels, and
//! there these kernels follow the C (engine ADR-0041: a kernel written from
//! the algorithm matches the original's SIMD on conformant input and its C
//! on non-conformant input).
//!
//! The installation: the init mirrors (`simd/arm/itx.rs`, `simd/x86/itx.rs`)
//! keep the original's selection, the `assign_itx*_fn` calls of its header
//! under its CPU flags, and fill those slots from `Kernels` (`ITX1` …
//! `ITX16` name the slots of each macro). Below NEON / AVX2, and where the
//! original keeps C (12 bits on ARM64 but WHT, the large DCT_DCT slots at 12
//! bits on x86-64), the C kernels stay. On x86-64 `fearless_simd`'s `Avx2`
//! level needs x86-64-v3, a little more than dav1d's AVX2 flag; on a CPU
//! with the flag but without that level `Kernels::new_*` gives `None` and the
//! C kernels stay (with `all_simd` 0, as their tables need).

use core::ffi::c_int;
use std::sync::OnceLock;

mod blocks;
pub mod transforms;

use blocks::{Block, Depth, ScanExtent};

#[cfg(target_arch = "aarch64")]
type Level = fearless_simd::aarch64::Neon;
#[cfg(target_arch = "x86_64")]
type Level = fearless_simd::x86::Avx2;

/// Whether the CPU has the level the kernels are compiled for (on x86-64 the
/// library's `Avx2` needs x86-64-v3; without it the C kernels stay).
fn available() -> bool {
    #[cfg(target_arch = "aarch64")]
    return fearless_simd::Level::new().as_neon().is_some();
    #[cfg(target_arch = "x86_64")]
    return fearless_simd::Level::new().as_avx2().is_some();
}

/// The level's token.
///
/// # Safety
/// `available()` is true (the kernels are installed only then).
#[inline(always)]
unsafe fn level() -> Level {
    // SAFETY: the caller's contract.
    unsafe { Level::assume_supported() }
}

/// `itxfm_fn` (src/itx.h) at 8 bpc.
pub type ItxFn = unsafe extern "C" fn(*mut u8, isize, *mut i16, c_int);
/// `itxfm_fn` at 16 bpc, with `bitdepth_max`.
pub type ItxFn16 = unsafe extern "C" fn(*mut u16, isize, *mut i32, c_int, c_int);

/// `enum RectTxfmSize` (src/levels.h): width and height of each index.
const SIZES: [(usize, usize); 19] = [
    (4, 4),
    (8, 8),
    (16, 16),
    (32, 32),
    (64, 64),
    (4, 8),
    (8, 4),
    (8, 16),
    (16, 8),
    (16, 32),
    (32, 16),
    (32, 64),
    (64, 32),
    (4, 16),
    (16, 4),
    (8, 32),
    (32, 8),
    (16, 64),
    (64, 16),
];

/// The `enum RectTxfmSize` of a `w`x`h` block.
const fn rect_tx_size(w: usize, h: usize) -> usize {
    let mut tx = 0;
    while SIZES[tx].0 != w || SIZES[tx].1 != h {
        tx += 1;
    }
    tx
}

// `enum RectTxfmSize` and `enum TxfmType` (src/levels.h).
pub const TX_4X4: usize = 0;
pub const TX_8X8: usize = 1;
pub const TX_16X16: usize = 2;
pub const TX_32X32: usize = 3;
pub const TX_64X64: usize = 4;
pub const RTX_4X8: usize = 5;
pub const RTX_8X4: usize = 6;
pub const RTX_8X16: usize = 7;
pub const RTX_16X8: usize = 8;
pub const RTX_16X32: usize = 9;
pub const RTX_32X16: usize = 10;
pub const RTX_32X64: usize = 11;
pub const RTX_64X32: usize = 12;
pub const RTX_4X16: usize = 13;
pub const RTX_16X4: usize = 14;
pub const RTX_8X32: usize = 15;
pub const RTX_32X8: usize = 16;
pub const RTX_16X64: usize = 17;
pub const RTX_64X16: usize = 18;
pub const DCT_DCT: usize = 0;
pub const ADST_DCT: usize = 1;
pub const DCT_ADST: usize = 2;
pub const ADST_ADST: usize = 3;
pub const FLIPADST_DCT: usize = 4;
pub const DCT_FLIPADST: usize = 5;
pub const FLIPADST_FLIPADST: usize = 6;
pub const ADST_FLIPADST: usize = 7;
pub const FLIPADST_ADST: usize = 8;
pub const IDTX: usize = 9;
pub const V_DCT: usize = 10;
pub const H_DCT: usize = 11;
pub const V_ADST: usize = 12;
pub const H_ADST: usize = 13;
pub const V_FLIPADST: usize = 14;
pub const H_FLIPADST: usize = 15;
pub const WHT_WHT: usize = 16;

/// The slots of `assign_itx1_fn` (src/arm/itx.h, src/x86/itx.h): DCT_DCT.
pub const ITX1: &[usize] = &[DCT_DCT];
/// The slots of `assign_itx2_fn`: and the identity.
pub const ITX2: &[usize] = &[DCT_DCT, IDTX];
/// The slots of `assign_itx12_fn`: and the ADST, flipped ADST and V_DCT /
/// H_DCT combinations.
pub const ITX12: &[usize] = &[
    DCT_DCT,
    IDTX,
    ADST_DCT,
    FLIPADST_DCT,
    H_DCT,
    DCT_ADST,
    ADST_ADST,
    FLIPADST_ADST,
    DCT_FLIPADST,
    ADST_FLIPADST,
    FLIPADST_FLIPADST,
    V_DCT,
];
/// The slots of `assign_itx16_fn`: every type but WHT.
pub const ITX16: &[usize] = &[
    DCT_DCT,
    IDTX,
    ADST_DCT,
    FLIPADST_DCT,
    H_DCT,
    DCT_ADST,
    ADST_ADST,
    FLIPADST_ADST,
    DCT_FLIPADST,
    ADST_FLIPADST,
    FLIPADST_FLIPADST,
    V_DCT,
    H_ADST,
    H_FLIPADST,
    V_ADST,
    V_FLIPADST,
];
/// The slot of `assign_itx_fn(…, identity_identity, IDTX, …)`.
pub const IDENTITY: &[usize] = &[IDTX];

// --- the DSP-table boundary ----------------------------------------------------

/// For each `enum RectTxfmSize`, the extent of the scan up to each `eob`:
/// the C's `dav1d_last_nonzero_col_from_eob` (scan.c, `init_tbl`), which the
/// C builds only while some slot keeps a C kernel, and the same for the
/// columns.
fn scan_extents() -> &'static [ScanExtent; 19] {
    static TABLE: OnceLock<[ScanExtent; 19]> = OnceLock::new();
    TABLE.get_or_init(|| {
        core::array::from_fn(|tx| {
            let (w, h) = SIZES[tx];
            let (sw, sh) = (w.min(32), h.min(32));
            // SAFETY: `dav1d_scans` is a constant table of `sw * sh` entries
            // per size (scan.c); nothing writes it.
            let scan = unsafe {
                let scans = *core::ptr::addr_of!(crate::common::scan::dav1d_scans);
                core::slice::from_raw_parts(scans[tx], sw * sh)
            };
            // Coefficient (x, y) is at `y + x * sh`.
            ScanExtent { rows: running_max(scan, |rc| rc % sh), columns: running_max(scan, |rc| rc / sh) }
        })
    })
}

/// For each scan position, the largest `coordinate` of the positions up to it.
fn running_max(scan: &[u16], coordinate: impl Fn(usize) -> usize) -> Box<[u8]> {
    let mut max = 0;
    scan.iter()
        .map(|&rc| {
            max = max.max(coordinate(rc.into()));
            max as u8
        })
        .collect()
}

/// `last_nonzero_col + 1` of `inv_txfm_add_c` (for the tests).
#[cfg(test)]
fn coded_rows(tx: usize, tx_type: usize, eob: usize) -> usize {
    let (w, h) = SIZES[tx];
    blocks::coded_extent(tx_type, w, h.min(32), eob, &scan_extents()[tx]).0
}

/// The `H` rows of `W` pixels at `dst`.
///
/// # Safety
/// The original's contract of an `itxfm_fn`: the block's pixels are
/// writable, not otherwise borrowed, and `stride` (bytes) separates its rows.
#[inline(always)]
unsafe fn rows<'a, P, const W: usize, const H: usize>(dst: *mut P, stride: isize) -> [&'a mut [P]; H] {
    let stride = stride / core::mem::size_of::<P>() as isize;
    // SAFETY: row `y` of the block (the caller's contract).
    core::array::from_fn(|y| unsafe { core::slice::from_raw_parts_mut(dst.offset(y as isize * stride), W) })
}

/// The block's `len` coefficients.
///
/// # Safety
/// The contract of an `itxfm_fn`: `min(W, 32) * min(H, 32)` coefficients,
/// not otherwise borrowed; the ones past `eob` in scan order are zero.
#[inline(always)]
unsafe fn coefficients<'a, C>(coeff: *mut C, len: usize) -> &'a mut [C] {
    // SAFETY: the caller's contract.
    unsafe { core::slice::from_raw_parts_mut(coeff, len) }
}

/// The block of an `itxfm_fn` call (not WHT).
///
/// # Safety
/// As for `rows` and `coefficients`.
#[inline(always)]
unsafe fn block<'a, P, C, const W: usize, const H: usize>(
    dst: *mut P,
    stride: isize,
    coeff: *mut C,
    eob: c_int,
) -> Block<'a, P, C, W, H> {
    let tx = const { rect_tx_size(W, H) };
    Block {
        rows: rows::<P, W, H>(dst, stride),
        coeff: coefficients(coeff, W.min(32) * H.min(32)),
        eob: eob as usize,
        extent: &scan_extents()[tx],
    }
}

/// An `itxfm_fn` at 8 bpc: block `W`x`H` (`SW`, `SH` = min(W, 32), min(H,
/// 32)) for DSP-table
/// slot `SLOT` (an `enum TxfmType` but WHT).
unsafe extern "C" fn itx_8bpc<const W: usize, const H: usize, const SW: usize, const SH: usize, const SLOT: usize>(
    dst: *mut u8,
    stride: isize,
    coeff: *mut i16,
    eob: c_int,
) {
    let block = block::<_, _, W, H>(dst, stride, coeff, eob);
    blocks::inverse_transform_add::<_, _, _, W, H, SW, SH, SLOT>(level(), block, Depth::eight());
}

/// An `itxfm_fn` at 16 bpc.
unsafe extern "C" fn itx_16bpc<const W: usize, const H: usize, const SW: usize, const SH: usize, const SLOT: usize>(
    dst: *mut u16,
    stride: isize,
    coeff: *mut i32,
    eob: c_int,
    bitdepth_max: c_int,
) {
    let block = block::<_, _, W, H>(dst, stride, coeff, eob);
    blocks::inverse_transform_add::<_, _, _, W, H, SW, SH, SLOT>(level(), block, Depth::high(bitdepth_max));
}

/// The WHT `itxfm_fn` at 8 bpc.
unsafe extern "C" fn wht_8bpc(dst: *mut u8, stride: isize, coeff: *mut i16, _eob: c_int) {
    blocks::wht_add(level(), &mut rows::<_, 4, 4>(dst, stride), coefficients(coeff, 16), Depth::eight().pixel_max);
}

/// The WHT `itxfm_fn` at 16 bpc.
unsafe extern "C" fn wht_16bpc(dst: *mut u16, stride: isize, coeff: *mut i32, _eob: c_int, bitdepth_max: c_int) {
    blocks::wht_add(level(), &mut rows::<_, 4, 4>(dst, stride), coefficients(coeff, 16), bitdepth_max);
}

// --- the kernels' table -------------------------------------------------------

/// The entries of one block size, by `enum TxfmType` (WHT apart).
fn entries_8bpc<const W: usize, const H: usize, const SW: usize, const SH: usize>() -> [ItxFn; 16] {
    [
        itx_8bpc::<W, H, SW, SH, 0>,
        itx_8bpc::<W, H, SW, SH, 1>,
        itx_8bpc::<W, H, SW, SH, 2>,
        itx_8bpc::<W, H, SW, SH, 3>,
        itx_8bpc::<W, H, SW, SH, 4>,
        itx_8bpc::<W, H, SW, SH, 5>,
        itx_8bpc::<W, H, SW, SH, 6>,
        itx_8bpc::<W, H, SW, SH, 7>,
        itx_8bpc::<W, H, SW, SH, 8>,
        itx_8bpc::<W, H, SW, SH, 9>,
        itx_8bpc::<W, H, SW, SH, 10>,
        itx_8bpc::<W, H, SW, SH, 11>,
        itx_8bpc::<W, H, SW, SH, 12>,
        itx_8bpc::<W, H, SW, SH, 13>,
        itx_8bpc::<W, H, SW, SH, 14>,
        itx_8bpc::<W, H, SW, SH, 15>,
    ]
}

/// As `entries_8bpc`, at 16 bpc.
fn entries_16bpc<const W: usize, const H: usize, const SW: usize, const SH: usize>() -> [ItxFn16; 16] {
    [
        itx_16bpc::<W, H, SW, SH, 0>,
        itx_16bpc::<W, H, SW, SH, 1>,
        itx_16bpc::<W, H, SW, SH, 2>,
        itx_16bpc::<W, H, SW, SH, 3>,
        itx_16bpc::<W, H, SW, SH, 4>,
        itx_16bpc::<W, H, SW, SH, 5>,
        itx_16bpc::<W, H, SW, SH, 6>,
        itx_16bpc::<W, H, SW, SH, 7>,
        itx_16bpc::<W, H, SW, SH, 8>,
        itx_16bpc::<W, H, SW, SH, 9>,
        itx_16bpc::<W, H, SW, SH, 10>,
        itx_16bpc::<W, H, SW, SH, 11>,
        itx_16bpc::<W, H, SW, SH, 12>,
        itx_16bpc::<W, H, SW, SH, 13>,
        itx_16bpc::<W, H, SW, SH, 14>,
        itx_16bpc::<W, H, SW, SH, 15>,
    ]
}

/// The entries, by `enum RectTxfmSize` and `enum TxfmType` (`SW`,
/// `SH` are min(W, 32), min(H, 32)).
fn table_8bpc() -> [[ItxFn; 16]; 19] {
    [
        entries_8bpc::<4, 4, 4, 4>(),
        entries_8bpc::<8, 8, 8, 8>(),
        entries_8bpc::<16, 16, 16, 16>(),
        entries_8bpc::<32, 32, 32, 32>(),
        entries_8bpc::<64, 64, 32, 32>(),
        entries_8bpc::<4, 8, 4, 8>(),
        entries_8bpc::<8, 4, 8, 4>(),
        entries_8bpc::<8, 16, 8, 16>(),
        entries_8bpc::<16, 8, 16, 8>(),
        entries_8bpc::<16, 32, 16, 32>(),
        entries_8bpc::<32, 16, 32, 16>(),
        entries_8bpc::<32, 64, 32, 32>(),
        entries_8bpc::<64, 32, 32, 32>(),
        entries_8bpc::<4, 16, 4, 16>(),
        entries_8bpc::<16, 4, 16, 4>(),
        entries_8bpc::<8, 32, 8, 32>(),
        entries_8bpc::<32, 8, 32, 8>(),
        entries_8bpc::<16, 64, 16, 32>(),
        entries_8bpc::<64, 16, 32, 16>(),
    ]
}

fn table_16bpc() -> [[ItxFn16; 16]; 19] {
    [
        entries_16bpc::<4, 4, 4, 4>(),
        entries_16bpc::<8, 8, 8, 8>(),
        entries_16bpc::<16, 16, 16, 16>(),
        entries_16bpc::<32, 32, 32, 32>(),
        entries_16bpc::<64, 64, 32, 32>(),
        entries_16bpc::<4, 8, 4, 8>(),
        entries_16bpc::<8, 4, 8, 4>(),
        entries_16bpc::<8, 16, 8, 16>(),
        entries_16bpc::<16, 8, 16, 8>(),
        entries_16bpc::<16, 32, 16, 32>(),
        entries_16bpc::<32, 16, 32, 16>(),
        entries_16bpc::<32, 64, 32, 32>(),
        entries_16bpc::<64, 32, 32, 32>(),
        entries_16bpc::<4, 16, 4, 16>(),
        entries_16bpc::<16, 4, 16, 4>(),
        entries_16bpc::<8, 32, 8, 32>(),
        entries_16bpc::<32, 8, 32, 8>(),
        entries_16bpc::<16, 64, 16, 32>(),
        entries_16bpc::<64, 16, 32, 16>(),
    ]
}

/// The kernels of one pixel type (`F`: `ItxFn` or `ItxFn16`), for the init
/// mirrors to fill the slots where the original assigns its assembly.
pub struct Kernels<F> {
    table: [[F; 16]; 19],
    wht: F,
}

impl Kernels<ItxFn> {
    /// The 8 bpc kernels, if the CPU has the level (module documentation).
    pub fn new_8bpc() -> Option<Self> {
        if !available() {
            return None;
        }
        scan_extents();
        Some(Kernels { table: table_8bpc(), wht: wht_8bpc })
    }
}

impl Kernels<ItxFn16> {
    /// The 16 bpc kernels (10 and 12 bits), if the CPU has the level.
    pub fn new_16bpc() -> Option<Self> {
        if !available() {
            return None;
        }
        scan_extents();
        Some(Kernels { table: table_16bpc(), wht: wht_16bpc })
    }
}

impl<F: Copy> Kernels<F> {
    /// `assign_itx_fn( , 4, 4, wht_wht, WHT_WHT, …)`.
    pub fn assign_wht(&self, t: &mut [[Option<F>; 17]; 19]) {
        t[TX_4X4][WHT_WHT] = Some(self.wht);
    }

    /// `assign_itx<n>_fn` for the block size `tx` (`enum RectTxfmSize`):
    /// the slots `slots` (`ITX1` … `ITX16`, `IDENTITY`).
    pub fn assign(&self, t: &mut [[Option<F>; 17]; 19], tx: usize, slots: &[usize]) {
        for &tx_type in slots {
            t[tx][tx_type] = Some(self.table[tx][tx_type]);
        }
    }
}

#[cfg(test)]
mod tests;
