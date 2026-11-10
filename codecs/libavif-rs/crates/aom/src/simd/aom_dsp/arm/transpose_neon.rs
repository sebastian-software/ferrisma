/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/arm/transpose_neon.h (ADR 0056).
// Intrinsic sequences, helper structure, and names follow the original; only
// the AOM_ARCH_AARCH64 branches are retained. Pointer and array parameters stay
// raw pointers (`T *` -> `*mut T`, `const T *` -> `*const T`), so call sites
// keep the C shape (`&mut x` coerces to `*mut T`).
#![allow(dead_code)]

use core::arch::aarch64::*;

#[inline(always)]
pub unsafe fn transpose_concat_elems_u8_4x4(a0: uint8x8_t, a1: uint8x8_t, a2: uint8x8_t,
                                            a3: uint8x8_t, b: *mut uint8x16_t) {
    // Transpose 8-bit elements and concatenate result rows as follows:
    // a0: 00, 01, 02, 03, XX, XX, XX, XX
    // a1: 10, 11, 12, 13, XX, XX, XX, XX
    // a2: 20, 21, 22, 23, XX, XX, XX, XX
    // a3: 30, 31, 32, 33, XX, XX, XX, XX
    //
    // b: 00, 10, 20, 30, 01, 11, 21, 31, 02, 12, 22, 32, 03, 13, 23, 33

    let a0q = vcombine_u8(a0, vdup_n_u8(0));
    let a1q = vcombine_u8(a1, vdup_n_u8(0));
    let a2q = vcombine_u8(a2, vdup_n_u8(0));
    let a3q = vcombine_u8(a3, vdup_n_u8(0));

    let a02 = vzipq_u8(a0q, a2q).0;
    let a13 = vzipq_u8(a1q, a3q).0;

    *b = vzipq_u8(a02, a13).0;
}

#[inline(always)]
pub unsafe fn transpose_concat_elems_u8_8x4(a0: uint8x8_t, a1: uint8x8_t, a2: uint8x8_t,
                                            a3: uint8x8_t, b0: *mut uint8x16_t,
                                            b1: *mut uint8x16_t) {
    // Transpose 8-bit elements and concatenate result rows as follows:
    // a0: 00, 01, 02, 03, 04, 05, 06, 07
    // a1: 10, 11, 12, 13, 14, 15, 16, 17
    // a2: 20, 21, 22, 23, 24, 25, 26, 27
    // a3: 30, 31, 32, 33, 34, 35, 36, 37
    //
    // b0: 00, 10, 20, 30, 01, 11, 21, 31, 02, 12, 22, 32, 03, 13, 23, 33
    // b1: 04, 14, 24, 34, 05, 15, 25, 35, 06, 16, 26, 36, 07, 17, 27, 37

    let a0q = vcombine_u8(a0, vdup_n_u8(0));
    let a1q = vcombine_u8(a1, vdup_n_u8(0));
    let a2q = vcombine_u8(a2, vdup_n_u8(0));
    let a3q = vcombine_u8(a3, vdup_n_u8(0));

    let a02 = vzipq_u8(a0q, a2q).0;
    let a13 = vzipq_u8(a1q, a3q).0;

    let a0123 = vzipq_u8(a02, a13);

    *b0 = a0123.0;
    *b1 = a0123.1;
}

#[inline(always)]
pub unsafe fn transpose_concat_elems_s8_4x4(a0: int8x8_t, a1: int8x8_t, a2: int8x8_t,
                                            a3: int8x8_t, b: *mut int8x16_t) {
    // Transpose 8-bit elements and concatenate result rows as follows:
    // a0: 00, 01, 02, 03, XX, XX, XX, XX
    // a1: 10, 11, 12, 13, XX, XX, XX, XX
    // a2: 20, 21, 22, 23, XX, XX, XX, XX
    // a3: 30, 31, 32, 33, XX, XX, XX, XX
    //
    // b: 00, 10, 20, 30, 01, 11, 21, 31, 02, 12, 22, 32, 03, 13, 23, 33

    let a0q = vcombine_s8(a0, vdup_n_s8(0));
    let a1q = vcombine_s8(a1, vdup_n_s8(0));
    let a2q = vcombine_s8(a2, vdup_n_s8(0));
    let a3q = vcombine_s8(a3, vdup_n_s8(0));

    let a02 = vzipq_s8(a0q, a2q).0;
    let a13 = vzipq_s8(a1q, a3q).0;

    *b = vzipq_s8(a02, a13).0;
}

#[inline(always)]
pub unsafe fn transpose_concat_elems_s8_8x4(a0: int8x8_t, a1: int8x8_t, a2: int8x8_t,
                                            a3: int8x8_t, b0: *mut int8x16_t,
                                            b1: *mut int8x16_t) {
    // Transpose 8-bit elements and concatenate result rows as follows:
    // a0: 00, 01, 02, 03, 04, 05, 06, 07
    // a1: 10, 11, 12, 13, 14, 15, 16, 17
    // a2: 20, 21, 22, 23, 24, 25, 26, 27
    // a3: 30, 31, 32, 33, 34, 35, 36, 37
    //
    // b0: 00, 10, 20, 30, 01, 11, 21, 31, 02, 12, 22, 32, 03, 13, 23, 33
    // b1: 04, 14, 24, 34, 05, 15, 25, 35, 06, 16, 26, 36, 07, 17, 27, 37

    let a0q = vcombine_s8(a0, vdup_n_s8(0));
    let a1q = vcombine_s8(a1, vdup_n_s8(0));
    let a2q = vcombine_s8(a2, vdup_n_s8(0));
    let a3q = vcombine_s8(a3, vdup_n_s8(0));

    let a02 = vzipq_s8(a0q, a2q).0;
    let a13 = vzipq_s8(a1q, a3q).0;

    let a0123 = vzipq_s8(a02, a13);

    *b0 = a0123.0;
    *b1 = a0123.1;
}

#[inline(always)]
pub unsafe fn transpose_concat_elems_s16_4x4(s0: int16x4_t, s1: int16x4_t, s2: int16x4_t,
                                             s3: int16x4_t, res: *mut int16x8_t) {
    // Transpose 16-bit elements and concatenate result rows as follows:
    // s0: 00, 01, 02, 03
    // s1: 10, 11, 12, 13
    // s2: 20, 21, 22, 23
    // s3: 30, 31, 32, 33
    //
    // res[0]: 00 10 20 30 01 11 21 31
    // res[1]: 02 12 22 32 03 13 23 33

    let s0q = vcombine_s16(s0, vdup_n_s16(0));
    let s1q = vcombine_s16(s1, vdup_n_s16(0));
    let s2q = vcombine_s16(s2, vdup_n_s16(0));
    let s3q = vcombine_s16(s3, vdup_n_s16(0));

    let s02 = vzipq_s16(s0q, s2q).0;
    let s13 = vzipq_s16(s1q, s3q).0;

    let s0123 = vzipq_s16(s02, s13);

    *res.add(0) = s0123.0;
    *res.add(1) = s0123.1;
}

#[inline(always)]
pub unsafe fn transpose_concat_elems_s16_8x4(s0: int16x8_t, s1: int16x8_t, s2: int16x8_t,
                                             s3: int16x8_t, res: *mut int16x8_t) {
    // Transpose 16-bit elements and concatenate result rows as follows:
    // s0: 00, 01, 02, 03, 04, 05, 06, 07
    // s1: 10, 11, 12, 13, 14, 15, 16, 17
    // s2: 20, 21, 22, 23, 24, 25, 26, 27
    // s3: 30, 31, 32, 33, 34, 35, 36, 37
    //
    // res[0]: 00 10 20 30 01 11 21 31
    // res[1]: 02 12 22 32 03 13 23 33
    // res[2]: 04 14 24 34 05 15 25 35
    // res[3]: 06 16 26 36 07 17 27 37

    let s02 = vzipq_s16(s0, s2);
    let s13 = vzipq_s16(s1, s3);

    let s0123_lo = vzipq_s16(s02.0, s13.0);
    let s0123_hi = vzipq_s16(s02.1, s13.1);

    *res.add(0) = s0123_lo.0;
    *res.add(1) = s0123_lo.1;
    *res.add(2) = s0123_hi.0;
    *res.add(3) = s0123_hi.1;
}

#[inline(always)]
pub unsafe fn transpose_elems_u8_8x8(
    a0: uint8x8_t, a1: uint8x8_t, a2: uint8x8_t, a3: uint8x8_t, a4: uint8x8_t, a5: uint8x8_t,
    a6: uint8x8_t, a7: uint8x8_t, o0: *mut uint8x8_t, o1: *mut uint8x8_t, o2: *mut uint8x8_t,
    o3: *mut uint8x8_t, o4: *mut uint8x8_t, o5: *mut uint8x8_t, o6: *mut uint8x8_t,
    o7: *mut uint8x8_t,
) {
    // Swap 8 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07
    // a1: 10 11 12 13 14 15 16 17
    // a2: 20 21 22 23 24 25 26 27
    // a3: 30 31 32 33 34 35 36 37
    // a4: 40 41 42 43 44 45 46 47
    // a5: 50 51 52 53 54 55 56 57
    // a6: 60 61 62 63 64 65 66 67
    // a7: 70 71 72 73 74 75 76 77
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16  40 50 42 52 44 54 46 56
    // b0.val[1]: 01 11 03 13 05 15 07 17  41 51 43 53 45 55 47 57
    // b1.val[0]: 20 30 22 32 24 34 26 36  60 70 62 72 64 74 66 76
    // b1.val[1]: 21 31 23 33 25 35 27 37  61 71 63 73 65 75 67 77

    let b0 = vtrnq_u8(vcombine_u8(a0, a4), vcombine_u8(a1, a5));
    let b1 = vtrnq_u8(vcombine_u8(a2, a6), vcombine_u8(a3, a7));

    // Swap 16 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34  40 50 60 70 44 54 64 74
    // c0.val[1]: 02 12 22 32 06 16 26 36  42 52 62 72 46 56 66 76
    // c1.val[0]: 01 11 21 31 05 15 25 35  41 51 61 71 45 55 65 75
    // c1.val[1]: 03 13 23 33 07 17 27 37  43 53 63 73 47 57 67 77

    let c0 = vtrnq_u16(vreinterpretq_u16_u8(b0.0), vreinterpretq_u16_u8(b1.0));
    let c1 = vtrnq_u16(vreinterpretq_u16_u8(b0.1), vreinterpretq_u16_u8(b1.1));

    // Unzip 32 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70  01 11 21 31 41 51 61 71
    // d0.val[1]: 04 14 24 34 44 54 64 74  05 15 25 35 45 55 65 75
    // d1.val[0]: 02 12 22 32 42 52 62 72  03 13 23 33 43 53 63 73
    // d1.val[1]: 06 16 26 36 46 56 66 76  07 17 27 37 47 57 67 77
    let d0 = vuzpq_u32(vreinterpretq_u32_u16(c0.0), vreinterpretq_u32_u16(c1.0));
    let d1 = vuzpq_u32(vreinterpretq_u32_u16(c0.1), vreinterpretq_u32_u16(c1.1));

    *o0 = vreinterpret_u8_u32(vget_low_u32(d0.0));
    *o1 = vreinterpret_u8_u32(vget_high_u32(d0.0));
    *o2 = vreinterpret_u8_u32(vget_low_u32(d1.0));
    *o3 = vreinterpret_u8_u32(vget_high_u32(d1.0));
    *o4 = vreinterpret_u8_u32(vget_low_u32(d0.1));
    *o5 = vreinterpret_u8_u32(vget_high_u32(d0.1));
    *o6 = vreinterpret_u8_u32(vget_low_u32(d1.1));
    *o7 = vreinterpret_u8_u32(vget_high_u32(d1.1));
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_u8_8x8(a0: *mut uint8x8_t, a1: *mut uint8x8_t,
                                             a2: *mut uint8x8_t, a3: *mut uint8x8_t,
                                             a4: *mut uint8x8_t, a5: *mut uint8x8_t,
                                             a6: *mut uint8x8_t, a7: *mut uint8x8_t) {
    transpose_elems_u8_8x8(*a0, *a1, *a2, *a3, *a4, *a5, *a6, *a7, a0, a1, a2, a3, a4, a5, a6,
                           a7);
}

#[inline(always)]
pub unsafe fn transpose_arrays_u8_8x8(in_: *const uint8x8_t, out: *mut uint8x8_t) {
    transpose_elems_u8_8x8(*in_.add(0), *in_.add(1), *in_.add(2), *in_.add(3), *in_.add(4),
                           *in_.add(5), *in_.add(6), *in_.add(7), out.add(0), out.add(1),
                           out.add(2), out.add(3), out.add(4), out.add(5), out.add(6),
                           out.add(7));
}

#[inline(always)]
pub unsafe fn transpose_arrays_u8_8x16(x: *const uint8x8_t, d: *mut uint8x16_t) {
    let x = |i: usize| *x.add(i);
    let w0 = vzip_u8(x(0), x(1));
    let w1 = vzip_u8(x(2), x(3));
    let w2 = vzip_u8(x(4), x(5));
    let w3 = vzip_u8(x(6), x(7));

    let w8 = vzip_u8(x(8), x(9));
    let w9 = vzip_u8(x(10), x(11));
    let w10 = vzip_u8(x(12), x(13));
    let w11 = vzip_u8(x(14), x(15));

    let mut w4 = vzip_u16(vreinterpret_u16_u8(w0.0), vreinterpret_u16_u8(w1.0));
    let mut w5 = vzip_u16(vreinterpret_u16_u8(w2.0), vreinterpret_u16_u8(w3.0));
    let mut w12 = vzip_u16(vreinterpret_u16_u8(w8.0), vreinterpret_u16_u8(w9.0));
    let mut w13 = vzip_u16(vreinterpret_u16_u8(w10.0), vreinterpret_u16_u8(w11.0));

    let mut w6 = vzip_u32(vreinterpret_u32_u16(w4.0), vreinterpret_u32_u16(w5.0));
    let mut w7 = vzip_u32(vreinterpret_u32_u16(w4.1), vreinterpret_u32_u16(w5.1));
    let mut w14 = vzip_u32(vreinterpret_u32_u16(w12.0), vreinterpret_u32_u16(w13.0));
    let mut w15 = vzip_u32(vreinterpret_u32_u16(w12.1), vreinterpret_u32_u16(w13.1));

    // Store first 4-line result
    *d.add(0) = vreinterpretq_u8_u32(vcombine_u32(w6.0, w14.0));
    *d.add(1) = vreinterpretq_u8_u32(vcombine_u32(w6.1, w14.1));
    *d.add(2) = vreinterpretq_u8_u32(vcombine_u32(w7.0, w15.0));
    *d.add(3) = vreinterpretq_u8_u32(vcombine_u32(w7.1, w15.1));

    w4 = vzip_u16(vreinterpret_u16_u8(w0.1), vreinterpret_u16_u8(w1.1));
    w5 = vzip_u16(vreinterpret_u16_u8(w2.1), vreinterpret_u16_u8(w3.1));
    w12 = vzip_u16(vreinterpret_u16_u8(w8.1), vreinterpret_u16_u8(w9.1));
    w13 = vzip_u16(vreinterpret_u16_u8(w10.1), vreinterpret_u16_u8(w11.1));

    w6 = vzip_u32(vreinterpret_u32_u16(w4.0), vreinterpret_u32_u16(w5.0));
    w7 = vzip_u32(vreinterpret_u32_u16(w4.1), vreinterpret_u32_u16(w5.1));
    w14 = vzip_u32(vreinterpret_u32_u16(w12.0), vreinterpret_u32_u16(w13.0));
    w15 = vzip_u32(vreinterpret_u32_u16(w12.1), vreinterpret_u32_u16(w13.1));

    // Store second 4-line result
    *d.add(4) = vreinterpretq_u8_u32(vcombine_u32(w6.0, w14.0));
    *d.add(5) = vreinterpretq_u8_u32(vcombine_u32(w6.1, w14.1));
    *d.add(6) = vreinterpretq_u8_u32(vcombine_u32(w7.0, w15.0));
    *d.add(7) = vreinterpretq_u8_u32(vcombine_u32(w7.1, w15.1));
}

#[inline(always)]
pub unsafe fn transpose_arrays_u8_16x8(x: *const uint8x16_t, d: *mut uint8x8_t) {
    let x = |i: usize| *x.add(i);
    let w0 = vzipq_u8(x(0), x(1));
    let w1 = vzipq_u8(x(2), x(3));
    let w2 = vzipq_u8(x(4), x(5));
    let w3 = vzipq_u8(x(6), x(7));

    let w4 = vzipq_u16(vreinterpretq_u16_u8(w0.0), vreinterpretq_u16_u8(w1.0));
    let w5 = vzipq_u16(vreinterpretq_u16_u8(w2.0), vreinterpretq_u16_u8(w3.0));
    let w6 = vzipq_u16(vreinterpretq_u16_u8(w0.1), vreinterpretq_u16_u8(w1.1));
    let w7 = vzipq_u16(vreinterpretq_u16_u8(w2.1), vreinterpretq_u16_u8(w3.1));

    let w8 = vzipq_u32(vreinterpretq_u32_u16(w4.0), vreinterpretq_u32_u16(w5.0));
    let w9 = vzipq_u32(vreinterpretq_u32_u16(w6.0), vreinterpretq_u32_u16(w7.0));
    let w10 = vzipq_u32(vreinterpretq_u32_u16(w4.1), vreinterpretq_u32_u16(w5.1));
    let w11 = vzipq_u32(vreinterpretq_u32_u16(w6.1), vreinterpretq_u32_u16(w7.1));

    *d.add(0) = vreinterpret_u8_u32(vget_low_u32(w8.0));
    *d.add(1) = vreinterpret_u8_u32(vget_high_u32(w8.0));
    *d.add(2) = vreinterpret_u8_u32(vget_low_u32(w8.1));
    *d.add(3) = vreinterpret_u8_u32(vget_high_u32(w8.1));
    *d.add(4) = vreinterpret_u8_u32(vget_low_u32(w10.0));
    *d.add(5) = vreinterpret_u8_u32(vget_high_u32(w10.0));
    *d.add(6) = vreinterpret_u8_u32(vget_low_u32(w10.1));
    *d.add(7) = vreinterpret_u8_u32(vget_high_u32(w10.1));
    *d.add(8) = vreinterpret_u8_u32(vget_low_u32(w9.0));
    *d.add(9) = vreinterpret_u8_u32(vget_high_u32(w9.0));
    *d.add(10) = vreinterpret_u8_u32(vget_low_u32(w9.1));
    *d.add(11) = vreinterpret_u8_u32(vget_high_u32(w9.1));
    *d.add(12) = vreinterpret_u8_u32(vget_low_u32(w11.0));
    *d.add(13) = vreinterpret_u8_u32(vget_high_u32(w11.0));
    *d.add(14) = vreinterpret_u8_u32(vget_low_u32(w11.1));
    *d.add(15) = vreinterpret_u8_u32(vget_high_u32(w11.1));
}

#[inline(always)]
pub unsafe fn aom_vtrnq_u64_to_u16(a0: uint32x4_t, a1: uint32x4_t) -> uint16x8x2_t {
    uint16x8x2_t(
        vreinterpretq_u16_u64(vtrn1q_u64(vreinterpretq_u64_u32(a0), vreinterpretq_u64_u32(a1))),
        vreinterpretq_u16_u64(vtrn2q_u64(vreinterpretq_u64_u32(a0), vreinterpretq_u64_u32(a1))),
    )
}

#[inline(always)]
pub unsafe fn transpose_arrays_u8_16x16(x: *const uint8x16_t, d: *mut uint8x16_t) {
    let x = |i: usize| *x.add(i);
    let w0 = vzipq_u8(x(0), x(1));
    let w1 = vzipq_u8(x(2), x(3));
    let w2 = vzipq_u8(x(4), x(5));
    let w3 = vzipq_u8(x(6), x(7));

    let w4 = vzipq_u8(x(8), x(9));
    let w5 = vzipq_u8(x(10), x(11));
    let w6 = vzipq_u8(x(12), x(13));
    let w7 = vzipq_u8(x(14), x(15));

    let mut w8 = vzipq_u16(vreinterpretq_u16_u8(w0.0), vreinterpretq_u16_u8(w1.0));
    let mut w9 = vzipq_u16(vreinterpretq_u16_u8(w2.0), vreinterpretq_u16_u8(w3.0));
    let mut w10 = vzipq_u16(vreinterpretq_u16_u8(w4.0), vreinterpretq_u16_u8(w5.0));
    let mut w11 = vzipq_u16(vreinterpretq_u16_u8(w6.0), vreinterpretq_u16_u8(w7.0));

    let mut w12 = vzipq_u32(vreinterpretq_u32_u16(w8.0), vreinterpretq_u32_u16(w9.0));
    let mut w13 = vzipq_u32(vreinterpretq_u32_u16(w10.0), vreinterpretq_u32_u16(w11.0));
    let mut w14 = vzipq_u32(vreinterpretq_u32_u16(w8.1), vreinterpretq_u32_u16(w9.1));
    let mut w15 = vzipq_u32(vreinterpretq_u32_u16(w10.1), vreinterpretq_u32_u16(w11.1));

    let mut d01 = aom_vtrnq_u64_to_u16(w12.0, w13.0);
    *d.add(0) = vreinterpretq_u8_u16(d01.0);
    *d.add(1) = vreinterpretq_u8_u16(d01.1);
    let mut d23 = aom_vtrnq_u64_to_u16(w12.1, w13.1);
    *d.add(2) = vreinterpretq_u8_u16(d23.0);
    *d.add(3) = vreinterpretq_u8_u16(d23.1);
    let mut d45 = aom_vtrnq_u64_to_u16(w14.0, w15.0);
    *d.add(4) = vreinterpretq_u8_u16(d45.0);
    *d.add(5) = vreinterpretq_u8_u16(d45.1);
    let mut d67 = aom_vtrnq_u64_to_u16(w14.1, w15.1);
    *d.add(6) = vreinterpretq_u8_u16(d67.0);
    *d.add(7) = vreinterpretq_u8_u16(d67.1);

    // upper half
    w8 = vzipq_u16(vreinterpretq_u16_u8(w0.1), vreinterpretq_u16_u8(w1.1));
    w9 = vzipq_u16(vreinterpretq_u16_u8(w2.1), vreinterpretq_u16_u8(w3.1));
    w10 = vzipq_u16(vreinterpretq_u16_u8(w4.1), vreinterpretq_u16_u8(w5.1));
    w11 = vzipq_u16(vreinterpretq_u16_u8(w6.1), vreinterpretq_u16_u8(w7.1));

    w12 = vzipq_u32(vreinterpretq_u32_u16(w8.0), vreinterpretq_u32_u16(w9.0));
    w13 = vzipq_u32(vreinterpretq_u32_u16(w10.0), vreinterpretq_u32_u16(w11.0));
    w14 = vzipq_u32(vreinterpretq_u32_u16(w8.1), vreinterpretq_u32_u16(w9.1));
    w15 = vzipq_u32(vreinterpretq_u32_u16(w10.1), vreinterpretq_u32_u16(w11.1));

    d01 = aom_vtrnq_u64_to_u16(w12.0, w13.0);
    *d.add(8) = vreinterpretq_u8_u16(d01.0);
    *d.add(9) = vreinterpretq_u8_u16(d01.1);
    d23 = aom_vtrnq_u64_to_u16(w12.1, w13.1);
    *d.add(10) = vreinterpretq_u8_u16(d23.0);
    *d.add(11) = vreinterpretq_u8_u16(d23.1);
    d45 = aom_vtrnq_u64_to_u16(w14.0, w15.0);
    *d.add(12) = vreinterpretq_u8_u16(d45.0);
    *d.add(13) = vreinterpretq_u8_u16(d45.1);
    d67 = aom_vtrnq_u64_to_u16(w14.1, w15.1);
    *d.add(14) = vreinterpretq_u8_u16(d67.0);
    *d.add(15) = vreinterpretq_u8_u16(d67.1);
}

#[inline(always)]
pub unsafe fn transpose_arrays_u8_32x16(x: *const uint8x16x2_t, d: *mut uint8x16_t) {
    let mut x2: [uint8x16_t; 32] = [vdupq_n_u8(0); 32];
    for i in 0..16 {
        x2[i] = (*x.add(i)).0;
        x2[i + 16] = (*x.add(i)).1;
    }
    transpose_arrays_u8_16x16(x2.as_ptr(), d);
    transpose_arrays_u8_16x16(x2.as_ptr().add(16), d.add(16));
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_u8_8x4(a0: *mut uint8x8_t, a1: *mut uint8x8_t,
                                             a2: *mut uint8x8_t, a3: *mut uint8x8_t) {
    // Swap 8 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07
    // a1: 10 11 12 13 14 15 16 17
    // a2: 20 21 22 23 24 25 26 27
    // a3: 30 31 32 33 34 35 36 37
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37

    let b0 = vtrn_u8(*a0, *a1);
    let b1 = vtrn_u8(*a2, *a3);

    // Swap 16 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 02 12 22 32 06 16 26 36
    // c1.val[0]: 01 11 21 31 05 15 25 35
    // c1.val[1]: 03 13 23 33 07 17 27 37

    let c0 = vtrn_u16(vreinterpret_u16_u8(b0.0), vreinterpret_u16_u8(b1.0));
    let c1 = vtrn_u16(vreinterpret_u16_u8(b0.1), vreinterpret_u16_u8(b1.1));

    *a0 = vreinterpret_u8_u16(c0.0);
    *a1 = vreinterpret_u8_u16(c1.0);
    *a2 = vreinterpret_u8_u16(c0.1);
    *a3 = vreinterpret_u8_u16(c1.1);
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_u8_16x4(a0: *mut uint8x16_t, a1: *mut uint8x16_t,
                                              a2: *mut uint8x16_t, a3: *mut uint8x16_t) {
    // Swap 8 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07 08 09 010 011 012 013 014 015
    // a1: 10 11 12 13 14 15 16 17 18 19 110 111 112 113 114 115
    // a2: 20 21 22 23 24 25 26 27 28 29 210 211 212 213 214 215
    // a3: 30 31 32 33 34 35 36 37 38 39 310 311 312 313 314 315
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16 08 18 010 110 012 112 014 114
    // b0.val[1]: 01 11 03 13 05 15 07 17 09 19 011 111 013 113 015 115
    // b1.val[0]: 20 30 22 32 24 34 26 36 28 38 210 310 212 312 214 314
    // b1.val[1]: 21 31 23 33 25 35 27 37 29 39 211 311 213 313 215 315

    let b0 = vtrnq_u8(*a0, *a1);
    let b1 = vtrnq_u8(*a2, *a3);

    // Swap 16 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34 08  18  28  38  012 112 212 312
    // c0.val[1]: 02 12 22 32 06 16 26 36 09  19  29  39  013 113 213 313
    // c1.val[0]: 01 11 21 31 05 15 25 35 010 110 210 310 014 114 214 314
    // c1.val[1]: 03 13 23 33 07 17 27 37 011 111 211 311 015 115 215 315

    let c0 = vtrnq_u16(vreinterpretq_u16_u8(b0.0), vreinterpretq_u16_u8(b1.0));
    let c1 = vtrnq_u16(vreinterpretq_u16_u8(b0.1), vreinterpretq_u16_u8(b1.1));

    *a0 = vreinterpretq_u8_u16(c0.0);
    *a1 = vreinterpretq_u8_u16(c1.0);
    *a2 = vreinterpretq_u8_u16(c0.1);
    *a3 = vreinterpretq_u8_u16(c1.1);
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_u8_4x4(a0: *mut uint8x8_t, a1: *mut uint8x8_t) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03  10 11 12 13
    // a1: 20 21 22 23  30 31 32 33
    // to:
    // b0.val[0]: 00 01 20 21  10 11 30 31
    // b0.val[1]: 02 03 22 23  12 13 32 33

    let b0 = vtrn_u16(vreinterpret_u16_u8(*a0), vreinterpret_u16_u8(*a1));

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 01 20 21  02 03 22 23
    // c0.val[1]: 10 11 30 31  12 13 32 33

    let c0 = vtrn_u32(vreinterpret_u32_u16(b0.0), vreinterpret_u32_u16(b0.1));

    // Swap 8 bit elements resulting in:
    // d0.val[0]: 00 10 20 30  02 12 22 32
    // d0.val[1]: 01 11 21 31  03 13 23 33

    let d0 = vtrn_u8(vreinterpret_u8_u32(c0.0), vreinterpret_u8_u32(c0.1));

    *a0 = d0.0;
    *a1 = d0.1;
}

#[inline(always)]
pub unsafe fn transpose_elems_u8_4x8(a0: uint8x8_t, a1: uint8x8_t, a2: uint8x8_t, a3: uint8x8_t,
                                     a4: uint8x8_t, a5: uint8x8_t, a6: uint8x8_t, a7: uint8x8_t,
                                     o0: *mut uint8x8_t, o1: *mut uint8x8_t,
                                     o2: *mut uint8x8_t, o3: *mut uint8x8_t) {
    // Swap 32 bit elements. Goes from:
    // a0: 00 01 02 03 XX XX XX XX
    // a1: 10 11 12 13 XX XX XX XX
    // a2: 20 21 22 23 XX XX XX XX
    // a3; 30 31 32 33 XX XX XX XX
    // a4: 40 41 42 43 XX XX XX XX
    // a5: 50 51 52 53 XX XX XX XX
    // a6: 60 61 62 63 XX XX XX XX
    // a7: 70 71 72 73 XX XX XX XX
    // to:
    // b0.val[0]: 00 01 02 03 40 41 42 43
    // b1.val[0]: 10 11 12 13 50 51 52 53
    // b2.val[0]: 20 21 22 23 60 61 62 63
    // b3.val[0]: 30 31 32 33 70 71 72 73

    let b0 = vtrn_u32(vreinterpret_u32_u8(a0), vreinterpret_u32_u8(a4));
    let b1 = vtrn_u32(vreinterpret_u32_u8(a1), vreinterpret_u32_u8(a5));
    let b2 = vtrn_u32(vreinterpret_u32_u8(a2), vreinterpret_u32_u8(a6));
    let b3 = vtrn_u32(vreinterpret_u32_u8(a3), vreinterpret_u32_u8(a7));

    // Swap 16 bit elements resulting in:
    // c0.val[0]: 00 01 20 21 40 41 60 61
    // c0.val[1]: 02 03 22 23 42 43 62 63
    // c1.val[0]: 10 11 30 31 50 51 70 71
    // c1.val[1]: 12 13 32 33 52 53 72 73

    let c0 = vtrn_u16(vreinterpret_u16_u32(b0.0), vreinterpret_u16_u32(b2.0));
    let c1 = vtrn_u16(vreinterpret_u16_u32(b1.0), vreinterpret_u16_u32(b3.0));

    // Swap 8 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70
    // d0.val[1]: 01 11 21 31 41 51 61 71
    // d1.val[0]: 02 12 22 32 42 52 62 72
    // d1.val[1]: 03 13 23 33 43 53 63 73

    let d0 = vtrn_u8(vreinterpret_u8_u16(c0.0), vreinterpret_u8_u16(c1.0));
    let d1 = vtrn_u8(vreinterpret_u8_u16(c0.1), vreinterpret_u8_u16(c1.1));

    *o0 = d0.0;
    *o1 = d0.1;
    *o2 = d1.0;
    *o3 = d1.1;
}

#[inline(always)]
pub unsafe fn transpose_array_inplace_u16_4x4(a: *mut uint16x4_t) {
    // Input:
    // 00 01 02 03
    // 10 11 12 13
    // 20 21 22 23
    // 30 31 32 33

    // b:
    // 00 10 02 12
    // 01 11 03 13
    let b = vtrn_u16(*a.add(0), *a.add(1));
    // c:
    // 20 30 22 32
    // 21 31 23 33
    let c = vtrn_u16(*a.add(2), *a.add(3));
    // d:
    // 00 10 20 30
    // 02 12 22 32
    let d = vtrn_u32(vreinterpret_u32_u16(b.0), vreinterpret_u32_u16(c.0));
    // e:
    // 01 11 21 31
    // 03 13 23 33
    let e = vtrn_u32(vreinterpret_u32_u16(b.1), vreinterpret_u32_u16(c.1));

    // Output:
    // 00 10 20 30
    // 01 11 21 31
    // 02 12 22 32
    // 03 13 23 33
    *a.add(0) = vreinterpret_u16_u32(d.0);
    *a.add(1) = vreinterpret_u16_u32(e.0);
    *a.add(2) = vreinterpret_u16_u32(d.1);
    *a.add(3) = vreinterpret_u16_u32(e.1);
}

#[inline(always)]
pub unsafe fn transpose_array_inplace_u16_4x8(a: *mut uint16x8_t) {
    // 4x8 Input:
    // a[0]: 00 01 02 03 04 05 06 07
    // a[1]: 10 11 12 13 14 15 16 17
    // a[2]: 20 21 22 23 24 25 26 27
    // a[3]: 30 31 32 33 34 35 36 37

    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37
    let b0 = vtrnq_u16(*a.add(0), *a.add(1));
    let b1 = vtrnq_u16(*a.add(2), *a.add(3));

    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 02 12 22 32 06 16 26 36
    // c1.val[0]: 01 11 21 31 05 15 25 35
    // c1.val[1]: 03 13 23 33 07 17 27 37
    let c0 = vtrnq_u32(vreinterpretq_u32_u16(b0.0), vreinterpretq_u32_u16(b1.0));
    let c1 = vtrnq_u32(vreinterpretq_u32_u16(b0.1), vreinterpretq_u32_u16(b1.1));

    // 8x4 Output:
    // a[0]: 00 10 20 30 04 14 24 34
    // a[1]: 01 11 21 31 05 15 25 35
    // a[2]: 02 12 22 32 06 16 26 36
    // a[3]: 03 13 23 33 07 17 27 37
    *a.add(0) = vreinterpretq_u16_u32(c0.0);
    *a.add(1) = vreinterpretq_u16_u32(c1.0);
    *a.add(2) = vreinterpretq_u16_u32(c0.1);
    *a.add(3) = vreinterpretq_u16_u32(c1.1);
}

// Special transpose for loop filter.
// 4x8 Input:
// p_q:  p3 p2 p1 p0 q0 q1 q2 q3
// a[0]: 00 01 02 03 04 05 06 07
// a[1]: 10 11 12 13 14 15 16 17
// a[2]: 20 21 22 23 24 25 26 27
// a[3]: 30 31 32 33 34 35 36 37
// 8x4 Output:
// a[0]: 03 13 23 33 04 14 24 34  p0q0
// a[1]: 02 12 22 32 05 15 25 35  p1q1
// a[2]: 01 11 21 31 06 16 26 36  p2q2
// a[3]: 00 10 20 30 07 17 27 37  p3q3
// Direct reapplication of the function will reset the high halves, but
// reverse the low halves:
// p_q:  p0 p1 p2 p3 q0 q1 q2 q3
// a[0]: 33 32 31 30 04 05 06 07
// a[1]: 23 22 21 20 14 15 16 17
// a[2]: 13 12 11 10 24 25 26 27
// a[3]: 03 02 01 00 34 35 36 37
// Simply reordering the inputs (3, 2, 1, 0) will reset the low halves, but
// reverse the high halves.
// The standard transpose_u16_4x8q will produce the same reversals, but with the
// order of the low halves also restored relative to the high halves. This is
// preferable because it puts all values from the same source row back together,
// but some post-processing is inevitable.
#[inline(always)]
pub unsafe fn loop_filter_transpose_u16_4x8q(a: *mut uint16x8_t) {
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37
    let b0 = vtrnq_u16(*a.add(0), *a.add(1));
    let b1 = vtrnq_u16(*a.add(2), *a.add(3));

    // Reverse odd vectors to bring the appropriate items to the front of zips.
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // r0       : 03 13 01 11 07 17 05 15
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // r1       : 23 33 21 31 27 37 25 35
    let r0 = vrev64q_u32(vreinterpretq_u32_u16(b0.1));
    let r1 = vrev64q_u32(vreinterpretq_u32_u16(b1.1));

    // Zip to complete the halves.
    // c0.val[0]: 00 10 20 30 02 12 22 32  p3p1
    // c0.val[1]: 04 14 24 34 06 16 26 36  q0q2
    // c1.val[0]: 03 13 23 33 01 11 21 31  p0p2
    // c1.val[1]: 07 17 27 37 05 15 25 35  q3q1
    let c0 = vzipq_u32(vreinterpretq_u32_u16(b0.0), vreinterpretq_u32_u16(b1.0));
    let c1 = vzipq_u32(r0, r1);

    // d0.val[0]: 00 10 20 30 07 17 27 37  p3q3
    // d0.val[1]: 02 12 22 32 05 15 25 35  p1q1
    // d1.val[0]: 03 13 23 33 04 14 24 34  p0q0
    // d1.val[1]: 01 11 21 31 06 16 26 36  p2q2
    let d0 = aom_vtrnq_u64_to_u16(c0.0, c1.1);
    // The third row of c comes first here to swap p2 with q0.
    let d1 = aom_vtrnq_u64_to_u16(c1.0, c0.1);

    // 8x4 Output:
    // a[0]: 03 13 23 33 04 14 24 34  p0q0
    // a[1]: 02 12 22 32 05 15 25 35  p1q1
    // a[2]: 01 11 21 31 06 16 26 36  p2q2
    // a[3]: 00 10 20 30 07 17 27 37  p3q3
    *a.add(0) = d1.0; // p0q0
    *a.add(1) = d0.1; // p1q1
    *a.add(2) = d1.1; // p2q2
    *a.add(3) = d0.0; // p3q3
}

#[inline(always)]
pub unsafe fn transpose_elems_u16_4x8(
    a0: uint16x4_t, a1: uint16x4_t, a2: uint16x4_t, a3: uint16x4_t, a4: uint16x4_t,
    a5: uint16x4_t, a6: uint16x4_t, a7: uint16x4_t, o0: *mut uint16x8_t, o1: *mut uint16x8_t,
    o2: *mut uint16x8_t, o3: *mut uint16x8_t,
) {
    // Combine rows. Goes from:
    // a0: 00 01 02 03
    // a1: 10 11 12 13
    // a2: 20 21 22 23
    // a3: 30 31 32 33
    // a4: 40 41 42 43
    // a5: 50 51 52 53
    // a6: 60 61 62 63
    // a7: 70 71 72 73
    // to:
    // b0: 00 01 02 03 40 41 42 43
    // b1: 10 11 12 13 50 51 52 53
    // b2: 20 21 22 23 60 61 62 63
    // b3: 30 31 32 33 70 71 72 73

    let b0 = vcombine_u16(a0, a4);
    let b1 = vcombine_u16(a1, a5);
    let b2 = vcombine_u16(a2, a6);
    let b3 = vcombine_u16(a3, a7);

    // Swap 16 bit elements resulting in:
    // c0.val[0]: 00 10 02 12 40 50 42 52
    // c0.val[1]: 01 11 03 13 41 51 43 53
    // c1.val[0]: 20 30 22 32 60 70 62 72
    // c1.val[1]: 21 31 23 33 61 71 63 73

    let c0 = vtrnq_u16(b0, b1);
    let c1 = vtrnq_u16(b2, b3);

    // Swap 32 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70
    // d0.val[1]: 02 12 22 32 42 52 62 72
    // d1.val[0]: 01 11 21 31 41 51 61 71
    // d1.val[1]: 03 13 23 33 43 53 63 73

    let d0 = vtrnq_u32(vreinterpretq_u32_u16(c0.0), vreinterpretq_u32_u16(c1.0));
    let d1 = vtrnq_u32(vreinterpretq_u32_u16(c0.1), vreinterpretq_u32_u16(c1.1));

    *o0 = vreinterpretq_u16_u32(d0.0);
    *o1 = vreinterpretq_u16_u32(d1.0);
    *o2 = vreinterpretq_u16_u32(d0.1);
    *o3 = vreinterpretq_u16_u32(d1.1);
}

#[inline(always)]
pub unsafe fn transpose_elems_s16_4x8(
    a0: int16x4_t, a1: int16x4_t, a2: int16x4_t, a3: int16x4_t, a4: int16x4_t, a5: int16x4_t,
    a6: int16x4_t, a7: int16x4_t, o0: *mut int16x8_t, o1: *mut int16x8_t, o2: *mut int16x8_t,
    o3: *mut int16x8_t,
) {
    // Combine rows. Goes from:
    // a0: 00 01 02 03
    // a1: 10 11 12 13
    // a2: 20 21 22 23
    // a3: 30 31 32 33
    // a4: 40 41 42 43
    // a5: 50 51 52 53
    // a6: 60 61 62 63
    // a7: 70 71 72 73
    // to:
    // b0: 00 01 02 03 40 41 42 43
    // b1: 10 11 12 13 50 51 52 53
    // b2: 20 21 22 23 60 61 62 63
    // b3: 30 31 32 33 70 71 72 73

    let b0 = vcombine_s16(a0, a4);
    let b1 = vcombine_s16(a1, a5);
    let b2 = vcombine_s16(a2, a6);
    let b3 = vcombine_s16(a3, a7);

    // Swap 16 bit elements resulting in:
    // c0.val[0]: 00 10 02 12 40 50 42 52
    // c0.val[1]: 01 11 03 13 41 51 43 53
    // c1.val[0]: 20 30 22 32 60 70 62 72
    // c1.val[1]: 21 31 23 33 61 71 63 73

    let c0 = vtrnq_s16(b0, b1);
    let c1 = vtrnq_s16(b2, b3);

    // Swap 32 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70
    // d0.val[1]: 02 12 22 32 42 52 62 72
    // d1.val[0]: 01 11 21 31 41 51 61 71
    // d1.val[1]: 03 13 23 33 43 53 63 73

    let d0 = vtrnq_s32(vreinterpretq_s32_s16(c0.0), vreinterpretq_s32_s16(c1.0));
    let d1 = vtrnq_s32(vreinterpretq_s32_s16(c0.1), vreinterpretq_s32_s16(c1.1));

    *o0 = vreinterpretq_s16_s32(d0.0);
    *o1 = vreinterpretq_s16_s32(d1.0);
    *o2 = vreinterpretq_s16_s32(d0.1);
    *o3 = vreinterpretq_s16_s32(d1.1);
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_u16_8x8(
    a0: *mut uint16x8_t, a1: *mut uint16x8_t, a2: *mut uint16x8_t, a3: *mut uint16x8_t,
    a4: *mut uint16x8_t, a5: *mut uint16x8_t, a6: *mut uint16x8_t, a7: *mut uint16x8_t,
) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07
    // a1: 10 11 12 13 14 15 16 17
    // a2: 20 21 22 23 24 25 26 27
    // a3: 30 31 32 33 34 35 36 37
    // a4: 40 41 42 43 44 45 46 47
    // a5: 50 51 52 53 54 55 56 57
    // a6: 60 61 62 63 64 65 66 67
    // a7: 70 71 72 73 74 75 76 77
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37
    // b2.val[0]: 40 50 42 52 44 54 46 56
    // b2.val[1]: 41 51 43 53 45 55 47 57
    // b3.val[0]: 60 70 62 72 64 74 66 76
    // b3.val[1]: 61 71 63 73 65 75 67 77

    let b0 = vtrnq_u16(*a0, *a1);
    let b1 = vtrnq_u16(*a2, *a3);
    let b2 = vtrnq_u16(*a4, *a5);
    let b3 = vtrnq_u16(*a6, *a7);

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 02 12 22 32 06 16 26 36
    // c1.val[0]: 01 11 21 31 05 15 25 35
    // c1.val[1]: 03 13 23 33 07 17 27 37
    // c2.val[0]: 40 50 60 70 44 54 64 74
    // c2.val[1]: 42 52 62 72 46 56 66 76
    // c3.val[0]: 41 51 61 71 45 55 65 75
    // c3.val[1]: 43 53 63 73 47 57 67 77

    let c0 = vtrnq_u32(vreinterpretq_u32_u16(b0.0), vreinterpretq_u32_u16(b1.0));
    let c1 = vtrnq_u32(vreinterpretq_u32_u16(b0.1), vreinterpretq_u32_u16(b1.1));
    let c2 = vtrnq_u32(vreinterpretq_u32_u16(b2.0), vreinterpretq_u32_u16(b3.0));
    let c3 = vtrnq_u32(vreinterpretq_u32_u16(b2.1), vreinterpretq_u32_u16(b3.1));

    // Swap 64 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70
    // d0.val[1]: 04 14 24 34 44 54 64 74
    // d1.val[0]: 01 11 21 31 41 51 61 71
    // d1.val[1]: 05 15 25 35 45 55 65 75
    // d2.val[0]: 02 12 22 32 42 52 62 72
    // d2.val[1]: 06 16 26 36 46 56 66 76
    // d3.val[0]: 03 13 23 33 43 53 63 73
    // d3.val[1]: 07 17 27 37 47 57 67 77

    let d0 = aom_vtrnq_u64_to_u16(c0.0, c2.0);
    let d1 = aom_vtrnq_u64_to_u16(c1.0, c3.0);
    let d2 = aom_vtrnq_u64_to_u16(c0.1, c2.1);
    let d3 = aom_vtrnq_u64_to_u16(c1.1, c3.1);

    *a0 = d0.0;
    *a1 = d1.0;
    *a2 = d2.0;
    *a3 = d3.0;
    *a4 = d0.1;
    *a5 = d1.1;
    *a6 = d2.1;
    *a7 = d3.1;
}

#[inline(always)]
pub unsafe fn aom_vtrnq_s64_to_s16(a0: int32x4_t, a1: int32x4_t) -> int16x8x2_t {
    int16x8x2_t(
        vreinterpretq_s16_s64(vtrn1q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1))),
        vreinterpretq_s16_s64(vtrn2q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1))),
    )
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_s16_8x8(
    a0: *mut int16x8_t, a1: *mut int16x8_t, a2: *mut int16x8_t, a3: *mut int16x8_t,
    a4: *mut int16x8_t, a5: *mut int16x8_t, a6: *mut int16x8_t, a7: *mut int16x8_t,
) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07
    // a1: 10 11 12 13 14 15 16 17
    // a2: 20 21 22 23 24 25 26 27
    // a3: 30 31 32 33 34 35 36 37
    // a4: 40 41 42 43 44 45 46 47
    // a5: 50 51 52 53 54 55 56 57
    // a6: 60 61 62 63 64 65 66 67
    // a7: 70 71 72 73 74 75 76 77
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37
    // b2.val[0]: 40 50 42 52 44 54 46 56
    // b2.val[1]: 41 51 43 53 45 55 47 57
    // b3.val[0]: 60 70 62 72 64 74 66 76
    // b3.val[1]: 61 71 63 73 65 75 67 77

    let b0 = vtrnq_s16(*a0, *a1);
    let b1 = vtrnq_s16(*a2, *a3);
    let b2 = vtrnq_s16(*a4, *a5);
    let b3 = vtrnq_s16(*a6, *a7);

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 02 12 22 32 06 16 26 36
    // c1.val[0]: 01 11 21 31 05 15 25 35
    // c1.val[1]: 03 13 23 33 07 17 27 37
    // c2.val[0]: 40 50 60 70 44 54 64 74
    // c2.val[1]: 42 52 62 72 46 56 66 76
    // c3.val[0]: 41 51 61 71 45 55 65 75
    // c3.val[1]: 43 53 63 73 47 57 67 77

    let c0 = vtrnq_s32(vreinterpretq_s32_s16(b0.0), vreinterpretq_s32_s16(b1.0));
    let c1 = vtrnq_s32(vreinterpretq_s32_s16(b0.1), vreinterpretq_s32_s16(b1.1));
    let c2 = vtrnq_s32(vreinterpretq_s32_s16(b2.0), vreinterpretq_s32_s16(b3.0));
    let c3 = vtrnq_s32(vreinterpretq_s32_s16(b2.1), vreinterpretq_s32_s16(b3.1));

    // Swap 64 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70
    // d0.val[1]: 04 14 24 34 44 54 64 74
    // d1.val[0]: 01 11 21 31 41 51 61 71
    // d1.val[1]: 05 15 25 35 45 55 65 75
    // d2.val[0]: 02 12 22 32 42 52 62 72
    // d2.val[1]: 06 16 26 36 46 56 66 76
    // d3.val[0]: 03 13 23 33 43 53 63 73
    // d3.val[1]: 07 17 27 37 47 57 67 77

    let d0 = aom_vtrnq_s64_to_s16(c0.0, c2.0);
    let d1 = aom_vtrnq_s64_to_s16(c1.0, c3.0);
    let d2 = aom_vtrnq_s64_to_s16(c0.1, c2.1);
    let d3 = aom_vtrnq_s64_to_s16(c1.1, c3.1);

    *a0 = d0.0;
    *a1 = d1.0;
    *a2 = d2.0;
    *a3 = d3.0;
    *a4 = d0.1;
    *a5 = d1.1;
    *a6 = d2.1;
    *a7 = d3.1;
}

#[inline(always)]
pub unsafe fn transpose_arrays_s16_8x8(a: *const int16x8_t, out: *mut int16x8_t) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07
    // a1: 10 11 12 13 14 15 16 17
    // a2: 20 21 22 23 24 25 26 27
    // a3: 30 31 32 33 34 35 36 37
    // a4: 40 41 42 43 44 45 46 47
    // a5: 50 51 52 53 54 55 56 57
    // a6: 60 61 62 63 64 65 66 67
    // a7: 70 71 72 73 74 75 76 77
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37
    // b2.val[0]: 40 50 42 52 44 54 46 56
    // b2.val[1]: 41 51 43 53 45 55 47 57
    // b3.val[0]: 60 70 62 72 64 74 66 76
    // b3.val[1]: 61 71 63 73 65 75 67 77

    let b0 = vtrnq_s16(*a.add(0), *a.add(1));
    let b1 = vtrnq_s16(*a.add(2), *a.add(3));
    let b2 = vtrnq_s16(*a.add(4), *a.add(5));
    let b3 = vtrnq_s16(*a.add(6), *a.add(7));

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 02 12 22 32 06 16 26 36
    // c1.val[0]: 01 11 21 31 05 15 25 35
    // c1.val[1]: 03 13 23 33 07 17 27 37
    // c2.val[0]: 40 50 60 70 44 54 64 74
    // c2.val[1]: 42 52 62 72 46 56 66 76
    // c3.val[0]: 41 51 61 71 45 55 65 75
    // c3.val[1]: 43 53 63 73 47 57 67 77

    let c0 = vtrnq_s32(vreinterpretq_s32_s16(b0.0), vreinterpretq_s32_s16(b1.0));
    let c1 = vtrnq_s32(vreinterpretq_s32_s16(b0.1), vreinterpretq_s32_s16(b1.1));
    let c2 = vtrnq_s32(vreinterpretq_s32_s16(b2.0), vreinterpretq_s32_s16(b3.0));
    let c3 = vtrnq_s32(vreinterpretq_s32_s16(b2.1), vreinterpretq_s32_s16(b3.1));

    // Swap 64 bit elements resulting in:
    // d0.val[0]: 00 10 20 30 40 50 60 70
    // d0.val[1]: 04 14 24 34 44 54 64 74
    // d1.val[0]: 01 11 21 31 41 51 61 71
    // d1.val[1]: 05 15 25 35 45 55 65 75
    // d2.val[0]: 02 12 22 32 42 52 62 72
    // d2.val[1]: 06 16 26 36 46 56 66 76
    // d3.val[0]: 03 13 23 33 43 53 63 73
    // d3.val[1]: 07 17 27 37 47 57 67 77

    let d0 = aom_vtrnq_s64_to_s16(c0.0, c2.0);
    let d1 = aom_vtrnq_s64_to_s16(c1.0, c3.0);
    let d2 = aom_vtrnq_s64_to_s16(c0.1, c2.1);
    let d3 = aom_vtrnq_s64_to_s16(c1.1, c3.1);

    *out.add(0) = d0.0;
    *out.add(1) = d1.0;
    *out.add(2) = d2.0;
    *out.add(3) = d3.0;
    *out.add(4) = d0.1;
    *out.add(5) = d1.1;
    *out.add(6) = d2.1;
    *out.add(7) = d3.1;
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_s16_8x4(a0: *mut int16x8_t, a1: *mut int16x8_t,
                                              a2: *mut int16x8_t, a3: *mut int16x8_t) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03 04 05 06 07
    // a1: 10 11 12 13 14 15 16 17
    // a2: 20 21 22 23 24 25 26 27
    // a3: 30 31 32 33 34 35 36 37
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37

    let b0 = vtrnq_s16(*a0, *a1);
    let b1 = vtrnq_s16(*a2, *a3);

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 01 11 21 31 05 15 25 35
    // c1.val[0]: 02 12 22 32 06 16 26 36
    // c1.val[1]: 03 13 23 33 07 17 27 37

    let c0 = vtrnq_s32(vreinterpretq_s32_s16(b0.0), vreinterpretq_s32_s16(b1.0));
    let c1 = vtrnq_s32(vreinterpretq_s32_s16(b0.1), vreinterpretq_s32_s16(b1.1));

    *a0 = vreinterpretq_s16_s32(c0.0);
    *a1 = vreinterpretq_s16_s32(c1.0);
    *a2 = vreinterpretq_s16_s32(c0.1);
    *a3 = vreinterpretq_s16_s32(c1.1);
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_u16_4x4(a0: *mut uint16x4_t, a1: *mut uint16x4_t,
                                              a2: *mut uint16x4_t, a3: *mut uint16x4_t) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03
    // a1: 10 11 12 13
    // a2: 20 21 22 23
    // a3: 30 31 32 33
    // to:
    // b0.val[0]: 00 10 02 12
    // b0.val[1]: 01 11 03 13
    // b1.val[0]: 20 30 22 32
    // b1.val[1]: 21 31 23 33

    let b0 = vtrn_u16(*a0, *a1);
    let b1 = vtrn_u16(*a2, *a3);

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30
    // c0.val[1]: 02 12 22 32
    // c1.val[0]: 01 11 21 31
    // c1.val[1]: 03 13 23 33

    let c0 = vtrn_u32(vreinterpret_u32_u16(b0.0), vreinterpret_u32_u16(b1.0));
    let c1 = vtrn_u32(vreinterpret_u32_u16(b0.1), vreinterpret_u32_u16(b1.1));

    *a0 = vreinterpret_u16_u32(c0.0);
    *a1 = vreinterpret_u16_u32(c1.0);
    *a2 = vreinterpret_u16_u32(c0.1);
    *a3 = vreinterpret_u16_u32(c1.1);
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_s16_4x4(a0: *mut int16x4_t, a1: *mut int16x4_t,
                                              a2: *mut int16x4_t, a3: *mut int16x4_t) {
    // Swap 16 bit elements. Goes from:
    // a0: 00 01 02 03
    // a1: 10 11 12 13
    // a2: 20 21 22 23
    // a3: 30 31 32 33
    // to:
    // b0.val[0]: 00 10 02 12
    // b0.val[1]: 01 11 03 13
    // b1.val[0]: 20 30 22 32
    // b1.val[1]: 21 31 23 33

    let b0 = vtrn_s16(*a0, *a1);
    let b1 = vtrn_s16(*a2, *a3);

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30
    // c0.val[1]: 02 12 22 32
    // c1.val[0]: 01 11 21 31
    // c1.val[1]: 03 13 23 33

    let c0 = vtrn_s32(vreinterpret_s32_s16(b0.0), vreinterpret_s32_s16(b1.0));
    let c1 = vtrn_s32(vreinterpret_s32_s16(b0.1), vreinterpret_s32_s16(b1.1));

    *a0 = vreinterpret_s16_s32(c0.0);
    *a1 = vreinterpret_s16_s32(c1.0);
    *a2 = vreinterpret_s16_s32(c0.1);
    *a3 = vreinterpret_s16_s32(c1.1);
}

#[inline(always)]
pub unsafe fn aom_vtrnq_s64_to_s32(a0: int32x4_t, a1: int32x4_t) -> int32x4x2_t {
    int32x4x2_t(
        vreinterpretq_s32_s64(vtrn1q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1))),
        vreinterpretq_s32_s64(vtrn2q_s64(vreinterpretq_s64_s32(a0), vreinterpretq_s64_s32(a1))),
    )
}

#[inline(always)]
pub unsafe fn transpose_elems_s32_4x4(a0: int32x4_t, a1: int32x4_t, a2: int32x4_t,
                                      a3: int32x4_t, o0: *mut int32x4_t, o1: *mut int32x4_t,
                                      o2: *mut int32x4_t, o3: *mut int32x4_t) {
    // Swap 32 bit elements. Goes from:
    // a0: 00 01 02 03
    // a1: 10 11 12 13
    // a2: 20 21 22 23
    // a3: 30 31 32 33
    // to:
    // b0.val[0]: 00 10 02 12
    // b0.val[1]: 01 11 03 13
    // b1.val[0]: 20 30 22 32
    // b1.val[1]: 21 31 23 33

    let b0 = vtrnq_s32(a0, a1);
    let b1 = vtrnq_s32(a2, a3);

    // Swap 64 bit elements resulting in:
    // c0.val[0]: 00 10 20 30
    // c0.val[1]: 02 12 22 32
    // c1.val[0]: 01 11 21 31
    // c1.val[1]: 03 13 23 33

    let c0 = aom_vtrnq_s64_to_s32(b0.0, b1.0);
    let c1 = aom_vtrnq_s64_to_s32(b0.1, b1.1);

    *o0 = c0.0;
    *o1 = c1.0;
    *o2 = c0.1;
    *o3 = c1.1;
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_s32_4x4(a0: *mut int32x4_t, a1: *mut int32x4_t,
                                              a2: *mut int32x4_t, a3: *mut int32x4_t) {
    transpose_elems_s32_4x4(*a0, *a1, *a2, *a3, a0, a1, a2, a3);
}

#[inline(always)]
pub unsafe fn transpose_arrays_s32_4x4(in_: *const int32x4_t, out: *mut int32x4_t) {
    transpose_elems_s32_4x4(*in_.add(0), *in_.add(1), *in_.add(2), *in_.add(3), out.add(0),
                            out.add(1), out.add(2), out.add(3));
}

#[inline(always)]
pub unsafe fn transpose_arrays_s32_4nx4n(in_: *const int32x4_t, out: *mut int32x4_t, width: i32,
                                         height: i32) {
    let h = height >> 2;
    let w = width >> 2;
    for j in 0..w {
        for i in 0..h {
            transpose_arrays_s32_4x4(in_.offset((j * height + i * 4) as isize),
                                     out.offset((i * width + j * 4) as isize));
        }
    }
}

macro_rules! TRANSPOSE_ARRAYS_S32_WXH_NEON {
    ($name:ident, $w:expr, $h:expr) => {
        #[inline(always)]
        pub unsafe fn $name(in_: *const int32x4_t, out: *mut int32x4_t) {
            transpose_arrays_s32_4nx4n(in_, out, $w, $h);
        }
    };
}

TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_4x8, 4, 8);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_4x16, 4, 16);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_8x4, 8, 4);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_8x8, 8, 8);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_8x16, 8, 16);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_8x32, 8, 32);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_16x8, 16, 8);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_16x16, 16, 16);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_16x32, 16, 32);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_16x64, 16, 64);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_32x8, 32, 8);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_32x16, 32, 16);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_32x32, 32, 32);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_32x64, 32, 64);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_64x16, 64, 16);
TRANSPOSE_ARRAYS_S32_WXH_NEON!(transpose_arrays_s32_64x32, 64, 32);

#[inline(always)]
pub unsafe fn aom_vtrn1q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    vtrn1q_s64(a, b)
}

#[inline(always)]
pub unsafe fn aom_vtrn2q_s64(a: int64x2_t, b: int64x2_t) -> int64x2_t {
    vtrn2q_s64(a, b)
}

#[inline(always)]
pub unsafe fn transpose_elems_s32_4x8(mut a0: int32x4_t, mut a1: int32x4_t, mut a2: int32x4_t,
                                      mut a3: int32x4_t, mut a4: int32x4_t, mut a5: int32x4_t,
                                      mut a6: int32x4_t, mut a7: int32x4_t,
                                      o0: *mut int32x4x2_t, o1: *mut int32x4x2_t,
                                      o2: *mut int32x4x2_t, o3: *mut int32x4x2_t) {
    // Perform a 4 x 8 matrix transpose by building on top of the existing 4 x 4
    // matrix transpose implementation:
    // [ A ]^T => [ A^T B^T ]
    // [ B ]

    transpose_elems_inplace_s32_4x4(&mut a0, &mut a1, &mut a2, &mut a3); // A^T
    transpose_elems_inplace_s32_4x4(&mut a4, &mut a5, &mut a6, &mut a7); // B^T

    (*o0).0 = a0;
    (*o1).0 = a1;
    (*o2).0 = a2;
    (*o3).0 = a3;

    (*o0).1 = a4;
    (*o1).1 = a5;
    (*o2).1 = a6;
    (*o3).1 = a7;
}

#[inline(always)]
pub unsafe fn transpose_elems_inplace_s32_8x8(
    a0: *mut int32x4x2_t, a1: *mut int32x4x2_t, a2: *mut int32x4x2_t, a3: *mut int32x4x2_t,
    a4: *mut int32x4x2_t, a5: *mut int32x4x2_t, a6: *mut int32x4x2_t, a7: *mut int32x4x2_t,
) {
    // Perform an 8 x 8 matrix transpose by building on top of the existing 4 x 4
    // matrix transpose implementation:
    // [ A B ]^T => [ A^T C^T ]
    // [ C D ]      [ B^T D^T ]

    let mut q0_v1 = (*a0).0;
    let mut q0_v2 = (*a1).0;
    let mut q0_v3 = (*a2).0;
    let mut q0_v4 = (*a3).0;

    let mut q1_v1 = (*a0).1;
    let mut q1_v2 = (*a1).1;
    let mut q1_v3 = (*a2).1;
    let mut q1_v4 = (*a3).1;

    let mut q2_v1 = (*a4).0;
    let mut q2_v2 = (*a5).0;
    let mut q2_v3 = (*a6).0;
    let mut q2_v4 = (*a7).0;

    let mut q3_v1 = (*a4).1;
    let mut q3_v2 = (*a5).1;
    let mut q3_v3 = (*a6).1;
    let mut q3_v4 = (*a7).1;

    transpose_elems_inplace_s32_4x4(&mut q0_v1, &mut q0_v2, &mut q0_v3, &mut q0_v4); // A^T
    transpose_elems_inplace_s32_4x4(&mut q1_v1, &mut q1_v2, &mut q1_v3, &mut q1_v4); // B^T
    transpose_elems_inplace_s32_4x4(&mut q2_v1, &mut q2_v2, &mut q2_v3, &mut q2_v4); // C^T
    transpose_elems_inplace_s32_4x4(&mut q3_v1, &mut q3_v2, &mut q3_v3, &mut q3_v4); // D^T

    (*a0).0 = q0_v1;
    (*a1).0 = q0_v2;
    (*a2).0 = q0_v3;
    (*a3).0 = q0_v4;

    (*a0).1 = q2_v1;
    (*a1).1 = q2_v2;
    (*a2).1 = q2_v3;
    (*a3).1 = q2_v4;

    (*a4).0 = q1_v1;
    (*a5).0 = q1_v2;
    (*a6).0 = q1_v3;
    (*a7).0 = q1_v4;

    (*a4).1 = q3_v1;
    (*a5).1 = q3_v2;
    (*a6).1 = q3_v3;
    (*a7).1 = q3_v4;
}

#[inline(always)]
pub unsafe fn transpose_arrays_s16_4x4(in_: *const int16x4_t, out: *mut int16x4_t) {
    let mut a0 = *in_.add(0);
    let mut a1 = *in_.add(1);
    let mut a2 = *in_.add(2);
    let mut a3 = *in_.add(3);

    transpose_elems_inplace_s16_4x4(&mut a0, &mut a1, &mut a2, &mut a3);

    *out.add(0) = a0;
    *out.add(1) = a1;
    *out.add(2) = a2;
    *out.add(3) = a3;
}

#[inline(always)]
pub unsafe fn transpose_arrays_s16_4x8(in_: *const int16x4_t, out: *mut int16x8_t) {
    let a0 = vzip1q_s16(vcombine_s16(*in_.add(0), vdup_n_s16(0)),
                        vcombine_s16(*in_.add(1), vdup_n_s16(0)));
    let a1 = vzip1q_s16(vcombine_s16(*in_.add(2), vdup_n_s16(0)),
                        vcombine_s16(*in_.add(3), vdup_n_s16(0)));
    let a2 = vzip1q_s16(vcombine_s16(*in_.add(4), vdup_n_s16(0)),
                        vcombine_s16(*in_.add(5), vdup_n_s16(0)));
    let a3 = vzip1q_s16(vcombine_s16(*in_.add(6), vdup_n_s16(0)),
                        vcombine_s16(*in_.add(7), vdup_n_s16(0)));

    let b02 = vzipq_s32(vreinterpretq_s32_s16(a0), vreinterpretq_s32_s16(a1));
    let b13 = vzipq_s32(vreinterpretq_s32_s16(a2), vreinterpretq_s32_s16(a3));

    *out.add(0) = vreinterpretq_s16_s64(vzip1q_s64(vreinterpretq_s64_s32(b02.0),
                                                   vreinterpretq_s64_s32(b13.0)));
    *out.add(1) = vreinterpretq_s16_s64(vzip2q_s64(vreinterpretq_s64_s32(b02.0),
                                                   vreinterpretq_s64_s32(b13.0)));
    *out.add(2) = vreinterpretq_s16_s64(vzip1q_s64(vreinterpretq_s64_s32(b02.1),
                                                   vreinterpretq_s64_s32(b13.1)));
    *out.add(3) = vreinterpretq_s16_s64(vzip2q_s64(vreinterpretq_s64_s32(b02.1),
                                                   vreinterpretq_s64_s32(b13.1)));
}

#[inline(always)]
pub unsafe fn transpose_arrays_s16_8x4(in_: *const int16x8_t, out: *mut int16x4_t) {
    // Swap 16 bit elements. Goes from:
    // in[0]: 00 01 02 03 04 05 06 07
    // in[1]: 10 11 12 13 14 15 16 17
    // in[2]: 20 21 22 23 24 25 26 27
    // in[3]: 30 31 32 33 34 35 36 37
    // to:
    // b0.val[0]: 00 10 02 12 04 14 06 16
    // b0.val[1]: 01 11 03 13 05 15 07 17
    // b1.val[0]: 20 30 22 32 24 34 26 36
    // b1.val[1]: 21 31 23 33 25 35 27 37

    let b0 = vtrnq_s16(*in_.add(0), *in_.add(1));
    let b1 = vtrnq_s16(*in_.add(2), *in_.add(3));

    // Swap 32 bit elements resulting in:
    // c0.val[0]: 00 10 20 30 04 14 24 34
    // c0.val[1]: 02 12 22 32 06 16 26 36
    // c1.val[0]: 01 11 21 31 05 15 25 35
    // c1.val[1]: 03 13 23 33 07 17 27 37

    let c0 = vtrnq_u32(vreinterpretq_u32_s16(b0.0), vreinterpretq_u32_s16(b1.0));
    let c1 = vtrnq_u32(vreinterpretq_u32_s16(b0.1), vreinterpretq_u32_s16(b1.1));

    // Unpack 64 bit elements resulting in:
    // out[0]: 00 10 20 30
    // out[1]: 01 11 21 31
    // out[2]: 02 12 22 32
    // out[3]: 03 13 23 33
    // out[4]: 04 14 24 34
    // out[5]: 05 15 25 35
    // out[6]: 06 16 26 36
    // out[7]: 07 17 27 37

    *out.add(0) = vget_low_s16(vreinterpretq_s16_u32(c0.0));
    *out.add(1) = vget_low_s16(vreinterpretq_s16_u32(c1.0));
    *out.add(2) = vget_low_s16(vreinterpretq_s16_u32(c0.1));
    *out.add(3) = vget_low_s16(vreinterpretq_s16_u32(c1.1));
    *out.add(4) = vget_high_s16(vreinterpretq_s16_u32(c0.0));
    *out.add(5) = vget_high_s16(vreinterpretq_s16_u32(c1.0));
    *out.add(6) = vget_high_s16(vreinterpretq_s16_u32(c0.1));
    *out.add(7) = vget_high_s16(vreinterpretq_s16_u32(c1.1));
}

#[inline(always)]
pub unsafe fn transpose_arrays_s64_4x4(in_: *const int64x2_t, out: *mut int64x2_t) {
    // Perform a 4x4 matrix transpose going from:
    // in[0] = 00 01
    // in[1] = 02 03
    // in[2] = 10 11
    // in[3] = 12 13
    // in[4] = 20 21
    // in[5] = 22 23
    // in[6] = 30 31
    // in[7] = 32 33
    //
    // to:
    // out[0] = 00 10
    // out[1] = 20 30
    // out[2] = 01 11
    // out[3] = 21 31
    // out[4] = 02 12
    // out[5] = 22 32
    // out[6] = 03 13
    // out[7] = 23 33

    let in_ = |i: usize| *in_.add(i);
    *out.add(0) = aom_vtrn1q_s64(in_(0), in_(2));
    *out.add(1) = aom_vtrn1q_s64(in_(4), in_(6));
    *out.add(2) = aom_vtrn2q_s64(in_(0), in_(2));
    *out.add(3) = aom_vtrn2q_s64(in_(4), in_(6));
    *out.add(4) = aom_vtrn1q_s64(in_(1), in_(3));
    *out.add(5) = aom_vtrn1q_s64(in_(5), in_(7));
    *out.add(6) = aom_vtrn2q_s64(in_(1), in_(3));
    *out.add(7) = aom_vtrn2q_s64(in_(5), in_(7));
}
