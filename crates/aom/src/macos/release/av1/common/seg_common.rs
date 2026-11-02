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

// Mechanically translated from upstream/aom/av1/common/seg_common.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const SEG_LVL_MAX: C2RustUnnamed = 8;
pub const SEG_LVL_GLOBALMV: C2RustUnnamed = 7;
pub const SEG_LVL_SKIP: C2RustUnnamed = 6;
pub const SEG_LVL_REF_FRAME: C2RustUnnamed = 5;
pub const SEG_LVL_ALT_LF_V: C2RustUnnamed = 4;
pub const SEG_LVL_ALT_LF_U: C2RustUnnamed = 3;
pub const SEG_LVL_ALT_LF_Y_H: C2RustUnnamed = 2;
pub const SEG_LVL_ALT_LF_Y_V: C2RustUnnamed = 1;
pub const SEG_LVL_ALT_Q: C2RustUnnamed = 0;
pub type SEG_LVL_FEATURES = uint8_t;
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
pub const MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAXQ: core::ffi::c_int = 255 as core::ffi::c_int;
pub const MAX_LOOP_FILTER: core::ffi::c_int = 63 as core::ffi::c_int;
static mut seg_feature_data_signed: [core::ffi::c_int; 8] = [
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    1 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
static mut seg_feature_data_max: [core::ffi::c_int; 8] = [
    MAXQ,
    MAX_LOOP_FILTER,
    MAX_LOOP_FILTER,
    MAX_LOOP_FILTER,
    MAX_LOOP_FILTER,
    7 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_clearall_segfeatures(mut seg: *mut segmentation) {
    memset(
        &raw mut (*seg).feature_data as *mut [[int16_t; 8]; 8] as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[[int16_t; 8]; 8]>() as size_t,
    );
    memset(
        &raw mut (*seg).feature_mask as *mut [core::ffi::c_uint; 8]
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[core::ffi::c_uint; 8]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_calculate_segdata(mut seg: *mut segmentation) {
    (*seg).segid_preskip = 0 as uint8_t;
    (*seg).last_active_segid = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < MAX_SEGMENTS {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < SEG_LVL_MAX as core::ffi::c_int {
            if (*seg).feature_mask[i as usize]
                & ((1 as core::ffi::c_int) << j) as core::ffi::c_uint != 0
            {
                (*seg).segid_preskip = ((*seg).segid_preskip as core::ffi::c_int
                    | (j >= SEG_LVL_REF_FRAME as core::ffi::c_int) as core::ffi::c_int)
                    as uint8_t;
                (*seg).last_active_segid = i;
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_enable_segfeature(
    mut seg: *mut segmentation,
    mut segment_id: core::ffi::c_int,
    mut feature_id: SEG_LVL_FEATURES,
) {
    (*seg).feature_mask[segment_id as usize]
        |= ((1 as core::ffi::c_int) << feature_id as core::ffi::c_int)
            as core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_seg_feature_data_max(
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return seg_feature_data_max[feature_id as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_is_segfeature_signed(
    mut feature_id: SEG_LVL_FEATURES,
) -> core::ffi::c_int {
    return seg_feature_data_signed[feature_id as usize];
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_segdata(
    mut seg: *mut segmentation,
    mut segment_id: core::ffi::c_int,
    mut feature_id: SEG_LVL_FEATURES,
    mut seg_data: core::ffi::c_int,
) {
    seg_data < 0 as core::ffi::c_int;
    (*seg).feature_data[segment_id as usize][feature_id as usize] = seg_data as int16_t;
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
