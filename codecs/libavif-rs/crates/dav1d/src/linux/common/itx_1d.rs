/*
 * Copyright © 2018-2019, VideoLAN and dav1d authors
 * Copyright © 2018-2019, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Mechanically translated from upstream/dav1d/src/itx_1d.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
pub type ptrdiff_t = isize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type Tx1dType = core::ffi::c_uint;
pub const N_TX_1D_TYPES: Tx1dType = 4;
pub const FLIPADST: Tx1dType = 3;
pub const IDENTITY: Tx1dType = 2;
pub const ADST: Tx1dType = 1;
pub const DCT: Tx1dType = 0;
pub type itx_1d_fn = Option<
    unsafe extern "C" fn(
        *mut int32_t,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[inline]
unsafe extern "C" fn iclip(
    v: core::ffi::c_int,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) -> core::ffi::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline(never)]
unsafe extern "C" fn inv_dct4_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    tx64: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            69 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 94],
                [core::ffi::c_char; 94],
            >(
                *b"void inv_dct4_1d_internal_c(int32_t *const, const ptrdiff_t, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let in0: core::ffi::c_int = *c.offset((0 as ptrdiff_t * stride) as isize);
    let in1: core::ffi::c_int = *c.offset((1 as ptrdiff_t * stride) as isize);
    let mut t0: core::ffi::c_int = 0;
    let mut t1: core::ffi::c_int = 0;
    let mut t2: core::ffi::c_int = 0;
    let mut t3: core::ffi::c_int = 0;
    if tx64 != 0 {
        t1 = in0 * 181 as core::ffi::c_int + 128 as core::ffi::c_int
            >> 8 as core::ffi::c_int;
        t0 = t1;
        t2 = in1 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t3 = in1 * 3784 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
    } else {
        let in2: core::ffi::c_int = *c.offset((2 as ptrdiff_t * stride) as isize);
        let in3: core::ffi::c_int = *c.offset((3 as ptrdiff_t * stride) as isize);
        t0 = (in0 + in2) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
            >> 8 as core::ffi::c_int;
        t1 = (in0 - in2) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
            >> 8 as core::ffi::c_int;
        t2 = (in1 * 1567 as core::ffi::c_int
            - in3 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in3;
        t3 = (in1 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in3 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in1;
    }
    *c.offset((0 as ptrdiff_t * stride) as isize) = iclip(t0 + t3, min, max) as int32_t;
    *c.offset((1 as ptrdiff_t * stride) as isize) = iclip(t1 + t2, min, max) as int32_t;
    *c.offset((2 as ptrdiff_t * stride) as isize) = iclip(t1 - t2, min, max) as int32_t;
    *c.offset((3 as ptrdiff_t * stride) as isize) = iclip(t0 - t3, min, max) as int32_t;
}
unsafe extern "C" fn inv_dct4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_dct4_1d_internal_c(c, stride, min, max, 0 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_dct8_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    tx64: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            102 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 94],
                [core::ffi::c_char; 94],
            >(
                *b"void inv_dct8_1d_internal_c(int32_t *const, const ptrdiff_t, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    inv_dct4_1d_internal_c(c, stride << 1 as core::ffi::c_int, min, max, tx64);
    let in1: core::ffi::c_int = *c.offset((1 as ptrdiff_t * stride) as isize);
    let in3: core::ffi::c_int = *c.offset((3 as ptrdiff_t * stride) as isize);
    let mut t4a: core::ffi::c_int = 0;
    let mut t5a: core::ffi::c_int = 0;
    let mut t6a: core::ffi::c_int = 0;
    let mut t7a: core::ffi::c_int = 0;
    if tx64 != 0 {
        t4a = in1 * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t5a = in3 * -(2276 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t6a = in3 * 3406 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t7a = in1 * 4017 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
    } else {
        let in5: core::ffi::c_int = *c.offset((5 as ptrdiff_t * stride) as isize);
        let in7: core::ffi::c_int = *c.offset((7 as ptrdiff_t * stride) as isize);
        t4a = (in1 * 799 as core::ffi::c_int
            - in7 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in7;
        t5a = in5 * 1703 as core::ffi::c_int - in3 * 1138 as core::ffi::c_int
            + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
        t6a = in5 * 1138 as core::ffi::c_int + in3 * 1703 as core::ffi::c_int
            + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
        t7a = (in1 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in7 * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in1;
    }
    let t4: core::ffi::c_int = iclip(t4a + t5a, min, max) as core::ffi::c_int;
    t5a = iclip(t4a - t5a, min, max);
    let t7: core::ffi::c_int = iclip(t7a + t6a, min, max) as core::ffi::c_int;
    t6a = iclip(t7a - t6a, min, max);
    let t5: core::ffi::c_int = (t6a - t5a) * 181 as core::ffi::c_int
        + 128 as core::ffi::c_int >> 8 as core::ffi::c_int;
    let t6: core::ffi::c_int = (t6a + t5a) * 181 as core::ffi::c_int
        + 128 as core::ffi::c_int >> 8 as core::ffi::c_int;
    let t0: core::ffi::c_int = *c.offset((0 as ptrdiff_t * stride) as isize);
    let t1: core::ffi::c_int = *c.offset((2 as ptrdiff_t * stride) as isize);
    let t2: core::ffi::c_int = *c.offset((4 as ptrdiff_t * stride) as isize);
    let t3: core::ffi::c_int = *c.offset((6 as ptrdiff_t * stride) as isize);
    *c.offset((0 as ptrdiff_t * stride) as isize) = iclip(t0 + t7, min, max) as int32_t;
    *c.offset((1 as ptrdiff_t * stride) as isize) = iclip(t1 + t6, min, max) as int32_t;
    *c.offset((2 as ptrdiff_t * stride) as isize) = iclip(t2 + t5, min, max) as int32_t;
    *c.offset((3 as ptrdiff_t * stride) as isize) = iclip(t3 + t4, min, max) as int32_t;
    *c.offset((4 as ptrdiff_t * stride) as isize) = iclip(t3 - t4, min, max) as int32_t;
    *c.offset((5 as ptrdiff_t * stride) as isize) = iclip(t2 - t5, min, max) as int32_t;
    *c.offset((6 as ptrdiff_t * stride) as isize) = iclip(t1 - t6, min, max) as int32_t;
    *c.offset((7 as ptrdiff_t * stride) as isize) = iclip(t0 - t7, min, max) as int32_t;
}
unsafe extern "C" fn inv_dct8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_dct8_1d_internal_c(c, stride, min, max, 0 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_dct16_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    mut tx64: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            155 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 89],
                [core::ffi::c_char; 89],
            >(
                *b"void inv_dct16_1d_internal_c(int32_t *const, const ptrdiff_t, const int, const int, int)\0",
            ))
                .as_ptr(),
        );
    };
    inv_dct8_1d_internal_c(c, stride << 1 as core::ffi::c_int, min, max, tx64);
    let in1: core::ffi::c_int = *c.offset((1 as ptrdiff_t * stride) as isize);
    let in3: core::ffi::c_int = *c.offset((3 as ptrdiff_t * stride) as isize);
    let in5: core::ffi::c_int = *c.offset((5 as ptrdiff_t * stride) as isize);
    let in7: core::ffi::c_int = *c.offset((7 as ptrdiff_t * stride) as isize);
    let mut t8a: core::ffi::c_int = 0;
    let mut t9a: core::ffi::c_int = 0;
    let mut t10a: core::ffi::c_int = 0;
    let mut t11a: core::ffi::c_int = 0;
    let mut t12a: core::ffi::c_int = 0;
    let mut t13a: core::ffi::c_int = 0;
    let mut t14a: core::ffi::c_int = 0;
    let mut t15a: core::ffi::c_int = 0;
    if tx64 != 0 {
        t8a = in1 * 401 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t9a = in7 * -(2598 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t10a = in5 * 1931 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t11a = in3 * -(1189 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t12a = in3 * 3920 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t13a = in5 * 3612 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t14a = in7 * 3166 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t15a = in1 * 4076 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
    } else {
        let in9: core::ffi::c_int = *c.offset((9 as ptrdiff_t * stride) as isize);
        let in11: core::ffi::c_int = *c.offset((11 as ptrdiff_t * stride) as isize);
        let in13: core::ffi::c_int = *c.offset((13 as ptrdiff_t * stride) as isize);
        let in15: core::ffi::c_int = *c.offset((15 as ptrdiff_t * stride) as isize);
        t8a = (in1 * 401 as core::ffi::c_int
            - in15 * (4076 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in15;
        t9a = in9 * 1583 as core::ffi::c_int - in7 * 1299 as core::ffi::c_int
            + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
        t10a = (in5 * 1931 as core::ffi::c_int
            - in11 * (3612 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in11;
        t11a = (in13 * (3920 as core::ffi::c_int - 4096 as core::ffi::c_int)
            - in3 * 1189 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in13;
        t12a = (in13 * 1189 as core::ffi::c_int
            + in3 * (3920 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in3;
        t13a = (in5 * (3612 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in11 * 1931 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in5;
        t14a = in9 * 1299 as core::ffi::c_int + in7 * 1583 as core::ffi::c_int
            + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
        t15a = (in1 * (4076 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in15 * 401 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in1;
    }
    let mut t8: core::ffi::c_int = iclip(t8a + t9a, min, max);
    let mut t9: core::ffi::c_int = iclip(t8a - t9a, min, max);
    let mut t10: core::ffi::c_int = iclip(t11a - t10a, min, max);
    let mut t11: core::ffi::c_int = iclip(t11a + t10a, min, max);
    let mut t12: core::ffi::c_int = iclip(t12a + t13a, min, max);
    let mut t13: core::ffi::c_int = iclip(t12a - t13a, min, max);
    let mut t14: core::ffi::c_int = iclip(t15a - t14a, min, max);
    let mut t15: core::ffi::c_int = iclip(t15a + t14a, min, max);
    t9a = (t14 * 1567 as core::ffi::c_int
        - t9 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t9;
    t14a = (t14 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t9 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t14;
    t10a = (-(t13 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t10 * 1567 as core::ffi::c_int) + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t13;
    t13a = (t13 * 1567 as core::ffi::c_int
        - t10 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t10;
    t8a = iclip(t8 + t11, min, max);
    t9 = iclip(t9a + t10a, min, max);
    t10 = iclip(t9a - t10a, min, max);
    t11a = iclip(t8 - t11, min, max);
    t12a = iclip(t15 - t12, min, max);
    t13 = iclip(t14a - t13a, min, max);
    t14 = iclip(t14a + t13a, min, max);
    t15a = iclip(t15 + t12, min, max);
    t10a = (t13 - t10) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t13a = (t13 + t10) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t11 = (t12a - t11a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t12 = (t12a + t11a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    let t0: core::ffi::c_int = *c.offset((0 as ptrdiff_t * stride) as isize);
    let t1: core::ffi::c_int = *c.offset((2 as ptrdiff_t * stride) as isize);
    let t2: core::ffi::c_int = *c.offset((4 as ptrdiff_t * stride) as isize);
    let t3: core::ffi::c_int = *c.offset((6 as ptrdiff_t * stride) as isize);
    let t4: core::ffi::c_int = *c.offset((8 as ptrdiff_t * stride) as isize);
    let t5: core::ffi::c_int = *c.offset((10 as ptrdiff_t * stride) as isize);
    let t6: core::ffi::c_int = *c.offset((12 as ptrdiff_t * stride) as isize);
    let t7: core::ffi::c_int = *c.offset((14 as ptrdiff_t * stride) as isize);
    *c.offset((0 as ptrdiff_t * stride) as isize) = iclip(t0 + t15a, min, max)
        as int32_t;
    *c.offset((1 as ptrdiff_t * stride) as isize) = iclip(t1 + t14, min, max) as int32_t;
    *c.offset((2 as ptrdiff_t * stride) as isize) = iclip(t2 + t13a, min, max)
        as int32_t;
    *c.offset((3 as ptrdiff_t * stride) as isize) = iclip(t3 + t12, min, max) as int32_t;
    *c.offset((4 as ptrdiff_t * stride) as isize) = iclip(t4 + t11, min, max) as int32_t;
    *c.offset((5 as ptrdiff_t * stride) as isize) = iclip(t5 + t10a, min, max)
        as int32_t;
    *c.offset((6 as ptrdiff_t * stride) as isize) = iclip(t6 + t9, min, max) as int32_t;
    *c.offset((7 as ptrdiff_t * stride) as isize) = iclip(t7 + t8a, min, max) as int32_t;
    *c.offset((8 as ptrdiff_t * stride) as isize) = iclip(t7 - t8a, min, max) as int32_t;
    *c.offset((9 as ptrdiff_t * stride) as isize) = iclip(t6 - t9, min, max) as int32_t;
    *c.offset((10 as ptrdiff_t * stride) as isize) = iclip(t5 - t10a, min, max)
        as int32_t;
    *c.offset((11 as ptrdiff_t * stride) as isize) = iclip(t4 - t11, min, max)
        as int32_t;
    *c.offset((12 as ptrdiff_t * stride) as isize) = iclip(t3 - t12, min, max)
        as int32_t;
    *c.offset((13 as ptrdiff_t * stride) as isize) = iclip(t2 - t13a, min, max)
        as int32_t;
    *c.offset((14 as ptrdiff_t * stride) as isize) = iclip(t1 - t14, min, max)
        as int32_t;
    *c.offset((15 as ptrdiff_t * stride) as isize) = iclip(t0 - t15a, min, max)
        as int32_t;
}
unsafe extern "C" fn inv_dct16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_dct16_1d_internal_c(c, stride, min, max, 0 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_dct32_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    tx64: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            250 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 95],
                [core::ffi::c_char; 95],
            >(
                *b"void inv_dct32_1d_internal_c(int32_t *const, const ptrdiff_t, const int, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    inv_dct16_1d_internal_c(c, stride << 1 as core::ffi::c_int, min, max, tx64);
    let in1: core::ffi::c_int = *c.offset((1 as ptrdiff_t * stride) as isize);
    let in3: core::ffi::c_int = *c.offset((3 as ptrdiff_t * stride) as isize);
    let in5: core::ffi::c_int = *c.offset((5 as ptrdiff_t * stride) as isize);
    let in7: core::ffi::c_int = *c.offset((7 as ptrdiff_t * stride) as isize);
    let in9: core::ffi::c_int = *c.offset((9 as ptrdiff_t * stride) as isize);
    let in11: core::ffi::c_int = *c.offset((11 as ptrdiff_t * stride) as isize);
    let in13: core::ffi::c_int = *c.offset((13 as ptrdiff_t * stride) as isize);
    let in15: core::ffi::c_int = *c.offset((15 as ptrdiff_t * stride) as isize);
    let mut t16a: core::ffi::c_int = 0;
    let mut t17a: core::ffi::c_int = 0;
    let mut t18a: core::ffi::c_int = 0;
    let mut t19a: core::ffi::c_int = 0;
    let mut t20a: core::ffi::c_int = 0;
    let mut t21a: core::ffi::c_int = 0;
    let mut t22a: core::ffi::c_int = 0;
    let mut t23a: core::ffi::c_int = 0;
    let mut t24a: core::ffi::c_int = 0;
    let mut t25a: core::ffi::c_int = 0;
    let mut t26a: core::ffi::c_int = 0;
    let mut t27a: core::ffi::c_int = 0;
    let mut t28a: core::ffi::c_int = 0;
    let mut t29a: core::ffi::c_int = 0;
    let mut t30a: core::ffi::c_int = 0;
    let mut t31a: core::ffi::c_int = 0;
    if tx64 != 0 {
        t16a = in1 * 201 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t17a = in15 * -(2751 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t18a = in9 * 1751 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t19a = in7 * -(1380 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t20a = in5 * 995 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t21a = in11 * -(2106 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t22a = in13 * 2440 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t23a = in3 * -(601 as core::ffi::c_int) + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t24a = in3 * 4052 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t25a = in13 * 3290 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t26a = in11 * 3513 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t27a = in5 * 3973 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t28a = in7 * 3857 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t29a = in9 * 3703 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t30a = in15 * 3035 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
        t31a = in1 * 4091 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int;
    } else {
        let in17: core::ffi::c_int = *c.offset((17 as ptrdiff_t * stride) as isize);
        let in19: core::ffi::c_int = *c.offset((19 as ptrdiff_t * stride) as isize);
        let in21: core::ffi::c_int = *c.offset((21 as ptrdiff_t * stride) as isize);
        let in23: core::ffi::c_int = *c.offset((23 as ptrdiff_t * stride) as isize);
        let in25: core::ffi::c_int = *c.offset((25 as ptrdiff_t * stride) as isize);
        let in27: core::ffi::c_int = *c.offset((27 as ptrdiff_t * stride) as isize);
        let in29: core::ffi::c_int = *c.offset((29 as ptrdiff_t * stride) as isize);
        let in31: core::ffi::c_int = *c.offset((31 as ptrdiff_t * stride) as isize);
        t16a = (in1 * 201 as core::ffi::c_int
            - in31 * (4091 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in31;
        t17a = (in17 * (3035 as core::ffi::c_int - 4096 as core::ffi::c_int)
            - in15 * 2751 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in17;
        t18a = (in9 * 1751 as core::ffi::c_int
            - in23 * (3703 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in23;
        t19a = (in25 * (3857 as core::ffi::c_int - 4096 as core::ffi::c_int)
            - in7 * 1380 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in25;
        t20a = (in5 * 995 as core::ffi::c_int
            - in27 * (3973 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in27;
        t21a = (in21 * (3513 as core::ffi::c_int - 4096 as core::ffi::c_int)
            - in11 * 2106 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in21;
        t22a = in13 * 1220 as core::ffi::c_int - in19 * 1645 as core::ffi::c_int
            + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
        t23a = (in29 * (4052 as core::ffi::c_int - 4096 as core::ffi::c_int)
            - in3 * 601 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in29;
        t24a = (in29 * 601 as core::ffi::c_int
            + in3 * (4052 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in3;
        t25a = in13 * 1645 as core::ffi::c_int + in19 * 1220 as core::ffi::c_int
            + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
        t26a = (in21 * 2106 as core::ffi::c_int
            + in11 * (3513 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in11;
        t27a = (in5 * (3973 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in27 * 995 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in5;
        t28a = (in25 * 1380 as core::ffi::c_int
            + in7 * (3857 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in7;
        t29a = (in9 * (3703 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in23 * 1751 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in9;
        t30a = (in17 * 2751 as core::ffi::c_int
            + in15 * (3035 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in15;
        t31a = (in1 * (4091 as core::ffi::c_int - 4096 as core::ffi::c_int)
            + in31 * 201 as core::ffi::c_int + 2048 as core::ffi::c_int
            >> 12 as core::ffi::c_int) + in1;
    }
    let mut t16: core::ffi::c_int = iclip(t16a + t17a, min, max);
    let mut t17: core::ffi::c_int = iclip(t16a - t17a, min, max);
    let mut t18: core::ffi::c_int = iclip(t19a - t18a, min, max);
    let mut t19: core::ffi::c_int = iclip(t19a + t18a, min, max);
    let mut t20: core::ffi::c_int = iclip(t20a + t21a, min, max);
    let mut t21: core::ffi::c_int = iclip(t20a - t21a, min, max);
    let mut t22: core::ffi::c_int = iclip(t23a - t22a, min, max);
    let mut t23: core::ffi::c_int = iclip(t23a + t22a, min, max);
    let mut t24: core::ffi::c_int = iclip(t24a + t25a, min, max);
    let mut t25: core::ffi::c_int = iclip(t24a - t25a, min, max);
    let mut t26: core::ffi::c_int = iclip(t27a - t26a, min, max);
    let mut t27: core::ffi::c_int = iclip(t27a + t26a, min, max);
    let mut t28: core::ffi::c_int = iclip(t28a + t29a, min, max);
    let mut t29: core::ffi::c_int = iclip(t28a - t29a, min, max);
    let mut t30: core::ffi::c_int = iclip(t31a - t30a, min, max);
    let mut t31: core::ffi::c_int = iclip(t31a + t30a, min, max);
    t17a = (t30 * 799 as core::ffi::c_int
        - t17 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t17;
    t30a = (t30 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t17 * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t30;
    t18a = (-(t29 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t18 * 799 as core::ffi::c_int) + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t29;
    t29a = (t29 * 799 as core::ffi::c_int
        - t18 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t18;
    t21a = t26 * 1703 as core::ffi::c_int - t21 * 1138 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t26a = t26 * 1138 as core::ffi::c_int + t21 * 1703 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t22a = -(t25 * 1138 as core::ffi::c_int + t22 * 1703 as core::ffi::c_int)
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t25a = t25 * 1703 as core::ffi::c_int - t22 * 1138 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t16a = iclip(t16 + t19, min, max);
    t17 = iclip(t17a + t18a, min, max);
    t18 = iclip(t17a - t18a, min, max);
    t19a = iclip(t16 - t19, min, max);
    t20a = iclip(t23 - t20, min, max);
    t21 = iclip(t22a - t21a, min, max);
    t22 = iclip(t22a + t21a, min, max);
    t23a = iclip(t23 + t20, min, max);
    t24a = iclip(t24 + t27, min, max);
    t25 = iclip(t25a + t26a, min, max);
    t26 = iclip(t25a - t26a, min, max);
    t27a = iclip(t24 - t27, min, max);
    t28a = iclip(t31 - t28, min, max);
    t29 = iclip(t30a - t29a, min, max);
    t30 = iclip(t30a + t29a, min, max);
    t31a = iclip(t31 + t28, min, max);
    t18a = (t29 * 1567 as core::ffi::c_int
        - t18 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t18;
    t29a = (t29 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t18 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t29;
    t19 = (t28a * 1567 as core::ffi::c_int
        - t19a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t19a;
    t28 = (t28a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t19a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t28a;
    t20 = (-(t27a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t20a * 1567 as core::ffi::c_int) + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t27a;
    t27 = (t27a * 1567 as core::ffi::c_int
        - t20a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t20a;
    t21a = (-(t26 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t21 * 1567 as core::ffi::c_int) + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t26;
    t26a = (t26 * 1567 as core::ffi::c_int
        - t21 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t21;
    t16 = iclip(t16a + t23a, min, max);
    t17a = iclip(t17 + t22, min, max);
    t18 = iclip(t18a + t21a, min, max);
    t19a = iclip(t19 + t20, min, max);
    t20a = iclip(t19 - t20, min, max);
    t21 = iclip(t18a - t21a, min, max);
    t22a = iclip(t17 - t22, min, max);
    t23 = iclip(t16a - t23a, min, max);
    t24 = iclip(t31a - t24a, min, max);
    t25a = iclip(t30 - t25, min, max);
    t26 = iclip(t29a - t26a, min, max);
    t27a = iclip(t28 - t27, min, max);
    t28a = iclip(t28 + t27, min, max);
    t29 = iclip(t29a + t26a, min, max);
    t30a = iclip(t30 + t25, min, max);
    t31 = iclip(t31a + t24a, min, max);
    t20 = (t27a - t20a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t27 = (t27a + t20a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t21a = (t26 - t21) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t26a = (t26 + t21) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t22 = (t25a - t22a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t25 = (t25a + t22a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t23a = (t24 - t23) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t24a = (t24 + t23) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    let t0: core::ffi::c_int = *c.offset((0 as ptrdiff_t * stride) as isize);
    let t1: core::ffi::c_int = *c.offset((2 as ptrdiff_t * stride) as isize);
    let t2: core::ffi::c_int = *c.offset((4 as ptrdiff_t * stride) as isize);
    let t3: core::ffi::c_int = *c.offset((6 as ptrdiff_t * stride) as isize);
    let t4: core::ffi::c_int = *c.offset((8 as ptrdiff_t * stride) as isize);
    let t5: core::ffi::c_int = *c.offset((10 as ptrdiff_t * stride) as isize);
    let t6: core::ffi::c_int = *c.offset((12 as ptrdiff_t * stride) as isize);
    let t7: core::ffi::c_int = *c.offset((14 as ptrdiff_t * stride) as isize);
    let t8: core::ffi::c_int = *c.offset((16 as ptrdiff_t * stride) as isize);
    let t9: core::ffi::c_int = *c.offset((18 as ptrdiff_t * stride) as isize);
    let t10: core::ffi::c_int = *c.offset((20 as ptrdiff_t * stride) as isize);
    let t11: core::ffi::c_int = *c.offset((22 as ptrdiff_t * stride) as isize);
    let t12: core::ffi::c_int = *c.offset((24 as ptrdiff_t * stride) as isize);
    let t13: core::ffi::c_int = *c.offset((26 as ptrdiff_t * stride) as isize);
    let t14: core::ffi::c_int = *c.offset((28 as ptrdiff_t * stride) as isize);
    let t15: core::ffi::c_int = *c.offset((30 as ptrdiff_t * stride) as isize);
    *c.offset((0 as ptrdiff_t * stride) as isize) = iclip(t0 + t31, min, max) as int32_t;
    *c.offset((1 as ptrdiff_t * stride) as isize) = iclip(t1 + t30a, min, max)
        as int32_t;
    *c.offset((2 as ptrdiff_t * stride) as isize) = iclip(t2 + t29, min, max) as int32_t;
    *c.offset((3 as ptrdiff_t * stride) as isize) = iclip(t3 + t28a, min, max)
        as int32_t;
    *c.offset((4 as ptrdiff_t * stride) as isize) = iclip(t4 + t27, min, max) as int32_t;
    *c.offset((5 as ptrdiff_t * stride) as isize) = iclip(t5 + t26a, min, max)
        as int32_t;
    *c.offset((6 as ptrdiff_t * stride) as isize) = iclip(t6 + t25, min, max) as int32_t;
    *c.offset((7 as ptrdiff_t * stride) as isize) = iclip(t7 + t24a, min, max)
        as int32_t;
    *c.offset((8 as ptrdiff_t * stride) as isize) = iclip(t8 + t23a, min, max)
        as int32_t;
    *c.offset((9 as ptrdiff_t * stride) as isize) = iclip(t9 + t22, min, max) as int32_t;
    *c.offset((10 as ptrdiff_t * stride) as isize) = iclip(t10 + t21a, min, max)
        as int32_t;
    *c.offset((11 as ptrdiff_t * stride) as isize) = iclip(t11 + t20, min, max)
        as int32_t;
    *c.offset((12 as ptrdiff_t * stride) as isize) = iclip(t12 + t19a, min, max)
        as int32_t;
    *c.offset((13 as ptrdiff_t * stride) as isize) = iclip(t13 + t18, min, max)
        as int32_t;
    *c.offset((14 as ptrdiff_t * stride) as isize) = iclip(t14 + t17a, min, max)
        as int32_t;
    *c.offset((15 as ptrdiff_t * stride) as isize) = iclip(t15 + t16, min, max)
        as int32_t;
    *c.offset((16 as ptrdiff_t * stride) as isize) = iclip(t15 - t16, min, max)
        as int32_t;
    *c.offset((17 as ptrdiff_t * stride) as isize) = iclip(t14 - t17a, min, max)
        as int32_t;
    *c.offset((18 as ptrdiff_t * stride) as isize) = iclip(t13 - t18, min, max)
        as int32_t;
    *c.offset((19 as ptrdiff_t * stride) as isize) = iclip(t12 - t19a, min, max)
        as int32_t;
    *c.offset((20 as ptrdiff_t * stride) as isize) = iclip(t11 - t20, min, max)
        as int32_t;
    *c.offset((21 as ptrdiff_t * stride) as isize) = iclip(t10 - t21a, min, max)
        as int32_t;
    *c.offset((22 as ptrdiff_t * stride) as isize) = iclip(t9 - t22, min, max)
        as int32_t;
    *c.offset((23 as ptrdiff_t * stride) as isize) = iclip(t8 - t23a, min, max)
        as int32_t;
    *c.offset((24 as ptrdiff_t * stride) as isize) = iclip(t7 - t24a, min, max)
        as int32_t;
    *c.offset((25 as ptrdiff_t * stride) as isize) = iclip(t6 - t25, min, max)
        as int32_t;
    *c.offset((26 as ptrdiff_t * stride) as isize) = iclip(t5 - t26a, min, max)
        as int32_t;
    *c.offset((27 as ptrdiff_t * stride) as isize) = iclip(t4 - t27, min, max)
        as int32_t;
    *c.offset((28 as ptrdiff_t * stride) as isize) = iclip(t3 - t28a, min, max)
        as int32_t;
    *c.offset((29 as ptrdiff_t * stride) as isize) = iclip(t2 - t29, min, max)
        as int32_t;
    *c.offset((30 as ptrdiff_t * stride) as isize) = iclip(t1 - t30a, min, max)
        as int32_t;
    *c.offset((31 as ptrdiff_t * stride) as isize) = iclip(t0 - t31, min, max)
        as int32_t;
}
unsafe extern "C" fn inv_dct32_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_dct32_1d_internal_c(c, stride, min, max, 0 as core::ffi::c_int);
}
unsafe extern "C" fn inv_dct64_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            439 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 75],
                [core::ffi::c_char; 75],
            >(
                *b"void inv_dct64_1d_c(int32_t *const, const ptrdiff_t, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    inv_dct32_1d_internal_c(
        c,
        stride << 1 as core::ffi::c_int,
        min,
        max,
        1 as core::ffi::c_int,
    );
    let in1: core::ffi::c_int = *c.offset((1 as ptrdiff_t * stride) as isize);
    let in3: core::ffi::c_int = *c.offset((3 as ptrdiff_t * stride) as isize);
    let in5: core::ffi::c_int = *c.offset((5 as ptrdiff_t * stride) as isize);
    let in7: core::ffi::c_int = *c.offset((7 as ptrdiff_t * stride) as isize);
    let in9: core::ffi::c_int = *c.offset((9 as ptrdiff_t * stride) as isize);
    let in11: core::ffi::c_int = *c.offset((11 as ptrdiff_t * stride) as isize);
    let in13: core::ffi::c_int = *c.offset((13 as ptrdiff_t * stride) as isize);
    let in15: core::ffi::c_int = *c.offset((15 as ptrdiff_t * stride) as isize);
    let in17: core::ffi::c_int = *c.offset((17 as ptrdiff_t * stride) as isize);
    let in19: core::ffi::c_int = *c.offset((19 as ptrdiff_t * stride) as isize);
    let in21: core::ffi::c_int = *c.offset((21 as ptrdiff_t * stride) as isize);
    let in23: core::ffi::c_int = *c.offset((23 as ptrdiff_t * stride) as isize);
    let in25: core::ffi::c_int = *c.offset((25 as ptrdiff_t * stride) as isize);
    let in27: core::ffi::c_int = *c.offset((27 as ptrdiff_t * stride) as isize);
    let in29: core::ffi::c_int = *c.offset((29 as ptrdiff_t * stride) as isize);
    let in31: core::ffi::c_int = *c.offset((31 as ptrdiff_t * stride) as isize);
    let mut t32a: core::ffi::c_int = in1 * 101 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t33a: core::ffi::c_int = in31 * -(2824 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t34a: core::ffi::c_int = in17 * 1660 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t35a: core::ffi::c_int = in15 * -(1474 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t36a: core::ffi::c_int = in9 * 897 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t37a: core::ffi::c_int = in23 * -(2191 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t38a: core::ffi::c_int = in25 * 2359 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t39a: core::ffi::c_int = in7 * -(700 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t40a: core::ffi::c_int = in5 * 501 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t41a: core::ffi::c_int = in27 * -(2520 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t42a: core::ffi::c_int = in21 * 2019 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t43a: core::ffi::c_int = in11 * -(1092 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t44a: core::ffi::c_int = in13 * 1285 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t45a: core::ffi::c_int = in19 * -(1842 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t46a: core::ffi::c_int = in29 * 2675 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t47a: core::ffi::c_int = in3 * -(301 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t48a: core::ffi::c_int = in3 * 4085 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t49a: core::ffi::c_int = in29 * 3102 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t50a: core::ffi::c_int = in19 * 3659 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t51a: core::ffi::c_int = in13 * 3889 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t52a: core::ffi::c_int = in11 * 3948 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t53a: core::ffi::c_int = in21 * 3564 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t54a: core::ffi::c_int = in27 * 3229 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t55a: core::ffi::c_int = in5 * 4065 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t56a: core::ffi::c_int = in7 * 4036 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t57a: core::ffi::c_int = in25 * 3349 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t58a: core::ffi::c_int = in23 * 3461 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t59a: core::ffi::c_int = in9 * 3996 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t60a: core::ffi::c_int = in15 * 3822 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t61a: core::ffi::c_int = in17 * 3745 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t62a: core::ffi::c_int = in31 * 2967 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t63a: core::ffi::c_int = in1 * 4095 as core::ffi::c_int
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
    let mut t32: core::ffi::c_int = iclip(t32a + t33a, min, max);
    let mut t33: core::ffi::c_int = iclip(t32a - t33a, min, max);
    let mut t34: core::ffi::c_int = iclip(t35a - t34a, min, max);
    let mut t35: core::ffi::c_int = iclip(t35a + t34a, min, max);
    let mut t36: core::ffi::c_int = iclip(t36a + t37a, min, max);
    let mut t37: core::ffi::c_int = iclip(t36a - t37a, min, max);
    let mut t38: core::ffi::c_int = iclip(t39a - t38a, min, max);
    let mut t39: core::ffi::c_int = iclip(t39a + t38a, min, max);
    let mut t40: core::ffi::c_int = iclip(t40a + t41a, min, max);
    let mut t41: core::ffi::c_int = iclip(t40a - t41a, min, max);
    let mut t42: core::ffi::c_int = iclip(t43a - t42a, min, max);
    let mut t43: core::ffi::c_int = iclip(t43a + t42a, min, max);
    let mut t44: core::ffi::c_int = iclip(t44a + t45a, min, max);
    let mut t45: core::ffi::c_int = iclip(t44a - t45a, min, max);
    let mut t46: core::ffi::c_int = iclip(t47a - t46a, min, max);
    let mut t47: core::ffi::c_int = iclip(t47a + t46a, min, max);
    let mut t48: core::ffi::c_int = iclip(t48a + t49a, min, max);
    let mut t49: core::ffi::c_int = iclip(t48a - t49a, min, max);
    let mut t50: core::ffi::c_int = iclip(t51a - t50a, min, max);
    let mut t51: core::ffi::c_int = iclip(t51a + t50a, min, max);
    let mut t52: core::ffi::c_int = iclip(t52a + t53a, min, max);
    let mut t53: core::ffi::c_int = iclip(t52a - t53a, min, max);
    let mut t54: core::ffi::c_int = iclip(t55a - t54a, min, max);
    let mut t55: core::ffi::c_int = iclip(t55a + t54a, min, max);
    let mut t56: core::ffi::c_int = iclip(t56a + t57a, min, max);
    let mut t57: core::ffi::c_int = iclip(t56a - t57a, min, max);
    let mut t58: core::ffi::c_int = iclip(t59a - t58a, min, max);
    let mut t59: core::ffi::c_int = iclip(t59a + t58a, min, max);
    let mut t60: core::ffi::c_int = iclip(t60a + t61a, min, max);
    let mut t61: core::ffi::c_int = iclip(t60a - t61a, min, max);
    let mut t62: core::ffi::c_int = iclip(t63a - t62a, min, max);
    let mut t63: core::ffi::c_int = iclip(t63a + t62a, min, max);
    t33a = (t33 * (4096 as core::ffi::c_int - 4076 as core::ffi::c_int)
        + t62 * 401 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t33;
    t34a = (t34 * -(401 as core::ffi::c_int)
        + t61 * (4096 as core::ffi::c_int - 4076 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t61;
    t37a = t37 * -(1299 as core::ffi::c_int) + t58 * 1583 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t38a = t38 * -(1583 as core::ffi::c_int) + t57 * -(1299 as core::ffi::c_int)
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t41a = (t41 * (4096 as core::ffi::c_int - 3612 as core::ffi::c_int)
        + t54 * 1931 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t41;
    t42a = (t42 * -(1931 as core::ffi::c_int)
        + t53 * (4096 as core::ffi::c_int - 3612 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t53;
    t45a = (t45 * -(1189 as core::ffi::c_int)
        + t50 * (3920 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t50;
    t46a = (t46 * (4096 as core::ffi::c_int - 3920 as core::ffi::c_int)
        + t49 * -(1189 as core::ffi::c_int) + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t46;
    t49a = (t46 * -(1189 as core::ffi::c_int)
        + t49 * (3920 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t49;
    t50a = (t45 * (3920 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t50 * 1189 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t45;
    t53a = (t42 * (4096 as core::ffi::c_int - 3612 as core::ffi::c_int)
        + t53 * 1931 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t42;
    t54a = (t41 * 1931 as core::ffi::c_int
        + t54 * (3612 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t54;
    t57a = t38 * -(1299 as core::ffi::c_int) + t57 * 1583 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t58a = t37 * 1583 as core::ffi::c_int + t58 * 1299 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t61a = (t34 * (4096 as core::ffi::c_int - 4076 as core::ffi::c_int)
        + t61 * 401 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t34;
    t62a = (t33 * 401 as core::ffi::c_int
        + t62 * (4076 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t62;
    t32a = iclip(t32 + t35, min, max);
    t33 = iclip(t33a + t34a, min, max);
    t34 = iclip(t33a - t34a, min, max);
    t35a = iclip(t32 - t35, min, max);
    t36a = iclip(t39 - t36, min, max);
    t37 = iclip(t38a - t37a, min, max);
    t38 = iclip(t38a + t37a, min, max);
    t39a = iclip(t39 + t36, min, max);
    t40a = iclip(t40 + t43, min, max);
    t41 = iclip(t41a + t42a, min, max);
    t42 = iclip(t41a - t42a, min, max);
    t43a = iclip(t40 - t43, min, max);
    t44a = iclip(t47 - t44, min, max);
    t45 = iclip(t46a - t45a, min, max);
    t46 = iclip(t46a + t45a, min, max);
    t47a = iclip(t47 + t44, min, max);
    t48a = iclip(t48 + t51, min, max);
    t49 = iclip(t49a + t50a, min, max);
    t50 = iclip(t49a - t50a, min, max);
    t51a = iclip(t48 - t51, min, max);
    t52a = iclip(t55 - t52, min, max);
    t53 = iclip(t54a - t53a, min, max);
    t54 = iclip(t54a + t53a, min, max);
    t55a = iclip(t55 + t52, min, max);
    t56a = iclip(t56 + t59, min, max);
    t57 = iclip(t57a + t58a, min, max);
    t58 = iclip(t57a - t58a, min, max);
    t59a = iclip(t56 - t59, min, max);
    t60a = iclip(t63 - t60, min, max);
    t61 = iclip(t62a - t61a, min, max);
    t62 = iclip(t62a + t61a, min, max);
    t63a = iclip(t63 + t60, min, max);
    t34a = (t34 * (4096 as core::ffi::c_int - 4017 as core::ffi::c_int)
        + t61 * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t34;
    t35 = (t35a * (4096 as core::ffi::c_int - 4017 as core::ffi::c_int)
        + t60a * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t35a;
    t36 = (t36a * -(799 as core::ffi::c_int)
        + t59a * (4096 as core::ffi::c_int - 4017 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t59a;
    t37a = (t37 * -(799 as core::ffi::c_int)
        + t58 * (4096 as core::ffi::c_int - 4017 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t58;
    t42a = t42 * -(1138 as core::ffi::c_int) + t53 * 1703 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t43 = t43a * -(1138 as core::ffi::c_int) + t52a * 1703 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t44 = t44a * -(1703 as core::ffi::c_int) + t51a * -(1138 as core::ffi::c_int)
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t45a = t45 * -(1703 as core::ffi::c_int) + t50 * -(1138 as core::ffi::c_int)
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t50a = t45 * -(1138 as core::ffi::c_int) + t50 * 1703 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t51 = t44a * -(1138 as core::ffi::c_int) + t51a * 1703 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t52 = t43a * 1703 as core::ffi::c_int + t52a * 1138 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t53a = t42 * 1703 as core::ffi::c_int + t53 * 1138 as core::ffi::c_int
        + 1024 as core::ffi::c_int >> 11 as core::ffi::c_int;
    t58a = (t37 * (4096 as core::ffi::c_int - 4017 as core::ffi::c_int)
        + t58 * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t37;
    t59 = (t36a * (4096 as core::ffi::c_int - 4017 as core::ffi::c_int)
        + t59a * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t36a;
    t60 = (t35a * 799 as core::ffi::c_int
        + t60a * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t60a;
    t61a = (t34 * 799 as core::ffi::c_int
        + t61 * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t61;
    t32 = iclip(t32a + t39a, min, max);
    t33a = iclip(t33 + t38, min, max);
    t34 = iclip(t34a + t37a, min, max);
    t35a = iclip(t35 + t36, min, max);
    t36a = iclip(t35 - t36, min, max);
    t37 = iclip(t34a - t37a, min, max);
    t38a = iclip(t33 - t38, min, max);
    t39 = iclip(t32a - t39a, min, max);
    t40 = iclip(t47a - t40a, min, max);
    t41a = iclip(t46 - t41, min, max);
    t42 = iclip(t45a - t42a, min, max);
    t43a = iclip(t44 - t43, min, max);
    t44a = iclip(t44 + t43, min, max);
    t45 = iclip(t45a + t42a, min, max);
    t46a = iclip(t46 + t41, min, max);
    t47 = iclip(t47a + t40a, min, max);
    t48 = iclip(t48a + t55a, min, max);
    t49a = iclip(t49 + t54, min, max);
    t50 = iclip(t50a + t53a, min, max);
    t51a = iclip(t51 + t52, min, max);
    t52a = iclip(t51 - t52, min, max);
    t53 = iclip(t50a - t53a, min, max);
    t54a = iclip(t49 - t54, min, max);
    t55 = iclip(t48a - t55a, min, max);
    t56 = iclip(t63a - t56a, min, max);
    t57a = iclip(t62 - t57, min, max);
    t58 = iclip(t61a - t58a, min, max);
    t59a = iclip(t60 - t59, min, max);
    t60a = iclip(t60 + t59, min, max);
    t61 = iclip(t61a + t58a, min, max);
    t62a = iclip(t62 + t57, min, max);
    t63 = iclip(t63a + t56a, min, max);
    t36 = (t36a * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t59a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t36a;
    t37a = (t37 * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t58 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t37;
    t38 = (t38a * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t57a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t38a;
    t39a = (t39 * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t56 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t39;
    t40a = (t40 * -(1567 as core::ffi::c_int)
        + t55 * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t55;
    t41 = (t41a * -(1567 as core::ffi::c_int)
        + t54a * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t54a;
    t42a = (t42 * -(1567 as core::ffi::c_int)
        + t53 * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t53;
    t43 = (t43a * -(1567 as core::ffi::c_int)
        + t52a * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t52a;
    t52 = (t43a * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t52a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t43a;
    t53a = (t42 * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t53 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t42;
    t54 = (t41a * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t54a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t41a;
    t55a = (t40 * (4096 as core::ffi::c_int - 3784 as core::ffi::c_int)
        + t55 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) - t40;
    t56a = (t39 * 1567 as core::ffi::c_int
        + t56 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t56;
    t57 = (t38a * 1567 as core::ffi::c_int
        + t57a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t57a;
    t58a = (t37 * 1567 as core::ffi::c_int
        + t58 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t58;
    t59 = (t36a * 1567 as core::ffi::c_int
        + t59a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t59a;
    t32a = iclip(t32 + t47, min, max);
    t33 = iclip(t33a + t46a, min, max);
    t34a = iclip(t34 + t45, min, max);
    t35 = iclip(t35a + t44a, min, max);
    t36a = iclip(t36 + t43, min, max);
    t37 = iclip(t37a + t42a, min, max);
    t38a = iclip(t38 + t41, min, max);
    t39 = iclip(t39a + t40a, min, max);
    t40 = iclip(t39a - t40a, min, max);
    t41a = iclip(t38 - t41, min, max);
    t42 = iclip(t37a - t42a, min, max);
    t43a = iclip(t36 - t43, min, max);
    t44 = iclip(t35a - t44a, min, max);
    t45a = iclip(t34 - t45, min, max);
    t46 = iclip(t33a - t46a, min, max);
    t47a = iclip(t32 - t47, min, max);
    t48a = iclip(t63 - t48, min, max);
    t49 = iclip(t62a - t49a, min, max);
    t50a = iclip(t61 - t50, min, max);
    t51 = iclip(t60a - t51a, min, max);
    t52a = iclip(t59 - t52, min, max);
    t53 = iclip(t58a - t53a, min, max);
    t54a = iclip(t57 - t54, min, max);
    t55 = iclip(t56a - t55a, min, max);
    t56 = iclip(t56a + t55a, min, max);
    t57a = iclip(t57 + t54, min, max);
    t58 = iclip(t58a + t53a, min, max);
    t59a = iclip(t59 + t52, min, max);
    t60 = iclip(t60a + t51a, min, max);
    t61a = iclip(t61 + t50, min, max);
    t62 = iclip(t62a + t49a, min, max);
    t63a = iclip(t63 + t48, min, max);
    t40a = (t55 - t40) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t41 = (t54a - t41a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t42a = (t53 - t42) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t43 = (t52a - t43a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t44a = (t51 - t44) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t45 = (t50a - t45a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t46a = (t49 - t46) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t47 = (t48a - t47a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t48 = (t47a + t48a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t49a = (t46 + t49) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t50 = (t45a + t50a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t51a = (t44 + t51) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t52 = (t43a + t52a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t53a = (t42 + t53) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t54 = (t41a + t54a) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    t55a = (t40 + t55) * 181 as core::ffi::c_int + 128 as core::ffi::c_int
        >> 8 as core::ffi::c_int;
    let t0: core::ffi::c_int = *c.offset((0 as ptrdiff_t * stride) as isize);
    let t1: core::ffi::c_int = *c.offset((2 as ptrdiff_t * stride) as isize);
    let t2: core::ffi::c_int = *c.offset((4 as ptrdiff_t * stride) as isize);
    let t3: core::ffi::c_int = *c.offset((6 as ptrdiff_t * stride) as isize);
    let t4: core::ffi::c_int = *c.offset((8 as ptrdiff_t * stride) as isize);
    let t5: core::ffi::c_int = *c.offset((10 as ptrdiff_t * stride) as isize);
    let t6: core::ffi::c_int = *c.offset((12 as ptrdiff_t * stride) as isize);
    let t7: core::ffi::c_int = *c.offset((14 as ptrdiff_t * stride) as isize);
    let t8: core::ffi::c_int = *c.offset((16 as ptrdiff_t * stride) as isize);
    let t9: core::ffi::c_int = *c.offset((18 as ptrdiff_t * stride) as isize);
    let t10: core::ffi::c_int = *c.offset((20 as ptrdiff_t * stride) as isize);
    let t11: core::ffi::c_int = *c.offset((22 as ptrdiff_t * stride) as isize);
    let t12: core::ffi::c_int = *c.offset((24 as ptrdiff_t * stride) as isize);
    let t13: core::ffi::c_int = *c.offset((26 as ptrdiff_t * stride) as isize);
    let t14: core::ffi::c_int = *c.offset((28 as ptrdiff_t * stride) as isize);
    let t15: core::ffi::c_int = *c.offset((30 as ptrdiff_t * stride) as isize);
    let t16: core::ffi::c_int = *c.offset((32 as ptrdiff_t * stride) as isize);
    let t17: core::ffi::c_int = *c.offset((34 as ptrdiff_t * stride) as isize);
    let t18: core::ffi::c_int = *c.offset((36 as ptrdiff_t * stride) as isize);
    let t19: core::ffi::c_int = *c.offset((38 as ptrdiff_t * stride) as isize);
    let t20: core::ffi::c_int = *c.offset((40 as ptrdiff_t * stride) as isize);
    let t21: core::ffi::c_int = *c.offset((42 as ptrdiff_t * stride) as isize);
    let t22: core::ffi::c_int = *c.offset((44 as ptrdiff_t * stride) as isize);
    let t23: core::ffi::c_int = *c.offset((46 as ptrdiff_t * stride) as isize);
    let t24: core::ffi::c_int = *c.offset((48 as ptrdiff_t * stride) as isize);
    let t25: core::ffi::c_int = *c.offset((50 as ptrdiff_t * stride) as isize);
    let t26: core::ffi::c_int = *c.offset((52 as ptrdiff_t * stride) as isize);
    let t27: core::ffi::c_int = *c.offset((54 as ptrdiff_t * stride) as isize);
    let t28: core::ffi::c_int = *c.offset((56 as ptrdiff_t * stride) as isize);
    let t29: core::ffi::c_int = *c.offset((58 as ptrdiff_t * stride) as isize);
    let t30: core::ffi::c_int = *c.offset((60 as ptrdiff_t * stride) as isize);
    let t31: core::ffi::c_int = *c.offset((62 as ptrdiff_t * stride) as isize);
    *c.offset((0 as ptrdiff_t * stride) as isize) = iclip(t0 + t63a, min, max)
        as int32_t;
    *c.offset((1 as ptrdiff_t * stride) as isize) = iclip(t1 + t62, min, max) as int32_t;
    *c.offset((2 as ptrdiff_t * stride) as isize) = iclip(t2 + t61a, min, max)
        as int32_t;
    *c.offset((3 as ptrdiff_t * stride) as isize) = iclip(t3 + t60, min, max) as int32_t;
    *c.offset((4 as ptrdiff_t * stride) as isize) = iclip(t4 + t59a, min, max)
        as int32_t;
    *c.offset((5 as ptrdiff_t * stride) as isize) = iclip(t5 + t58, min, max) as int32_t;
    *c.offset((6 as ptrdiff_t * stride) as isize) = iclip(t6 + t57a, min, max)
        as int32_t;
    *c.offset((7 as ptrdiff_t * stride) as isize) = iclip(t7 + t56, min, max) as int32_t;
    *c.offset((8 as ptrdiff_t * stride) as isize) = iclip(t8 + t55a, min, max)
        as int32_t;
    *c.offset((9 as ptrdiff_t * stride) as isize) = iclip(t9 + t54, min, max) as int32_t;
    *c.offset((10 as ptrdiff_t * stride) as isize) = iclip(t10 + t53a, min, max)
        as int32_t;
    *c.offset((11 as ptrdiff_t * stride) as isize) = iclip(t11 + t52, min, max)
        as int32_t;
    *c.offset((12 as ptrdiff_t * stride) as isize) = iclip(t12 + t51a, min, max)
        as int32_t;
    *c.offset((13 as ptrdiff_t * stride) as isize) = iclip(t13 + t50, min, max)
        as int32_t;
    *c.offset((14 as ptrdiff_t * stride) as isize) = iclip(t14 + t49a, min, max)
        as int32_t;
    *c.offset((15 as ptrdiff_t * stride) as isize) = iclip(t15 + t48, min, max)
        as int32_t;
    *c.offset((16 as ptrdiff_t * stride) as isize) = iclip(t16 + t47, min, max)
        as int32_t;
    *c.offset((17 as ptrdiff_t * stride) as isize) = iclip(t17 + t46a, min, max)
        as int32_t;
    *c.offset((18 as ptrdiff_t * stride) as isize) = iclip(t18 + t45, min, max)
        as int32_t;
    *c.offset((19 as ptrdiff_t * stride) as isize) = iclip(t19 + t44a, min, max)
        as int32_t;
    *c.offset((20 as ptrdiff_t * stride) as isize) = iclip(t20 + t43, min, max)
        as int32_t;
    *c.offset((21 as ptrdiff_t * stride) as isize) = iclip(t21 + t42a, min, max)
        as int32_t;
    *c.offset((22 as ptrdiff_t * stride) as isize) = iclip(t22 + t41, min, max)
        as int32_t;
    *c.offset((23 as ptrdiff_t * stride) as isize) = iclip(t23 + t40a, min, max)
        as int32_t;
    *c.offset((24 as ptrdiff_t * stride) as isize) = iclip(t24 + t39, min, max)
        as int32_t;
    *c.offset((25 as ptrdiff_t * stride) as isize) = iclip(t25 + t38a, min, max)
        as int32_t;
    *c.offset((26 as ptrdiff_t * stride) as isize) = iclip(t26 + t37, min, max)
        as int32_t;
    *c.offset((27 as ptrdiff_t * stride) as isize) = iclip(t27 + t36a, min, max)
        as int32_t;
    *c.offset((28 as ptrdiff_t * stride) as isize) = iclip(t28 + t35, min, max)
        as int32_t;
    *c.offset((29 as ptrdiff_t * stride) as isize) = iclip(t29 + t34a, min, max)
        as int32_t;
    *c.offset((30 as ptrdiff_t * stride) as isize) = iclip(t30 + t33, min, max)
        as int32_t;
    *c.offset((31 as ptrdiff_t * stride) as isize) = iclip(t31 + t32a, min, max)
        as int32_t;
    *c.offset((32 as ptrdiff_t * stride) as isize) = iclip(t31 - t32a, min, max)
        as int32_t;
    *c.offset((33 as ptrdiff_t * stride) as isize) = iclip(t30 - t33, min, max)
        as int32_t;
    *c.offset((34 as ptrdiff_t * stride) as isize) = iclip(t29 - t34a, min, max)
        as int32_t;
    *c.offset((35 as ptrdiff_t * stride) as isize) = iclip(t28 - t35, min, max)
        as int32_t;
    *c.offset((36 as ptrdiff_t * stride) as isize) = iclip(t27 - t36a, min, max)
        as int32_t;
    *c.offset((37 as ptrdiff_t * stride) as isize) = iclip(t26 - t37, min, max)
        as int32_t;
    *c.offset((38 as ptrdiff_t * stride) as isize) = iclip(t25 - t38a, min, max)
        as int32_t;
    *c.offset((39 as ptrdiff_t * stride) as isize) = iclip(t24 - t39, min, max)
        as int32_t;
    *c.offset((40 as ptrdiff_t * stride) as isize) = iclip(t23 - t40a, min, max)
        as int32_t;
    *c.offset((41 as ptrdiff_t * stride) as isize) = iclip(t22 - t41, min, max)
        as int32_t;
    *c.offset((42 as ptrdiff_t * stride) as isize) = iclip(t21 - t42a, min, max)
        as int32_t;
    *c.offset((43 as ptrdiff_t * stride) as isize) = iclip(t20 - t43, min, max)
        as int32_t;
    *c.offset((44 as ptrdiff_t * stride) as isize) = iclip(t19 - t44a, min, max)
        as int32_t;
    *c.offset((45 as ptrdiff_t * stride) as isize) = iclip(t18 - t45, min, max)
        as int32_t;
    *c.offset((46 as ptrdiff_t * stride) as isize) = iclip(t17 - t46a, min, max)
        as int32_t;
    *c.offset((47 as ptrdiff_t * stride) as isize) = iclip(t16 - t47, min, max)
        as int32_t;
    *c.offset((48 as ptrdiff_t * stride) as isize) = iclip(t15 - t48, min, max)
        as int32_t;
    *c.offset((49 as ptrdiff_t * stride) as isize) = iclip(t14 - t49a, min, max)
        as int32_t;
    *c.offset((50 as ptrdiff_t * stride) as isize) = iclip(t13 - t50, min, max)
        as int32_t;
    *c.offset((51 as ptrdiff_t * stride) as isize) = iclip(t12 - t51a, min, max)
        as int32_t;
    *c.offset((52 as ptrdiff_t * stride) as isize) = iclip(t11 - t52, min, max)
        as int32_t;
    *c.offset((53 as ptrdiff_t * stride) as isize) = iclip(t10 - t53a, min, max)
        as int32_t;
    *c.offset((54 as ptrdiff_t * stride) as isize) = iclip(t9 - t54, min, max)
        as int32_t;
    *c.offset((55 as ptrdiff_t * stride) as isize) = iclip(t8 - t55a, min, max)
        as int32_t;
    *c.offset((56 as ptrdiff_t * stride) as isize) = iclip(t7 - t56, min, max)
        as int32_t;
    *c.offset((57 as ptrdiff_t * stride) as isize) = iclip(t6 - t57a, min, max)
        as int32_t;
    *c.offset((58 as ptrdiff_t * stride) as isize) = iclip(t5 - t58, min, max)
        as int32_t;
    *c.offset((59 as ptrdiff_t * stride) as isize) = iclip(t4 - t59a, min, max)
        as int32_t;
    *c.offset((60 as ptrdiff_t * stride) as isize) = iclip(t3 - t60, min, max)
        as int32_t;
    *c.offset((61 as ptrdiff_t * stride) as isize) = iclip(t2 - t61a, min, max)
        as int32_t;
    *c.offset((62 as ptrdiff_t * stride) as isize) = iclip(t1 - t62, min, max)
        as int32_t;
    *c.offset((63 as ptrdiff_t * stride) as isize) = iclip(t0 - t63a, min, max)
        as int32_t;
}
#[inline(never)]
unsafe extern "C" fn inv_adst4_1d_internal_c(
    in_0: *const int32_t,
    in_s: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    out: *mut int32_t,
    out_s: ptrdiff_t,
) {
    if in_s > 0 as ptrdiff_t && out_s != 0 as ptrdiff_t {} else {
        __assert_fail(
            b"in_s > 0 && out_s != 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            788 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 123],
                [core::ffi::c_char; 123],
            >(
                *b"void inv_adst4_1d_internal_c(const int32_t *const, const ptrdiff_t, const int, const int, int32_t *const, const ptrdiff_t)\0",
            ))
                .as_ptr(),
        );
    };
    let in0: core::ffi::c_int = *in_0.offset((0 as ptrdiff_t * in_s) as isize);
    let in1: core::ffi::c_int = *in_0.offset((1 as ptrdiff_t * in_s) as isize);
    let in2: core::ffi::c_int = *in_0.offset((2 as ptrdiff_t * in_s) as isize);
    let in3: core::ffi::c_int = *in_0.offset((3 as ptrdiff_t * in_s) as isize);
    *out.offset((0 as ptrdiff_t * out_s) as isize) = ((1321 as core::ffi::c_int * in0
        + (3803 as core::ffi::c_int - 4096 as core::ffi::c_int) * in2
        + (2482 as core::ffi::c_int - 4096 as core::ffi::c_int) * in3
        + (3344 as core::ffi::c_int - 4096 as core::ffi::c_int) * in1
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in2 + in3 + in1)
        as int32_t;
    *out.offset((1 as ptrdiff_t * out_s) as isize) = (((2482 as core::ffi::c_int
        - 4096 as core::ffi::c_int) * in0 - 1321 as core::ffi::c_int * in2
        - (3803 as core::ffi::c_int - 4096 as core::ffi::c_int) * in3
        + (3344 as core::ffi::c_int - 4096 as core::ffi::c_int) * in1
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in0 - in3 + in1)
        as int32_t;
    *out.offset((2 as ptrdiff_t * out_s) as isize) = (209 as core::ffi::c_int
        * (in0 - in2 + in3) + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((3 as ptrdiff_t * out_s) as isize) = (((3803 as core::ffi::c_int
        - 4096 as core::ffi::c_int) * in0
        + (2482 as core::ffi::c_int - 4096 as core::ffi::c_int) * in2
        - 1321 as core::ffi::c_int * in3
        - (3344 as core::ffi::c_int - 4096 as core::ffi::c_int) * in1
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in0 + in2 - in1)
        as int32_t;
}
#[inline(never)]
unsafe extern "C" fn inv_adst8_1d_internal_c(
    in_0: *const int32_t,
    in_s: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    out: *mut int32_t,
    out_s: ptrdiff_t,
) {
    if in_s > 0 as ptrdiff_t && out_s != 0 as ptrdiff_t {} else {
        __assert_fail(
            b"in_s > 0 && out_s != 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            809 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 123],
                [core::ffi::c_char; 123],
            >(
                *b"void inv_adst8_1d_internal_c(const int32_t *const, const ptrdiff_t, const int, const int, int32_t *const, const ptrdiff_t)\0",
            ))
                .as_ptr(),
        );
    };
    let in0: core::ffi::c_int = *in_0.offset((0 as ptrdiff_t * in_s) as isize);
    let in1: core::ffi::c_int = *in_0.offset((1 as ptrdiff_t * in_s) as isize);
    let in2: core::ffi::c_int = *in_0.offset((2 as ptrdiff_t * in_s) as isize);
    let in3: core::ffi::c_int = *in_0.offset((3 as ptrdiff_t * in_s) as isize);
    let in4: core::ffi::c_int = *in_0.offset((4 as ptrdiff_t * in_s) as isize);
    let in5: core::ffi::c_int = *in_0.offset((5 as ptrdiff_t * in_s) as isize);
    let in6: core::ffi::c_int = *in_0.offset((6 as ptrdiff_t * in_s) as isize);
    let in7: core::ffi::c_int = *in_0.offset((7 as ptrdiff_t * in_s) as isize);
    let t0a: core::ffi::c_int = ((4076 as core::ffi::c_int - 4096 as core::ffi::c_int)
        * in7 + 401 as core::ffi::c_int * in0 + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in7;
    let t1a: core::ffi::c_int = (401 as core::ffi::c_int * in7
        - (4076 as core::ffi::c_int - 4096 as core::ffi::c_int) * in0
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in0;
    let t2a: core::ffi::c_int = ((3612 as core::ffi::c_int - 4096 as core::ffi::c_int)
        * in5 + 1931 as core::ffi::c_int * in2 + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in5;
    let t3a: core::ffi::c_int = (1931 as core::ffi::c_int * in5
        - (3612 as core::ffi::c_int - 4096 as core::ffi::c_int) * in2
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in2;
    let mut t4a: core::ffi::c_int = 1299 as core::ffi::c_int * in3
        + 1583 as core::ffi::c_int * in4 + 1024 as core::ffi::c_int
        >> 11 as core::ffi::c_int;
    let mut t5a: core::ffi::c_int = 1583 as core::ffi::c_int * in3
        - 1299 as core::ffi::c_int * in4 + 1024 as core::ffi::c_int
        >> 11 as core::ffi::c_int;
    let mut t6a: core::ffi::c_int = (1189 as core::ffi::c_int * in1
        + (3920 as core::ffi::c_int - 4096 as core::ffi::c_int) * in6
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in6;
    let mut t7a: core::ffi::c_int = ((3920 as core::ffi::c_int
        - 4096 as core::ffi::c_int) * in1 - 1189 as core::ffi::c_int * in6
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in1;
    let t0: core::ffi::c_int = iclip(t0a + t4a, min, max) as core::ffi::c_int;
    let t1: core::ffi::c_int = iclip(t1a + t5a, min, max) as core::ffi::c_int;
    let mut t2: core::ffi::c_int = iclip(t2a + t6a, min, max);
    let mut t3: core::ffi::c_int = iclip(t3a + t7a, min, max);
    let t4: core::ffi::c_int = iclip(t0a - t4a, min, max) as core::ffi::c_int;
    let t5: core::ffi::c_int = iclip(t1a - t5a, min, max) as core::ffi::c_int;
    let mut t6: core::ffi::c_int = iclip(t2a - t6a, min, max);
    let mut t7: core::ffi::c_int = iclip(t3a - t7a, min, max);
    t4a = ((3784 as core::ffi::c_int - 4096 as core::ffi::c_int) * t4
        + 1567 as core::ffi::c_int * t5 + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t4;
    t5a = (1567 as core::ffi::c_int * t4
        - (3784 as core::ffi::c_int - 4096 as core::ffi::c_int) * t5
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t5;
    t6a = ((3784 as core::ffi::c_int - 4096 as core::ffi::c_int) * t7
        - 1567 as core::ffi::c_int * t6 + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t7;
    t7a = (1567 as core::ffi::c_int * t7
        + (3784 as core::ffi::c_int - 4096 as core::ffi::c_int) * t6
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t6;
    *out.offset((0 as ptrdiff_t * out_s) as isize) = iclip(t0 + t2, min, max) as int32_t;
    *out.offset((7 as ptrdiff_t * out_s) as isize) = -iclip(t1 + t3, min, max)
        as int32_t;
    t2 = iclip(t0 - t2, min, max);
    t3 = iclip(t1 - t3, min, max);
    *out.offset((1 as ptrdiff_t * out_s) as isize) = -iclip(t4a + t6a, min, max)
        as int32_t;
    *out.offset((6 as ptrdiff_t * out_s) as isize) = iclip(t5a + t7a, min, max)
        as int32_t;
    t6 = iclip(t4a - t6a, min, max);
    t7 = iclip(t5a - t7a, min, max);
    *out.offset((3 as ptrdiff_t * out_s) as isize) = -((t2 + t3)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((4 as ptrdiff_t * out_s) as isize) = ((t2 - t3) * 181 as core::ffi::c_int
        + 128 as core::ffi::c_int >> 8 as core::ffi::c_int) as int32_t;
    *out.offset((2 as ptrdiff_t * out_s) as isize) = ((t6 + t7) * 181 as core::ffi::c_int
        + 128 as core::ffi::c_int >> 8 as core::ffi::c_int) as int32_t;
    *out.offset((5 as ptrdiff_t * out_s) as isize) = -((t6 - t7)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
}
#[inline(never)]
unsafe extern "C" fn inv_adst16_1d_internal_c(
    in_0: *const int32_t,
    in_s: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
    out: *mut int32_t,
    out_s: ptrdiff_t,
) {
    if in_s > 0 as ptrdiff_t && out_s != 0 as ptrdiff_t {} else {
        __assert_fail(
            b"in_s > 0 && out_s != 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            858 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 124],
                [core::ffi::c_char; 124],
            >(
                *b"void inv_adst16_1d_internal_c(const int32_t *const, const ptrdiff_t, const int, const int, int32_t *const, const ptrdiff_t)\0",
            ))
                .as_ptr(),
        );
    };
    let in0: core::ffi::c_int = *in_0.offset((0 as ptrdiff_t * in_s) as isize);
    let in1: core::ffi::c_int = *in_0.offset((1 as ptrdiff_t * in_s) as isize);
    let in2: core::ffi::c_int = *in_0.offset((2 as ptrdiff_t * in_s) as isize);
    let in3: core::ffi::c_int = *in_0.offset((3 as ptrdiff_t * in_s) as isize);
    let in4: core::ffi::c_int = *in_0.offset((4 as ptrdiff_t * in_s) as isize);
    let in5: core::ffi::c_int = *in_0.offset((5 as ptrdiff_t * in_s) as isize);
    let in6: core::ffi::c_int = *in_0.offset((6 as ptrdiff_t * in_s) as isize);
    let in7: core::ffi::c_int = *in_0.offset((7 as ptrdiff_t * in_s) as isize);
    let in8: core::ffi::c_int = *in_0.offset((8 as ptrdiff_t * in_s) as isize);
    let in9: core::ffi::c_int = *in_0.offset((9 as ptrdiff_t * in_s) as isize);
    let in10: core::ffi::c_int = *in_0.offset((10 as ptrdiff_t * in_s) as isize);
    let in11: core::ffi::c_int = *in_0.offset((11 as ptrdiff_t * in_s) as isize);
    let in12: core::ffi::c_int = *in_0.offset((12 as ptrdiff_t * in_s) as isize);
    let in13: core::ffi::c_int = *in_0.offset((13 as ptrdiff_t * in_s) as isize);
    let in14: core::ffi::c_int = *in_0.offset((14 as ptrdiff_t * in_s) as isize);
    let in15: core::ffi::c_int = *in_0.offset((15 as ptrdiff_t * in_s) as isize);
    let mut t0: core::ffi::c_int = (in15
        * (4091 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + in0 * 201 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in15;
    let mut t1: core::ffi::c_int = (in15 * 201 as core::ffi::c_int
        - in0 * (4091 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in0;
    let mut t2: core::ffi::c_int = (in13
        * (3973 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + in2 * 995 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in13;
    let mut t3: core::ffi::c_int = (in13 * 995 as core::ffi::c_int
        - in2 * (3973 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in2;
    let mut t4: core::ffi::c_int = (in11
        * (3703 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + in4 * 1751 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in11;
    let mut t5: core::ffi::c_int = (in11 * 1751 as core::ffi::c_int
        - in4 * (3703 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - in4;
    let mut t6: core::ffi::c_int = in9 * 1645 as core::ffi::c_int
        + in6 * 1220 as core::ffi::c_int + 1024 as core::ffi::c_int
        >> 11 as core::ffi::c_int;
    let mut t7: core::ffi::c_int = in9 * 1220 as core::ffi::c_int
        - in6 * 1645 as core::ffi::c_int + 1024 as core::ffi::c_int
        >> 11 as core::ffi::c_int;
    let mut t8: core::ffi::c_int = (in7 * 2751 as core::ffi::c_int
        + in8 * (3035 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in8;
    let mut t9: core::ffi::c_int = (in7
        * (3035 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - in8 * 2751 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in7;
    let mut t10: core::ffi::c_int = (in5 * 2106 as core::ffi::c_int
        + in10 * (3513 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in10;
    let mut t11: core::ffi::c_int = (in5
        * (3513 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - in10 * 2106 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in5;
    let mut t12: core::ffi::c_int = (in3 * 1380 as core::ffi::c_int
        + in12 * (3857 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in12;
    let mut t13: core::ffi::c_int = (in3
        * (3857 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - in12 * 1380 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in3;
    let mut t14: core::ffi::c_int = (in1 * 601 as core::ffi::c_int
        + in14 * (4052 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + in14;
    let mut t15: core::ffi::c_int = (in1
        * (4052 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - in14 * 601 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + in1;
    let mut t0a: core::ffi::c_int = iclip(t0 + t8, min, max);
    let mut t1a: core::ffi::c_int = iclip(t1 + t9, min, max);
    let mut t2a: core::ffi::c_int = iclip(t2 + t10, min, max);
    let mut t3a: core::ffi::c_int = iclip(t3 + t11, min, max);
    let mut t4a: core::ffi::c_int = iclip(t4 + t12, min, max);
    let mut t5a: core::ffi::c_int = iclip(t5 + t13, min, max);
    let mut t6a: core::ffi::c_int = iclip(t6 + t14, min, max);
    let mut t7a: core::ffi::c_int = iclip(t7 + t15, min, max);
    let mut t8a: core::ffi::c_int = iclip(t0 - t8, min, max);
    let mut t9a: core::ffi::c_int = iclip(t1 - t9, min, max);
    let mut t10a: core::ffi::c_int = iclip(t2 - t10, min, max);
    let mut t11a: core::ffi::c_int = iclip(t3 - t11, min, max);
    let mut t12a: core::ffi::c_int = iclip(t4 - t12, min, max);
    let mut t13a: core::ffi::c_int = iclip(t5 - t13, min, max);
    let mut t14a: core::ffi::c_int = iclip(t6 - t14, min, max);
    let mut t15a: core::ffi::c_int = iclip(t7 - t15, min, max);
    t8 = (t8a * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t9a * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t8a;
    t9 = (t8a * 799 as core::ffi::c_int
        - t9a * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t9a;
    t10 = (t10a * 2276 as core::ffi::c_int
        + t11a * (3406 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t11a;
    t11 = (t10a * (3406 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - t11a * 2276 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t10a;
    t12 = (t13a * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - t12a * 799 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t13a;
    t13 = (t13a * 799 as core::ffi::c_int
        + t12a * (4017 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t12a;
    t14 = (t15a * 2276 as core::ffi::c_int
        - t14a * (3406 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t14a;
    t15 = (t15a * (3406 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t14a * 2276 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t15a;
    t0 = iclip(t0a + t4a, min, max);
    t1 = iclip(t1a + t5a, min, max);
    t2 = iclip(t2a + t6a, min, max);
    t3 = iclip(t3a + t7a, min, max);
    t4 = iclip(t0a - t4a, min, max);
    t5 = iclip(t1a - t5a, min, max);
    t6 = iclip(t2a - t6a, min, max);
    t7 = iclip(t3a - t7a, min, max);
    t8a = iclip(t8 + t12, min, max);
    t9a = iclip(t9 + t13, min, max);
    t10a = iclip(t10 + t14, min, max);
    t11a = iclip(t11 + t15, min, max);
    t12a = iclip(t8 - t12, min, max);
    t13a = iclip(t9 - t13, min, max);
    t14a = iclip(t10 - t14, min, max);
    t15a = iclip(t11 - t15, min, max);
    t4a = (t4 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t5 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t4;
    t5a = (t4 * 1567 as core::ffi::c_int
        - t5 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t5;
    t6a = (t7 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - t6 * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t7;
    t7a = (t7 * 1567 as core::ffi::c_int
        + t6 * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t6;
    t12 = (t12a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + t13a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t12a;
    t13 = (t12a * 1567 as core::ffi::c_int
        - t13a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) - t13a;
    t14 = (t15a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        - t14a * 1567 as core::ffi::c_int + 2048 as core::ffi::c_int
        >> 12 as core::ffi::c_int) + t15a;
    t15 = (t15a * 1567 as core::ffi::c_int
        + t14a * (3784 as core::ffi::c_int - 4096 as core::ffi::c_int)
        + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int) + t14a;
    *out.offset((0 as ptrdiff_t * out_s) as isize) = iclip(t0 + t2, min, max) as int32_t;
    *out.offset((15 as ptrdiff_t * out_s) as isize) = -iclip(t1 + t3, min, max)
        as int32_t;
    t2a = iclip(t0 - t2, min, max);
    t3a = iclip(t1 - t3, min, max);
    *out.offset((3 as ptrdiff_t * out_s) as isize) = -iclip(t4a + t6a, min, max)
        as int32_t;
    *out.offset((12 as ptrdiff_t * out_s) as isize) = iclip(t5a + t7a, min, max)
        as int32_t;
    t6 = iclip(t4a - t6a, min, max);
    t7 = iclip(t5a - t7a, min, max);
    *out.offset((1 as ptrdiff_t * out_s) as isize) = -iclip(t8a + t10a, min, max)
        as int32_t;
    *out.offset((14 as ptrdiff_t * out_s) as isize) = iclip(t9a + t11a, min, max)
        as int32_t;
    t10 = iclip(t8a - t10a, min, max);
    t11 = iclip(t9a - t11a, min, max);
    *out.offset((2 as ptrdiff_t * out_s) as isize) = iclip(t12 + t14, min, max)
        as int32_t;
    *out.offset((13 as ptrdiff_t * out_s) as isize) = -iclip(t13 + t15, min, max)
        as int32_t;
    t14a = iclip(t12 - t14, min, max);
    t15a = iclip(t13 - t15, min, max);
    *out.offset((7 as ptrdiff_t * out_s) as isize) = -((t2a + t3a)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((8 as ptrdiff_t * out_s) as isize) = ((t2a - t3a)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((4 as ptrdiff_t * out_s) as isize) = ((t6 + t7) * 181 as core::ffi::c_int
        + 128 as core::ffi::c_int >> 8 as core::ffi::c_int) as int32_t;
    *out.offset((11 as ptrdiff_t * out_s) as isize) = -((t6 - t7)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((6 as ptrdiff_t * out_s) as isize) = ((t10 + t11)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((9 as ptrdiff_t * out_s) as isize) = -((t10 - t11)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((5 as ptrdiff_t * out_s) as isize) = -((t14a + t15a)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
    *out.offset((10 as ptrdiff_t * out_s) as isize) = ((t14a - t15a)
        * 181 as core::ffi::c_int + 128 as core::ffi::c_int >> 8 as core::ffi::c_int)
        as int32_t;
}
unsafe extern "C" fn inv_flipadst4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_adst4_1d_internal_c(
        c,
        stride,
        min,
        max,
        &raw mut *c
            .offset(
                ((4 as core::ffi::c_int - 1 as core::ffi::c_int) as ptrdiff_t * stride)
                    as isize,
            ),
        -stride,
    );
}
unsafe extern "C" fn inv_adst4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_adst4_1d_internal_c(c, stride, min, max, c, stride);
}
unsafe extern "C" fn inv_adst8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_adst8_1d_internal_c(c, stride, min, max, c, stride);
}
unsafe extern "C" fn inv_flipadst8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_adst8_1d_internal_c(
        c,
        stride,
        min,
        max,
        &raw mut *c
            .offset(
                ((8 as core::ffi::c_int - 1 as core::ffi::c_int) as ptrdiff_t * stride)
                    as isize,
            ),
        -stride,
    );
}
unsafe extern "C" fn inv_adst16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_adst16_1d_internal_c(c, stride, min, max, c, stride);
}
unsafe extern "C" fn inv_flipadst16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    inv_adst16_1d_internal_c(
        c,
        stride,
        min,
        max,
        &raw mut *c
            .offset(
                ((16 as core::ffi::c_int - 1 as core::ffi::c_int) as ptrdiff_t * stride)
                    as isize,
            ),
        -stride,
    );
}
unsafe extern "C" fn inv_identity4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            986 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 79],
                [core::ffi::c_char; 79],
            >(
                *b"void inv_identity4_1d_c(int32_t *const, const ptrdiff_t, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        let in_0: core::ffi::c_int = *c.offset((stride * i as ptrdiff_t) as isize);
        *c.offset((stride * i as ptrdiff_t) as isize) = (in_0
            + (in_0 * 1697 as core::ffi::c_int + 2048 as core::ffi::c_int
                >> 12 as core::ffi::c_int)) as int32_t;
        i += 1;
    }
}
unsafe extern "C" fn inv_identity8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            996 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 79],
                [core::ffi::c_char; 79],
            >(
                *b"void inv_identity8_1d_c(int32_t *const, const ptrdiff_t, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        let fresh1 = &raw mut *c.offset((stride * i as ptrdiff_t) as isize);
        *fresh1 = (*fresh1 as core::ffi::c_int * 2 as core::ffi::c_int) as int32_t;
        i += 1;
    }
}
unsafe extern "C" fn inv_identity16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            1004 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 80],
                [core::ffi::c_char; 80],
            >(
                *b"void inv_identity16_1d_c(int32_t *const, const ptrdiff_t, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 16 as core::ffi::c_int {
        let in_0: core::ffi::c_int = *c.offset((stride * i as ptrdiff_t) as isize);
        *c.offset((stride * i as ptrdiff_t) as isize) = (2 as core::ffi::c_int * in_0
            + (in_0 * 1697 as core::ffi::c_int + 1024 as core::ffi::c_int
                >> 11 as core::ffi::c_int)) as int32_t;
        i += 1;
    }
}
unsafe extern "C" fn inv_identity32_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            1014 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 80],
                [core::ffi::c_char; 80],
            >(
                *b"void inv_identity32_1d_c(int32_t *const, const ptrdiff_t, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 32 as core::ffi::c_int {
        let fresh0 = &raw mut *c.offset((stride * i as ptrdiff_t) as isize);
        *fresh0 = (*fresh0 as core::ffi::c_int * 4 as core::ffi::c_int) as int32_t;
        i += 1;
    }
}
#[no_mangle]
pub static mut dav1d_tx1d_fns: [[itx_1d_fn; 4]; 5] = unsafe {
    [
        [
            Some(
                inv_dct4_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_adst4_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_identity4_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_flipadst4_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
        ],
        [
            Some(
                inv_dct8_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_adst8_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_identity8_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_flipadst8_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
        ],
        [
            Some(
                inv_dct16_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_adst16_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_identity16_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            Some(
                inv_flipadst16_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
        ],
        [
            Some(
                inv_dct32_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            Some(
                inv_identity32_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
        ],
        [
            Some(
                inv_dct64_1d_c
                    as unsafe extern "C" fn(
                        *mut int32_t,
                        ptrdiff_t,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> (),
            ),
            None,
            None,
            None,
        ],
    ]
};
#[no_mangle]
pub static mut dav1d_tx1d_types: [[uint8_t; 2]; 16] = [
    [DCT as core::ffi::c_int as uint8_t, DCT as core::ffi::c_int as uint8_t],
    [ADST as core::ffi::c_int as uint8_t, DCT as core::ffi::c_int as uint8_t],
    [DCT as core::ffi::c_int as uint8_t, ADST as core::ffi::c_int as uint8_t],
    [ADST as core::ffi::c_int as uint8_t, ADST as core::ffi::c_int as uint8_t],
    [FLIPADST as core::ffi::c_int as uint8_t, DCT as core::ffi::c_int as uint8_t],
    [DCT as core::ffi::c_int as uint8_t, FLIPADST as core::ffi::c_int as uint8_t],
    [FLIPADST as core::ffi::c_int as uint8_t, FLIPADST as core::ffi::c_int as uint8_t],
    [ADST as core::ffi::c_int as uint8_t, FLIPADST as core::ffi::c_int as uint8_t],
    [FLIPADST as core::ffi::c_int as uint8_t, ADST as core::ffi::c_int as uint8_t],
    [IDENTITY as core::ffi::c_int as uint8_t, IDENTITY as core::ffi::c_int as uint8_t],
    [DCT as core::ffi::c_int as uint8_t, IDENTITY as core::ffi::c_int as uint8_t],
    [IDENTITY as core::ffi::c_int as uint8_t, DCT as core::ffi::c_int as uint8_t],
    [ADST as core::ffi::c_int as uint8_t, IDENTITY as core::ffi::c_int as uint8_t],
    [IDENTITY as core::ffi::c_int as uint8_t, ADST as core::ffi::c_int as uint8_t],
    [FLIPADST as core::ffi::c_int as uint8_t, IDENTITY as core::ffi::c_int as uint8_t],
    [IDENTITY as core::ffi::c_int as uint8_t, FLIPADST as core::ffi::c_int as uint8_t],
];
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_wht4_1d_c(c: *mut int32_t, stride: ptrdiff_t) {
    if stride > 0 as ptrdiff_t {} else {
        __assert_fail(
            b"stride > 0\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/itx_1d.c\0" as *const u8
                as *const core::ffi::c_char,
            1067 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 58],
                [core::ffi::c_char; 58],
            >(*b"void dav1d_inv_wht4_1d_c(int32_t *const, const ptrdiff_t)\0"))
                .as_ptr(),
        );
    };
    let in0: core::ffi::c_int = *c.offset((0 as ptrdiff_t * stride) as isize);
    let in1: core::ffi::c_int = *c.offset((1 as ptrdiff_t * stride) as isize);
    let in2: core::ffi::c_int = *c.offset((2 as ptrdiff_t * stride) as isize);
    let in3: core::ffi::c_int = *c.offset((3 as ptrdiff_t * stride) as isize);
    let t0: core::ffi::c_int = in0 + in1;
    let t2: core::ffi::c_int = in2 - in3;
    let t4: core::ffi::c_int = t0 - t2 >> 1 as core::ffi::c_int;
    let t3: core::ffi::c_int = t4 - in3;
    let t1: core::ffi::c_int = t4 - in1;
    *c.offset((0 as ptrdiff_t * stride) as isize) = (t0 - t3) as int32_t;
    *c.offset((1 as ptrdiff_t * stride) as isize) = t3 as int32_t;
    *c.offset((2 as ptrdiff_t * stride) as isize) = t1 as int32_t;
    *c.offset((3 as ptrdiff_t * stride) as isize) = (t2 + t1) as int32_t;
}
