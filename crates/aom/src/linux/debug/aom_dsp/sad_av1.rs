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

// Mechanically translated from upstream/aom/aom_dsp/sad_av1.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
#[inline]
unsafe extern "C-unwind" fn masked_sad(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut m: *const uint8_t,
    mut m_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad: core::ffi::c_uint = 0 as core::ffi::c_uint;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            let pred: int16_t = (*m.offset(x as isize) as core::ffi::c_int
                * *a.offset(x as isize) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - *m.offset(x as isize) as core::ffi::c_int)
                    * *b.offset(x as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as int16_t;
            sad = sad
                .wrapping_add(
                    abs(
                        pred as core::ffi::c_int
                            - *src.offset(x as isize) as core::ffi::c_int,
                    ) as core::ffi::c_uint,
                );
            x += 1;
        }
        src = src.offset(src_stride as isize);
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        m = m.offset(m_stride as isize);
        y += 1;
    }
    return sad;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            128 as core::ffi::c_int,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            128 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            128 as core::ffi::c_int,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            128 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            64 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            64 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            64 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            32 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            32 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            32 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            16 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            16 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            16 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            8 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            8 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            8 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            4 as core::ffi::c_int,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            4 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            4 as core::ffi::c_int,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            4 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            4 as core::ffi::c_int,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            4 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            16 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            8 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            32 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            16 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sad64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return masked_sad(
            src,
            src_stride,
            ref_0,
            ref_stride,
            second_pred,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return masked_sad(
            src,
            src_stride,
            second_pred,
            64 as core::ffi::c_int,
            ref_0,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[inline]
unsafe extern "C-unwind" fn highbd_masked_sad(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut m: *const uint8_t,
    mut m_stride: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut src: *const uint16_t = ((src8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut a: *const uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *const uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            let pred: uint16_t = (*m.offset(x as isize) as core::ffi::c_int
                * *a.offset(x as isize) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - *m.offset(x as isize) as core::ffi::c_int)
                    * *b.offset(x as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as uint16_t;
            sad = sad
                .wrapping_add(
                    abs(
                        pred as core::ffi::c_int
                            - *src.offset(x as isize) as core::ffi::c_int,
                    ) as core::ffi::c_uint,
                );
            x += 1;
        }
        src = src.offset(src_stride as isize);
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        m = m.offset(m_stride as isize);
        y += 1;
    }
    return sad;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad128x128_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            128 as core::ffi::c_int,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            128 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad128x64_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            128 as core::ffi::c_int,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            128 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            128 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad64x128_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            64 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            128 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad64x64_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            64 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad64x32_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            64 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad32x64_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            32 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad32x32_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            32 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad32x16_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            32 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad16x32_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            16 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad16x16_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            16 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad16x8_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            16 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad8x16_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            8 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad8x8_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            8 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad8x4_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            8 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad4x8_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            4 as core::ffi::c_int,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            4 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad4x4_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            4 as core::ffi::c_int,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            4 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad4x16_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            4 as core::ffi::c_int,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            4 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            4 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad16x4_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            16 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            4 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad8x32_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            8 as core::ffi::c_int,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            8 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            8 as core::ffi::c_int,
            32 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad32x8_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            32 as core::ffi::c_int,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            32 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            32 as core::ffi::c_int,
            8 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad16x64_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            16 as core::ffi::c_int,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            16 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            16 as core::ffi::c_int,
            64 as core::ffi::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_masked_sad64x16_c(
    mut src8: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred8: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) -> core::ffi::c_uint {
    if invert_mask == 0 {
        return highbd_masked_sad(
            src8,
            src_stride,
            ref8,
            ref_stride,
            second_pred8,
            64 as core::ffi::c_int,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    } else {
        return highbd_masked_sad(
            src8,
            src_stride,
            second_pred8,
            64 as core::ffi::c_int,
            ref8,
            ref_stride,
            msk,
            msk_stride,
            64 as core::ffi::c_int,
            16 as core::ffi::c_int,
        )
    };
}
#[inline]
unsafe extern "C-unwind" fn obmc_sad(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad: core::ffi::c_uint = 0 as core::ffi::c_uint;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            sad = sad
                .wrapping_add(
                    (abs(
                        *wsrc.offset(x as isize)
                            - *pre.offset(x as isize) as core::ffi::c_int
                                * *mask.offset(x as isize),
                    )
                        + ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 12 as core::ffi::c_int)
                        as core::ffi::c_uint,
                );
            x += 1;
        }
        pre = pre.offset(pre_stride as isize);
        wsrc = wsrc.offset(width as isize);
        mask = mask.offset(width as isize);
        y += 1;
    }
    return sad;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad128x128_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad128x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad64x128_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad64x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad64x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad32x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad32x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad32x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad16x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad16x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad16x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad8x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad8x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad8x4_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad4x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad4x4_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad4x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad16x4_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad8x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad32x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad16x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sad64x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn highbd_obmc_sad(
    mut pre8: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_uint {
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut sad: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut pre: *const uint16_t = ((pre8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    y = 0 as core::ffi::c_int;
    while y < height {
        x = 0 as core::ffi::c_int;
        while x < width {
            sad = sad
                .wrapping_add(
                    (abs(
                        *wsrc.offset(x as isize)
                            - *pre.offset(x as isize) as core::ffi::c_int
                                * *mask.offset(x as isize),
                    )
                        + ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 12 as core::ffi::c_int)
                        as core::ffi::c_uint,
                );
            x += 1;
        }
        pre = pre.offset(pre_stride as isize);
        wsrc = wsrc.offset(width as isize);
        mask = mask.offset(width as isize);
        y += 1;
    }
    return sad;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad128x128_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad128x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad64x128_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad64x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad64x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad32x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad32x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad32x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad16x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad16x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad16x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad8x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad8x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad8x4_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad4x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad4x4_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad4x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad16x4_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad8x32_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad32x8_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad16x64_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_obmc_sad64x16_c(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
) -> core::ffi::c_uint {
    return highbd_obmc_sad(
        ref_0,
        ref_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
    );
}

extern "C" {
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
}
