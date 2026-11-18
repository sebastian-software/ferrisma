// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Variant 1: `cdef_tmpl.c` as plain Rust on fixed-size arrays, left to
//! LLVM's vectorizers. No intrinsics and no `unsafe`.
//!
//! The filter works on `LANES` pixels at once (one AVX2 register of 16-bit
//! lanes, two NEON registers): two rows of an 8-wide block, four rows of a
//! 4-wide one. Every step is a lane-wise loop over such an array, which is
//! the shape the loop and SLP vectorizers turn into vector instructions.
//! On x86-64 the functions are compiled for AVX2 (`target_feature`); on
//! AArch64 NEON is the baseline and nothing is needed.

use core::ffi::{c_int, c_uint};

use super::{
    best_direction, opposite, round_sum, DirBlock, Filtered, Padded, Params, Variant, ALT_WEIGHTS,
    DIAGONAL_WEIGHTS, LINE_WEIGHT,
};

const LANES: usize = 16;
type Lanes = [i16; LANES];

pub struct AutoVec;

impl Variant for AutoVec {
    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
    unsafe fn filter<const W: usize, const H: usize>(input: &Padded, params: &Params) -> Filtered {
        match (params.pri_strength != 0, params.sec_strength != 0) {
            (true, true) => filter::<W, H, true, true>(input, params),
            (true, false) => filter::<W, H, true, false>(input, params),
            _ => filter::<W, H, false, true>(input, params),
        }
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
    unsafe fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
        find_dir(block)
    }
}

/// `constrain` of cdef_tmpl.c for the tap value `p` around the pixel `px`.
#[inline]
fn constrain(p: i16, px: i16, strength: u16, shift: u32) -> i16 {
    let adiff = p.abs_diff(px);
    let c = adiff.min(strength.saturating_sub(adiff >> shift)) as i16;
    if p < px {
        -c
    } else {
        c
    }
}

/// Adds `tap * constrain(..)` of the neighbors `p` to `sum`.
#[inline]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn add_tap(sum: &mut Lanes, px: &Lanes, p: &Lanes, strength: u16, shift: u32, tap: i16) {
    for ((s, &px), &p) in sum.iter_mut().zip(px).zip(p) {
        *s += tap * constrain(p, px, strength, shift);
    }
}

/// Widens `[min, max]` by the neighbors `p`; the padding marker never
/// counts (unsigned minimum, signed maximum, as the C's `umin`/`imax`).
#[inline]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn add_range(min: &mut Lanes, max: &mut Lanes, p: &Lanes) {
    for ((lo, hi), &p) in min.iter_mut().zip(max.iter_mut()).zip(p) {
        *lo = (*lo as u16).min(p as u16) as i16;
        *hi = (*hi).max(p);
    }
}

/// `cdef_filter_block_c`, for the strengths that are not zero.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn filter<const W: usize, const H: usize, const PRI: bool, const SEC: bool>(
    input: &Padded,
    params: &Params,
) -> Filtered {
    let rows_per_step = LANES / W;
    let mut out = [[0; 8]; 8];
    for y in (0..H).step_by(rows_per_step) {
        let px: Lanes = input.lanes::<W, LANES>(y, (0, 0));
        let mut sum = [0; LANES];
        let (mut min, mut max) = (px, px);
        for k in 0..2 {
            if PRI {
                let off = params.primary(k);
                for off in [off, opposite(off)] {
                    let p = input.lanes::<W, LANES>(y, off);
                    add_tap(&mut sum, &px, &p, params.pri_strength, params.pri_shift, params.pri_taps[k]);
                    if SEC {
                        add_range(&mut min, &mut max, &p);
                    }
                }
            }
            if SEC {
                // "sec_tap starts at 2 and becomes 1"
                let tap = 2 - k as i16;
                for off in params.secondary(k) {
                    for off in [off, opposite(off)] {
                        let s = input.lanes::<W, LANES>(y, off);
                        add_tap(&mut sum, &px, &s, params.sec_strength, params.sec_shift, tap);
                        if PRI {
                            add_range(&mut min, &mut max, &s);
                        }
                    }
                }
            }
        }
        let mut result = [0; LANES];
        for (i, r) in result.iter_mut().enumerate() {
            let filtered = px[i] + round_sum(sum[i]);
            *r = if PRI && SEC { filtered.clamp(min[i], max[i]) } else { filtered };
        }
        for (row, values) in out[y..].iter_mut().zip(result.chunks_exact(W)) {
            row[..W].copy_from_slice(values);
        }
    }
    out
}

/// `cdef_find_dir_c`: the partial sums along the eight directions, then
/// their weighted squares.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
    let mut hv = [[0i32; 8]; 2];
    let mut diag = [[0i32; 15]; 2];
    let mut alt = [[0i32; 11]; 4];
    for (y, row) in block.iter().enumerate() {
        for (x, &px) in row.iter().enumerate() {
            let px = px as i32;
            diag[0][y + x] += px;
            alt[0][y + (x >> 1)] += px;
            hv[0][y] += px;
            alt[1][3 + y - (x >> 1)] += px;
            diag[1][7 + y - x] += px;
            alt[2][3 - (y >> 1) + x] += px;
            hv[1][x] += px;
            alt[3][(y >> 1) + x] += px;
        }
    }
    let cost = [
        weighted(&diag[0], DIAGONAL_WEIGHTS),
        weighted(&alt[0], ALT_WEIGHTS),
        weighted(&hv[0], [LINE_WEIGHT; 8]),
        weighted(&alt[1], ALT_WEIGHTS),
        weighted(&diag[1], DIAGONAL_WEIGHTS),
        weighted(&alt[2], ALT_WEIGHTS),
        weighted(&hv[1], [LINE_WEIGHT; 8]),
        weighted(&alt[3], ALT_WEIGHTS),
    ];
    best_direction(cost)
}

/// The sum of the weighted squares of partial sums.
#[inline]
fn weighted<const N: usize>(sums: &[i32; N], weights: [u32; N]) -> u32 {
    sums.iter().zip(weights).map(|(&s, w)| (s * s) as u32 * w).sum()
}
