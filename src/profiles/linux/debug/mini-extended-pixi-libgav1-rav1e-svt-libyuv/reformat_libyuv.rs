// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/reformat_libyuv.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type YuvConstants;
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifGetPixelFormatInfo(format: avifPixelFormat, info: *mut avifPixelFormatInfo);
    fn avifImageAllocatePlanes(
        image: *mut avifImage,
        planes: avifPlanesFlags,
    ) -> avifResult;
    fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
    fn avifRGBFormatChannelCount(format: avifRGBFormat) -> uint32_t;
    fn avifImagePlane(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> *mut uint8_t;
    fn avifImagePlaneRowBytes(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneWidth(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneHeight(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImageSetDefaults(image: *mut avifImage);
    fn avifImageCopyNoAlloc(dstImage: *mut avifImage, srcImage: *const avifImage);
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static kYuvI601Constants: YuvConstants;
    static kYuvJPEGConstants: YuvConstants;
    static kYuvH709Constants: YuvConstants;
    static kYuvF709Constants: YuvConstants;
    static kYuv2020Constants: YuvConstants;
    static kYuvV2020Constants: YuvConstants;
    static kYvuI601Constants: YuvConstants;
    static kYvuJPEGConstants: YuvConstants;
    static kYvuH709Constants: YuvConstants;
    static kYvuF709Constants: YuvConstants;
    static kYvu2020Constants: YuvConstants;
    static kYvuV2020Constants: YuvConstants;
    fn BGRAToARGB(
        src_bgra: *const uint8_t,
        src_stride_bgra: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ABGRToARGB(
        src_abgr: *const uint8_t,
        src_stride_abgr: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGBAToARGB(
        src_rgba: *const uint8_t,
        src_stride_rgba: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGB24ToARGB(
        src_rgb24: *const uint8_t,
        src_stride_rgb24: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RAWToARGB(
        src_raw: *const uint8_t,
        src_stride_raw: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I420ToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I422ToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I444ToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I444ToRGB24Matrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgb24: *mut uint8_t,
        dst_stride_rgb24: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I010ToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I012ToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I210ToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I410ToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I420AlphaToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint8_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I422AlphaToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint8_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I444AlphaToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint8_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I010AlphaToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint16_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I210AlphaToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint16_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I410AlphaToARGBMatrix(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint16_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I422ToRGBAMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgba: *mut uint8_t,
        dst_stride_rgba: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I420ToRGBAMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgba: *mut uint8_t,
        dst_stride_rgba: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I420ToRGB24Matrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgb24: *mut uint8_t,
        dst_stride_rgb24: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I420ToRGB565Matrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgb565: *mut uint8_t,
        dst_stride_rgb565: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I422ToRGB565Matrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgb565: *mut uint8_t,
        dst_stride_rgb565: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I400ToARGBMatrix(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn I420ToARGBMatrixFilter(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I422ToARGBMatrixFilter(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I422ToRGB24MatrixFilter(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgb24: *mut uint8_t,
        dst_stride_rgb24: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I420ToRGB24MatrixFilter(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        dst_rgb24: *mut uint8_t,
        dst_stride_rgb24: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I010ToARGBMatrixFilter(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I210ToARGBMatrixFilter(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I420AlphaToARGBMatrixFilter(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint8_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I422AlphaToARGBMatrixFilter(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint8_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint8_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint8_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I010AlphaToARGBMatrixFilter(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint16_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn I210AlphaToARGBMatrixFilter(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        src_u: *const uint16_t,
        src_stride_u: core::ffi::c_int,
        src_v: *const uint16_t,
        src_stride_v: core::ffi::c_int,
        src_a: *const uint16_t,
        src_stride_a: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        yuvconstants: *const YuvConstants,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
        attenuate: core::ffi::c_int,
        filter: FilterMode,
    ) -> core::ffi::c_int;
    fn Convert16To8Plane(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        scale: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn HalfFloatPlane(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        dst_y: *mut uint16_t,
        dst_stride_y: core::ffi::c_int,
        scale: core::ffi::c_float,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBAttenuate(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBUnattenuate(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_argb: *mut uint8_t,
        dst_stride_argb: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToI420(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn BGRAToI420(
        src_bgra: *const uint8_t,
        src_stride_bgra: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ABGRToI420(
        src_abgr: *const uint8_t,
        src_stride_abgr: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGBAToI420(
        src_rgba: *const uint8_t,
        src_stride_rgba: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGB24ToI420(
        src_rgb24: *const uint8_t,
        src_stride_rgb24: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGB24ToJ420(
        src_rgb24: *const uint8_t,
        src_stride_rgb24: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RAWToI420(
        src_raw: *const uint8_t,
        src_stride_raw: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RAWToI444(
        src_raw: *const uint8_t,
        src_stride_raw: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RAWToJ420(
        src_raw: *const uint8_t,
        src_stride_raw: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RAWToJ444(
        src_raw: *const uint8_t,
        src_stride_raw: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGB24ToJ400(
        src_rgb24: *const uint8_t,
        src_stride_rgb24: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RAWToJ400(
        src_raw: *const uint8_t,
        src_stride_raw: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToI444(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToI422(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        dst_u: *mut uint8_t,
        dst_stride_u: core::ffi::c_int,
        dst_v: *mut uint8_t,
        dst_stride_v: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToJ420(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        dst_uj: *mut uint8_t,
        dst_stride_uj: core::ffi::c_int,
        dst_vj: *mut uint8_t,
        dst_stride_vj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToJ422(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        dst_uj: *mut uint8_t,
        dst_stride_uj: core::ffi::c_int,
        dst_vj: *mut uint8_t,
        dst_stride_vj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToJ400(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ABGRToJ420(
        src_abgr: *const uint8_t,
        src_stride_abgr: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        dst_uj: *mut uint8_t,
        dst_stride_uj: core::ffi::c_int,
        dst_vj: *mut uint8_t,
        dst_stride_vj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ABGRToJ422(
        src_abgr: *const uint8_t,
        src_stride_abgr: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        dst_uj: *mut uint8_t,
        dst_stride_uj: core::ffi::c_int,
        dst_vj: *mut uint8_t,
        dst_stride_vj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ABGRToJ400(
        src_abgr: *const uint8_t,
        src_stride_abgr: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn RGBAToJ400(
        src_rgba: *const uint8_t,
        src_stride_rgba: core::ffi::c_int,
        dst_yj: *mut uint8_t,
        dst_stride_yj: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ARGBToI400(
        src_argb: *const uint8_t,
        src_stride_argb: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type avifBool = core::ffi::c_int;
pub type avifPlanesFlag = core::ffi::c_uint;
pub const AVIF_PLANES_ALL: avifPlanesFlag = 255;
pub const AVIF_PLANES_A: avifPlanesFlag = 2;
pub const AVIF_PLANES_YUV: avifPlanesFlag = 1;
pub type avifPlanesFlags = uint32_t;
pub type avifChannelIndex = core::ffi::c_uint;
pub const AVIF_CHAN_A: avifChannelIndex = 3;
pub const AVIF_CHAN_V: avifChannelIndex = 2;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_Y: avifChannelIndex = 0;
pub type avifResult = core::ffi::c_uint;
pub const AVIF_RESULT_NO_AV1_ITEMS_FOUND: avifResult = 10;
pub const AVIF_RESULT_DECODE_SAMPLE_TRANSFORM_FAILED: avifResult = 34;
pub const AVIF_RESULT_ENCODE_SAMPLE_TRANSFORM_FAILED: avifResult = 33;
pub const AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE: avifResult = 32;
pub const AVIF_RESULT_DECODE_GAIN_MAP_FAILED: avifResult = 31;
pub const AVIF_RESULT_ENCODE_GAIN_MAP_FAILED: avifResult = 30;
pub const AVIF_RESULT_INTERNAL_ERROR: avifResult = 29;
pub const AVIF_RESULT_INCOMPATIBLE_IMAGE: avifResult = 28;
pub const AVIF_RESULT_CANNOT_CHANGE_SETTING: avifResult = 27;
pub const AVIF_RESULT_OUT_OF_MEMORY: avifResult = 26;
pub const AVIF_RESULT_NOT_IMPLEMENTED: avifResult = 25;
pub const AVIF_RESULT_INVALID_ARGUMENT: avifResult = 24;
pub const AVIF_RESULT_WAITING_ON_IO: avifResult = 23;
pub const AVIF_RESULT_IO_ERROR: avifResult = 22;
pub const AVIF_RESULT_IO_NOT_SET: avifResult = 21;
pub const AVIF_RESULT_TRUNCATED_DATA: avifResult = 20;
pub const AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION: avifResult = 19;
pub const AVIF_RESULT_INVALID_IMAGE_GRID: avifResult = 18;
pub const AVIF_RESULT_INVALID_EXIF_PAYLOAD: avifResult = 17;
pub const AVIF_RESULT_NO_IMAGES_REMAINING: avifResult = 16;
pub const AVIF_RESULT_NO_CODEC_AVAILABLE: avifResult = 15;
pub const AVIF_RESULT_ISPE_SIZE_MISMATCH: avifResult = 14;
pub const AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH: avifResult = 13;
pub const AVIF_RESULT_DECODE_ALPHA_FAILED: avifResult = 12;
pub const AVIF_RESULT_DECODE_COLOR_FAILED: avifResult = 11;
pub const AVIF_RESULT_MISSING_IMAGE_ITEM: avifResult = 10;
pub const AVIF_RESULT_BMFF_PARSE_FAILED: avifResult = 9;
pub const AVIF_RESULT_ENCODE_ALPHA_FAILED: avifResult = 8;
pub const AVIF_RESULT_ENCODE_COLOR_FAILED: avifResult = 7;
pub const AVIF_RESULT_UNSUPPORTED_DEPTH: avifResult = 6;
pub const AVIF_RESULT_REFORMAT_FAILED: avifResult = 5;
pub const AVIF_RESULT_NO_YUV_FORMAT_SELECTED: avifResult = 4;
pub const AVIF_RESULT_NO_CONTENT: avifResult = 3;
pub const AVIF_RESULT_INVALID_FTYP: avifResult = 2;
pub const AVIF_RESULT_UNKNOWN_ERROR: avifResult = 1;
pub const AVIF_RESULT_OK: avifResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRWData {
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type avifPixelFormat = core::ffi::c_uint;
pub const AVIF_PIXEL_FORMAT_COUNT: avifPixelFormat = 5;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelFormatInfo {
    pub monochrome: avifBool,
    pub chromaShiftX: core::ffi::c_int,
    pub chromaShiftY: core::ffi::c_int,
}
pub type avifChromaSamplePosition = core::ffi::c_uint;
pub const AVIF_CHROMA_SAMPLE_POSITION_RESERVED: avifChromaSamplePosition = 3;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub type avifRange = core::ffi::c_uint;
pub const AVIF_RANGE_FULL: avifRange = 1;
pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AVIF_COLOR_PRIMARIES_EBU3213: C2RustUnnamed = 22;
pub const AVIF_COLOR_PRIMARIES_DCI_P3: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: C2RustUnnamed = 11;
pub const AVIF_COLOR_PRIMARIES_XYZ: C2RustUnnamed = 10;
pub const AVIF_COLOR_PRIMARIES_BT2100: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_BT2020: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: C2RustUnnamed = 8;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: C2RustUnnamed = 7;
pub const AVIF_COLOR_PRIMARIES_BT601: C2RustUnnamed = 6;
pub const AVIF_COLOR_PRIMARIES_BT470BG: C2RustUnnamed = 5;
pub const AVIF_COLOR_PRIMARIES_BT470M: C2RustUnnamed = 4;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: C2RustUnnamed = 2;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_SRGB: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_BT709: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_UNKNOWN: C2RustUnnamed = 0;
pub type avifColorPrimaries = uint16_t;
pub type avifTransferCharacteristics = uint16_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AVIF_MATRIX_COEFFICIENTS_LAST: C2RustUnnamed_0 = 18;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RO: C2RustUnnamed_0 = 17;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RE: C2RustUnnamed_0 = 16;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: C2RustUnnamed_0 = 14;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: C2RustUnnamed_0 = 13;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: C2RustUnnamed_0 = 12;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: C2RustUnnamed_0 = 11;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: C2RustUnnamed_0 = 10;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: C2RustUnnamed_0 = 9;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: C2RustUnnamed_0 = 8;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: C2RustUnnamed_0 = 7;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: C2RustUnnamed_0 = 6;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: C2RustUnnamed_0 = 5;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: C2RustUnnamed_0 = 4;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: C2RustUnnamed_0 = 2;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: C2RustUnnamed_0 = 1;
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: C2RustUnnamed_0 = 0;
pub type avifMatrixCoefficients = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSignedFraction {
    pub n: int32_t,
    pub d: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifUnsignedFraction {
    pub n: uint32_t,
    pub d: uint32_t,
}
pub type avifTransformFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: uint32_t,
    pub vSpacing: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCleanApertureBox {
    pub widthN: uint32_t,
    pub widthD: uint32_t,
    pub heightN: uint32_t,
    pub heightD: uint32_t,
    pub horizOffN: uint32_t,
    pub horizOffD: uint32_t,
    pub vertOffN: uint32_t,
    pub vertOffD: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageRotation {
    pub angle: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageMirror {
    pub axis: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifContentLightLevelInformationBox {
    pub maxCLL: uint16_t,
    pub maxPALL: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImage {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t,
    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvChromaSamplePosition: avifChromaSamplePosition,
    pub yuvPlanes: [*mut uint8_t; 3],
    pub yuvRowBytes: [uint32_t; 3],
    pub imageOwnsYUVPlanes: avifBool,
    pub alphaPlane: *mut uint8_t,
    pub alphaRowBytes: uint32_t,
    pub imageOwnsAlphaPlane: avifBool,
    pub alphaPremultiplied: avifBool,
    pub icc: avifRWData,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub clli: avifContentLightLevelInformationBox,
    pub transformFlags: avifTransformFlags,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,
    pub exif: avifRWData,
    pub xmp: avifRWData,
    pub properties: *mut avifImageItemProperty,
    pub numProperties: size_t,
    pub gainMap: *mut avifGainMap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifGainMap {
    pub image: *mut avifImage,
    pub gainMapMin: [avifSignedFraction; 3],
    pub gainMapMax: [avifSignedFraction; 3],
    pub gainMapGamma: [avifUnsignedFraction; 3],
    pub baseOffset: [avifSignedFraction; 3],
    pub alternateOffset: [avifSignedFraction; 3],
    pub baseHdrHeadroom: avifUnsignedFraction,
    pub alternateHdrHeadroom: avifUnsignedFraction,
    pub useBaseColorSpace: avifBool,
    pub altICC: avifRWData,
    pub altColorPrimaries: avifColorPrimaries,
    pub altTransferCharacteristics: avifTransferCharacteristics,
    pub altMatrixCoefficients: avifMatrixCoefficients,
    pub altYUVRange: avifRange,
    pub altDepth: uint32_t,
    pub altPlaneCount: uint32_t,
    pub altCLLI: avifContentLightLevelInformationBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageItemProperty {
    pub boxtype: [uint8_t; 4],
    pub usertype: [uint8_t; 16],
    pub boxPayload: avifRWData,
}
pub type avifRGBFormat = core::ffi::c_uint;
pub const AVIF_RGB_FORMAT_COUNT: avifRGBFormat = 10;
pub const AVIF_RGB_FORMAT_AGRAY: avifRGBFormat = 9;
pub const AVIF_RGB_FORMAT_GRAYA: avifRGBFormat = 8;
pub const AVIF_RGB_FORMAT_GRAY: avifRGBFormat = 7;
pub const AVIF_RGB_FORMAT_RGB_565: avifRGBFormat = 6;
pub const AVIF_RGB_FORMAT_ABGR: avifRGBFormat = 5;
pub const AVIF_RGB_FORMAT_BGRA: avifRGBFormat = 4;
pub const AVIF_RGB_FORMAT_BGR: avifRGBFormat = 3;
pub const AVIF_RGB_FORMAT_ARGB: avifRGBFormat = 2;
pub const AVIF_RGB_FORMAT_RGBA: avifRGBFormat = 1;
pub const AVIF_RGB_FORMAT_RGB: avifRGBFormat = 0;
pub type avifChromaUpsampling = core::ffi::c_uint;
pub const AVIF_CHROMA_UPSAMPLING_BILINEAR: avifChromaUpsampling = 4;
pub const AVIF_CHROMA_UPSAMPLING_NEAREST: avifChromaUpsampling = 3;
pub const AVIF_CHROMA_UPSAMPLING_BEST_QUALITY: avifChromaUpsampling = 2;
pub const AVIF_CHROMA_UPSAMPLING_FASTEST: avifChromaUpsampling = 1;
pub const AVIF_CHROMA_UPSAMPLING_AUTOMATIC: avifChromaUpsampling = 0;
pub type avifChromaDownsampling = core::ffi::c_uint;
pub const AVIF_CHROMA_DOWNSAMPLING_SHARP_YUV: avifChromaDownsampling = 4;
pub const AVIF_CHROMA_DOWNSAMPLING_AVERAGE: avifChromaDownsampling = 3;
pub const AVIF_CHROMA_DOWNSAMPLING_BEST_QUALITY: avifChromaDownsampling = 2;
pub const AVIF_CHROMA_DOWNSAMPLING_FASTEST: avifChromaDownsampling = 1;
pub const AVIF_CHROMA_DOWNSAMPLING_AUTOMATIC: avifChromaDownsampling = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRGBImage {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t,
    pub format: avifRGBFormat,
    pub chromaUpsampling: avifChromaUpsampling,
    pub chromaDownsampling: avifChromaDownsampling,
    pub avoidLibYUV: avifBool,
    pub ignoreAlpha: avifBool,
    pub alphaPremultiplied: avifBool,
    pub isFloat: avifBool,
    pub maxThreads: core::ffi::c_int,
    pub pixels: *mut uint8_t,
    pub rowBytes: uint32_t,
}
pub type RGBtoYUV = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub type RGBtoY = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub type YUVAToRGBMatrix = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LibyuvConversionFunction {
    pub yuv400ToRgbMatrix: YUV400ToRGBMatrix,
    pub yuvToRgbMatrixFilter: YUVToRGBMatrixFilter,
    pub yuvaToRgbMatrixFilter: YUVAToRGBMatrixFilter,
    pub yuvToRgbMatrix: YUVToRGBMatrix,
    pub yuvaToRgbMatrix: YUVAToRGBMatrix,
    pub yuvToRgbMatrixFilterHighBitDepth: YUVToRGBMatrixFilterHighBitDepth,
    pub yuvaToRgbMatrixFilterHighBitDepth: YUVAToRGBMatrixFilterHighBitDepth,
    pub yuvToRgbMatrixHighBitDepth: YUVToRGBMatrixHighBitDepth,
    pub yuvaToRgbMatrixHighBitDepth: YUVAToRGBMatrixHighBitDepth,
}
pub type YUVAToRGBMatrixHighBitDepth = Option<
    unsafe extern "C" fn(
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub type YUVToRGBMatrixHighBitDepth = Option<
    unsafe extern "C" fn(
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub type YUVAToRGBMatrixFilterHighBitDepth = Option<
    unsafe extern "C" fn(
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        FilterMode,
    ) -> core::ffi::c_int,
>;
pub type FilterMode = core::ffi::c_uint;
pub const kFilterBox: FilterMode = 3;
pub const kFilterBilinear: FilterMode = 2;
pub const kFilterLinear: FilterMode = 1;
pub const kFilterNone: FilterMode = 0;
pub type YUVToRGBMatrixFilterHighBitDepth = Option<
    unsafe extern "C" fn(
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
        FilterMode,
    ) -> core::ffi::c_int,
>;
pub type YUVToRGBMatrix = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub type YUVAToRGBMatrixFilter = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        FilterMode,
    ) -> core::ffi::c_int,
>;
pub type YUVToRGBMatrixFilter = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
        FilterMode,
    ) -> core::ffi::c_int,
>;
pub type YUV400ToRGBMatrix = Option<
    unsafe extern "C" fn(
        *const uint8_t,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
        *const YuvConstants,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> core::ffi::c_int,
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const LIBYUV_VERSION: core::ffi::c_int = 1960 as core::ffi::c_int;
unsafe extern "C" fn avifReorderARGBThenConvertToYUV(
    mut ReorderARGB: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            core::ffi::c_int,
            *mut uint8_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    mut ConvertToYUV: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            core::ffi::c_int,
            *mut uint8_t,
            core::ffi::c_int,
            *mut uint8_t,
            core::ffi::c_int,
            *mut uint8_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut dst_format: avifPixelFormat,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut format_info: avifPixelFormatInfo = avifPixelFormatInfo {
        monochrome: 0,
        chromaShiftX: 0,
        chromaShiftY: 0,
    };
    avifGetPixelFormatInfo(dst_format, &mut format_info);
    let min_num_rows: core::ffi::c_int = if format_info.chromaShiftY
        == 1 as core::ffi::c_int
    {
        2 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let mut src_argb: *mut uint8_t = 0 as *mut uint8_t;
    if width as int64_t * 4 as int64_t > INT_MAX as int64_t {
        return -(1 as core::ffi::c_int);
    }
    let src_stride_argb: core::ffi::c_int = width * 4 as core::ffi::c_int;
    let soft_allocation_limit: core::ffi::c_int = 16384 as core::ffi::c_int;
    let mut num_allocated_rows: core::ffi::c_int = 0;
    if height == 1 as core::ffi::c_int
        || src_stride_argb as int64_t * height as int64_t
            <= soft_allocation_limit as int64_t
    {
        num_allocated_rows = height;
    } else {
        if src_stride_argb as int64_t * min_num_rows as int64_t > INT_MAX as int64_t {
            return -(1 as core::ffi::c_int);
        }
        num_allocated_rows = (if 1 as core::ffi::c_int
            > soft_allocation_limit / (src_stride_argb * min_num_rows)
        {
            1 as core::ffi::c_int
        } else {
            soft_allocation_limit / (src_stride_argb * min_num_rows)
        }) * min_num_rows;
    }
    src_argb = avifAlloc((num_allocated_rows * src_stride_argb) as size_t)
        as *mut uint8_t;
    if src_argb.is_null() {
        return -(1 as core::ffi::c_int);
    }
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < height {
        let num_rows: core::ffi::c_int = if num_allocated_rows < height - y {
            num_allocated_rows
        } else {
            height - y
        };
        if ReorderARGB
            .expect(
                "non-null function pointer",
            )(src_abgr, src_stride_abgr, src_argb, src_stride_argb, width, num_rows) != 0
            || ConvertToYUV
                .expect(
                    "non-null function pointer",
                )(
                src_argb,
                src_stride_argb,
                dst_y,
                dst_stride_y,
                dst_u,
                dst_stride_u,
                dst_v,
                dst_stride_v,
                width,
                num_rows,
            ) != 0
        {
            avifFree(src_argb as *mut core::ffi::c_void);
            return -(1 as core::ffi::c_int);
        }
        src_abgr = src_abgr
            .offset(
                (num_rows as size_t).wrapping_mul(src_stride_abgr as size_t) as isize,
            );
        dst_y = dst_y
            .offset((num_rows as size_t).wrapping_mul(dst_stride_y as size_t) as isize);
        dst_u = dst_u
            .offset(
                ((num_rows >> format_info.chromaShiftY) as size_t)
                    .wrapping_mul(dst_stride_u as size_t) as isize,
            );
        dst_v = dst_v
            .offset(
                ((num_rows >> format_info.chromaShiftY) as size_t)
                    .wrapping_mul(dst_stride_v as size_t) as isize,
            );
        y += num_allocated_rows;
    }
    avifFree(src_argb as *mut core::ffi::c_void);
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn avifRAWToI422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RAWToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifRAWToJ422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RAWToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToJ422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifABGRToI444(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            ABGRToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI444
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV444,
        width,
        height,
    );
}
unsafe extern "C" fn avifABGRToI422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            ABGRToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifBGRAToI444(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            BGRAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI444
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV444,
        width,
        height,
    );
}
unsafe extern "C" fn avifBGRAToI422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            BGRAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifBGRAToJ422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            BGRAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToJ422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifBGRAToJ420(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            BGRAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToJ420
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV420,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGB24ToI444(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGB24ToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI444
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV444,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGB24ToI422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGB24ToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGB24ToJ422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGB24ToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToJ422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGBAToI444(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGBAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI444
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV444,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGBAToI422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGBAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToI422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGBAToJ422(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGBAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToJ422
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV422,
        width,
        height,
    );
}
unsafe extern "C" fn avifRGBAToJ420(
    mut src_abgr: *const uint8_t,
    mut src_stride_abgr: core::ffi::c_int,
    mut dst_y: *mut uint8_t,
    mut dst_stride_y: core::ffi::c_int,
    mut dst_u: *mut uint8_t,
    mut dst_stride_u: core::ffi::c_int,
    mut dst_v: *mut uint8_t,
    mut dst_stride_v: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> core::ffi::c_int {
    return avifReorderARGBThenConvertToYUV(
        Some(
            RGBAToARGB
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        Some(
            ARGBToJ420
                as unsafe extern "C" fn(
                    *const uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    *mut uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> core::ffi::c_int,
        ),
        src_abgr,
        src_stride_abgr,
        dst_y,
        dst_stride_y,
        dst_u,
        dst_stride_u,
        dst_v,
        dst_stride_v,
        AVIF_PIXEL_FORMAT_YUV420,
        width,
        height,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifImageRGBToYUVLibYUV(
    mut image: *mut avifImage,
    mut rgb: *const avifRGBImage,
) -> avifResult {
    if (*image).width > INT_MAX as uint32_t || (*image).height > INT_MAX as uint32_t
        || (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
            > INT_MAX as uint32_t || (*rgb).rowBytes > INT_MAX as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*image).depth == 8 as uint32_t && (*rgb).depth == 8 as uint32_t {
        return avifImageRGBToYUVLibYUV8bpc(image, rgb);
    }
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
unsafe extern "C" fn avifImageRGBToYUVLibYUV8bpc(
    mut image: *mut avifImage,
    mut rgb: *const avifRGBImage,
) -> avifResult {
    'c_3561: {
        if (*image).depth == 8 as uint32_t && (*rgb).depth == 8 as uint32_t {} else {
            __assert_fail(
                b"(image->depth == 8) && (rgb->depth == 8)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat_libyuv.c\0"
                    as *const u8 as *const core::ffi::c_char,
                287 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 74],
                    [core::ffi::c_char; 74],
                >(
                    *b"avifResult avifImageRGBToYUVLibYUV8bpc(avifImage *, const avifRGBImage *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_BT470BG as core::ffi::c_int
        || (*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_BT601 as core::ffi::c_int
    {
        if (*image).yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            let mut lutRgbToY: [[RGBtoY; 10]; 2] = [
                [
                    None,
                    None,
                    None,
                    None,
                    Some(
                        ARGBToI400
                            as unsafe extern "C" fn(
                                *const uint8_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [
                    Some(
                        RAWToJ400
                            as unsafe extern "C" fn(
                                *const uint8_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        ABGRToJ400
                            as unsafe extern "C" fn(
                                *const uint8_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                    Some(
                        RGB24ToJ400
                            as unsafe extern "C" fn(
                                *const uint8_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        ARGBToJ400
                            as unsafe extern "C" fn(
                                *const uint8_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        RGBAToJ400
                            as unsafe extern "C" fn(
                                *const uint8_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                    None,
                    None,
                    None,
                ],
            ];
            let mut rgbToY: RGBtoY = lutRgbToY[(*image).yuvRange
                as usize][(*rgb).format as usize];
            if rgbToY.is_some() {
                if rgbToY
                    .expect(
                        "non-null function pointer",
                    )(
                    (*rgb).pixels,
                    (*rgb).rowBytes as core::ffi::c_int,
                    (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                    (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                        as core::ffi::c_int,
                    (*image).width as core::ffi::c_int,
                    (*image).height as core::ffi::c_int,
                ) != 0 as core::ffi::c_int
                {
                    return AVIF_RESULT_REFORMAT_FAILED;
                }
                return AVIF_RESULT_OK;
            }
        } else {
            let mut lutRgbToYuv: [[[RGBtoYUV; 5]; 10]; 2] = [
                [
                    [
                        None,
                        Some(
                            RAWToI444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifRAWToI422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            RAWToI420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        Some(
                            avifABGRToI444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifABGRToI422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            ABGRToI420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        Some(
                            avifBGRAToI444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifBGRAToI422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            BGRAToI420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        Some(
                            avifRGB24ToI444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifRGB24ToI422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            RGB24ToI420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        Some(
                            ARGBToI444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            ARGBToI422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            ARGBToI420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        Some(
                            avifRGBAToI444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifRGBAToI422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            RGBAToI420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [None, None, None, None, None],
                    [None; 5],
                    [None; 5],
                    [None; 5],
                ],
                [
                    [
                        None,
                        Some(
                            RAWToJ444
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifRAWToJ422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            RAWToJ420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        None,
                        Some(
                            ABGRToJ422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            ABGRToJ420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        None,
                        Some(
                            avifBGRAToJ422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifBGRAToJ420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        None,
                        Some(
                            avifRGB24ToJ422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            RGB24ToJ420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        None,
                        Some(
                            ARGBToJ422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            ARGBToJ420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [
                        None,
                        None,
                        Some(
                            avifRGBAToJ422
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        Some(
                            avifRGBAToJ420
                                as unsafe extern "C" fn(
                                    *const uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    *mut uint8_t,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                    core::ffi::c_int,
                                ) -> core::ffi::c_int,
                        ),
                        None,
                    ],
                    [None, None, None, None, None],
                    [None; 5],
                    [None; 5],
                    [None; 5],
                ],
            ];
            let mut rgbToYuv: RGBtoYUV = lutRgbToYuv[(*image).yuvRange
                as usize][(*rgb).format as usize][(*image).yuvFormat as usize];
            if rgbToYuv.is_some() {
                if rgbToYuv
                    .expect(
                        "non-null function pointer",
                    )(
                    (*rgb).pixels,
                    (*rgb).rowBytes as core::ffi::c_int,
                    (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                    (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                        as core::ffi::c_int,
                    (*image).yuvPlanes[AVIF_CHAN_U as core::ffi::c_int as usize],
                    (*image).yuvRowBytes[AVIF_CHAN_U as core::ffi::c_int as usize]
                        as core::ffi::c_int,
                    (*image).yuvPlanes[AVIF_CHAN_V as core::ffi::c_int as usize],
                    (*image).yuvRowBytes[AVIF_CHAN_V as core::ffi::c_int as usize]
                        as core::ffi::c_int,
                    (*image).width as core::ffi::c_int,
                    (*image).height as core::ffi::c_int,
                ) != 0 as core::ffi::c_int
                {
                    return AVIF_RESULT_REFORMAT_FAILED;
                }
                return AVIF_RESULT_OK;
            }
        }
    }
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
static mut lutIsYVU: [avifBool; 10] = [
    AVIF_TRUE,
    AVIF_TRUE,
    AVIF_TRUE,
    AVIF_FALSE,
    AVIF_FALSE,
    AVIF_FALSE,
    AVIF_FALSE,
    0,
    0,
    0,
];
unsafe extern "C" fn nearestNeighborFilterAllowed(
    mut chromaUpsampling: core::ffi::c_int,
) -> avifBool {
    return (chromaUpsampling != AVIF_CHROMA_UPSAMPLING_BILINEAR as core::ffi::c_int
        && chromaUpsampling != AVIF_CHROMA_UPSAMPLING_BEST_QUALITY as core::ffi::c_int)
        as core::ffi::c_int;
}
unsafe extern "C" fn getLibYUVConversionFunction(
    mut yuvFormat: avifPixelFormat,
    mut yuvDepth: core::ffi::c_int,
    mut rgb: *mut avifRGBImage,
    mut alphaPreferred: avifBool,
    mut lcf: *mut LibyuvConversionFunction,
) -> avifBool {
    static mut lutYuv400ToRgbMatrix: [YUV400ToRGBMatrix; 10] = unsafe {
        [
            None,
            Some(
                I400ToARGBMatrix
                    as unsafe extern "C" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint8_t,
                        core::ffi::c_int,
                        *const YuvConstants,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> core::ffi::c_int,
            ),
            None,
            None,
            Some(
                I400ToARGBMatrix
                    as unsafe extern "C" fn(
                        *const uint8_t,
                        core::ffi::c_int,
                        *mut uint8_t,
                        core::ffi::c_int,
                        *const YuvConstants,
                        core::ffi::c_int,
                        core::ffi::c_int,
                    ) -> core::ffi::c_int,
            ),
            None,
            None,
            None,
            None,
            None,
        ]
    };
    static mut lutYuvToRgbMatrixFilter: [[YUVToRGBMatrixFilter; 5]; 10] = unsafe {
        [
            [
                None,
                None,
                Some(
                    I422ToRGB24MatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToRGB24MatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                None,
                Some(
                    I422ToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None, None, None, None, None],
            [
                None,
                None,
                Some(
                    I422ToRGB24MatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToRGB24MatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                None,
                Some(
                    I422ToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None, None, None, None, None],
            [None, None, None, None, None],
            [None; 5],
            [None; 5],
            [None; 5],
        ]
    };
    static mut lutYuvaToRgbMatrixFilter: [[YUVAToRGBMatrixFilter; 5]; 10] = unsafe {
        [
            [None, None, None, None, None],
            [
                None,
                None,
                Some(
                    I422AlphaToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420AlphaToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None, None, None, None, None],
            [None, None, None, None, None],
            [
                None,
                None,
                Some(
                    I422AlphaToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420AlphaToARGBMatrixFilter
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            FilterMode,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None, None, None, None, None],
            [None, None, None, None, None],
            [None; 5],
            [None; 5],
            [None; 5],
        ]
    };
    static mut lutYuvToRgbMatrix: [[YUVToRGBMatrix; 5]; 10] = unsafe {
        [
            [
                None,
                Some(
                    I444ToRGB24Matrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
                Some(
                    I420ToRGB24Matrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                Some(
                    I444ToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I422ToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                None,
                Some(
                    I422ToRGBAMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToRGBAMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                Some(
                    I444ToRGB24Matrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
                Some(
                    I420ToRGB24Matrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                Some(
                    I444ToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I422ToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                None,
                Some(
                    I422ToRGBAMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToRGBAMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [
                None,
                None,
                Some(
                    I422ToRGB565Matrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420ToRGB565Matrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None; 5],
            [None; 5],
            [None; 5],
        ]
    };
    static mut lutYuvaToRgbMatrix: [[YUVAToRGBMatrix; 5]; 10] = unsafe {
        [
            [None, None, None, None, None],
            [
                None,
                Some(
                    I444AlphaToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I422AlphaToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420AlphaToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None, None, None, None, None],
            [None, None, None, None, None],
            [
                None,
                Some(
                    I444AlphaToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I422AlphaToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                Some(
                    I420AlphaToARGBMatrix
                        as unsafe extern "C" fn(
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *const uint8_t,
                            core::ffi::c_int,
                            *mut uint8_t,
                            core::ffi::c_int,
                            *const YuvConstants,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> core::ffi::c_int,
                ),
                None,
            ],
            [None, None, None, None, None],
            [None, None, None, None, None],
            [None; 5],
            [None; 5],
            [None; 5],
        ]
    };
    static mut lutYuvToRgbMatrixFilterHighBitDepth: [[[YUVToRGBMatrixFilterHighBitDepth; 5]; 10]; 2] = unsafe {
        [
            [
                [None, None, None, None, None],
                [
                    None,
                    None,
                    Some(
                        I210ToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010ToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [
                    None,
                    None,
                    Some(
                        I210ToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010ToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
            [
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
        ]
    };
    static mut lutYuvaToRgbMatrixFilterHighBitDepth: [[[YUVAToRGBMatrixFilterHighBitDepth; 5]; 10]; 2] = unsafe {
        [
            [
                [None, None, None, None, None],
                [
                    None,
                    None,
                    Some(
                        I210AlphaToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010AlphaToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [
                    None,
                    None,
                    Some(
                        I210AlphaToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010AlphaToARGBMatrixFilter
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                FilterMode,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
            [
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
        ]
    };
    static mut lutYuvToRgbMatrixHighBitDepth: [[[YUVToRGBMatrixHighBitDepth; 5]; 10]; 2] = unsafe {
        [
            [
                [None, None, None, None, None],
                [
                    None,
                    Some(
                        I410ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I210ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [
                    None,
                    Some(
                        I410ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I210ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
            [
                [None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    Some(
                        I012ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [
                    None,
                    None,
                    None,
                    Some(
                        I012ToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
        ]
    };
    static mut lutYuvaToRgbMatrixHighBitDepth: [[[YUVAToRGBMatrixHighBitDepth; 5]; 10]; 2] = unsafe {
        [
            [
                [None, None, None, None, None],
                [
                    None,
                    Some(
                        I410AlphaToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I210AlphaToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010AlphaToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [
                    None,
                    Some(
                        I410AlphaToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I210AlphaToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    Some(
                        I010AlphaToARGBMatrix
                            as unsafe extern "C" fn(
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *const uint16_t,
                                core::ffi::c_int,
                                *mut uint8_t,
                                core::ffi::c_int,
                                *const YuvConstants,
                                core::ffi::c_int,
                                core::ffi::c_int,
                                core::ffi::c_int,
                            ) -> core::ffi::c_int,
                    ),
                    None,
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
            [
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None; 5],
                [None; 5],
                [None; 5],
            ],
        ]
    };
    memset(
        lcf as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<LibyuvConversionFunction>() as size_t,
    );
    'c_7854: {
        if (*rgb).depth == 8 as uint32_t {} else {
            __assert_fail(
                b"rgb->depth == 8\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat_libyuv.c\0"
                    as *const u8 as *const core::ffi::c_char,
                715 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 113],
                    [core::ffi::c_char; 113],
                >(
                    *b"avifBool getLibYUVConversionFunction(avifPixelFormat, int, avifRGBImage *, avifBool, LibyuvConversionFunction *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if yuvDepth > 8 as core::ffi::c_int {
        'c_7803: {
            if yuvDepth == 10 as core::ffi::c_int || yuvDepth == 12 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"yuvDepth == 10 || yuvDepth == 12\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat_libyuv.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    717 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 113],
                        [core::ffi::c_char; 113],
                    >(
                        *b"avifBool getLibYUVConversionFunction(avifPixelFormat, int, avifRGBImage *, avifBool, LibyuvConversionFunction *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut depthIndex: core::ffi::c_int = if yuvDepth == 10 as core::ffi::c_int {
            0 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        if yuvFormat as core::ffi::c_uint
            != AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
        {
            if alphaPreferred != 0 {
                (*lcf).yuvaToRgbMatrixFilterHighBitDepth = lutYuvaToRgbMatrixFilterHighBitDepth[depthIndex
                    as usize][(*rgb).format as usize][yuvFormat as usize];
                if ((*lcf).yuvaToRgbMatrixFilterHighBitDepth).is_some() {
                    return AVIF_TRUE;
                }
            }
            (*lcf).yuvToRgbMatrixFilterHighBitDepth = lutYuvToRgbMatrixFilterHighBitDepth[depthIndex
                as usize][(*rgb).format as usize][yuvFormat as usize];
            if ((*lcf).yuvToRgbMatrixFilterHighBitDepth).is_some() {
                return AVIF_TRUE;
            }
        }
        if yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
            || nearestNeighborFilterAllowed((*rgb).chromaUpsampling as core::ffi::c_int)
                != 0
        {
            if alphaPreferred != 0 {
                (*lcf).yuvaToRgbMatrixHighBitDepth = lutYuvaToRgbMatrixHighBitDepth[depthIndex
                    as usize][(*rgb).format as usize][yuvFormat as usize];
                if ((*lcf).yuvaToRgbMatrixHighBitDepth).is_some() {
                    return AVIF_TRUE;
                }
            }
            (*lcf).yuvToRgbMatrixHighBitDepth = lutYuvToRgbMatrixHighBitDepth[depthIndex
                as usize][(*rgb).format as usize][yuvFormat as usize];
            if ((*lcf).yuvToRgbMatrixHighBitDepth).is_some() {
                return AVIF_TRUE;
            }
        }
    }
    if yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
    {
        (*lcf).yuv400ToRgbMatrix = lutYuv400ToRgbMatrix[(*rgb).format as usize];
        return ((*lcf).yuv400ToRgbMatrix).is_some() as core::ffi::c_int;
    }
    if yuvFormat as core::ffi::c_uint
        != AVIF_PIXEL_FORMAT_YUV444 as core::ffi::c_int as core::ffi::c_uint
    {
        if alphaPreferred != 0 {
            (*lcf).yuvaToRgbMatrixFilter = lutYuvaToRgbMatrixFilter[(*rgb).format
                as usize][yuvFormat as usize];
            if ((*lcf).yuvaToRgbMatrixFilter).is_some() {
                return AVIF_TRUE;
            }
        }
        (*lcf).yuvToRgbMatrixFilter = lutYuvToRgbMatrixFilter[(*rgb).format
            as usize][yuvFormat as usize];
        if ((*lcf).yuvToRgbMatrixFilter).is_some() {
            return AVIF_TRUE;
        }
        if nearestNeighborFilterAllowed((*rgb).chromaUpsampling as core::ffi::c_int) == 0
        {
            return AVIF_FALSE;
        }
    }
    if alphaPreferred != 0 {
        (*lcf).yuvaToRgbMatrix = lutYuvaToRgbMatrix[(*rgb).format
            as usize][yuvFormat as usize];
        if ((*lcf).yuvaToRgbMatrix).is_some() {
            return AVIF_TRUE;
        }
    }
    (*lcf).yuvToRgbMatrix = lutYuvToRgbMatrix[(*rgb).format
        as usize][yuvFormat as usize];
    return ((*lcf).yuvToRgbMatrix).is_some() as core::ffi::c_int;
}
unsafe extern "C" fn getLibYUVConstants(
    mut image: *const avifImage,
    mut matrixYUV: *mut *const YuvConstants,
    mut matrixYVU: *mut *const YuvConstants,
) {
    let yuv400WithIdentityMatrix: avifBool = ((*image).yuvFormat as core::ffi::c_uint
        == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        && (*image).matrixCoefficients as core::ffi::c_int
            == AVIF_MATRIX_COEFFICIENTS_IDENTITY as core::ffi::c_int)
        as core::ffi::c_int;
    let matrixCoefficients: avifMatrixCoefficients = (if yuv400WithIdentityMatrix != 0 {
        AVIF_MATRIX_COEFFICIENTS_BT601 as core::ffi::c_int
    } else {
        (*image).matrixCoefficients as core::ffi::c_int
    }) as avifMatrixCoefficients;
    if (*image).yuvRange as core::ffi::c_uint
        == AVIF_RANGE_FULL as core::ffi::c_int as core::ffi::c_uint
    {
        match matrixCoefficients as core::ffi::c_int {
            1 => {
                *matrixYUV = &kYuvF709Constants;
                *matrixYVU = &kYvuF709Constants;
            }
            5 | 6 | 2 => {
                *matrixYUV = &kYuvJPEGConstants;
                *matrixYVU = &kYvuJPEGConstants;
            }
            9 => {
                *matrixYUV = &kYuvV2020Constants;
                *matrixYVU = &kYvuV2020Constants;
            }
            12 => {
                match (*image).colorPrimaries as core::ffi::c_int {
                    1 | 2 => {
                        *matrixYUV = &kYuvF709Constants;
                        *matrixYVU = &kYvuF709Constants;
                    }
                    5 | 6 => {
                        *matrixYUV = &kYuvJPEGConstants;
                        *matrixYVU = &kYvuJPEGConstants;
                    }
                    9 => {
                        *matrixYUV = &kYuvV2020Constants;
                        *matrixYVU = &kYvuV2020Constants;
                    }
                    0 | 4 | 7 | 8 | 10 | 11 | 12 | 22 | _ => {}
                }
            }
            0 | 4 | 7 | 8 | 10 | 11 | 13 | 14 | _ => {}
        }
    } else {
        match matrixCoefficients as core::ffi::c_int {
            1 => {
                *matrixYUV = &kYuvH709Constants;
                *matrixYVU = &kYvuH709Constants;
            }
            5 | 6 | 2 => {
                *matrixYUV = &kYuvI601Constants as *const YuvConstants;
                *matrixYVU = &kYvuI601Constants;
            }
            9 => {
                *matrixYUV = &kYuv2020Constants;
                *matrixYVU = &kYvu2020Constants;
            }
            12 => {
                match (*image).colorPrimaries as core::ffi::c_int {
                    1 | 2 => {
                        *matrixYUV = &kYuvH709Constants;
                        *matrixYVU = &kYvuH709Constants;
                    }
                    5 | 6 => {
                        *matrixYUV = &kYuvI601Constants as *const YuvConstants;
                        *matrixYVU = &kYvuI601Constants;
                    }
                    9 => {
                        *matrixYUV = &kYuv2020Constants;
                        *matrixYVU = &kYvu2020Constants;
                    }
                    0 | 4 | 7 | 8 | 10 | 11 | 12 | 22 | _ => {}
                }
            }
            0 | 4 | 7 | 8 | 10 | 11 | 13 | 14 | _ => {}
        }
    };
}
unsafe extern "C" fn avifImageDownshiftTo8bpc(
    mut image: *const avifImage,
    mut image8: *mut avifImage,
    mut downshiftAlpha: avifBool,
) -> avifResult {
    avifImageSetDefaults(image8);
    avifImageCopyNoAlloc(image8, image);
    (*image8).depth = 8 as uint32_t;
    'c_4601: {
        if downshiftAlpha == 0 || !((*image).alphaPlane).is_null() {} else {
            __assert_fail(
                b"!downshiftAlpha || image->alphaPlane\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/reformat_libyuv.c\0"
                    as *const u8 as *const core::ffi::c_char,
                912 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 78],
                    [core::ffi::c_char; 78],
                >(
                    *b"avifResult avifImageDownshiftTo8bpc(const avifImage *, avifImage *, avifBool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let result__: avifResult = avifImageAllocatePlanes(
        image8,
        (if downshiftAlpha != 0 {
            AVIF_PLANES_ALL as core::ffi::c_int
        } else {
            AVIF_PLANES_YUV as core::ffi::c_int
        }) as avifPlanesFlags,
    ) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    let scale: core::ffi::c_int = (1 as core::ffi::c_int)
        << (24 as uint32_t).wrapping_sub((*image).depth);
    let mut plane: core::ffi::c_int = AVIF_CHAN_Y as core::ffi::c_int;
    while plane
        <= (if downshiftAlpha != 0 {
            AVIF_CHAN_A as core::ffi::c_int
        } else {
            AVIF_CHAN_V as core::ffi::c_int
        })
    {
        let planeWidth: uint32_t = avifImagePlaneWidth(image, plane) as uint32_t;
        if !(planeWidth == 0 as uint32_t) {
            Convert16To8Plane(
                avifImagePlane(image, plane) as *const uint16_t,
                (avifImagePlaneRowBytes(image, plane)).wrapping_div(2 as uint32_t)
                    as core::ffi::c_int,
                avifImagePlane(image8, plane),
                avifImagePlaneRowBytes(image8, plane) as core::ffi::c_int,
                scale,
                planeWidth as core::ffi::c_int,
                avifImagePlaneHeight(image, plane) as core::ffi::c_int,
            );
        }
        plane += 1;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageYUVToRGBLibYUV(
    mut image: *const avifImage,
    mut rgb: *mut avifRGBImage,
    mut reformatAlpha: avifBool,
    mut alphaReformattedWithLibYUV: *mut avifBool,
) -> avifResult {
    *alphaReformattedWithLibYUV = AVIF_FALSE as avifBool;
    if (*image).width > INT_MAX as uint32_t || (*image).height > INT_MAX as uint32_t
        || (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
            > INT_MAX as uint32_t || (*rgb).rowBytes > INT_MAX as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*rgb).depth != 8 as uint32_t
        || (*image).depth != 8 as uint32_t && (*image).depth != 10 as uint32_t
            && (*image).depth != 12 as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut matrixYUV: *const YuvConstants = 0 as *const YuvConstants;
    let mut matrixYVU: *const YuvConstants = 0 as *const YuvConstants;
    getLibYUVConstants(image, &mut matrixYUV, &mut matrixYVU);
    if matrixYVU.is_null() {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let mut lcf: LibyuvConversionFunction = LibyuvConversionFunction {
        yuv400ToRgbMatrix: None,
        yuvToRgbMatrixFilter: None,
        yuvaToRgbMatrixFilter: None,
        yuvToRgbMatrix: None,
        yuvaToRgbMatrix: None,
        yuvToRgbMatrixFilterHighBitDepth: None,
        yuvaToRgbMatrixFilterHighBitDepth: None,
        yuvToRgbMatrixHighBitDepth: None,
        yuvaToRgbMatrixHighBitDepth: None,
    };
    let alphaPreferred: avifBool = (reformatAlpha != 0
        && !((*image).alphaPlane).is_null() && (*image).alphaRowBytes != 0)
        as core::ffi::c_int;
    if getLibYUVConversionFunction(
        (*image).yuvFormat,
        (*image).depth as core::ffi::c_int,
        rgb,
        alphaPreferred,
        &mut lcf,
    ) == 0
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if ((*image).alphaPlane).is_null() || (*image).alphaRowBytes == 0 {
        *alphaReformattedWithLibYUV = AVIF_TRUE as avifBool;
    }
    let mut isYVU: avifBool = lutIsYVU[(*rgb).format as usize];
    let mut matrix: *const YuvConstants = if isYVU != 0 { matrixYVU } else { matrixYUV };
    let mut libyuvResult: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut uPlaneIndex: core::ffi::c_int = if isYVU != 0 {
        AVIF_CHAN_V as core::ffi::c_int
    } else {
        AVIF_CHAN_U as core::ffi::c_int
    };
    let mut vPlaneIndex: core::ffi::c_int = if isYVU != 0 {
        AVIF_CHAN_U as core::ffi::c_int
    } else {
        AVIF_CHAN_V as core::ffi::c_int
    };
    let filter: FilterMode = (if (*rgb).chromaUpsampling as core::ffi::c_uint
        == AVIF_CHROMA_UPSAMPLING_FASTEST as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).chromaUpsampling as core::ffi::c_uint
            == AVIF_CHROMA_UPSAMPLING_NEAREST as core::ffi::c_int as core::ffi::c_uint
    {
        kFilterNone as core::ffi::c_int
    } else {
        kFilterBilinear as core::ffi::c_int
    }) as FilterMode;
    if (lcf.yuvToRgbMatrixFilterHighBitDepth).is_some() {
        libyuvResult = (lcf.yuvToRgbMatrixFilterHighBitDepth)
            .expect(
                "non-null function pointer",
            )(
            (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                as *const uint16_t,
            ((*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize])
                .wrapping_div(2 as uint32_t) as core::ffi::c_int,
            (*image).yuvPlanes[uPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[uPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*image).yuvPlanes[vPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[vPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            matrix,
            (*image).width as core::ffi::c_int,
            (*image).height as core::ffi::c_int,
            filter,
        );
    } else if (lcf.yuvaToRgbMatrixFilterHighBitDepth).is_some() {
        libyuvResult = (lcf.yuvaToRgbMatrixFilterHighBitDepth)
            .expect(
                "non-null function pointer",
            )(
            (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                as *const uint16_t,
            ((*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize])
                .wrapping_div(2 as uint32_t) as core::ffi::c_int,
            (*image).yuvPlanes[uPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[uPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*image).yuvPlanes[vPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[vPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*image).alphaPlane as *const uint16_t,
            ((*image).alphaRowBytes).wrapping_div(2 as uint32_t) as core::ffi::c_int,
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            matrix,
            (*image).width as core::ffi::c_int,
            (*image).height as core::ffi::c_int,
            0 as core::ffi::c_int,
            filter,
        );
        *alphaReformattedWithLibYUV = AVIF_TRUE as avifBool;
    } else if (lcf.yuvToRgbMatrixHighBitDepth).is_some() {
        libyuvResult = (lcf.yuvToRgbMatrixHighBitDepth)
            .expect(
                "non-null function pointer",
            )(
            (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                as *const uint16_t,
            ((*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize])
                .wrapping_div(2 as uint32_t) as core::ffi::c_int,
            (*image).yuvPlanes[uPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[uPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*image).yuvPlanes[vPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[vPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            matrix,
            (*image).width as core::ffi::c_int,
            (*image).height as core::ffi::c_int,
        );
    } else if (lcf.yuvaToRgbMatrixHighBitDepth).is_some() {
        libyuvResult = (lcf.yuvaToRgbMatrixHighBitDepth)
            .expect(
                "non-null function pointer",
            )(
            (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                as *const uint16_t,
            ((*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize])
                .wrapping_div(2 as uint32_t) as core::ffi::c_int,
            (*image).yuvPlanes[uPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[uPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*image).yuvPlanes[vPlaneIndex as usize] as *const uint16_t,
            ((*image).yuvRowBytes[vPlaneIndex as usize]).wrapping_div(2 as uint32_t)
                as core::ffi::c_int,
            (*image).alphaPlane as *const uint16_t,
            ((*image).alphaRowBytes).wrapping_div(2 as uint32_t) as core::ffi::c_int,
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            matrix,
            (*image).width as core::ffi::c_int,
            (*image).height as core::ffi::c_int,
            0 as core::ffi::c_int,
        );
        *alphaReformattedWithLibYUV = AVIF_TRUE as avifBool;
    } else {
        let mut image8: avifImage = avifImage {
            width: 0,
            height: 0,
            depth: 0,
            yuvFormat: AVIF_PIXEL_FORMAT_NONE,
            yuvRange: AVIF_RANGE_LIMITED,
            yuvChromaSamplePosition: AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN,
            yuvPlanes: [0 as *mut uint8_t; 3],
            yuvRowBytes: [0; 3],
            imageOwnsYUVPlanes: 0,
            alphaPlane: 0 as *mut uint8_t,
            alphaRowBytes: 0,
            imageOwnsAlphaPlane: 0,
            alphaPremultiplied: 0,
            icc: avifRWData {
                data: 0 as *mut uint8_t,
                size: 0,
            },
            colorPrimaries: 0,
            transferCharacteristics: 0,
            matrixCoefficients: 0,
            clli: avifContentLightLevelInformationBox {
                maxCLL: 0,
                maxPALL: 0,
            },
            transformFlags: 0,
            pasp: avifPixelAspectRatioBox {
                hSpacing: 0,
                vSpacing: 0,
            },
            clap: avifCleanApertureBox {
                widthN: 0,
                widthD: 0,
                heightN: 0,
                heightD: 0,
                horizOffN: 0,
                horizOffD: 0,
                vertOffN: 0,
                vertOffD: 0,
            },
            irot: avifImageRotation { angle: 0 },
            imir: avifImageMirror { axis: 0 },
            exif: avifRWData {
                data: 0 as *mut uint8_t,
                size: 0,
            },
            xmp: avifRWData {
                data: 0 as *mut uint8_t,
                size: 0,
            },
            properties: 0 as *mut avifImageItemProperty,
            numProperties: 0,
            gainMap: 0 as *mut avifGainMap,
        };
        let mut inputIsHighBitDepth: avifBool = ((*image).depth > 8 as uint32_t)
            as core::ffi::c_int;
        if inputIsHighBitDepth != 0 {
            let downshiftAlpha: avifBool = ((lcf.yuvaToRgbMatrixFilter).is_some()
                || (lcf.yuvaToRgbMatrix).is_some()) as core::ffi::c_int;
            let result__: avifResult = avifImageDownshiftTo8bpc(
                image,
                &mut image8,
                downshiftAlpha,
            ) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            image = &mut image8;
        }
        if (lcf.yuv400ToRgbMatrix).is_some() {
            libyuvResult = (lcf.yuv400ToRgbMatrix)
                .expect(
                    "non-null function pointer",
                )(
                (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                (*rgb).pixels,
                (*rgb).rowBytes as core::ffi::c_int,
                matrix,
                (*image).width as core::ffi::c_int,
                (*image).height as core::ffi::c_int,
            );
        } else if (lcf.yuvToRgbMatrixFilter).is_some() {
            libyuvResult = (lcf.yuvToRgbMatrixFilter)
                .expect(
                    "non-null function pointer",
                )(
                (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                (*image).yuvPlanes[uPlaneIndex as usize],
                (*image).yuvRowBytes[uPlaneIndex as usize] as core::ffi::c_int,
                (*image).yuvPlanes[vPlaneIndex as usize],
                (*image).yuvRowBytes[vPlaneIndex as usize] as core::ffi::c_int,
                (*rgb).pixels,
                (*rgb).rowBytes as core::ffi::c_int,
                matrix,
                (*image).width as core::ffi::c_int,
                (*image).height as core::ffi::c_int,
                filter,
            );
        } else if (lcf.yuvaToRgbMatrixFilter).is_some() {
            libyuvResult = (lcf.yuvaToRgbMatrixFilter)
                .expect(
                    "non-null function pointer",
                )(
                (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                (*image).yuvPlanes[uPlaneIndex as usize],
                (*image).yuvRowBytes[uPlaneIndex as usize] as core::ffi::c_int,
                (*image).yuvPlanes[vPlaneIndex as usize],
                (*image).yuvRowBytes[vPlaneIndex as usize] as core::ffi::c_int,
                (*image).alphaPlane,
                (*image).alphaRowBytes as core::ffi::c_int,
                (*rgb).pixels,
                (*rgb).rowBytes as core::ffi::c_int,
                matrix,
                (*image).width as core::ffi::c_int,
                (*image).height as core::ffi::c_int,
                0 as core::ffi::c_int,
                filter,
            );
            *alphaReformattedWithLibYUV = AVIF_TRUE as avifBool;
        } else if (lcf.yuvToRgbMatrix).is_some() {
            libyuvResult = (lcf.yuvToRgbMatrix)
                .expect(
                    "non-null function pointer",
                )(
                (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                (*image).yuvPlanes[uPlaneIndex as usize],
                (*image).yuvRowBytes[uPlaneIndex as usize] as core::ffi::c_int,
                (*image).yuvPlanes[vPlaneIndex as usize],
                (*image).yuvRowBytes[vPlaneIndex as usize] as core::ffi::c_int,
                (*rgb).pixels,
                (*rgb).rowBytes as core::ffi::c_int,
                matrix,
                (*image).width as core::ffi::c_int,
                (*image).height as core::ffi::c_int,
            );
        } else if (lcf.yuvaToRgbMatrix).is_some() {
            libyuvResult = (lcf.yuvaToRgbMatrix)
                .expect(
                    "non-null function pointer",
                )(
                (*image).yuvPlanes[AVIF_CHAN_Y as core::ffi::c_int as usize],
                (*image).yuvRowBytes[AVIF_CHAN_Y as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                (*image).yuvPlanes[uPlaneIndex as usize],
                (*image).yuvRowBytes[uPlaneIndex as usize] as core::ffi::c_int,
                (*image).yuvPlanes[vPlaneIndex as usize],
                (*image).yuvRowBytes[vPlaneIndex as usize] as core::ffi::c_int,
                (*image).alphaPlane,
                (*image).alphaRowBytes as core::ffi::c_int,
                (*rgb).pixels,
                (*rgb).rowBytes as core::ffi::c_int,
                matrix,
                (*image).width as core::ffi::c_int,
                (*image).height as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
            *alphaReformattedWithLibYUV = AVIF_TRUE as avifBool;
        }
        if inputIsHighBitDepth != 0 {
            avifImageFreePlanes(
                &mut image8,
                AVIF_PLANES_ALL as core::ffi::c_int as avifPlanesFlags,
            );
            image = 0 as *const avifImage;
        }
    }
    return (if libyuvResult != 0 as core::ffi::c_int {
        AVIF_RESULT_REFORMAT_FAILED as core::ffi::c_int
    } else {
        AVIF_RESULT_OK as core::ffi::c_int
    }) as avifResult;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImagePremultiplyAlphaLibYUV(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    if (*rgb).width > INT_MAX as uint32_t || (*rgb).height > INT_MAX as uint32_t
        || (*rgb).rowBytes > INT_MAX as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*rgb).depth != 8 as uint32_t {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGBA as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_BGRA as core::ffi::c_int as core::ffi::c_uint
    {
        if ARGBAttenuate(
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            (*rgb).width as core::ffi::c_int,
            (*rgb).height as core::ffi::c_int,
        ) != 0 as core::ffi::c_int
        {
            return AVIF_RESULT_REFORMAT_FAILED;
        }
        return AVIF_RESULT_OK;
    }
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageUnpremultiplyAlphaLibYUV(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    if (*rgb).width > INT_MAX as uint32_t || (*rgb).height > INT_MAX as uint32_t
        || (*rgb).rowBytes > INT_MAX as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*rgb).depth != 8 as uint32_t {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    if (*rgb).format as core::ffi::c_uint
        == AVIF_RGB_FORMAT_RGBA as core::ffi::c_int as core::ffi::c_uint
        || (*rgb).format as core::ffi::c_uint
            == AVIF_RGB_FORMAT_BGRA as core::ffi::c_int as core::ffi::c_uint
    {
        if ARGBUnattenuate(
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            (*rgb).pixels,
            (*rgb).rowBytes as core::ffi::c_int,
            (*rgb).width as core::ffi::c_int,
            (*rgb).height as core::ffi::c_int,
        ) != 0 as core::ffi::c_int
        {
            return AVIF_RESULT_REFORMAT_FAILED;
        }
        return AVIF_RESULT_OK;
    }
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifRGBImageToF16LibYUV(
    mut rgb: *mut avifRGBImage,
) -> avifResult {
    if (*rgb).width > INT_MAX as uint32_t || (*rgb).height > INT_MAX as uint32_t
        || (*rgb).rowBytes > INT_MAX as uint32_t
    {
        return AVIF_RESULT_NOT_IMPLEMENTED;
    }
    let scale: core::ffi::c_float = 1.0f32
        / (((1 as core::ffi::c_int) << (*rgb).depth) - 1 as core::ffi::c_int)
            as core::ffi::c_float;
    let result: core::ffi::c_int = HalfFloatPlane(
        (*rgb).pixels as *const uint16_t,
        (*rgb).rowBytes as core::ffi::c_int,
        (*rgb).pixels as *mut uint16_t,
        (*rgb).rowBytes as core::ffi::c_int,
        scale,
        ((*rgb).width).wrapping_mul(avifRGBFormatChannelCount((*rgb).format))
            as core::ffi::c_int,
        (*rgb).height as core::ffi::c_int,
    ) as core::ffi::c_int;
    return (if result == 0 as core::ffi::c_int {
        AVIF_RESULT_OK as core::ffi::c_int
    } else {
        AVIF_RESULT_INVALID_ARGUMENT as core::ffi::c_int
    }) as avifResult;
}
#[no_mangle]
pub unsafe extern "C" fn avifLibYUVVersion() -> core::ffi::c_uint {
    return LIBYUV_VERSION as core::ffi::c_uint;
}
