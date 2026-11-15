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

// Translated from upstream/aom/aom_dsp/x86/highbd_loopfilter_sse2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `sse2`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use crate::simd::aom_dsp__x86__lpf_common_sse2::*;
use crate::simd::aom_dsp__x86__synonyms::*;
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);

// SSE2
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn pixel_clamp(min: *const __m128i, max: *const __m128i, pixel: *mut __m128i) {
    *pixel = _mm_min_epi16(*pixel, *max);
    *pixel = _mm_max_epi16(*pixel, *min);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn abs_diff16(a: __m128i, b: __m128i) -> __m128i {
    return _mm_or_si128(_mm_subs_epu16(a, b), _mm_subs_epu16(b, a));
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_limit(
    bl: *const u8,
    l: *const u8,
    t: *const u8,
    bd: i32,
    blt: *mut __m128i,
    lt: *mut __m128i,
    thr: *mut __m128i,
    t80_out: *mut __m128i,
) {
    let shift: i32 = bd - 8;
    let zero: __m128i = _mm_setzero_si128();
    let mut x: __m128i = _mm_unpacklo_epi8(_mm_load_si128(bl as *const __m128i), zero);
    *blt = _mm_sll_epi16(x, _mm_cvtsi32_si128(shift));
    x = _mm_unpacklo_epi8(_mm_load_si128(l as *const __m128i), zero);
    *lt = _mm_sll_epi16(x, _mm_cvtsi32_si128(shift));
    x = _mm_unpacklo_epi8(_mm_load_si128(t as *const __m128i), zero);
    *thr = _mm_sll_epi16(x, _mm_cvtsi32_si128(shift));
    *t80_out = _mm_set1_epi16((1 << (bd - 1)) as i16);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn get_limit_dual(
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
    blt_out: *mut __m128i,
    lt_out: *mut __m128i,
    thr_out: *mut __m128i,
    t80_out: *mut __m128i,
) {
    let shift: i32 = bd - 8;
    let zero: __m128i = _mm_setzero_si128();
    let mut x0: __m128i = _mm_unpacklo_epi8(_mm_load_si128(_blimit0 as *const __m128i), zero);
    let mut x1: __m128i = _mm_unpacklo_epi8(_mm_load_si128(_blimit1 as *const __m128i), zero);
    x0 = _mm_unpacklo_epi64(x0, x1);
    *blt_out = _mm_sll_epi16(x0, _mm_cvtsi32_si128(shift));
    x0 = _mm_unpacklo_epi8(_mm_load_si128(_limit0 as *const __m128i), zero);
    x1 = _mm_unpacklo_epi8(_mm_load_si128(_limit1 as *const __m128i), zero);
    x0 = _mm_unpacklo_epi64(x0, x1);
    *lt_out = _mm_sll_epi16(x0, _mm_cvtsi32_si128(shift));
    x0 = _mm_unpacklo_epi8(_mm_load_si128(_thresh0 as *const __m128i), zero);
    x1 = _mm_unpacklo_epi8(_mm_load_si128(_thresh1 as *const __m128i), zero);
    x0 = _mm_unpacklo_epi64(x0, x1);
    *thr_out = _mm_sll_epi16(x0, _mm_cvtsi32_si128(shift));
    *t80_out = _mm_set1_epi16((1 << (bd - 1)) as i16);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn load_highbd_pixel(
    s: *const u16,
    size: i32,
    pitch: i32,
    p: *mut __m128i,
    q: *mut __m128i,
) {
    let mut i: i32 = 0;
    {
        i = 0;
        while i < size {
            *p.offset(i as isize) = _mm_loadu_si128(s.offset(-(((i + 1) * pitch) as isize))
                as *mut __m128i
                as *const __m128i);
            *q.offset(i as isize) =
                _mm_loadu_si128(s.offset((i * pitch) as isize) as *mut __m128i as *const __m128i);
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_filter_mask_dual(
    p: *const __m128i,
    q: *const __m128i,
    l: *const __m128i,
    bl: *const __m128i,
    mask: *mut __m128i,
) {
    let mut abs_p0q0: __m128i = abs_diff16(*p, *q);
    let mut abs_p1q1: __m128i = abs_diff16(*p.offset(1), *q.offset(1));
    abs_p0q0 = _mm_adds_epu16(abs_p0q0, abs_p0q0);
    abs_p1q1 = _mm_srli_epi16::<{ 1 }>(abs_p1q1);
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi16(1);
    let ffff: __m128i = _mm_set1_epi16(0xFFFF_i32 as i16);
    let mut max: __m128i = _mm_subs_epu16(_mm_adds_epu16(abs_p0q0, abs_p1q1), *bl);
    max = _mm_xor_si128(_mm_cmpeq_epi16(max, zero), ffff);
    max = _mm_and_si128(max, _mm_adds_epu16(*l, one));
    let mut i: i32 = 0;
    {
        i = 1;
        while i < 4 {
            max = _mm_max_epi16(
                max,
                abs_diff16(*p.offset(i as isize), *p.offset((i - 1) as isize)),
            );
            max = _mm_max_epi16(
                max,
                abs_diff16(*q.offset(i as isize), *q.offset((i - 1) as isize)),
            );
            i += 1;
        }
    }
    max = _mm_subs_epu16(max, *l);
    *mask = _mm_cmpeq_epi16(max, zero);
}

// return ~mask
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_hev_filter_mask_x_sse2(
    pq: *mut __m128i,
    x: i32,
    p1p0: *mut __m128i,
    q1q0: *mut __m128i,
    abs_p1p0: *mut __m128i,
    l: *mut __m128i,
    bl: *mut __m128i,
    t: *mut __m128i,
    hev: *mut __m128i,
    mask: *mut __m128i,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi16(1);
    let ffff: __m128i = _mm_set1_epi16(0xFFFF_i32 as i16);
    let mut abs_p0q0_p1q1: __m128i;
    let mut abs_p0q0: __m128i;
    let mut abs_p1q1: __m128i;
    let mut abs_q1q0: __m128i;
    let mut max: __m128i;
    let mut max01: __m128i;
    let mut h: __m128i;
    *p1p0 = _mm_unpacklo_epi64(*pq, *pq.offset(1));
    *q1q0 = _mm_unpackhi_epi64(*pq, *pq.offset(1));
    abs_p0q0_p1q1 = abs_diff16(*p1p0, *q1q0);
    abs_p0q0 = _mm_adds_epu16(abs_p0q0_p1q1, abs_p0q0_p1q1);
    abs_p0q0 = _mm_unpacklo_epi64(abs_p0q0, zero);
    abs_p1q1 = _mm_srli_si128::<8>(abs_p0q0_p1q1);
    abs_p1q1 = _mm_srli_epi16::<{ 1 }>(abs_p1q1); // divide by 2
    max = _mm_subs_epu16(_mm_adds_epu16(abs_p0q0, abs_p1q1), *bl);
    max = _mm_xor_si128(_mm_cmpeq_epi16(max, zero), ffff);
    // mask |= (abs(*p0 - *q0) * 2 + abs(*p1 - *q1) / 2  > blimit) * -1;
    // So taking maximums continues to work:
    max = _mm_and_si128(max, _mm_adds_epu16(*l, one));
    *abs_p1p0 = abs_diff16(*pq, *pq.offset(1));
    abs_q1q0 = _mm_srli_si128::<8>(*abs_p1p0);
    max01 = _mm_max_epi16(*abs_p1p0, abs_q1q0);
    // mask |= (abs(*p1 - *p0) > limit) * -1;
    // mask |= (abs(*q1 - *q0) > limit) * -1;
    h = _mm_subs_epu16(max01, *t);
    *hev = _mm_xor_si128(_mm_cmpeq_epi16(h, zero), ffff);
    // replicate for the further "merged variables" usage
    *hev = _mm_unpacklo_epi64(*hev, *hev);
    max = _mm_max_epi16(max, max01);
    let mut i: i32 = 0;
    {
        i = 2;
        while i < x {
            max = _mm_max_epi16(
                max,
                abs_diff16(*pq.offset(i as isize), *pq.offset((i - 1) as isize)),
            );
            i += 1;
        }
    }
    max = _mm_max_epi16(max, _mm_srli_si128::<8>(max));
    max = _mm_subs_epu16(max, *l);
    *mask = _mm_cmpeq_epi16(max, zero);
}

//  ~mask
#[inline]
#[target_feature(enable = "sse2")]
unsafe fn flat_mask_internal(
    th: *const __m128i,
    pq: *const __m128i,
    start: i32,
    end: i32,
    flat: *mut __m128i,
) {
    let mut i: i32 = 0;
    let mut max: __m128i = _mm_max_epi16(
        abs_diff16(*pq.offset(start as isize), *pq),
        abs_diff16(*pq.offset((start + 1) as isize), *pq),
    );
    {
        i = start + 2;
        while i < end {
            max = _mm_max_epi16(max, abs_diff16(*pq.offset(i as isize), *pq));
            i += 1;
        }
    }
    max = _mm_max_epi16(max, _mm_srli_si128::<8>(max));
    let mut ft: __m128i;
    ft = _mm_subs_epu16(max, *th);
    let zero: __m128i = _mm_setzero_si128();
    *flat = _mm_cmpeq_epi16(ft, zero);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn flat_mask_internal_dual(
    th: *const __m128i,
    p: *const __m128i,
    q: *const __m128i,
    start: i32,
    end: i32,
    flat: *mut __m128i,
) {
    let mut i: i32 = 0;
    let mut max: __m128i = _mm_max_epi16(
        abs_diff16(*q.offset(start as isize), *q),
        abs_diff16(*p.offset(start as isize), *p),
    );
    {
        i = start + 1;
        while i < end {
            max = _mm_max_epi16(max, abs_diff16(*p.offset(i as isize), *p));
            max = _mm_max_epi16(max, abs_diff16(*q.offset(i as isize), *q));
            i += 1;
        }
    }
    let mut ft: __m128i;
    ft = _mm_subs_epu16(max, *th);
    let zero: __m128i = _mm_setzero_si128();
    *flat = _mm_cmpeq_epi16(ft, zero);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_flat_mask4_sse2(
    pq: *mut __m128i,
    flat: *mut __m128i,
    flat2: *mut __m128i,
    bd: i32,
) {
    // check the distance 1,2,3 against 0
    let mut th: __m128i = _mm_set1_epi16(1);
    th = _mm_sll_epi16(th, _mm_cvtsi32_si128(bd - 8));
    flat_mask_internal(
        &raw mut th as *const __m128i,
        pq as *const __m128i,
        1,
        4,
        flat,
    );
    flat_mask_internal(
        &raw mut th as *const __m128i,
        pq as *const __m128i,
        4,
        7,
        flat2,
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_flat_mask4_dual_sse2(
    p: *const __m128i,
    q: *const __m128i,
    flat: *mut __m128i,
    flat2: *mut __m128i,
    bd: i32,
) {
    // check the distance 1,2,3 against 0
    let mut th: __m128i = _mm_set1_epi16(1);
    th = _mm_sll_epi16(th, _mm_cvtsi32_si128(bd - 8));
    flat_mask_internal_dual(&raw mut th as *const __m128i, p, q, 1, 4, flat);
    flat_mask_internal_dual(&raw mut th as *const __m128i, p, q, 4, 7, flat2);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_filter4_sse2(
    p1p0: *mut __m128i,
    q1q0: *mut __m128i,
    hev: *mut __m128i,
    mask: *mut __m128i,
    qs1qs0: *mut __m128i,
    ps1ps0: *mut __m128i,
    t80: *mut __m128i,
    bd: i32,
) {
    let zero: __m128i = _mm_setzero_si128();
    let one: __m128i = _mm_set1_epi16(1);
    let pmax: __m128i = _mm_subs_epi16(
        _mm_subs_epi16(_mm_sll_epi16(one, _mm_cvtsi32_si128(bd)), one),
        *t80,
    );
    let pmin: __m128i = _mm_subs_epi16(zero, *t80);
    let t3t4: __m128i = _mm_set_epi16(3, 3, 3, 3, 4, 4, 4, 4);
    let mut ps1ps0_work: __m128i = core::mem::zeroed();
    let mut qs1qs0_work: __m128i = core::mem::zeroed();
    let mut work: __m128i = core::mem::zeroed();
    let mut filt: __m128i = core::mem::zeroed();
    let mut filter2filter1: __m128i = core::mem::zeroed();
    let mut filter2filt: __m128i;
    let mut filter1filt: __m128i;
    ps1ps0_work = _mm_subs_epi16(*p1p0, *t80);
    qs1qs0_work = _mm_subs_epi16(*q1q0, *t80);
    work = _mm_subs_epi16(ps1ps0_work, qs1qs0_work);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut work);
    filt = _mm_and_si128(_mm_srli_si128::<8>(work), *hev);
    filt = _mm_subs_epi16(filt, work);
    filt = _mm_subs_epi16(filt, work);
    filt = _mm_subs_epi16(filt, work);
    // (aom_filter + 3 * (qs0 - ps0)) & mask
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut filt);
    filt = _mm_and_si128(filt, *mask);
    filt = _mm_unpacklo_epi64(filt, filt);
    filter2filter1 = _mm_adds_epi16(filt, t3t4);
    // signed_short_clamp
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut filter2filter1);
    filter2filter1 = _mm_srai_epi16::<{ 3 }>(filter2filter1);
    // >> 3
    filt = _mm_unpacklo_epi64(filter2filter1, filter2filter1);
    // filt >> 1
    filt = _mm_adds_epi16(filt, one);
    filt = _mm_srai_epi16::<{ 1 }>(filt);
    filt = _mm_andnot_si128(*hev, filt);
    filter2filt = _mm_unpackhi_epi64(filter2filter1, filt);
    filter1filt = _mm_unpacklo_epi64(filter2filter1, filt);
    qs1qs0_work = _mm_subs_epi16(qs1qs0_work, filter1filt);
    ps1ps0_work = _mm_adds_epi16(ps1ps0_work, filter2filt);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut qs1qs0_work);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut ps1ps0_work);
    *qs1qs0 = _mm_adds_epi16(qs1qs0_work, *t80);
    *ps1ps0 = _mm_adds_epi16(ps1ps0_work, *t80);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_filter4_dual_sse2(
    p: *mut __m128i,
    q: *mut __m128i,
    ps: *mut __m128i,
    qs: *mut __m128i,
    mask: *const __m128i,
    th: *const __m128i,
    bd: i32,
    t80: *mut __m128i,
) {
    let mut ps0: __m128i = _mm_subs_epi16(*p, *t80);
    let mut ps1: __m128i = _mm_subs_epi16(*p.offset(1), *t80);
    let mut qs0: __m128i = _mm_subs_epi16(*q, *t80);
    let mut qs1: __m128i = _mm_subs_epi16(*q.offset(1), *t80);
    let one: __m128i = _mm_set1_epi16(1);
    let pmax: __m128i = _mm_subs_epi16(
        _mm_subs_epi16(_mm_sll_epi16(one, _mm_cvtsi32_si128(bd)), one),
        *t80,
    );
    let zero: __m128i = _mm_setzero_si128();
    let pmin: __m128i = _mm_subs_epi16(zero, *t80);
    let mut filter: __m128i = _mm_subs_epi16(ps1, qs1);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut filter);
    // hev_filter
    let mut hev: __m128i;
    let abs_p1p0: __m128i = abs_diff16(*p.offset(1), *p);
    let abs_q1q0: __m128i = abs_diff16(*q.offset(1), *q);
    let mut h: __m128i = _mm_max_epi16(abs_p1p0, abs_q1q0);
    h = _mm_subs_epu16(h, *th);
    let ffff: __m128i = _mm_cmpeq_epi16(h, h);
    hev = _mm_xor_si128(_mm_cmpeq_epi16(h, zero), ffff);
    filter = _mm_and_si128(filter, hev);
    let x: __m128i = _mm_subs_epi16(qs0, ps0);
    filter = _mm_adds_epi16(filter, x);
    filter = _mm_adds_epi16(filter, x);
    filter = _mm_adds_epi16(filter, x);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut filter);
    filter = _mm_and_si128(filter, *mask);
    let t3: __m128i = _mm_set1_epi16(3);
    let t4: __m128i = _mm_set1_epi16(4);
    let mut filter1: __m128i = _mm_adds_epi16(filter, t4);
    let mut filter2: __m128i = _mm_adds_epi16(filter, t3);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut filter1);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut filter2);
    filter1 = _mm_srai_epi16::<{ 3 }>(filter1);
    filter2 = _mm_srai_epi16::<{ 3 }>(filter2);
    qs0 = _mm_subs_epi16(qs0, filter1);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut qs0);
    ps0 = _mm_adds_epi16(ps0, filter2);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut ps0);
    *qs = _mm_adds_epi16(qs0, *t80);
    *ps = _mm_adds_epi16(ps0, *t80);
    filter = _mm_adds_epi16(filter1, one);
    filter = _mm_srai_epi16::<{ 1 }>(filter);
    filter = _mm_andnot_si128(hev, filter);
    qs1 = _mm_subs_epi16(qs1, filter);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut qs1);
    ps1 = _mm_adds_epi16(ps1, filter);
    pixel_clamp(&raw const pmin, &raw const pmax, &raw mut ps1);
    *qs.offset(1) = _mm_adds_epi16(qs1, *t80);
    *ps.offset(1) = _mm_adds_epi16(ps1, *t80);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_14_sse2(
    p: *mut __m128i,
    q: *mut __m128i,
    pq: *mut __m128i,
    blt: *const u8,
    lt: *const u8,
    thr: *const u8,
    bd: i32,
) {
    let mut i: i32 = 0;
    let zero: __m128i = _mm_setzero_si128();
    let mut blimit: __m128i = core::mem::zeroed();
    let mut limit: __m128i = core::mem::zeroed();
    let mut thresh: __m128i = core::mem::zeroed();
    let mut t80: __m128i = core::mem::zeroed();
    get_limit(
        blt,
        lt,
        thr,
        bd,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
        &raw mut t80,
    );
    {
        i = 0;
        while i < 7 {
            *pq.offset(i as isize) =
                _mm_unpacklo_epi64(*p.offset(i as isize), *q.offset(i as isize));
            i += 1;
        }
    }
    let mut mask: __m128i = core::mem::zeroed();
    let mut hevhev: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut abs_p1p0: __m128i = core::mem::zeroed();
    highbd_hev_filter_mask_x_sse2(
        pq,
        4,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut abs_p1p0,
        &raw mut limit,
        &raw mut blimit,
        &raw mut thresh,
        &raw mut hevhev,
        &raw mut mask,
    );
    let mut ps0ps1: __m128i = core::mem::zeroed();
    let mut qs0qs1: __m128i = core::mem::zeroed();
    // filter4
    highbd_filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hevhev,
        &raw mut mask,
        &raw mut qs0qs1,
        &raw mut ps0ps1,
        &raw mut t80,
        bd,
    );
    let mut flat: __m128i = core::mem::zeroed();
    let mut flat2: __m128i = core::mem::zeroed();
    highbd_flat_mask4_sse2(pq, &raw mut flat, &raw mut flat2, bd);
    flat = _mm_and_si128(flat, mask);
    flat2 = _mm_and_si128(flat2, flat);
    // replicate for the further "merged variables" usage
    flat = _mm_unpacklo_epi64(flat, flat);
    flat2 = _mm_unpacklo_epi64(flat2, flat2);
    // flat and wide flat calculations
    // if flat ==0 then flat2 is zero as well and we don't need any calc below
    // sse4.1 if (0==_mm_test_all_zeros(flat,ff))
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat, zero)) {
        let mut flat_p: [__m128i; 3] = core::mem::zeroed();
        let mut flat_q: [__m128i; 3] = core::mem::zeroed();
        let mut flat_pq: [__m128i; 3] = core::mem::zeroed();
        let mut flat2_p: [__m128i; 6] = core::mem::zeroed();
        let mut flat2_q: [__m128i; 6] = core::mem::zeroed();
        let mut flat2_pq: [__m128i; 6] = core::mem::zeroed();
        let mut sum_p6: __m128i;
        let mut sum_p3: __m128i;
        let eight: __m128i = _mm_set1_epi16(8);
        let four: __m128i = _mm_set1_epi16(4);
        let mut work0: __m128i;
        let mut work0_0: __m128i;
        let mut work0_1: __m128i;
        let mut sum_p_0: __m128i;
        let mut sum_p: __m128i =
            _mm_add_epi16(*pq.offset(5), _mm_add_epi16(*pq.offset(4), *pq.offset(3)));
        let mut sum_lp: __m128i = _mm_add_epi16(*pq, _mm_add_epi16(*pq.offset(2), *pq.offset(1)));
        sum_p = _mm_add_epi16(sum_p, sum_lp);
        let mut sum_lq: __m128i = _mm_srli_si128::<8>(sum_lp);
        let mut sum_q: __m128i = _mm_srli_si128::<8>(sum_p);
        sum_p_0 = _mm_add_epi16(eight, _mm_add_epi16(sum_p, sum_q));
        sum_lp = _mm_add_epi16(four, _mm_add_epi16(sum_lp, sum_lq));
        flat_p[0] = _mm_add_epi16(sum_lp, _mm_add_epi16(*pq.offset(3), *pq));
        flat_q[0] = _mm_add_epi16(sum_lp, _mm_add_epi16(*q.offset(3), *q));
        sum_p6 = _mm_add_epi16(*pq.offset(6), *pq.offset(6));
        sum_p3 = _mm_add_epi16(*pq.offset(3), *pq.offset(3));
        sum_q = _mm_sub_epi16(sum_p_0, *pq.offset(5));
        sum_p = _mm_sub_epi16(sum_p_0, *q.offset(5));
        work0_0 = _mm_add_epi16(_mm_add_epi16(*pq.offset(6), *pq), *pq.offset(1));
        work0_1 = _mm_add_epi16(
            sum_p6,
            _mm_add_epi16(*pq.offset(1), _mm_add_epi16(*pq.offset(2), *pq)),
        );
        sum_lq = _mm_sub_epi16(sum_lp, *pq.offset(2));
        sum_lp = _mm_sub_epi16(sum_lp, *q.offset(2));
        work0 = _mm_add_epi16(sum_p3, *pq.offset(1));
        flat_p[1] = _mm_add_epi16(sum_lp, work0);
        flat_q[1] = _mm_add_epi16(sum_lq, _mm_srli_si128::<8>(work0));
        flat_pq[0] = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(flat_p[0], flat_q[0]));
        flat_pq[1] = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(flat_p[1], flat_q[1]));
        sum_lp = _mm_sub_epi16(sum_lp, *q.offset(1));
        sum_lq = _mm_sub_epi16(sum_lq, *pq.offset(1));
        sum_p3 = _mm_add_epi16(sum_p3, *pq.offset(3));
        work0 = _mm_add_epi16(sum_p3, *pq.offset(2));
        flat_p[2] = _mm_add_epi16(sum_lp, work0);
        flat_q[2] = _mm_add_epi16(sum_lq, _mm_srli_si128::<8>(work0));
        flat_pq[2] = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(flat_p[2], flat_q[2]));
        let mut flat2_mask: i32 =
            (0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat2, zero))) as i32;
        if flat2_mask != 0 {
            flat2_p[0] = _mm_add_epi16(sum_p_0, _mm_add_epi16(work0_0, *q));
            flat2_q[0] = _mm_add_epi16(sum_p_0, _mm_add_epi16(_mm_srli_si128::<8>(work0_0), *pq));
            flat2_p[1] = _mm_add_epi16(sum_p, work0_1);
            flat2_q[1] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0_1));
            flat2_pq[0] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[0], flat2_q[0]));
            flat2_pq[1] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[1], flat2_q[1]));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(4));
            sum_q = _mm_sub_epi16(sum_q, *pq.offset(4));
            sum_p6 = _mm_add_epi16(sum_p6, *pq.offset(6));
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(*pq.offset(2), _mm_add_epi16(*pq.offset(3), *pq.offset(1))),
            );
            flat2_p[2] = _mm_add_epi16(sum_p, work0);
            flat2_q[2] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[2] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[2], flat2_q[2]));
            sum_p6 = _mm_add_epi16(sum_p6, *pq.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(3));
            sum_q = _mm_sub_epi16(sum_q, *pq.offset(3));
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(*pq.offset(3), _mm_add_epi16(*pq.offset(4), *pq.offset(2))),
            );
            flat2_p[3] = _mm_add_epi16(sum_p, work0);
            flat2_q[3] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[3] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[3], flat2_q[3]));
            sum_p6 = _mm_add_epi16(sum_p6, *pq.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(2));
            sum_q = _mm_sub_epi16(sum_q, *pq.offset(2));
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(*pq.offset(4), _mm_add_epi16(*pq.offset(5), *pq.offset(3))),
            );
            flat2_p[4] = _mm_add_epi16(sum_p, work0);
            flat2_q[4] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[4] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[4], flat2_q[4]));
            sum_p6 = _mm_add_epi16(sum_p6, *pq.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(1));
            sum_q = _mm_sub_epi16(sum_q, *pq.offset(1));
            work0 = _mm_add_epi16(
                sum_p6,
                _mm_add_epi16(*pq.offset(5), _mm_add_epi16(*pq.offset(6), *pq.offset(4))),
            );
            flat2_p[5] = _mm_add_epi16(sum_p, work0);
            flat2_q[5] = _mm_add_epi16(sum_q, _mm_srli_si128::<8>(work0));
            flat2_pq[5] = _mm_srli_epi16::<{ 4 }>(_mm_unpacklo_epi64(flat2_p[5], flat2_q[5]));
        } // flat2
          // ~~~~~~~~~~ apply flat ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          // highbd_filter8
        *pq = _mm_unpacklo_epi64(ps0ps1, qs0qs1);
        *pq.offset(1) = _mm_unpackhi_epi64(ps0ps1, qs0qs1);
        {
            i = 0;
            while i < 3 {
                *pq.offset(i as isize) = _mm_andnot_si128(flat, *pq.offset(i as isize));
                flat_pq[i as usize] = _mm_and_si128(flat, flat_pq[i as usize]);
                *pq.offset(i as isize) = _mm_or_si128(*pq.offset(i as isize), flat_pq[i as usize]);
                i += 1;
            }
        }
        // wide flat
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        if flat2_mask != 0 {
            {
                i = 0;
                while i < 6 {
                    *pq.offset(i as isize) = _mm_andnot_si128(flat2, *pq.offset(i as isize));
                    flat2_pq[i as usize] = _mm_and_si128(flat2, flat2_pq[i as usize]);
                    *pq.offset(i as isize) =
                        _mm_or_si128(*pq.offset(i as isize), flat2_pq[i as usize]);
                    i += 1;
                }
            }
        }
    } else {
        // full list of pq values
        *pq = _mm_unpacklo_epi64(ps0ps1, qs0qs1);
        *pq.offset(1) = _mm_unpackhi_epi64(ps0ps1, qs0qs1);
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_sse2(
    s: *mut u16,
    pitch: i32,
    blimit: *const u8,
    limit: *const u8,
    thresh: *const u8,
    bd: i32,
) {
    let mut p: [__m128i; 7] = core::mem::zeroed();
    let mut q: [__m128i; 7] = core::mem::zeroed();
    let mut pq: [__m128i; 7] = core::mem::zeroed();
    let mut i: i32 = 0;
    {
        i = 0;
        while i < 7 {
            p[i as usize] = _mm_loadl_epi64(
                s.offset(-(((i + 1) * pitch) as isize)) as *mut __m128i as *const __m128i,
            );
            q[i as usize] =
                _mm_loadl_epi64(s.offset((i * pitch) as isize) as *mut __m128i as *const __m128i);
            i += 1;
        }
    }
    highbd_lpf_internal_14_sse2(
        p.as_mut_ptr(),
        q.as_mut_ptr(),
        pq.as_mut_ptr(),
        blimit,
        limit,
        thresh,
        bd,
    );
    {
        i = 0;
        while i < 6 {
            _mm_storel_epi64(
                s.offset(-(((i + 1) * pitch) as isize)) as *mut __m128i,
                pq[i as usize],
            );
            _mm_storel_epi64(
                s.offset((i * pitch) as isize) as *mut __m128i,
                _mm_srli_si128::<8>(pq[i as usize]),
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_14_dual_sse2(
    p: *mut __m128i,
    q: *mut __m128i,
    blt0: *const u8,
    lt0: *const u8,
    thr0: *const u8,
    blt1: *const u8,
    lt1: *const u8,
    thr1: *const u8,
    bd: i32,
) {
    let mut blimit: __m128i = core::mem::zeroed();
    let mut limit: __m128i = core::mem::zeroed();
    let mut thresh: __m128i = core::mem::zeroed();
    let mut t80: __m128i = core::mem::zeroed();
    let zero: __m128i = _mm_setzero_si128();
    get_limit_dual(
        blt0,
        lt0,
        thr0,
        blt1,
        lt1,
        thr1,
        bd,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
        &raw mut t80,
    );
    let mut mask: __m128i = core::mem::zeroed();
    highbd_filter_mask_dual(
        p as *const __m128i,
        q as *const __m128i,
        &raw mut limit as *const __m128i,
        &raw mut blimit as *const __m128i,
        &raw mut mask,
    );
    let mut flat: __m128i = core::mem::zeroed();
    let mut flat2: __m128i = core::mem::zeroed();
    highbd_flat_mask4_dual_sse2(
        p as *const __m128i,
        q as *const __m128i,
        &raw mut flat,
        &raw mut flat2,
        bd,
    );
    flat = _mm_and_si128(flat, mask);
    flat2 = _mm_and_si128(flat2, flat);
    let mut ps: [__m128i; 2] = core::mem::zeroed();
    let mut qs: [__m128i; 2] = core::mem::zeroed();
    highbd_filter4_dual_sse2(
        p,
        q,
        ps.as_mut_ptr(),
        qs.as_mut_ptr(),
        &raw mut mask as *const __m128i,
        &raw mut thresh as *const __m128i,
        bd,
        &raw mut t80,
    );
    // flat and wide flat calculations
    // if flat ==0 then flat2 is zero as well and we don't need any calc below
    // sse4.1 if (0==_mm_test_all_zeros(flat,ff))
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat, zero)) {
        let mut flat_p: [__m128i; 3] = core::mem::zeroed();
        let mut flat_q: [__m128i; 3] = core::mem::zeroed();
        let mut flat2_p: [__m128i; 6] = core::mem::zeroed();
        let mut flat2_q: [__m128i; 6] = core::mem::zeroed();
        let eight: __m128i = _mm_set1_epi16(8);
        let four: __m128i = _mm_set1_epi16(4);
        let mut sum_p_0: __m128i =
            _mm_add_epi16(*p.offset(5), _mm_add_epi16(*p.offset(4), *p.offset(3)));
        let mut sum_q: __m128i =
            _mm_add_epi16(*q.offset(5), _mm_add_epi16(*q.offset(4), *q.offset(3)));
        let mut sum_lp: __m128i = _mm_add_epi16(*p, _mm_add_epi16(*p.offset(2), *p.offset(1)));
        sum_p_0 = _mm_add_epi16(sum_p_0, sum_lp);
        let mut sum_lq: __m128i = _mm_add_epi16(*q, _mm_add_epi16(*q.offset(2), *q.offset(1)));
        sum_q = _mm_add_epi16(sum_q, sum_lq);
        sum_p_0 = _mm_add_epi16(eight, _mm_add_epi16(sum_p_0, sum_q));
        sum_lp = _mm_add_epi16(four, _mm_add_epi16(sum_lp, sum_lq));
        flat_p[0] = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(sum_lp, _mm_add_epi16(*p.offset(3), *p)));
        flat_q[0] = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(sum_lp, _mm_add_epi16(*q.offset(3), *q)));
        let mut sum_p6: __m128i = _mm_add_epi16(*p.offset(6), *p.offset(6));
        let mut sum_q6: __m128i = _mm_add_epi16(*q.offset(6), *q.offset(6));
        let mut sum_p3: __m128i = _mm_add_epi16(*p.offset(3), *p.offset(3));
        let mut sum_q3: __m128i = _mm_add_epi16(*q.offset(3), *q.offset(3));
        sum_q = _mm_sub_epi16(sum_p_0, *p.offset(5));
        let mut sum_p: __m128i = _mm_sub_epi16(sum_p_0, *q.offset(5));
        sum_lq = _mm_sub_epi16(sum_lp, *p.offset(2));
        sum_lp = _mm_sub_epi16(sum_lp, *q.offset(2));
        flat_p[1] =
            _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(sum_lp, _mm_add_epi16(sum_p3, *p.offset(1))));
        flat_q[1] =
            _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(sum_lq, _mm_add_epi16(sum_q3, *q.offset(1))));
        sum_lp = _mm_sub_epi16(sum_lp, *q.offset(1));
        sum_lq = _mm_sub_epi16(sum_lq, *p.offset(1));
        sum_p3 = _mm_add_epi16(sum_p3, *p.offset(3));
        sum_q3 = _mm_add_epi16(sum_q3, *q.offset(3));
        flat_p[2] =
            _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(sum_lp, _mm_add_epi16(sum_p3, *p.offset(2))));
        flat_q[2] =
            _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(sum_lq, _mm_add_epi16(sum_q3, *q.offset(2))));
        let mut flat2_mask: i32 =
            (0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat2, zero))) as i32;
        if flat2_mask != 0 {
            flat2_p[0] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p_0,
                _mm_add_epi16(
                    _mm_add_epi16(*p.offset(6), *p),
                    _mm_add_epi16(*p.offset(1), *q),
                ),
            ));
            flat2_q[0] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p_0,
                _mm_add_epi16(
                    _mm_add_epi16(*q.offset(6), *q),
                    _mm_add_epi16(*p, *q.offset(1)),
                ),
            ));
            flat2_p[1] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p,
                _mm_add_epi16(
                    sum_p6,
                    _mm_add_epi16(*p.offset(1), _mm_add_epi16(*p.offset(2), *p)),
                ),
            ));
            flat2_q[1] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_q,
                _mm_add_epi16(
                    sum_q6,
                    _mm_add_epi16(*q.offset(1), _mm_add_epi16(*q, *q.offset(2))),
                ),
            ));
            sum_p6 = _mm_add_epi16(sum_p6, *p.offset(6));
            sum_q6 = _mm_add_epi16(sum_q6, *q.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(4));
            sum_q = _mm_sub_epi16(sum_q, *p.offset(4));
            flat2_p[2] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p,
                _mm_add_epi16(
                    sum_p6,
                    _mm_add_epi16(*p.offset(2), _mm_add_epi16(*p.offset(3), *p.offset(1))),
                ),
            ));
            flat2_q[2] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_q,
                _mm_add_epi16(
                    sum_q6,
                    _mm_add_epi16(*q.offset(2), _mm_add_epi16(*q.offset(1), *q.offset(3))),
                ),
            ));
            sum_p6 = _mm_add_epi16(sum_p6, *p.offset(6));
            sum_q6 = _mm_add_epi16(sum_q6, *q.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(3));
            sum_q = _mm_sub_epi16(sum_q, *p.offset(3));
            flat2_p[3] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p,
                _mm_add_epi16(
                    sum_p6,
                    _mm_add_epi16(*p.offset(3), _mm_add_epi16(*p.offset(4), *p.offset(2))),
                ),
            ));
            flat2_q[3] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_q,
                _mm_add_epi16(
                    sum_q6,
                    _mm_add_epi16(*q.offset(3), _mm_add_epi16(*q.offset(2), *q.offset(4))),
                ),
            ));
            sum_p6 = _mm_add_epi16(sum_p6, *p.offset(6));
            sum_q6 = _mm_add_epi16(sum_q6, *q.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(2));
            sum_q = _mm_sub_epi16(sum_q, *p.offset(2));
            flat2_p[4] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p,
                _mm_add_epi16(
                    sum_p6,
                    _mm_add_epi16(*p.offset(4), _mm_add_epi16(*p.offset(5), *p.offset(3))),
                ),
            ));
            flat2_q[4] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_q,
                _mm_add_epi16(
                    sum_q6,
                    _mm_add_epi16(*q.offset(4), _mm_add_epi16(*q.offset(3), *q.offset(5))),
                ),
            ));
            sum_p6 = _mm_add_epi16(sum_p6, *p.offset(6));
            sum_q6 = _mm_add_epi16(sum_q6, *q.offset(6));
            sum_p = _mm_sub_epi16(sum_p, *q.offset(1));
            sum_q = _mm_sub_epi16(sum_q, *p.offset(1));
            flat2_p[5] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_p,
                _mm_add_epi16(
                    sum_p6,
                    _mm_add_epi16(*p.offset(5), _mm_add_epi16(*p.offset(6), *p.offset(4))),
                ),
            ));
            flat2_q[5] = _mm_srli_epi16::<{ 4 }>(_mm_add_epi16(
                sum_q,
                _mm_add_epi16(
                    sum_q6,
                    _mm_add_epi16(*q.offset(5), _mm_add_epi16(*q.offset(4), *q.offset(6))),
                ),
            ));
        }
        // highbd_filter8
        let mut i: i32 = 0;
        {
            i = 0;
            while i < 2 {
                ps[i as usize] = _mm_andnot_si128(flat, ps[i as usize]);
                flat_p[i as usize] = _mm_and_si128(flat, flat_p[i as usize]);
                *p.offset(i as isize) = _mm_or_si128(ps[i as usize], flat_p[i as usize]);
                qs[i as usize] = _mm_andnot_si128(flat, qs[i as usize]);
                flat_q[i as usize] = _mm_and_si128(flat, flat_q[i as usize]);
                *q.offset(i as isize) = _mm_or_si128(qs[i as usize], flat_q[i as usize]);
                i += 1;
            }
        }
        *p.offset(2) = _mm_andnot_si128(flat, *p.offset(2));
        //  p2 remains unchanged if !(flat && mask)
        flat_p[2] = _mm_and_si128(flat, flat_p[2]);
        //  when (flat && mask)
        *p.offset(2) = _mm_or_si128(*p.offset(2), flat_p[2]); // full list of p2 values
        *q.offset(2) = _mm_andnot_si128(flat, *q.offset(2));
        flat_q[2] = _mm_and_si128(flat, flat_q[2]);
        *q.offset(2) = _mm_or_si128(*q.offset(2), flat_q[2]); // full list of q2 values
        {
            i = 0;
            while i < 2 {
                ps[i as usize] = _mm_andnot_si128(flat, ps[i as usize]);
                flat_p[i as usize] = _mm_and_si128(flat, flat_p[i as usize]);
                *p.offset(i as isize) = _mm_or_si128(ps[i as usize], flat_p[i as usize]);
                qs[i as usize] = _mm_andnot_si128(flat, qs[i as usize]);
                flat_q[i as usize] = _mm_and_si128(flat, flat_q[i as usize]);
                *q.offset(i as isize) = _mm_or_si128(qs[i as usize], flat_q[i as usize]);
                i += 1;
            }
        }
        // highbd_filter16
        if flat2_mask != 0 {
            {
                i = 0;
                while i < 6 {
                    //  p[i] remains unchanged if !(flat2 && flat && mask)
                    *p.offset(i as isize) = _mm_andnot_si128(flat2, *p.offset(i as isize));
                    flat2_p[i as usize] = _mm_and_si128(flat2, flat2_p[i as usize]);
                    //  get values for when (flat2 && flat && mask)
                    *p.offset(i as isize) =
                        _mm_or_si128(*p.offset(i as isize), flat2_p[i as usize]); // full list of p values
                    *q.offset(i as isize) = _mm_andnot_si128(flat2, *q.offset(i as isize));
                    flat2_q[i as usize] = _mm_and_si128(flat2, flat2_q[i as usize]);
                    *q.offset(i as isize) =
                        _mm_or_si128(*q.offset(i as isize), flat2_q[i as usize]);
                    i += 1;
                }
            }
        }
    } else {
        *p = ps[0];
        *q = qs[0];
        *p.offset(1) = ps[1];
        *q.offset(1) = qs[1];
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_14_dual_sse2(
    s: *mut u16,
    pitch: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut p: [__m128i; 7] = core::mem::zeroed();
    let mut q: [__m128i; 7] = core::mem::zeroed();
    let mut i: i32 = 0;
    load_highbd_pixel(s as *const u16, 7, pitch, p.as_mut_ptr(), q.as_mut_ptr());
    highbd_lpf_internal_14_dual_sse2(
        p.as_mut_ptr(),
        q.as_mut_ptr(),
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
    );
    {
        i = 0;
        while i < 6 {
            _mm_storeu_si128(
                s.offset(-(((i + 1) * pitch) as isize)) as *mut __m128i,
                p[i as usize],
            );
            _mm_storeu_si128(
                s.offset((i * pitch) as isize) as *mut __m128i,
                q[i as usize],
            );
            i += 1;
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_6_sse2(
    p2: *mut __m128i,
    p1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1: *mut __m128i,
    q2: *mut __m128i,
    p1p0_out: *mut __m128i,
    q1q0_out: *mut __m128i,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
    bd: i32,
) {
    let mut blimit: __m128i = core::mem::zeroed();
    let mut limit: __m128i = core::mem::zeroed();
    let mut thresh: __m128i = core::mem::zeroed();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut pq: [__m128i; 3] = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut abs_p1p0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i;
    let mut qs1qs0: __m128i;
    let mut flat_p1p0: __m128i;
    let mut flat_q0q1: __m128i;
    pq[0] = _mm_unpacklo_epi64(*p0, *q0);
    pq[1] = _mm_unpacklo_epi64(*p1, *q1);
    pq[2] = _mm_unpacklo_epi64(*p2, *q2);
    let zero: __m128i = _mm_setzero_si128();
    let four: __m128i = _mm_set1_epi16(4);
    let mut t80: __m128i = core::mem::zeroed();
    let one: __m128i = _mm_set1_epi16(0x1);
    get_limit(
        _blimit,
        _limit,
        _thresh,
        bd,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
        &raw mut t80,
    );
    highbd_hev_filter_mask_x_sse2(
        pq.as_mut_ptr(),
        3,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut abs_p1p0,
        &raw mut limit,
        &raw mut blimit,
        &raw mut thresh,
        &raw mut hev,
        &raw mut mask,
    );
    // lp filter
    highbd_filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        q1q0_out,
        p1p0_out,
        &raw mut t80,
        bd,
    );
    // flat_mask
    flat = _mm_max_epi16(abs_diff16(pq[2], pq[0]), abs_p1p0);
    flat = _mm_max_epi16(flat, _mm_srli_si128::<8>(flat));
    flat = _mm_subs_epu16(flat, _mm_sll_epi16(one, _mm_cvtsi32_si128(bd - 8)));
    flat = _mm_cmpeq_epi16(flat, zero);
    flat = _mm_and_si128(flat, mask);
    // replicate for the further "merged variables" usage
    flat = _mm_unpacklo_epi64(flat, flat);
    // 5 tap filter
    // need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat, zero)) {
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_c: __m128i;
        let mut pq0x2_pq1: __m128i;
        let mut pq1_pq2: __m128i;
        // op1
        pq0x2_pq1 = _mm_add_epi16(_mm_add_epi16(pq[0], pq[0]), pq[1]); // p0 *2 + p1
        pq1_pq2 = _mm_add_epi16(pq[1], pq[2]); // p1 + p2
        workp_a = _mm_add_epi16(_mm_add_epi16(pq0x2_pq1, four), pq1_pq2); // p2 + p0 * 2 + p1 * 2 + 4
        workp_b = _mm_add_epi16(_mm_add_epi16(pq[2], pq[2]), *q0);
        workp_b = _mm_add_epi16(workp_a, workp_b); // p2 * 3 + p1 * 2 + p0 * 2 + q0 + 4
                                                   // op0
        workp_c = _mm_srli_si128::<8>(pq0x2_pq1); // q0 * 2 + q1
        workp_a = _mm_add_epi16(workp_a, workp_c); // p2 + p0 * 2 + p1 * 2 + q0 * 2 + q1 + 4
        workp_b = _mm_unpacklo_epi64(workp_a, workp_b);
        flat_p1p0 = _mm_srli_epi16::<{ 3 }>(workp_b);
        // oq0
        workp_a = _mm_sub_epi16(_mm_sub_epi16(workp_a, pq[2]), pq[1]); // p0 * 2 + p1  + q0 * 2 + q1 + 4
        workp_b = _mm_srli_si128::<8>(pq1_pq2);
        workp_a = _mm_add_epi16(workp_a, workp_b); // p0 * 2 + p1  + q0 * 2 + q1 * 2 + q2 + 4
                                                   // workp_shft0 = _mm_srli_epi16(workp_a, 3);
                                                   // oq1
        workp_c = _mm_sub_epi16(_mm_sub_epi16(workp_a, pq[1]), pq[0]); // p0   + q0 * 2 + q1 * 2 + q2 + 4
        workp_b = _mm_add_epi16(*q2, *q2);
        workp_b = _mm_add_epi16(workp_c, workp_b); // p0  + q0 * 2 + q1 * 2 + q2 * 3 + 4
        workp_a = _mm_unpacklo_epi64(workp_a, workp_b);
        flat_q0q1 = _mm_srli_epi16::<{ 3 }>(workp_a);
        qs1qs0 = _mm_andnot_si128(flat, *q1q0_out);
        q1q0 = _mm_and_si128(flat, flat_q0q1);
        *q1q0_out = _mm_or_si128(qs1qs0, q1q0);
        ps1ps0 = _mm_andnot_si128(flat, *p1p0_out);
        p1p0 = _mm_and_si128(flat, flat_p1p0);
        *p1p0_out = _mm_or_si128(ps1ps0, p1p0);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_6_dual_sse2(
    p2: *mut __m128i,
    p1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1: *mut __m128i,
    q2: *mut __m128i,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut blimit0: __m128i = core::mem::zeroed();
    let mut limit0: __m128i = core::mem::zeroed();
    let mut thresh0: __m128i = core::mem::zeroed();
    let mut t80: __m128i = core::mem::zeroed();
    let mut mask: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut work: __m128i;
    let mut abs_p1q1: __m128i;
    let mut abs_p0q0: __m128i;
    let mut abs_p1p0: __m128i;
    let mut abs_p2p1: __m128i;
    let mut abs_q1q0: __m128i;
    let mut abs_q2q1: __m128i;
    let mut op1: __m128i;
    let mut op0: __m128i;
    let mut oq0: __m128i;
    let mut oq1: __m128i;
    let four: __m128i = _mm_set1_epi16(4);
    let one: __m128i = _mm_set1_epi16(0x1);
    let ffff: __m128i = _mm_cmpeq_epi16(one, one);
    get_limit_dual(
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
        &raw mut blimit0,
        &raw mut limit0,
        &raw mut thresh0,
        &raw mut t80,
    );
    abs_p2p1 = abs_diff16(*p2, *p1);
    abs_p1p0 = abs_diff16(*p1, *p0);
    abs_q1q0 = abs_diff16(*q1, *q0);
    abs_q2q1 = abs_diff16(*q2, *q1);
    abs_p0q0 = abs_diff16(*p0, *q0);
    abs_p1q1 = abs_diff16(*p1, *q1);
    abs_p0q0 = _mm_adds_epu16(abs_p0q0, abs_p0q0);
    abs_p1q1 = _mm_srli_epi16::<{ 1 }>(abs_p1q1);
    mask = _mm_subs_epu16(_mm_adds_epu16(abs_p0q0, abs_p1q1), blimit0);
    mask = _mm_xor_si128(_mm_cmpeq_epi16(mask, zero), ffff);
    // mask |= (abs(*p0 - *q0) * 2 + abs(*p1 - *q1) / 2  > blimit) * -1;
    // So taking maximums continues to work:
    mask = _mm_and_si128(mask, _mm_adds_epu16(limit0, one));
    mask = _mm_max_epi16(abs_q2q1, mask);
    work = _mm_max_epi16(abs_p1p0, abs_q1q0);
    mask = _mm_max_epi16(work, mask);
    mask = _mm_max_epi16(mask, abs_p2p1);
    mask = _mm_subs_epu16(mask, limit0);
    mask = _mm_cmpeq_epi16(mask, zero);
    // lp filter
    let mut ps: [__m128i; 2] = core::mem::zeroed();
    let mut qs: [__m128i; 2] = core::mem::zeroed();
    let mut p: [__m128i; 2] = core::mem::zeroed();
    let mut q: [__m128i; 2] = core::mem::zeroed();
    {
        p[0] = *p0;
        p[1] = *p1;
        q[0] = *q0;
        q[1] = *q1;
        // filter_mask and hev_mask
        highbd_filter4_dual_sse2(
            p.as_mut_ptr(),
            q.as_mut_ptr(),
            ps.as_mut_ptr(),
            qs.as_mut_ptr(),
            &raw mut mask as *const __m128i,
            &raw mut thresh0 as *const __m128i,
            bd,
            &raw mut t80,
        );
    }
    // flat_mask
    flat = _mm_max_epi16(abs_diff16(*q2, *q0), abs_diff16(*p2, *p0));
    flat = _mm_max_epi16(flat, work);
    flat = _mm_subs_epu16(flat, _mm_sll_epi16(one, _mm_cvtsi32_si128(bd - 8)));
    flat = _mm_cmpeq_epi16(flat, zero);
    flat = _mm_and_si128(flat, mask); // flat & mask
                                      // 5 tap filter
                                      // need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat, zero)) {
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_shft0: __m128i;
        let mut workp_shft1: __m128i;
        // op1
        workp_a = _mm_add_epi16(_mm_add_epi16(*p0, *p0), _mm_add_epi16(*p1, *p1)); // *p0 *2 + *p1 * 2
        workp_a = _mm_add_epi16(_mm_add_epi16(workp_a, four), *p2); // *p2 + *p0 * 2 + *p1 * 2 + 4
        workp_b = _mm_add_epi16(_mm_add_epi16(*p2, *p2), *q0);
        workp_shft0 = _mm_add_epi16(workp_a, workp_b); // *p2 * 3 + *p1 * 2 + *p0 * 2 + *q0 + 4
        op1 = _mm_srli_epi16::<{ 3 }>(workp_shft0);
        // op0
        workp_b = _mm_add_epi16(_mm_add_epi16(*q0, *q0), *q1); // *q0 * 2 + *q1
        workp_a = _mm_add_epi16(workp_a, workp_b); // *p2 + *p0 * 2 + *p1 * 2 + *q0 * 2 + *q1 + 4
        op0 = _mm_srli_epi16::<{ 3 }>(workp_a);
        // oq0
        workp_a = _mm_sub_epi16(_mm_sub_epi16(workp_a, *p2), *p1); // *p0 * 2 + *p1  + *q0 * 2 + *q1 + 4
        workp_b = _mm_add_epi16(*q1, *q2);
        workp_shft0 = _mm_add_epi16(workp_a, workp_b); // *p0 * 2 + *p1  + *q0 * 2 + *q1 * 2 + *q2 + 4
        oq0 = _mm_srli_epi16::<{ 3 }>(workp_shft0);
        // oq1
        workp_a = _mm_sub_epi16(_mm_sub_epi16(workp_shft0, *p1), *p0); // *p0   + *q0 * 2 + *q1 * 2 + *q2 + 4
        workp_b = _mm_add_epi16(*q2, *q2);
        workp_shft1 = _mm_add_epi16(workp_a, workp_b); // *p0  + *q0 * 2 + *q1 * 2 + *q2 * 3 + 4
        oq1 = _mm_srli_epi16::<{ 3 }>(workp_shft1);
        qs[0] = _mm_andnot_si128(flat, qs[0]);
        oq0 = _mm_and_si128(flat, oq0);
        *q0 = _mm_or_si128(qs[0], oq0);
        qs[1] = _mm_andnot_si128(flat, qs[1]);
        oq1 = _mm_and_si128(flat, oq1);
        *q1 = _mm_or_si128(qs[1], oq1);
        ps[0] = _mm_andnot_si128(flat, ps[0]);
        op0 = _mm_and_si128(flat, op0);
        *p0 = _mm_or_si128(ps[0], op0);
        ps[1] = _mm_andnot_si128(flat, ps[1]);
        op1 = _mm_and_si128(flat, op1);
        *p1 = _mm_or_si128(ps[1], op1);
    } else {
        *q0 = qs[0];
        *q1 = qs[1];
        *p0 = ps[0];
        *p1 = ps[1];
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_6_sse2(
    s: *mut u16,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
    bd: i32,
) {
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut p1p0_out: __m128i = core::mem::zeroed();
    let mut q1q0_out: __m128i = core::mem::zeroed();
    p2 = _mm_loadl_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadl_epi64(s.offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    q1 = _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadl_epi64(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_lpf_internal_6_sse2(
        &raw mut p2,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut q2,
        &raw mut p1p0_out,
        &raw mut q1q0_out,
        _blimit,
        _limit,
        _thresh,
        bd,
    );
    _mm_storel_epi64(
        s.offset(-((2 * p) as isize)) as *mut __m128i,
        _mm_srli_si128::<8>(p1p0_out),
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, p1p0_out);
    _mm_storel_epi64(s.offset((0 * p) as isize) as *mut __m128i, q1q0_out);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q1q0_out),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_6_dual_sse2(
    s: *mut u16,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    p2 = _mm_loadu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    p1 = _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadu_si128(s.offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    q1 = _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    q2 = _mm_loadu_si128(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_lpf_internal_6_dual_sse2(
        &raw mut p2,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut q2,
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
    );
    _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, p1);
    _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, p0);
    _mm_storeu_si128(s.offset((0 * p) as isize) as *mut __m128i, q0);
    _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, q1);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_8_sse2(
    p3: *mut __m128i,
    q3: *mut __m128i,
    p2: *mut __m128i,
    q2: *mut __m128i,
    p1: *mut __m128i,
    q1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1q0_out: *mut __m128i,
    p1p0_out: *mut __m128i,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
    bd: i32,
) {
    let zero: __m128i = _mm_setzero_si128();
    let mut blimit: __m128i = core::mem::zeroed();
    let mut limit: __m128i = core::mem::zeroed();
    let mut thresh: __m128i = core::mem::zeroed();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut pq: [__m128i; 4] = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut ps1ps0: __m128i;
    let mut qs1qs0: __m128i;
    let mut work_a: __m128i;
    let mut opq2: __m128i;
    let mut flat_p1p0: __m128i;
    let mut flat_q0q1: __m128i;
    pq[0] = _mm_unpacklo_epi64(*p0, *q0);
    pq[1] = _mm_unpacklo_epi64(*p1, *q1);
    pq[2] = _mm_unpacklo_epi64(*p2, *q2);
    pq[3] = _mm_unpacklo_epi64(*p3, *q3);
    let mut abs_p1p0: __m128i = core::mem::zeroed();
    let four: __m128i = _mm_set1_epi16(4);
    let mut t80: __m128i = core::mem::zeroed();
    let one: __m128i = _mm_set1_epi16(0x1);
    get_limit(
        _blimit,
        _limit,
        _thresh,
        bd,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
        &raw mut t80,
    );
    highbd_hev_filter_mask_x_sse2(
        pq.as_mut_ptr(),
        4,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut abs_p1p0,
        &raw mut limit,
        &raw mut blimit,
        &raw mut thresh,
        &raw mut hev,
        &raw mut mask,
    );
    // lp filter
    highbd_filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        q1q0_out,
        p1p0_out,
        &raw mut t80,
        bd,
    );
    // flat_mask4
    flat = _mm_max_epi16(abs_diff16(pq[2], pq[0]), abs_diff16(pq[3], pq[0]));
    flat = _mm_max_epi16(abs_p1p0, flat);
    flat = _mm_max_epi16(flat, _mm_srli_si128::<8>(flat));
    flat = _mm_subs_epu16(flat, _mm_sll_epi16(one, _mm_cvtsi32_si128(bd - 8)));
    flat = _mm_cmpeq_epi16(flat, zero);
    flat = _mm_and_si128(flat, mask);
    // replicate for the further "merged variables" usage
    flat = _mm_unpacklo_epi64(flat, flat);
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat, zero)) {
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        let mut workp_c: __m128i;
        let mut workp_shft0: __m128i;
        let mut workp_shft1: __m128i;
        // Added before shift for rounding part of ROUND_POWER_OF_TWO
        // o*p2
        workp_a = _mm_add_epi16(_mm_add_epi16(*p3, *p3), _mm_add_epi16(*p2, *p1));
        workp_a = _mm_add_epi16(_mm_add_epi16(workp_a, four), *p0);
        workp_c = _mm_add_epi16(_mm_add_epi16(*q0, *p2), *p3);
        workp_c = _mm_add_epi16(workp_a, workp_c);
        // o*p1
        workp_b = _mm_add_epi16(_mm_add_epi16(*q0, *q1), *p1);
        workp_shft0 = _mm_add_epi16(workp_a, workp_b);
        // o*p0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p3), *q2);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *p1), *p0);
        workp_shft1 = _mm_add_epi16(workp_a, workp_b);
        flat_p1p0 = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(workp_shft1, workp_shft0));
        // oq0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p3), *q3);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *p0), *q0);
        workp_shft0 = _mm_add_epi16(workp_a, workp_b);
        // oq1
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p2), *q3);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *q0), *q1);
        workp_shft1 = _mm_add_epi16(workp_a, workp_b);
        flat_q0q1 = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(workp_shft0, workp_shft1));
        // oq2
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p1), *q3);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *q1), *q2);
        workp_a = _mm_add_epi16(workp_a, workp_b);
        opq2 = _mm_srli_epi16::<{ 3 }>(_mm_unpacklo_epi64(workp_c, workp_a));
        qs1qs0 = _mm_andnot_si128(flat, *q1q0_out);
        q1q0 = _mm_and_si128(flat, flat_q0q1);
        *q1q0_out = _mm_or_si128(qs1qs0, q1q0);
        ps1ps0 = _mm_andnot_si128(flat, *p1p0_out);
        p1p0 = _mm_and_si128(flat, flat_p1p0);
        *p1p0_out = _mm_or_si128(ps1ps0, p1p0);
        work_a = _mm_andnot_si128(flat, pq[2]);
        *p2 = _mm_and_si128(flat, opq2);
        *p2 = _mm_or_si128(work_a, *p2);
        *q2 = _mm_srli_si128::<8>(*p2);
    }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_8_dual_sse2(
    p3: *mut __m128i,
    q3: *mut __m128i,
    p2: *mut __m128i,
    q2: *mut __m128i,
    p1: *mut __m128i,
    q1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut blimit0: __m128i = core::mem::zeroed();
    let mut limit0: __m128i = core::mem::zeroed();
    let mut thresh0: __m128i = core::mem::zeroed();
    let mut t80: __m128i = core::mem::zeroed();
    let mut mask: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut work_a: __m128i;
    let mut op2: __m128i;
    let mut oq2: __m128i;
    let mut op1: __m128i;
    let mut op0: __m128i;
    let mut oq0: __m128i;
    let mut oq1: __m128i;
    let mut abs_p1q1: __m128i;
    let mut abs_p0q0: __m128i;
    let mut work0: __m128i;
    let mut work1: __m128i;
    let mut work2: __m128i;
    let zero: __m128i = _mm_setzero_si128();
    let four: __m128i = _mm_set1_epi16(4);
    let one: __m128i = _mm_set1_epi16(0x1);
    let ffff: __m128i = _mm_cmpeq_epi16(one, one);
    get_limit_dual(
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
        &raw mut blimit0,
        &raw mut limit0,
        &raw mut thresh0,
        &raw mut t80,
    );
    abs_p0q0 = abs_diff16(*p0, *q0);
    abs_p1q1 = abs_diff16(*p1, *q1);
    abs_p0q0 = _mm_adds_epu16(abs_p0q0, abs_p0q0);
    abs_p1q1 = _mm_srli_epi16::<{ 1 }>(abs_p1q1);
    mask = _mm_subs_epu16(_mm_adds_epu16(abs_p0q0, abs_p1q1), blimit0);
    mask = _mm_xor_si128(_mm_cmpeq_epi16(mask, zero), ffff);
    // mask |= (abs(*p0 - q0) * 2 + abs(*p1 - q1) / 2  > blimit) * -1;
    // So taking maximums continues to work:
    mask = _mm_and_si128(mask, _mm_adds_epu16(limit0, one));
    work0 = _mm_max_epi16(abs_diff16(*p3, *p2), abs_diff16(*p2, *p1));
    work1 = _mm_max_epi16(abs_diff16(*p1, *p0), abs_diff16(*q1, *q0)); // tbu 4 flat
    work0 = _mm_max_epi16(work0, work1);
    work2 = _mm_max_epi16(abs_diff16(*q2, *q1), abs_diff16(*q2, *q3));
    work2 = _mm_max_epi16(work2, work0);
    mask = _mm_max_epi16(work2, mask);
    mask = _mm_subs_epu16(mask, limit0);
    mask = _mm_cmpeq_epi16(mask, zero);
    // lp filter
    let mut ps: [__m128i; 2] = core::mem::zeroed();
    let mut qs: [__m128i; 2] = core::mem::zeroed();
    let mut p: [__m128i; 2] = core::mem::zeroed();
    let mut q: [__m128i; 2] = core::mem::zeroed();
    {
        p[0] = *p0;
        p[1] = *p1;
        q[0] = *q0;
        q[1] = *q1;
        // filter_mask and hev_mask
        highbd_filter4_dual_sse2(
            p.as_mut_ptr(),
            q.as_mut_ptr(),
            ps.as_mut_ptr(),
            qs.as_mut_ptr(),
            &raw mut mask as *const __m128i,
            &raw mut thresh0 as *const __m128i,
            bd,
            &raw mut t80,
        );
    }
    flat = _mm_max_epi16(abs_diff16(*p2, *p0), abs_diff16(*q2, *q0));
    flat = _mm_max_epi16(work1, flat);
    work0 = _mm_max_epi16(abs_diff16(*p3, *p0), abs_diff16(*q3, *q0));
    flat = _mm_max_epi16(work0, flat);
    flat = _mm_subs_epu16(flat, _mm_sll_epi16(one, _mm_cvtsi32_si128(bd - 8)));
    flat = _mm_cmpeq_epi16(flat, zero);
    flat = _mm_and_si128(flat, mask); // flat & mask
                                      // filter8 need it only if flat !=0
    if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi16(flat, zero)) {
        let mut workp_a: __m128i;
        let mut workp_b: __m128i;
        // Added before shift for rounding part of ROUND_POWER_OF_TWO
        // o*p2
        workp_a = _mm_add_epi16(_mm_add_epi16(*p3, *p3), _mm_add_epi16(*p2, *p1));
        workp_a = _mm_add_epi16(_mm_add_epi16(workp_a, four), *p0);
        workp_b = _mm_add_epi16(_mm_add_epi16(*q0, *p2), *p3);
        op2 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // o*p1
        workp_b = _mm_add_epi16(_mm_add_epi16(*q0, *q1), *p1);
        op1 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // o*p0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p3), *q2);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *p1), *p0);
        op0 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // oq0
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p3), *q3);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *p0), *q0);
        oq0 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // oq1
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p2), *q3);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *q0), *q1);
        oq1 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        // oq2
        workp_a = _mm_add_epi16(_mm_sub_epi16(workp_a, *p1), *q3);
        workp_b = _mm_add_epi16(_mm_sub_epi16(workp_b, *q1), *q2);
        oq2 = _mm_srli_epi16::<{ 3 }>(_mm_add_epi16(workp_a, workp_b));
        qs[0] = _mm_andnot_si128(flat, qs[0]);
        oq0 = _mm_and_si128(flat, oq0);
        *q0 = _mm_or_si128(qs[0], oq0);
        qs[1] = _mm_andnot_si128(flat, qs[1]);
        oq1 = _mm_and_si128(flat, oq1);
        *q1 = _mm_or_si128(qs[1], oq1);
        ps[0] = _mm_andnot_si128(flat, ps[0]);
        op0 = _mm_and_si128(flat, op0);
        *p0 = _mm_or_si128(ps[0], op0);
        ps[1] = _mm_andnot_si128(flat, ps[1]);
        op1 = _mm_and_si128(flat, op1);
        *p1 = _mm_or_si128(ps[1], op1);
        work_a = _mm_andnot_si128(flat, *q2);
        *q2 = _mm_and_si128(flat, oq2);
        *q2 = _mm_or_si128(work_a, *q2);
        work_a = _mm_andnot_si128(flat, *p2);
        *p2 = _mm_and_si128(flat, op2);
        *p2 = _mm_or_si128(work_a, *p2);
    } else {
        *q0 = qs[0];
        *q1 = qs[1];
        *p0 = ps[0];
        *p1 = ps[1];
    }
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_sse2(
    s: *mut u16,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
    bd: i32,
) {
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut q3: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    p3 = _mm_loadl_epi64(s.offset(-((4 * p) as isize)) as *mut __m128i as *const __m128i);
    q3 = _mm_loadl_epi64(s.offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    p2 = _mm_loadl_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    q2 = _mm_loadl_epi64(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    p1 = _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    p0 = _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadl_epi64(s.offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_lpf_internal_8_sse2(
        &raw mut p3,
        &raw mut q3,
        &raw mut p2,
        &raw mut q2,
        &raw mut p1,
        &raw mut q1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1q0,
        &raw mut p1p0,
        _blimit,
        _limit,
        _thresh,
        bd,
    );
    _mm_storel_epi64(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
    _mm_storel_epi64(
        s.offset(-((2 * p) as isize)) as *mut __m128i,
        _mm_srli_si128::<8>(p1p0),
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, p1p0);
    _mm_storel_epi64(s.offset((0 * p) as isize) as *mut __m128i, q1q0);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q1q0),
    );
    _mm_storel_epi64(s.offset((2 * p) as isize) as *mut __m128i, q2);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_8_dual_sse2(
    s: *mut u16,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut q3: __m128i = core::mem::zeroed();
    p3 = _mm_loadu_si128(s.offset(-((4 * p) as isize)) as *mut __m128i as *const __m128i);
    q3 = _mm_loadu_si128(s.offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    p2 = _mm_loadu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i as *const __m128i);
    q2 = _mm_loadu_si128(s.offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    p1 = _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    q1 = _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    q0 = _mm_loadu_si128(s.offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_lpf_internal_8_dual_sse2(
        &raw mut p3,
        &raw mut q3,
        &raw mut p2,
        &raw mut q2,
        &raw mut p1,
        &raw mut q1,
        &raw mut p0,
        &raw mut q0,
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
    );
    _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
    _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, p1);
    _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, p0);
    _mm_storeu_si128(s.offset((0 * p) as isize) as *mut __m128i, q0);
    _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, q1);
    _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, q2);
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_4_sse2(
    p1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1: *mut __m128i,
    q1q0_out: *mut __m128i,
    p1p0_out: *mut __m128i,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
    bd: i32,
) {
    let mut blimit: __m128i = core::mem::zeroed();
    let mut limit: __m128i = core::mem::zeroed();
    let mut thresh: __m128i = core::mem::zeroed();
    let mut mask: __m128i = core::mem::zeroed();
    let mut hev: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut pq: [__m128i; 2] = core::mem::zeroed();
    let mut abs_p1p0: __m128i = core::mem::zeroed();
    let mut t80: __m128i = core::mem::zeroed();
    get_limit(
        _blimit,
        _limit,
        _thresh,
        bd,
        &raw mut blimit,
        &raw mut limit,
        &raw mut thresh,
        &raw mut t80,
    );
    pq[0] = _mm_unpacklo_epi64(*p0, *q0);
    pq[1] = _mm_unpacklo_epi64(*p1, *q1);
    highbd_hev_filter_mask_x_sse2(
        pq.as_mut_ptr(),
        2,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut abs_p1p0,
        &raw mut limit,
        &raw mut blimit,
        &raw mut thresh,
        &raw mut hev,
        &raw mut mask,
    );
    highbd_filter4_sse2(
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut hev,
        &raw mut mask,
        q1q0_out,
        p1p0_out,
        &raw mut t80,
        bd,
    );
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn highbd_lpf_internal_4_dual_sse2(
    p1: *mut __m128i,
    p0: *mut __m128i,
    q0: *mut __m128i,
    q1: *mut __m128i,
    ps: *mut __m128i,
    qs: *mut __m128i,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut blimit0: __m128i = core::mem::zeroed();
    let mut limit0: __m128i = core::mem::zeroed();
    let mut thresh0: __m128i = core::mem::zeroed();
    let mut mask: __m128i = core::mem::zeroed();
    let mut flat: __m128i;
    let mut p: [__m128i; 2] = core::mem::zeroed();
    let mut q: [__m128i; 2] = core::mem::zeroed();
    let zero: __m128i = _mm_setzero_si128();
    let mut abs_p0q0: __m128i = abs_diff16(*q0, *p0);
    let mut abs_p1q1: __m128i = abs_diff16(*q1, *p1);
    let mut abs_p1p0: __m128i = abs_diff16(*p1, *p0);
    let mut abs_q1q0: __m128i = abs_diff16(*q1, *q0);
    let ffff: __m128i = _mm_cmpeq_epi16(abs_p1p0, abs_p1p0);
    let one: __m128i = _mm_set1_epi16(1);
    let mut t80: __m128i = core::mem::zeroed();
    get_limit_dual(
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
        &raw mut blimit0,
        &raw mut limit0,
        &raw mut thresh0,
        &raw mut t80,
    );
    // filter_mask and hev_mask
    flat = _mm_max_epi16(abs_p1p0, abs_q1q0);
    abs_p0q0 = _mm_adds_epu16(abs_p0q0, abs_p0q0);
    abs_p1q1 = _mm_srli_epi16::<{ 1 }>(abs_p1q1);
    mask = _mm_subs_epu16(_mm_adds_epu16(abs_p0q0, abs_p1q1), blimit0);
    mask = _mm_xor_si128(_mm_cmpeq_epi16(mask, zero), ffff);
    // mask |= (abs(*p0 - *q0) * 2 + abs(*p1 - *q1) / 2  > blimit) * -1;
    // So taking maximums continues to work:
    mask = _mm_and_si128(mask, _mm_adds_epu16(limit0, one));
    mask = _mm_max_epi16(flat, mask);
    mask = _mm_subs_epu16(mask, limit0);
    mask = _mm_cmpeq_epi16(mask, zero);
    p[0] = *p0;
    p[1] = *p1;
    q[0] = *q0;
    q[1] = *q1;
    highbd_filter4_dual_sse2(
        p.as_mut_ptr(),
        q.as_mut_ptr(),
        ps,
        qs,
        &raw mut mask as *const __m128i,
        &raw mut thresh0 as *const __m128i,
        bd,
        &raw mut t80,
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_sse2(
    s: *mut u16,
    p: i32,
    _blimit: *const u8,
    _limit: *const u8,
    _thresh: *const u8,
    bd: i32,
) {
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1: __m128i =
        _mm_loadl_epi64(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    let mut p0: __m128i =
        _mm_loadl_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    let mut q0: __m128i =
        _mm_loadl_epi64(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    let mut q1: __m128i =
        _mm_loadl_epi64(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_lpf_internal_4_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut q1q0,
        &raw mut p1p0,
        _blimit,
        _limit,
        _thresh,
        bd,
    );
    _mm_storel_epi64(
        s.offset(-((2 * p) as isize)) as *mut __m128i,
        _mm_srli_si128::<8>(p1p0),
    );
    _mm_storel_epi64(s.offset(-((1 * p) as isize)) as *mut __m128i, p1p0);
    _mm_storel_epi64(s.offset((0 * p) as isize) as *mut __m128i, q1q0);
    _mm_storel_epi64(
        s.offset((1 * p) as isize) as *mut __m128i,
        _mm_srli_si128::<8>(q1q0),
    );
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_horizontal_4_dual_sse2(
    s: *mut u16,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut p1: __m128i =
        _mm_loadu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i as *const __m128i);
    let mut p0: __m128i =
        _mm_loadu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i as *const __m128i);
    let mut q0: __m128i =
        _mm_loadu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i as *const __m128i);
    let mut q1: __m128i =
        _mm_loadu_si128(s.offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    let mut ps: [__m128i; 2] = core::mem::zeroed();
    let mut qs: [__m128i; 2] = core::mem::zeroed();
    highbd_lpf_internal_4_dual_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        ps.as_mut_ptr(),
        qs.as_mut_ptr(),
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
    );
    _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, ps[1]);
    _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, ps[0]);
    _mm_storeu_si128(s.offset((0 * p) as isize) as *mut __m128i, qs[0]);
    _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, qs[1]);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_sse2(
    s: *mut u16,
    p: i32,
    blimit: *const u8,
    limit: *const u8,
    thresh: *const u8,
    bd: i32,
) {
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    x0 = _mm_loadl_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose4x8_8x4_low_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    highbd_lpf_internal_4_sse2(
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut q1q0,
        &raw mut p1p0,
        blimit,
        limit,
        thresh,
        bd,
    );
    p1 = _mm_srli_si128::<8>(p1p0);
    q1 = _mm_srli_si128::<8>(q1q0);
    // transpose from 8x4 to 4x8
    highbd_transpose4x8_8x4_low_sse2(
        &raw mut p1,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q1,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    _mm_storel_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i, d0);
    _mm_storel_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i, d1);
    _mm_storel_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i, d2);
    _mm_storel_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i, d3);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_4_dual_sse2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut ps: [__m128i; 2] = core::mem::zeroed();
    let mut qs: [__m128i; 2] = core::mem::zeroed();
    x0 = _mm_loadl_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadl_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadl_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadl_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadl_epi64(s.offset(-2).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadl_epi64(s.offset(-2).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadl_epi64(s.offset(-2).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x7 = _mm_loadl_epi64(s.offset(-2).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose8x8_low_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    highbd_lpf_internal_4_dual_sse2(
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        ps.as_mut_ptr(),
        qs.as_mut_ptr(),
        blimit0,
        limit0,
        thresh0,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
    highbd_transpose4x8_8x4_sse2(
        ps.as_mut_ptr().offset(1),
        ps.as_mut_ptr(),
        qs.as_mut_ptr(),
        qs.as_mut_ptr().offset(1),
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    _mm_storel_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i, d0);
    _mm_storel_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i, d1);
    _mm_storel_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i, d2);
    _mm_storel_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i, d3);
    _mm_storel_epi64(s.offset(-2).offset((4 * p) as isize) as *mut __m128i, d4);
    _mm_storel_epi64(s.offset(-2).offset((5 * p) as isize) as *mut __m128i, d5);
    _mm_storel_epi64(s.offset(-2).offset((6 * p) as isize) as *mut __m128i, d6);
    _mm_storel_epi64(s.offset(-2).offset((7 * p) as isize) as *mut __m128i, d7);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_6_sse2(
    s: *mut u16,
    p: i32,
    blimit: *const u8,
    limit: *const u8,
    thresh: *const u8,
    bd: i32,
) {
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    x3 = _mm_loadu_si128(s.offset(-3).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadu_si128(s.offset(-3).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadu_si128(s.offset(-3).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x0 = _mm_loadu_si128(s.offset(-3).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose4x8_8x4_sse2(
        &raw mut x3,
        &raw mut x2,
        &raw mut x1,
        &raw mut x0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    highbd_lpf_internal_6_sse2(
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut p1p0,
        &raw mut q1q0,
        blimit,
        limit,
        thresh,
        bd,
    );
    p0 = _mm_srli_si128::<8>(p1p0);
    q0 = _mm_srli_si128::<8>(q1q0);
    highbd_transpose4x8_8x4_low_sse2(
        &raw mut p0,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    _mm_storel_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i, d0);
    _mm_storel_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i, d1);
    _mm_storel_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i, d2);
    _mm_storel_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i, d3);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_6_dual_sse2(
    s: *mut u16,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
    _blimit1: *const u8,
    _limit1: *const u8,
    _thresh1: *const u8,
    bd: i32,
) {
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut q1: __m128i = core::mem::zeroed();
    let mut p2: __m128i = core::mem::zeroed();
    let mut q2: __m128i = core::mem::zeroed();
    x0 = _mm_loadu_si128(s.offset(-3).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadu_si128(s.offset(-3).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadu_si128(s.offset(-3).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadu_si128(s.offset(-3).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadu_si128(s.offset(-3).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadu_si128(s.offset(-3).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadu_si128(s.offset(-3).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x7 = _mm_loadu_si128(s.offset(-3).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose8x8_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
        &raw mut p2,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut q2,
        &raw mut d6,
        &raw mut d7,
    );
    highbd_lpf_internal_6_dual_sse2(
        &raw mut p2,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut q2,
        _blimit0,
        _limit0,
        _thresh0,
        _blimit1,
        _limit1,
        _thresh1,
        bd,
    );
    highbd_transpose4x8_8x4_sse2(
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut q1,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    _mm_storel_epi64(s.offset(-2).offset((0 * p) as isize) as *mut __m128i, d0);
    _mm_storel_epi64(s.offset(-2).offset((1 * p) as isize) as *mut __m128i, d1);
    _mm_storel_epi64(s.offset(-2).offset((2 * p) as isize) as *mut __m128i, d2);
    _mm_storel_epi64(s.offset(-2).offset((3 * p) as isize) as *mut __m128i, d3);
    _mm_storel_epi64(s.offset(-2).offset((4 * p) as isize) as *mut __m128i, d4);
    _mm_storel_epi64(s.offset(-2).offset((5 * p) as isize) as *mut __m128i, d5);
    _mm_storel_epi64(s.offset(-2).offset((6 * p) as isize) as *mut __m128i, d6);
    _mm_storel_epi64(s.offset(-2).offset((7 * p) as isize) as *mut __m128i, d7);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_sse2(
    s: *mut u16,
    p: i32,
    blimit: *const u8,
    limit: *const u8,
    thresh: *const u8,
    bd: i32,
) {
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut q1q0: __m128i = core::mem::zeroed();
    let mut p1p0: __m128i = core::mem::zeroed();
    p3 = _mm_loadu_si128(s.offset(-4).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    p2 = _mm_loadu_si128(s.offset(-4).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    p1 = _mm_loadu_si128(s.offset(-4).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    p0 = _mm_loadu_si128(s.offset(-4).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose4x8_8x4_sse2(
        &raw mut p3,
        &raw mut p2,
        &raw mut p1,
        &raw mut p0,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    // Loop filtering
    highbd_lpf_internal_8_sse2(
        &raw mut d0,
        &raw mut d7,
        &raw mut d1,
        &raw mut d6,
        &raw mut d2,
        &raw mut d5,
        &raw mut d3,
        &raw mut d4,
        &raw mut q1q0,
        &raw mut p1p0,
        blimit,
        limit,
        thresh,
        bd,
    );
    p0 = _mm_srli_si128::<8>(p1p0);
    q0 = _mm_srli_si128::<8>(q1q0);
    highbd_transpose8x8_low_sse2(
        &raw mut d0,
        &raw mut d1,
        &raw mut p0,
        &raw mut p1p0,
        &raw mut q1q0,
        &raw mut q0,
        &raw mut d6,
        &raw mut d7,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    _mm_storeu_si128(s.offset(-4).offset((0 * p) as isize) as *mut __m128i, d0);
    _mm_storeu_si128(s.offset(-4).offset((1 * p) as isize) as *mut __m128i, d1);
    _mm_storeu_si128(s.offset(-4).offset((2 * p) as isize) as *mut __m128i, d2);
    _mm_storeu_si128(s.offset(-4).offset((3 * p) as isize) as *mut __m128i, d3);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_8_dual_sse2(
    s: *mut u16,
    p: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    let mut x0: __m128i = core::mem::zeroed();
    let mut x1: __m128i = core::mem::zeroed();
    let mut x2: __m128i = core::mem::zeroed();
    let mut x3: __m128i = core::mem::zeroed();
    let mut x4: __m128i = core::mem::zeroed();
    let mut x5: __m128i = core::mem::zeroed();
    let mut x6: __m128i = core::mem::zeroed();
    let mut x7: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d4: __m128i = core::mem::zeroed();
    let mut d5: __m128i = core::mem::zeroed();
    let mut d6: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    x0 = _mm_loadu_si128(s.offset(-4).offset((0 * p) as isize) as *mut __m128i as *const __m128i);
    x1 = _mm_loadu_si128(s.offset(-4).offset((1 * p) as isize) as *mut __m128i as *const __m128i);
    x2 = _mm_loadu_si128(s.offset(-4).offset((2 * p) as isize) as *mut __m128i as *const __m128i);
    x3 = _mm_loadu_si128(s.offset(-4).offset((3 * p) as isize) as *mut __m128i as *const __m128i);
    x4 = _mm_loadu_si128(s.offset(-4).offset((4 * p) as isize) as *mut __m128i as *const __m128i);
    x5 = _mm_loadu_si128(s.offset(-4).offset((5 * p) as isize) as *mut __m128i as *const __m128i);
    x6 = _mm_loadu_si128(s.offset(-4).offset((6 * p) as isize) as *mut __m128i as *const __m128i);
    x7 = _mm_loadu_si128(s.offset(-4).offset((7 * p) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose8x8_sse2(
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
    );
    highbd_lpf_internal_8_dual_sse2(
        &raw mut d0,
        &raw mut d7,
        &raw mut d1,
        &raw mut d6,
        &raw mut d2,
        &raw mut d5,
        &raw mut d3,
        &raw mut d4,
        blimit0,
        limit0,
        thresh0,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
    highbd_transpose8x8_sse2(
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
        &raw mut d4,
        &raw mut d5,
        &raw mut d6,
        &raw mut d7,
        &raw mut x0,
        &raw mut x1,
        &raw mut x2,
        &raw mut x3,
        &raw mut x4,
        &raw mut x5,
        &raw mut x6,
        &raw mut x7,
    );
    _mm_storeu_si128(s.offset(-4).offset((0 * p) as isize) as *mut __m128i, x0);
    _mm_storeu_si128(s.offset(-4).offset((1 * p) as isize) as *mut __m128i, x1);
    _mm_storeu_si128(s.offset(-4).offset((2 * p) as isize) as *mut __m128i, x2);
    _mm_storeu_si128(s.offset(-4).offset((3 * p) as isize) as *mut __m128i, x3);
    _mm_storeu_si128(s.offset(-4).offset((4 * p) as isize) as *mut __m128i, x4);
    _mm_storeu_si128(s.offset(-4).offset((5 * p) as isize) as *mut __m128i, x5);
    _mm_storeu_si128(s.offset(-4).offset((6 * p) as isize) as *mut __m128i, x6);
    _mm_storeu_si128(s.offset(-4).offset((7 * p) as isize) as *mut __m128i, x7);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_sse2(
    s: *mut u16,
    pitch: i32,
    blimit: *const u8,
    limit: *const u8,
    thresh: *const u8,
    bd: i32,
) {
    let mut q: [__m128i; 7] = core::mem::zeroed();
    let mut p: [__m128i; 7] = core::mem::zeroed();
    let mut pq: [__m128i; 7] = core::mem::zeroed();
    let mut p6: __m128i = core::mem::zeroed();
    let mut p5: __m128i = core::mem::zeroed();
    let mut p4: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut p6_2: __m128i = core::mem::zeroed();
    let mut p5_2: __m128i = core::mem::zeroed();
    let mut p4_2: __m128i = core::mem::zeroed();
    let mut p3_2: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d1: __m128i = core::mem::zeroed();
    let mut d2: __m128i = core::mem::zeroed();
    let mut d3: __m128i = core::mem::zeroed();
    let mut d0_2: __m128i = core::mem::zeroed();
    let mut d1_2: __m128i = core::mem::zeroed();
    let mut d2_2: __m128i = core::mem::zeroed();
    let mut d3_2: __m128i = core::mem::zeroed();
    let mut d7_2: __m128i = core::mem::zeroed();
    p6 = _mm_loadu_si128(
        s.offset(-8).offset((0 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p5 = _mm_loadu_si128(
        s.offset(-8).offset((1 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p4 = _mm_loadu_si128(
        s.offset(-8).offset((2 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p3 = _mm_loadu_si128(
        s.offset(-8).offset((3 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    highbd_transpose4x8_8x4_sse2(
        &raw mut p6,
        &raw mut p5,
        &raw mut p4,
        &raw mut p3,
        &raw mut d0,
        p.as_mut_ptr().offset(6),
        p.as_mut_ptr().offset(5),
        p.as_mut_ptr().offset(4),
        p.as_mut_ptr().offset(3),
        p.as_mut_ptr().offset(2),
        p.as_mut_ptr().offset(1),
        p.as_mut_ptr(),
    );
    p6_2 = _mm_loadu_si128(s.offset((0 * pitch) as isize) as *mut __m128i as *const __m128i);
    p5_2 = _mm_loadu_si128(s.offset((1 * pitch) as isize) as *mut __m128i as *const __m128i);
    p4_2 = _mm_loadu_si128(s.offset((2 * pitch) as isize) as *mut __m128i as *const __m128i);
    p3_2 = _mm_loadu_si128(s.offset((3 * pitch) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose4x8_8x4_sse2(
        &raw mut p6_2,
        &raw mut p5_2,
        &raw mut p4_2,
        &raw mut p3_2,
        q.as_mut_ptr(),
        q.as_mut_ptr().offset(1),
        q.as_mut_ptr().offset(2),
        q.as_mut_ptr().offset(3),
        q.as_mut_ptr().offset(4),
        q.as_mut_ptr().offset(5),
        q.as_mut_ptr().offset(6),
        &raw mut d7_2,
    );
    highbd_lpf_internal_14_sse2(
        p.as_mut_ptr(),
        q.as_mut_ptr(),
        pq.as_mut_ptr(),
        blimit,
        limit,
        thresh,
        bd,
    );
    highbd_transpose8x8_low_sse2(
        &raw mut d0,
        p.as_mut_ptr().offset(6),
        pq.as_mut_ptr().offset(5),
        pq.as_mut_ptr().offset(4),
        pq.as_mut_ptr().offset(3),
        pq.as_mut_ptr().offset(2),
        pq.as_mut_ptr().offset(1),
        pq.as_mut_ptr(),
        &raw mut d0,
        &raw mut d1,
        &raw mut d2,
        &raw mut d3,
    );
    q[0] = _mm_srli_si128::<8>(pq[0]);
    q[1] = _mm_srli_si128::<8>(pq[1]);
    q[2] = _mm_srli_si128::<8>(pq[2]);
    q[3] = _mm_srli_si128::<8>(pq[3]);
    q[4] = _mm_srli_si128::<8>(pq[4]);
    q[5] = _mm_srli_si128::<8>(pq[5]);
    highbd_transpose8x8_low_sse2(
        q.as_mut_ptr(),
        q.as_mut_ptr().offset(1),
        q.as_mut_ptr().offset(2),
        q.as_mut_ptr().offset(3),
        q.as_mut_ptr().offset(4),
        q.as_mut_ptr().offset(5),
        q.as_mut_ptr().offset(6),
        &raw mut d7_2,
        &raw mut d0_2,
        &raw mut d1_2,
        &raw mut d2_2,
        &raw mut d3_2,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((0 * pitch) as isize) as *mut __m128i,
        d0,
    );
    _mm_storeu_si128(s.offset((0 * pitch) as isize) as *mut __m128i, d0_2);
    _mm_storeu_si128(
        s.offset(-8).offset((1 * pitch) as isize) as *mut __m128i,
        d1,
    );
    _mm_storeu_si128(s.offset((1 * pitch) as isize) as *mut __m128i, d1_2);
    _mm_storeu_si128(
        s.offset(-8).offset((2 * pitch) as isize) as *mut __m128i,
        d2,
    );
    _mm_storeu_si128(s.offset((2 * pitch) as isize) as *mut __m128i, d2_2);
    _mm_storeu_si128(
        s.offset(-8).offset((3 * pitch) as isize) as *mut __m128i,
        d3,
    );
    _mm_storeu_si128(s.offset((3 * pitch) as isize) as *mut __m128i, d3_2);
}

#[no_mangle]
#[target_feature(enable = "sse2")]
pub unsafe extern "C-unwind" fn aom_highbd_lpf_vertical_14_dual_sse2(
    s: *mut u16,
    pitch: i32,
    blimit0: *const u8,
    limit0: *const u8,
    thresh0: *const u8,
    blimit1: *const u8,
    limit1: *const u8,
    thresh1: *const u8,
    bd: i32,
) {
    let mut q: [__m128i; 7] = core::mem::zeroed();
    let mut p: [__m128i; 7] = core::mem::zeroed();
    let mut p6: __m128i = core::mem::zeroed();
    let mut p5: __m128i = core::mem::zeroed();
    let mut p4: __m128i = core::mem::zeroed();
    let mut p3: __m128i = core::mem::zeroed();
    let mut p2: __m128i = core::mem::zeroed();
    let mut p1: __m128i = core::mem::zeroed();
    let mut p0: __m128i = core::mem::zeroed();
    let mut q0: __m128i = core::mem::zeroed();
    let mut p6_2: __m128i = core::mem::zeroed();
    let mut p5_2: __m128i = core::mem::zeroed();
    let mut p4_2: __m128i = core::mem::zeroed();
    let mut p3_2: __m128i = core::mem::zeroed();
    let mut p2_2: __m128i = core::mem::zeroed();
    let mut p1_2: __m128i = core::mem::zeroed();
    let mut q0_2: __m128i = core::mem::zeroed();
    let mut p0_2: __m128i = core::mem::zeroed();
    let mut d0: __m128i = core::mem::zeroed();
    let mut d7: __m128i = core::mem::zeroed();
    let mut d0_out: __m128i = core::mem::zeroed();
    let mut d1_out: __m128i = core::mem::zeroed();
    let mut d2_out: __m128i = core::mem::zeroed();
    let mut d3_out: __m128i = core::mem::zeroed();
    let mut d4_out: __m128i = core::mem::zeroed();
    let mut d5_out: __m128i = core::mem::zeroed();
    let mut d6_out: __m128i = core::mem::zeroed();
    let mut d7_out: __m128i = core::mem::zeroed();
    p6 = _mm_loadu_si128(
        s.offset(-8).offset((0 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p5 = _mm_loadu_si128(
        s.offset(-8).offset((1 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p4 = _mm_loadu_si128(
        s.offset(-8).offset((2 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p3 = _mm_loadu_si128(
        s.offset(-8).offset((3 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p2 = _mm_loadu_si128(
        s.offset(-8).offset((4 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p1 = _mm_loadu_si128(
        s.offset(-8).offset((5 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    p0 = _mm_loadu_si128(
        s.offset(-8).offset((6 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    q0 = _mm_loadu_si128(
        s.offset(-8).offset((7 * pitch) as isize) as *mut __m128i as *const __m128i
    );
    highbd_transpose8x8_sse2(
        &raw mut p6,
        &raw mut p5,
        &raw mut p4,
        &raw mut p3,
        &raw mut p2,
        &raw mut p1,
        &raw mut p0,
        &raw mut q0,
        &raw mut d0,
        p.as_mut_ptr().offset(6),
        p.as_mut_ptr().offset(5),
        p.as_mut_ptr().offset(4),
        p.as_mut_ptr().offset(3),
        p.as_mut_ptr().offset(2),
        p.as_mut_ptr().offset(1),
        p.as_mut_ptr(),
    );
    p6_2 = _mm_loadu_si128(s.offset((0 * pitch) as isize) as *mut __m128i as *const __m128i);
    p5_2 = _mm_loadu_si128(s.offset((1 * pitch) as isize) as *mut __m128i as *const __m128i);
    p4_2 = _mm_loadu_si128(s.offset((2 * pitch) as isize) as *mut __m128i as *const __m128i);
    p3_2 = _mm_loadu_si128(s.offset((3 * pitch) as isize) as *mut __m128i as *const __m128i);
    p2_2 = _mm_loadu_si128(s.offset((4 * pitch) as isize) as *mut __m128i as *const __m128i);
    p1_2 = _mm_loadu_si128(s.offset((5 * pitch) as isize) as *mut __m128i as *const __m128i);
    p0_2 = _mm_loadu_si128(s.offset((6 * pitch) as isize) as *mut __m128i as *const __m128i);
    q0_2 = _mm_loadu_si128(s.offset((7 * pitch) as isize) as *mut __m128i as *const __m128i);
    highbd_transpose8x8_sse2(
        &raw mut p6_2,
        &raw mut p5_2,
        &raw mut p4_2,
        &raw mut p3_2,
        &raw mut p2_2,
        &raw mut p1_2,
        &raw mut p0_2,
        &raw mut q0_2,
        q.as_mut_ptr(),
        q.as_mut_ptr().offset(1),
        q.as_mut_ptr().offset(2),
        q.as_mut_ptr().offset(3),
        q.as_mut_ptr().offset(4),
        q.as_mut_ptr().offset(5),
        q.as_mut_ptr().offset(6),
        &raw mut d7,
    );
    highbd_lpf_internal_14_dual_sse2(
        p.as_mut_ptr(),
        q.as_mut_ptr(),
        blimit0,
        limit0,
        thresh0,
        blimit1,
        limit1,
        thresh1,
        bd,
    );
    highbd_transpose8x8_sse2(
        &raw mut d0,
        p.as_mut_ptr().offset(6),
        p.as_mut_ptr().offset(5),
        p.as_mut_ptr().offset(4),
        p.as_mut_ptr().offset(3),
        p.as_mut_ptr().offset(2),
        p.as_mut_ptr().offset(1),
        p.as_mut_ptr(),
        &raw mut d0_out,
        &raw mut d1_out,
        &raw mut d2_out,
        &raw mut d3_out,
        &raw mut d4_out,
        &raw mut d5_out,
        &raw mut d6_out,
        &raw mut d7_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((0 * pitch) as isize) as *mut __m128i,
        d0_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((1 * pitch) as isize) as *mut __m128i,
        d1_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((2 * pitch) as isize) as *mut __m128i,
        d2_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((3 * pitch) as isize) as *mut __m128i,
        d3_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((4 * pitch) as isize) as *mut __m128i,
        d4_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((5 * pitch) as isize) as *mut __m128i,
        d5_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((6 * pitch) as isize) as *mut __m128i,
        d6_out,
    );
    _mm_storeu_si128(
        s.offset(-8).offset((7 * pitch) as isize) as *mut __m128i,
        d7_out,
    );
    highbd_transpose8x8_sse2(
        q.as_mut_ptr(),
        q.as_mut_ptr().offset(1),
        q.as_mut_ptr().offset(2),
        q.as_mut_ptr().offset(3),
        q.as_mut_ptr().offset(4),
        q.as_mut_ptr().offset(5),
        q.as_mut_ptr().offset(6),
        &raw mut d7,
        &raw mut d0_out,
        &raw mut d1_out,
        &raw mut d2_out,
        &raw mut d3_out,
        &raw mut d4_out,
        &raw mut d5_out,
        &raw mut d6_out,
        &raw mut d7_out,
    );
    _mm_storeu_si128(s.offset((0 * pitch) as isize) as *mut __m128i, d0_out);
    _mm_storeu_si128(s.offset((1 * pitch) as isize) as *mut __m128i, d1_out);
    _mm_storeu_si128(s.offset((2 * pitch) as isize) as *mut __m128i, d2_out);
    _mm_storeu_si128(s.offset((3 * pitch) as isize) as *mut __m128i, d3_out);
    _mm_storeu_si128(s.offset((4 * pitch) as isize) as *mut __m128i, d4_out);
    _mm_storeu_si128(s.offset((5 * pitch) as isize) as *mut __m128i, d5_out);
    _mm_storeu_si128(s.offset((6 * pitch) as isize) as *mut __m128i, d6_out);
    _mm_storeu_si128(s.offset((7 * pitch) as isize) as *mut __m128i, d7_out);
}
