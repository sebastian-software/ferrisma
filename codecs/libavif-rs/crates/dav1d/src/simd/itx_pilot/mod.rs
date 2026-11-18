// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Experiment (branch `v1/dav1d-itx-pilot`, not for `main`): the inverse
//! transforms written from the algorithm with the portable vector types of
//! `fearless_simd` (variant 2 of the CDEF pilot), one implementation for
//! NEON and for x86-64 AVX2, at 8, 10 and 12 bits.
//!
//! - `transforms`: the 1-D transforms of `itx_1d.c` (DCT 4..64, ADST 4..16,
//!   flipped ADST, identity, WHT) on vectors, one row or column per lane;
//! - `blocks`: `inv_txfm_add_c` of `itx_tmpl.c` (two passes, the transpose
//!   between them, rounding and clipping as the C does, the C's DC-only path)
//!   and the 4x4 WHT;
//! - this module: the DSP-table boundary (the only raw-pointer code) and the
//!   switch.
//!
//! The numerics: the C computes in `int`; the vectors have 32-bit lanes and
//! the same expressions, so every value is the C's (including the C's
//! rearranged products, which keep 12-bit input free of overflow). Only the
//! final addition to the pixels runs in 16-bit lanes, where that is exact
//! (`blocks::Pixel`).
//!
//! So the pilot gives the C's pixels for every input. The original's
//! assembly gives them where the intermediates stay within the ranges of a
//! conformant stream (itx_1d.c: SIMD may overflow beyond them); with
//! coefficients over the whole `int16_t` range at 8 bpc about half of the
//! calls differ between the original's C and its NEON or AVX2 kernels, and
//! there the pilot follows the C, the shipped kernels the assembly.
//!
//! The switch: the module exists only when the crate is built with
//! `--cfg dav1d_itx_pilot`, so a normal build is the shipped port,
//! unchanged. In such a build `DAV1D_ITX_PILOT=v2` makes the shipped init
//! (`simd/arm/itx.rs`, `simd/x86/itx.rs`) hand every slot it has just given a
//! SIMD kernel to the pilot instead, so the slots stay the original's
//! selection: below NEON / AVX2, and where the original keeps C (12 bits on
//! ARM64, the large DCT_DCT slots at 12 bits on x86-64), the C kernels stay.

use core::ffi::c_int;
use std::sync::OnceLock;

mod blocks;
pub mod transforms;

use blocks::{Block, Depth, ScanExtent};

#[cfg(target_arch = "aarch64")]
type Level = fearless_simd::aarch64::Neon;
#[cfg(target_arch = "x86_64")]
type Level = fearless_simd::x86::Avx2;

/// Whether the CPU has the level the pilot is compiled for (on x86-64 the
/// library's `Avx2` needs x86-64-v3; without it the shipped kernels stay).
fn available() -> bool {
    #[cfg(target_arch = "aarch64")]
    return fearless_simd::Level::new().as_neon().is_some();
    #[cfg(target_arch = "x86_64")]
    return fearless_simd::Level::new().as_avx2().is_some();
}

/// The level's token.
///
/// # Safety
/// `available()` is true (the pilot is installed only then).
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

/// `WHT_WHT` (src/levels.h).
const WHT_WHT: usize = 16;

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
    let tx_type = const { blocks::kernel_type(SLOT) };
    let block = block::<_, _, W, H>(dst, stride, coeff, eob);
    blocks::inverse_transform_add::<_, _, _, W, H, SW, SH>(level(), block, tx_type, Depth::eight());
}

/// An `itxfm_fn` at 16 bpc.
unsafe extern "C" fn itx_16bpc<const W: usize, const H: usize, const SW: usize, const SH: usize, const SLOT: usize>(
    dst: *mut u16,
    stride: isize,
    coeff: *mut i32,
    eob: c_int,
    bitdepth_max: c_int,
) {
    let tx_type = const { blocks::kernel_type(SLOT) };
    let block = block::<_, _, W, H>(dst, stride, coeff, eob);
    blocks::inverse_transform_add::<_, _, _, W, H, SW, SH>(level(), block, tx_type, Depth::high(bitdepth_max));
}

/// The WHT `itxfm_fn` at 8 bpc.
unsafe extern "C" fn wht_8bpc(dst: *mut u8, stride: isize, coeff: *mut i16, _eob: c_int) {
    blocks::wht_add(level(), &mut rows::<_, 4, 4>(dst, stride), coefficients(coeff, 16), Depth::eight().pixel_max);
}

/// The WHT `itxfm_fn` at 16 bpc.
unsafe extern "C" fn wht_16bpc(dst: *mut u16, stride: isize, coeff: *mut i32, _eob: c_int, bitdepth_max: c_int) {
    blocks::wht_add(level(), &mut rows::<_, 4, 4>(dst, stride), coefficients(coeff, 16), bitdepth_max);
}

// --- the switch ---------------------------------------------------------------

/// Whether `DAV1D_ITX_PILOT` selects the pilot.
fn selected() -> bool {
    match std::env::var("DAV1D_ITX_PILOT").ok().as_deref() {
        None => false,
        Some("v2") => true,
        Some(other) => panic!("DAV1D_ITX_PILOT={other}: expected v2"),
    }
}

/// The pilot's entries of one block size, by `enum TxfmType` (WHT apart).
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

/// The pilot's entries, by `enum RectTxfmSize` and `enum TxfmType` (`SW`,
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

/// Replaces each slot of `table` that differs from `before` (the slots the
/// shipped init has just given a SIMD kernel) by the pilot's kernel.
/// `address` compares the function pointers.
fn replace_changed<F: Copy>(
    table: &mut [[Option<F>; 17]; 19],
    before: &[[Option<F>; 17]; 19],
    address: fn(F) -> usize,
    pilot: impl Fn(usize, usize) -> F,
) {
    for (tx, (row, row_before)) in table.iter_mut().zip(before).enumerate() {
        for (tx_type, (slot, slot_before)) in row.iter_mut().zip(row_before).enumerate() {
            if slot.map(address) != slot_before.map(address) {
                *slot = Some(pilot(tx, tx_type));
            }
        }
    }
}

/// Called by the shipped `itx_dsp_init_*_8bpc` after it has assigned its
/// SIMD kernels, with the table as it was before: hands those slots to the
/// pilot when `DAV1D_ITX_PILOT=v2`.
pub fn install_8bpc(table: &mut [[Option<ItxFn>; 17]; 19], before: &[[Option<ItxFn>; 17]; 19]) {
    if !selected() || !available() {
        return;
    }
    scan_extents();
    let pilot = table_8bpc();
    replace_changed(table, before, |f| f as usize, |tx, tx_type| if tx_type == WHT_WHT { wht_8bpc } else { pilot[tx][tx_type] });
}

/// As `install_8bpc`, at 16 bpc.
pub fn install_16bpc(table: &mut [[Option<ItxFn16>; 17]; 19], before: &[[Option<ItxFn16>; 17]; 19]) {
    if !selected() || !available() {
        return;
    }
    scan_extents();
    let pilot = table_16bpc();
    replace_changed(table, before, |f| f as usize, |tx, tx_type| if tx_type == WHT_WHT { wht_16bpc } else { pilot[tx][tx_type] });
}

#[cfg(test)]
mod tests;
