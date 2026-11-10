/*
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

// Mechanically translated from upstream/dav1d/src/lf_apply_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type __builtin_va_list = *mut core::ffi::c_char;
pub type __darwin_size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> ()>,
    pub __arg: *mut core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [core::ffi::c_char; 8176],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type size_t = __darwin_size_t;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
pub type pixel = uint8_t;
pub type coef = int16_t;
pub type atomic_int = core::ffi::c_int;
pub type atomic_uint = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const uint8_t,
    pub ref_0: *mut Dav1dRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRef {
    pub data: *mut core::ffi::c_void,
    pub const_data: *const core::ffi::c_void,
    pub ref_cnt: atomic_int,
    pub free_ref: core::ffi::c_int,
    pub free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    pub user_data: *mut core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub ref_0: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext {
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub refp: [Dav1dThreadPicture; 7],
    pub cur: Dav1dPicture,
    pub sr_cur: Dav1dThreadPicture,
    pub mvs_ref: *mut Dav1dRef,
    pub mvs: *mut refmvs_temporal_block,
    pub ref_mvs: [*mut refmvs_temporal_block; 7],
    pub ref_mvs_ref: [*mut Dav1dRef; 7],
    pub cur_segmap_ref: *mut Dav1dRef,
    pub prev_segmap_ref: *mut Dav1dRef,
    pub cur_segmap: *mut uint8_t,
    pub prev_segmap: *const uint8_t,
    pub refpoc: [uint8_t; 7],
    pub refrefpoc: [[uint8_t; 7]; 7],
    pub gmv_warp_allowed: [uint8_t; 7],
    pub in_cdf: CdfThreadContext,
    pub out_cdf: CdfThreadContext,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: core::ffi::c_int,
    pub n_tile_data: core::ffi::c_int,
    pub svc: [[ScalableMotionParams; 2]; 7],
    pub resize_step: [core::ffi::c_int; 2],
    pub resize_start: [core::ffi::c_int; 2],
    pub c: *const Dav1dContext,
    pub ts: *mut Dav1dTileState,
    pub n_ts: core::ffi::c_int,
    pub dsp: *const Dav1dDSPContext,
    pub bd_fn: C2RustUnnamed_27,
    pub ipred_edge_sz: core::ffi::c_int,
    pub ipred_edge: [*mut pixel; 3],
    pub b4_stride: ptrdiff_t,
    pub w4: core::ffi::c_int,
    pub h4: core::ffi::c_int,
    pub bw: core::ffi::c_int,
    pub bh: core::ffi::c_int,
    pub sb128w: core::ffi::c_int,
    pub sb128h: core::ffi::c_int,
    pub sbh: core::ffi::c_int,
    pub sb_shift: core::ffi::c_int,
    pub sb_step: core::ffi::c_int,
    pub sr_sb128w: core::ffi::c_int,
    pub dq: [[[uint16_t; 2]; 3]; 8],
    pub qm: [[*const uint8_t; 3]; 19],
    pub a: *mut BlockContext,
    pub a_sz: core::ffi::c_int,
    pub rf: refmvs_frame,
    pub jnt_weights: [[uint8_t; 7]; 7],
    pub bitdepth_max: core::ffi::c_int,
    pub frame_thread: C2RustUnnamed_19,
    pub lf: C2RustUnnamed_18,
    pub task_thread: C2RustUnnamed,
    pub tile_thread: FrameTileThreadData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameTileThreadData {
    pub lowest_pixel_mem: *mut [[core::ffi::c_int; 2]; 7],
    pub lowest_pixel_mem_sz: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub ttd: *mut TaskThreadData,
    pub tasks: *mut Dav1dTask,
    pub tile_tasks: [*mut Dav1dTask; 2],
    pub init_task: Dav1dTask,
    pub num_tasks: core::ffi::c_int,
    pub num_tile_tasks: core::ffi::c_int,
    pub init_done: atomic_int,
    pub done: [atomic_int; 2],
    pub retval: core::ffi::c_int,
    pub update_set: core::ffi::c_int,
    pub error: atomic_int,
    pub task_counter: atomic_int,
    pub task_head: *mut Dav1dTask,
    pub task_tail: *mut Dav1dTask,
    pub task_cur_prev: *mut Dav1dTask,
    pub pending_tasks: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub merge: atomic_int,
    pub lock: pthread_mutex_t,
    pub head: *mut Dav1dTask,
    pub tail: *mut Dav1dTask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTask {
    pub frame_idx: core::ffi::c_uint,
    pub type_0: TaskType,
    pub sby: core::ffi::c_int,
    pub recon_progress: core::ffi::c_int,
    pub deblock_progress: core::ffi::c_int,
    pub deps_skip: core::ffi::c_int,
    pub next: *mut Dav1dTask,
}
pub type TaskType = core::ffi::c_uint;
pub const DAV1D_TASK_TYPE_FG_APPLY: TaskType = 12;
pub const DAV1D_TASK_TYPE_FG_PREP: TaskType = 11;
pub const DAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS: TaskType = 10;
pub const DAV1D_TASK_TYPE_LOOP_RESTORATION: TaskType = 9;
pub const DAV1D_TASK_TYPE_SUPER_RESOLUTION: TaskType = 8;
pub const DAV1D_TASK_TYPE_CDEF: TaskType = 7;
pub const DAV1D_TASK_TYPE_DEBLOCK_ROWS: TaskType = 6;
pub const DAV1D_TASK_TYPE_DEBLOCK_COLS: TaskType = 5;
pub const DAV1D_TASK_TYPE_TILE_RECONSTRUCTION: TaskType = 4;
pub const DAV1D_TASK_TYPE_ENTROPY_PROGRESS: TaskType = 3;
pub const DAV1D_TASK_TYPE_TILE_ENTROPY: TaskType = 2;
pub const DAV1D_TASK_TYPE_INIT_CDF: TaskType = 1;
pub const DAV1D_TASK_TYPE_INIT: TaskType = 0;
pub type pthread_mutex_t = libc::pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskThreadData {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub first: atomic_uint,
    pub cur: core::ffi::c_uint,
    pub reset_task_cur: atomic_uint,
    pub cond_signaled: atomic_int,
    pub delayed_fg: C2RustUnnamed_1,
    pub inited: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub exec: core::ffi::c_int,
    pub finished: core::ffi::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub c2rust_unnamed_0: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub __dav1d_align_grain_lut_16bpc: [crate::platform::Align16; 0],

    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub __dav1d_align_scaling_16bpc: [crate::platform::Align64; 0],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub __dav1d_align_grain_lut_8bpc: [crate::platform::Align16; 0],

    pub grain_lut_8bpc: [[[int8_t; 82]; 74]; 3],
    pub __dav1d_align_scaling_8bpc: [crate::platform::Align64; 0],
    pub scaling_8bpc: [[uint8_t; 256]; 3],
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
pub struct Dav1dMasteringDisplay {
    pub primaries: [[uint16_t; 2]; 3],
    pub white_point: [uint16_t; 2],
    pub max_luminance: uint32_t,
    pub min_luminance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContentLightLevel {
    pub max_content_light_level: uint16_t,
    pub max_frame_average_light_level: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: core::ffi::c_int,
    pub h: core::ffi::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: core::ffi::c_int,
}
pub type Dav1dPixelLayout = core::ffi::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_17,
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
    pub super_res: C2RustUnnamed_16,
    pub have_render_size: uint8_t,
    pub allow_intrabc: uint8_t,
    pub frame_ref_short_signaling: uint8_t,
    pub refidx: [int8_t; 7],
    pub hp: uint8_t,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: uint8_t,
    pub use_ref_frame_mvs: uint8_t,
    pub refresh_context: uint8_t,
    pub tiling: C2RustUnnamed_15,
    pub quant: C2RustUnnamed_14,
    pub segmentation: C2RustUnnamed_13,
    pub delta: C2RustUnnamed_10,
    pub all_lossless: uint8_t,
    pub loopfilter: C2RustUnnamed_9,
    pub cdef: C2RustUnnamed_8,
    pub restoration: C2RustUnnamed_7,
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
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub p: C2RustUnnamed_6,
    pub abcd: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
pub type Dav1dWarpedMotionType = core::ffi::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
pub type Dav1dTxfmMode = core::ffi::c_uint;
pub const DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [uint8_t; 2],
}
pub type Dav1dRestorationType = core::ffi::c_uint;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub damping: uint8_t,
    pub n_bits: uint8_t,
    pub y_strength: [uint8_t; 8],
    pub uv_strength: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [int8_t; 2],
    pub ref_delta: [int8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub q: C2RustUnnamed_12,
    pub lf: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub present: uint8_t,
    pub res_log2: uint8_t,
    pub multi: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub present: uint8_t,
    pub res_log2: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: uint8_t,
    pub last_active_segid: int8_t,
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
pub struct C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
pub type Dav1dFilterMode = core::ffi::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub width_scale_denominator: uint8_t,
    pub enabled: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: uint32_t,
}
pub type Dav1dFrameType = core::ffi::c_uint;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub data: Dav1dFilmGrainData,
    pub present: uint8_t,
    pub update: uint8_t,
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
pub type Dav1dAdaptiveBoolean = core::ffi::c_uint;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const DAV1D_OFF: Dav1dAdaptiveBoolean = 0;
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
pub type Dav1dChromaSamplePosition = core::ffi::c_uint;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;
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
pub type pthread_cond_t = libc::pthread_cond_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub level: *mut [uint8_t; 4],
    pub mask: *mut Av1Filter,
    pub lr_mask: *mut Av1Restoration,
    pub mask_sz: core::ffi::c_int,
    pub lr_mask_sz: core::ffi::c_int,
    pub cdef_buf_plane_sz: [core::ffi::c_int; 2],
    pub cdef_buf_sbh: core::ffi::c_int,
    pub lr_buf_plane_sz: [core::ffi::c_int; 2],
    pub re_sz: core::ffi::c_int,
    pub lim_lut: Av1FilterLUT,
    pub __dav1d_align_lvl: [crate::platform::Align16; 0],
    pub lvl: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub last_sharpness: core::ffi::c_int,
    pub tx_lpf_right_edge: [*mut uint8_t; 2],
    pub cdef_line_buf: *mut uint8_t,
    pub lr_line_buf: *mut uint8_t,
    pub cdef_line: [[*mut pixel; 3]; 2],
    pub cdef_lpf_line: [*mut pixel; 3],
    pub lr_lpf_line: [*mut pixel; 3],
    pub start_of_tile_row: *mut uint8_t,
    pub start_of_tile_row_sz: core::ffi::c_int,
    pub need_cdef_lpf_copy: core::ffi::c_int,
    pub p: [*mut pixel; 3],
    pub sr_p: [*mut pixel; 3],
    pub restore_planes: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1FilterLUT {
    pub __dav1d_align_e: [crate::platform::Align16; 0],

    pub e: [uint8_t; 64],
    pub __dav1d_align_i: [crate::platform::Align16; 0],
    pub i: [uint8_t; 64],
    pub __dav1d_align_sharp: [crate::platform::Align16; 0],
    pub sharp: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Restoration {
    pub lr: [[Av1RestorationUnit; 4]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1RestorationUnit {
    pub type_0: uint8_t,
    pub filter_h: [int8_t; 3],
    pub filter_v: [int8_t; 3],
    pub sgr_weights: [int8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Filter {
    pub filter_y: [[[[uint16_t; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[uint16_t; 2]; 2]; 32]; 2],
    pub cdef_idx: [int8_t; 4],
    pub noskip_mask: [[uint16_t; 2]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub next_tile_row: [core::ffi::c_int; 2],
    pub entropy_progress: atomic_int,
    pub deblock_progress: atomic_int,
    pub frame_progress: *mut atomic_uint,
    pub copy_lpf_progress: *mut atomic_uint,
    pub b: *mut Av1Block,
    pub cbi: *mut int16_t,
    pub pal: *mut [[pixel; 8]; 3],
    pub pal_idx: *mut uint8_t,
    pub cf: *mut coef,
    pub prog_sz: core::ffi::c_int,
    pub cbi_sz: core::ffi::c_int,
    pub pal_sz: core::ffi::c_int,
    pub pal_idx_sz: core::ffi::c_int,
    pub cf_sz: core::ffi::c_int,
    pub tile_start_off: *mut core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Av1Block {
    pub bl: uint8_t,
    pub bs: uint8_t,
    pub bp: uint8_t,
    pub intra: uint8_t,
    pub seg_id: uint8_t,
    pub skip_mode: uint8_t,
    pub skip: uint8_t,
    pub uvtx: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub c2rust_unnamed: C2RustUnnamed_26,
    pub c2rust_unnamed_0: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub c2rust_unnamed: C2RustUnnamed_22,
    pub comp_type: uint8_t,
    pub inter_mode: uint8_t,
    pub motion_mode: uint8_t,
    pub drl_idx: uint8_t,
    pub ref_0: [int8_t; 2],
    pub max_ytx: uint8_t,
    pub filter2d: uint8_t,
    pub interintra_type: uint8_t,
    pub tx_split0: uint8_t,
    pub tx_split1: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub c2rust_unnamed: C2RustUnnamed_25,
    pub c2rust_unnamed_0: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub mv2d: mv,
    pub matrix: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_24,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub y: int16_t,
    pub x: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub mv: [mv; 2],
    pub wedge_idx: uint8_t,
    pub mask_sign: uint8_t,
    pub interintra_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub y_mode: uint8_t,
    pub uv_mode: uint8_t,
    pub tx: uint8_t,
    pub pal_sz: [uint8_t; 2],
    pub y_angle: int8_t,
    pub uv_angle: int8_t,
    pub cfl_alpha: [int8_t; 2],
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
#[repr(C, packed(4))]
pub struct refmvs_block {
    pub mv: refmvs_mvpair,
    pub ref_0: refmvs_refpair,
    pub bs: uint8_t,
    pub mf: uint8_t,
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
#[repr(C, packed)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub ref_0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockContext {
    pub __dav1d_align_mode: [crate::platform::Align8; 0],

    pub mode: [uint8_t; 32],
    pub __dav1d_align_lcoef: [crate::platform::Align8; 0],
    pub lcoef: [uint8_t; 32],
    pub __dav1d_align_ccoef: [crate::platform::Align8; 0],
    pub ccoef: [[uint8_t; 32]; 2],
    pub __dav1d_align_seg_pred: [crate::platform::Align8; 0],
    pub seg_pred: [uint8_t; 32],
    pub __dav1d_align_skip: [crate::platform::Align8; 0],
    pub skip: [uint8_t; 32],
    pub __dav1d_align_skip_mode: [crate::platform::Align8; 0],
    pub skip_mode: [uint8_t; 32],
    pub __dav1d_align_intra: [crate::platform::Align8; 0],
    pub intra: [uint8_t; 32],
    pub __dav1d_align_comp_type: [crate::platform::Align8; 0],
    pub comp_type: [uint8_t; 32],
    pub __dav1d_align_ref_0: [crate::platform::Align8; 0],
    pub ref_0: [[int8_t; 32]; 2],
    pub __dav1d_align_filter: [crate::platform::Align8; 0],
    pub filter: [[uint8_t; 32]; 2],
    pub __dav1d_align_tx_intra: [crate::platform::Align8; 0],
    pub tx_intra: [int8_t; 32],
    pub __dav1d_align_tx: [crate::platform::Align8; 0],
    pub tx: [int8_t; 32],
    pub __dav1d_align_tx_lpf_y: [crate::platform::Align8; 0],
    pub tx_lpf_y: [uint8_t; 32],
    pub __dav1d_align_tx_lpf_uv: [crate::platform::Align8; 0],
    pub tx_lpf_uv: [uint8_t; 32],
    pub __dav1d_align_partition: [crate::platform::Align8; 0],
    pub partition: [uint8_t; 16],
    pub __dav1d_align_uvmode: [crate::platform::Align8; 0],
    pub uvmode: [uint8_t; 32],
    pub __dav1d_align_pal_sz: [crate::platform::Align8; 0],
    pub pal_sz: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option<
        unsafe extern "C" fn(*mut Dav1dTaskContext, core::ffi::c_int) -> (),
    >,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
    pub copy_pal_block_y: copy_pal_block_fn,
    pub copy_pal_block_uv: copy_pal_block_fn,
    pub read_pal_plane: read_pal_plane_fn,
    pub read_pal_uv: read_pal_uv_fn,
}
pub type read_pal_uv_fn = Option<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        *mut Av1Block,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext {
    pub c: *const Dav1dContext,
    pub f: *const Dav1dFrameContext,
    pub ts: *mut Dav1dTileState,
    pub bx: core::ffi::c_int,
    pub by: core::ffi::c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub c2rust_unnamed: C2RustUnnamed_43,
    pub c2rust_unnamed_0: C2RustUnnamed_42,
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub scratch: C2RustUnnamed_30,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: core::ffi::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: C2RustUnnamed_29,
    pub task_thread: C2RustUnnamed_28,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub td: thread_data,
    pub ttd: *mut TaskThreadData,
    pub fttd: *mut FrameTileThreadData,
    pub flushed: core::ffi::c_int,
    pub die: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_data {
    pub thread: pthread_t,
    pub cond: pthread_cond_t,
    pub lock: pthread_mutex_t,
    pub inited: core::ffi::c_int,
}
pub type pthread_t = libc::pthread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub pass: core::ffi::c_int,
}
pub type Filter2d = core::ffi::c_uint;
pub const N_2D_FILTERS: Filter2d = 10;
pub const FILTER_2D_BILINEAR: Filter2d = 9;
pub const FILTER_2D_8TAP_SMOOTH_SHARP: Filter2d = 8;
pub const FILTER_2D_8TAP_SMOOTH: Filter2d = 7;
pub const FILTER_2D_8TAP_SMOOTH_REGULAR: Filter2d = 6;
pub const FILTER_2D_8TAP_SHARP: Filter2d = 5;
pub const FILTER_2D_8TAP_SHARP_SMOOTH: Filter2d = 4;
pub const FILTER_2D_8TAP_SHARP_REGULAR: Filter2d = 3;
pub const FILTER_2D_8TAP_REGULAR_SHARP: Filter2d = 2;
pub const FILTER_2D_8TAP_REGULAR_SMOOTH: Filter2d = 1;
pub const FILTER_2D_8TAP_REGULAR: Filter2d = 0;
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union C2RustUnnamed_30 {
    pub c2rust_unnamed: C2RustUnnamed_38,
    pub c2rust_unnamed_0: C2RustUnnamed_31,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub c2rust_unnamed: C2RustUnnamed_36,
    pub c2rust_unnamed_0: C2RustUnnamed_35,
    pub pal_idx_y: [uint8_t; 2048],
    pub pal_idx_uv: [uint8_t; 4096],
    pub c2rust_unnamed_1: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub c2rust_unnamed: C2RustUnnamed_34,
    pub c2rust_unnamed_0: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_33 {
    pub interintra_16bpc: [uint16_t; 4096],
    pub edge_16bpc: [uint16_t; 257],
    pub __dav1d_align_pal_16bpc: [crate::platform::Align16; 0],
    pub pal_16bpc: [[uint16_t; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub interintra_8bpc: [uint8_t; 4096],
    pub edge_8bpc: [uint8_t; 257],
    pub __dav1d_align_pal_8bpc: [crate::platform::Align8; 0],
    pub pal_8bpc: [[uint8_t; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
    pub ac: [int16_t; 1024],
    pub txtp_map: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub c2rust_unnamed: C2RustUnnamed_40,
    pub c2rust_unnamed_0: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: C2RustUnnamed_41,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub compinter: [[int16_t; 16384]; 2],
    pub seg_mask: [uint8_t; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_42 {
    pub al_pal_8bpc: [[[[uint8_t; 8]; 3]; 32]; 2],
    pub al_pal_16bpc: [[[[uint16_t; 8]; 3]; 32]; 2],
}
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union C2RustUnnamed_43 {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: C2RustUnnamed_44,
    pub tile_row: C2RustUnnamed_44,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
    pub start: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: C2RustUnnamed_47,
    pub progress: [atomic_int; 2],
    pub frame_thread: [C2RustUnnamed_46; 2],
    pub lowest_pixel: *mut [[core::ffi::c_int; 2]; 7],
    pub dqmem: [[[uint16_t; 2]; 3]; 8],
    pub dq: *const [[uint16_t; 2]; 3],
    pub last_qidx: core::ffi::c_int,
    pub last_delta_lf: C2RustUnnamed_45,
    pub __dav1d_align_lflvlmem: [crate::platform::Align16; 0],
    pub lflvlmem: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[uint8_t; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_45 {
    pub i8_0: [int8_t; 4],
    pub u32_0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub pal_idx: *mut uint8_t,
    pub cbi: *mut int16_t,
    pub cf: *mut coef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub col_start: core::ffi::c_int,
    pub col_end: core::ffi::c_int,
    pub row_start: core::ffi::c_int,
    pub row_end: core::ffi::c_int,
    pub col: core::ffi::c_int,
    pub row: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MsacContext {
    pub buf_pos: *const uint8_t,
    pub buf_end: *const uint8_t,
    pub dif: ec_win,
    pub rng: core::ffi::c_uint,
    pub cnt: core::ffi::c_int,
    pub allow_update_cdf: core::ffi::c_int,
}
pub type ec_win = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfContext {
    pub coef: CdfCoefContext,
    pub m: CdfModeContext,
    pub mv: CdfMvContext,
    pub __dav1d_align_kfym: [crate::platform::Align32; 0],
    pub kfym: [[[uint16_t; 16]; 5]; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvContext {
    pub comp: [CdfMvComponent; 2],
    pub __dav1d_align_joint: [crate::platform::Align8; 0],
    pub joint: [uint16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvComponent {
    pub __dav1d_align_classes: [crate::platform::Align32; 0],

    pub classes: [uint16_t; 16],
    pub __dav1d_align_sign: [crate::platform::Align4; 0],
    pub sign: [uint16_t; 2],
    pub __dav1d_align_class0: [crate::platform::Align4; 0],
    pub class0: [uint16_t; 2],
    pub __dav1d_align_class0_fp: [crate::platform::Align8; 0],
    pub class0_fp: [[uint16_t; 4]; 2],
    pub __dav1d_align_class0_hp: [crate::platform::Align4; 0],
    pub class0_hp: [uint16_t; 2],
    pub __dav1d_align_classN: [crate::platform::Align4; 0],
    pub classN: [[uint16_t; 2]; 10],
    pub __dav1d_align_classN_fp: [crate::platform::Align8; 0],
    pub classN_fp: [uint16_t; 4],
    pub __dav1d_align_classN_hp: [crate::platform::Align4; 0],
    pub classN_hp: [uint16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfModeContext {
    pub __dav1d_align_uv_mode: [crate::platform::Align32; 0],

    pub uv_mode: [[[uint16_t; 16]; 13]; 2],
    pub __dav1d_align_partition: [crate::platform::Align32; 0],
    pub partition: [[[uint16_t; 16]; 4]; 5],
    pub __dav1d_align_cfl_alpha: [crate::platform::Align32; 0],
    pub cfl_alpha: [[uint16_t; 16]; 6],
    pub __dav1d_align_txtp_inter1: [crate::platform::Align32; 0],
    pub txtp_inter1: [[uint16_t; 16]; 2],
    pub __dav1d_align_txtp_inter2: [crate::platform::Align32; 0],
    pub txtp_inter2: [uint16_t; 16],
    pub __dav1d_align_txtp_intra1: [crate::platform::Align16; 0],
    pub txtp_intra1: [[[uint16_t; 8]; 13]; 2],
    pub __dav1d_align_txtp_intra2: [crate::platform::Align16; 0],
    pub txtp_intra2: [[[uint16_t; 8]; 13]; 3],
    pub __dav1d_align_cfl_sign: [crate::platform::Align16; 0],
    pub cfl_sign: [uint16_t; 8],
    pub __dav1d_align_angle_delta: [crate::platform::Align16; 0],
    pub angle_delta: [[uint16_t; 8]; 8],
    pub __dav1d_align_filter_intra: [crate::platform::Align16; 0],
    pub filter_intra: [uint16_t; 8],
    pub __dav1d_align_seg_id: [crate::platform::Align16; 0],
    pub seg_id: [[uint16_t; 8]; 3],
    pub __dav1d_align_pal_sz: [crate::platform::Align16; 0],
    pub pal_sz: [[[uint16_t; 8]; 7]; 2],
    pub __dav1d_align_color_map: [crate::platform::Align16; 0],
    pub color_map: [[[[uint16_t; 8]; 5]; 7]; 2],
    pub __dav1d_align_txsz: [crate::platform::Align8; 0],
    pub txsz: [[[uint16_t; 4]; 3]; 4],
    pub __dav1d_align_delta_q: [crate::platform::Align8; 0],
    pub delta_q: [uint16_t; 4],
    pub __dav1d_align_delta_lf: [crate::platform::Align8; 0],
    pub delta_lf: [[uint16_t; 4]; 5],
    pub __dav1d_align_restore_switchable: [crate::platform::Align8; 0],
    pub restore_switchable: [uint16_t; 4],
    pub __dav1d_align_restore_wiener: [crate::platform::Align4; 0],
    pub restore_wiener: [uint16_t; 2],
    pub __dav1d_align_restore_sgrproj: [crate::platform::Align4; 0],
    pub restore_sgrproj: [uint16_t; 2],
    pub __dav1d_align_txtp_inter3: [crate::platform::Align4; 0],
    pub txtp_inter3: [[uint16_t; 2]; 4],
    pub __dav1d_align_use_filter_intra: [crate::platform::Align4; 0],
    pub use_filter_intra: [[uint16_t; 2]; 22],
    pub __dav1d_align_txpart: [crate::platform::Align4; 0],
    pub txpart: [[[uint16_t; 2]; 3]; 7],
    pub __dav1d_align_skip: [crate::platform::Align4; 0],
    pub skip: [[uint16_t; 2]; 3],
    pub __dav1d_align_pal_y: [crate::platform::Align4; 0],
    pub pal_y: [[[uint16_t; 2]; 3]; 7],
    pub __dav1d_align_pal_uv: [crate::platform::Align4; 0],
    pub pal_uv: [[uint16_t; 2]; 2],
    pub __dav1d_align_intrabc: [crate::platform::Align4; 0],
    pub intrabc: [uint16_t; 2],
    pub __dav1d_align_y_mode: [crate::platform::Align32; 0],
    pub y_mode: [[uint16_t; 16]; 4],
    pub __dav1d_align_wedge_idx: [crate::platform::Align32; 0],
    pub wedge_idx: [[uint16_t; 16]; 9],
    pub __dav1d_align_comp_inter_mode: [crate::platform::Align16; 0],
    pub comp_inter_mode: [[uint16_t; 8]; 8],
    pub __dav1d_align_filter: [crate::platform::Align8; 0],
    pub filter: [[[uint16_t; 4]; 8]; 2],
    pub __dav1d_align_interintra_mode: [crate::platform::Align8; 0],
    pub interintra_mode: [[uint16_t; 4]; 4],
    pub __dav1d_align_motion_mode: [crate::platform::Align8; 0],
    pub motion_mode: [[uint16_t; 4]; 22],
    pub __dav1d_align_skip_mode: [crate::platform::Align4; 0],
    pub skip_mode: [[uint16_t; 2]; 3],
    pub __dav1d_align_newmv_mode: [crate::platform::Align4; 0],
    pub newmv_mode: [[uint16_t; 2]; 6],
    pub __dav1d_align_globalmv_mode: [crate::platform::Align4; 0],
    pub globalmv_mode: [[uint16_t; 2]; 2],
    pub __dav1d_align_refmv_mode: [crate::platform::Align4; 0],
    pub refmv_mode: [[uint16_t; 2]; 6],
    pub __dav1d_align_drl_bit: [crate::platform::Align4; 0],
    pub drl_bit: [[uint16_t; 2]; 3],
    pub __dav1d_align_intra: [crate::platform::Align4; 0],
    pub intra: [[uint16_t; 2]; 4],
    pub __dav1d_align_comp: [crate::platform::Align4; 0],
    pub comp: [[uint16_t; 2]; 5],
    pub __dav1d_align_comp_dir: [crate::platform::Align4; 0],
    pub comp_dir: [[uint16_t; 2]; 5],
    pub __dav1d_align_jnt_comp: [crate::platform::Align4; 0],
    pub jnt_comp: [[uint16_t; 2]; 6],
    pub __dav1d_align_mask_comp: [crate::platform::Align4; 0],
    pub mask_comp: [[uint16_t; 2]; 6],
    pub __dav1d_align_wedge_comp: [crate::platform::Align4; 0],
    pub wedge_comp: [[uint16_t; 2]; 9],
    pub __dav1d_align_ref_0: [crate::platform::Align4; 0],
    pub ref_0: [[[uint16_t; 2]; 3]; 6],
    pub __dav1d_align_comp_fwd_ref: [crate::platform::Align4; 0],
    pub comp_fwd_ref: [[[uint16_t; 2]; 3]; 3],
    pub __dav1d_align_comp_bwd_ref: [crate::platform::Align4; 0],
    pub comp_bwd_ref: [[[uint16_t; 2]; 3]; 2],
    pub __dav1d_align_comp_uni_ref: [crate::platform::Align4; 0],
    pub comp_uni_ref: [[[uint16_t; 2]; 3]; 3],
    pub __dav1d_align_seg_pred: [crate::platform::Align4; 0],
    pub seg_pred: [[uint16_t; 2]; 3],
    pub __dav1d_align_interintra: [crate::platform::Align4; 0],
    pub interintra: [[uint16_t; 2]; 7],
    pub __dav1d_align_interintra_wedge: [crate::platform::Align4; 0],
    pub interintra_wedge: [[uint16_t; 2]; 7],
    pub __dav1d_align_obmc: [crate::platform::Align4; 0],
    pub obmc: [[uint16_t; 2]; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfCoefContext {
    pub __dav1d_align_eob_bin_16: [crate::platform::Align16; 0],

    pub eob_bin_16: [[[uint16_t; 8]; 2]; 2],
    pub __dav1d_align_eob_bin_32: [crate::platform::Align16; 0],
    pub eob_bin_32: [[[uint16_t; 8]; 2]; 2],
    pub __dav1d_align_eob_bin_64: [crate::platform::Align16; 0],
    pub eob_bin_64: [[[uint16_t; 8]; 2]; 2],
    pub __dav1d_align_eob_bin_128: [crate::platform::Align16; 0],
    pub eob_bin_128: [[[uint16_t; 8]; 2]; 2],
    pub __dav1d_align_eob_bin_256: [crate::platform::Align32; 0],
    pub eob_bin_256: [[[uint16_t; 16]; 2]; 2],
    pub __dav1d_align_eob_bin_512: [crate::platform::Align32; 0],
    pub eob_bin_512: [[uint16_t; 16]; 2],
    pub __dav1d_align_eob_bin_1024: [crate::platform::Align32; 0],
    pub eob_bin_1024: [[uint16_t; 16]; 2],
    pub __dav1d_align_eob_base_tok: [crate::platform::Align8; 0],
    pub eob_base_tok: [[[[uint16_t; 4]; 4]; 2]; 5],
    pub __dav1d_align_base_tok: [crate::platform::Align8; 0],
    pub base_tok: [[[[uint16_t; 4]; 41]; 2]; 5],
    pub __dav1d_align_br_tok: [crate::platform::Align8; 0],
    pub br_tok: [[[[uint16_t; 4]; 21]; 2]; 4],
    pub __dav1d_align_eob_hi_bit: [crate::platform::Align4; 0],
    pub eob_hi_bit: [[[[uint16_t; 2]; 9]; 2]; 5],
    pub __dav1d_align_skip: [crate::platform::Align4; 0],
    pub skip: [[[uint16_t; 2]; 13]; 5],
    pub __dav1d_align_dc_sign: [crate::platform::Align4; 0],
    pub dc_sign: [[[uint16_t; 2]; 3]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext {
    pub fc: *mut Dav1dFrameContext,
    pub n_fc: core::ffi::c_uint,
    pub tc: *mut Dav1dTaskContext,
    pub n_tc: core::ffi::c_uint,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: core::ffi::c_int,
    pub n_tile_data: core::ffi::c_int,
    pub n_tiles: core::ffi::c_int,
    pub seq_hdr_pool: *mut Dav1dMemPool,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_pool: *mut Dav1dMemPool,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub content_light_ref: *mut Dav1dRef,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display_ref: *mut Dav1dRef,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35_ref: *mut Dav1dRef,
    pub itut_t35: *mut Dav1dITUTT35,
    pub n_itut_t35: core::ffi::c_int,
    pub in_0: Dav1dData,
    pub out: Dav1dThreadPicture,
    pub cache: Dav1dThreadPicture,
    pub flush_mem: atomic_int,
    pub flush: *mut atomic_int,
    pub frame_thread: C2RustUnnamed_51,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [C2RustUnnamed_50; 8],
    pub cdf_pool: *mut Dav1dMemPool,
    pub cdf: [CdfThreadContext; 8],
    pub dsp: [Dav1dDSPContext; 3],
    pub pal_dsp: Dav1dPalDSPContext,
    pub refmvs_dsp: Dav1dRefmvsDSPContext,
    pub allocator: Dav1dPicAllocator,
    pub apply_grain: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub operating_point_idc: core::ffi::c_uint,
    pub all_layers: core::ffi::c_int,
    pub max_spatial_id: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub strict_std_compliance: core::ffi::c_int,
    pub output_invisible_frames: core::ffi::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub drain: core::ffi::c_int,
    pub frame_flags: PictureFlags,
    pub event_flags: Dav1dEventFlags,
    pub cached_error_props: Dav1dDataProps,
    pub cached_error: core::ffi::c_int,
    pub logger: Dav1dLogger,
    pub picture_pool: *mut Dav1dMemPool,
    pub pic_ctx_pool: *mut Dav1dMemPool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPool {
    pub lock: pthread_mutex_t,
    pub buf: *mut Dav1dMemPoolBuffer,
    pub ref_cnt: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMemPoolBuffer {
    pub next: *mut Dav1dMemPoolBuffer,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLogger {
    pub cookie: *mut core::ffi::c_void,
    pub callback: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            va_list,
        ) -> (),
    >,
}
pub type va_list<'a> = core::ffi::VaList<'a>;
pub type Dav1dEventFlags = core::ffi::c_uint;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags = 2;
pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags = 1;
pub type PictureFlags = core::ffi::c_uint;
pub const PICTURE_FLAG_NEW_TEMPORAL_UNIT: PictureFlags = 4;
pub const PICTURE_FLAG_NEW_OP_PARAMS_INFO: PictureFlags = 2;
pub const PICTURE_FLAG_NEW_SEQUENCE: PictureFlags = 1;
pub type Dav1dDecodeFrameType = core::ffi::c_uint;
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;
pub type Dav1dInloopFilterType = core::ffi::c_uint;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPicAllocator {
    pub cookie: *mut core::ffi::c_void,
    pub alloc_picture_callback: Option<
        unsafe extern "C" fn(
            *mut Dav1dPicture,
            *mut core::ffi::c_void,
        ) -> core::ffi::c_int,
    >,
    pub release_picture_callback: Option<
        unsafe extern "C" fn(*mut Dav1dPicture, *mut core::ffi::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRefmvsDSPContext {
    pub load_tmvs: load_tmvs_fn,
    pub save_tmvs: save_tmvs_fn,
    pub splat_mv: splat_mv_fn,
}
pub type splat_mv_fn = Option<
    unsafe extern "C" fn(
        *mut *mut refmvs_block,
        *const refmvs_block,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPalDSPContext {
    pub pal_idx_finish: pal_idx_finish_fn,
}
pub type pal_idx_finish_fn = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDSPContext {
    pub fg: Dav1dFilmGrainDSPContext,
    pub ipred: Dav1dIntraPredDSPContext,
    pub mc: Dav1dMCDSPContext,
    pub itx: Dav1dInvTxfmDSPContext,
    pub lf: Dav1dLoopFilterDSPContext,
    pub cdef: Dav1dCdefDSPContext,
    pub lr: Dav1dLoopRestorationDSPContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}
pub type looprestorationfilter_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
    ) -> (),
>;
pub type LrEdgeFlags = core::ffi::c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub __dav1d_align_filter: [crate::platform::Align16; 0],

    pub filter: [[int16_t; 8]; 2],
    pub sgr: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}
pub type const_left_pixel_row = *const [pixel; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const pixel,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        CdefEdgeFlags,
    ) -> (),
>;
pub type CdefEdgeFlags = core::ffi::c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;
pub type const_left_pixel_row_2px = *const [pixel; 2];
pub type cdef_dir_fn = Option<
    unsafe extern "C" fn(
        *const pixel,
        ptrdiff_t,
        *mut core::ffi::c_uint,
    ) -> core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const uint32_t,
        *const [uint8_t; 4],
        ptrdiff_t,
        *const Av1FilterLUT,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, core::ffi::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
pub type resize_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
    ) -> (),
>;
pub type warp8x8t_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *mut uint8_t,
        core::ffi::c_int,
    ) -> (),
>;
pub type mask_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_avg_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mct_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type mc_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        *const int16_t,
        core::ffi::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const pixel,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type angular_ipred_fn = Option<
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *const pixel,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
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
pub type entry = int8_t;
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
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type generate_grain_y_fn = Option<
    unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfThreadContext {
    pub ref_0: *mut Dav1dRef,
    pub data: C2RustUnnamed_49,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_49 {
    pub cdf: *mut CdfContext,
    pub qcat: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_50 {
    pub p: Dav1dThreadPicture,
    pub segmap: *mut Dav1dRef,
    pub refmvs: *mut Dav1dRef,
    pub refpoc: [uint8_t; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dThreadPicture {
    pub p: Dav1dPicture,
    pub visible: core::ffi::c_int,
    pub showable: core::ffi::c_int,
    pub flags: PictureFlags,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_51 {
    pub out_delayed: *mut Dav1dThreadPicture,
    pub next: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileGroup {
    pub data: Dav1dData,
    pub start: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
pub type read_pal_plane_fn = Option<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        *mut Av1Block,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type copy_pal_block_fn = Option<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub type read_coef_blocks_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> (),
>;
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
pub type backup_ipred_edge_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
>;
pub type filter_sbrow_fn = Option<
    unsafe extern "C" fn(*mut Dav1dFrameContext, core::ffi::c_int) -> (),
>;
pub type recon_b_inter_fn = Option<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        BlockSize,
        *const Av1Block,
    ) -> core::ffi::c_int,
>;
pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(
        *mut Dav1dTaskContext,
        BlockSize,
        EdgeFlags,
        *const Av1Block,
    ) -> (),
>;
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
#[repr(C)]
pub struct ScalableMotionParams {
    pub scale: core::ffi::c_int,
    pub step: core::ffi::c_int,
}
pub const LR_RESTORE_V: LrRestorePlanes = 4;
pub const LR_RESTORE_U: LrRestorePlanes = 2;
pub const LR_RESTORE_Y: LrRestorePlanes = 1;
pub type LrRestorePlanes = core::ffi::c_uint;
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn backup_lpf(
    f: *const Dav1dFrameContext,
    mut dst: *mut pixel,
    dst_stride: ptrdiff_t,
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    ss_ver: core::ffi::c_int,
    sb128: core::ffi::c_int,
    mut row: core::ffi::c_int,
    row_h: core::ffi::c_int,
    src_w: core::ffi::c_int,
    h: core::ffi::c_int,
    ss_hor: core::ffi::c_int,
    lr_backup: core::ffi::c_int,
) {
    let cdef_backup: core::ffi::c_int = (lr_backup == 0) as core::ffi::c_int;
    let dst_w: core::ffi::c_int = if (*(*f).frame_hdr).super_res.enabled
        as core::ffi::c_int != 0
    {
        (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize] + ss_hor >> ss_hor
    } else {
        src_w
    };
    let mut stripe_h: core::ffi::c_int = ((64 as core::ffi::c_int)
        << (cdef_backup & sb128))
        - 8 as core::ffi::c_int * (row == 0) as core::ffi::c_int >> ss_ver;
    src = src
        .offset(((stripe_h - 2 as core::ffi::c_int) as ptrdiff_t * src_stride) as isize);
    if (*(*f).c).n_tc == 1 as core::ffi::c_uint {
        if row != 0 {
            let top: core::ffi::c_int = (4 as core::ffi::c_int) << sb128;
            memcpy(
                &raw mut *dst.offset((dst_stride * 0 as ptrdiff_t) as isize) as *mut pixel
                    as *mut core::ffi::c_void,
                &raw mut *dst.offset((dst_stride * top as ptrdiff_t) as isize) as *mut pixel
                    as *const core::ffi::c_void,
                dst_w as size_t,
            );
            memcpy(
                &raw mut *dst.offset((dst_stride * 1 as ptrdiff_t) as isize) as *mut pixel
                    as *mut core::ffi::c_void,
                &raw mut *dst
                    .offset(
                        (dst_stride * (top + 1 as core::ffi::c_int) as ptrdiff_t)
                            as isize,
                    ) as *mut pixel as *const core::ffi::c_void,
                dst_w as size_t,
            );
            memcpy(
                &raw mut *dst.offset((dst_stride * 2 as ptrdiff_t) as isize) as *mut pixel
                    as *mut core::ffi::c_void,
                &raw mut *dst
                    .offset(
                        (dst_stride * (top + 2 as core::ffi::c_int) as ptrdiff_t)
                            as isize,
                    ) as *mut pixel as *const core::ffi::c_void,
                dst_w as size_t,
            );
            memcpy(
                &raw mut *dst.offset((dst_stride * 3 as ptrdiff_t) as isize) as *mut pixel
                    as *mut core::ffi::c_void,
                &raw mut *dst
                    .offset(
                        (dst_stride * (top + 3 as core::ffi::c_int) as ptrdiff_t)
                            as isize,
                    ) as *mut pixel as *const core::ffi::c_void,
                dst_w as size_t,
            );
        }
        dst = dst.offset((4 as ptrdiff_t * dst_stride) as isize);
    }
    if lr_backup != 0
        && (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
            != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
    {
        while row + stripe_h <= row_h {
            let n_lines: core::ffi::c_int = 4 as core::ffi::c_int
                - (row + stripe_h + 1 as core::ffi::c_int == h) as core::ffi::c_int;
            ((*(*f).dsp).mc.resize)
                .expect(
                    "non-null function pointer",
                )(
                dst,
                dst_stride,
                src,
                src_stride,
                dst_w,
                n_lines,
                src_w,
                (*f).resize_step[ss_hor as usize],
                (*f).resize_start[ss_hor as usize],
            );
            row += stripe_h;
            stripe_h = 64 as core::ffi::c_int >> ss_ver;
            src = src.offset((stripe_h as ptrdiff_t * src_stride) as isize);
            dst = dst.offset((n_lines as ptrdiff_t * dst_stride) as isize);
            if n_lines == 3 as core::ffi::c_int {
                memcpy(
                    dst as *mut core::ffi::c_void,
                    &raw mut *dst.offset(-dst_stride as isize) as *mut pixel
                        as *const core::ffi::c_void,
                    dst_w as size_t,
                );
                dst = dst.offset(dst_stride as isize);
            }
        }
    } else {
        while row + stripe_h <= row_h {
            let n_lines_0: core::ffi::c_int = 4 as core::ffi::c_int
                - (row + stripe_h + 1 as core::ffi::c_int == h) as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < 4 as core::ffi::c_int {
                memcpy(
                    dst as *mut core::ffi::c_void,
                    (if i == n_lines_0 {
                        &raw mut *dst.offset(-dst_stride as isize) as *mut pixel
                            as *const pixel
                    } else {
                        src
                    }) as *const core::ffi::c_void,
                    src_w as size_t,
                );
                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
                i += 1;
            }
            row += stripe_h;
            stripe_h = 64 as core::ffi::c_int >> ss_ver;
            src = src
                .offset(
                    ((stripe_h - 4 as core::ffi::c_int) as ptrdiff_t * src_stride)
                        as isize,
                );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_copy_lpf_8bpc(
    f: *mut Dav1dFrameContext,
    mut src: *const *mut pixel,
    sby: core::ffi::c_int,
) {
    let have_tt: core::ffi::c_int = ((*(*f).c).n_tc > 1 as core::ffi::c_uint)
        as core::ffi::c_int;
    let resize: core::ffi::c_int = ((*(*f).frame_hdr)
        .width[0 as core::ffi::c_int as usize]
        != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]) as core::ffi::c_int;
    let offset: core::ffi::c_int = 8 as core::ffi::c_int
        * (sby != 0) as core::ffi::c_int;
    let src_stride: *const ptrdiff_t = ((*f).cur.stride).as_mut_ptr();
    let lr_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let tt_off: core::ffi::c_int = have_tt * sby
        * ((4 as core::ffi::c_int) << (*(*f).seq_hdr).sb128 as core::ffi::c_int);
    let dst: [*mut pixel; 3] = [
        ((*f).lf.lr_lpf_line[0 as core::ffi::c_int as usize])
            .offset(
                (tt_off as ptrdiff_t * *lr_stride.offset(0 as core::ffi::c_int as isize))
                    as isize,
            ),
        ((*f).lf.lr_lpf_line[1 as core::ffi::c_int as usize])
            .offset(
                (tt_off as ptrdiff_t * *lr_stride.offset(1 as core::ffi::c_int as isize))
                    as isize,
            ),
        ((*f).lf.lr_lpf_line[2 as core::ffi::c_int as usize])
            .offset(
                (tt_off as ptrdiff_t * *lr_stride.offset(1 as core::ffi::c_int as isize))
                    as isize,
            ),
    ];
    let restore_planes: core::ffi::c_int = (*f).lf.restore_planes;
    if (*(*f).seq_hdr).cdef as core::ffi::c_int != 0
        || restore_planes & LR_RESTORE_Y as core::ffi::c_int != 0
    {
        let h: core::ffi::c_int = (*f).cur.p.h;
        let w: core::ffi::c_int = (*f).bw << 2 as core::ffi::c_int;
        let row_h: core::ffi::c_int = imin(
            (sby + 1 as core::ffi::c_int)
                << 6 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int,
            h - 1 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let y_stripe: core::ffi::c_int = (sby
            << 6 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int)
            - offset;
        if restore_planes & LR_RESTORE_Y as core::ffi::c_int != 0 || resize == 0 {
            backup_lpf(
                f,
                dst[0 as core::ffi::c_int as usize],
                *lr_stride.offset(0 as core::ffi::c_int as isize),
                (*src.offset(0 as core::ffi::c_int as isize))
                    .offset(
                        -((offset as ptrdiff_t
                            * *src_stride.offset(0 as core::ffi::c_int as isize))
                            as isize),
                    ),
                *src_stride.offset(0 as core::ffi::c_int as isize),
                0 as core::ffi::c_int,
                (*(*f).seq_hdr).sb128 as core::ffi::c_int,
                y_stripe,
                row_h,
                w,
                h,
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
            );
        }
        if have_tt != 0 && resize != 0 {
            let cdef_off_y: ptrdiff_t = (sby * 4 as core::ffi::c_int) as ptrdiff_t
                * *src_stride.offset(0 as core::ffi::c_int as isize);
            backup_lpf(
                f,
                ((*f).lf.cdef_lpf_line[0 as core::ffi::c_int as usize])
                    .offset(cdef_off_y as isize),
                *src_stride.offset(0 as core::ffi::c_int as isize),
                (*src.offset(0 as core::ffi::c_int as isize))
                    .offset(
                        -((offset as ptrdiff_t
                            * *src_stride.offset(0 as core::ffi::c_int as isize))
                            as isize),
                    ),
                *src_stride.offset(0 as core::ffi::c_int as isize),
                0 as core::ffi::c_int,
                (*(*f).seq_hdr).sb128 as core::ffi::c_int,
                y_stripe,
                row_h,
                w,
                h,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            );
        }
    }
    if ((*(*f).seq_hdr).cdef as core::ffi::c_int != 0
        || restore_planes
            & (LR_RESTORE_U as core::ffi::c_int | LR_RESTORE_V as core::ffi::c_int) != 0)
        && (*f).cur.p.layout as core::ffi::c_uint
            != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int as core::ffi::c_uint
    {
        let ss_ver: core::ffi::c_int = ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let ss_hor: core::ffi::c_int = ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let h_0: core::ffi::c_int = (*f).cur.p.h + ss_ver >> ss_ver;
        let w_0: core::ffi::c_int = (*f).bw << 2 as core::ffi::c_int - ss_hor;
        let row_h_0: core::ffi::c_int = imin(
            (sby + 1 as core::ffi::c_int)
                << 6 as core::ffi::c_int - ss_ver
                    + (*(*f).seq_hdr).sb128 as core::ffi::c_int,
            h_0 - 1 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let offset_uv: core::ffi::c_int = offset >> ss_ver;
        let y_stripe_0: core::ffi::c_int = (sby
            << 6 as core::ffi::c_int - ss_ver
                + (*(*f).seq_hdr).sb128 as core::ffi::c_int) - offset_uv;
        let cdef_off_uv: ptrdiff_t = (sby * 4 as core::ffi::c_int) as ptrdiff_t
            * *src_stride.offset(1 as core::ffi::c_int as isize);
        if (*(*f).seq_hdr).cdef as core::ffi::c_int != 0
            || restore_planes & LR_RESTORE_U as core::ffi::c_int != 0
        {
            if restore_planes & LR_RESTORE_U as core::ffi::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[1 as core::ffi::c_int as usize],
                    *lr_stride.offset(1 as core::ffi::c_int as isize),
                    (*src.offset(1 as core::ffi::c_int as isize))
                        .offset(
                            -((offset_uv as ptrdiff_t
                                * *src_stride.offset(1 as core::ffi::c_int as isize))
                                as isize),
                        ),
                    *src_stride.offset(1 as core::ffi::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128 as core::ffi::c_int,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as core::ffi::c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf(
                    f,
                    ((*f).lf.cdef_lpf_line[1 as core::ffi::c_int as usize])
                        .offset(cdef_off_uv as isize),
                    *src_stride.offset(1 as core::ffi::c_int as isize),
                    (*src.offset(1 as core::ffi::c_int as isize))
                        .offset(
                            -((offset_uv as ptrdiff_t
                                * *src_stride.offset(1 as core::ffi::c_int as isize))
                                as isize),
                        ),
                    *src_stride.offset(1 as core::ffi::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128 as core::ffi::c_int,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as core::ffi::c_int,
                );
            }
        }
        if (*(*f).seq_hdr).cdef as core::ffi::c_int != 0
            || restore_planes & LR_RESTORE_V as core::ffi::c_int != 0
        {
            if restore_planes & LR_RESTORE_V as core::ffi::c_int != 0 || resize == 0 {
                backup_lpf(
                    f,
                    dst[2 as core::ffi::c_int as usize],
                    *lr_stride.offset(1 as core::ffi::c_int as isize),
                    (*src.offset(2 as core::ffi::c_int as isize))
                        .offset(
                            -((offset_uv as ptrdiff_t
                                * *src_stride.offset(1 as core::ffi::c_int as isize))
                                as isize),
                        ),
                    *src_stride.offset(1 as core::ffi::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128 as core::ffi::c_int,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    1 as core::ffi::c_int,
                );
            }
            if have_tt != 0 && resize != 0 {
                backup_lpf(
                    f,
                    ((*f).lf.cdef_lpf_line[2 as core::ffi::c_int as usize])
                        .offset(cdef_off_uv as isize),
                    *src_stride.offset(1 as core::ffi::c_int as isize),
                    (*src.offset(2 as core::ffi::c_int as isize))
                        .offset(
                            -((offset_uv as ptrdiff_t
                                * *src_stride.offset(1 as core::ffi::c_int as isize))
                                as isize),
                        ),
                    *src_stride.offset(1 as core::ffi::c_int as isize),
                    ss_ver,
                    (*(*f).seq_hdr).sb128 as core::ffi::c_int,
                    y_stripe_0,
                    row_h_0,
                    w_0,
                    h_0,
                    ss_hor,
                    0 as core::ffi::c_int,
                );
            }
        }
    }
}
#[inline]
unsafe extern "C" fn filter_plane_cols_y(
    f: *const Dav1dFrameContext,
    have_left: core::ffi::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 3],
    mut dst: *mut pixel,
    ls: ptrdiff_t,
    w: core::ffi::c_int,
    starty4: core::ffi::c_int,
    endy4: core::ffi::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [uint32_t; 4] = [0; 4];
            if starty4 == 0 {
                hmask[0 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as uint32_t;
                hmask[1 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as uint32_t;
                hmask[2 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[2 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as uint32_t;
                if endy4 > 16 as core::ffi::c_int {
                    hmask[0 as core::ffi::c_int as usize] = (hmask[0 as core::ffi::c_int
                        as usize] as core::ffi::c_uint
                        | ((*mask
                            .offset(
                                x as isize,
                            ))[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint) << 16 as core::ffi::c_int) as uint32_t;
                    hmask[1 as core::ffi::c_int as usize] = (hmask[1 as core::ffi::c_int
                        as usize] as core::ffi::c_uint
                        | ((*mask
                            .offset(
                                x as isize,
                            ))[1 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint) << 16 as core::ffi::c_int) as uint32_t;
                    hmask[2 as core::ffi::c_int as usize] = (hmask[2 as core::ffi::c_int
                        as usize] as core::ffi::c_uint
                        | ((*mask
                            .offset(
                                x as isize,
                            ))[2 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint) << 16 as core::ffi::c_int) as uint32_t;
                }
            } else {
                hmask[0 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                    as uint32_t;
                hmask[1 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                    as uint32_t;
                hmask[2 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[2 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                    as uint32_t;
            }
            hmask[3 as core::ffi::c_int as usize] = 0 as uint32_t;
            ((*dsp)
                .lf
                .loop_filter_sb[0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *dst.offset((x * 4 as core::ffi::c_int) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize))
                    .as_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                endy4 - starty4,
            );
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn filter_plane_rows_y(
    f: *const Dav1dFrameContext,
    have_top: core::ffi::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 3],
    mut dst: *mut pixel,
    ls: ptrdiff_t,
    w: core::ffi::c_int,
    starty4: core::ffi::c_int,
    endy4: core::ffi::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut y: core::ffi::c_int = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [uint32_t; 4] = [
                (*mask
                    .offset(
                        y as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        << 16 as core::ffi::c_int,
                (*mask
                    .offset(
                        y as isize,
                    ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        << 16 as core::ffi::c_int,
                (*mask
                    .offset(
                        y as isize,
                    ))[2 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[2 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        << 16 as core::ffi::c_int,
                0 as core::ffi::c_int as uint32_t,
            ];
            ((*dsp)
                .lf
                .loop_filter_sb[0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                dst,
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0 as core::ffi::c_int as isize))
                    .as_ptr()
                    .offset(1 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                w,
            );
        }
        y += 1;
        dst = dst.offset((4 as ptrdiff_t * ls) as isize);
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[inline]
unsafe extern "C" fn filter_plane_cols_uv(
    f: *const Dav1dFrameContext,
    have_left: core::ffi::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: core::ffi::c_int,
    starty4: core::ffi::c_int,
    endy4: core::ffi::c_int,
    ss_ver: core::ffi::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [uint32_t; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as uint32_t;
                hmask[1 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as uint32_t;
                if endy4 > 16 as core::ffi::c_int >> ss_ver {
                    hmask[0 as core::ffi::c_int as usize] = (hmask[0 as core::ffi::c_int
                        as usize] as core::ffi::c_uint
                        | ((*mask
                            .offset(
                                x as isize,
                            ))[0 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint) << (16 as core::ffi::c_int >> ss_ver))
                        as uint32_t;
                    hmask[1 as core::ffi::c_int as usize] = (hmask[1 as core::ffi::c_int
                        as usize] as core::ffi::c_uint
                        | ((*mask
                            .offset(
                                x as isize,
                            ))[1 as core::ffi::c_int
                            as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_uint) << (16 as core::ffi::c_int >> ss_ver))
                        as uint32_t;
                }
            } else {
                hmask[0 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                    as uint32_t;
                hmask[1 as core::ffi::c_int as usize] = (*mask
                    .offset(
                        x as isize,
                    ))[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                    as uint32_t;
            }
            hmask[2 as core::ffi::c_int as usize] = 0 as uint32_t;
            ((*dsp)
                .lf
                .loop_filter_sb[1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *u.offset((x * 4 as core::ffi::c_int) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize))
                    .as_ptr()
                    .offset(2 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                endy4 - starty4,
            );
            ((*dsp)
                .lf
                .loop_filter_sb[1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *v.offset((x * 4 as core::ffi::c_int) as isize),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize))
                    .as_ptr()
                    .offset(3 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                endy4 - starty4,
            );
        }
        x += 1;
    }
}
#[inline]
unsafe extern "C" fn filter_plane_rows_uv(
    f: *const Dav1dFrameContext,
    have_top: core::ffi::c_int,
    mut lvl: *const [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[uint16_t; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: core::ffi::c_int,
    starty4: core::ffi::c_int,
    endy4: core::ffi::c_int,
    ss_hor: core::ffi::c_int,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let mut off_l: ptrdiff_t = 0 as ptrdiff_t;
    let mut y: core::ffi::c_int = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [uint32_t; 3] = [
                (*mask
                    .offset(
                        y as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        << (16 as core::ffi::c_int >> ss_hor),
                (*mask
                    .offset(
                        y as isize,
                    ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                    as core::ffi::c_uint
                    | ((*mask
                        .offset(
                            y as isize,
                        ))[1 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        << (16 as core::ffi::c_int >> ss_hor),
                0 as core::ffi::c_int as uint32_t,
            ];
            ((*dsp)
                .lf
                .loop_filter_sb[1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *u.offset(off_l as isize),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0 as core::ffi::c_int as isize))
                    .as_ptr()
                    .offset(2 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                w,
            );
            ((*dsp)
                .lf
                .loop_filter_sb[1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *v.offset(off_l as isize),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0 as core::ffi::c_int as isize))
                    .as_ptr()
                    .offset(3 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [uint8_t; 4],
                b4_stride,
                &(*f).lf.lim_lut,
                w,
            );
        }
        y += 1;
        off_l += 4 as ptrdiff_t * ls;
        lvl = lvl.offset(b4_stride as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_cols_8bpc(
    f: *const Dav1dFrameContext,
    mut p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    mut sby: core::ffi::c_int,
    start_of_tile_row: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    let mut have_left: core::ffi::c_int = 0;
    let is_sb64: core::ffi::c_int = ((*(*f).seq_hdr).sb128 == 0) as core::ffi::c_int;
    let starty4: core::ffi::c_int = (sby & is_sb64) << 4 as core::ffi::c_int;
    let sbsz: core::ffi::c_int = 32 as core::ffi::c_int >> is_sb64;
    let sbl2: core::ffi::c_int = 5 as core::ffi::c_int - is_sb64;
    let halign: core::ffi::c_int = (*f).bh + 31 as core::ffi::c_int
        & !(31 as core::ffi::c_int);
    let ss_ver: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let vmask: core::ffi::c_int = 16 as core::ffi::c_int >> ss_ver;
    let hmask: core::ffi::c_int = 16 as core::ffi::c_int >> ss_hor;
    let vmax: core::ffi::c_uint = (1 as core::ffi::c_uint) << vmask;
    let hmax: core::ffi::c_uint = (1 as core::ffi::c_uint) << hmask;
    let endy4: core::ffi::c_uint = (starty4 + imin((*f).h4 - sby * sbsz, sbsz))
        as core::ffi::c_uint;
    let uv_endy4: core::ffi::c_uint = endy4.wrapping_add(ss_ver as core::ffi::c_uint)
        >> ss_ver;
    let mut lpf_y: *const uint8_t = &raw mut *(*((*f).lf.tx_lpf_right_edge)
        .as_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .offset((sby << sbl2) as isize) as *mut uint8_t;
    let mut lpf_uv: *const uint8_t = &raw mut *(*((*f).lf.tx_lpf_right_edge)
        .as_ptr()
        .offset(1 as core::ffi::c_int as isize))
        .offset((sby << sbl2 - ss_ver) as isize) as *mut uint8_t;
    let mut tile_col: core::ffi::c_int = 1 as core::ffi::c_int;
    loop {
        x = (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize] as core::ffi::c_int;
        if x << sbl2 >= (*f).bw {
            break;
        }
        let bx4: core::ffi::c_int = if x & is_sb64 != 0 {
            16 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        let cbx4: core::ffi::c_int = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
            .filter_y[0 as core::ffi::c_int as usize][bx4 as usize])
            .as_mut_ptr() as *mut [uint16_t; 2];
        let mut y: core::ffi::c_uint = starty4 as core::ffi::c_uint;
        let mut mask: core::ffi::c_uint = ((1 as core::ffi::c_int) << y)
            as core::ffi::c_uint;
        while y < endy4 {
            let sidx: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
                as core::ffi::c_int;
            let smask: core::ffi::c_uint = mask >> (sidx << 4 as core::ffi::c_int);
            let idx: core::ffi::c_int = 2 as core::ffi::c_int
                * ((*y_hmask.offset(2 as core::ffi::c_int as isize))[sidx as usize]
                    as core::ffi::c_uint & smask != 0) as core::ffi::c_int
                + ((*y_hmask.offset(1 as core::ffi::c_int as isize))[sidx as usize]
                    as core::ffi::c_uint & smask != 0) as core::ffi::c_int;
            let fresh0 = &raw mut (*y_hmask
                .offset(2 as core::ffi::c_int as isize))[sidx as usize];
            *fresh0 = (*fresh0 as core::ffi::c_uint & !smask) as uint16_t;
            let fresh1 = &raw mut (*y_hmask
                .offset(1 as core::ffi::c_int as isize))[sidx as usize];
            *fresh1 = (*fresh1 as core::ffi::c_uint & !smask) as uint16_t;
            let fresh2 = &raw mut (*y_hmask
                .offset(0 as core::ffi::c_int as isize))[sidx as usize];
            *fresh2 = (*fresh2 as core::ffi::c_uint & !smask) as uint16_t;
            let fresh3 = &raw mut (*y_hmask
                .offset(
                    imin(
                        idx,
                        *lpf_y
                            .offset(
                                y.wrapping_sub(starty4 as core::ffi::c_uint) as isize,
                            ) as core::ffi::c_int,
                    ) as isize,
                ))[sidx as usize];
            *fresh3 = (*fresh3 as core::ffi::c_uint | smask) as uint16_t;
            y = y.wrapping_add(1);
            mask <<= 1 as core::ffi::c_int;
        }
        if (*f).cur.p.layout as core::ffi::c_uint
            != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int as core::ffi::c_uint
        {
            let uv_hmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
                .filter_uv[0 as core::ffi::c_int as usize][cbx4 as usize])
                .as_mut_ptr() as *mut [uint16_t; 2];
            let mut y_0: core::ffi::c_uint = (starty4 >> ss_ver) as core::ffi::c_uint;
            let mut uv_mask: core::ffi::c_uint = ((1 as core::ffi::c_int) << y_0)
                as core::ffi::c_uint;
            while y_0 < uv_endy4 {
                let sidx_0: core::ffi::c_int = (uv_mask >= vmax) as core::ffi::c_int;
                let smask_0: core::ffi::c_uint = uv_mask
                    >> (sidx_0 << 4 as core::ffi::c_int - ss_ver);
                let idx_0: core::ffi::c_int = ((*uv_hmask
                    .offset(1 as core::ffi::c_int as isize))[sidx_0 as usize]
                    as core::ffi::c_uint & smask_0 != 0) as core::ffi::c_int;
                let fresh4 = &raw mut (*uv_hmask
                    .offset(1 as core::ffi::c_int as isize))[sidx_0 as usize];
                *fresh4 = (*fresh4 as core::ffi::c_uint & !smask_0) as uint16_t;
                let fresh5 = &raw mut (*uv_hmask
                    .offset(0 as core::ffi::c_int as isize))[sidx_0 as usize];
                *fresh5 = (*fresh5 as core::ffi::c_uint & !smask_0) as uint16_t;
                let fresh6 = &raw mut (*uv_hmask
                    .offset(
                        imin(
                            idx_0,
                            *lpf_uv
                                .offset(
                                    y_0.wrapping_sub((starty4 >> ss_ver) as core::ffi::c_uint)
                                        as isize,
                                ) as core::ffi::c_int,
                        ) as isize,
                    ))[sidx_0 as usize];
                *fresh6 = (*fresh6 as core::ffi::c_uint | smask_0) as uint16_t;
                y_0 = y_0.wrapping_add(1);
                uv_mask <<= 1 as core::ffi::c_int;
            }
        }
        lpf_y = lpf_y.offset(halign as isize);
        lpf_uv = lpf_uv.offset((halign >> ss_ver) as isize);
        tile_col += 1;
    }
    if start_of_tile_row != 0 {
        let mut a: *const BlockContext = 0 as *const BlockContext;
        x = 0 as core::ffi::c_int;
        a = &raw mut *((*f).a)
            .offset(((*f).sb128w * (start_of_tile_row - 1 as core::ffi::c_int)) as isize)
            as *mut BlockContext;
        while x < (*f).sb128w {
            let y_vmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
                .filter_y[1 as core::ffi::c_int as usize][starty4 as usize])
                .as_mut_ptr() as *mut [uint16_t; 2];
            let w: core::ffi::c_uint = imin(
                32 as core::ffi::c_int,
                (*f).w4 - (x << 5 as core::ffi::c_int),
            ) as core::ffi::c_uint;
            let mut mask_0: core::ffi::c_uint = 1 as core::ffi::c_uint;
            let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while i < w {
                let sidx_1: core::ffi::c_int = (mask_0 >= 0x10000 as core::ffi::c_uint)
                    as core::ffi::c_int;
                let smask_1: core::ffi::c_uint = mask_0
                    >> (sidx_1 << 4 as core::ffi::c_int);
                let idx_1: core::ffi::c_int = 2 as core::ffi::c_int
                    * ((*y_vmask.offset(2 as core::ffi::c_int as isize))[sidx_1 as usize]
                        as core::ffi::c_uint & smask_1 != 0) as core::ffi::c_int
                    + ((*y_vmask.offset(1 as core::ffi::c_int as isize))[sidx_1 as usize]
                        as core::ffi::c_uint & smask_1 != 0) as core::ffi::c_int;
                let fresh7 = &raw mut (*y_vmask
                    .offset(2 as core::ffi::c_int as isize))[sidx_1 as usize];
                *fresh7 = (*fresh7 as core::ffi::c_uint & !smask_1) as uint16_t;
                let fresh8 = &raw mut (*y_vmask
                    .offset(1 as core::ffi::c_int as isize))[sidx_1 as usize];
                *fresh8 = (*fresh8 as core::ffi::c_uint & !smask_1) as uint16_t;
                let fresh9 = &raw mut (*y_vmask
                    .offset(0 as core::ffi::c_int as isize))[sidx_1 as usize];
                *fresh9 = (*fresh9 as core::ffi::c_uint & !smask_1) as uint16_t;
                let fresh10 = &raw mut (*y_vmask
                    .offset(
                        imin(idx_1, (*a).tx_lpf_y[i as usize] as core::ffi::c_int)
                            as isize,
                    ))[sidx_1 as usize];
                *fresh10 = (*fresh10 as core::ffi::c_uint | smask_1) as uint16_t;
                mask_0 <<= 1 as core::ffi::c_int;
                i = i.wrapping_add(1);
            }
            if (*f).cur.p.layout as core::ffi::c_uint
                != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int as core::ffi::c_uint
            {
                let cw: core::ffi::c_uint = w.wrapping_add(ss_hor as core::ffi::c_uint)
                    >> ss_hor;
                let uv_vmask: *mut [uint16_t; 2] = ((*lflvl.offset(x as isize))
                    .filter_uv[1 as core::ffi::c_int
                    as usize][(starty4 >> ss_ver) as usize])
                    .as_mut_ptr() as *mut [uint16_t; 2];
                let mut uv_mask_0: core::ffi::c_uint = 1 as core::ffi::c_uint;
                let mut i_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
                while i_0 < cw {
                    let sidx_2: core::ffi::c_int = (uv_mask_0 >= hmax)
                        as core::ffi::c_int;
                    let smask_2: core::ffi::c_uint = uv_mask_0
                        >> (sidx_2 << 4 as core::ffi::c_int - ss_hor);
                    let idx_2: core::ffi::c_int = ((*uv_vmask
                        .offset(1 as core::ffi::c_int as isize))[sidx_2 as usize]
                        as core::ffi::c_uint & smask_2 != 0) as core::ffi::c_int;
                    let fresh11 = &raw mut (*uv_vmask
                        .offset(1 as core::ffi::c_int as isize))[sidx_2 as usize];
                    *fresh11 = (*fresh11 as core::ffi::c_uint & !smask_2) as uint16_t;
                    let fresh12 = &raw mut (*uv_vmask
                        .offset(0 as core::ffi::c_int as isize))[sidx_2 as usize];
                    *fresh12 = (*fresh12 as core::ffi::c_uint & !smask_2) as uint16_t;
                    let fresh13 = &raw mut (*uv_vmask
                        .offset(
                            imin(idx_2, (*a).tx_lpf_uv[i_0 as usize] as core::ffi::c_int)
                                as isize,
                        ))[sidx_2 as usize];
                    *fresh13 = (*fresh13 as core::ffi::c_uint | smask_2) as uint16_t;
                    uv_mask_0 <<= 1 as core::ffi::c_int;
                    i_0 = i_0.wrapping_add(1);
                }
            }
            x += 1;
            a = a.offset(1);
        }
    }
    let mut ptr: *mut pixel = 0 as *mut pixel;
    let mut level_ptr: *mut [uint8_t; 4] = ((*f).lf.level)
        .offset(((*f).b4_stride * sby as ptrdiff_t * sbsz as ptrdiff_t) as isize);
    ptr = *p.offset(0 as core::ffi::c_int as isize);
    have_left = 0 as core::ffi::c_int;
    x = 0 as core::ffi::c_int;
    while x < (*f).sb128w {
        filter_plane_cols_y(
            f,
            have_left,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[0 as core::ffi::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 3],
            ptr,
            (*f).cur.stride[0 as core::ffi::c_int as usize],
            imin(32 as core::ffi::c_int, (*f).w4 - x * 32 as core::ffi::c_int),
            starty4,
            endy4 as core::ffi::c_int,
        );
        x += 1;
        have_left = 1 as core::ffi::c_int;
        ptr = ptr.offset(128 as core::ffi::c_int as isize);
        level_ptr = level_ptr.offset(32 as core::ffi::c_int as isize);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0
        && (*(*f).frame_hdr).loopfilter.level_v == 0
    {
        return;
    }
    let mut uv_off: ptrdiff_t = 0;
    level_ptr = ((*f).lf.level)
        .offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as ptrdiff_t) as isize);
    uv_off = 0 as ptrdiff_t;
    have_left = 0 as core::ffi::c_int;
    x = 0 as core::ffi::c_int;
    while x < (*f).sb128w {
        filter_plane_cols_uv(
            f,
            have_left,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[0 as core::ffi::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 2],
            &raw mut *(*p.offset(1 as core::ffi::c_int as isize)).offset(uv_off as isize),
            &raw mut *(*p.offset(2 as core::ffi::c_int as isize)).offset(uv_off as isize),
            (*f).cur.stride[1 as core::ffi::c_int as usize],
            imin(32 as core::ffi::c_int, (*f).w4 - x * 32 as core::ffi::c_int) + ss_hor
                >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as core::ffi::c_int,
            ss_ver,
        );
        x += 1;
        have_left = 1 as core::ffi::c_int;
        uv_off += (128 as core::ffi::c_int >> ss_hor) as ptrdiff_t;
        level_ptr = level_ptr.offset((32 as core::ffi::c_int >> ss_hor) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_loopfilter_sbrow_rows_8bpc(
    f: *const Dav1dFrameContext,
    mut p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    mut sby: core::ffi::c_int,
) {
    let mut x: core::ffi::c_int = 0;
    let have_top: core::ffi::c_int = (sby > 0 as core::ffi::c_int) as core::ffi::c_int;
    let is_sb64: core::ffi::c_int = ((*(*f).seq_hdr).sb128 == 0) as core::ffi::c_int;
    let starty4: core::ffi::c_int = (sby & is_sb64) << 4 as core::ffi::c_int;
    let sbsz: core::ffi::c_int = 32 as core::ffi::c_int >> is_sb64;
    let ss_ver: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let endy4: core::ffi::c_uint = (starty4 + imin((*f).h4 - sby * sbsz, sbsz))
        as core::ffi::c_uint;
    let uv_endy4: core::ffi::c_uint = endy4.wrapping_add(ss_ver as core::ffi::c_uint)
        >> ss_ver;
    let mut ptr: *mut pixel = 0 as *mut pixel;
    let mut level_ptr: *mut [uint8_t; 4] = ((*f).lf.level)
        .offset(((*f).b4_stride * sby as ptrdiff_t * sbsz as ptrdiff_t) as isize);
    ptr = *p.offset(0 as core::ffi::c_int as isize);
    x = 0 as core::ffi::c_int;
    while x < (*f).sb128w {
        filter_plane_rows_y(
            f,
            have_top,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[1 as core::ffi::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 3],
            ptr,
            (*f).cur.stride[0 as core::ffi::c_int as usize],
            imin(32 as core::ffi::c_int, (*f).w4 - x * 32 as core::ffi::c_int),
            starty4,
            endy4 as core::ffi::c_int,
        );
        x += 1;
        ptr = ptr.offset(128 as core::ffi::c_int as isize);
        level_ptr = level_ptr.offset(32 as core::ffi::c_int as isize);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0
        && (*(*f).frame_hdr).loopfilter.level_v == 0
    {
        return;
    }
    let mut uv_off: ptrdiff_t = 0;
    level_ptr = ((*f).lf.level)
        .offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as ptrdiff_t) as isize);
    uv_off = 0 as ptrdiff_t;
    x = 0 as core::ffi::c_int;
    while x < (*f).sb128w {
        filter_plane_rows_uv(
            f,
            have_top,
            level_ptr as *const [uint8_t; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[1 as core::ffi::c_int as usize])
                .as_mut_ptr() as *const [[uint16_t; 2]; 2],
            &raw mut *(*p.offset(1 as core::ffi::c_int as isize)).offset(uv_off as isize),
            &raw mut *(*p.offset(2 as core::ffi::c_int as isize)).offset(uv_off as isize),
            (*f).cur.stride[1 as core::ffi::c_int as usize],
            imin(32 as core::ffi::c_int, (*f).w4 - x * 32 as core::ffi::c_int) + ss_hor
                >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as core::ffi::c_int,
            ss_hor,
        );
        x += 1;
        uv_off += (128 as core::ffi::c_int >> ss_hor) as ptrdiff_t;
        level_ptr = level_ptr.offset((32 as core::ffi::c_int >> ss_hor) as isize);
    }
}
