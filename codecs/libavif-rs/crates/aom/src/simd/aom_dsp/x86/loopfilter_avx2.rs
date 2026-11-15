/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/aom_dsp/x86/loopfilter_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
//
// Translation notes:
// - Every function carries `avx2`, the flag upstream compiles the file with.
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

// AVX2
static filt_loopfilter_avx2: Aligned32<[u8; 32]> = Aligned32([
    0, 128, 1, 128, 2, 128, 3, 128, 4, 128, 5, 128, 6, 128, 7, 128, 8, 128, 9, 128, 10, 128, 11,
    128, 12, 128, 13, 128, 14, 128, 15, 128,
]);

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_6_quad_avx2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut p256_2: __m256i;
    let mut q256_2: __m256i;
    let mut p256_1: __m256i;
    let mut q256_1: __m256i;
    let mut p256_0: __m256i;
    let mut q256_0: __m256i;
    let mut p2: __m128i;
    let mut p1: __m128i;
    let mut p0: __m128i;
    let mut q0: __m128i;
    let mut q1: __m128i;
    let mut q2: __m128i;
    let mut mask: __m128i;
    let mut flat: __m128i;
    let thresh_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_thresh0 as i32));
    let limit_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_limit0 as i32));
    let blimit_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_blimit0 as i32));
    let zero: __m128i = _mm_setzero_si128();
    let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
    p256_2 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((3 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    p256_1 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((2 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    p256_0 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((1 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    q256_0 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((0 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    q256_1 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((1 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    q256_2 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((2 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    p2 = _mm256_castsi256_si128(p256_2);
    p1 = _mm256_castsi256_si128(p256_1);
    p0 = _mm256_castsi256_si128(p256_0);
    q0 = _mm256_castsi256_si128(q256_0);
    q1 = _mm256_castsi256_si128(q256_1);
    q2 = _mm256_castsi256_si128(q256_2);
    {
        let mut work: __m128i;
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let abs_p1p0: __m128i = _mm_or_si128(_mm_subs_epu8(p1, p0), _mm_subs_epu8(p0, p1));
        let abs_q1q0: __m128i = _mm_or_si128(_mm_subs_epu8(q1, q0), _mm_subs_epu8(q0, q1));
        let mut abs_p0q0: __m128i = _mm_or_si128(_mm_subs_epu8(p0, q0), _mm_subs_epu8(q0, p0));
        let mut abs_p1q1: __m128i = _mm_or_si128(_mm_subs_epu8(p1, q1), _mm_subs_epu8(q1, p1));
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(flat, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(p2, p1), _mm_subs_epu8(p1, p2)),
            _mm_or_si128(_mm_subs_epu8(q2, q1), _mm_subs_epu8(q1, q2)),
        );
        mask = _mm_max_epu8(work, mask);
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    // loop filter
    {
        let t4: __m128i = _mm_set1_epi8(4);
        let t3: __m128i = _mm_set1_epi8(3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t1: __m128i = _mm_set1_epi8(0x1);
        let t7f: __m128i = _mm_set1_epi8(0x7f);
        let one: __m128i = _mm_set1_epi8(1);
        let mut hev: __m128i;
        hev = _mm_subs_epu8(flat, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        let mut ps1: __m128i = _mm_xor_si128(p1, t80);
        let mut ps0: __m128i = _mm_xor_si128(p0, t80);
        let mut qs0: __m128i = _mm_xor_si128(q0, t80);
        let mut qs1: __m128i = _mm_xor_si128(q1, t80);
        let mut filt: __m128i;
        let mut work_a: __m128i;
        let mut filter1: __m128i;
        let mut filter2: __m128i;
        let mut flat_p1: __m128i;
        let mut flat_p0: __m128i;
        let mut flat_q0: __m128i;
        let mut flat_q1: __m128i;
        filt = _mm_and_si128(_mm_subs_epi8(ps1, qs1), hev);
        work_a = _mm_subs_epi8(qs0, ps0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        filter1 = _mm_adds_epi8(filt, t4);
        filter2 = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        qs0 = _mm_xor_si128(_mm_subs_epi8(qs0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        ps0 = _mm_xor_si128(_mm_adds_epi8(ps0, filter2), t80);
        filt = _mm_adds_epi8(filter1, t1);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        ps1 = _mm_xor_si128(_mm_adds_epi8(ps1, filt), t80);
        qs1 = _mm_xor_si128(_mm_subs_epi8(qs1, filt), t80);
        let mut work: __m128i;
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(p2, p0), _mm_subs_epu8(p0, p2)),
            _mm_or_si128(_mm_subs_epu8(q2, q0), _mm_subs_epu8(q0, q2)),
        );
        flat = _mm_max_epu8(work, flat);
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
            let four: __m256i = _mm256_set1_epi16(4);
            let mut pixetFilter: __m256i;
            let mut add: __m256i;
            let mut res: __m256i;
            let filter: __m256i =
                _mm256_load_si256(filt_loopfilter_avx2.0.as_ptr() as *const __m256i);
            p256_2 = _mm256_shuffle_epi8(p256_2, filter);
            p256_1 = _mm256_shuffle_epi8(p256_1, filter);
            p256_0 = _mm256_shuffle_epi8(p256_0, filter);
            q256_0 = _mm256_shuffle_epi8(q256_0, filter);
            q256_1 = _mm256_shuffle_epi8(q256_1, filter);
            q256_2 = _mm256_shuffle_epi8(q256_2, filter);
            pixetFilter = _mm256_slli_epi16::<{ 1 }>(_mm256_add_epi16(
                p256_2,
                _mm256_add_epi16(p256_1, p256_0),
            ));
            pixetFilter = _mm256_add_epi16(pixetFilter, _mm256_add_epi16(p256_2, q256_0));
            pixetFilter = _mm256_add_epi16(four, pixetFilter);
            res = _mm256_srli_epi16::<{ 3 }>(pixetFilter);
            flat_p1 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res, res,
            )));
            p1 = _mm_andnot_si128(flat, ps1);
            flat_p1 = _mm_and_si128(flat, flat_p1);
            p1 = _mm_or_si128(flat_p1, p1);
            add = _mm256_add_epi16(
                _mm256_sub_epi16(q256_1, p256_2),
                _mm256_sub_epi16(q256_0, p256_2),
            );
            pixetFilter = _mm256_add_epi16(pixetFilter, add);
            res = _mm256_srli_epi16::<{ 3 }>(pixetFilter);
            flat_p0 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res, res,
            )));
            p0 = _mm_andnot_si128(flat, ps0);
            flat_p0 = _mm_and_si128(flat, flat_p0);
            p0 = _mm_or_si128(flat_p0, p0);
            add = _mm256_add_epi16(
                _mm256_sub_epi16(q256_2, p256_2),
                _mm256_sub_epi16(q256_1, p256_1),
            );
            pixetFilter = _mm256_add_epi16(pixetFilter, add);
            res = _mm256_srli_epi16::<{ 3 }>(pixetFilter);
            flat_q0 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res, res,
            )));
            q0 = _mm_andnot_si128(flat, qs0);
            flat_q0 = _mm_and_si128(flat, flat_q0);
            q0 = _mm_or_si128(flat_q0, q0);
            add = _mm256_add_epi16(
                _mm256_sub_epi16(q256_2, p256_1),
                _mm256_sub_epi16(q256_2, p256_0),
            );
            pixetFilter = _mm256_add_epi16(pixetFilter, add);
            res = _mm256_srli_epi16::<{ 3 }>(pixetFilter);
            flat_q1 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res, res,
            )));
            q1 = _mm_andnot_si128(flat, qs1);
            flat_q1 = _mm_and_si128(flat, flat_q1);
            q1 = _mm_or_si128(flat_q1, q1);
            _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, p1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, p0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, q0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, q1);
            _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, q2);
        } else {
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, ps1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, ps0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, qs0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, qs1);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_8_quad_avx2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut p256_3: __m256i;
    let mut q256_3: __m256i;
    let mut p256_2: __m256i;
    let mut q256_2: __m256i;
    let mut p256_1: __m256i;
    let mut q256_1: __m256i;
    let mut p256_0: __m256i;
    let mut q256_0: __m256i;
    let mut p3: __m128i;
    let mut p2: __m128i;
    let mut p1: __m128i;
    let mut p0: __m128i;
    let mut q0: __m128i;
    let mut q1: __m128i;
    let mut q2: __m128i;
    let mut q3: __m128i;
    let mut mask: __m128i;
    let mut flat: __m128i;
    let thresh_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_thresh0 as i32));
    let limit_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_limit0 as i32));
    let blimit_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_blimit0 as i32));
    let zero: __m128i = _mm_setzero_si128();
    let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
    p256_3 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((4 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    p256_2 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((3 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    p256_1 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((2 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    p256_0 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((1 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    q256_0 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((0 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    q256_1 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((1 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    q256_2 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((2 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    q256_3 = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((3 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    p3 = _mm256_castsi256_si128(p256_3);
    p2 = _mm256_castsi256_si128(p256_2);
    p1 = _mm256_castsi256_si128(p256_1);
    p0 = _mm256_castsi256_si128(p256_0);
    q0 = _mm256_castsi256_si128(q256_0);
    q1 = _mm256_castsi256_si128(q256_1);
    q2 = _mm256_castsi256_si128(q256_2);
    q3 = _mm256_castsi256_si128(q256_3);
    {
        let mut work: __m128i;
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let abs_p1p0: __m128i = _mm_or_si128(_mm_subs_epu8(p1, p0), _mm_subs_epu8(p0, p1));
        let abs_q1q0: __m128i = _mm_or_si128(_mm_subs_epu8(q1, q0), _mm_subs_epu8(q0, q1));
        let mut abs_p0q0: __m128i = _mm_or_si128(_mm_subs_epu8(p0, q0), _mm_subs_epu8(q0, p0));
        let mut abs_p1q1: __m128i = _mm_or_si128(_mm_subs_epu8(p1, q1), _mm_subs_epu8(q1, p1));
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(flat, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(p2, p1), _mm_subs_epu8(p1, p2)),
            _mm_or_si128(_mm_subs_epu8(p3, p2), _mm_subs_epu8(p2, p3)),
        );
        mask = _mm_max_epu8(work, mask);
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(q2, q1), _mm_subs_epu8(q1, q2)),
            _mm_or_si128(_mm_subs_epu8(q3, q2), _mm_subs_epu8(q2, q3)),
        );
        mask = _mm_max_epu8(work, mask);
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    // loop filter
    {
        let t4: __m128i = _mm_set1_epi8(4);
        let t3: __m128i = _mm_set1_epi8(3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t1: __m128i = _mm_set1_epi8(0x1);
        let t7f: __m128i = _mm_set1_epi8(0x7f);
        let one: __m128i = _mm_set1_epi8(1);
        let mut hev: __m128i;
        hev = _mm_subs_epu8(flat, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        let mut ps1: __m128i = _mm_xor_si128(p1, t80);
        let mut ps0: __m128i = _mm_xor_si128(p0, t80);
        let mut qs0: __m128i = _mm_xor_si128(q0, t80);
        let mut qs1: __m128i = _mm_xor_si128(q1, t80);
        let mut filt: __m128i;
        let mut work_a: __m128i;
        let mut filter1: __m128i;
        let mut filter2: __m128i;
        let mut flat_p2: __m128i;
        let mut flat_p1: __m128i;
        let mut flat_p0: __m128i;
        let mut flat_q0: __m128i;
        let mut flat_q1: __m128i;
        let mut flat_q2: __m128i;
        filt = _mm_and_si128(_mm_subs_epi8(ps1, qs1), hev);
        work_a = _mm_subs_epi8(qs0, ps0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        filter1 = _mm_adds_epi8(filt, t4);
        filter2 = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        qs0 = _mm_xor_si128(_mm_subs_epi8(qs0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        ps0 = _mm_xor_si128(_mm_adds_epi8(ps0, filter2), t80);
        filt = _mm_adds_epi8(filter1, t1);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        ps1 = _mm_xor_si128(_mm_adds_epi8(ps1, filt), t80);
        qs1 = _mm_xor_si128(_mm_subs_epi8(qs1, filt), t80);
        let mut work: __m128i;
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(p2, p0), _mm_subs_epu8(p0, p2)),
            _mm_or_si128(_mm_subs_epu8(q2, q0), _mm_subs_epu8(q0, q2)),
        );
        flat = _mm_max_epu8(work, flat);
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(p3, p0), _mm_subs_epu8(p0, p3)),
            _mm_or_si128(_mm_subs_epu8(q3, q0), _mm_subs_epu8(q0, q3)),
        );
        flat = _mm_max_epu8(work, flat);
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
            let four: __m256i = _mm256_set1_epi16(4);
            let mut pixetFilter_p2p1p0: __m256i;
            let mut p2p1p0: __m256i;
            let mut q2q1q0: __m256i;
            let mut pixetFilter_q2q1q0: __m256i;
            let mut sum_p: __m256i;
            let mut sum_q: __m256i;
            let mut res_p: __m256i;
            let mut res_q: __m256i;
            let filter: __m256i =
                _mm256_load_si256(filt_loopfilter_avx2.0.as_ptr() as *const __m256i);
            p256_3 = _mm256_shuffle_epi8(p256_3, filter);
            p256_2 = _mm256_shuffle_epi8(p256_2, filter);
            p256_1 = _mm256_shuffle_epi8(p256_1, filter);
            p256_0 = _mm256_shuffle_epi8(p256_0, filter);
            q256_0 = _mm256_shuffle_epi8(q256_0, filter);
            q256_1 = _mm256_shuffle_epi8(q256_1, filter);
            q256_2 = _mm256_shuffle_epi8(q256_2, filter);
            q256_3 = _mm256_shuffle_epi8(q256_3, filter);
            p2p1p0 = _mm256_add_epi16(p256_0, _mm256_add_epi16(p256_2, p256_1));
            q2q1q0 = _mm256_add_epi16(q256_0, _mm256_add_epi16(q256_2, q256_1));
            pixetFilter_p2p1p0 = _mm256_add_epi16(four, _mm256_add_epi16(p2p1p0, q2q1q0));
            pixetFilter_q2q1q0 = pixetFilter_p2p1p0;
            pixetFilter_p2p1p0 = _mm256_add_epi16(pixetFilter_p2p1p0, p256_3);
            res_p = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_p2p1p0, p256_0));
            flat_p0 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res_p, res_p,
            )));
            p0 = _mm_andnot_si128(flat, ps0);
            flat_p0 = _mm_and_si128(flat, flat_p0);
            p0 = _mm_or_si128(flat_p0, p0);
            pixetFilter_q2q1q0 = _mm256_add_epi16(pixetFilter_q2q1q0, q256_3);
            res_q = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_q2q1q0, q256_0));
            flat_q0 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res_q, res_q,
            )));
            q0 = _mm_andnot_si128(flat, qs0);
            flat_q0 = _mm_and_si128(flat, flat_q0);
            q0 = _mm_or_si128(flat_q0, q0);
            sum_p = _mm256_sub_epi16(p256_3, q256_2);
            pixetFilter_p2p1p0 = _mm256_add_epi16(pixetFilter_p2p1p0, sum_p);
            res_p = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_p2p1p0, p256_1));
            flat_p1 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res_p, res_p,
            )));
            p1 = _mm_andnot_si128(flat, ps1);
            flat_p1 = _mm_and_si128(flat, flat_p1);
            p1 = _mm_or_si128(flat_p1, p1);
            sum_q = _mm256_sub_epi16(q256_3, p256_2);
            pixetFilter_q2q1q0 = _mm256_add_epi16(pixetFilter_q2q1q0, sum_q);
            res_q = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_q2q1q0, q256_1));
            flat_q1 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res_q, res_q,
            )));
            q1 = _mm_andnot_si128(flat, qs1);
            flat_q1 = _mm_and_si128(flat, flat_q1);
            q1 = _mm_or_si128(flat_q1, q1);
            sum_p = _mm256_sub_epi16(p256_3, q256_1);
            pixetFilter_p2p1p0 = _mm256_add_epi16(pixetFilter_p2p1p0, sum_p);
            res_p = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_p2p1p0, p256_2));
            flat_p2 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res_p, res_p,
            )));
            p2 = _mm_andnot_si128(flat, p2);
            flat_p2 = _mm_and_si128(flat, flat_p2);
            p2 = _mm_or_si128(flat_p2, p2);
            sum_q = _mm256_sub_epi16(q256_3, p256_1);
            pixetFilter_q2q1q0 = _mm256_add_epi16(pixetFilter_q2q1q0, sum_q);
            res_q = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_q2q1q0, q256_2));
            flat_q2 = _mm256_castsi256_si128(_mm256_permute4x64_epi64::<168>(_mm256_packus_epi16(
                res_q, res_q,
            )));
            q2 = _mm_andnot_si128(flat, q2);
            flat_q2 = _mm_and_si128(flat, flat_q2);
            q2 = _mm_or_si128(flat_q2, q2);
            _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, p1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, p0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, q0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, q1);
            _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, q2);
        } else {
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, ps1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, ps0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, qs0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, qs1);
        }
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn trans_store_16x16_lpf_vert14(
    in0: *mut u8,
    in_p: i32,
    out: *mut u8,
    out_p: i32,
    is_store_avx2: i32,
) {
    let x0: __m128i = _mm_loadu_si128(in0 as *mut __m128i as *const __m128i);
    let x1: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 1) as isize) as *mut __m128i as *const __m128i);
    let x2: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 2) as isize) as *mut __m128i as *const __m128i);
    let x3: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 3) as isize) as *mut __m128i as *const __m128i);
    let x4: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 4) as isize) as *mut __m128i as *const __m128i);
    let x5: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 5) as isize) as *mut __m128i as *const __m128i);
    let x6: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 6) as isize) as *mut __m128i as *const __m128i);
    let x7: __m128i =
        _mm_loadu_si128(in0.offset((in_p * 7) as isize) as *mut __m128i as *const __m128i);
    let y0: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x0),
        _mm_loadu_si128(in0.offset((in_p * 8) as isize) as *mut __m128i as *const __m128i),
    );
    let y1: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x1),
        _mm_loadu_si128(in0.offset((in_p * 9) as isize) as *mut __m128i as *const __m128i),
    );
    let y2: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x2),
        _mm_loadu_si128(in0.offset((in_p * 10) as isize) as *mut __m128i as *const __m128i),
    );
    let y3: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x3),
        _mm_loadu_si128(in0.offset((in_p * 11) as isize) as *mut __m128i as *const __m128i),
    );
    let y4: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x4),
        _mm_loadu_si128(in0.offset((in_p * 12) as isize) as *mut __m128i as *const __m128i),
    );
    let y5: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x5),
        _mm_loadu_si128(in0.offset((in_p * 13) as isize) as *mut __m128i as *const __m128i),
    );
    let y6: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x6),
        _mm_loadu_si128(in0.offset((in_p * 14) as isize) as *mut __m128i as *const __m128i),
    );
    let y7: __m256i = _mm256_insertf128_si256::<0x1>(
        _mm256_castsi128_si256(x7),
        _mm_loadu_si128(in0.offset((in_p * 15) as isize) as *mut __m128i as *const __m128i),
    );
    let y_s00: __m256i = _mm256_unpacklo_epi8(y0, y1);
    let y_s01: __m256i = _mm256_unpackhi_epi8(y0, y1);
    let y_s02: __m256i = _mm256_unpacklo_epi8(y2, y3);
    let y_s03: __m256i = _mm256_unpackhi_epi8(y2, y3);
    let y_s04: __m256i = _mm256_unpacklo_epi8(y4, y5);
    let y_s05: __m256i = _mm256_unpackhi_epi8(y4, y5);
    let y_s06: __m256i = _mm256_unpacklo_epi8(y6, y7);
    let y_s07: __m256i = _mm256_unpackhi_epi8(y6, y7);
    let y_s10: __m256i = _mm256_unpacklo_epi16(y_s00, y_s02);
    let y_s11: __m256i = _mm256_unpackhi_epi16(y_s00, y_s02);
    let y_s12: __m256i = _mm256_unpacklo_epi16(y_s01, y_s03);
    let y_s13: __m256i = _mm256_unpackhi_epi16(y_s01, y_s03);
    let y_s14: __m256i = _mm256_unpacklo_epi16(y_s04, y_s06);
    let y_s15: __m256i = _mm256_unpackhi_epi16(y_s04, y_s06);
    let y_s16: __m256i = _mm256_unpacklo_epi16(y_s05, y_s07);
    let y_s17: __m256i = _mm256_unpackhi_epi16(y_s05, y_s07);
    let y_s20: __m256i = _mm256_unpacklo_epi32(y_s10, y_s14);
    let y_s21: __m256i = _mm256_unpackhi_epi32(y_s10, y_s14);
    let y_s22: __m256i = _mm256_unpacklo_epi32(y_s11, y_s15);
    let y_s23: __m256i = _mm256_unpackhi_epi32(y_s11, y_s15);
    let y_s24: __m256i = _mm256_unpacklo_epi32(y_s12, y_s16);
    let y_s25: __m256i = _mm256_unpackhi_epi32(y_s12, y_s16);
    let y_s26: __m256i = _mm256_unpacklo_epi32(y_s13, y_s17);
    let y_s27: __m256i = _mm256_unpackhi_epi32(y_s13, y_s17);
    let row_s01: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s20);
    let row_s23: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s21);
    let row_s45: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s22);
    let row_s67: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s23);
    let row_s89: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s24);
    let row_s1011: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s25);
    let row_s1213: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s26);
    let row_s1415: __m256i = _mm256_permute4x64_epi64::<0xd8>(y_s27);
    if is_store_avx2 != 0 {
        _mm256_storeu_si256(out as *mut __m256i, row_s01);
        _mm256_storeu_si256(out.offset((2 * out_p) as isize) as *mut __m256i, row_s23);
        _mm256_storeu_si256(out.offset((4 * out_p) as isize) as *mut __m256i, row_s45);
        _mm256_storeu_si256(out.offset((6 * out_p) as isize) as *mut __m256i, row_s67);
        _mm256_storeu_si256(out.offset((8 * out_p) as isize) as *mut __m256i, row_s89);
        _mm256_storeu_si256(out.offset((10 * out_p) as isize) as *mut __m256i, row_s1011);
        _mm256_storeu_si256(out.offset((12 * out_p) as isize) as *mut __m256i, row_s1213);
        _mm256_storeu_si256(out.offset((14 * out_p) as isize) as *mut __m256i, row_s1415);
    } else {
        _mm_storeu_si128(out as *mut __m128i, _mm256_castsi256_si128(row_s01));
        _mm_storeu_si128(
            out.offset((2 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s23),
        );
        _mm_storeu_si128(
            out.offset((4 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s45),
        );
        _mm_storeu_si128(
            out.offset((6 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s67),
        );
        _mm_storeu_si128(
            out.offset((8 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s89),
        );
        _mm_storeu_si128(
            out.offset((10 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s1011),
        );
        _mm_storeu_si128(
            out.offset((12 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s1213),
        );
        _mm_storeu_si128(
            out.offset((14 * out_p) as isize) as *mut __m128i,
            _mm256_castsi256_si128(row_s1415),
        );
        _mm_storeu_si128(
            out.offset((1 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s01),
        );
        _mm_storeu_si128(
            out.offset((3 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s23),
        );
        _mm_storeu_si128(
            out.offset((5 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s45),
        );
        _mm_storeu_si128(
            out.offset((7 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s67),
        );
        _mm_storeu_si128(
            out.offset((9 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s89),
        );
        _mm_storeu_si128(
            out.offset((11 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s1011),
        );
        _mm_storeu_si128(
            out.offset((13 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s1213),
        );
        _mm_storeu_si128(
            out.offset((15 * out_p) as isize) as *mut __m128i,
            _mm256_extracti128_si256::<1>(row_s1415),
        );
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_lpf_horizontal_14_quad_avx2(
    s: *mut u8,
    p: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut mask: __m128i;
    let mut flat: __m128i;
    let zero: __m128i = _mm_setzero_si128();
    let ff: __m128i = _mm_cmpeq_epi8(zero, zero);
    let mut p256_3: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((4 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    let mut p256_2: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((3 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    let mut p256_1: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((2 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    let mut p256_0: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((1 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    let mut q256_0: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset(-((0 * p) as isize)) as *const __m128d as *mut __m128d,
    )));
    let mut q256_1: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((1 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    let mut q256_2: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((2 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    let mut q256_3: __m256i = _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
        s.offset((3 * p) as isize) as *const __m128d as *mut __m128d,
    )));
    let mut p3: __m128i = _mm256_castsi256_si128(p256_3);
    let mut p2: __m128i = _mm256_castsi256_si128(p256_2);
    let mut p1: __m128i = _mm256_castsi256_si128(p256_1);
    let mut p0: __m128i = _mm256_castsi256_si128(p256_0);
    let mut q0: __m128i = _mm256_castsi256_si128(q256_0);
    let mut q1: __m128i = _mm256_castsi256_si128(q256_1);
    let mut q2: __m128i = _mm256_castsi256_si128(q256_2);
    let mut q3: __m128i = _mm256_castsi256_si128(q256_3);
    {
        let limit_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_limit0 as i32));
        let blimit_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_blimit0 as i32));
        let fe: __m128i = _mm_set1_epi8(0xfe_i32 as i8);
        let abs_p1p0: __m128i = _mm_or_si128(_mm_subs_epu8(p1, p0), _mm_subs_epu8(p0, p1));
        let abs_q1q0: __m128i = _mm_or_si128(_mm_subs_epu8(q1, q0), _mm_subs_epu8(q0, q1));
        let mut abs_p0q0: __m128i = _mm_or_si128(_mm_subs_epu8(p0, q0), _mm_subs_epu8(q0, p0));
        let mut abs_p1q1: __m128i = _mm_or_si128(_mm_subs_epu8(p1, q1), _mm_subs_epu8(q1, p1));
        flat = _mm_max_epu8(abs_p1p0, abs_q1q0);
        abs_p0q0 = _mm_adds_epu8(abs_p0q0, abs_p0q0);
        abs_p1q1 = _mm_srli_epi16::<{ 1 }>(_mm_and_si128(abs_p1q1, fe));
        mask = _mm_subs_epu8(_mm_adds_epu8(abs_p0q0, abs_p1q1), blimit_v);
        mask = _mm_xor_si128(_mm_cmpeq_epi8(mask, zero), ff);
        // mask |= (abs(p0 - q0) * 2 + abs(p1 - q1) / 2  > blimit) * -1;
        mask = _mm_max_epu8(flat, mask);
        // mask |= (abs(p1 - p0) > limit) * -1;
        // mask |= (abs(q1 - q0) > limit) * -1;
        let mut work: __m128i = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(p2, p1), _mm_subs_epu8(p1, p2)),
            _mm_or_si128(_mm_subs_epu8(p3, p2), _mm_subs_epu8(p2, p3)),
        );
        mask = _mm_max_epu8(work, mask);
        work = _mm_max_epu8(
            _mm_or_si128(_mm_subs_epu8(q2, q1), _mm_subs_epu8(q1, q2)),
            _mm_or_si128(_mm_subs_epu8(q3, q2), _mm_subs_epu8(q2, q3)),
        );
        mask = _mm_max_epu8(work, mask);
        mask = _mm_subs_epu8(mask, limit_v);
        mask = _mm_cmpeq_epi8(mask, zero);
    }
    if 0xffff == _mm_movemask_epi8(_mm_cmpeq_epi8(mask, zero)) {
        return;
    }
    // loop filter
    {
        let thresh_v: __m128i = _mm_broadcastb_epi8(_mm_cvtsi32_si128(*_thresh0 as i32));
        let one: __m128i = _mm_set1_epi8(1);
        let t3: __m128i = _mm_set1_epi8(3);
        let t4: __m128i = _mm_add_epi8(one, t3);
        let t80: __m128i = _mm_set1_epi8(0x80_i32 as i8);
        let te0: __m128i = _mm_set1_epi8(0xe0_i32 as i8);
        let t1f: __m128i = _mm_set1_epi8(0x1f);
        let t7f: __m128i = _mm_sub_epi8(t80, one);
        let mut hev: __m128i = _mm_subs_epu8(flat, thresh_v);
        hev = _mm_xor_si128(_mm_cmpeq_epi8(hev, zero), ff);
        let mut ps1: __m128i = _mm_xor_si128(p1, t80);
        let mut ps0: __m128i = _mm_xor_si128(p0, t80);
        let mut qs0: __m128i = _mm_xor_si128(q0, t80);
        let mut qs1: __m128i = _mm_xor_si128(q1, t80);
        let mut filt: __m128i = _mm_and_si128(_mm_subs_epi8(ps1, qs1), hev);
        let mut work_a: __m128i = _mm_subs_epi8(qs0, ps0);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_adds_epi8(filt, work_a);
        filt = _mm_and_si128(filt, mask);
        let mut filter1: __m128i = _mm_adds_epi8(filt, t4);
        let mut filter2: __m128i = _mm_adds_epi8(filt, t3);
        work_a = _mm_cmpgt_epi8(zero, filter1);
        filter1 = _mm_srli_epi16::<{ 3 }>(filter1);
        work_a = _mm_and_si128(work_a, te0);
        filter1 = _mm_and_si128(filter1, t1f);
        filter1 = _mm_or_si128(filter1, work_a);
        qs0 = _mm_xor_si128(_mm_subs_epi8(qs0, filter1), t80);
        work_a = _mm_cmpgt_epi8(zero, filter2);
        filter2 = _mm_srli_epi16::<{ 3 }>(filter2);
        work_a = _mm_and_si128(work_a, te0);
        filter2 = _mm_and_si128(filter2, t1f);
        filter2 = _mm_or_si128(filter2, work_a);
        ps0 = _mm_xor_si128(_mm_adds_epi8(ps0, filter2), t80);
        filt = _mm_adds_epi8(filter1, one);
        work_a = _mm_cmpgt_epi8(zero, filt);
        filt = _mm_srli_epi16::<{ 1 }>(filt);
        work_a = _mm_and_si128(work_a, t80);
        filt = _mm_and_si128(filt, t7f);
        filt = _mm_or_si128(filt, work_a);
        filt = _mm_andnot_si128(hev, filt);
        ps1 = _mm_xor_si128(_mm_adds_epi8(ps1, filt), t80);
        qs1 = _mm_xor_si128(_mm_subs_epi8(qs1, filt), t80);
        // Derive flat
        let mut p0q0256: __m256i = _mm256_blend_epi32::<0xf0>(p256_0, q256_0);
        let mut p2q2256: __m256i = _mm256_blend_epi32::<0xf0>(p256_2, q256_2);
        let mut p3q3256: __m256i = _mm256_blend_epi32::<0xf0>(p256_3, q256_3);
        let ps0qs0256: __m256i = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(ps0), qs0);
        let ps1qs1256: __m256i = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(ps1), qs1);
        let work01: __m256i = _mm256_or_si256(
            _mm256_subs_epu8(p2q2256, p0q0256),
            _mm256_subs_epu8(p0q0256, p2q2256),
        );
        let work02: __m256i = _mm256_or_si256(
            _mm256_subs_epu8(p3q3256, p0q0256),
            _mm256_subs_epu8(p0q0256, p3q3256),
        );
        let max0_256: __m256i = _mm256_max_epu8(work01, work02);
        let max1_256: __m128i = _mm_max_epu8(
            _mm256_castsi256_si128(max0_256),
            _mm256_extractf128_si256::<1>(max0_256),
        );
        flat = _mm_max_epu8(max1_256, flat);
        flat = _mm_subs_epu8(flat, one);
        flat = _mm_cmpeq_epi8(flat, zero);
        flat = _mm_and_si128(flat, mask);
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // flat and wide flat calculations
        if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat, zero)) {
            let flat256: __m256i =
                _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(flat), flat);
            let eight: __m256i = _mm256_set1_epi16(8);
            let four: __m256i = _mm256_set1_epi16(4);
            let mut p256_4: __m256i =
                _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
                    s.offset(-((5 * p) as isize)) as *const __m128d as *mut __m128d,
                )));
            let mut q256_4: __m256i =
                _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
                    s.offset((4 * p) as isize) as *const __m128d as *mut __m128d,
                )));
            let mut p256_5: __m256i =
                _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
                    s.offset(-((6 * p) as isize)) as *const __m128d as *mut __m128d,
                )));
            let mut q256_5: __m256i =
                _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
                    s.offset((5 * p) as isize) as *const __m128d as *mut __m128d,
                )));
            let mut p256_6: __m256i =
                _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
                    s.offset(-((7 * p) as isize)) as *const __m128d as *mut __m128d,
                )));
            let mut q256_6: __m256i =
                _mm256_castpd_si256(_mm256_broadcast_pd(&core::ptr::read_unaligned(
                    s.offset((6 * p) as isize) as *const __m128d as *mut __m128d,
                )));
            // Derive flat2
            let mut p4q4256: __m256i = _mm256_blend_epi32::<0xf0>(p256_4, q256_4);
            let mut p5q5256: __m256i = _mm256_blend_epi32::<0xf0>(p256_5, q256_5);
            let p6q6256: __m256i = _mm256_blend_epi32::<0xf0>(p256_6, q256_6);
            let work1: __m256i = _mm256_or_si256(
                _mm256_subs_epu8(p4q4256, p0q0256),
                _mm256_subs_epu8(p0q0256, p4q4256),
            );
            let work2: __m256i = _mm256_or_si256(
                _mm256_subs_epu8(p5q5256, p0q0256),
                _mm256_subs_epu8(p0q0256, p5q5256),
            );
            let work3: __m256i = _mm256_or_si256(
                _mm256_subs_epu8(p6q6256, p0q0256),
                _mm256_subs_epu8(p0q0256, p6q6256),
            );
            let mut flat2_256: __m256i = _mm256_max_epu8(work1, work2);
            flat2_256 = _mm256_max_epu8(flat2_256, work3);
            let mut flat2: __m128i = _mm_max_epu8(
                _mm256_castsi256_si128(flat2_256),
                _mm256_extractf128_si256::<1>(flat2_256),
            );
            flat2 = _mm_subs_epu8(flat2, one);
            flat2 = _mm_cmpeq_epi8(flat2, zero);
            flat2 = _mm_and_si128(flat2, flat); // flat2 & flat & mask
            let filter: __m256i =
                _mm256_load_si256(filt_loopfilter_avx2.0.as_ptr() as *const __m256i);
            p256_3 = _mm256_shuffle_epi8(p256_3, filter);
            p256_2 = _mm256_shuffle_epi8(p256_2, filter);
            p256_1 = _mm256_shuffle_epi8(p256_1, filter);
            p256_0 = _mm256_shuffle_epi8(p256_0, filter);
            q256_0 = _mm256_shuffle_epi8(q256_0, filter);
            q256_1 = _mm256_shuffle_epi8(q256_1, filter);
            q256_2 = _mm256_shuffle_epi8(q256_2, filter);
            q256_3 = _mm256_shuffle_epi8(q256_3, filter);
            let p2p1p0: __m256i = _mm256_add_epi16(p256_0, _mm256_add_epi16(p256_2, p256_1));
            let q2q1q0: __m256i = _mm256_add_epi16(q256_0, _mm256_add_epi16(q256_2, q256_1));
            let mut pixetFilter_p2p1p0: __m256i =
                _mm256_add_epi16(four, _mm256_add_epi16(p2p1p0, q2q1q0));
            let mut pixetFilter_q2q1q0: __m256i = pixetFilter_p2p1p0;
            // Derive p0 and q0
            pixetFilter_p2p1p0 = _mm256_add_epi16(pixetFilter_p2p1p0, p256_3);
            let mut res_p: __m256i =
                _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_p2p1p0, p256_0));
            pixetFilter_q2q1q0 = _mm256_add_epi16(pixetFilter_q2q1q0, q256_3);
            let mut res_q: __m256i =
                _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_q2q1q0, q256_0));
            let mut flat_p0q0: __m256i =
                _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
            p0q0256 = _mm256_andnot_si256(flat256, ps0qs0256);
            flat_p0q0 = _mm256_and_si256(flat256, flat_p0q0);
            p0q0256 = _mm256_or_si256(flat_p0q0, p0q0256);
            p0 = _mm256_castsi256_si128(p0q0256);
            q0 = _mm256_extractf128_si256::<1>(p0q0256);
            // Derive p1 and q1
            let mut sum_p: __m256i = _mm256_sub_epi16(p256_3, q256_2);
            pixetFilter_p2p1p0 = _mm256_add_epi16(pixetFilter_p2p1p0, sum_p);
            res_p = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_p2p1p0, p256_1));
            let mut sum_q: __m256i = _mm256_sub_epi16(q256_3, p256_2);
            pixetFilter_q2q1q0 = _mm256_add_epi16(pixetFilter_q2q1q0, sum_q);
            res_q = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_q2q1q0, q256_1));
            let mut flat_p1q1: __m256i =
                _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
            let mut p1q1256: __m256i = _mm256_andnot_si256(flat256, ps1qs1256);
            flat_p1q1 = _mm256_and_si256(flat256, flat_p1q1);
            p1q1256 = _mm256_or_si256(flat_p1q1, p1q1256);
            p1 = _mm256_castsi256_si128(p1q1256);
            q1 = _mm256_extractf128_si256::<1>(p1q1256);
            // Derive p2 and q2
            sum_p = _mm256_sub_epi16(p256_3, q256_1);
            pixetFilter_p2p1p0 = _mm256_add_epi16(pixetFilter_p2p1p0, sum_p);
            res_p = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_p2p1p0, p256_2));
            sum_q = _mm256_sub_epi16(q256_3, p256_1);
            pixetFilter_q2q1q0 = _mm256_add_epi16(pixetFilter_q2q1q0, sum_q);
            res_q = _mm256_srli_epi16::<{ 3 }>(_mm256_add_epi16(pixetFilter_q2q1q0, q256_2));
            let mut flat_p2q2: __m256i =
                _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
            p2q2256 = _mm256_andnot_si256(flat256, p2q2256);
            flat_p2q2 = _mm256_and_si256(flat256, flat_p2q2);
            p2q2256 = _mm256_or_si256(flat_p2q2, p2q2256);
            p2 = _mm256_castsi256_si128(p2q2256);
            q2 = _mm256_extractf128_si256::<1>(p2q2256);
            if 0xffff != _mm_movemask_epi8(_mm_cmpeq_epi8(flat2, zero)) {
                flat2_256 = _mm256_insertf128_si256::<0x1>(_mm256_castsi128_si256(flat2), flat2);
                p256_6 = _mm256_shuffle_epi8(p256_6, filter);
                p256_5 = _mm256_shuffle_epi8(p256_5, filter);
                p256_4 = _mm256_shuffle_epi8(p256_4, filter);
                q256_4 = _mm256_shuffle_epi8(q256_4, filter);
                q256_5 = _mm256_shuffle_epi8(q256_5, filter);
                q256_6 = _mm256_shuffle_epi8(q256_6, filter);
                let mut pixelFilter_p: __m256i =
                    _mm256_add_epi16(p256_5, _mm256_add_epi16(p256_4, p256_3));
                let mut pixelFilter_q: __m256i =
                    _mm256_add_epi16(q256_5, _mm256_add_epi16(q256_4, q256_3));
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, p2p1p0);
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, q2q1q0);
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, p256_0);
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, q256_0);
                pixelFilter_p =
                    _mm256_add_epi16(eight, _mm256_add_epi16(pixelFilter_p, pixelFilter_q));
                pixelFilter_q = pixelFilter_p;
                // Derive p0 and q0
                pixelFilter_p = _mm256_add_epi16(_mm256_add_epi16(p256_6, p256_1), pixelFilter_p);
                res_p = _mm256_srli_epi16::<{ 4 }>(pixelFilter_p);
                pixelFilter_q = _mm256_add_epi16(_mm256_add_epi16(q256_6, q256_1), pixelFilter_q);
                res_q = _mm256_srli_epi16::<{ 4 }>(pixelFilter_q);
                let mut flat2_p0q0: __m256i =
                    _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
                p0q0256 = _mm256_andnot_si256(flat2_256, p0q0256);
                flat2_p0q0 = _mm256_and_si256(flat2_256, flat2_p0q0);
                p0q0256 = _mm256_or_si256(flat2_p0q0, p0q0256);
                p0 = _mm256_castsi256_si128(p0q0256);
                q0 = _mm256_extractf128_si256::<1>(p0q0256);
                _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, p0);
                _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, q0);
                // Derive p1 and q1
                sum_p = _mm256_add_epi16(
                    _mm256_sub_epi16(p256_6, q256_5),
                    _mm256_sub_epi16(p256_2, q256_0),
                );
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, sum_p);
                res_p = _mm256_srli_epi16::<{ 4 }>(pixelFilter_p);
                sum_q = _mm256_add_epi16(
                    _mm256_sub_epi16(q256_6, p256_5),
                    _mm256_sub_epi16(q256_2, p256_0),
                );
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, sum_q);
                res_q = _mm256_srli_epi16::<{ 4 }>(pixelFilter_q);
                let mut flat2_p1q1: __m256i =
                    _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
                p1q1256 = _mm256_andnot_si256(flat2_256, p1q1256);
                flat2_p1q1 = _mm256_and_si256(flat2_256, flat2_p1q1);
                p1q1256 = _mm256_or_si256(flat2_p1q1, p1q1256);
                p1 = _mm256_castsi256_si128(p1q1256);
                q1 = _mm256_extractf128_si256::<1>(p1q1256);
                _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, p1);
                _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, q1);
                // Derive p2 and q2
                sum_p = _mm256_add_epi16(
                    _mm256_sub_epi16(p256_6, q256_4),
                    _mm256_sub_epi16(p256_3, p256_0),
                );
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, sum_p);
                res_p = _mm256_srli_epi16::<{ 4 }>(pixelFilter_p);
                sum_q = _mm256_add_epi16(
                    _mm256_sub_epi16(q256_6, p256_4),
                    _mm256_sub_epi16(q256_3, q256_0),
                );
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, sum_q);
                res_q = _mm256_srli_epi16::<{ 4 }>(pixelFilter_q);
                let mut flat2_p2q2: __m256i =
                    _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
                p2q2256 = _mm256_andnot_si256(flat2_256, p2q2256);
                flat2_p2q2 = _mm256_and_si256(flat2_256, flat2_p2q2);
                p2q2256 = _mm256_or_si256(flat2_p2q2, p2q2256);
                p2 = _mm256_castsi256_si128(p2q2256);
                q2 = _mm256_extractf128_si256::<1>(p2q2256);
                _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
                _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, q2);
                // Derive p3 and q3
                sum_p = _mm256_add_epi16(
                    _mm256_sub_epi16(p256_6, q256_3),
                    _mm256_sub_epi16(p256_4, p256_1),
                );
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, sum_p);
                res_p = _mm256_srli_epi16::<{ 4 }>(pixelFilter_p);
                sum_q = _mm256_add_epi16(
                    _mm256_sub_epi16(q256_6, p256_3),
                    _mm256_sub_epi16(q256_4, q256_1),
                );
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, sum_q);
                res_q = _mm256_srli_epi16::<{ 4 }>(pixelFilter_q);
                let mut flat2_p3q3: __m256i =
                    _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
                p3q3256 = _mm256_andnot_si256(flat2_256, p3q3256);
                flat2_p3q3 = _mm256_and_si256(flat2_256, flat2_p3q3);
                p3q3256 = _mm256_or_si256(flat2_p3q3, p3q3256);
                p3 = _mm256_castsi256_si128(p3q3256);
                q3 = _mm256_extractf128_si256::<1>(p3q3256);
                _mm_storeu_si128(s.offset(-((4 * p) as isize)) as *mut __m128i, p3);
                _mm_storeu_si128(s.offset((3 * p) as isize) as *mut __m128i, q3);
                // Derive p4 and q4
                sum_p = _mm256_add_epi16(
                    _mm256_sub_epi16(p256_6, q256_2),
                    _mm256_sub_epi16(p256_5, p256_2),
                );
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, sum_p);
                res_p = _mm256_srli_epi16::<{ 4 }>(pixelFilter_p);
                sum_q = _mm256_add_epi16(
                    _mm256_sub_epi16(q256_6, p256_2),
                    _mm256_sub_epi16(q256_5, q256_2),
                );
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, sum_q);
                res_q = _mm256_srli_epi16::<{ 4 }>(pixelFilter_q);
                let mut flat2_p4q4: __m256i =
                    _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
                p4q4256 = _mm256_andnot_si256(flat2_256, p4q4256);
                flat2_p4q4 = _mm256_and_si256(flat2_256, flat2_p4q4);
                p4q4256 = _mm256_or_si256(flat2_p4q4, p4q4256);
                _mm_storeu_si128(
                    s.offset(-((5 * p) as isize)) as *mut __m128i,
                    _mm256_castsi256_si128(p4q4256),
                );
                _mm_storeu_si128(
                    s.offset((4 * p) as isize) as *mut __m128i,
                    _mm256_extractf128_si256::<1>(p4q4256),
                );
                // Derive p5 and q5
                sum_p = _mm256_add_epi16(
                    _mm256_sub_epi16(p256_6, q256_1),
                    _mm256_sub_epi16(p256_6, p256_3),
                );
                pixelFilter_p = _mm256_add_epi16(pixelFilter_p, sum_p);
                res_p = _mm256_srli_epi16::<{ 4 }>(pixelFilter_p);
                sum_q = _mm256_add_epi16(
                    _mm256_sub_epi16(q256_6, p256_1),
                    _mm256_sub_epi16(q256_6, q256_3),
                );
                pixelFilter_q = _mm256_add_epi16(pixelFilter_q, sum_q);
                res_q = _mm256_srli_epi16::<{ 4 }>(pixelFilter_q);
                let mut flat2_p5q5: __m256i =
                    _mm256_permute4x64_epi64::<0xd8>(_mm256_packus_epi16(res_p, res_q));
                p5q5256 = _mm256_andnot_si256(flat2_256, p5q5256);
                flat2_p5q5 = _mm256_and_si256(flat2_256, flat2_p5q5);
                p5q5256 = _mm256_or_si256(flat2_p5q5, p5q5256);
                _mm_storeu_si128(
                    s.offset(-((6 * p) as isize)) as *mut __m128i,
                    _mm256_castsi256_si128(p5q5256),
                );
                _mm_storeu_si128(
                    s.offset((5 * p) as isize) as *mut __m128i,
                    _mm256_extractf128_si256::<1>(p5q5256),
                );
            } else {
                _mm_storeu_si128(s.offset(-((3 * p) as isize)) as *mut __m128i, p2);
                _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, p1);
                _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, p0);
                _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, q0);
                _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, q1);
                _mm_storeu_si128(s.offset((2 * p) as isize) as *mut __m128i, q2);
            }
        } else {
            _mm_storeu_si128(s.offset(-((2 * p) as isize)) as *mut __m128i, ps1);
            _mm_storeu_si128(s.offset(-((1 * p) as isize)) as *mut __m128i, ps0);
            _mm_storeu_si128(s.offset(-((0 * p) as isize)) as *mut __m128i, qs0);
            _mm_storeu_si128(s.offset((1 * p) as isize) as *mut __m128i, qs1);
        }
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn aom_lpf_vertical_14_quad_avx2(
    s: *mut u8,
    pitch: i32,
    _blimit0: *const u8,
    _limit0: *const u8,
    _thresh0: *const u8,
) {
    let mut t_dst: Aligned16<[u8; 256]> = core::mem::zeroed();
    // Transpose 16x16
    trans_store_16x16_lpf_vert14(s.offset(-8), pitch, t_dst.0.as_mut_ptr(), 16, 1);
    // Loop filtering
    aom_lpf_horizontal_14_quad_avx2(
        t_dst.0.as_mut_ptr().offset((8 * 16) as isize),
        16,
        _blimit0,
        _limit0,
        _thresh0,
    );
    // Transpose back
    trans_store_16x16_lpf_vert14(t_dst.0.as_mut_ptr(), 16, s.offset(-8), pitch, 0);
}
