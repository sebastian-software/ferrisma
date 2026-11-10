/*
 * Copyright © 2020, VideoLAN and dav1d authors
 * Copyright © 2020, Two Orioles, LLC
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

// Mechanically translated from upstream/dav1d/src/refmvs.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn posix_memalign(
        __memptr: *mut *mut core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> core::ffi::c_int;
    fn abs(__x: core::ffi::c_int) -> core::ffi::c_int;
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
    fn free(__ptr: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type ptrdiff_t = isize;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
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
pub type BlockSize = core::ffi::c_uint;
pub const N_BS_SIZES: BlockSize = 22;
pub const BS_4x4: BlockSize = 21;
pub const BS_4x8: BlockSize = 20;
pub const BS_4x16: BlockSize = 19;
pub const BS_8x4: BlockSize = 18;
pub const BS_8x8: BlockSize = 17;
pub const BS_8x16: BlockSize = 16;
pub const BS_8x32: BlockSize = 15;
pub const BS_16x4: BlockSize = 14;
pub const BS_16x8: BlockSize = 13;
pub const BS_16x16: BlockSize = 12;
pub const BS_16x32: BlockSize = 11;
pub const BS_16x64: BlockSize = 10;
pub const BS_32x8: BlockSize = 9;
pub const BS_32x16: BlockSize = 8;
pub const BS_32x32: BlockSize = 7;
pub const BS_32x64: BlockSize = 6;
pub const BS_64x16: BlockSize = 5;
pub const BS_64x32: BlockSize = 4;
pub const BS_64x64: BlockSize = 3;
pub const BS_64x128: BlockSize = 2;
pub const BS_128x64: BlockSize = 1;
pub const BS_128x128: BlockSize = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub y: int16_t,
    pub x: int16_t,
}
pub type EdgeFlags = core::ffi::c_uint;
pub const EDGE_ALL_TR_AND_BL: EdgeFlags = 63;
pub const EDGE_ALL_LEFT_HAS_BOTTOM: EdgeFlags = 56;
pub const EDGE_ALL_TOP_HAS_RIGHT: EdgeFlags = 7;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub ref_0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union refmvs_refpair {
    pub ref_0: [int8_t; 2],
    pub pair: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union refmvs_mvpair {
    pub mv: [mv; 2],
    pub n: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct refmvs_block {
    pub mv: refmvs_mvpair,
    pub ref_0: refmvs_refpair,
    pub bs: uint8_t,
    pub mf: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_frame {
    pub frm_hdr: *const Dav1dFrameHeader,
    pub iw4: core::ffi::c_int,
    pub ih4: core::ffi::c_int,
    pub iw8: core::ffi::c_int,
    pub ih8: core::ffi::c_int,
    pub sbsz: core::ffi::c_int,
    pub use_ref_frame_mvs: core::ffi::c_int,
    pub sign_bias: [uint8_t; 7],
    pub mfmv_sign: [uint8_t; 7],
    pub pocdiff: [int8_t; 7],
    pub mfmv_ref: [uint8_t; 3],
    pub mfmv_ref2cur: [int8_t; 3],
    pub mfmv_ref2ref: [[uint8_t; 7]; 3],
    pub n_mfmvs: core::ffi::c_int,
    pub n_blocks: core::ffi::c_int,
    pub rp: *mut refmvs_temporal_block,
    pub rp_ref: *const *mut refmvs_temporal_block,
    pub rp_proj: *mut refmvs_temporal_block,
    pub rp_stride: ptrdiff_t,
    pub r: *mut refmvs_block,
    pub n_tile_threads: core::ffi::c_int,
    pub n_frame_threads: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: C2RustUnnamed_13,
    pub tile_row: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub start: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_candidate {
    pub mv: refmvs_mvpair,
    pub weight: core::ffi::c_int,
}
pub type load_tmvs_fn = Option<
    unsafe extern "C" fn(
        *const refmvs_frame,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type save_tmvs_fn = Option<
    unsafe extern "C" fn(
        *mut refmvs_temporal_block,
        ptrdiff_t,
        *const *mut refmvs_block,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type splat_mv_fn = Option<
    unsafe extern "C" fn(
        *mut *mut refmvs_block,
        *const refmvs_block,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRefmvsDSPContext {
    pub load_tmvs: load_tmvs_fn,
    pub save_tmvs: save_tmvs_fn,
    pub splat_mv: splat_mv_fn,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn imax(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: core::ffi::c_int,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) -> core::ffi::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign(
    v: core::ffi::c_int,
    s: core::ffi::c_int,
) -> core::ffi::c_int {
    return if s < 0 as core::ffi::c_int { -v } else { v };
}
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
pub const INVALID_REF2CUR: core::ffi::c_int = -(32 as core::ffi::c_int);
unsafe extern "C" fn add_spatial_candidate(
    mvstack: *mut refmvs_candidate,
    cnt: *mut core::ffi::c_int,
    weight: core::ffi::c_int,
    b: *const refmvs_block,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    have_newmv_match: *mut core::ffi::c_int,
    have_refmv_match: *mut core::ffi::c_int,
) {
    if (*b).mv.mv[0 as core::ffi::c_int as usize].n == INVALID_MV as uint32_t {
        return;
    }
    if ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
        == -(1 as core::ffi::c_int)
    {
        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
        while n < 2 as core::ffi::c_int {
            if (*b).ref_0.ref_0[n as usize] as core::ffi::c_int
                == ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            {
                let cand_mv: mv = if (*b).mf as core::ffi::c_int & 1 as core::ffi::c_int
                    != 0
                    && (*gmv.offset(0 as core::ffi::c_int as isize)).n
                        != INVALID_MV as uint32_t
                {
                    *gmv.offset(0 as core::ffi::c_int as isize)
                } else {
                    (*b).mv.mv[n as usize]
                };
                *have_refmv_match = 1 as core::ffi::c_int;
                *have_newmv_match
                    |= (*b).mf as core::ffi::c_int >> 1 as core::ffi::c_int;
                let last: core::ffi::c_int = *cnt;
                let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                while m < last {
                    if (*mvstack.offset(m as isize))
                        .mv
                        .mv[0 as core::ffi::c_int as usize]
                        .n == cand_mv.n
                    {
                        (*mvstack.offset(m as isize)).weight += weight;
                        return;
                    }
                    m += 1;
                }
                if last < 8 as core::ffi::c_int {
                    (*mvstack.offset(last as isize))
                        .mv
                        .mv[0 as core::ffi::c_int as usize] = cand_mv;
                    (*mvstack.offset(last as isize)).weight = weight;
                    *cnt = last + 1 as core::ffi::c_int;
                }
                return;
            }
            n += 1;
        }
    } else if (*b).ref_0.pair as core::ffi::c_int == ref_0.pair as core::ffi::c_int {
        let cand_mv_0: refmvs_mvpair = refmvs_mvpair {
            mv: [
                if (*b).mf as core::ffi::c_int & 1 as core::ffi::c_int != 0
                    && (*gmv.offset(0 as core::ffi::c_int as isize)).n
                        != INVALID_MV as uint32_t
                {
                    *gmv.offset(0 as core::ffi::c_int as isize)
                } else {
                    (*b).mv.mv[0 as core::ffi::c_int as usize]
                },
                if (*b).mf as core::ffi::c_int & 1 as core::ffi::c_int != 0
                    && (*gmv.offset(1 as core::ffi::c_int as isize)).n
                        != INVALID_MV as uint32_t
                {
                    *gmv.offset(1 as core::ffi::c_int as isize)
                } else {
                    (*b).mv.mv[1 as core::ffi::c_int as usize]
                },
            ],
        };
        *have_refmv_match = 1 as core::ffi::c_int;
        *have_newmv_match |= (*b).mf as core::ffi::c_int >> 1 as core::ffi::c_int;
        let last_0: core::ffi::c_int = *cnt;
        let mut n_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while n_0 < last_0 {
            if (*mvstack.offset(n_0 as isize)).mv.n == cand_mv_0.n {
                (*mvstack.offset(n_0 as isize)).weight += weight;
                return;
            }
            n_0 += 1;
        }
        if last_0 < 8 as core::ffi::c_int {
            (*mvstack.offset(last_0 as isize)).mv = cand_mv_0;
            (*mvstack.offset(last_0 as isize)).weight = weight;
            *cnt = last_0 + 1 as core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn scan_row(
    mvstack: *mut refmvs_candidate,
    cnt: *mut core::ffi::c_int,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    mut b: *const refmvs_block,
    bw4: core::ffi::c_int,
    w4: core::ffi::c_int,
    max_rows: core::ffi::c_int,
    step: core::ffi::c_int,
    have_newmv_match: *mut core::ffi::c_int,
    have_refmv_match: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cand_b: *const refmvs_block = b;
    let first_cand_bs: BlockSize = (*cand_b).bs as BlockSize;
    let first_cand_b_dim: *const uint8_t = (dav1d_block_dimensions[first_cand_bs
        as usize])
        .as_ptr();
    let mut cand_bw4: core::ffi::c_int = *first_cand_b_dim
        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int;
    let mut len: core::ffi::c_int = imax(step, imin(bw4, cand_bw4));
    if bw4 <= cand_bw4 {
        let weight: core::ffi::c_int = if bw4 == 1 as core::ffi::c_int {
            2 as core::ffi::c_int
        } else {
            imax(
                2 as core::ffi::c_int,
                imin(
                    2 as core::ffi::c_int * max_rows,
                    *first_cand_b_dim.offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int,
                ),
            ) as core::ffi::c_int
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1 as core::ffi::c_int;
    }
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2 as core::ffi::c_int,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        x += len;
        if x >= w4 {
            return 1 as core::ffi::c_int;
        }
        cand_b = &*b.offset(x as isize) as *const refmvs_block;
        cand_bw4 = dav1d_block_dimensions[(*cand_b).bs
            as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int;
        if !(cand_bw4 < bw4) {
            unreachable!();
        }
        len = imax(step, cand_bw4);
    };
}
unsafe extern "C" fn scan_col(
    mvstack: *mut refmvs_candidate,
    cnt: *mut core::ffi::c_int,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    mut b: *const *mut refmvs_block,
    bh4: core::ffi::c_int,
    h4: core::ffi::c_int,
    bx4: core::ffi::c_int,
    max_cols: core::ffi::c_int,
    step: core::ffi::c_int,
    have_newmv_match: *mut core::ffi::c_int,
    have_refmv_match: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cand_b: *const refmvs_block = &raw mut *(*b
        .offset(0 as core::ffi::c_int as isize))
        .offset(bx4 as isize) as *mut refmvs_block;
    let first_cand_bs: BlockSize = (*cand_b).bs as BlockSize;
    let first_cand_b_dim: *const uint8_t = (dav1d_block_dimensions[first_cand_bs
        as usize])
        .as_ptr();
    let mut cand_bh4: core::ffi::c_int = *first_cand_b_dim
        .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int;
    let mut len: core::ffi::c_int = imax(step, imin(bh4, cand_bh4));
    if bh4 <= cand_bh4 {
        let weight: core::ffi::c_int = if bh4 == 1 as core::ffi::c_int {
            2 as core::ffi::c_int
        } else {
            imax(
                2 as core::ffi::c_int,
                imin(
                    2 as core::ffi::c_int * max_cols,
                    *first_cand_b_dim.offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int,
                ),
            ) as core::ffi::c_int
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1 as core::ffi::c_int;
    }
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2 as core::ffi::c_int,
            cand_b,
            ref_0,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        y += len;
        if y >= h4 {
            return 1 as core::ffi::c_int;
        }
        cand_b = &raw mut *(*b.offset(y as isize)).offset(bx4 as isize) as *mut refmvs_block;
        cand_bh4 = dav1d_block_dimensions[(*cand_b).bs
            as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int;
        if !(cand_bh4 < bh4) {
            unreachable!();
        }
        len = imax(step, cand_bh4);
    };
}
#[inline]
unsafe extern "C" fn mv_projection(
    mv: mv,
    num: core::ffi::c_int,
    den: core::ffi::c_int,
) -> mv {
    static mut div_mult: [uint16_t; 32] = [
        0 as core::ffi::c_int as uint16_t,
        16384 as core::ffi::c_int as uint16_t,
        8192 as core::ffi::c_int as uint16_t,
        5461 as core::ffi::c_int as uint16_t,
        4096 as core::ffi::c_int as uint16_t,
        3276 as core::ffi::c_int as uint16_t,
        2730 as core::ffi::c_int as uint16_t,
        2340 as core::ffi::c_int as uint16_t,
        2048 as core::ffi::c_int as uint16_t,
        1820 as core::ffi::c_int as uint16_t,
        1638 as core::ffi::c_int as uint16_t,
        1489 as core::ffi::c_int as uint16_t,
        1365 as core::ffi::c_int as uint16_t,
        1260 as core::ffi::c_int as uint16_t,
        1170 as core::ffi::c_int as uint16_t,
        1092 as core::ffi::c_int as uint16_t,
        1024 as core::ffi::c_int as uint16_t,
        963 as core::ffi::c_int as uint16_t,
        910 as core::ffi::c_int as uint16_t,
        862 as core::ffi::c_int as uint16_t,
        819 as core::ffi::c_int as uint16_t,
        780 as core::ffi::c_int as uint16_t,
        744 as core::ffi::c_int as uint16_t,
        712 as core::ffi::c_int as uint16_t,
        682 as core::ffi::c_int as uint16_t,
        655 as core::ffi::c_int as uint16_t,
        630 as core::ffi::c_int as uint16_t,
        606 as core::ffi::c_int as uint16_t,
        585 as core::ffi::c_int as uint16_t,
        564 as core::ffi::c_int as uint16_t,
        546 as core::ffi::c_int as uint16_t,
        528 as core::ffi::c_int as uint16_t,
    ];
    if !(den > 0 as core::ffi::c_int && den < 32 as core::ffi::c_int) {
        unreachable!();
    }
    if !(num > -(32 as core::ffi::c_int) && num < 32 as core::ffi::c_int) {
        unreachable!();
    }
    let frac: core::ffi::c_int = num * div_mult[den as usize] as core::ffi::c_int;
    let y: core::ffi::c_int = mv.c2rust_unnamed.y as core::ffi::c_int * frac;
    let x: core::ffi::c_int = mv.c2rust_unnamed.x as core::ffi::c_int * frac;
    return mv {
        c2rust_unnamed: {
            let mut init = C2RustUnnamed_12 {
                y: iclip(
                    y + 8192 as core::ffi::c_int + (y >> 31 as core::ffi::c_int)
                        >> 14 as core::ffi::c_int,
                    -(0x3fff as core::ffi::c_int),
                    0x3fff as core::ffi::c_int,
                ) as int16_t,
                x: iclip(
                    x + 8192 as core::ffi::c_int + (x >> 31 as core::ffi::c_int)
                        >> 14 as core::ffi::c_int,
                    -(0x3fff as core::ffi::c_int),
                    0x3fff as core::ffi::c_int,
                ) as int16_t,
            };
            init
        },
    };
}
unsafe extern "C" fn add_temporal_candidate(
    rf: *const refmvs_frame,
    mvstack: *mut refmvs_candidate,
    cnt: *mut core::ffi::c_int,
    rb: *const refmvs_temporal_block,
    ref_0: refmvs_refpair,
    globalmv_ctx: *mut core::ffi::c_int,
    mut gmv: *const mv,
) {
    if (*rb).mv.n == INVALID_MV as uint32_t {
        return;
    }
    let mut mv: mv = mv_projection(
        (*rb).mv as mv,
        (*rf)
            .pocdiff[(ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize] as core::ffi::c_int,
        (*rb).ref_0 as core::ffi::c_int,
    );
    fix_mv_precision((*rf).frm_hdr, &raw mut mv);
    let last: core::ffi::c_int = *cnt;
    if ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
        == -(1 as core::ffi::c_int)
    {
        if !globalmv_ctx.is_null() {
            *globalmv_ctx = (abs(
                mv.c2rust_unnamed.x as core::ffi::c_int
                    - (*gmv.offset(0 as core::ffi::c_int as isize)).c2rust_unnamed.x
                        as core::ffi::c_int,
            )
                | abs(
                    mv.c2rust_unnamed.y as core::ffi::c_int
                        - (*gmv.offset(0 as core::ffi::c_int as isize)).c2rust_unnamed.y
                            as core::ffi::c_int,
                ) >= 16 as core::ffi::c_int) as core::ffi::c_int;
        }
        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
        while n < last {
            if (*mvstack.offset(n as isize)).mv.mv[0 as core::ffi::c_int as usize].n
                == mv.n
            {
                (*mvstack.offset(n as isize)).weight += 2 as core::ffi::c_int;
                return;
            }
            n += 1;
        }
        if last < 8 as core::ffi::c_int {
            (*mvstack.offset(last as isize)).mv.mv[0 as core::ffi::c_int as usize] = mv
                as mv;
            (*mvstack.offset(last as isize)).weight = 2 as core::ffi::c_int;
            *cnt = last + 1 as core::ffi::c_int;
        }
    } else {
        let mut mvp: refmvs_mvpair = refmvs_mvpair {
            mv: [
                mv,
                mv_projection(
                    (*rb).mv as mv,
                    (*rf)
                        .pocdiff[(ref_0.ref_0[1 as core::ffi::c_int as usize]
                        as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                        as core::ffi::c_int,
                    (*rb).ref_0 as core::ffi::c_int,
                ),
            ],
        };
        fix_mv_precision(
            (*rf).frm_hdr,
            &raw mut *(mvp.mv).as_mut_ptr().offset(1 as core::ffi::c_int as isize),
        );
        let mut n_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while n_0 < last {
            if (*mvstack.offset(n_0 as isize)).mv.n == mvp.n {
                (*mvstack.offset(n_0 as isize)).weight += 2 as core::ffi::c_int;
                return;
            }
            n_0 += 1;
        }
        if last < 8 as core::ffi::c_int {
            (*mvstack.offset(last as isize)).mv = mvp;
            (*mvstack.offset(last as isize)).weight = 2 as core::ffi::c_int;
            *cnt = last + 1 as core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn add_compound_extended_candidate(
    same: *mut refmvs_candidate,
    same_count: *mut core::ffi::c_int,
    cand_b: *const refmvs_block,
    sign0: core::ffi::c_int,
    sign1: core::ffi::c_int,
    ref_0: refmvs_refpair,
    sign_bias: *const uint8_t,
) {
    let diff: *mut refmvs_candidate = &raw mut *same.offset(2 as core::ffi::c_int as isize)
        as *mut refmvs_candidate;
    let diff_count: *mut core::ffi::c_int = &raw mut *same_count
        .offset(2 as core::ffi::c_int as isize) as *mut core::ffi::c_int;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < 2 as core::ffi::c_int {
        let cand_ref: core::ffi::c_int = (*cand_b).ref_0.ref_0[n as usize]
            as core::ffi::c_int;
        if cand_ref <= 0 as core::ffi::c_int {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if cand_ref == ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int {
            if *same_count.offset(0 as core::ffi::c_int as isize) < 2 as core::ffi::c_int
            {
                let fresh5 = &raw mut *same_count.offset(0 as core::ffi::c_int as isize);
                let fresh6 = *fresh5;
                *fresh5 = *fresh5 + 1;
                (*same.offset(fresh6 as isize)).mv.mv[0 as core::ffi::c_int as usize] = cand_mv;
            }
            if *diff_count.offset(1 as core::ffi::c_int as isize) < 2 as core::ffi::c_int
            {
                if sign1
                    ^ *sign_bias.offset((cand_ref - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int != 0
                {
                    cand_mv.c2rust_unnamed.y = -(cand_mv.c2rust_unnamed.y
                        as core::ffi::c_int) as int16_t;
                    cand_mv.c2rust_unnamed.x = -(cand_mv.c2rust_unnamed.x
                        as core::ffi::c_int) as int16_t;
                }
                let fresh7 = &raw mut *diff_count.offset(1 as core::ffi::c_int as isize);
                let fresh8 = *fresh7;
                *fresh7 = *fresh7 + 1;
                (*diff.offset(fresh8 as isize)).mv.mv[1 as core::ffi::c_int as usize] = cand_mv;
            }
        } else if cand_ref
            == ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
        {
            if *same_count.offset(1 as core::ffi::c_int as isize) < 2 as core::ffi::c_int
            {
                let fresh9 = &raw mut *same_count.offset(1 as core::ffi::c_int as isize);
                let fresh10 = *fresh9;
                *fresh9 = *fresh9 + 1;
                (*same.offset(fresh10 as isize)).mv.mv[1 as core::ffi::c_int as usize] = cand_mv;
            }
            if *diff_count.offset(0 as core::ffi::c_int as isize) < 2 as core::ffi::c_int
            {
                if sign0
                    ^ *sign_bias.offset((cand_ref - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int != 0
                {
                    cand_mv.c2rust_unnamed.y = -(cand_mv.c2rust_unnamed.y
                        as core::ffi::c_int) as int16_t;
                    cand_mv.c2rust_unnamed.x = -(cand_mv.c2rust_unnamed.x
                        as core::ffi::c_int) as int16_t;
                }
                let fresh11 = &raw mut *diff_count.offset(0 as core::ffi::c_int as isize);
                let fresh12 = *fresh11;
                *fresh11 = *fresh11 + 1;
                (*diff.offset(fresh12 as isize)).mv.mv[0 as core::ffi::c_int as usize] = cand_mv;
            }
        } else {
            let mut i_cand_mv: mv = mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_12 {
                        y: -(cand_mv.c2rust_unnamed.y as core::ffi::c_int) as int16_t,
                        x: -(cand_mv.c2rust_unnamed.x as core::ffi::c_int) as int16_t,
                    };
                    init
                },
            };
            if *diff_count.offset(0 as core::ffi::c_int as isize) < 2 as core::ffi::c_int
            {
                let fresh13 = &raw mut *diff_count.offset(0 as core::ffi::c_int as isize);
                let fresh14 = *fresh13;
                *fresh13 = *fresh13 + 1;
                (*diff.offset(fresh14 as isize)).mv.mv[0 as core::ffi::c_int as usize] = if sign0
                    ^ *sign_bias.offset((cand_ref - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int != 0
                {
                    i_cand_mv
                } else {
                    cand_mv
                };
            }
            if *diff_count.offset(1 as core::ffi::c_int as isize) < 2 as core::ffi::c_int
            {
                let fresh15 = &raw mut *diff_count.offset(1 as core::ffi::c_int as isize);
                let fresh16 = *fresh15;
                *fresh15 = *fresh15 + 1;
                (*diff.offset(fresh16 as isize)).mv.mv[1 as core::ffi::c_int as usize] = if sign1
                    ^ *sign_bias.offset((cand_ref - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int != 0
                {
                    i_cand_mv
                } else {
                    cand_mv
                };
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn add_single_extended_candidate(
    mut mvstack: *mut refmvs_candidate,
    cnt: *mut core::ffi::c_int,
    cand_b: *const refmvs_block,
    sign: core::ffi::c_int,
    sign_bias: *const uint8_t,
) {
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < 2 as core::ffi::c_int {
        let cand_ref: core::ffi::c_int = (*cand_b).ref_0.ref_0[n as usize]
            as core::ffi::c_int;
        if cand_ref <= 0 as core::ffi::c_int {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if sign
            ^ *sign_bias.offset((cand_ref - 1 as core::ffi::c_int) as isize)
                as core::ffi::c_int != 0
        {
            cand_mv.c2rust_unnamed.y = -(cand_mv.c2rust_unnamed.y as core::ffi::c_int)
                as int16_t;
            cand_mv.c2rust_unnamed.x = -(cand_mv.c2rust_unnamed.x as core::ffi::c_int)
                as int16_t;
        }
        let mut m: core::ffi::c_int = 0;
        let last: core::ffi::c_int = *cnt;
        m = 0 as core::ffi::c_int;
        while m < last {
            if cand_mv.n
                == (*mvstack.offset(m as isize)).mv.mv[0 as core::ffi::c_int as usize].n
            {
                break;
            }
            m += 1;
        }
        if m == last {
            (*mvstack.offset(m as isize)).mv.mv[0 as core::ffi::c_int as usize] = cand_mv;
            (*mvstack.offset(m as isize)).weight = 2 as core::ffi::c_int;
            *cnt = last + 1 as core::ffi::c_int;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_find(
    rt: *const refmvs_tile,
    mut mvstack: *mut refmvs_candidate,
    cnt: *mut core::ffi::c_int,
    ctx: *mut core::ffi::c_int,
    ref_0: refmvs_refpair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: core::ffi::c_int,
    bx4: core::ffi::c_int,
) {
    let rf: *const refmvs_frame = (*rt).rf;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: core::ffi::c_int = *b_dim.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let w4: core::ffi::c_int = imin(
        imin(bw4, 16 as core::ffi::c_int),
        (*rt).tile_col.end - bx4,
    ) as core::ffi::c_int;
    let bh4: core::ffi::c_int = *b_dim.offset(1 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let h4: core::ffi::c_int = imin(
        imin(bh4, 16 as core::ffi::c_int),
        (*rt).tile_row.end - by4,
    ) as core::ffi::c_int;
    let mut gmv: [mv; 2] = [mv {
        c2rust_unnamed: C2RustUnnamed_12 { y: 0, x: 0 },
    }; 2];
    let mut tgmv: [mv; 2] = [mv {
        c2rust_unnamed: C2RustUnnamed_12 { y: 0, x: 0 },
    }; 2];
    *cnt = 0 as core::ffi::c_int;
    if !(ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
        >= 0 as core::ffi::c_int
        && ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            <= 8 as core::ffi::c_int
        && ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
            >= -(1 as core::ffi::c_int)
        && ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
            <= 8 as core::ffi::c_int)
    {
        unreachable!();
    }
    if ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
        > 0 as core::ffi::c_int
    {
        tgmv[0 as core::ffi::c_int as usize] = get_gmv_2d(
            &*((*(*rf).frm_hdr).gmv)
                .as_ptr()
                .offset(
                    (*(ref_0.ref_0).as_ptr().offset(0 as core::ffi::c_int as isize)
                        as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                ),
            bx4,
            by4,
            bw4,
            bh4,
            (*rf).frm_hdr,
        );
        gmv[0 as core::ffi::c_int as usize] = if (*(*rf).frm_hdr)
            .gmv[(ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize]
            .type_0 as core::ffi::c_uint
            > DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int as core::ffi::c_uint
        {
            tgmv[0 as core::ffi::c_int as usize]
        } else {
            mv { n: INVALID_MV }
        };
    } else {
        tgmv[0 as core::ffi::c_int as usize] = mv {
            n: 0 as core::ffi::c_int as uint32_t,
        };
        gmv[0 as core::ffi::c_int as usize] = mv { n: INVALID_MV };
    }
    if ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
        > 0 as core::ffi::c_int
    {
        tgmv[1 as core::ffi::c_int as usize] = get_gmv_2d(
            &*((*(*rf).frm_hdr).gmv)
                .as_ptr()
                .offset(
                    (*(ref_0.ref_0).as_ptr().offset(1 as core::ffi::c_int as isize)
                        as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                ),
            bx4,
            by4,
            bw4,
            bh4,
            (*rf).frm_hdr,
        );
        gmv[1 as core::ffi::c_int as usize] = if (*(*rf).frm_hdr)
            .gmv[(ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                - 1 as core::ffi::c_int) as usize]
            .type_0 as core::ffi::c_uint
            > DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int as core::ffi::c_uint
        {
            tgmv[1 as core::ffi::c_int as usize]
        } else {
            mv { n: INVALID_MV }
        };
    }
    let mut have_newmv: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut have_col_mvs: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut have_row_mvs: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut max_rows: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut n_rows: core::ffi::c_uint = !(0 as core::ffi::c_int) as core::ffi::c_uint;
    let mut b_top: *const refmvs_block = 0 as *const refmvs_block;
    if by4 > (*rt).tile_row.start {
        max_rows = imin(
            by4 - (*rt).tile_row.start + 1 as core::ffi::c_int >> 1 as core::ffi::c_int,
            2 as core::ffi::c_int + (bh4 > 1 as core::ffi::c_int) as core::ffi::c_int,
        ) as core::ffi::c_uint;
        b_top = &raw mut *(*((*rt).r)
            .as_ptr()
            .offset(
                ((by4 & 31 as core::ffi::c_int) - 1 as core::ffi::c_int
                    + 5 as core::ffi::c_int) as isize,
            ))
            .offset(bx4 as isize) as *mut refmvs_block;
        n_rows = scan_row(
            mvstack as *mut refmvs_candidate,
            cnt,
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            b_top,
            bw4,
            w4,
            max_rows as core::ffi::c_int,
            if bw4 >= 16 as core::ffi::c_int {
                4 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            },
            &raw mut have_newmv,
            &raw mut have_row_mvs,
        ) as core::ffi::c_uint;
    }
    let mut max_cols: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut n_cols: core::ffi::c_uint = !(0 as core::ffi::c_uint);
    let mut b_left: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
    if bx4 > (*rt).tile_col.start {
        max_cols = imin(
            bx4 - (*rt).tile_col.start + 1 as core::ffi::c_int >> 1 as core::ffi::c_int,
            2 as core::ffi::c_int + (bw4 > 1 as core::ffi::c_int) as core::ffi::c_int,
        ) as core::ffi::c_uint;
        b_left = &*((*rt).r)
            .as_ptr()
            .offset(((by4 & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize)
            as *const *mut refmvs_block;
        n_cols = scan_col(
            mvstack as *mut refmvs_candidate,
            cnt,
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            b_left,
            bh4,
            h4,
            bx4 - 1 as core::ffi::c_int,
            max_cols as core::ffi::c_int,
            if bh4 >= 16 as core::ffi::c_int {
                4 as core::ffi::c_int
            } else {
                1 as core::ffi::c_int
            },
            &raw mut have_newmv,
            &raw mut have_col_mvs,
        ) as core::ffi::c_uint;
    }
    if n_rows != !(0 as core::ffi::c_uint)
        && edge_flags as core::ffi::c_uint
            & EDGE_I444_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint != 0
        && imax(bw4, bh4) <= 16 as core::ffi::c_int && bw4 + bx4 < (*rt).tile_col.end
    {
        add_spatial_candidate(
            mvstack as *mut refmvs_candidate,
            cnt,
            4 as core::ffi::c_int,
            &*b_top.offset(bw4 as isize),
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            &raw mut have_newmv,
            &raw mut have_row_mvs,
        );
    }
    let nearest_match: core::ffi::c_int = have_col_mvs + have_row_mvs;
    let nearest_cnt: core::ffi::c_int = *cnt;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < nearest_cnt {
        (*mvstack.offset(n as isize)).weight += 640 as core::ffi::c_int;
        n += 1;
    }
    let mut globalmv_ctx: core::ffi::c_int = (*(*rf).frm_hdr).use_ref_frame_mvs
        as core::ffi::c_int;
    if (*rf).use_ref_frame_mvs != 0 {
        let stride: ptrdiff_t = (*rf).rp_stride;
        let by8: core::ffi::c_int = by4 >> 1 as core::ffi::c_int;
        let bx8: core::ffi::c_int = bx4 >> 1 as core::ffi::c_int;
        let rbi: *const refmvs_temporal_block = &raw mut *((*rt).rp_proj)
            .offset(
                ((by8 & 15 as core::ffi::c_int) as ptrdiff_t * stride + bx8 as ptrdiff_t)
                    as isize,
            ) as *mut refmvs_temporal_block;
        let mut rb: *const refmvs_temporal_block = rbi;
        let step_h: core::ffi::c_int = if bw4 >= 16 as core::ffi::c_int {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        let step_v: core::ffi::c_int = if bh4 >= 16 as core::ffi::c_int {
            2 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        };
        let w8: core::ffi::c_int = imin(
            w4 + 1 as core::ffi::c_int >> 1 as core::ffi::c_int,
            8 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let h8: core::ffi::c_int = imin(
            h4 + 1 as core::ffi::c_int >> 1 as core::ffi::c_int,
            8 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < h8 {
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < w8 {
                add_temporal_candidate(
                    rf,
                    mvstack as *mut refmvs_candidate,
                    cnt,
                    &*rb.offset(x as isize),
                    ref_0,
                    if x | y == 0 {
                        &raw mut globalmv_ctx
                    } else {
                        0 as *mut core::ffi::c_int
                    },
                    tgmv.as_mut_ptr() as *const mv,
                );
                x += step_h;
            }
            rb = rb.offset((stride * step_v as ptrdiff_t) as isize);
            y += step_v;
        }
        if imin(bw4, bh4) >= 2 as core::ffi::c_int
            && imax(bw4, bh4) < 16 as core::ffi::c_int
        {
            let bh8: core::ffi::c_int = bh4 >> 1 as core::ffi::c_int;
            let bw8: core::ffi::c_int = bw4 >> 1 as core::ffi::c_int;
            rb = &*rbi.offset((bh8 as ptrdiff_t * stride) as isize)
                as *const refmvs_temporal_block;
            let has_bottom: core::ffi::c_int = (by8 + bh8
                < imin(
                    (*rt).tile_row.end >> 1 as core::ffi::c_int,
                    (by8 & !(7 as core::ffi::c_int)) + 8 as core::ffi::c_int,
                )) as core::ffi::c_int;
            if has_bottom != 0
                && bx8 - 1 as core::ffi::c_int
                    >= imax(
                        (*rt).tile_col.start >> 1 as core::ffi::c_int,
                        bx8 & !(7 as core::ffi::c_int),
                    )
            {
                add_temporal_candidate(
                    rf,
                    mvstack as *mut refmvs_candidate,
                    cnt,
                    &*rb.offset(-(1 as core::ffi::c_int) as isize),
                    ref_0,
                    0 as *mut core::ffi::c_int,
                    0 as *const mv,
                );
            }
            if bx8 + bw8
                < imin(
                    (*rt).tile_col.end >> 1 as core::ffi::c_int,
                    (bx8 & !(7 as core::ffi::c_int)) + 8 as core::ffi::c_int,
                )
            {
                if has_bottom != 0 {
                    add_temporal_candidate(
                        rf,
                        mvstack as *mut refmvs_candidate,
                        cnt,
                        &*rb.offset(bw8 as isize),
                        ref_0,
                        0 as *mut core::ffi::c_int,
                        0 as *const mv,
                    );
                }
                if (by8 + bh8 - 1 as core::ffi::c_int)
                    < imin(
                        (*rt).tile_row.end >> 1 as core::ffi::c_int,
                        (by8 & !(7 as core::ffi::c_int)) + 8 as core::ffi::c_int,
                    )
                {
                    add_temporal_candidate(
                        rf,
                        mvstack as *mut refmvs_candidate,
                        cnt,
                        &*rb.offset((bw8 as ptrdiff_t - stride) as isize),
                        ref_0,
                        0 as *mut core::ffi::c_int,
                        0 as *const mv,
                    );
                }
            }
        }
    }
    if !(*cnt <= 8 as core::ffi::c_int) {
        unreachable!();
    }
    let mut have_dummy_newmv_match: core::ffi::c_int = 0;
    if n_rows | n_cols != !(0 as core::ffi::c_uint) {
        add_spatial_candidate(
            mvstack as *mut refmvs_candidate,
            cnt,
            4 as core::ffi::c_int,
            &*b_top.offset(-(1 as core::ffi::c_int) as isize),
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            &raw mut have_dummy_newmv_match,
            &raw mut have_row_mvs,
        );
    }
    let mut n_0: core::ffi::c_int = 2 as core::ffi::c_int;
    while n_0 <= 3 as core::ffi::c_int {
        if n_0 as core::ffi::c_uint > n_rows && n_0 as core::ffi::c_uint <= max_rows {
            n_rows = n_rows
                .wrapping_add(
                    scan_row(
                        mvstack as *mut refmvs_candidate,
                        cnt,
                        ref_0,
                        gmv.as_mut_ptr() as *const mv,
                        &raw mut *(*((*rt).r)
                            .as_ptr()
                            .offset(
                                (((by4 & 31 as core::ffi::c_int)
                                    - 2 as core::ffi::c_int * n_0 + 1 as core::ffi::c_int
                                    | 1 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize,
                            ))
                            .offset((bx4 | 1 as core::ffi::c_int) as isize),
                        bw4,
                        w4,
                        (1 as core::ffi::c_uint)
                            .wrapping_add(max_rows)
                            .wrapping_sub(n_0 as core::ffi::c_uint) as core::ffi::c_int,
                        if bw4 >= 16 as core::ffi::c_int {
                            4 as core::ffi::c_int
                        } else {
                            2 as core::ffi::c_int
                        },
                        &raw mut have_dummy_newmv_match,
                        &raw mut have_row_mvs,
                    ) as core::ffi::c_uint,
                );
        }
        if n_0 as core::ffi::c_uint > n_cols && n_0 as core::ffi::c_uint <= max_cols {
            n_cols = n_cols
                .wrapping_add(
                    scan_col(
                        mvstack as *mut refmvs_candidate,
                        cnt,
                        ref_0,
                        gmv.as_mut_ptr() as *const mv,
                        &*((*rt).r)
                            .as_ptr()
                            .offset(
                                ((by4 & 31 as core::ffi::c_int | 1 as core::ffi::c_int)
                                    + 5 as core::ffi::c_int) as isize,
                            ),
                        bh4,
                        h4,
                        bx4 - n_0 * 2 as core::ffi::c_int + 1 as core::ffi::c_int
                            | 1 as core::ffi::c_int,
                        (1 as core::ffi::c_uint)
                            .wrapping_add(max_cols)
                            .wrapping_sub(n_0 as core::ffi::c_uint) as core::ffi::c_int,
                        if bh4 >= 16 as core::ffi::c_int {
                            4 as core::ffi::c_int
                        } else {
                            2 as core::ffi::c_int
                        },
                        &raw mut have_dummy_newmv_match,
                        &raw mut have_col_mvs,
                    ) as core::ffi::c_uint,
                );
        }
        n_0 += 1;
    }
    if !(*cnt <= 8 as core::ffi::c_int) {
        unreachable!();
    }
    let ref_match_count: core::ffi::c_int = have_col_mvs + have_row_mvs;
    let mut refmv_ctx: core::ffi::c_int = 0;
    let mut newmv_ctx: core::ffi::c_int = 0;
    match nearest_match {
        0 => {
            refmv_ctx = imin(2 as core::ffi::c_int, ref_match_count);
            newmv_ctx = (ref_match_count > 0 as core::ffi::c_int) as core::ffi::c_int;
        }
        1 => {
            refmv_ctx = imin(
                ref_match_count * 3 as core::ffi::c_int,
                4 as core::ffi::c_int,
            );
            newmv_ctx = 3 as core::ffi::c_int - have_newmv;
        }
        2 => {
            refmv_ctx = 5 as core::ffi::c_int;
            newmv_ctx = 5 as core::ffi::c_int - have_newmv;
        }
        _ => {}
    }
    let mut len: core::ffi::c_int = nearest_cnt;
    while len != 0 {
        let mut last: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut n_1: core::ffi::c_int = 1 as core::ffi::c_int;
        while n_1 < len {
            if (*mvstack.offset((n_1 - 1 as core::ffi::c_int) as isize)).weight
                < (*mvstack.offset(n_1 as isize)).weight
            {
                let mut tmp: refmvs_candidate = *mvstack
                    .offset((n_1 - 1 as core::ffi::c_int) as isize);
                *mvstack.offset((n_1 - 1 as core::ffi::c_int) as isize) = *mvstack
                    .offset(n_1 as isize);
                *mvstack.offset(n_1 as isize) = tmp;
                last = n_1;
            }
            n_1 += 1;
        }
        len = last;
    }
    len = *cnt;
    while len > nearest_cnt {
        let mut last_0: core::ffi::c_int = nearest_cnt;
        let mut n_2: core::ffi::c_int = nearest_cnt + 1 as core::ffi::c_int;
        while n_2 < len {
            if (*mvstack.offset((n_2 - 1 as core::ffi::c_int) as isize)).weight
                < (*mvstack.offset(n_2 as isize)).weight
            {
                let mut tmp_0: refmvs_candidate = *mvstack
                    .offset((n_2 - 1 as core::ffi::c_int) as isize);
                *mvstack.offset((n_2 - 1 as core::ffi::c_int) as isize) = *mvstack
                    .offset(n_2 as isize);
                *mvstack.offset(n_2 as isize) = tmp_0;
                last_0 = n_2;
            }
            n_2 += 1;
        }
        len = last_0;
    }
    if ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
        > 0 as core::ffi::c_int
    {
        if *cnt < 2 as core::ffi::c_int {
            let sign0: core::ffi::c_int = (*rf)
                .sign_bias[(ref_0.ref_0[0 as core::ffi::c_int as usize]
                as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int;
            let sign1: core::ffi::c_int = (*rf)
                .sign_bias[(ref_0.ref_0[1 as core::ffi::c_int as usize]
                as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int;
            let sz4: core::ffi::c_int = imin(w4, h4) as core::ffi::c_int;
            let same: *mut refmvs_candidate = &raw mut *mvstack.offset(*cnt as isize)
                as *mut refmvs_candidate;
            let mut same_count: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
            if n_rows != !(0 as core::ffi::c_uint) {
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < sz4 {
                    let cand_b: *const refmvs_block = &*b_top.offset(x_0 as isize)
                        as *const refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b,
                        sign0,
                        sign1,
                        ref_0,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    x_0
                        += dav1d_block_dimensions[(*cand_b).bs
                            as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_int;
                }
            }
            if n_cols != !(0 as core::ffi::c_uint) {
                let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_0 < sz4 {
                    let cand_b_0: *const refmvs_block = &raw mut *(*b_left
                        .offset(y_0 as isize))
                        .offset((bx4 - 1 as core::ffi::c_int) as isize)
                        as *mut refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b_0,
                        sign0,
                        sign1,
                        ref_0,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    y_0
                        += dav1d_block_dimensions[(*cand_b_0).bs
                            as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_int;
                }
            }
            let diff: *mut refmvs_candidate = &raw mut *same
                .offset(2 as core::ffi::c_int as isize) as *mut refmvs_candidate;
            let diff_count: *const core::ffi::c_int = &raw mut *same_count
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as isize) as *mut core::ffi::c_int;
            let mut current_block_118: u64;
            let mut n_3: core::ffi::c_int = 0 as core::ffi::c_int;
            while n_3 < 2 as core::ffi::c_int {
                let mut m: core::ffi::c_int = same_count[n_3 as usize];
                if !(m >= 2 as core::ffi::c_int) {
                    let l: core::ffi::c_int = *diff_count.offset(n_3 as isize);
                    if l != 0 {
                        (*same.offset(m as isize)).mv.mv[n_3 as usize] = (*diff
                            .offset(0 as core::ffi::c_int as isize))
                            .mv
                            .mv[n_3 as usize];
                        m += 1;
                        if m == 2 as core::ffi::c_int {
                            current_block_118 = 0;
                        } else if l == 2 as core::ffi::c_int {
                            (*same.offset(1 as core::ffi::c_int as isize))
                                .mv
                                .mv[n_3 as usize] = (*diff
                                .offset(1 as core::ffi::c_int as isize))
                                .mv
                                .mv[n_3 as usize];
                            current_block_118 = 0;
                        } else {
                            current_block_118 = 1;
                        }
                    } else {
                        current_block_118 = 1;
                    }
                    match current_block_118 {
                        0 => {}
                        _ => {
                            loop {
                                (*same.offset(m as isize)).mv.mv[n_3 as usize] = tgmv[n_3
                                    as usize];
                                m += 1;
                                if !(m < 2 as core::ffi::c_int) {
                                    break;
                                }
                            }
                        }
                    }
                }
                n_3 += 1;
            }
            let mut n_4: core::ffi::c_int = *cnt;
            if n_4 == 1 as core::ffi::c_int
                && (*mvstack.offset(0 as core::ffi::c_int as isize)).mv.n
                    == (*same.offset(0 as core::ffi::c_int as isize)).mv.n
            {
                (*mvstack.offset(1 as core::ffi::c_int as isize)).mv = (*mvstack
                    .offset(2 as core::ffi::c_int as isize))
                    .mv;
            }
            loop {
                (*mvstack.offset(n_4 as isize)).weight = 2 as core::ffi::c_int;
                n_4 += 1;
                if !(n_4 < 2 as core::ffi::c_int) {
                    break;
                }
            }
            *cnt = 2 as core::ffi::c_int;
        }
        let left: core::ffi::c_int = -(bx4 + bw4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let right: core::ffi::c_int = ((*rf).iw4 - bx4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let top: core::ffi::c_int = -(by4 + bh4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let bottom: core::ffi::c_int = ((*rf).ih4 - by4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let n_refmvs: core::ffi::c_int = *cnt;
        let mut n_5: core::ffi::c_int = 0 as core::ffi::c_int;
        loop {
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int,
                left,
                right,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int,
                top,
                bottom,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[1 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[1 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int,
                left,
                right,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[1 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[1 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int,
                top,
                bottom,
            ) as int16_t;
            n_5 += 1;
            if !(n_5 < n_refmvs) {
                break;
            }
        }
        match refmv_ctx >> 1 as core::ffi::c_int {
            0 => {
                *ctx = imin(newmv_ctx, 1 as core::ffi::c_int);
            }
            1 => {
                *ctx = 1 as core::ffi::c_int + imin(newmv_ctx, 3 as core::ffi::c_int);
            }
            2 => {
                *ctx = iclip(
                    3 as core::ffi::c_int + newmv_ctx,
                    4 as core::ffi::c_int,
                    7 as core::ffi::c_int,
                );
            }
            _ => {}
        }
        return;
    } else if *cnt < 2 as core::ffi::c_int
        && ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            > 0 as core::ffi::c_int
    {
        let sign: core::ffi::c_int = (*rf)
            .sign_bias[(ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
        let sz4_0: core::ffi::c_int = imin(w4, h4) as core::ffi::c_int;
        if n_rows != !(0 as core::ffi::c_uint) {
            let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_1 < sz4_0 && *cnt < 2 as core::ffi::c_int {
                let cand_b_1: *const refmvs_block = &*b_top.offset(x_1 as isize)
                    as *const refmvs_block;
                add_single_extended_candidate(
                    mvstack,
                    cnt,
                    cand_b_1,
                    sign,
                    ((*rf).sign_bias).as_ptr(),
                );
                x_1
                    += dav1d_block_dimensions[(*cand_b_1).bs
                        as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int;
            }
        }
        if n_cols != !(0 as core::ffi::c_uint) {
            let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while y_1 < sz4_0 && *cnt < 2 as core::ffi::c_int {
                let cand_b_2: *const refmvs_block = &raw mut *(*b_left.offset(y_1 as isize))
                    .offset((bx4 - 1 as core::ffi::c_int) as isize) as *mut refmvs_block;
                add_single_extended_candidate(
                    mvstack,
                    cnt,
                    cand_b_2,
                    sign,
                    ((*rf).sign_bias).as_ptr(),
                );
                y_1
                    += dav1d_block_dimensions[(*cand_b_2).bs
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int;
            }
        }
    }
    if !(*cnt <= 8 as core::ffi::c_int) {
        unreachable!();
    }
    let mut n_refmvs_0: core::ffi::c_int = *cnt;
    if n_refmvs_0 != 0 {
        let left_0: core::ffi::c_int = -(bx4 + bw4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let right_0: core::ffi::c_int = ((*rf).iw4 - bx4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let top_0: core::ffi::c_int = -(by4 + bh4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let bottom_0: core::ffi::c_int = ((*rf).ih4 - by4 + 4 as core::ffi::c_int)
            * 4 as core::ffi::c_int * 8 as core::ffi::c_int;
        let mut n_6: core::ffi::c_int = 0 as core::ffi::c_int;
        loop {
            (*mvstack.offset(n_6 as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_6 as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int,
                left_0,
                right_0,
            ) as int16_t;
            (*mvstack.offset(n_6 as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_6 as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int,
                top_0,
                bottom_0,
            ) as int16_t;
            n_6 += 1;
            if !(n_6 < n_refmvs_0) {
                break;
            }
        }
    }
    let mut n_7: core::ffi::c_int = *cnt;
    while n_7 < 2 as core::ffi::c_int {
        (*mvstack.offset(n_7 as isize)).mv.mv[0 as core::ffi::c_int as usize] = tgmv[0
            as core::ffi::c_int as usize];
        n_7 += 1;
    }
    *ctx = refmv_ctx << 4 as core::ffi::c_int | globalmv_ctx << 3 as core::ffi::c_int
        | newmv_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_tile_sbrow_init(
    rt: *mut refmvs_tile,
    rf: *const refmvs_frame,
    tile_col_start4: core::ffi::c_int,
    tile_col_end4: core::ffi::c_int,
    tile_row_start4: core::ffi::c_int,
    tile_row_end4: core::ffi::c_int,
    sby: core::ffi::c_int,
    mut tile_row_idx: core::ffi::c_int,
    pass: core::ffi::c_int,
) {
    if (*rf).n_tile_threads == 1 as core::ffi::c_int {
        tile_row_idx = 0 as core::ffi::c_int;
    }
    (*rt).rp_proj = &raw mut *((*rf).rp_proj)
        .offset((16 as ptrdiff_t * (*rf).rp_stride * tile_row_idx as ptrdiff_t) as isize)
        as *mut refmvs_temporal_block;
    let r_stride: ptrdiff_t = (*rf).rp_stride * 2 as ptrdiff_t;
    let pass_off: ptrdiff_t = (if (*rf).n_frame_threads > 1 as core::ffi::c_int
        && pass == 2 as core::ffi::c_int
    {
        35 as core::ffi::c_int * 2 as core::ffi::c_int * (*rf).n_blocks
    } else {
        0 as core::ffi::c_int
    }) as ptrdiff_t;
    let mut r: *mut refmvs_block = &raw mut *((*rf).r)
        .offset(
            (35 as ptrdiff_t * r_stride * tile_row_idx as ptrdiff_t + pass_off) as isize,
        ) as *mut refmvs_block;
    let sbsz: core::ffi::c_int = (*rf).sbsz;
    let off: core::ffi::c_int = sbsz * sby & 16 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < sbsz {
        (*rt).r[(off + 5 as core::ffi::c_int + i) as usize] = r;
        i += 1;
        r = r.offset(r_stride as isize);
    }
    (*rt).r[(off + 0 as core::ffi::c_int) as usize] = r;
    r = r.offset(r_stride as isize);
    (*rt).r[(off + 1 as core::ffi::c_int) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 2 as core::ffi::c_int) as usize] = r;
    r = r.offset(r_stride as isize);
    (*rt).r[(off + 3 as core::ffi::c_int) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 4 as core::ffi::c_int) as usize] = r;
    if sby & 1 as core::ffi::c_int != 0 {
        let tmp: *mut core::ffi::c_void = (*rt).r[(off + 0 as core::ffi::c_int) as usize]
            as *mut core::ffi::c_void;
        (*rt).r[(off + 0 as core::ffi::c_int) as usize] = (*rt)
            .r[(off + sbsz + 0 as core::ffi::c_int) as usize];
        (*rt).r[(off + sbsz + 0 as core::ffi::c_int) as usize] = tmp
            as *mut refmvs_block;
        let tmp_0: *mut core::ffi::c_void = (*rt)
            .r[(off + 2 as core::ffi::c_int) as usize] as *mut core::ffi::c_void;
        (*rt).r[(off + 2 as core::ffi::c_int) as usize] = (*rt)
            .r[(off + sbsz + 2 as core::ffi::c_int) as usize];
        (*rt).r[(off + sbsz + 2 as core::ffi::c_int) as usize] = tmp_0
            as *mut refmvs_block;
        let tmp_1: *mut core::ffi::c_void = (*rt)
            .r[(off + 4 as core::ffi::c_int) as usize] as *mut core::ffi::c_void;
        (*rt).r[(off + 4 as core::ffi::c_int) as usize] = (*rt)
            .r[(off + sbsz + 4 as core::ffi::c_int) as usize];
        (*rt).r[(off + sbsz + 4 as core::ffi::c_int) as usize] = tmp_1
            as *mut refmvs_block;
    }
    (*rt).rf = rf;
    (*rt).tile_row.start = tile_row_start4;
    (*rt).tile_row.end = imin(tile_row_end4, (*rf).ih4);
    (*rt).tile_col.start = tile_col_start4;
    (*rt).tile_col.end = imin(tile_col_end4, (*rf).iw4);
}
unsafe extern "C" fn load_tmvs_c(
    rf: *const refmvs_frame,
    mut tile_row_idx: core::ffi::c_int,
    col_start8: core::ffi::c_int,
    col_end8: core::ffi::c_int,
    row_start8: core::ffi::c_int,
    mut row_end8: core::ffi::c_int,
) {
    if (*rf).n_tile_threads == 1 as core::ffi::c_int {
        tile_row_idx = 0 as core::ffi::c_int;
    }
    if !(row_start8 >= 0 as core::ffi::c_int) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as core::ffi::c_uint <= 16 as core::ffi::c_uint) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    let col_start8i: core::ffi::c_int = imax(
        col_start8 - 8 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ) as core::ffi::c_int;
    let col_end8i: core::ffi::c_int = imin(col_end8 + 8 as core::ffi::c_int, (*rf).iw8)
        as core::ffi::c_int;
    let stride: ptrdiff_t = (*rf).rp_stride;
    let mut rp_proj: *mut refmvs_temporal_block = &raw mut *((*rf).rp_proj)
        .offset(
            (16 as ptrdiff_t * stride * tile_row_idx as ptrdiff_t
                + (row_start8 & 15 as core::ffi::c_int) as ptrdiff_t * stride) as isize,
        ) as *mut refmvs_temporal_block;
    let mut y: core::ffi::c_int = row_start8;
    while y < row_end8 {
        let mut x: core::ffi::c_int = col_start8;
        while x < col_end8 {
            (*rp_proj.offset(x as isize)).mv.n = INVALID_MV as uint32_t;
            x += 1;
        }
        rp_proj = rp_proj.offset(stride as isize);
        y += 1;
    }
    rp_proj = &raw mut *((*rf).rp_proj)
        .offset((16 as ptrdiff_t * stride * tile_row_idx as ptrdiff_t) as isize)
        as *mut refmvs_temporal_block;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < (*rf).n_mfmvs {
        let ref2cur: core::ffi::c_int = (*rf).mfmv_ref2cur[n as usize]
            as core::ffi::c_int;
        if !(ref2cur == INVALID_REF2CUR) {
            let ref_0: core::ffi::c_int = (*rf).mfmv_ref[n as usize] as core::ffi::c_int;
            let ref_sign: core::ffi::c_int = ref_0 - 4 as core::ffi::c_int;
            let mut r: *const refmvs_temporal_block = &raw mut *(*((*rf).rp_ref)
                .offset(ref_0 as isize))
                .offset((row_start8 as ptrdiff_t * stride) as isize)
                as *mut refmvs_temporal_block;
            let mut y_0: core::ffi::c_int = row_start8;
            while y_0 < row_end8 {
                let y_sb_align: core::ffi::c_int = y_0 & !(7 as core::ffi::c_int);
                let y_proj_start: core::ffi::c_int = imax(y_sb_align, row_start8)
                    as core::ffi::c_int;
                let y_proj_end: core::ffi::c_int = imin(
                    y_sb_align + 8 as core::ffi::c_int,
                    row_end8,
                ) as core::ffi::c_int;
                let mut x_0: core::ffi::c_int = col_start8i;
                while x_0 < col_end8i {
                    let mut rb: *const refmvs_temporal_block = &*r.offset(x_0 as isize)
                        as *const refmvs_temporal_block;
                    let b_ref: core::ffi::c_int = (*rb).ref_0 as core::ffi::c_int;
                    if !(b_ref == 0) {
                        let ref2ref: core::ffi::c_int = (*rf)
                            .mfmv_ref2ref[n
                            as usize][(b_ref - 1 as core::ffi::c_int) as usize]
                            as core::ffi::c_int;
                        if !(ref2ref == 0) {
                            let b_mv: mv = (*rb).mv;
                            let offset: mv = mv_projection(b_mv as mv, ref2cur, ref2ref)
                                as mv;
                            let mut pos_x: core::ffi::c_int = x_0
                                + apply_sign(
                                    abs(offset.c2rust_unnamed.x as core::ffi::c_int)
                                        >> 6 as core::ffi::c_int,
                                    offset.c2rust_unnamed.x as core::ffi::c_int ^ ref_sign,
                                );
                            let pos_y: core::ffi::c_int = y_0
                                + apply_sign(
                                    abs(offset.c2rust_unnamed.y as core::ffi::c_int)
                                        >> 6 as core::ffi::c_int,
                                    offset.c2rust_unnamed.y as core::ffi::c_int ^ ref_sign,
                                ) as core::ffi::c_int;
                            if pos_y >= y_proj_start && pos_y < y_proj_end {
                                let pos: ptrdiff_t = (pos_y & 15 as core::ffi::c_int)
                                    as ptrdiff_t * stride;
                                loop {
                                    let x_sb_align: core::ffi::c_int = x_0
                                        & !(7 as core::ffi::c_int);
                                    if pos_x
                                        >= imax(x_sb_align - 8 as core::ffi::c_int, col_start8)
                                        && pos_x
                                            < imin(x_sb_align + 16 as core::ffi::c_int, col_end8)
                                    {
                                        (*rp_proj.offset((pos + pos_x as ptrdiff_t) as isize)).mv = (*rb)
                                            .mv;
                                        (*rp_proj.offset((pos + pos_x as ptrdiff_t) as isize))
                                            .ref_0 = ref2ref as uint8_t;
                                    }
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).ref_0 as core::ffi::c_int != b_ref
                                        || (*rb).mv.n != b_mv.n
                                    {
                                        break;
                                    }
                                    pos_x += 1;
                                }
                            } else {
                                loop {
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).ref_0 as core::ffi::c_int != b_ref
                                        || (*rb).mv.n != b_mv.n
                                    {
                                        break;
                                    }
                                }
                            }
                            x_0 -= 1;
                        }
                    }
                    x_0 += 1;
                }
                r = r.offset(stride as isize);
                y_0 += 1;
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn save_tmvs_c(
    mut rp: *mut refmvs_temporal_block,
    stride: ptrdiff_t,
    rr: *const *mut refmvs_block,
    ref_sign: *const uint8_t,
    col_end8: core::ffi::c_int,
    row_end8: core::ffi::c_int,
    col_start8: core::ffi::c_int,
    row_start8: core::ffi::c_int,
) {
    let mut y: core::ffi::c_int = row_start8;
    while y < row_end8 {
        let b: *const refmvs_block = *rr
            .offset(((y & 15 as core::ffi::c_int) * 2 as core::ffi::c_int) as isize);
        let mut x: core::ffi::c_int = col_start8;
        while x < col_end8 {
            let cand_b: *const refmvs_block = &*b
                .offset((x * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                as *const refmvs_block;
            let bw8: core::ffi::c_int = dav1d_block_dimensions[(*cand_b).bs
                as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int
                + 1 as core::ffi::c_int >> 1 as core::ffi::c_int;
            if (*cand_b).ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
                && *ref_sign
                    .offset(
                        ((*cand_b).ref_0.ref_0[1 as core::ffi::c_int as usize]
                            as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int != 0
                && abs(
                    (*cand_b).mv.mv[1 as core::ffi::c_int as usize].c2rust_unnamed.y
                        as core::ffi::c_int,
                )
                    | abs(
                        (*cand_b).mv.mv[1 as core::ffi::c_int as usize].c2rust_unnamed.x
                            as core::ffi::c_int,
                    ) < 4096 as core::ffi::c_int
            {
                let tmv: refmvs_temporal_block = {
                    let mut init = refmvs_temporal_block {
                        mv: (*cand_b).mv.mv[1 as core::ffi::c_int as usize],
                        ref_0: (*cand_b).ref_0.ref_0[1 as core::ffi::c_int as usize]
                            as uint8_t,
                    };
                    init
                };
                let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
                while n < bw8 {
                    *rp.offset(x as isize) = tmv;
                    n += 1;
                    x += 1;
                }
            } else if (*cand_b).ref_0.ref_0[0 as core::ffi::c_int as usize]
                as core::ffi::c_int > 0 as core::ffi::c_int
                && *ref_sign
                    .offset(
                        ((*cand_b).ref_0.ref_0[0 as core::ffi::c_int as usize]
                            as core::ffi::c_int - 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int != 0
                && abs(
                    (*cand_b).mv.mv[0 as core::ffi::c_int as usize].c2rust_unnamed.y
                        as core::ffi::c_int,
                )
                    | abs(
                        (*cand_b).mv.mv[0 as core::ffi::c_int as usize].c2rust_unnamed.x
                            as core::ffi::c_int,
                    ) < 4096 as core::ffi::c_int
            {
                let tmv_0: refmvs_temporal_block = {
                    let mut init = refmvs_temporal_block {
                        mv: (*cand_b).mv.mv[0 as core::ffi::c_int as usize],
                        ref_0: (*cand_b).ref_0.ref_0[0 as core::ffi::c_int as usize]
                            as uint8_t,
                    };
                    init
                };
                let mut n_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while n_0 < bw8 {
                    *rp.offset(x as isize) = tmv_0;
                    n_0 += 1;
                    x += 1;
                }
            } else {
                let tmv_1: refmvs_temporal_block = {
                    let mut init = refmvs_temporal_block {
                        mv: mv {
                            n: 0 as core::ffi::c_int as uint32_t,
                        },
                        ref_0: 0 as uint8_t,
                    };
                    init
                };
                let mut n_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while n_1 < bw8 {
                    *rp.offset(x as isize) = tmv_1;
                    n_1 += 1;
                    x += 1;
                }
            }
        }
        rp = rp.offset(stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_init_frame(
    rf: *mut refmvs_frame,
    seq_hdr: *const Dav1dSequenceHeader,
    frm_hdr: *const Dav1dFrameHeader,
    mut ref_poc: *const uint8_t,
    rp: *mut refmvs_temporal_block,
    mut ref_ref_poc: *const [uint8_t; 7],
    mut rp_ref: *const *mut refmvs_temporal_block,
    n_tile_threads: core::ffi::c_int,
    n_frame_threads: core::ffi::c_int,
) -> core::ffi::c_int {
    let rp_stride: core::ffi::c_int = ((*frm_hdr).width[0 as core::ffi::c_int as usize]
        + 127 as core::ffi::c_int & !(127 as core::ffi::c_int)) >> 3 as core::ffi::c_int;
    let n_tile_rows: core::ffi::c_int = if n_tile_threads > 1 as core::ffi::c_int {
        (*frm_hdr).tiling.rows as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    let n_blocks: core::ffi::c_int = rp_stride * n_tile_rows;
    (*rf).sbsz = (16 as core::ffi::c_int) << (*seq_hdr).sb128 as core::ffi::c_int;
    (*rf).frm_hdr = frm_hdr;
    (*rf).iw8 = (*frm_hdr).width[0 as core::ffi::c_int as usize] + 7 as core::ffi::c_int
        >> 3 as core::ffi::c_int;
    (*rf).ih8 = (*frm_hdr).height + 7 as core::ffi::c_int >> 3 as core::ffi::c_int;
    (*rf).iw4 = (*rf).iw8 << 1 as core::ffi::c_int;
    (*rf).ih4 = (*rf).ih8 << 1 as core::ffi::c_int;
    (*rf).rp = rp;
    (*rf).rp_stride = rp_stride as ptrdiff_t;
    (*rf).n_tile_threads = n_tile_threads;
    (*rf).n_frame_threads = n_frame_threads;
    if n_blocks != (*rf).n_blocks {
        let r_sz: size_t = (::core::mem::size_of::<refmvs_block>() as size_t)
            .wrapping_mul(35 as size_t)
            .wrapping_mul(2 as size_t)
            .wrapping_mul(n_blocks as size_t)
            .wrapping_mul(
                (1 as core::ffi::c_int
                    + (n_frame_threads > 1 as core::ffi::c_int) as core::ffi::c_int)
                    as size_t,
            );
        let rp_proj_sz: size_t = (::core::mem::size_of::<refmvs_temporal_block>()
            as size_t)
            .wrapping_mul(16 as size_t)
            .wrapping_mul(n_blocks as size_t);
        dav1d_free_aligned_internal((*rf).r as *mut core::ffi::c_void);
        (*rf).r = dav1d_alloc_aligned_internal(
            r_sz.wrapping_add(rp_proj_sz),
            64 as size_t,
        ) as *mut refmvs_block;
        if ((*rf).r).is_null() {
            (*rf).n_blocks = 0 as core::ffi::c_int;
            return -(12 as core::ffi::c_int);
        }
        (*rf).rp_proj = ((*rf).r as uintptr_t).wrapping_add(r_sz as uintptr_t)
            as *mut refmvs_temporal_block;
        (*rf).n_blocks = n_blocks;
    }
    let poc: core::ffi::c_int = (*frm_hdr).frame_offset as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 7 as core::ffi::c_int {
        let poc_diff: core::ffi::c_int = get_poc_diff(
            (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
            *ref_poc.offset(i as isize) as core::ffi::c_int,
            poc,
        ) as core::ffi::c_int;
        (*rf).sign_bias[i as usize] = (poc_diff > 0 as core::ffi::c_int)
            as core::ffi::c_int as uint8_t;
        (*rf).mfmv_sign[i as usize] = (poc_diff < 0 as core::ffi::c_int)
            as core::ffi::c_int as uint8_t;
        (*rf).pocdiff[i as usize] = iclip(
            get_poc_diff(
                (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
                poc,
                *ref_poc.offset(i as isize) as core::ffi::c_int,
            ),
            -(31 as core::ffi::c_int),
            31 as core::ffi::c_int,
        ) as int8_t;
        i += 1;
    }
    (*rf).n_mfmvs = 0 as core::ffi::c_int;
    (*rf).rp_ref = rp_ref as *const *mut refmvs_temporal_block;
    if (*frm_hdr).use_ref_frame_mvs as core::ffi::c_int != 0
        && (*seq_hdr).order_hint_n_bits as core::ffi::c_int != 0
    {
        let mut total: core::ffi::c_int = 2 as core::ffi::c_int;
        if !(*rp_ref.offset(0 as core::ffi::c_int as isize)).is_null()
            && (*ref_ref_poc
                .offset(0 as core::ffi::c_int as isize))[6 as core::ffi::c_int as usize]
                as core::ffi::c_int
                != *ref_poc.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
        {
            let fresh0 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh0 as usize] = 0 as uint8_t;
            total = 3 as core::ffi::c_int;
        }
        if !(*rp_ref.offset(4 as core::ffi::c_int as isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
                *ref_poc.offset(4 as core::ffi::c_int as isize) as core::ffi::c_int,
                (*frm_hdr).frame_offset as core::ffi::c_int,
            ) > 0 as core::ffi::c_int
        {
            let fresh1 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh1 as usize] = 4 as uint8_t;
        }
        if !(*rp_ref.offset(5 as core::ffi::c_int as isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
                *ref_poc.offset(5 as core::ffi::c_int as isize) as core::ffi::c_int,
                (*frm_hdr).frame_offset as core::ffi::c_int,
            ) > 0 as core::ffi::c_int
        {
            let fresh2 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh2 as usize] = 5 as uint8_t;
        }
        if (*rf).n_mfmvs < total
            && !(*rp_ref.offset(6 as core::ffi::c_int as isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
                *ref_poc.offset(6 as core::ffi::c_int as isize) as core::ffi::c_int,
                (*frm_hdr).frame_offset as core::ffi::c_int,
            ) > 0 as core::ffi::c_int
        {
            let fresh3 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh3 as usize] = 6 as uint8_t;
        }
        if (*rf).n_mfmvs < total
            && !(*rp_ref.offset(1 as core::ffi::c_int as isize)).is_null()
        {
            let fresh4 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh4 as usize] = 1 as uint8_t;
        }
        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
        while n < (*rf).n_mfmvs {
            let rpoc: core::ffi::c_int = *ref_poc
                .offset((*rf).mfmv_ref[n as usize] as isize) as core::ffi::c_int;
            let diff1: core::ffi::c_int = get_poc_diff(
                (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
                rpoc,
                (*frm_hdr).frame_offset as core::ffi::c_int,
            ) as core::ffi::c_int;
            if abs(diff1) > 31 as core::ffi::c_int {
                (*rf).mfmv_ref2cur[n as usize] = INVALID_REF2CUR as int8_t;
            } else {
                (*rf).mfmv_ref2cur[n as usize] = (if ((*rf).mfmv_ref[n as usize]
                    as core::ffi::c_int) < 4 as core::ffi::c_int
                {
                    -diff1
                } else {
                    diff1
                }) as int8_t;
                let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
                while m < 7 as core::ffi::c_int {
                    let rrpoc: core::ffi::c_int = (*ref_ref_poc
                        .offset((*rf).mfmv_ref[n as usize] as isize))[m as usize]
                        as core::ffi::c_int;
                    let diff2: core::ffi::c_int = get_poc_diff(
                        (*seq_hdr).order_hint_n_bits as core::ffi::c_int,
                        rpoc,
                        rrpoc,
                    ) as core::ffi::c_int;
                    (*rf).mfmv_ref2ref[n as usize][m as usize] = (if diff2
                        as core::ffi::c_uint > 31 as core::ffi::c_uint
                    {
                        0 as core::ffi::c_int
                    } else {
                        diff2
                    }) as uint8_t;
                    m += 1;
                }
            }
            n += 1;
        }
    }
    (*rf).use_ref_frame_mvs = ((*rf).n_mfmvs > 0 as core::ffi::c_int)
        as core::ffi::c_int;
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn splat_mv_c(
    mut rr: *mut *mut refmvs_block,
    rmv: *const refmvs_block,
    bx4: core::ffi::c_int,
    bw4: core::ffi::c_int,
    mut bh4: core::ffi::c_int,
) {
    loop {
        let fresh17 = rr;
        rr = rr.offset(1);
        let r: *mut refmvs_block = (*fresh17).offset(bx4 as isize);
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < bw4 {
            *r.offset(x as isize) = *rmv;
            x += 1;
        }
        bh4 -= 1;
        if !(bh4 != 0) {
            break;
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_refmvs_dsp_init(c: *mut Dav1dRefmvsDSPContext) {
    (*c).load_tmvs = Some(
        load_tmvs_c
            as unsafe extern "C" fn(
                *const refmvs_frame,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as load_tmvs_fn;
    (*c).save_tmvs = Some(
        save_tmvs_c
            as unsafe extern "C" fn(
                *mut refmvs_temporal_block,
                ptrdiff_t,
                *const *mut refmvs_block,
                *const uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as save_tmvs_fn;
    (*c).splat_mv = Some(
        splat_mv_c
            as unsafe extern "C" fn(
                *mut *mut refmvs_block,
                *const refmvs_block,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as splat_mv_fn;
}
#[inline]
unsafe extern "C" fn dav1d_alloc_aligned_internal(
    sz: size_t,
    align: size_t,
) -> *mut core::ffi::c_void {
    if align & align.wrapping_sub(1 as size_t) != 0 {
        unreachable!();
    }
    let mut ptr: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    if posix_memalign(&raw mut ptr, align, sz) != 0 {
        return NULL;
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn dav1d_free_aligned_internal(mut ptr: *mut core::ffi::c_void) {
    free(ptr);
}
#[inline]
unsafe extern "C" fn get_poc_diff(
    order_hint_n_bits: core::ffi::c_int,
    poc0: core::ffi::c_int,
    poc1: core::ffi::c_int,
) -> core::ffi::c_int {
    if order_hint_n_bits == 0 {
        return 0 as core::ffi::c_int;
    }
    let mask: core::ffi::c_int = (1 as core::ffi::c_int)
        << order_hint_n_bits - 1 as core::ffi::c_int;
    let diff: core::ffi::c_int = poc0 - poc1;
    return (diff & mask - 1 as core::ffi::c_int) - (diff & mask);
}
#[inline]
unsafe extern "C" fn fix_int_mv_precision(mv: *mut mv) {
    (*mv).c2rust_unnamed.x = (((*mv).c2rust_unnamed.x as core::ffi::c_int
        - ((*mv).c2rust_unnamed.x as core::ffi::c_int >> 15 as core::ffi::c_int)
        + 3 as core::ffi::c_int) as core::ffi::c_uint & !(7 as core::ffi::c_uint))
        as int16_t;
    (*mv).c2rust_unnamed.y = (((*mv).c2rust_unnamed.y as core::ffi::c_int
        - ((*mv).c2rust_unnamed.y as core::ffi::c_int >> 15 as core::ffi::c_int)
        + 3 as core::ffi::c_int) as core::ffi::c_uint & !(7 as core::ffi::c_uint))
        as int16_t;
}
#[inline]
unsafe extern "C" fn fix_mv_precision(hdr: *const Dav1dFrameHeader, mv: *mut mv) {
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(mv);
    } else if (*hdr).hp == 0 {
        (*mv).c2rust_unnamed.x = (((*mv).c2rust_unnamed.x as core::ffi::c_int
            - ((*mv).c2rust_unnamed.x as core::ffi::c_int >> 15 as core::ffi::c_int))
            as core::ffi::c_uint & !(1 as core::ffi::c_uint)) as int16_t;
        (*mv).c2rust_unnamed.y = (((*mv).c2rust_unnamed.y as core::ffi::c_int
            - ((*mv).c2rust_unnamed.y as core::ffi::c_int >> 15 as core::ffi::c_int))
            as core::ffi::c_uint & !(1 as core::ffi::c_uint)) as int16_t;
    }
}
#[inline]
unsafe extern "C" fn get_gmv_2d(
    gmv: *const Dav1dWarpedMotionParams,
    bx4: core::ffi::c_int,
    by4: core::ffi::c_int,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
    hdr: *const Dav1dFrameHeader,
) -> mv {
    match (*gmv).type_0 as core::ffi::c_uint {
        2 => {
            if !((*gmv).matrix[5 as core::ffi::c_int as usize]
                == (*gmv).matrix[2 as core::ffi::c_int as usize])
            {
                unreachable!();
            }
            if !((*gmv).matrix[4 as core::ffi::c_int as usize]
                == -(*gmv).matrix[3 as core::ffi::c_int as usize])
            {
                unreachable!();
            }
        }
        1 => {
            let mut res_0: mv = mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_12 {
                        y: ((*gmv).matrix[0 as core::ffi::c_int as usize]
                            >> 13 as core::ffi::c_int) as int16_t,
                        x: ((*gmv).matrix[1 as core::ffi::c_int as usize]
                            >> 13 as core::ffi::c_int) as int16_t,
                    };
                    init
                },
            };
            if (*hdr).force_integer_mv != 0 {
                fix_int_mv_precision(&raw mut res_0);
            }
            return res_0;
        }
        0 => {
            return mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_12 {
                        y: 0 as int16_t,
                        x: 0 as int16_t,
                    };
                    init
                },
            };
        }
        3 | _ => {}
    }
    let x: core::ffi::c_int = bx4 * 4 as core::ffi::c_int + bw4 * 2 as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let y: core::ffi::c_int = by4 * 4 as core::ffi::c_int + bh4 * 2 as core::ffi::c_int
        - 1 as core::ffi::c_int;
    let xc: core::ffi::c_int = ((*gmv).matrix[2 as core::ffi::c_int as usize]
        - ((1 as core::ffi::c_int) << 16 as core::ffi::c_int)) * x
        + (*gmv).matrix[3 as core::ffi::c_int as usize] * y
        + (*gmv).matrix[0 as core::ffi::c_int as usize];
    let yc: core::ffi::c_int = ((*gmv).matrix[5 as core::ffi::c_int as usize]
        - ((1 as core::ffi::c_int) << 16 as core::ffi::c_int)) * y
        + (*gmv).matrix[4 as core::ffi::c_int as usize] * x
        + (*gmv).matrix[1 as core::ffi::c_int as usize];
    let shift: core::ffi::c_int = 16 as core::ffi::c_int
        - (3 as core::ffi::c_int - ((*hdr).hp == 0) as core::ffi::c_int);
    let round: core::ffi::c_int = (1 as core::ffi::c_int) << shift
        >> 1 as core::ffi::c_int;
    let mut res: mv = mv {
        c2rust_unnamed: {
            let mut init = C2RustUnnamed_12 {
                y: apply_sign(
                    abs(yc) + round >> shift << ((*hdr).hp == 0) as core::ffi::c_int,
                    yc,
                ) as int16_t,
                x: apply_sign(
                    abs(xc) + round >> shift << ((*hdr).hp == 0) as core::ffi::c_int,
                    xc,
                ) as int16_t,
            };
            init
        },
    };
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(&raw mut res);
    }
    return res;
}
