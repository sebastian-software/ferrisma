/*
 * Copyright © 2018, Niklas Haas
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Mechanically translated from upstream/dav1d/src/fg_apply_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type Dav1dRef;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
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
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ptrdiff_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: int64_t,
    pub duration: int64_t,
    pub offset: int64_t,
    pub size: size_t,
    pub user_data: Dav1dUserData,
}
pub type Dav1dTxfmMode = core::ffi::c_uint;
pub const DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
pub type Dav1dFilterMode = core::ffi::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub type Dav1dAdaptiveBoolean = core::ffi::c_uint;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const DAV1D_OFF: Dav1dAdaptiveBoolean = 0;
pub type Dav1dRestorationType = core::ffi::c_uint;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
pub type Dav1dWarpedMotionType = core::ffi::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
pub type Dav1dPixelLayout = core::ffi::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub type Dav1dFrameType = core::ffi::c_uint;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;
pub type Dav1dColorPrimaries = core::ffi::c_uint;
pub const DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;
pub const DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = 22;
pub const DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = 12;
pub const DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = 11;
pub const DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = 10;
pub const DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = 9;
pub const DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = 8;
pub const DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = 7;
pub const DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = 6;
pub const DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = 5;
pub const DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = 4;
pub const DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = 2;
pub const DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = 1;
pub type Dav1dTransferCharacteristics = core::ffi::c_uint;
pub const DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;
pub const DAV1D_TRC_HLG: Dav1dTransferCharacteristics = 18;
pub const DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics = 17;
pub const DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics = 16;
pub const DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics = 15;
pub const DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics = 14;
pub const DAV1D_TRC_SRGB: Dav1dTransferCharacteristics = 13;
pub const DAV1D_TRC_BT1361: Dav1dTransferCharacteristics = 12;
pub const DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics = 11;
pub const DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics = 10;
pub const DAV1D_TRC_LOG100: Dav1dTransferCharacteristics = 9;
pub const DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics = 8;
pub const DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics = 7;
pub const DAV1D_TRC_BT601: Dav1dTransferCharacteristics = 6;
pub const DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics = 5;
pub const DAV1D_TRC_BT470M: Dav1dTransferCharacteristics = 4;
pub const DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics = 2;
pub const DAV1D_TRC_BT709: Dav1dTransferCharacteristics = 1;
pub type Dav1dMatrixCoefficients = core::ffi::c_uint;
pub const DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;
pub const DAV1D_MC_ICTCP: Dav1dMatrixCoefficients = 14;
pub const DAV1D_MC_CHROMAT_CL: Dav1dMatrixCoefficients = 13;
pub const DAV1D_MC_CHROMAT_NCL: Dav1dMatrixCoefficients = 12;
pub const DAV1D_MC_SMPTE2085: Dav1dMatrixCoefficients = 11;
pub const DAV1D_MC_BT2020_CL: Dav1dMatrixCoefficients = 10;
pub const DAV1D_MC_BT2020_NCL: Dav1dMatrixCoefficients = 9;
pub const DAV1D_MC_SMPTE_YCGCO: Dav1dMatrixCoefficients = 8;
pub const DAV1D_MC_SMPTE240: Dav1dMatrixCoefficients = 7;
pub const DAV1D_MC_BT601: Dav1dMatrixCoefficients = 6;
pub const DAV1D_MC_BT470BG: Dav1dMatrixCoefficients = 5;
pub const DAV1D_MC_FCC: Dav1dMatrixCoefficients = 4;
pub const DAV1D_MC_UNKNOWN: Dav1dMatrixCoefficients = 2;
pub const DAV1D_MC_BT709: Dav1dMatrixCoefficients = 1;
pub const DAV1D_MC_IDENTITY: Dav1dMatrixCoefficients = 0;
pub type Dav1dChromaSamplePosition = core::ffi::c_uint;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContentLightLevel {
    pub max_content_light_level: uint16_t,
    pub max_frame_average_light_level: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMasteringDisplay {
    pub primaries: [[uint16_t; 2]; 3],
    pub white_point: [uint16_t; 2],
    pub max_luminance: uint32_t,
    pub min_luminance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dITUTT35 {
    pub country_code: uint8_t,
    pub country_code_extension_byte: uint8_t,
    pub payload_size: size_t,
    pub payload: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeader {
    pub profile: uint8_t,
    pub max_width: core::ffi::c_int,
    pub max_height: core::ffi::c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: uint8_t,
    pub color_range: uint8_t,
    pub num_operating_points: uint8_t,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; 32],
    pub still_picture: uint8_t,
    pub reduced_still_picture_header: uint8_t,
    pub timing_info_present: uint8_t,
    pub num_units_in_tick: uint32_t,
    pub time_scale: uint32_t,
    pub equal_picture_interval: uint8_t,
    pub num_ticks_per_picture: uint32_t,
    pub decoder_model_info_present: uint8_t,
    pub encoder_decoder_buffer_delay_length: uint8_t,
    pub num_units_in_decoding_tick: uint32_t,
    pub buffer_removal_delay_length: uint8_t,
    pub frame_presentation_delay_length: uint8_t,
    pub display_model_info_present: uint8_t,
    pub width_n_bits: uint8_t,
    pub height_n_bits: uint8_t,
    pub frame_id_numbers_present: uint8_t,
    pub delta_frame_id_n_bits: uint8_t,
    pub frame_id_n_bits: uint8_t,
    pub sb128: uint8_t,
    pub filter_intra: uint8_t,
    pub intra_edge_filter: uint8_t,
    pub inter_intra: uint8_t,
    pub masked_compound: uint8_t,
    pub warped_motion: uint8_t,
    pub dual_filter: uint8_t,
    pub order_hint: uint8_t,
    pub jnt_comp: uint8_t,
    pub ref_frame_mvs: uint8_t,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: uint8_t,
    pub super_res: uint8_t,
    pub cdef: uint8_t,
    pub restoration: uint8_t,
    pub ss_hor: uint8_t,
    pub ss_ver: uint8_t,
    pub monochrome: uint8_t,
    pub color_description_present: uint8_t,
    pub separate_uv_delta_q: uint8_t,
    pub film_grain_present: uint8_t,
    pub operating_parameter_info: [Dav1dSequenceHeaderOperatingParameterInfo; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: uint32_t,
    pub encoder_buffer_delay: uint32_t,
    pub low_delay_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: uint8_t,
    pub minor_level: uint8_t,
    pub initial_display_delay: uint8_t,
    pub idc: uint16_t,
    pub tier: uint8_t,
    pub decoder_model_param_present: uint8_t,
    pub display_model_param_present: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSegmentationData {
    pub delta_q: int16_t,
    pub delta_lf_y_v: int8_t,
    pub delta_lf_y_h: int8_t,
    pub delta_lf_u: int8_t,
    pub delta_lf_v: int8_t,
    pub ref_0: int8_t,
    pub skip: uint8_t,
    pub globalmv: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: uint8_t,
    pub last_active_segid: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [int8_t; 2],
    pub ref_delta: [int8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainData {
    pub seed: core::ffi::c_uint,
    pub num_y_points: core::ffi::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: core::ffi::c_int,
    pub num_uv_points: [core::ffi::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: core::ffi::c_int,
    pub ar_coeff_lag: core::ffi::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 28]; 2],
    pub ar_coeff_shift: uint64_t,
    pub grain_scale_shift: core::ffi::c_int,
    pub uv_mult: [core::ffi::c_int; 2],
    pub uv_luma_mult: [core::ffi::c_int; 2],
    pub uv_offset: [core::ffi::c_int; 2],
    pub overlap_flag: core::ffi::c_int,
    pub clip_to_restricted_range: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_11,
    pub frame_type: Dav1dFrameType,
    pub width: [core::ffi::c_int; 2],
    pub height: core::ffi::c_int,
    pub frame_offset: uint8_t,
    pub temporal_id: uint8_t,
    pub spatial_id: uint8_t,
    pub show_existing_frame: uint8_t,
    pub existing_frame_idx: uint8_t,
    pub frame_id: uint32_t,
    pub frame_presentation_delay: uint32_t,
    pub show_frame: uint8_t,
    pub showable_frame: uint8_t,
    pub error_resilient_mode: uint8_t,
    pub disable_cdf_update: uint8_t,
    pub allow_screen_content_tools: uint8_t,
    pub force_integer_mv: uint8_t,
    pub frame_size_override: uint8_t,
    pub primary_ref_frame: uint8_t,
    pub buffer_removal_time_present: uint8_t,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; 32],
    pub refresh_frame_flags: uint8_t,
    pub render_width: core::ffi::c_int,
    pub render_height: core::ffi::c_int,
    pub super_res: C2RustUnnamed_10,
    pub have_render_size: uint8_t,
    pub allow_intrabc: uint8_t,
    pub frame_ref_short_signaling: uint8_t,
    pub refidx: [int8_t; 7],
    pub hp: uint8_t,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: uint8_t,
    pub use_ref_frame_mvs: uint8_t,
    pub refresh_context: uint8_t,
    pub tiling: C2RustUnnamed_9,
    pub quant: C2RustUnnamed_8,
    pub segmentation: C2RustUnnamed_7,
    pub delta: C2RustUnnamed_4,
    pub all_lossless: uint8_t,
    pub loopfilter: C2RustUnnamed_3,
    pub cdef: C2RustUnnamed_2,
    pub restoration: C2RustUnnamed_1,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: uint8_t,
    pub skip_mode_allowed: uint8_t,
    pub skip_mode_enabled: uint8_t,
    pub skip_mode_refs: [int8_t; 2],
    pub warp_motion: uint8_t,
    pub reduced_txtp_set: uint8_t,
    pub gmv: [Dav1dWarpedMotionParams; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub damping: uint8_t,
    pub n_bits: uint8_t,
    pub y_strength: [uint8_t; 8],
    pub uv_strength: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub level_y: [uint8_t; 2],
    pub level_u: uint8_t,
    pub level_v: uint8_t,
    pub mode_ref_delta_enabled: uint8_t,
    pub mode_ref_delta_update: uint8_t,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub q: C2RustUnnamed_6,
    pub lf: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub present: uint8_t,
    pub res_log2: uint8_t,
    pub multi: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub present: uint8_t,
    pub res_log2: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub enabled: uint8_t,
    pub update_map: uint8_t,
    pub temporal: uint8_t,
    pub update_data: uint8_t,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [uint8_t; 8],
    pub qidx: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub yac: uint8_t,
    pub ydc_delta: int8_t,
    pub udc_delta: int8_t,
    pub uac_delta: int8_t,
    pub vdc_delta: int8_t,
    pub vac_delta: int8_t,
    pub qm: uint8_t,
    pub qm_y: uint8_t,
    pub qm_u: uint8_t,
    pub qm_v: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub uniform: uint8_t,
    pub n_bytes: uint8_t,
    pub min_log2_cols: uint8_t,
    pub max_log2_cols: uint8_t,
    pub log2_cols: uint8_t,
    pub cols: uint8_t,
    pub min_log2_rows: uint8_t,
    pub max_log2_rows: uint8_t,
    pub log2_rows: uint8_t,
    pub rows: uint8_t,
    pub col_start_sb: [uint16_t; 65],
    pub row_start_sb: [uint16_t; 65],
    pub update: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub width_scale_denominator: uint8_t,
    pub enabled: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub data: Dav1dFilmGrainData,
    pub present: uint8_t,
    pub update: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: core::ffi::c_int,
    pub h: core::ffi::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut core::ffi::c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub n_itut_t35: size_t,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub ref_0: *mut Dav1dRef,
    pub allocator_data: *mut core::ffi::c_void,
}
pub type pixel = uint8_t;
pub type entry = int8_t;
pub type generate_grain_y_fn = Option<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> (),
>;
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type fguv_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        *const pixel,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        core::ffi::c_int,
        core::ffi::c_int,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
pub const FG_BLOCK_SIZE: core::ffi::c_int = 32 as core::ffi::c_int;
pub const SCALING_SIZE: core::ffi::c_int = 256 as core::ffi::c_int;
unsafe extern "C" fn generate_scaling(
    bitdepth: core::ffi::c_int,
    mut points: *const [uint8_t; 2],
    num: core::ffi::c_int,
    mut scaling: *mut uint8_t,
) {
    let shift_x: core::ffi::c_int = 0 as core::ffi::c_int;
    let scaling_size: core::ffi::c_int = SCALING_SIZE;
    if num == 0 as core::ffi::c_int {
        memset(
            scaling as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            scaling_size as size_t,
        );
        return;
    }
    memset(
        scaling as *mut core::ffi::c_void,
        (*points.offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
            as core::ffi::c_int,
        (((*points
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            as core::ffi::c_int) << shift_x) as size_t,
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num - 1 as core::ffi::c_int {
        let bx: core::ffi::c_int = (*points
            .offset(i as isize))[0 as core::ffi::c_int as usize] as core::ffi::c_int;
        let by: core::ffi::c_int = (*points
            .offset(i as isize))[1 as core::ffi::c_int as usize] as core::ffi::c_int;
        let ex: core::ffi::c_int = (*points
            .offset(
                (i + 1 as core::ffi::c_int) as isize,
            ))[0 as core::ffi::c_int as usize] as core::ffi::c_int;
        let ey: core::ffi::c_int = (*points
            .offset(
                (i + 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize] as core::ffi::c_int;
        let dx: core::ffi::c_int = ex - bx;
        let dy: core::ffi::c_int = ey - by;
        if dx > 0 as core::ffi::c_int {} else {
            __assert_fail(
                b"dx > 0\0" as *const u8 as *const core::ffi::c_char,
                b"../dav1d-native-source/src/fg_apply_tmpl.c\0" as *const u8
                    as *const core::ffi::c_char,
                70 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 77],
                    [core::ffi::c_char; 77],
                >(
                    *b"void generate_scaling(const int, const uint8_t (*)[2], const int, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        };
        let delta: core::ffi::c_int = dy
            * ((0x10000 as core::ffi::c_int + (dx >> 1 as core::ffi::c_int)) / dx);
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut d: core::ffi::c_int = 0x8000 as core::ffi::c_int;
        while x < dx {
            *scaling.offset((bx + x << shift_x) as isize) = (by
                + (d >> 16 as core::ffi::c_int)) as uint8_t;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n: core::ffi::c_int = ((*points
        .offset((num - 1 as core::ffi::c_int) as isize))[0 as core::ffi::c_int as usize]
        as core::ffi::c_int) << shift_x;
    memset(
        &raw mut *scaling.offset(n as isize) as *mut uint8_t as *mut core::ffi::c_void,
        (*points
            .offset(
                (num - 1 as core::ffi::c_int) as isize,
            ))[1 as core::ffi::c_int as usize] as core::ffi::c_int,
        (scaling_size - n) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_prep_grain_8bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *mut [uint8_t; 256],
    mut grain_lut: *mut [[entry; 82]; 74],
) {
    let data: *const Dav1dFilmGrainData = &raw mut (*(*out).frame_hdr).film_grain.data;
    ((*dsp).generate_grain_y)
        .expect(
            "non-null function pointer",
        )((*grain_lut.offset(0 as core::ffi::c_int as isize)).as_mut_ptr(), data);
    if (*data).num_uv_points[0 as core::ffi::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as core::ffi::c_uint)
            .wrapping_sub(1 as core::ffi::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(1 as core::ffi::c_int as isize)).as_mut_ptr(),
            (*grain_lut.offset(0 as core::ffi::c_int as isize)).as_mut_ptr()
                as *const [entry; 82],
            data,
            0 as intptr_t,
        );
    }
    if (*data).num_uv_points[1 as core::ffi::c_int as usize] != 0
        || (*data).chroma_scaling_from_luma != 0
    {
        ((*dsp)
            .generate_grain_uv[((*in_0).p.layout as core::ffi::c_uint)
            .wrapping_sub(1 as core::ffi::c_uint) as usize])
            .expect(
                "non-null function pointer",
            )(
            (*grain_lut.offset(2 as core::ffi::c_int as isize)).as_mut_ptr(),
            (*grain_lut.offset(0 as core::ffi::c_int as isize)).as_mut_ptr()
                as *const [entry; 82],
            data,
            1 as intptr_t,
        );
    }
    if (*data).num_y_points != 0 || (*data).chroma_scaling_from_luma != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).y_points).as_ptr() as *const [uint8_t; 2],
            (*data).num_y_points,
            (*scaling.offset(0 as core::ffi::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[0 as core::ffi::c_int as usize] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[0 as core::ffi::c_int as usize]).as_ptr()
                as *const [uint8_t; 2],
            (*data).num_uv_points[0 as core::ffi::c_int as usize],
            (*scaling.offset(1 as core::ffi::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*data).num_uv_points[1 as core::ffi::c_int as usize] != 0 {
        generate_scaling(
            (*in_0).p.bpc,
            ((*data).uv_points[1 as core::ffi::c_int as usize]).as_ptr()
                as *const [uint8_t; 2],
            (*data).num_uv_points[1 as core::ffi::c_int as usize],
            (*scaling.offset(2 as core::ffi::c_int as isize)).as_mut_ptr(),
        );
    }
    if (*out).stride[0 as core::ffi::c_int as usize]
        == (*in_0).stride[0 as core::ffi::c_int as usize]
    {} else {
        __assert_fail(
            b"out->stride[0] == in->stride[0]\0" as *const u8
                as *const core::ffi::c_char,
            b"../dav1d-native-source/src/fg_apply_tmpl.c\0" as *const u8
                as *const core::ffi::c_char,
            129 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 151],
                [core::ffi::c_char; 151],
            >(
                *b"void dav1d_prep_grain_8bpc(const Dav1dFilmGrainDSPContext *const, Dav1dPicture *const, const Dav1dPicture *const, uint8_t (*)[256], entry (*)[74][82])\0",
            ))
                .as_ptr(),
        );
    };
    if (*data).num_y_points == 0 {
        let stride: ptrdiff_t = (*out).stride[0 as core::ffi::c_int as usize];
        let sz: ptrdiff_t = (*out).p.h as ptrdiff_t * stride;
        if sz < 0 as ptrdiff_t {
            memcpy(
                ((*out).data[0 as core::ffi::c_int as usize] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *mut core::ffi::c_void,
                ((*in_0).data[0 as core::ffi::c_int as usize] as *mut uint8_t)
                    .offset(sz as isize)
                    .offset(-(stride as isize)) as *const core::ffi::c_void,
                -sz as size_t,
            );
        } else {
            memcpy(
                (*out).data[0 as core::ffi::c_int as usize],
                (*in_0).data[0 as core::ffi::c_int as usize],
                sz as size_t,
            );
        }
    }
    if (*in_0).p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int as core::ffi::c_uint
        && (*data).chroma_scaling_from_luma == 0
    {
        if (*out).stride[1 as core::ffi::c_int as usize]
            == (*in_0).stride[1 as core::ffi::c_int as usize]
        {} else {
            __assert_fail(
                b"out->stride[1] == in->stride[1]\0" as *const u8
                    as *const core::ffi::c_char,
                b"../dav1d-native-source/src/fg_apply_tmpl.c\0" as *const u8
                    as *const core::ffi::c_char,
                141 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 151],
                    [core::ffi::c_char; 151],
                >(
                    *b"void dav1d_prep_grain_8bpc(const Dav1dFilmGrainDSPContext *const, Dav1dPicture *const, const Dav1dPicture *const, uint8_t (*)[256], entry (*)[74][82])\0",
                ))
                    .as_ptr(),
            );
        };
        let ss_ver: core::ffi::c_int = ((*in_0).p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let stride_0: ptrdiff_t = (*out).stride[1 as core::ffi::c_int as usize];
        let sz_0: ptrdiff_t = ((*out).p.h + ss_ver >> ss_ver) as ptrdiff_t * stride_0;
        if sz_0 < 0 as ptrdiff_t {
            if (*data).num_uv_points[0 as core::ffi::c_int as usize] == 0 {
                memcpy(
                    ((*out).data[1 as core::ffi::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut core::ffi::c_void,
                    ((*in_0).data[1 as core::ffi::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const core::ffi::c_void,
                    -sz_0 as size_t,
                );
            }
            if (*data).num_uv_points[1 as core::ffi::c_int as usize] == 0 {
                memcpy(
                    ((*out).data[2 as core::ffi::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *mut core::ffi::c_void,
                    ((*in_0).data[2 as core::ffi::c_int as usize] as *mut uint8_t)
                        .offset(sz_0 as isize)
                        .offset(-(stride_0 as isize)) as *const core::ffi::c_void,
                    -sz_0 as size_t,
                );
            }
        } else {
            if (*data).num_uv_points[0 as core::ffi::c_int as usize] == 0 {
                memcpy(
                    (*out).data[1 as core::ffi::c_int as usize],
                    (*in_0).data[1 as core::ffi::c_int as usize],
                    sz_0 as size_t,
                );
            }
            if (*data).num_uv_points[1 as core::ffi::c_int as usize] == 0 {
                memcpy(
                    (*out).data[2 as core::ffi::c_int as usize],
                    (*in_0).data[2 as core::ffi::c_int as usize],
                    sz_0 as size_t,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_row_8bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
    mut scaling: *const [uint8_t; 256],
    mut grain_lut: *const [[entry; 82]; 74],
    row: core::ffi::c_int,
) {
    let data: *const Dav1dFilmGrainData = &raw mut (*(*out).frame_hdr).film_grain.data;
    let ss_y: core::ffi::c_int = ((*in_0).p.layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_x: core::ffi::c_int = ((*in_0).p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let cpw: core::ffi::c_int = (*out).p.w + ss_x >> ss_x;
    let is_id: core::ffi::c_int = ((*(*out).seq_hdr).mtrx as core::ffi::c_uint
        == DAV1D_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let luma_src: *mut pixel = ((*in_0).data[0 as core::ffi::c_int as usize]
        as *mut pixel)
        .offset(
            ((row * FG_BLOCK_SIZE) as ptrdiff_t
                * (*in_0).stride[0 as core::ffi::c_int as usize]) as isize,
        );
    if (*data).num_y_points != 0 {
        let bh: core::ffi::c_int = imin((*out).p.h - row * FG_BLOCK_SIZE, FG_BLOCK_SIZE)
            as core::ffi::c_int;
        ((*dsp).fgy_32x32xn)
            .expect(
                "non-null function pointer",
            )(
            ((*out).data[0 as core::ffi::c_int as usize] as *mut pixel)
                .offset(
                    ((row * FG_BLOCK_SIZE) as ptrdiff_t
                        * (*out).stride[0 as core::ffi::c_int as usize]) as isize,
                ),
            luma_src,
            (*out).stride[0 as core::ffi::c_int as usize],
            data,
            (*out).p.w as size_t,
            (*scaling.offset(0 as core::ffi::c_int as isize)).as_ptr(),
            (*grain_lut.offset(0 as core::ffi::c_int as isize)).as_ptr(),
            bh,
            row,
        );
    }
    if (*data).num_uv_points[0 as core::ffi::c_int as usize] == 0
        && (*data).num_uv_points[1 as core::ffi::c_int as usize] == 0
        && (*data).chroma_scaling_from_luma == 0
    {
        return;
    }
    let bh_0: core::ffi::c_int = imin((*out).p.h - row * FG_BLOCK_SIZE, FG_BLOCK_SIZE)
        as core::ffi::c_int + ss_y >> ss_y;
    if (*out).p.w & ss_x != 0 {
        let mut ptr: *mut pixel = luma_src;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < bh_0 {
            *ptr.offset((*out).p.w as isize) = *ptr
                .offset(((*out).p.w - 1 as core::ffi::c_int) as isize);
            ptr = ptr
                .offset(
                    ((*in_0).stride[0 as core::ffi::c_int as usize] << ss_y) as isize,
                );
            y += 1;
        }
    }
    let uv_off: ptrdiff_t = (row * FG_BLOCK_SIZE) as ptrdiff_t
        * (*out).stride[1 as core::ffi::c_int as usize] >> ss_y;
    if (*data).chroma_scaling_from_luma != 0 {
        let mut pl: core::ffi::c_int = 0 as core::ffi::c_int;
        while pl < 2 as core::ffi::c_int {
            ((*dsp)
                .fguv_32x32xn[((*in_0).p.layout as core::ffi::c_uint)
                .wrapping_sub(1 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                ((*out).data[(1 as core::ffi::c_int + pl) as usize] as *mut pixel)
                    .offset(uv_off as isize),
                ((*in_0).data[(1 as core::ffi::c_int + pl) as usize] as *const pixel)
                    .offset(uv_off as isize),
                (*in_0).stride[1 as core::ffi::c_int as usize],
                data,
                cpw as size_t,
                (*scaling.offset(0 as core::ffi::c_int as isize)).as_ptr(),
                (*grain_lut.offset((1 as core::ffi::c_int + pl) as isize)).as_ptr(),
                bh_0,
                row,
                luma_src,
                (*in_0).stride[0 as core::ffi::c_int as usize],
                pl,
                is_id,
            );
            pl += 1;
        }
    } else {
        let mut pl_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while pl_0 < 2 as core::ffi::c_int {
            if (*data).num_uv_points[pl_0 as usize] != 0 {
                ((*dsp)
                    .fguv_32x32xn[((*in_0).p.layout as core::ffi::c_uint)
                    .wrapping_sub(1 as core::ffi::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*out).data[(1 as core::ffi::c_int + pl_0) as usize] as *mut pixel)
                        .offset(uv_off as isize),
                    ((*in_0).data[(1 as core::ffi::c_int + pl_0) as usize]
                        as *const pixel)
                        .offset(uv_off as isize),
                    (*in_0).stride[1 as core::ffi::c_int as usize],
                    data,
                    cpw as size_t,
                    (*scaling.offset((1 as core::ffi::c_int + pl_0) as isize)).as_ptr(),
                    (*grain_lut.offset((1 as core::ffi::c_int + pl_0) as isize))
                        .as_ptr(),
                    bh_0,
                    row,
                    luma_src,
                    (*in_0).stride[0 as core::ffi::c_int as usize],
                    pl_0,
                    is_id,
                );
            }
            pl_0 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain_8bpc(
    dsp: *const Dav1dFilmGrainDSPContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) {
    let mut grain_lut: [[[entry; 82]; 74]; 3] = [[[0; 82]; 74]; 3];
    let mut scaling: [[uint8_t; 256]; 3] = [[0; 256]; 3];
    let rows: core::ffi::c_int = ((*out).p.h + FG_BLOCK_SIZE - 1 as core::ffi::c_int)
        / FG_BLOCK_SIZE;
    dav1d_prep_grain_8bpc(dsp, out, in_0, scaling.as_mut_ptr(), grain_lut.as_mut_ptr());
    let mut row: core::ffi::c_int = 0 as core::ffi::c_int;
    while row < rows {
        dav1d_apply_grain_row_8bpc(
            dsp,
            out,
            in_0,
            scaling.as_mut_ptr() as *const [uint8_t; 256],
            grain_lut.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
