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

// Mechanically translated from upstream/aom/aom_dsp/avg.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type ptrdiff_t = isize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
pub type tran_low_t = int32_t;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_minmax_8x8_c(
    mut s: *const uint8_t,
    mut p: core::ffi::c_int,
    mut d: *const uint8_t,
    mut dp: core::ffi::c_int,
    mut min: *mut core::ffi::c_int,
    mut max: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    *min = 255 as core::ffi::c_int;
    *max = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            let mut diff: core::ffi::c_int = abs(
                *s.offset(j as isize) as core::ffi::c_int
                    - *d.offset(j as isize) as core::ffi::c_int,
            );
            *min = if diff < *min { diff } else { *min };
            *max = if diff > *max { diff } else { *max };
            j += 1;
        }
        i += 1;
        s = s.offset(p as isize);
        d = d.offset(dp as isize);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_avg_4x4_c(
    mut s: *const uint8_t,
    mut p: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            sum += *s.offset(j as isize) as core::ffi::c_int;
            j += 1;
        }
        i += 1;
        s = s.offset(p as isize);
    }
    return (sum + 8 as core::ffi::c_int >> 4 as core::ffi::c_int) as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_avg_8x8_c(
    mut s: *const uint8_t,
    mut p: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            sum += *s.offset(j as isize) as core::ffi::c_int;
            j += 1;
        }
        i += 1;
        s = s.offset(p as isize);
    }
    return (sum + 32 as core::ffi::c_int >> 6 as core::ffi::c_int) as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_avg_8x8_quad_c(
    mut s: *const uint8_t,
    mut p: core::ffi::c_int,
    mut x16_idx: core::ffi::c_int,
    mut y16_idx: core::ffi::c_int,
    mut avg: *mut core::ffi::c_int,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < 4 as core::ffi::c_int {
        let x8_idx: core::ffi::c_int = x16_idx
            + ((k & 1 as core::ffi::c_int) << 3 as core::ffi::c_int);
        let y8_idx: core::ffi::c_int = y16_idx
            + ((k >> 1 as core::ffi::c_int) << 3 as core::ffi::c_int);
        let mut s_tmp: *const uint8_t = s
            .offset((y8_idx * p) as isize)
            .offset(x8_idx as isize);
        *avg.offset(k as isize) = aom_avg_8x8_c(s_tmp, p) as core::ffi::c_int;
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_avg_8x8_c(
    mut s8: *const uint8_t,
    mut p: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut s: *const uint16_t = ((s8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            sum += *s.offset(j as isize) as core::ffi::c_int;
            j += 1;
        }
        i += 1;
        s = s.offset(p as isize);
    }
    return (sum + 32 as core::ffi::c_int >> 6 as core::ffi::c_int) as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_avg_4x4_c(
    mut s8: *const uint8_t,
    mut p: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut s: *const uint16_t = ((s8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            sum += *s.offset(j as isize) as core::ffi::c_int;
            j += 1;
        }
        i += 1;
        s = s.offset(p as isize);
    }
    return (sum + 8 as core::ffi::c_int >> 4 as core::ffi::c_int) as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_minmax_8x8_c(
    mut s8: *const uint8_t,
    mut p: core::ffi::c_int,
    mut d8: *const uint8_t,
    mut dp: core::ffi::c_int,
    mut min: *mut core::ffi::c_int,
    mut max: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut s: *const uint16_t = ((s8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut d: *const uint16_t = ((d8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    *min = 65535 as core::ffi::c_int;
    *max = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            let mut diff: core::ffi::c_int = abs(
                *s.offset(j as isize) as core::ffi::c_int
                    - *d.offset(j as isize) as core::ffi::c_int,
            );
            *min = if diff < *min { diff } else { *min };
            *max = if diff > *max { diff } else { *max };
            j += 1;
        }
        i += 1;
        s = s.offset(p as isize);
        d = d.offset(dp as isize);
    }
}
unsafe extern "C-unwind" fn hadamard_col4(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int16_t,
) {
    let mut b0: int16_t = (*src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int
        >> 1 as core::ffi::c_int) as int16_t;
    let mut b1: int16_t = (*src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int
        >> 1 as core::ffi::c_int) as int16_t;
    let mut b2: int16_t = (*src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int
        >> 1 as core::ffi::c_int) as int16_t;
    let mut b3: int16_t = (*src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int
        >> 1 as core::ffi::c_int) as int16_t;
    *coeff.offset(0 as core::ffi::c_int as isize) = (b0 as core::ffi::c_int
        + b2 as core::ffi::c_int) as int16_t;
    *coeff.offset(1 as core::ffi::c_int as isize) = (b1 as core::ffi::c_int
        + b3 as core::ffi::c_int) as int16_t;
    *coeff.offset(2 as core::ffi::c_int as isize) = (b0 as core::ffi::c_int
        - b2 as core::ffi::c_int) as int16_t;
    *coeff.offset(3 as core::ffi::c_int as isize) = (b1 as core::ffi::c_int
        - b3 as core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_4x4_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    let mut buffer: [int16_t; 16] = [0; 16];
    let mut buffer2: [int16_t; 16] = [0; 16];
    let mut tmp_buf: *mut int16_t = &raw mut *buffer
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut int16_t;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        hadamard_col4(src_diff, src_stride, tmp_buf);
        tmp_buf = tmp_buf.offset(4 as core::ffi::c_int as isize);
        src_diff = src_diff.offset(1);
        idx += 1;
    }
    tmp_buf = &raw mut *buffer.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut int16_t;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        hadamard_col4(
            tmp_buf,
            4 as ptrdiff_t,
            buffer2.as_mut_ptr().offset((4 as core::ffi::c_int * idx) as isize),
        );
        tmp_buf = tmp_buf.offset(1);
        idx += 1;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            *coeff.offset((i * 4 as core::ffi::c_int + j) as isize) = buffer2[(j
                * 4 as core::ffi::c_int + i) as usize] as tran_low_t;
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn hadamard_col8(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int16_t,
) {
    let mut b0: int16_t = (*src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b1: int16_t = (*src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b2: int16_t = (*src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b3: int16_t = (*src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b4: int16_t = (*src_diff.offset((4 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((5 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b5: int16_t = (*src_diff.offset((4 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((5 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b6: int16_t = (*src_diff.offset((6 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((7 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b7: int16_t = (*src_diff.offset((6 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((7 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut c0: int16_t = (b0 as core::ffi::c_int + b2 as core::ffi::c_int) as int16_t;
    let mut c1: int16_t = (b1 as core::ffi::c_int + b3 as core::ffi::c_int) as int16_t;
    let mut c2: int16_t = (b0 as core::ffi::c_int - b2 as core::ffi::c_int) as int16_t;
    let mut c3: int16_t = (b1 as core::ffi::c_int - b3 as core::ffi::c_int) as int16_t;
    let mut c4: int16_t = (b4 as core::ffi::c_int + b6 as core::ffi::c_int) as int16_t;
    let mut c5: int16_t = (b5 as core::ffi::c_int + b7 as core::ffi::c_int) as int16_t;
    let mut c6: int16_t = (b4 as core::ffi::c_int - b6 as core::ffi::c_int) as int16_t;
    let mut c7: int16_t = (b5 as core::ffi::c_int - b7 as core::ffi::c_int) as int16_t;
    *coeff.offset(0 as core::ffi::c_int as isize) = (c0 as core::ffi::c_int
        + c4 as core::ffi::c_int) as int16_t;
    *coeff.offset(7 as core::ffi::c_int as isize) = (c1 as core::ffi::c_int
        + c5 as core::ffi::c_int) as int16_t;
    *coeff.offset(3 as core::ffi::c_int as isize) = (c2 as core::ffi::c_int
        + c6 as core::ffi::c_int) as int16_t;
    *coeff.offset(4 as core::ffi::c_int as isize) = (c3 as core::ffi::c_int
        + c7 as core::ffi::c_int) as int16_t;
    *coeff.offset(2 as core::ffi::c_int as isize) = (c0 as core::ffi::c_int
        - c4 as core::ffi::c_int) as int16_t;
    *coeff.offset(6 as core::ffi::c_int as isize) = (c1 as core::ffi::c_int
        - c5 as core::ffi::c_int) as int16_t;
    *coeff.offset(1 as core::ffi::c_int as isize) = (c2 as core::ffi::c_int
        - c6 as core::ffi::c_int) as int16_t;
    *coeff.offset(5 as core::ffi::c_int as isize) = (c3 as core::ffi::c_int
        - c7 as core::ffi::c_int) as int16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_8x8_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    let mut buffer: [int16_t; 64] = [0; 64];
    let mut buffer2: [int16_t; 64] = [0; 64];
    let mut tmp_buf: *mut int16_t = &raw mut *buffer
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut int16_t;
    idx = 0 as core::ffi::c_int;
    while idx < 8 as core::ffi::c_int {
        hadamard_col8(src_diff, src_stride, tmp_buf);
        tmp_buf = tmp_buf.offset(8 as core::ffi::c_int as isize);
        src_diff = src_diff.offset(1);
        idx += 1;
    }
    tmp_buf = &raw mut *buffer.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut int16_t;
    idx = 0 as core::ffi::c_int;
    while idx < 8 as core::ffi::c_int {
        hadamard_col8(
            tmp_buf,
            8 as ptrdiff_t,
            buffer2.as_mut_ptr().offset((8 as core::ffi::c_int * idx) as isize),
        );
        tmp_buf = tmp_buf.offset(1);
        idx += 1;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            *coeff.offset((i * 8 as core::ffi::c_int + j) as isize) = buffer2[(j
                * 8 as core::ffi::c_int + i) as usize] as tran_low_t;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int16_t,
) {
    let mut buffer: [int16_t; 64] = [0; 64];
    let mut buffer2: [int16_t; 64] = [0; 64];
    let mut tmp_buf: *mut int16_t = &raw mut *buffer
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut int16_t;
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < 8 as core::ffi::c_int {
        hadamard_col8(src_diff, src_stride, tmp_buf);
        tmp_buf = tmp_buf.offset(8 as core::ffi::c_int as isize);
        src_diff = src_diff.offset(1);
        idx += 1;
    }
    tmp_buf = &raw mut *buffer.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut int16_t;
    let mut idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx_0 < 8 as core::ffi::c_int {
        hadamard_col8(
            tmp_buf,
            8 as ptrdiff_t,
            buffer2.as_mut_ptr().offset((8 as core::ffi::c_int * idx_0) as isize),
        );
        tmp_buf = tmp_buf.offset(1);
        idx_0 += 1;
    }
    let mut idx_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx_1 < 64 as core::ffi::c_int {
        *coeff.offset(idx_1 as isize) = buffer2[idx_1 as usize];
        idx_1 += 1;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            *coeff.offset((i * 8 as core::ffi::c_int + j) as isize) = buffer2[(j
                * 8 as core::ffi::c_int + i) as usize];
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_8x8_dual_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int16_t,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        aom_hadamard_lp_8x8_c(
            src_diff.offset((i * 8 as core::ffi::c_int) as isize),
            src_stride,
            coeff.offset((i * 64 as core::ffi::c_int) as isize),
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_16x16_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        let mut src_ptr: *const int16_t = src_diff
            .offset(
                (((idx >> 1 as core::ffi::c_int) * 8 as core::ffi::c_int) as ptrdiff_t
                    * src_stride) as isize,
            )
            .offset(((idx & 0x1 as core::ffi::c_int) * 8 as core::ffi::c_int) as isize);
        aom_hadamard_8x8_c(
            src_ptr,
            src_stride,
            coeff.offset((idx * 64 as core::ffi::c_int) as isize),
        );
        idx += 1;
    }
    idx = 0 as core::ffi::c_int;
    while idx < 64 as core::ffi::c_int {
        let mut a0: tran_low_t = *coeff.offset(0 as core::ffi::c_int as isize);
        let mut a1: tran_low_t = *coeff.offset(64 as core::ffi::c_int as isize);
        let mut a2: tran_low_t = *coeff.offset(128 as core::ffi::c_int as isize);
        let mut a3: tran_low_t = *coeff.offset(192 as core::ffi::c_int as isize);
        let mut b0: tran_low_t = a0 + a1 >> 1 as core::ffi::c_int;
        let mut b1: tran_low_t = a0 - a1 >> 1 as core::ffi::c_int;
        let mut b2: tran_low_t = a2 + a3 >> 1 as core::ffi::c_int;
        let mut b3: tran_low_t = a2 - a3 >> 1 as core::ffi::c_int;
        *coeff.offset(0 as core::ffi::c_int as isize) = b0 + b2;
        *coeff.offset(64 as core::ffi::c_int as isize) = b1 + b3;
        *coeff.offset(128 as core::ffi::c_int as isize) = b0 - b2;
        *coeff.offset(192 as core::ffi::c_int as isize) = b1 - b3;
        coeff = coeff.offset(1);
        idx += 1;
    }
    coeff = coeff.offset(-(64 as core::ffi::c_int as isize));
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 16 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < 4 as core::ffi::c_int {
            let mut temp: tran_low_t = *coeff
                .offset(
                    (i * 16 as core::ffi::c_int + 4 as core::ffi::c_int + j) as isize,
                );
            *coeff
                .offset(
                    (i * 16 as core::ffi::c_int + 4 as core::ffi::c_int + j) as isize,
                ) = *coeff
                .offset(
                    (i * 16 as core::ffi::c_int + 8 as core::ffi::c_int + j) as isize,
                );
            *coeff
                .offset(
                    (i * 16 as core::ffi::c_int + 8 as core::ffi::c_int + j) as isize,
                ) = temp;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_lp_16x16_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int16_t,
) {
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        let mut src_ptr: *const int16_t = src_diff
            .offset(
                (((idx >> 1 as core::ffi::c_int) * 8 as core::ffi::c_int) as ptrdiff_t
                    * src_stride) as isize,
            )
            .offset(((idx & 0x1 as core::ffi::c_int) * 8 as core::ffi::c_int) as isize);
        aom_hadamard_lp_8x8_c(
            src_ptr,
            src_stride,
            coeff.offset((idx * 64 as core::ffi::c_int) as isize),
        );
        idx += 1;
    }
    let mut idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx_0 < 64 as core::ffi::c_int {
        let mut a0: int16_t = *coeff.offset(0 as core::ffi::c_int as isize);
        let mut a1: int16_t = *coeff.offset(64 as core::ffi::c_int as isize);
        let mut a2: int16_t = *coeff.offset(128 as core::ffi::c_int as isize);
        let mut a3: int16_t = *coeff.offset(192 as core::ffi::c_int as isize);
        let mut b0: int16_t = (a0 as core::ffi::c_int + a1 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as int16_t;
        let mut b1: int16_t = (a0 as core::ffi::c_int - a1 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as int16_t;
        let mut b2: int16_t = (a2 as core::ffi::c_int + a3 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as int16_t;
        let mut b3: int16_t = (a2 as core::ffi::c_int - a3 as core::ffi::c_int
            >> 1 as core::ffi::c_int) as int16_t;
        *coeff.offset(0 as core::ffi::c_int as isize) = (b0 as core::ffi::c_int
            + b2 as core::ffi::c_int) as int16_t;
        *coeff.offset(64 as core::ffi::c_int as isize) = (b1 as core::ffi::c_int
            + b3 as core::ffi::c_int) as int16_t;
        *coeff.offset(128 as core::ffi::c_int as isize) = (b0 as core::ffi::c_int
            - b2 as core::ffi::c_int) as int16_t;
        *coeff.offset(192 as core::ffi::c_int as isize) = (b1 as core::ffi::c_int
            - b3 as core::ffi::c_int) as int16_t;
        coeff = coeff.offset(1);
        idx_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_hadamard_32x32_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        let mut src_ptr: *const int16_t = src_diff
            .offset(
                (((idx >> 1 as core::ffi::c_int) * 16 as core::ffi::c_int) as ptrdiff_t
                    * src_stride) as isize,
            )
            .offset(((idx & 0x1 as core::ffi::c_int) * 16 as core::ffi::c_int) as isize);
        aom_hadamard_16x16_c(
            src_ptr,
            src_stride,
            coeff.offset((idx * 256 as core::ffi::c_int) as isize),
        );
        idx += 1;
    }
    idx = 0 as core::ffi::c_int;
    while idx < 256 as core::ffi::c_int {
        let mut a0: tran_low_t = *coeff.offset(0 as core::ffi::c_int as isize);
        let mut a1: tran_low_t = *coeff.offset(256 as core::ffi::c_int as isize);
        let mut a2: tran_low_t = *coeff.offset(512 as core::ffi::c_int as isize);
        let mut a3: tran_low_t = *coeff.offset(768 as core::ffi::c_int as isize);
        let mut b0: tran_low_t = a0 + a1 >> 2 as core::ffi::c_int;
        let mut b1: tran_low_t = a0 - a1 >> 2 as core::ffi::c_int;
        let mut b2: tran_low_t = a2 + a3 >> 2 as core::ffi::c_int;
        let mut b3: tran_low_t = a2 - a3 >> 2 as core::ffi::c_int;
        *coeff.offset(0 as core::ffi::c_int as isize) = b0 + b2;
        *coeff.offset(256 as core::ffi::c_int as isize) = b1 + b3;
        *coeff.offset(512 as core::ffi::c_int as isize) = b0 - b2;
        *coeff.offset(768 as core::ffi::c_int as isize) = b1 - b3;
        coeff = coeff.offset(1);
        idx += 1;
    }
}
unsafe extern "C-unwind" fn hadamard_highbd_col8_first_pass(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int16_t,
) {
    let mut b0: int16_t = (*src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b1: int16_t = (*src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b2: int16_t = (*src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b3: int16_t = (*src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b4: int16_t = (*src_diff.offset((4 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((5 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b5: int16_t = (*src_diff.offset((4 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((5 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b6: int16_t = (*src_diff.offset((6 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        + *src_diff.offset((7 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut b7: int16_t = (*src_diff.offset((6 as ptrdiff_t * src_stride) as isize)
        as core::ffi::c_int
        - *src_diff.offset((7 as ptrdiff_t * src_stride) as isize) as core::ffi::c_int)
        as int16_t;
    let mut c0: int16_t = (b0 as core::ffi::c_int + b2 as core::ffi::c_int) as int16_t;
    let mut c1: int16_t = (b1 as core::ffi::c_int + b3 as core::ffi::c_int) as int16_t;
    let mut c2: int16_t = (b0 as core::ffi::c_int - b2 as core::ffi::c_int) as int16_t;
    let mut c3: int16_t = (b1 as core::ffi::c_int - b3 as core::ffi::c_int) as int16_t;
    let mut c4: int16_t = (b4 as core::ffi::c_int + b6 as core::ffi::c_int) as int16_t;
    let mut c5: int16_t = (b5 as core::ffi::c_int + b7 as core::ffi::c_int) as int16_t;
    let mut c6: int16_t = (b4 as core::ffi::c_int - b6 as core::ffi::c_int) as int16_t;
    let mut c7: int16_t = (b5 as core::ffi::c_int - b7 as core::ffi::c_int) as int16_t;
    *coeff.offset(0 as core::ffi::c_int as isize) = (c0 as core::ffi::c_int
        + c4 as core::ffi::c_int) as int16_t;
    *coeff.offset(7 as core::ffi::c_int as isize) = (c1 as core::ffi::c_int
        + c5 as core::ffi::c_int) as int16_t;
    *coeff.offset(3 as core::ffi::c_int as isize) = (c2 as core::ffi::c_int
        + c6 as core::ffi::c_int) as int16_t;
    *coeff.offset(4 as core::ffi::c_int as isize) = (c3 as core::ffi::c_int
        + c7 as core::ffi::c_int) as int16_t;
    *coeff.offset(2 as core::ffi::c_int as isize) = (c0 as core::ffi::c_int
        - c4 as core::ffi::c_int) as int16_t;
    *coeff.offset(6 as core::ffi::c_int as isize) = (c1 as core::ffi::c_int
        - c5 as core::ffi::c_int) as int16_t;
    *coeff.offset(1 as core::ffi::c_int as isize) = (c2 as core::ffi::c_int
        - c6 as core::ffi::c_int) as int16_t;
    *coeff.offset(5 as core::ffi::c_int as isize) = (c3 as core::ffi::c_int
        - c7 as core::ffi::c_int) as int16_t;
}
unsafe extern "C-unwind" fn hadamard_highbd_col8_second_pass(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut int32_t,
) {
    let mut b0: int32_t = *src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as int32_t + *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b1: int32_t = *src_diff.offset((0 as ptrdiff_t * src_stride) as isize)
        as int32_t - *src_diff.offset((1 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b2: int32_t = *src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as int32_t + *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b3: int32_t = *src_diff.offset((2 as ptrdiff_t * src_stride) as isize)
        as int32_t - *src_diff.offset((3 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b4: int32_t = *src_diff.offset((4 as ptrdiff_t * src_stride) as isize)
        as int32_t + *src_diff.offset((5 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b5: int32_t = *src_diff.offset((4 as ptrdiff_t * src_stride) as isize)
        as int32_t - *src_diff.offset((5 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b6: int32_t = *src_diff.offset((6 as ptrdiff_t * src_stride) as isize)
        as int32_t + *src_diff.offset((7 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut b7: int32_t = *src_diff.offset((6 as ptrdiff_t * src_stride) as isize)
        as int32_t - *src_diff.offset((7 as ptrdiff_t * src_stride) as isize) as int32_t;
    let mut c0: int32_t = b0 + b2;
    let mut c1: int32_t = b1 + b3;
    let mut c2: int32_t = b0 - b2;
    let mut c3: int32_t = b1 - b3;
    let mut c4: int32_t = b4 + b6;
    let mut c5: int32_t = b5 + b7;
    let mut c6: int32_t = b4 - b6;
    let mut c7: int32_t = b5 - b7;
    *coeff.offset(0 as core::ffi::c_int as isize) = c0 + c4;
    *coeff.offset(7 as core::ffi::c_int as isize) = c1 + c5;
    *coeff.offset(3 as core::ffi::c_int as isize) = c2 + c6;
    *coeff.offset(4 as core::ffi::c_int as isize) = c3 + c7;
    *coeff.offset(2 as core::ffi::c_int as isize) = c0 - c4;
    *coeff.offset(6 as core::ffi::c_int as isize) = c1 - c5;
    *coeff.offset(1 as core::ffi::c_int as isize) = c2 - c6;
    *coeff.offset(5 as core::ffi::c_int as isize) = c3 - c7;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_8x8_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    let mut buffer: [int16_t; 64] = [0; 64];
    let mut buffer2: [int32_t; 64] = [0; 64];
    let mut tmp_buf: *mut int16_t = &raw mut *buffer
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut int16_t;
    idx = 0 as core::ffi::c_int;
    while idx < 8 as core::ffi::c_int {
        hadamard_highbd_col8_first_pass(src_diff, src_stride, tmp_buf);
        tmp_buf = tmp_buf.offset(8 as core::ffi::c_int as isize);
        src_diff = src_diff.offset(1);
        idx += 1;
    }
    tmp_buf = &raw mut *buffer.as_mut_ptr().offset(0 as core::ffi::c_int as isize)
        as *mut int16_t;
    idx = 0 as core::ffi::c_int;
    while idx < 8 as core::ffi::c_int {
        hadamard_highbd_col8_second_pass(
            tmp_buf,
            8 as ptrdiff_t,
            buffer2.as_mut_ptr().offset((8 as core::ffi::c_int * idx) as isize),
        );
        tmp_buf = tmp_buf.offset(1);
        idx += 1;
    }
    idx = 0 as core::ffi::c_int;
    while idx < 64 as core::ffi::c_int {
        *coeff.offset(idx as isize) = buffer2[idx as usize];
        idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_16x16_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        let mut src_ptr: *const int16_t = src_diff
            .offset(
                (((idx >> 1 as core::ffi::c_int) * 8 as core::ffi::c_int) as ptrdiff_t
                    * src_stride) as isize,
            )
            .offset(((idx & 0x1 as core::ffi::c_int) * 8 as core::ffi::c_int) as isize);
        aom_highbd_hadamard_8x8_c(
            src_ptr,
            src_stride,
            coeff.offset((idx * 64 as core::ffi::c_int) as isize),
        );
        idx += 1;
    }
    idx = 0 as core::ffi::c_int;
    while idx < 64 as core::ffi::c_int {
        let mut a0: tran_low_t = *coeff.offset(0 as core::ffi::c_int as isize);
        let mut a1: tran_low_t = *coeff.offset(64 as core::ffi::c_int as isize);
        let mut a2: tran_low_t = *coeff.offset(128 as core::ffi::c_int as isize);
        let mut a3: tran_low_t = *coeff.offset(192 as core::ffi::c_int as isize);
        let mut b0: tran_low_t = a0 + a1 >> 1 as core::ffi::c_int;
        let mut b1: tran_low_t = a0 - a1 >> 1 as core::ffi::c_int;
        let mut b2: tran_low_t = a2 + a3 >> 1 as core::ffi::c_int;
        let mut b3: tran_low_t = a2 - a3 >> 1 as core::ffi::c_int;
        *coeff.offset(0 as core::ffi::c_int as isize) = b0 + b2;
        *coeff.offset(64 as core::ffi::c_int as isize) = b1 + b3;
        *coeff.offset(128 as core::ffi::c_int as isize) = b0 - b2;
        *coeff.offset(192 as core::ffi::c_int as isize) = b1 - b3;
        coeff = coeff.offset(1);
        idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_hadamard_32x32_c(
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    let mut idx: core::ffi::c_int = 0;
    idx = 0 as core::ffi::c_int;
    while idx < 4 as core::ffi::c_int {
        let mut src_ptr: *const int16_t = src_diff
            .offset(
                (((idx >> 1 as core::ffi::c_int) * 16 as core::ffi::c_int) as ptrdiff_t
                    * src_stride) as isize,
            )
            .offset(((idx & 0x1 as core::ffi::c_int) * 16 as core::ffi::c_int) as isize);
        aom_highbd_hadamard_16x16_c(
            src_ptr,
            src_stride,
            coeff.offset((idx * 256 as core::ffi::c_int) as isize),
        );
        idx += 1;
    }
    idx = 0 as core::ffi::c_int;
    while idx < 256 as core::ffi::c_int {
        let mut a0: tran_low_t = *coeff.offset(0 as core::ffi::c_int as isize);
        let mut a1: tran_low_t = *coeff.offset(256 as core::ffi::c_int as isize);
        let mut a2: tran_low_t = *coeff.offset(512 as core::ffi::c_int as isize);
        let mut a3: tran_low_t = *coeff.offset(768 as core::ffi::c_int as isize);
        let mut b0: tran_low_t = a0 + a1 >> 2 as core::ffi::c_int;
        let mut b1: tran_low_t = a0 - a1 >> 2 as core::ffi::c_int;
        let mut b2: tran_low_t = a2 + a3 >> 2 as core::ffi::c_int;
        let mut b3: tran_low_t = a2 - a3 >> 2 as core::ffi::c_int;
        *coeff.offset(0 as core::ffi::c_int as isize) = b0 + b2;
        *coeff.offset(256 as core::ffi::c_int as isize) = b1 + b3;
        *coeff.offset(512 as core::ffi::c_int as isize) = b0 - b2;
        *coeff.offset(768 as core::ffi::c_int as isize) = b1 - b3;
        coeff = coeff.offset(1);
        idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_satd_c(
    mut coeff: *const tran_low_t,
    mut length: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut satd: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < length {
        satd += abs(*coeff.offset(i as isize));
        i += 1;
    }
    return satd;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_satd_lp_c(
    mut coeff: *const int16_t,
    mut length: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut satd: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < length {
        satd += abs(*coeff.offset(i as isize) as core::ffi::c_int);
        i += 1;
    }
    return satd;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_int_pro_row_c(
    mut hbuf: *mut int16_t,
    mut ref_0: *const uint8_t,
    ref_stride: core::ffi::c_int,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut norm_factor: core::ffi::c_int,
) {
    if height >= 2 as core::ffi::c_int {} else {
        __assert_fail(
            b"height >= 2\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_dsp/avg.c\0" as *const u8 as *const core::ffi::c_char,
            532 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 89],
                [core::ffi::c_char; 89],
            >(
                *b"void aom_int_pro_row_c(int16_t *, const uint8_t *, const int, const int, const int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < width {
        *hbuf.offset(idx as isize) = 0 as int16_t;
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < height {
            let fresh0 = &raw mut *hbuf.offset(idx as isize);
            *fresh0 = (*fresh0 as core::ffi::c_int
                + *ref_0.offset((i * ref_stride) as isize) as core::ffi::c_int)
                as int16_t;
            i += 1;
        }
        let fresh1 = &raw mut *hbuf.offset(idx as isize);
        *fresh1 = (*fresh1 as core::ffi::c_int >> norm_factor) as int16_t;
        ref_0 = ref_0.offset(1);
        idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_int_pro_col_c(
    mut vbuf: *mut int16_t,
    mut ref_0: *const uint8_t,
    ref_stride: core::ffi::c_int,
    width: core::ffi::c_int,
    height: core::ffi::c_int,
    mut norm_factor: core::ffi::c_int,
) {
    let mut ht: core::ffi::c_int = 0 as core::ffi::c_int;
    while ht < height {
        let mut sum: int16_t = 0 as int16_t;
        let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx < width {
            sum = (sum as core::ffi::c_int
                + *ref_0.offset(idx as isize) as core::ffi::c_int) as int16_t;
            idx += 1;
        }
        *vbuf.offset(ht as isize) = (sum as core::ffi::c_int >> norm_factor) as int16_t;
        ref_0 = ref_0.offset(ref_stride as isize);
        ht += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_vector_var_c(
    mut ref_0: *const int16_t,
    mut src: *const int16_t,
    mut bwl: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut width: core::ffi::c_int = (4 as core::ffi::c_int) << bwl;
    let mut sse: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut mean: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut var: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < width {
        let mut diff: core::ffi::c_int = *ref_0.offset(i as isize) as core::ffi::c_int
            - *src.offset(i as isize) as core::ffi::c_int;
        mean += diff;
        sse += diff * diff;
        i += 1;
    }
    let mean_abs: core::ffi::c_uint = abs(mean) as core::ffi::c_uint;
    var = (sse as core::ffi::c_uint)
        .wrapping_sub(mean_abs.wrapping_mul(mean_abs) >> bwl + 2 as core::ffi::c_int)
        as core::ffi::c_int;
    return var;
}

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
}
