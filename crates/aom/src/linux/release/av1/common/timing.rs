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

// Mechanically translated from upstream/aom/av1/common/timing.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __int8_t = i8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type BITSTREAM_PROFILE = int8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_dec_model_info {
    pub num_units_in_decoding_tick: uint32_t,
    pub encoder_decoder_buffer_delay_length: core::ffi::c_int,
    pub buffer_removal_time_length: core::ffi::c_int,
    pub frame_presentation_time_length: core::ffi::c_int,
}
pub type aom_dec_model_info_t = aom_dec_model_info;
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
pub type aom_dec_model_op_parameters_t = aom_dec_model_op_parameters;
pub const UNDEFINED_RATE: core::ffi::c_int = (1 as core::ffi::c_int)
    << 21 as core::ffi::c_int;
pub const INVALID_RATE: core::ffi::c_int = 0 as core::ffi::c_int;
static mut main_kbps: [int32_t; 32] = [
    1500 as core::ffi::c_int,
    3000 as core::ffi::c_int,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    6000 as core::ffi::c_int,
    10000 as core::ffi::c_int,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    12000 as core::ffi::c_int,
    20000 as core::ffi::c_int,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    30000 as core::ffi::c_int,
    40000 as core::ffi::c_int,
    60000 as core::ffi::c_int,
    60000 as core::ffi::c_int,
    60000 as core::ffi::c_int,
    100000 as core::ffi::c_int,
    160000 as core::ffi::c_int,
    160000 as core::ffi::c_int,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
];
static mut high_kbps: [int32_t; 32] = [
    INVALID_RATE,
    INVALID_RATE,
    INVALID_RATE,
    INVALID_RATE,
    INVALID_RATE,
    INVALID_RATE,
    INVALID_RATE,
    INVALID_RATE,
    30000 as core::ffi::c_int,
    50000 as core::ffi::c_int,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    100000 as core::ffi::c_int,
    160000 as core::ffi::c_int,
    240000 as core::ffi::c_int,
    240000 as core::ffi::c_int,
    240000 as core::ffi::c_int,
    480000 as core::ffi::c_int,
    800000 as core::ffi::c_int,
    800000 as core::ffi::c_int,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
    UNDEFINED_RATE,
];
static mut bitrate_profile_factor: [core::ffi::c_int; 8] = [
    1 as core::ffi::c_int,
    2 as core::ffi::c_int,
    3 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
    0 as core::ffi::c_int,
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_max_level_bitrate(
    mut seq_profile: BITSTREAM_PROFILE,
    mut seq_level_idx: core::ffi::c_int,
    mut seq_tier: core::ffi::c_int,
) -> int64_t {
    let mut bitrate: int64_t = 0;
    if seq_tier != 0 {
        bitrate = (high_kbps[seq_level_idx as usize]
            * bitrate_profile_factor[seq_profile as usize]) as int64_t;
    } else {
        bitrate = (main_kbps[seq_level_idx as usize]
            * bitrate_profile_factor[seq_profile as usize]) as int64_t;
    }
    return bitrate * 1000 as int64_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_aom_dec_model_info(
    mut decoder_model: *mut aom_dec_model_info_t,
) {
    (*decoder_model).encoder_decoder_buffer_delay_length = 16 as core::ffi::c_int;
    (*decoder_model).buffer_removal_time_length = 10 as core::ffi::c_int;
    (*decoder_model).frame_presentation_time_length = 10 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_dec_model_op_parameters(
    mut op_params: *mut aom_dec_model_op_parameters_t,
) {
    (*op_params).decoder_model_param_present_flag = 1 as core::ffi::c_int;
    (*op_params).decoder_buffer_delay = (90000 as core::ffi::c_int
        >> 1 as core::ffi::c_int) as uint32_t;
    (*op_params).encoder_buffer_delay = (90000 as core::ffi::c_int
        >> 1 as core::ffi::c_int) as uint32_t;
    (*op_params).low_delay_mode_flag = 0 as core::ffi::c_int;
    (*op_params).display_model_param_present_flag = 1 as core::ffi::c_int;
    (*op_params).initial_display_delay = 8 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_set_resource_availability_parameters(
    mut op_params: *mut aom_dec_model_op_parameters_t,
) {
    (*op_params).decoder_model_param_present_flag = 0 as core::ffi::c_int;
    (*op_params).decoder_buffer_delay = 70000 as uint32_t;
    (*op_params).encoder_buffer_delay = 20000 as uint32_t;
    (*op_params).low_delay_mode_flag = 0 as core::ffi::c_int;
    (*op_params).display_model_param_present_flag = 1 as core::ffi::c_int;
    (*op_params).initial_display_delay = 8 as core::ffi::c_int;
}
