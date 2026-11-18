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
    best_direction, round_sum, DirBlock, Filtered, Padded, Params, Variant, ALT_WEIGHTS,
    DIAGONAL_WEIGHTS, LINE_WEIGHT,
};


pub struct AutoVec;

impl Variant for AutoVec {
    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
    unsafe fn filter<const W: usize, const H: usize>(input: &Padded, params: &Params) -> Filtered {
        // 16 lanes per step: two rows of an 8-wide block, four of a 4-wide one.
        by_strengths::<W, H, 16>(input, params)
    }

    #[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
    unsafe fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
        find_dir(block)
    }
}

#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn by_strengths<const W: usize, const H: usize, const L: usize>(input: &Padded, params: &Params) -> Filtered {
    match (params.pri_strength != 0, params.sec_strength != 0) {
        (true, true) => filter::<W, H, L, true, true>(input, params),
        (true, false) => filter::<W, H, L, true, false>(input, params),
        _ => filter::<W, H, L, false, true>(input, params),
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
fn add_tap<const L: usize>(sum: &mut [i16; L], px: &[i16; L], p: &[i16; L], strength: u16, shift: u32, tap: i16) {
    for ((s, &px), &p) in sum.iter_mut().zip(px).zip(p) {
        *s += tap * constrain(p, px, strength, shift);
    }
}

/// Widens `[min, max]` by the neighbors `p`; the padding marker never
/// counts (unsigned minimum, signed maximum, as the C's `umin`/`imax`).
#[inline]
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn add_range<const L: usize>(min: &mut [i16; L], max: &mut [i16; L], p: &[i16; L]) {
    for ((lo, hi), &p) in min.iter_mut().zip(max.iter_mut()).zip(p) {
        *lo = (*lo as u16).min(p as u16) as i16;
        *hi = (*hi).max(p);
    }
}

/// `cdef_filter_block_c`, for the strengths that are not zero.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn filter<const W: usize, const H: usize, const L: usize, const PRI: bool, const SEC: bool>(
    input: &Padded,
    params: &Params,
) -> Filtered {
    let mut out = [[0; 8]; 8];
    for y in (0..H).step_by(L / W) {
        let result = filter_rows::<W, L, PRI, SEC>(input, params, y);
        for (row, values) in out[y..].iter_mut().zip(result.chunks_exact(W)) {
            row[..W].copy_from_slice(values);
        }
    }
    out
}

/// The filtered pixels of rows `y..y + L / W`.
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn filter_rows<const W: usize, const L: usize, const PRI: bool, const SEC: bool>(
    input: &Padded,
    params: &Params,
    y: usize,
) -> [i16; L] {
    let px = input.lanes::<W, L>(y, 0);
    let mut sum = [0; L];
    let (mut min, mut max) = (px, px);
    for k in 0..2 {
        if PRI {
            let off = params.primary(k);
            for off in [off, -off] {
                let p = input.lanes::<W, L>(y, off);
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
                for off in [off, -off] {
                    let s = input.lanes::<W, L>(y, off);
                    add_tap(&mut sum, &px, &s, params.sec_strength, params.sec_shift, tap);
                    if PRI {
                        add_range(&mut min, &mut max, &s);
                    }
                }
            }
        }
    }
    let mut result = [0; L];
    for (i, r) in result.iter_mut().enumerate() {
        let filtered = px[i] + round_sum(sum[i]);
        // iclip(): never min > max here, as px is in both.
        *r = if PRI && SEC { filtered.max(min[i]).min(max[i]) } else { filtered };
    }
    result
}

/// `cdef_find_dir_c`: the partial sums along the eight directions, then
/// their weighted squares. Each row adds to a run of consecutive entries of
/// every partial-sum line (the C's index expressions, read per row).
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "avx2"))]
fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
    let mut hv = [[0i16; 8]; 2];
    let mut diag = [[0i16; 15]; 2];
    let mut alt = [[0i16; 11]; 4];
    for (y, row) in block.iter().enumerate() {
        let reversed: [i16; 8] = core::array::from_fn(|x| row[7 - x]);
        let pairs: [i16; 4] = core::array::from_fn(|j| row[2 * j] + row[2 * j + 1]);
        let reversed_pairs: [i16; 4] = core::array::from_fn(|j| pairs[3 - j]);
        add(&mut diag[0][y..y + 8], row); // diag[0][y + x]
        add(&mut diag[1][y..y + 8], &reversed); // diag[1][7 + y - x]
        add(&mut alt[0][y..y + 4], &pairs); // alt[0][y + x / 2]
        add(&mut alt[1][y..y + 4], &reversed_pairs); // alt[1][3 + y - x / 2]
        add(&mut alt[2][3 - y / 2..11 - y / 2], row); // alt[2][3 - y / 2 + x]
        add(&mut alt[3][y / 2..8 + y / 2], row); // alt[3][y / 2 + x]
        hv[0][y] = row.iter().sum(); // hv[0][y]
        add(&mut hv[1], row); // hv[1][x]
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

#[inline]
fn add(sums: &mut [i16], values: &[i16]) {
    for (s, &v) in sums.iter_mut().zip(values) {
        *s += v;
    }
}

/// The sum of the weighted squares of partial sums.
#[inline]
fn weighted<const N: usize>(sums: &[i16; N], weights: [u32; N]) -> u32 {
    sums.iter().zip(weights).map(|(&s, w)| (s as i32 * s as i32) as u32 * w).sum()
}
