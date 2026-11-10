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

// Mechanically translated from upstream/aom/av1/common/reconintra.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
extern "C-unwind" {
    static av1_ss_size_lookup: [[[BLOCK_SIZE; 2]; 2]; 22];
    fn aom_dc_128_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_128_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_left_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_dc_top_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_h_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    static mut aom_highbd_dc_128_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_128_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_left_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_dc_top_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_h_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_paeth_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_h_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_smooth_v_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_16x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_16x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_16x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_16x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_16x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_32x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_32x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_32x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_32x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_4x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_4x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_4x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_64x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_64x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_64x64: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_8x16: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_8x32: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_8x4: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut aom_highbd_v_predictor_8x8: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
        ) -> (),
    >;
    fn aom_paeth_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_paeth_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_h_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_smooth_v_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_16x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_16x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_16x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_16x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_16x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_32x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_32x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_32x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_32x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_4x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_4x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_4x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_64x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_64x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_64x64_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_8x16_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_8x32_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_8x4_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    fn aom_v_predictor_8x8_c(
        dst: *mut uint8_t,
        y_stride: ptrdiff_t,
        above: *const uint8_t,
        left: *const uint8_t,
    );
    static mut av1_filter_intra_edge: Option<
        unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int, core::ffi::c_int) -> (),
    >;
    static mut av1_filter_intra_predictor: Option<
        unsafe extern "C-unwind" fn(
            *mut uint8_t,
            ptrdiff_t,
            TX_SIZE,
            *const uint8_t,
            *const uint8_t,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_highbd_dr_prediction_z1: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_highbd_dr_prediction_z2: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_highbd_dr_prediction_z3: Option<
        unsafe extern "C-unwind" fn(
            *mut uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_highbd_filter_intra_edge: Option<
        unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> (),
    >;
    static mut av1_highbd_upsample_intra_edge: Option<
        unsafe extern "C-unwind" fn(*mut uint16_t, core::ffi::c_int, core::ffi::c_int) -> (),
    >;
    static mut av1_upsample_intra_edge: Option<
        unsafe extern "C-unwind" fn(*mut uint8_t, core::ffi::c_int) -> (),
    >;
    fn av1_cfl_predict_block(
        xd: *mut MACROBLOCKD,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        tx_size: TX_SIZE,
        plane: core::ffi::c_int,
    );
    fn cfl_store_dc_pred(
        xd: *mut MACROBLOCKD,
        input: *const uint8_t,
        pred_plane: CFL_PRED_TYPE,
        width: core::ffi::c_int,
    );
    fn cfl_load_dc_pred(
        xd: *mut MACROBLOCKD,
        dst: *mut uint8_t,
        dst_stride: core::ffi::c_int,
        tx_size: TX_SIZE,
        pred_plane: CFL_PRED_TYPE,
    );
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type uintptr_t = usize;
pub type qm_val_t = uint8_t;
pub type aom_color_primaries = core::ffi::c_uint;
pub const AOM_CICP_CP_RESERVED_23: aom_color_primaries = 23;
pub const AOM_CICP_CP_EBU_3213: aom_color_primaries = 22;
pub const AOM_CICP_CP_RESERVED_13: aom_color_primaries = 13;
pub const AOM_CICP_CP_SMPTE_432: aom_color_primaries = 12;
pub const AOM_CICP_CP_SMPTE_431: aom_color_primaries = 11;
pub const AOM_CICP_CP_XYZ: aom_color_primaries = 10;
pub const AOM_CICP_CP_BT_2020: aom_color_primaries = 9;
pub const AOM_CICP_CP_GENERIC_FILM: aom_color_primaries = 8;
pub const AOM_CICP_CP_SMPTE_240: aom_color_primaries = 7;
pub const AOM_CICP_CP_BT_601: aom_color_primaries = 6;
pub const AOM_CICP_CP_BT_470_B_G: aom_color_primaries = 5;
pub const AOM_CICP_CP_BT_470_M: aom_color_primaries = 4;
pub const AOM_CICP_CP_RESERVED_3: aom_color_primaries = 3;
pub const AOM_CICP_CP_UNSPECIFIED: aom_color_primaries = 2;
pub const AOM_CICP_CP_BT_709: aom_color_primaries = 1;
pub const AOM_CICP_CP_RESERVED_0: aom_color_primaries = 0;
pub type aom_color_primaries_t = aom_color_primaries;
pub type aom_transfer_characteristics = core::ffi::c_uint;
pub const AOM_CICP_TC_RESERVED_19: aom_transfer_characteristics = 19;
pub const AOM_CICP_TC_HLG: aom_transfer_characteristics = 18;
pub const AOM_CICP_TC_SMPTE_428: aom_transfer_characteristics = 17;
pub const AOM_CICP_TC_SMPTE_2084: aom_transfer_characteristics = 16;
pub const AOM_CICP_TC_BT_2020_12_BIT: aom_transfer_characteristics = 15;
pub const AOM_CICP_TC_BT_2020_10_BIT: aom_transfer_characteristics = 14;
pub const AOM_CICP_TC_SRGB: aom_transfer_characteristics = 13;
pub const AOM_CICP_TC_BT_1361: aom_transfer_characteristics = 12;
pub const AOM_CICP_TC_IEC_61966: aom_transfer_characteristics = 11;
pub const AOM_CICP_TC_LOG_100_SQRT10: aom_transfer_characteristics = 10;
pub const AOM_CICP_TC_LOG_100: aom_transfer_characteristics = 9;
pub const AOM_CICP_TC_LINEAR: aom_transfer_characteristics = 8;
pub const AOM_CICP_TC_SMPTE_240: aom_transfer_characteristics = 7;
pub const AOM_CICP_TC_BT_601: aom_transfer_characteristics = 6;
pub const AOM_CICP_TC_BT_470_B_G: aom_transfer_characteristics = 5;
pub const AOM_CICP_TC_BT_470_M: aom_transfer_characteristics = 4;
pub const AOM_CICP_TC_RESERVED_3: aom_transfer_characteristics = 3;
pub const AOM_CICP_TC_UNSPECIFIED: aom_transfer_characteristics = 2;
pub const AOM_CICP_TC_BT_709: aom_transfer_characteristics = 1;
pub const AOM_CICP_TC_RESERVED_0: aom_transfer_characteristics = 0;
pub type aom_transfer_characteristics_t = aom_transfer_characteristics;
pub type aom_matrix_coefficients = core::ffi::c_uint;
pub const AOM_CICP_MC_YCGCO_RO: aom_matrix_coefficients = 17;
pub const AOM_CICP_MC_YCGCO_RE: aom_matrix_coefficients = 16;
pub const AOM_CICP_MC_IPT_C2: aom_matrix_coefficients = 15;
pub const AOM_CICP_MC_RESERVED_15: aom_matrix_coefficients = 15;
pub const AOM_CICP_MC_ICTCP: aom_matrix_coefficients = 14;
pub const AOM_CICP_MC_CHROMAT_CL: aom_matrix_coefficients = 13;
pub const AOM_CICP_MC_CHROMAT_NCL: aom_matrix_coefficients = 12;
pub const AOM_CICP_MC_SMPTE_2085: aom_matrix_coefficients = 11;
pub const AOM_CICP_MC_BT_2020_CL: aom_matrix_coefficients = 10;
pub const AOM_CICP_MC_BT_2020_NCL: aom_matrix_coefficients = 9;
pub const AOM_CICP_MC_SMPTE_YCGCO: aom_matrix_coefficients = 8;
pub const AOM_CICP_MC_SMPTE_240: aom_matrix_coefficients = 7;
pub const AOM_CICP_MC_BT_601: aom_matrix_coefficients = 6;
pub const AOM_CICP_MC_BT_470_B_G: aom_matrix_coefficients = 5;
pub const AOM_CICP_MC_FCC: aom_matrix_coefficients = 4;
pub const AOM_CICP_MC_RESERVED_3: aom_matrix_coefficients = 3;
pub const AOM_CICP_MC_UNSPECIFIED: aom_matrix_coefficients = 2;
pub const AOM_CICP_MC_BT_709: aom_matrix_coefficients = 1;
pub const AOM_CICP_MC_IDENTITY: aom_matrix_coefficients = 0;
pub type aom_matrix_coefficients_t = aom_matrix_coefficients;
pub type aom_color_range = core::ffi::c_uint;
pub const AOM_CR_FULL_RANGE: aom_color_range = 1;
pub const AOM_CR_STUDIO_RANGE: aom_color_range = 0;
pub type aom_color_range_t = aom_color_range;
pub type aom_chroma_sample_position = core::ffi::c_uint;
pub const AOM_CSP_RESERVED: aom_chroma_sample_position = 3;
pub const AOM_CSP_COLOCATED: aom_chroma_sample_position = 2;
pub const AOM_CSP_VERTICAL: aom_chroma_sample_position = 1;
pub const AOM_CSP_UNKNOWN: aom_chroma_sample_position = 0;
pub type aom_chroma_sample_position_t = aom_chroma_sample_position;
pub type aom_metadata_insert_flags = core::ffi::c_uint;
pub const AOM_MIF_ANY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 18;
pub const AOM_MIF_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 17;
pub const AOM_MIF_NON_KEY_FRAME_LAYER_SPECIFIC: aom_metadata_insert_flags = 16;
pub const AOM_MIF_ANY_FRAME: aom_metadata_insert_flags = 2;
pub const AOM_MIF_KEY_FRAME: aom_metadata_insert_flags = 1;
pub const AOM_MIF_NON_KEY_FRAME: aom_metadata_insert_flags = 0;
pub type aom_metadata_insert_flags_t = aom_metadata_insert_flags;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_metadata_array {
    pub sz: size_t,
    pub metadata_array: *mut *mut aom_metadata_t,
}
pub type aom_metadata_t = aom_metadata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_metadata {
    pub type_0: uint32_t,
    pub payload: *mut uint8_t,
    pub sz: size_t,
    pub insert_flag: aom_metadata_insert_flags_t,
}
pub type aom_metadata_array_t = aom_metadata_array;
pub type aom_codec_err_t = core::ffi::c_uint;
pub const AOM_CODEC_LIST_END: aom_codec_err_t = 9;
pub const AOM_CODEC_INVALID_PARAM: aom_codec_err_t = 8;
pub const AOM_CODEC_CORRUPT_FRAME: aom_codec_err_t = 7;
pub const AOM_CODEC_UNSUP_FEATURE: aom_codec_err_t = 6;
pub const AOM_CODEC_UNSUP_BITSTREAM: aom_codec_err_t = 5;
pub const AOM_CODEC_INCAPABLE: aom_codec_err_t = 4;
pub const AOM_CODEC_ABI_MISMATCH: aom_codec_err_t = 3;
pub const AOM_CODEC_MEM_ERROR: aom_codec_err_t = 2;
pub const AOM_CODEC_ERROR: aom_codec_err_t = 1;
pub const AOM_CODEC_OK: aom_codec_err_t = 0;
pub type aom_bit_depth_t = aom_bit_depth;
pub type aom_bit_depth = core::ffi::c_uint;
pub const AOM_BITS_12: aom_bit_depth = 12;
pub const AOM_BITS_10: aom_bit_depth = 10;
pub const AOM_BITS_8: aom_bit_depth = 8;
pub type aom_release_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
pub type aom_codec_frame_buffer_t = aom_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_get_frame_buffer_cb_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        size_t,
        *mut aom_codec_frame_buffer_t,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_pyramid {
    pub mutex: pthread_mutex_t,
    pub max_levels: core::ffi::c_int,
    pub filled_levels: core::ffi::c_int,
    pub buffer_alloc: *mut uint8_t,
    pub layers: *mut PyramidLayer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyramidLayer {
    pub buffer: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type pthread_mutex_t = libc::pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct corner_list {
    pub mutex: pthread_mutex_t,
    pub valid: bool,
    pub num_corners: core::ffi::c_int,
    pub corners: [core::ffi::c_int; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yv12_buffer_config {
    pub c2rust_unnamed: C2RustUnnamed_9,
    pub c2rust_unnamed_0: C2RustUnnamed_7,
    pub c2rust_unnamed_1: C2RustUnnamed_5,
    pub c2rust_unnamed_2: C2RustUnnamed_3,
    pub c2rust_unnamed_3: C2RustUnnamed_1,
    pub c2rust_unnamed_4: C2RustUnnamed,
    pub use_external_reference_buffers: core::ffi::c_int,
    pub store_buf_adr: [*mut uint8_t; 3],
    pub y_pyramid: *mut image_pyramid,
    pub corners: *mut corner_list,
    pub buffer_alloc: *mut uint8_t,
    pub buffer_alloc_sz: size_t,
    pub border: core::ffi::c_int,
    pub frame_size: size_t,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub bit_depth: core::ffi::c_uint,
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub monochrome: uint8_t,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub color_range: aom_color_range_t,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub corrupted: core::ffi::c_int,
    pub flags: core::ffi::c_int,
    pub metadata: *mut aom_metadata_array_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub buffers: [*mut uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub y_buffer: *mut uint8_t,
    pub u_buffer: *mut uint8_t,
    pub v_buffer: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub strides: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub y_stride: core::ffi::c_int,
    pub uv_stride: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub crop_heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub y_crop_height: core::ffi::c_int,
    pub uv_crop_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub crop_widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub y_crop_width: core::ffi::c_int,
    pub uv_crop_width: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub c2rust_unnamed: C2RustUnnamed_8,
    pub heights: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub y_height: core::ffi::c_int,
    pub uv_height: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub c2rust_unnamed: C2RustUnnamed_10,
    pub widths: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub y_width: core::ffi::c_int,
    pub uv_width: core::ffi::c_int,
}
pub type YV12_BUFFER_CONFIG = yv12_buffer_config;
pub type C2RustUnnamed_11 = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed_11 = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed_11 = 4;
pub const TX_SIZES: C2RustUnnamed_11 = 5;
pub const TX_SIZES_ALL: C2RustUnnamed_11 = 19;
pub const TX_64X16: C2RustUnnamed_11 = 18;
pub const TX_16X64: C2RustUnnamed_11 = 17;
pub const TX_32X8: C2RustUnnamed_11 = 16;
pub const TX_8X32: C2RustUnnamed_11 = 15;
pub const TX_16X4: C2RustUnnamed_11 = 14;
pub const TX_4X16: C2RustUnnamed_11 = 13;
pub const TX_64X32: C2RustUnnamed_11 = 12;
pub const TX_32X64: C2RustUnnamed_11 = 11;
pub const TX_32X16: C2RustUnnamed_11 = 10;
pub const TX_16X32: C2RustUnnamed_11 = 9;
pub const TX_16X8: C2RustUnnamed_11 = 8;
pub const TX_8X16: C2RustUnnamed_11 = 7;
pub const TX_8X4: C2RustUnnamed_11 = 6;
pub const TX_4X8: C2RustUnnamed_11 = 5;
pub const TX_64X64: C2RustUnnamed_11 = 4;
pub const TX_32X32: C2RustUnnamed_11 = 3;
pub const TX_16X16: C2RustUnnamed_11 = 2;
pub const TX_8X8: C2RustUnnamed_11 = 1;
pub const TX_4X4: C2RustUnnamed_11 = 0;
pub type TX_SIZE = uint8_t;
pub type TX_TYPE = uint8_t;
pub type BITSTREAM_PROFILE = int8_t;
pub type BLOCK_SIZE = core::ffi::c_uchar;
pub const BLOCK_LARGEST: BLOCK_SIZE = 15;
pub const BLOCK_INVALID: BLOCK_SIZE = 255;
pub const BLOCK_SIZES: BLOCK_SIZE = 16;
pub const BLOCK_SIZES_ALL: BLOCK_SIZE = 22;
pub const BLOCK_64X16: BLOCK_SIZE = 21;
pub const BLOCK_16X64: BLOCK_SIZE = 20;
pub const BLOCK_32X8: BLOCK_SIZE = 19;
pub const BLOCK_8X32: BLOCK_SIZE = 18;
pub const BLOCK_16X4: BLOCK_SIZE = 17;
pub const BLOCK_4X16: BLOCK_SIZE = 16;
pub const BLOCK_128X128: BLOCK_SIZE = 15;
pub const BLOCK_128X64: BLOCK_SIZE = 14;
pub const BLOCK_64X128: BLOCK_SIZE = 13;
pub const BLOCK_64X64: BLOCK_SIZE = 12;
pub const BLOCK_64X32: BLOCK_SIZE = 11;
pub const BLOCK_32X64: BLOCK_SIZE = 10;
pub const BLOCK_32X32: BLOCK_SIZE = 9;
pub const BLOCK_32X16: BLOCK_SIZE = 8;
pub const BLOCK_16X32: BLOCK_SIZE = 7;
pub const BLOCK_16X16: BLOCK_SIZE = 6;
pub const BLOCK_16X8: BLOCK_SIZE = 5;
pub const BLOCK_8X16: BLOCK_SIZE = 4;
pub const BLOCK_8X8: BLOCK_SIZE = 3;
pub const BLOCK_8X4: BLOCK_SIZE = 2;
pub const BLOCK_4X8: BLOCK_SIZE = 1;
pub const BLOCK_4X4: BLOCK_SIZE = 0;
pub type C2RustUnnamed_12 = core::ffi::c_uint;
pub const PARTITION_INVALID: C2RustUnnamed_12 = 255;
pub const PARTITION_TYPES: C2RustUnnamed_12 = 4;
pub const EXT_PARTITION_TYPES: C2RustUnnamed_12 = 10;
pub const PARTITION_VERT_4: C2RustUnnamed_12 = 9;
pub const PARTITION_HORZ_4: C2RustUnnamed_12 = 8;
pub const PARTITION_VERT_B: C2RustUnnamed_12 = 7;
pub const PARTITION_VERT_A: C2RustUnnamed_12 = 6;
pub const PARTITION_HORZ_B: C2RustUnnamed_12 = 5;
pub const PARTITION_HORZ_A: C2RustUnnamed_12 = 4;
pub const PARTITION_SPLIT: C2RustUnnamed_12 = 3;
pub const PARTITION_VERT: C2RustUnnamed_12 = 2;
pub const PARTITION_HORZ: C2RustUnnamed_12 = 1;
pub const PARTITION_NONE: C2RustUnnamed_12 = 0;
pub type PARTITION_TYPE = uint8_t;
pub type PARTITION_CONTEXT = core::ffi::c_char;
pub type TX_MODE = uint8_t;
pub type PLANE_TYPE = uint8_t;
pub type CFL_PRED_TYPE = uint8_t;
pub type CFL_ALLOWED_TYPE = uint8_t;
pub type C2RustUnnamed_13 = core::ffi::c_uint;
pub const INTRA_INVALID: C2RustUnnamed_13 = 25;
pub const INTRA_MODES: C2RustUnnamed_13 = 13;
pub const INTER_MODE_END: C2RustUnnamed_13 = 25;
pub const INTER_MODE_START: C2RustUnnamed_13 = 13;
pub const COMP_INTER_MODE_NUM: C2RustUnnamed_13 = 8;
pub const COMP_INTER_MODE_END: C2RustUnnamed_13 = 25;
pub const COMP_INTER_MODE_START: C2RustUnnamed_13 = 17;
pub const SINGLE_INTER_MODE_NUM: C2RustUnnamed_13 = 4;
pub const SINGLE_INTER_MODE_END: C2RustUnnamed_13 = 17;
pub const SINGLE_INTER_MODE_START: C2RustUnnamed_13 = 13;
pub const INTRA_MODE_NUM: C2RustUnnamed_13 = 13;
pub const DIR_MODE_END: C2RustUnnamed_13 = 9;
pub const DIR_MODE_START: C2RustUnnamed_13 = 1;
pub const INTRA_MODE_END: C2RustUnnamed_13 = 13;
pub const INTRA_MODE_START: C2RustUnnamed_13 = 0;
pub const PRED_MODE_INVALID: C2RustUnnamed_13 = 25;
pub const MB_MODE_COUNT: C2RustUnnamed_13 = 25;
pub const NEW_NEWMV: C2RustUnnamed_13 = 24;
pub const GLOBAL_GLOBALMV: C2RustUnnamed_13 = 23;
pub const NEW_NEARMV: C2RustUnnamed_13 = 22;
pub const NEAR_NEWMV: C2RustUnnamed_13 = 21;
pub const NEW_NEARESTMV: C2RustUnnamed_13 = 20;
pub const NEAREST_NEWMV: C2RustUnnamed_13 = 19;
pub const NEAR_NEARMV: C2RustUnnamed_13 = 18;
pub const NEAREST_NEARESTMV: C2RustUnnamed_13 = 17;
pub const NEWMV: C2RustUnnamed_13 = 16;
pub const GLOBALMV: C2RustUnnamed_13 = 15;
pub const NEARMV: C2RustUnnamed_13 = 14;
pub const NEARESTMV: C2RustUnnamed_13 = 13;
pub const PAETH_PRED: C2RustUnnamed_13 = 12;
pub const SMOOTH_H_PRED: C2RustUnnamed_13 = 11;
pub const SMOOTH_V_PRED: C2RustUnnamed_13 = 10;
pub const SMOOTH_PRED: C2RustUnnamed_13 = 9;
pub const D67_PRED: C2RustUnnamed_13 = 8;
pub const D203_PRED: C2RustUnnamed_13 = 7;
pub const D157_PRED: C2RustUnnamed_13 = 6;
pub const D113_PRED: C2RustUnnamed_13 = 5;
pub const D135_PRED: C2RustUnnamed_13 = 4;
pub const D45_PRED: C2RustUnnamed_13 = 3;
pub const H_PRED: C2RustUnnamed_13 = 2;
pub const V_PRED: C2RustUnnamed_13 = 1;
pub const DC_PRED: C2RustUnnamed_13 = 0;
pub type PREDICTION_MODE = uint8_t;
pub type C2RustUnnamed_14 = core::ffi::c_uint;
pub const UV_MODE_INVALID: C2RustUnnamed_14 = 15;
pub const UV_INTRA_MODES: C2RustUnnamed_14 = 14;
pub const UV_CFL_PRED: C2RustUnnamed_14 = 13;
pub const UV_PAETH_PRED: C2RustUnnamed_14 = 12;
pub const UV_SMOOTH_H_PRED: C2RustUnnamed_14 = 11;
pub const UV_SMOOTH_V_PRED: C2RustUnnamed_14 = 10;
pub const UV_SMOOTH_PRED: C2RustUnnamed_14 = 9;
pub const UV_D67_PRED: C2RustUnnamed_14 = 8;
pub const UV_D203_PRED: C2RustUnnamed_14 = 7;
pub const UV_D157_PRED: C2RustUnnamed_14 = 6;
pub const UV_D113_PRED: C2RustUnnamed_14 = 5;
pub const UV_D135_PRED: C2RustUnnamed_14 = 4;
pub const UV_D45_PRED: C2RustUnnamed_14 = 3;
pub const UV_H_PRED: C2RustUnnamed_14 = 2;
pub const UV_V_PRED: C2RustUnnamed_14 = 1;
pub const UV_DC_PRED: C2RustUnnamed_14 = 0;
pub type UV_PREDICTION_MODE = uint8_t;
pub type MOTION_MODE = uint8_t;
pub type INTERINTRA_MODE = uint8_t;
pub type COMPOUND_TYPE = uint8_t;
pub type C2RustUnnamed_15 = core::ffi::c_uint;
pub const FILTER_INTRA_MODES: C2RustUnnamed_15 = 5;
pub const FILTER_PAETH_PRED: C2RustUnnamed_15 = 4;
pub const FILTER_D157_PRED: C2RustUnnamed_15 = 3;
pub const FILTER_H_PRED: C2RustUnnamed_15 = 2;
pub const FILTER_V_PRED: C2RustUnnamed_15 = 1;
pub const FILTER_DC_PRED: C2RustUnnamed_15 = 0;
pub type FILTER_INTRA_MODE = uint8_t;
pub type AV1_LEVEL = uint8_t;
pub type TXFM_CONTEXT = uint8_t;
pub type C2RustUnnamed_16 = core::ffi::c_int;
pub const SINGLE_REFS: C2RustUnnamed_16 = 7;
pub const BWD_REFS: C2RustUnnamed_16 = 3;
pub const FWD_REFS: C2RustUnnamed_16 = 4;
pub const INTER_REFS_PER_FRAME: C2RustUnnamed_16 = 7;
pub const EXTREF_FRAME: C2RustUnnamed_16 = 8;
pub const REF_FRAMES: C2RustUnnamed_16 = 8;
pub const ALTREF_FRAME: C2RustUnnamed_16 = 7;
pub const ALTREF2_FRAME: C2RustUnnamed_16 = 6;
pub const BWDREF_FRAME: C2RustUnnamed_16 = 5;
pub const GOLDEN_FRAME: C2RustUnnamed_16 = 4;
pub const LAST3_FRAME: C2RustUnnamed_16 = 3;
pub const LAST2_FRAME: C2RustUnnamed_16 = 2;
pub const LAST_FRAME: C2RustUnnamed_16 = 1;
pub const INTRA_FRAME: C2RustUnnamed_16 = 0;
pub const NONE_FRAME: C2RustUnnamed_16 = -1;
pub type MV_REFERENCE_FRAME = int8_t;
pub type RestorationType = core::ffi::c_uint;
pub const RESTORE_TYPES: RestorationType = 4;
pub const RESTORE_SWITCHABLE_TYPES: RestorationType = 3;
pub const RESTORE_SWITCHABLE: RestorationType = 3;
pub const RESTORE_SGRPROJ: RestorationType = 2;
pub const RESTORE_WIENER: RestorationType = 1;
pub const RESTORE_NONE: RestorationType = 0;
pub type aom_cdf_prob = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation {
    pub enabled: uint8_t,
    pub update_map: uint8_t,
    pub update_data: uint8_t,
    pub temporal_update: uint8_t,
    pub feature_data: [[int16_t; 8]; 8],
    pub feature_mask: [core::ffi::c_uint; 8],
    pub last_active_segid: core::ffi::c_int,
    pub segid_preskip: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segmentation_probs {
    pub pred_cdf: [[aom_cdf_prob; 3]; 3],
    pub spatial_pred_seg_cdf: [[aom_cdf_prob; 9]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AV1Common {
    pub current_frame: CurrentFrame,
    pub error: *mut aom_internal_error_info,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub superres_upscaled_width: core::ffi::c_int,
    pub superres_upscaled_height: core::ffi::c_int,
    pub superres_scale_denominator: uint8_t,
    pub buffer_removal_times: [uint32_t; 33],
    pub frame_presentation_time: uint32_t,
    pub prev_frame: *mut RefCntBuffer,
    pub cur_frame: *mut RefCntBuffer,
    pub remapped_ref_idx: [core::ffi::c_int; 8],
    pub sf_identity: scale_factors,
    pub ref_scale_factors: [scale_factors; 8],
    pub ref_frame_map: [*mut RefCntBuffer; 8],
    pub show_frame: core::ffi::c_int,
    pub showable_frame: core::ffi::c_int,
    pub show_existing_frame: core::ffi::c_int,
    pub features: FeatureFlags,
    pub mi_params: CommonModeInfoParams,
    pub quant_params: CommonQuantParams,
    pub seg: segmentation,
    pub last_frame_seg_map: *mut uint8_t,
    pub lf_info: loop_filter_info_n,
    pub lf: loopfilter,
    pub rst_info: [RestorationInfo; 3],
    pub rst_tmpbuf: *mut int32_t,
    pub rlbs: *mut RestorationLineBuffers,
    pub rst_frame: YV12_BUFFER_CONFIG,
    pub cdef_info: CdefInfo,
    pub film_grain_params: aom_film_grain_t,
    pub delta_q_info: DeltaQInfo,
    pub global_motion: [WarpedMotionParams; 8],
    pub seq_params: *mut SequenceHeader,
    pub fc: *mut FRAME_CONTEXT,
    pub default_frame_context: *mut FRAME_CONTEXT,
    pub tiles: CommonTileParams,
    pub buffer_pool: *mut BufferPool,
    pub above_contexts: CommonContexts,
    pub current_frame_id: core::ffi::c_int,
    pub ref_frame_id: [core::ffi::c_int; 8],
    pub tpl_mvs: *mut TPL_MV_REF,
    pub tpl_mvs_mem_size: core::ffi::c_int,
    pub ref_frame_sign_bias: [core::ffi::c_int; 8],
    pub ref_frame_side: [int8_t; 8],
    pub temporal_layer_id: core::ffi::c_int,
    pub spatial_layer_id: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPL_MV_REF {
    pub mfmv0: int_mv,
    pub ref_frame_offset: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_mv {
    pub as_int: uint32_t,
    pub as_mv: MV,
    pub as_fullmv: FULLPEL_MV,
}
pub type FULLPEL_MV = fullpel_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fullpel_mv {
    pub row: int16_t,
    pub col: int16_t,
}
pub type MV = mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mv {
    pub row: int16_t,
    pub col: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonContexts {
    pub partition: *mut *mut PARTITION_CONTEXT,
    pub entropy: [*mut *mut ENTROPY_CONTEXT; 3],
    pub txfm: *mut *mut TXFM_CONTEXT,
    pub num_planes: core::ffi::c_int,
    pub num_tile_rows: core::ffi::c_int,
    pub num_mi_cols: core::ffi::c_int,
}
pub type ENTROPY_CONTEXT = core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferPool {
    pub pool_mutex: pthread_mutex_t,
    pub cb_priv: *mut core::ffi::c_void,
    pub get_fb_cb: aom_get_frame_buffer_cb_fn_t,
    pub release_fb_cb: aom_release_frame_buffer_cb_fn_t,
    pub frame_bufs: *mut RefCntBuffer,
    pub num_frame_bufs: uint8_t,
    pub int_frame_buffers: InternalFrameBufferList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBufferList {
    pub num_internal_frame_buffers: core::ffi::c_int,
    pub int_fb: *mut InternalFrameBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBuffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub in_use: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RefCntBuffer {
    pub ref_count: core::ffi::c_int,
    pub order_hint: core::ffi::c_uint,
    pub ref_order_hints: [core::ffi::c_uint; 7],
    pub display_order_hint: core::ffi::c_uint,
    pub ref_display_order_hint: [core::ffi::c_uint; 7],
    pub pyramid_level: core::ffi::c_uint,
    pub base_qindex: core::ffi::c_int,
    pub mvs: *mut MV_REF,
    pub seg_map: *mut uint8_t,
    pub seg: segmentation,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub global_motion: [WarpedMotionParams; 8],
    pub showable_frame: core::ffi::c_int,
    pub film_grain_params_present: uint8_t,
    pub film_grain_params: aom_film_grain_t,
    pub raw_frame_buffer: aom_codec_frame_buffer_t,
    pub buf: YV12_BUFFER_CONFIG,
    pub temporal_id: core::ffi::c_int,
    pub spatial_id: core::ffi::c_int,
    pub frame_type: FRAME_TYPE,
    pub interp_filter_selected: [core::ffi::c_int; 4],
    pub ref_deltas: [int8_t; 8],
    pub mode_deltas: [int8_t; 2],
    pub frame_context: FRAME_CONTEXT,
    pub filter_level: [core::ffi::c_int; 2],
}
pub type FRAME_CONTEXT = frame_contexts;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_contexts {
    pub txb_skip_cdf: [[[aom_cdf_prob; 3]; 13]; 5],
    pub eob_extra_cdf: [[[[aom_cdf_prob; 3]; 9]; 2]; 5],
    pub dc_sign_cdf: [[[aom_cdf_prob; 3]; 3]; 2],
    pub eob_flag_cdf16: [[[aom_cdf_prob; 6]; 2]; 2],
    pub eob_flag_cdf32: [[[aom_cdf_prob; 7]; 2]; 2],
    pub eob_flag_cdf64: [[[aom_cdf_prob; 8]; 2]; 2],
    pub eob_flag_cdf128: [[[aom_cdf_prob; 9]; 2]; 2],
    pub eob_flag_cdf256: [[[aom_cdf_prob; 10]; 2]; 2],
    pub eob_flag_cdf512: [[[aom_cdf_prob; 11]; 2]; 2],
    pub eob_flag_cdf1024: [[[aom_cdf_prob; 12]; 2]; 2],
    pub coeff_base_eob_cdf: [[[[aom_cdf_prob; 4]; 4]; 2]; 5],
    pub coeff_base_cdf: [[[[aom_cdf_prob; 5]; 42]; 2]; 5],
    pub coeff_br_cdf: [[[[aom_cdf_prob; 5]; 21]; 2]; 5],
    pub newmv_cdf: [[aom_cdf_prob; 3]; 6],
    pub zeromv_cdf: [[aom_cdf_prob; 3]; 2],
    pub refmv_cdf: [[aom_cdf_prob; 3]; 6],
    pub drl_cdf: [[aom_cdf_prob; 3]; 3],
    pub inter_compound_mode_cdf: [[aom_cdf_prob; 9]; 8],
    pub compound_type_cdf: [[aom_cdf_prob; 3]; 22],
    pub wedge_idx_cdf: [[aom_cdf_prob; 17]; 22],
    pub interintra_cdf: [[aom_cdf_prob; 3]; 4],
    pub wedge_interintra_cdf: [[aom_cdf_prob; 3]; 22],
    pub interintra_mode_cdf: [[aom_cdf_prob; 5]; 4],
    pub motion_mode_cdf: [[aom_cdf_prob; 4]; 22],
    pub obmc_cdf: [[aom_cdf_prob; 3]; 22],
    pub palette_y_size_cdf: [[aom_cdf_prob; 8]; 7],
    pub palette_uv_size_cdf: [[aom_cdf_prob; 8]; 7],
    pub palette_y_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7],
    pub palette_uv_color_index_cdf: [[[aom_cdf_prob; 9]; 5]; 7],
    pub palette_y_mode_cdf: [[[aom_cdf_prob; 3]; 3]; 7],
    pub palette_uv_mode_cdf: [[aom_cdf_prob; 3]; 2],
    pub comp_inter_cdf: [[aom_cdf_prob; 3]; 5],
    pub single_ref_cdf: [[[aom_cdf_prob; 3]; 6]; 3],
    pub comp_ref_type_cdf: [[aom_cdf_prob; 3]; 5],
    pub uni_comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3],
    pub comp_ref_cdf: [[[aom_cdf_prob; 3]; 3]; 3],
    pub comp_bwdref_cdf: [[[aom_cdf_prob; 3]; 2]; 3],
    pub txfm_partition_cdf: [[aom_cdf_prob; 3]; 21],
    pub compound_index_cdf: [[aom_cdf_prob; 3]; 6],
    pub comp_group_idx_cdf: [[aom_cdf_prob; 3]; 6],
    pub skip_mode_cdfs: [[aom_cdf_prob; 3]; 3],
    pub skip_txfm_cdfs: [[aom_cdf_prob; 3]; 3],
    pub intra_inter_cdf: [[aom_cdf_prob; 3]; 4],
    pub nmvc: nmv_context,
    pub ndvc: nmv_context,
    pub intrabc_cdf: [aom_cdf_prob; 3],
    pub seg: segmentation_probs,
    pub filter_intra_cdfs: [[aom_cdf_prob; 3]; 22],
    pub filter_intra_mode_cdf: [aom_cdf_prob; 6],
    pub switchable_restore_cdf: [aom_cdf_prob; 4],
    pub wiener_restore_cdf: [aom_cdf_prob; 3],
    pub sgrproj_restore_cdf: [aom_cdf_prob; 3],
    pub y_mode_cdf: [[aom_cdf_prob; 14]; 4],
    pub uv_mode_cdf: [[[aom_cdf_prob; 15]; 13]; 2],
    pub partition_cdf: [[aom_cdf_prob; 11]; 20],
    pub switchable_interp_cdf: [[aom_cdf_prob; 4]; 16],
    pub kf_y_cdf: [[[aom_cdf_prob; 14]; 5]; 5],
    pub angle_delta_cdf: [[aom_cdf_prob; 8]; 8],
    pub tx_size_cdf: [[[aom_cdf_prob; 4]; 3]; 4],
    pub delta_q_cdf: [aom_cdf_prob; 5],
    pub delta_lf_multi_cdf: [[aom_cdf_prob; 5]; 4],
    pub delta_lf_cdf: [aom_cdf_prob; 5],
    pub intra_ext_tx_cdf: [[[[aom_cdf_prob; 17]; 13]; 4]; 3],
    pub inter_ext_tx_cdf: [[[aom_cdf_prob; 17]; 4]; 4],
    pub cfl_sign_cdf: [aom_cdf_prob; 9],
    pub cfl_alpha_cdf: [[aom_cdf_prob; 17]; 6],
    pub initialized: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmv_context {
    pub joints_cdf: [aom_cdf_prob; 5],
    pub comps: [nmv_component; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nmv_component {
    pub classes_cdf: [aom_cdf_prob; 12],
    pub class0_fp_cdf: [[aom_cdf_prob; 5]; 2],
    pub fp_cdf: [aom_cdf_prob; 5],
    pub sign_cdf: [aom_cdf_prob; 3],
    pub class0_hp_cdf: [aom_cdf_prob; 3],
    pub hp_cdf: [aom_cdf_prob; 3],
    pub class0_cdf: [aom_cdf_prob; 3],
    pub bits_cdf: [[aom_cdf_prob; 3]; 10],
}
pub type FRAME_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_t {
    pub apply_grain: core::ffi::c_int,
    pub update_parameters: core::ffi::c_int,
    pub scaling_points_y: [[core::ffi::c_int; 2]; 14],
    pub num_y_points: core::ffi::c_int,
    pub scaling_points_cb: [[core::ffi::c_int; 2]; 10],
    pub num_cb_points: core::ffi::c_int,
    pub scaling_points_cr: [[core::ffi::c_int; 2]; 10],
    pub num_cr_points: core::ffi::c_int,
    pub scaling_shift: core::ffi::c_int,
    pub ar_coeff_lag: core::ffi::c_int,
    pub ar_coeffs_y: [core::ffi::c_int; 24],
    pub ar_coeffs_cb: [core::ffi::c_int; 25],
    pub ar_coeffs_cr: [core::ffi::c_int; 25],
    pub ar_coeff_shift: core::ffi::c_int,
    pub cb_mult: core::ffi::c_int,
    pub cb_luma_mult: core::ffi::c_int,
    pub cb_offset: core::ffi::c_int,
    pub cr_mult: core::ffi::c_int,
    pub cr_luma_mult: core::ffi::c_int,
    pub cr_offset: core::ffi::c_int,
    pub overlap_flag: core::ffi::c_int,
    pub clip_to_restricted_range: core::ffi::c_int,
    pub bit_depth: core::ffi::c_uint,
    pub chroma_scaling_from_luma: core::ffi::c_int,
    pub grain_scale_shift: core::ffi::c_int,
    pub random_seed: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WarpedMotionParams {
    pub wmmat: [int32_t; 6],
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
    pub wmtype: TransformationType,
    pub invalid: int8_t,
}
pub type TransformationType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MV_REF {
    pub mv: int_mv,
    pub ref_frame: MV_REFERENCE_FRAME,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonTileParams {
    pub cols: core::ffi::c_int,
    pub rows: core::ffi::c_int,
    pub max_width_sb: core::ffi::c_int,
    pub max_height_sb: core::ffi::c_int,
    pub min_inner_width: core::ffi::c_int,
    pub uniform_spacing: core::ffi::c_int,
    pub log2_cols: core::ffi::c_int,
    pub log2_rows: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub min_log2_cols: core::ffi::c_int,
    pub min_log2_rows: core::ffi::c_int,
    pub max_log2_cols: core::ffi::c_int,
    pub max_log2_rows: core::ffi::c_int,
    pub min_log2: core::ffi::c_int,
    pub col_start_sb: [core::ffi::c_int; 65],
    pub row_start_sb: [core::ffi::c_int; 65],
    pub large_scale: core::ffi::c_uint,
    pub single_tile_decoding: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SequenceHeader {
    pub num_bits_width: core::ffi::c_int,
    pub num_bits_height: core::ffi::c_int,
    pub max_frame_width: core::ffi::c_int,
    pub max_frame_height: core::ffi::c_int,
    pub frame_id_numbers_present_flag: uint8_t,
    pub frame_id_length: core::ffi::c_int,
    pub delta_frame_id_length: core::ffi::c_int,
    pub sb_size: BLOCK_SIZE,
    pub mib_size: core::ffi::c_int,
    pub mib_size_log2: core::ffi::c_int,
    pub order_hint_info: OrderHintInfo,
    pub force_screen_content_tools: uint8_t,
    pub still_picture: uint8_t,
    pub reduced_still_picture_hdr: uint8_t,
    pub force_integer_mv: uint8_t,
    pub enable_filter_intra: uint8_t,
    pub enable_intra_edge_filter: uint8_t,
    pub enable_interintra_compound: uint8_t,
    pub enable_masked_compound: uint8_t,
    pub enable_dual_filter: uint8_t,
    pub enable_warped_motion: uint8_t,
    pub enable_superres: uint8_t,
    pub enable_cdef: uint8_t,
    pub enable_restoration: uint8_t,
    pub profile: BITSTREAM_PROFILE,
    pub bit_depth: aom_bit_depth_t,
    pub use_highbitdepth: uint8_t,
    pub monochrome: uint8_t,
    pub color_primaries: aom_color_primaries_t,
    pub transfer_characteristics: aom_transfer_characteristics_t,
    pub matrix_coefficients: aom_matrix_coefficients_t,
    pub color_range: core::ffi::c_int,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub chroma_sample_position: aom_chroma_sample_position_t,
    pub separate_uv_delta_q: uint8_t,
    pub film_grain_params_present: uint8_t,
    pub operating_points_cnt_minus_1: core::ffi::c_int,
    pub operating_point_idc: [core::ffi::c_int; 32],
    pub has_nonzero_operating_point_idc: bool,
    pub timing_info_present: core::ffi::c_int,
    pub timing_info: aom_timing_info_t,
    pub decoder_model_info_present_flag: uint8_t,
    pub decoder_model_info: aom_dec_model_info_t,
    pub display_model_info_present_flag: uint8_t,
    pub seq_level_idx: [AV1_LEVEL; 32],
    pub tier: [uint8_t; 32],
    pub op_params: [aom_dec_model_op_parameters_t; 33],
}
pub type aom_dec_model_op_parameters_t = aom_dec_model_op_parameters;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_op_parameters {
    pub decoder_model_param_present_flag: core::ffi::c_int,
    pub bitrate: int64_t,
    pub buffer_size: int64_t,
    pub decoder_buffer_delay: uint32_t,
    pub encoder_buffer_delay: uint32_t,
    pub low_delay_mode_flag: core::ffi::c_int,
    pub display_model_param_present_flag: core::ffi::c_int,
    pub initial_display_delay: core::ffi::c_int,
}
pub type aom_dec_model_info_t = aom_dec_model_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_info {
    pub num_units_in_decoding_tick: uint32_t,
    pub encoder_decoder_buffer_delay_length: core::ffi::c_int,
    pub buffer_removal_time_length: core::ffi::c_int,
    pub frame_presentation_time_length: core::ffi::c_int,
}
pub type aom_timing_info_t = aom_timing;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_timing {
    pub num_units_in_display_tick: uint32_t,
    pub time_scale: uint32_t,
    pub equal_picture_interval: core::ffi::c_int,
    pub num_ticks_per_picture: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OrderHintInfo {
    pub enable_order_hint: core::ffi::c_int,
    pub order_hint_bits_minus_1: core::ffi::c_int,
    pub enable_dist_wtd_comp: core::ffi::c_int,
    pub enable_ref_frame_mvs: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeltaQInfo {
    pub delta_q_present_flag: core::ffi::c_int,
    pub delta_q_res: core::ffi::c_int,
    pub delta_lf_present_flag: core::ffi::c_int,
    pub delta_lf_res: core::ffi::c_int,
    pub delta_lf_multi: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdefInfo {
    pub colbuf: [*mut uint16_t; 3],
    pub linebuf: [*mut uint16_t; 3],
    pub srcbuf: *mut uint16_t,
    pub allocated_colbuf_size: [size_t; 3],
    pub allocated_linebuf_size: [size_t; 3],
    pub allocated_srcbuf_size: size_t,
    pub cdef_damping: core::ffi::c_int,
    pub nb_cdef_strengths: core::ffi::c_int,
    pub cdef_strengths: [core::ffi::c_int; 16],
    pub cdef_uv_strengths: [core::ffi::c_int; 16],
    pub cdef_bits: core::ffi::c_int,
    pub allocated_mi_rows: core::ffi::c_int,
    pub allocated_num_workers: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationLineBuffers {
    pub tmp_save_above: [[uint16_t; 392]; 3],
    pub tmp_save_below: [[uint16_t; 392]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationInfo {
    pub frame_restoration_type: RestorationType,
    pub restoration_unit_size: core::ffi::c_int,
    pub num_rest_units: core::ffi::c_int,
    pub vert_units: core::ffi::c_int,
    pub horz_units: core::ffi::c_int,
    pub unit_info: *mut RestorationUnitInfo,
    pub boundaries: RestorationStripeBoundaries,
    pub optimized_lr: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationStripeBoundaries {
    pub stripe_boundary_above: *mut uint8_t,
    pub stripe_boundary_below: *mut uint8_t,
    pub stripe_boundary_stride: core::ffi::c_int,
    pub stripe_boundary_size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RestorationUnitInfo {
    pub restoration_type: RestorationType,
    pub wiener_info: WienerInfo,
    pub sgrproj_info: SgrprojInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SgrprojInfo {
    pub ep: core::ffi::c_int,
    pub xqd: [core::ffi::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WienerInfo {
    pub __aom_align_vfilter: [crate::platform::Align16; 0],

    pub vfilter: InterpKernel,
    pub __aom_align_hfilter: [crate::platform::Align16; 0],
    pub hfilter: InterpKernel,
}
pub type InterpKernel = [int16_t; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loopfilter {
    pub filter_level: [core::ffi::c_int; 2],
    pub filter_level_u: core::ffi::c_int,
    pub filter_level_v: core::ffi::c_int,
    pub backup_filter_level: [core::ffi::c_int; 2],
    pub backup_filter_level_u: core::ffi::c_int,
    pub backup_filter_level_v: core::ffi::c_int,
    pub sharpness_level: core::ffi::c_int,
    pub mode_ref_delta_enabled: uint8_t,
    pub mode_ref_delta_update: uint8_t,
    pub ref_deltas: [int8_t; 8],
    pub mode_deltas: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_info_n {
    pub lfthr: [loop_filter_thresh; 64],
    pub lvl: [[[[[uint8_t; 2]; 8]; 2]; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_filter_thresh {
    pub __aom_align_mblim: [crate::platform::Align16; 0],

    pub mblim: [uint8_t; 16],
    pub __aom_align_lim: [crate::platform::Align16; 0],
    pub lim: [uint8_t; 16],
    pub __aom_align_hev_thr: [crate::platform::Align16; 0],
    pub hev_thr: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonQuantParams {
    pub base_qindex: core::ffi::c_int,
    pub sharpness: core::ffi::c_int,
    pub y_dc_delta_q: core::ffi::c_int,
    pub u_dc_delta_q: core::ffi::c_int,
    pub v_dc_delta_q: core::ffi::c_int,
    pub u_ac_delta_q: core::ffi::c_int,
    pub v_ac_delta_q: core::ffi::c_int,
    pub y_dequant_QTX: [[int16_t; 2]; 8],
    pub u_dequant_QTX: [[int16_t; 2]; 8],
    pub v_dequant_QTX: [[int16_t; 2]; 8],
    pub giqmatrix: [[[*const qm_val_t; 19]; 3]; 16],
    pub gqmatrix: [[[*const qm_val_t; 19]; 3]; 16],
    pub y_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub u_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub v_iqmatrix: [[*const qm_val_t; 19]; 8],
    pub using_qmatrix: bool,
    pub qmatrix_level_y: core::ffi::c_int,
    pub qmatrix_level_u: core::ffi::c_int,
    pub qmatrix_level_v: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonModeInfoParams {
    pub mb_rows: core::ffi::c_int,
    pub mb_cols: core::ffi::c_int,
    pub MBs: core::ffi::c_int,
    pub mi_rows: core::ffi::c_int,
    pub mi_cols: core::ffi::c_int,
    pub mi_alloc: *mut MB_MODE_INFO,
    pub mi_alloc_size: core::ffi::c_int,
    pub mi_alloc_stride: core::ffi::c_int,
    pub mi_alloc_bsize: BLOCK_SIZE,
    pub mi_grid_base: *mut *mut MB_MODE_INFO,
    pub mi_grid_size: core::ffi::c_int,
    pub mi_stride: core::ffi::c_int,
    pub tx_type_map: *mut TX_TYPE,
    pub free_mi: Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>,
    pub setup_mi: Option<unsafe extern "C-unwind" fn(*mut CommonModeInfoParams) -> ()>,
    pub set_mb_mi: Option<
        unsafe extern "C-unwind" fn(
            *mut CommonModeInfoParams,
            core::ffi::c_int,
            core::ffi::c_int,
            BLOCK_SIZE,
        ) -> (),
    >,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct MB_MODE_INFO {
    pub bsize: BLOCK_SIZE,
    pub partition: PARTITION_TYPE,
    pub mode: PREDICTION_MODE,
    pub uv_mode: UV_PREDICTION_MODE,
    pub current_qindex: core::ffi::c_int,
    pub mv: [int_mv; 2],
    pub ref_frame: [MV_REFERENCE_FRAME; 2],
    pub interp_filters: int_interpfilters,
    pub motion_mode: MOTION_MODE,
    pub num_proj_ref: uint8_t,
    pub overlappable_neighbors: uint8_t,
    pub wm_params: WarpedMotionParams,
    pub interintra_mode: INTERINTRA_MODE,
    pub interintra_wedge_index: int8_t,
    pub interinter_comp: INTERINTER_COMPOUND_DATA,
    pub angle_delta: [int8_t; 2],
    pub filter_intra_mode_info: FILTER_INTRA_MODE_INFO,
    pub cfl_alpha_signs: int8_t,
    pub cfl_alpha_idx: uint8_t,
    pub palette_mode_info: PALETTE_MODE_INFO,
    pub skip_txfm: uint8_t,
    pub tx_size: TX_SIZE,
    pub inter_tx_size: [TX_SIZE; 16],
    pub delta_lf_from_base: int8_t,
    pub delta_lf: [int8_t; 4],
    #[bitfield(name = "segment_id", ty = "uint8_t", bits = "0..=2")]
    #[bitfield(name = "seg_id_predicted", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "ref_mv_idx", ty = "uint8_t", bits = "4..=5")]
    #[bitfield(name = "skip_mode", ty = "uint8_t", bits = "6..=6")]
    #[bitfield(name = "use_intrabc", ty = "uint8_t", bits = "7..=7")]
    #[bitfield(name = "comp_group_idx", ty = "uint8_t", bits = "8..=8")]
    #[bitfield(name = "compound_idx", ty = "uint8_t", bits = "9..=9")]
    #[bitfield(name = "use_wedge_interintra", ty = "uint8_t", bits = "10..=10")]
    #[bitfield(name = "cdef_strength", ty = "int8_t", bits = "11..=14")]
    pub segment_id_seg_id_predicted_ref_mv_idx_skip_mode_use_intrabc_comp_group_idx_compound_idx_use_wedge_interintra_cdef_strength: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTE_MODE_INFO {
    pub palette_colors: [uint16_t; 24],
    pub palette_size: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILTER_INTRA_MODE_INFO {
    pub filter_intra_mode: FILTER_INTRA_MODE,
    pub use_filter_intra: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct INTERINTER_COMPOUND_DATA {
    pub seg_mask: *mut uint8_t,
    pub wedge_index: int8_t,
    pub wedge_sign: int8_t,
    pub mask_type: DIFFWTD_MASK_TYPE,
    pub type_0: COMPOUND_TYPE,
}
pub type DIFFWTD_MASK_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union int_interpfilters {
    pub as_int: uint32_t,
    pub as_filters: InterpFilters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterpFilters {
    pub y_filter: uint16_t,
    pub x_filter: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FeatureFlags {
    pub disable_cdf_update: bool,
    pub allow_high_precision_mv: bool,
    pub cur_frame_force_integer_mv: bool,
    pub allow_screen_content_tools: bool,
    pub allow_intrabc: bool,
    pub allow_warped_motion: bool,
    pub allow_ref_frame_mvs: bool,
    pub coded_lossless: bool,
    pub all_lossless: bool,
    pub reduced_tx_set_used: bool,
    pub error_resilient_mode: bool,
    pub switchable_motion_mode: bool,
    pub tx_mode: TX_MODE,
    pub interp_filter: InterpFilter,
    pub primary_ref_frame: core::ffi::c_int,
    pub byte_alignment: core::ffi::c_int,
    pub refresh_frame_context: REFRESH_FRAME_CONTEXT_MODE,
}
pub type REFRESH_FRAME_CONTEXT_MODE = uint8_t;
pub type InterpFilter = core::ffi::c_uchar;
pub const INTERP_INVALID: InterpFilter = 255;
pub const EXTRA_FILTERS: InterpFilter = 2;
pub const SWITCHABLE: InterpFilter = 4;
pub const SWITCHABLE_FILTERS: InterpFilter = 3;
pub const INTERP_FILTERS_ALL: InterpFilter = 5;
pub const MULTITAP_SHARP2: InterpFilter = 4;
pub const BILINEAR: InterpFilter = 3;
pub const MULTITAP_SHARP: InterpFilter = 2;
pub const EIGHTTAP_SMOOTH: InterpFilter = 1;
pub const EIGHTTAP_REGULAR: InterpFilter = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scale_factors {
    pub x_scale_fp: core::ffi::c_int,
    pub y_scale_fp: core::ffi::c_int,
    pub x_step_q4: core::ffi::c_int,
    pub y_step_q4: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
pub type jmp_buf = [core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CurrentFrame {
    pub frame_type: FRAME_TYPE,
    pub reference_mode: REFERENCE_MODE,
    pub order_hint: core::ffi::c_uint,
    pub display_order_hint: core::ffi::c_uint,
    pub pyramid_level: core::ffi::c_uint,
    pub frame_number: core::ffi::c_uint,
    pub skip_mode_info: SkipModeInfo,
    pub refresh_frame_flags: core::ffi::c_int,
    pub frame_refs_short_signaling: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SkipModeInfo {
    pub skip_mode_allowed: core::ffi::c_int,
    pub skip_mode_flag: core::ffi::c_int,
    pub ref_frame_idx_0: core::ffi::c_int,
    pub ref_frame_idx_1: core::ffi::c_int,
}
pub type REFERENCE_MODE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd {
    pub mi_row: core::ffi::c_int,
    pub mi_col: core::ffi::c_int,
    pub mi_stride: core::ffi::c_int,
    pub is_chroma_ref: bool,
    pub plane: [macroblockd_plane; 3],
    pub tile: TileInfo,
    pub mi: *mut *mut MB_MODE_INFO,
    pub up_available: bool,
    pub left_available: bool,
    pub chroma_up_available: bool,
    pub chroma_left_available: bool,
    pub left_mbmi: *mut MB_MODE_INFO,
    pub above_mbmi: *mut MB_MODE_INFO,
    pub chroma_left_mbmi: *mut MB_MODE_INFO,
    pub chroma_above_mbmi: *mut MB_MODE_INFO,
    pub tx_type_map: *mut uint8_t,
    pub tx_type_map_stride: core::ffi::c_int,
    pub mb_to_left_edge: core::ffi::c_int,
    pub mb_to_right_edge: core::ffi::c_int,
    pub mb_to_top_edge: core::ffi::c_int,
    pub mb_to_bottom_edge: core::ffi::c_int,
    pub block_ref_scale_factors: [*const scale_factors; 2],
    pub cur_buf: *const YV12_BUFFER_CONFIG,
    pub above_entropy_context: [*mut ENTROPY_CONTEXT; 3],
    pub left_entropy_context: [[ENTROPY_CONTEXT; 32]; 3],
    pub above_partition_context: *mut PARTITION_CONTEXT,
    pub left_partition_context: [PARTITION_CONTEXT; 32],
    pub above_txfm_context: *mut TXFM_CONTEXT,
    pub left_txfm_context: *mut TXFM_CONTEXT,
    pub left_txfm_context_buffer: [TXFM_CONTEXT; 32],
    pub wiener_info: [WienerInfo; 3],
    pub sgrproj_info: [SgrprojInfo; 3],
    pub width: uint8_t,
    pub height: uint8_t,
    pub ref_mv_stack: [[CANDIDATE_MV; 8]; 29],
    pub weight: [[uint16_t; 8]; 29],
    pub is_last_vertical_rect: bool,
    pub is_first_horizontal_rect: bool,
    pub neighbors_ref_counts: [uint8_t; 8],
    pub tile_ctx: *mut FRAME_CONTEXT,
    pub bd: core::ffi::c_int,
    pub qindex: [core::ffi::c_int; 8],
    pub lossless: [core::ffi::c_int; 8],
    pub current_base_qindex: core::ffi::c_int,
    pub cur_frame_force_integer_mv: core::ffi::c_int,
    pub error_info: *mut aom_internal_error_info,
    pub global_motion: *const WarpedMotionParams,
    pub delta_lf_from_base: int8_t,
    pub delta_lf: [int8_t; 4],
    pub cdef_transmitted: [bool; 4],
    pub seg_mask: *mut uint8_t,
    pub cfl: CFL_CTX,
    pub color_index_map_offset: [uint16_t; 2],
    pub tmp_conv_dst: *mut CONV_BUF_TYPE,
    pub tmp_obmc_bufs: [*mut uint8_t; 2],
    pub tmp_upsample_pred: *mut uint8_t,
}
pub type CONV_BUF_TYPE = uint16_t;
pub type CFL_CTX = cfl_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfl_ctx {
    pub recon_buf_q3: [uint16_t; 1024],
    pub ac_buf_q3: [int16_t; 1024],
    pub dc_pred_is_cached: [bool; 2],
    pub use_dc_pred_cache: bool,
    pub dc_pred_cache: [[int16_t; 32]; 2],
    pub buf_height: core::ffi::c_int,
    pub buf_width: core::ffi::c_int,
    pub are_parameters_computed: core::ffi::c_int,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub store_y: core::ffi::c_int,
}
pub type CANDIDATE_MV = candidate_mv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct candidate_mv {
    pub this_mv: int_mv,
    pub comp_mv: int_mv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TileInfo {
    pub mi_row_start: core::ffi::c_int,
    pub mi_row_end: core::ffi::c_int,
    pub mi_col_start: core::ffi::c_int,
    pub mi_col_end: core::ffi::c_int,
    pub tile_row: core::ffi::c_int,
    pub tile_col: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct macroblockd_plane {
    pub plane_type: PLANE_TYPE,
    pub subsampling_x: core::ffi::c_int,
    pub subsampling_y: core::ffi::c_int,
    pub dst: buf_2d,
    pub pre: [buf_2d; 2],
    pub above_entropy_context: *mut ENTROPY_CONTEXT,
    pub left_entropy_context: *mut ENTROPY_CONTEXT,
    pub seg_dequant_QTX: [[int16_t; 2]; 8],
    pub color_index_map: *mut uint8_t,
    pub width: uint8_t,
    pub height: uint8_t,
    pub seg_iqmatrix: [[*mut qm_val_t; 19]; 8],
    pub seg_qmatrix: [[*mut qm_val_t; 19]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_2d {
    pub buf: *mut uint8_t,
    pub buf0: *mut uint8_t,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
}
pub type pthread_once_t = libc::pthread_once_t;
pub type MACROBLOCKD = macroblockd;
pub type AV1_COMMON = AV1Common;
pub type intra_high_pred_fn = Option<
    unsafe extern "C-unwind" fn(
        *mut uint16_t,
        ptrdiff_t,
        *const uint16_t,
        *const uint16_t,
        core::ffi::c_int,
    ) -> (),
>;
pub type intra_pred_fn = Option<
    unsafe extern "C-unwind" fn(*mut uint8_t, ptrdiff_t, *const uint8_t, *const uint8_t) -> (),
>;
pub const NEED_BOTTOMLEFT: C2RustUnnamed_17 = 32;
pub const NEED_ABOVELEFT: C2RustUnnamed_17 = 16;
pub const NEED_ABOVE: C2RustUnnamed_17 = 4;
pub const NEED_LEFT: C2RustUnnamed_17 = 2;
pub const NEED_ABOVERIGHT: C2RustUnnamed_17 = 8;
pub type C2RustUnnamed_17 = core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clip_pixel(mut val: core::ffi::c_int) -> uint8_t {
    return (if val > 255 as core::ffi::c_int {
        255 as core::ffi::c_int
    } else if val < 0 as core::ffi::c_int {
        0 as core::ffi::c_int
    } else {
        val
    }) as uint8_t;
}
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
#[inline]
unsafe extern "C-unwind" fn clip_pixel_highbd(
    mut val: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) -> uint16_t {
    match bd {
        10 => {
            return clamp(val, 0 as core::ffi::c_int, 1023 as core::ffi::c_int)
                as uint16_t;
        }
        12 => {
            return clamp(val, 0 as core::ffi::c_int, 4095 as core::ffi::c_int)
                as uint16_t;
        }
        8 | _ => {
            return clamp(val, 0 as core::ffi::c_int, 255 as core::ffi::c_int) as uint16_t;
        }
    };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AOM_PLANE_U: core::ffi::c_int = 1 as core::ffi::c_int;
pub const YV12_FLAG_HIGHBITDEPTH: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_SB_SIZE_LOG2: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MI_SIZE_LOG2: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_MIB_SIZE_LOG2: core::ffi::c_int = MAX_SB_SIZE_LOG2 - MI_SIZE_LOG2;
pub const PALETTE_MAX_SIZE: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_TX_SIZE_LOG2: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_TX_SIZE: core::ffi::c_int = (1 as core::ffi::c_int) << MAX_TX_SIZE_LOG2;
pub const ANGLE_STEP: core::ffi::c_int = 3 as core::ffi::c_int;
static mut mi_size_wide_log2: [uint8_t; 22] = [
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
];
static mut mi_size_high_log2: [uint8_t; 22] = [
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    3 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
];
static mut mi_size_wide: [uint8_t; 22] = [
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
];
static mut mi_size_high: [uint8_t; 22] = [
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    2 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
];
static mut block_size_wide: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
];
static mut block_size_high: [uint8_t; 22] = [
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    128 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
    32 as core::ffi::c_int as uint8_t,
    8 as core::ffi::c_int as uint8_t,
    64 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
];
static mut tx_size_wide: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
];
static mut tx_size_high: [core::ffi::c_int; 19] = [
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    32 as core::ffi::c_int,
    64 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    64 as core::ffi::c_int,
    32 as core::ffi::c_int,
    16 as core::ffi::c_int,
    4 as core::ffi::c_int,
    32 as core::ffi::c_int,
    8 as core::ffi::c_int,
    64 as core::ffi::c_int,
    16 as core::ffi::c_int,
];
static mut tx_size_wide_unit: [core::ffi::c_int; 19] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    1 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
];
static mut tx_size_high_unit: [core::ffi::c_int; 19] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    4 as core::ffi::c_int,
    8 as core::ffi::c_int,
    16 as core::ffi::c_int,
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
    4 as core::ffi::c_int,
    2 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    16 as core::ffi::c_int,
    8 as core::ffi::c_int,
    4 as core::ffi::c_int,
    1 as core::ffi::c_int,
    8 as core::ffi::c_int,
    2 as core::ffi::c_int,
    16 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
pub const _PTHREAD_ONCE_SIG_init: core::ffi::c_int = 0x30b1bcba as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn aom_memset16(
    mut dest: *mut core::ffi::c_void,
    mut val: core::ffi::c_int,
    mut length: size_t,
) -> *mut core::ffi::c_void {
    let mut i: size_t = 0;
    let mut dest16: *mut uint16_t = dest as *mut uint16_t;
    i = 0 as size_t;
    while i < length {
        let fresh0 = dest16;
        dest16 = dest16.offset(1);
        *fresh0 = val as uint16_t;
        i = i.wrapping_add(1);
    }
    return dest;
}
#[inline]
unsafe extern "C-unwind" fn is_intrabc_block(
    mut mbmi: *const MB_MODE_INFO,
) -> core::ffi::c_int {
    return (*mbmi).use_intrabc() as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn get_uv_mode(mut mode: UV_PREDICTION_MODE) -> PREDICTION_MODE {
    if !((mode as core::ffi::c_int) < UV_INTRA_MODES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"get_uv_mode\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            350 as core::ffi::c_int,
            b"mode < UV_INTRA_MODES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    static mut uv2y: [PREDICTION_MODE; 16] = [
        DC_PRED as core::ffi::c_int as PREDICTION_MODE,
        V_PRED as core::ffi::c_int as PREDICTION_MODE,
        H_PRED as core::ffi::c_int as PREDICTION_MODE,
        D45_PRED as core::ffi::c_int as PREDICTION_MODE,
        D135_PRED as core::ffi::c_int as PREDICTION_MODE,
        D113_PRED as core::ffi::c_int as PREDICTION_MODE,
        D157_PRED as core::ffi::c_int as PREDICTION_MODE,
        D203_PRED as core::ffi::c_int as PREDICTION_MODE,
        D67_PRED as core::ffi::c_int as PREDICTION_MODE,
        SMOOTH_PRED as core::ffi::c_int as PREDICTION_MODE,
        SMOOTH_V_PRED as core::ffi::c_int as PREDICTION_MODE,
        SMOOTH_H_PRED as core::ffi::c_int as PREDICTION_MODE,
        PAETH_PRED as core::ffi::c_int as PREDICTION_MODE,
        DC_PRED as core::ffi::c_int as PREDICTION_MODE,
        INTRA_INVALID as core::ffi::c_int as PREDICTION_MODE,
        INTRA_INVALID as core::ffi::c_int as PREDICTION_MODE,
    ];
    return uv2y[mode as usize];
}
#[inline]
unsafe extern "C-unwind" fn is_inter_block(mut mbmi: *const MB_MODE_INFO) -> core::ffi::c_int {
    return (is_intrabc_block(mbmi) != 0
        || (*mbmi).ref_frame[0 as core::ffi::c_int as usize] as core::ffi::c_int
            > INTRA_FRAME as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn is_cur_buf_hbd(mut xd: *const MACROBLOCKD) -> core::ffi::c_int {
    return if (*(*xd).cur_buf).flags & YV12_FLAG_HIGHBITDEPTH != 0 {
        1 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
static mut mode_to_angle_map: [uint8_t; 13] = [
    0 as core::ffi::c_int as uint8_t,
    90 as core::ffi::c_int as uint8_t,
    180 as core::ffi::c_int as uint8_t,
    45 as core::ffi::c_int as uint8_t,
    135 as core::ffi::c_int as uint8_t,
    113 as core::ffi::c_int as uint8_t,
    157 as core::ffi::c_int as uint8_t,
    203 as core::ffi::c_int as uint8_t,
    67 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
#[inline]
unsafe extern "C-unwind" fn get_plane_block_size(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> BLOCK_SIZE {
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1194 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_x >= 0 as core::ffi::c_int && subsampling_x < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1195 as core::ffi::c_int,
            b"subsampling_x >= 0 && subsampling_x < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_y >= 0 as core::ffi::c_int && subsampling_y < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_plane_block_size\0"))
                .as_ptr(),
            b"blockd.h\0" as *const u8 as *const core::ffi::c_char,
            1196 as core::ffi::c_int,
            b"subsampling_y >= 0 && subsampling_y < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    return av1_ss_size_lookup[bsize
        as usize][subsampling_x as usize][subsampling_y as usize];
}
unsafe extern "C-unwind" fn aom_once(mut func: Option<unsafe extern "C-unwind" fn() -> ()>) {
    static mut lock: pthread_once_t = libc::PTHREAD_ONCE_INIT;
    pthread_once(&raw mut lock, func as Option<unsafe extern "C-unwind" fn() -> ()>);
}
#[inline]
unsafe extern "C-unwind" fn is_cfl_allowed(mut xd: *const MACROBLOCKD) -> CFL_ALLOWED_TYPE {
    let mut mbmi: *const MB_MODE_INFO = *((*xd).mi)
        .offset(0 as core::ffi::c_int as isize);
    let bsize: BLOCK_SIZE = (*mbmi).bsize;
    if !((bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"is_cfl_allowed\0"))
                .as_ptr(),
            b"cfl.h\0" as *const u8 as *const core::ffi::c_char,
            22 as core::ffi::c_int,
            b"bsize < BLOCK_SIZES_ALL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if (*xd).lossless[(*mbmi).segment_id() as usize] != 0 {
        let ssx: core::ffi::c_int = (*xd).plane[AOM_PLANE_U as usize].subsampling_x;
        let ssy: core::ffi::c_int = (*xd).plane[AOM_PLANE_U as usize].subsampling_y;
        let plane_bsize: core::ffi::c_int = get_plane_block_size(bsize, ssx, ssy)
            as core::ffi::c_int;
        return (plane_bsize == BLOCK_4X4 as core::ffi::c_int) as core::ffi::c_int
            as CFL_ALLOWED_TYPE;
    }
    return (block_size_wide[bsize as usize] as core::ffi::c_int <= 32 as core::ffi::c_int
        && block_size_high[bsize as usize] as core::ffi::c_int <= 32 as core::ffi::c_int)
        as core::ffi::c_int as CFL_ALLOWED_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn get_cfl_pred_type(mut plane: core::ffi::c_int) -> CFL_PRED_TYPE {
    if !(plane > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"get_cfl_pred_type\0"))
                .as_ptr(),
            b"cfl.h\0" as *const u8 as *const core::ffi::c_char,
            65 as core::ffi::c_int,
            b"plane > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return (plane - 1 as core::ffi::c_int) as CFL_PRED_TYPE;
}
#[inline]
unsafe extern "C-unwind" fn av1_is_directional_mode(
    mut mode: PREDICTION_MODE,
) -> core::ffi::c_int {
    return (mode as core::ffi::c_int >= V_PRED as core::ffi::c_int
        && mode as core::ffi::c_int <= D67_PRED as core::ffi::c_int) as core::ffi::c_int;
}
static mut dr_intra_derivative: [int16_t; 90] = [
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    1023 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    547 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    372 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    273 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    215 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    178 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    151 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    132 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    116 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    102 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    90 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    80 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    71 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    64 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    57 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    51 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    45 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    40 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    35 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    31 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    27 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    23 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    19 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    15 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    11 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    7 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    3 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
    0 as core::ffi::c_int as int16_t,
];
#[inline]
unsafe extern "C-unwind" fn av1_get_dx(mut angle: core::ffi::c_int) -> core::ffi::c_int {
    if angle > 0 as core::ffi::c_int && angle < 90 as core::ffi::c_int {
        return dr_intra_derivative[angle as usize] as core::ffi::c_int
    } else if angle > 90 as core::ffi::c_int && angle < 180 as core::ffi::c_int {
        return dr_intra_derivative[(180 as core::ffi::c_int - angle) as usize]
            as core::ffi::c_int
    } else {
        return 1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_get_dy(mut angle: core::ffi::c_int) -> core::ffi::c_int {
    if angle > 90 as core::ffi::c_int && angle < 180 as core::ffi::c_int {
        return dr_intra_derivative[(angle - 90 as core::ffi::c_int) as usize]
            as core::ffi::c_int
    } else if angle > 180 as core::ffi::c_int && angle < 270 as core::ffi::c_int {
        return dr_intra_derivative[(270 as core::ffi::c_int - angle) as usize]
            as core::ffi::c_int
    } else {
        return 1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn av1_use_intra_edge_upsample(
    mut bs0: core::ffi::c_int,
    mut bs1: core::ffi::c_int,
    mut delta: core::ffi::c_int,
    mut type_0: core::ffi::c_int,
) -> core::ffi::c_int {
    let d: core::ffi::c_int = abs(delta) as core::ffi::c_int;
    let blk_wh: core::ffi::c_int = bs0 + bs1;
    if d == 0 as core::ffi::c_int || d >= 40 as core::ffi::c_int {
        return 0 as core::ffi::c_int;
    }
    return if type_0 != 0 {
        (blk_wh <= 8 as core::ffi::c_int) as core::ffi::c_int
    } else {
        (blk_wh <= 16 as core::ffi::c_int) as core::ffi::c_int
    };
}
pub const INTRA_EDGE_TAPS: core::ffi::c_int = 5 as core::ffi::c_int;
pub const NUM_INTRA_NEIGHBOUR_PIXELS: core::ffi::c_int = MAX_TX_SIZE
    * 2 as core::ffi::c_int + 32 as core::ffi::c_int;
static mut extend_modes: [uint8_t; 13] = [
    (NEED_ABOVE as core::ffi::c_int | NEED_LEFT as core::ffi::c_int) as uint8_t,
    NEED_ABOVE as core::ffi::c_int as uint8_t,
    NEED_LEFT as core::ffi::c_int as uint8_t,
    (NEED_ABOVE as core::ffi::c_int | NEED_ABOVERIGHT as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int
        | NEED_ABOVELEFT as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int
        | NEED_ABOVELEFT as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int
        | NEED_ABOVELEFT as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_BOTTOMLEFT as core::ffi::c_int) as uint8_t,
    (NEED_ABOVE as core::ffi::c_int | NEED_ABOVERIGHT as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int) as uint8_t,
    (NEED_LEFT as core::ffi::c_int | NEED_ABOVE as core::ffi::c_int
        | NEED_ABOVELEFT as core::ffi::c_int) as uint8_t,
];
static mut has_tr_4x4: [uint8_t; 128] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
];
static mut has_tr_4x8: [uint8_t; 64] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
];
static mut has_tr_8x4: [uint8_t; 64] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_tr_8x8: [uint8_t; 32] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
];
static mut has_tr_8x16: [uint8_t; 16] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
];
static mut has_tr_16x8: [uint8_t; 16] = [
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_tr_16x16: [uint8_t; 8] = [
    255 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
];
static mut has_tr_16x32: [uint8_t; 4] = [
    255 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
];
static mut has_tr_32x16: [uint8_t; 4] = [
    15 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    7 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
];
static mut has_tr_32x32: [uint8_t; 2] = [
    95 as core::ffi::c_int as uint8_t,
    87 as core::ffi::c_int as uint8_t,
];
static mut has_tr_32x64: [uint8_t; 1] = [127 as core::ffi::c_int as uint8_t];
static mut has_tr_64x32: [uint8_t; 1] = [19 as core::ffi::c_int as uint8_t];
static mut has_tr_64x64: [uint8_t; 1] = [7 as core::ffi::c_int as uint8_t];
static mut has_tr_64x128: [uint8_t; 1] = [3 as core::ffi::c_int as uint8_t];
static mut has_tr_128x64: [uint8_t; 1] = [1 as core::ffi::c_int as uint8_t];
static mut has_tr_128x128: [uint8_t; 1] = [1 as core::ffi::c_int as uint8_t];
static mut has_tr_4x16: [uint8_t; 32] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
];
static mut has_tr_16x4: [uint8_t; 32] = [
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_tr_8x32: [uint8_t; 8] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
];
static mut has_tr_32x8: [uint8_t; 8] = [
    15 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    7 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    5 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_tr_16x64: [uint8_t; 2] = [
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
];
static mut has_tr_64x16: [uint8_t; 2] = [
    3 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
];
static mut has_tr_tables: [*const uint8_t; 22] = unsafe {
    [
        core::ptr::addr_of!(has_tr_4x4).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_4x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_8x4).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_8x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_8x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_16x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_16x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_16x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_32x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_32x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_32x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_64x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_64x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_64x128).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_128x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_128x128).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_4x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_16x4).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_8x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_32x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_16x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_64x16).cast::<uint8_t>() as *const uint8_t,
    ]
};
static mut has_tr_vert_8x8: [uint8_t; 32] = [
    255 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_tr_vert_16x16: [uint8_t; 8] = [
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    127 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    119 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_tr_vert_32x32: [uint8_t; 2] = [
    15 as core::ffi::c_int as uint8_t,
    7 as core::ffi::c_int as uint8_t,
];
static mut has_tr_vert_64x64: [uint8_t; 1] = [3 as core::ffi::c_int as uint8_t];
static mut has_tr_vert_tables: [*const uint8_t; 16] = unsafe {
    [
        0 as *const uint8_t,
        core::ptr::addr_of!(has_tr_4x8).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_tr_vert_8x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_8x16).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_tr_vert_16x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_16x32).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_tr_vert_32x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_32x64).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_tr_vert_64x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_tr_64x128).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_tr_128x128).cast::<uint8_t>() as *const uint8_t,
    ]
};
unsafe extern "C-unwind" fn get_has_tr_table(
    mut partition: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
) -> *const uint8_t {
    let mut ret: *const uint8_t = 0 as *const uint8_t;
    if partition as core::ffi::c_int == PARTITION_VERT_A as core::ffi::c_int
        || partition as core::ffi::c_int == PARTITION_VERT_B as core::ffi::c_int
    {
        if !((bsize as core::ffi::c_int) < BLOCK_SIZES as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"get_has_tr_table\0"))
                    .as_ptr(),
                b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                187 as core::ffi::c_int,
                b"bsize < BLOCK_SIZES\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        ret = has_tr_vert_tables[bsize as usize];
    } else {
        ret = has_tr_tables[bsize as usize];
    }
    if ret.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"get_has_tr_table\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            192 as core::ffi::c_int,
            b"ret\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ret;
}
unsafe extern "C-unwind" fn has_top_right(
    mut sb_size: BLOCK_SIZE,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut top_available: core::ffi::c_int,
    mut right_available: core::ffi::c_int,
    mut partition: PARTITION_TYPE,
    mut txsz: TX_SIZE,
    mut row_off: core::ffi::c_int,
    mut col_off: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
) -> core::ffi::c_int {
    if top_available == 0 || right_available == 0 {
        return 0 as core::ffi::c_int;
    }
    let bw_unit: core::ffi::c_int = mi_size_wide[bsize as usize] as core::ffi::c_int;
    let plane_bw_unit: core::ffi::c_int = if bw_unit >> ss_x > 1 as core::ffi::c_int {
        bw_unit >> ss_x
    } else {
        1 as core::ffi::c_int
    };
    let top_right_count_unit: core::ffi::c_int = tx_size_wide_unit[txsz as usize];
    if row_off > 0 as core::ffi::c_int {
        if block_size_wide[bsize as usize] as core::ffi::c_int
            > block_size_wide[BLOCK_64X64 as core::ffi::c_int as usize]
                as core::ffi::c_int
        {
            if row_off
                == mi_size_high[BLOCK_64X64 as core::ffi::c_int as usize]
                    as core::ffi::c_int >> ss_y
                && col_off + top_right_count_unit
                    == mi_size_wide[BLOCK_64X64 as core::ffi::c_int as usize]
                        as core::ffi::c_int >> ss_x
            {
                return 1 as core::ffi::c_int;
            }
            let plane_bw_unit_64: core::ffi::c_int = mi_size_wide[BLOCK_64X64
                as core::ffi::c_int as usize] as core::ffi::c_int >> ss_x;
            let col_off_64: core::ffi::c_int = col_off % plane_bw_unit_64;
            return (col_off_64 + top_right_count_unit < plane_bw_unit_64)
                as core::ffi::c_int;
        }
        return (col_off + top_right_count_unit < plane_bw_unit) as core::ffi::c_int;
    } else {
        if col_off + top_right_count_unit < plane_bw_unit {
            return 1 as core::ffi::c_int;
        }
        let bw_in_mi_log2: core::ffi::c_int = mi_size_wide_log2[bsize as usize]
            as core::ffi::c_int;
        let bh_in_mi_log2: core::ffi::c_int = mi_size_high_log2[bsize as usize]
            as core::ffi::c_int;
        let sb_mi_size: core::ffi::c_int = mi_size_high[sb_size as usize]
            as core::ffi::c_int;
        let blk_row_in_sb: core::ffi::c_int = (mi_row
            & sb_mi_size - 1 as core::ffi::c_int) >> bh_in_mi_log2;
        let blk_col_in_sb: core::ffi::c_int = (mi_col
            & sb_mi_size - 1 as core::ffi::c_int) >> bw_in_mi_log2;
        if blk_row_in_sb == 0 as core::ffi::c_int {
            return 1 as core::ffi::c_int;
        }
        if (blk_col_in_sb + 1 as core::ffi::c_int) << bw_in_mi_log2 >= sb_mi_size {
            return 0 as core::ffi::c_int;
        }
        let this_blk_index: core::ffi::c_int = ((blk_row_in_sb + 0 as core::ffi::c_int)
            << MAX_MIB_SIZE_LOG2 - bw_in_mi_log2) + blk_col_in_sb
            + 0 as core::ffi::c_int;
        let idx1: core::ffi::c_int = this_blk_index / 8 as core::ffi::c_int;
        let idx2: core::ffi::c_int = this_blk_index % 8 as core::ffi::c_int;
        let mut has_tr_table: *const uint8_t = get_has_tr_table(partition, bsize);
        return *has_tr_table.offset(idx1 as isize) as core::ffi::c_int >> idx2
            & 1 as core::ffi::c_int;
    };
}
static mut has_bl_4x4: [uint8_t; 128] = [
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_4x8: [uint8_t; 64] = [
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_8x4: [uint8_t; 64] = [
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_8x8: [uint8_t; 32] = [
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    85 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_8x16: [uint8_t; 16] = [
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_16x8: [uint8_t; 16] = [
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_16x16: [uint8_t; 8] = [
    84 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_16x32: [uint8_t; 4] = [
    16 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_32x16: [uint8_t; 4] = [
    78 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
    78 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
];
static mut has_bl_32x32: [uint8_t; 2] = [
    4 as core::ffi::c_int as uint8_t,
    4 as core::ffi::c_int as uint8_t,
];
static mut has_bl_32x64: [uint8_t; 1] = [0 as core::ffi::c_int as uint8_t];
static mut has_bl_64x32: [uint8_t; 1] = [34 as core::ffi::c_int as uint8_t];
static mut has_bl_64x64: [uint8_t; 1] = [0 as core::ffi::c_int as uint8_t];
static mut has_bl_64x128: [uint8_t; 1] = [0 as core::ffi::c_int as uint8_t];
static mut has_bl_128x64: [uint8_t; 1] = [0 as core::ffi::c_int as uint8_t];
static mut has_bl_128x128: [uint8_t; 1] = [0 as core::ffi::c_int as uint8_t];
static mut has_bl_4x16: [uint8_t; 32] = [
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_16x4: [uint8_t; 32] = [
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    84 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_8x32: [uint8_t; 8] = [
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_32x8: [uint8_t; 8] = [
    238 as core::ffi::c_int as uint8_t,
    78 as core::ffi::c_int as uint8_t,
    238 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
    238 as core::ffi::c_int as uint8_t,
    78 as core::ffi::c_int as uint8_t,
    238 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
];
static mut has_bl_16x64: [uint8_t; 2] = [
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_64x16: [uint8_t; 2] = [
    42 as core::ffi::c_int as uint8_t,
    42 as core::ffi::c_int as uint8_t,
];
static mut has_bl_tables: [*const uint8_t; 22] = unsafe {
    [
        core::ptr::addr_of!(has_bl_4x4).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_4x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_8x4).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_8x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_8x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_16x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_16x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_16x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_32x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_32x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_32x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_64x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_64x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_64x128).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_128x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_128x128).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_4x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_16x4).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_8x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_32x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_16x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_64x16).cast::<uint8_t>() as *const uint8_t,
    ]
};
static mut has_bl_vert_8x8: [uint8_t; 32] = [
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    1 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    17 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    255 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_vert_16x16: [uint8_t; 8] = [
    254 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    16 as core::ffi::c_int as uint8_t,
    254 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
];
static mut has_bl_vert_32x32: [uint8_t; 2] = [
    14 as core::ffi::c_int as uint8_t,
    14 as core::ffi::c_int as uint8_t,
];
static mut has_bl_vert_64x64: [uint8_t; 1] = [2 as core::ffi::c_int as uint8_t];
static mut has_bl_vert_tables: [*const uint8_t; 16] = unsafe {
    [
        0 as *const uint8_t,
        core::ptr::addr_of!(has_bl_4x8).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_bl_vert_8x8).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_8x16).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_bl_vert_16x16).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_16x32).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_bl_vert_32x32).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_32x64).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_bl_vert_64x64).cast::<uint8_t>() as *const uint8_t,
        core::ptr::addr_of!(has_bl_64x128).cast::<uint8_t>() as *const uint8_t,
        0 as *const uint8_t,
        core::ptr::addr_of!(has_bl_128x128).cast::<uint8_t>() as *const uint8_t,
    ]
};
unsafe extern "C-unwind" fn get_has_bl_table(
    mut partition: PARTITION_TYPE,
    mut bsize: BLOCK_SIZE,
) -> *const uint8_t {
    let mut ret: *const uint8_t = 0 as *const uint8_t;
    if partition as core::ffi::c_int == PARTITION_VERT_A as core::ffi::c_int
        || partition as core::ffi::c_int == PARTITION_VERT_B as core::ffi::c_int
    {
        if !((bsize as core::ffi::c_int) < BLOCK_SIZES as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"get_has_bl_table\0"))
                    .as_ptr(),
                b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                372 as core::ffi::c_int,
                b"bsize < BLOCK_SIZES\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        ret = has_bl_vert_tables[bsize as usize];
    } else {
        ret = has_bl_tables[bsize as usize];
    }
    if ret.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"get_has_bl_table\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            377 as core::ffi::c_int,
            b"ret\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return ret;
}
unsafe extern "C-unwind" fn has_bottom_left(
    mut sb_size: BLOCK_SIZE,
    mut bsize: BLOCK_SIZE,
    mut mi_row: core::ffi::c_int,
    mut mi_col: core::ffi::c_int,
    mut bottom_available: core::ffi::c_int,
    mut left_available: core::ffi::c_int,
    mut partition: PARTITION_TYPE,
    mut txsz: TX_SIZE,
    mut row_off: core::ffi::c_int,
    mut col_off: core::ffi::c_int,
    mut ss_x: core::ffi::c_int,
    mut ss_y: core::ffi::c_int,
) -> core::ffi::c_int {
    if bottom_available == 0 || left_available == 0 {
        return 0 as core::ffi::c_int;
    }
    if block_size_wide[bsize as usize] as core::ffi::c_int
        > block_size_wide[BLOCK_64X64 as core::ffi::c_int as usize] as core::ffi::c_int
        && col_off > 0 as core::ffi::c_int
    {
        let plane_bw_unit_64: core::ffi::c_int = mi_size_wide[BLOCK_64X64
            as core::ffi::c_int as usize] as core::ffi::c_int >> ss_x;
        let col_off_64: core::ffi::c_int = col_off % plane_bw_unit_64;
        if col_off_64 == 0 as core::ffi::c_int {
            let plane_bh_unit_64: core::ffi::c_int = mi_size_high[BLOCK_64X64
                as core::ffi::c_int as usize] as core::ffi::c_int >> ss_y;
            let row_off_64: core::ffi::c_int = row_off % plane_bh_unit_64;
            let plane_bh_unit: core::ffi::c_int = if mi_size_high[bsize as usize]
                as core::ffi::c_int >> ss_y < plane_bh_unit_64
            {
                mi_size_high[bsize as usize] as core::ffi::c_int >> ss_y
            } else {
                plane_bh_unit_64
            };
            return (row_off_64 + tx_size_high_unit[txsz as usize] < plane_bh_unit)
                as core::ffi::c_int;
        }
    }
    if col_off > 0 as core::ffi::c_int {
        return 0 as core::ffi::c_int
    } else {
        let bh_unit: core::ffi::c_int = mi_size_high[bsize as usize] as core::ffi::c_int;
        let plane_bh_unit_0: core::ffi::c_int = if bh_unit >> ss_y
            > 1 as core::ffi::c_int
        {
            bh_unit >> ss_y
        } else {
            1 as core::ffi::c_int
        };
        let bottom_left_count_unit: core::ffi::c_int = tx_size_high_unit[txsz as usize];
        if row_off + bottom_left_count_unit < plane_bh_unit_0 {
            return 1 as core::ffi::c_int;
        }
        let bw_in_mi_log2: core::ffi::c_int = mi_size_wide_log2[bsize as usize]
            as core::ffi::c_int;
        let bh_in_mi_log2: core::ffi::c_int = mi_size_high_log2[bsize as usize]
            as core::ffi::c_int;
        let sb_mi_size: core::ffi::c_int = mi_size_high[sb_size as usize]
            as core::ffi::c_int;
        let blk_row_in_sb: core::ffi::c_int = (mi_row
            & sb_mi_size - 1 as core::ffi::c_int) >> bh_in_mi_log2;
        let blk_col_in_sb: core::ffi::c_int = (mi_col
            & sb_mi_size - 1 as core::ffi::c_int) >> bw_in_mi_log2;
        if blk_col_in_sb == 0 as core::ffi::c_int {
            let blk_start_row_off: core::ffi::c_int = blk_row_in_sb
                << bh_in_mi_log2 + MI_SIZE_LOG2 - MI_SIZE_LOG2 >> ss_y;
            let row_off_in_sb: core::ffi::c_int = blk_start_row_off + row_off;
            let sb_height_unit: core::ffi::c_int = sb_mi_size >> ss_y;
            return (row_off_in_sb + bottom_left_count_unit < sb_height_unit)
                as core::ffi::c_int;
        }
        if (blk_row_in_sb + 1 as core::ffi::c_int) << bh_in_mi_log2 >= sb_mi_size {
            return 0 as core::ffi::c_int;
        }
        let this_blk_index: core::ffi::c_int = ((blk_row_in_sb + 0 as core::ffi::c_int)
            << MAX_MIB_SIZE_LOG2 - bw_in_mi_log2) + blk_col_in_sb
            + 0 as core::ffi::c_int;
        let idx1: core::ffi::c_int = this_blk_index / 8 as core::ffi::c_int;
        let idx2: core::ffi::c_int = this_blk_index % 8 as core::ffi::c_int;
        let mut has_bl_table: *const uint8_t = get_has_bl_table(partition, bsize);
        return *has_bl_table.offset(idx1 as isize) as core::ffi::c_int >> idx2
            & 1 as core::ffi::c_int;
    };
}
static mut pred: [[intra_pred_fn; 19]; 13] = [[None; 19]; 13];
static mut dc_pred: [[[intra_pred_fn; 19]; 2]; 2] = [[[None; 19]; 2]; 2];
static mut pred_high: [[intra_high_pred_fn; 19]; 13] = [[None; 19]; 13];
static mut dc_pred_high: [[[intra_high_pred_fn; 19]; 2]; 2] = [[[None; 19]; 2]; 2];
unsafe extern "C-unwind" fn init_intra_predictors_internal() {
    if !((::core::mem::size_of::<[uint8_t; 13]>() as usize)
        .wrapping_div(::core::mem::size_of::<uint8_t>() as usize) as core::ffi::c_int
        == INTRA_MODES as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 31],
                [core::ffi::c_char; 31],
            >(*b"init_intra_predictors_internal\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            465 as core::ffi::c_int,
            b"NELEMENTS(mode_to_angle_map) == INTRA_MODES\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    pred[V_PRED as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[V_PRED as core::ffi::c_int as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_v_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[H_PRED as core::ffi::c_int as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_h_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[PAETH_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_paeth_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_v_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_smooth_h_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_dc_128_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_dc_top_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_dc_left_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_4x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_8x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_16x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_32x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_64x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_4x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_8x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_8x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_16x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_16x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_32x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_32x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_64x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_4x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_16x4_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_8x32_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_32x8_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_16x64_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    dc_pred[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = Some(
        aom_dc_predictor_64x16_c
            as unsafe extern "C-unwind" fn(
                *mut uint8_t,
                ptrdiff_t,
                *const uint8_t,
                *const uint8_t,
            ) -> (),
    ) as intra_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_v_predictor_4x4
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_v_predictor_8x8
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_v_predictor_16x16
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_v_predictor_32x32
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_v_predictor_64x64
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_v_predictor_4x8
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_v_predictor_8x4
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_v_predictor_8x16
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_v_predictor_16x8
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_v_predictor_16x32
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_v_predictor_32x16
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_v_predictor_32x64
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_v_predictor_64x32
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_v_predictor_4x16
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_v_predictor_16x4
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_v_predictor_8x32
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_v_predictor_32x8
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_v_predictor_16x64
        as intra_high_pred_fn;
    pred_high[V_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_v_predictor_64x16
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_h_predictor_4x4
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_h_predictor_8x8
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_h_predictor_16x16
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_h_predictor_32x32
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_h_predictor_64x64
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_h_predictor_4x8
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_h_predictor_8x4
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_h_predictor_8x16
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_h_predictor_16x8
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_h_predictor_16x32
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_h_predictor_32x16
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_h_predictor_32x64
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_h_predictor_64x32
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_h_predictor_4x16
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_h_predictor_16x4
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_h_predictor_8x32
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_h_predictor_32x8
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_h_predictor_16x64
        as intra_high_pred_fn;
    pred_high[H_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_h_predictor_64x16
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_4x4
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_8x8
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_16x16
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_32x32
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_64x64
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_4x8
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_8x4
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_8x16
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_16x8
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_16x32
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_32x16
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_32x64
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_64x32
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_4x16
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_16x4
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_8x32
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_32x8
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_16x64
        as intra_high_pred_fn;
    pred_high[PAETH_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_paeth_predictor_64x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_4x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_8x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_16x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_32x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_64x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_4x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_8x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_8x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_16x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_16x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_32x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_32x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_64x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_4x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_16x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_8x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_32x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_16x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_smooth_predictor_64x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_4x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_8x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_16x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_32x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_64x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_4x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_8x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_8x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_16x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_16x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_32x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_32x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_64x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_4x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_16x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_8x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_32x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_16x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_V_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_smooth_v_predictor_64x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_4x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_8x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_16x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_32x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_64x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_4x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_8x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_8x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_16x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_16x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_32x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_32x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_64x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_4x16
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_16x4
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_8x32
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_32x8
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_16x64
        as intra_high_pred_fn;
    pred_high[SMOOTH_H_PRED as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_smooth_h_predictor_64x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_4x4
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_8x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_16x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_32x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_64x64
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_4x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_8x4
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_8x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_16x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_16x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_32x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_32x64
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_64x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_4x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_16x4
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_8x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_32x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_16x64
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_dc_128_predictor_64x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_4x4
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_8x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_16x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_32x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_64x64
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_4x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_8x4
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_8x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_16x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_16x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_32x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_32x64
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_64x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_4x16
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_16x4
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_8x32
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_32x8
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_16x64
        as intra_high_pred_fn;
    dc_pred_high[0 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_dc_top_predictor_64x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_4x4
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_8x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_16x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_32x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_64x64
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_4x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_8x4
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_8x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_16x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_16x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_32x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_32x64
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_64x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_4x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_16x4
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_8x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_32x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_16x64
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][0 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_dc_left_predictor_64x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X4 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_4x4
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X8 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_8x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X16 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_16x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X32 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_32x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X64 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_64x64
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_4X8 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_4x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int as usize][TX_8X4 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_8x4
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X16 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_8x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X8 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_16x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X32 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_16x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X16 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_32x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X64 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_32x64
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X32 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_64x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_4X16 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_4x16
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X4 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_16x4
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_8X32 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_8x32
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_32X8 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_32x8
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_16X64 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_16x64
        as intra_high_pred_fn;
    dc_pred_high[1 as core::ffi::c_int
        as usize][1 as core::ffi::c_int
        as usize][TX_64X16 as core::ffi::c_int as usize] = aom_highbd_dc_predictor_64x16
        as intra_high_pred_fn;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z1_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
    mut upsample_above: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut base: core::ffi::c_int = 0;
    let mut shift: core::ffi::c_int = 0;
    let mut val: core::ffi::c_int = 0;
    if !(dy == 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_dr_prediction_z1_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            539 as core::ffi::c_int,
            b"dy == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dx > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_dr_prediction_z1_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            540 as core::ffi::c_int,
            b"dx > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let max_base_x: core::ffi::c_int = (bw + bh - 1 as core::ffi::c_int)
        << upsample_above;
    let frac_bits: core::ffi::c_int = 6 as core::ffi::c_int - upsample_above;
    let base_inc: core::ffi::c_int = (1 as core::ffi::c_int) << upsample_above;
    x = dx;
    r = 0 as core::ffi::c_int;
    while r < bh {
        base = x >> frac_bits;
        shift = (x << upsample_above & 0x3f as core::ffi::c_int)
            >> 1 as core::ffi::c_int;
        if base >= max_base_x {
            let mut i: core::ffi::c_int = r;
            while i < bh {
                memset(
                    dst as *mut core::ffi::c_void,
                    *above.offset(max_base_x as isize) as core::ffi::c_int,
                    (bw as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                );
                dst = dst.offset(stride as isize);
                i += 1;
            }
            return;
        }
        c = 0 as core::ffi::c_int;
        while c < bw {
            if base < max_base_x {
                val = *above.offset(base as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift)
                    + *above.offset((base + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift;
                *dst.offset(c as isize) = (val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int) as uint8_t;
            } else {
                *dst.offset(c as isize) = *above.offset(max_base_x as isize);
            }
            c += 1;
            base += base_inc;
        }
        r += 1;
        dst = dst.offset(stride as isize);
        x += dx;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z2_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
    mut upsample_above: core::ffi::c_int,
    mut upsample_left: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
) {
    if !(dx > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_dr_prediction_z2_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            574 as core::ffi::c_int,
            b"dx > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dy > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_dr_prediction_z2_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            575 as core::ffi::c_int,
            b"dy > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let min_base_x: core::ffi::c_int = -((1 as core::ffi::c_int) << upsample_above);
    let min_base_y: core::ffi::c_int = -((1 as core::ffi::c_int) << upsample_left);
    let frac_bits_x: core::ffi::c_int = 6 as core::ffi::c_int - upsample_above;
    let frac_bits_y: core::ffi::c_int = 6 as core::ffi::c_int - upsample_left;
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < bw {
            let mut val: core::ffi::c_int = 0;
            let mut y: core::ffi::c_int = r + 1 as core::ffi::c_int;
            let mut x: core::ffi::c_int = (c << 6 as core::ffi::c_int) - y * dx;
            let base_x: core::ffi::c_int = x >> frac_bits_x;
            if base_x >= min_base_x {
                let shift: core::ffi::c_int = (x
                    * ((1 as core::ffi::c_int) << upsample_above)
                    & 0x3f as core::ffi::c_int) >> 1 as core::ffi::c_int;
                val = *above.offset(base_x as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift)
                    + *above.offset((base_x + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift;
                val = val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int;
            } else {
                x = c + 1 as core::ffi::c_int;
                y = (r << 6 as core::ffi::c_int) - x * dy;
                let base_y: core::ffi::c_int = y >> frac_bits_y;
                if !(base_y >= min_base_y) as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 23],
                            [core::ffi::c_char; 23],
                        >(*b"av1_dr_prediction_z2_c\0"))
                            .as_ptr(),
                        b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                        597 as core::ffi::c_int,
                        b"base_y >= min_base_y\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                let shift_0: core::ffi::c_int = (y
                    * ((1 as core::ffi::c_int) << upsample_left)
                    & 0x3f as core::ffi::c_int) >> 1 as core::ffi::c_int;
                val = *left.offset(base_y as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift_0)
                    + *left.offset((base_y + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift_0;
                val = val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int;
            }
            *dst.offset(c as isize) = val as uint8_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_dr_prediction_z3_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
    mut upsample_left: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut base: core::ffi::c_int = 0;
    let mut shift: core::ffi::c_int = 0;
    let mut val: core::ffi::c_int = 0;
    if !(dx == 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_dr_prediction_z3_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            617 as core::ffi::c_int,
            b"dx == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dy > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"av1_dr_prediction_z3_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            618 as core::ffi::c_int,
            b"dy > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let max_base_y: core::ffi::c_int = (bw + bh - 1 as core::ffi::c_int)
        << upsample_left;
    let frac_bits: core::ffi::c_int = 6 as core::ffi::c_int - upsample_left;
    let base_inc: core::ffi::c_int = (1 as core::ffi::c_int) << upsample_left;
    y = dy;
    c = 0 as core::ffi::c_int;
    while c < bw {
        base = y >> frac_bits;
        shift = (y << upsample_left & 0x3f as core::ffi::c_int) >> 1 as core::ffi::c_int;
        r = 0 as core::ffi::c_int;
        while r < bh {
            if base < max_base_y {
                val = *left.offset(base as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift)
                    + *left.offset((base + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift;
                *dst.offset((r as ptrdiff_t * stride + c as ptrdiff_t) as isize) = (val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int) as uint8_t;
                r += 1;
                base += base_inc;
            } else {
                while r < bh {
                    *dst.offset((r as ptrdiff_t * stride + c as ptrdiff_t) as isize) = *left
                        .offset(max_base_y as isize);
                    r += 1;
                }
                break;
            }
        }
        c += 1;
        y += dy;
    }
}
unsafe extern "C-unwind" fn dr_predictor(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut tx_size: TX_SIZE,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
    mut upsample_above: core::ffi::c_int,
    mut upsample_left: core::ffi::c_int,
    mut angle: core::ffi::c_int,
) {
    let dx: core::ffi::c_int = av1_get_dx(angle) as core::ffi::c_int;
    let dy: core::ffi::c_int = av1_get_dy(angle) as core::ffi::c_int;
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    if !(angle > 0 as core::ffi::c_int && angle < 270 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 13],
                [core::ffi::c_char; 13],
            >(*b"dr_predictor\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            647 as core::ffi::c_int,
            b"angle > 0 && angle < 270\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if angle > 0 as core::ffi::c_int && angle < 90 as core::ffi::c_int {
        av1_dr_prediction_z1_c(dst, stride, bw, bh, above, left, upsample_above, dx, dy);
    } else if angle > 90 as core::ffi::c_int && angle < 180 as core::ffi::c_int {
        av1_dr_prediction_z2_c(
            dst,
            stride,
            bw,
            bh,
            above,
            left,
            upsample_above,
            upsample_left,
            dx,
            dy,
        );
    } else if angle > 180 as core::ffi::c_int && angle < 270 as core::ffi::c_int {
        av1_dr_prediction_z3_c(dst, stride, bw, bh, above, left, upsample_left, dx, dy);
    } else if angle == 90 as core::ffi::c_int {
        (pred[V_PRED as core::ffi::c_int as usize][tx_size as usize])
            .expect("non-null function pointer")(dst, stride, above, left);
    } else if angle == 180 as core::ffi::c_int {
        (pred[H_PRED as core::ffi::c_int as usize][tx_size as usize])
            .expect("non-null function pointer")(dst, stride, above, left);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z1_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut upsample_above: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut base: core::ffi::c_int = 0;
    let mut shift: core::ffi::c_int = 0;
    let mut val: core::ffi::c_int = 0;
    if !(dy == 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_highbd_dr_prediction_z1_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            676 as core::ffi::c_int,
            b"dy == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dx > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_highbd_dr_prediction_z1_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            677 as core::ffi::c_int,
            b"dx > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let max_base_x: core::ffi::c_int = (bw + bh - 1 as core::ffi::c_int)
        << upsample_above;
    let frac_bits: core::ffi::c_int = 6 as core::ffi::c_int - upsample_above;
    let base_inc: core::ffi::c_int = (1 as core::ffi::c_int) << upsample_above;
    x = dx;
    r = 0 as core::ffi::c_int;
    while r < bh {
        base = x >> frac_bits;
        shift = (x << upsample_above & 0x3f as core::ffi::c_int)
            >> 1 as core::ffi::c_int;
        if base >= max_base_x {
            let mut i: core::ffi::c_int = r;
            while i < bh {
                aom_memset16(
                    dst as *mut core::ffi::c_void,
                    *above.offset(max_base_x as isize) as core::ffi::c_int,
                    bw as size_t,
                );
                dst = dst.offset(stride as isize);
                i += 1;
            }
            return;
        }
        c = 0 as core::ffi::c_int;
        while c < bw {
            if base < max_base_x {
                val = *above.offset(base as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift)
                    + *above.offset((base + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift;
                *dst.offset(c as isize) = (val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int) as uint16_t;
            } else {
                *dst.offset(c as isize) = *above.offset(max_base_x as isize);
            }
            c += 1;
            base += base_inc;
        }
        r += 1;
        dst = dst.offset(stride as isize);
        x += dx;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z2_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut upsample_above: core::ffi::c_int,
    mut upsample_left: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    if !(dx > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_highbd_dr_prediction_z2_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            712 as core::ffi::c_int,
            b"dx > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dy > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_highbd_dr_prediction_z2_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            713 as core::ffi::c_int,
            b"dy > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let min_base_x: core::ffi::c_int = -((1 as core::ffi::c_int) << upsample_above);
    let min_base_y: core::ffi::c_int = -((1 as core::ffi::c_int) << upsample_left);
    let frac_bits_x: core::ffi::c_int = 6 as core::ffi::c_int - upsample_above;
    let frac_bits_y: core::ffi::c_int = 6 as core::ffi::c_int - upsample_left;
    let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
    while r < bh {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < bw {
            let mut val: core::ffi::c_int = 0;
            let mut y: core::ffi::c_int = r + 1 as core::ffi::c_int;
            let mut x: core::ffi::c_int = (c << 6 as core::ffi::c_int) - y * dx;
            let base_x: core::ffi::c_int = x >> frac_bits_x;
            if base_x >= min_base_x {
                let shift: core::ffi::c_int = (x
                    * ((1 as core::ffi::c_int) << upsample_above)
                    & 0x3f as core::ffi::c_int) >> 1 as core::ffi::c_int;
                val = *above.offset(base_x as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift)
                    + *above.offset((base_x + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift;
                val = val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int;
            } else {
                x = c + 1 as core::ffi::c_int;
                y = (r << 6 as core::ffi::c_int) - x * dy;
                let base_y: core::ffi::c_int = y >> frac_bits_y;
                if !(base_y >= min_base_y) as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 30],
                            [core::ffi::c_char; 30],
                        >(*b"av1_highbd_dr_prediction_z2_c\0"))
                            .as_ptr(),
                        b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                        735 as core::ffi::c_int,
                        b"base_y >= min_base_y\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                let shift_0: core::ffi::c_int = (y
                    * ((1 as core::ffi::c_int) << upsample_left)
                    & 0x3f as core::ffi::c_int) >> 1 as core::ffi::c_int;
                val = *left.offset(base_y as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift_0)
                    + *left.offset((base_y + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift_0;
                val = val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int;
            }
            *dst.offset(c as isize) = val as uint16_t;
            c += 1;
        }
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_dr_prediction_z3_c(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut upsample_left: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut dy: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut base: core::ffi::c_int = 0;
    let mut shift: core::ffi::c_int = 0;
    let mut val: core::ffi::c_int = 0;
    if !(dx == 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_highbd_dr_prediction_z3_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            756 as core::ffi::c_int,
            b"dx == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(dy > 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"av1_highbd_dr_prediction_z3_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            757 as core::ffi::c_int,
            b"dy > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let max_base_y: core::ffi::c_int = (bw + bh - 1 as core::ffi::c_int)
        << upsample_left;
    let frac_bits: core::ffi::c_int = 6 as core::ffi::c_int - upsample_left;
    let base_inc: core::ffi::c_int = (1 as core::ffi::c_int) << upsample_left;
    y = dy;
    c = 0 as core::ffi::c_int;
    while c < bw {
        base = y >> frac_bits;
        shift = (y << upsample_left & 0x3f as core::ffi::c_int) >> 1 as core::ffi::c_int;
        r = 0 as core::ffi::c_int;
        while r < bh {
            if base < max_base_y {
                val = *left.offset(base as isize) as core::ffi::c_int
                    * (32 as core::ffi::c_int - shift)
                    + *left.offset((base + 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int * shift;
                *dst.offset((r as ptrdiff_t * stride + c as ptrdiff_t) as isize) = (val
                    + ((1 as core::ffi::c_int) << 5 as core::ffi::c_int
                        >> 1 as core::ffi::c_int) >> 5 as core::ffi::c_int) as uint16_t;
                r += 1;
                base += base_inc;
            } else {
                while r < bh {
                    *dst.offset((r as ptrdiff_t * stride + c as ptrdiff_t) as isize) = *left
                        .offset(max_base_y as isize);
                    r += 1;
                }
                break;
            }
        }
        c += 1;
        y += dy;
    }
}
unsafe extern "C-unwind" fn highbd_dr_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut tx_size: TX_SIZE,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut upsample_above: core::ffi::c_int,
    mut upsample_left: core::ffi::c_int,
    mut angle: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let dx: core::ffi::c_int = av1_get_dx(angle) as core::ffi::c_int;
    let dy: core::ffi::c_int = av1_get_dy(angle) as core::ffi::c_int;
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    if !(angle > 0 as core::ffi::c_int && angle < 270 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"highbd_dr_predictor\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            787 as core::ffi::c_int,
            b"angle > 0 && angle < 270\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if angle > 0 as core::ffi::c_int && angle < 90 as core::ffi::c_int {
        av1_highbd_dr_prediction_z1
            .expect(
                "non-null function pointer",
            )(dst, stride, bw, bh, above, left, upsample_above, dx, dy, bd);
    } else if angle > 90 as core::ffi::c_int && angle < 180 as core::ffi::c_int {
        av1_highbd_dr_prediction_z2
            .expect(
                "non-null function pointer",
            )(
            dst,
            stride,
            bw,
            bh,
            above,
            left,
            upsample_above,
            upsample_left,
            dx,
            dy,
            bd,
        );
    } else if angle > 180 as core::ffi::c_int && angle < 270 as core::ffi::c_int {
        av1_highbd_dr_prediction_z3
            .expect(
                "non-null function pointer",
            )(dst, stride, bw, bh, above, left, upsample_left, dx, dy, bd);
    } else if angle == 90 as core::ffi::c_int {
        (pred_high[V_PRED as core::ffi::c_int as usize][tx_size as usize])
            .expect("non-null function pointer")(dst, stride, above, left, bd);
    } else if angle == 180 as core::ffi::c_int {
        (pred_high[H_PRED as core::ffi::c_int as usize][tx_size as usize])
            .expect("non-null function pointer")(dst, stride, above, left, bd);
    }
}
#[no_mangle]
pub static mut av1_filter_intra_taps: crate::platform::Aligned16<[[[int8_t; 8]; 8]; 5]> = crate::platform::Aligned16([
    [
        [
            -(6 as core::ffi::c_int) as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(5 as core::ffi::c_int) as int8_t,
            2 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            9 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(3 as core::ffi::c_int) as int8_t,
            1 as core::ffi::c_int as int8_t,
            1 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            7 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(3 as core::ffi::c_int) as int8_t,
            1 as core::ffi::c_int as int8_t,
            1 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            5 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(3 as core::ffi::c_int) as int8_t,
            2 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            9 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(3 as core::ffi::c_int) as int8_t,
            2 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            7 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(3 as core::ffi::c_int) as int8_t,
            1 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            5 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
    ],
    [
        [
            -(10 as core::ffi::c_int) as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(6 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(2 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(10 as core::ffi::c_int) as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(6 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(2 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
    ],
    [
        [
            -(8 as core::ffi::c_int) as int8_t,
            8 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(8 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            8 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(8 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            8 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(8 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            8 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(4 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            16 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
    ],
    [
        [
            -(2 as core::ffi::c_int) as int8_t,
            8 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(1 as core::ffi::c_int) as int8_t,
            3 as core::ffi::c_int as int8_t,
            8 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(1 as core::ffi::c_int) as int8_t,
            2 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            8 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            0 as core::ffi::c_int as int8_t,
            1 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            8 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(1 as core::ffi::c_int) as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(1 as core::ffi::c_int) as int8_t,
            3 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            6 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(1 as core::ffi::c_int) as int8_t,
            2 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(1 as core::ffi::c_int) as int8_t,
            2 as core::ffi::c_int as int8_t,
            2 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            4 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            3 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
    ],
    [
        [
            -(12 as core::ffi::c_int) as int8_t,
            14 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            14 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(10 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            14 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(9 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            14 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            11 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(8 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            14 as core::ffi::c_int as int8_t,
            10 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(10 as core::ffi::c_int) as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            14 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(9 as core::ffi::c_int) as int8_t,
            1 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(8 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            1 as core::ffi::c_int as int8_t,
            11 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
        [
            -(7 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            1 as core::ffi::c_int as int8_t,
            12 as core::ffi::c_int as int8_t,
            1 as core::ffi::c_int as int8_t,
            9 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
        ],
    ],
]);
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_intra_predictor_c(
    mut dst: *mut uint8_t,
    mut stride: ptrdiff_t,
    mut tx_size: TX_SIZE,
    mut above: *const uint8_t,
    mut left: *const uint8_t,
    mut mode: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut buffer: [[uint8_t; 33]; 33] = [[0; 33]; 33];
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    if !(bw <= 32 as core::ffi::c_int && bh <= 32 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"av1_filter_intra_predictor_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            868 as core::ffi::c_int,
            b"bw <= 32 && bh <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    r = 0 as core::ffi::c_int;
    while r < bh {
        buffer[(r + 1 as core::ffi::c_int) as usize][0 as core::ffi::c_int as usize] = *left
            .offset(r as isize);
        r += 1;
    }
    memcpy(
        (buffer[0 as core::ffi::c_int as usize]).as_mut_ptr() as *mut core::ffi::c_void,
        &*above.offset(-(1 as core::ffi::c_int) as isize) as *const uint8_t
            as *const core::ffi::c_void,
        ((bw + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    r = 1 as core::ffi::c_int;
    while r < bh + 1 as core::ffi::c_int {
        c = 1 as core::ffi::c_int;
        while c < bw + 1 as core::ffi::c_int {
            let p0: uint8_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c - 1 as core::ffi::c_int) as usize];
            let p1: uint8_t = buffer[(r - 1 as core::ffi::c_int) as usize][c as usize];
            let p2: uint8_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c + 1 as core::ffi::c_int) as usize];
            let p3: uint8_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c + 2 as core::ffi::c_int) as usize];
            let p4: uint8_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c + 3 as core::ffi::c_int) as usize];
            let p5: uint8_t = buffer[r as usize][(c - 1 as core::ffi::c_int) as usize];
            let p6: uint8_t = buffer[(r + 1 as core::ffi::c_int)
                as usize][(c - 1 as core::ffi::c_int) as usize];
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < 8 as core::ffi::c_int {
                let mut r_offset: core::ffi::c_int = k >> 2 as core::ffi::c_int;
                let mut c_offset: core::ffi::c_int = k & 0x3 as core::ffi::c_int;
                let mut pr: core::ffi::c_int = av1_filter_intra_taps.0[mode
                    as usize][k as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_int * p0 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][1 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p1 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][2 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p2 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][3 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p3 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][4 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p4 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][5 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p5 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][6 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p6 as core::ffi::c_int;
                buffer[(r + r_offset) as usize][(c + c_offset) as usize] = clip_pixel(
                    pr
                        + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 4 as core::ffi::c_int,
                );
                k += 1;
            }
            c += 4 as core::ffi::c_int;
        }
        r += 2 as core::ffi::c_int;
    }
    r = 0 as core::ffi::c_int;
    while r < bh {
        memcpy(
            dst as *mut core::ffi::c_void,
            &raw mut *(*buffer.as_mut_ptr().offset((r + 1 as core::ffi::c_int) as isize))
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize) as *mut uint8_t
                as *const core::ffi::c_void,
            (bw as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
unsafe extern "C-unwind" fn highbd_filter_intra_predictor(
    mut dst: *mut uint16_t,
    mut stride: ptrdiff_t,
    mut tx_size: TX_SIZE,
    mut above: *const uint16_t,
    mut left: *const uint16_t,
    mut mode: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    let mut r: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut buffer: [[uint16_t; 33]; 33] = [[0; 33]; 33];
    let bw: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let bh: core::ffi::c_int = tx_size_high[tx_size as usize];
    if !(bw <= 32 as core::ffi::c_int && bh <= 32 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 30],
                [core::ffi::c_char; 30],
            >(*b"highbd_filter_intra_predictor\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            918 as core::ffi::c_int,
            b"bw <= 32 && bh <= 32\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    r = 0 as core::ffi::c_int;
    while r < bh {
        buffer[(r + 1 as core::ffi::c_int) as usize][0 as core::ffi::c_int as usize] = *left
            .offset(r as isize);
        r += 1;
    }
    memcpy(
        (buffer[0 as core::ffi::c_int as usize]).as_mut_ptr() as *mut core::ffi::c_void,
        &*above.offset(-(1 as core::ffi::c_int) as isize) as *const uint16_t
            as *const core::ffi::c_void,
        ((bw + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    r = 1 as core::ffi::c_int;
    while r < bh + 1 as core::ffi::c_int {
        c = 1 as core::ffi::c_int;
        while c < bw + 1 as core::ffi::c_int {
            let p0: uint16_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c - 1 as core::ffi::c_int) as usize];
            let p1: uint16_t = buffer[(r - 1 as core::ffi::c_int) as usize][c as usize];
            let p2: uint16_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c + 1 as core::ffi::c_int) as usize];
            let p3: uint16_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c + 2 as core::ffi::c_int) as usize];
            let p4: uint16_t = buffer[(r - 1 as core::ffi::c_int)
                as usize][(c + 3 as core::ffi::c_int) as usize];
            let p5: uint16_t = buffer[r as usize][(c - 1 as core::ffi::c_int) as usize];
            let p6: uint16_t = buffer[(r + 1 as core::ffi::c_int)
                as usize][(c - 1 as core::ffi::c_int) as usize];
            let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
            while k < 8 as core::ffi::c_int {
                let mut r_offset: core::ffi::c_int = k >> 2 as core::ffi::c_int;
                let mut c_offset: core::ffi::c_int = k & 0x3 as core::ffi::c_int;
                let mut pr: core::ffi::c_int = av1_filter_intra_taps.0[mode
                    as usize][k as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_int * p0 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][1 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p1 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][2 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p2 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][3 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p3 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][4 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p4 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][5 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p5 as core::ffi::c_int
                    + av1_filter_intra_taps.0[mode
                        as usize][k as usize][6 as core::ffi::c_int as usize]
                        as core::ffi::c_int * p6 as core::ffi::c_int;
                buffer[(r + r_offset) as usize][(c + c_offset) as usize] = clip_pixel_highbd(
                    pr
                        + ((1 as core::ffi::c_int) << 4 as core::ffi::c_int
                            >> 1 as core::ffi::c_int) >> 4 as core::ffi::c_int,
                    bd,
                );
                k += 1;
            }
            c += 4 as core::ffi::c_int;
        }
        r += 2 as core::ffi::c_int;
    }
    r = 0 as core::ffi::c_int;
    while r < bh {
        memcpy(
            dst as *mut core::ffi::c_void,
            &raw mut *(*buffer.as_mut_ptr().offset((r + 1 as core::ffi::c_int) as isize))
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize) as *mut uint16_t
                as *const core::ffi::c_void,
            (bw as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
        );
        dst = dst.offset(stride as isize);
        r += 1;
    }
}
unsafe extern "C-unwind" fn is_smooth(
    mut mbmi: *const MB_MODE_INFO,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    if plane == 0 as core::ffi::c_int {
        let mode: PREDICTION_MODE = (*mbmi).mode;
        return (mode as core::ffi::c_int == SMOOTH_PRED as core::ffi::c_int
            || mode as core::ffi::c_int == SMOOTH_V_PRED as core::ffi::c_int
            || mode as core::ffi::c_int == SMOOTH_H_PRED as core::ffi::c_int)
            as core::ffi::c_int;
    } else {
        if is_inter_block(mbmi) != 0 {
            return 0 as core::ffi::c_int;
        }
        let uv_mode: UV_PREDICTION_MODE = (*mbmi).uv_mode;
        return (uv_mode as core::ffi::c_int == UV_SMOOTH_PRED as core::ffi::c_int
            || uv_mode as core::ffi::c_int == UV_SMOOTH_V_PRED as core::ffi::c_int
            || uv_mode as core::ffi::c_int == UV_SMOOTH_H_PRED as core::ffi::c_int)
            as core::ffi::c_int;
    };
}
unsafe extern "C-unwind" fn get_intra_edge_filter_type(
    mut xd: *const MACROBLOCKD,
    mut plane: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut above: *const MB_MODE_INFO = 0 as *const MB_MODE_INFO;
    let mut left: *const MB_MODE_INFO = 0 as *const MB_MODE_INFO;
    if plane == 0 as core::ffi::c_int {
        above = (*xd).above_mbmi;
        left = (*xd).left_mbmi;
    } else {
        above = (*xd).chroma_above_mbmi;
        left = (*xd).chroma_left_mbmi;
    }
    return (!above.is_null() && is_smooth(above, plane) != 0
        || !left.is_null() && is_smooth(left, plane) != 0) as core::ffi::c_int;
}
unsafe extern "C-unwind" fn intra_edge_filter_strength(
    mut bs0: core::ffi::c_int,
    mut bs1: core::ffi::c_int,
    mut delta: core::ffi::c_int,
    mut type_0: core::ffi::c_int,
) -> core::ffi::c_int {
    let d: core::ffi::c_int = abs(delta) as core::ffi::c_int;
    let mut strength: core::ffi::c_int = 0 as core::ffi::c_int;
    let blk_wh: core::ffi::c_int = bs0 + bs1;
    if type_0 == 0 as core::ffi::c_int {
        if blk_wh <= 8 as core::ffi::c_int {
            if d >= 56 as core::ffi::c_int {
                strength = 1 as core::ffi::c_int;
            }
        } else if blk_wh <= 12 as core::ffi::c_int {
            if d >= 40 as core::ffi::c_int {
                strength = 1 as core::ffi::c_int;
            }
        } else if blk_wh <= 16 as core::ffi::c_int {
            if d >= 40 as core::ffi::c_int {
                strength = 1 as core::ffi::c_int;
            }
        } else if blk_wh <= 24 as core::ffi::c_int {
            if d >= 8 as core::ffi::c_int {
                strength = 1 as core::ffi::c_int;
            }
            if d >= 16 as core::ffi::c_int {
                strength = 2 as core::ffi::c_int;
            }
            if d >= 32 as core::ffi::c_int {
                strength = 3 as core::ffi::c_int;
            }
        } else if blk_wh <= 32 as core::ffi::c_int {
            if d >= 1 as core::ffi::c_int {
                strength = 1 as core::ffi::c_int;
            }
            if d >= 4 as core::ffi::c_int {
                strength = 2 as core::ffi::c_int;
            }
            if d >= 32 as core::ffi::c_int {
                strength = 3 as core::ffi::c_int;
            }
        } else if d >= 1 as core::ffi::c_int {
            strength = 3 as core::ffi::c_int;
        }
    } else if blk_wh <= 8 as core::ffi::c_int {
        if d >= 40 as core::ffi::c_int {
            strength = 1 as core::ffi::c_int;
        }
        if d >= 64 as core::ffi::c_int {
            strength = 2 as core::ffi::c_int;
        }
    } else if blk_wh <= 16 as core::ffi::c_int {
        if d >= 20 as core::ffi::c_int {
            strength = 1 as core::ffi::c_int;
        }
        if d >= 48 as core::ffi::c_int {
            strength = 2 as core::ffi::c_int;
        }
    } else if blk_wh <= 24 as core::ffi::c_int {
        if d >= 4 as core::ffi::c_int {
            strength = 3 as core::ffi::c_int;
        }
    } else if d >= 1 as core::ffi::c_int {
        strength = 3 as core::ffi::c_int;
    }
    return strength;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_filter_intra_edge_c(
    mut p: *mut uint8_t,
    mut sz: core::ffi::c_int,
    mut strength: core::ffi::c_int,
) {
    if strength == 0 {
        return;
    }
    let kernel: [[core::ffi::c_int; 5]; 3] = [
        [
            0 as core::ffi::c_int,
            4 as core::ffi::c_int,
            8 as core::ffi::c_int,
            4 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ],
        [
            0 as core::ffi::c_int,
            5 as core::ffi::c_int,
            6 as core::ffi::c_int,
            5 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ],
        [
            2 as core::ffi::c_int,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
            2 as core::ffi::c_int,
        ],
    ];
    let filt: core::ffi::c_int = strength - 1 as core::ffi::c_int;
    let mut edge: [uint8_t; 129] = [0; 129];
    memcpy(
        edge.as_mut_ptr() as *mut core::ffi::c_void,
        p as *const core::ffi::c_void,
        (sz as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
    while i < sz {
        let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < INTRA_EDGE_TAPS {
            let mut k: core::ffi::c_int = i - 2 as core::ffi::c_int + j;
            k = if k < 0 as core::ffi::c_int { 0 as core::ffi::c_int } else { k };
            k = if k > sz - 1 as core::ffi::c_int {
                sz - 1 as core::ffi::c_int
            } else {
                k
            };
            s
                += edge[k as usize] as core::ffi::c_int
                    * kernel[filt as usize][j as usize];
            j += 1;
        }
        s = s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
        *p.offset(i as isize) = s as uint8_t;
        i += 1;
    }
}
unsafe extern "C-unwind" fn filter_intra_edge_corner(
    mut p_above: *mut uint8_t,
    mut p_left: *mut uint8_t,
) {
    let kernel: [core::ffi::c_int; 3] = [
        5 as core::ffi::c_int,
        6 as core::ffi::c_int,
        5 as core::ffi::c_int,
    ];
    let mut s: core::ffi::c_int = *p_left.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int * kernel[0 as core::ffi::c_int as usize]
        + *p_above.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
            * kernel[1 as core::ffi::c_int as usize]
        + *p_above.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            * kernel[2 as core::ffi::c_int as usize];
    s = s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
    *p_above.offset(-(1 as core::ffi::c_int) as isize) = s as uint8_t;
    *p_left.offset(-(1 as core::ffi::c_int) as isize) = s as uint8_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_upsample_intra_edge_c(
    mut p: *mut uint8_t,
    mut sz: core::ffi::c_int,
) {
    if !(sz <= 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"av1_upsample_intra_edge_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1063 as core::ffi::c_int,
            b"sz <= MAX_UPSAMPLE_SZ\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut in_0: [uint8_t; 19] = [0; 19];
    in_0[0 as core::ffi::c_int as usize] = *p.offset(-(1 as core::ffi::c_int) as isize);
    in_0[1 as core::ffi::c_int as usize] = *p.offset(-(1 as core::ffi::c_int) as isize);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < sz {
        in_0[(i + 2 as core::ffi::c_int) as usize] = *p.offset(i as isize);
        i += 1;
    }
    in_0[(sz + 2 as core::ffi::c_int) as usize] = *p
        .offset((sz - 1 as core::ffi::c_int) as isize);
    *p.offset(-(2 as core::ffi::c_int) as isize) = in_0[0 as core::ffi::c_int as usize];
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < sz {
        let mut s: core::ffi::c_int = -(in_0[i_0 as usize] as core::ffi::c_int)
            + 9 as core::ffi::c_int
                * in_0[(i_0 + 1 as core::ffi::c_int) as usize] as core::ffi::c_int
            + 9 as core::ffi::c_int
                * in_0[(i_0 + 2 as core::ffi::c_int) as usize] as core::ffi::c_int
            - in_0[(i_0 + 3 as core::ffi::c_int) as usize] as core::ffi::c_int;
        s = clip_pixel(s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int)
            as core::ffi::c_int;
        *p.offset((2 as core::ffi::c_int * i_0 - 1 as core::ffi::c_int) as isize) = s
            as uint8_t;
        *p.offset((2 as core::ffi::c_int * i_0) as isize) = in_0[(i_0
            + 2 as core::ffi::c_int) as usize];
        i_0 += 1;
    }
}
unsafe extern "C-unwind" fn build_directional_and_filter_intra_predictors(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut mode: PREDICTION_MODE,
    mut p_angle: core::ffi::c_int,
    mut filter_intra_mode: FILTER_INTRA_MODE,
    mut tx_size: TX_SIZE,
    mut disable_edge_filter: core::ffi::c_int,
    mut n_top_px: core::ffi::c_int,
    mut n_topright_px: core::ffi::c_int,
    mut n_left_px: core::ffi::c_int,
    mut n_bottomleft_px: core::ffi::c_int,
    mut intra_edge_filter_type: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut above_ref: *const uint8_t = ref_0.offset(-(ref_stride as isize));
    let mut left_ref: *const uint8_t = ref_0.offset(-(1 as core::ffi::c_int as isize));
    let mut left_data: crate::platform::Aligned16<[uint8_t; 160]> = crate::platform::Aligned16([0; 160]);
    let mut above_data: crate::platform::Aligned16<[uint8_t; 160]> = crate::platform::Aligned16([0; 160]);
    let above_row: *mut uint8_t = above_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    let left_col: *mut uint8_t = left_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    let txwpx: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let txhpx: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut need_left: core::ffi::c_int = extend_modes[mode as usize] as core::ffi::c_int
        & NEED_LEFT as core::ffi::c_int;
    let mut need_above: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVE as core::ffi::c_int;
    let mut need_above_left: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVELEFT as core::ffi::c_int;
    let is_dr_mode: core::ffi::c_int = av1_is_directional_mode(mode) as core::ffi::c_int;
    let use_filter_intra: core::ffi::c_int = (filter_intra_mode as core::ffi::c_int
        != FILTER_INTRA_MODES as core::ffi::c_int) as core::ffi::c_int;
    if !(use_filter_intra != 0 || is_dr_mode != 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1103 as core::ffi::c_int,
            b"use_filter_intra || is_dr_mode\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    memset(
        left_data.0.as_mut_ptr() as *mut core::ffi::c_void,
        129 as core::ffi::c_int,
        NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
    );
    memset(
        above_data.0.as_mut_ptr() as *mut core::ffi::c_void,
        127 as core::ffi::c_int,
        NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
    );
    if is_dr_mode != 0 {
        if p_angle <= 90 as core::ffi::c_int {
            need_above = 1 as core::ffi::c_int;
            need_left = 0 as core::ffi::c_int;
            need_above_left = 1 as core::ffi::c_int;
        } else if p_angle < 180 as core::ffi::c_int {
            need_above = 1 as core::ffi::c_int;
            need_left = 1 as core::ffi::c_int;
            need_above_left = 1 as core::ffi::c_int;
        } else {
            need_above = 0 as core::ffi::c_int;
            need_left = 1 as core::ffi::c_int;
            need_above_left = 1 as core::ffi::c_int;
        }
    }
    if use_filter_intra != 0 {
        need_above_left = 1 as core::ffi::c_int;
        need_above = need_above_left;
        need_left = need_above;
    }
    if !(n_top_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1129 as core::ffi::c_int,
            b"n_top_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_topright_px >= -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1130 as core::ffi::c_int,
            b"n_topright_px >= -1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_left_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1131 as core::ffi::c_int,
            b"n_left_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_bottomleft_px >= -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1132 as core::ffi::c_int,
            b"n_bottomleft_px >= -1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if need_above == 0 && n_left_px == 0 as core::ffi::c_int
        || need_left == 0 && n_top_px == 0 as core::ffi::c_int
    {
        let mut val: core::ffi::c_int = 0;
        if need_left != 0 {
            val = if n_top_px > 0 as core::ffi::c_int {
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                129 as core::ffi::c_int
            };
        } else {
            val = if n_left_px > 0 as core::ffi::c_int {
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                127 as core::ffi::c_int
            };
        }
        i = 0 as core::ffi::c_int;
        while i < txhpx {
            memset(dst as *mut core::ffi::c_void, val, txwpx as size_t);
            dst = dst.offset(dst_stride as isize);
            i += 1;
        }
        return;
    }
    if need_left != 0 {
        let num_left_pixels_needed: core::ffi::c_int = txhpx
            + (if n_bottomleft_px >= 0 as core::ffi::c_int {
                txwpx
            } else {
                0 as core::ffi::c_int
            });
        i = 0 as core::ffi::c_int;
        if n_left_px > 0 as core::ffi::c_int {
            while i < n_left_px {
                *left_col.offset(i as isize) = *left_ref
                    .offset((i * ref_stride) as isize);
                i += 1;
            }
            if n_bottomleft_px > 0 as core::ffi::c_int {
                if !(i == txhpx) as core::ffi::c_int as core::ffi::c_long != 0 {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 46],
                            [core::ffi::c_char; 46],
                        >(*b"build_directional_and_filter_intra_predictors\0"))
                            .as_ptr(),
                        b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                        1156 as core::ffi::c_int,
                        b"i == txhpx\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                while i < txhpx + n_bottomleft_px {
                    *left_col.offset(i as isize) = *left_ref
                        .offset((i * ref_stride) as isize);
                    i += 1;
                }
            }
            if i < num_left_pixels_needed {
                memset(
                    &raw mut *left_col.offset(i as isize) as *mut uint8_t
                        as *mut core::ffi::c_void,
                    *left_col.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (num_left_pixels_needed - i) as size_t,
                );
            }
        } else if n_top_px > 0 as core::ffi::c_int {
            memset(
                left_col as *mut core::ffi::c_void,
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                num_left_pixels_needed as size_t,
            );
        }
    }
    if need_above != 0 {
        let num_top_pixels_needed: core::ffi::c_int = txwpx
            + (if n_topright_px >= 0 as core::ffi::c_int {
                txhpx
            } else {
                0 as core::ffi::c_int
            });
        if n_top_px > 0 as core::ffi::c_int {
            memcpy(
                above_row as *mut core::ffi::c_void,
                above_ref as *const core::ffi::c_void,
                n_top_px as size_t,
            );
            i = n_top_px;
            if n_topright_px > 0 as core::ffi::c_int {
                if !(n_top_px == txwpx) as core::ffi::c_int as core::ffi::c_long != 0 {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 46],
                            [core::ffi::c_char; 46],
                        >(*b"build_directional_and_filter_intra_predictors\0"))
                            .as_ptr(),
                        b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                        1174 as core::ffi::c_int,
                        b"n_top_px == txwpx\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                memcpy(
                    above_row.offset(txwpx as isize) as *mut core::ffi::c_void,
                    above_ref.offset(txwpx as isize) as *const core::ffi::c_void,
                    n_topright_px as size_t,
                );
                i += n_topright_px;
            }
            if i < num_top_pixels_needed {
                memset(
                    &raw mut *above_row.offset(i as isize) as *mut uint8_t
                        as *mut core::ffi::c_void,
                    *above_row.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (num_top_pixels_needed - i) as size_t,
                );
            }
        } else if n_left_px > 0 as core::ffi::c_int {
            memset(
                above_row as *mut core::ffi::c_void,
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                num_top_pixels_needed as size_t,
            );
        }
    }
    if need_above_left != 0 {
        if n_top_px > 0 as core::ffi::c_int && n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(-(1 as core::ffi::c_int) as isize);
        } else if n_top_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(0 as core::ffi::c_int as isize);
        } else if n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *left_ref
                .offset(0 as core::ffi::c_int as isize);
        } else {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = 128 as uint8_t;
        }
        *left_col.offset(-(1 as core::ffi::c_int) as isize) = *above_row
            .offset(-(1 as core::ffi::c_int) as isize);
    }
    if use_filter_intra != 0 {
        av1_filter_intra_predictor
            .expect(
                "non-null function pointer",
            )(
            dst,
            dst_stride as ptrdiff_t,
            tx_size,
            above_row,
            left_col,
            filter_intra_mode as core::ffi::c_int,
        );
        return;
    }
    if (is_dr_mode == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1204 as core::ffi::c_int,
            b"is_dr_mode\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut upsample_above: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut upsample_left: core::ffi::c_int = 0 as core::ffi::c_int;
    if disable_edge_filter == 0 {
        let need_right: core::ffi::c_int = (p_angle < 90 as core::ffi::c_int)
            as core::ffi::c_int;
        let need_bottom: core::ffi::c_int = (p_angle > 180 as core::ffi::c_int)
            as core::ffi::c_int;
        if p_angle != 90 as core::ffi::c_int && p_angle != 180 as core::ffi::c_int {
            if (need_above_left == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 46],
                        [core::ffi::c_char; 46],
                    >(*b"build_directional_and_filter_intra_predictors\0"))
                        .as_ptr(),
                    b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                    1211 as core::ffi::c_int,
                    b"need_above_left\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let ab_le: core::ffi::c_int = 1 as core::ffi::c_int;
            if need_above != 0 && need_left != 0
                && txwpx + txhpx >= 24 as core::ffi::c_int
            {
                filter_intra_edge_corner(above_row, left_col);
            }
            if need_above != 0 && n_top_px > 0 as core::ffi::c_int {
                let strength: core::ffi::c_int = intra_edge_filter_strength(
                    txwpx,
                    txhpx,
                    p_angle - 90 as core::ffi::c_int,
                    intra_edge_filter_type,
                ) as core::ffi::c_int;
                let n_px: core::ffi::c_int = n_top_px + ab_le
                    + (if need_right != 0 { txhpx } else { 0 as core::ffi::c_int });
                av1_filter_intra_edge
                    .expect(
                        "non-null function pointer",
                    )(above_row.offset(-(ab_le as isize)), n_px, strength);
            }
            if need_left != 0 && n_left_px > 0 as core::ffi::c_int {
                let strength_0: core::ffi::c_int = intra_edge_filter_strength(
                    txhpx,
                    txwpx,
                    p_angle - 180 as core::ffi::c_int,
                    intra_edge_filter_type,
                ) as core::ffi::c_int;
                let n_px_0: core::ffi::c_int = n_left_px + ab_le
                    + (if need_bottom != 0 { txwpx } else { 0 as core::ffi::c_int });
                av1_filter_intra_edge
                    .expect(
                        "non-null function pointer",
                    )(left_col.offset(-(ab_le as isize)), n_px_0, strength_0);
            }
        }
        upsample_above = av1_use_intra_edge_upsample(
            txwpx,
            txhpx,
            p_angle - 90 as core::ffi::c_int,
            intra_edge_filter_type,
        );
        if need_above != 0 && upsample_above != 0 {
            let n_px_1: core::ffi::c_int = txwpx
                + (if need_right != 0 { txhpx } else { 0 as core::ffi::c_int });
            av1_upsample_intra_edge
                .expect("non-null function pointer")(above_row, n_px_1);
        }
        upsample_left = av1_use_intra_edge_upsample(
            txhpx,
            txwpx,
            p_angle - 180 as core::ffi::c_int,
            intra_edge_filter_type,
        );
        if need_left != 0 && upsample_left != 0 {
            let n_px_2: core::ffi::c_int = txhpx
                + (if need_bottom != 0 { txwpx } else { 0 as core::ffi::c_int });
            av1_upsample_intra_edge
                .expect("non-null function pointer")(left_col, n_px_2);
        }
    }
    dr_predictor(
        dst,
        dst_stride as ptrdiff_t,
        tx_size,
        above_row,
        left_col,
        upsample_above,
        upsample_left,
        p_angle,
    );
}
unsafe extern "C-unwind" fn build_non_directional_intra_predictors(
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut mode: PREDICTION_MODE,
    mut tx_size: TX_SIZE,
    mut n_top_px: core::ffi::c_int,
    mut n_left_px: core::ffi::c_int,
) {
    let mut above_ref: *const uint8_t = ref_0.offset(-(ref_stride as isize));
    let mut left_ref: *const uint8_t = ref_0.offset(-(1 as core::ffi::c_int as isize));
    let txwpx: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let txhpx: core::ffi::c_int = tx_size_high[tx_size as usize];
    let need_left: core::ffi::c_int = extend_modes[mode as usize] as core::ffi::c_int
        & NEED_LEFT as core::ffi::c_int as core::ffi::c_int;
    let need_above: core::ffi::c_int = extend_modes[mode as usize] as core::ffi::c_int
        & NEED_ABOVE as core::ffi::c_int as core::ffi::c_int;
    let need_above_left: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVELEFT as core::ffi::c_int as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    if !(n_top_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"build_non_directional_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1259 as core::ffi::c_int,
            b"n_top_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_left_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"build_non_directional_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1260 as core::ffi::c_int,
            b"n_left_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(mode as core::ffi::c_int == DC_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_V_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_H_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == PAETH_PRED as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"build_non_directional_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1262 as core::ffi::c_int,
            b"mode == DC_PRED || mode == SMOOTH_PRED || mode == SMOOTH_V_PRED || mode == SMOOTH_H_PRED || mode == PAETH_PRED\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if need_above == 0 && n_left_px == 0 as core::ffi::c_int
        || need_left == 0 && n_top_px == 0 as core::ffi::c_int
    {
        let mut val: core::ffi::c_int = 0 as core::ffi::c_int;
        if need_left != 0 {
            val = if n_top_px > 0 as core::ffi::c_int {
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                129 as core::ffi::c_int
            };
        } else {
            val = if n_left_px > 0 as core::ffi::c_int {
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                127 as core::ffi::c_int
            };
        }
        i = 0 as core::ffi::c_int;
        while i < txhpx {
            memset(dst as *mut core::ffi::c_void, val, txwpx as size_t);
            dst = dst.offset(dst_stride as isize);
            i += 1;
        }
        return;
    }
    let mut left_data: crate::platform::Aligned16<[uint8_t; 160]> = crate::platform::Aligned16([0; 160]);
    let mut above_data: crate::platform::Aligned16<[uint8_t; 160]> = crate::platform::Aligned16([0; 160]);
    let above_row: *mut uint8_t = above_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    let left_col: *mut uint8_t = left_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    if need_left != 0 {
        memset(
            left_data.0.as_mut_ptr() as *mut core::ffi::c_void,
            129 as core::ffi::c_int,
            NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
        );
        if n_left_px > 0 as core::ffi::c_int {
            i = 0 as core::ffi::c_int;
            while i < n_left_px {
                *left_col.offset(i as isize) = *left_ref
                    .offset((i * ref_stride) as isize);
                i += 1;
            }
            if i < txhpx {
                memset(
                    &raw mut *left_col.offset(i as isize) as *mut uint8_t
                        as *mut core::ffi::c_void,
                    *left_col.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (txhpx - i) as size_t,
                );
            }
        } else if n_top_px > 0 as core::ffi::c_int {
            memset(
                left_col as *mut core::ffi::c_void,
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                txhpx as size_t,
            );
        }
    }
    if need_above != 0 {
        memset(
            above_data.0.as_mut_ptr() as *mut core::ffi::c_void,
            127 as core::ffi::c_int,
            NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
        );
        if n_top_px > 0 as core::ffi::c_int {
            memcpy(
                above_row as *mut core::ffi::c_void,
                above_ref as *const core::ffi::c_void,
                n_top_px as size_t,
            );
            i = n_top_px;
            if i < txwpx {
                memset(
                    &raw mut *above_row.offset(i as isize) as *mut uint8_t
                        as *mut core::ffi::c_void,
                    *above_row.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (txwpx - i) as size_t,
                );
            }
        } else if n_left_px > 0 as core::ffi::c_int {
            memset(
                above_row as *mut core::ffi::c_void,
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                txwpx as size_t,
            );
        }
    }
    if need_above_left != 0 {
        if n_top_px > 0 as core::ffi::c_int && n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(-(1 as core::ffi::c_int) as isize);
        } else if n_top_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(0 as core::ffi::c_int as isize);
        } else if n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *left_ref
                .offset(0 as core::ffi::c_int as isize);
        } else {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = 128 as uint8_t;
        }
        *left_col.offset(-(1 as core::ffi::c_int) as isize) = *above_row
            .offset(-(1 as core::ffi::c_int) as isize);
    }
    if mode as core::ffi::c_int == DC_PRED as core::ffi::c_int {
        (dc_pred[(n_left_px > 0 as core::ffi::c_int) as core::ffi::c_int
            as usize][(n_top_px > 0 as core::ffi::c_int) as core::ffi::c_int
            as usize][tx_size as usize])
            .expect(
                "non-null function pointer",
            )(dst, dst_stride as ptrdiff_t, above_row, left_col);
    } else {
        (pred[mode as usize][tx_size as usize])
            .expect(
                "non-null function pointer",
            )(dst, dst_stride as ptrdiff_t, above_row, left_col);
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_filter_intra_edge_c(
    mut p: *mut uint16_t,
    mut sz: core::ffi::c_int,
    mut strength: core::ffi::c_int,
) {
    if strength == 0 {
        return;
    }
    let kernel: [[core::ffi::c_int; 5]; 3] = [
        [
            0 as core::ffi::c_int,
            4 as core::ffi::c_int,
            8 as core::ffi::c_int,
            4 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ],
        [
            0 as core::ffi::c_int,
            5 as core::ffi::c_int,
            6 as core::ffi::c_int,
            5 as core::ffi::c_int,
            0 as core::ffi::c_int,
        ],
        [
            2 as core::ffi::c_int,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
            4 as core::ffi::c_int,
            2 as core::ffi::c_int,
        ],
    ];
    let filt: core::ffi::c_int = strength - 1 as core::ffi::c_int;
    let mut edge: [uint16_t; 129] = [0; 129];
    memcpy(
        edge.as_mut_ptr() as *mut core::ffi::c_void,
        p as *const core::ffi::c_void,
        (sz as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
    while i < sz {
        let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < INTRA_EDGE_TAPS {
            let mut k: core::ffi::c_int = i - 2 as core::ffi::c_int + j;
            k = if k < 0 as core::ffi::c_int { 0 as core::ffi::c_int } else { k };
            k = if k > sz - 1 as core::ffi::c_int {
                sz - 1 as core::ffi::c_int
            } else {
                k
            };
            s
                += edge[k as usize] as core::ffi::c_int
                    * kernel[filt as usize][j as usize];
            j += 1;
        }
        s = s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
        *p.offset(i as isize) = s as uint16_t;
        i += 1;
    }
}
unsafe extern "C-unwind" fn highbd_filter_intra_edge_corner(
    mut p_above: *mut uint16_t,
    mut p_left: *mut uint16_t,
) {
    let kernel: [core::ffi::c_int; 3] = [
        5 as core::ffi::c_int,
        6 as core::ffi::c_int,
        5 as core::ffi::c_int,
    ];
    let mut s: core::ffi::c_int = *p_left.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int * kernel[0 as core::ffi::c_int as usize]
        + *p_above.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
            * kernel[1 as core::ffi::c_int as usize]
        + *p_above.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            * kernel[2 as core::ffi::c_int as usize];
    s = s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
    *p_above.offset(-(1 as core::ffi::c_int) as isize) = s as uint16_t;
    *p_left.offset(-(1 as core::ffi::c_int) as isize) = s as uint16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_upsample_intra_edge_c(
    mut p: *mut uint16_t,
    mut sz: core::ffi::c_int,
    mut bd: core::ffi::c_int,
) {
    if !(sz <= 16 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_highbd_upsample_intra_edge_c\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1362 as core::ffi::c_int,
            b"sz <= MAX_UPSAMPLE_SZ\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut in_0: [uint16_t; 19] = [0; 19];
    in_0[0 as core::ffi::c_int as usize] = *p.offset(-(1 as core::ffi::c_int) as isize);
    in_0[1 as core::ffi::c_int as usize] = *p.offset(-(1 as core::ffi::c_int) as isize);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < sz {
        in_0[(i + 2 as core::ffi::c_int) as usize] = *p.offset(i as isize);
        i += 1;
    }
    in_0[(sz + 2 as core::ffi::c_int) as usize] = *p
        .offset((sz - 1 as core::ffi::c_int) as isize);
    *p.offset(-(2 as core::ffi::c_int) as isize) = in_0[0 as core::ffi::c_int as usize];
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < sz {
        let mut s: core::ffi::c_int = -(in_0[i_0 as usize] as core::ffi::c_int)
            + 9 as core::ffi::c_int
                * in_0[(i_0 + 1 as core::ffi::c_int) as usize] as core::ffi::c_int
            + 9 as core::ffi::c_int
                * in_0[(i_0 + 2 as core::ffi::c_int) as usize] as core::ffi::c_int
            - in_0[(i_0 + 3 as core::ffi::c_int) as usize] as core::ffi::c_int;
        s = s + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
        s = clip_pixel_highbd(s, bd) as core::ffi::c_int;
        *p.offset((2 as core::ffi::c_int * i_0 - 1 as core::ffi::c_int) as isize) = s
            as uint16_t;
        *p.offset((2 as core::ffi::c_int * i_0) as isize) = in_0[(i_0
            + 2 as core::ffi::c_int) as usize];
        i_0 += 1;
    }
}
unsafe extern "C-unwind" fn highbd_build_directional_and_filter_intra_predictors(
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut mode: PREDICTION_MODE,
    mut p_angle: core::ffi::c_int,
    mut filter_intra_mode: FILTER_INTRA_MODE,
    mut tx_size: TX_SIZE,
    mut disable_edge_filter: core::ffi::c_int,
    mut n_top_px: core::ffi::c_int,
    mut n_topright_px: core::ffi::c_int,
    mut n_left_px: core::ffi::c_int,
    mut n_bottomleft_px: core::ffi::c_int,
    mut intra_edge_filter_type: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let ref_0: *const uint16_t = ((ref8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let mut left_data: crate::platform::Aligned16<[uint16_t; 160]> = crate::platform::Aligned16([0; 160]);
    let mut above_data: crate::platform::Aligned16<[uint16_t; 160]> = crate::platform::Aligned16([0; 160]);
    let above_row: *mut uint16_t = above_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    let left_col: *mut uint16_t = left_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    let txwpx: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let txhpx: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut need_left: core::ffi::c_int = extend_modes[mode as usize] as core::ffi::c_int
        & NEED_LEFT as core::ffi::c_int;
    let mut need_above: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVE as core::ffi::c_int;
    let mut need_above_left: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVELEFT as core::ffi::c_int;
    let mut above_ref: *const uint16_t = ref_0.offset(-(ref_stride as isize));
    let mut left_ref: *const uint16_t = ref_0.offset(-(1 as core::ffi::c_int as isize));
    let is_dr_mode: core::ffi::c_int = av1_is_directional_mode(mode) as core::ffi::c_int;
    let use_filter_intra: core::ffi::c_int = (filter_intra_mode as core::ffi::c_int
        != FILTER_INTRA_MODES as core::ffi::c_int) as core::ffi::c_int;
    if !(use_filter_intra != 0 || is_dr_mode != 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1406 as core::ffi::c_int,
            b"use_filter_intra || is_dr_mode\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let base: core::ffi::c_int = (128 as core::ffi::c_int)
        << bit_depth - 8 as core::ffi::c_int;
    aom_memset16(
        left_data.0.as_mut_ptr() as *mut core::ffi::c_void,
        base + 1 as core::ffi::c_int,
        NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
    );
    aom_memset16(
        above_data.0.as_mut_ptr() as *mut core::ffi::c_void,
        base - 1 as core::ffi::c_int,
        NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
    );
    if is_dr_mode != 0 {
        if p_angle <= 90 as core::ffi::c_int {
            need_above = 1 as core::ffi::c_int;
            need_left = 0 as core::ffi::c_int;
            need_above_left = 1 as core::ffi::c_int;
        } else if p_angle < 180 as core::ffi::c_int {
            need_above = 1 as core::ffi::c_int;
            need_left = 1 as core::ffi::c_int;
            need_above_left = 1 as core::ffi::c_int;
        } else {
            need_above = 0 as core::ffi::c_int;
            need_left = 1 as core::ffi::c_int;
            need_above_left = 1 as core::ffi::c_int;
        }
    }
    if use_filter_intra != 0 {
        need_above_left = 1 as core::ffi::c_int;
        need_above = need_above_left;
        need_left = need_above;
    }
    if !(n_top_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1432 as core::ffi::c_int,
            b"n_top_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_topright_px >= -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1433 as core::ffi::c_int,
            b"n_topright_px >= -1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_left_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1434 as core::ffi::c_int,
            b"n_left_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_bottomleft_px >= -(1 as core::ffi::c_int)) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1435 as core::ffi::c_int,
            b"n_bottomleft_px >= -1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if need_above == 0 && n_left_px == 0 as core::ffi::c_int
        || need_left == 0 && n_top_px == 0 as core::ffi::c_int
    {
        let mut val: core::ffi::c_int = 0;
        if need_left != 0 {
            val = if n_top_px > 0 as core::ffi::c_int {
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                base + 1 as core::ffi::c_int
            };
        } else {
            val = if n_left_px > 0 as core::ffi::c_int {
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                base - 1 as core::ffi::c_int
            };
        }
        i = 0 as core::ffi::c_int;
        while i < txhpx {
            aom_memset16(dst as *mut core::ffi::c_void, val, txwpx as size_t);
            dst = dst.offset(dst_stride as isize);
            i += 1;
        }
        return;
    }
    if need_left != 0 {
        let num_left_pixels_needed: core::ffi::c_int = txhpx
            + (if n_bottomleft_px >= 0 as core::ffi::c_int {
                txwpx
            } else {
                0 as core::ffi::c_int
            });
        i = 0 as core::ffi::c_int;
        if n_left_px > 0 as core::ffi::c_int {
            while i < n_left_px {
                *left_col.offset(i as isize) = *left_ref
                    .offset((i * ref_stride) as isize);
                i += 1;
            }
            if n_bottomleft_px > 0 as core::ffi::c_int {
                if !(i == txhpx) as core::ffi::c_int as core::ffi::c_long != 0 {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 53],
                            [core::ffi::c_char; 53],
                        >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                            .as_ptr(),
                        b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                        1459 as core::ffi::c_int,
                        b"i == txhpx\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                while i < txhpx + n_bottomleft_px {
                    *left_col.offset(i as isize) = *left_ref
                        .offset((i * ref_stride) as isize);
                    i += 1;
                }
            }
            if i < num_left_pixels_needed {
                aom_memset16(
                    &raw mut *left_col.offset(i as isize) as *mut uint16_t
                        as *mut core::ffi::c_void,
                    *left_col.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (num_left_pixels_needed - i) as size_t,
                );
            }
        } else if n_top_px > 0 as core::ffi::c_int {
            aom_memset16(
                left_col as *mut core::ffi::c_void,
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                num_left_pixels_needed as size_t,
            );
        }
    }
    if need_above != 0 {
        let num_top_pixels_needed: core::ffi::c_int = txwpx
            + (if n_topright_px >= 0 as core::ffi::c_int {
                txhpx
            } else {
                0 as core::ffi::c_int
            });
        if n_top_px > 0 as core::ffi::c_int {
            memcpy(
                above_row as *mut core::ffi::c_void,
                above_ref as *const core::ffi::c_void,
                (n_top_px as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            i = n_top_px;
            if n_topright_px > 0 as core::ffi::c_int {
                if !(n_top_px == txwpx) as core::ffi::c_int as core::ffi::c_long != 0 {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 53],
                            [core::ffi::c_char; 53],
                        >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                            .as_ptr(),
                        b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                        1477 as core::ffi::c_int,
                        b"n_top_px == txwpx\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                memcpy(
                    above_row.offset(txwpx as isize) as *mut core::ffi::c_void,
                    above_ref.offset(txwpx as isize) as *const core::ffi::c_void,
                    (n_topright_px as size_t)
                        .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
                );
                i += n_topright_px;
            }
            if i < num_top_pixels_needed {
                aom_memset16(
                    &raw mut *above_row.offset(i as isize) as *mut uint16_t
                        as *mut core::ffi::c_void,
                    *above_row.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (num_top_pixels_needed - i) as size_t,
                );
            }
        } else if n_left_px > 0 as core::ffi::c_int {
            aom_memset16(
                above_row as *mut core::ffi::c_void,
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                num_top_pixels_needed as size_t,
            );
        }
    }
    if need_above_left != 0 {
        if n_top_px > 0 as core::ffi::c_int && n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(-(1 as core::ffi::c_int) as isize);
        } else if n_top_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(0 as core::ffi::c_int as isize);
        } else if n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *left_ref
                .offset(0 as core::ffi::c_int as isize);
        } else {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = base as uint16_t;
        }
        *left_col.offset(-(1 as core::ffi::c_int) as isize) = *above_row
            .offset(-(1 as core::ffi::c_int) as isize);
    }
    if use_filter_intra != 0 {
        highbd_filter_intra_predictor(
            dst,
            dst_stride as ptrdiff_t,
            tx_size,
            above_row,
            left_col,
            filter_intra_mode as core::ffi::c_int,
            bit_depth,
        );
        return;
    }
    if (is_dr_mode == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1509 as core::ffi::c_int,
            b"is_dr_mode\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut upsample_above: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut upsample_left: core::ffi::c_int = 0 as core::ffi::c_int;
    if disable_edge_filter == 0 {
        let need_right: core::ffi::c_int = (p_angle < 90 as core::ffi::c_int)
            as core::ffi::c_int;
        let need_bottom: core::ffi::c_int = (p_angle > 180 as core::ffi::c_int)
            as core::ffi::c_int;
        if p_angle != 90 as core::ffi::c_int && p_angle != 180 as core::ffi::c_int {
            if (need_above_left == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 53],
                        [core::ffi::c_char; 53],
                    >(*b"highbd_build_directional_and_filter_intra_predictors\0"))
                        .as_ptr(),
                    b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                    1516 as core::ffi::c_int,
                    b"need_above_left\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let ab_le: core::ffi::c_int = 1 as core::ffi::c_int;
            if need_above != 0 && need_left != 0
                && txwpx + txhpx >= 24 as core::ffi::c_int
            {
                highbd_filter_intra_edge_corner(above_row, left_col);
            }
            if need_above != 0 && n_top_px > 0 as core::ffi::c_int {
                let strength: core::ffi::c_int = intra_edge_filter_strength(
                    txwpx,
                    txhpx,
                    p_angle - 90 as core::ffi::c_int,
                    intra_edge_filter_type,
                ) as core::ffi::c_int;
                let n_px: core::ffi::c_int = n_top_px + ab_le
                    + (if need_right != 0 { txhpx } else { 0 as core::ffi::c_int });
                av1_highbd_filter_intra_edge
                    .expect(
                        "non-null function pointer",
                    )(above_row.offset(-(ab_le as isize)), n_px, strength);
            }
            if need_left != 0 && n_left_px > 0 as core::ffi::c_int {
                let strength_0: core::ffi::c_int = intra_edge_filter_strength(
                    txhpx,
                    txwpx,
                    p_angle - 180 as core::ffi::c_int,
                    intra_edge_filter_type,
                ) as core::ffi::c_int;
                let n_px_0: core::ffi::c_int = n_left_px + ab_le
                    + (if need_bottom != 0 { txwpx } else { 0 as core::ffi::c_int });
                av1_highbd_filter_intra_edge
                    .expect(
                        "non-null function pointer",
                    )(left_col.offset(-(ab_le as isize)), n_px_0, strength_0);
            }
        }
        upsample_above = av1_use_intra_edge_upsample(
            txwpx,
            txhpx,
            p_angle - 90 as core::ffi::c_int,
            intra_edge_filter_type,
        );
        if need_above != 0 && upsample_above != 0 {
            let n_px_1: core::ffi::c_int = txwpx
                + (if need_right != 0 { txhpx } else { 0 as core::ffi::c_int });
            av1_highbd_upsample_intra_edge
                .expect("non-null function pointer")(above_row, n_px_1, bit_depth);
        }
        upsample_left = av1_use_intra_edge_upsample(
            txhpx,
            txwpx,
            p_angle - 180 as core::ffi::c_int,
            intra_edge_filter_type,
        );
        if need_left != 0 && upsample_left != 0 {
            let n_px_2: core::ffi::c_int = txhpx
                + (if need_bottom != 0 { txwpx } else { 0 as core::ffi::c_int });
            av1_highbd_upsample_intra_edge
                .expect("non-null function pointer")(left_col, n_px_2, bit_depth);
        }
    }
    highbd_dr_predictor(
        dst,
        dst_stride as ptrdiff_t,
        tx_size,
        above_row,
        left_col,
        upsample_above,
        upsample_left,
        p_angle,
        bit_depth,
    );
}
unsafe extern "C-unwind" fn highbd_build_non_directional_intra_predictors(
    mut ref8: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut dst8: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut mode: PREDICTION_MODE,
    mut tx_size: TX_SIZE,
    mut n_top_px: core::ffi::c_int,
    mut n_left_px: core::ffi::c_int,
    mut bit_depth: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dst: *mut uint16_t = ((dst8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let ref_0: *const uint16_t = ((ref8 as uintptr_t) << 1 as core::ffi::c_int)
        as *mut uint16_t;
    let txwpx: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let txhpx: core::ffi::c_int = tx_size_high[tx_size as usize];
    let mut need_left: core::ffi::c_int = extend_modes[mode as usize] as core::ffi::c_int
        & NEED_LEFT as core::ffi::c_int;
    let mut need_above: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVE as core::ffi::c_int;
    let mut need_above_left: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVELEFT as core::ffi::c_int;
    let mut above_ref: *const uint16_t = ref_0.offset(-(ref_stride as isize));
    let mut left_ref: *const uint16_t = ref_0.offset(-(1 as core::ffi::c_int as isize));
    let base: core::ffi::c_int = (128 as core::ffi::c_int)
        << bit_depth - 8 as core::ffi::c_int;
    if !(n_top_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"highbd_build_non_directional_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1570 as core::ffi::c_int,
            b"n_top_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(n_left_px >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"highbd_build_non_directional_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1571 as core::ffi::c_int,
            b"n_left_px >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(mode as core::ffi::c_int == DC_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_V_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == SMOOTH_H_PRED as core::ffi::c_int
        || mode as core::ffi::c_int == PAETH_PRED as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"highbd_build_non_directional_intra_predictors\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1573 as core::ffi::c_int,
            b"mode == DC_PRED || mode == SMOOTH_PRED || mode == SMOOTH_V_PRED || mode == SMOOTH_H_PRED || mode == PAETH_PRED\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if need_above == 0 && n_left_px == 0 as core::ffi::c_int
        || need_left == 0 && n_top_px == 0 as core::ffi::c_int
    {
        let mut val: core::ffi::c_int = 0 as core::ffi::c_int;
        if need_left != 0 {
            val = if n_top_px > 0 as core::ffi::c_int {
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                base + 1 as core::ffi::c_int
            };
        } else {
            val = if n_left_px > 0 as core::ffi::c_int {
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            } else {
                base - 1 as core::ffi::c_int
            };
        }
        i = 0 as core::ffi::c_int;
        while i < txhpx {
            aom_memset16(dst as *mut core::ffi::c_void, val, txwpx as size_t);
            dst = dst.offset(dst_stride as isize);
            i += 1;
        }
        return;
    }
    let mut left_data: crate::platform::Aligned16<[uint16_t; 160]> = crate::platform::Aligned16([0; 160]);
    let mut above_data: crate::platform::Aligned16<[uint16_t; 160]> = crate::platform::Aligned16([0; 160]);
    let above_row: *mut uint16_t = above_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    let left_col: *mut uint16_t = left_data.0
        .as_mut_ptr()
        .offset(16 as core::ffi::c_int as isize);
    if need_left != 0 {
        aom_memset16(
            left_data.0.as_mut_ptr() as *mut core::ffi::c_void,
            base + 1 as core::ffi::c_int,
            NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
        );
        if n_left_px > 0 as core::ffi::c_int {
            i = 0 as core::ffi::c_int;
            while i < n_left_px {
                *left_col.offset(i as isize) = *left_ref
                    .offset((i * ref_stride) as isize);
                i += 1;
            }
            if i < txhpx {
                aom_memset16(
                    &raw mut *left_col.offset(i as isize) as *mut uint16_t
                        as *mut core::ffi::c_void,
                    *left_col.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (txhpx - i) as size_t,
                );
            }
        } else if n_top_px > 0 as core::ffi::c_int {
            aom_memset16(
                left_col as *mut core::ffi::c_void,
                *above_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                txhpx as size_t,
            );
        }
    }
    if need_above != 0 {
        aom_memset16(
            above_data.0.as_mut_ptr() as *mut core::ffi::c_void,
            base - 1 as core::ffi::c_int,
            NUM_INTRA_NEIGHBOUR_PIXELS as size_t,
        );
        if n_top_px > 0 as core::ffi::c_int {
            memcpy(
                above_row as *mut core::ffi::c_void,
                above_ref as *const core::ffi::c_void,
                (n_top_px as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            i = n_top_px;
            if i < txwpx {
                aom_memset16(
                    &raw mut *above_row.offset(i as isize) as *mut uint16_t
                        as *mut core::ffi::c_void,
                    *above_row.offset((i - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (txwpx - i) as size_t,
                );
            }
        } else if n_left_px > 0 as core::ffi::c_int {
            aom_memset16(
                above_row as *mut core::ffi::c_void,
                *left_ref.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                txwpx as size_t,
            );
        }
    }
    if need_above_left != 0 {
        if n_top_px > 0 as core::ffi::c_int && n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(-(1 as core::ffi::c_int) as isize);
        } else if n_top_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *above_ref
                .offset(0 as core::ffi::c_int as isize);
        } else if n_left_px > 0 as core::ffi::c_int {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = *left_ref
                .offset(0 as core::ffi::c_int as isize);
        } else {
            *above_row.offset(-(1 as core::ffi::c_int) as isize) = base as uint16_t;
        }
        *left_col.offset(-(1 as core::ffi::c_int) as isize) = *above_row
            .offset(-(1 as core::ffi::c_int) as isize);
    }
    if mode as core::ffi::c_int == DC_PRED as core::ffi::c_int {
        (dc_pred_high[(n_left_px > 0 as core::ffi::c_int) as core::ffi::c_int
            as usize][(n_top_px > 0 as core::ffi::c_int) as core::ffi::c_int
            as usize][tx_size as usize])
            .expect(
                "non-null function pointer",
            )(dst, dst_stride as ptrdiff_t, above_row, left_col, bit_depth);
    } else {
        (pred_high[mode as usize][tx_size as usize])
            .expect(
                "non-null function pointer",
            )(dst, dst_stride as ptrdiff_t, above_row, left_col, bit_depth);
    };
}
#[inline]
unsafe extern "C-unwind" fn scale_chroma_bsize(
    mut bsize: BLOCK_SIZE,
    mut subsampling_x: core::ffi::c_int,
    mut subsampling_y: core::ffi::c_int,
) -> BLOCK_SIZE {
    if !(subsampling_x >= 0 as core::ffi::c_int && subsampling_x < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"scale_chroma_bsize\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1639 as core::ffi::c_int,
            b"subsampling_x >= 0 && subsampling_x < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(subsampling_y >= 0 as core::ffi::c_int && subsampling_y < 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"scale_chroma_bsize\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1640 as core::ffi::c_int,
            b"subsampling_y >= 0 && subsampling_y < 2\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut bs: BLOCK_SIZE = bsize;
    match bsize as core::ffi::c_int {
        0 => {
            if subsampling_x == 1 as core::ffi::c_int
                && subsampling_y == 1 as core::ffi::c_int
            {
                bs = BLOCK_8X8;
            } else if subsampling_x == 1 as core::ffi::c_int {
                bs = BLOCK_8X4;
            } else if subsampling_y == 1 as core::ffi::c_int {
                bs = BLOCK_4X8;
            }
        }
        1 => {
            if subsampling_x == 1 as core::ffi::c_int
                && subsampling_y == 1 as core::ffi::c_int
            {
                bs = BLOCK_8X8;
            } else if subsampling_x == 1 as core::ffi::c_int {
                bs = BLOCK_8X8;
            } else if subsampling_y == 1 as core::ffi::c_int {
                bs = BLOCK_4X8;
            }
        }
        2 => {
            if subsampling_x == 1 as core::ffi::c_int
                && subsampling_y == 1 as core::ffi::c_int
            {
                bs = BLOCK_8X8;
            } else if subsampling_x == 1 as core::ffi::c_int {
                bs = BLOCK_8X4;
            } else if subsampling_y == 1 as core::ffi::c_int {
                bs = BLOCK_8X8;
            }
        }
        16 => {
            if subsampling_x == 1 as core::ffi::c_int
                && subsampling_y == 1 as core::ffi::c_int
            {
                bs = BLOCK_8X16;
            } else if subsampling_x == 1 as core::ffi::c_int {
                bs = BLOCK_8X16;
            } else if subsampling_y == 1 as core::ffi::c_int {
                bs = BLOCK_4X16;
            }
        }
        17 => {
            if subsampling_x == 1 as core::ffi::c_int
                && subsampling_y == 1 as core::ffi::c_int
            {
                bs = BLOCK_16X8;
            } else if subsampling_x == 1 as core::ffi::c_int {
                bs = BLOCK_16X4;
            } else if subsampling_y == 1 as core::ffi::c_int {
                bs = BLOCK_16X8;
            }
        }
        _ => {}
    }
    return bs;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_predict_intra_block(
    mut xd: *const MACROBLOCKD,
    mut sb_size: BLOCK_SIZE,
    mut enable_intra_edge_filter: core::ffi::c_int,
    mut wpx: core::ffi::c_int,
    mut hpx: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut mode: PREDICTION_MODE,
    mut angle_delta: core::ffi::c_int,
    mut use_palette: core::ffi::c_int,
    mut filter_intra_mode: FILTER_INTRA_MODE,
    mut ref_0: *const uint8_t,
    mut ref_stride: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut col_off: core::ffi::c_int,
    mut row_off: core::ffi::c_int,
    mut plane: core::ffi::c_int,
) {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let txwpx: core::ffi::c_int = tx_size_wide[tx_size as usize];
    let txhpx: core::ffi::c_int = tx_size_high[tx_size as usize];
    let x: core::ffi::c_int = col_off << MI_SIZE_LOG2;
    let y: core::ffi::c_int = row_off << MI_SIZE_LOG2;
    let is_hbd: core::ffi::c_int = is_cur_buf_hbd(xd) as core::ffi::c_int;
    if !((mode as core::ffi::c_int) < INTRA_MODES as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"av1_predict_intra_block\0"))
                .as_ptr(),
            b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
            1703 as core::ffi::c_int,
            b"mode < INTRA_MODES\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if use_palette != 0 {
        let mut r: core::ffi::c_int = 0;
        let mut c: core::ffi::c_int = 0;
        let map: *const uint8_t = ((*xd)
            .plane[(plane != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
            .color_index_map)
            .offset(
                (*xd)
                    .color_index_map_offset[(plane != 0 as core::ffi::c_int)
                    as core::ffi::c_int as usize] as core::ffi::c_int as isize,
            );
        let palette: *const uint16_t = ((*mbmi).palette_mode_info.palette_colors)
            .as_ptr()
            .offset((plane * PALETTE_MAX_SIZE) as isize);
        if is_hbd != 0 {
            let mut dst16: *mut uint16_t = ((dst as uintptr_t) << 1 as core::ffi::c_int)
                as *mut uint16_t;
            r = 0 as core::ffi::c_int;
            while r < txhpx {
                c = 0 as core::ffi::c_int;
                while c < txwpx {
                    *dst16.offset((r * dst_stride + c) as isize) = *palette
                        .offset(*map.offset(((r + y) * wpx + c + x) as isize) as isize);
                    c += 1;
                }
                r += 1;
            }
        } else {
            r = 0 as core::ffi::c_int;
            while r < txhpx {
                c = 0 as core::ffi::c_int;
                while c < txwpx {
                    *dst.offset((r * dst_stride + c) as isize) = *palette
                        .offset(*map.offset(((r + y) * wpx + c + x) as isize) as isize)
                        as uint8_t;
                    c += 1;
                }
                r += 1;
            }
        }
        return;
    }
    let pd: *const macroblockd_plane = &*((*xd).plane).as_ptr().offset(plane as isize)
        as *const macroblockd_plane;
    let ss_x: core::ffi::c_int = (*pd).subsampling_x;
    let ss_y: core::ffi::c_int = (*pd).subsampling_y;
    let have_top: core::ffi::c_int = (row_off != 0
        || (if ss_y != 0 {
            (*xd).chroma_up_available as core::ffi::c_int
        } else {
            (*xd).up_available as core::ffi::c_int
        }) != 0) as core::ffi::c_int;
    let have_left: core::ffi::c_int = (col_off != 0
        || (if ss_x != 0 {
            (*xd).chroma_left_available as core::ffi::c_int
        } else {
            (*xd).left_available as core::ffi::c_int
        }) != 0) as core::ffi::c_int;
    let xr: core::ffi::c_int = ((*xd).mb_to_right_edge >> 3 as core::ffi::c_int + ss_x)
        + wpx - x - txwpx;
    let yd: core::ffi::c_int = ((*xd).mb_to_bottom_edge >> 3 as core::ffi::c_int + ss_y)
        + hpx - y - txhpx;
    let use_filter_intra: core::ffi::c_int = (filter_intra_mode as core::ffi::c_int
        != FILTER_INTRA_MODES as core::ffi::c_int) as core::ffi::c_int;
    let is_dr_mode: core::ffi::c_int = av1_is_directional_mode(mode) as core::ffi::c_int;
    let n_top_px: core::ffi::c_int = if have_top != 0 {
        if txwpx < xr + txwpx { txwpx } else { xr + txwpx }
    } else {
        0 as core::ffi::c_int
    };
    let n_left_px: core::ffi::c_int = if have_left != 0 {
        if txhpx < yd + txhpx { txhpx } else { yd + txhpx }
    } else {
        0 as core::ffi::c_int
    };
    if use_filter_intra == 0 && is_dr_mode == 0 {
        if is_hbd != 0 {
            highbd_build_non_directional_intra_predictors(
                ref_0,
                ref_stride,
                dst,
                dst_stride,
                mode,
                tx_size,
                n_top_px,
                n_left_px,
                (*xd).bd,
            );
            return;
        }
        build_non_directional_intra_predictors(
            ref_0,
            ref_stride,
            dst,
            dst_stride,
            mode,
            tx_size,
            n_top_px,
            n_left_px,
        );
        return;
    }
    let txw: core::ffi::c_int = tx_size_wide_unit[tx_size as usize];
    let txh: core::ffi::c_int = tx_size_high_unit[tx_size as usize];
    let mi_row: core::ffi::c_int = -(*xd).mb_to_top_edge
        >> 3 as core::ffi::c_int + MI_SIZE_LOG2;
    let mi_col: core::ffi::c_int = -(*xd).mb_to_left_edge
        >> 3 as core::ffi::c_int + MI_SIZE_LOG2;
    let right_available: core::ffi::c_int = (mi_col + (col_off + txw << ss_x)
        < (*xd).tile.mi_col_end) as core::ffi::c_int;
    let bottom_available: core::ffi::c_int = (yd > 0 as core::ffi::c_int
        && mi_row + (row_off + txh << ss_y) < (*xd).tile.mi_row_end) as core::ffi::c_int;
    let partition: PARTITION_TYPE = (*mbmi).partition;
    let mut bsize: BLOCK_SIZE = (*mbmi).bsize;
    if ss_x != 0 || ss_y != 0 {
        bsize = scale_chroma_bsize(bsize, ss_x, ss_y);
    }
    let mut p_angle: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut need_top_right: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_ABOVERIGHT as core::ffi::c_int;
    let mut need_bottom_left: core::ffi::c_int = extend_modes[mode as usize]
        as core::ffi::c_int & NEED_BOTTOMLEFT as core::ffi::c_int;
    if use_filter_intra != 0 {
        need_top_right = 0 as core::ffi::c_int;
        need_bottom_left = 0 as core::ffi::c_int;
    }
    if is_dr_mode != 0 {
        p_angle = mode_to_angle_map[mode as usize] as core::ffi::c_int + angle_delta;
        need_top_right = (p_angle < 90 as core::ffi::c_int) as core::ffi::c_int;
        need_bottom_left = (p_angle > 180 as core::ffi::c_int) as core::ffi::c_int;
    }
    let have_top_right: core::ffi::c_int = if need_top_right != 0 {
        has_top_right(
            sb_size,
            bsize,
            mi_row,
            mi_col,
            have_top,
            right_available,
            partition,
            tx_size,
            row_off,
            col_off,
            ss_x,
            ss_y,
        ) as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
    let have_bottom_left: core::ffi::c_int = if need_bottom_left != 0 {
        has_bottom_left(
            sb_size,
            bsize,
            mi_row,
            mi_col,
            bottom_available,
            have_left,
            partition,
            tx_size,
            row_off,
            col_off,
            ss_x,
            ss_y,
        ) as core::ffi::c_int
    } else {
        -(1 as core::ffi::c_int)
    };
    let disable_edge_filter: core::ffi::c_int = (enable_intra_edge_filter == 0)
        as core::ffi::c_int;
    let intra_edge_filter_type: core::ffi::c_int = get_intra_edge_filter_type(xd, plane)
        as core::ffi::c_int;
    let n_topright_px: core::ffi::c_int = if have_top_right > 0 as core::ffi::c_int {
        if txwpx < xr { txwpx } else { xr }
    } else {
        have_top_right
    };
    let n_bottomleft_px: core::ffi::c_int = if have_bottom_left > 0 as core::ffi::c_int {
        if txhpx < yd { txhpx } else { yd }
    } else {
        have_bottom_left
    };
    if is_hbd != 0 {
        highbd_build_directional_and_filter_intra_predictors(
            ref_0,
            ref_stride,
            dst,
            dst_stride,
            mode,
            p_angle,
            filter_intra_mode,
            tx_size,
            disable_edge_filter,
            n_top_px,
            n_topright_px,
            n_left_px,
            n_bottomleft_px,
            intra_edge_filter_type,
            (*xd).bd,
        );
        return;
    }
    build_directional_and_filter_intra_predictors(
        ref_0,
        ref_stride,
        dst,
        dst_stride,
        mode,
        p_angle,
        filter_intra_mode,
        tx_size,
        disable_edge_filter,
        n_top_px,
        n_topright_px,
        n_left_px,
        n_bottomleft_px,
        intra_edge_filter_type,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_predict_intra_block_facade(
    mut cm: *const AV1_COMMON,
    mut xd: *mut MACROBLOCKD,
    mut plane: core::ffi::c_int,
    mut blk_col: core::ffi::c_int,
    mut blk_row: core::ffi::c_int,
    mut tx_size: TX_SIZE,
) {
    let mbmi: *const MB_MODE_INFO = *((*xd).mi).offset(0 as core::ffi::c_int as isize);
    let pd: *mut macroblockd_plane = &raw mut *((*xd).plane)
        .as_mut_ptr()
        .offset(plane as isize) as *mut macroblockd_plane;
    let dst_stride: core::ffi::c_int = (*pd).dst.stride;
    let mut dst: *mut uint8_t = &raw mut *((*pd).dst.buf)
        .offset((blk_row * dst_stride + blk_col << MI_SIZE_LOG2) as isize)
        as *mut uint8_t;
    let mode: PREDICTION_MODE = (if plane == AOM_PLANE_Y {
        (*mbmi).mode as core::ffi::c_int
    } else {
        get_uv_mode((*mbmi).uv_mode) as core::ffi::c_int
    }) as PREDICTION_MODE;
    let use_palette: core::ffi::c_int = ((*mbmi)
        .palette_mode_info
        .palette_size[(plane != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
        as core::ffi::c_int > 0 as core::ffi::c_int) as core::ffi::c_int;
    let filter_intra_mode: FILTER_INTRA_MODE = (if plane == AOM_PLANE_Y
        && (*mbmi).filter_intra_mode_info.use_filter_intra as core::ffi::c_int != 0
    {
        (*mbmi).filter_intra_mode_info.filter_intra_mode as core::ffi::c_int
    } else {
        FILTER_INTRA_MODES as core::ffi::c_int
    }) as FILTER_INTRA_MODE;
    let angle_delta: core::ffi::c_int = (*mbmi)
        .angle_delta[(plane != AOM_PLANE_Y) as core::ffi::c_int as usize]
        as core::ffi::c_int * ANGLE_STEP;
    let mut seq_params: *const SequenceHeader = (*cm).seq_params;
    if plane != AOM_PLANE_Y
        && (*mbmi).uv_mode as core::ffi::c_int == UV_CFL_PRED as core::ffi::c_int
    {
        if (is_cfl_allowed(xd) == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 31],
                    [core::ffi::c_char; 31],
                >(*b"av1_predict_intra_block_facade\0"))
                    .as_ptr(),
                b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                1856 as core::ffi::c_int,
                b"is_cfl_allowed(xd)\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let plane_bsize: BLOCK_SIZE = get_plane_block_size(
            (*mbmi).bsize,
            (*pd).subsampling_x,
            (*pd).subsampling_y,
        ) as BLOCK_SIZE;
        if !((plane_bsize as core::ffi::c_int) < BLOCK_SIZES_ALL as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 31],
                    [core::ffi::c_char; 31],
                >(*b"av1_predict_intra_block_facade\0"))
                    .as_ptr(),
                b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                1860 as core::ffi::c_int,
                b"plane_bsize < BLOCK_SIZES_ALL\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        if (*xd).lossless[(*mbmi).segment_id() as usize] == 0 {
            if !(blk_col == 0 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_predict_intra_block_facade\0"))
                        .as_ptr(),
                    b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                    1862 as core::ffi::c_int,
                    b"blk_col == 0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !(blk_row == 0 as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_predict_intra_block_facade\0"))
                        .as_ptr(),
                    b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                    1863 as core::ffi::c_int,
                    b"blk_row == 0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !(block_size_wide[plane_bsize as usize] as core::ffi::c_int
                == tx_size_wide[tx_size as usize]) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_predict_intra_block_facade\0"))
                        .as_ptr(),
                    b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                    1864 as core::ffi::c_int,
                    b"block_size_wide[plane_bsize] == tx_size_wide[tx_size]\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if !(block_size_high[plane_bsize as usize] as core::ffi::c_int
                == tx_size_high[tx_size as usize]) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 31],
                        [core::ffi::c_char; 31],
                    >(*b"av1_predict_intra_block_facade\0"))
                        .as_ptr(),
                    b"reconintra.c\0" as *const u8 as *const core::ffi::c_char,
                    1865 as core::ffi::c_int,
                    b"block_size_high[plane_bsize] == tx_size_high[tx_size]\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
        let cfl: *mut CFL_CTX = &raw mut (*xd).cfl;
        let mut pred_plane: CFL_PRED_TYPE = get_cfl_pred_type(plane);
        if !(*cfl).dc_pred_is_cached[pred_plane as usize] {
            av1_predict_intra_block(
                xd,
                (*seq_params).sb_size,
                (*seq_params).enable_intra_edge_filter as core::ffi::c_int,
                (*pd).width as core::ffi::c_int,
                (*pd).height as core::ffi::c_int,
                tx_size,
                mode,
                angle_delta,
                use_palette,
                filter_intra_mode,
                dst,
                dst_stride,
                dst,
                dst_stride,
                blk_col,
                blk_row,
                plane,
            );
            if (*cfl).use_dc_pred_cache {
                cfl_store_dc_pred(xd, dst, pred_plane, tx_size_wide[tx_size as usize]);
                (*cfl).dc_pred_is_cached[pred_plane as usize] = true_0 != 0;
            }
        } else {
            cfl_load_dc_pred(xd, dst, dst_stride, tx_size, pred_plane);
        }
        av1_cfl_predict_block(xd, dst, dst_stride, tx_size, plane);
        return;
    }
    av1_predict_intra_block(
        xd,
        (*seq_params).sb_size,
        (*seq_params).enable_intra_edge_filter as core::ffi::c_int,
        (*pd).width as core::ffi::c_int,
        (*pd).height as core::ffi::c_int,
        tx_size,
        mode,
        angle_delta,
        use_palette,
        filter_intra_mode,
        dst,
        dst_stride,
        dst,
        dst_stride,
        blk_col,
        blk_row,
        plane,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_init_intra_predictors() {
    aom_once(Some(init_intra_predictors_internal as unsafe extern "C-unwind" fn() -> ()));
}

extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C-unwind" fn() -> ()>,
    ) -> core::ffi::c_int;
}
