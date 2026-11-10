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

// Mechanically translated from upstream/aom/av1/encoder/dwt.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __darwin_size_t = usize;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type tran_low_t = int32_t;
unsafe extern "C-unwind" fn analysis_53_row(
    mut length: core::ffi::c_int,
    mut x: *mut tran_low_t,
    mut lowpass: *mut tran_low_t,
    mut highpass: *mut tran_low_t,
) {
    let mut n: core::ffi::c_int = 0;
    let mut r: tran_low_t = 0;
    let mut a: *mut tran_low_t = 0 as *mut tran_low_t;
    let mut b: *mut tran_low_t = 0 as *mut tran_low_t;
    n = length >> 1 as core::ffi::c_int;
    b = highpass;
    a = lowpass;
    loop {
        n -= 1;
        if !(n != 0) {
            break;
        }
        let fresh7 = x;
        x = x.offset(1);
        r = *fresh7;
        let fresh8 = a;
        a = a.offset(1);
        *fresh8 = r * 2 as tran_low_t;
        let fresh9 = b;
        b = b.offset(1);
        *fresh9 = *x
            - (r + *x.offset(1 as core::ffi::c_int as isize) + 1 as tran_low_t
                >> 1 as core::ffi::c_int);
        x = x.offset(1);
    }
    let fresh10 = x;
    x = x.offset(1);
    r = *fresh10;
    *a = r * 2 as tran_low_t;
    *b = *x - r;
    n = length >> 1 as core::ffi::c_int;
    b = highpass;
    a = lowpass;
    r = *highpass;
    loop {
        let fresh11 = n;
        n = n - 1;
        if !(fresh11 != 0) {
            break;
        }
        let fresh12 = a;
        a = a.offset(1);
        *fresh12 = (*fresh12 as core::ffi::c_int
            + (r + *b + 1 as tran_low_t >> 1 as core::ffi::c_int) as core::ffi::c_int)
            as tran_low_t;
        let fresh13 = b;
        b = b.offset(1);
        r = *fresh13;
    };
}
unsafe extern "C-unwind" fn analysis_53_col(
    mut length: core::ffi::c_int,
    mut x: *mut tran_low_t,
    mut lowpass: *mut tran_low_t,
    mut highpass: *mut tran_low_t,
) {
    let mut n: core::ffi::c_int = 0;
    let mut r: tran_low_t = 0;
    let mut a: *mut tran_low_t = 0 as *mut tran_low_t;
    let mut b: *mut tran_low_t = 0 as *mut tran_low_t;
    n = length >> 1 as core::ffi::c_int;
    b = highpass;
    a = lowpass;
    loop {
        n -= 1;
        if !(n != 0) {
            break;
        }
        let fresh0 = x;
        x = x.offset(1);
        r = *fresh0;
        let fresh1 = a;
        a = a.offset(1);
        *fresh1 = r;
        let fresh2 = b;
        b = b.offset(1);
        *fresh2 = *x * 2 as tran_low_t - (r + *x.offset(1 as core::ffi::c_int as isize))
            + 2 as tran_low_t >> 2 as core::ffi::c_int;
        x = x.offset(1);
    }
    let fresh3 = x;
    x = x.offset(1);
    r = *fresh3;
    *a = r;
    *b = *x - r + 1 as tran_low_t >> 1 as core::ffi::c_int;
    n = length >> 1 as core::ffi::c_int;
    b = highpass;
    a = lowpass;
    r = *highpass;
    loop {
        let fresh4 = n;
        n = n - 1;
        if !(fresh4 != 0) {
            break;
        }
        let fresh5 = a;
        a = a.offset(1);
        *fresh5 = (*fresh5 as core::ffi::c_int
            + (r + *b + 1 as tran_low_t >> 1 as core::ffi::c_int) as core::ffi::c_int)
            as tran_low_t;
        let fresh6 = b;
        b = b.offset(1);
        r = *fresh6;
    };
}
unsafe extern "C-unwind" fn dyadic_analyze_53_uint8_input(
    mut levels: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut x: *const uint8_t,
    mut pitch_x: core::ffi::c_int,
    mut c: *mut tran_low_t,
    mut pitch_c: core::ffi::c_int,
    mut dwt_scale_bits: core::ffi::c_int,
    mut hbd: core::ffi::c_int,
) {
    let mut lv: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut nh: core::ffi::c_int = 0;
    let mut nw: core::ffi::c_int = 0;
    let mut hh: core::ffi::c_int = height;
    let mut hw: core::ffi::c_int = width;
    let mut buffer: [tran_low_t; 128] = [0; 128];
    if hbd != 0 {
        let mut x16: *const uint16_t = ((x as uintptr_t) << 1 as core::ffi::c_int)
            as *mut uint16_t;
        i = 0 as core::ffi::c_int;
        while i < height {
            j = 0 as core::ffi::c_int;
            while j < width {
                *c.offset((i * pitch_c + j) as isize) = ((*x16
                    .offset((i * pitch_x + j) as isize) as core::ffi::c_int)
                    << dwt_scale_bits) as tran_low_t;
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int;
        while i < height {
            j = 0 as core::ffi::c_int;
            while j < width {
                *c.offset((i * pitch_c + j) as isize) = ((*x
                    .offset((i * pitch_x + j) as isize) as core::ffi::c_int)
                    << dwt_scale_bits) as tran_low_t;
                j += 1;
            }
            i += 1;
        }
    }
    lv = 0 as core::ffi::c_int;
    while lv < levels {
        nh = hh;
        hh = hh + 1 as core::ffi::c_int >> 1 as core::ffi::c_int;
        nw = hw;
        hw = hw + 1 as core::ffi::c_int >> 1 as core::ffi::c_int;
        if nh < 2 as core::ffi::c_int || nw < 2 as core::ffi::c_int {
            return;
        }
        i = 0 as core::ffi::c_int;
        while i < nh {
            memcpy(
                buffer.as_mut_ptr() as *mut core::ffi::c_void,
                &raw mut *c.offset((i * pitch_c) as isize) as *mut tran_low_t
                    as *const core::ffi::c_void,
                (nw as size_t)
                    .wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
            );
            analysis_53_row(
                nw,
                buffer.as_mut_ptr(),
                &raw mut *c.offset((i * pitch_c) as isize),
                (&raw mut *c.offset((i * pitch_c) as isize) as *mut tran_low_t)
                    .offset(hw as isize),
            );
            i += 1;
        }
        j = 0 as core::ffi::c_int;
        while j < nw {
            i = 0 as core::ffi::c_int;
            while i < nh {
                buffer[(i + nh) as usize] = *c.offset((i * pitch_c + j) as isize);
                i += 1;
            }
            analysis_53_col(
                nh,
                buffer.as_mut_ptr().offset(nh as isize),
                buffer.as_mut_ptr(),
                buffer.as_mut_ptr().offset(hh as isize),
            );
            i = 0 as core::ffi::c_int;
            while i < nh {
                *c.offset((i * pitch_c + j) as isize) = buffer[i as usize];
                i += 1;
            }
            j += 1;
        }
        lv += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fdwt8x8_uint8_input_c(
    mut input: *const uint8_t,
    mut output: *mut tran_low_t,
    mut stride: core::ffi::c_int,
    mut hbd: core::ffi::c_int,
) {
    dyadic_analyze_53_uint8_input(
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        input,
        stride,
        output,
        8 as core::ffi::c_int,
        2 as core::ffi::c_int,
        hbd,
    );
}
unsafe extern "C-unwind" fn haar_ac_sad(
    mut output: *const tran_low_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut stride: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut acsad: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < bw {
            if r >= bh / 2 as core::ffi::c_int || c >= bw / 2 as core::ffi::c_int {
                acsad += abs(*output.offset((r * stride + c) as isize));
            }
            c += 1;
        }
        r += 1;
    }
    return acsad;
}
unsafe extern "C-unwind" fn haar_ac_sad_8x8_uint8_input(
    mut input: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut hbd: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut output: [tran_low_t; 64] = [0; 64];
    av1_fdwt8x8_uint8_input_c(input, output.as_mut_ptr(), stride, hbd);
    return haar_ac_sad(
        output.as_mut_ptr(),
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_haar_ac_sad_mxn_uint8_input(
    mut input: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut hbd: core::ffi::c_int,
    mut num_8x8_rows: core::ffi::c_int,
    mut num_8x8_cols: core::ffi::c_int,
) -> int64_t {
    let mut wavelet_energy: int64_t = 0 as int64_t;
    let mut r8: core::ffi::c_int = 0 as core::ffi::c_int;
    while r8 < num_8x8_rows {
        let mut c8: core::ffi::c_int = 0 as core::ffi::c_int;
        while c8 < num_8x8_cols {
            wavelet_energy
                += haar_ac_sad_8x8_uint8_input(
                    input
                        .offset((c8 * 8 as core::ffi::c_int) as isize)
                        .offset((r8 * 8 as core::ffi::c_int * stride) as isize),
                    stride,
                    hbd,
                ) as int64_t;
            c8 += 1;
        }
        r8 += 1;
    }
    return wavelet_energy;
}

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
