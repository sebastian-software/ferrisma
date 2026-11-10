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

// Mechanically translated from upstream/aom/aom_dsp/variance.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uintptr_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
static mut bilinear_filters_2t: [[uint8_t; 2]; 8] = [
    [128 as core::ffi::c_int as uint8_t, 0 as core::ffi::c_int as uint8_t],
    [112 as core::ffi::c_int as uint8_t, 16 as core::ffi::c_int as uint8_t],
    [96 as core::ffi::c_int as uint8_t, 32 as core::ffi::c_int as uint8_t],
    [80 as core::ffi::c_int as uint8_t, 48 as core::ffi::c_int as uint8_t],
    [64 as core::ffi::c_int as uint8_t, 64 as core::ffi::c_int as uint8_t],
    [48 as core::ffi::c_int as uint8_t, 80 as core::ffi::c_int as uint8_t],
    [32 as core::ffi::c_int as uint8_t, 96 as core::ffi::c_int as uint8_t],
    [16 as core::ffi::c_int as uint8_t, 112 as core::ffi::c_int as uint8_t],
];
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_mb_ss_c(mut a: *const int16_t) -> core::ffi::c_uint {
    let mut i: core::ffi::c_uint = 0;
    let mut sum: core::ffi::c_uint = 0 as core::ffi::c_uint;
    i = 0 as core::ffi::c_uint;
    while i < 256 as core::ffi::c_uint {
        sum = sum
            .wrapping_add(
                (*a.offset(i as isize) as core::ffi::c_int
                    * *a.offset(i as isize) as core::ffi::c_int) as core::ffi::c_uint,
            );
        i = i.wrapping_add(1);
    }
    return sum;
}
unsafe extern "C-unwind" fn variance(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut sum: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut tsum: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tsse: uint32_t = 0 as uint32_t;
    i = 0 as core::ffi::c_int;
    while i < h {
        j = 0 as core::ffi::c_int;
        while j < w {
            let diff: core::ffi::c_int = *a.offset(j as isize) as core::ffi::c_int
                - *b.offset(j as isize) as core::ffi::c_int;
            tsum += diff;
            tsse = tsse.wrapping_add((diff * diff) as uint32_t);
            j += 1;
        }
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        i += 1;
    }
    *sum = tsum;
    *sse = tsse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sse_odd_size(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> uint32_t {
    let mut sse: uint32_t = 0;
    let mut sum: core::ffi::c_int = 0;
    variance(a, a_stride, b, b_stride, w, h, &raw mut sse, &raw mut sum);
    return sse;
}
unsafe extern "C-unwind" fn var_filter_block2d_bil_first_pass_c(
    mut a: *const uint8_t,
    mut b: *mut uint16_t,
    mut src_pixels_per_line: core::ffi::c_uint,
    mut pixel_step: core::ffi::c_uint,
    mut output_height: core::ffi::c_uint,
    mut output_width: core::ffi::c_uint,
    mut filter: *const uint8_t,
) {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    i = 0 as core::ffi::c_uint;
    while i < output_height {
        j = 0 as core::ffi::c_uint;
        while j < output_width {
            *b.offset(j as isize) = (*a.offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int
                * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *a.offset(pixel_step as isize) as core::ffi::c_int
                    * *filter.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int) as uint16_t;
            a = a.offset(1);
            j = j.wrapping_add(1);
        }
        a = a.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
        b = b.offset(output_width as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn var_filter_block2d_bil_second_pass_c(
    mut a: *const uint16_t,
    mut b: *mut uint8_t,
    mut src_pixels_per_line: core::ffi::c_uint,
    mut pixel_step: core::ffi::c_uint,
    mut output_height: core::ffi::c_uint,
    mut output_width: core::ffi::c_uint,
    mut filter: *const uint8_t,
) {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    i = 0 as core::ffi::c_uint;
    while i < output_height {
        j = 0 as core::ffi::c_uint;
        while j < output_width {
            *b.offset(j as isize) = (*a.offset(0 as core::ffi::c_int as isize)
                as core::ffi::c_int
                * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *a.offset(pixel_step as isize) as core::ffi::c_int
                    * *filter.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int) as uint8_t;
            a = a.offset(1);
            j = j.wrapping_add(1);
        }
        a = a.offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
        b = b.offset(output_width as isize);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_8x8_quad_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse8x8: *mut uint32_t,
    mut sum8x8: *mut core::ffi::c_int,
    mut tot_sse: *mut core::ffi::c_uint,
    mut tot_sum: *mut core::ffi::c_int,
    mut var8x8: *mut uint32_t,
) {
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < 4 as core::ffi::c_int {
        variance(
            a.offset((k * 8 as core::ffi::c_int) as isize),
            a_stride,
            b.offset((k * 8 as core::ffi::c_int) as isize),
            b_stride,
            8 as core::ffi::c_int,
            8 as core::ffi::c_int,
            &raw mut *sse8x8.offset(k as isize),
            &raw mut *sum8x8.offset(k as isize),
        );
        k += 1;
    }
    *tot_sse = (*tot_sse)
        .wrapping_add(
            (*sse8x8.offset(0 as core::ffi::c_int as isize))
                .wrapping_add(*sse8x8.offset(1 as core::ffi::c_int as isize))
                .wrapping_add(*sse8x8.offset(2 as core::ffi::c_int as isize))
                .wrapping_add(*sse8x8.offset(3 as core::ffi::c_int as isize))
                as core::ffi::c_uint,
        );
    *tot_sum
        += *sum8x8.offset(0 as core::ffi::c_int as isize)
            + *sum8x8.offset(1 as core::ffi::c_int as isize)
            + *sum8x8.offset(2 as core::ffi::c_int as isize)
            + *sum8x8.offset(3 as core::ffi::c_int as isize);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        *var8x8.offset(i as isize) = (*sse8x8.offset(i as isize))
            .wrapping_sub(
                (*sum8x8.offset(i as isize) as int64_t
                    * *sum8x8.offset(i as isize) as int64_t >> 6 as core::ffi::c_int)
                    as uint32_t,
            );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_var_sse_sum_16x16_dual_c(
    mut src_ptr: *const uint8_t,
    mut source_stride: core::ffi::c_int,
    mut ref_ptr: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse16x16: *mut uint32_t,
    mut tot_sse: *mut core::ffi::c_uint,
    mut tot_sum: *mut core::ffi::c_int,
    mut var16x16: *mut uint32_t,
) {
    let mut sum16x16: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
    let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
    while k < 2 as core::ffi::c_int {
        variance(
            src_ptr.offset((k * 16 as core::ffi::c_int) as isize),
            source_stride,
            ref_ptr.offset((k * 16 as core::ffi::c_int) as isize),
            ref_stride,
            16 as core::ffi::c_int,
            16 as core::ffi::c_int,
            &raw mut *sse16x16.offset(k as isize),
            &raw mut *sum16x16.as_mut_ptr().offset(k as isize),
        );
        k += 1;
    }
    *tot_sse = (*tot_sse)
        .wrapping_add(
            (*sse16x16.offset(0 as core::ffi::c_int as isize))
                .wrapping_add(*sse16x16.offset(1 as core::ffi::c_int as isize))
                as core::ffi::c_uint,
        );
    *tot_sum
        += sum16x16[0 as core::ffi::c_int as usize]
            + sum16x16[1 as core::ffi::c_int as usize];
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        *var16x16.offset(i as isize) = (*sse16x16.offset(i as isize))
            .wrapping_sub(
                (sum16x16[i as usize] as int64_t * sum16x16[i as usize] as int64_t
                    >> 8 as core::ffi::c_int) as uint32_t,
            );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance128x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint8_t; 16384] = [0; 16384];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance128x128_c(
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance128x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint8_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
    );
    return aom_variance128x128_c(
        temp3.0.as_mut_ptr(),
        128 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance128x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance128x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance128x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
    );
    return aom_variance128x64_c(
        temp3.0.as_mut_ptr(),
        128 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance128x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance128x64_c(
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance64x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance64x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance64x128_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance64x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
    );
    return aom_variance64x128_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance64x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint8_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
    );
    return aom_variance64x64_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance64x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint8_t; 4096] = [0; 4096];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance64x64_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance64x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance64x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
    );
    return aom_variance64x32_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance64x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance64x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance64x32_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance32x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
    );
    return aom_variance32x64_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance32x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance32x64_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance32x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance32x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance32x32_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance32x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance32x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
    );
    return aom_variance32x32_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance32x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint8_t; 512] = [0; 512];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance32x16_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance32x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance32x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint8_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 512]> = crate::platform::Aligned16([0; 512]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
    );
    return aom_variance32x16_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance16x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance16x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint8_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 512]> = crate::platform::Aligned16([0; 512]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
    );
    return aom_variance16x32_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance16x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint8_t; 512] = [0; 512];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance16x32_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint8_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 256]> = crate::platform::Aligned16([0; 256]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
    );
    return aom_variance16x16_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint8_t; 256] = [0; 256];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance16x16_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint8_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 128]> = crate::platform::Aligned16([0; 128]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
    );
    return aom_variance16x8_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint8_t; 128] = [0; 128];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance16x8_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint8_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 128]> = crate::platform::Aligned16([0; 128]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
    );
    return aom_variance8x16_c(
        temp3.0.as_mut_ptr(),
        8 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint8_t; 128] = [0; 128];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance8x16_c(
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint8_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 64]> = crate::platform::Aligned16([0; 64]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
    );
    return aom_variance8x8_c(temp3.0.as_mut_ptr(), 8 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint8_t; 64] = [0; 64];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance8x8_c(temp2.as_mut_ptr(), 8 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance8x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance8x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint8_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 32]> = crate::platform::Aligned16([0; 32]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
    );
    return aom_variance8x4_c(temp3.0.as_mut_ptr(), 8 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance8x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint8_t; 32] = [0; 32];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance8x4_c(temp2.as_mut_ptr(), 8 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance4x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance4x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint8_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 32]> = crate::platform::Aligned16([0; 32]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
    );
    return aom_variance4x8_c(temp3.0.as_mut_ptr(), 4 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance4x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint8_t; 32] = [0; 32];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance4x8_c(temp2.as_mut_ptr(), 4 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance4x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance4x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint8_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 16]> = crate::platform::Aligned16([0; 16]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
    );
    return aom_variance4x4_c(temp3.0.as_mut_ptr(), 4 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance4x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint8_t; 16] = [0; 16];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance4x4_c(temp2.as_mut_ptr(), 4 as core::ffi::c_int, b, b_stride, sse)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance4x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance4x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint8_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 64]> = crate::platform::Aligned16([0; 64]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
    );
    return aom_variance4x16_c(
        temp3.0.as_mut_ptr(),
        4 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance4x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint8_t; 64] = [0; 64];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance4x16_c(
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance16x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint8_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 64]> = crate::platform::Aligned16([0; 64]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
    );
    return aom_variance16x4_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance16x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance16x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint8_t; 64] = [0; 64];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance16x4_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance8x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance8x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint8_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 256]> = crate::platform::Aligned16([0; 256]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
    );
    return aom_variance8x32_c(
        temp3.0.as_mut_ptr(),
        8 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance8x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint8_t; 256] = [0; 256];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance8x32_c(
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance32x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance32x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint8_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 256]> = crate::platform::Aligned16([0; 256]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
    );
    return aom_variance32x8_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance32x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint8_t; 256] = [0; 256];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance32x8_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance16x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
    );
    return aom_variance16x64_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance16x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance16x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance16x64_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_variance64x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_variance64x16_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_sub_pixel_avg_variance64x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    var_filter_block2d_bil_first_pass_c(
        a,
        fdata3.as_mut_ptr(),
        a_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_avg_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
    );
    return aom_variance64x16_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        b,
        b_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_variance64x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_comp_avg_pred_c(
    mut comp_pred: *mut uint8_t,
    mut pred: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            let tmp: core::ffi::c_int = *pred.offset(j as isize) as core::ffi::c_int
                + *ref_0.offset(j as isize) as core::ffi::c_int;
            *comp_pred.offset(j as isize) = (tmp
                + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int) as uint8_t;
            j += 1;
        }
        comp_pred = comp_pred.offset(width as isize);
        pred = pred.offset(width as isize);
        ref_0 = ref_0.offset(ref_stride as isize);
        i += 1;
    }
}
unsafe extern "C-unwind" fn highbd_variance64(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut uint64_t,
    mut sum: *mut int64_t,
) {
    let mut a: *const uint16_t = ((a8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut b: *const uint16_t = ((b8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut tsum: int64_t = 0 as int64_t;
    let mut tsse: uint64_t = 0 as uint64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < h {
        let mut lsum: int32_t = 0 as int32_t;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < w {
            let diff: core::ffi::c_int = *a.offset(j as isize) as core::ffi::c_int
                - *b.offset(j as isize) as core::ffi::c_int;
            lsum = (lsum as core::ffi::c_int + diff) as int32_t;
            tsse = tsse.wrapping_add((diff * diff) as uint32_t as uint64_t);
            j += 1;
        }
        tsum += lsum as int64_t;
        a = a.offset(a_stride as isize);
        b = b.offset(b_stride as isize);
        i += 1;
    }
    *sum = tsum;
    *sse = tsse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_sse_odd_size(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> uint64_t {
    let mut sse: uint64_t = 0;
    let mut sum: int64_t = 0;
    highbd_variance64(a, a_stride, b, b_stride, w, h, &raw mut sse, &raw mut sum);
    return sse;
}
unsafe extern "C-unwind" fn highbd_8_variance(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut sum: *mut core::ffi::c_int,
) {
    let mut sse_long: uint64_t = 0 as uint64_t;
    let mut sum_long: int64_t = 0 as int64_t;
    highbd_variance64(a8, a_stride, b8, b_stride, w, h, &raw mut sse_long, &raw mut sum_long);
    *sse = sse_long as uint32_t;
    *sum = sum_long as core::ffi::c_int;
}
unsafe extern "C-unwind" fn highbd_10_variance(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut sum: *mut core::ffi::c_int,
) {
    let mut sse_long: uint64_t = 0 as uint64_t;
    let mut sum_long: int64_t = 0 as int64_t;
    highbd_variance64(a8, a_stride, b8, b_stride, w, h, &raw mut sse_long, &raw mut sum_long);
    *sse = (sse_long
        .wrapping_add(
            ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as uint64_t,
        ) >> 4 as core::ffi::c_int) as uint32_t;
    *sum = (sum_long
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 2 as core::ffi::c_int) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn highbd_12_variance(
    mut a8: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b8: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut sum: *mut core::ffi::c_int,
) {
    let mut sse_long: uint64_t = 0 as uint64_t;
    let mut sum_long: int64_t = 0 as int64_t;
    highbd_variance64(a8, a_stride, b8, b_stride, w, h, &raw mut sse_long, &raw mut sum_long);
    *sse = (sse_long
        .wrapping_add(
            ((1 as core::ffi::c_int) << 8 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as uint64_t,
        ) >> 8 as core::ffi::c_int) as uint32_t;
    *sum = (sum_long
        + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 4 as core::ffi::c_int) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_var_filter_block2d_bil_first_pass(
    mut src_ptr8: *const uint8_t,
    mut output_ptr: *mut uint16_t,
    mut src_pixels_per_line: core::ffi::c_uint,
    mut pixel_step: core::ffi::c_int,
    mut output_height: core::ffi::c_uint,
    mut output_width: core::ffi::c_uint,
    mut filter: *const uint8_t,
) {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    let mut src_ptr: *mut uint16_t = ((src_ptr8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    i = 0 as core::ffi::c_uint;
    while i < output_height {
        j = 0 as core::ffi::c_uint;
        while j < output_width {
            *output_ptr.offset(j as isize) = (*src_ptr
                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *src_ptr.offset(pixel_step as isize) as core::ffi::c_int
                    * *filter.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int) as uint16_t;
            src_ptr = src_ptr.offset(1);
            j = j.wrapping_add(1);
        }
        src_ptr = src_ptr
            .offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
        output_ptr = output_ptr.offset(output_width as isize);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_var_filter_block2d_bil_second_pass(
    mut src_ptr: *const uint16_t,
    mut output_ptr: *mut uint16_t,
    mut src_pixels_per_line: core::ffi::c_uint,
    mut pixel_step: core::ffi::c_uint,
    mut output_height: core::ffi::c_uint,
    mut output_width: core::ffi::c_uint,
    mut filter: *const uint8_t,
) {
    let mut i: core::ffi::c_uint = 0;
    let mut j: core::ffi::c_uint = 0;
    i = 0 as core::ffi::c_uint;
    while i < output_height {
        j = 0 as core::ffi::c_uint;
        while j < output_width {
            *output_ptr.offset(j as isize) = (*src_ptr
                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                * *filter.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                + *src_ptr.offset(pixel_step as isize) as core::ffi::c_int
                    * *filter.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int) as uint16_t;
            src_ptr = src_ptr.offset(1);
            j = j.wrapping_add(1);
        }
        src_ptr = src_ptr
            .offset(src_pixels_per_line.wrapping_sub(output_width) as isize);
        output_ptr = output_ptr.offset(output_width as isize);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance128x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance128x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
    );
    return aom_highbd_12_variance128x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
    );
    return aom_highbd_8_variance128x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance128x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance128x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance128x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance128x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
    );
    return aom_highbd_10_variance128x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance128x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
    );
    return aom_highbd_8_variance128x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance128x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance128x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance128x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance128x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
    );
    return aom_highbd_12_variance128x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
    );
    return aom_highbd_10_variance128x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance128x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_12_variance64x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance64x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance64x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance64x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance64x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x128_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_8_variance64x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance64x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_10_variance64x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance64x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance64x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance64x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance64x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance64x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_10_variance64x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_12_variance64x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_8_variance64x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance64x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_10_variance64x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance64x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_12_variance64x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_8_variance64x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance64x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance64x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance64x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance32x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance32x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance32x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_12_variance32x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_10_variance32x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance32x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance32x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_8_variance32x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_10_variance32x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance32x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_8_variance32x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_12_variance32x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance32x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance32x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance32x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance32x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance32x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance32x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance32x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_8_variance32x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_12_variance32x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_10_variance32x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance32x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance32x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance16x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance16x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance16x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_12_variance16x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance16x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance16x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_10_variance16x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_8_variance16x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_12_variance16x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance16x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance16x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_8_variance16x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_10_variance16x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance16x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance16x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance16x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_12_variance16x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_8_variance16x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance16x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_10_variance16x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_10_variance8x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_12_variance8x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance8x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_8_variance8x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance8x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance8x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance8x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_8_variance8x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_12_variance8x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_10_variance8x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance8x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance8x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance8x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance8x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance8x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance8x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance8x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_12_variance8x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_8_variance8x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_10_variance8x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance8x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance8x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_8_variance4x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance4x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance4x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_12_variance4x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance4x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance4x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance4x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_10_variance4x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance4x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance4x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16]> = crate::platform::Aligned16([0; 16]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_8_variance4x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16]> = crate::platform::Aligned16([0; 16]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_12_variance4x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance4x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance4x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance4x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance4x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance4x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16]> = crate::platform::Aligned16([0; 16]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_10_variance4x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance4x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_8_variance4x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance4x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_12_variance4x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
    );
    return aom_highbd_10_variance4x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance4x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance4x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance4x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance4x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance16x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance16x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_10_variance16x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance16x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance16x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_8_variance16x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_12_variance16x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance16x4_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_8_variance8x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_12_variance8x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance8x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
    );
    return aom_highbd_10_variance8x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance8x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance8x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance8x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance8x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance8x32_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance32x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance32x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance32x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance32x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_12_variance32x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance32x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance32x8_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_10_variance32x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
    );
    return aom_highbd_8_variance32x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance16x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance16x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance16x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance16x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_12_variance16x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_10_variance16x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance16x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
    );
    return aom_highbd_8_variance16x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance16x64_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_variance64x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_variance64x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_variance64x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_sub_pixel_avg_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_8_variance64x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_variance64x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_variance64x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_variance64x16_c(
    mut a: *const uint8_t,
    mut a_stride: core::ffi::c_int,
    mut b: *const uint8_t,
    mut b_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        a,
        a_stride,
        b,
        b_stride,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_sub_pixel_avg_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_12_variance64x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_sub_pixel_avg_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut dst: *const uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
    mut second_pred: *const uint8_t,
) -> uint32_t {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_avg_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
    );
    return aom_highbd_10_variance64x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        dst,
        dst_stride,
        sse,
    ) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_mse16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_mse16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_12_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_mse16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_10_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_mse16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_mse16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_10_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_mse16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_12_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_mse8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_mse8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_12_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_mse8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_10_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_mse8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_8_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_mse8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_12_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_mse8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut sse: *mut uint32_t,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_10_variance(
        src,
        src_stride,
        ref_0,
        ref_stride,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return *sse;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_comp_avg_pred_c(
    mut comp_pred8: *mut uint8_t,
    mut pred8: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut pred: *mut uint16_t = ((pred8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut ref_0: *mut uint16_t = ((ref8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut comp_pred: *mut uint16_t = ((comp_pred8 as uintptr_t)
        << 1 as core::ffi::c_int) as *mut uint16_t;
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            let tmp: core::ffi::c_int = *pred.offset(j as isize) as core::ffi::c_int
                + *ref_0.offset(j as isize) as core::ffi::c_int;
            *comp_pred.offset(j as isize) = (tmp
                + ((1 as core::ffi::c_int) << 1 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 1 as core::ffi::c_int) as uint16_t;
            j += 1;
        }
        comp_pred = comp_pred.offset(width as isize);
        pred = pred.offset(width as isize);
        ref_0 = ref_0.offset(ref_stride as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_comp_mask_pred_c(
    mut comp_pred: *mut uint8_t,
    mut pred: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut src0: *const uint8_t = if invert_mask != 0 { pred } else { ref_0 };
    let mut src1: *const uint8_t = if invert_mask != 0 { ref_0 } else { pred };
    let stride0: core::ffi::c_int = if invert_mask != 0 { width } else { ref_stride };
    let stride1: core::ffi::c_int = if invert_mask != 0 { ref_stride } else { width };
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            *comp_pred.offset(j as isize) = (*mask.offset(j as isize) as core::ffi::c_int
                * *src0.offset(j as isize) as core::ffi::c_int
                + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                    - *mask.offset(j as isize) as core::ffi::c_int)
                    * *src1.offset(j as isize) as core::ffi::c_int
                + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as uint8_t;
            j += 1;
        }
        comp_pred = comp_pred.offset(width as isize);
        src0 = src0.offset(stride0 as isize);
        src1 = src1.offset(stride1 as isize);
        mask = mask.offset(mask_stride as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint8_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 16]> = crate::platform::Aligned16([0; 16]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance4x4_c(
        temp3.0.as_mut_ptr(),
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint8_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 32]> = crate::platform::Aligned16([0; 32]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance4x8_c(
        temp3.0.as_mut_ptr(),
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint8_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 32]> = crate::platform::Aligned16([0; 32]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance8x4_c(
        temp3.0.as_mut_ptr(),
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint8_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 64]> = crate::platform::Aligned16([0; 64]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance8x8_c(
        temp3.0.as_mut_ptr(),
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint8_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 128]> = crate::platform::Aligned16([0; 128]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance8x16_c(
        temp3.0.as_mut_ptr(),
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint8_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 128]> = crate::platform::Aligned16([0; 128]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance16x8_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint8_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 256]> = crate::platform::Aligned16([0; 256]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance16x16_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint8_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 512]> = crate::platform::Aligned16([0; 512]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance16x32_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint8_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 512]> = crate::platform::Aligned16([0; 512]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance32x16_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance32x32_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance32x64_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance64x32_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint8_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance64x64_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance64x128_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance128x64_c(
        temp3.0.as_mut_ptr(),
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint8_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance128x128_c(
        temp3.0.as_mut_ptr(),
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint8_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 64]> = crate::platform::Aligned16([0; 64]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance4x16_c(
        temp3.0.as_mut_ptr(),
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint8_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 64]> = crate::platform::Aligned16([0; 64]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance16x4_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint8_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 256]> = crate::platform::Aligned16([0; 256]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance8x32_c(
        temp3.0.as_mut_ptr(),
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint8_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 256]> = crate::platform::Aligned16([0; 256]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance32x8_c(
        temp3.0.as_mut_ptr(),
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance16x64_c(
        temp3.0.as_mut_ptr(),
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_masked_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint8_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    var_filter_block2d_bil_first_pass_c(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_comp_mask_pred_c(
        temp3.0.as_mut_ptr(),
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_variance64x16_c(
        temp3.0.as_mut_ptr(),
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_comp_mask_pred_c(
    mut comp_pred8: *mut uint8_t,
    mut pred8: *const uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut mask: *const uint8_t,
    mut mask_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut pred: *mut uint16_t = ((pred8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut ref_0: *mut uint16_t = ((ref8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut comp_pred: *mut uint16_t = ((comp_pred8 as uintptr_t)
        << 1 as core::ffi::c_int) as *mut uint16_t;
    i = 0 as core::ffi::c_int;
    while i < height {
        j = 0 as core::ffi::c_int;
        while j < width {
            if invert_mask == 0 {
                *comp_pred.offset(j as isize) = (*mask.offset(j as isize)
                    as core::ffi::c_int * *ref_0.offset(j as isize) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                        - *mask.offset(j as isize) as core::ffi::c_int)
                        * *pred.offset(j as isize) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as uint16_t;
            } else {
                *comp_pred.offset(j as isize) = (*mask.offset(j as isize)
                    as core::ffi::c_int * *pred.offset(j as isize) as core::ffi::c_int
                    + (((1 as core::ffi::c_int) << 6 as core::ffi::c_int)
                        - *mask.offset(j as isize) as core::ffi::c_int)
                        * *ref_0.offset(j as isize) as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << 6 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 6 as core::ffi::c_int) as uint16_t;
            }
            j += 1;
        }
        comp_pred = comp_pred.offset(width as isize);
        pred = pred.offset(width as isize);
        ref_0 = ref_0.offset(ref_stride as isize);
        mask = mask.offset(mask_stride as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16]> = crate::platform::Aligned16([0; 16]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance4x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16]> = crate::platform::Aligned16([0; 16]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance4x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance4x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16]> = crate::platform::Aligned16([0; 16]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance4x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance4x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance4x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance4x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance4x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance8x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance8x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance8x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 32]> = crate::platform::Aligned16([0; 32]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance8x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance8x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance8x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance8x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance8x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance8x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance8x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance8x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance8x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance16x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance16x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance16x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 128]> = crate::platform::Aligned16([0; 128]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance16x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance16x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance16x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance16x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance16x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance16x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance16x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance16x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance16x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance32x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance32x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance32x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 512]> = crate::platform::Aligned16([0; 512]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance32x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance32x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance32x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance32x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance32x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance32x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance32x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance32x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance32x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance64x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance64x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance64x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 2048]> = crate::platform::Aligned16([0; 2048]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance64x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance64x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance64x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance64x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 4096]> = crate::platform::Aligned16([0; 4096]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance64x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance64x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance64x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance64x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance64x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance128x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance128x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance128x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 8192]> = crate::platform::Aligned16([0; 8192]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance128x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance128x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance128x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance128x128_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 16384]> = crate::platform::Aligned16([0; 16384]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance128x128_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance4x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance4x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance4x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance4x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance16x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance16x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance16x4_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 64]> = crate::platform::Aligned16([0; 64]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance16x4_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance8x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance8x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance8x32_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance8x32_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance32x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance32x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance32x8_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 256]> = crate::platform::Aligned16([0; 256]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance32x8_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance16x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance16x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance16x64_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance16x64_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_masked_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_10_variance64x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_masked_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_8_variance64x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_masked_sub_pixel_variance64x16_c(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut second_pred: *const uint8_t,
    mut msk: *const uint8_t,
    mut msk_stride: core::ffi::c_int,
    mut invert_mask: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    let mut temp3: crate::platform::Aligned16<[uint16_t; 1024]> = crate::platform::Aligned16([0; 1024]);
    aom_highbd_var_filter_block2d_bil_first_pass(
        src,
        fdata3.as_mut_ptr(),
        src_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    aom_highbd_comp_mask_pred_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        second_pred,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        msk,
        msk_stride,
        invert_mask,
    );
    return aom_highbd_12_variance64x16_c(
        (temp3.0.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        ref_0,
        ref_stride,
        sse as *mut uint32_t,
    );
}
#[inline]
unsafe extern "C-unwind" fn obmc_variance(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut sum: *mut core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut tsse: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut tsum: core::ffi::c_int = 0 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < h {
        j = 0 as core::ffi::c_int;
        while j < w {
            let mut diff: core::ffi::c_int = if *wsrc.offset(j as isize)
                - *pre.offset(j as isize) as int32_t * *mask.offset(j as isize)
                < 0 as int32_t
            {
                -(-(*wsrc.offset(j as isize)
                    - *pre.offset(j as isize) as core::ffi::c_int
                        * *mask.offset(j as isize))
                    + ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 12 as core::ffi::c_int)
            } else {
                *wsrc.offset(j as isize)
                    - *pre.offset(j as isize) as core::ffi::c_int
                        * *mask.offset(j as isize)
                    + ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 12 as core::ffi::c_int
            };
            tsum += diff;
            tsse = tsse.wrapping_add((diff * diff) as core::ffi::c_uint);
            j += 1;
        }
        pre = pre.offset(pre_stride as isize);
        wsrc = wsrc.offset(w as isize);
        mask = mask.offset(w as isize);
        i += 1;
    }
    *sse = tsse;
    *sum = tsum;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint8_t; 16] = [0; 16];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance4x4_c(
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint8_t; 32] = [0; 32];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance4x8_c(
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint8_t; 32] = [0; 32];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance8x4_c(
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint8_t; 64] = [0; 64];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance8x8_c(
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint8_t; 128] = [0; 128];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance8x16_c(
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint8_t; 128] = [0; 128];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance16x8_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint8_t; 256] = [0; 256];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance16x16_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint8_t; 512] = [0; 512];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance16x32_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint8_t; 512] = [0; 512];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance32x16_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance32x32_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance32x64_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint8_t; 2048] = [0; 2048];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance64x32_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint8_t; 4096] = [0; 4096];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance64x64_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance64x128_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint8_t; 8192] = [0; 8192];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance128x64_c(
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint8_t; 16384] = [0; 16384];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance128x128_c(
        temp2.as_mut_ptr(),
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint8_t; 64] = [0; 64];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance4x16_c(
        temp2.as_mut_ptr(),
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint8_t; 64] = [0; 64];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance16x4_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint8_t; 256] = [0; 256];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance8x32_c(
        temp2.as_mut_ptr(),
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint8_t; 256] = [0; 256];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance32x8_c(
        temp2.as_mut_ptr(),
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance16x64_c(
        temp2.as_mut_ptr(),
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_obmc_sub_pixel_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint8_t; 1024] = [0; 1024];
    var_filter_block2d_bil_first_pass_c(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_uint,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    var_filter_block2d_bil_second_pass_c(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_obmc_variance64x16_c(
        temp2.as_mut_ptr(),
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[inline]
unsafe extern "C-unwind" fn highbd_obmc_variance64(
    mut pre8: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut uint64_t,
    mut sum: *mut int64_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut pre: *mut uint16_t = ((pre8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut tsse: uint64_t = 0 as uint64_t;
    let mut tsum: int64_t = 0 as int64_t;
    i = 0 as core::ffi::c_int;
    while i < h {
        j = 0 as core::ffi::c_int;
        while j < w {
            let mut diff: core::ffi::c_int = if *wsrc.offset(j as isize)
                - *pre.offset(j as isize) as int32_t * *mask.offset(j as isize)
                < 0 as int32_t
            {
                -(-(*wsrc.offset(j as isize)
                    - *pre.offset(j as isize) as core::ffi::c_int
                        * *mask.offset(j as isize))
                    + ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 12 as core::ffi::c_int)
            } else {
                *wsrc.offset(j as isize)
                    - *pre.offset(j as isize) as core::ffi::c_int
                        * *mask.offset(j as isize)
                    + ((1 as core::ffi::c_int) << 12 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 12 as core::ffi::c_int
            };
            tsum += diff as int64_t;
            tsse = tsse.wrapping_add((diff * diff) as uint64_t);
            j += 1;
        }
        pre = pre.offset(pre_stride as isize);
        wsrc = wsrc.offset(w as isize);
        mask = mask.offset(w as isize);
        i += 1;
    }
    *sse = tsse;
    *sum = tsum;
}
#[inline]
unsafe extern "C-unwind" fn highbd_obmc_variance(
    mut pre8: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut sum: *mut core::ffi::c_int,
) {
    let mut sum64: int64_t = 0;
    let mut sse64: uint64_t = 0;
    highbd_obmc_variance64(pre8, pre_stride, wsrc, mask, w, h, &raw mut sse64, &raw mut sum64);
    *sum = sum64 as core::ffi::c_int;
    *sse = sse64 as core::ffi::c_uint;
}
#[inline]
unsafe extern "C-unwind" fn highbd_10_obmc_variance(
    mut pre8: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut sum: *mut core::ffi::c_int,
) {
    let mut sum64: int64_t = 0;
    let mut sse64: uint64_t = 0;
    highbd_obmc_variance64(pre8, pre_stride, wsrc, mask, w, h, &raw mut sse64, &raw mut sum64);
    *sum = (sum64
        + ((1 as core::ffi::c_int) << 2 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 2 as core::ffi::c_int) as core::ffi::c_int;
    *sse = (sse64
        .wrapping_add(
            ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as uint64_t,
        ) >> 4 as core::ffi::c_int) as core::ffi::c_uint;
}
#[inline]
unsafe extern "C-unwind" fn highbd_12_obmc_variance(
    mut pre8: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
    mut sse: *mut core::ffi::c_uint,
    mut sum: *mut core::ffi::c_int,
) {
    let mut sum64: int64_t = 0;
    let mut sse64: uint64_t = 0;
    highbd_obmc_variance64(pre8, pre_stride, wsrc, mask, w, h, &raw mut sse64, &raw mut sum64);
    *sum = (sum64
        + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int >> 1 as core::ffi::c_int)
            as int64_t >> 4 as core::ffi::c_int) as core::ffi::c_int;
    *sse = (sse64
        .wrapping_add(
            ((1 as core::ffi::c_int) << 8 as core::ffi::c_int >> 1 as core::ffi::c_int)
                as uint64_t,
        ) >> 8 as core::ffi::c_int) as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance4x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance4x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance4x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 20] = [0; 20];
    let mut temp2: [uint16_t; 16] = [0; 16];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance4x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance4x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance4x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance4x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 36] = [0; 36];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance4x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance8x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance8x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance8x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 40] = [0; 40];
    let mut temp2: [uint16_t; 32] = [0; 32];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance8x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance8x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance8x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance8x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 72] = [0; 72];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance8x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance8x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance8x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance8x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 136] = [0; 136];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance8x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance16x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance16x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance16x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 144] = [0; 144];
    let mut temp2: [uint16_t; 128] = [0; 128];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance16x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance16x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance16x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance16x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 272] = [0; 272];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance16x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance16x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance16x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance16x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 528] = [0; 528];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance16x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance32x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance32x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance32x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 544] = [0; 544];
    let mut temp2: [uint16_t; 512] = [0; 512];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance32x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance32x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance32x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance32x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1056] = [0; 1056];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance32x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance32x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance32x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance32x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2080] = [0; 2080];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance32x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance64x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance64x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance64x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 2112] = [0; 2112];
    let mut temp2: [uint16_t; 2048] = [0; 2048];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance64x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance64x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance64x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance64x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 4160] = [0; 4160];
    let mut temp2: [uint16_t; 4096] = [0; 4096];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance64x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance64x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance64x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance64x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8256] = [0; 8256];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance64x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance128x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance128x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance128x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 8320] = [0; 8320];
    let mut temp2: [uint16_t; 8192] = [0; 8192];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance128x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        128 as core::ffi::c_int,
        128 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (128 as core::ffi::c_int * 128 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance128x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance128x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance128x128_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 16512] = [0; 16512];
    let mut temp2: [uint16_t; 16384] = [0; 16384];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (128 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        128 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance128x128_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        128 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (4 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance4x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance4x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance4x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 68] = [0; 68];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        4 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance4x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        4 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        4 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 4 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance16x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance16x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance16x4_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 80] = [0; 80];
    let mut temp2: [uint16_t; 64] = [0; 64];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (4 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        4 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance16x4_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (8 as core::ffi::c_int * 32 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance8x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance8x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance8x32_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 264] = [0; 264];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (32 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        8 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance8x32_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        8 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (32 as core::ffi::c_int * 8 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance32x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance32x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance32x8_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 288] = [0; 288];
    let mut temp2: [uint16_t; 256] = [0; 256];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (8 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        32 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        8 as core::ffi::c_uint,
        32 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance32x8_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        32 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        16 as core::ffi::c_int,
        64 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (16 as core::ffi::c_int * 64 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance16x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance16x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance16x64_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1040] = [0; 1040];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (64 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        16 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance16x64_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        16 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    highbd_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    return (*sse)
        .wrapping_sub(
            (sum as int64_t * sum as int64_t
                / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t)
                as core::ffi::c_uint,
        );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_12_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut sum: core::ffi::c_int = 0;
    let mut var: int64_t = 0;
    highbd_10_obmc_variance(
        pre,
        pre_stride,
        wsrc,
        mask,
        64 as core::ffi::c_int,
        16 as core::ffi::c_int,
        sse,
        &raw mut sum,
    );
    var = *sse as int64_t
        - sum as int64_t * sum as int64_t
            / (64 as core::ffi::c_int * 16 as core::ffi::c_int) as int64_t;
    return if var >= 0 as int64_t {
        var as core::ffi::c_uint
    } else {
        0 as core::ffi::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_8_obmc_sub_pixel_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_8_obmc_variance64x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_10_obmc_sub_pixel_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_10_obmc_variance64x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_12_obmc_sub_pixel_variance64x16_c(
    mut pre: *const uint8_t,
    mut pre_stride: core::ffi::c_int,
    mut xoffset: core::ffi::c_int,
    mut yoffset: core::ffi::c_int,
    mut wsrc: *const int32_t,
    mut mask: *const int32_t,
    mut sse: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut fdata3: [uint16_t; 1088] = [0; 1088];
    let mut temp2: [uint16_t; 1024] = [0; 1024];
    aom_highbd_var_filter_block2d_bil_first_pass(
        pre,
        fdata3.as_mut_ptr(),
        pre_stride as core::ffi::c_uint,
        1 as core::ffi::c_int,
        (16 as core::ffi::c_int + 1 as core::ffi::c_int) as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[xoffset as usize]).as_ptr(),
    );
    aom_highbd_var_filter_block2d_bil_second_pass(
        fdata3.as_mut_ptr(),
        temp2.as_mut_ptr(),
        64 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        16 as core::ffi::c_uint,
        64 as core::ffi::c_uint,
        (bilinear_filters_2t[yoffset as usize]).as_ptr(),
    );
    return aom_highbd_12_obmc_variance64x16_c(
        (temp2.as_mut_ptr() as uintptr_t >> 1 as core::ffi::c_int) as *mut uint8_t,
        64 as core::ffi::c_int,
        wsrc,
        mask,
        sse,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_c(
    mut dst: *mut uint8_t,
    mut dstride: core::ffi::c_int,
    mut src: *mut uint16_t,
    mut sstride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> uint64_t {
    let mut sum: uint64_t = 0 as uint64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < h {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < w {
            let mut e: core::ffi::c_int = *dst.offset((i * dstride + j) as isize)
                as uint16_t as core::ffi::c_int
                - *src.offset((i * sstride + j) as isize) as core::ffi::c_int;
            sum = sum.wrapping_add((e * e) as uint64_t);
            j += 1;
        }
        i += 1;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse_16xh_16bit_c(
    mut dst: *mut uint8_t,
    mut dstride: core::ffi::c_int,
    mut src: *mut uint16_t,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> uint64_t {
    let mut src_temp: *mut uint16_t = src;
    let mut dst_temp: *mut uint8_t = dst;
    let num_blks: core::ffi::c_int = 16 as core::ffi::c_int / w;
    let mut sum: int64_t = 0 as int64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_blks {
        sum = (sum as uint64_t)
            .wrapping_add(aom_mse_wxh_16bit_c(dst_temp, dstride, src_temp, w, w, h))
            as int64_t as int64_t;
        dst_temp = dst_temp.offset(w as isize);
        src_temp = src_temp.offset((w * h) as isize);
        i += 1;
    }
    return sum as uint64_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_calc_variance_stat_c(
    mut src: *const uint8_t,
    mut stride: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
) -> int64_t {
    let mut dclevel: crate::platform::Aligned16<[uint8_t; 16900]> = crate::platform::Aligned16([0; 16900]);
    let mut pstride: core::ffi::c_int = bw + 2 as core::ffi::c_int;
    let mut pred_ptr: *mut uint8_t = &raw mut *dclevel.0
        .as_mut_ptr()
        .offset((pstride + 1 as core::ffi::c_int) as isize) as *mut uint8_t;
    static mut gau_filter: [[core::ffi::c_int; 3]; 3] = [
        [1 as core::ffi::c_int, 2 as core::ffi::c_int, 1 as core::ffi::c_int],
        [2 as core::ffi::c_int, 4 as core::ffi::c_int, 2 as core::ffi::c_int],
        [1 as core::ffi::c_int, 2 as core::ffi::c_int, 1 as core::ffi::c_int],
    ];
    let mut idy: core::ffi::c_int = -(1 as core::ffi::c_int);
    while idy < bh + 1 as core::ffi::c_int {
        let mut idx: core::ffi::c_int = -(1 as core::ffi::c_int);
        while idx < bw + 1 as core::ffi::c_int {
            let mut offset_idy: core::ffi::c_int = idy;
            let mut offset_idx: core::ffi::c_int = idx;
            if idy == -(1 as core::ffi::c_int) {
                offset_idy = 0 as core::ffi::c_int;
            }
            if idy == bh {
                offset_idy = bh - 1 as core::ffi::c_int;
            }
            if idx == -(1 as core::ffi::c_int) {
                offset_idx = 0 as core::ffi::c_int;
            }
            if idx == bw {
                offset_idx = bw - 1 as core::ffi::c_int;
            }
            let mut offset: core::ffi::c_int = offset_idy * stride + offset_idx;
            *pred_ptr.offset((idy * pstride + idx) as isize) = *src
                .offset(offset as isize);
            idx += 1;
        }
        idy += 1;
    }
    let mut var_stats: int64_t = 0 as int64_t;
    let mut idy_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy_0 < bh {
        let mut idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx_0 < bw {
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut iy: core::ffi::c_int = 0 as core::ffi::c_int;
            while iy < 3 as core::ffi::c_int {
                let mut ix: core::ffi::c_int = 0 as core::ffi::c_int;
                while ix < 3 as core::ffi::c_int {
                    sum
                        += *pred_ptr
                            .offset(
                                ((idy_0 + iy - 1 as core::ffi::c_int) * pstride
                                    + (idx_0 + ix - 1 as core::ffi::c_int)) as isize,
                            ) as core::ffi::c_int * gau_filter[iy as usize][ix as usize];
                    ix += 1;
                }
                iy += 1;
            }
            sum = sum >> 4 as core::ffi::c_int;
            let mut diff: int64_t = (*pred_ptr.offset((idy_0 * pstride + idx_0) as isize)
                as core::ffi::c_int - sum) as int64_t;
            var_stats += diff * diff;
            idx_0 += 1;
        }
        idy_0 += 1;
    }
    var_stats <<= 4 as core::ffi::c_int;
    return var_stats;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_calc_variance_stat_c(
    mut src: *const uint16_t,
    mut stride: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
) -> int64_t {
    let mut dclevel: crate::platform::Aligned16<[uint16_t; 16900]> = crate::platform::Aligned16([0; 16900]);
    let mut pstride: core::ffi::c_int = bw + 2 as core::ffi::c_int;
    let mut pred_ptr: *mut uint16_t = &raw mut *dclevel.0
        .as_mut_ptr()
        .offset((pstride + 1 as core::ffi::c_int) as isize) as *mut uint16_t;
    static mut gau_filter: [[core::ffi::c_int; 3]; 3] = [
        [1 as core::ffi::c_int, 2 as core::ffi::c_int, 1 as core::ffi::c_int],
        [2 as core::ffi::c_int, 4 as core::ffi::c_int, 2 as core::ffi::c_int],
        [1 as core::ffi::c_int, 2 as core::ffi::c_int, 1 as core::ffi::c_int],
    ];
    let mut idy: core::ffi::c_int = -(1 as core::ffi::c_int);
    while idy < bh + 1 as core::ffi::c_int {
        let mut idx: core::ffi::c_int = -(1 as core::ffi::c_int);
        while idx < bw + 1 as core::ffi::c_int {
            let mut offset_idy: core::ffi::c_int = idy;
            let mut offset_idx: core::ffi::c_int = idx;
            if idy == -(1 as core::ffi::c_int) {
                offset_idy = 0 as core::ffi::c_int;
            }
            if idy == bh {
                offset_idy = bh - 1 as core::ffi::c_int;
            }
            if idx == -(1 as core::ffi::c_int) {
                offset_idx = 0 as core::ffi::c_int;
            }
            if idx == bw {
                offset_idx = bw - 1 as core::ffi::c_int;
            }
            let mut offset: core::ffi::c_int = offset_idy * stride + offset_idx;
            *pred_ptr.offset((idy * pstride + idx) as isize) = *src
                .offset(offset as isize);
            idx += 1;
        }
        idy += 1;
    }
    let mut var_stats: int64_t = 0 as int64_t;
    let mut idy_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while idy_0 < bh {
        let mut idx_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while idx_0 < bw {
            let mut sum: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut iy: core::ffi::c_int = 0 as core::ffi::c_int;
            while iy < 3 as core::ffi::c_int {
                let mut ix: core::ffi::c_int = 0 as core::ffi::c_int;
                while ix < 3 as core::ffi::c_int {
                    sum
                        += *pred_ptr
                            .offset(
                                ((idy_0 + iy - 1 as core::ffi::c_int) * pstride
                                    + (idx_0 + ix - 1 as core::ffi::c_int)) as isize,
                            ) as core::ffi::c_int * gau_filter[iy as usize][ix as usize];
                    ix += 1;
                }
                iy += 1;
            }
            sum = sum >> 4 as core::ffi::c_int;
            let mut diff: int64_t = (*pred_ptr.offset((idy_0 * pstride + idx_0) as isize)
                as core::ffi::c_int - sum) as int64_t;
            var_stats += diff * diff;
            idx_0 += 1;
        }
        idy_0 += 1;
    }
    var_stats <<= 4 as core::ffi::c_int;
    return var_stats;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_mse_wxh_16bit_highbd_c(
    mut dst: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut src: *mut uint16_t,
    mut sstride: core::ffi::c_int,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> uint64_t {
    let mut sum: uint64_t = 0 as uint64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < h {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < w {
            let mut e: core::ffi::c_int = *dst.offset((i * dstride + j) as isize)
                as core::ffi::c_int
                - *src.offset((i * sstride + j) as isize) as core::ffi::c_int;
            sum = sum.wrapping_add((e * e) as uint64_t);
            j += 1;
        }
        i += 1;
    }
    return sum;
}
