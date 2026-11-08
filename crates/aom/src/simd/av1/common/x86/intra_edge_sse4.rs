/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/av1/common/x86/intra_edge_sse4.c
// (ADR 0056, ADR 0058). Upstream compiles this file with -msse4.1.
//
// Translation notes:
// - The C local `in` is a Rust keyword and is spelled `in_`.
// - DECLARE_ALIGNED(16, static const ...) tables stay function-local statics
//   in a 16-byte aligned wrapper; `v_const` keeps its declared five rows, the
//   fifth zero-initialised as in C.
// - `(char)` / `int` arguments to `_mm_set1_epi8` and `_mm_set1_epi16` are
//   cast to `i8` / `i16`, the parameter types of the intrinsics, which keeps
//   the low 8 / 16 bits as the C conversions do.
// - The CONFIG_AV1_HIGHBITDEPTH half is kept: the translated configuration
//   enables high bit depth.
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_filter_intra_edge_sse4_1(p: *mut u8, sz: i32, strength: i32) {
    if strength == 0 {
        return;
    }

    #[rustfmt::skip]
    static kern: Aligned16<[[i8; 16]; 3]> = Aligned16([
        [ 4, 8, 4, 0, 4, 8, 4, 0, 4, 8, 4, 0, 4, 8, 4, 0 ],  // strength 1: 4,8,4
        [ 5, 6, 5, 0, 5, 6, 5, 0, 5, 6, 5, 0, 5, 6, 5, 0 ],  // strength 2: 5,6,5
        [ 2, 4, 4, 4, 2, 0, 0, 0, 2, 4, 4, 4, 2, 0, 0, 0 ],  // strength 3: 2,4,4,4,2
    ]);

    #[rustfmt::skip]
    static v_const: Aligned16<[[i8; 16]; 5]> = Aligned16([
        [ 0, 1, 2, 3, 1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6 ],
        [ 4, 5, 6, 7, 5, 6, 7, 8, 6, 7, 8, 9, 7, 8, 9, 10 ],
        [ 0, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 8 ],
        [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 ],
        [ 0; 16 ],
    ]);

    // Extend the first and last samples to simplify the loop for the 5-tap case
    *p.offset(-1) = *p;
    let last = _mm_set1_epi8(*p.offset((sz - 1) as isize) as i8);
    _mm_storeu_si128(p.offset(sz as isize) as *mut __m128i, last);

    // Adjust input pointer for filter support area
    let mut in_: *mut u8 = if strength == 3 { p.offset(-1) } else { p };

    // Avoid modifying first sample
    let mut out: *mut u8 = p.add(1);
    let mut len = sz - 1;

    let use_3tap_filter = (strength < 3) as i32;

    if use_3tap_filter != 0 {
        let coef0 = _mm_lddqu_si128(kern.0[(strength - 1) as usize].as_ptr() as *const __m128i);
        let shuf0 = _mm_lddqu_si128(v_const.0[0].as_ptr() as *const __m128i);
        let shuf1 = _mm_lddqu_si128(v_const.0[1].as_ptr() as *const __m128i);
        let iden = _mm_lddqu_si128(v_const.0[3].as_ptr() as *const __m128i);
        let mut in0 = _mm_lddqu_si128(in_ as *const __m128i);
        while len > 0 {
            let n_out = if len < 8 { len } else { 8 };
            let mut d0 = _mm_shuffle_epi8(in0, shuf0);
            let mut d1 = _mm_shuffle_epi8(in0, shuf1);
            d0 = _mm_maddubs_epi16(d0, coef0);
            d1 = _mm_maddubs_epi16(d1, coef0);
            d0 = _mm_hadd_epi16(d0, d1);
            let eight = _mm_set1_epi16(8);
            d0 = _mm_add_epi16(d0, eight);
            d0 = _mm_srai_epi16::<4>(d0);
            d0 = _mm_packus_epi16(d0, d0);
            let mut out0 = _mm_lddqu_si128(out as *const __m128i);
            let n0 = _mm_set1_epi8(n_out as i8);
            let mask = _mm_cmpgt_epi8(n0, iden);
            out0 = _mm_blendv_epi8(out0, d0, mask);
            _mm_storel_epi64(out as *mut __m128i, out0);
            let in1 = _mm_lddqu_si128(in_.add(16) as *const __m128i);
            in0 = _mm_alignr_epi8::<8>(in1, in0);
            in_ = in_.add(8);
            out = out.add(8);
            len -= n_out;
        }
    } else {
        // 5-tap filter
        let coef0 = _mm_lddqu_si128(kern.0[(strength - 1) as usize].as_ptr() as *const __m128i);
        let two = _mm_set1_epi8(2);
        let shuf_a = _mm_lddqu_si128(v_const.0[2].as_ptr() as *const __m128i);
        let shuf_b = _mm_add_epi8(shuf_a, two);
        let shuf_c = _mm_add_epi8(shuf_b, two);
        let shuf_d = _mm_add_epi8(shuf_c, two);
        let iden = _mm_lddqu_si128(v_const.0[3].as_ptr() as *const __m128i);
        let mut in0 = _mm_lddqu_si128(in_ as *const __m128i);
        while len > 0 {
            let n_out = if len < 8 { len } else { 8 };
            let mut d0 = _mm_shuffle_epi8(in0, shuf_a);
            let mut d1 = _mm_shuffle_epi8(in0, shuf_b);
            let mut d2 = _mm_shuffle_epi8(in0, shuf_c);
            let mut d3 = _mm_shuffle_epi8(in0, shuf_d);
            d0 = _mm_maddubs_epi16(d0, coef0);
            d1 = _mm_maddubs_epi16(d1, coef0);
            d2 = _mm_maddubs_epi16(d2, coef0);
            d3 = _mm_maddubs_epi16(d3, coef0);
            d0 = _mm_hadd_epi16(d0, d1);
            d2 = _mm_hadd_epi16(d2, d3);
            d0 = _mm_hadd_epi16(d0, d2);
            let eight = _mm_set1_epi16(8);
            d0 = _mm_add_epi16(d0, eight);
            d0 = _mm_srai_epi16::<4>(d0);
            d0 = _mm_packus_epi16(d0, d0);
            let mut out0 = _mm_lddqu_si128(out as *const __m128i);
            let n0 = _mm_set1_epi8(n_out as i8);
            let mask = _mm_cmpgt_epi8(n0, iden);
            out0 = _mm_blendv_epi8(out0, d0, mask);
            _mm_storel_epi64(out as *mut __m128i, out0);
            let in1 = _mm_lddqu_si128(in_.add(16) as *const __m128i);
            in0 = _mm_alignr_epi8::<8>(in1, in0);
            in_ = in_.add(8);
            out = out.add(8);
            len -= n_out;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_upsample_intra_edge_sse4_1(p: *mut u8, sz: i32) {
    // interpolate half-sample positions
    debug_assert!(sz <= 24);

    #[rustfmt::skip]
    static kernel: Aligned16<[[i8; 16]; 1]> = Aligned16([
        [ -1, 9, 9, -1, -1, 9, 9, -1, -1, 9, 9, -1, -1, 9, 9, -1 ],
    ]);

    #[rustfmt::skip]
    static v_const: Aligned16<[[i8; 16]; 2]> = Aligned16([
        [ 0, 1, 2, 3, 1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6 ],
        [ 4, 5, 6, 7, 5, 6, 7, 8, 6, 7, 8, 9, 7, 8, 9, 10 ],
    ]);

    // Extend first/last samples (upper-left p[-1], last p[sz-1])
    // to support 4-tap filter
    *p.offset(-2) = *p.offset(-1);
    *p.offset(sz as isize) = *p.offset((sz - 1) as isize);

    let in_: *mut u8 = p.offset(-2);
    let mut out: *mut u8 = p.offset(-2);

    let mut n = sz + 1; // Input length including upper-left sample

    let mut in0 = _mm_lddqu_si128(in_ as *const __m128i);
    let mut in16 = _mm_lddqu_si128(in_.add(16) as *const __m128i);

    let coef0 = _mm_lddqu_si128(kernel.0[0].as_ptr() as *const __m128i);
    let shuf0 = _mm_lddqu_si128(v_const.0[0].as_ptr() as *const __m128i);
    let shuf1 = _mm_lddqu_si128(v_const.0[1].as_ptr() as *const __m128i);

    while n > 0 {
        let in8 = _mm_alignr_epi8::<8>(in16, in0);
        let mut d0 = _mm_shuffle_epi8(in0, shuf0);
        let mut d1 = _mm_shuffle_epi8(in0, shuf1);
        let mut d2 = _mm_shuffle_epi8(in8, shuf0);
        let mut d3 = _mm_shuffle_epi8(in8, shuf1);
        d0 = _mm_maddubs_epi16(d0, coef0);
        d1 = _mm_maddubs_epi16(d1, coef0);
        d2 = _mm_maddubs_epi16(d2, coef0);
        d3 = _mm_maddubs_epi16(d3, coef0);
        d0 = _mm_hadd_epi16(d0, d1);
        d2 = _mm_hadd_epi16(d2, d3);
        let eight = _mm_set1_epi16(8);
        d0 = _mm_add_epi16(d0, eight);
        d2 = _mm_add_epi16(d2, eight);
        d0 = _mm_srai_epi16::<4>(d0);
        d2 = _mm_srai_epi16::<4>(d2);
        d0 = _mm_packus_epi16(d0, d2);
        let in1 = _mm_alignr_epi8::<1>(in16, in0);
        let out0 = _mm_unpacklo_epi8(in1, d0);
        let out1 = _mm_unpackhi_epi8(in1, d0);
        _mm_storeu_si128(out as *mut __m128i, out0);
        _mm_storeu_si128(out.add(16) as *mut __m128i, out1);
        in0 = in16;
        in16 = _mm_setzero_si128();
        out = out.add(32);
        n -= 16;
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_highbd_filter_intra_edge_sse4_1(p: *mut u16, sz: i32, strength: i32) {
    if strength == 0 {
        return;
    }

    #[rustfmt::skip]
    static kern: Aligned16<[[i16; 8]; 3]> = Aligned16([
        [ 4, 8, 4, 8, 4, 8, 4, 8 ],  // strength 1: 4,8,4
        [ 5, 6, 5, 6, 5, 6, 5, 6 ],  // strength 2: 5,6,5
        [ 2, 4, 2, 4, 2, 4, 2, 4 ],  // strength 3: 2,4,4,4,2
    ]);

    #[rustfmt::skip]
    static v_const: Aligned16<[[i16; 8]; 1]> = Aligned16([[ 0, 1, 2, 3, 4, 5, 6, 7 ]]);

    // Extend the first and last samples to simplify the loop for the 5-tap case
    *p.offset(-1) = *p;
    let last = _mm_set1_epi16(*p.offset((sz - 1) as isize) as i16);
    _mm_storeu_si128(p.offset(sz as isize) as *mut __m128i, last);

    // Adjust input pointer for filter support area
    let mut in_: *mut u16 = if strength == 3 { p.offset(-1) } else { p };

    // Avoid modifying first sample
    let mut out: *mut u16 = p.add(1);
    let mut len = sz - 1;

    let use_3tap_filter = (strength < 3) as i32;

    if use_3tap_filter != 0 {
        let coef0 = _mm_lddqu_si128(kern.0[(strength - 1) as usize].as_ptr() as *const __m128i);
        let iden = _mm_lddqu_si128(v_const.0[0].as_ptr() as *const __m128i);
        let mut in0 = _mm_lddqu_si128(in_ as *const __m128i);
        let mut in8 = _mm_lddqu_si128(in_.add(8) as *const __m128i);
        while len > 0 {
            let n_out = if len < 8 { len } else { 8 };
            let in1 = _mm_alignr_epi8::<2>(in8, in0);
            let in2 = _mm_alignr_epi8::<4>(in8, in0);
            let in02 = _mm_add_epi16(in0, in2);
            let mut d0 = _mm_unpacklo_epi16(in02, in1);
            let mut d1 = _mm_unpackhi_epi16(in02, in1);
            d0 = _mm_mullo_epi16(d0, coef0);
            d1 = _mm_mullo_epi16(d1, coef0);
            d0 = _mm_hadd_epi16(d0, d1);
            let eight = _mm_set1_epi16(8);
            d0 = _mm_add_epi16(d0, eight);
            d0 = _mm_srli_epi16::<4>(d0);
            let mut out0 = _mm_lddqu_si128(out as *const __m128i);
            let n0 = _mm_set1_epi16(n_out as i16);
            let mask = _mm_cmpgt_epi16(n0, iden);
            out0 = _mm_blendv_epi8(out0, d0, mask);
            _mm_storeu_si128(out as *mut __m128i, out0);
            in_ = in_.add(8);
            in0 = in8;
            in8 = _mm_lddqu_si128(in_.add(8) as *const __m128i);
            out = out.add(8);
            len -= n_out;
        }
    } else {
        // 5-tap filter
        let coef0 = _mm_lddqu_si128(kern.0[(strength - 1) as usize].as_ptr() as *const __m128i);
        let iden = _mm_lddqu_si128(v_const.0[0].as_ptr() as *const __m128i);
        let mut in0 = _mm_lddqu_si128(in_ as *const __m128i);
        let mut in8 = _mm_lddqu_si128(in_.add(8) as *const __m128i);
        while len > 0 {
            let n_out = if len < 8 { len } else { 8 };
            let in1 = _mm_alignr_epi8::<2>(in8, in0);
            let in2 = _mm_alignr_epi8::<4>(in8, in0);
            let in3 = _mm_alignr_epi8::<6>(in8, in0);
            let in4 = _mm_alignr_epi8::<8>(in8, in0);
            let in04 = _mm_add_epi16(in0, in4);
            let mut in123 = _mm_add_epi16(in1, in2);
            in123 = _mm_add_epi16(in123, in3);
            let mut d0 = _mm_unpacklo_epi16(in04, in123);
            let mut d1 = _mm_unpackhi_epi16(in04, in123);
            d0 = _mm_mullo_epi16(d0, coef0);
            d1 = _mm_mullo_epi16(d1, coef0);
            d0 = _mm_hadd_epi16(d0, d1);
            let eight = _mm_set1_epi16(8);
            d0 = _mm_add_epi16(d0, eight);
            d0 = _mm_srli_epi16::<4>(d0);
            let mut out0 = _mm_lddqu_si128(out as *const __m128i);
            let n0 = _mm_set1_epi16(n_out as i16);
            let mask = _mm_cmpgt_epi16(n0, iden);
            out0 = _mm_blendv_epi8(out0, d0, mask);
            _mm_storeu_si128(out as *mut __m128i, out0);
            in_ = in_.add(8);
            in0 = in8;
            in8 = _mm_lddqu_si128(in_.add(8) as *const __m128i);
            out = out.add(8);
            len -= n_out;
        }
    }
}

#[no_mangle]
#[target_feature(enable = "sse4.1")]
pub unsafe extern "C-unwind" fn av1_highbd_upsample_intra_edge_sse4_1(p: *mut u16, sz: i32, bd: i32) {
    // interpolate half-sample positions
    debug_assert!(sz <= 24);

    #[rustfmt::skip]
    static kernel: Aligned16<[[i16; 8]; 1]> = Aligned16([[ -1, 9, -1, 9, -1, 9, -1, 9 ]]);

    // Extend first/last samples (upper-left p[-1], last p[sz-1])
    // to support 4-tap filter
    *p.offset(-2) = *p.offset(-1);
    *p.offset(sz as isize) = *p.offset((sz - 1) as isize);

    let in_: *mut u16 = p.offset(-2);
    let mut out: *mut u16 = in_;
    let mut n = sz + 1;

    let mut in0 = _mm_lddqu_si128(in_ as *const __m128i);
    let mut in8 = _mm_lddqu_si128(in_.add(8) as *const __m128i);
    let mut in16 = _mm_lddqu_si128(in_.add(16) as *const __m128i);
    let mut in24 = _mm_lddqu_si128(in_.add(24) as *const __m128i);

    while n > 0 {
        let in1 = _mm_alignr_epi8::<2>(in8, in0);
        let in2 = _mm_alignr_epi8::<4>(in8, in0);
        let in3 = _mm_alignr_epi8::<6>(in8, in0);
        let sum0 = _mm_add_epi16(in0, in3);
        let sum1 = _mm_add_epi16(in1, in2);
        let mut d0 = _mm_unpacklo_epi16(sum0, sum1);
        let mut d1 = _mm_unpackhi_epi16(sum0, sum1);
        let coef0 = _mm_lddqu_si128(kernel.0[0].as_ptr() as *const __m128i);
        d0 = _mm_madd_epi16(d0, coef0);
        d1 = _mm_madd_epi16(d1, coef0);
        let eight = _mm_set1_epi32(8);
        d0 = _mm_add_epi32(d0, eight);
        d1 = _mm_add_epi32(d1, eight);
        d0 = _mm_srai_epi32::<4>(d0);
        d1 = _mm_srai_epi32::<4>(d1);
        d0 = _mm_packus_epi32(d0, d1);
        let max0 = _mm_set1_epi16(((1 << bd) - 1) as i16);
        d0 = _mm_min_epi16(d0, max0);
        let out0 = _mm_unpacklo_epi16(in1, d0);
        let out1 = _mm_unpackhi_epi16(in1, d0);
        _mm_storeu_si128(out as *mut __m128i, out0);
        _mm_storeu_si128(out.add(8) as *mut __m128i, out1);
        in0 = in8;
        in8 = in16;
        in16 = in24;
        in24 = _mm_setzero_si128();
        out = out.add(16);
        n -= 8;
    }
}
