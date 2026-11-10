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

// Mechanically translated from upstream/aom/av1/common/frame_buffers.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type uint8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_codec_frame_buffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub priv_0: *mut core::ffi::c_void,
}
pub type aom_codec_frame_buffer_t = aom_codec_frame_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBuffer {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub in_use: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InternalFrameBufferList {
    pub num_internal_frame_buffers: core::ffi::c_int,
    pub int_fb: *mut InternalFrameBuffer,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AOM_MAXIMUM_WORK_BUFFERS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const AOM_MAXIMUM_REF_BUFFERS: core::ffi::c_int = 8 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_alloc_internal_frame_buffers(
    mut list: *mut InternalFrameBufferList,
) -> core::ffi::c_int {
    if list.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 33],
                [core::ffi::c_char; 33],
            >(*b"av1_alloc_internal_frame_buffers\0"))
                .as_ptr(),
            b"frame_buffers.c\0" as *const u8 as *const core::ffi::c_char,
            18 as core::ffi::c_int,
            b"list != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    av1_free_internal_frame_buffers(list);
    (*list).num_internal_frame_buffers = AOM_MAXIMUM_REF_BUFFERS
        + AOM_MAXIMUM_WORK_BUFFERS;
    (*list).int_fb = aom_calloc(
        (*list).num_internal_frame_buffers as size_t,
        ::core::mem::size_of::<InternalFrameBuffer>() as size_t,
    ) as *mut InternalFrameBuffer;
    if ((*list).int_fb).is_null() {
        (*list).num_internal_frame_buffers = 0 as core::ffi::c_int;
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_free_internal_frame_buffers(
    mut list: *mut InternalFrameBufferList,
) {
    let mut i: core::ffi::c_int = 0;
    if list.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 32],
                [core::ffi::c_char; 32],
            >(*b"av1_free_internal_frame_buffers\0"))
                .as_ptr(),
            b"frame_buffers.c\0" as *const u8 as *const core::ffi::c_char,
            35 as core::ffi::c_int,
            b"list != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    i = 0 as core::ffi::c_int;
    while i < (*list).num_internal_frame_buffers {
        aom_free((*((*list).int_fb).offset(i as isize)).data as *mut core::ffi::c_void);
        let fresh0 = &raw mut (*((*list).int_fb).offset(i as isize)).data;
        *fresh0 = 0 as *mut uint8_t;
        i += 1;
    }
    aom_free((*list).int_fb as *mut core::ffi::c_void);
    (*list).int_fb = 0 as *mut InternalFrameBuffer;
    (*list).num_internal_frame_buffers = 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_zero_unused_internal_frame_buffers(
    mut list: *mut InternalFrameBufferList,
) {
    let mut i: core::ffi::c_int = 0;
    if list.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"av1_zero_unused_internal_frame_buffers\0"))
                .as_ptr(),
            b"frame_buffers.c\0" as *const u8 as *const core::ffi::c_char,
            49 as core::ffi::c_int,
            b"list != NULL\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    i = 0 as core::ffi::c_int;
    while i < (*list).num_internal_frame_buffers {
        if !((*((*list).int_fb).offset(i as isize)).data).is_null()
            && (*((*list).int_fb).offset(i as isize)).in_use == 0
        {
            memset(
                (*((*list).int_fb).offset(i as isize)).data as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (*((*list).int_fb).offset(i as isize)).size,
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_frame_buffer(
    mut cb_priv: *mut core::ffi::c_void,
    mut min_size: size_t,
    mut fb: *mut aom_codec_frame_buffer_t,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let int_fb_list: *mut InternalFrameBufferList = cb_priv
        as *mut InternalFrameBufferList;
    if int_fb_list.is_null() {
        return -(1 as core::ffi::c_int);
    }
    i = 0 as core::ffi::c_int;
    while i < (*int_fb_list).num_internal_frame_buffers {
        if (*((*int_fb_list).int_fb).offset(i as isize)).in_use == 0 {
            break;
        }
        i += 1;
    }
    if i == (*int_fb_list).num_internal_frame_buffers {
        return -(1 as core::ffi::c_int);
    }
    if (*((*int_fb_list).int_fb).offset(i as isize)).size < min_size {
        aom_free(
            (*((*int_fb_list).int_fb).offset(i as isize)).data as *mut core::ffi::c_void,
        );
        let fresh1 = &raw mut (*((*int_fb_list).int_fb).offset(i as isize)).data;
        *fresh1 = aom_calloc(1 as size_t, min_size) as *mut uint8_t;
        if ((*((*int_fb_list).int_fb).offset(i as isize)).data).is_null() {
            (*((*int_fb_list).int_fb).offset(i as isize)).size = 0 as size_t;
            return -(1 as core::ffi::c_int);
        }
        (*((*int_fb_list).int_fb).offset(i as isize)).size = min_size;
    }
    (*fb).data = (*((*int_fb_list).int_fb).offset(i as isize)).data;
    (*fb).size = (*((*int_fb_list).int_fb).offset(i as isize)).size;
    (*((*int_fb_list).int_fb).offset(i as isize)).in_use = 1 as core::ffi::c_int;
    (*fb).priv_0 = &raw mut *((*int_fb_list).int_fb).offset(i as isize)
        as *mut InternalFrameBuffer as *mut core::ffi::c_void;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_release_frame_buffer(
    mut cb_priv: *mut core::ffi::c_void,
    mut fb: *mut aom_codec_frame_buffer_t,
) -> core::ffi::c_int {
    let int_fb: *mut InternalFrameBuffer = (*fb).priv_0 as *mut InternalFrameBuffer;
    if !int_fb.is_null() {
        (*int_fb).in_use = 0 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
