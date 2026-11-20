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

// Mechanically translated from upstream/aom/aom_dsp/fft.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type aom_fft_1d_func_t = Option<
    unsafe extern "C-unwind" fn(
        *const core::ffi::c_float,
        *mut core::ffi::c_float,
        core::ffi::c_int,
    ) -> (),
>;
pub type aom_fft_transpose_func_t = Option<
    unsafe extern "C-unwind" fn(
        *const core::ffi::c_float,
        *mut core::ffi::c_float,
        core::ffi::c_int,
    ) -> (),
>;
pub type aom_fft_unpack_func_t = Option<
    unsafe extern "C-unwind" fn(
        *const core::ffi::c_float,
        *mut core::ffi::c_float,
        core::ffi::c_int,
    ) -> (),
>;
#[inline]
unsafe extern "C-unwind" fn simple_transpose(
    mut A: *const core::ffi::c_float,
    mut B: *mut core::ffi::c_float,
    mut n: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < n {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < n {
            *B.offset((y * n + x) as isize) = *A.offset((x * n + y) as isize);
            x += 1;
        }
        y += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn unpack_2d_output(
    mut col_fft: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut n: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y <= n / 2 as core::ffi::c_int {
        let y2: core::ffi::c_int = y + n / 2 as core::ffi::c_int;
        let y_extra: core::ffi::c_int = (y2 > n / 2 as core::ffi::c_int && y2 < n)
            as core::ffi::c_int;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x <= n / 2 as core::ffi::c_int {
            let x2: core::ffi::c_int = x + n / 2 as core::ffi::c_int;
            let x_extra: core::ffi::c_int = (x2 > n / 2 as core::ffi::c_int && x2 < n)
                as core::ffi::c_int;
            *output.offset((2 as core::ffi::c_int * (y * n + x)) as isize) = *col_fft
                .offset((y * n + x) as isize)
                - (if x_extra != 0 && y_extra != 0 {
                    *col_fft.offset((y2 * n + x2) as isize)
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                });
            *output
                .offset(
                    (2 as core::ffi::c_int * (y * n + x) + 1 as core::ffi::c_int)
                        as isize,
                ) = (if y_extra != 0 {
                *col_fft.offset((y2 * n + x) as isize)
            } else {
                0 as core::ffi::c_int as core::ffi::c_float
            })
                + (if x_extra != 0 {
                    *col_fft.offset((y * n + x2) as isize)
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                });
            if y_extra != 0 {
                *output.offset((2 as core::ffi::c_int * ((n - y) * n + x)) as isize) = *col_fft
                    .offset((y * n + x) as isize)
                    + (if x_extra != 0 && y_extra != 0 {
                        *col_fft.offset((y2 * n + x2) as isize)
                    } else {
                        0 as core::ffi::c_int as core::ffi::c_float
                    });
                *output
                    .offset(
                        (2 as core::ffi::c_int * ((n - y) * n + x)
                            + 1 as core::ffi::c_int) as isize,
                    ) = -(if y_extra != 0 {
                    *col_fft.offset((y2 * n + x) as isize)
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                })
                    + (if x_extra != 0 {
                        *col_fft.offset((y * n + x2) as isize)
                    } else {
                        0 as core::ffi::c_int as core::ffi::c_float
                    });
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft_2d_gen(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut n: core::ffi::c_int,
    mut tform: aom_fft_1d_func_t,
    mut transpose: aom_fft_transpose_func_t,
    mut unpack: aom_fft_unpack_func_t,
    mut vec_size: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < n {
        tform
            .expect(
                "non-null function pointer",
            )(input.offset(x as isize), output.offset(x as isize), n);
        x += vec_size;
    }
    transpose.expect("non-null function pointer")(output, temp, n);
    let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while x_0 < n {
        tform
            .expect(
                "non-null function pointer",
            )(temp.offset(x_0 as isize), output.offset(x_0 as isize), n);
        x_0 += vec_size;
    }
    transpose.expect("non-null function pointer")(output, temp, n);
    unpack.expect("non-null function pointer")(temp, output, n);
}
#[inline]
unsafe extern "C-unwind" fn store_float(
    mut output: *mut core::ffi::c_float,
    mut input: core::ffi::c_float,
) {
    *output = input;
}
#[inline]
unsafe extern "C-unwind" fn add_float(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    return a + b;
}
#[inline]
unsafe extern "C-unwind" fn sub_float(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    return a - b;
}
#[inline]
unsafe extern "C-unwind" fn mul_float(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    return a * b;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft1d_2_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    store_float(output.offset((0 as core::ffi::c_int * stride) as isize), i0 + i1);
    store_float(output.offset((1 as core::ffi::c_int * stride) as isize), i0 - i1);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft1d_4_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let w0: core::ffi::c_float = add_float(i0, i2) as core::ffi::c_float;
    let w1: core::ffi::c_float = sub_float(i0, i2) as core::ffi::c_float;
    let w2: core::ffi::c_float = add_float(i1, i3) as core::ffi::c_float;
    let w3: core::ffi::c_float = sub_float(i1, i3) as core::ffi::c_float;
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(w0, w2),
    );
    store_float(output.offset((1 as core::ffi::c_int * stride) as isize), w1);
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        sub_float(w0, w2),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        sub_float(kWeight0, w3),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft1d_8_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let kWeight2: core::ffi::c_float = 0.707107f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let i4: core::ffi::c_float = *input
        .offset((4 as core::ffi::c_int * stride) as isize);
    let i5: core::ffi::c_float = *input
        .offset((5 as core::ffi::c_int * stride) as isize);
    let i6: core::ffi::c_float = *input
        .offset((6 as core::ffi::c_int * stride) as isize);
    let i7: core::ffi::c_float = *input
        .offset((7 as core::ffi::c_int * stride) as isize);
    let w0: core::ffi::c_float = add_float(i0, i4) as core::ffi::c_float;
    let w1: core::ffi::c_float = sub_float(i0, i4) as core::ffi::c_float;
    let w2: core::ffi::c_float = add_float(i2, i6) as core::ffi::c_float;
    let w3: core::ffi::c_float = sub_float(i2, i6) as core::ffi::c_float;
    let w4: core::ffi::c_float = add_float(w0, w2) as core::ffi::c_float;
    let w5: core::ffi::c_float = sub_float(w0, w2) as core::ffi::c_float;
    let w7: core::ffi::c_float = add_float(i1, i5) as core::ffi::c_float;
    let w8: core::ffi::c_float = sub_float(i1, i5) as core::ffi::c_float;
    let w9: core::ffi::c_float = add_float(i3, i7) as core::ffi::c_float;
    let w10: core::ffi::c_float = sub_float(i3, i7) as core::ffi::c_float;
    let w11: core::ffi::c_float = add_float(w7, w9) as core::ffi::c_float;
    let w12: core::ffi::c_float = sub_float(w7, w9) as core::ffi::c_float;
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(w4, w11),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(w1, mul_float(kWeight2, sub_float(w8, w10))),
    );
    store_float(output.offset((2 as core::ffi::c_int * stride) as isize), w5);
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        sub_float(w1, mul_float(kWeight2, sub_float(w8, w10))),
    );
    store_float(
        output.offset((4 as core::ffi::c_int * stride) as isize),
        sub_float(w4, w11),
    );
    store_float(
        output.offset((5 as core::ffi::c_int * stride) as isize),
        sub_float(sub_float(kWeight0, w3), mul_float(kWeight2, add_float(w10, w8))),
    );
    store_float(
        output.offset((6 as core::ffi::c_int * stride) as isize),
        sub_float(kWeight0, w12),
    );
    store_float(
        output.offset((7 as core::ffi::c_int * stride) as isize),
        sub_float(w3, mul_float(kWeight2, add_float(w10, w8))),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft1d_16_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let kWeight2: core::ffi::c_float = 0.707107f32;
    let kWeight3: core::ffi::c_float = 0.92388f32;
    let kWeight4: core::ffi::c_float = 0.382683f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let i4: core::ffi::c_float = *input
        .offset((4 as core::ffi::c_int * stride) as isize);
    let i5: core::ffi::c_float = *input
        .offset((5 as core::ffi::c_int * stride) as isize);
    let i6: core::ffi::c_float = *input
        .offset((6 as core::ffi::c_int * stride) as isize);
    let i7: core::ffi::c_float = *input
        .offset((7 as core::ffi::c_int * stride) as isize);
    let i8: core::ffi::c_float = *input
        .offset((8 as core::ffi::c_int * stride) as isize);
    let i9: core::ffi::c_float = *input
        .offset((9 as core::ffi::c_int * stride) as isize);
    let i10: core::ffi::c_float = *input
        .offset((10 as core::ffi::c_int * stride) as isize);
    let i11: core::ffi::c_float = *input
        .offset((11 as core::ffi::c_int * stride) as isize);
    let i12: core::ffi::c_float = *input
        .offset((12 as core::ffi::c_int * stride) as isize);
    let i13: core::ffi::c_float = *input
        .offset((13 as core::ffi::c_int * stride) as isize);
    let i14: core::ffi::c_float = *input
        .offset((14 as core::ffi::c_int * stride) as isize);
    let i15: core::ffi::c_float = *input
        .offset((15 as core::ffi::c_int * stride) as isize);
    let w0: core::ffi::c_float = add_float(i0, i8) as core::ffi::c_float;
    let w1: core::ffi::c_float = sub_float(i0, i8) as core::ffi::c_float;
    let w2: core::ffi::c_float = add_float(i4, i12) as core::ffi::c_float;
    let w3: core::ffi::c_float = sub_float(i4, i12) as core::ffi::c_float;
    let w4: core::ffi::c_float = add_float(w0, w2) as core::ffi::c_float;
    let w5: core::ffi::c_float = sub_float(w0, w2) as core::ffi::c_float;
    let w7: core::ffi::c_float = add_float(i2, i10) as core::ffi::c_float;
    let w8: core::ffi::c_float = sub_float(i2, i10) as core::ffi::c_float;
    let w9: core::ffi::c_float = add_float(i6, i14) as core::ffi::c_float;
    let w10: core::ffi::c_float = sub_float(i6, i14) as core::ffi::c_float;
    let w11: core::ffi::c_float = add_float(w7, w9) as core::ffi::c_float;
    let w12: core::ffi::c_float = sub_float(w7, w9) as core::ffi::c_float;
    let w14: core::ffi::c_float = add_float(w4, w11) as core::ffi::c_float;
    let w15: core::ffi::c_float = sub_float(w4, w11) as core::ffi::c_float;
    let w16: [core::ffi::c_float; 2] = [
        add_float(w1, mul_float(kWeight2, sub_float(w8, w10))),
        sub_float(sub_float(kWeight0, w3), mul_float(kWeight2, add_float(w10, w8))),
    ];
    let w18: [core::ffi::c_float; 2] = [
        sub_float(w1, mul_float(kWeight2, sub_float(w8, w10))),
        sub_float(w3, mul_float(kWeight2, add_float(w10, w8))),
    ];
    let w19: core::ffi::c_float = add_float(i1, i9) as core::ffi::c_float;
    let w20: core::ffi::c_float = sub_float(i1, i9) as core::ffi::c_float;
    let w21: core::ffi::c_float = add_float(i5, i13) as core::ffi::c_float;
    let w22: core::ffi::c_float = sub_float(i5, i13) as core::ffi::c_float;
    let w23: core::ffi::c_float = add_float(w19, w21) as core::ffi::c_float;
    let w24: core::ffi::c_float = sub_float(w19, w21) as core::ffi::c_float;
    let w26: core::ffi::c_float = add_float(i3, i11) as core::ffi::c_float;
    let w27: core::ffi::c_float = sub_float(i3, i11) as core::ffi::c_float;
    let w28: core::ffi::c_float = add_float(i7, i15) as core::ffi::c_float;
    let w29: core::ffi::c_float = sub_float(i7, i15) as core::ffi::c_float;
    let w30: core::ffi::c_float = add_float(w26, w28) as core::ffi::c_float;
    let w31: core::ffi::c_float = sub_float(w26, w28) as core::ffi::c_float;
    let w33: core::ffi::c_float = add_float(w23, w30) as core::ffi::c_float;
    let w34: core::ffi::c_float = sub_float(w23, w30) as core::ffi::c_float;
    let w35: [core::ffi::c_float; 2] = [
        add_float(w20, mul_float(kWeight2, sub_float(w27, w29))),
        sub_float(sub_float(kWeight0, w22), mul_float(kWeight2, add_float(w29, w27))),
    ];
    let w37: [core::ffi::c_float; 2] = [
        sub_float(w20, mul_float(kWeight2, sub_float(w27, w29))),
        sub_float(w22, mul_float(kWeight2, add_float(w29, w27))),
    ];
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(w14, w33),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(
            w16[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w35[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w35[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        add_float(w5, mul_float(kWeight2, sub_float(w24, w31))),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        add_float(
            w18[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w37[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w37[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(output.offset((4 as core::ffi::c_int * stride) as isize), w15);
    store_float(
        output.offset((5 as core::ffi::c_int * stride) as isize),
        add_float(
            w18[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w37[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w37[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((6 as core::ffi::c_int * stride) as isize),
        sub_float(w5, mul_float(kWeight2, sub_float(w24, w31))),
    );
    store_float(
        output.offset((7 as core::ffi::c_int * stride) as isize),
        add_float(
            w16[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w35[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w35[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((8 as core::ffi::c_int * stride) as isize),
        sub_float(w14, w33),
    );
    store_float(
        output.offset((9 as core::ffi::c_int * stride) as isize),
        add_float(
            w16[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w35[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w35[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((10 as core::ffi::c_int * stride) as isize),
        sub_float(sub_float(kWeight0, w12), mul_float(kWeight2, add_float(w31, w24))),
    );
    store_float(
        output.offset((11 as core::ffi::c_int * stride) as isize),
        add_float(
            w18[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w37[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w37[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((12 as core::ffi::c_int * stride) as isize),
        sub_float(kWeight0, w34),
    );
    store_float(
        output.offset((13 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w18[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight3, w37[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w37[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((14 as core::ffi::c_int * stride) as isize),
        sub_float(w12, mul_float(kWeight2, add_float(w31, w24))),
    );
    store_float(
        output.offset((15 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w16[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight4, w35[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w35[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft1d_32_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let kWeight2: core::ffi::c_float = 0.707107f32;
    let kWeight3: core::ffi::c_float = 0.92388f32;
    let kWeight4: core::ffi::c_float = 0.382683f32;
    let kWeight5: core::ffi::c_float = 0.980785f32;
    let kWeight6: core::ffi::c_float = 0.19509f32;
    let kWeight7: core::ffi::c_float = 0.83147f32;
    let kWeight8: core::ffi::c_float = 0.55557f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let i4: core::ffi::c_float = *input
        .offset((4 as core::ffi::c_int * stride) as isize);
    let i5: core::ffi::c_float = *input
        .offset((5 as core::ffi::c_int * stride) as isize);
    let i6: core::ffi::c_float = *input
        .offset((6 as core::ffi::c_int * stride) as isize);
    let i7: core::ffi::c_float = *input
        .offset((7 as core::ffi::c_int * stride) as isize);
    let i8: core::ffi::c_float = *input
        .offset((8 as core::ffi::c_int * stride) as isize);
    let i9: core::ffi::c_float = *input
        .offset((9 as core::ffi::c_int * stride) as isize);
    let i10: core::ffi::c_float = *input
        .offset((10 as core::ffi::c_int * stride) as isize);
    let i11: core::ffi::c_float = *input
        .offset((11 as core::ffi::c_int * stride) as isize);
    let i12: core::ffi::c_float = *input
        .offset((12 as core::ffi::c_int * stride) as isize);
    let i13: core::ffi::c_float = *input
        .offset((13 as core::ffi::c_int * stride) as isize);
    let i14: core::ffi::c_float = *input
        .offset((14 as core::ffi::c_int * stride) as isize);
    let i15: core::ffi::c_float = *input
        .offset((15 as core::ffi::c_int * stride) as isize);
    let i16: core::ffi::c_float = *input
        .offset((16 as core::ffi::c_int * stride) as isize);
    let i17: core::ffi::c_float = *input
        .offset((17 as core::ffi::c_int * stride) as isize);
    let i18: core::ffi::c_float = *input
        .offset((18 as core::ffi::c_int * stride) as isize);
    let i19: core::ffi::c_float = *input
        .offset((19 as core::ffi::c_int * stride) as isize);
    let i20: core::ffi::c_float = *input
        .offset((20 as core::ffi::c_int * stride) as isize);
    let i21: core::ffi::c_float = *input
        .offset((21 as core::ffi::c_int * stride) as isize);
    let i22: core::ffi::c_float = *input
        .offset((22 as core::ffi::c_int * stride) as isize);
    let i23: core::ffi::c_float = *input
        .offset((23 as core::ffi::c_int * stride) as isize);
    let i24: core::ffi::c_float = *input
        .offset((24 as core::ffi::c_int * stride) as isize);
    let i25: core::ffi::c_float = *input
        .offset((25 as core::ffi::c_int * stride) as isize);
    let i26: core::ffi::c_float = *input
        .offset((26 as core::ffi::c_int * stride) as isize);
    let i27: core::ffi::c_float = *input
        .offset((27 as core::ffi::c_int * stride) as isize);
    let i28: core::ffi::c_float = *input
        .offset((28 as core::ffi::c_int * stride) as isize);
    let i29: core::ffi::c_float = *input
        .offset((29 as core::ffi::c_int * stride) as isize);
    let i30: core::ffi::c_float = *input
        .offset((30 as core::ffi::c_int * stride) as isize);
    let i31: core::ffi::c_float = *input
        .offset((31 as core::ffi::c_int * stride) as isize);
    let w0: core::ffi::c_float = add_float(i0, i16) as core::ffi::c_float;
    let w1: core::ffi::c_float = sub_float(i0, i16) as core::ffi::c_float;
    let w2: core::ffi::c_float = add_float(i8, i24) as core::ffi::c_float;
    let w3: core::ffi::c_float = sub_float(i8, i24) as core::ffi::c_float;
    let w4: core::ffi::c_float = add_float(w0, w2) as core::ffi::c_float;
    let w5: core::ffi::c_float = sub_float(w0, w2) as core::ffi::c_float;
    let w7: core::ffi::c_float = add_float(i4, i20) as core::ffi::c_float;
    let w8: core::ffi::c_float = sub_float(i4, i20) as core::ffi::c_float;
    let w9: core::ffi::c_float = add_float(i12, i28) as core::ffi::c_float;
    let w10: core::ffi::c_float = sub_float(i12, i28) as core::ffi::c_float;
    let w11: core::ffi::c_float = add_float(w7, w9) as core::ffi::c_float;
    let w12: core::ffi::c_float = sub_float(w7, w9) as core::ffi::c_float;
    let w14: core::ffi::c_float = add_float(w4, w11) as core::ffi::c_float;
    let w15: core::ffi::c_float = sub_float(w4, w11) as core::ffi::c_float;
    let w16: [core::ffi::c_float; 2] = [
        add_float(w1, mul_float(kWeight2, sub_float(w8, w10))),
        sub_float(sub_float(kWeight0, w3), mul_float(kWeight2, add_float(w10, w8))),
    ];
    let w18: [core::ffi::c_float; 2] = [
        sub_float(w1, mul_float(kWeight2, sub_float(w8, w10))),
        sub_float(w3, mul_float(kWeight2, add_float(w10, w8))),
    ];
    let w19: core::ffi::c_float = add_float(i2, i18) as core::ffi::c_float;
    let w20: core::ffi::c_float = sub_float(i2, i18) as core::ffi::c_float;
    let w21: core::ffi::c_float = add_float(i10, i26) as core::ffi::c_float;
    let w22: core::ffi::c_float = sub_float(i10, i26) as core::ffi::c_float;
    let w23: core::ffi::c_float = add_float(w19, w21) as core::ffi::c_float;
    let w24: core::ffi::c_float = sub_float(w19, w21) as core::ffi::c_float;
    let w26: core::ffi::c_float = add_float(i6, i22) as core::ffi::c_float;
    let w27: core::ffi::c_float = sub_float(i6, i22) as core::ffi::c_float;
    let w28: core::ffi::c_float = add_float(i14, i30) as core::ffi::c_float;
    let w29: core::ffi::c_float = sub_float(i14, i30) as core::ffi::c_float;
    let w30: core::ffi::c_float = add_float(w26, w28) as core::ffi::c_float;
    let w31: core::ffi::c_float = sub_float(w26, w28) as core::ffi::c_float;
    let w33: core::ffi::c_float = add_float(w23, w30) as core::ffi::c_float;
    let w34: core::ffi::c_float = sub_float(w23, w30) as core::ffi::c_float;
    let w35: [core::ffi::c_float; 2] = [
        add_float(w20, mul_float(kWeight2, sub_float(w27, w29))),
        sub_float(sub_float(kWeight0, w22), mul_float(kWeight2, add_float(w29, w27))),
    ];
    let w37: [core::ffi::c_float; 2] = [
        sub_float(w20, mul_float(kWeight2, sub_float(w27, w29))),
        sub_float(w22, mul_float(kWeight2, add_float(w29, w27))),
    ];
    let w38: core::ffi::c_float = add_float(w14, w33) as core::ffi::c_float;
    let w39: core::ffi::c_float = sub_float(w14, w33) as core::ffi::c_float;
    let w40: [core::ffi::c_float; 2] = [
        add_float(
            w16[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w35[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w35[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w16[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w35[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w35[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w41: [core::ffi::c_float; 2] = [
        add_float(w5, mul_float(kWeight2, sub_float(w24, w31))),
        sub_float(sub_float(kWeight0, w12), mul_float(kWeight2, add_float(w31, w24))),
    ];
    let w42: [core::ffi::c_float; 2] = [
        add_float(
            w18[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w37[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w37[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w18[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w37[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w37[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w44: [core::ffi::c_float; 2] = [
        add_float(
            w18[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w37[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w37[1 as core::ffi::c_int as usize]),
            ),
        ),
        sub_float(
            sub_float(kWeight0, w18[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight3, w37[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w37[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w45: [core::ffi::c_float; 2] = [
        sub_float(w5, mul_float(kWeight2, sub_float(w24, w31))),
        sub_float(w12, mul_float(kWeight2, add_float(w31, w24))),
    ];
    let w46: [core::ffi::c_float; 2] = [
        add_float(
            w16[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w35[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w35[1 as core::ffi::c_int as usize]),
            ),
        ),
        sub_float(
            sub_float(kWeight0, w16[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight4, w35[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w35[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w47: core::ffi::c_float = add_float(i1, i17) as core::ffi::c_float;
    let w48: core::ffi::c_float = sub_float(i1, i17) as core::ffi::c_float;
    let w49: core::ffi::c_float = add_float(i9, i25) as core::ffi::c_float;
    let w50: core::ffi::c_float = sub_float(i9, i25) as core::ffi::c_float;
    let w51: core::ffi::c_float = add_float(w47, w49) as core::ffi::c_float;
    let w52: core::ffi::c_float = sub_float(w47, w49) as core::ffi::c_float;
    let w54: core::ffi::c_float = add_float(i5, i21) as core::ffi::c_float;
    let w55: core::ffi::c_float = sub_float(i5, i21) as core::ffi::c_float;
    let w56: core::ffi::c_float = add_float(i13, i29) as core::ffi::c_float;
    let w57: core::ffi::c_float = sub_float(i13, i29) as core::ffi::c_float;
    let w58: core::ffi::c_float = add_float(w54, w56) as core::ffi::c_float;
    let w59: core::ffi::c_float = sub_float(w54, w56) as core::ffi::c_float;
    let w61: core::ffi::c_float = add_float(w51, w58) as core::ffi::c_float;
    let w62: core::ffi::c_float = sub_float(w51, w58) as core::ffi::c_float;
    let w63: [core::ffi::c_float; 2] = [
        add_float(w48, mul_float(kWeight2, sub_float(w55, w57))),
        sub_float(sub_float(kWeight0, w50), mul_float(kWeight2, add_float(w57, w55))),
    ];
    let w65: [core::ffi::c_float; 2] = [
        sub_float(w48, mul_float(kWeight2, sub_float(w55, w57))),
        sub_float(w50, mul_float(kWeight2, add_float(w57, w55))),
    ];
    let w66: core::ffi::c_float = add_float(i3, i19) as core::ffi::c_float;
    let w67: core::ffi::c_float = sub_float(i3, i19) as core::ffi::c_float;
    let w68: core::ffi::c_float = add_float(i11, i27) as core::ffi::c_float;
    let w69: core::ffi::c_float = sub_float(i11, i27) as core::ffi::c_float;
    let w70: core::ffi::c_float = add_float(w66, w68) as core::ffi::c_float;
    let w71: core::ffi::c_float = sub_float(w66, w68) as core::ffi::c_float;
    let w73: core::ffi::c_float = add_float(i7, i23) as core::ffi::c_float;
    let w74: core::ffi::c_float = sub_float(i7, i23) as core::ffi::c_float;
    let w75: core::ffi::c_float = add_float(i15, i31) as core::ffi::c_float;
    let w76: core::ffi::c_float = sub_float(i15, i31) as core::ffi::c_float;
    let w77: core::ffi::c_float = add_float(w73, w75) as core::ffi::c_float;
    let w78: core::ffi::c_float = sub_float(w73, w75) as core::ffi::c_float;
    let w80: core::ffi::c_float = add_float(w70, w77) as core::ffi::c_float;
    let w81: core::ffi::c_float = sub_float(w70, w77) as core::ffi::c_float;
    let w82: [core::ffi::c_float; 2] = [
        add_float(w67, mul_float(kWeight2, sub_float(w74, w76))),
        sub_float(sub_float(kWeight0, w69), mul_float(kWeight2, add_float(w76, w74))),
    ];
    let w84: [core::ffi::c_float; 2] = [
        sub_float(w67, mul_float(kWeight2, sub_float(w74, w76))),
        sub_float(w69, mul_float(kWeight2, add_float(w76, w74))),
    ];
    let w85: core::ffi::c_float = add_float(w61, w80) as core::ffi::c_float;
    let w86: core::ffi::c_float = sub_float(w61, w80) as core::ffi::c_float;
    let w87: [core::ffi::c_float; 2] = [
        add_float(
            w63[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w82[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w82[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w63[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w82[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w82[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w88: [core::ffi::c_float; 2] = [
        add_float(w52, mul_float(kWeight2, sub_float(w71, w78))),
        sub_float(sub_float(kWeight0, w59), mul_float(kWeight2, add_float(w78, w71))),
    ];
    let w89: [core::ffi::c_float; 2] = [
        add_float(
            w65[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w84[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w84[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w65[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w84[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w84[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w91: [core::ffi::c_float; 2] = [
        add_float(
            w65[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w84[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w84[1 as core::ffi::c_int as usize]),
            ),
        ),
        sub_float(
            sub_float(kWeight0, w65[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight3, w84[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w84[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w92: [core::ffi::c_float; 2] = [
        sub_float(w52, mul_float(kWeight2, sub_float(w71, w78))),
        sub_float(w59, mul_float(kWeight2, add_float(w78, w71))),
    ];
    let w93: [core::ffi::c_float; 2] = [
        add_float(
            w63[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w82[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w82[1 as core::ffi::c_int as usize]),
            ),
        ),
        sub_float(
            sub_float(kWeight0, w63[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight4, w82[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w82[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(w38, w85),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(
            w40[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight5, w87[0 as core::ffi::c_int as usize]),
                mul_float(kWeight6, w87[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        add_float(
            w41[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w88[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w88[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        add_float(
            w42[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight7, w89[0 as core::ffi::c_int as usize]),
                mul_float(kWeight8, w89[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((4 as core::ffi::c_int * stride) as isize),
        add_float(w15, mul_float(kWeight2, sub_float(w62, w81))),
    );
    store_float(
        output.offset((5 as core::ffi::c_int * stride) as isize),
        add_float(
            w44[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight8, w91[0 as core::ffi::c_int as usize]),
                mul_float(kWeight7, w91[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((6 as core::ffi::c_int * stride) as isize),
        add_float(
            w45[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w92[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w92[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((7 as core::ffi::c_int * stride) as isize),
        add_float(
            w46[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight6, w93[0 as core::ffi::c_int as usize]),
                mul_float(kWeight5, w93[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(output.offset((8 as core::ffi::c_int * stride) as isize), w39);
    store_float(
        output.offset((9 as core::ffi::c_int * stride) as isize),
        add_float(
            w46[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight6, w93[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight5, w93[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((10 as core::ffi::c_int * stride) as isize),
        add_float(
            w45[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w92[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w92[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((11 as core::ffi::c_int * stride) as isize),
        add_float(
            w44[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight8, w91[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight7, w91[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((12 as core::ffi::c_int * stride) as isize),
        sub_float(w15, mul_float(kWeight2, sub_float(w62, w81))),
    );
    store_float(
        output.offset((13 as core::ffi::c_int * stride) as isize),
        add_float(
            w42[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight7, w89[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight8, w89[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((14 as core::ffi::c_int * stride) as isize),
        add_float(
            w41[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w88[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w88[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((15 as core::ffi::c_int * stride) as isize),
        add_float(
            w40[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight5, w87[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight6, w87[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((16 as core::ffi::c_int * stride) as isize),
        sub_float(w38, w85),
    );
    store_float(
        output.offset((17 as core::ffi::c_int * stride) as isize),
        add_float(
            w40[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight5, w87[1 as core::ffi::c_int as usize]),
                mul_float(kWeight6, w87[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((18 as core::ffi::c_int * stride) as isize),
        add_float(
            w41[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w88[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w88[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((19 as core::ffi::c_int * stride) as isize),
        add_float(
            w42[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight7, w89[1 as core::ffi::c_int as usize]),
                mul_float(kWeight8, w89[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((20 as core::ffi::c_int * stride) as isize),
        sub_float(sub_float(kWeight0, w34), mul_float(kWeight2, add_float(w81, w62))),
    );
    store_float(
        output.offset((21 as core::ffi::c_int * stride) as isize),
        add_float(
            w44[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight8, w91[1 as core::ffi::c_int as usize]),
                mul_float(kWeight7, w91[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((22 as core::ffi::c_int * stride) as isize),
        add_float(
            w45[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w92[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w92[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((23 as core::ffi::c_int * stride) as isize),
        add_float(
            w46[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight6, w93[1 as core::ffi::c_int as usize]),
                mul_float(kWeight5, w93[0 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((24 as core::ffi::c_int * stride) as isize),
        sub_float(kWeight0, w86),
    );
    store_float(
        output.offset((25 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w46[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight5, w93[0 as core::ffi::c_int as usize]),
                mul_float(kWeight6, w93[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((26 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w45[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight3, w92[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w92[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((27 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w44[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight7, w91[0 as core::ffi::c_int as usize]),
                mul_float(kWeight8, w91[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((28 as core::ffi::c_int * stride) as isize),
        sub_float(w34, mul_float(kWeight2, add_float(w81, w62))),
    );
    store_float(
        output.offset((29 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w42[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight8, w89[0 as core::ffi::c_int as usize]),
                mul_float(kWeight7, w89[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((30 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w41[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight4, w88[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w88[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((31 as core::ffi::c_int * stride) as isize),
        sub_float(
            sub_float(kWeight0, w40[1 as core::ffi::c_int as usize]),
            sub_float(
                mul_float(kWeight6, w87[0 as core::ffi::c_int as usize]),
                mul_float(kWeight5, w87[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft2x2_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_fft_2d_gen(
        input,
        temp,
        output,
        2 as core::ffi::c_int,
        Some(
            aom_fft1d_2_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            unpack_2d_output
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft4x4_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_fft_2d_gen(
        input,
        temp,
        output,
        4 as core::ffi::c_int,
        Some(
            aom_fft1d_4_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            unpack_2d_output
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft8x8_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_fft_2d_gen(
        input,
        temp,
        output,
        8 as core::ffi::c_int,
        Some(
            aom_fft1d_8_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            unpack_2d_output
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft16x16_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_fft_2d_gen(
        input,
        temp,
        output,
        16 as core::ffi::c_int,
        Some(
            aom_fft1d_16_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            unpack_2d_output
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fft32x32_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_fft_2d_gen(
        input,
        temp,
        output,
        32 as core::ffi::c_int,
        Some(
            aom_fft1d_32_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            unpack_2d_output
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ifft_2d_gen(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut n: core::ffi::c_int,
    mut fft_single: aom_fft_1d_func_t,
    mut fft_multi: aom_fft_1d_func_t,
    mut ifft_multi: aom_fft_1d_func_t,
    mut transpose: aom_fft_transpose_func_t,
    mut vec_size: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y <= n / 2 as core::ffi::c_int {
        *output.offset((y * n) as isize) = *input
            .offset((2 as core::ffi::c_int * y * n) as isize);
        *output.offset((y * n + 1 as core::ffi::c_int) as isize) = *input
            .offset(
                (2 as core::ffi::c_int * (y * n + n / 2 as core::ffi::c_int)) as isize,
            );
        y += 1;
    }
    let mut y_0: core::ffi::c_int = n / 2 as core::ffi::c_int + 1 as core::ffi::c_int;
    while y_0 < n {
        *output.offset((y_0 * n) as isize) = *input
            .offset(
                (2 as core::ffi::c_int * (y_0 - n / 2 as core::ffi::c_int) * n
                    + 1 as core::ffi::c_int) as isize,
            );
        *output.offset((y_0 * n + 1 as core::ffi::c_int) as isize) = *input
            .offset(
                (2 as core::ffi::c_int
                    * ((y_0 - n / 2 as core::ffi::c_int) * n + n / 2 as core::ffi::c_int)
                    + 1 as core::ffi::c_int) as isize,
            );
        y_0 += 1;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        ifft_multi
            .expect(
                "non-null function pointer",
            )(output.offset(i as isize), temp.offset(i as isize), n);
        i += vec_size;
    }
    let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_1 < n {
        let mut x: core::ffi::c_int = 1 as core::ffi::c_int;
        while x < n / 2 as core::ffi::c_int {
            *output.offset((y_1 * n + (x + 1 as core::ffi::c_int)) as isize) = *input
                .offset((2 as core::ffi::c_int * (y_1 * n + x)) as isize);
            x += 1;
        }
        let mut x_0: core::ffi::c_int = 1 as core::ffi::c_int;
        while x_0 < n / 2 as core::ffi::c_int {
            *output.offset((y_1 * n + (x_0 + n / 2 as core::ffi::c_int)) as isize) = *input
                .offset(
                    (2 as core::ffi::c_int * (y_1 * n + x_0) + 1 as core::ffi::c_int)
                        as isize,
                );
            x_0 += 1;
        }
        y_1 += 1;
    }
    let mut y_2: core::ffi::c_int = 2 as core::ffi::c_int;
    while y_2 < vec_size {
        fft_single
            .expect(
                "non-null function pointer",
            )(output.offset(y_2 as isize), temp.offset(y_2 as isize), n);
        y_2 += 1;
    }
    let mut y_3: core::ffi::c_int = if 2 as core::ffi::c_int > vec_size {
        2 as core::ffi::c_int
    } else {
        vec_size
    };
    while y_3 < n {
        fft_multi
            .expect(
                "non-null function pointer",
            )(output.offset(y_3 as isize), temp.offset(y_3 as isize), n);
        y_3 += vec_size;
    }
    let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while x_1 < n {
        *output.offset(x_1 as isize) = *temp.offset((x_1 * n) as isize);
        *output.offset((n / 2 as core::ffi::c_int * n + x_1) as isize) = *temp
            .offset((x_1 * n + 1 as core::ffi::c_int) as isize);
        x_1 += 1;
    }
    let mut y_4: core::ffi::c_int = 1 as core::ffi::c_int;
    while y_4 < n / 2 as core::ffi::c_int {
        let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_2 <= n / 2 as core::ffi::c_int {
            *output.offset((x_2 + y_4 * n) as isize) = *temp
                .offset((y_4 + 1 as core::ffi::c_int + x_2 * n) as isize)
                + (if x_2 > 0 as core::ffi::c_int && x_2 < n / 2 as core::ffi::c_int {
                    *temp
                        .offset(
                            (y_4 + n / 2 as core::ffi::c_int
                                + (x_2 + n / 2 as core::ffi::c_int) * n) as isize,
                        )
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                });
            x_2 += 1;
        }
        let mut x_3: core::ffi::c_int = n / 2 as core::ffi::c_int
            + 1 as core::ffi::c_int;
        while x_3 < n {
            *output.offset((x_3 + y_4 * n) as isize) = *temp
                .offset((y_4 + 1 as core::ffi::c_int + (n - x_3) * n) as isize)
                - *temp
                    .offset(
                        (y_4 + n / 2 as core::ffi::c_int
                            + (n - x_3 + n / 2 as core::ffi::c_int) * n) as isize,
                    );
            x_3 += 1;
        }
        let mut x_4: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_4 <= n / 2 as core::ffi::c_int {
            *output.offset((x_4 + (y_4 + n / 2 as core::ffi::c_int) * n) as isize) = *temp
                .offset((y_4 + n / 2 as core::ffi::c_int + x_4 * n) as isize)
                - (if x_4 > 0 as core::ffi::c_int && x_4 < n / 2 as core::ffi::c_int {
                    *temp
                        .offset(
                            (y_4 + 1 as core::ffi::c_int
                                + (x_4 + n / 2 as core::ffi::c_int) * n) as isize,
                        )
                } else {
                    0 as core::ffi::c_int as core::ffi::c_float
                });
            x_4 += 1;
        }
        let mut x_5: core::ffi::c_int = n / 2 as core::ffi::c_int
            + 1 as core::ffi::c_int;
        while x_5 < n {
            *output.offset((x_5 + (y_4 + n / 2 as core::ffi::c_int) * n) as isize) = *temp
                .offset(
                    (y_4 + 1 as core::ffi::c_int
                        + (n - x_5 + n / 2 as core::ffi::c_int) * n) as isize,
                )
                + *temp
                    .offset((y_4 + n / 2 as core::ffi::c_int + (n - x_5) * n) as isize);
            x_5 += 1;
        }
        y_4 += 1;
    }
    let mut y_5: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_5 < n {
        ifft_multi
            .expect(
                "non-null function pointer",
            )(output.offset(y_5 as isize), temp.offset(y_5 as isize), n);
        y_5 += vec_size;
    }
    transpose.expect("non-null function pointer")(temp, output, n);
}
unsafe extern "C-unwind" fn aom_ifft1d_2_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    store_float(output.offset((0 as core::ffi::c_int * stride) as isize), i0 + i1);
    store_float(output.offset((1 as core::ffi::c_int * stride) as isize), i0 - i1);
}
unsafe extern "C-unwind" fn aom_ifft1d_4_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let w2: core::ffi::c_float = add_float(i0, i2) as core::ffi::c_float;
    let w3: core::ffi::c_float = sub_float(i0, i2) as core::ffi::c_float;
    let w4: [core::ffi::c_float; 2] = [add_float(i1, i1), sub_float(i3, i3)];
    let w5: [core::ffi::c_float; 2] = [
        sub_float(i1, i1),
        sub_float(sub_float(kWeight0, i3), i3),
    ];
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(w2, w4[0 as core::ffi::c_int as usize]),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(w3, w5[1 as core::ffi::c_int as usize]),
    );
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        sub_float(w2, w4[0 as core::ffi::c_int as usize]),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        sub_float(w3, w5[1 as core::ffi::c_int as usize]),
    );
}
unsafe extern "C-unwind" fn aom_ifft1d_8_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let kWeight2: core::ffi::c_float = 0.707107f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let i4: core::ffi::c_float = *input
        .offset((4 as core::ffi::c_int * stride) as isize);
    let i5: core::ffi::c_float = *input
        .offset((5 as core::ffi::c_int * stride) as isize);
    let i6: core::ffi::c_float = *input
        .offset((6 as core::ffi::c_int * stride) as isize);
    let i7: core::ffi::c_float = *input
        .offset((7 as core::ffi::c_int * stride) as isize);
    let w6: core::ffi::c_float = add_float(i0, i4) as core::ffi::c_float;
    let w7: core::ffi::c_float = sub_float(i0, i4) as core::ffi::c_float;
    let w8: [core::ffi::c_float; 2] = [add_float(i2, i2), sub_float(i6, i6)];
    let w9: [core::ffi::c_float; 2] = [
        sub_float(i2, i2),
        sub_float(sub_float(kWeight0, i6), i6),
    ];
    let w10: [core::ffi::c_float; 2] = [
        add_float(w6, w8[0 as core::ffi::c_int as usize]),
        w8[1 as core::ffi::c_int as usize],
    ];
    let w11: [core::ffi::c_float; 2] = [
        sub_float(w6, w8[0 as core::ffi::c_int as usize]),
        sub_float(kWeight0, w8[1 as core::ffi::c_int as usize]),
    ];
    let w12: [core::ffi::c_float; 2] = [
        add_float(w7, w9[1 as core::ffi::c_int as usize]),
        sub_float(kWeight0, w9[0 as core::ffi::c_int as usize]),
    ];
    let w13: [core::ffi::c_float; 2] = [
        sub_float(w7, w9[1 as core::ffi::c_int as usize]),
        w9[0 as core::ffi::c_int as usize],
    ];
    let w14: [core::ffi::c_float; 2] = [add_float(i1, i3), sub_float(i7, i5)];
    let w15: [core::ffi::c_float; 2] = [
        sub_float(i1, i3),
        sub_float(sub_float(kWeight0, i5), i7),
    ];
    let w16: [core::ffi::c_float; 2] = [add_float(i3, i1), sub_float(i5, i7)];
    let w17: [core::ffi::c_float; 2] = [
        sub_float(i3, i1),
        sub_float(sub_float(kWeight0, i7), i5),
    ];
    let w18: [core::ffi::c_float; 2] = [
        add_float(
            w14[0 as core::ffi::c_int as usize],
            w16[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w14[1 as core::ffi::c_int as usize],
            w16[1 as core::ffi::c_int as usize],
        ),
    ];
    let w19: [core::ffi::c_float; 2] = [
        sub_float(
            w14[0 as core::ffi::c_int as usize],
            w16[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w14[1 as core::ffi::c_int as usize],
            w16[1 as core::ffi::c_int as usize],
        ),
    ];
    let w20: [core::ffi::c_float; 2] = [
        add_float(
            w15[0 as core::ffi::c_int as usize],
            w17[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w15[1 as core::ffi::c_int as usize],
            w17[0 as core::ffi::c_int as usize],
        ),
    ];
    let w21: [core::ffi::c_float; 2] = [
        sub_float(
            w15[0 as core::ffi::c_int as usize],
            w17[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w15[1 as core::ffi::c_int as usize],
            w17[0 as core::ffi::c_int as usize],
        ),
    ];
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(
            w10[0 as core::ffi::c_int as usize],
            w18[0 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(
            w12[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w20[0 as core::ffi::c_int as usize],
                    w20[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        add_float(
            w11[0 as core::ffi::c_int as usize],
            w19[1 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        sub_float(
            w13[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w21[0 as core::ffi::c_int as usize],
                    w21[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((4 as core::ffi::c_int * stride) as isize),
        sub_float(
            w10[0 as core::ffi::c_int as usize],
            w18[0 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((5 as core::ffi::c_int * stride) as isize),
        add_float(
            w12[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w20[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w20[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((6 as core::ffi::c_int * stride) as isize),
        sub_float(
            w11[0 as core::ffi::c_int as usize],
            w19[1 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((7 as core::ffi::c_int * stride) as isize),
        add_float(
            w13[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w21[0 as core::ffi::c_int as usize],
                    w21[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
}
unsafe extern "C-unwind" fn aom_ifft1d_16_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let kWeight2: core::ffi::c_float = 0.707107f32;
    let kWeight3: core::ffi::c_float = 0.92388f32;
    let kWeight4: core::ffi::c_float = 0.382683f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let i4: core::ffi::c_float = *input
        .offset((4 as core::ffi::c_int * stride) as isize);
    let i5: core::ffi::c_float = *input
        .offset((5 as core::ffi::c_int * stride) as isize);
    let i6: core::ffi::c_float = *input
        .offset((6 as core::ffi::c_int * stride) as isize);
    let i7: core::ffi::c_float = *input
        .offset((7 as core::ffi::c_int * stride) as isize);
    let i8: core::ffi::c_float = *input
        .offset((8 as core::ffi::c_int * stride) as isize);
    let i9: core::ffi::c_float = *input
        .offset((9 as core::ffi::c_int * stride) as isize);
    let i10: core::ffi::c_float = *input
        .offset((10 as core::ffi::c_int * stride) as isize);
    let i11: core::ffi::c_float = *input
        .offset((11 as core::ffi::c_int * stride) as isize);
    let i12: core::ffi::c_float = *input
        .offset((12 as core::ffi::c_int * stride) as isize);
    let i13: core::ffi::c_float = *input
        .offset((13 as core::ffi::c_int * stride) as isize);
    let i14: core::ffi::c_float = *input
        .offset((14 as core::ffi::c_int * stride) as isize);
    let i15: core::ffi::c_float = *input
        .offset((15 as core::ffi::c_int * stride) as isize);
    let w14: core::ffi::c_float = add_float(i0, i8) as core::ffi::c_float;
    let w15: core::ffi::c_float = sub_float(i0, i8) as core::ffi::c_float;
    let w16: [core::ffi::c_float; 2] = [add_float(i4, i4), sub_float(i12, i12)];
    let w17: [core::ffi::c_float; 2] = [
        sub_float(i4, i4),
        sub_float(sub_float(kWeight0, i12), i12),
    ];
    let w18: [core::ffi::c_float; 2] = [
        add_float(w14, w16[0 as core::ffi::c_int as usize]),
        w16[1 as core::ffi::c_int as usize],
    ];
    let w19: [core::ffi::c_float; 2] = [
        sub_float(w14, w16[0 as core::ffi::c_int as usize]),
        sub_float(kWeight0, w16[1 as core::ffi::c_int as usize]),
    ];
    let w20: [core::ffi::c_float; 2] = [
        add_float(w15, w17[1 as core::ffi::c_int as usize]),
        sub_float(kWeight0, w17[0 as core::ffi::c_int as usize]),
    ];
    let w21: [core::ffi::c_float; 2] = [
        sub_float(w15, w17[1 as core::ffi::c_int as usize]),
        w17[0 as core::ffi::c_int as usize],
    ];
    let w22: [core::ffi::c_float; 2] = [add_float(i2, i6), sub_float(i14, i10)];
    let w23: [core::ffi::c_float; 2] = [
        sub_float(i2, i6),
        sub_float(sub_float(kWeight0, i10), i14),
    ];
    let w24: [core::ffi::c_float; 2] = [add_float(i6, i2), sub_float(i10, i14)];
    let w25: [core::ffi::c_float; 2] = [
        sub_float(i6, i2),
        sub_float(sub_float(kWeight0, i14), i10),
    ];
    let w26: [core::ffi::c_float; 2] = [
        add_float(
            w22[0 as core::ffi::c_int as usize],
            w24[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w22[1 as core::ffi::c_int as usize],
            w24[1 as core::ffi::c_int as usize],
        ),
    ];
    let w27: [core::ffi::c_float; 2] = [
        sub_float(
            w22[0 as core::ffi::c_int as usize],
            w24[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w22[1 as core::ffi::c_int as usize],
            w24[1 as core::ffi::c_int as usize],
        ),
    ];
    let w28: [core::ffi::c_float; 2] = [
        add_float(
            w23[0 as core::ffi::c_int as usize],
            w25[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w23[1 as core::ffi::c_int as usize],
            w25[0 as core::ffi::c_int as usize],
        ),
    ];
    let w29: [core::ffi::c_float; 2] = [
        sub_float(
            w23[0 as core::ffi::c_int as usize],
            w25[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w23[1 as core::ffi::c_int as usize],
            w25[0 as core::ffi::c_int as usize],
        ),
    ];
    let w30: [core::ffi::c_float; 2] = [
        add_float(
            w18[0 as core::ffi::c_int as usize],
            w26[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w18[1 as core::ffi::c_int as usize],
            w26[1 as core::ffi::c_int as usize],
        ),
    ];
    let w31: [core::ffi::c_float; 2] = [
        sub_float(
            w18[0 as core::ffi::c_int as usize],
            w26[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w18[1 as core::ffi::c_int as usize],
            w26[1 as core::ffi::c_int as usize],
        ),
    ];
    let w32: [core::ffi::c_float; 2] = [
        add_float(
            w20[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w28[0 as core::ffi::c_int as usize],
                    w28[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w20[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w28[1 as core::ffi::c_int as usize],
                    w28[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w33: [core::ffi::c_float; 2] = [
        add_float(
            w20[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w28[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w28[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w20[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w28[0 as core::ffi::c_int as usize],
                    w28[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w34: [core::ffi::c_float; 2] = [
        add_float(
            w19[0 as core::ffi::c_int as usize],
            w27[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w19[1 as core::ffi::c_int as usize],
            w27[0 as core::ffi::c_int as usize],
        ),
    ];
    let w35: [core::ffi::c_float; 2] = [
        sub_float(
            w19[0 as core::ffi::c_int as usize],
            w27[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w19[1 as core::ffi::c_int as usize],
            w27[0 as core::ffi::c_int as usize],
        ),
    ];
    let w36: [core::ffi::c_float; 2] = [
        sub_float(
            w21[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w29[0 as core::ffi::c_int as usize],
                    w29[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w21[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w29[1 as core::ffi::c_int as usize],
                    w29[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w37: [core::ffi::c_float; 2] = [
        add_float(
            w21[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w29[0 as core::ffi::c_int as usize],
                    w29[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w21[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w29[1 as core::ffi::c_int as usize],
                    w29[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w38: [core::ffi::c_float; 2] = [add_float(i1, i7), sub_float(i15, i9)];
    let w39: [core::ffi::c_float; 2] = [
        sub_float(i1, i7),
        sub_float(sub_float(kWeight0, i9), i15),
    ];
    let w40: [core::ffi::c_float; 2] = [add_float(i5, i3), sub_float(i11, i13)];
    let w41: [core::ffi::c_float; 2] = [
        sub_float(i5, i3),
        sub_float(sub_float(kWeight0, i13), i11),
    ];
    let w42: [core::ffi::c_float; 2] = [
        add_float(
            w38[0 as core::ffi::c_int as usize],
            w40[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w38[1 as core::ffi::c_int as usize],
            w40[1 as core::ffi::c_int as usize],
        ),
    ];
    let w43: [core::ffi::c_float; 2] = [
        sub_float(
            w38[0 as core::ffi::c_int as usize],
            w40[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w38[1 as core::ffi::c_int as usize],
            w40[1 as core::ffi::c_int as usize],
        ),
    ];
    let w44: [core::ffi::c_float; 2] = [
        add_float(
            w39[0 as core::ffi::c_int as usize],
            w41[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w39[1 as core::ffi::c_int as usize],
            w41[0 as core::ffi::c_int as usize],
        ),
    ];
    let w45: [core::ffi::c_float; 2] = [
        sub_float(
            w39[0 as core::ffi::c_int as usize],
            w41[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w39[1 as core::ffi::c_int as usize],
            w41[0 as core::ffi::c_int as usize],
        ),
    ];
    let w46: [core::ffi::c_float; 2] = [add_float(i3, i5), sub_float(i13, i11)];
    let w47: [core::ffi::c_float; 2] = [
        sub_float(i3, i5),
        sub_float(sub_float(kWeight0, i11), i13),
    ];
    let w48: [core::ffi::c_float; 2] = [add_float(i7, i1), sub_float(i9, i15)];
    let w49: [core::ffi::c_float; 2] = [
        sub_float(i7, i1),
        sub_float(sub_float(kWeight0, i15), i9),
    ];
    let w50: [core::ffi::c_float; 2] = [
        add_float(
            w46[0 as core::ffi::c_int as usize],
            w48[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w46[1 as core::ffi::c_int as usize],
            w48[1 as core::ffi::c_int as usize],
        ),
    ];
    let w51: [core::ffi::c_float; 2] = [
        sub_float(
            w46[0 as core::ffi::c_int as usize],
            w48[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w46[1 as core::ffi::c_int as usize],
            w48[1 as core::ffi::c_int as usize],
        ),
    ];
    let w52: [core::ffi::c_float; 2] = [
        add_float(
            w47[0 as core::ffi::c_int as usize],
            w49[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w47[1 as core::ffi::c_int as usize],
            w49[0 as core::ffi::c_int as usize],
        ),
    ];
    let w53: [core::ffi::c_float; 2] = [
        sub_float(
            w47[0 as core::ffi::c_int as usize],
            w49[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w47[1 as core::ffi::c_int as usize],
            w49[0 as core::ffi::c_int as usize],
        ),
    ];
    let w54: [core::ffi::c_float; 2] = [
        add_float(
            w42[0 as core::ffi::c_int as usize],
            w50[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w42[1 as core::ffi::c_int as usize],
            w50[1 as core::ffi::c_int as usize],
        ),
    ];
    let w55: [core::ffi::c_float; 2] = [
        sub_float(
            w42[0 as core::ffi::c_int as usize],
            w50[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w42[1 as core::ffi::c_int as usize],
            w50[1 as core::ffi::c_int as usize],
        ),
    ];
    let w56: [core::ffi::c_float; 2] = [
        add_float(
            w44[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w52[0 as core::ffi::c_int as usize],
                    w52[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w44[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w52[1 as core::ffi::c_int as usize],
                    w52[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w57: [core::ffi::c_float; 2] = [
        add_float(
            w44[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w52[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w52[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w44[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w52[0 as core::ffi::c_int as usize],
                    w52[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w58: [core::ffi::c_float; 2] = [
        add_float(
            w43[0 as core::ffi::c_int as usize],
            w51[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w43[1 as core::ffi::c_int as usize],
            w51[0 as core::ffi::c_int as usize],
        ),
    ];
    let w59: [core::ffi::c_float; 2] = [
        sub_float(
            w43[0 as core::ffi::c_int as usize],
            w51[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w43[1 as core::ffi::c_int as usize],
            w51[0 as core::ffi::c_int as usize],
        ),
    ];
    let w60: [core::ffi::c_float; 2] = [
        sub_float(
            w45[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w53[0 as core::ffi::c_int as usize],
                    w53[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w45[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w53[1 as core::ffi::c_int as usize],
                    w53[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w61: [core::ffi::c_float; 2] = [
        add_float(
            w45[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w53[0 as core::ffi::c_int as usize],
                    w53[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w45[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w53[1 as core::ffi::c_int as usize],
                    w53[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(
            w30[0 as core::ffi::c_int as usize],
            w54[0 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(
            w32[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w56[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w56[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        add_float(
            w34[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w58[0 as core::ffi::c_int as usize],
                    w58[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        add_float(
            w36[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w60[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w60[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((4 as core::ffi::c_int * stride) as isize),
        add_float(
            w31[0 as core::ffi::c_int as usize],
            w55[1 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((5 as core::ffi::c_int * stride) as isize),
        sub_float(
            w33[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w57[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w57[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((6 as core::ffi::c_int * stride) as isize),
        sub_float(
            w35[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w59[0 as core::ffi::c_int as usize],
                    w59[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((7 as core::ffi::c_int * stride) as isize),
        sub_float(
            w37[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w61[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w61[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((8 as core::ffi::c_int * stride) as isize),
        sub_float(
            w30[0 as core::ffi::c_int as usize],
            w54[0 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((9 as core::ffi::c_int * stride) as isize),
        add_float(
            w32[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w56[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w56[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((10 as core::ffi::c_int * stride) as isize),
        add_float(
            w34[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w58[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w58[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((11 as core::ffi::c_int * stride) as isize),
        add_float(
            w36[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w60[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w60[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((12 as core::ffi::c_int * stride) as isize),
        sub_float(
            w31[0 as core::ffi::c_int as usize],
            w55[1 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((13 as core::ffi::c_int * stride) as isize),
        add_float(
            w33[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w57[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w57[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((14 as core::ffi::c_int * stride) as isize),
        add_float(
            w35[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w59[0 as core::ffi::c_int as usize],
                    w59[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((15 as core::ffi::c_int * stride) as isize),
        add_float(
            w37[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w61[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w61[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
}
unsafe extern "C-unwind" fn aom_ifft1d_32_float(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut stride: core::ffi::c_int,
) {
    let kWeight0: core::ffi::c_float = 0.0f32;
    let kWeight2: core::ffi::c_float = 0.707107f32;
    let kWeight3: core::ffi::c_float = 0.92388f32;
    let kWeight4: core::ffi::c_float = 0.382683f32;
    let kWeight5: core::ffi::c_float = 0.980785f32;
    let kWeight6: core::ffi::c_float = 0.19509f32;
    let kWeight7: core::ffi::c_float = 0.83147f32;
    let kWeight8: core::ffi::c_float = 0.55557f32;
    let i0: core::ffi::c_float = *input
        .offset((0 as core::ffi::c_int * stride) as isize);
    let i1: core::ffi::c_float = *input
        .offset((1 as core::ffi::c_int * stride) as isize);
    let i2: core::ffi::c_float = *input
        .offset((2 as core::ffi::c_int * stride) as isize);
    let i3: core::ffi::c_float = *input
        .offset((3 as core::ffi::c_int * stride) as isize);
    let i4: core::ffi::c_float = *input
        .offset((4 as core::ffi::c_int * stride) as isize);
    let i5: core::ffi::c_float = *input
        .offset((5 as core::ffi::c_int * stride) as isize);
    let i6: core::ffi::c_float = *input
        .offset((6 as core::ffi::c_int * stride) as isize);
    let i7: core::ffi::c_float = *input
        .offset((7 as core::ffi::c_int * stride) as isize);
    let i8: core::ffi::c_float = *input
        .offset((8 as core::ffi::c_int * stride) as isize);
    let i9: core::ffi::c_float = *input
        .offset((9 as core::ffi::c_int * stride) as isize);
    let i10: core::ffi::c_float = *input
        .offset((10 as core::ffi::c_int * stride) as isize);
    let i11: core::ffi::c_float = *input
        .offset((11 as core::ffi::c_int * stride) as isize);
    let i12: core::ffi::c_float = *input
        .offset((12 as core::ffi::c_int * stride) as isize);
    let i13: core::ffi::c_float = *input
        .offset((13 as core::ffi::c_int * stride) as isize);
    let i14: core::ffi::c_float = *input
        .offset((14 as core::ffi::c_int * stride) as isize);
    let i15: core::ffi::c_float = *input
        .offset((15 as core::ffi::c_int * stride) as isize);
    let i16: core::ffi::c_float = *input
        .offset((16 as core::ffi::c_int * stride) as isize);
    let i17: core::ffi::c_float = *input
        .offset((17 as core::ffi::c_int * stride) as isize);
    let i18: core::ffi::c_float = *input
        .offset((18 as core::ffi::c_int * stride) as isize);
    let i19: core::ffi::c_float = *input
        .offset((19 as core::ffi::c_int * stride) as isize);
    let i20: core::ffi::c_float = *input
        .offset((20 as core::ffi::c_int * stride) as isize);
    let i21: core::ffi::c_float = *input
        .offset((21 as core::ffi::c_int * stride) as isize);
    let i22: core::ffi::c_float = *input
        .offset((22 as core::ffi::c_int * stride) as isize);
    let i23: core::ffi::c_float = *input
        .offset((23 as core::ffi::c_int * stride) as isize);
    let i24: core::ffi::c_float = *input
        .offset((24 as core::ffi::c_int * stride) as isize);
    let i25: core::ffi::c_float = *input
        .offset((25 as core::ffi::c_int * stride) as isize);
    let i26: core::ffi::c_float = *input
        .offset((26 as core::ffi::c_int * stride) as isize);
    let i27: core::ffi::c_float = *input
        .offset((27 as core::ffi::c_int * stride) as isize);
    let i28: core::ffi::c_float = *input
        .offset((28 as core::ffi::c_int * stride) as isize);
    let i29: core::ffi::c_float = *input
        .offset((29 as core::ffi::c_int * stride) as isize);
    let i30: core::ffi::c_float = *input
        .offset((30 as core::ffi::c_int * stride) as isize);
    let i31: core::ffi::c_float = *input
        .offset((31 as core::ffi::c_int * stride) as isize);
    let w30: core::ffi::c_float = add_float(i0, i16) as core::ffi::c_float;
    let w31: core::ffi::c_float = sub_float(i0, i16) as core::ffi::c_float;
    let w32: [core::ffi::c_float; 2] = [add_float(i8, i8), sub_float(i24, i24)];
    let w33: [core::ffi::c_float; 2] = [
        sub_float(i8, i8),
        sub_float(sub_float(kWeight0, i24), i24),
    ];
    let w34: [core::ffi::c_float; 2] = [
        add_float(w30, w32[0 as core::ffi::c_int as usize]),
        w32[1 as core::ffi::c_int as usize],
    ];
    let w35: [core::ffi::c_float; 2] = [
        sub_float(w30, w32[0 as core::ffi::c_int as usize]),
        sub_float(kWeight0, w32[1 as core::ffi::c_int as usize]),
    ];
    let w36: [core::ffi::c_float; 2] = [
        add_float(w31, w33[1 as core::ffi::c_int as usize]),
        sub_float(kWeight0, w33[0 as core::ffi::c_int as usize]),
    ];
    let w37: [core::ffi::c_float; 2] = [
        sub_float(w31, w33[1 as core::ffi::c_int as usize]),
        w33[0 as core::ffi::c_int as usize],
    ];
    let w38: [core::ffi::c_float; 2] = [add_float(i4, i12), sub_float(i28, i20)];
    let w39: [core::ffi::c_float; 2] = [
        sub_float(i4, i12),
        sub_float(sub_float(kWeight0, i20), i28),
    ];
    let w40: [core::ffi::c_float; 2] = [add_float(i12, i4), sub_float(i20, i28)];
    let w41: [core::ffi::c_float; 2] = [
        sub_float(i12, i4),
        sub_float(sub_float(kWeight0, i28), i20),
    ];
    let w42: [core::ffi::c_float; 2] = [
        add_float(
            w38[0 as core::ffi::c_int as usize],
            w40[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w38[1 as core::ffi::c_int as usize],
            w40[1 as core::ffi::c_int as usize],
        ),
    ];
    let w43: [core::ffi::c_float; 2] = [
        sub_float(
            w38[0 as core::ffi::c_int as usize],
            w40[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w38[1 as core::ffi::c_int as usize],
            w40[1 as core::ffi::c_int as usize],
        ),
    ];
    let w44: [core::ffi::c_float; 2] = [
        add_float(
            w39[0 as core::ffi::c_int as usize],
            w41[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w39[1 as core::ffi::c_int as usize],
            w41[0 as core::ffi::c_int as usize],
        ),
    ];
    let w45: [core::ffi::c_float; 2] = [
        sub_float(
            w39[0 as core::ffi::c_int as usize],
            w41[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w39[1 as core::ffi::c_int as usize],
            w41[0 as core::ffi::c_int as usize],
        ),
    ];
    let w46: [core::ffi::c_float; 2] = [
        add_float(
            w34[0 as core::ffi::c_int as usize],
            w42[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w34[1 as core::ffi::c_int as usize],
            w42[1 as core::ffi::c_int as usize],
        ),
    ];
    let w47: [core::ffi::c_float; 2] = [
        sub_float(
            w34[0 as core::ffi::c_int as usize],
            w42[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w34[1 as core::ffi::c_int as usize],
            w42[1 as core::ffi::c_int as usize],
        ),
    ];
    let w48: [core::ffi::c_float; 2] = [
        add_float(
            w36[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w44[0 as core::ffi::c_int as usize],
                    w44[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w36[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w44[1 as core::ffi::c_int as usize],
                    w44[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w49: [core::ffi::c_float; 2] = [
        add_float(
            w36[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w44[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w44[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w36[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w44[0 as core::ffi::c_int as usize],
                    w44[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w50: [core::ffi::c_float; 2] = [
        add_float(
            w35[0 as core::ffi::c_int as usize],
            w43[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w35[1 as core::ffi::c_int as usize],
            w43[0 as core::ffi::c_int as usize],
        ),
    ];
    let w51: [core::ffi::c_float; 2] = [
        sub_float(
            w35[0 as core::ffi::c_int as usize],
            w43[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w35[1 as core::ffi::c_int as usize],
            w43[0 as core::ffi::c_int as usize],
        ),
    ];
    let w52: [core::ffi::c_float; 2] = [
        sub_float(
            w37[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w45[0 as core::ffi::c_int as usize],
                    w45[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w37[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w45[1 as core::ffi::c_int as usize],
                    w45[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w53: [core::ffi::c_float; 2] = [
        add_float(
            w37[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w45[0 as core::ffi::c_int as usize],
                    w45[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w37[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w45[1 as core::ffi::c_int as usize],
                    w45[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w54: [core::ffi::c_float; 2] = [add_float(i2, i14), sub_float(i30, i18)];
    let w55: [core::ffi::c_float; 2] = [
        sub_float(i2, i14),
        sub_float(sub_float(kWeight0, i18), i30),
    ];
    let w56: [core::ffi::c_float; 2] = [add_float(i10, i6), sub_float(i22, i26)];
    let w57: [core::ffi::c_float; 2] = [
        sub_float(i10, i6),
        sub_float(sub_float(kWeight0, i26), i22),
    ];
    let w58: [core::ffi::c_float; 2] = [
        add_float(
            w54[0 as core::ffi::c_int as usize],
            w56[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w54[1 as core::ffi::c_int as usize],
            w56[1 as core::ffi::c_int as usize],
        ),
    ];
    let w59: [core::ffi::c_float; 2] = [
        sub_float(
            w54[0 as core::ffi::c_int as usize],
            w56[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w54[1 as core::ffi::c_int as usize],
            w56[1 as core::ffi::c_int as usize],
        ),
    ];
    let w60: [core::ffi::c_float; 2] = [
        add_float(
            w55[0 as core::ffi::c_int as usize],
            w57[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w55[1 as core::ffi::c_int as usize],
            w57[0 as core::ffi::c_int as usize],
        ),
    ];
    let w61: [core::ffi::c_float; 2] = [
        sub_float(
            w55[0 as core::ffi::c_int as usize],
            w57[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w55[1 as core::ffi::c_int as usize],
            w57[0 as core::ffi::c_int as usize],
        ),
    ];
    let w62: [core::ffi::c_float; 2] = [add_float(i6, i10), sub_float(i26, i22)];
    let w63: [core::ffi::c_float; 2] = [
        sub_float(i6, i10),
        sub_float(sub_float(kWeight0, i22), i26),
    ];
    let w64: [core::ffi::c_float; 2] = [add_float(i14, i2), sub_float(i18, i30)];
    let w65: [core::ffi::c_float; 2] = [
        sub_float(i14, i2),
        sub_float(sub_float(kWeight0, i30), i18),
    ];
    let w66: [core::ffi::c_float; 2] = [
        add_float(
            w62[0 as core::ffi::c_int as usize],
            w64[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w62[1 as core::ffi::c_int as usize],
            w64[1 as core::ffi::c_int as usize],
        ),
    ];
    let w67: [core::ffi::c_float; 2] = [
        sub_float(
            w62[0 as core::ffi::c_int as usize],
            w64[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w62[1 as core::ffi::c_int as usize],
            w64[1 as core::ffi::c_int as usize],
        ),
    ];
    let w68: [core::ffi::c_float; 2] = [
        add_float(
            w63[0 as core::ffi::c_int as usize],
            w65[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w63[1 as core::ffi::c_int as usize],
            w65[0 as core::ffi::c_int as usize],
        ),
    ];
    let w69: [core::ffi::c_float; 2] = [
        sub_float(
            w63[0 as core::ffi::c_int as usize],
            w65[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w63[1 as core::ffi::c_int as usize],
            w65[0 as core::ffi::c_int as usize],
        ),
    ];
    let w70: [core::ffi::c_float; 2] = [
        add_float(
            w58[0 as core::ffi::c_int as usize],
            w66[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w58[1 as core::ffi::c_int as usize],
            w66[1 as core::ffi::c_int as usize],
        ),
    ];
    let w71: [core::ffi::c_float; 2] = [
        sub_float(
            w58[0 as core::ffi::c_int as usize],
            w66[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w58[1 as core::ffi::c_int as usize],
            w66[1 as core::ffi::c_int as usize],
        ),
    ];
    let w72: [core::ffi::c_float; 2] = [
        add_float(
            w60[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w68[0 as core::ffi::c_int as usize],
                    w68[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w60[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w68[1 as core::ffi::c_int as usize],
                    w68[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w73: [core::ffi::c_float; 2] = [
        add_float(
            w60[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w68[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w68[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w60[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w68[0 as core::ffi::c_int as usize],
                    w68[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w74: [core::ffi::c_float; 2] = [
        add_float(
            w59[0 as core::ffi::c_int as usize],
            w67[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w59[1 as core::ffi::c_int as usize],
            w67[0 as core::ffi::c_int as usize],
        ),
    ];
    let w75: [core::ffi::c_float; 2] = [
        sub_float(
            w59[0 as core::ffi::c_int as usize],
            w67[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w59[1 as core::ffi::c_int as usize],
            w67[0 as core::ffi::c_int as usize],
        ),
    ];
    let w76: [core::ffi::c_float; 2] = [
        sub_float(
            w61[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w69[0 as core::ffi::c_int as usize],
                    w69[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w61[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w69[1 as core::ffi::c_int as usize],
                    w69[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w77: [core::ffi::c_float; 2] = [
        add_float(
            w61[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w69[0 as core::ffi::c_int as usize],
                    w69[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w61[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w69[1 as core::ffi::c_int as usize],
                    w69[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w78: [core::ffi::c_float; 2] = [
        add_float(
            w46[0 as core::ffi::c_int as usize],
            w70[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w46[1 as core::ffi::c_int as usize],
            w70[1 as core::ffi::c_int as usize],
        ),
    ];
    let w79: [core::ffi::c_float; 2] = [
        sub_float(
            w46[0 as core::ffi::c_int as usize],
            w70[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w46[1 as core::ffi::c_int as usize],
            w70[1 as core::ffi::c_int as usize],
        ),
    ];
    let w80: [core::ffi::c_float; 2] = [
        add_float(
            w48[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w72[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w72[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w48[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w72[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w72[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w81: [core::ffi::c_float; 2] = [
        add_float(
            w48[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w72[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w72[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w48[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w72[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w72[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w82: [core::ffi::c_float; 2] = [
        add_float(
            w50[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w74[0 as core::ffi::c_int as usize],
                    w74[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w50[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w74[1 as core::ffi::c_int as usize],
                    w74[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w83: [core::ffi::c_float; 2] = [
        add_float(
            w50[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w74[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w74[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w50[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w74[0 as core::ffi::c_int as usize],
                    w74[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w84: [core::ffi::c_float; 2] = [
        add_float(
            w52[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w76[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w76[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w52[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w76[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w76[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w85: [core::ffi::c_float; 2] = [
        add_float(
            w52[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w76[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w76[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w52[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w76[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w76[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w86: [core::ffi::c_float; 2] = [
        add_float(
            w47[0 as core::ffi::c_int as usize],
            w71[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w47[1 as core::ffi::c_int as usize],
            w71[0 as core::ffi::c_int as usize],
        ),
    ];
    let w87: [core::ffi::c_float; 2] = [
        sub_float(
            w47[0 as core::ffi::c_int as usize],
            w71[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w47[1 as core::ffi::c_int as usize],
            w71[0 as core::ffi::c_int as usize],
        ),
    ];
    let w88: [core::ffi::c_float; 2] = [
        sub_float(
            w49[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w73[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w73[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w49[1 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w73[1 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w73[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w89: [core::ffi::c_float; 2] = [
        add_float(
            w49[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w73[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w73[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w49[1 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w73[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w73[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w90: [core::ffi::c_float; 2] = [
        sub_float(
            w51[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w75[0 as core::ffi::c_int as usize],
                    w75[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w51[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w75[1 as core::ffi::c_int as usize],
                    w75[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w91: [core::ffi::c_float; 2] = [
        add_float(
            w51[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w75[0 as core::ffi::c_int as usize],
                    w75[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w51[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w75[1 as core::ffi::c_int as usize],
                    w75[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w92: [core::ffi::c_float; 2] = [
        sub_float(
            w53[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w77[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w77[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w53[1 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w77[1 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w77[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w93: [core::ffi::c_float; 2] = [
        add_float(
            w53[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w77[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w77[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w53[1 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w77[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w77[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w94: [core::ffi::c_float; 2] = [add_float(i1, i15), sub_float(i31, i17)];
    let w95: [core::ffi::c_float; 2] = [
        sub_float(i1, i15),
        sub_float(sub_float(kWeight0, i17), i31),
    ];
    let w96: [core::ffi::c_float; 2] = [add_float(i9, i7), sub_float(i23, i25)];
    let w97: [core::ffi::c_float; 2] = [
        sub_float(i9, i7),
        sub_float(sub_float(kWeight0, i25), i23),
    ];
    let w98: [core::ffi::c_float; 2] = [
        add_float(
            w94[0 as core::ffi::c_int as usize],
            w96[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w94[1 as core::ffi::c_int as usize],
            w96[1 as core::ffi::c_int as usize],
        ),
    ];
    let w99: [core::ffi::c_float; 2] = [
        sub_float(
            w94[0 as core::ffi::c_int as usize],
            w96[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w94[1 as core::ffi::c_int as usize],
            w96[1 as core::ffi::c_int as usize],
        ),
    ];
    let w100: [core::ffi::c_float; 2] = [
        add_float(
            w95[0 as core::ffi::c_int as usize],
            w97[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w95[1 as core::ffi::c_int as usize],
            w97[0 as core::ffi::c_int as usize],
        ),
    ];
    let w101: [core::ffi::c_float; 2] = [
        sub_float(
            w95[0 as core::ffi::c_int as usize],
            w97[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w95[1 as core::ffi::c_int as usize],
            w97[0 as core::ffi::c_int as usize],
        ),
    ];
    let w102: [core::ffi::c_float; 2] = [add_float(i5, i11), sub_float(i27, i21)];
    let w103: [core::ffi::c_float; 2] = [
        sub_float(i5, i11),
        sub_float(sub_float(kWeight0, i21), i27),
    ];
    let w104: [core::ffi::c_float; 2] = [add_float(i13, i3), sub_float(i19, i29)];
    let w105: [core::ffi::c_float; 2] = [
        sub_float(i13, i3),
        sub_float(sub_float(kWeight0, i29), i19),
    ];
    let w106: [core::ffi::c_float; 2] = [
        add_float(
            w102[0 as core::ffi::c_int as usize],
            w104[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w102[1 as core::ffi::c_int as usize],
            w104[1 as core::ffi::c_int as usize],
        ),
    ];
    let w107: [core::ffi::c_float; 2] = [
        sub_float(
            w102[0 as core::ffi::c_int as usize],
            w104[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w102[1 as core::ffi::c_int as usize],
            w104[1 as core::ffi::c_int as usize],
        ),
    ];
    let w108: [core::ffi::c_float; 2] = [
        add_float(
            w103[0 as core::ffi::c_int as usize],
            w105[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w103[1 as core::ffi::c_int as usize],
            w105[0 as core::ffi::c_int as usize],
        ),
    ];
    let w109: [core::ffi::c_float; 2] = [
        sub_float(
            w103[0 as core::ffi::c_int as usize],
            w105[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w103[1 as core::ffi::c_int as usize],
            w105[0 as core::ffi::c_int as usize],
        ),
    ];
    let w110: [core::ffi::c_float; 2] = [
        add_float(
            w98[0 as core::ffi::c_int as usize],
            w106[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w98[1 as core::ffi::c_int as usize],
            w106[1 as core::ffi::c_int as usize],
        ),
    ];
    let w111: [core::ffi::c_float; 2] = [
        sub_float(
            w98[0 as core::ffi::c_int as usize],
            w106[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w98[1 as core::ffi::c_int as usize],
            w106[1 as core::ffi::c_int as usize],
        ),
    ];
    let w112: [core::ffi::c_float; 2] = [
        add_float(
            w100[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w108[0 as core::ffi::c_int as usize],
                    w108[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w100[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w108[1 as core::ffi::c_int as usize],
                    w108[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w113: [core::ffi::c_float; 2] = [
        add_float(
            w100[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w108[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w108[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w100[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w108[0 as core::ffi::c_int as usize],
                    w108[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w114: [core::ffi::c_float; 2] = [
        add_float(
            w99[0 as core::ffi::c_int as usize],
            w107[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w99[1 as core::ffi::c_int as usize],
            w107[0 as core::ffi::c_int as usize],
        ),
    ];
    let w115: [core::ffi::c_float; 2] = [
        sub_float(
            w99[0 as core::ffi::c_int as usize],
            w107[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w99[1 as core::ffi::c_int as usize],
            w107[0 as core::ffi::c_int as usize],
        ),
    ];
    let w116: [core::ffi::c_float; 2] = [
        sub_float(
            w101[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w109[0 as core::ffi::c_int as usize],
                    w109[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w101[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w109[1 as core::ffi::c_int as usize],
                    w109[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w117: [core::ffi::c_float; 2] = [
        add_float(
            w101[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w109[0 as core::ffi::c_int as usize],
                    w109[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w101[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w109[1 as core::ffi::c_int as usize],
                    w109[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w118: [core::ffi::c_float; 2] = [add_float(i3, i13), sub_float(i29, i19)];
    let w119: [core::ffi::c_float; 2] = [
        sub_float(i3, i13),
        sub_float(sub_float(kWeight0, i19), i29),
    ];
    let w120: [core::ffi::c_float; 2] = [add_float(i11, i5), sub_float(i21, i27)];
    let w121: [core::ffi::c_float; 2] = [
        sub_float(i11, i5),
        sub_float(sub_float(kWeight0, i27), i21),
    ];
    let w122: [core::ffi::c_float; 2] = [
        add_float(
            w118[0 as core::ffi::c_int as usize],
            w120[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w118[1 as core::ffi::c_int as usize],
            w120[1 as core::ffi::c_int as usize],
        ),
    ];
    let w123: [core::ffi::c_float; 2] = [
        sub_float(
            w118[0 as core::ffi::c_int as usize],
            w120[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w118[1 as core::ffi::c_int as usize],
            w120[1 as core::ffi::c_int as usize],
        ),
    ];
    let w124: [core::ffi::c_float; 2] = [
        add_float(
            w119[0 as core::ffi::c_int as usize],
            w121[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w119[1 as core::ffi::c_int as usize],
            w121[0 as core::ffi::c_int as usize],
        ),
    ];
    let w125: [core::ffi::c_float; 2] = [
        sub_float(
            w119[0 as core::ffi::c_int as usize],
            w121[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w119[1 as core::ffi::c_int as usize],
            w121[0 as core::ffi::c_int as usize],
        ),
    ];
    let w126: [core::ffi::c_float; 2] = [add_float(i7, i9), sub_float(i25, i23)];
    let w127: [core::ffi::c_float; 2] = [
        sub_float(i7, i9),
        sub_float(sub_float(kWeight0, i23), i25),
    ];
    let w128: [core::ffi::c_float; 2] = [add_float(i15, i1), sub_float(i17, i31)];
    let w129: [core::ffi::c_float; 2] = [
        sub_float(i15, i1),
        sub_float(sub_float(kWeight0, i31), i17),
    ];
    let w130: [core::ffi::c_float; 2] = [
        add_float(
            w126[0 as core::ffi::c_int as usize],
            w128[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w126[1 as core::ffi::c_int as usize],
            w128[1 as core::ffi::c_int as usize],
        ),
    ];
    let w131: [core::ffi::c_float; 2] = [
        sub_float(
            w126[0 as core::ffi::c_int as usize],
            w128[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w126[1 as core::ffi::c_int as usize],
            w128[1 as core::ffi::c_int as usize],
        ),
    ];
    let w132: [core::ffi::c_float; 2] = [
        add_float(
            w127[0 as core::ffi::c_int as usize],
            w129[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w127[1 as core::ffi::c_int as usize],
            w129[0 as core::ffi::c_int as usize],
        ),
    ];
    let w133: [core::ffi::c_float; 2] = [
        sub_float(
            w127[0 as core::ffi::c_int as usize],
            w129[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w127[1 as core::ffi::c_int as usize],
            w129[0 as core::ffi::c_int as usize],
        ),
    ];
    let w134: [core::ffi::c_float; 2] = [
        add_float(
            w122[0 as core::ffi::c_int as usize],
            w130[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w122[1 as core::ffi::c_int as usize],
            w130[1 as core::ffi::c_int as usize],
        ),
    ];
    let w135: [core::ffi::c_float; 2] = [
        sub_float(
            w122[0 as core::ffi::c_int as usize],
            w130[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w122[1 as core::ffi::c_int as usize],
            w130[1 as core::ffi::c_int as usize],
        ),
    ];
    let w136: [core::ffi::c_float; 2] = [
        add_float(
            w124[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w132[0 as core::ffi::c_int as usize],
                    w132[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w124[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w132[1 as core::ffi::c_int as usize],
                    w132[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w137: [core::ffi::c_float; 2] = [
        add_float(
            w124[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w132[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w132[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w124[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w132[0 as core::ffi::c_int as usize],
                    w132[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w138: [core::ffi::c_float; 2] = [
        add_float(
            w123[0 as core::ffi::c_int as usize],
            w131[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w123[1 as core::ffi::c_int as usize],
            w131[0 as core::ffi::c_int as usize],
        ),
    ];
    let w139: [core::ffi::c_float; 2] = [
        sub_float(
            w123[0 as core::ffi::c_int as usize],
            w131[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w123[1 as core::ffi::c_int as usize],
            w131[0 as core::ffi::c_int as usize],
        ),
    ];
    let w140: [core::ffi::c_float; 2] = [
        sub_float(
            w125[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w133[0 as core::ffi::c_int as usize],
                    w133[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w125[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w133[1 as core::ffi::c_int as usize],
                    w133[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w141: [core::ffi::c_float; 2] = [
        add_float(
            w125[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w133[0 as core::ffi::c_int as usize],
                    w133[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w125[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w133[1 as core::ffi::c_int as usize],
                    w133[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w142: [core::ffi::c_float; 2] = [
        add_float(
            w110[0 as core::ffi::c_int as usize],
            w134[0 as core::ffi::c_int as usize],
        ),
        add_float(
            w110[1 as core::ffi::c_int as usize],
            w134[1 as core::ffi::c_int as usize],
        ),
    ];
    let w143: [core::ffi::c_float; 2] = [
        sub_float(
            w110[0 as core::ffi::c_int as usize],
            w134[0 as core::ffi::c_int as usize],
        ),
        sub_float(
            w110[1 as core::ffi::c_int as usize],
            w134[1 as core::ffi::c_int as usize],
        ),
    ];
    let w144: [core::ffi::c_float; 2] = [
        add_float(
            w112[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w136[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w136[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w112[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w136[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w136[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w145: [core::ffi::c_float; 2] = [
        add_float(
            w112[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w136[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w136[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w112[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w136[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w136[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w146: [core::ffi::c_float; 2] = [
        add_float(
            w114[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w138[0 as core::ffi::c_int as usize],
                    w138[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w114[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w138[1 as core::ffi::c_int as usize],
                    w138[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w147: [core::ffi::c_float; 2] = [
        add_float(
            w114[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w138[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w138[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w114[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w138[0 as core::ffi::c_int as usize],
                    w138[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w148: [core::ffi::c_float; 2] = [
        add_float(
            w116[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w140[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w140[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w116[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w140[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w140[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w149: [core::ffi::c_float; 2] = [
        add_float(
            w116[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w140[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w140[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w116[1 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w140[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w140[1 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w150: [core::ffi::c_float; 2] = [
        add_float(
            w111[0 as core::ffi::c_int as usize],
            w135[1 as core::ffi::c_int as usize],
        ),
        sub_float(
            w111[1 as core::ffi::c_int as usize],
            w135[0 as core::ffi::c_int as usize],
        ),
    ];
    let w151: [core::ffi::c_float; 2] = [
        sub_float(
            w111[0 as core::ffi::c_int as usize],
            w135[1 as core::ffi::c_int as usize],
        ),
        add_float(
            w111[1 as core::ffi::c_int as usize],
            w135[0 as core::ffi::c_int as usize],
        ),
    ];
    let w152: [core::ffi::c_float; 2] = [
        sub_float(
            w113[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w137[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w137[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w113[1 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w137[1 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w137[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w153: [core::ffi::c_float; 2] = [
        add_float(
            w113[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w137[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w137[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w113[1 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w137[1 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w137[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w154: [core::ffi::c_float; 2] = [
        sub_float(
            w115[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w139[0 as core::ffi::c_int as usize],
                    w139[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        sub_float(
            w115[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w139[1 as core::ffi::c_int as usize],
                    w139[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w155: [core::ffi::c_float; 2] = [
        add_float(
            w115[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w139[0 as core::ffi::c_int as usize],
                    w139[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
        add_float(
            w115[1 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w139[1 as core::ffi::c_int as usize],
                    w139[0 as core::ffi::c_int as usize],
                ),
            ),
        ),
    ];
    let w156: [core::ffi::c_float; 2] = [
        sub_float(
            w117[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w141[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w141[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w117[1 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w141[1 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w141[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    let w157: [core::ffi::c_float; 2] = [
        add_float(
            w117[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w141[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w141[1 as core::ffi::c_int as usize]),
            ),
        ),
        add_float(
            w117[1 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w141[1 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w141[0 as core::ffi::c_int as usize]),
            ),
        ),
    ];
    store_float(
        output.offset((0 as core::ffi::c_int * stride) as isize),
        add_float(
            w78[0 as core::ffi::c_int as usize],
            w142[0 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((1 as core::ffi::c_int * stride) as isize),
        add_float(
            w80[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight5, w144[0 as core::ffi::c_int as usize]),
                mul_float(kWeight6, w144[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((2 as core::ffi::c_int * stride) as isize),
        add_float(
            w82[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight3, w146[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w146[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((3 as core::ffi::c_int * stride) as isize),
        add_float(
            w84[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight7, w148[0 as core::ffi::c_int as usize]),
                mul_float(kWeight8, w148[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((4 as core::ffi::c_int * stride) as isize),
        add_float(
            w86[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                add_float(
                    w150[0 as core::ffi::c_int as usize],
                    w150[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((5 as core::ffi::c_int * stride) as isize),
        add_float(
            w88[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight8, w152[0 as core::ffi::c_int as usize]),
                mul_float(kWeight7, w152[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((6 as core::ffi::c_int * stride) as isize),
        add_float(
            w90[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight4, w154[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w154[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((7 as core::ffi::c_int * stride) as isize),
        add_float(
            w92[0 as core::ffi::c_int as usize],
            add_float(
                mul_float(kWeight6, w156[0 as core::ffi::c_int as usize]),
                mul_float(kWeight5, w156[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((8 as core::ffi::c_int * stride) as isize),
        add_float(
            w79[0 as core::ffi::c_int as usize],
            w143[1 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((9 as core::ffi::c_int * stride) as isize),
        sub_float(
            w81[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight6, w145[0 as core::ffi::c_int as usize]),
                mul_float(kWeight5, w145[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((10 as core::ffi::c_int * stride) as isize),
        sub_float(
            w83[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w147[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w147[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((11 as core::ffi::c_int * stride) as isize),
        sub_float(
            w85[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight8, w149[0 as core::ffi::c_int as usize]),
                mul_float(kWeight7, w149[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((12 as core::ffi::c_int * stride) as isize),
        sub_float(
            w87[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w151[0 as core::ffi::c_int as usize],
                    w151[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((13 as core::ffi::c_int * stride) as isize),
        sub_float(
            w89[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight7, w153[0 as core::ffi::c_int as usize]),
                mul_float(kWeight8, w153[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((14 as core::ffi::c_int * stride) as isize),
        sub_float(
            w91[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w155[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w155[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((15 as core::ffi::c_int * stride) as isize),
        sub_float(
            w93[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight5, w157[0 as core::ffi::c_int as usize]),
                mul_float(kWeight6, w157[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((16 as core::ffi::c_int * stride) as isize),
        sub_float(
            w78[0 as core::ffi::c_int as usize],
            w142[0 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((17 as core::ffi::c_int * stride) as isize),
        add_float(
            w80[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight5, w144[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight6, w144[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((18 as core::ffi::c_int * stride) as isize),
        add_float(
            w82[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight3, w146[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight4, w146[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((19 as core::ffi::c_int * stride) as isize),
        add_float(
            w84[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight7, w148[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight8, w148[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((20 as core::ffi::c_int * stride) as isize),
        add_float(
            w86[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight2, w150[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight2, w150[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((21 as core::ffi::c_int * stride) as isize),
        add_float(
            w88[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight8, w152[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight7, w152[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((22 as core::ffi::c_int * stride) as isize),
        add_float(
            w90[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight4, w154[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight3, w154[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((23 as core::ffi::c_int * stride) as isize),
        add_float(
            w92[0 as core::ffi::c_int as usize],
            sub_float(
                sub_float(
                    kWeight0,
                    mul_float(kWeight6, w156[0 as core::ffi::c_int as usize]),
                ),
                mul_float(kWeight5, w156[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((24 as core::ffi::c_int * stride) as isize),
        sub_float(
            w79[0 as core::ffi::c_int as usize],
            w143[1 as core::ffi::c_int as usize],
        ),
    );
    store_float(
        output.offset((25 as core::ffi::c_int * stride) as isize),
        add_float(
            w81[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight6, w145[0 as core::ffi::c_int as usize]),
                mul_float(kWeight5, w145[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((26 as core::ffi::c_int * stride) as isize),
        add_float(
            w83[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight4, w147[0 as core::ffi::c_int as usize]),
                mul_float(kWeight3, w147[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((27 as core::ffi::c_int * stride) as isize),
        add_float(
            w85[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight8, w149[0 as core::ffi::c_int as usize]),
                mul_float(kWeight7, w149[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((28 as core::ffi::c_int * stride) as isize),
        add_float(
            w87[0 as core::ffi::c_int as usize],
            mul_float(
                kWeight2,
                sub_float(
                    w151[0 as core::ffi::c_int as usize],
                    w151[1 as core::ffi::c_int as usize],
                ),
            ),
        ),
    );
    store_float(
        output.offset((29 as core::ffi::c_int * stride) as isize),
        add_float(
            w89[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight7, w153[0 as core::ffi::c_int as usize]),
                mul_float(kWeight8, w153[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((30 as core::ffi::c_int * stride) as isize),
        add_float(
            w91[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight3, w155[0 as core::ffi::c_int as usize]),
                mul_float(kWeight4, w155[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
    store_float(
        output.offset((31 as core::ffi::c_int * stride) as isize),
        add_float(
            w93[0 as core::ffi::c_int as usize],
            sub_float(
                mul_float(kWeight5, w157[0 as core::ffi::c_int as usize]),
                mul_float(kWeight6, w157[1 as core::ffi::c_int as usize]),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ifft2x2_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_ifft_2d_gen(
        input,
        temp,
        output,
        2 as core::ffi::c_int,
        Some(
            aom_fft1d_2_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_fft1d_2_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_ifft1d_2_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ifft4x4_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_ifft_2d_gen(
        input,
        temp,
        output,
        4 as core::ffi::c_int,
        Some(
            aom_fft1d_4_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_fft1d_4_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_ifft1d_4_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ifft8x8_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_ifft_2d_gen(
        input,
        temp,
        output,
        8 as core::ffi::c_int,
        Some(
            aom_fft1d_8_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_fft1d_8_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_ifft1d_8_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ifft16x16_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_ifft_2d_gen(
        input,
        temp,
        output,
        16 as core::ffi::c_int,
        Some(
            aom_fft1d_16_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_fft1d_16_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_ifft1d_16_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_ifft32x32_float_c(
    mut input: *const core::ffi::c_float,
    mut temp: *mut core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    aom_ifft_2d_gen(
        input,
        temp,
        output,
        32 as core::ffi::c_int,
        Some(
            aom_fft1d_32_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_fft1d_32_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            aom_ifft1d_32_float
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        Some(
            simple_transpose
                as unsafe extern "C-unwind" fn(
                    *const core::ffi::c_float,
                    *mut core::ffi::c_float,
                    core::ffi::c_int,
                ) -> (),
        ),
        1 as core::ffi::c_int,
    );
}
