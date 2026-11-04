/*
 *  Copyright 2011 The LibYuv Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS. All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Mechanically translated from upstream/libavif/third_party/libyuv/source/scale.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
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
    fn CopyPlane(
        src_y: *const uint8_t,
        src_stride_y: core::ffi::c_int,
        dst_y: *mut uint8_t,
        dst_stride_y: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn CopyPlane_16(
        src_y: *const uint16_t,
        src_stride_y: core::ffi::c_int,
        dst_y: *mut uint16_t,
        dst_stride_y: core::ffi::c_int,
        width: core::ffi::c_int,
        height: core::ffi::c_int,
    );
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn InterpolateRow_C(
        dst_ptr: *mut uint8_t,
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        width: core::ffi::c_int,
        source_y_fraction: core::ffi::c_int,
    );
    fn InterpolateRow_16_C(
        dst_ptr: *mut uint16_t,
        src_ptr: *const uint16_t,
        src_stride: ptrdiff_t,
        width: core::ffi::c_int,
        source_y_fraction: core::ffi::c_int,
    );
    fn ScalePlaneVertical(
        src_height: core::ffi::c_int,
        dst_width: core::ffi::c_int,
        dst_height: core::ffi::c_int,
        src_stride: core::ffi::c_int,
        dst_stride: core::ffi::c_int,
        src_argb: *const uint8_t,
        dst_argb: *mut uint8_t,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
        dy: core::ffi::c_int,
        bpp: core::ffi::c_int,
        filtering: FilterMode,
    );
    fn ScalePlaneVertical_16(
        src_height: core::ffi::c_int,
        dst_width: core::ffi::c_int,
        dst_height: core::ffi::c_int,
        src_stride: core::ffi::c_int,
        dst_stride: core::ffi::c_int,
        src_argb: *const uint16_t,
        dst_argb: *mut uint16_t,
        x: core::ffi::c_int,
        y: core::ffi::c_int,
        dy: core::ffi::c_int,
        wpp: core::ffi::c_int,
        filtering: FilterMode,
    );
    fn ScaleFilterReduce(
        src_width: core::ffi::c_int,
        src_height: core::ffi::c_int,
        dst_width: core::ffi::c_int,
        dst_height: core::ffi::c_int,
        filtering: FilterMode,
    ) -> FilterMode;
    fn FixedDiv_C(num: core::ffi::c_int, div: core::ffi::c_int) -> core::ffi::c_int;
    fn FixedDiv1_C(num: core::ffi::c_int, div: core::ffi::c_int) -> core::ffi::c_int;
    fn ScaleSlope(
        src_width: core::ffi::c_int,
        src_height: core::ffi::c_int,
        dst_width: core::ffi::c_int,
        dst_height: core::ffi::c_int,
        filtering: FilterMode,
        x: *mut core::ffi::c_int,
        y: *mut core::ffi::c_int,
        dx: *mut core::ffi::c_int,
        dy: *mut core::ffi::c_int,
    );
    fn ScaleRowUp2_Linear_Any_C(
        src_ptr: *const uint8_t,
        dst_ptr: *mut uint8_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleRowUp2_Bilinear_Any_C(
        src_ptr: *const uint8_t,
        src_stride: ptrdiff_t,
        dst_ptr: *mut uint8_t,
        dst_stride: ptrdiff_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleRowUp2_Linear_16_Any_C(
        src_ptr: *const uint16_t,
        dst_ptr: *mut uint16_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleRowUp2_Bilinear_16_Any_C(
        src_ptr: *const uint16_t,
        src_stride: ptrdiff_t,
        dst_ptr: *mut uint16_t,
        dst_stride: ptrdiff_t,
        dst_width: core::ffi::c_int,
    );
    fn ScaleCols_C(
        dst_ptr: *mut uint8_t,
        src_ptr: *const uint8_t,
        dst_width: core::ffi::c_int,
        x: core::ffi::c_int,
        dx: core::ffi::c_int,
    );
    fn ScaleCols_16_C(
        dst_ptr: *mut uint16_t,
        src_ptr: *const uint16_t,
        dst_width: core::ffi::c_int,
        x: core::ffi::c_int,
        dx: core::ffi::c_int,
    );
    fn ScaleColsUp2_C(
        dst_ptr: *mut uint8_t,
        src_ptr: *const uint8_t,
        dst_width: core::ffi::c_int,
        _: core::ffi::c_int,
        _: core::ffi::c_int,
    );
    fn ScaleColsUp2_16_C(
        dst_ptr: *mut uint16_t,
        src_ptr: *const uint16_t,
        dst_width: core::ffi::c_int,
        _: core::ffi::c_int,
        _: core::ffi::c_int,
    );
    fn ScaleFilterCols_C(
        dst_ptr: *mut uint8_t,
        src_ptr: *const uint8_t,
        dst_width: core::ffi::c_int,
        x: core::ffi::c_int,
        dx: core::ffi::c_int,
    );
    fn ScaleFilterCols_16_C(
        dst_ptr: *mut uint16_t,
        src_ptr: *const uint16_t,
        dst_width: core::ffi::c_int,
        x: core::ffi::c_int,
        dx: core::ffi::c_int,
    );
    fn ScaleFilterCols64_C(
        dst_ptr: *mut uint8_t,
        src_ptr: *const uint8_t,
        dst_width: core::ffi::c_int,
        x32: core::ffi::c_int,
        dx: core::ffi::c_int,
    );
    fn ScaleFilterCols64_16_C(
        dst_ptr: *mut uint16_t,
        src_ptr: *const uint16_t,
        dst_width: core::ffi::c_int,
        x32: core::ffi::c_int,
        dx: core::ffi::c_int,
    );
    fn ScaleAddRow_C(
        src_ptr: *const uint8_t,
        dst_ptr: *mut uint16_t,
        src_width: core::ffi::c_int,
    );
    fn ScaleAddRow_16_C(
        src_ptr: *const uint16_t,
        dst_ptr: *mut uint32_t,
        src_width: core::ffi::c_int,
    );
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type intptr_t = isize;
pub type FilterMode = core::ffi::c_uint;
pub const kFilterBox: FilterMode = 3;
pub const kFilterBilinear: FilterMode = 2;
pub const kFilterLinear: FilterMode = 1;
pub const kFilterNone: FilterMode = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn Abs(mut v: core::ffi::c_int) -> core::ffi::c_int {
    return if v >= 0 as core::ffi::c_int { v } else { -v };
}
#[inline]
unsafe extern "C" fn SumPixels(
    mut iboxwidth: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
) -> uint32_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut x: core::ffi::c_int = 0;
    'c_2070: {
        if iboxwidth > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"iboxwidth > 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                31 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 42],
                    [core::ffi::c_char; 42],
                >(*b"uint32_t SumPixels(int, const uint16_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0 as core::ffi::c_int;
    while x < iboxwidth {
        sum = sum.wrapping_add(*src_ptr.offset(x as isize) as uint32_t);
        x += 1;
    }
    return sum;
}
#[inline]
unsafe extern "C" fn SumPixels_16(
    mut iboxwidth: core::ffi::c_int,
    mut src_ptr: *const uint32_t,
) -> uint32_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut x: core::ffi::c_int = 0;
    'c_4471: {
        if iboxwidth > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"iboxwidth > 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                41 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 45],
                    [core::ffi::c_char; 45],
                >(*b"uint32_t SumPixels_16(int, const uint32_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    x = 0 as core::ffi::c_int;
    while x < iboxwidth {
        sum = sum.wrapping_add(*src_ptr.offset(x as isize));
        x += 1;
    }
    return sum;
}
unsafe extern "C" fn ScaleAddCols2_C(
    mut dst_width: core::ffi::c_int,
    mut boxheight: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut scaletbl: [core::ffi::c_int; 2] = [0; 2];
    let mut minboxwidth: core::ffi::c_int = dx >> 16 as core::ffi::c_int;
    let mut boxwidth: core::ffi::c_int = 0;
    scaletbl[0 as core::ffi::c_int as usize] = 65536 as core::ffi::c_int
        / ((if minboxwidth < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            minboxwidth
        }) * boxheight);
    scaletbl[1 as core::ffi::c_int as usize] = 65536 as core::ffi::c_int
        / ((if (minboxwidth + 1 as core::ffi::c_int) < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            minboxwidth + 1 as core::ffi::c_int
        }) * boxheight);
    i = 0 as core::ffi::c_int;
    while i < dst_width {
        let mut ix: core::ffi::c_int = x >> 16 as core::ffi::c_int;
        x += dx;
        boxwidth = if (x >> 16 as core::ffi::c_int) - ix < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            (x >> 16 as core::ffi::c_int) - ix
        };
        let mut scaletbl_index: core::ffi::c_int = boxwidth - minboxwidth;
        'c_2196: {
            if scaletbl_index == 0 as core::ffi::c_int
                || scaletbl_index == 1 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"(scaletbl_index == 0) || (scaletbl_index == 1)\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    65 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 70],
                        [core::ffi::c_char; 70],
                    >(
                        *b"void ScaleAddCols2_C(int, int, int, int, const uint16_t *, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let fresh2 = dst_ptr;
        dst_ptr = dst_ptr.offset(1);
        *fresh2 = ((SumPixels(boxwidth, src_ptr.offset(ix as isize)))
            .wrapping_mul(scaletbl[scaletbl_index as usize] as uint32_t)
            >> 16 as core::ffi::c_int) as uint8_t;
        i += 1;
    }
}
unsafe extern "C" fn ScaleAddCols2_16_C(
    mut dst_width: core::ffi::c_int,
    mut boxheight: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut src_ptr: *const uint32_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut scaletbl: [core::ffi::c_int; 2] = [0; 2];
    let mut minboxwidth: core::ffi::c_int = dx >> 16 as core::ffi::c_int;
    let mut boxwidth: core::ffi::c_int = 0;
    scaletbl[0 as core::ffi::c_int as usize] = 65536 as core::ffi::c_int
        / ((if minboxwidth < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            minboxwidth
        }) * boxheight);
    scaletbl[1 as core::ffi::c_int as usize] = 65536 as core::ffi::c_int
        / ((if (minboxwidth + 1 as core::ffi::c_int) < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            minboxwidth + 1 as core::ffi::c_int
        }) * boxheight);
    i = 0 as core::ffi::c_int;
    while i < dst_width {
        let mut ix: core::ffi::c_int = x >> 16 as core::ffi::c_int;
        x += dx;
        boxwidth = if (x >> 16 as core::ffi::c_int) - ix < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            (x >> 16 as core::ffi::c_int) - ix
        };
        let mut scaletbl_index: core::ffi::c_int = boxwidth - minboxwidth;
        'c_4588: {
            if scaletbl_index == 0 as core::ffi::c_int
                || scaletbl_index == 1 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"(scaletbl_index == 0) || (scaletbl_index == 1)\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    89 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 74],
                        [core::ffi::c_char; 74],
                    >(
                        *b"void ScaleAddCols2_16_C(int, int, int, int, const uint32_t *, uint16_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let fresh4 = dst_ptr;
        dst_ptr = dst_ptr.offset(1);
        *fresh4 = ((SumPixels_16(boxwidth, src_ptr.offset(ix as isize)))
            .wrapping_mul(scaletbl[scaletbl_index as usize] as uint32_t)
            >> 16 as core::ffi::c_int) as uint16_t;
        i += 1;
    }
}
unsafe extern "C" fn ScaleAddCols0_C(
    mut dst_width: core::ffi::c_int,
    mut boxheight: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint8_t,
) {
    let mut scaleval: core::ffi::c_int = 65536 as core::ffi::c_int / boxheight;
    let mut i: core::ffi::c_int = 0;
    src_ptr = src_ptr.offset((x >> 16 as core::ffi::c_int) as isize);
    i = 0 as core::ffi::c_int;
    while i < dst_width {
        let fresh0 = dst_ptr;
        dst_ptr = dst_ptr.offset(1);
        *fresh0 = (*src_ptr.offset(i as isize) as core::ffi::c_int * scaleval
            >> 16 as core::ffi::c_int) as uint8_t;
        i += 1;
    }
}
unsafe extern "C" fn ScaleAddCols1_C(
    mut dst_width: core::ffi::c_int,
    mut boxheight: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint8_t,
) {
    let mut boxwidth: core::ffi::c_int = if (dx >> 16 as core::ffi::c_int)
        < 1 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        dx >> 16 as core::ffi::c_int
    };
    let mut scaleval: core::ffi::c_int = 65536 as core::ffi::c_int
        / (boxwidth * boxheight);
    let mut i: core::ffi::c_int = 0;
    x >>= 16 as core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < dst_width {
        let fresh1 = dst_ptr;
        dst_ptr = dst_ptr.offset(1);
        *fresh1 = ((SumPixels(boxwidth, src_ptr.offset(x as isize)))
            .wrapping_mul(scaleval as uint32_t) >> 16 as core::ffi::c_int) as uint8_t;
        x += boxwidth;
        i += 1;
    }
}
unsafe extern "C" fn ScaleAddCols1_16_C(
    mut dst_width: core::ffi::c_int,
    mut boxheight: core::ffi::c_int,
    mut x: core::ffi::c_int,
    mut dx: core::ffi::c_int,
    mut src_ptr: *const uint32_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut boxwidth: core::ffi::c_int = if (dx >> 16 as core::ffi::c_int)
        < 1 as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else {
        dx >> 16 as core::ffi::c_int
    };
    let mut scaleval: core::ffi::c_int = 65536 as core::ffi::c_int
        / (boxwidth * boxheight);
    let mut i: core::ffi::c_int = 0;
    i = 0 as core::ffi::c_int;
    while i < dst_width {
        let fresh3 = dst_ptr;
        dst_ptr = dst_ptr.offset(1);
        *fresh3 = ((SumPixels_16(boxwidth, src_ptr.offset(x as isize)))
            .wrapping_mul(scaleval as uint32_t) >> 16 as core::ffi::c_int) as uint16_t;
        x += boxwidth;
        i += 1;
    }
}
unsafe extern "C" fn ScalePlaneBox(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
) -> core::ffi::c_int {
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    let max_y: core::ffi::c_int = src_height << 16 as core::ffi::c_int;
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        kFilterBox,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    let mut row16_mem: *mut core::ffi::c_void = malloc(
        (src_width * 2 as core::ffi::c_int + 63 as core::ffi::c_int) as size_t,
    );
    let mut row16: *mut uint8_t = (row16_mem as intptr_t + 63 as intptr_t
        & !(63 as core::ffi::c_int) as intptr_t) as *mut uint8_t;
    if row16.is_null() {
        return 1 as core::ffi::c_int;
    }
    let mut ScaleAddCols: Option<
        unsafe extern "C" fn(
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            *const uint16_t,
            *mut uint8_t,
        ) -> (),
    > = if dx & 0xffff as core::ffi::c_int != 0 {
        Some(
            ScaleAddCols2_C
                as unsafe extern "C" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *mut uint8_t,
                ) -> (),
        )
    } else if dx != 0x10000 as core::ffi::c_int {
        Some(
            ScaleAddCols1_C
                as unsafe extern "C" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *mut uint8_t,
                ) -> (),
        )
    } else {
        Some(
            ScaleAddCols0_C
                as unsafe extern "C" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint16_t,
                    *mut uint8_t,
                ) -> (),
        )
    };
    let mut ScaleAddRow: Option<
        unsafe extern "C" fn(*const uint8_t, *mut uint16_t, core::ffi::c_int) -> (),
    > = Some(
        ScaleAddRow_C
            as unsafe extern "C" fn(
                *const uint8_t,
                *mut uint16_t,
                core::ffi::c_int,
            ) -> (),
    );
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        let mut boxheight: core::ffi::c_int = 0;
        let mut iy: core::ffi::c_int = y >> 16 as core::ffi::c_int;
        let mut src: *const uint8_t = src_ptr
            .offset((iy as int64_t * src_stride as int64_t) as isize);
        y += dy;
        if y > max_y {
            y = max_y;
        }
        boxheight = if (y >> 16 as core::ffi::c_int) - iy < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            (y >> 16 as core::ffi::c_int) - iy
        };
        memset(
            row16 as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (src_width * 2 as core::ffi::c_int) as size_t,
        );
        k = 0 as core::ffi::c_int;
        while k < boxheight {
            ScaleAddRow
                .expect(
                    "non-null function pointer",
                )(src, row16 as *mut uint16_t, src_width);
            src = src.offset(src_stride as isize);
            k += 1;
        }
        ScaleAddCols
            .expect(
                "non-null function pointer",
            )(dst_width, boxheight, x, dx, row16 as *mut uint16_t, dst_ptr);
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        j += 1;
    }
    free(row16_mem);
    row16 = 0 as *mut uint8_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn ScalePlaneBox_16(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
) -> core::ffi::c_int {
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    let max_y: core::ffi::c_int = src_height << 16 as core::ffi::c_int;
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        kFilterBox,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    let mut row32_mem: *mut core::ffi::c_void = malloc(
        (src_width * 4 as core::ffi::c_int + 63 as core::ffi::c_int) as size_t,
    );
    let mut row32: *mut uint8_t = (row32_mem as intptr_t + 63 as intptr_t
        & !(63 as core::ffi::c_int) as intptr_t) as *mut uint8_t;
    if row32.is_null() {
        return 1 as core::ffi::c_int;
    }
    let mut ScaleAddCols: Option<
        unsafe extern "C" fn(
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            *const uint32_t,
            *mut uint16_t,
        ) -> (),
    > = if dx & 0xffff as core::ffi::c_int != 0 {
        Some(
            ScaleAddCols2_16_C
                as unsafe extern "C" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint32_t,
                    *mut uint16_t,
                ) -> (),
        )
    } else {
        Some(
            ScaleAddCols1_16_C
                as unsafe extern "C" fn(
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    *const uint32_t,
                    *mut uint16_t,
                ) -> (),
        )
    };
    let mut ScaleAddRow: Option<
        unsafe extern "C" fn(*const uint16_t, *mut uint32_t, core::ffi::c_int) -> (),
    > = Some(
        ScaleAddRow_16_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *mut uint32_t,
                core::ffi::c_int,
            ) -> (),
    );
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        let mut boxheight: core::ffi::c_int = 0;
        let mut iy: core::ffi::c_int = y >> 16 as core::ffi::c_int;
        let mut src: *const uint16_t = src_ptr
            .offset((iy as int64_t * src_stride as int64_t) as isize);
        y += dy;
        if y > max_y {
            y = max_y;
        }
        boxheight = if (y >> 16 as core::ffi::c_int) - iy < 1 as core::ffi::c_int {
            1 as core::ffi::c_int
        } else {
            (y >> 16 as core::ffi::c_int) - iy
        };
        memset(
            row32 as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (src_width * 4 as core::ffi::c_int) as size_t,
        );
        k = 0 as core::ffi::c_int;
        while k < boxheight {
            ScaleAddRow
                .expect(
                    "non-null function pointer",
                )(src, row32 as *mut uint32_t, src_width);
            src = src.offset(src_stride as isize);
            k += 1;
        }
        ScaleAddCols
            .expect(
                "non-null function pointer",
            )(dst_width, boxheight, x, dx, row32 as *mut uint32_t, dst_ptr);
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        j += 1;
    }
    free(row32_mem);
    row32 = 0 as *mut uint8_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn ScalePlaneBilinearDown(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row_mem: *mut core::ffi::c_void = malloc(
        (src_width + 63 as core::ffi::c_int) as size_t,
    );
    let mut row: *mut uint8_t = (row_mem as intptr_t + 63 as intptr_t
        & !(63 as core::ffi::c_int) as intptr_t) as *mut uint8_t;
    if row.is_null() {
        return 1 as core::ffi::c_int;
    }
    let max_y: core::ffi::c_int = (src_height - 1 as core::ffi::c_int)
        << 16 as core::ffi::c_int;
    let mut j: core::ffi::c_int = 0;
    let mut ScaleFilterCols: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = if src_width >= 32768 as core::ffi::c_int {
        Some(
            ScaleFilterCols64_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    } else {
        Some(
            ScaleFilterCols_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    };
    let mut InterpolateRow: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        InterpolateRow_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    if y > max_y {
        y = max_y;
    }
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        let mut yi: core::ffi::c_int = y >> 16 as core::ffi::c_int;
        let mut src: *const uint8_t = src_ptr
            .offset((yi as int64_t * src_stride as int64_t) as isize);
        if filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
        {
            ScaleFilterCols
                .expect("non-null function pointer")(dst_ptr, src, dst_width, x, dx);
        } else {
            let mut yf: core::ffi::c_int = y >> 8 as core::ffi::c_int
                & 255 as core::ffi::c_int;
            InterpolateRow
                .expect(
                    "non-null function pointer",
                )(row, src, src_stride as ptrdiff_t, src_width, yf);
            ScaleFilterCols
                .expect("non-null function pointer")(dst_ptr, row, dst_width, x, dx);
        }
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        y += dy;
        if y > max_y {
            y = max_y;
        }
        j += 1;
    }
    free(row_mem);
    row = 0 as *mut uint8_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn ScalePlaneBilinearDown_16(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut row_mem: *mut core::ffi::c_void = malloc(
        (src_width * 2 as core::ffi::c_int + 63 as core::ffi::c_int) as size_t,
    );
    let mut row: *mut uint8_t = (row_mem as intptr_t + 63 as intptr_t
        & !(63 as core::ffi::c_int) as intptr_t) as *mut uint8_t;
    if row.is_null() {
        return 1 as core::ffi::c_int;
    }
    let max_y: core::ffi::c_int = (src_height - 1 as core::ffi::c_int)
        << 16 as core::ffi::c_int;
    let mut j: core::ffi::c_int = 0;
    let mut ScaleFilterCols: Option<
        unsafe extern "C" fn(
            *mut uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = if src_width >= 32768 as core::ffi::c_int {
        Some(
            ScaleFilterCols64_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    } else {
        Some(
            ScaleFilterCols_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    };
    let mut InterpolateRow: Option<
        unsafe extern "C" fn(
            *mut uint16_t,
            *const uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        InterpolateRow_16_C
            as unsafe extern "C" fn(
                *mut uint16_t,
                *const uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    if y > max_y {
        y = max_y;
    }
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        let mut yi: core::ffi::c_int = y >> 16 as core::ffi::c_int;
        let mut src: *const uint16_t = src_ptr
            .offset((yi as int64_t * src_stride as int64_t) as isize);
        if filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
        {
            ScaleFilterCols
                .expect("non-null function pointer")(dst_ptr, src, dst_width, x, dx);
        } else {
            let mut yf: core::ffi::c_int = y >> 8 as core::ffi::c_int
                & 255 as core::ffi::c_int;
            InterpolateRow
                .expect(
                    "non-null function pointer",
                )(row as *mut uint16_t, src, src_stride as ptrdiff_t, src_width, yf);
            ScaleFilterCols
                .expect(
                    "non-null function pointer",
                )(dst_ptr, row as *mut uint16_t, dst_width, x, dx);
        }
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        y += dy;
        if y > max_y {
            y = max_y;
        }
        j += 1;
    }
    free(row_mem);
    row = 0 as *mut uint8_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn ScalePlaneBilinearUp(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    let mut j: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    let max_y: core::ffi::c_int = (src_height - 1 as core::ffi::c_int)
        << 16 as core::ffi::c_int;
    let mut InterpolateRow: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        InterpolateRow_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    let mut ScaleFilterCols: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = if filtering as core::ffi::c_uint != 0 {
        Some(
            ScaleFilterCols_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    } else {
        Some(
            ScaleCols_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    };
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    if filtering as core::ffi::c_uint != 0 && src_width >= 32768 as core::ffi::c_int {
        ScaleFilterCols = Some(
            ScaleFilterCols64_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if filtering as u64 == 0 && src_width * 2 as core::ffi::c_int == dst_width
        && x < 0x8000 as core::ffi::c_int
    {
        ScaleFilterCols = Some(
            ScaleColsUp2_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if y > max_y {
        y = max_y;
    }
    let mut yi: core::ffi::c_int = y >> 16 as core::ffi::c_int;
    let mut src: *const uint8_t = src_ptr
        .offset((yi as int64_t * src_stride as int64_t) as isize);
    let row_size: core::ffi::c_int = dst_width + 31 as core::ffi::c_int
        & !(31 as core::ffi::c_int);
    let mut row_mem: *mut core::ffi::c_void = malloc(
        (row_size * 2 as core::ffi::c_int + 63 as core::ffi::c_int) as size_t,
    );
    let mut row: *mut uint8_t = (row_mem as intptr_t + 63 as intptr_t
        & !(63 as core::ffi::c_int) as intptr_t) as *mut uint8_t;
    if row.is_null() {
        return 1 as core::ffi::c_int;
    }
    let mut rowptr: *mut uint8_t = row;
    let mut rowstride: core::ffi::c_int = row_size;
    let mut lasty: core::ffi::c_int = yi;
    ScaleFilterCols.expect("non-null function pointer")(rowptr, src, dst_width, x, dx);
    if src_height > 1 as core::ffi::c_int {
        src = src.offset(src_stride as isize);
    }
    ScaleFilterCols
        .expect(
            "non-null function pointer",
        )(rowptr.offset(rowstride as isize), src, dst_width, x, dx);
    if src_height > 2 as core::ffi::c_int {
        src = src.offset(src_stride as isize);
    }
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        yi = y >> 16 as core::ffi::c_int;
        if yi != lasty {
            if y > max_y {
                y = max_y;
                yi = y >> 16 as core::ffi::c_int;
                src = src_ptr.offset((yi as int64_t * src_stride as int64_t) as isize);
            }
            if yi != lasty {
                ScaleFilterCols
                    .expect("non-null function pointer")(rowptr, src, dst_width, x, dx);
                rowptr = rowptr.offset(rowstride as isize);
                rowstride = -rowstride;
                lasty = yi;
                if (y + 65536 as core::ffi::c_int) < max_y {
                    src = src.offset(src_stride as isize);
                }
            }
        }
        if filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
        {
            InterpolateRow
                .expect(
                    "non-null function pointer",
                )(dst_ptr, rowptr, 0 as ptrdiff_t, dst_width, 0 as core::ffi::c_int);
        } else {
            let mut yf: core::ffi::c_int = y >> 8 as core::ffi::c_int
                & 255 as core::ffi::c_int;
            InterpolateRow
                .expect(
                    "non-null function pointer",
                )(dst_ptr, rowptr, rowstride as ptrdiff_t, dst_width, yf);
        }
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        y += dy;
        j += 1;
    }
    free(row_mem);
    row = 0 as *mut uint8_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn ScalePlaneUp2_Linear(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
) {
    let mut ScaleRowUp: Option<
        unsafe extern "C" fn(*const uint8_t, *mut uint8_t, core::ffi::c_int) -> (),
    > = Some(
        ScaleRowUp2_Linear_Any_C
            as unsafe extern "C" fn(*const uint8_t, *mut uint8_t, core::ffi::c_int) -> (),
    );
    let mut i: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut dy: core::ffi::c_int = 0;
    'c_1678: {
        if src_width == (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_width == ((dst_width + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                483 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 84],
                    [core::ffi::c_char; 84],
                >(
                    *b"void ScalePlaneUp2_Linear(int, int, int, int, int, int, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dst_height == 1 as core::ffi::c_int {
        ScaleRowUp
            .expect(
                "non-null function pointer",
            )(
            src_ptr
                .offset(
                    (((src_height - 1 as core::ffi::c_int) / 2 as core::ffi::c_int)
                        as int64_t * src_stride as int64_t) as isize,
                ),
            dst_ptr,
            dst_width,
        );
    } else {
        dy = FixedDiv_C(
            src_height - 1 as core::ffi::c_int,
            dst_height - 1 as core::ffi::c_int,
        );
        y = ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) - 1 as core::ffi::c_int;
        i = 0 as core::ffi::c_int;
        while i < dst_height {
            ScaleRowUp
                .expect(
                    "non-null function pointer",
                )(
                src_ptr
                    .offset(
                        ((y >> 16 as core::ffi::c_int) as int64_t
                            * src_stride as int64_t) as isize,
                    ),
                dst_ptr,
                dst_width,
            );
            dst_ptr = dst_ptr.offset(dst_stride as isize);
            y += dy;
            i += 1;
        }
    };
}
unsafe extern "C" fn ScalePlaneUp2_Bilinear(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
) {
    let mut Scale2RowUp: Option<
        unsafe extern "C" fn(
            *const uint8_t,
            ptrdiff_t,
            *mut uint8_t,
            ptrdiff_t,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        ScaleRowUp2_Bilinear_Any_C
            as unsafe extern "C" fn(
                *const uint8_t,
                ptrdiff_t,
                *mut uint8_t,
                ptrdiff_t,
                core::ffi::c_int,
            ) -> (),
    );
    let mut x: core::ffi::c_int = 0;
    'c_1429: {
        if src_width == (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_width == ((dst_width + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                518 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 86],
                    [core::ffi::c_char; 86],
                >(
                    *b"void ScalePlaneUp2_Bilinear(int, int, int, int, int, int, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_1367: {
        if src_height == (dst_height + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_height == ((dst_height + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                519 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 86],
                    [core::ffi::c_char; 86],
                >(
                    *b"void ScalePlaneUp2_Bilinear(int, int, int, int, int, int, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    Scale2RowUp
        .expect(
            "non-null function pointer",
        )(src_ptr, 0 as ptrdiff_t, dst_ptr, 0 as ptrdiff_t, dst_width);
    dst_ptr = dst_ptr.offset(dst_stride as isize);
    x = 0 as core::ffi::c_int;
    while x < src_height - 1 as core::ffi::c_int {
        Scale2RowUp
            .expect(
                "non-null function pointer",
            )(
            src_ptr,
            src_stride as ptrdiff_t,
            dst_ptr,
            dst_stride as ptrdiff_t,
            dst_width,
        );
        src_ptr = src_ptr.offset(src_stride as isize);
        dst_ptr = dst_ptr.offset((2 as core::ffi::c_int * dst_stride) as isize);
        x += 1;
    }
    if dst_height & 1 as core::ffi::c_int == 0 {
        Scale2RowUp
            .expect(
                "non-null function pointer",
            )(src_ptr, 0 as ptrdiff_t, dst_ptr, 0 as ptrdiff_t, dst_width);
    }
}
unsafe extern "C" fn ScalePlaneUp2_12_Linear(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut ScaleRowUp: Option<
        unsafe extern "C" fn(*const uint16_t, *mut uint16_t, core::ffi::c_int) -> (),
    > = Some(
        ScaleRowUp2_Linear_16_Any_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *mut uint16_t,
                core::ffi::c_int,
            ) -> (),
    );
    let mut i: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut dy: core::ffi::c_int = 0;
    'c_5570: {
        if src_width == (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_width == ((dst_width + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                556 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 89],
                    [core::ffi::c_char; 89],
                >(
                    *b"void ScalePlaneUp2_12_Linear(int, int, int, int, int, int, const uint16_t *, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dst_height == 1 as core::ffi::c_int {
        ScaleRowUp
            .expect(
                "non-null function pointer",
            )(
            src_ptr
                .offset(
                    (((src_height - 1 as core::ffi::c_int) / 2 as core::ffi::c_int)
                        as int64_t * src_stride as int64_t) as isize,
                ),
            dst_ptr,
            dst_width,
        );
    } else {
        dy = FixedDiv_C(
            src_height - 1 as core::ffi::c_int,
            dst_height - 1 as core::ffi::c_int,
        );
        y = ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) - 1 as core::ffi::c_int;
        i = 0 as core::ffi::c_int;
        while i < dst_height {
            ScaleRowUp
                .expect(
                    "non-null function pointer",
                )(
                src_ptr
                    .offset(
                        ((y >> 16 as core::ffi::c_int) as int64_t
                            * src_stride as int64_t) as isize,
                    ),
                dst_ptr,
                dst_width,
            );
            dst_ptr = dst_ptr.offset(dst_stride as isize);
            y += dy;
            i += 1;
        }
    };
}
unsafe extern "C" fn ScalePlaneUp2_12_Bilinear(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut Scale2RowUp: Option<
        unsafe extern "C" fn(
            *const uint16_t,
            ptrdiff_t,
            *mut uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        ScaleRowUp2_Bilinear_16_Any_C
            as unsafe extern "C" fn(
                *const uint16_t,
                ptrdiff_t,
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
            ) -> (),
    );
    let mut x: core::ffi::c_int = 0;
    'c_5338: {
        if src_width == (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_width == ((dst_width + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                592 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 91],
                    [core::ffi::c_char; 91],
                >(
                    *b"void ScalePlaneUp2_12_Bilinear(int, int, int, int, int, int, const uint16_t *, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_5288: {
        if src_height == (dst_height + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_height == ((dst_height + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                593 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 91],
                    [core::ffi::c_char; 91],
                >(
                    *b"void ScalePlaneUp2_12_Bilinear(int, int, int, int, int, int, const uint16_t *, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    Scale2RowUp
        .expect(
            "non-null function pointer",
        )(src_ptr, 0 as ptrdiff_t, dst_ptr, 0 as ptrdiff_t, dst_width);
    dst_ptr = dst_ptr.offset(dst_stride as isize);
    x = 0 as core::ffi::c_int;
    while x < src_height - 1 as core::ffi::c_int {
        Scale2RowUp
            .expect(
                "non-null function pointer",
            )(
            src_ptr,
            src_stride as ptrdiff_t,
            dst_ptr,
            dst_stride as ptrdiff_t,
            dst_width,
        );
        src_ptr = src_ptr.offset(src_stride as isize);
        dst_ptr = dst_ptr.offset((2 as core::ffi::c_int * dst_stride) as isize);
        x += 1;
    }
    if dst_height & 1 as core::ffi::c_int == 0 {
        Scale2RowUp
            .expect(
                "non-null function pointer",
            )(src_ptr, 0 as ptrdiff_t, dst_ptr, 0 as ptrdiff_t, dst_width);
    }
}
unsafe extern "C" fn ScalePlaneUp2_16_Linear(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut ScaleRowUp: Option<
        unsafe extern "C" fn(*const uint16_t, *mut uint16_t, core::ffi::c_int) -> (),
    > = Some(
        ScaleRowUp2_Linear_16_Any_C
            as unsafe extern "C" fn(
                *const uint16_t,
                *mut uint16_t,
                core::ffi::c_int,
            ) -> (),
    );
    let mut i: core::ffi::c_int = 0;
    let mut y: core::ffi::c_int = 0;
    let mut dy: core::ffi::c_int = 0;
    'c_4152: {
        if src_width == (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_width == ((dst_width + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                623 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 89],
                    [core::ffi::c_char; 89],
                >(
                    *b"void ScalePlaneUp2_16_Linear(int, int, int, int, int, int, const uint16_t *, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dst_height == 1 as core::ffi::c_int {
        ScaleRowUp
            .expect(
                "non-null function pointer",
            )(
            src_ptr
                .offset(
                    (((src_height - 1 as core::ffi::c_int) / 2 as core::ffi::c_int)
                        as int64_t * src_stride as int64_t) as isize,
                ),
            dst_ptr,
            dst_width,
        );
    } else {
        dy = FixedDiv_C(
            src_height - 1 as core::ffi::c_int,
            dst_height - 1 as core::ffi::c_int,
        );
        y = ((1 as core::ffi::c_int) << 15 as core::ffi::c_int) - 1 as core::ffi::c_int;
        i = 0 as core::ffi::c_int;
        while i < dst_height {
            ScaleRowUp
                .expect(
                    "non-null function pointer",
                )(
                src_ptr
                    .offset(
                        ((y >> 16 as core::ffi::c_int) as int64_t
                            * src_stride as int64_t) as isize,
                    ),
                dst_ptr,
                dst_width,
            );
            dst_ptr = dst_ptr.offset(dst_stride as isize);
            y += dy;
            i += 1;
        }
    };
}
unsafe extern "C" fn ScalePlaneUp2_16_Bilinear(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut Scale2RowUp: Option<
        unsafe extern "C" fn(
            *const uint16_t,
            ptrdiff_t,
            *mut uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        ScaleRowUp2_Bilinear_16_Any_C
            as unsafe extern "C" fn(
                *const uint16_t,
                ptrdiff_t,
                *mut uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
            ) -> (),
    );
    let mut x: core::ffi::c_int = 0;
    'c_3911: {
        if src_width == (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_width == ((dst_width + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                654 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 91],
                    [core::ffi::c_char; 91],
                >(
                    *b"void ScalePlaneUp2_16_Bilinear(int, int, int, int, int, int, const uint16_t *, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    'c_3861: {
        if src_height == (dst_height + 1 as core::ffi::c_int) / 2 as core::ffi::c_int
        {} else {
            __assert_fail(
                b"src_height == ((dst_height + 1) / 2)\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/third_party/libyuv/source/scale.c\0"
                    as *const u8 as *const core::ffi::c_char,
                655 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 91],
                    [core::ffi::c_char; 91],
                >(
                    *b"void ScalePlaneUp2_16_Bilinear(int, int, int, int, int, int, const uint16_t *, uint16_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    Scale2RowUp
        .expect(
            "non-null function pointer",
        )(src_ptr, 0 as ptrdiff_t, dst_ptr, 0 as ptrdiff_t, dst_width);
    dst_ptr = dst_ptr.offset(dst_stride as isize);
    x = 0 as core::ffi::c_int;
    while x < src_height - 1 as core::ffi::c_int {
        Scale2RowUp
            .expect(
                "non-null function pointer",
            )(
            src_ptr,
            src_stride as ptrdiff_t,
            dst_ptr,
            dst_stride as ptrdiff_t,
            dst_width,
        );
        src_ptr = src_ptr.offset(src_stride as isize);
        dst_ptr = dst_ptr.offset((2 as core::ffi::c_int * dst_stride) as isize);
        x += 1;
    }
    if dst_height & 1 as core::ffi::c_int == 0 {
        Scale2RowUp
            .expect(
                "non-null function pointer",
            )(src_ptr, 0 as ptrdiff_t, dst_ptr, 0 as ptrdiff_t, dst_width);
    }
}
unsafe extern "C" fn ScalePlaneBilinearUp_16(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    let mut j: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    let max_y: core::ffi::c_int = (src_height - 1 as core::ffi::c_int)
        << 16 as core::ffi::c_int;
    let mut InterpolateRow: Option<
        unsafe extern "C" fn(
            *mut uint16_t,
            *const uint16_t,
            ptrdiff_t,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        InterpolateRow_16_C
            as unsafe extern "C" fn(
                *mut uint16_t,
                *const uint16_t,
                ptrdiff_t,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    let mut ScaleFilterCols: Option<
        unsafe extern "C" fn(
            *mut uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = if filtering as core::ffi::c_uint != 0 {
        Some(
            ScaleFilterCols_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    } else {
        Some(
            ScaleCols_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
    };
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    if filtering as core::ffi::c_uint != 0 && src_width >= 32768 as core::ffi::c_int {
        ScaleFilterCols = Some(
            ScaleFilterCols64_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if filtering as u64 == 0 && src_width * 2 as core::ffi::c_int == dst_width
        && x < 0x8000 as core::ffi::c_int
    {
        ScaleFilterCols = Some(
            ScaleColsUp2_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    if y > max_y {
        y = max_y;
    }
    let mut yi: core::ffi::c_int = y >> 16 as core::ffi::c_int;
    let mut src: *const uint16_t = src_ptr
        .offset((yi as int64_t * src_stride as int64_t) as isize);
    let row_size: core::ffi::c_int = dst_width + 31 as core::ffi::c_int
        & !(31 as core::ffi::c_int);
    let mut row_mem: *mut core::ffi::c_void = malloc(
        (row_size * 4 as core::ffi::c_int + 63 as core::ffi::c_int) as size_t,
    );
    let mut row: *mut uint8_t = (row_mem as intptr_t + 63 as intptr_t
        & !(63 as core::ffi::c_int) as intptr_t) as *mut uint8_t;
    let mut rowstride: core::ffi::c_int = row_size;
    let mut lasty: core::ffi::c_int = yi;
    let mut rowptr: *mut uint16_t = row as *mut uint16_t;
    if row.is_null() {
        return 1 as core::ffi::c_int;
    }
    ScaleFilterCols.expect("non-null function pointer")(rowptr, src, dst_width, x, dx);
    if src_height > 1 as core::ffi::c_int {
        src = src.offset(src_stride as isize);
    }
    ScaleFilterCols
        .expect(
            "non-null function pointer",
        )(rowptr.offset(rowstride as isize), src, dst_width, x, dx);
    if src_height > 2 as core::ffi::c_int {
        src = src.offset(src_stride as isize);
    }
    j = 0 as core::ffi::c_int;
    while j < dst_height {
        yi = y >> 16 as core::ffi::c_int;
        if yi != lasty {
            if y > max_y {
                y = max_y;
                yi = y >> 16 as core::ffi::c_int;
                src = src_ptr.offset((yi as int64_t * src_stride as int64_t) as isize);
            }
            if yi != lasty {
                ScaleFilterCols
                    .expect("non-null function pointer")(rowptr, src, dst_width, x, dx);
                rowptr = rowptr.offset(rowstride as isize);
                rowstride = -rowstride;
                lasty = yi;
                if (y + 65536 as core::ffi::c_int) < max_y {
                    src = src.offset(src_stride as isize);
                }
            }
        }
        if filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
        {
            InterpolateRow
                .expect(
                    "non-null function pointer",
                )(dst_ptr, rowptr, 0 as ptrdiff_t, dst_width, 0 as core::ffi::c_int);
        } else {
            let mut yf: core::ffi::c_int = y >> 8 as core::ffi::c_int
                & 255 as core::ffi::c_int;
            InterpolateRow
                .expect(
                    "non-null function pointer",
                )(dst_ptr, rowptr, rowstride as ptrdiff_t, dst_width, yf);
        }
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        y += dy;
        j += 1;
    }
    free(row_mem);
    row = 0 as *mut uint8_t;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn ScalePlaneSimple(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint8_t,
    mut dst_ptr: *mut uint8_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut ScaleCols: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        ScaleCols_C
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        kFilterNone,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    if src_width * 2 as core::ffi::c_int == dst_width && x < 0x8000 as core::ffi::c_int {
        ScaleCols = Some(
            ScaleColsUp2_C
                as unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint8_t,
                    *const uint8_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    i = 0 as core::ffi::c_int;
    while i < dst_height {
        ScaleCols
            .expect(
                "non-null function pointer",
            )(
            dst_ptr,
            src_ptr
                .offset(
                    ((y >> 16 as core::ffi::c_int) as int64_t * src_stride as int64_t)
                        as isize,
                ),
            dst_width,
            x,
            dx,
        );
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        y += dy;
        i += 1;
    }
}
unsafe extern "C" fn ScalePlaneSimple_16(
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut src_stride: core::ffi::c_int,
    mut dst_stride: core::ffi::c_int,
    mut src_ptr: *const uint16_t,
    mut dst_ptr: *mut uint16_t,
) {
    let mut i: core::ffi::c_int = 0;
    let mut ScaleCols: Option<
        unsafe extern "C" fn(
            *mut uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    > = Some(
        ScaleCols_16_C
            as unsafe extern "C" fn(
                *mut uint16_t,
                *const uint16_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    );
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dx: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
    ScaleSlope(
        src_width,
        src_height,
        dst_width,
        dst_height,
        kFilterNone,
        &mut x,
        &mut y,
        &mut dx,
        &mut dy,
    );
    src_width = Abs(src_width);
    if src_width * 2 as core::ffi::c_int == dst_width && x < 0x8000 as core::ffi::c_int {
        ScaleCols = Some(
            ScaleColsUp2_16_C
                as unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut uint16_t,
                    *const uint16_t,
                    core::ffi::c_int,
                    core::ffi::c_int,
                    core::ffi::c_int,
                ) -> (),
            >;
    }
    i = 0 as core::ffi::c_int;
    while i < dst_height {
        ScaleCols
            .expect(
                "non-null function pointer",
            )(
            dst_ptr,
            src_ptr
                .offset(
                    ((y >> 16 as core::ffi::c_int) as int64_t * src_stride as int64_t)
                        as isize,
                ),
            dst_width,
            x,
            dx,
        );
        dst_ptr = dst_ptr.offset(dst_stride as isize);
        y += dy;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ScalePlane(
    mut src: *const uint8_t,
    mut src_stride: core::ffi::c_int,
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst: *mut uint8_t,
    mut dst_stride: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    filtering = ScaleFilterReduce(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
    );
    if src_height < 0 as core::ffi::c_int {
        src_height = -src_height;
        src = src
            .offset(
                ((src_height - 1 as core::ffi::c_int) as int64_t * src_stride as int64_t)
                    as isize,
            );
        src_stride = -src_stride;
    }
    if dst_width == src_width && dst_height == src_height {
        CopyPlane(src, src_stride, dst, dst_stride, dst_width, dst_height);
        return 0 as core::ffi::c_int;
    }
    if dst_width == src_width
        && filtering as core::ffi::c_uint
            != kFilterBox as core::ffi::c_int as core::ffi::c_uint
    {
        let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        if dst_height <= src_height {
            dy = FixedDiv_C(src_height, dst_height);
            y = if dy < 0 as core::ffi::c_int {
                -((-dy >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int))
            } else {
                (dy >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int)
            };
        } else if src_height > 1 as core::ffi::c_int
            && dst_height > 1 as core::ffi::c_int
        {
            dy = FixedDiv1_C(src_height, dst_height);
        }
        ScalePlaneVertical(
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
            0 as core::ffi::c_int,
            y,
            dy,
            1 as core::ffi::c_int,
            filtering,
        );
        return 0 as core::ffi::c_int;
    }
    if filtering as core::ffi::c_uint
        == kFilterBox as core::ffi::c_int as core::ffi::c_uint
        && (dst_height * 2 as core::ffi::c_int) < src_height
    {
        return ScalePlaneBox(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
    }
    if (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_width
        && filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
    {
        ScalePlaneUp2_Linear(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
        return 0 as core::ffi::c_int;
    }
    if (dst_height + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_height
        && (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_width
        && (filtering as core::ffi::c_uint
            == kFilterBilinear as core::ffi::c_int as core::ffi::c_uint
            || filtering as core::ffi::c_uint
                == kFilterBox as core::ffi::c_int as core::ffi::c_uint)
    {
        ScalePlaneUp2_Bilinear(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
        return 0 as core::ffi::c_int;
    }
    if filtering as core::ffi::c_uint != 0 && dst_height > src_height {
        return ScalePlaneBilinearUp(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
            filtering,
        );
    }
    if filtering as u64 != 0 {
        return ScalePlaneBilinearDown(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
            filtering,
        );
    }
    ScalePlaneSimple(
        src_width,
        src_height,
        dst_width,
        dst_height,
        src_stride,
        dst_stride,
        src,
        dst,
    );
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ScalePlane_16(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    filtering = ScaleFilterReduce(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
    );
    if src_height < 0 as core::ffi::c_int {
        src_height = -src_height;
        src = src
            .offset(
                ((src_height - 1 as core::ffi::c_int) as int64_t * src_stride as int64_t)
                    as isize,
            );
        src_stride = -src_stride;
    }
    if dst_width == src_width && dst_height == src_height {
        CopyPlane_16(src, src_stride, dst, dst_stride, dst_width, dst_height);
        return 0 as core::ffi::c_int;
    }
    if dst_width == src_width
        && filtering as core::ffi::c_uint
            != kFilterBox as core::ffi::c_int as core::ffi::c_uint
    {
        let mut dy: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        if dst_height <= src_height {
            dy = FixedDiv_C(src_height, dst_height);
            y = if dy < 0 as core::ffi::c_int {
                -((-dy >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int))
            } else {
                (dy >> 1 as core::ffi::c_int) + -(32768 as core::ffi::c_int)
            };
        } else if src_height > 1 as core::ffi::c_int
            && dst_height > 1 as core::ffi::c_int
        {
            dy = FixedDiv1_C(src_height, dst_height);
        }
        ScalePlaneVertical_16(
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
            0 as core::ffi::c_int,
            y,
            dy,
            1 as core::ffi::c_int,
            filtering,
        );
        return 0 as core::ffi::c_int;
    }
    if filtering as core::ffi::c_uint
        == kFilterBox as core::ffi::c_int as core::ffi::c_uint
        && (dst_height * 2 as core::ffi::c_int) < src_height
    {
        return ScalePlaneBox_16(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
    }
    if (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_width
        && filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
    {
        ScalePlaneUp2_16_Linear(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
        return 0 as core::ffi::c_int;
    }
    if (dst_height + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_height
        && (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_width
        && (filtering as core::ffi::c_uint
            == kFilterBilinear as core::ffi::c_int as core::ffi::c_uint
            || filtering as core::ffi::c_uint
                == kFilterBox as core::ffi::c_int as core::ffi::c_uint)
    {
        ScalePlaneUp2_16_Bilinear(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
        return 0 as core::ffi::c_int;
    }
    if filtering as core::ffi::c_uint != 0 && dst_height > src_height {
        return ScalePlaneBilinearUp_16(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
            filtering,
        );
    }
    if filtering as u64 != 0 {
        return ScalePlaneBilinearDown_16(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
            filtering,
        );
    }
    ScalePlaneSimple_16(
        src_width,
        src_height,
        dst_width,
        dst_height,
        src_stride,
        dst_stride,
        src,
        dst,
    );
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ScalePlane_12(
    mut src: *const uint16_t,
    mut src_stride: core::ffi::c_int,
    mut src_width: core::ffi::c_int,
    mut src_height: core::ffi::c_int,
    mut dst: *mut uint16_t,
    mut dst_stride: core::ffi::c_int,
    mut dst_width: core::ffi::c_int,
    mut dst_height: core::ffi::c_int,
    mut filtering: FilterMode,
) -> core::ffi::c_int {
    filtering = ScaleFilterReduce(
        src_width,
        src_height,
        dst_width,
        dst_height,
        filtering,
    );
    if src_height < 0 as core::ffi::c_int {
        src_height = -src_height;
        src = src
            .offset(
                ((src_height - 1 as core::ffi::c_int) as int64_t * src_stride as int64_t)
                    as isize,
            );
        src_stride = -src_stride;
    }
    if (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_width
        && filtering as core::ffi::c_uint
            == kFilterLinear as core::ffi::c_int as core::ffi::c_uint
    {
        ScalePlaneUp2_12_Linear(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
        return 0 as core::ffi::c_int;
    }
    if (dst_height + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_height
        && (dst_width + 1 as core::ffi::c_int) / 2 as core::ffi::c_int == src_width
        && (filtering as core::ffi::c_uint
            == kFilterBilinear as core::ffi::c_int as core::ffi::c_uint
            || filtering as core::ffi::c_uint
                == kFilterBox as core::ffi::c_int as core::ffi::c_uint)
    {
        ScalePlaneUp2_12_Bilinear(
            src_width,
            src_height,
            dst_width,
            dst_height,
            src_stride,
            dst_stride,
            src,
            dst,
        );
        return 0 as core::ffi::c_int;
    }
    return ScalePlane_16(
        src,
        src_stride,
        src_width,
        src_height,
        dst,
        dst_stride,
        dst_width,
        dst_height,
        filtering,
    );
}
