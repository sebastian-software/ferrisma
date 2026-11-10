/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/encoder/cnn.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type AVxWorkerImpl;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_get_worker_interface() -> *const AVxWorkerInterface;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type size_t = usize;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const SIGMOID: C2RustUnnamed = 3;
pub const SOFTSIGN: C2RustUnnamed = 2;
pub const RELU: C2RustUnnamed = 1;
pub const NONE: C2RustUnnamed = 0;
pub type ACTIVATION = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNN_CONFIG {
    pub num_layers: core::ffi::c_int,
    pub is_residue: core::ffi::c_int,
    pub ext_width: core::ffi::c_int,
    pub ext_height: core::ffi::c_int,
    pub strict_bounds: core::ffi::c_int,
    pub layer_config: [CNN_LAYER_CONFIG; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNN_LAYER_CONFIG {
    pub in_channels: core::ffi::c_int,
    pub filter_width: core::ffi::c_int,
    pub filter_height: core::ffi::c_int,
    pub out_channels: core::ffi::c_int,
    pub skip_width: core::ffi::c_int,
    pub skip_height: core::ffi::c_int,
    pub maxpool: core::ffi::c_int,
    pub weights: *const core::ffi::c_float,
    pub bias: *const core::ffi::c_float,
    pub pad: PADDING_TYPE,
    pub activation: ACTIVATION,
    pub deconvolve: core::ffi::c_int,
    pub branch: core::ffi::c_int,
    pub branch_copy_type: BRANCH_COPY,
    pub branch_combine_type: BRANCH_COMBINE,
    pub branch_config: CNN_BRANCH_CONFIG,
    pub bn_params: CNN_BATCHNORM_PARAMS,
    pub output_num: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNN_BATCHNORM_PARAMS {
    pub bn_gamma: *const core::ffi::c_float,
    pub bn_beta: *const core::ffi::c_float,
    pub bn_mean: *const core::ffi::c_float,
    pub bn_std: *const core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNN_BRANCH_CONFIG {
    pub input_to_branches: core::ffi::c_int,
    pub channels_to_copy: core::ffi::c_int,
    pub branches_to_combine: core::ffi::c_int,
}
pub type BRANCH_COMBINE = uint8_t;
pub type BRANCH_COPY = uint8_t;
pub type PADDING_TYPE = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNN_THREAD_DATA {
    pub num_workers: core::ffi::c_int,
    pub workers: *mut AVxWorker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorker {
    pub impl_: *mut AVxWorkerImpl,
    pub status_: AVxWorkerStatus,
    pub thread_name: *const core::ffi::c_char,
    pub hook: AVxWorkerHook,
    pub data1: *mut core::ffi::c_void,
    pub data2: *mut core::ffi::c_void,
    pub had_error: core::ffi::c_int,
}
pub type AVxWorkerHook = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> core::ffi::c_int,
>;
pub type AVxWorkerStatus = core::ffi::c_uint;
pub const AVX_WORKER_STATUS_WORKING: AVxWorkerStatus = 2;
pub const AVX_WORKER_STATUS_OK: AVxWorkerStatus = 1;
pub const AVX_WORKER_STATUS_NOT_OK: AVxWorkerStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CNN_MULTI_OUT {
    pub num_outputs: core::ffi::c_int,
    pub output_channels: *const core::ffi::c_int,
    pub output_strides: *const core::ffi::c_int,
    pub output_buffer: *mut *mut core::ffi::c_float,
}
pub const PADDING_VALID: C2RustUnnamed_0 = 2;
pub const PADDING_SAME_REPLICATE: C2RustUnnamed_0 = 1;
pub const PADDING_SAME_ZERO: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TENSOR {
    pub allocsize: core::ffi::c_int,
    pub channels: core::ffi::c_int,
    pub width: core::ffi::c_int,
    pub height: core::ffi::c_int,
    pub stride: core::ffi::c_int,
    pub buf: [*mut core::ffi::c_float; 256],
}
pub const BRANCH_COMBINED: C2RustUnnamed_1 = 3;
pub const BRANCH_CAT: C2RustUnnamed_2 = 2;
pub const BRANCH_ADD: C2RustUnnamed_2 = 1;
pub const BRANCH_OUTPUT: C2RustUnnamed_1 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVxWorkerInterface {
    pub init: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub reset: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub sync: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> core::ffi::c_int>,
    pub launch: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub execute: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
    pub end: Option<unsafe extern "C-unwind" fn(*mut AVxWorker) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONVOLVE_OPS {
    pub input: *mut *const core::ffi::c_float,
    pub in_width: core::ffi::c_int,
    pub in_height: core::ffi::c_int,
    pub in_stride: core::ffi::c_int,
    pub layer_config: *const CNN_LAYER_CONFIG,
    pub output: *mut *mut core::ffi::c_float,
    pub out_stride: core::ffi::c_int,
    pub start_idx: core::ffi::c_int,
    pub th_step: core::ffi::c_int,
}
pub const BRANCH_INPUT: C2RustUnnamed_1 = 1;
pub const BRANCH_NOC: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const BRANCH_NO_COPY: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const CNN_MAX_BRANCHES: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn softsign(mut x: core::ffi::c_float) -> core::ffi::c_float {
    return x / (fabsf(x) + 1.0f32);
}
#[inline]
unsafe extern "C-unwind" fn relu(mut x: core::ffi::c_float) -> core::ffi::c_float {
    return if x < 0 as core::ffi::c_int as core::ffi::c_float {
        0 as core::ffi::c_int as core::ffi::c_float
    } else {
        x
    };
}
unsafe extern "C-unwind" fn init_tensor(mut tensor: *mut TENSOR) {
    memset(
        tensor as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<TENSOR>() as size_t,
    );
}
unsafe extern "C-unwind" fn free_tensor(mut tensor: *mut TENSOR) {
    if (*tensor).allocsize != 0 {
        aom_free(
            (*tensor).buf[0 as core::ffi::c_int as usize] as *mut core::ffi::c_void,
        );
        (*tensor).buf[0 as core::ffi::c_int as usize] = 0 as *mut core::ffi::c_float;
        (*tensor).allocsize = 0 as core::ffi::c_int;
    }
}
unsafe extern "C-unwind" fn realloc_tensor(
    mut tensor: *mut TENSOR,
    mut channels: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
) -> bool {
    let newallocsize: core::ffi::c_int = channels * width * height;
    if (*tensor).allocsize < newallocsize {
        free_tensor(tensor);
        (*tensor).buf[0 as core::ffi::c_int as usize] = aom_malloc(
            (::core::mem::size_of::<core::ffi::c_float>() as size_t)
                .wrapping_mul(newallocsize as size_t),
        ) as *mut core::ffi::c_float;
        if ((*tensor).buf[0 as core::ffi::c_int as usize]).is_null() {
            return false_0 != 0;
        }
        (*tensor).allocsize = newallocsize;
    }
    (*tensor).width = width;
    (*tensor).height = height;
    (*tensor).stride = width;
    (*tensor).channels = channels;
    let mut c: core::ffi::c_int = 1 as core::ffi::c_int;
    while c < channels {
        (*tensor).buf[c as usize] = &raw mut *(*((*tensor).buf)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .offset((c * width * height) as isize) as *mut core::ffi::c_float;
        c += 1;
    }
    return true_0 != 0;
}
unsafe extern "C-unwind" fn copy_tensor(
    mut src: *const TENSOR,
    mut copy_channels: core::ffi::c_int,
    mut dst_offset: core::ffi::c_int,
    mut dst: *mut TENSOR,
) {
    if (*src).width == (*dst).width {} else {
        __assert_fail(
            b"src->width == dst->width\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            76 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"void copy_tensor(const TENSOR *, int, int, TENSOR *)\0"))
                .as_ptr(),
        );
    };
    if (*src).height == (*dst).height {} else {
        __assert_fail(
            b"src->height == dst->height\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            77 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"void copy_tensor(const TENSOR *, int, int, TENSOR *)\0"))
                .as_ptr(),
        );
    };
    if copy_channels <= (*src).channels {} else {
        __assert_fail(
            b"copy_channels <= src->channels\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            78 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 53],
                [core::ffi::c_char; 53],
            >(*b"void copy_tensor(const TENSOR *, int, int, TENSOR *)\0"))
                .as_ptr(),
        );
    };
    if (*src).stride == (*dst).width && (*dst).stride == (*dst).width {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < copy_channels {
            memcpy(
                (*dst).buf[(dst_offset + c) as usize] as *mut core::ffi::c_void,
                (*src).buf[c as usize] as *const core::ffi::c_void,
                (::core::mem::size_of::<core::ffi::c_float>() as size_t)
                    .wrapping_mul((*src).width as size_t)
                    .wrapping_mul((*src).height as size_t),
            );
            c += 1;
        }
    } else {
        let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while c_0 < copy_channels {
            let mut r: core::ffi::c_int = 0 as core::ffi::c_int;
            while r < (*dst).height {
                memcpy(
                    &raw mut *(*((*dst).buf)
                        .as_mut_ptr()
                        .offset((dst_offset + c_0) as isize))
                        .offset((r * (*dst).stride) as isize) as *mut core::ffi::c_float
                        as *mut core::ffi::c_void,
                    &raw mut *(*((*src).buf).as_ptr().offset(c_0 as isize))
                        .offset((r * (*src).stride) as isize) as *mut core::ffi::c_float
                        as *const core::ffi::c_void,
                    ((*dst).width as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_float>() as size_t,
                        ),
                );
                r += 1;
            }
            c_0 += 1;
        }
    };
}
unsafe extern "C-unwind" fn assign_tensor(
    mut tensor: *mut TENSOR,
    mut buf: *mut *mut core::ffi::c_float,
    mut channels: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
) {
    (*tensor).allocsize = 0 as core::ffi::c_int;
    (*tensor).channels = channels;
    (*tensor).width = width;
    (*tensor).height = height;
    (*tensor).stride = stride;
    if !buf.is_null() {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < channels {
            (*tensor).buf[c as usize] = *buf.offset(c as isize);
            c += 1;
        }
    } else {
        let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while c_0 < channels {
            (*tensor).buf[c_0 as usize] = 0 as *mut core::ffi::c_float;
            c_0 += 1;
        }
    };
}
unsafe extern "C-unwind" fn swap_tensor(mut t1: *mut TENSOR, mut t2: *mut TENSOR) {
    let mut t: TENSOR = *t1;
    *t1 = *t2;
    *t2 = t;
}
unsafe extern "C-unwind" fn concat_tensor(
    mut src: *const TENSOR,
    mut dst: *mut TENSOR,
) -> bool {
    if (*src).width == (*dst).width {} else {
        __assert_fail(
            b"src->width == dst->width\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            118 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"_Bool concat_tensor(const TENSOR *, TENSOR *)\0"))
                .as_ptr(),
        );
    };
    if (*src).height == (*dst).height {} else {
        __assert_fail(
            b"src->height == dst->height\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            119 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 46],
                [core::ffi::c_char; 46],
            >(*b"_Bool concat_tensor(const TENSOR *, TENSOR *)\0"))
                .as_ptr(),
        );
    };
    let dst_channels: core::ffi::c_int = (*dst).channels;
    let channels: core::ffi::c_int = (*dst).channels + (*src).channels;
    let newallocsize: core::ffi::c_int = channels * (*dst).width * (*dst).height;
    if (*dst).allocsize < newallocsize {
        let mut t: TENSOR = TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        };
        init_tensor(&raw mut t);
        if !realloc_tensor(&raw mut t, channels, (*dst).width, (*dst).height) {
            return false_0 != 0;
        }
        copy_tensor(dst, (*dst).channels, 0 as core::ffi::c_int, &raw mut t);
        swap_tensor(dst, &raw mut t);
        free_tensor(&raw mut t);
    }
    let mut c: core::ffi::c_int = 1 as core::ffi::c_int;
    while c < channels {
        (*dst).buf[c as usize] = &raw mut *(*((*dst).buf)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .offset((c * (*dst).width * (*dst).height) as isize)
            as *mut core::ffi::c_float;
        c += 1;
    }
    copy_tensor(src, (*src).channels, dst_channels, dst);
    return true_0 != 0;
}
unsafe extern "C-unwind" fn check_tensor_equal_dims(
    mut t1: *mut TENSOR,
    mut t2: *mut TENSOR,
) -> core::ffi::c_int {
    return ((*t1).width == (*t2).width && (*t1).height == (*t2).height)
        as core::ffi::c_int;
}
unsafe extern "C-unwind" fn check_tensor_equal_size(
    mut t1: *mut TENSOR,
    mut t2: *mut TENSOR,
) -> core::ffi::c_int {
    return ((*t1).channels == (*t2).channels && (*t1).width == (*t2).width
        && (*t1).height == (*t2).height) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_find_cnn_layer_output_size(
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut out_width: *mut core::ffi::c_int,
    mut out_height: *mut core::ffi::c_int,
) {
    if (*layer_config).skip_width > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"layer_config->skip_width > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            155 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 86],
                [core::ffi::c_char; 86],
            >(
                *b"void av1_find_cnn_layer_output_size(int, int, const CNN_LAYER_CONFIG *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*layer_config).skip_height > 0 as core::ffi::c_int {} else {
        __assert_fail(
            b"layer_config->skip_height > 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            156 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 86],
                [core::ffi::c_char; 86],
            >(
                *b"void av1_find_cnn_layer_output_size(int, int, const CNN_LAYER_CONFIG *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    if (*layer_config).deconvolve == 0 {
        match (*layer_config).pad as core::ffi::c_int {
            0 | 1 => {
                *out_width = (in_width + (*layer_config).skip_width
                    - 1 as core::ffi::c_int) / (*layer_config).skip_width;
                *out_height = (in_height + (*layer_config).skip_height
                    - 1 as core::ffi::c_int) / (*layer_config).skip_height;
            }
            2 => {
                *out_width = (in_width - (*layer_config).filter_width
                    + (*layer_config).skip_width) / (*layer_config).skip_width;
                *out_height = (in_height - (*layer_config).filter_height
                    + (*layer_config).skip_height) / (*layer_config).skip_height;
            }
            _ => {
                if 0 as core::ffi::c_int != 0
                    && !(b"Unknown padding type\0" as *const u8
                        as *const core::ffi::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"0 && \"Unknown padding type\"\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                            as *const core::ffi::c_char,
                        174 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 86],
                            [core::ffi::c_char; 86],
                        >(
                            *b"void av1_find_cnn_layer_output_size(int, int, const CNN_LAYER_CONFIG *, int *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
    } else {
        match (*layer_config).pad as core::ffi::c_int {
            0 | 1 => {
                *out_width = in_width * (*layer_config).skip_width;
                *out_height = in_height * (*layer_config).skip_height;
            }
            2 => {
                *out_width = (in_width - 1 as core::ffi::c_int)
                    * (*layer_config).skip_width + (*layer_config).filter_width;
                *out_height = (in_height - 1 as core::ffi::c_int)
                    * (*layer_config).skip_height + (*layer_config).filter_height;
            }
            _ => {
                if 0 as core::ffi::c_int != 0
                    && !(b"Unknown padding type\0" as *const u8
                        as *const core::ffi::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"0 && \"Unknown padding type\"\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                            as *const core::ffi::c_char,
                        189 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 86],
                            [core::ffi::c_char; 86],
                        >(
                            *b"void av1_find_cnn_layer_output_size(int, int, const CNN_LAYER_CONFIG *, int *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
    };
}
unsafe extern "C-unwind" fn find_cnn_out_channels(
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut channels_per_branch: *mut core::ffi::c_int,
) {
    let mut branch: core::ffi::c_int = (*layer_config).branch;
    let mut branch_config: *const CNN_BRANCH_CONFIG = &(*layer_config).branch_config;
    let mut b: core::ffi::c_int = 0 as core::ffi::c_int;
    while b < CNN_MAX_BRANCHES {
        if (*branch_config).input_to_branches & (1 as core::ffi::c_int) << b != 0
            && b != branch
        {
            if (*layer_config).branch_copy_type as core::ffi::c_int
                == BRANCH_INPUT as core::ffi::c_int
            {
                *channels_per_branch.offset(b as isize) = (*layer_config).in_channels;
            } else if (*layer_config).branch_copy_type as core::ffi::c_int
                == BRANCH_OUTPUT as core::ffi::c_int
            {
                *channels_per_branch.offset(b as isize) = (*layer_config).out_channels;
            } else if (*layer_config).branch_copy_type as core::ffi::c_int
                == BRANCH_COMBINED as core::ffi::c_int
            {
                *channels_per_branch.offset(b as isize) = (*layer_config).out_channels;
                let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
                while c < CNN_MAX_BRANCHES {
                    if (*branch_config).branches_to_combine
                        & (1 as core::ffi::c_int) << c != 0 && c != branch
                    {
                        if *channels_per_branch.offset(c as isize)
                            > 0 as core::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"channels_per_branch[c] > 0\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                208 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 60],
                                    [core::ffi::c_char; 60],
                                >(
                                    *b"void find_cnn_out_channels(const CNN_LAYER_CONFIG *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        *channels_per_branch.offset(b as isize)
                            += *channels_per_branch.offset(c as isize);
                    }
                    c += 1;
                }
            }
        }
        b += 1;
    }
    *channels_per_branch.offset(branch as isize) = (*layer_config).out_channels;
    let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while c_0 < CNN_MAX_BRANCHES {
        if (*branch_config).branches_to_combine & (1 as core::ffi::c_int) << c_0 != 0
            && c_0 != branch
        {
            if *channels_per_branch.offset(c_0 as isize) > 0 as core::ffi::c_int
            {} else {
                __assert_fail(
                    b"channels_per_branch[c] > 0\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                        as *const core::ffi::c_char,
                    218 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 60],
                        [core::ffi::c_char; 60],
                    >(*b"void find_cnn_out_channels(const CNN_LAYER_CONFIG *, int *)\0"))
                        .as_ptr(),
                );
            };
            *channels_per_branch.offset(branch as isize)
                += *channels_per_branch.offset(c_0 as isize);
        }
        c_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn cnn_has_at_least_one_output(
    mut cnn_config: *const CNN_CONFIG,
) -> core::ffi::c_int {
    let num_layers: core::ffi::c_int = (*cnn_config).num_layers;
    let mut layer_configs: *const CNN_LAYER_CONFIG = ((*cnn_config).layer_config)
        .as_ptr();
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while idx < num_layers {
        if (*layer_configs.offset(idx as isize)).output_num != -(1 as core::ffi::c_int) {
            return 1 as core::ffi::c_int;
        }
        idx += 1;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_find_cnn_output_size(
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut cnn_config: *const CNN_CONFIG,
    mut out_width: *mut core::ffi::c_int,
    mut out_height: *mut core::ffi::c_int,
    mut out_channels: *mut core::ffi::c_int,
) {
    let mut channels_per_branch: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    let mut i_width: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    let mut i_height: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    i_width[0 as core::ffi::c_int as usize] = in_width
        + (*cnn_config).ext_width * 2 as core::ffi::c_int;
    i_height[0 as core::ffi::c_int as usize] = in_height
        + (*cnn_config).ext_height * 2 as core::ffi::c_int;
    if cnn_has_at_least_one_output(cnn_config) != 0 {} else {
        __assert_fail(
            b"cnn_has_at_least_one_output(cnn_config)\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            248 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 81],
                [core::ffi::c_char; 81],
            >(
                *b"void av1_find_cnn_output_size(int, int, const CNN_CONFIG *, int *, int *, int *)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*cnn_config).num_layers {
        let mut layer_config: *const CNN_LAYER_CONFIG = &*((*cnn_config).layer_config)
            .as_ptr()
            .offset(i as isize) as *const CNN_LAYER_CONFIG;
        let mut branch_config: *const CNN_BRANCH_CONFIG = &(*layer_config).branch_config;
        let branch: core::ffi::c_int = (*layer_config).branch;
        let mut o_width: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut o_height: core::ffi::c_int = 0 as core::ffi::c_int;
        if (*layer_config).branch_copy_type as core::ffi::c_int
            == BRANCH_INPUT as core::ffi::c_int
        {
            let mut b: core::ffi::c_int = 0 as core::ffi::c_int;
            while b < CNN_MAX_BRANCHES {
                if (*branch_config).input_to_branches & (1 as core::ffi::c_int) << b != 0
                    && b != branch
                {
                    if i_width[branch as usize] > 0 as core::ffi::c_int
                        && i_height[branch as usize] > 0 as core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"i_width[branch] > 0 && i_height[branch] > 0\0" as *const u8
                                as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                as *const core::ffi::c_char,
                            260 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 81],
                                [core::ffi::c_char; 81],
                            >(
                                *b"void av1_find_cnn_output_size(int, int, const CNN_CONFIG *, int *, int *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    i_width[b as usize] = i_width[branch as usize];
                    i_height[b as usize] = i_height[branch as usize];
                }
                b += 1;
            }
        }
        av1_find_cnn_layer_output_size(
            i_width[branch as usize],
            i_height[branch as usize],
            layer_config,
            &raw mut o_width,
            &raw mut o_height,
        );
        i_width[branch as usize] = o_width;
        i_height[branch as usize] = o_height;
        if (*layer_config).branch_copy_type as core::ffi::c_int
            == BRANCH_OUTPUT as core::ffi::c_int
        {
            let mut b_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while b_0 < CNN_MAX_BRANCHES {
                if (*branch_config).input_to_branches & (1 as core::ffi::c_int) << b_0
                    != 0 && b_0 != branch
                {
                    i_width[b_0 as usize] = o_width;
                    i_height[b_0 as usize] = o_height;
                }
                b_0 += 1;
            }
        }
        find_cnn_out_channels(layer_config, channels_per_branch.as_mut_ptr());
        let output_num: core::ffi::c_int = (*layer_config).output_num;
        if output_num != -(1 as core::ffi::c_int) {
            *out_width.offset(output_num as isize) = o_width;
            *out_height.offset(output_num as isize) = o_height;
            *out_channels.offset(output_num as isize) = channels_per_branch[(*layer_config)
                .branch as usize];
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_start_shift_convolve(
    mut width: core::ffi::c_int,
    mut filt_width: core::ffi::c_int,
    mut stride: core::ffi::c_int,
) -> core::ffi::c_int {
    let mod_0: core::ffi::c_int = width % stride;
    let filt_off: core::ffi::c_int = (filt_width - 1 as core::ffi::c_int)
        / 2 as core::ffi::c_int;
    let dif: core::ffi::c_int = if mod_0 != 0 {
        mod_0 - 1 as core::ffi::c_int
    } else {
        stride - 1 as core::ffi::c_int
    };
    return if ((dif + filt_width % 2 as core::ffi::c_int) / 2 as core::ffi::c_int)
        < filt_off
    {
        (dif + filt_width % 2 as core::ffi::c_int) / 2 as core::ffi::c_int
    } else {
        filt_off
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_add_c(
    mut output: *mut *mut core::ffi::c_float,
    mut channels: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut add: *mut *const core::ffi::c_float,
) {
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < channels {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < height {
            let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
            while j < width {
                *(*output.offset(c as isize)).offset((i * stride + j) as isize)
                    += *(*add.offset(c as isize)).offset((i * stride + j) as isize);
                j += 1;
            }
            i += 1;
        }
        c += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_activate_c(
    mut output: *mut *mut core::ffi::c_float,
    mut channels: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut layer_activation: ACTIVATION,
) {
    if layer_activation as core::ffi::c_int == RELU as core::ffi::c_int {
        let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
        while c < channels {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < height {
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j < width {
                    *(*output.offset(c as isize)).offset((i * stride + j) as isize) = relu(
                        *(*output.offset(c as isize)).offset((i * stride + j) as isize),
                    );
                    j += 1;
                }
                i += 1;
            }
            c += 1;
        }
    } else if layer_activation as core::ffi::c_int == SOFTSIGN as core::ffi::c_int {
        let mut c_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while c_0 < channels {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < height {
                let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while j_0 < width {
                    *(*output.offset(c_0 as isize))
                        .offset((i_0 * stride + j_0) as isize) = softsign(
                        *(*output.offset(c_0 as isize))
                            .offset((i_0 * stride + j_0) as isize),
                    );
                    j_0 += 1;
                }
                i_0 += 1;
            }
            c_0 += 1;
        }
    } else if layer_activation as core::ffi::c_int == SIGMOID as core::ffi::c_int {
        if 0 as core::ffi::c_int != 0
            && !(b"Sigmoid has not been supported in CNN.\0" as *const u8
                as *const core::ffi::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"0 && \"Sigmoid has not been supported in CNN.\"\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                    as *const core::ffi::c_char,
                324 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 66],
                    [core::ffi::c_char; 66],
                >(
                    *b"void av1_cnn_activate_c(float **, int, int, int, int, ACTIVATION)\0",
                ))
                    .as_ptr(),
            );
        };
    } else if layer_activation as core::ffi::c_int != NONE as core::ffi::c_int {
        if 0 as core::ffi::c_int != 0
            && !(b"Unknown activation type\0" as *const u8 as *const core::ffi::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"0 && \"Unknown activation type\"\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                    as *const core::ffi::c_char,
                326 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 66],
                    [core::ffi::c_char; 66],
                >(
                    *b"void av1_cnn_activate_c(float **, int, int, int, int, ACTIVATION)\0",
                ))
                    .as_ptr(),
            );
        };
    }
}
unsafe extern "C-unwind" fn copy_active_tensor_to_branches(
    mut layer_active_tensor: *const TENSOR,
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut branch: core::ffi::c_int,
    mut branch_output: *mut TENSOR,
) -> bool {
    let mut branch_config: *const CNN_BRANCH_CONFIG = &(*layer_config).branch_config;
    let mut b: core::ffi::c_int = 0 as core::ffi::c_int;
    while b < CNN_MAX_BRANCHES {
        if (*branch_config).input_to_branches & (1 as core::ffi::c_int) << b != 0
            && b != branch
        {
            let mut copy_channels: core::ffi::c_int = if (*branch_config)
                .channels_to_copy > 0 as core::ffi::c_int
            {
                (*branch_config).channels_to_copy
            } else {
                (*layer_active_tensor).channels
            };
            if !realloc_tensor(
                &raw mut *branch_output.offset(b as isize),
                copy_channels,
                (*layer_active_tensor).width,
                (*layer_active_tensor).height,
            ) {
                return false_0 != 0;
            }
            copy_tensor(
                layer_active_tensor,
                copy_channels,
                0 as core::ffi::c_int,
                &raw mut *branch_output.offset(b as isize),
            );
        }
        b += 1;
    }
    return true_0 != 0;
}
unsafe extern "C-unwind" fn convolve_maxpool_padding_zero(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    cstep: core::ffi::c_int,
    filter_width_half: core::ffi::c_int,
    filter_height_half: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < in_height {
            let mut w: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut v: core::ffi::c_int = 0 as core::ffi::c_int;
            while w < in_width {
                let mut hh: core::ffi::c_int = h;
                while hh
                    < (if in_height < h + (*layer_config).skip_height {
                        in_height
                    } else {
                        h + (*layer_config).skip_height
                    })
                {
                    let mut ww: core::ffi::c_int = w;
                    while ww
                        < (if in_width < w + (*layer_config).skip_width {
                            in_width
                        } else {
                            w + (*layer_config).skip_width
                        })
                    {
                        let mut sum: core::ffi::c_float = *((*layer_config).bias)
                            .offset(i as isize);
                        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                        while k < (*layer_config).in_channels {
                            let mut off: core::ffi::c_int = k
                                * (*layer_config).out_channels + i;
                            let mut l: core::ffi::c_int = 0 as core::ffi::c_int;
                            while l < (*layer_config).filter_height {
                                let ii: core::ffi::c_int = hh + l - filter_height_half;
                                let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                                while m < (*layer_config).filter_width {
                                    let jj: core::ffi::c_int = ww + m - filter_width_half;
                                    if !(ii < 0 as core::ffi::c_int || ii >= in_height
                                        || jj < 0 as core::ffi::c_int || jj >= in_width)
                                    {
                                        sum
                                            += *((*layer_config).weights).offset(off as isize)
                                                * *(*input.offset(k as isize))
                                                    .offset((ii * in_stride + jj) as isize);
                                    }
                                    m += 1;
                                    off += cstep;
                                }
                                l += 1;
                            }
                            k += 1;
                        }
                        let a: core::ffi::c_float = sum;
                        if h == hh && w == ww {
                            *(*output.offset(i as isize))
                                .offset((u * out_stride + v) as isize) = a;
                        } else {
                            *(*output.offset(i as isize))
                                .offset((u * out_stride + v) as isize) = if *(*output
                                .offset(i as isize))
                                .offset((u * out_stride + v) as isize) > a
                            {
                                *(*output.offset(i as isize))
                                    .offset((u * out_stride + v) as isize)
                            } else {
                                a
                            };
                        }
                        ww += 1;
                    }
                    hh += 1;
                }
                w += (*layer_config).skip_width;
                v += 1;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn convolve_maxpool_padding_replicate(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    cstep: core::ffi::c_int,
    filter_width_half: core::ffi::c_int,
    filter_height_half: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < in_height {
            let mut w: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut v: core::ffi::c_int = 0 as core::ffi::c_int;
            while w < in_width {
                let mut hh: core::ffi::c_int = h;
                while hh
                    < (if in_height < h + (*layer_config).skip_height {
                        in_height
                    } else {
                        h + (*layer_config).skip_height
                    })
                {
                    let mut ww: core::ffi::c_int = w;
                    while ww
                        < (if in_width < w + (*layer_config).skip_width {
                            in_width
                        } else {
                            w + (*layer_config).skip_width
                        })
                    {
                        let mut sum: core::ffi::c_float = *((*layer_config).bias)
                            .offset(i as isize);
                        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                        while k < (*layer_config).in_channels {
                            let mut off: core::ffi::c_int = k
                                * (*layer_config).out_channels + i;
                            let mut l: core::ffi::c_int = 0 as core::ffi::c_int;
                            while l < (*layer_config).filter_height {
                                let ii: core::ffi::c_int = if hh + l - filter_height_half
                                    < 0 as core::ffi::c_int
                                {
                                    0 as core::ffi::c_int
                                } else if hh + l - filter_height_half >= in_height {
                                    in_height - 1 as core::ffi::c_int
                                } else {
                                    hh + l - filter_height_half
                                };
                                let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                                while m < (*layer_config).filter_width {
                                    let jj: core::ffi::c_int = if ww + m - filter_width_half
                                        < 0 as core::ffi::c_int
                                    {
                                        0 as core::ffi::c_int
                                    } else if ww + m - filter_width_half >= in_width {
                                        in_width - 1 as core::ffi::c_int
                                    } else {
                                        ww + m - filter_width_half
                                    };
                                    if ii >= 0 as core::ffi::c_int && ii < in_height
                                        && jj >= 0 as core::ffi::c_int && jj < in_width
                                    {} else {
                                        __assert_fail(
                                            b"ii >= 0 && ii < in_height && jj >= 0 && jj < in_width\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                                as *const core::ffi::c_char,
                                            419 as core::ffi::c_uint,
                                            (::core::mem::transmute::<
                                                [u8; 150],
                                                [core::ffi::c_char; 150],
                                            >(
                                                *b"void convolve_maxpool_padding_replicate(const float **, int, int, int, const CNN_LAYER_CONFIG *const, float **, int, const int, const int, const int)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                    sum
                                        += *((*layer_config).weights).offset(off as isize)
                                            * *(*input.offset(k as isize))
                                                .offset((ii * in_stride + jj) as isize);
                                    m += 1;
                                    off += cstep;
                                }
                                l += 1;
                            }
                            k += 1;
                        }
                        let a: core::ffi::c_float = sum;
                        if h == hh && w == ww {
                            *(*output.offset(i as isize))
                                .offset((u * out_stride + v) as isize) = a;
                        } else {
                            *(*output.offset(i as isize))
                                .offset((u * out_stride + v) as isize) = if *(*output
                                .offset(i as isize))
                                .offset((u * out_stride + v) as isize) > a
                            {
                                *(*output.offset(i as isize))
                                    .offset((u * out_stride + v) as isize)
                            } else {
                                a
                            };
                        }
                        ww += 1;
                    }
                    hh += 1;
                }
                w += (*layer_config).skip_width;
                v += 1;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn convolve_maxpool_padding_valid(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    cstep: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < in_height - (*layer_config).filter_height + 1 as core::ffi::c_int {
            let mut w: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut v: core::ffi::c_int = 0 as core::ffi::c_int;
            while w < in_width - (*layer_config).filter_width + 1 as core::ffi::c_int {
                let mut hh: core::ffi::c_int = h;
                while hh
                    < (if in_height < h + (*layer_config).skip_height {
                        in_height
                    } else {
                        h + (*layer_config).skip_height
                    })
                {
                    let mut ww: core::ffi::c_int = w;
                    while ww
                        < (if in_width < w + (*layer_config).skip_width {
                            in_width
                        } else {
                            w + (*layer_config).skip_width
                        })
                    {
                        let mut sum: core::ffi::c_float = *((*layer_config).bias)
                            .offset(i as isize);
                        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                        while k < (*layer_config).in_channels {
                            let mut off: core::ffi::c_int = k
                                * (*layer_config).out_channels + i;
                            let mut l: core::ffi::c_int = 0 as core::ffi::c_int;
                            while l < (*layer_config).filter_height {
                                let ii: core::ffi::c_int = hh + l;
                                let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                                while m < (*layer_config).filter_width {
                                    let jj: core::ffi::c_int = ww + m;
                                    if ii >= 0 as core::ffi::c_int && ii < in_height
                                        && jj >= 0 as core::ffi::c_int && jj < in_width
                                    {} else {
                                        __assert_fail(
                                            b"ii >= 0 && ii < in_height && jj >= 0 && jj < in_width\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                                as *const core::ffi::c_char,
                                            461 as core::ffi::c_uint,
                                            (::core::mem::transmute::<
                                                [u8; 124],
                                                [core::ffi::c_char; 124],
                                            >(
                                                *b"void convolve_maxpool_padding_valid(const float **, int, int, int, const CNN_LAYER_CONFIG *const, float **, int, const int)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    };
                                    sum
                                        += *((*layer_config).weights).offset(off as isize)
                                            * *(*input.offset(k as isize))
                                                .offset((ii * in_stride + jj) as isize);
                                    m += 1;
                                    off += cstep;
                                }
                                l += 1;
                            }
                            k += 1;
                        }
                        let a: core::ffi::c_float = sum;
                        if h == hh && w == ww {
                            *(*output.offset(i as isize))
                                .offset((u * out_stride + v) as isize) = a;
                        } else {
                            *(*output.offset(i as isize))
                                .offset((u * out_stride + v) as isize) = if *(*output
                                .offset(i as isize))
                                .offset((u * out_stride + v) as isize) > a
                            {
                                *(*output.offset(i as isize))
                                    .offset((u * out_stride + v) as isize)
                            } else {
                                a
                            };
                        }
                        ww += 1;
                    }
                    hh += 1;
                }
                w += (*layer_config).skip_width;
                v += 1;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn convolve_element_wise(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    mut start_idx: core::ffi::c_int,
    mut step: core::ffi::c_int,
) {
    let start_h: core::ffi::c_int = get_start_shift_convolve(
        in_height,
        (*layer_config).filter_height,
        (*layer_config).skip_height,
    ) as core::ffi::c_int;
    let start_w: core::ffi::c_int = get_start_shift_convolve(
        in_width,
        (*layer_config).filter_width,
        (*layer_config).skip_width,
    ) as core::ffi::c_int + start_idx * (*layer_config).skip_width;
    let out_w_step: core::ffi::c_int = if step > 1 as core::ffi::c_int {
        step
    } else {
        1 as core::ffi::c_int
    };
    let in_w_step: core::ffi::c_int = (*layer_config).skip_width * out_w_step;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = start_h;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < in_height {
            let in_h: core::ffi::c_int = h * in_stride;
            let out_h: core::ffi::c_int = u * out_stride + start_idx;
            let mut w: core::ffi::c_int = start_w;
            let mut out_index: core::ffi::c_int = out_h;
            while w < in_width {
                let mut sum: core::ffi::c_float = *((*layer_config).bias)
                    .offset(i as isize);
                let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                while k < (*layer_config).in_channels {
                    sum
                        += *((*layer_config).weights)
                            .offset((k * (*layer_config).out_channels + i) as isize)
                            * *(*input.offset(k as isize)).offset((in_h + w) as isize);
                    k += 1;
                }
                *(*output.offset(i as isize)).offset(out_index as isize) = sum;
                w += in_w_step;
                out_index += out_w_step;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += 1;
    }
}
unsafe extern "C-unwind" fn convolve_no_maxpool_padding_zero(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    mut start_idx: core::ffi::c_int,
    cstep: core::ffi::c_int,
    filter_width_half: core::ffi::c_int,
    filter_height_half: core::ffi::c_int,
    ii_shift: core::ffi::c_int,
    jj_shift: core::ffi::c_int,
    channel_step: core::ffi::c_int,
) {
    let start_h: core::ffi::c_int = get_start_shift_convolve(
        in_height,
        (*layer_config).filter_height,
        (*layer_config).skip_height,
    ) as core::ffi::c_int;
    let start_w: core::ffi::c_int = get_start_shift_convolve(
        in_width,
        (*layer_config).filter_width,
        (*layer_config).skip_width,
    ) as core::ffi::c_int;
    let end_ii_shift: core::ffi::c_int = filter_height_half + 1 as core::ffi::c_int;
    let end_jj_shift: core::ffi::c_int = filter_width_half + 1 as core::ffi::c_int;
    let top_filter_margin: core::ffi::c_int = (*layer_config).filter_width * ii_shift;
    let right_filter_margin: core::ffi::c_int = end_jj_shift - in_width;
    let mut i: core::ffi::c_int = start_idx;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = start_h;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < in_height {
            let out_h: core::ffi::c_int = u * out_stride;
            let top_cstep: core::ffi::c_int = (if 0 as core::ffi::c_int
                > top_filter_margin - h * (*layer_config).filter_width
            {
                0 as core::ffi::c_int
            } else {
                top_filter_margin - h * (*layer_config).filter_width
            }) * cstep + i;
            let start_ii: core::ffi::c_int = if 0 as core::ffi::c_int > h - ii_shift {
                0 as core::ffi::c_int
            } else {
                h - ii_shift
            };
            let end_ii: core::ffi::c_int = if in_height < h + end_ii_shift {
                in_height
            } else {
                h + end_ii_shift
            };
            let mut w: core::ffi::c_int = start_w;
            let mut out_index: core::ffi::c_int = out_h;
            while w < in_width {
                let left_cstep: core::ffi::c_int = (if 0 as core::ffi::c_int
                    > jj_shift - w
                {
                    0 as core::ffi::c_int
                } else {
                    jj_shift - w
                }) * cstep;
                let right_cstep: core::ffi::c_int = (if 0 as core::ffi::c_int
                    > right_filter_margin + w
                {
                    0 as core::ffi::c_int
                } else {
                    right_filter_margin + w
                }) * cstep;
                let start_jj: core::ffi::c_int = if 0 as core::ffi::c_int > w - jj_shift
                {
                    0 as core::ffi::c_int
                } else {
                    w - jj_shift
                };
                let end_jj: core::ffi::c_int = if in_width < w + end_jj_shift {
                    in_width
                } else {
                    w + end_jj_shift
                };
                let mut sum: core::ffi::c_float = *((*layer_config).bias)
                    .offset(i as isize);
                let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                while k < (*layer_config).in_channels {
                    let mut off: core::ffi::c_int = k * (*layer_config).out_channels
                        + top_cstep;
                    let mut ii: core::ffi::c_int = start_ii;
                    while ii < end_ii {
                        off += left_cstep;
                        let mut jj: core::ffi::c_int = start_jj;
                        while jj < end_jj {
                            sum
                                += *((*layer_config).weights).offset(off as isize)
                                    * *(*input.offset(k as isize))
                                        .offset((ii * in_stride + jj) as isize);
                            jj += 1;
                            off += cstep;
                        }
                        off += right_cstep;
                        ii += 1;
                    }
                    k += 1;
                }
                *(*output.offset(i as isize)).offset(out_index as isize) = sum;
                w += (*layer_config).skip_width;
                out_index += 1;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += channel_step;
    }
}
unsafe extern "C-unwind" fn convolve_no_maxpool_padding_replicate(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    mut start_idx: core::ffi::c_int,
    cstep: core::ffi::c_int,
    ii_shift: core::ffi::c_int,
    jj_shift: core::ffi::c_int,
    channel_step: core::ffi::c_int,
) {
    let start_h: core::ffi::c_int = get_start_shift_convolve(
        in_height,
        (*layer_config).filter_height,
        (*layer_config).skip_height,
    ) as core::ffi::c_int - ii_shift;
    let start_w: core::ffi::c_int = get_start_shift_convolve(
        in_width,
        (*layer_config).filter_width,
        (*layer_config).skip_width,
    ) as core::ffi::c_int - jj_shift;
    let end_h: core::ffi::c_int = in_height - ii_shift;
    let end_w: core::ffi::c_int = in_width - jj_shift;
    let mut i: core::ffi::c_int = start_idx;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = start_h;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < end_h {
            let out_h: core::ffi::c_int = u * out_stride;
            let upper_ii_index: core::ffi::c_int = (*layer_config).filter_height + h;
            let mut w: core::ffi::c_int = start_w;
            let mut out_index: core::ffi::c_int = out_h;
            while w < end_w {
                let upper_jj_index: core::ffi::c_int = (*layer_config).filter_width + w;
                let mut sum: core::ffi::c_float = *((*layer_config).bias)
                    .offset(i as isize);
                let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                while k < (*layer_config).in_channels {
                    let mut off: core::ffi::c_int = k * (*layer_config).out_channels + i;
                    let mut ii: core::ffi::c_int = h;
                    while ii < upper_ii_index {
                        let clamped_ii: core::ffi::c_int = if ii < 0 as core::ffi::c_int
                        {
                            0 as core::ffi::c_int
                        } else if ii >= in_height {
                            in_height - 1 as core::ffi::c_int
                        } else {
                            ii
                        };
                        let mut jj: core::ffi::c_int = w;
                        while jj < upper_jj_index {
                            let clamped_jj: core::ffi::c_int = if jj
                                < 0 as core::ffi::c_int
                            {
                                0 as core::ffi::c_int
                            } else if jj >= in_width {
                                in_width - 1 as core::ffi::c_int
                            } else {
                                jj
                            };
                            if clamped_ii >= 0 as core::ffi::c_int
                                && clamped_ii < in_height
                                && clamped_jj >= 0 as core::ffi::c_int
                                && clamped_jj < in_width
                            {} else {
                                __assert_fail(
                                    b"clamped_ii >= 0 && clamped_ii < in_height && clamped_jj >= 0 && clamped_jj < in_width\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                        as *const core::ffi::c_char,
                                    600 as core::ffi::c_uint,
                                    (::core::mem::transmute::<
                                        [u8; 169],
                                        [core::ffi::c_char; 169],
                                    >(
                                        *b"void convolve_no_maxpool_padding_replicate(const float **, int, int, int, const CNN_LAYER_CONFIG *const, float **, int, int, const int, const int, const int, const int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            };
                            sum
                                += *((*layer_config).weights).offset(off as isize)
                                    * *(*input.offset(k as isize))
                                        .offset((clamped_ii * in_stride + clamped_jj) as isize);
                            off += cstep;
                            jj += 1;
                        }
                        ii += 1;
                    }
                    k += 1;
                }
                *(*output.offset(i as isize)).offset(out_index as isize) = sum;
                w += (*layer_config).skip_width;
                out_index += 1;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += channel_step;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_convolve_no_maxpool_padding_valid_c(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    mut start_idx: core::ffi::c_int,
    mut cstep: core::ffi::c_int,
    mut channel_step: core::ffi::c_int,
) {
    if (*layer_config).skip_height == 1 as core::ffi::c_int
        && (*layer_config).skip_width == 1 as core::ffi::c_int
        || (*layer_config).maxpool == 0
    {} else {
        __assert_fail(
            b"(layer_config->skip_height == 1 && layer_config->skip_width == 1) || !layer_config->maxpool\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            620 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 136],
                [core::ffi::c_char; 136],
            >(
                *b"void av1_cnn_convolve_no_maxpool_padding_valid_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if (*layer_config).filter_height > 1 as core::ffi::c_int
        || (*layer_config).filter_width > 1 as core::ffi::c_int
    {} else {
        __assert_fail(
            b"layer_config->filter_height > 1 || layer_config->filter_width > 1\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            621 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 136],
                [core::ffi::c_char; 136],
            >(
                *b"void av1_cnn_convolve_no_maxpool_padding_valid_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    if (*layer_config).pad as core::ffi::c_int == PADDING_VALID as core::ffi::c_int
    {} else {
        __assert_fail(
            b"layer_config->pad == PADDING_VALID\0" as *const u8
                as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            622 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 136],
                [core::ffi::c_char; 136],
            >(
                *b"void av1_cnn_convolve_no_maxpool_padding_valid_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut i: core::ffi::c_int = start_idx;
    while i < (*layer_config).out_channels {
        let mut h: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
        while h < in_height - (*layer_config).filter_height + 1 as core::ffi::c_int {
            let out_h: core::ffi::c_int = u * out_stride;
            let upper_ii_index: core::ffi::c_int = (*layer_config).filter_height + h;
            let mut w: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut out_index: core::ffi::c_int = out_h;
            while w < in_width - (*layer_config).filter_width + 1 as core::ffi::c_int {
                let upper_jj_index: core::ffi::c_int = (*layer_config).filter_width + w;
                let mut sum: core::ffi::c_float = *((*layer_config).bias)
                    .offset(i as isize);
                let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                while k < (*layer_config).in_channels {
                    let mut off: core::ffi::c_int = k * (*layer_config).out_channels + i;
                    let mut ii: core::ffi::c_int = h;
                    while ii < upper_ii_index {
                        let mut jj: core::ffi::c_int = w;
                        while jj < upper_jj_index {
                            if ii >= 0 as core::ffi::c_int && ii < in_height
                                && jj >= 0 as core::ffi::c_int && jj < in_width
                            {} else {
                                __assert_fail(
                                    b"ii >= 0 && ii < in_height && jj >= 0 && jj < in_width\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                        as *const core::ffi::c_char,
                                    637 as core::ffi::c_uint,
                                    (::core::mem::transmute::<
                                        [u8; 136],
                                        [core::ffi::c_char; 136],
                                    >(
                                        *b"void av1_cnn_convolve_no_maxpool_padding_valid_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            };
                            sum
                                += *((*layer_config).weights).offset(off as isize)
                                    * *(*input.offset(k as isize))
                                        .offset((ii * in_stride + jj) as isize);
                            off += cstep;
                            jj += 1;
                        }
                        ii += 1;
                    }
                    k += 1;
                }
                *(*output.offset(i as isize)).offset(out_index as isize) = sum;
                w += (*layer_config).skip_width;
                out_index += 1;
            }
            h += (*layer_config).skip_height;
            u += 1;
        }
        i += channel_step;
    }
}
unsafe extern "C-unwind" fn av1_cnn_convolve(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
    mut start_idx: core::ffi::c_int,
    mut step: core::ffi::c_int,
) {
    if (*layer_config).deconvolve == 0 {} else {
        __assert_fail(
            b"!layer_config->deconvolve\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            654 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 104],
                [core::ffi::c_char; 104],
            >(
                *b"void av1_cnn_convolve(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    let cstep: core::ffi::c_int = (*layer_config).in_channels
        * (*layer_config).out_channels;
    let filter_height_half: core::ffi::c_int = (*layer_config).filter_height
        >> 1 as core::ffi::c_int;
    let filter_width_half: core::ffi::c_int = (*layer_config).filter_width
        >> 1 as core::ffi::c_int;
    let channel_step: core::ffi::c_int = if step > 1 as core::ffi::c_int {
        step
    } else {
        1 as core::ffi::c_int
    };
    if (*layer_config).maxpool != 0
        && ((*layer_config).skip_height > 1 as core::ffi::c_int
            || (*layer_config).skip_width > 1 as core::ffi::c_int)
    {
        match (*layer_config).pad as core::ffi::c_int {
            0 => {
                convolve_maxpool_padding_zero(
                    input,
                    in_width,
                    in_height,
                    in_stride,
                    layer_config,
                    output,
                    out_stride,
                    cstep,
                    filter_width_half,
                    filter_height_half,
                );
            }
            1 => {
                convolve_maxpool_padding_replicate(
                    input,
                    in_width,
                    in_height,
                    in_stride,
                    layer_config,
                    output,
                    out_stride,
                    cstep,
                    filter_width_half,
                    filter_height_half,
                );
            }
            2 => {
                convolve_maxpool_padding_valid(
                    input,
                    in_width,
                    in_height,
                    in_stride,
                    layer_config,
                    output,
                    out_stride,
                    cstep,
                );
            }
            _ => {
                if 0 as core::ffi::c_int != 0
                    && !(b"Unknown padding type\0" as *const u8
                        as *const core::ffi::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"0 && \"Unknown padding type\"\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                            as *const core::ffi::c_char,
                        677 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 104],
                            [core::ffi::c_char; 104],
                        >(
                            *b"void av1_cnn_convolve(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
    } else {
        if (*layer_config).filter_height == 1 as core::ffi::c_int
            && (*layer_config).filter_width == 1 as core::ffi::c_int
        {
            convolve_element_wise(
                input,
                in_width,
                in_height,
                in_stride,
                layer_config,
                output,
                out_stride,
                start_idx,
                step,
            );
            return;
        }
        let ii_shift: core::ffi::c_int = filter_height_half
            - ((*layer_config).filter_height - 1 as core::ffi::c_int)
                % 2 as core::ffi::c_int;
        let jj_shift: core::ffi::c_int = filter_width_half
            - ((*layer_config).filter_width - 1 as core::ffi::c_int)
                % 2 as core::ffi::c_int;
        match (*layer_config).pad as core::ffi::c_int {
            0 => {
                convolve_no_maxpool_padding_zero(
                    input,
                    in_width,
                    in_height,
                    in_stride,
                    layer_config,
                    output,
                    out_stride,
                    start_idx,
                    cstep,
                    filter_width_half,
                    filter_height_half,
                    ii_shift,
                    jj_shift,
                    channel_step,
                );
            }
            1 => {
                convolve_no_maxpool_padding_replicate(
                    input,
                    in_width,
                    in_height,
                    in_stride,
                    layer_config,
                    output,
                    out_stride,
                    start_idx,
                    cstep,
                    ii_shift,
                    jj_shift,
                    channel_step,
                );
            }
            2 => {
                av1_cnn_convolve_no_maxpool_padding_valid_c(
                    input,
                    in_width,
                    in_height,
                    in_stride,
                    layer_config,
                    output,
                    out_stride,
                    start_idx,
                    cstep,
                    channel_step,
                );
            }
            _ => {
                if 0 as core::ffi::c_int != 0
                    && !(b"Unknown padding type\0" as *const u8
                        as *const core::ffi::c_char)
                        .is_null()
                {} else {
                    __assert_fail(
                        b"0 && \"Unknown padding type\"\0" as *const u8
                            as *const core::ffi::c_char,
                        b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                            as *const core::ffi::c_char,
                        707 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 104],
                            [core::ffi::c_char; 104],
                        >(
                            *b"void av1_cnn_convolve(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
    };
}
unsafe extern "C-unwind" fn convolve_layer(
    mut arg1: *mut core::ffi::c_void,
    mut arg2: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let mut convolve_ops: *const CONVOLVE_OPS = arg1 as *const CONVOLVE_OPS;
    av1_cnn_convolve(
        (*convolve_ops).input,
        (*convolve_ops).in_width,
        (*convolve_ops).in_height,
        (*convolve_ops).in_stride,
        (*convolve_ops).layer_config,
        (*convolve_ops).output,
        (*convolve_ops).out_stride,
        (*convolve_ops).start_idx,
        (*convolve_ops).th_step,
    );
    return 1 as core::ffi::c_int;
}
unsafe extern "C-unwind" fn convolve_layer_mt(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut thread_data: *const CNN_THREAD_DATA,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
) {
    let winterface: *const AVxWorkerInterface = aom_get_worker_interface()
        as *const AVxWorkerInterface;
    let num_workers: core::ffi::c_int = (*thread_data).num_workers;
    if !((*thread_data).workers).is_null() {} else {
        __assert_fail(
            b"thread_data->workers\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            729 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 120],
                [core::ffi::c_char; 120],
            >(
                *b"void convolve_layer_mt(const float **, int, int, int, const CNN_LAYER_CONFIG *, const CNN_THREAD_DATA *, float **, int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut convolve_ops: [CONVOLVE_OPS; 32] = [CONVOLVE_OPS {
        input: 0 as *mut *const core::ffi::c_float,
        in_width: 0,
        in_height: 0,
        in_stride: 0,
        layer_config: 0 as *const CNN_LAYER_CONFIG,
        output: 0 as *mut *mut core::ffi::c_float,
        out_stride: 0,
        start_idx: 0,
        th_step: 0,
    }; 32];
    let mut th: core::ffi::c_int = 0 as core::ffi::c_int;
    while th
        < (if num_workers < 32 as core::ffi::c_int {
            num_workers
        } else {
            32 as core::ffi::c_int
        })
    {
        let worker: *mut AVxWorker = &raw mut *((*thread_data).workers).offset(th as isize)
            as *mut AVxWorker;
        ((*winterface).reset).expect("non-null function pointer")(worker);
        let mut convolve_op: CONVOLVE_OPS = {
            let mut init = CONVOLVE_OPS {
                input: input,
                in_width: in_width,
                in_height: in_height,
                in_stride: in_stride,
                layer_config: layer_config,
                output: output,
                out_stride: out_stride,
                start_idx: th,
                th_step: num_workers,
            };
            init
        };
        convolve_ops[th as usize] = convolve_op;
        (*worker).hook = Some(
            convolve_layer
                as unsafe extern "C-unwind" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> core::ffi::c_int,
        ) as AVxWorkerHook;
        (*worker).data1 = &raw mut *convolve_ops.as_mut_ptr().offset(th as isize)
            as *mut CONVOLVE_OPS as *mut core::ffi::c_void;
        (*worker).data2 = NULL;
        if th == num_workers - 1 as core::ffi::c_int {
            ((*winterface).execute).expect("non-null function pointer")(worker);
        } else {
            ((*winterface).launch).expect("non-null function pointer")(worker);
        }
        th += 1;
    }
    let mut th_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while th_0
        < (if num_workers < 32 as core::ffi::c_int {
            num_workers
        } else {
            32 as core::ffi::c_int
        })
    {
        ((*winterface).sync)
            .expect(
                "non-null function pointer",
            )(&raw mut *((*thread_data).workers).offset(th_0 as isize));
        th_0 += 1;
    }
}
#[inline]
unsafe extern "C-unwind" fn get_start_shift_deconvolve(
    mut filt_width: core::ffi::c_int,
    mut stride: core::ffi::c_int,
) -> core::ffi::c_int {
    let dif: core::ffi::c_int = if filt_width - stride > 0 as core::ffi::c_int {
        filt_width - stride
    } else {
        0 as core::ffi::c_int
    };
    return dif / 2 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_batchnorm_c(
    mut image: *mut *mut core::ffi::c_float,
    mut channels: core::ffi::c_int,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut gamma: *const core::ffi::c_float,
    mut beta: *const core::ffi::c_float,
    mut mean: *const core::ffi::c_float,
    mut std: *const core::ffi::c_float,
) {
    if !gamma.is_null() && !beta.is_null() && !beta.is_null() && !std.is_null()
        && !(b"batchnorm has null parameter!\0" as *const u8 as *const core::ffi::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"gamma && beta && beta && std && \"batchnorm has null parameter!\"\0"
                as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            766 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 115],
                [core::ffi::c_char; 115],
            >(
                *b"void av1_cnn_batchnorm_c(float **, int, int, int, int, const float *, const float *, const float *, const float *)\0",
            ))
                .as_ptr(),
        );
    };
    let mut ch: core::ffi::c_int = 0 as core::ffi::c_int;
    while ch < channels {
        let ch_gamma: core::ffi::c_float = *gamma.offset(ch as isize);
        let ch_beta: core::ffi::c_float = *beta.offset(ch as isize);
        let ch_mean: core::ffi::c_float = *mean.offset(ch as isize);
        let ch_std: core::ffi::c_float = *std.offset(ch as isize);
        let mut image_row: *mut core::ffi::c_float = *image.offset(ch as isize);
        let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
        while row < height {
            let mut col: core::ffi::c_int = 0 as core::ffi::c_int;
            while col < width {
                *image_row.offset(col as isize) = ch_gamma
                    * (*image_row.offset(col as isize) - ch_mean) / ch_std + ch_beta;
                col += 1;
            }
            image_row = image_row.offset(stride as isize);
            row += 1;
        }
        ch += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_deconvolve_c(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut layer_config: *const CNN_LAYER_CONFIG,
    mut output: *mut *mut core::ffi::c_float,
    mut out_stride: core::ffi::c_int,
) {
    if (*layer_config).deconvolve != 0 {} else {
        __assert_fail(
            b"layer_config->deconvolve\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8 as *const core::ffi::c_char,
            787 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 98],
                [core::ffi::c_char; 98],
            >(
                *b"void av1_cnn_deconvolve_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int)\0",
            ))
                .as_ptr(),
        );
    };
    let cstep: core::ffi::c_int = (*layer_config).in_channels
        * (*layer_config).out_channels;
    let mut out_width: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut out_height: core::ffi::c_int = 0 as core::ffi::c_int;
    av1_find_cnn_layer_output_size(
        in_width,
        in_height,
        layer_config,
        &raw mut out_width,
        &raw mut out_height,
    );
    match (*layer_config).pad as core::ffi::c_int {
        0 => {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < (*layer_config).out_channels {
                let mut u: core::ffi::c_int = 0 as core::ffi::c_int;
                while u < out_height {
                    let mut v: core::ffi::c_int = 0 as core::ffi::c_int;
                    while v < out_width {
                        let mut sum: core::ffi::c_float = *((*layer_config).bias)
                            .offset(i as isize);
                        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
                        while k < (*layer_config).in_channels {
                            let mut off: core::ffi::c_int = k
                                * (*layer_config).out_channels + i;
                            let mut l: core::ffi::c_int = 0 as core::ffi::c_int;
                            while l < (*layer_config).filter_height {
                                let h: core::ffi::c_int = u - l
                                    + get_start_shift_deconvolve(
                                        (*layer_config).filter_height,
                                        (*layer_config).skip_height,
                                    ) as core::ffi::c_int;
                                let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                                while m < (*layer_config).filter_width {
                                    let w: core::ffi::c_int = v - m
                                        + get_start_shift_deconvolve(
                                            (*layer_config).filter_width,
                                            (*layer_config).skip_width,
                                        ) as core::ffi::c_int;
                                    if !(h % (*layer_config).skip_height
                                        != 0 as core::ffi::c_int
                                        || w % (*layer_config).skip_width != 0 as core::ffi::c_int)
                                    {
                                        let ii: core::ffi::c_int = h / (*layer_config).skip_height;
                                        let jj: core::ffi::c_int = w / (*layer_config).skip_width;
                                        if !(ii < 0 as core::ffi::c_int || ii >= in_height
                                            || jj < 0 as core::ffi::c_int || jj >= in_width)
                                        {
                                            sum
                                                += *((*layer_config).weights).offset(off as isize)
                                                    * *(*input.offset(k as isize))
                                                        .offset((ii * in_stride + jj) as isize);
                                        }
                                    }
                                    m += 1;
                                    off += cstep;
                                }
                                l += 1;
                            }
                            k += 1;
                        }
                        *(*output.offset(i as isize))
                            .offset((u * out_stride + v) as isize) = sum;
                        v += 1;
                    }
                    u += 1;
                }
                i += 1;
            }
        }
        1 => {
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < (*layer_config).out_channels {
                let mut u_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while u_0 < out_height {
                    let mut v_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while v_0 < out_width {
                        let mut sum_0: core::ffi::c_float = *((*layer_config).bias)
                            .offset(i_0 as isize);
                        let mut k_0: core::ffi::c_int = 0 as core::ffi::c_int;
                        while k_0 < (*layer_config).in_channels {
                            let mut off_0: core::ffi::c_int = k_0
                                * (*layer_config).out_channels + i_0;
                            let mut l_0: core::ffi::c_int = 0 as core::ffi::c_int;
                            while l_0 < (*layer_config).filter_height {
                                let h_0: core::ffi::c_int = u_0 - l_0
                                    + get_start_shift_deconvolve(
                                        (*layer_config).filter_height,
                                        (*layer_config).skip_height,
                                    ) as core::ffi::c_int;
                                let mut m_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                while m_0 < (*layer_config).filter_width {
                                    let w_0: core::ffi::c_int = v_0 - m_0
                                        + get_start_shift_deconvolve(
                                            (*layer_config).filter_width,
                                            (*layer_config).skip_width,
                                        ) as core::ffi::c_int;
                                    if !(h_0 % (*layer_config).skip_height
                                        != 0 as core::ffi::c_int
                                        || w_0 % (*layer_config).skip_width
                                            != 0 as core::ffi::c_int)
                                    {
                                        let ii_0: core::ffi::c_int = if h_0
                                            / (*layer_config).skip_height < 0 as core::ffi::c_int
                                        {
                                            0 as core::ffi::c_int
                                        } else if h_0 / (*layer_config).skip_height >= in_height {
                                            in_height - 1 as core::ffi::c_int
                                        } else {
                                            h_0 / (*layer_config).skip_height
                                        };
                                        let jj_0: core::ffi::c_int = if w_0
                                            / (*layer_config).skip_width < 0 as core::ffi::c_int
                                        {
                                            0 as core::ffi::c_int
                                        } else if w_0 / (*layer_config).skip_width >= in_width {
                                            in_width - 1 as core::ffi::c_int
                                        } else {
                                            w_0 / (*layer_config).skip_width
                                        };
                                        if ii_0 >= 0 as core::ffi::c_int && ii_0 < in_height
                                            && jj_0 >= 0 as core::ffi::c_int && jj_0 < in_width
                                        {} else {
                                            __assert_fail(
                                                b"ii >= 0 && ii < in_height && jj >= 0 && jj < in_width\0"
                                                    as *const u8 as *const core::ffi::c_char,
                                                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                                    as *const core::ffi::c_char,
                                                856 as core::ffi::c_uint,
                                                (::core::mem::transmute::<
                                                    [u8; 98],
                                                    [core::ffi::c_char; 98],
                                                >(
                                                    *b"void av1_cnn_deconvolve_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        };
                                        sum_0
                                            += *((*layer_config).weights).offset(off_0 as isize)
                                                * *(*input.offset(k_0 as isize))
                                                    .offset((ii_0 * in_stride + jj_0) as isize);
                                    }
                                    m_0 += 1;
                                    off_0 += cstep;
                                }
                                l_0 += 1;
                            }
                            k_0 += 1;
                        }
                        *(*output.offset(i_0 as isize))
                            .offset((u_0 * out_stride + v_0) as isize) = sum_0;
                        v_0 += 1;
                    }
                    u_0 += 1;
                }
                i_0 += 1;
            }
        }
        2 => {
            let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_1 < (*layer_config).out_channels {
                let mut u_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while u_1 < out_height {
                    let mut v_1: core::ffi::c_int = 0 as core::ffi::c_int;
                    while v_1 < out_width {
                        let mut sum_1: core::ffi::c_float = *((*layer_config).bias)
                            .offset(i_1 as isize);
                        let mut k_1: core::ffi::c_int = 0 as core::ffi::c_int;
                        while k_1 < (*layer_config).in_channels {
                            let mut off_1: core::ffi::c_int = k_1
                                * (*layer_config).out_channels + i_1;
                            let mut l_1: core::ffi::c_int = 0 as core::ffi::c_int;
                            while l_1 < (*layer_config).filter_height {
                                let h_1: core::ffi::c_int = u_1 - l_1;
                                let mut m_1: core::ffi::c_int = 0 as core::ffi::c_int;
                                while m_1 < (*layer_config).filter_width {
                                    let w_1: core::ffi::c_int = v_1 - m_1;
                                    if !(h_1 % (*layer_config).skip_height
                                        != 0 as core::ffi::c_int
                                        || w_1 % (*layer_config).skip_width
                                            != 0 as core::ffi::c_int)
                                    {
                                        let ii_1: core::ffi::c_int = h_1
                                            / (*layer_config).skip_height;
                                        let jj_1: core::ffi::c_int = w_1
                                            / (*layer_config).skip_width;
                                        if !(ii_1 < 0 as core::ffi::c_int || ii_1 >= in_height
                                            || jj_1 < 0 as core::ffi::c_int || jj_1 >= in_width)
                                        {
                                            sum_1
                                                += *((*layer_config).weights).offset(off_1 as isize)
                                                    * *(*input.offset(k_1 as isize))
                                                        .offset((ii_1 * in_stride + jj_1) as isize);
                                        }
                                    }
                                    m_1 += 1;
                                    off_1 += cstep;
                                }
                                l_1 += 1;
                            }
                            k_1 += 1;
                        }
                        *(*output.offset(i_1 as isize))
                            .offset((u_1 * out_stride + v_1) as isize) = sum_1;
                        v_1 += 1;
                    }
                    u_1 += 1;
                }
                i_1 += 1;
            }
        }
        _ => {
            if 0 as core::ffi::c_int != 0
                && !(b"Unknown padding type\0" as *const u8 as *const core::ffi::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"0 && \"Unknown padding type\"\0" as *const u8
                        as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                        as *const core::ffi::c_char,
                    896 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 98],
                        [core::ffi::c_char; 98],
                    >(
                        *b"void av1_cnn_deconvolve_c(const float **, int, int, int, const CNN_LAYER_CONFIG *, float **, int)\0",
                    ))
                        .as_ptr(),
                );
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_predict_c(
    mut input: *mut *const core::ffi::c_float,
    mut in_width: core::ffi::c_int,
    mut in_height: core::ffi::c_int,
    mut in_stride: core::ffi::c_int,
    mut cnn_config: *const CNN_CONFIG,
    mut thread_data: *const CNN_THREAD_DATA,
    mut output_struct: *mut CNN_MULTI_OUT,
) -> bool {
    let mut current_block: u64;
    let mut success: bool = false_0 != 0;
    let mut tensor1: [TENSOR; 4] = [
        {
            let mut init = TENSOR {
                allocsize: 0 as core::ffi::c_int,
                channels: 0,
                width: 0,
                height: 0,
                stride: 0,
                buf: [0 as *mut core::ffi::c_float; 256],
            };
            init
        },
        TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        },
        TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        },
        TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        },
    ];
    let mut tensor2: [TENSOR; 4] = [
        {
            let mut init = TENSOR {
                allocsize: 0 as core::ffi::c_int,
                channels: 0,
                width: 0,
                height: 0,
                stride: 0,
                buf: [0 as *mut core::ffi::c_float; 256],
            };
            init
        },
        TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        },
        TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        },
        TENSOR {
            allocsize: 0,
            channels: 0,
            width: 0,
            height: 0,
            stride: 0,
            buf: [0 as *mut core::ffi::c_float; 256],
        },
    ];
    let mut output: [*mut *mut core::ffi::c_float; 4] = [0
        as *mut *mut core::ffi::c_float; 4];
    let mut out_chs: *const core::ffi::c_int = (*output_struct).output_channels;
    output[0 as core::ffi::c_int as usize] = (*output_struct).output_buffer;
    let mut out_idx: core::ffi::c_int = 1 as core::ffi::c_int;
    while out_idx < (*output_struct).num_outputs {
        output[out_idx as usize] = (output[(out_idx - 1 as core::ffi::c_int) as usize])
            .offset(
                *out_chs.offset((out_idx - 1 as core::ffi::c_int) as isize) as isize,
            );
        out_idx += 1;
    }
    let mut i_width: core::ffi::c_int = in_width;
    let mut i_height: core::ffi::c_int = in_height;
    let mut o_width: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut o_height: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut b: core::ffi::c_int = 0 as core::ffi::c_int;
    while b < CNN_MAX_BRANCHES {
        init_tensor(&raw mut *tensor1.as_mut_ptr().offset(b as isize));
        init_tensor(&raw mut *tensor2.as_mut_ptr().offset(b as isize));
        b += 1;
    }
    let mut out_stride: *const core::ffi::c_int = (*output_struct).output_strides;
    let mut layer: core::ffi::c_int = 0 as core::ffi::c_int;
    's_60: loop {
        if !(layer < (*cnn_config).num_layers) {
            current_block = 0;
            break;
        }
        let mut layer_config: *const CNN_LAYER_CONFIG = &*((*cnn_config).layer_config)
            .as_ptr()
            .offset(layer as isize) as *const CNN_LAYER_CONFIG;
        let branch: core::ffi::c_int = (*layer_config).branch;
        let mut branch_config: *const CNN_BRANCH_CONFIG = &(*layer_config).branch_config;
        if layer == 0 as core::ffi::c_int {
            if branch == 0 as core::ffi::c_int {} else {
                __assert_fail(
                    b"branch == 0\0" as *const u8 as *const core::ffi::c_char,
                    b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                        as *const core::ffi::c_char,
                    931 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 117],
                        [core::ffi::c_char; 117],
                    >(
                        *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                    ))
                        .as_ptr(),
                );
            };
            assign_tensor(
                &raw mut *tensor1.as_mut_ptr().offset(branch as isize),
                input as *mut *mut core::ffi::c_float,
                (*layer_config).in_channels,
                in_width,
                in_height,
                in_stride,
            );
        } else {
            swap_tensor(
                &raw mut *tensor1.as_mut_ptr().offset(branch as isize),
                &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
            );
            i_width = tensor1[branch as usize].width;
            i_height = tensor1[branch as usize].height;
        }
        av1_find_cnn_layer_output_size(
            i_width,
            i_height,
            layer_config,
            &raw mut o_width,
            &raw mut o_height,
        );
        let output_num: core::ffi::c_int = (*layer_config).output_num;
        if output_num == -(1 as core::ffi::c_int) {
            if !realloc_tensor(
                &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                (*layer_config).out_channels,
                o_width,
                o_height,
            ) {
                current_block = 1;
                break;
            }
        } else {
            free_tensor(&raw mut *tensor2.as_mut_ptr().offset(branch as isize));
            assign_tensor(
                &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                output[output_num as usize],
                (*layer_config).out_channels,
                o_width,
                o_height,
                *out_stride.offset(output_num as isize),
            );
        }
        if !((*layer_config).branch_combine_type as core::ffi::c_int
            != BRANCH_NOC as core::ffi::c_int)
            || (*branch_config).branches_to_combine & (1 as core::ffi::c_int) << branch
                == 0
        {} else {
            __assert_fail(
                b"IMPLIES(layer_config->branch_combine_type != BRANCH_NOC, !(branch_config->branches_to_combine & (1 << branch)))\0"
                    as *const u8 as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                    as *const core::ffi::c_char,
                961 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 117],
                    [core::ffi::c_char; 117],
                >(
                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                ))
                    .as_ptr(),
            );
        };
        if (*layer_config).branch_copy_type as core::ffi::c_int
            == BRANCH_INPUT as core::ffi::c_int
        {
            if !copy_active_tensor_to_branches(
                &raw mut *tensor1.as_mut_ptr().offset(branch as isize),
                layer_config,
                branch,
                tensor2.as_mut_ptr(),
            ) {
                current_block = 1;
                break;
            }
        }
        if tensor1[branch as usize].channels == (*layer_config).in_channels {} else {
            __assert_fail(
                b"tensor1[branch].channels == layer_config->in_channels\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                    as *const core::ffi::c_char,
                970 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 117],
                    [core::ffi::c_char; 117],
                >(
                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                ))
                    .as_ptr(),
            );
        };
        if tensor2[branch as usize].channels == (*layer_config).out_channels {} else {
            __assert_fail(
                b"tensor2[branch].channels == layer_config->out_channels\0" as *const u8
                    as *const core::ffi::c_char,
                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                    as *const core::ffi::c_char,
                971 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 117],
                    [core::ffi::c_char; 117],
                >(
                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                ))
                    .as_ptr(),
            );
        };
        if (*cnn_config).layer_config[layer as usize].deconvolve == 0 {
            if (*thread_data).num_workers > 1 as core::ffi::c_int {
                convolve_layer_mt(
                    (tensor1[branch as usize].buf).as_mut_ptr()
                        as *mut *const core::ffi::c_float,
                    tensor1[branch as usize].width,
                    tensor1[branch as usize].height,
                    tensor1[branch as usize].stride,
                    layer_config,
                    thread_data,
                    (tensor2[branch as usize].buf).as_mut_ptr(),
                    tensor2[branch as usize].stride,
                );
            } else {
                av1_cnn_convolve(
                    (tensor1[branch as usize].buf).as_mut_ptr()
                        as *mut *const core::ffi::c_float,
                    tensor1[branch as usize].width,
                    tensor1[branch as usize].height,
                    tensor1[branch as usize].stride,
                    layer_config,
                    (tensor2[branch as usize].buf).as_mut_ptr(),
                    tensor2[branch as usize].stride,
                    0 as core::ffi::c_int,
                    1 as core::ffi::c_int,
                );
            }
        } else {
            av1_cnn_deconvolve_c(
                (tensor1[branch as usize].buf).as_mut_ptr()
                    as *mut *const core::ffi::c_float,
                tensor1[branch as usize].width,
                tensor1[branch as usize].height,
                tensor1[branch as usize].stride,
                layer_config,
                (tensor2[branch as usize].buf).as_mut_ptr(),
                tensor2[branch as usize].stride,
            );
        }
        if (*layer_config).branch_copy_type as core::ffi::c_int
            == BRANCH_OUTPUT as core::ffi::c_int
        {
            if !copy_active_tensor_to_branches(
                &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                layer_config,
                branch,
                tensor2.as_mut_ptr(),
            ) {
                current_block = 1;
                break;
            }
        }
        if (*layer_config).branch_combine_type as core::ffi::c_int
            == BRANCH_ADD as core::ffi::c_int
        {
            let mut b_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while b_0 < CNN_MAX_BRANCHES {
                if (*branch_config).branches_to_combine & (1 as core::ffi::c_int) << b_0
                    != 0 && b_0 != branch
                {
                    if check_tensor_equal_size(
                        &raw mut *tensor2.as_mut_ptr().offset(b_0 as isize),
                        &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                    ) != 0
                    {} else {
                        __assert_fail(
                            b"check_tensor_equal_size(&tensor2[b], &tensor2[branch])\0"
                                as *const u8 as *const core::ffi::c_char,
                            b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                as *const core::ffi::c_char,
                            1004 as core::ffi::c_uint,
                            (::core::mem::transmute::<
                                [u8; 117],
                                [core::ffi::c_char; 117],
                            >(
                                *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    av1_cnn_add_c(
                        (tensor2[branch as usize].buf).as_mut_ptr(),
                        tensor2[branch as usize].channels,
                        tensor2[branch as usize].width,
                        tensor2[branch as usize].height,
                        tensor2[branch as usize].stride,
                        (tensor2[b_0 as usize].buf).as_mut_ptr()
                            as *mut *const core::ffi::c_float,
                    );
                }
                b_0 += 1;
            }
        }
        av1_cnn_activate_c(
            (tensor2[branch as usize].buf).as_mut_ptr(),
            tensor2[branch as usize].channels,
            tensor2[branch as usize].width,
            tensor2[branch as usize].height,
            tensor2[branch as usize].stride,
            (*layer_config).activation,
        );
        if !((*layer_config).bn_params.bn_gamma).is_null() {
            av1_cnn_batchnorm_c(
                (tensor2[branch as usize].buf).as_mut_ptr(),
                tensor2[branch as usize].channels,
                tensor2[branch as usize].width,
                tensor2[branch as usize].height,
                tensor2[branch as usize].stride,
                (*layer_config).bn_params.bn_gamma,
                (*layer_config).bn_params.bn_beta,
                (*layer_config).bn_params.bn_mean,
                (*layer_config).bn_params.bn_std,
            );
        }
        if (*layer_config).branch_combine_type as core::ffi::c_int
            == BRANCH_CAT as core::ffi::c_int
        {
            if output_num == -(1 as core::ffi::c_int) {
                let mut b_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while b_1 < CNN_MAX_BRANCHES {
                    if (*branch_config).branches_to_combine
                        & (1 as core::ffi::c_int) << b_1 != 0 && b_1 != branch
                    {
                        if check_tensor_equal_dims(
                            &raw mut *tensor2.as_mut_ptr().offset(b_1 as isize),
                            &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                        ) != 0
                        {} else {
                            __assert_fail(
                                b"check_tensor_equal_dims(&tensor2[b], &tensor2[branch])\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                1030 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 117],
                                    [core::ffi::c_char; 117],
                                >(
                                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        if tensor2[b_1 as usize].channels > 0 as core::ffi::c_int
                        {} else {
                            __assert_fail(
                                b"tensor2[b].channels > 0\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                1031 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 117],
                                    [core::ffi::c_char; 117],
                                >(
                                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        if !concat_tensor(
                            &raw mut *tensor2.as_mut_ptr().offset(b_1 as isize),
                            &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                        ) {
                            current_block = 1;
                            break 's_60;
                        }
                    }
                    b_1 += 1;
                }
            } else {
                let existing_channels: core::ffi::c_int = tensor2[branch as usize]
                    .channels;
                let mut num_chs: core::ffi::c_int = existing_channels;
                let mut b_2: core::ffi::c_int = 0 as core::ffi::c_int;
                while b_2 < CNN_MAX_BRANCHES {
                    if (*branch_config).branches_to_combine
                        & (1 as core::ffi::c_int) << b_2 != 0 && b_2 != branch
                    {
                        if check_tensor_equal_dims(
                            &raw mut *tensor2.as_mut_ptr().offset(b_2 as isize),
                            &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                        ) != 0
                        {} else {
                            __assert_fail(
                                b"check_tensor_equal_dims(&tensor2[b], &tensor2[branch])\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                1040 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 117],
                                    [core::ffi::c_char; 117],
                                >(
                                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        num_chs += tensor2[b_2 as usize].channels;
                    }
                    b_2 += 1;
                }
                assign_tensor(
                    &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                    output[output_num as usize],
                    num_chs,
                    o_width,
                    o_height,
                    *out_stride.offset(output_num as isize),
                );
                num_chs = existing_channels;
                let mut b_3: core::ffi::c_int = 0 as core::ffi::c_int;
                while b_3 < CNN_MAX_BRANCHES {
                    if (*branch_config).branches_to_combine
                        & (1 as core::ffi::c_int) << b_3 != 0 && b_3 != branch
                    {
                        if check_tensor_equal_dims(
                            &raw mut *tensor2.as_mut_ptr().offset(b_3 as isize),
                            &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                        ) != 0
                        {} else {
                            __assert_fail(
                                b"check_tensor_equal_dims(&tensor2[b], &tensor2[branch])\0"
                                    as *const u8 as *const core::ffi::c_char,
                                b"upstream/aom/av1/encoder/cnn.c\0" as *const u8
                                    as *const core::ffi::c_char,
                                1051 as core::ffi::c_uint,
                                (::core::mem::transmute::<
                                    [u8; 117],
                                    [core::ffi::c_char; 117],
                                >(
                                    *b"_Bool av1_cnn_predict_c(const float **, int, int, int, const CNN_CONFIG *, const CNN_THREAD_DATA *, CNN_MULTI_OUT *)\0",
                                ))
                                    .as_ptr(),
                            );
                        };
                        copy_tensor(
                            &raw mut *tensor2.as_mut_ptr().offset(b_3 as isize),
                            tensor2[b_3 as usize].channels,
                            num_chs,
                            &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                        );
                        num_chs += tensor2[b_3 as usize].channels;
                    }
                    b_3 += 1;
                }
            }
        }
        if (*layer_config).branch_copy_type as core::ffi::c_int
            == BRANCH_COMBINED as core::ffi::c_int
        {
            if !copy_active_tensor_to_branches(
                &raw mut *tensor2.as_mut_ptr().offset(branch as isize),
                layer_config,
                branch,
                tensor2.as_mut_ptr(),
            ) {
                current_block = 1;
                break;
            }
        }
        layer += 1;
    }
    match current_block {
        0 => {
            success = true_0 != 0;
        }
        _ => {}
    }
    let mut b_4: core::ffi::c_int = 0 as core::ffi::c_int;
    while b_4 < CNN_MAX_BRANCHES {
        free_tensor(&raw mut *tensor1.as_mut_ptr().offset(b_4 as isize));
        free_tensor(&raw mut *tensor2.as_mut_ptr().offset(b_4 as isize));
        b_4 += 1;
    }
    return success;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_predict_img_multi_out(
    mut dgd: *mut *mut uint8_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut cnn_config: *const CNN_CONFIG,
    mut thread_data: *const CNN_THREAD_DATA,
    mut output: *mut CNN_MULTI_OUT,
) -> bool {
    let max_val: core::ffi::c_float = 255.0f32;
    let in_width: core::ffi::c_int = width
        + 2 as core::ffi::c_int * (*cnn_config).ext_width;
    let in_height: core::ffi::c_int = height
        + 2 as core::ffi::c_int * (*cnn_config).ext_height;
    let in_channels: core::ffi::c_int = (*cnn_config)
        .layer_config[0 as core::ffi::c_int as usize]
        .in_channels;
    let mut inputs: [*mut core::ffi::c_float; 256] = [0 as *mut core::ffi::c_float; 256];
    let mut input_: *mut core::ffi::c_float = aom_malloc(
        ((in_width * in_height * in_channels) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    if input_.is_null() {
        return false_0 != 0;
    }
    let in_stride: core::ffi::c_int = in_width;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < in_channels {
        inputs[c as usize] = input_.offset((c * in_stride * in_height) as isize);
        let mut input: *mut core::ffi::c_float = (inputs[c as usize])
            .offset(((*cnn_config).ext_height * in_stride) as isize)
            .offset((*cnn_config).ext_width as isize);
        if (*cnn_config).strict_bounds != 0 {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < height {
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j < width {
                    *input.offset((i * in_stride + j) as isize) = *(*dgd
                        .offset(c as isize))
                        .offset((i * stride + j) as isize) as core::ffi::c_float
                        / max_val;
                    j += 1;
                }
                i += 1;
            }
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < height {
                let mut j_0: core::ffi::c_int = -(*cnn_config).ext_width;
                while j_0 < 0 as core::ffi::c_int {
                    *input.offset((i_0 * in_stride + j_0) as isize) = *input
                        .offset((i_0 * in_stride) as isize);
                    j_0 += 1;
                }
                let mut j_1: core::ffi::c_int = width;
                while j_1 < width + (*cnn_config).ext_width {
                    *input.offset((i_0 * in_stride + j_1) as isize) = *input
                        .offset(
                            (i_0 * in_stride + width - 1 as core::ffi::c_int) as isize,
                        );
                    j_1 += 1;
                }
                i_0 += 1;
            }
            let mut i_1: core::ffi::c_int = -(*cnn_config).ext_height;
            while i_1 < 0 as core::ffi::c_int {
                memcpy(
                    &raw mut *input
                        .offset((i_1 * in_stride - (*cnn_config).ext_width) as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *input.offset(-(*cnn_config).ext_width as isize)
                        as *mut core::ffi::c_float as *const core::ffi::c_void,
                    (in_width as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_float>() as size_t,
                        ),
                );
                i_1 += 1;
            }
            let mut i_2: core::ffi::c_int = height;
            while i_2 < height + (*cnn_config).ext_height {
                memcpy(
                    &raw mut *input
                        .offset((i_2 * in_stride - (*cnn_config).ext_width) as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *input
                        .offset(
                            ((height - 1 as core::ffi::c_int) * in_stride
                                - (*cnn_config).ext_width) as isize,
                        ) as *mut core::ffi::c_float as *const core::ffi::c_void,
                    (in_width as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_float>() as size_t,
                        ),
                );
                i_2 += 1;
            }
        } else {
            let mut i_3: core::ffi::c_int = -(*cnn_config).ext_height;
            while i_3 < height + (*cnn_config).ext_height {
                let mut j_2: core::ffi::c_int = -(*cnn_config).ext_width;
                while j_2 < width + (*cnn_config).ext_width {
                    *input.offset((i_3 * in_stride + j_2) as isize) = *(*dgd
                        .offset(c as isize))
                        .offset((i_3 * stride + j_2) as isize) as core::ffi::c_float
                        / max_val;
                    j_2 += 1;
                }
                i_3 += 1;
            }
        }
        c += 1;
    }
    let mut success: bool = av1_cnn_predict_c(
        inputs.as_mut_ptr() as *mut *const core::ffi::c_float,
        in_width,
        in_height,
        in_stride,
        cnn_config,
        thread_data,
        output,
    );
    aom_free(input_ as *mut core::ffi::c_void);
    return success;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cnn_predict_img_multi_out_highbd(
    mut dgd: *mut *mut uint16_t,
    mut width: core::ffi::c_int,
    mut height: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut cnn_config: *const CNN_CONFIG,
    mut thread_data: *const CNN_THREAD_DATA,
    mut bit_depth: core::ffi::c_int,
    mut output: *mut CNN_MULTI_OUT,
) -> bool {
    let max_val: core::ffi::c_float = (((1 as core::ffi::c_int) << bit_depth)
        - 1 as core::ffi::c_int) as core::ffi::c_float;
    let in_width: core::ffi::c_int = width
        + 2 as core::ffi::c_int * (*cnn_config).ext_width;
    let in_height: core::ffi::c_int = height
        + 2 as core::ffi::c_int * (*cnn_config).ext_height;
    let in_channels: core::ffi::c_int = (*cnn_config)
        .layer_config[0 as core::ffi::c_int as usize]
        .in_channels;
    let mut inputs: [*mut core::ffi::c_float; 256] = [0 as *mut core::ffi::c_float; 256];
    let mut input_: *mut core::ffi::c_float = aom_malloc(
        ((in_width * in_height * in_channels) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t),
    ) as *mut core::ffi::c_float;
    if input_.is_null() {
        return false_0 != 0;
    }
    let in_stride: core::ffi::c_int = in_width;
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < in_channels {
        inputs[c as usize] = input_.offset((c * in_stride * in_height) as isize);
        let mut input: *mut core::ffi::c_float = (inputs[c as usize])
            .offset(((*cnn_config).ext_height * in_stride) as isize)
            .offset((*cnn_config).ext_width as isize);
        if (*cnn_config).strict_bounds != 0 {
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < height {
                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                while j < width {
                    *input.offset((i * in_stride + j) as isize) = *(*dgd
                        .offset(c as isize))
                        .offset((i * stride + j) as isize) as core::ffi::c_float
                        / max_val;
                    j += 1;
                }
                i += 1;
            }
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_0 < height {
                let mut j_0: core::ffi::c_int = -(*cnn_config).ext_width;
                while j_0 < 0 as core::ffi::c_int {
                    *input.offset((i_0 * in_stride + j_0) as isize) = *input
                        .offset((i_0 * in_stride) as isize);
                    j_0 += 1;
                }
                let mut j_1: core::ffi::c_int = width;
                while j_1 < width + (*cnn_config).ext_width {
                    *input.offset((i_0 * in_stride + j_1) as isize) = *input
                        .offset(
                            (i_0 * in_stride + width - 1 as core::ffi::c_int) as isize,
                        );
                    j_1 += 1;
                }
                i_0 += 1;
            }
            let mut i_1: core::ffi::c_int = -(*cnn_config).ext_height;
            while i_1 < 0 as core::ffi::c_int {
                memcpy(
                    &raw mut *input
                        .offset((i_1 * in_stride - (*cnn_config).ext_width) as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *input.offset(-(*cnn_config).ext_width as isize)
                        as *mut core::ffi::c_float as *const core::ffi::c_void,
                    (in_width as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_float>() as size_t,
                        ),
                );
                i_1 += 1;
            }
            let mut i_2: core::ffi::c_int = height;
            while i_2 < height + (*cnn_config).ext_height {
                memcpy(
                    &raw mut *input
                        .offset((i_2 * in_stride - (*cnn_config).ext_width) as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *input
                        .offset(
                            ((height - 1 as core::ffi::c_int) * in_stride
                                - (*cnn_config).ext_width) as isize,
                        ) as *mut core::ffi::c_float as *const core::ffi::c_void,
                    (in_width as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<core::ffi::c_float>() as size_t,
                        ),
                );
                i_2 += 1;
            }
        } else {
            let mut i_3: core::ffi::c_int = -(*cnn_config).ext_height;
            while i_3 < height + (*cnn_config).ext_height {
                let mut j_2: core::ffi::c_int = -(*cnn_config).ext_width;
                while j_2 < width + (*cnn_config).ext_width {
                    *input.offset((i_3 * in_stride + j_2) as isize) = *(*dgd
                        .offset(c as isize))
                        .offset((i_3 * stride + j_2) as isize) as core::ffi::c_float
                        / max_val;
                    j_2 += 1;
                }
                i_3 += 1;
            }
        }
        c += 1;
    }
    let mut success: bool = av1_cnn_predict_c(
        inputs.as_mut_ptr() as *mut *const core::ffi::c_float,
        in_width,
        in_height,
        in_stride,
        cnn_config,
        thread_data,
        output,
    );
    aom_free(input_ as *mut core::ffi::c_void);
    return success;
}
pub const true_0: core::ffi::c_int = 1 as core::ffi::c_int;
pub const false_0: core::ffi::c_int = 0 as core::ffi::c_int;

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn fabsf(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
