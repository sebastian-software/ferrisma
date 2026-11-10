/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/x86/cdef_block_avx2.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -mavx2.
//
// The C file starts with
//
//   #include "aom_dsp/aom_simd.h"
//   #define SIMD_FUNC(name) name##_avx2
//   #include "av1/common/cdef_block_simd.h"
//
// cdef_block_simd.h is a template: it is written against libaom's portable
// SIMD abstraction (v64/v128/v256 in aom_dsp/simd) and every file that
// includes it instantiates it for its own ISA, naming the exported functions
// through SIMD_FUNC. Its AVX2 instantiation therefore belongs to this file and
// is translated here first, with SIMD_FUNC(name) spelled out as name_avx2 and
// the abstraction calls resolved to the translations in
// crate::simd::aom_dsp__simd__v{64,128,256}_intrinsics_x86 (whose -mavx2
// branches are documented there). The functions that cdef_block_avx2.c itself
// defines follow in the second half. Neither half calls any SSE4.1 function
// from cdef_block_sse4.c.
//
// The header's static helpers carry #[target_feature(enable = "avx2")] like
// the rest of the file. CDEF_INLINE and SIMD_INLINE (static AOM_FORCE_INLINE
// outside 32-bit MSVC) become `#[inline]`. C `static` / `static inline`
// functions are private. The scalar helpers get_msb() (aom_ports/bitops.h),
// AOMMAX (aom_dsp/aom_dsp_common.h) and the cdef_block.h constants are
// translated privately below, as in cdef_block_neon.rs.
//
// The C original reads the four-byte source words of
// cdef_copy_rect8_8bit_to_16bit_avx2's 4-pixel loop through a plain
// `const int32_t *`; x86 scalar loads have no alignment requirement, so the
// translation uses read_unaligned(), which emits the same load.
use core::arch::x86_64::*;
use core::ffi::c_void;
use core::ptr::addr_of;

use crate::av1__common__cdef_block::{cdef_directions, cdef_pri_taps, cdef_sec_taps};
use crate::simd::aom_dsp__simd__v128_intrinsics_x86::*;
use crate::simd::aom_dsp__simd__v256_intrinsics_x86::*;
use crate::simd::aom_dsp__simd__v64_intrinsics_x86::*;

// av1/common/cdef_block.h (MAX_SB_SIZE_LOG2 = 7 from av1/common/enums.h):
// CDEF_BSTRIDE = ALIGN_POWER_OF_TWO((1 << MAX_SB_SIZE_LOG2) + 2 * CDEF_HBORDER, 3)
const CDEF_BSTRIDE: i32 = 144;
// av1/common/cdef_block.h
const CDEF_VERY_LARGE: i32 = 0x4000;

// aom_ports/bitops.h get_msb(): the index of the most significant set bit.
#[inline]
fn get_msb(n: u32) -> i32 {
    debug_assert!(n != 0);
    (31 ^ n.leading_zeros()) as i32
}

// AOMMAX(a, b) from aom_dsp/aom_dsp_common.h
#[inline]
fn aommax(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// `cdef_directions[i][j]` from av1/common/cdef_block.c (a pointer two
/// entries into cdef_directions_padded, so `i` may range over -2..=9).
#[inline]
unsafe fn cdef_dir(i: i32, j: usize) -> i32 {
    (*cdef_directions.offset(i as isize))[j]
}

#[inline]
unsafe fn cdef_pri_taps_row(i: i32) -> *const i32 {
    (*addr_of!(cdef_pri_taps))[i as usize].as_ptr()
}

#[inline]
unsafe fn cdef_sec_taps_ptr() -> *const i32 {
    (*addr_of!(cdef_sec_taps)).as_ptr()
}

// ---------------------------------------------------------------------------
// av1/common/cdef_block_simd.h, instantiated with SIMD_FUNC(name) name##_avx2
// ---------------------------------------------------------------------------

/* partial A is a 16-bit vector of the form:
   [x8 x7 x6 x5 x4 x3 x2 x1] and partial B has the form:
   [0  y1 y2 y3 y4 y5 y6 y7].
   This function computes (x1^2+y1^2)*C1 + (x2^2+y2^2)*C2 + ...
   (x7^2+y2^7)*C7 + (x8^2+0^2)*C8 where the C1..C8 constants are in const1
   and const2. */
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fold_mul_and_sum(mut partiala: v128, mut partialb: v128, const1: v128,
                           const2: v128) -> v128 {
    let tmp: v128;
    /* Reverse partial B. */
    partialb = v128_shuffle_8(partialb,
                              v128_from_32(0x0f0e0100, 0x03020504, 0x07060908, 0x0b0a0d0c));
    /* Interleave the x and y values of identical indices and pair x8 with 0. */
    tmp = partiala;
    partiala = v128_ziplo_16(partialb, partiala);
    partialb = v128_ziphi_16(partialb, tmp);
    /* Square and add the corresponding x and y values. */
    partiala = v128_madd_s16(partiala, partiala);
    partialb = v128_madd_s16(partialb, partialb);
    /* Multiply by constant. */
    partiala = v128_mullo_s32(partiala, const1);
    partialb = v128_mullo_s32(partialb, const2);
    /* Sum all results. */
    partiala = v128_add_32(partiala, partialb);
    partiala
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn hsum4(mut x0: v128, mut x1: v128, mut x2: v128, mut x3: v128) -> v128 {
    let t0 = v128_ziplo_32(x1, x0);
    let t1 = v128_ziplo_32(x3, x2);
    let t2 = v128_ziphi_32(x1, x0);
    let t3 = v128_ziphi_32(x3, x2);
    x0 = v128_ziplo_64(t1, t0);
    x1 = v128_ziphi_64(t1, t0);
    x2 = v128_ziplo_64(t3, t2);
    x3 = v128_ziphi_64(t3, t2);
    v128_add_32(v128_add_32(x0, x1), v128_add_32(x2, x3))
}

/* Computes cost for directions 0, 5, 6 and 7. We can call this function again
   to compute the remaining directions. */
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn compute_directions(lines: *const v128, tmp_cost1: *mut i32) -> v128 {
    let lines = core::slice::from_raw_parts(lines, 8);
    let mut partial4a: v128;
    let mut partial4b: v128;
    let mut partial5a: v128;
    let mut partial5b: v128;
    let mut partial7a: v128;
    let mut partial7b: v128;
    let mut partial6: v128;
    let mut tmp: v128;
    /* Partial sums for lines 0 and 1. */
    partial4a = v128_shl_n_byte::<14>(lines[0]);
    partial4b = v128_shr_n_byte::<2>(lines[0]);
    partial4a = v128_add_16(partial4a, v128_shl_n_byte::<12>(lines[1]));
    partial4b = v128_add_16(partial4b, v128_shr_n_byte::<4>(lines[1]));
    tmp = v128_add_16(lines[0], lines[1]);
    partial5a = v128_shl_n_byte::<10>(tmp);
    partial5b = v128_shr_n_byte::<6>(tmp);
    partial7a = v128_shl_n_byte::<4>(tmp);
    partial7b = v128_shr_n_byte::<12>(tmp);
    partial6 = tmp;

    /* Partial sums for lines 2 and 3. */
    partial4a = v128_add_16(partial4a, v128_shl_n_byte::<10>(lines[2]));
    partial4b = v128_add_16(partial4b, v128_shr_n_byte::<6>(lines[2]));
    partial4a = v128_add_16(partial4a, v128_shl_n_byte::<8>(lines[3]));
    partial4b = v128_add_16(partial4b, v128_shr_n_byte::<8>(lines[3]));
    tmp = v128_add_16(lines[2], lines[3]);
    partial5a = v128_add_16(partial5a, v128_shl_n_byte::<8>(tmp));
    partial5b = v128_add_16(partial5b, v128_shr_n_byte::<8>(tmp));
    partial7a = v128_add_16(partial7a, v128_shl_n_byte::<6>(tmp));
    partial7b = v128_add_16(partial7b, v128_shr_n_byte::<10>(tmp));
    partial6 = v128_add_16(partial6, tmp);

    /* Partial sums for lines 4 and 5. */
    partial4a = v128_add_16(partial4a, v128_shl_n_byte::<6>(lines[4]));
    partial4b = v128_add_16(partial4b, v128_shr_n_byte::<10>(lines[4]));
    partial4a = v128_add_16(partial4a, v128_shl_n_byte::<4>(lines[5]));
    partial4b = v128_add_16(partial4b, v128_shr_n_byte::<12>(lines[5]));
    tmp = v128_add_16(lines[4], lines[5]);
    partial5a = v128_add_16(partial5a, v128_shl_n_byte::<6>(tmp));
    partial5b = v128_add_16(partial5b, v128_shr_n_byte::<10>(tmp));
    partial7a = v128_add_16(partial7a, v128_shl_n_byte::<8>(tmp));
    partial7b = v128_add_16(partial7b, v128_shr_n_byte::<8>(tmp));
    partial6 = v128_add_16(partial6, tmp);

    /* Partial sums for lines 6 and 7. */
    partial4a = v128_add_16(partial4a, v128_shl_n_byte::<2>(lines[6]));
    partial4b = v128_add_16(partial4b, v128_shr_n_byte::<14>(lines[6]));
    partial4a = v128_add_16(partial4a, lines[7]);
    tmp = v128_add_16(lines[6], lines[7]);
    partial5a = v128_add_16(partial5a, v128_shl_n_byte::<4>(tmp));
    partial5b = v128_add_16(partial5b, v128_shr_n_byte::<12>(tmp));
    partial7a = v128_add_16(partial7a, v128_shl_n_byte::<10>(tmp));
    partial7b = v128_add_16(partial7b, v128_shr_n_byte::<6>(tmp));
    partial6 = v128_add_16(partial6, tmp);

    /* Compute costs in terms of partial sums. */
    partial4a = fold_mul_and_sum(partial4a, partial4b, v128_from_32(210, 280, 420, 840),
                                 v128_from_32(105, 120, 140, 168));
    partial7a = fold_mul_and_sum(partial7a, partial7b, v128_from_32(210, 420, 0, 0),
                                 v128_from_32(105, 105, 105, 140));
    partial5a = fold_mul_and_sum(partial5a, partial5b, v128_from_32(210, 420, 0, 0),
                                 v128_from_32(105, 105, 105, 140));
    partial6 = v128_madd_s16(partial6, partial6);
    partial6 = v128_mullo_s32(partial6, v128_dup_32(105));

    partial4a = hsum4(partial4a, partial5a, partial6, partial7a);
    v128_store_unaligned(tmp_cost1.cast(), partial4a);
    partial4a
}

/* transpose and reverse the order of the lines -- equivalent to a 90-degree
   counter-clockwise rotation of the pixels. */
// The caller passes the same array as `in` and `res`; every input is read
// before the first result is written, as in the C.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn array_reverse_transpose_8x8(in_: *const v128, res: *mut v128) {
    let tr0_0 = v128_ziplo_16(*in_.add(1), *in_.add(0));
    let tr0_1 = v128_ziplo_16(*in_.add(3), *in_.add(2));
    let tr0_2 = v128_ziphi_16(*in_.add(1), *in_.add(0));
    let tr0_3 = v128_ziphi_16(*in_.add(3), *in_.add(2));
    let tr0_4 = v128_ziplo_16(*in_.add(5), *in_.add(4));
    let tr0_5 = v128_ziplo_16(*in_.add(7), *in_.add(6));
    let tr0_6 = v128_ziphi_16(*in_.add(5), *in_.add(4));
    let tr0_7 = v128_ziphi_16(*in_.add(7), *in_.add(6));

    let tr1_0 = v128_ziplo_32(tr0_1, tr0_0);
    let tr1_1 = v128_ziplo_32(tr0_5, tr0_4);
    let tr1_2 = v128_ziphi_32(tr0_1, tr0_0);
    let tr1_3 = v128_ziphi_32(tr0_5, tr0_4);
    let tr1_4 = v128_ziplo_32(tr0_3, tr0_2);
    let tr1_5 = v128_ziplo_32(tr0_7, tr0_6);
    let tr1_6 = v128_ziphi_32(tr0_3, tr0_2);
    let tr1_7 = v128_ziphi_32(tr0_7, tr0_6);

    *res.add(7) = v128_ziplo_64(tr1_1, tr1_0);
    *res.add(6) = v128_ziphi_64(tr1_1, tr1_0);
    *res.add(5) = v128_ziplo_64(tr1_3, tr1_2);
    *res.add(4) = v128_ziphi_64(tr1_3, tr1_2);
    *res.add(3) = v128_ziplo_64(tr1_5, tr1_4);
    *res.add(2) = v128_ziphi_64(tr1_5, tr1_4);
    *res.add(1) = v128_ziplo_64(tr1_7, tr1_6);
    *res.add(0) = v128_ziphi_64(tr1_7, tr1_6);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_find_dir_avx2(img: *const u16, stride: i32, var: *mut i32,
                                                   coeff_shift: i32) -> i32 {
    let mut cost: [i32; 8] = [0; 8];
    let best_cost: i32;
    let mut best_dir: i32;
    let mut lines: [v128; 8] = core::mem::zeroed();
    for i in 0..8 {
        lines[i as usize] = v128_load_unaligned(img.offset((i * stride) as isize).cast());
        lines[i as usize] =
            v128_sub_16(v128_shr_s16(lines[i as usize], coeff_shift as u32), v128_dup_16(128));
    }

    /* Compute "mostly vertical" directions. */
    let dir47 = compute_directions(lines.as_ptr(), cost.as_mut_ptr().add(4));

    let lines_ptr = lines.as_mut_ptr();
    array_reverse_transpose_8x8(lines_ptr, lines_ptr);

    /* Compute "mostly horizontal" directions. */
    let dir03 = compute_directions(lines.as_ptr(), cost.as_mut_ptr());

    let mut max = v128_max_s32(dir03, dir47);
    max = v128_max_s32(max, v128_align::<8>(max, max));
    max = v128_max_s32(max, v128_align::<4>(max, max));
    best_cost = v128_low_u32(max) as i32;
    let t = v128_pack_s32_s16(v128_cmpeq_32(max, dir47), v128_cmpeq_32(max, dir03));
    best_dir = v128_movemask_8(v128_pack_s16_s8(t, t)) as i32;
    best_dir = get_msb((best_dir ^ (best_dir - 1)) as u32); // Count trailing zeros

    /* Difference between the optimal variance and the variance along the
       orthogonal direction. Again, the sum(x^2) terms cancel out. */
    *var = best_cost.wrapping_sub(cost[((best_dir + 4) & 7) as usize]);
    /* We'd normally divide by 840, but dividing by 1024 is close enough
       for what we're going to do with this. */
    *var >>= 10;
    best_dir
}

// sign(a-b) * min(abs(a-b), max(0, threshold - (abs(a-b) >> adjdamp)))
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn constrain16(a: v256, b: v256, threshold: u32, adjdamp: u32) -> v256 {
    let mut diff = v256_sub_16(a, b);
    let sign = v256_shr_n_s16::<15>(diff);
    diff = v256_abs_s16(diff);
    let s = v256_ssub_u16(v256_dup_16(threshold as u16), v256_shr_u16(diff, adjdamp));
    v256_xor(v256_add_16(sign, v256_min_s16(diff, s)), sign)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_max_primary(is_lowbd: i32, tap: *const v256, mut max: v256,
                          cdef_large_value_mask: v256) -> v256 {
    if is_lowbd != 0 {
        let mut max_u8: v256;
        max_u8 = *tap.add(0);
        max_u8 = v256_max_u8(max_u8, *tap.add(1));
        max_u8 = v256_max_u8(max_u8, *tap.add(2));
        max_u8 = v256_max_u8(max_u8, *tap.add(3));
        /* The source is 16 bits, however, we only really care about the lower
        8 bits.  The upper 8 bits contain the "large" flag.  After the final
        primary max has been calculated, zero out the upper 8 bits.  Use this
        to find the "16 bit" max. */
        max = v256_max_s16(max, v256_and(max_u8, cdef_large_value_mask));
    } else {
        /* Convert CDEF_VERY_LARGE to 0 before calculating max. */
        max = v256_max_s16(max, v256_and(*tap.add(0), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(1), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(2), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(3), cdef_large_value_mask));
    }
    max
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_max_secondary(is_lowbd: i32, tap: *const v256, mut max: v256,
                            cdef_large_value_mask: v256) -> v256 {
    if is_lowbd != 0 {
        let mut max_u8: v256;
        max_u8 = *tap.add(0);
        max_u8 = v256_max_u8(max_u8, *tap.add(1));
        max_u8 = v256_max_u8(max_u8, *tap.add(2));
        max_u8 = v256_max_u8(max_u8, *tap.add(3));
        max_u8 = v256_max_u8(max_u8, *tap.add(4));
        max_u8 = v256_max_u8(max_u8, *tap.add(5));
        max_u8 = v256_max_u8(max_u8, *tap.add(6));
        max_u8 = v256_max_u8(max_u8, *tap.add(7));
        /* The source is 16 bits, however, we only really care about the lower
        8 bits.  The upper 8 bits contain the "large" flag.  After the final
        primary max has been calculated, zero out the upper 8 bits.  Use this
        to find the "16 bit" max. */
        max = v256_max_s16(max, v256_and(max_u8, cdef_large_value_mask));
    } else {
        /* Convert CDEF_VERY_LARGE to 0 before calculating max. */
        max = v256_max_s16(max, v256_and(*tap.add(0), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(1), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(2), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(3), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(4), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(5), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(6), cdef_large_value_mask));
        max = v256_max_s16(max, v256_and(*tap.add(7), cdef_large_value_mask));
    }
    max
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn filter_block_4x4(is_lowbd: i32, dest: *mut c_void, dstride: i32, in_: *const u16,
                           pri_strength: i32, sec_strength: i32, dir: i32,
                           mut pri_damping: i32, mut sec_damping: i32, coeff_shift: i32,
                           height: i32, enable_primary: i32, enable_secondary: i32) {
    let dst8 = dest as *mut u8;
    let dst16 = dest as *mut u16;
    let clipping_required = enable_primary != 0 && enable_secondary != 0;
    let mut p0: v256;
    let mut p1: v256;
    let mut p2: v256;
    let mut p3: v256;
    let mut sum: v256;
    let mut row: v256;
    let mut res: v256;
    let mut max: v256;
    let mut min: v256;
    let cdef_large_value_mask = v256_dup_16((!CDEF_VERY_LARGE) as u16);
    let po1 = cdef_dir(dir, 0);
    let po2 = cdef_dir(dir, 1);
    let s1o1 = cdef_dir(dir + 2, 0);
    let s1o2 = cdef_dir(dir + 2, 1);
    let s2o1 = cdef_dir(dir - 2, 0);
    let s2o2 = cdef_dir(dir - 2, 1);
    let pri_taps = cdef_pri_taps_row((pri_strength >> coeff_shift) & 1);
    let sec_taps = cdef_sec_taps_ptr();
    let mut i: i32;

    if enable_primary != 0 && pri_strength != 0 {
        pri_damping = aommax(0, pri_damping - get_msb(pri_strength as u32));
    }
    if enable_secondary != 0 && sec_strength != 0 {
        sec_damping = aommax(0, sec_damping - get_msb(sec_strength as u32));
    }

    i = 0;
    while i < height {
        sum = v256_zero();
        row = v256_from_v64(v64_load_aligned(in_.offset(((i + 0) * CDEF_BSTRIDE) as isize).cast()),
                            v64_load_aligned(in_.offset(((i + 1) * CDEF_BSTRIDE) as isize).cast()),
                            v64_load_aligned(in_.offset(((i + 2) * CDEF_BSTRIDE) as isize).cast()),
                            v64_load_aligned(in_.offset(((i + 3) * CDEF_BSTRIDE) as isize).cast()));
        min = row;
        max = min;

        if enable_primary != 0 {
            let mut tap: [v256; 4] = core::mem::zeroed();
            // Primary near taps
            tap[0] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE + po1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + po1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE + po1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE + po1) as isize).cast()));
            p0 = constrain16(tap[0], row, pri_strength as u32, pri_damping as u32);
            tap[1] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE - po1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - po1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE - po1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE - po1) as isize).cast()));
            p1 = constrain16(tap[1], row, pri_strength as u32, pri_damping as u32);

            // sum += pri_taps[0] * (p0 + p1)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*pri_taps.add(0) as u16),
                                                  v256_add_16(p0, p1)));

            // Primary far taps
            tap[2] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE + po2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + po2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE + po2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE + po2) as isize).cast()));
            p0 = constrain16(tap[2], row, pri_strength as u32, pri_damping as u32);
            tap[3] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE - po2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - po2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE - po2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE - po2) as isize).cast()));
            p1 = constrain16(tap[3], row, pri_strength as u32, pri_damping as u32);

            // sum += pri_taps[1] * (p0 + p1)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*pri_taps.add(1) as u16),
                                                  v256_add_16(p0, p1)));
            if clipping_required {
                max = get_max_primary(is_lowbd, tap.as_ptr(), max, cdef_large_value_mask);

                min = v256_min_s16(min, tap[0]);
                min = v256_min_s16(min, tap[1]);
                min = v256_min_s16(min, tap[2]);
                min = v256_min_s16(min, tap[3]);
            }
        }

        if enable_secondary != 0 {
            let mut tap: [v256; 8] = core::mem::zeroed();
            // Secondary near taps
            tap[0] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE + s1o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s1o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE + s1o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE + s1o1) as isize).cast()));
            p0 = constrain16(tap[0], row, sec_strength as u32, sec_damping as u32);
            tap[1] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE - s1o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s1o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE - s1o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE - s1o1) as isize).cast()));
            p1 = constrain16(tap[1], row, sec_strength as u32, sec_damping as u32);
            tap[2] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE + s2o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s2o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE + s2o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE + s2o1) as isize).cast()));
            p2 = constrain16(tap[2], row, sec_strength as u32, sec_damping as u32);
            tap[3] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE - s2o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s2o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE - s2o1) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE - s2o1) as isize).cast()));
            p3 = constrain16(tap[3], row, sec_strength as u32, sec_damping as u32);

            // sum += sec_taps[0] * (p0 + p1 + p2 + p3)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*sec_taps.add(0) as u16),
                                                  v256_add_16(v256_add_16(p0, p1),
                                                              v256_add_16(p2, p3))));

            // Secondary far taps
            tap[4] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE + s1o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s1o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE + s1o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE + s1o2) as isize).cast()));
            p0 = constrain16(tap[4], row, sec_strength as u32, sec_damping as u32);
            tap[5] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE - s1o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s1o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE - s1o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE - s1o2) as isize).cast()));
            p1 = constrain16(tap[5], row, sec_strength as u32, sec_damping as u32);
            tap[6] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE + s2o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s2o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE + s2o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE + s2o2) as isize).cast()));
            p2 = constrain16(tap[6], row, sec_strength as u32, sec_damping as u32);
            tap[7] = v256_from_v64(
                v64_load_unaligned(in_.offset(((i + 0) * CDEF_BSTRIDE - s2o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s2o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 2) * CDEF_BSTRIDE - s2o2) as isize).cast()),
                v64_load_unaligned(in_.offset(((i + 3) * CDEF_BSTRIDE - s2o2) as isize).cast()));
            p3 = constrain16(tap[7], row, sec_strength as u32, sec_damping as u32);

            // sum += sec_taps[1] * (p0 + p1 + p2 + p3)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*sec_taps.add(1) as u16),
                                                  v256_add_16(v256_add_16(p0, p1),
                                                              v256_add_16(p2, p3))));

            if clipping_required {
                max = get_max_secondary(is_lowbd, tap.as_ptr(), max, cdef_large_value_mask);

                min = v256_min_s16(min, tap[0]);
                min = v256_min_s16(min, tap[1]);
                min = v256_min_s16(min, tap[2]);
                min = v256_min_s16(min, tap[3]);
                min = v256_min_s16(min, tap[4]);
                min = v256_min_s16(min, tap[5]);
                min = v256_min_s16(min, tap[6]);
                min = v256_min_s16(min, tap[7]);
            }
        }

        // res = row + ((sum - (sum < 0) + 8) >> 4)
        sum = v256_add_16(sum, v256_cmplt_s16(sum, v256_zero()));
        res = v256_add_16(sum, v256_dup_16(8));
        res = v256_shr_n_s16::<4>(res);
        res = v256_add_16(row, res);
        if clipping_required {
            res = v256_min_s16(v256_max_s16(res, min), max);
        }

        if is_lowbd != 0 {
            let res_128 = v256_low_v128(v256_pack_s16_u8(res, res));
            u32_store_aligned(dst8.offset(((i + 0) * dstride) as isize).cast(),
                              v64_high_u32(v128_high_v64(res_128)));
            u32_store_aligned(dst8.offset(((i + 1) * dstride) as isize).cast(),
                              v64_low_u32(v128_high_v64(res_128)));
            u32_store_aligned(dst8.offset(((i + 2) * dstride) as isize).cast(),
                              v64_high_u32(v128_low_v64(res_128)));
            u32_store_aligned(dst8.offset(((i + 3) * dstride) as isize).cast(),
                              v64_low_u32(v128_low_v64(res_128)));
        } else {
            v64_store_aligned(dst16.offset(((i + 0) * dstride) as isize).cast(),
                              v128_high_v64(v256_high_v128(res)));
            v64_store_aligned(dst16.offset(((i + 1) * dstride) as isize).cast(),
                              v128_low_v64(v256_high_v128(res)));
            v64_store_aligned(dst16.offset(((i + 2) * dstride) as isize).cast(),
                              v128_high_v64(v256_low_v128(res)));
            v64_store_aligned(dst16.offset(((i + 3) * dstride) as isize).cast(),
                              v128_low_v64(v256_low_v128(res)));
        }
        i += 4;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn filter_block_8x8(is_lowbd: i32, dest: *mut c_void, dstride: i32, in_: *const u16,
                           pri_strength: i32, sec_strength: i32, dir: i32,
                           mut pri_damping: i32, mut sec_damping: i32, coeff_shift: i32,
                           height: i32, enable_primary: i32, enable_secondary: i32) {
    let dst8 = dest as *mut u8;
    let dst16 = dest as *mut u16;
    let clipping_required = enable_primary != 0 && enable_secondary != 0;
    let mut i: i32;
    let mut sum: v256;
    let mut p0: v256;
    let mut p1: v256;
    let mut p2: v256;
    let mut p3: v256;
    let mut row: v256;
    let mut res: v256;
    let cdef_large_value_mask = v256_dup_16((!CDEF_VERY_LARGE) as u16);
    let mut max: v256;
    let mut min: v256;
    let po1 = cdef_dir(dir, 0);
    let po2 = cdef_dir(dir, 1);
    let s1o1 = cdef_dir(dir + 2, 0);
    let s1o2 = cdef_dir(dir + 2, 1);
    let s2o1 = cdef_dir(dir - 2, 0);
    let s2o2 = cdef_dir(dir - 2, 1);
    let pri_taps = cdef_pri_taps_row((pri_strength >> coeff_shift) & 1);
    let sec_taps = cdef_sec_taps_ptr();

    if enable_primary != 0 && pri_strength != 0 {
        pri_damping = aommax(0, pri_damping - get_msb(pri_strength as u32));
    }
    if enable_secondary != 0 && sec_strength != 0 {
        sec_damping = aommax(0, sec_damping - get_msb(sec_strength as u32));
    }

    i = 0;
    while i < height {
        let mut tap: [v256; 8] = core::mem::zeroed();
        sum = v256_zero();
        row = v256_from_v128(v128_load_aligned(in_.offset((i * CDEF_BSTRIDE) as isize).cast()),
                             v128_load_aligned(in_.offset(((i + 1) * CDEF_BSTRIDE) as isize).cast()));

        max = row;
        min = max;
        if enable_primary != 0 {
            // Primary near taps
            tap[0] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE + po1) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + po1) as isize).cast()));
            tap[1] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE - po1) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - po1) as isize).cast()));
            p0 = constrain16(tap[0], row, pri_strength as u32, pri_damping as u32);
            p1 = constrain16(tap[1], row, pri_strength as u32, pri_damping as u32);

            // sum += pri_taps[0] * (p0 + p1)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*pri_taps.add(0) as u16),
                                                  v256_add_16(p0, p1)));

            // Primary far taps
            tap[2] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE + po2) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + po2) as isize).cast()));
            tap[3] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE - po2) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - po2) as isize).cast()));
            p0 = constrain16(tap[2], row, pri_strength as u32, pri_damping as u32);
            p1 = constrain16(tap[3], row, pri_strength as u32, pri_damping as u32);

            // sum += pri_taps[1] * (p0 + p1)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*pri_taps.add(1) as u16),
                                                  v256_add_16(p0, p1)));

            if clipping_required {
                max = get_max_primary(is_lowbd, tap.as_ptr(), max, cdef_large_value_mask);

                min = v256_min_s16(min, tap[0]);
                min = v256_min_s16(min, tap[1]);
                min = v256_min_s16(min, tap[2]);
                min = v256_min_s16(min, tap[3]);
            }
            // End primary
        }

        if enable_secondary != 0 {
            // Secondary near taps
            tap[0] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE + s1o1) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s1o1) as isize).cast()));
            tap[1] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE - s1o1) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s1o1) as isize).cast()));
            tap[2] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE + s2o1) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s2o1) as isize).cast()));
            tap[3] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE - s2o1) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s2o1) as isize).cast()));
            p0 = constrain16(tap[0], row, sec_strength as u32, sec_damping as u32);
            p1 = constrain16(tap[1], row, sec_strength as u32, sec_damping as u32);
            p2 = constrain16(tap[2], row, sec_strength as u32, sec_damping as u32);
            p3 = constrain16(tap[3], row, sec_strength as u32, sec_damping as u32);

            // sum += sec_taps[0] * (p0 + p1 + p2 + p3)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*sec_taps.add(0) as u16),
                                                  v256_add_16(v256_add_16(p0, p1),
                                                              v256_add_16(p2, p3))));

            // Secondary far taps
            tap[4] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE + s1o2) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s1o2) as isize).cast()));
            tap[5] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE - s1o2) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s1o2) as isize).cast()));
            tap[6] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE + s2o2) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE + s2o2) as isize).cast()));
            tap[7] = v256_from_v128(
                v128_load_unaligned(in_.offset((i * CDEF_BSTRIDE - s2o2) as isize).cast()),
                v128_load_unaligned(in_.offset(((i + 1) * CDEF_BSTRIDE - s2o2) as isize).cast()));
            p0 = constrain16(tap[4], row, sec_strength as u32, sec_damping as u32);
            p1 = constrain16(tap[5], row, sec_strength as u32, sec_damping as u32);
            p2 = constrain16(tap[6], row, sec_strength as u32, sec_damping as u32);
            p3 = constrain16(tap[7], row, sec_strength as u32, sec_damping as u32);

            // sum += sec_taps[1] * (p0 + p1 + p2 + p3)
            sum = v256_add_16(sum, v256_mullo_s16(v256_dup_16(*sec_taps.add(1) as u16),
                                                  v256_add_16(v256_add_16(p0, p1),
                                                              v256_add_16(p2, p3))));

            if clipping_required {
                max = get_max_secondary(is_lowbd, tap.as_ptr(), max, cdef_large_value_mask);

                min = v256_min_s16(min, tap[0]);
                min = v256_min_s16(min, tap[1]);
                min = v256_min_s16(min, tap[2]);
                min = v256_min_s16(min, tap[3]);
                min = v256_min_s16(min, tap[4]);
                min = v256_min_s16(min, tap[5]);
                min = v256_min_s16(min, tap[6]);
                min = v256_min_s16(min, tap[7]);
            }
            // End secondary
        }

        // res = row + ((sum - (sum < 0) + 8) >> 4)
        sum = v256_add_16(sum, v256_cmplt_s16(sum, v256_zero()));
        res = v256_add_16(sum, v256_dup_16(8));
        res = v256_shr_n_s16::<4>(res);
        res = v256_add_16(row, res);
        if clipping_required {
            res = v256_min_s16(v256_max_s16(res, min), max);
        }

        if is_lowbd != 0 {
            let res_128 = v256_low_v128(v256_pack_s16_u8(res, res));
            v64_store_aligned(dst8.offset((i * dstride) as isize).cast(), v128_high_v64(res_128));
            v64_store_aligned(dst8.offset(((i + 1) * dstride) as isize).cast(),
                              v128_low_v64(res_128));
        } else {
            v128_store_unaligned(dst16.offset((i * dstride) as isize).cast(), v256_high_v128(res));
            v128_store_unaligned(dst16.offset(((i + 1) * dstride) as isize).cast(),
                                 v256_low_v128(res));
        }
        i += 2;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn copy_block_4xh(is_lowbd: i32, dest: *mut c_void, dstride: i32, in_: *const u16,
                         height: i32) {
    let dst8 = dest as *mut u8;
    let dst16 = dest as *mut u16;
    let mut i: i32;
    i = 0;
    while i < height {
        let row0 = v128_from_v64(v64_load_aligned(in_.offset(((i + 0) * CDEF_BSTRIDE) as isize).cast()),
                                 v64_load_aligned(in_.offset(((i + 1) * CDEF_BSTRIDE) as isize).cast()));
        let row1 = v128_from_v64(v64_load_aligned(in_.offset(((i + 2) * CDEF_BSTRIDE) as isize).cast()),
                                 v64_load_aligned(in_.offset(((i + 3) * CDEF_BSTRIDE) as isize).cast()));
        if is_lowbd != 0 {
            /* Note: v128_pack_s16_u8(). The parameter order is swapped internally */
            let res_128 = v128_pack_s16_u8(row1, row0);
            u32_store_aligned(dst8.offset(((i + 0) * dstride) as isize).cast(),
                              v64_high_u32(v128_low_v64(res_128)));
            u32_store_aligned(dst8.offset(((i + 1) * dstride) as isize).cast(),
                              v64_low_u32(v128_low_v64(res_128)));
            u32_store_aligned(dst8.offset(((i + 2) * dstride) as isize).cast(),
                              v64_high_u32(v128_high_v64(res_128)));
            u32_store_aligned(dst8.offset(((i + 3) * dstride) as isize).cast(),
                              v64_low_u32(v128_high_v64(res_128)));
        } else {
            v64_store_aligned(dst16.offset(((i + 0) * dstride) as isize).cast(),
                              v128_high_v64(row0));
            v64_store_aligned(dst16.offset(((i + 1) * dstride) as isize).cast(),
                              v128_low_v64(row0));
            v64_store_aligned(dst16.offset(((i + 2) * dstride) as isize).cast(),
                              v128_high_v64(row1));
            v64_store_aligned(dst16.offset(((i + 3) * dstride) as isize).cast(),
                              v128_low_v64(row1));
        }
        i += 4;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn copy_block_8xh(is_lowbd: i32, dest: *mut c_void, dstride: i32, in_: *const u16,
                         height: i32) {
    let dst8 = dest as *mut u8;
    let dst16 = dest as *mut u16;
    let mut i: i32;
    i = 0;
    while i < height {
        let row0 = v128_load_aligned(in_.offset((i * CDEF_BSTRIDE) as isize).cast());
        let row1 = v128_load_aligned(in_.offset(((i + 1) * CDEF_BSTRIDE) as isize).cast());
        if is_lowbd != 0 {
            /* Note: v128_pack_s16_u8(). The parameter order is swapped internally */
            let res_128 = v128_pack_s16_u8(row1, row0);
            v64_store_aligned(dst8.offset((i * dstride) as isize).cast(), v128_low_v64(res_128));
            v64_store_aligned(dst8.offset(((i + 1) * dstride) as isize).cast(),
                              v128_high_v64(res_128));
        } else {
            v128_store_unaligned(dst16.offset((i * dstride) as isize).cast(), row0);
            v128_store_unaligned(dst16.offset(((i + 1) * dstride) as isize).cast(), row1);
        }
        i += 2;
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_8_0_avx2(dest: *mut c_void, dstride: i32,
                                                     in_: *const u16, pri_strength: i32,
                                                     sec_strength: i32, dir: i32,
                                                     pri_damping: i32, sec_damping: i32,
                                                     coeff_shift: i32, block_width: i32,
                                                     block_height: i32) {
    if block_width == 8 {
        filter_block_8x8(/*is_lowbd=*/ 1, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 1);
    } else {
        filter_block_4x4(/*is_lowbd=*/ 1, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 1);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_8_1_avx2(dest: *mut c_void, dstride: i32,
                                                     in_: *const u16, pri_strength: i32,
                                                     sec_strength: i32, dir: i32,
                                                     pri_damping: i32, sec_damping: i32,
                                                     coeff_shift: i32, block_width: i32,
                                                     block_height: i32) {
    if block_width == 8 {
        filter_block_8x8(/*is_lowbd=*/ 1, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 0);
    } else {
        filter_block_4x4(/*is_lowbd=*/ 1, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 0);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_8_2_avx2(dest: *mut c_void, dstride: i32,
                                                     in_: *const u16, pri_strength: i32,
                                                     sec_strength: i32, dir: i32,
                                                     pri_damping: i32, sec_damping: i32,
                                                     coeff_shift: i32, block_width: i32,
                                                     block_height: i32) {
    if block_width == 8 {
        filter_block_8x8(/*is_lowbd=*/ 1, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 0, /*enable_secondary=*/ 1);
    } else {
        filter_block_4x4(/*is_lowbd=*/ 1, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 0, /*enable_secondary=*/ 1);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_8_3_avx2(dest: *mut c_void, dstride: i32,
                                                     in_: *const u16, pri_strength: i32,
                                                     sec_strength: i32, dir: i32,
                                                     pri_damping: i32, sec_damping: i32,
                                                     coeff_shift: i32, block_width: i32,
                                                     block_height: i32) {
    let _ = pri_strength;
    let _ = sec_strength;
    let _ = dir;
    let _ = pri_damping;
    let _ = sec_damping;
    let _ = coeff_shift;
    let _ = block_width;

    if block_width == 8 {
        copy_block_8xh(/*is_lowbd=*/ 1, dest, dstride, in_, block_height);
    } else {
        copy_block_4xh(/*is_lowbd=*/ 1, dest, dstride, in_, block_height);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_16_0_avx2(dest: *mut c_void, dstride: i32,
                                                      in_: *const u16, pri_strength: i32,
                                                      sec_strength: i32, dir: i32,
                                                      pri_damping: i32, sec_damping: i32,
                                                      coeff_shift: i32, block_width: i32,
                                                      block_height: i32) {
    if block_width == 8 {
        filter_block_8x8(/*is_lowbd=*/ 0, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 1);
    } else {
        filter_block_4x4(/*is_lowbd=*/ 0, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 1);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_16_1_avx2(dest: *mut c_void, dstride: i32,
                                                      in_: *const u16, pri_strength: i32,
                                                      sec_strength: i32, dir: i32,
                                                      pri_damping: i32, sec_damping: i32,
                                                      coeff_shift: i32, block_width: i32,
                                                      block_height: i32) {
    if block_width == 8 {
        filter_block_8x8(/*is_lowbd=*/ 0, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 0);
    } else {
        filter_block_4x4(/*is_lowbd=*/ 0, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 1, /*enable_secondary=*/ 0);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_16_2_avx2(dest: *mut c_void, dstride: i32,
                                                      in_: *const u16, pri_strength: i32,
                                                      sec_strength: i32, dir: i32,
                                                      pri_damping: i32, sec_damping: i32,
                                                      coeff_shift: i32, block_width: i32,
                                                      block_height: i32) {
    if block_width == 8 {
        filter_block_8x8(/*is_lowbd=*/ 0, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 0, /*enable_secondary=*/ 1);
    } else {
        filter_block_4x4(/*is_lowbd=*/ 0, dest, dstride, in_, pri_strength, sec_strength, dir,
                         pri_damping, sec_damping, coeff_shift, block_height,
                         /*enable_primary=*/ 0, /*enable_secondary=*/ 1);
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_filter_16_3_avx2(dest: *mut c_void, dstride: i32,
                                                      in_: *const u16, pri_strength: i32,
                                                      sec_strength: i32, dir: i32,
                                                      pri_damping: i32, sec_damping: i32,
                                                      coeff_shift: i32, block_width: i32,
                                                      block_height: i32) {
    let _ = pri_strength;
    let _ = sec_strength;
    let _ = dir;
    let _ = pri_damping;
    let _ = sec_damping;
    let _ = coeff_shift;
    let _ = block_width;
    if block_width == 8 {
        copy_block_8xh(/*is_lowbd=*/ 0, dest, dstride, in_, block_height);
    } else {
        copy_block_4xh(/*is_lowbd=*/ 0, dest, dstride, in_, block_height);
    }
}

// #if CONFIG_AV1_HIGHBITDEPTH (enabled in both translated profiles)
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_copy_rect8_16bit_to_16bit_avx2(dst: *mut u16, dstride: i32,
                                                                    src: *const u16,
                                                                    sstride: i32, width: i32,
                                                                    height: i32) {
    let mut i: i32;
    let mut j: i32;
    i = 0;
    while i < height {
        j = 0;
        while j < (width & !0x7) {
            let row = v128_load_unaligned(src.offset((i * sstride + j) as isize).cast());
            v128_store_unaligned(dst.offset((i * dstride + j) as isize).cast(), row);
            j += 8;
        }
        while j < width {
            *dst.offset((i * dstride + j) as isize) = *src.offset((i * sstride + j) as isize);
            j += 1;
        }
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// av1/common/x86/cdef_block_avx2.c
// ---------------------------------------------------------------------------

/* partial A is a 16-bit vector of the form:
[x8 - - x1 | x16 - - x9] and partial B has the form:
[0  y1 - y7 | 0 y9 - y15].
This function computes (x1^2+y1^2)*C1 + (x2^2+y2^2)*C2 + ...
(x7^2+y2^7)*C7 + (x8^2+0^2)*C8 on each 128-bit lane. Here the C1..C8 constants
are in const1 and const2. */
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fold_mul_and_sum_avx2(partiala: *mut __m256i, partialb: *mut __m256i,
                                const1: *const __m256i, const2: *const __m256i) -> __m256i {
    // Mask used to shuffle the elements present in 256bit register.
    static shuffle_reg_256bit: [i32; 8] = [0x0b0a0d0c, 0x07060908, 0x03020504, 0x0f0e0100,
                                           0x0b0a0d0c, 0x07060908, 0x03020504, 0x0f0e0100];
    let tmp: __m256i;
    /* Reverse partial B. */
    *partialb = _mm256_shuffle_epi8(*partialb,
                                    _mm256_loadu_si256(shuffle_reg_256bit.as_ptr() as *const __m256i));

    /* Interleave the x and y values of identical indices and pair x8 with 0. */
    tmp = *partiala;
    *partiala = _mm256_unpacklo_epi16(*partiala, *partialb);
    *partialb = _mm256_unpackhi_epi16(tmp, *partialb);

    /* Square and add the corresponding x and y values. */
    *partiala = _mm256_madd_epi16(*partiala, *partiala);
    *partialb = _mm256_madd_epi16(*partialb, *partialb);
    /* Multiply by constant. */
    *partiala = _mm256_mullo_epi32(*partiala, *const1);
    *partialb = _mm256_mullo_epi32(*partialb, *const2);
    /* Sum all results. */
    *partiala = _mm256_add_epi32(*partiala, *partialb);
    *partiala
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn hsum4_avx2(x0: *mut __m256i, x1: *mut __m256i, x2: *mut __m256i,
                     x3: *mut __m256i) -> __m256i {
    let t0 = _mm256_unpacklo_epi32(*x0, *x1);
    let t1 = _mm256_unpacklo_epi32(*x2, *x3);
    let t2 = _mm256_unpackhi_epi32(*x0, *x1);
    let t3 = _mm256_unpackhi_epi32(*x2, *x3);

    *x0 = _mm256_unpacklo_epi64(t0, t1);
    *x1 = _mm256_unpackhi_epi64(t0, t1);
    *x2 = _mm256_unpacklo_epi64(t2, t3);
    *x3 = _mm256_unpackhi_epi64(t2, t3);
    _mm256_add_epi32(_mm256_add_epi32(*x0, *x1), _mm256_add_epi32(*x2, *x3))
}

/* Computes cost for directions 0, 5, 6 and 7. We can call this function again
to compute the remaining directions. */
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn compute_directions_avx2(lines: *const __m256i, cost_frist_8x8: *mut i32,
                                  cost_second_8x8: *mut i32) -> __m256i {
    let lines = core::slice::from_raw_parts(lines, 8);
    let mut partial4a: __m256i;
    let mut partial4b: __m256i;
    let mut partial5a: __m256i;
    let mut partial5b: __m256i;
    let mut partial7a: __m256i;
    let mut partial7b: __m256i;
    let mut partial6: __m256i;
    let mut tmp: __m256i;
    /* Partial sums for lines 0 and 1. */
    partial4a = _mm256_slli_si256::<14>(lines[0]);
    partial4b = _mm256_srli_si256::<2>(lines[0]);
    partial4a = _mm256_add_epi16(partial4a, _mm256_slli_si256::<12>(lines[1]));
    partial4b = _mm256_add_epi16(partial4b, _mm256_srli_si256::<4>(lines[1]));
    tmp = _mm256_add_epi16(lines[0], lines[1]);
    partial5a = _mm256_slli_si256::<10>(tmp);
    partial5b = _mm256_srli_si256::<6>(tmp);
    partial7a = _mm256_slli_si256::<4>(tmp);
    partial7b = _mm256_srli_si256::<12>(tmp);
    partial6 = tmp;

    /* Partial sums for lines 2 and 3. */
    partial4a = _mm256_add_epi16(partial4a, _mm256_slli_si256::<10>(lines[2]));
    partial4b = _mm256_add_epi16(partial4b, _mm256_srli_si256::<6>(lines[2]));
    partial4a = _mm256_add_epi16(partial4a, _mm256_slli_si256::<8>(lines[3]));
    partial4b = _mm256_add_epi16(partial4b, _mm256_srli_si256::<8>(lines[3]));
    tmp = _mm256_add_epi16(lines[2], lines[3]);
    partial5a = _mm256_add_epi16(partial5a, _mm256_slli_si256::<8>(tmp));
    partial5b = _mm256_add_epi16(partial5b, _mm256_srli_si256::<8>(tmp));
    partial7a = _mm256_add_epi16(partial7a, _mm256_slli_si256::<6>(tmp));
    partial7b = _mm256_add_epi16(partial7b, _mm256_srli_si256::<10>(tmp));
    partial6 = _mm256_add_epi16(partial6, tmp);

    /* Partial sums for lines 4 and 5. */
    partial4a = _mm256_add_epi16(partial4a, _mm256_slli_si256::<6>(lines[4]));
    partial4b = _mm256_add_epi16(partial4b, _mm256_srli_si256::<10>(lines[4]));
    partial4a = _mm256_add_epi16(partial4a, _mm256_slli_si256::<4>(lines[5]));
    partial4b = _mm256_add_epi16(partial4b, _mm256_srli_si256::<12>(lines[5]));
    tmp = _mm256_add_epi16(lines[4], lines[5]);
    partial5a = _mm256_add_epi16(partial5a, _mm256_slli_si256::<6>(tmp));
    partial5b = _mm256_add_epi16(partial5b, _mm256_srli_si256::<10>(tmp));
    partial7a = _mm256_add_epi16(partial7a, _mm256_slli_si256::<8>(tmp));
    partial7b = _mm256_add_epi16(partial7b, _mm256_srli_si256::<8>(tmp));
    partial6 = _mm256_add_epi16(partial6, tmp);

    /* Partial sums for lines 6 and 7. */
    partial4a = _mm256_add_epi16(partial4a, _mm256_slli_si256::<2>(lines[6]));
    partial4b = _mm256_add_epi16(partial4b, _mm256_srli_si256::<14>(lines[6]));
    partial4a = _mm256_add_epi16(partial4a, lines[7]);
    tmp = _mm256_add_epi16(lines[6], lines[7]);
    partial5a = _mm256_add_epi16(partial5a, _mm256_slli_si256::<4>(tmp));
    partial5b = _mm256_add_epi16(partial5b, _mm256_srli_si256::<12>(tmp));
    partial7a = _mm256_add_epi16(partial7a, _mm256_slli_si256::<10>(tmp));
    partial7b = _mm256_add_epi16(partial7b, _mm256_srli_si256::<6>(tmp));
    partial6 = _mm256_add_epi16(partial6, tmp);

    let const_reg_1 = _mm256_set_epi32(210, 280, 420, 840, 210, 280, 420, 840);
    let const_reg_2 = _mm256_set_epi32(105, 120, 140, 168, 105, 120, 140, 168);
    let const_reg_3 = _mm256_set_epi32(210, 420, 0, 0, 210, 420, 0, 0);
    let const_reg_4 = _mm256_set_epi32(105, 105, 105, 140, 105, 105, 105, 140);

    /* Compute costs in terms of partial sums. */
    partial4a = fold_mul_and_sum_avx2(&mut partial4a, &mut partial4b, &const_reg_1, &const_reg_2);
    partial7a = fold_mul_and_sum_avx2(&mut partial7a, &mut partial7b, &const_reg_3, &const_reg_4);
    partial5a = fold_mul_and_sum_avx2(&mut partial5a, &mut partial5b, &const_reg_3, &const_reg_4);
    partial6 = _mm256_madd_epi16(partial6, partial6);
    partial6 = _mm256_mullo_epi32(partial6, _mm256_set1_epi32(105));

    partial4a = hsum4_avx2(&mut partial4a, &mut partial5a, &mut partial6, &mut partial7a);
    _mm_storeu_si128(cost_frist_8x8 as *mut __m128i, _mm256_castsi256_si128(partial4a));
    _mm_storeu_si128(cost_second_8x8 as *mut __m128i, _mm256_extractf128_si256::<1>(partial4a));

    partial4a
}

/* transpose and reverse the order of the lines -- equivalent to a 90-degree
counter-clockwise rotation of the pixels. */
// The caller passes the same array as `in` and `res`; every input is read
// before the first result is written, as in the C.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn array_reverse_transpose_8x8_avx2(in_: *const __m256i, res: *mut __m256i) {
    let tr0_0 = _mm256_unpacklo_epi16(*in_.add(0), *in_.add(1));
    let tr0_1 = _mm256_unpacklo_epi16(*in_.add(2), *in_.add(3));
    let tr0_2 = _mm256_unpackhi_epi16(*in_.add(0), *in_.add(1));
    let tr0_3 = _mm256_unpackhi_epi16(*in_.add(2), *in_.add(3));
    let tr0_4 = _mm256_unpacklo_epi16(*in_.add(4), *in_.add(5));
    let tr0_5 = _mm256_unpacklo_epi16(*in_.add(6), *in_.add(7));
    let tr0_6 = _mm256_unpackhi_epi16(*in_.add(4), *in_.add(5));
    let tr0_7 = _mm256_unpackhi_epi16(*in_.add(6), *in_.add(7));

    let tr1_0 = _mm256_unpacklo_epi32(tr0_0, tr0_1);
    let tr1_1 = _mm256_unpacklo_epi32(tr0_4, tr0_5);
    let tr1_2 = _mm256_unpackhi_epi32(tr0_0, tr0_1);
    let tr1_3 = _mm256_unpackhi_epi32(tr0_4, tr0_5);
    let tr1_4 = _mm256_unpacklo_epi32(tr0_2, tr0_3);
    let tr1_5 = _mm256_unpacklo_epi32(tr0_6, tr0_7);
    let tr1_6 = _mm256_unpackhi_epi32(tr0_2, tr0_3);
    let tr1_7 = _mm256_unpackhi_epi32(tr0_6, tr0_7);

    *res.add(7) = _mm256_unpacklo_epi64(tr1_0, tr1_1);
    *res.add(6) = _mm256_unpackhi_epi64(tr1_0, tr1_1);
    *res.add(5) = _mm256_unpacklo_epi64(tr1_2, tr1_3);
    *res.add(4) = _mm256_unpackhi_epi64(tr1_2, tr1_3);
    *res.add(3) = _mm256_unpacklo_epi64(tr1_4, tr1_5);
    *res.add(2) = _mm256_unpackhi_epi64(tr1_4, tr1_5);
    *res.add(1) = _mm256_unpacklo_epi64(tr1_6, tr1_7);
    *res.add(0) = _mm256_unpackhi_epi64(tr1_6, tr1_7);
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_find_dir_dual_avx2(img1: *const u16, img2: *const u16,
                                                        stride: i32, var_out_1st: *mut i32,
                                                        var_out_2nd: *mut i32, coeff_shift: i32,
                                                        out_dir_1st_8x8: *mut i32,
                                                        out_dir_2nd_8x8: *mut i32) {
    let mut cost_first_8x8: [i32; 8] = [0; 8];
    let mut cost_second_8x8: [i32; 8] = [0; 8];
    // Used to store the best cost for 2 8x8's.
    let mut best_cost: [i32; 2] = [0; 2];
    // Best direction for 2 8x8's.
    let mut best_dir: [i32; 2] = [0; 2];

    let const_coeff_shift_reg = _mm_cvtsi32_si128(coeff_shift);
    let const_128_reg = _mm256_set1_epi16(128);
    let mut lines: [__m256i; 8] = core::mem::zeroed();
    for i in 0..8 {
        let src_1 = _mm_loadu_si128(img1.offset((i * stride) as isize) as *const __m128i);
        let src_2 = _mm_loadu_si128(img2.offset((i * stride) as isize) as *const __m128i);

        lines[i as usize] = _mm256_insertf128_si256::<1>(_mm256_castsi128_si256(src_1), src_2);
        lines[i as usize] = _mm256_sub_epi16(
            _mm256_sra_epi16(lines[i as usize], const_coeff_shift_reg), const_128_reg);
    }

    /* Compute "mostly vertical" directions. */
    let dir47 = compute_directions_avx2(lines.as_ptr(), cost_first_8x8.as_mut_ptr().add(4),
                                        cost_second_8x8.as_mut_ptr().add(4));

    /* Transpose and reverse the order of the lines. */
    let lines_ptr = lines.as_mut_ptr();
    array_reverse_transpose_8x8_avx2(lines_ptr, lines_ptr);

    /* Compute "mostly horizontal" directions. */
    let dir03 = compute_directions_avx2(lines.as_ptr(), cost_first_8x8.as_mut_ptr(),
                                        cost_second_8x8.as_mut_ptr());

    let mut max = _mm256_max_epi32(dir03, dir47);
    max = _mm256_max_epi32(max, _mm256_or_si256(_mm256_srli_si256::<8>(max),
                                                _mm256_slli_si256::<{ 16 - (8) }>(max)));
    max = _mm256_max_epi32(max, _mm256_or_si256(_mm256_srli_si256::<4>(max),
                                                _mm256_slli_si256::<{ 16 - (4) }>(max)));

    let first_8x8_output = _mm256_castsi256_si128(max);
    let second_8x8_output = _mm256_extractf128_si256::<1>(max);
    let cmpeg_res_00 = _mm_cmpeq_epi32(first_8x8_output, _mm256_castsi256_si128(dir47));
    let cmpeg_res_01 = _mm_cmpeq_epi32(first_8x8_output, _mm256_castsi256_si128(dir03));
    let cmpeg_res_10 = _mm_cmpeq_epi32(second_8x8_output, _mm256_extractf128_si256::<1>(dir47));
    let cmpeg_res_11 = _mm_cmpeq_epi32(second_8x8_output, _mm256_extractf128_si256::<1>(dir03));
    let t_first_8x8 = _mm_packs_epi32(cmpeg_res_01, cmpeg_res_00);
    let t_second_8x8 = _mm_packs_epi32(cmpeg_res_11, cmpeg_res_10);

    best_cost[0] = _mm_cvtsi128_si32(_mm256_castsi256_si128(max));
    best_cost[1] = _mm_cvtsi128_si32(second_8x8_output);
    best_dir[0] = _mm_movemask_epi8(_mm_packs_epi16(t_first_8x8, t_first_8x8));
    best_dir[0] = get_msb((best_dir[0] ^ (best_dir[0] - 1)) as u32); // Count trailing zeros
    best_dir[1] = _mm_movemask_epi8(_mm_packs_epi16(t_second_8x8, t_second_8x8));
    best_dir[1] = get_msb((best_dir[1] ^ (best_dir[1] - 1)) as u32); // Count trailing zeros

    /* Difference between the optimal variance and the variance along the
       orthogonal direction. Again, the sum(x^2) terms cancel out. */
    *var_out_1st = best_cost[0].wrapping_sub(cost_first_8x8[((best_dir[0] + 4) & 7) as usize]);
    *var_out_2nd = best_cost[1].wrapping_sub(cost_second_8x8[((best_dir[1] + 4) & 7) as usize]);

    /* We'd normally divide by 840, but dividing by 1024 is close enough
    for what we're going to do with this. */
    *var_out_1st >>= 10;
    *var_out_2nd >>= 10;
    *out_dir_1st_8x8 = best_dir[0];
    *out_dir_2nd_8x8 = best_dir[1];
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn cdef_copy_rect8_8bit_to_16bit_avx2(dst: *mut u16, dstride: i32,
                                                                   src: *const u8, sstride: i32,
                                                                   width: i32, height: i32) {
    let mut j: i32 = 0;
    let mut remaining_width = width;
    debug_assert!(height % 2 == 0);
    debug_assert!(height > 0);
    debug_assert!(width > 0);

    // Process multiple 32 pixels at a time.
    if remaining_width > 31 {
        let mut i: i32 = 0;
        loop {
            j = 0;
            loop {
                let row00 = _mm_loadu_si128(
                    src.offset(((i + 0) * sstride + (j + 0)) as isize) as *const __m128i);
                let row01 = _mm_loadu_si128(
                    src.offset(((i + 0) * sstride + (j + 16)) as isize) as *const __m128i);
                let row10 = _mm_loadu_si128(
                    src.offset(((i + 1) * sstride + (j + 0)) as isize) as *const __m128i);
                let row11 = _mm_loadu_si128(
                    src.offset(((i + 1) * sstride + (j + 16)) as isize) as *const __m128i);
                _mm256_storeu_si256(dst.offset(((i + 0) * dstride + (j + 0)) as isize) as *mut __m256i,
                                    _mm256_cvtepu8_epi16(row00));
                _mm256_storeu_si256(dst.offset(((i + 0) * dstride + (j + 16)) as isize) as *mut __m256i,
                                    _mm256_cvtepu8_epi16(row01));
                _mm256_storeu_si256(dst.offset(((i + 1) * dstride + (j + 0)) as isize) as *mut __m256i,
                                    _mm256_cvtepu8_epi16(row10));
                _mm256_storeu_si256(dst.offset(((i + 1) * dstride + (j + 16)) as isize) as *mut __m256i,
                                    _mm256_cvtepu8_epi16(row11));
                j += 32;
                if !(j <= width - 32) {
                    break;
                }
            }
            i += 2;
            if !(i < height) {
                break;
            }
        }
        remaining_width = width & 31;
    }

    // Process 16 pixels at a time.
    if remaining_width > 15 {
        let mut i: i32 = 0;
        loop {
            let row0 = _mm_loadu_si128(src.offset(((i + 0) * sstride + j) as isize) as *const __m128i);
            let row1 = _mm_loadu_si128(src.offset(((i + 1) * sstride + j) as isize) as *const __m128i);
            _mm256_storeu_si256(dst.offset(((i + 0) * dstride + j) as isize) as *mut __m256i,
                                _mm256_cvtepu8_epi16(row0));
            _mm256_storeu_si256(dst.offset(((i + 1) * dstride + j) as isize) as *mut __m256i,
                                _mm256_cvtepu8_epi16(row1));
            i += 2;
            if !(i < height) {
                break;
            }
        }
        remaining_width = width & 15;
        j += 16;
    }

    // Process 8 pixels at a time.
    if remaining_width > 7 {
        let mut i: i32 = 0;
        loop {
            let row0 = _mm_loadl_epi64(src.offset(((i + 0) * sstride + j) as isize) as *const __m128i);
            let row1 = _mm_loadl_epi64(src.offset(((i + 1) * sstride + j) as isize) as *const __m128i);
            _mm_storeu_si128(dst.offset(((i + 0) * dstride + j) as isize) as *mut __m128i,
                             _mm_unpacklo_epi8(row0, _mm_setzero_si128()));
            _mm_storeu_si128(dst.offset(((i + 1) * dstride + j) as isize) as *mut __m128i,
                             _mm_unpacklo_epi8(row1, _mm_setzero_si128()));
            i += 2;
            if !(i < height) {
                break;
            }
        }
        remaining_width = width & 7;
        j += 8;
    }

    // Process 4 pixels at a time.
    if remaining_width > 3 {
        let mut i: i32 = 0;
        loop {
            let row0 = _mm_cvtsi32_si128(
                (src.offset(((i + 0) * sstride + j) as isize) as *const i32).read_unaligned());
            let row1 = _mm_cvtsi32_si128(
                (src.offset(((i + 1) * sstride + j) as isize) as *const i32).read_unaligned());
            _mm_storel_epi64(dst.offset(((i + 0) * dstride + j) as isize) as *mut __m128i,
                             _mm_unpacklo_epi8(row0, _mm_setzero_si128()));
            _mm_storel_epi64(dst.offset(((i + 1) * dstride + j) as isize) as *mut __m128i,
                             _mm_unpacklo_epi8(row1, _mm_setzero_si128()));
            i += 2;
            if !(i < height) {
                break;
            }
        }
        remaining_width = width & 3;
        j += 4;
    }

    // Process the remaining pixels.
    if remaining_width != 0 {
        for i in 0..height {
            for k in j..width {
                *dst.offset((i * dstride + k) as isize) = *src.offset((i * sstride + k) as isize) as u16;
            }
        }
    }
}
