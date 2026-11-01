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

// Mechanically translated from upstream/dav1d/src/thread_task.c.
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
    fn abort() -> !;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
    fn prctl(__option: core::ffi::c_int, ...) -> core::ffi::c_int;
    fn dav1d_cdf_thread_update(
        hdr: *const Dav1dFrameHeader,
        dst: *mut CdfContext,
        src: *const CdfContext,
    );
    fn dav1d_decode_frame_init(f: *mut Dav1dFrameContext) -> core::ffi::c_int;
    fn dav1d_decode_frame_init_cdf(f: *mut Dav1dFrameContext) -> core::ffi::c_int;
    fn dav1d_decode_frame_exit(f: *mut Dav1dFrameContext, retval: core::ffi::c_int);
    fn dav1d_decode_tile_sbrow(t: *mut Dav1dTaskContext) -> core::ffi::c_int;
    fn dav1d_prep_grain_16bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
        scaling: *mut core::ffi::c_void,
        grain_lut: *mut core::ffi::c_void,
    );
    fn dav1d_prep_grain_8bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
        scaling: *mut core::ffi::c_void,
        grain_lut: *mut core::ffi::c_void,
    );
    fn dav1d_apply_grain_row_8bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
        scaling: *mut core::ffi::c_void,
        grain_lut: *mut core::ffi::c_void,
        row: core::ffi::c_int,
    );
    fn dav1d_apply_grain_row_16bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
        scaling: *mut core::ffi::c_void,
        grain_lut: *mut core::ffi::c_void,
        row: core::ffi::c_int,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type ptrdiff_t = isize;
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
pub type pixel = uint8_t;
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
    pub cf: *mut core::ffi::c_void,
    pub prog_sz: core::ffi::c_int,
    pub cbi_sz: core::ffi::c_int,
    pub pal_sz: core::ffi::c_int,
    pub pal_idx_sz: core::ffi::c_int,
    pub cf_sz: core::ffi::c_int,
    pub tile_start_off: *mut core::ffi::c_uint,
}
pub type coef = ();
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
    pub cf: *mut core::ffi::c_void,
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
pub type const_left_pixel_row = *const core::ffi::c_void;
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
pub type const_left_pixel_row_2px = *const core::ffi::c_void;
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
    unsafe extern "C" fn(
        *mut pixel,
        ptrdiff_t,
        *mut core::ffi::c_void,
        core::ffi::c_int,
    ) -> (),
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
pub const __ATOMIC_SEQ_CST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: core::ffi::c_int = -__INT_MAX__ - 1 as core::ffi::c_int;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn ctz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.trailing_zeros() as i32;
}
pub const PR_SET_NAME: core::ffi::c_int = 15 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn dav1d_set_thread_name(name: *const core::ffi::c_char) {
    prctl(PR_SET_NAME, name);
}
#[inline]
unsafe extern "C" fn imax(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn umin(
    a: core::ffi::c_uint,
    b: core::ffi::c_uint,
) -> core::ffi::c_uint {
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
pub const FG_BLOCK_SIZE: core::ffi::c_int = 32 as core::ffi::c_int;
pub const FRAME_ERROR: core::ffi::c_uint = UINT_MAX.wrapping_sub(1 as core::ffi::c_uint);
pub const TILE_ERROR: core::ffi::c_int = INT_MAX - 1 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn reset_task_cur(
    c: *const Dav1dContext,
    ttd: *mut TaskThreadData,
    mut frame_idx: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut min_frame_idx: core::ffi::c_uint = 0;
    let mut cur_frame_idx: core::ffi::c_uint = 0;
    let mut current_block: u64;
    let first: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*ttd).first,
    );
    let mut reset_frame_idx: core::ffi::c_uint = ::core::intrinsics::atomic_xchg::<_, {core::intrinsics::AtomicOrdering::SeqCst}>(
        &raw mut (*ttd).reset_task_cur,
        (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint),
    );
    if reset_frame_idx < first {
        if frame_idx == UINT_MAX {
            return 0 as core::ffi::c_int;
        }
        reset_frame_idx = UINT_MAX;
    }
    if (*ttd).cur == 0
        && ((*((*c).fc).offset(first as isize)).task_thread.task_cur_prev).is_null()
    {
        return 0 as core::ffi::c_int;
    }
    if reset_frame_idx != UINT_MAX {
        if frame_idx == UINT_MAX {
            if reset_frame_idx > first.wrapping_add((*ttd).cur) {
                return 0 as core::ffi::c_int;
            }
            (*ttd).cur = reset_frame_idx.wrapping_sub(first);
            current_block = 0;
        } else {
            current_block = 1;
        }
    } else {
        if frame_idx == UINT_MAX {
            return 0 as core::ffi::c_int;
        }
        current_block = 1;
    }
    match current_block {
        1 => {
            if frame_idx < first {
                frame_idx = frame_idx.wrapping_add((*c).n_fc);
            }
            min_frame_idx = umin(reset_frame_idx, frame_idx) as core::ffi::c_uint;
            cur_frame_idx = first.wrapping_add((*ttd).cur);
            if (*ttd).cur < (*c).n_fc && cur_frame_idx < min_frame_idx {
                return 0 as core::ffi::c_int;
            }
            (*ttd).cur = min_frame_idx.wrapping_sub(first);
            while (*ttd).cur < (*c).n_fc {
                if !((*((*c).fc)
                    .offset(
                        first.wrapping_add((*ttd).cur).wrapping_rem((*c).n_fc) as isize,
                    ))
                    .task_thread
                    .task_head)
                    .is_null()
                {
                    break;
                }
                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
            }
        }
        _ => {}
    }
    let mut i: core::ffi::c_uint = (*ttd).cur;
    while i < (*c).n_fc {
        let fresh0 = &raw mut (*((*c).fc)
            .offset(first.wrapping_add(i).wrapping_rem((*c).n_fc) as isize))
            .task_thread
            .task_cur_prev;
        *fresh0 = 0 as *mut Dav1dTask;
        i = i.wrapping_add(1);
    }
    return 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn reset_task_cur_async(
    ttd: *mut TaskThreadData,
    mut frame_idx: core::ffi::c_uint,
    mut n_frames: core::ffi::c_uint,
) {
    let first: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*ttd).first,
    );
    if frame_idx < first {
        frame_idx = frame_idx.wrapping_add(n_frames);
    }
    let mut last_idx: core::ffi::c_uint = frame_idx;
    loop {
        frame_idx = last_idx;
        last_idx = ::core::intrinsics::atomic_xchg::<_, {core::intrinsics::AtomicOrdering::SeqCst}>(
            &raw mut (*ttd).reset_task_cur,
            frame_idx,
        );
        if !(last_idx < frame_idx) {
            break;
        }
    }
    if frame_idx == first
        && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(&raw mut (*ttd).first as *mut atomic_uint)
            != first
    {
        let mut expected: core::ffi::c_uint = frame_idx;
        let fresh1 = ::core::intrinsics::atomic_cxchg::<_, {core::intrinsics::AtomicOrdering::SeqCst}, {core::intrinsics::AtomicOrdering::SeqCst}>(
            &raw mut (*ttd).reset_task_cur,
            *&raw mut expected,
            (2147483647 as core::ffi::c_uint)
                .wrapping_mul(2 as core::ffi::c_uint)
                .wrapping_add(1 as core::ffi::c_uint),
        );
        *&raw mut expected = fresh1.0;
        fresh1.1;
    }
}
unsafe extern "C" fn insert_tasks_between(
    f: *mut Dav1dFrameContext,
    first: *mut Dav1dTask,
    last: *mut Dav1dTask,
    a: *mut Dav1dTask,
    b: *mut Dav1dTask,
    cond_signal: core::ffi::c_int,
) {
    let ttd: *mut TaskThreadData = (*f).task_thread.ttd;
    if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*(*f).c).flush) != 0 {
        return;
    }
    if a.is_null() || (*a).next == b {} else {
        __assert_fail(
            b"!a || a->next == b\0" as *const u8 as *const core::ffi::c_char,
            b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                as *const core::ffi::c_char,
            105 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 135],
                [core::ffi::c_char; 135],
            >(
                *b"void insert_tasks_between(Dav1dFrameContext *const, Dav1dTask *const, Dav1dTask *const, Dav1dTask *const, Dav1dTask *const, const int)\0",
            ))
                .as_ptr(),
        );
    };
    if a.is_null() {
        (*f).task_thread.task_head = first as *mut Dav1dTask;
    } else {
        (*a).next = first as *mut Dav1dTask;
    }
    if b.is_null() {
        (*f).task_thread.task_tail = last as *mut Dav1dTask;
    }
    (*last).next = b as *mut Dav1dTask;
    reset_task_cur((*f).c, ttd, (*first).frame_idx);
    if cond_signal != 0
        && ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
            &raw mut (*ttd).cond_signaled as *mut atomic_int,
            1 as core::ffi::c_int,
        ) == 0
    {
        pthread_cond_signal(&raw mut (*ttd).cond);
    }
}
unsafe extern "C" fn insert_tasks(
    f: *mut Dav1dFrameContext,
    first: *mut Dav1dTask,
    last: *mut Dav1dTask,
    cond_signal: core::ffi::c_int,
) {
    let mut t_ptr: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut prev_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut current_block_22: u64;
    t_ptr = (*f).task_thread.task_head as *mut Dav1dTask;
    while !t_ptr.is_null() {
        if (*t_ptr).type_0 as core::ffi::c_uint
            == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int as core::ffi::c_uint
        {
            if (*first).type_0 as core::ffi::c_uint
                > DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int as core::ffi::c_uint
            {
                current_block_22 = 2;
            } else if (*first).sby > (*t_ptr).sby {
                current_block_22 = 2;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                current_block_22 = 3;
            }
        } else {
            if (*first).type_0 as core::ffi::c_uint
                == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int as core::ffi::c_uint
            {
                insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                return;
            }
            if (*first).sby > (*t_ptr).sby {
                current_block_22 = 2;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                if (*first).type_0 as core::ffi::c_uint
                    > (*t_ptr).type_0 as core::ffi::c_uint
                {
                    current_block_22 = 2;
                } else {
                    if ((*first).type_0 as core::ffi::c_uint)
                        < (*t_ptr).type_0 as core::ffi::c_uint
                    {
                        insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                        return;
                    }
                    current_block_22 = 3;
                }
            }
        }
        match current_block_22 {
            3 => {
                if (*first).type_0 as core::ffi::c_uint
                    == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as core::ffi::c_int
                        as core::ffi::c_uint
                    || (*first).type_0 as core::ffi::c_uint
                        == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int
                            as core::ffi::c_uint
                {} else {
                    __assert_fail(
                        b"first->type == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION || first->type == DAV1D_TASK_TYPE_TILE_ENTROPY\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                            as *const core::ffi::c_char,
                        155 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"void insert_tasks(Dav1dFrameContext *const, Dav1dTask *const, Dav1dTask *const, const int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if (*first).type_0 as core::ffi::c_uint
                    == (*t_ptr).type_0 as core::ffi::c_uint
                {} else {
                    __assert_fail(
                        b"first->type == t_ptr->type\0" as *const u8
                            as *const core::ffi::c_char,
                        b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                            as *const core::ffi::c_char,
                        156 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"void insert_tasks(Dav1dFrameContext *const, Dav1dTask *const, Dav1dTask *const, const int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if (*t_ptr).sby == (*first).sby {} else {
                    __assert_fail(
                        b"t_ptr->sby == first->sby\0" as *const u8
                            as *const core::ffi::c_char,
                        b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                            as *const core::ffi::c_char,
                        157 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"void insert_tasks(Dav1dFrameContext *const, Dav1dTask *const, Dav1dTask *const, const int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                let p: core::ffi::c_int = ((*first).type_0 as core::ffi::c_uint
                    == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int
                        as core::ffi::c_uint) as core::ffi::c_int;
                let t_tile_idx: core::ffi::c_int = first
                    .offset_from((*f).task_thread.tile_tasks[p as usize])
                    as core::ffi::c_long as core::ffi::c_int;
                let p_tile_idx: core::ffi::c_int = t_ptr
                    .offset_from((*f).task_thread.tile_tasks[p as usize])
                    as core::ffi::c_long as core::ffi::c_int;
                if t_tile_idx != p_tile_idx {} else {
                    __assert_fail(
                        b"t_tile_idx != p_tile_idx\0" as *const u8
                            as *const core::ffi::c_char,
                        b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                            as *const core::ffi::c_char,
                        161 as core::ffi::c_uint,
                        (::core::mem::transmute::<
                            [u8; 91],
                            [core::ffi::c_char; 91],
                        >(
                            *b"void insert_tasks(Dav1dFrameContext *const, Dav1dTask *const, Dav1dTask *const, const int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                if !(t_tile_idx > p_tile_idx) {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
            }
            _ => {}
        }
        prev_t = t_ptr;
        t_ptr = (*t_ptr).next as *mut Dav1dTask;
    }
    insert_tasks_between(f, first, last, prev_t, 0 as *mut Dav1dTask, cond_signal);
}
#[inline]
unsafe extern "C" fn insert_task(
    f: *mut Dav1dFrameContext,
    t: *mut Dav1dTask,
    cond_signal: core::ffi::c_int,
) {
    insert_tasks(f, t, t, cond_signal);
}
#[inline]
unsafe extern "C" fn add_pending(f: *mut Dav1dFrameContext, t: *mut Dav1dTask) {
    pthread_mutex_lock(&raw mut (*f).task_thread.pending_tasks.lock);
    (*t).next = 0 as *mut Dav1dTask;
    if ((*f).task_thread.pending_tasks.head).is_null() {
        (*f).task_thread.pending_tasks.head = t;
    } else {
        (*(*f).task_thread.pending_tasks.tail).next = t as *mut Dav1dTask;
    }
    (*f).task_thread.pending_tasks.tail = t;
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*f).task_thread.pending_tasks.merge,
        1 as core::ffi::c_int,
    );
    pthread_mutex_unlock(&raw mut (*f).task_thread.pending_tasks.lock);
}
#[inline]
unsafe extern "C" fn merge_pending_frame(f: *mut Dav1dFrameContext) -> core::ffi::c_int {
    let merge: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*f).task_thread.pending_tasks.merge,
    );
    if merge != 0 {
        pthread_mutex_lock(&raw mut (*f).task_thread.pending_tasks.lock);
        let mut t: *mut Dav1dTask = (*f).task_thread.pending_tasks.head;
        (*f).task_thread.pending_tasks.head = 0 as *mut Dav1dTask;
        (*f).task_thread.pending_tasks.tail = 0 as *mut Dav1dTask;
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*f).task_thread.pending_tasks.merge,
            0 as core::ffi::c_int,
        );
        pthread_mutex_unlock(&raw mut (*f).task_thread.pending_tasks.lock);
        while !t.is_null() {
            let tmp: *mut Dav1dTask = (*t).next as *mut Dav1dTask;
            insert_task(f, t, 0 as core::ffi::c_int);
            t = tmp;
        }
    }
    return merge;
}
#[inline]
unsafe extern "C" fn merge_pending(c: *const Dav1dContext) -> core::ffi::c_int {
    let mut res: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while i < (*c).n_fc {
        res |= merge_pending_frame(&raw mut *((*c).fc).offset(i as isize));
        i = i.wrapping_add(1);
    }
    return res;
}
unsafe extern "C" fn create_filter_sbrow(
    f: *mut Dav1dFrameContext,
    pass: core::ffi::c_int,
    mut res_t: *mut *mut Dav1dTask,
) -> core::ffi::c_int {
    let has_deblock: core::ffi::c_int = ((*(*f).frame_hdr)
        .loopfilter
        .level_y[0 as core::ffi::c_int as usize] as core::ffi::c_int != 0
        || (*(*f).frame_hdr).loopfilter.level_y[1 as core::ffi::c_int as usize]
            as core::ffi::c_int != 0) as core::ffi::c_int;
    let has_cdef: core::ffi::c_int = (*(*f).seq_hdr).cdef as core::ffi::c_int;
    let has_resize: core::ffi::c_int = ((*(*f).frame_hdr)
        .width[0 as core::ffi::c_int as usize]
        != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]) as core::ffi::c_int;
    let has_lr: core::ffi::c_int = (*f).lf.restore_planes;
    let mut tasks: *mut Dav1dTask = (*f).task_thread.tasks as *mut Dav1dTask;
    let uses_2pass: core::ffi::c_int = ((*(*f).c).n_fc > 1 as core::ffi::c_uint)
        as core::ffi::c_int;
    let mut num_tasks: core::ffi::c_int = (*f).sbh
        * (1 as core::ffi::c_int + uses_2pass);
    if num_tasks > (*f).task_thread.num_tasks {
        let size: size_t = (::core::mem::size_of::<Dav1dTask>() as size_t)
            .wrapping_mul(num_tasks as size_t);
        tasks = realloc((*f).task_thread.tasks as *mut core::ffi::c_void, size)
            as *mut Dav1dTask;
        if tasks.is_null() {
            return -(1 as core::ffi::c_int);
        }
        memset(tasks as *mut core::ffi::c_void, 0 as core::ffi::c_int, size);
        (*f).task_thread.tasks = tasks as *mut Dav1dTask;
        (*f).task_thread.num_tasks = num_tasks;
    }
    tasks = tasks.offset(((*f).sbh * (pass & 1 as core::ffi::c_int)) as isize);
    if pass & 1 as core::ffi::c_int != 0 {
        (*f).frame_thread.entropy_progress = 0 as core::ffi::c_int;
    } else {
        let prog_sz: core::ffi::c_int = ((*f).sbh + 31 as core::ffi::c_int
            & !(31 as core::ffi::c_int)) >> 5 as core::ffi::c_int;
        if prog_sz > (*f).frame_thread.prog_sz {
            let prog: *mut atomic_uint = realloc(
                (*f).frame_thread.frame_progress as *mut core::ffi::c_void,
                ((2 as core::ffi::c_int * prog_sz) as size_t)
                    .wrapping_mul(::core::mem::size_of::<atomic_uint>() as size_t),
            ) as *mut atomic_uint;
            if prog.is_null() {
                return -(1 as core::ffi::c_int);
            }
            (*f).frame_thread.frame_progress = prog;
            (*f).frame_thread.copy_lpf_progress = prog.offset(prog_sz as isize);
        }
        (*f).frame_thread.prog_sz = prog_sz;
        memset(
            (*f).frame_thread.frame_progress as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (prog_sz as size_t)
                .wrapping_mul(::core::mem::size_of::<atomic_uint>() as size_t),
        );
        memset(
            (*f).frame_thread.copy_lpf_progress as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (prog_sz as size_t)
                .wrapping_mul(::core::mem::size_of::<atomic_uint>() as size_t),
        );
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*f).frame_thread.deblock_progress,
            0 as core::ffi::c_int,
        );
    }
    (*f).frame_thread.next_tile_row[(pass & 1 as core::ffi::c_int) as usize] = 0
        as core::ffi::c_int;
    let mut t: *mut Dav1dTask = &raw mut *tasks.offset(0 as core::ffi::c_int as isize)
        as *mut Dav1dTask;
    (*t).sby = 0 as core::ffi::c_int;
    (*t).recon_progress = 1 as core::ffi::c_int;
    (*t).deblock_progress = 0 as core::ffi::c_int;
    (*t).type_0 = (if pass == 1 as core::ffi::c_int {
        DAV1D_TASK_TYPE_ENTROPY_PROGRESS as core::ffi::c_int
    } else if has_deblock != 0 {
        DAV1D_TASK_TYPE_DEBLOCK_COLS as core::ffi::c_int
    } else if has_cdef != 0 || has_lr != 0 {
        DAV1D_TASK_TYPE_DEBLOCK_ROWS as core::ffi::c_int
    } else if has_resize != 0 {
        DAV1D_TASK_TYPE_SUPER_RESOLUTION as core::ffi::c_int
    } else {
        DAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS as core::ffi::c_int
    }) as TaskType;
    (*t).frame_idx = f.offset_from((*(*f).c).fc) as core::ffi::c_long as core::ffi::c_int
        as core::ffi::c_uint;
    *res_t = t;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_task_create_tile_sbrow(
    f: *mut Dav1dFrameContext,
    pass: core::ffi::c_int,
    cond_signal: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut tasks: *mut Dav1dTask = (*f)
        .task_thread
        .tile_tasks[0 as core::ffi::c_int as usize];
    let uses_2pass: core::ffi::c_int = ((*(*f).c).n_fc > 1 as core::ffi::c_uint)
        as core::ffi::c_int;
    let n_tasks_per_pass: core::ffi::c_int = (*(*f).frame_hdr).tiling.cols
        as core::ffi::c_int * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int;
    let n_tasks: core::ffi::c_int = n_tasks_per_pass
        * (1 as core::ffi::c_int + uses_2pass);
    if pass < 2 as core::ffi::c_int {
        if n_tasks > (*f).task_thread.num_tile_tasks {
            let size: size_t = (::core::mem::size_of::<Dav1dTask>() as size_t)
                .wrapping_mul(n_tasks as size_t);
            tasks = realloc(
                (*f).task_thread.tile_tasks[0 as core::ffi::c_int as usize]
                    as *mut core::ffi::c_void,
                size,
            ) as *mut Dav1dTask;
            if tasks.is_null() {
                return -(1 as core::ffi::c_int);
            }
            memset(tasks as *mut core::ffi::c_void, 0 as core::ffi::c_int, size);
            (*f).task_thread.tile_tasks[0 as core::ffi::c_int as usize] = tasks
                as *mut Dav1dTask;
            (*f).task_thread.num_tile_tasks = n_tasks;
        }
        (*f).task_thread.tile_tasks[1 as core::ffi::c_int as usize] = tasks
            .offset(n_tasks_per_pass as isize) as *mut Dav1dTask;
    }
    if n_tasks <= (*f).task_thread.num_tile_tasks {} else {
        __assert_fail(
            b"n_tasks <= f->task_thread.num_tile_tasks\0" as *const u8
                as *const core::ffi::c_char,
            b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                as *const core::ffi::c_char,
            286 as core::ffi::c_uint,
            (::core::mem::transmute::<
                [u8; 81],
                [core::ffi::c_char; 81],
            >(
                *b"int dav1d_task_create_tile_sbrow(Dav1dFrameContext *const, const int, const int)\0",
            ))
                .as_ptr(),
        );
    };
    let mut pf_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    if create_filter_sbrow(f, pass, &raw mut pf_t) != 0 {
        return -(1 as core::ffi::c_int);
    }
    let p1_tasks: *mut Dav1dTask = (*f)
        .task_thread
        .tile_tasks[1 as core::ffi::c_int as usize];
    let mut prev_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    if pass == 2 as core::ffi::c_int {
        prev_t = &raw mut *p1_tasks
            .offset((n_tasks_per_pass - 1 as core::ffi::c_int) as isize)
            as *mut Dav1dTask;
        if (*(*f).frame_hdr).tiling.rows as core::ffi::c_int == 1 as core::ffi::c_int {
            prev_t = (*prev_t).next as *mut Dav1dTask;
        }
    }
    tasks = tasks.offset(((pass & 1 as core::ffi::c_int) * n_tasks_per_pass) as isize);
    let mut tile_idx: core::ffi::c_int = 0 as core::ffi::c_int;
    while tile_idx < n_tasks_per_pass {
        let ts: *mut Dav1dTileState = &raw mut *((*f).ts).offset(tile_idx as isize)
            as *mut Dav1dTileState;
        let mut t: *mut Dav1dTask = &raw mut *tasks.offset(tile_idx as isize)
            as *mut Dav1dTask;
        (*t).sby = (*ts).tiling.row_start >> (*f).sb_shift;
        if !pf_t.is_null() && (*t).sby != 0 {
            (*prev_t).next = pf_t as *mut Dav1dTask;
            prev_t = pf_t;
            pf_t = 0 as *mut Dav1dTask;
        }
        (*t).recon_progress = 0 as core::ffi::c_int;
        (*t).deblock_progress = 0 as core::ffi::c_int;
        (*t).deps_skip = 0 as core::ffi::c_int;
        (*t).type_0 = (if pass != 1 as core::ffi::c_int {
            DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as core::ffi::c_int
        } else {
            DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int
        }) as TaskType;
        (*t).frame_idx = f.offset_from((*(*f).c).fc) as core::ffi::c_long
            as core::ffi::c_int as core::ffi::c_uint;
        if !prev_t.is_null() {
            (*prev_t).next = t as *mut Dav1dTask;
        }
        prev_t = t;
        tile_idx += 1;
    }
    if !pf_t.is_null() {
        (*prev_t).next = pf_t as *mut Dav1dTask;
        prev_t = pf_t;
    }
    (*prev_t).next = 0 as *mut Dav1dTask;
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*f).task_thread.done)
            .as_mut_ptr()
            .offset((pass & 1 as core::ffi::c_int) as isize) as *mut atomic_int,
        0 as core::ffi::c_int,
    );
    if pass & 1 as core::ffi::c_int == 0 {
        pthread_mutex_lock(&raw mut (*f).task_thread.pending_tasks.lock);
        if ((*f).task_thread.pending_tasks.head).is_null() {} else {
            __assert_fail(
                b"f->task_thread.pending_tasks.head == NULL\0" as *const u8
                    as *const core::ffi::c_char,
                b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                    as *const core::ffi::c_char,
                332 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 81],
                    [core::ffi::c_char; 81],
                >(
                    *b"int dav1d_task_create_tile_sbrow(Dav1dFrameContext *const, const int, const int)\0",
                ))
                    .as_ptr(),
            );
        };
        (*f).task_thread.pending_tasks.head = (*f)
            .task_thread
            .tile_tasks[(pass == 2 as core::ffi::c_int) as core::ffi::c_int as usize]
            as *mut Dav1dTask;
        (*f).task_thread.pending_tasks.tail = prev_t;
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*f).task_thread.pending_tasks.merge,
            1 as core::ffi::c_int,
        );
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*f).task_thread.init_done,
            1 as core::ffi::c_int,
        );
        pthread_mutex_unlock(&raw mut (*f).task_thread.pending_tasks.lock);
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_task_frame_init(f: *mut Dav1dFrameContext) {
    let c: *const Dav1dContext = (*f).c;
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*f).task_thread.init_done,
        0 as core::ffi::c_int,
    );
    let t: *mut Dav1dTask = &raw mut (*f).task_thread.init_task;
    (*t).type_0 = DAV1D_TASK_TYPE_INIT;
    (*t).frame_idx = f.offset_from((*c).fc) as core::ffi::c_long as core::ffi::c_int
        as core::ffi::c_uint;
    (*t).sby = 0 as core::ffi::c_int;
    (*t).deblock_progress = 0 as core::ffi::c_int;
    (*t).recon_progress = (*t).deblock_progress;
    insert_task(f, t, 1 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_task_delayed_fg(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) {
    let ttd: *mut TaskThreadData = &raw mut (*c).task_thread;
    (*ttd).delayed_fg.in_0 = in_0;
    (*ttd).delayed_fg.out = out;
    (*ttd).delayed_fg.type_0 = DAV1D_TASK_TYPE_FG_PREP;
    *(&raw mut *((*ttd).delayed_fg.progress)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize) as *mut atomic_int) = 0
        as core::ffi::c_int;
    *(&raw mut *((*ttd).delayed_fg.progress)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as isize) as *mut atomic_int) = 0
        as core::ffi::c_int;
    pthread_mutex_lock(&raw mut (*ttd).lock);
    (*ttd).delayed_fg.exec = 1 as core::ffi::c_int;
    (*ttd).delayed_fg.finished = 0 as core::ffi::c_int;
    pthread_cond_signal(&raw mut (*ttd).cond);
    loop {
        pthread_cond_wait(&raw mut (*ttd).delayed_fg.cond, &raw mut (*ttd).lock);
        if !((*ttd).delayed_fg.finished == 0) {
            break;
        }
    }
    pthread_mutex_unlock(&raw mut (*ttd).lock);
}
#[inline]
unsafe extern "C" fn ensure_progress(
    ttd: *mut TaskThreadData,
    f: *mut Dav1dFrameContext,
    t: *mut Dav1dTask,
    type_0: TaskType,
    state: *mut atomic_int,
    target: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut p1: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(state);
    if p1 < (*t).sby {
        (*t).type_0 = type_0;
        (*t).deblock_progress = 0 as core::ffi::c_int;
        (*t).recon_progress = (*t).deblock_progress;
        *target = (*t).sby;
        add_pending(f, t);
        pthread_mutex_lock(&raw mut (*ttd).lock);
        return 1 as core::ffi::c_int;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn check_tile(
    t: *mut Dav1dTask,
    f: *mut Dav1dFrameContext,
    frame_mt: core::ffi::c_int,
) -> core::ffi::c_int {
    let tp: core::ffi::c_int = ((*t).type_0 as core::ffi::c_uint
        == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let tile_idx: core::ffi::c_int = t
        .offset_from((*f).task_thread.tile_tasks[tp as usize]) as core::ffi::c_long
        as core::ffi::c_int;
    let ts: *mut Dav1dTileState = &raw mut *((*f).ts).offset(tile_idx as isize)
        as *mut Dav1dTileState;
    let p1: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*ts).progress).as_mut_ptr().offset(tp as isize) as *mut atomic_int,
    );
    if p1 < (*t).sby {
        return 1 as core::ffi::c_int;
    }
    let mut error: core::ffi::c_int = (p1 == TILE_ERROR) as core::ffi::c_int;
    error |= ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(&raw mut (*f).task_thread.error, error);
    if error == 0 && frame_mt != 0 && tp == 0 {
        let p2: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut *((*ts).progress).as_mut_ptr().offset(1 as core::ffi::c_int as isize)
                as *mut atomic_int,
        );
        if p2 <= (*t).sby {
            return 1 as core::ffi::c_int;
        }
        error = (p2 == TILE_ERROR) as core::ffi::c_int;
        error
            |= ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(&raw mut (*f).task_thread.error, error);
    }
    if error == 0 && frame_mt != 0
        && (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
            != 0
    {
        let mut p: *const Dav1dThreadPicture = &raw mut (*f).sr_cur;
        let ss_ver: core::ffi::c_int = ((*p).p.p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int;
        let p_b: core::ffi::c_uint = (((*t).sby + 1 as core::ffi::c_int)
            << (*f).sb_shift + 2 as core::ffi::c_int) as core::ffi::c_uint;
        let tile_sby: core::ffi::c_int = (*t).sby
            - ((*ts).tiling.row_start >> (*f).sb_shift);
        let lowest_px: *const [core::ffi::c_int; 2] = (*((*ts).lowest_pixel)
            .offset(tile_sby as isize))
            .as_mut_ptr() as *const [core::ffi::c_int; 2];
        let mut current_block_14: u64;
        let mut n: core::ffi::c_int = (*t).deps_skip;
        while n < 7 as core::ffi::c_int {
            let mut lowest: core::ffi::c_uint = 0;
            if tp != 0 {
                lowest = p_b;
                current_block_14 = 4;
            } else {
                let y: core::ffi::c_int = if (*lowest_px
                    .offset(n as isize))[0 as core::ffi::c_int as usize] == INT_MIN
                {
                    INT_MIN
                } else {
                    (*lowest_px.offset(n as isize))[0 as core::ffi::c_int as usize]
                        + 8 as core::ffi::c_int
                };
                let uv: core::ffi::c_int = if (*lowest_px
                    .offset(n as isize))[1 as core::ffi::c_int as usize] == INT_MIN
                {
                    INT_MIN
                } else {
                    (*lowest_px.offset(n as isize))[1 as core::ffi::c_int as usize]
                        * ((1 as core::ffi::c_int) << ss_ver) + 8 as core::ffi::c_int
                };
                let max: core::ffi::c_int = imax(y, uv) as core::ffi::c_int;
                if max == INT_MIN {
                    current_block_14 = 5;
                } else {
                    lowest = iclip(
                        max,
                        1 as core::ffi::c_int,
                        (*f).refp[n as usize].p.p.h,
                    ) as core::ffi::c_uint;
                    current_block_14 = 4;
                }
            }
            match current_block_14 {
                4 => {
                    let p3: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                        &raw mut *((*((*f).refp).as_mut_ptr().offset(n as isize)).progress)
                            .offset((tp == 0) as core::ffi::c_int as isize)
                            as *mut atomic_uint,
                    );
                    if p3 < lowest {
                        return 1 as core::ffi::c_int;
                    }
                    ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                        &raw mut (*f).task_thread.error,
                        (p3
                            == (2147483647 as core::ffi::c_uint)
                                .wrapping_mul(2 as core::ffi::c_uint)
                                .wrapping_add(1 as core::ffi::c_uint)
                                .wrapping_sub(1 as core::ffi::c_uint)) as core::ffi::c_int,
                    );
                }
                _ => {}
            }
            n += 1;
            (*t).deps_skip += 1;
        }
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_frame_progress(
    c: *const Dav1dContext,
    f: *const Dav1dFrameContext,
) -> core::ffi::c_int {
    let mut frame_prog: core::ffi::c_uint = if (*c).n_fc > 1 as core::ffi::c_uint {
        ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut *((*f).sr_cur.progress).offset(1 as core::ffi::c_int as isize)
                as *mut atomic_uint,
        )
    } else {
        0 as core::ffi::c_uint
    };
    if frame_prog >= FRAME_ERROR {
        return (*f).sbh - 1 as core::ffi::c_int;
    }
    let mut idx: core::ffi::c_int = (frame_prog >> (*f).sb_shift + 7 as core::ffi::c_int)
        as core::ffi::c_int;
    let mut prog: core::ffi::c_int = 0;
    loop {
        let mut state: *mut atomic_uint = &raw mut *((*f).frame_thread.frame_progress)
            .offset(idx as isize) as *mut atomic_uint;
        let val: core::ffi::c_uint = !::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(state);
        prog = if val != 0 { ctz(val) } else { 32 as core::ffi::c_int };
        if prog != 32 as core::ffi::c_int {
            break;
        }
        prog = 0 as core::ffi::c_int;
        idx += 1;
        if !(idx < (*f).frame_thread.prog_sz) {
            break;
        }
    }
    return (idx << 5 as core::ffi::c_int | prog) - 1 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn abort_frame(f: *mut Dav1dFrameContext, error: core::ffi::c_int) {
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*f).task_thread.error,
        if error == -(22 as core::ffi::c_int) {
            1 as core::ffi::c_int
        } else {
            -(1 as core::ffi::c_int)
        },
    );
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut (*f).task_thread.task_counter,
        0 as core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*f).task_thread.done).as_mut_ptr().offset(0 as core::ffi::c_int as isize)
            as *mut atomic_int,
        1 as core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*f).task_thread.done).as_mut_ptr().offset(1 as core::ffi::c_int as isize)
            as *mut atomic_int,
        1 as core::ffi::c_int,
    );
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*f).sr_cur.progress).offset(0 as core::ffi::c_int as isize)
            as *mut atomic_uint,
        (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)
            .wrapping_sub(1 as core::ffi::c_uint),
    );
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*f).sr_cur.progress).offset(1 as core::ffi::c_int as isize)
            as *mut atomic_uint,
        (2147483647 as core::ffi::c_uint)
            .wrapping_mul(2 as core::ffi::c_uint)
            .wrapping_add(1 as core::ffi::c_uint)
            .wrapping_sub(1 as core::ffi::c_uint),
    );
    dav1d_decode_frame_exit(f, error);
    (*f).n_tile_data = 0 as core::ffi::c_int;
    pthread_cond_signal(&raw mut (*f).task_thread.cond);
}
#[inline]
unsafe extern "C" fn delayed_fg_task(c: *const Dav1dContext, ttd: *mut TaskThreadData) {
    let in_0: *const Dav1dPicture = (*ttd).delayed_fg.in_0;
    let out: *mut Dav1dPicture = (*ttd).delayed_fg.out;
    let mut off: core::ffi::c_int = 0;
    if (*out).p.bpc != 8 as core::ffi::c_int {
        off = ((*out).p.bpc >> 1 as core::ffi::c_int) - 4 as core::ffi::c_int;
    }
    let mut row: core::ffi::c_int = 0;
    let mut progmax: core::ffi::c_int = 0;
    let mut done: core::ffi::c_int = 0;
    match (*ttd).delayed_fg.type_0 as core::ffi::c_uint {
        11 => {
            (*ttd).delayed_fg.exec = 0 as core::ffi::c_int;
            if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                &raw mut (*ttd).cond_signaled as *mut atomic_int,
            ) != 0
            {
                pthread_cond_signal(&raw mut (*ttd).cond);
            }
            pthread_mutex_unlock(&raw mut (*ttd).lock);
            match (*out).p.bpc {
                8 => {
                    dav1d_prep_grain_8bpc(
                        &(*((*c).dsp).as_ptr().offset(0 as core::ffi::c_int as isize))
                            .fg,
                        out,
                        in_0,
                        ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed.scaling_8bpc)
                            .as_mut_ptr() as *mut core::ffi::c_void,
                        ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed.grain_lut_8bpc)
                            .as_mut_ptr() as *mut core::ffi::c_void,
                    );
                }
                10 | 12 => {
                    dav1d_prep_grain_16bpc(
                        &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                        out,
                        in_0,
                        ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed_0.scaling_16bpc)
                            .as_mut_ptr() as *mut core::ffi::c_void,
                        ((*ttd)
                            .delayed_fg
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .grain_lut_16bpc)
                            .as_mut_ptr() as *mut core::ffi::c_void,
                    );
                }
                _ => {
                    abort();
                }
            }
            (*ttd).delayed_fg.type_0 = DAV1D_TASK_TYPE_FG_APPLY;
            pthread_mutex_lock(&raw mut (*ttd).lock);
            (*ttd).delayed_fg.exec = 1 as core::ffi::c_int;
        }
        12 => {}
        _ => {
            abort();
        }
    }
    row = ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
        &raw mut *((*ttd).delayed_fg.progress)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
        1 as core::ffi::c_int,
    );
    pthread_mutex_unlock(&raw mut (*ttd).lock);
    progmax = ((*out).p.h + FG_BLOCK_SIZE - 1 as core::ffi::c_int) / FG_BLOCK_SIZE;
    while row < progmax {
        if (row + 1 as core::ffi::c_int) < progmax {
            pthread_cond_signal(&raw mut (*ttd).cond);
        } else {
            pthread_mutex_lock(&raw mut (*ttd).lock);
            (*ttd).delayed_fg.exec = 0 as core::ffi::c_int;
            pthread_mutex_unlock(&raw mut (*ttd).lock);
        }
        match (*out).p.bpc {
            8 => {
                dav1d_apply_grain_row_8bpc(
                    &(*((*c).dsp).as_ptr().offset(0 as core::ffi::c_int as isize)).fg,
                    out,
                    in_0,
                    ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed.scaling_8bpc)
                        .as_mut_ptr() as *mut core::ffi::c_void,
                    ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed.grain_lut_8bpc)
                        .as_mut_ptr() as *mut core::ffi::c_void,
                    row,
                );
            }
            10 | 12 => {
                dav1d_apply_grain_row_16bpc(
                    &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                    out,
                    in_0,
                    ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed_0.scaling_16bpc)
                        .as_mut_ptr() as *mut core::ffi::c_void,
                    ((*ttd).delayed_fg.c2rust_unnamed.c2rust_unnamed_0.grain_lut_16bpc)
                        .as_mut_ptr() as *mut core::ffi::c_void,
                    row,
                );
            }
            _ => {
                abort();
            }
        }
        row = ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
            &raw mut *((*ttd).delayed_fg.progress)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
            1 as core::ffi::c_int,
        );
        ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
            &raw mut *((*ttd).delayed_fg.progress)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as isize) as *mut atomic_int,
            1 as core::ffi::c_int,
        );
    }
    pthread_mutex_lock(&raw mut (*ttd).lock);
    (*ttd).delayed_fg.exec = 0 as core::ffi::c_int;
    done = ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
        &raw mut *((*ttd).delayed_fg.progress)
            .as_mut_ptr()
            .offset(1 as core::ffi::c_int as isize) as *mut atomic_int,
        1 as core::ffi::c_int,
    ) + 1 as core::ffi::c_int;
    progmax = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
        &raw mut *((*ttd).delayed_fg.progress)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
    );
    if done >= progmax {
        (*ttd).delayed_fg.finished = 1 as core::ffi::c_int;
        pthread_cond_signal(&raw mut (*ttd).delayed_fg.cond);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_worker_task(
    mut data: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut flush: core::ffi::c_int = 0;
    let mut error_0: core::ffi::c_int = 0;
    let mut sby: core::ffi::c_int = 0;
    let mut f: *mut Dav1dFrameContext = 0 as *mut Dav1dFrameContext;
    let mut t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut prev_t: *mut Dav1dTask = 0 as *mut Dav1dTask;
    let mut current_block: u64;
    let tc: *mut Dav1dTaskContext = data as *mut Dav1dTaskContext;
    let c: *const Dav1dContext = (*tc).c;
    let ttd: *mut TaskThreadData = (*tc).task_thread.ttd;
    dav1d_set_thread_name(b"dav1d-worker\0" as *const u8 as *const core::ffi::c_char);
    pthread_mutex_lock(&raw mut (*ttd).lock);
    's_18: while !((*tc).task_thread.die != 0) {
        if !(::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*c).flush) != 0) {
            merge_pending(c);
            if (*ttd).delayed_fg.exec != 0 {
                delayed_fg_task(c, ttd);
                continue;
            } else {
                f = 0 as *mut Dav1dFrameContext;
                t = 0 as *mut Dav1dTask;
                prev_t = 0 as *mut Dav1dTask;
                if (*c).n_fc > 1 as core::ffi::c_uint {
                    let mut i: core::ffi::c_uint = 0 as core::ffi::c_uint;
                    loop {
                        if !(i < (*c).n_fc) {
                            current_block = 6;
                            break;
                        }
                        let first: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                            &raw mut (*ttd).first,
                        );
                        f = &raw mut *((*c).fc)
                            .offset(
                                first.wrapping_add(i).wrapping_rem((*c).n_fc) as isize,
                            ) as *mut Dav1dFrameContext;
                        if !(::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                            &raw mut (*f).task_thread.init_done as *mut atomic_int,
                        ) != 0)
                        {
                            t = (*f).task_thread.task_head as *mut Dav1dTask;
                            if !t.is_null() {
                                if (*t).type_0 as core::ffi::c_uint
                                    == DAV1D_TASK_TYPE_INIT as core::ffi::c_int
                                        as core::ffi::c_uint
                                {
                                    current_block = 7;
                                    break;
                                }
                                if (*t).type_0 as core::ffi::c_uint
                                    == DAV1D_TASK_TYPE_INIT_CDF as core::ffi::c_int
                                        as core::ffi::c_uint
                                {
                                    let p1: core::ffi::c_int = (if !((*f).in_cdf.progress)
                                        .is_null()
                                    {
                                        ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*f).in_cdf.progress)
                                    } else {
                                        1 as core::ffi::c_uint
                                    }) as core::ffi::c_int;
                                    if p1 != 0 {
                                        ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                            &raw mut (*f).task_thread.error,
                                            (p1
                                                == 2147483647 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                as core::ffi::c_int,
                                        );
                                        current_block = 7;
                                        break;
                                    }
                                }
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    current_block = 6;
                }
                's_107: loop {
                    match current_block {
                        6 => {
                            if (*ttd).cur < (*c).n_fc {
                                let first_0: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut (*ttd).first,
                                );
                                f = &raw mut *((*c).fc)
                                    .offset(
                                        first_0.wrapping_add((*ttd).cur).wrapping_rem((*c).n_fc)
                                            as isize,
                                    ) as *mut Dav1dFrameContext;
                                merge_pending_frame(f);
                                prev_t = (*f).task_thread.task_cur_prev as *mut Dav1dTask;
                                t = (if !prev_t.is_null() {
                                    (*prev_t).next
                                } else {
                                    (*f).task_thread.task_head
                                }) as *mut Dav1dTask;
                                while !t.is_null() {
                                    if !((*t).type_0 as core::ffi::c_uint
                                        == DAV1D_TASK_TYPE_INIT_CDF as core::ffi::c_int
                                            as core::ffi::c_uint)
                                    {
                                        if (*t).type_0 as core::ffi::c_uint
                                            == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int
                                                as core::ffi::c_uint
                                            || (*t).type_0 as core::ffi::c_uint
                                                == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as core::ffi::c_int
                                                    as core::ffi::c_uint
                                        {
                                            if check_tile(
                                                t,
                                                f,
                                                ((*c).n_fc > 1 as core::ffi::c_uint) as core::ffi::c_int,
                                            ) == 0
                                            {
                                                current_block = 7;
                                                continue 's_107;
                                            }
                                        } else if (*t).recon_progress != 0 {
                                            let p: core::ffi::c_int = ((*t).type_0 as core::ffi::c_uint
                                                == DAV1D_TASK_TYPE_ENTROPY_PROGRESS as core::ffi::c_int
                                                    as core::ffi::c_uint) as core::ffi::c_int;
                                            let mut error: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut (*f).task_thread.error,
                                            );
                                            if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut *((*f).task_thread.done)
                                                    .as_mut_ptr()
                                                    .offset(p as isize) as *mut atomic_int,
                                            ) == 0 || error != 0
                                            {} else {
                                                __assert_fail(
                                                    b"!atomic_load(&f->task_thread.done[p]) || error\0"
                                                        as *const u8 as *const core::ffi::c_char,
                                                    b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    618 as core::ffi::c_uint,
                                                    (::core::mem::transmute::<
                                                        [u8; 32],
                                                        [core::ffi::c_char; 32],
                                                    >(*b"void *dav1d_worker_task(void *)\0"))
                                                        .as_ptr(),
                                                );
                                            };
                                            let tile_row_base: core::ffi::c_int = (*(*f).frame_hdr)
                                                .tiling
                                                .cols as core::ffi::c_int
                                                * (*f).frame_thread.next_tile_row[p as usize];
                                            if p != 0 {
                                                let prog: *mut atomic_int = &raw mut (*f)
                                                    .frame_thread
                                                    .entropy_progress;
                                                let p1_0: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                    prog,
                                                );
                                                if p1_0 < (*t).sby {
                                                    current_block = 8;
                                                } else {
                                                    ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                                        &raw mut (*f).task_thread.error,
                                                        (p1_0
                                                            == 2147483647 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                            as core::ffi::c_int,
                                                    );
                                                    current_block = 9;
                                                }
                                            } else {
                                                current_block = 9;
                                            }
                                            match current_block {
                                                8 => {}
                                                _ => {
                                                    let mut tc_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                                    loop {
                                                        if !(tc_0
                                                            < (*(*f).frame_hdr).tiling.cols as core::ffi::c_int)
                                                        {
                                                            current_block = 10;
                                                            break;
                                                        }
                                                        let ts: *mut Dav1dTileState = &raw mut *((*f).ts)
                                                            .offset((tile_row_base + tc_0) as isize)
                                                            as *mut Dav1dTileState;
                                                        let p2: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                            &raw mut *((*ts).progress).as_mut_ptr().offset(p as isize)
                                                                as *mut atomic_int,
                                                        );
                                                        if p2 < (*t).recon_progress {
                                                            current_block = 8;
                                                            break;
                                                        }
                                                        ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                                            &raw mut (*f).task_thread.error,
                                                            (p2
                                                                == 2147483647 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                                as core::ffi::c_int,
                                                        );
                                                        tc_0 += 1;
                                                    }
                                                    match current_block {
                                                        8 => {}
                                                        _ => {
                                                            if ((*t).sby + 1 as core::ffi::c_int) < (*f).sbh {
                                                                let mut next_t: *mut Dav1dTask = &raw mut *t
                                                                    .offset(1 as core::ffi::c_int as isize) as *mut Dav1dTask;
                                                                *next_t = *t;
                                                                (*next_t).sby += 1;
                                                                let ntr: core::ffi::c_int = (*f)
                                                                    .frame_thread
                                                                    .next_tile_row[p as usize] + 1 as core::ffi::c_int;
                                                                let start: core::ffi::c_int = (*(*f).frame_hdr)
                                                                    .tiling
                                                                    .row_start_sb[ntr as usize] as core::ffi::c_int;
                                                                if (*next_t).sby == start {
                                                                    (*f).frame_thread.next_tile_row[p as usize] = ntr;
                                                                }
                                                                (*next_t).recon_progress = (*next_t).sby
                                                                    + 1 as core::ffi::c_int;
                                                                insert_task(f, next_t, 0 as core::ffi::c_int);
                                                            }
                                                            current_block = 7;
                                                            continue 's_107;
                                                        }
                                                    }
                                                }
                                            }
                                        } else if (*t).type_0 as core::ffi::c_uint
                                            == DAV1D_TASK_TYPE_CDEF as core::ffi::c_int
                                                as core::ffi::c_uint
                                        {
                                            let mut prog_0: *mut atomic_uint = (*f)
                                                .frame_thread
                                                .copy_lpf_progress;
                                            let p1_1: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut *prog_0
                                                    .offset(
                                                        ((*t).sby - 1 as core::ffi::c_int >> 5 as core::ffi::c_int)
                                                            as isize,
                                                    ) as *mut atomic_uint,
                                            ) as core::ffi::c_int;
                                            if p1_1 as core::ffi::c_uint
                                                & (1 as core::ffi::c_uint)
                                                    << ((*t).sby - 1 as core::ffi::c_int
                                                        & 31 as core::ffi::c_int) != 0
                                            {
                                                current_block = 7;
                                                continue 's_107;
                                            }
                                        } else {
                                            if (*t).deblock_progress != 0 {} else {
                                                __assert_fail(
                                                    b"t->deblock_progress\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    652 as core::ffi::c_uint,
                                                    (::core::mem::transmute::<
                                                        [u8; 32],
                                                        [core::ffi::c_char; 32],
                                                    >(*b"void *dav1d_worker_task(void *)\0"))
                                                        .as_ptr(),
                                                );
                                            };
                                            let p1_2: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut (*f).frame_thread.deblock_progress,
                                            );
                                            if p1_2 >= (*t).deblock_progress {
                                                ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                                    &raw mut (*f).task_thread.error,
                                                    (p1_2
                                                        == 2147483647 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                        as core::ffi::c_int,
                                                );
                                                current_block = 7;
                                                continue 's_107;
                                            }
                                        }
                                    }
                                    prev_t = t;
                                    t = (*t).next as *mut Dav1dTask;
                                    (*f).task_thread.task_cur_prev = prev_t as *mut Dav1dTask;
                                }
                                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
                                current_block = 6;
                            } else {
                                if reset_task_cur(c, ttd, UINT_MAX) != 0 {
                                    continue 's_18;
                                }
                                if merge_pending(c) != 0 {
                                    continue 's_18;
                                } else {
                                    current_block = 11;
                                    break;
                                }
                            }
                        }
                        _ => {
                            if !prev_t.is_null() {
                                (*prev_t).next = (*t).next;
                            } else {
                                (*f).task_thread.task_head = (*t).next;
                            }
                            if ((*t).next).is_null() {
                                (*f).task_thread.task_tail = prev_t as *mut Dav1dTask;
                            }
                            if (*t).type_0 as core::ffi::c_uint
                                > DAV1D_TASK_TYPE_INIT_CDF as core::ffi::c_int
                                    as core::ffi::c_uint
                                && ((*f).task_thread.task_head).is_null()
                            {
                                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
                            }
                            (*t).next = 0 as *mut Dav1dTask;
                            ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                &raw mut (*ttd).cond_signaled,
                                1 as core::ffi::c_int,
                            );
                            pthread_cond_signal(&raw mut (*ttd).cond);
                            pthread_mutex_unlock(&raw mut (*ttd).lock);
                            current_block = 12;
                            break;
                        }
                    }
                }
                match current_block {
                    11 => {}
                    _ => {
                        loop {
                            flush = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*c).flush);
                            error_0 = ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                &raw mut (*f).task_thread.error,
                                flush,
                            ) | flush;
                            (*tc).f = f;
                            sby = (*t).sby;
                            match (*t).type_0 as core::ffi::c_uint {
                                0 => {
                                    if (*c).n_fc > 1 as core::ffi::c_uint {} else {
                                        __assert_fail(
                                            b"c->n_fc > 1\0" as *const u8 as *const core::ffi::c_char,
                                            b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                                                as *const core::ffi::c_char,
                                            701 as core::ffi::c_uint,
                                            (::core::mem::transmute::<
                                                [u8; 32],
                                                [core::ffi::c_char; 32],
                                            >(*b"void *dav1d_worker_task(void *)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                    let mut res: core::ffi::c_int = dav1d_decode_frame_init(f);
                                    let mut p1_3: core::ffi::c_int = (if !((*f).in_cdf.progress)
                                        .is_null()
                                    {
                                        ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*f).in_cdf.progress)
                                    } else {
                                        1 as core::ffi::c_uint
                                    }) as core::ffi::c_int;
                                    if res != 0 || p1_3 == TILE_ERROR {
                                        pthread_mutex_lock(&raw mut (*ttd).lock);
                                        abort_frame(
                                            f,
                                            if res != 0 { res } else { -(22 as core::ffi::c_int) },
                                        );
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        continue 's_18;
                                    } else {
                                        (*t).type_0 = DAV1D_TASK_TYPE_INIT_CDF;
                                        if p1_3 != 0 {
                                            continue;
                                        }
                                        add_pending(f, t);
                                        pthread_mutex_lock(&raw mut (*ttd).lock);
                                        continue 's_18;
                                    }
                                }
                                1 => {
                                    if (*c).n_fc > 1 as core::ffi::c_uint {} else {
                                        __assert_fail(
                                            b"c->n_fc > 1\0" as *const u8 as *const core::ffi::c_char,
                                            b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                                                as *const core::ffi::c_char,
                                            717 as core::ffi::c_uint,
                                            (::core::mem::transmute::<
                                                [u8; 32],
                                                [core::ffi::c_char; 32],
                                            >(*b"void *dav1d_worker_task(void *)\0"))
                                                .as_ptr(),
                                        );
                                    };
                                    let mut res_0: core::ffi::c_int = -(22 as core::ffi::c_int);
                                    if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        res_0 = dav1d_decode_frame_init_cdf(f);
                                    }
                                    if (*(*f).frame_hdr).refresh_context as core::ffi::c_int
                                        != 0 && (*f).task_thread.update_set == 0
                                    {
                                        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            (*f).out_cdf.progress,
                                            (if res_0 < 0 as core::ffi::c_int {
                                                2147483647 as core::ffi::c_int - 1 as core::ffi::c_int
                                            } else {
                                                1 as core::ffi::c_int
                                            }) as core::ffi::c_uint,
                                        );
                                    }
                                    let mut p_0: core::ffi::c_int = 1 as core::ffi::c_int;
                                    while p_0 <= 2 as core::ffi::c_int && res_0 == 0 {
                                        res_0 = dav1d_task_create_tile_sbrow(
                                            f,
                                            p_0,
                                            0 as core::ffi::c_int,
                                        );
                                        p_0 += 1;
                                    }
                                    pthread_mutex_lock(&raw mut (*ttd).lock);
                                    if res_0 != 0 {
                                        abort_frame(f, -(12 as core::ffi::c_int));
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            &raw mut (*f).task_thread.init_done,
                                            1 as core::ffi::c_int,
                                        );
                                    }
                                    continue 's_18;
                                }
                                2 | 4 => {
                                    let p_1: core::ffi::c_int = ((*t).type_0
                                        as core::ffi::c_uint
                                        == DAV1D_TASK_TYPE_TILE_ENTROPY as core::ffi::c_int
                                            as core::ffi::c_uint) as core::ffi::c_int;
                                    let tile_idx: core::ffi::c_int = t
                                        .offset_from((*f).task_thread.tile_tasks[p_1 as usize])
                                        as core::ffi::c_long as core::ffi::c_int;
                                    let ts_0: *mut Dav1dTileState = &raw mut *((*f).ts)
                                        .offset(tile_idx as isize) as *mut Dav1dTileState;
                                    (*tc).ts = ts_0;
                                    (*tc).by = sby << (*f).sb_shift;
                                    let uses_2pass: core::ffi::c_int = ((*c).n_fc
                                        > 1 as core::ffi::c_uint) as core::ffi::c_int;
                                    (*tc).frame_thread.pass = if uses_2pass == 0 {
                                        0 as core::ffi::c_int
                                    } else {
                                        1 as core::ffi::c_int
                                            + ((*t).type_0 as core::ffi::c_uint
                                                == DAV1D_TASK_TYPE_TILE_RECONSTRUCTION as core::ffi::c_int
                                                    as core::ffi::c_uint) as core::ffi::c_int
                                    };
                                    if error_0 == 0 {
                                        error_0 = dav1d_decode_tile_sbrow(tc);
                                    }
                                    let progress: core::ffi::c_int = if error_0 != 0 {
                                        TILE_ERROR
                                    } else {
                                        1 as core::ffi::c_int + sby
                                    };
                                    ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                        &raw mut (*f).task_thread.error,
                                        error_0,
                                    );
                                    if (sby + 1 as core::ffi::c_int) << (*f).sb_shift
                                        < (*ts_0).tiling.row_end
                                    {
                                        (*t).sby += 1;
                                        (*t).deps_skip = 0 as core::ffi::c_int;
                                        if check_tile(t, f, uses_2pass) == 0 {
                                            ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut *((*ts_0).progress).as_mut_ptr().offset(p_1 as isize)
                                                    as *mut atomic_int,
                                                progress,
                                            );
                                            reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                            if ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                                &raw mut (*ttd).cond_signaled as *mut atomic_int,
                                                1 as core::ffi::c_int,
                                            ) == 0
                                            {
                                                pthread_cond_signal(&raw mut (*ttd).cond);
                                            }
                                        } else {
                                            ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut *((*ts_0).progress).as_mut_ptr().offset(p_1 as isize)
                                                    as *mut atomic_int,
                                                progress,
                                            );
                                            add_pending(f, t);
                                            pthread_mutex_lock(&raw mut (*ttd).lock);
                                            continue 's_18;
                                        }
                                    } else {
                                        pthread_mutex_lock(&raw mut (*ttd).lock);
                                        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            &raw mut *((*ts_0).progress).as_mut_ptr().offset(p_1 as isize)
                                                as *mut atomic_int,
                                            progress,
                                        );
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            &raw mut (*f).task_thread.error,
                                        );
                                        if (*(*f).frame_hdr).refresh_context as core::ffi::c_int
                                            != 0 && (*tc).frame_thread.pass <= 1 as core::ffi::c_int
                                            && (*f).task_thread.update_set != 0
                                            && (*(*f).frame_hdr).tiling.update as core::ffi::c_int
                                                == tile_idx
                                        {
                                            if error_0 == 0 {
                                                dav1d_cdf_thread_update(
                                                    (*f).frame_hdr,
                                                    (*f).out_cdf.data.cdf,
                                                    &raw mut (*((*f).ts)
                                                        .offset((*(*f).frame_hdr).tiling.update as isize))
                                                        .cdf,
                                                );
                                            }
                                            if (*c).n_fc > 1 as core::ffi::c_uint {
                                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                    (*f).out_cdf.progress,
                                                    (if error_0 != 0 {
                                                        2147483647 as core::ffi::c_int - 1 as core::ffi::c_int
                                                    } else {
                                                        1 as core::ffi::c_int
                                                    }) as core::ffi::c_uint,
                                                );
                                            }
                                        }
                                        if ::core::intrinsics::atomic_xsub::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                            &raw mut (*f).task_thread.task_counter as *mut atomic_int,
                                            1 as core::ffi::c_int,
                                        ) - 1 as core::ffi::c_int == 0 as core::ffi::c_int
                                            && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut *((*f).task_thread.done)
                                                    .as_mut_ptr()
                                                    .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
                                            ) != 0
                                            && (uses_2pass == 0
                                                || ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                    &raw mut *((*f).task_thread.done)
                                                        .as_mut_ptr()
                                                        .offset(1 as core::ffi::c_int as isize) as *mut atomic_int,
                                                ) != 0)
                                        {
                                            error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                &raw mut (*f).task_thread.error,
                                            );
                                            dav1d_decode_frame_exit(
                                                f,
                                                if error_0 == 1 as core::ffi::c_int {
                                                    -(22 as core::ffi::c_int)
                                                } else if error_0 != 0 {
                                                    -(12 as core::ffi::c_int)
                                                } else {
                                                    0 as core::ffi::c_int
                                                },
                                            );
                                            (*f).n_tile_data = 0 as core::ffi::c_int;
                                            pthread_cond_signal(&raw mut (*f).task_thread.cond);
                                        }
                                        if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            &raw mut (*f).task_thread.task_counter as *mut atomic_int,
                                        ) >= 0 as core::ffi::c_int
                                        {} else {
                                            __assert_fail(
                                                b"atomic_load(&f->task_thread.task_counter) >= 0\0"
                                                    as *const u8 as *const core::ffi::c_char,
                                                b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                                                    as *const core::ffi::c_char,
                                                787 as core::ffi::c_uint,
                                                (::core::mem::transmute::<
                                                    [u8; 32],
                                                    [core::ffi::c_char; 32],
                                                >(*b"void *dav1d_worker_task(void *)\0"))
                                                    .as_ptr(),
                                            );
                                        };
                                        if ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                            &raw mut (*ttd).cond_signaled as *mut atomic_int,
                                            1 as core::ffi::c_int,
                                        ) == 0
                                        {
                                            pthread_cond_signal(&raw mut (*ttd).cond);
                                        }
                                        continue 's_18;
                                    }
                                }
                                5 => {
                                    if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_deblock_cols)
                                            .expect("non-null function pointer")(f, sby);
                                    }
                                    if ensure_progress(
                                        ttd,
                                        f,
                                        t,
                                        DAV1D_TASK_TYPE_DEBLOCK_ROWS,
                                        &raw mut (*f).frame_thread.deblock_progress,
                                        &raw mut (*t).deblock_progress,
                                    ) != 0
                                    {
                                        continue 's_18;
                                    } else {
                                        current_block = 13;
                                        break;
                                    }
                                }
                                6 => {
                                    current_block = 13;
                                    break;
                                }
                                7 => {
                                    current_block = 14;
                                    break;
                                }
                                8 => {
                                    current_block = 15;
                                    break;
                                }
                                9 => {
                                    current_block = 16;
                                    break;
                                }
                                10 => {
                                    current_block = 17;
                                    break;
                                }
                                3 => {
                                    current_block = 18;
                                    break;
                                }
                                _ => {
                                    abort();
                                }
                            }
                        }
                        match current_block {
                            13 => {
                                if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut (*f).task_thread.error as *mut atomic_int,
                                ) == 0
                                {
                                    ((*f).bd_fn.filter_sbrow_deblock_rows)
                                        .expect("non-null function pointer")(f, sby);
                                }
                                if (*(*f).frame_hdr)
                                    .loopfilter
                                    .level_y[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                    != 0
                                    || (*(*f).frame_hdr)
                                        .loopfilter
                                        .level_y[1 as core::ffi::c_int as usize] as core::ffi::c_int
                                        != 0
                                {
                                    error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error,
                                    );
                                    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).frame_thread.deblock_progress,
                                        if error_0 != 0 {
                                            2147483647 as core::ffi::c_int - 1 as core::ffi::c_int
                                        } else {
                                            sby + 1 as core::ffi::c_int
                                        },
                                    );
                                    reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                    if ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                        &raw mut (*ttd).cond_signaled as *mut atomic_int,
                                        1 as core::ffi::c_int,
                                    ) == 0
                                    {
                                        pthread_cond_signal(&raw mut (*ttd).cond);
                                    }
                                } else if (*(*f).seq_hdr).cdef as core::ffi::c_int != 0
                                    || (*f).lf.restore_planes != 0
                                {
                                    ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                        &raw mut *((*f).frame_thread.copy_lpf_progress)
                                            .offset((sby >> 5 as core::ffi::c_int) as isize)
                                            as *mut atomic_uint,
                                        (1 as core::ffi::c_uint) << (sby & 31 as core::ffi::c_int),
                                    );
                                    if sby != 0 {
                                        let mut prog_1: core::ffi::c_int = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            &raw mut *((*f).frame_thread.copy_lpf_progress)
                                                .offset(
                                                    (sby - 1 as core::ffi::c_int >> 5 as core::ffi::c_int)
                                                        as isize,
                                                ) as *mut atomic_uint,
                                        ) as core::ffi::c_int;
                                        if !prog_1 as core::ffi::c_uint
                                            & (1 as core::ffi::c_uint)
                                                << (sby - 1 as core::ffi::c_int & 31 as core::ffi::c_int)
                                            != 0
                                        {
                                            (*t).type_0 = DAV1D_TASK_TYPE_CDEF;
                                            (*t).deblock_progress = 0 as core::ffi::c_int;
                                            (*t).recon_progress = (*t).deblock_progress;
                                            add_pending(f, t);
                                            pthread_mutex_lock(&raw mut (*ttd).lock);
                                            continue;
                                        }
                                    }
                                }
                                current_block = 14;
                            }
                            _ => {}
                        }
                        match current_block {
                            14 => {
                                if (*(*f).seq_hdr).cdef != 0 {
                                    if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_cdef)
                                            .expect("non-null function pointer")(tc, sby);
                                    }
                                    reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                    if ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                        &raw mut (*ttd).cond_signaled as *mut atomic_int,
                                        1 as core::ffi::c_int,
                                    ) == 0
                                    {
                                        pthread_cond_signal(&raw mut (*ttd).cond);
                                    }
                                }
                                current_block = 15;
                            }
                            _ => {}
                        }
                        match current_block {
                            15 => {
                                if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                    != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
                                {
                                    if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_resize)
                                            .expect("non-null function pointer")(f, sby);
                                    }
                                }
                                current_block = 16;
                            }
                            _ => {}
                        }
                        match current_block {
                            16 => {
                                if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut (*f).task_thread.error as *mut atomic_int,
                                ) == 0 && (*f).lf.restore_planes != 0
                                {
                                    ((*f).bd_fn.filter_sbrow_lr)
                                        .expect("non-null function pointer")(f, sby);
                                }
                                current_block = 17;
                            }
                            _ => {}
                        }
                        match current_block {
                            17 => {}
                            _ => {}
                        }
                        let uses_2pass_0: core::ffi::c_int = ((*c).n_fc
                            > 1 as core::ffi::c_uint) as core::ffi::c_int;
                        let sbh: core::ffi::c_int = (*f).sbh;
                        let sbsz: core::ffi::c_int = (*f).sb_step
                            * 4 as core::ffi::c_int;
                        if (*t).type_0 as core::ffi::c_uint
                            == DAV1D_TASK_TYPE_ENTROPY_PROGRESS as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                &raw mut (*f).task_thread.error,
                            );
                            let y: core::ffi::c_uint = if sby + 1 as core::ffi::c_int
                                == sbh
                            {
                                UINT_MAX
                            } else {
                                ((sby + 1 as core::ffi::c_int) as core::ffi::c_uint)
                                    .wrapping_mul(sbsz as core::ffi::c_uint)
                            };
                            if (*c).n_fc > 1 as core::ffi::c_uint {} else {
                                __assert_fail(
                                    b"c->n_fc > 1\0" as *const u8 as *const core::ffi::c_char,
                                    b"../dav1d-native-source/src/thread_task.c\0" as *const u8
                                        as *const core::ffi::c_char,
                                    862 as core::ffi::c_uint,
                                    (::core::mem::transmute::<
                                        [u8; 32],
                                        [core::ffi::c_char; 32],
                                    >(*b"void *dav1d_worker_task(void *)\0"))
                                        .as_ptr(),
                                );
                            };
                            if !((*f).sr_cur.p.data[0 as core::ffi::c_int as usize])
                                .is_null()
                            {
                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut *((*f).sr_cur.progress)
                                        .offset(0 as core::ffi::c_int as isize) as *mut atomic_uint,
                                    if error_0 != 0 {
                                        (2147483647 as core::ffi::c_uint)
                                            .wrapping_mul(2 as core::ffi::c_uint)
                                            .wrapping_add(1 as core::ffi::c_uint)
                                            .wrapping_sub(1 as core::ffi::c_uint)
                                    } else {
                                        y
                                    },
                                );
                            }
                            ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                &raw mut (*f).frame_thread.entropy_progress,
                                if error_0 != 0 {
                                    2147483647 as core::ffi::c_int - 1 as core::ffi::c_int
                                } else {
                                    sby + 1 as core::ffi::c_int
                                },
                            );
                            if sby + 1 as core::ffi::c_int == sbh {
                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut *((*f).task_thread.done)
                                        .as_mut_ptr()
                                        .offset(1 as core::ffi::c_int as isize) as *mut atomic_int,
                                    1 as core::ffi::c_int,
                                );
                            }
                            pthread_mutex_lock(&raw mut (*ttd).lock);
                            let num_tasks: core::ffi::c_int = ::core::intrinsics::atomic_xsub::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                &raw mut (*f).task_thread.task_counter,
                                1 as core::ffi::c_int,
                            ) - 1 as core::ffi::c_int;
                            if (sby + 1 as core::ffi::c_int) < sbh && num_tasks != 0 {
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            } else {
                                if num_tasks == 0
                                    && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut *((*f).task_thread.done)
                                            .as_mut_ptr()
                                            .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
                                    ) != 0
                                    && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut *((*f).task_thread.done)
                                            .as_mut_ptr()
                                            .offset(1 as core::ffi::c_int as isize) as *mut atomic_int,
                                    ) != 0
                                {
                                    error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error,
                                    );
                                    dav1d_decode_frame_exit(
                                        f,
                                        if error_0 == 1 as core::ffi::c_int {
                                            -(22 as core::ffi::c_int)
                                        } else if error_0 != 0 {
                                            -(12 as core::ffi::c_int)
                                        } else {
                                            0 as core::ffi::c_int
                                        },
                                    );
                                    (*f).n_tile_data = 0 as core::ffi::c_int;
                                    pthread_cond_signal(&raw mut (*f).task_thread.cond);
                                }
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            }
                        } else {
                            ::core::intrinsics::atomic_or::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                &raw mut *((*f).frame_thread.frame_progress)
                                    .offset((sby >> 5 as core::ffi::c_int) as isize)
                                    as *mut atomic_uint,
                                (1 as core::ffi::c_uint) << (sby & 31 as core::ffi::c_int),
                            );
                            pthread_mutex_lock(&raw mut (*f).task_thread.lock);
                            sby = get_frame_progress(c, f);
                            error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                &raw mut (*f).task_thread.error,
                            );
                            let y_0: core::ffi::c_uint = if sby + 1 as core::ffi::c_int
                                == sbh
                            {
                                UINT_MAX
                            } else {
                                ((sby + 1 as core::ffi::c_int) as core::ffi::c_uint)
                                    .wrapping_mul(sbsz as core::ffi::c_uint)
                            };
                            if (*c).n_fc > 1 as core::ffi::c_uint
                                && !((*f).sr_cur.p.data[0 as core::ffi::c_int as usize])
                                    .is_null()
                            {
                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut *((*f).sr_cur.progress)
                                        .offset(1 as core::ffi::c_int as isize) as *mut atomic_uint,
                                    if error_0 != 0 {
                                        (2147483647 as core::ffi::c_uint)
                                            .wrapping_mul(2 as core::ffi::c_uint)
                                            .wrapping_add(1 as core::ffi::c_uint)
                                            .wrapping_sub(1 as core::ffi::c_uint)
                                    } else {
                                        y_0
                                    },
                                );
                            }
                            pthread_mutex_unlock(&raw mut (*f).task_thread.lock);
                            if sby + 1 as core::ffi::c_int == sbh {
                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                    &raw mut *((*f).task_thread.done)
                                        .as_mut_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
                                    1 as core::ffi::c_int,
                                );
                            }
                            pthread_mutex_lock(&raw mut (*ttd).lock);
                            let num_tasks_0: core::ffi::c_int = ::core::intrinsics::atomic_xsub::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                &raw mut (*f).task_thread.task_counter,
                                1 as core::ffi::c_int,
                            ) - 1 as core::ffi::c_int;
                            if (sby + 1 as core::ffi::c_int) < sbh && num_tasks_0 != 0 {
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            } else {
                                if num_tasks_0 == 0
                                    && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut *((*f).task_thread.done)
                                            .as_mut_ptr()
                                            .offset(0 as core::ffi::c_int as isize) as *mut atomic_int,
                                    ) != 0
                                    && (uses_2pass_0 == 0
                                        || ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                            &raw mut *((*f).task_thread.done)
                                                .as_mut_ptr()
                                                .offset(1 as core::ffi::c_int as isize) as *mut atomic_int,
                                        ) != 0)
                                {
                                    error_0 = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                        &raw mut (*f).task_thread.error,
                                    );
                                    dav1d_decode_frame_exit(
                                        f,
                                        if error_0 == 1 as core::ffi::c_int {
                                            -(22 as core::ffi::c_int)
                                        } else if error_0 != 0 {
                                            -(12 as core::ffi::c_int)
                                        } else {
                                            0 as core::ffi::c_int
                                        },
                                    );
                                    (*f).n_tile_data = 0 as core::ffi::c_int;
                                    pthread_cond_signal(&raw mut (*f).task_thread.cond);
                                }
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        (*tc).task_thread.flushed = 1 as core::ffi::c_int;
        pthread_cond_signal(&raw mut (*tc).task_thread.td.cond);
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*ttd).cond_signaled,
            0 as core::ffi::c_int,
        );
        pthread_cond_wait(&raw mut (*ttd).cond, &raw mut (*ttd).lock);
        (*tc).task_thread.flushed = 0 as core::ffi::c_int;
        reset_task_cur(c, ttd, UINT_MAX);
    }
    pthread_mutex_unlock(&raw mut (*ttd).lock);
    return NULL;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
