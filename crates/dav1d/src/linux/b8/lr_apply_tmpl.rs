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

// Mechanically translated from upstream/dav1d/src/lr_apply_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
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
    static dav1d_sgr_params: [[uint16_t; 2]; 16];
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = usize;
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
    pub bd_fn: C2RustUnnamed_28,
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
    pub frame_thread: C2RustUnnamed_20,
    pub lf: C2RustUnnamed_19,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [core::ffi::c_char; 40],
    pub __align: core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: core::ffi::c_int,
    pub __count: core::ffi::c_uint,
    pub __owner: core::ffi::c_int,
    pub __nusers: core::ffi::c_uint,
    pub __kind: core::ffi::c_int,
    pub __spins: core::ffi::c_short,
    pub __elision: core::ffi::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [core::ffi::c_char; 48],
    pub __align: core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [core::ffi::c_uint; 2],
    pub __g_size: [core::ffi::c_uint; 2],
    pub __g1_orig_size: core::ffi::c_uint,
    pub __wrefs: core::ffi::c_uint,
    pub __g_signals: [core::ffi::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub __low: core::ffi::c_uint,
    pub __high: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
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
pub struct C2RustUnnamed_20 {
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
    pub c2rust_unnamed: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub c2rust_unnamed: C2RustUnnamed_27,
    pub c2rust_unnamed_0: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub c2rust_unnamed: C2RustUnnamed_23,
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
pub union C2RustUnnamed_23 {
    pub c2rust_unnamed: C2RustUnnamed_26,
    pub c2rust_unnamed_0: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub mv2d: mv,
    pub matrix: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_25,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub y: int16_t,
    pub x: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub mv: [mv; 2],
    pub wedge_idx: uint8_t,
    pub mask_sign: uint8_t,
    pub interintra_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
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
pub struct C2RustUnnamed_28 {
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
    pub c2rust_unnamed: C2RustUnnamed_44,
    pub c2rust_unnamed_0: C2RustUnnamed_43,
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub scratch: C2RustUnnamed_31,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: core::ffi::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: C2RustUnnamed_30,
    pub task_thread: C2RustUnnamed_29,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
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
pub struct C2RustUnnamed_30 {
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
pub union C2RustUnnamed_31 {
    pub c2rust_unnamed: C2RustUnnamed_39,
    pub c2rust_unnamed_0: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub c2rust_unnamed: C2RustUnnamed_37,
    pub c2rust_unnamed_0: C2RustUnnamed_36,
    pub pal_idx_y: [uint8_t; 2048],
    pub pal_idx_uv: [uint8_t; 4096],
    pub c2rust_unnamed_1: C2RustUnnamed_33,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub c2rust_unnamed: C2RustUnnamed_35,
    pub c2rust_unnamed_0: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub interintra_16bpc: [uint16_t; 4096],
    pub edge_16bpc: [uint16_t; 257],
    pub __dav1d_align_pal_16bpc: [crate::platform::Align16; 0],
    pub pal_16bpc: [[uint16_t; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub interintra_8bpc: [uint8_t; 4096],
    pub edge_8bpc: [uint8_t; 257],
    pub __dav1d_align_pal_8bpc: [crate::platform::Align8; 0],
    pub pal_8bpc: [[uint8_t; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub ac: [int16_t; 1024],
    pub txtp_map: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_37 {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: C2RustUnnamed_38,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub c2rust_unnamed: C2RustUnnamed_41,
    pub c2rust_unnamed_0: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_41 {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: C2RustUnnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub compinter: [[int16_t; 16384]; 2],
    pub seg_mask: [uint8_t; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_43 {
    pub al_pal_8bpc: [[[[uint8_t; 8]; 3]; 32]; 2],
    pub al_pal_16bpc: [[[[uint16_t; 8]; 3]; 32]; 2],
}
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union C2RustUnnamed_44 {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: C2RustUnnamed_45,
    pub tile_row: C2RustUnnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub start: core::ffi::c_int,
    pub end: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: C2RustUnnamed_48,
    pub progress: [atomic_int; 2],
    pub frame_thread: [C2RustUnnamed_47; 2],
    pub lowest_pixel: *mut [[core::ffi::c_int; 2]; 7],
    pub dqmem: [[[uint16_t; 2]; 3]; 8],
    pub dq: *const [[uint16_t; 2]; 3],
    pub last_qidx: core::ffi::c_int,
    pub last_delta_lf: C2RustUnnamed_46,
    pub __dav1d_align_lflvlmem: [crate::platform::Align16; 0],
    pub lflvlmem: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[uint8_t; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_46 {
    pub i8_0: [int8_t; 4],
    pub u32_0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_47 {
    pub pal_idx: *mut uint8_t,
    pub cbi: *mut int16_t,
    pub cf: *mut coef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
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
    pub frame_thread: C2RustUnnamed_52,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [C2RustUnnamed_51; 8],
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
            ::core::ffi::VaList,
        ) -> (),
    >,
}
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
    pub sgr: C2RustUnnamed_49,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_49 {
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
    pub data: C2RustUnnamed_50,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_50 {
    pub cdf: *mut CdfContext,
    pub qcat: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_51 {
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
pub struct C2RustUnnamed_52 {
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
pub type LrRestorePlanes = core::ffi::c_uint;
pub const LR_RESTORE_V: LrRestorePlanes = 4;
pub const LR_RESTORE_U: LrRestorePlanes = 2;
pub const LR_RESTORE_Y: LrRestorePlanes = 1;
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn lr_stripe(
    f: *const Dav1dFrameContext,
    mut p: *mut pixel,
    mut left: *const [pixel; 4],
    mut x: core::ffi::c_int,
    mut y: core::ffi::c_int,
    plane: core::ffi::c_int,
    unit_w: core::ffi::c_int,
    row_h: core::ffi::c_int,
    lr: *const Av1RestorationUnit,
    mut edges: LrEdgeFlags,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let chroma: core::ffi::c_int = (plane != 0) as core::ffi::c_int;
    let ss_ver: core::ffi::c_int = chroma
        & ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
    let stride: ptrdiff_t = (*f).sr_cur.p.stride[chroma as usize];
    let sby: core::ffi::c_int = y
        + (if y != 0 {
            (8 as core::ffi::c_int) << ss_ver
        } else {
            0 as core::ffi::c_int
        }) >> 6 as core::ffi::c_int - ss_ver + (*(*f).seq_hdr).sb128 as core::ffi::c_int;
    let have_tt: core::ffi::c_int = ((*(*f).c).n_tc > 1 as core::ffi::c_uint)
        as core::ffi::c_int;
    let mut lpf: *const pixel = ((*f).lf.lr_lpf_line[plane as usize])
        .offset(
            ((have_tt
                * (sby
                    * ((4 as core::ffi::c_int)
                        << (*(*f).seq_hdr).sb128 as core::ffi::c_int)
                    - 4 as core::ffi::c_int)) as ptrdiff_t * stride) as isize,
        )
        .offset(x as isize);
    let mut stripe_h: core::ffi::c_int = imin(
        64 as core::ffi::c_int - 8 as core::ffi::c_int * (y == 0) as core::ffi::c_int
            >> ss_ver,
        row_h - y,
    );
    let mut lr_fn: looprestorationfilter_fn = None;
    let mut params: LooprestorationParams = LooprestorationParams {
        filter: [[0; 8]; 2],
    };
    if (*lr).type_0 as core::ffi::c_int == DAV1D_RESTORATION_WIENER as core::ffi::c_int {
        let filter: *mut [int16_t; 8] = (params.filter).as_mut_ptr()
            as *mut [int16_t; 8];
        let fresh0 = &raw mut (*filter
            .offset(0 as core::ffi::c_int as isize))[6 as core::ffi::c_int as usize];
        *fresh0 = (*lr).filter_h[0 as core::ffi::c_int as usize] as int16_t;
        (*filter
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = *fresh0;
        let fresh1 = &raw mut (*filter
            .offset(0 as core::ffi::c_int as isize))[5 as core::ffi::c_int as usize];
        *fresh1 = (*lr).filter_h[1 as core::ffi::c_int as usize] as int16_t;
        (*filter
            .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = *fresh1;
        let fresh2 = &raw mut (*filter
            .offset(0 as core::ffi::c_int as isize))[4 as core::ffi::c_int as usize];
        *fresh2 = (*lr).filter_h[2 as core::ffi::c_int as usize] as int16_t;
        (*filter
            .offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = *fresh2;
        (*filter
            .offset(0 as core::ffi::c_int as isize))[3 as core::ffi::c_int as usize] = (-((*filter
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            as core::ffi::c_int
            + (*filter
                .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize]
                as core::ffi::c_int
            + (*filter
                .offset(0 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize]
                as core::ffi::c_int) * 2 as core::ffi::c_int) as int16_t;
        let fresh3 = &raw mut (*filter
            .offset(1 as core::ffi::c_int as isize))[6 as core::ffi::c_int as usize];
        *fresh3 = (*lr).filter_v[0 as core::ffi::c_int as usize] as int16_t;
        (*filter
            .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = *fresh3;
        let fresh4 = &raw mut (*filter
            .offset(1 as core::ffi::c_int as isize))[5 as core::ffi::c_int as usize];
        *fresh4 = (*lr).filter_v[1 as core::ffi::c_int as usize] as int16_t;
        (*filter
            .offset(1 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize] = *fresh4;
        let fresh5 = &raw mut (*filter
            .offset(1 as core::ffi::c_int as isize))[4 as core::ffi::c_int as usize];
        *fresh5 = (*lr).filter_v[2 as core::ffi::c_int as usize] as int16_t;
        (*filter
            .offset(1 as core::ffi::c_int as isize))[2 as core::ffi::c_int as usize] = *fresh5;
        (*filter
            .offset(1 as core::ffi::c_int as isize))[3 as core::ffi::c_int as usize] = (128
            as core::ffi::c_int
            - ((*filter
                .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                as core::ffi::c_int
                + (*filter
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[1 as core::ffi::c_int as usize] as core::ffi::c_int
                + (*filter
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[2 as core::ffi::c_int as usize] as core::ffi::c_int)
                * 2 as core::ffi::c_int) as int16_t;
        lr_fn = (*dsp)
            .lr
            .wiener[((*filter
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
            as core::ffi::c_int
            | (*filter
                .offset(1 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                as core::ffi::c_int == 0) as core::ffi::c_int as usize];
    } else {
        if (*lr).type_0 as core::ffi::c_int
            >= DAV1D_RESTORATION_SGRPROJ as core::ffi::c_int
        {} else {
            __assert_fail(
                b"lr->type >= DAV1D_RESTORATION_SGRPROJ\0" as *const u8
                    as *const core::ffi::c_char,
                b"../dav1d-native-source/src/lr_apply_tmpl.c\0" as *const u8
                    as *const core::ffi::c_char,
                74 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 170],
                    [core::ffi::c_char; 170],
                >(
                    *b"void lr_stripe(const Dav1dFrameContext *const, pixel *, const pixel (*)[4], int, int, const int, const int, const int, const Av1RestorationUnit *const, enum LrEdgeFlags)\0",
                ))
                    .as_ptr(),
            );
        };
        let sgr_idx: core::ffi::c_int = (*lr).type_0 as core::ffi::c_int
            - DAV1D_RESTORATION_SGRPROJ as core::ffi::c_int as core::ffi::c_int;
        let sgr_params: *const uint16_t = (dav1d_sgr_params[sgr_idx as usize]).as_ptr();
        params.sgr.s0 = *sgr_params.offset(0 as core::ffi::c_int as isize) as uint32_t;
        params.sgr.s1 = *sgr_params.offset(1 as core::ffi::c_int as isize) as uint32_t;
        params.sgr.w0 = (*lr).sgr_weights[0 as core::ffi::c_int as usize] as int16_t;
        params.sgr.w1 = (128 as core::ffi::c_int
            - ((*lr).sgr_weights[0 as core::ffi::c_int as usize] as core::ffi::c_int
                + (*lr).sgr_weights[1 as core::ffi::c_int as usize] as core::ffi::c_int))
            as int16_t;
        lr_fn = (*dsp)
            .lr
            .sgr[((*sgr_params.offset(0 as core::ffi::c_int as isize) != 0)
            as core::ffi::c_int
            + (*sgr_params.offset(1 as core::ffi::c_int as isize) != 0)
                as core::ffi::c_int * 2 as core::ffi::c_int - 1 as core::ffi::c_int)
            as usize];
    }
    while y + stripe_h <= row_h {
        edges = ::core::mem::transmute::<
            core::ffi::c_uint,
            LrEdgeFlags,
        >(
            edges as core::ffi::c_uint
                ^ (-((sby + 1 as core::ffi::c_int != (*f).sbh || y + stripe_h != row_h)
                    as core::ffi::c_int) as core::ffi::c_uint
                    ^ edges as core::ffi::c_uint)
                    & LR_HAVE_BOTTOM as core::ffi::c_int as core::ffi::c_uint,
        );
        lr_fn
            .expect(
                "non-null function pointer",
            )(
            p,
            stride,
            left as const_left_pixel_row,
            lpf,
            unit_w,
            stripe_h,
            &raw mut params,
            edges,
        );
        left = left.offset(stripe_h as isize);
        y += stripe_h;
        p = p.offset((stripe_h as ptrdiff_t * stride) as isize);
        edges = ::core::mem::transmute::<
            core::ffi::c_uint,
            LrEdgeFlags,
        >(
            edges as core::ffi::c_uint
                | LR_HAVE_TOP as core::ffi::c_int as core::ffi::c_uint,
        );
        stripe_h = imin(64 as core::ffi::c_int >> ss_ver, row_h - y);
        if stripe_h == 0 as core::ffi::c_int {
            break;
        }
        lpf = lpf.offset((4 as ptrdiff_t * stride) as isize);
    }
}
unsafe extern "C" fn backup4xU(
    mut dst: *mut [pixel; 4],
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    mut u: core::ffi::c_int,
) {
    while u > 0 as core::ffi::c_int {
        memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            4 as size_t,
        );
        u -= 1;
        dst = dst.offset(1);
        src = src.offset(src_stride as isize);
    }
}
unsafe extern "C" fn lr_sbrow(
    f: *const Dav1dFrameContext,
    mut p: *mut pixel,
    y: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    row_h: core::ffi::c_int,
    plane: core::ffi::c_int,
) {
    let chroma: core::ffi::c_int = (plane != 0) as core::ffi::c_int;
    let ss_ver: core::ffi::c_int = chroma
        & ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = chroma
        & ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
    let p_stride: ptrdiff_t = (*f).sr_cur.p.stride[chroma as usize];
    let unit_size_log2: core::ffi::c_int = (*(*f).frame_hdr)
        .restoration
        .unit_size[(plane != 0) as core::ffi::c_int as usize] as core::ffi::c_int;
    let unit_size: core::ffi::c_int = (1 as core::ffi::c_int) << unit_size_log2;
    let half_unit_size: core::ffi::c_int = unit_size >> 1 as core::ffi::c_int;
    let max_unit_size: core::ffi::c_int = unit_size + half_unit_size;
    let row_y: core::ffi::c_int = y
        + (8 as core::ffi::c_int >> ss_ver) * (y != 0) as core::ffi::c_int;
    let shift_hor: core::ffi::c_int = 7 as core::ffi::c_int - ss_hor;
    let mut pre_lr_border: [[[pixel; 4]; 136]; 2] = [[[0; 4]; 136]; 2];
    let mut lr: [*const Av1RestorationUnit; 2] = [0 as *const Av1RestorationUnit; 2];
    let mut edges: LrEdgeFlags = ((if y > 0 as core::ffi::c_int {
        LR_HAVE_TOP as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    }) | LR_HAVE_RIGHT as core::ffi::c_int) as LrEdgeFlags;
    let mut aligned_unit_pos: core::ffi::c_int = row_y
        & !(unit_size - 1 as core::ffi::c_int);
    if aligned_unit_pos != 0 && aligned_unit_pos + half_unit_size > h {
        aligned_unit_pos -= unit_size;
    }
    aligned_unit_pos <<= ss_ver;
    let sb_idx: core::ffi::c_int = (aligned_unit_pos >> 7 as core::ffi::c_int)
        * (*f).sr_sb128w;
    let unit_idx: core::ffi::c_int = (aligned_unit_pos >> 6 as core::ffi::c_int
        & 1 as core::ffi::c_int) << 1 as core::ffi::c_int;
    lr[0 as core::ffi::c_int as usize] = &raw mut *(*((*((*f).lf.lr_mask)
        .offset(sb_idx as isize))
        .lr)
        .as_mut_ptr()
        .offset(plane as isize))
        .as_mut_ptr()
        .offset(unit_idx as isize) as *mut Av1RestorationUnit;
    let mut restore: core::ffi::c_int = ((*lr[0 as core::ffi::c_int as usize]).type_0
        as core::ffi::c_int != DAV1D_RESTORATION_NONE as core::ffi::c_int)
        as core::ffi::c_int;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut bit: core::ffi::c_int = 0 as core::ffi::c_int;
    let backup_h: core::ffi::c_int = row_h - y;
    while x + max_unit_size <= w {
        let next_x: core::ffi::c_int = x + unit_size;
        let next_u_idx: core::ffi::c_int = unit_idx
            + (next_x >> shift_hor - 1 as core::ffi::c_int & 1 as core::ffi::c_int);
        lr[(bit == 0) as core::ffi::c_int as usize] = &raw mut *(*((*((*f).lf.lr_mask)
            .offset((sb_idx + (next_x >> shift_hor)) as isize))
            .lr)
            .as_mut_ptr()
            .offset(plane as isize))
            .as_mut_ptr()
            .offset(next_u_idx as isize) as *mut Av1RestorationUnit;
        let restore_next: core::ffi::c_int = ((*lr[(bit == 0) as core::ffi::c_int
            as usize])
            .type_0 as core::ffi::c_int != DAV1D_RESTORATION_NONE as core::ffi::c_int)
            as core::ffi::c_int;
        if restore_next != 0 {
            backup4xU(
                (pre_lr_border[bit as usize]).as_mut_ptr() as *mut [pixel; 4],
                p.offset(unit_size as isize).offset(-(4 as core::ffi::c_int as isize)),
                p_stride,
                backup_h,
            );
        }
        if restore != 0 {
            lr_stripe(
                f,
                p,
                (pre_lr_border[(bit == 0) as core::ffi::c_int as usize]).as_mut_ptr()
                    as *const [pixel; 4],
                x,
                y,
                plane,
                unit_size,
                row_h,
                lr[bit as usize],
                edges,
            );
        }
        x = next_x;
        restore = restore_next;
        p = p.offset(unit_size as isize);
        edges = ::core::mem::transmute::<
            core::ffi::c_uint,
            LrEdgeFlags,
        >(
            edges as core::ffi::c_uint
                | LR_HAVE_LEFT as core::ffi::c_int as core::ffi::c_uint,
        );
        bit ^= 1 as core::ffi::c_int;
    }
    if restore != 0 {
        edges = ::core::mem::transmute::<
            core::ffi::c_uint,
            LrEdgeFlags,
        >(
            edges as core::ffi::c_uint
                & !(LR_HAVE_RIGHT as core::ffi::c_int) as core::ffi::c_uint,
        );
        let unit_w: core::ffi::c_int = w - x;
        lr_stripe(
            f,
            p,
            (pre_lr_border[(bit == 0) as core::ffi::c_int as usize]).as_mut_ptr()
                as *const [pixel; 4],
            x,
            y,
            plane,
            unit_w,
            row_h,
            lr[bit as usize],
            edges,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_lr_sbrow_8bpc(
    f: *mut Dav1dFrameContext,
    mut dst: *const *mut pixel,
    sby: core::ffi::c_int,
) {
    let offset_y: core::ffi::c_int = 8 as core::ffi::c_int
        * (sby != 0) as core::ffi::c_int;
    let dst_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let restore_planes: core::ffi::c_int = (*f).lf.restore_planes;
    let sb128: core::ffi::c_int = (*(*f).seq_hdr).sb128 as core::ffi::c_int;
    let not_last: core::ffi::c_int = ((sby + 1 as core::ffi::c_int) < (*f).sbh)
        as core::ffi::c_int;
    if restore_planes & LR_RESTORE_Y as core::ffi::c_int != 0 {
        let h: core::ffi::c_int = (*f).sr_cur.p.p.h;
        let w: core::ffi::c_int = (*f).sr_cur.p.p.w;
        let next_row_y: core::ffi::c_int = (sby + 1 as core::ffi::c_int)
            << 6 as core::ffi::c_int + sb128;
        let row_h: core::ffi::c_int = imin(
            next_row_y - 8 as core::ffi::c_int * not_last,
            h,
        ) as core::ffi::c_int;
        let y_stripe: core::ffi::c_int = (sby << 6 as core::ffi::c_int + sb128)
            - offset_y;
        lr_sbrow(
            f,
            (*dst.offset(0 as core::ffi::c_int as isize))
                .offset(
                    -((offset_y as ptrdiff_t
                        * *dst_stride.offset(0 as core::ffi::c_int as isize)) as isize),
                ),
            y_stripe,
            w,
            h,
            row_h,
            0 as core::ffi::c_int,
        );
    }
    if restore_planes
        & (LR_RESTORE_U as core::ffi::c_int | LR_RESTORE_V as core::ffi::c_int) != 0
    {
        let ss_ver: core::ffi::c_int = ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let ss_hor: core::ffi::c_int = ((*f).sr_cur.p.p.layout as core::ffi::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let h_0: core::ffi::c_int = (*f).sr_cur.p.p.h + ss_ver >> ss_ver;
        let w_0: core::ffi::c_int = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let next_row_y_0: core::ffi::c_int = (sby + 1 as core::ffi::c_int)
            << 6 as core::ffi::c_int - ss_ver + sb128;
        let row_h_0: core::ffi::c_int = imin(
            next_row_y_0 - (8 as core::ffi::c_int >> ss_ver) * not_last,
            h_0,
        ) as core::ffi::c_int;
        let offset_uv: core::ffi::c_int = offset_y >> ss_ver;
        let y_stripe_0: core::ffi::c_int = (sby
            << 6 as core::ffi::c_int - ss_ver + sb128) - offset_uv;
        if restore_planes & LR_RESTORE_U as core::ffi::c_int != 0 {
            lr_sbrow(
                f,
                (*dst.offset(1 as core::ffi::c_int as isize))
                    .offset(
                        -((offset_uv as ptrdiff_t
                            * *dst_stride.offset(1 as core::ffi::c_int as isize))
                            as isize),
                    ),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                1 as core::ffi::c_int,
            );
        }
        if restore_planes & LR_RESTORE_V as core::ffi::c_int != 0 {
            lr_sbrow(
                f,
                (*dst.offset(2 as core::ffi::c_int as isize))
                    .offset(
                        -((offset_uv as ptrdiff_t
                            * *dst_stride.offset(1 as core::ffi::c_int as isize))
                            as isize),
                    ),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                2 as core::ffi::c_int,
            );
        }
    }
}
