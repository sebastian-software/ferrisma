/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
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

// Mechanically translated from upstream/dav1d/src/obu.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn realloc(__ptr: *mut core::ffi::c_void, __size: size_t) -> *mut core::ffi::c_void;
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn dav1d_ref_create(size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_create_using_pool(
        pool: *mut Dav1dMemPool,
        size: size_t,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn dav1d_cdf_thread_ref(dst: *mut CdfThreadContext, src: *mut CdfThreadContext);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData);
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    static dav1d_default_wm_params: Dav1dWarpedMotionParams;
    fn dav1d_thread_picture_ref(
        dst: *mut Dav1dThreadPicture,
        src: *const Dav1dThreadPicture,
    );
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_picture_free_itut_t35(
        data: *const uint8_t,
        user_data: *mut core::ffi::c_void,
    );
    fn dav1d_picture_copy_props(
        p: *mut Dav1dPicture,
        content_light: *mut Dav1dContentLightLevel,
        content_light_ref: *mut Dav1dRef,
        mastering_display: *mut Dav1dMasteringDisplay,
        mastering_display_ref: *mut Dav1dRef,
        itut_t35: *mut Dav1dITUTT35,
        itut_t35_ref: *mut Dav1dRef,
        n_itut_t35: size_t,
        props: *const Dav1dDataProps,
    );
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    fn dav1d_submit_frame(c: *mut Dav1dContext) -> core::ffi::c_int;
    fn dav1d_init_get_bits(c: *mut GetBits, data: *const uint8_t, sz: size_t);
    fn dav1d_get_bit(c: *mut GetBits) -> core::ffi::c_uint;
    fn dav1d_get_bits(c: *mut GetBits, n: core::ffi::c_int) -> core::ffi::c_uint;
    fn dav1d_get_sbits(c: *mut GetBits, n: core::ffi::c_int) -> core::ffi::c_int;
    fn dav1d_get_uleb128(c: *mut GetBits) -> core::ffi::c_uint;
    fn dav1d_get_uniform(c: *mut GetBits, max: core::ffi::c_uint) -> core::ffi::c_uint;
    fn dav1d_get_vlc(c: *mut GetBits) -> core::ffi::c_uint;
    fn dav1d_get_bits_subexp(
        c: *mut GetBits,
        ref_0: core::ffi::c_int,
        n: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn dav1d_log(c: *mut Dav1dContext, format: *const core::ffi::c_char, ...);
}
pub type __builtin_va_list = *mut core::ffi::c_char;
pub type __darwin_size_t = usize;
pub type __darwin_va_list = __builtin_va_list;
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
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type va_list = __darwin_va_list;
pub type size_t = __darwin_size_t;
pub type ptrdiff_t = isize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub type atomic_int = core::ffi::c_int;
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
pub type memory_order = core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_uint = core::ffi::c_uint;
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
    pub cf: *mut core::ffi::c_void,
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
pub type Dav1dObuType = core::ffi::c_uint;
pub const DAV1D_OBU_PADDING: Dav1dObuType = 15;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const DAV1D_OBU_TD: Dav1dObuType = 2;
pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;
pub type ObuMetaType = core::ffi::c_uint;
pub const OBU_META_TIMECODE: ObuMetaType = 5;
pub const OBU_META_ITUT_T35: ObuMetaType = 4;
pub const OBU_META_SCALABILITY: ObuMetaType = 3;
pub const OBU_META_HDR_MDCV: ObuMetaType = 2;
pub const OBU_META_HDR_CLL: ObuMetaType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBits {
    pub state: uint64_t,
    pub bits_left: core::ffi::c_int,
    pub error: core::ffi::c_int,
    pub ptr: *const uint8_t,
    pub ptr_start: *const uint8_t,
    pub ptr_end: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itut_t35_ctx_context {
    pub itut_t35: *mut Dav1dITUTT35,
    pub n_itut_t35: size_t,
    pub ref_0: Dav1dRef,
}
pub const __ATOMIC_SEQ_CST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UINT8_MAX: core::ffi::c_int = 255 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
#[inline]
unsafe extern "C" fn clz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.leading_zeros() as i32;
}
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
unsafe extern "C" fn iclip_u8(v: core::ffi::c_int) -> core::ffi::c_int {
    return iclip(v, 0 as core::ffi::c_int, 255 as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn ulog2(v: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ clz(v);
}
pub const DAV1D_MAX_TILE_COLS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const DAV1D_MAX_TILE_ROWS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const DAV1D_MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const DAV1D_PRIMARY_REF_NONE: core::ffi::c_int = 7 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn dav1d_ref_init(
    ref_0: *mut Dav1dRef,
    ptr: *const core::ffi::c_void,
    free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    user_data: *mut core::ffi::c_void,
    free_ref: core::ffi::c_int,
) -> *mut Dav1dRef {
    (*ref_0).data = NULL;
    (*ref_0).const_data = ptr;
    *&raw mut (*ref_0).ref_cnt = 1 as core::ffi::c_int;
    (*ref_0).free_ref = free_ref;
    (*ref_0).free_callback = free_callback;
    (*ref_0).user_data = user_data;
    return ref_0;
}
#[inline]
unsafe extern "C" fn dav1d_ref_inc(ref_0: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::Relaxed}>(
        &raw mut (*ref_0).ref_cnt,
        1 as core::ffi::c_int,
    );
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
unsafe extern "C" fn dav1d_bytealign_get_bits(mut c: *mut GetBits) {
    if !((*c).bits_left <= 7 as core::ffi::c_int) {
        unreachable!();
    }
    (*c).bits_left = 0 as core::ffi::c_int;
    (*c).state = 0 as uint64_t;
}
pub const FRAME_ERROR: core::ffi::c_uint = UINT_MAX.wrapping_sub(1 as core::ffi::c_uint);
unsafe extern "C" fn check_trailing_bits(
    gb: *mut GetBits,
    strict_std_compliance: core::ffi::c_int,
) -> core::ffi::c_int {
    let trailing_one_bit: core::ffi::c_int = dav1d_get_bit(gb) as core::ffi::c_int;
    if (*gb).error != 0 {
        return -(22 as core::ffi::c_int);
    }
    if strict_std_compliance == 0 {
        return 0 as core::ffi::c_int;
    }
    if trailing_one_bit == 0 || (*gb).state != 0 {
        return -(22 as core::ffi::c_int);
    }
    let mut size: ptrdiff_t = ((*gb).ptr_end).offset_from((*gb).ptr) as ptrdiff_t;
    while size > 0 as ptrdiff_t
        && *((*gb).ptr).offset((size - 1 as ptrdiff_t) as isize) as core::ffi::c_int
            == 0 as core::ffi::c_int
    {
        size -= 1;
    }
    if size != 0 {
        return -(22 as core::ffi::c_int);
    }
    return 0 as core::ffi::c_int;
}
#[inline(never)]
unsafe extern "C" fn parse_seq_hdr(
    hdr: *mut Dav1dSequenceHeader,
    gb: *mut GetBits,
    strict_std_compliance: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut current_block: u64;
    memset(
        hdr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<Dav1dSequenceHeader>() as size_t,
    );
    (*hdr).profile = dav1d_get_bits(gb, 3 as core::ffi::c_int) as uint8_t;
    if !((*hdr).profile as core::ffi::c_int > 2 as core::ffi::c_int) {
        (*hdr).still_picture = dav1d_get_bit(gb) as uint8_t;
        (*hdr).reduced_still_picture_header = dav1d_get_bit(gb) as uint8_t;
        if !((*hdr).reduced_still_picture_header as core::ffi::c_int != 0
            && (*hdr).still_picture == 0)
        {
            if (*hdr).reduced_still_picture_header != 0 {
                (*hdr).num_operating_points = 1 as uint8_t;
                (*hdr).operating_points[0 as core::ffi::c_int as usize].major_level = dav1d_get_bits(
                    gb,
                    3 as core::ffi::c_int,
                ) as uint8_t;
                (*hdr).operating_points[0 as core::ffi::c_int as usize].minor_level = dav1d_get_bits(
                    gb,
                    2 as core::ffi::c_int,
                ) as uint8_t;
                (*hdr)
                    .operating_points[0 as core::ffi::c_int as usize]
                    .initial_display_delay = 10 as uint8_t;
                current_block = 0;
            } else {
                (*hdr).timing_info_present = dav1d_get_bit(gb) as uint8_t;
                if (*hdr).timing_info_present != 0 {
                    (*hdr).num_units_in_tick = dav1d_get_bits(gb, 32 as core::ffi::c_int)
                        as uint32_t;
                    (*hdr).time_scale = dav1d_get_bits(gb, 32 as core::ffi::c_int)
                        as uint32_t;
                    if strict_std_compliance != 0
                        && ((*hdr).num_units_in_tick == 0 || (*hdr).time_scale == 0)
                    {
                        current_block = 1;
                    } else {
                        (*hdr).equal_picture_interval = dav1d_get_bit(gb) as uint8_t;
                        if (*hdr).equal_picture_interval != 0 {
                            let num_ticks_per_picture: core::ffi::c_uint = dav1d_get_vlc(
                                gb,
                            ) as core::ffi::c_uint;
                            if num_ticks_per_picture == UINT32_MAX {
                                current_block = 1;
                            } else {
                                (*hdr).num_ticks_per_picture = num_ticks_per_picture
                                    .wrapping_add(1 as core::ffi::c_uint) as uint32_t;
                                current_block = 2;
                            }
                        } else {
                            current_block = 2;
                        }
                        match current_block {
                            1 => {}
                            _ => {
                                (*hdr).decoder_model_info_present = dav1d_get_bit(gb)
                                    as uint8_t;
                                if (*hdr).decoder_model_info_present != 0 {
                                    (*hdr).encoder_decoder_buffer_delay_length = (dav1d_get_bits(
                                        gb,
                                        5 as core::ffi::c_int,
                                    ))
                                        .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                                    (*hdr).num_units_in_decoding_tick = dav1d_get_bits(
                                        gb,
                                        32 as core::ffi::c_int,
                                    ) as uint32_t;
                                    if strict_std_compliance != 0
                                        && (*hdr).num_units_in_decoding_tick == 0
                                    {
                                        current_block = 1;
                                    } else {
                                        (*hdr).buffer_removal_delay_length = (dav1d_get_bits(
                                            gb,
                                            5 as core::ffi::c_int,
                                        ))
                                            .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                                        (*hdr).frame_presentation_delay_length = (dav1d_get_bits(
                                            gb,
                                            5 as core::ffi::c_int,
                                        ))
                                            .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                                        current_block = 3;
                                    }
                                } else {
                                    current_block = 3;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 3;
                }
                match current_block {
                    1 => {}
                    _ => {
                        (*hdr).display_model_info_present = dav1d_get_bit(gb) as uint8_t;
                        (*hdr).num_operating_points = (dav1d_get_bits(
                            gb,
                            5 as core::ffi::c_int,
                        ))
                            .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                        loop {
                            if !(i < (*hdr).num_operating_points as core::ffi::c_int) {
                                current_block = 0;
                                break;
                            }
                            let op: *mut Dav1dSequenceHeaderOperatingPoint = &raw mut *((*hdr)
                                .operating_points)
                                .as_mut_ptr()
                                .offset(i as isize)
                                as *mut Dav1dSequenceHeaderOperatingPoint;
                            (*op).idc = dav1d_get_bits(gb, 12 as core::ffi::c_int)
                                as uint16_t;
                            if (*op).idc as core::ffi::c_int != 0
                                && ((*op).idc as core::ffi::c_int & 0xff as core::ffi::c_int
                                    == 0
                                    || (*op).idc as core::ffi::c_int & 0xf00 as core::ffi::c_int
                                        == 0)
                            {
                                current_block = 1;
                                break;
                            }
                            (*op).major_level = (2 as core::ffi::c_uint)
                                .wrapping_add(dav1d_get_bits(gb, 3 as core::ffi::c_int))
                                as uint8_t;
                            (*op).minor_level = dav1d_get_bits(gb, 2 as core::ffi::c_int)
                                as uint8_t;
                            if (*op).major_level as core::ffi::c_int
                                > 3 as core::ffi::c_int
                            {
                                (*op).tier = dav1d_get_bit(gb) as uint8_t;
                            }
                            if (*hdr).decoder_model_info_present != 0 {
                                (*op).decoder_model_param_present = dav1d_get_bit(gb)
                                    as uint8_t;
                                if (*op).decoder_model_param_present != 0 {
                                    let opi: *mut Dav1dSequenceHeaderOperatingParameterInfo = &raw mut *((*hdr)
                                        .operating_parameter_info)
                                        .as_mut_ptr()
                                        .offset(i as isize)
                                        as *mut Dav1dSequenceHeaderOperatingParameterInfo;
                                    (*opi).decoder_buffer_delay = dav1d_get_bits(
                                        gb,
                                        (*hdr).encoder_decoder_buffer_delay_length
                                            as core::ffi::c_int,
                                    ) as uint32_t;
                                    (*opi).encoder_buffer_delay = dav1d_get_bits(
                                        gb,
                                        (*hdr).encoder_decoder_buffer_delay_length
                                            as core::ffi::c_int,
                                    ) as uint32_t;
                                    (*opi).low_delay_mode = dav1d_get_bit(gb) as uint8_t;
                                }
                            }
                            if (*hdr).display_model_info_present != 0 {
                                (*op).display_model_param_present = dav1d_get_bit(gb)
                                    as uint8_t;
                            }
                            (*op).initial_display_delay = (if (*op)
                                .display_model_param_present as core::ffi::c_int != 0
                            {
                                (dav1d_get_bits(gb, 4 as core::ffi::c_int))
                                    .wrapping_add(1 as core::ffi::c_uint)
                            } else {
                                10 as core::ffi::c_uint
                            }) as uint8_t;
                            i += 1;
                        }
                    }
                }
            }
            match current_block {
                1 => {}
                _ => {
                    (*hdr).width_n_bits = (dav1d_get_bits(gb, 4 as core::ffi::c_int))
                        .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                    (*hdr).height_n_bits = (dav1d_get_bits(gb, 4 as core::ffi::c_int))
                        .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                    (*hdr).max_width = (dav1d_get_bits(
                        gb,
                        (*hdr).width_n_bits as core::ffi::c_int,
                    ))
                        .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
                    (*hdr).max_height = (dav1d_get_bits(
                        gb,
                        (*hdr).height_n_bits as core::ffi::c_int,
                    ))
                        .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
                    if (*hdr).reduced_still_picture_header == 0 {
                        (*hdr).frame_id_numbers_present = dav1d_get_bit(gb) as uint8_t;
                        if (*hdr).frame_id_numbers_present != 0 {
                            (*hdr).delta_frame_id_n_bits = (dav1d_get_bits(
                                gb,
                                4 as core::ffi::c_int,
                            ))
                                .wrapping_add(2 as core::ffi::c_uint) as uint8_t;
                            (*hdr).frame_id_n_bits = (dav1d_get_bits(
                                gb,
                                3 as core::ffi::c_int,
                            ))
                                .wrapping_add(
                                    (*hdr).delta_frame_id_n_bits as core::ffi::c_uint,
                                )
                                .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                        }
                    }
                    (*hdr).sb128 = dav1d_get_bit(gb) as uint8_t;
                    (*hdr).filter_intra = dav1d_get_bit(gb) as uint8_t;
                    (*hdr).intra_edge_filter = dav1d_get_bit(gb) as uint8_t;
                    if (*hdr).reduced_still_picture_header != 0 {
                        (*hdr).screen_content_tools = DAV1D_ADAPTIVE;
                        (*hdr).force_integer_mv = DAV1D_ADAPTIVE;
                    } else {
                        (*hdr).inter_intra = dav1d_get_bit(gb) as uint8_t;
                        (*hdr).masked_compound = dav1d_get_bit(gb) as uint8_t;
                        (*hdr).warped_motion = dav1d_get_bit(gb) as uint8_t;
                        (*hdr).dual_filter = dav1d_get_bit(gb) as uint8_t;
                        (*hdr).order_hint = dav1d_get_bit(gb) as uint8_t;
                        if (*hdr).order_hint != 0 {
                            (*hdr).jnt_comp = dav1d_get_bit(gb) as uint8_t;
                            (*hdr).ref_frame_mvs = dav1d_get_bit(gb) as uint8_t;
                        }
                        (*hdr).screen_content_tools = (if dav1d_get_bit(gb) != 0 {
                            DAV1D_ADAPTIVE as core::ffi::c_int as core::ffi::c_uint
                        } else {
                            dav1d_get_bit(gb)
                        }) as Dav1dAdaptiveBoolean;
                        (*hdr).force_integer_mv = (if (*hdr).screen_content_tools
                            as core::ffi::c_uint != 0
                        {
                            if dav1d_get_bit(gb) != 0 {
                                DAV1D_ADAPTIVE as core::ffi::c_int as core::ffi::c_uint
                            } else {
                                dav1d_get_bit(gb)
                            }
                        } else {
                            2 as core::ffi::c_uint
                        }) as Dav1dAdaptiveBoolean;
                        if (*hdr).order_hint != 0 {
                            (*hdr).order_hint_n_bits = (dav1d_get_bits(
                                gb,
                                3 as core::ffi::c_int,
                            ))
                                .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                        }
                    }
                    (*hdr).super_res = dav1d_get_bit(gb) as uint8_t;
                    (*hdr).cdef = dav1d_get_bit(gb) as uint8_t;
                    (*hdr).restoration = dav1d_get_bit(gb) as uint8_t;
                    (*hdr).hbd = dav1d_get_bit(gb) as uint8_t;
                    if (*hdr).profile as core::ffi::c_int == 2 as core::ffi::c_int
                        && (*hdr).hbd as core::ffi::c_int != 0
                    {
                        (*hdr).hbd = ((*hdr).hbd as core::ffi::c_uint)
                            .wrapping_add(dav1d_get_bit(gb)) as uint8_t as uint8_t;
                    }
                    if (*hdr).profile as core::ffi::c_int != 1 as core::ffi::c_int {
                        (*hdr).monochrome = dav1d_get_bit(gb) as uint8_t;
                    }
                    (*hdr).color_description_present = dav1d_get_bit(gb) as uint8_t;
                    if (*hdr).color_description_present != 0 {
                        (*hdr).pri = dav1d_get_bits(gb, 8 as core::ffi::c_int)
                            as Dav1dColorPrimaries;
                        (*hdr).trc = dav1d_get_bits(gb, 8 as core::ffi::c_int)
                            as Dav1dTransferCharacteristics;
                        (*hdr).mtrx = dav1d_get_bits(gb, 8 as core::ffi::c_int)
                            as Dav1dMatrixCoefficients;
                    } else {
                        (*hdr).pri = DAV1D_COLOR_PRI_UNKNOWN;
                        (*hdr).trc = DAV1D_TRC_UNKNOWN;
                        (*hdr).mtrx = DAV1D_MC_UNKNOWN;
                    }
                    if (*hdr).monochrome != 0 {
                        (*hdr).color_range = dav1d_get_bit(gb) as uint8_t;
                        (*hdr).layout = DAV1D_PIXEL_LAYOUT_I400;
                        (*hdr).ss_ver = 1 as uint8_t;
                        (*hdr).ss_hor = (*hdr).ss_ver;
                        (*hdr).chr = DAV1D_CHR_UNKNOWN;
                        current_block = 4;
                    } else if (*hdr).pri as core::ffi::c_uint
                        == DAV1D_COLOR_PRI_BT709 as core::ffi::c_int as core::ffi::c_uint
                        && (*hdr).trc as core::ffi::c_uint
                            == DAV1D_TRC_SRGB as core::ffi::c_int as core::ffi::c_uint
                        && (*hdr).mtrx as core::ffi::c_uint
                            == DAV1D_MC_IDENTITY as core::ffi::c_int as core::ffi::c_uint
                    {
                        (*hdr).layout = DAV1D_PIXEL_LAYOUT_I444;
                        (*hdr).color_range = 1 as uint8_t;
                        if (*hdr).profile as core::ffi::c_int != 1 as core::ffi::c_int
                            && !((*hdr).profile as core::ffi::c_int
                                == 2 as core::ffi::c_int
                                && (*hdr).hbd as core::ffi::c_int == 2 as core::ffi::c_int)
                        {
                            current_block = 1;
                        } else {
                            current_block = 4;
                        }
                    } else {
                        (*hdr).color_range = dav1d_get_bit(gb) as uint8_t;
                        match (*hdr).profile as core::ffi::c_int {
                            0 => {
                                (*hdr).layout = DAV1D_PIXEL_LAYOUT_I420;
                                (*hdr).ss_ver = 1 as uint8_t;
                                (*hdr).ss_hor = (*hdr).ss_ver;
                            }
                            1 => {
                                (*hdr).layout = DAV1D_PIXEL_LAYOUT_I444;
                            }
                            2 => {
                                if (*hdr).hbd as core::ffi::c_int == 2 as core::ffi::c_int {
                                    (*hdr).ss_hor = dav1d_get_bit(gb) as uint8_t;
                                    if (*hdr).ss_hor != 0 {
                                        (*hdr).ss_ver = dav1d_get_bit(gb) as uint8_t;
                                    }
                                } else {
                                    (*hdr).ss_hor = 1 as uint8_t;
                                }
                                (*hdr).layout = (if (*hdr).ss_hor as core::ffi::c_int != 0 {
                                    if (*hdr).ss_ver as core::ffi::c_int != 0 {
                                        DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int
                                    } else {
                                        DAV1D_PIXEL_LAYOUT_I422 as core::ffi::c_int
                                    }
                                } else {
                                    DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
                                }) as Dav1dPixelLayout;
                            }
                            _ => {}
                        }
                        (*hdr).chr = (if (*hdr).ss_hor as core::ffi::c_int
                            & (*hdr).ss_ver as core::ffi::c_int != 0
                        {
                            dav1d_get_bits(gb, 2 as core::ffi::c_int)
                        } else {
                            DAV1D_CHR_UNKNOWN as core::ffi::c_int as core::ffi::c_uint
                        }) as Dav1dChromaSamplePosition;
                        current_block = 4;
                    }
                    match current_block {
                        1 => {}
                        _ => {
                            if !(strict_std_compliance != 0
                                && (*hdr).mtrx as core::ffi::c_uint
                                    == DAV1D_MC_IDENTITY as core::ffi::c_int
                                        as core::ffi::c_uint
                                && (*hdr).layout as core::ffi::c_uint
                                    != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
                                        as core::ffi::c_uint)
                            {
                                if (*hdr).monochrome == 0 {
                                    (*hdr).separate_uv_delta_q = dav1d_get_bit(gb) as uint8_t;
                                }
                                (*hdr).film_grain_present = dav1d_get_bit(gb) as uint8_t;
                                return check_trailing_bits(gb, strict_std_compliance);
                            }
                        }
                    }
                }
            }
        }
    }
    return -(22 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: *mut Dav1dSequenceHeader,
    ptr: *const uint8_t,
    sz: size_t,
) -> core::ffi::c_int {
    if out.is_null() {
        return -(22 as core::ffi::c_int);
    }
    if ptr.is_null() {
        return -(22 as core::ffi::c_int);
    }
    if !(sz > 0 as size_t
        && sz <= (18446744073709551615 as size_t).wrapping_div(2 as size_t))
    {
        return -(22 as core::ffi::c_int);
    }
    let mut gb: GetBits = GetBits {
        state: 0,
        bits_left: 0,
        error: 0,
        ptr: 0 as *const uint8_t,
        ptr_start: 0 as *const uint8_t,
        ptr_end: 0 as *const uint8_t,
    };
    dav1d_init_get_bits(&raw mut gb, ptr, sz);
    let mut res: core::ffi::c_int = -(2 as core::ffi::c_int);
    loop {
        dav1d_get_bit(&raw mut gb);
        let type_0: Dav1dObuType = dav1d_get_bits(&raw mut gb, 4 as core::ffi::c_int)
            as Dav1dObuType;
        let has_extension: core::ffi::c_int = dav1d_get_bit(&raw mut gb) as core::ffi::c_int;
        let has_length_field: core::ffi::c_int = dav1d_get_bit(&raw mut gb)
            as core::ffi::c_int;
        dav1d_get_bits(
            &raw mut gb,
            1 as core::ffi::c_int + 8 as core::ffi::c_int * has_extension,
        );
        let mut obu_end: *const uint8_t = gb.ptr_end;
        if has_length_field != 0 {
            let len: size_t = dav1d_get_uleb128(&raw mut gb) as size_t;
            if len > obu_end.offset_from(gb.ptr) as core::ffi::c_long as size_t {
                return -(22 as core::ffi::c_int);
            }
            obu_end = (gb.ptr).offset(len as isize);
        }
        if type_0 as core::ffi::c_uint
            == DAV1D_OBU_SEQ_HDR as core::ffi::c_int as core::ffi::c_uint
        {
            res = parse_seq_hdr(out, &raw mut gb, 0 as core::ffi::c_int);
            if res < 0 as core::ffi::c_int {
                return res;
            }
            if gb.ptr > obu_end {
                return -(22 as core::ffi::c_int);
            }
            dav1d_bytealign_get_bits(&raw mut gb);
        }
        if gb.error != 0 {
            return -(22 as core::ffi::c_int);
        }
        if !(gb.state == 0 as uint64_t && gb.bits_left == 0 as core::ffi::c_int) {
            unreachable!();
        }
        gb.ptr = obu_end;
        if !(gb.ptr < gb.ptr_end) {
            break;
        }
    }
    return res;
}
unsafe extern "C" fn read_frame_size(
    c: *mut Dav1dContext,
    gb: *mut GetBits,
    use_ref: core::ffi::c_int,
) -> core::ffi::c_int {
    let seqhdr: *const Dav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Dav1dFrameHeader = (*c).frame_hdr;
    if use_ref != 0 {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < 7 as core::ffi::c_int {
            if dav1d_get_bit(gb) != 0 {
                let ref_0: *const Dav1dThreadPicture = &raw mut (*((*c).refs)
                    .as_mut_ptr()
                    .offset(
                        *((*(*c).frame_hdr).refidx).as_mut_ptr().offset(i as isize)
                            as isize,
                    ))
                    .p;
                if ((*ref_0).p.frame_hdr).is_null() {
                    return -(1 as core::ffi::c_int);
                }
                (*hdr).width[1 as core::ffi::c_int as usize] = (*(*ref_0).p.frame_hdr)
                    .width[1 as core::ffi::c_int as usize];
                (*hdr).height = (*(*ref_0).p.frame_hdr).height;
                (*hdr).render_width = (*(*ref_0).p.frame_hdr).render_width;
                (*hdr).render_height = (*(*ref_0).p.frame_hdr).render_height;
                (*hdr).super_res.enabled = ((*seqhdr).super_res as core::ffi::c_int != 0
                    && dav1d_get_bit(gb) != 0) as core::ffi::c_int as uint8_t;
                if (*hdr).super_res.enabled != 0 {
                    (*hdr).super_res.width_scale_denominator = (9 as core::ffi::c_uint)
                        .wrapping_add(dav1d_get_bits(gb, 3 as core::ffi::c_int))
                        as uint8_t;
                    let d: core::ffi::c_int = (*hdr).super_res.width_scale_denominator
                        as core::ffi::c_int;
                    (*hdr).width[0 as core::ffi::c_int as usize] = imax(
                        ((*hdr).width[1 as core::ffi::c_int as usize]
                            * 8 as core::ffi::c_int + (d >> 1 as core::ffi::c_int)) / d,
                        imin(
                            16 as core::ffi::c_int,
                            (*hdr).width[1 as core::ffi::c_int as usize],
                        ),
                    );
                } else {
                    (*hdr).super_res.width_scale_denominator = 8 as uint8_t;
                    (*hdr).width[0 as core::ffi::c_int as usize] = (*hdr)
                        .width[1 as core::ffi::c_int as usize];
                }
                return 0 as core::ffi::c_int;
            }
            i += 1;
        }
    }
    if (*hdr).frame_size_override != 0 {
        (*hdr).width[1 as core::ffi::c_int as usize] = (dav1d_get_bits(
            gb,
            (*seqhdr).width_n_bits as core::ffi::c_int,
        ))
            .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
        (*hdr).height = (dav1d_get_bits(gb, (*seqhdr).height_n_bits as core::ffi::c_int))
            .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
    } else {
        (*hdr).width[1 as core::ffi::c_int as usize] = (*seqhdr).max_width;
        (*hdr).height = (*seqhdr).max_height;
    }
    (*hdr).super_res.enabled = ((*seqhdr).super_res as core::ffi::c_int != 0
        && dav1d_get_bit(gb) != 0) as core::ffi::c_int as uint8_t;
    if (*hdr).super_res.enabled != 0 {
        (*hdr).super_res.width_scale_denominator = (9 as core::ffi::c_uint)
            .wrapping_add(dav1d_get_bits(gb, 3 as core::ffi::c_int)) as uint8_t;
        let d_0: core::ffi::c_int = (*hdr).super_res.width_scale_denominator
            as core::ffi::c_int;
        (*hdr).width[0 as core::ffi::c_int as usize] = imax(
            ((*hdr).width[1 as core::ffi::c_int as usize] * 8 as core::ffi::c_int
                + (d_0 >> 1 as core::ffi::c_int)) / d_0,
            imin(16 as core::ffi::c_int, (*hdr).width[1 as core::ffi::c_int as usize]),
        );
    } else {
        (*hdr).super_res.width_scale_denominator = 8 as uint8_t;
        (*hdr).width[0 as core::ffi::c_int as usize] = (*hdr)
            .width[1 as core::ffi::c_int as usize];
    }
    (*hdr).have_render_size = dav1d_get_bit(gb) as uint8_t;
    if (*hdr).have_render_size != 0 {
        (*hdr).render_width = (dav1d_get_bits(gb, 16 as core::ffi::c_int))
            .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
        (*hdr).render_height = (dav1d_get_bits(gb, 16 as core::ffi::c_int))
            .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
    } else {
        (*hdr).render_width = (*hdr).width[1 as core::ffi::c_int as usize];
        (*hdr).render_height = (*hdr).height;
    }
    return 0 as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn tile_log2(
    sz: core::ffi::c_int,
    tgt: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut k: core::ffi::c_int = 0;
    k = 0 as core::ffi::c_int;
    while sz << k < tgt {
        k += 1;
    }
    return k;
}
static mut default_mode_ref_deltas: Dav1dLoopfilterModeRefDeltas = {
    let mut init = Dav1dLoopfilterModeRefDeltas {
        mode_delta: [0 as core::ffi::c_int as int8_t, 0 as core::ffi::c_int as int8_t],
        ref_delta: [
            1 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            0 as core::ffi::c_int as int8_t,
            -(1 as core::ffi::c_int) as int8_t,
            0 as core::ffi::c_int as int8_t,
            -(1 as core::ffi::c_int) as int8_t,
            -(1 as core::ffi::c_int) as int8_t,
        ],
    };
    init
};
unsafe extern "C" fn parse_frame_hdr(
    c: *mut Dav1dContext,
    gb: *mut GetBits,
) -> core::ffi::c_int {
    let mut sbsz_min1: core::ffi::c_int = 0;
    let mut sbsz_log2: core::ffi::c_int = 0;
    let mut sbw: core::ffi::c_int = 0;
    let mut sbh: core::ffi::c_int = 0;
    let mut max_tile_width_sb: core::ffi::c_int = 0;
    let mut max_tile_area_sb: core::ffi::c_int = 0;
    let mut min_log2_tiles: core::ffi::c_int = 0;
    let mut delta_lossless: core::ffi::c_int = 0;
    let mut current_block: u64;
    let seqhdr: *const Dav1dSequenceHeader = (*c).seq_hdr;
    let hdr: *mut Dav1dFrameHeader = (*c).frame_hdr;
    if (*seqhdr).reduced_still_picture_header == 0 {
        (*hdr).show_existing_frame = dav1d_get_bit(gb) as uint8_t;
    }
    if (*hdr).show_existing_frame != 0 {
        (*hdr).existing_frame_idx = dav1d_get_bits(gb, 3 as core::ffi::c_int) as uint8_t;
        if (*seqhdr).decoder_model_info_present as core::ffi::c_int != 0
            && (*seqhdr).equal_picture_interval == 0
        {
            (*hdr).frame_presentation_delay = dav1d_get_bits(
                gb,
                (*seqhdr).frame_presentation_delay_length as core::ffi::c_int,
            ) as uint32_t;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr).frame_id = dav1d_get_bits(
                gb,
                (*seqhdr).frame_id_n_bits as core::ffi::c_int,
            ) as uint32_t;
            let ref_frame_hdr: *mut Dav1dFrameHeader = (*c)
                .refs[(*hdr).existing_frame_idx as usize]
                .p
                .p
                .frame_hdr;
            if ref_frame_hdr.is_null() || (*ref_frame_hdr).frame_id != (*hdr).frame_id {
                current_block = 5;
            } else {
                current_block = 6;
            }
        } else {
            current_block = 6;
        }
        match current_block {
            5 => {}
            _ => return 0 as core::ffi::c_int,
        }
    } else {
        if (*seqhdr).reduced_still_picture_header != 0 {
            (*hdr).frame_type = DAV1D_FRAME_TYPE_KEY;
            (*hdr).show_frame = 1 as uint8_t;
        } else {
            (*hdr).frame_type = dav1d_get_bits(gb, 2 as core::ffi::c_int)
                as Dav1dFrameType;
            (*hdr).show_frame = dav1d_get_bit(gb) as uint8_t;
        }
        if (*hdr).show_frame != 0 {
            if (*seqhdr).decoder_model_info_present as core::ffi::c_int != 0
                && (*seqhdr).equal_picture_interval == 0
            {
                (*hdr).frame_presentation_delay = dav1d_get_bits(
                    gb,
                    (*seqhdr).frame_presentation_delay_length as core::ffi::c_int,
                ) as uint32_t;
            }
            (*hdr).showable_frame = ((*hdr).frame_type as core::ffi::c_uint
                != DAV1D_FRAME_TYPE_KEY as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int as uint8_t;
        } else {
            (*hdr).showable_frame = dav1d_get_bit(gb) as uint8_t;
        }
        (*hdr).error_resilient_mode = ((*hdr).frame_type as core::ffi::c_uint
            == DAV1D_FRAME_TYPE_KEY as core::ffi::c_int as core::ffi::c_uint
            && (*hdr).show_frame as core::ffi::c_int != 0
            || (*hdr).frame_type as core::ffi::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as core::ffi::c_int as core::ffi::c_uint
            || (*seqhdr).reduced_still_picture_header as core::ffi::c_int != 0
            || dav1d_get_bit(gb) != 0) as core::ffi::c_int as uint8_t;
        (*hdr).disable_cdf_update = dav1d_get_bit(gb) as uint8_t;
        (*hdr).allow_screen_content_tools = (if (*seqhdr).screen_content_tools
            as core::ffi::c_uint
            == DAV1D_ADAPTIVE as core::ffi::c_int as core::ffi::c_uint
        {
            dav1d_get_bit(gb)
        } else {
            (*seqhdr).screen_content_tools as core::ffi::c_uint
        }) as uint8_t;
        if (*hdr).allow_screen_content_tools != 0 {
            (*hdr).force_integer_mv = (if (*seqhdr).force_integer_mv as core::ffi::c_uint
                == DAV1D_ADAPTIVE as core::ffi::c_int as core::ffi::c_uint
            {
                dav1d_get_bit(gb)
            } else {
                (*seqhdr).force_integer_mv as core::ffi::c_uint
            }) as uint8_t;
        }
        if (*hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint == 0 {
            (*hdr).force_integer_mv = 1 as uint8_t;
        }
        if (*seqhdr).frame_id_numbers_present != 0 {
            (*hdr).frame_id = dav1d_get_bits(
                gb,
                (*seqhdr).frame_id_n_bits as core::ffi::c_int,
            ) as uint32_t;
        }
        if (*seqhdr).reduced_still_picture_header == 0 {
            (*hdr).frame_size_override = (if (*hdr).frame_type as core::ffi::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as core::ffi::c_int as core::ffi::c_uint
            {
                1 as core::ffi::c_uint
            } else {
                dav1d_get_bit(gb)
            }) as uint8_t;
        }
        if (*seqhdr).order_hint != 0 {
            (*hdr).frame_offset = dav1d_get_bits(
                gb,
                (*seqhdr).order_hint_n_bits as core::ffi::c_int,
            ) as uint8_t;
        }
        (*hdr).primary_ref_frame = (if (*hdr).error_resilient_mode == 0
            && (*hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint != 0
        {
            dav1d_get_bits(gb, 3 as core::ffi::c_int)
        } else {
            DAV1D_PRIMARY_REF_NONE as core::ffi::c_uint
        }) as uint8_t;
        if (*seqhdr).decoder_model_info_present != 0 {
            (*hdr).buffer_removal_time_present = dav1d_get_bit(gb) as uint8_t;
            if (*hdr).buffer_removal_time_present != 0 {
                let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                while i < (*(*c).seq_hdr).num_operating_points as core::ffi::c_int {
                    let seqop: *const Dav1dSequenceHeaderOperatingPoint = &*((*seqhdr)
                        .operating_points)
                        .as_ptr()
                        .offset(i as isize) as *const Dav1dSequenceHeaderOperatingPoint;
                    let op: *mut Dav1dFrameHeaderOperatingPoint = &raw mut *((*hdr)
                        .operating_points)
                        .as_mut_ptr()
                        .offset(i as isize) as *mut Dav1dFrameHeaderOperatingPoint;
                    if (*seqop).decoder_model_param_present != 0 {
                        let mut in_temporal_layer: core::ffi::c_int = (*seqop).idc
                            as core::ffi::c_int >> (*hdr).temporal_id as core::ffi::c_int
                            & 1 as core::ffi::c_int;
                        let mut in_spatial_layer: core::ffi::c_int = (*seqop).idc
                            as core::ffi::c_int
                            >> (*hdr).spatial_id as core::ffi::c_int
                                + 8 as core::ffi::c_int & 1 as core::ffi::c_int;
                        if (*seqop).idc == 0
                            || in_temporal_layer != 0 && in_spatial_layer != 0
                        {
                            (*op).buffer_removal_time = dav1d_get_bits(
                                gb,
                                (*seqhdr).buffer_removal_delay_length as core::ffi::c_int,
                            ) as uint32_t;
                        }
                    }
                    i += 1;
                }
            }
        }
        if (*hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint == 0 {
            (*hdr).refresh_frame_flags = (if (*hdr).frame_type as core::ffi::c_uint
                == DAV1D_FRAME_TYPE_KEY as core::ffi::c_int as core::ffi::c_uint
                && (*hdr).show_frame as core::ffi::c_int != 0
            {
                0xff as core::ffi::c_uint
            } else {
                dav1d_get_bits(gb, 8 as core::ffi::c_int)
            }) as uint8_t;
            if (*hdr).refresh_frame_flags as core::ffi::c_int != 0xff as core::ffi::c_int
                && (*hdr).error_resilient_mode as core::ffi::c_int != 0
                && (*seqhdr).order_hint as core::ffi::c_int != 0
            {
                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_0 < 8 as core::ffi::c_int {
                    dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits as core::ffi::c_int);
                    i_0 += 1;
                }
            }
            if (*c).strict_std_compliance != 0
                && (*hdr).frame_type as core::ffi::c_uint
                    == DAV1D_FRAME_TYPE_INTRA as core::ffi::c_int as core::ffi::c_uint
                && (*hdr).refresh_frame_flags as core::ffi::c_int
                    == 0xff as core::ffi::c_int
            {
                current_block = 5;
            } else if read_frame_size(c, gb, 0 as core::ffi::c_int)
                < 0 as core::ffi::c_int
            {
                current_block = 5;
            } else {
                if (*hdr).allow_screen_content_tools as core::ffi::c_int != 0
                    && (*hdr).super_res.enabled == 0
                {
                    (*hdr).allow_intrabc = dav1d_get_bit(gb) as uint8_t;
                }
                current_block = 7;
            }
        } else {
            (*hdr).refresh_frame_flags = (if (*hdr).frame_type as core::ffi::c_uint
                == DAV1D_FRAME_TYPE_SWITCH as core::ffi::c_int as core::ffi::c_uint
            {
                0xff as core::ffi::c_uint
            } else {
                dav1d_get_bits(gb, 8 as core::ffi::c_int)
            }) as uint8_t;
            if (*hdr).error_resilient_mode as core::ffi::c_int != 0
                && (*seqhdr).order_hint as core::ffi::c_int != 0
            {
                let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_1 < 8 as core::ffi::c_int {
                    dav1d_get_bits(gb, (*seqhdr).order_hint_n_bits as core::ffi::c_int);
                    i_1 += 1;
                }
            }
            if (*seqhdr).order_hint != 0 {
                (*hdr).frame_ref_short_signaling = dav1d_get_bit(gb) as uint8_t;
                if (*hdr).frame_ref_short_signaling != 0 {
                    (*hdr).refidx[0 as core::ffi::c_int as usize] = dav1d_get_bits(
                        gb,
                        3 as core::ffi::c_int,
                    ) as int8_t;
                    (*hdr).refidx[2 as core::ffi::c_int as usize] = -(1
                        as core::ffi::c_int) as int8_t;
                    (*hdr).refidx[1 as core::ffi::c_int as usize] = (*hdr)
                        .refidx[2 as core::ffi::c_int as usize];
                    (*hdr).refidx[3 as core::ffi::c_int as usize] = dav1d_get_bits(
                        gb,
                        3 as core::ffi::c_int,
                    ) as int8_t;
                    let mut frame_offset_mem: [core::ffi::c_int; 9] = [0; 9];
                    let frame_offset: *mut core::ffi::c_int = &raw mut *frame_offset_mem
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize) as *mut core::ffi::c_int;
                    let mut earliest_ref: core::ffi::c_int = -(1 as core::ffi::c_int);
                    let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut earliest_offset: core::ffi::c_int = INT_MAX;
                    loop {
                        if !(i_2 < 8 as core::ffi::c_int) {
                            current_block = 8;
                            break;
                        }
                        let refhdr: *const Dav1dFrameHeader = (*c)
                            .refs[i_2 as usize]
                            .p
                            .p
                            .frame_hdr;
                        if refhdr.is_null() {
                            current_block = 5;
                            break;
                        }
                        let diff: core::ffi::c_int = get_poc_diff(
                            (*seqhdr).order_hint_n_bits as core::ffi::c_int,
                            (*refhdr).frame_offset as core::ffi::c_int,
                            (*hdr).frame_offset as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        *frame_offset.offset(i_2 as isize) = diff;
                        if diff < earliest_offset {
                            earliest_offset = diff;
                            earliest_ref = i_2;
                        }
                        i_2 += 1;
                    }
                    match current_block {
                        5 => {}
                        _ => {
                            *frame_offset
                                .offset(
                                    (*hdr).refidx[0 as core::ffi::c_int as usize] as isize,
                                ) = INT_MIN;
                            *frame_offset
                                .offset(
                                    (*hdr).refidx[3 as core::ffi::c_int as usize] as isize,
                                ) = INT_MIN;
                            if !(earliest_ref >= 0 as core::ffi::c_int) {
                                unreachable!();
                            }
                            let mut refidx: core::ffi::c_int = -(1 as core::ffi::c_int);
                            let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
                            let mut latest_offset: core::ffi::c_int = 0
                                as core::ffi::c_int;
                            while i_3 < 8 as core::ffi::c_int {
                                let hint: core::ffi::c_int = *frame_offset
                                    .offset(i_3 as isize);
                                if hint >= latest_offset {
                                    latest_offset = hint;
                                    refidx = i_3;
                                }
                                i_3 += 1;
                            }
                            *frame_offset.offset(refidx as isize) = INT_MIN;
                            (*hdr).refidx[6 as core::ffi::c_int as usize] = refidx
                                as int8_t;
                            let mut i_4: core::ffi::c_int = 4 as core::ffi::c_int;
                            while i_4 < 6 as core::ffi::c_int {
                                let mut earliest_offset_0: core::ffi::c_uint = UINT8_MAX
                                    as core::ffi::c_uint;
                                refidx = -(1 as core::ffi::c_int);
                                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                                while j < 8 as core::ffi::c_int {
                                    let hint_0: core::ffi::c_uint = *frame_offset
                                        .offset(j as isize) as core::ffi::c_uint;
                                    if hint_0 < earliest_offset_0 {
                                        earliest_offset_0 = hint_0;
                                        refidx = j;
                                    }
                                    j += 1;
                                }
                                *frame_offset.offset(refidx as isize) = INT_MIN;
                                (*hdr).refidx[i_4 as usize] = refidx as int8_t;
                                i_4 += 1;
                            }
                            let mut i_5: core::ffi::c_int = 1 as core::ffi::c_int;
                            while i_5 < 7 as core::ffi::c_int {
                                refidx = (*hdr).refidx[i_5 as usize] as core::ffi::c_int;
                                if refidx < 0 as core::ffi::c_int {
                                    let mut latest_offset_0: core::ffi::c_uint = !UINT8_MAX
                                        as core::ffi::c_uint;
                                    let mut j_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                    while j_0 < 8 as core::ffi::c_int {
                                        let hint_1: core::ffi::c_uint = *frame_offset
                                            .offset(j_0 as isize) as core::ffi::c_uint;
                                        if hint_1 >= latest_offset_0 {
                                            latest_offset_0 = hint_1;
                                            refidx = j_0;
                                        }
                                        j_0 += 1;
                                    }
                                    *frame_offset.offset(refidx as isize) = INT_MIN;
                                    (*hdr).refidx[i_5 as usize] = (if refidx
                                        >= 0 as core::ffi::c_int
                                    {
                                        refidx
                                    } else {
                                        earliest_ref
                                    }) as int8_t;
                                }
                                i_5 += 1;
                            }
                            current_block = 9;
                        }
                    }
                } else {
                    current_block = 9;
                }
            } else {
                current_block = 9;
            }
            match current_block {
                5 => {}
                _ => {
                    let mut i_6: core::ffi::c_int = 0 as core::ffi::c_int;
                    loop {
                        if !(i_6 < 7 as core::ffi::c_int) {
                            current_block = 10;
                            break;
                        }
                        if (*hdr).frame_ref_short_signaling == 0 {
                            (*hdr).refidx[i_6 as usize] = dav1d_get_bits(
                                gb,
                                3 as core::ffi::c_int,
                            ) as int8_t;
                        }
                        if (*seqhdr).frame_id_numbers_present != 0 {
                            let delta_ref_frame_id: core::ffi::c_uint = (dav1d_get_bits(
                                gb,
                                (*seqhdr).delta_frame_id_n_bits as core::ffi::c_int,
                            ) as core::ffi::c_uint)
                                .wrapping_add(1 as core::ffi::c_uint);
                            let ref_frame_id: core::ffi::c_uint = ((*hdr).frame_id
                                as core::ffi::c_uint)
                                .wrapping_add(
                                    ((1 as core::ffi::c_int)
                                        << (*seqhdr).frame_id_n_bits as core::ffi::c_int)
                                        as core::ffi::c_uint,
                                )
                                .wrapping_sub(delta_ref_frame_id)
                                & (((1 as core::ffi::c_int)
                                    << (*seqhdr).frame_id_n_bits as core::ffi::c_int)
                                    - 1 as core::ffi::c_int) as core::ffi::c_uint;
                            let ref_frame_hdr_0: *mut Dav1dFrameHeader = (*c)
                                .refs[(*hdr).refidx[i_6 as usize] as usize]
                                .p
                                .p
                                .frame_hdr;
                            if ref_frame_hdr_0.is_null()
                                || (*ref_frame_hdr_0).frame_id != ref_frame_id as uint32_t
                            {
                                current_block = 5;
                                break;
                            }
                        }
                        i_6 += 1;
                    }
                    match current_block {
                        5 => {}
                        _ => {
                            let use_ref: core::ffi::c_int = ((*hdr).error_resilient_mode
                                == 0 && (*hdr).frame_size_override as core::ffi::c_int != 0)
                                as core::ffi::c_int;
                            if read_frame_size(c, gb, use_ref) < 0 as core::ffi::c_int {
                                current_block = 5;
                            } else {
                                if (*hdr).force_integer_mv == 0 {
                                    (*hdr).hp = dav1d_get_bit(gb) as uint8_t;
                                }
                                (*hdr).subpel_filter_mode = (if dav1d_get_bit(gb) != 0 {
                                    DAV1D_FILTER_SWITCHABLE as core::ffi::c_int
                                        as core::ffi::c_uint
                                } else {
                                    dav1d_get_bits(gb, 2 as core::ffi::c_int)
                                }) as Dav1dFilterMode;
                                (*hdr).switchable_motion_mode = dav1d_get_bit(gb)
                                    as uint8_t;
                                if (*hdr).error_resilient_mode == 0
                                    && (*seqhdr).ref_frame_mvs as core::ffi::c_int != 0
                                    && (*seqhdr).order_hint as core::ffi::c_int != 0
                                    && (*hdr).frame_type as core::ffi::c_uint
                                        & 1 as core::ffi::c_uint != 0
                                {
                                    (*hdr).use_ref_frame_mvs = dav1d_get_bit(gb) as uint8_t;
                                }
                                current_block = 7;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            5 => {}
            _ => {
                if (*seqhdr).reduced_still_picture_header == 0
                    && (*hdr).disable_cdf_update == 0
                {
                    (*hdr).refresh_context = (dav1d_get_bit(gb) == 0) as core::ffi::c_int
                        as uint8_t;
                }
                (*hdr).tiling.uniform = dav1d_get_bit(gb) as uint8_t;
                sbsz_min1 = ((64 as core::ffi::c_int)
                    << (*seqhdr).sb128 as core::ffi::c_int) - 1 as core::ffi::c_int;
                sbsz_log2 = 6 as core::ffi::c_int + (*seqhdr).sb128 as core::ffi::c_int;
                sbw = (*hdr).width[0 as core::ffi::c_int as usize] + sbsz_min1
                    >> sbsz_log2;
                sbh = (*hdr).height + sbsz_min1 >> sbsz_log2;
                max_tile_width_sb = 4096 as core::ffi::c_int >> sbsz_log2;
                max_tile_area_sb = 4096 as core::ffi::c_int * 2304 as core::ffi::c_int
                    >> 2 as core::ffi::c_int * sbsz_log2;
                (*hdr).tiling.min_log2_cols = tile_log2(max_tile_width_sb, sbw)
                    as uint8_t;
                (*hdr).tiling.max_log2_cols = tile_log2(
                    1 as core::ffi::c_int,
                    imin(sbw, DAV1D_MAX_TILE_COLS),
                ) as uint8_t;
                (*hdr).tiling.max_log2_rows = tile_log2(
                    1 as core::ffi::c_int,
                    imin(sbh, DAV1D_MAX_TILE_ROWS),
                ) as uint8_t;
                min_log2_tiles = imax(
                    tile_log2(max_tile_area_sb, sbw * sbh),
                    (*hdr).tiling.min_log2_cols as core::ffi::c_int,
                ) as core::ffi::c_int;
                if (*hdr).tiling.uniform != 0 {
                    (*hdr).tiling.log2_cols = (*hdr).tiling.min_log2_cols;
                    while ((*hdr).tiling.log2_cols as core::ffi::c_int)
                        < (*hdr).tiling.max_log2_cols as core::ffi::c_int
                        && dav1d_get_bit(gb) != 0
                    {
                        (*hdr).tiling.log2_cols = ((*hdr).tiling.log2_cols)
                            .wrapping_add(1);
                    }
                    let tile_w: core::ffi::c_int = 1 as core::ffi::c_int
                        + (sbw - 1 as core::ffi::c_int
                            >> (*hdr).tiling.log2_cols as core::ffi::c_int);
                    (*hdr).tiling.cols = 0 as uint8_t;
                    let mut sbx: core::ffi::c_int = 0 as core::ffi::c_int;
                    while sbx < sbw {
                        (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbx
                            as uint16_t;
                        sbx += tile_w;
                        (*hdr).tiling.cols = ((*hdr).tiling.cols).wrapping_add(1);
                    }
                    (*hdr).tiling.min_log2_rows = imax(
                        min_log2_tiles - (*hdr).tiling.log2_cols as core::ffi::c_int,
                        0 as core::ffi::c_int,
                    ) as uint8_t;
                    (*hdr).tiling.log2_rows = (*hdr).tiling.min_log2_rows;
                    while ((*hdr).tiling.log2_rows as core::ffi::c_int)
                        < (*hdr).tiling.max_log2_rows as core::ffi::c_int
                        && dav1d_get_bit(gb) != 0
                    {
                        (*hdr).tiling.log2_rows = ((*hdr).tiling.log2_rows)
                            .wrapping_add(1);
                    }
                    let tile_h: core::ffi::c_int = 1 as core::ffi::c_int
                        + (sbh - 1 as core::ffi::c_int
                            >> (*hdr).tiling.log2_rows as core::ffi::c_int);
                    (*hdr).tiling.rows = 0 as uint8_t;
                    let mut sby: core::ffi::c_int = 0 as core::ffi::c_int;
                    while sby < sbh {
                        (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sby
                            as uint16_t;
                        sby += tile_h;
                        (*hdr).tiling.rows = ((*hdr).tiling.rows).wrapping_add(1);
                    }
                } else {
                    (*hdr).tiling.cols = 0 as uint8_t;
                    let mut widest_tile: core::ffi::c_int = 0 as core::ffi::c_int;
                    let mut max_tile_area_sb_0: core::ffi::c_int = sbw * sbh;
                    let mut sbx_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while sbx_0 < sbw
                        && ((*hdr).tiling.cols as core::ffi::c_int) < DAV1D_MAX_TILE_COLS
                    {
                        let tile_width_sb: core::ffi::c_int = imin(
                            sbw - sbx_0,
                            max_tile_width_sb,
                        ) as core::ffi::c_int;
                        let tile_w_0: core::ffi::c_int = (if tile_width_sb
                            > 1 as core::ffi::c_int
                        {
                            (1 as core::ffi::c_uint)
                                .wrapping_add(
                                    dav1d_get_uniform(gb, tile_width_sb as core::ffi::c_uint),
                                )
                        } else {
                            1 as core::ffi::c_uint
                        }) as core::ffi::c_int;
                        (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbx_0
                            as uint16_t;
                        sbx_0 += tile_w_0;
                        widest_tile = imax(widest_tile, tile_w_0);
                        (*hdr).tiling.cols = ((*hdr).tiling.cols).wrapping_add(1);
                    }
                    (*hdr).tiling.log2_cols = tile_log2(
                        1 as core::ffi::c_int,
                        (*hdr).tiling.cols as core::ffi::c_int,
                    ) as uint8_t;
                    if min_log2_tiles != 0 {
                        max_tile_area_sb_0 >>= min_log2_tiles + 1 as core::ffi::c_int;
                    }
                    let max_tile_height_sb: core::ffi::c_int = imax(
                        max_tile_area_sb_0 / widest_tile,
                        1 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    (*hdr).tiling.rows = 0 as uint8_t;
                    let mut sby_0: core::ffi::c_int = 0 as core::ffi::c_int;
                    while sby_0 < sbh
                        && ((*hdr).tiling.rows as core::ffi::c_int) < DAV1D_MAX_TILE_ROWS
                    {
                        let tile_height_sb: core::ffi::c_int = imin(
                            sbh - sby_0,
                            max_tile_height_sb,
                        ) as core::ffi::c_int;
                        let tile_h_0: core::ffi::c_int = (if tile_height_sb
                            > 1 as core::ffi::c_int
                        {
                            (1 as core::ffi::c_uint)
                                .wrapping_add(
                                    dav1d_get_uniform(gb, tile_height_sb as core::ffi::c_uint),
                                )
                        } else {
                            1 as core::ffi::c_uint
                        }) as core::ffi::c_int;
                        (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sby_0
                            as uint16_t;
                        sby_0 += tile_h_0;
                        (*hdr).tiling.rows = ((*hdr).tiling.rows).wrapping_add(1);
                    }
                    (*hdr).tiling.log2_rows = tile_log2(
                        1 as core::ffi::c_int,
                        (*hdr).tiling.rows as core::ffi::c_int,
                    ) as uint8_t;
                }
                (*hdr).tiling.col_start_sb[(*hdr).tiling.cols as usize] = sbw
                    as uint16_t;
                (*hdr).tiling.row_start_sb[(*hdr).tiling.rows as usize] = sbh
                    as uint16_t;
                if (*hdr).tiling.log2_cols as core::ffi::c_int != 0
                    || (*hdr).tiling.log2_rows as core::ffi::c_int != 0
                {
                    (*hdr).tiling.update = dav1d_get_bits(
                        gb,
                        (*hdr).tiling.log2_cols as core::ffi::c_int
                            + (*hdr).tiling.log2_rows as core::ffi::c_int,
                    ) as uint16_t;
                    if (*hdr).tiling.update as core::ffi::c_int
                        >= (*hdr).tiling.cols as core::ffi::c_int
                            * (*hdr).tiling.rows as core::ffi::c_int
                    {
                        current_block = 5;
                    } else {
                        (*hdr).tiling.n_bytes = (dav1d_get_bits(
                            gb,
                            2 as core::ffi::c_int,
                        ))
                            .wrapping_add(1 as core::ffi::c_uint) as uint8_t;
                        current_block = 11;
                    }
                } else {
                    current_block = 11;
                }
                match current_block {
                    5 => {}
                    _ => {
                        (*hdr).quant.yac = dav1d_get_bits(gb, 8 as core::ffi::c_int)
                            as uint8_t;
                        if dav1d_get_bit(gb) != 0 {
                            (*hdr).quant.ydc_delta = dav1d_get_sbits(
                                gb,
                                7 as core::ffi::c_int,
                            ) as int8_t;
                        }
                        if (*seqhdr).monochrome == 0 {
                            let diff_uv_delta: core::ffi::c_int = (if (*seqhdr)
                                .separate_uv_delta_q as core::ffi::c_int != 0
                            {
                                dav1d_get_bit(gb)
                            } else {
                                0 as core::ffi::c_uint
                            }) as core::ffi::c_int;
                            if dav1d_get_bit(gb) != 0 {
                                (*hdr).quant.udc_delta = dav1d_get_sbits(
                                    gb,
                                    7 as core::ffi::c_int,
                                ) as int8_t;
                            }
                            if dav1d_get_bit(gb) != 0 {
                                (*hdr).quant.uac_delta = dav1d_get_sbits(
                                    gb,
                                    7 as core::ffi::c_int,
                                ) as int8_t;
                            }
                            if diff_uv_delta != 0 {
                                if dav1d_get_bit(gb) != 0 {
                                    (*hdr).quant.vdc_delta = dav1d_get_sbits(
                                        gb,
                                        7 as core::ffi::c_int,
                                    ) as int8_t;
                                }
                                if dav1d_get_bit(gb) != 0 {
                                    (*hdr).quant.vac_delta = dav1d_get_sbits(
                                        gb,
                                        7 as core::ffi::c_int,
                                    ) as int8_t;
                                }
                            } else {
                                (*hdr).quant.vdc_delta = (*hdr).quant.udc_delta;
                                (*hdr).quant.vac_delta = (*hdr).quant.uac_delta;
                            }
                        }
                        (*hdr).quant.qm = dav1d_get_bit(gb) as uint8_t;
                        if (*hdr).quant.qm != 0 {
                            (*hdr).quant.qm_y = dav1d_get_bits(gb, 4 as core::ffi::c_int)
                                as uint8_t;
                            (*hdr).quant.qm_u = dav1d_get_bits(gb, 4 as core::ffi::c_int)
                                as uint8_t;
                            (*hdr).quant.qm_v = (if (*seqhdr).separate_uv_delta_q
                                as core::ffi::c_int != 0
                            {
                                dav1d_get_bits(gb, 4 as core::ffi::c_int)
                            } else {
                                (*hdr).quant.qm_u as core::ffi::c_uint
                            }) as uint8_t;
                        }
                        (*hdr).segmentation.enabled = dav1d_get_bit(gb) as uint8_t;
                        if (*hdr).segmentation.enabled != 0 {
                            if (*hdr).primary_ref_frame as core::ffi::c_int
                                == DAV1D_PRIMARY_REF_NONE
                            {
                                (*hdr).segmentation.update_map = 1 as uint8_t;
                                (*hdr).segmentation.update_data = 1 as uint8_t;
                            } else {
                                (*hdr).segmentation.update_map = dav1d_get_bit(gb)
                                    as uint8_t;
                                if (*hdr).segmentation.update_map != 0 {
                                    (*hdr).segmentation.temporal = dav1d_get_bit(gb) as uint8_t;
                                }
                                (*hdr).segmentation.update_data = dav1d_get_bit(gb)
                                    as uint8_t;
                            }
                            if (*hdr).segmentation.update_data != 0 {
                                (*hdr).segmentation.seg_data.last_active_segid = -(1
                                    as core::ffi::c_int) as int8_t;
                                let mut i_7: core::ffi::c_int = 0 as core::ffi::c_int;
                                while i_7 < DAV1D_MAX_SEGMENTS {
                                    let seg: *mut Dav1dSegmentationData = &raw mut *((*hdr)
                                        .segmentation
                                        .seg_data
                                        .d)
                                        .as_mut_ptr()
                                        .offset(i_7 as isize) as *mut Dav1dSegmentationData;
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_q = dav1d_get_sbits(gb, 9 as core::ffi::c_int)
                                            as int16_t;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_y_v = dav1d_get_sbits(
                                            gb,
                                            7 as core::ffi::c_int,
                                        ) as int8_t;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_y_h = dav1d_get_sbits(
                                            gb,
                                            7 as core::ffi::c_int,
                                        ) as int8_t;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_u = dav1d_get_sbits(
                                            gb,
                                            7 as core::ffi::c_int,
                                        ) as int8_t;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).delta_lf_v = dav1d_get_sbits(
                                            gb,
                                            7 as core::ffi::c_int,
                                        ) as int8_t;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                    }
                                    if dav1d_get_bit(gb) != 0 {
                                        (*seg).ref_0 = dav1d_get_bits(gb, 3 as core::ffi::c_int)
                                            as int8_t;
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                        (*hdr).segmentation.seg_data.preskip = 1 as uint8_t;
                                    } else {
                                        (*seg).ref_0 = -(1 as core::ffi::c_int) as int8_t;
                                    }
                                    (*seg).skip = dav1d_get_bit(gb) as uint8_t;
                                    if (*seg).skip != 0 {
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                        (*hdr).segmentation.seg_data.preskip = 1 as uint8_t;
                                    }
                                    (*seg).globalmv = dav1d_get_bit(gb) as uint8_t;
                                    if (*seg).globalmv != 0 {
                                        (*hdr).segmentation.seg_data.last_active_segid = i_7
                                            as int8_t;
                                        (*hdr).segmentation.seg_data.preskip = 1 as uint8_t;
                                    }
                                    i_7 += 1;
                                }
                                current_block = 12;
                            } else {
                                if !((*hdr).primary_ref_frame as core::ffi::c_int
                                    != 7 as core::ffi::c_int)
                                {
                                    unreachable!();
                                }
                                let pri_ref: core::ffi::c_int = (*hdr)
                                    .refidx[(*hdr).primary_ref_frame as usize]
                                    as core::ffi::c_int;
                                if ((*c).refs[pri_ref as usize].p.p.frame_hdr).is_null() {
                                    current_block = 5;
                                } else {
                                    (*hdr).segmentation.seg_data = (*(*c)
                                        .refs[pri_ref as usize]
                                        .p
                                        .p
                                        .frame_hdr)
                                        .segmentation
                                        .seg_data;
                                    current_block = 12;
                                }
                            }
                        } else {
                            let mut i_8: core::ffi::c_int = 0 as core::ffi::c_int;
                            while i_8 < DAV1D_MAX_SEGMENTS {
                                (*hdr).segmentation.seg_data.d[i_8 as usize].ref_0 = -(1
                                    as core::ffi::c_int) as int8_t;
                                i_8 += 1;
                            }
                            current_block = 12;
                        }
                        match current_block {
                            5 => {}
                            _ => {
                                if (*hdr).quant.yac != 0 {
                                    (*hdr).delta.q.present = dav1d_get_bit(gb) as uint8_t;
                                    if (*hdr).delta.q.present != 0 {
                                        (*hdr).delta.q.res_log2 = dav1d_get_bits(
                                            gb,
                                            2 as core::ffi::c_int,
                                        ) as uint8_t;
                                        if (*hdr).allow_intrabc == 0 {
                                            (*hdr).delta.lf.present = dav1d_get_bit(gb) as uint8_t;
                                            if (*hdr).delta.lf.present != 0 {
                                                (*hdr).delta.lf.res_log2 = dav1d_get_bits(
                                                    gb,
                                                    2 as core::ffi::c_int,
                                                ) as uint8_t;
                                                (*hdr).delta.lf.multi = dav1d_get_bit(gb) as uint8_t;
                                            }
                                        }
                                    }
                                }
                                delta_lossless = ((*hdr).quant.ydc_delta == 0
                                    && (*hdr).quant.udc_delta == 0
                                    && (*hdr).quant.uac_delta == 0
                                    && (*hdr).quant.vdc_delta == 0
                                    && (*hdr).quant.vac_delta == 0) as core::ffi::c_int;
                                (*hdr).all_lossless = 1 as uint8_t;
                                let mut i_9: core::ffi::c_int = 0 as core::ffi::c_int;
                                while i_9 < DAV1D_MAX_SEGMENTS {
                                    (*hdr).segmentation.qidx[i_9 as usize] = (if (*hdr)
                                        .segmentation
                                        .enabled as core::ffi::c_int != 0
                                    {
                                        iclip_u8(
                                            (*hdr).quant.yac as core::ffi::c_int
                                                + (*hdr).segmentation.seg_data.d[i_9 as usize].delta_q
                                                    as core::ffi::c_int,
                                        )
                                    } else {
                                        (*hdr).quant.yac as core::ffi::c_int
                                    }) as uint8_t;
                                    (*hdr).segmentation.lossless[i_9 as usize] = ((*hdr)
                                        .segmentation
                                        .qidx[i_9 as usize] == 0 && delta_lossless != 0)
                                        as core::ffi::c_int as uint8_t;
                                    (*hdr).all_lossless = ((*hdr).all_lossless
                                        as core::ffi::c_int
                                        & (*hdr).segmentation.lossless[i_9 as usize]
                                            as core::ffi::c_int) as uint8_t;
                                    i_9 += 1;
                                }
                                if (*hdr).all_lossless as core::ffi::c_int != 0
                                    || (*hdr).allow_intrabc as core::ffi::c_int != 0
                                {
                                    (*hdr).loopfilter.mode_ref_delta_enabled = 1 as uint8_t;
                                    (*hdr).loopfilter.mode_ref_delta_update = 1 as uint8_t;
                                    (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas;
                                    current_block = 13;
                                } else {
                                    (*hdr).loopfilter.level_y[0 as core::ffi::c_int as usize] = dav1d_get_bits(
                                        gb,
                                        6 as core::ffi::c_int,
                                    ) as uint8_t;
                                    (*hdr).loopfilter.level_y[1 as core::ffi::c_int as usize] = dav1d_get_bits(
                                        gb,
                                        6 as core::ffi::c_int,
                                    ) as uint8_t;
                                    if (*seqhdr).monochrome == 0
                                        && ((*hdr)
                                            .loopfilter
                                            .level_y[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                            != 0
                                            || (*hdr).loopfilter.level_y[1 as core::ffi::c_int as usize]
                                                as core::ffi::c_int != 0)
                                    {
                                        (*hdr).loopfilter.level_u = dav1d_get_bits(
                                            gb,
                                            6 as core::ffi::c_int,
                                        ) as uint8_t;
                                        (*hdr).loopfilter.level_v = dav1d_get_bits(
                                            gb,
                                            6 as core::ffi::c_int,
                                        ) as uint8_t;
                                    }
                                    (*hdr).loopfilter.sharpness = dav1d_get_bits(
                                        gb,
                                        3 as core::ffi::c_int,
                                    ) as uint8_t;
                                    if (*hdr).primary_ref_frame as core::ffi::c_int
                                        == DAV1D_PRIMARY_REF_NONE
                                    {
                                        (*hdr).loopfilter.mode_ref_deltas = default_mode_ref_deltas;
                                        current_block = 14;
                                    } else {
                                        let ref_0: core::ffi::c_int = (*hdr)
                                            .refidx[(*hdr).primary_ref_frame as usize]
                                            as core::ffi::c_int;
                                        if ((*c).refs[ref_0 as usize].p.p.frame_hdr).is_null() {
                                            current_block = 5;
                                        } else {
                                            (*hdr).loopfilter.mode_ref_deltas = (*(*c)
                                                .refs[ref_0 as usize]
                                                .p
                                                .p
                                                .frame_hdr)
                                                .loopfilter
                                                .mode_ref_deltas;
                                            current_block = 14;
                                        }
                                    }
                                    match current_block {
                                        5 => {}
                                        _ => {
                                            (*hdr).loopfilter.mode_ref_delta_enabled = dav1d_get_bit(gb)
                                                as uint8_t;
                                            if (*hdr).loopfilter.mode_ref_delta_enabled != 0 {
                                                (*hdr).loopfilter.mode_ref_delta_update = dav1d_get_bit(gb)
                                                    as uint8_t;
                                                if (*hdr).loopfilter.mode_ref_delta_update != 0 {
                                                    let mut i_10: core::ffi::c_int = 0 as core::ffi::c_int;
                                                    while i_10 < 8 as core::ffi::c_int {
                                                        if dav1d_get_bit(gb) != 0 {
                                                            (*hdr)
                                                                .loopfilter
                                                                .mode_ref_deltas
                                                                .ref_delta[i_10 as usize] = dav1d_get_sbits(
                                                                gb,
                                                                7 as core::ffi::c_int,
                                                            ) as int8_t;
                                                        }
                                                        i_10 += 1;
                                                    }
                                                    let mut i_11: core::ffi::c_int = 0 as core::ffi::c_int;
                                                    while i_11 < 2 as core::ffi::c_int {
                                                        if dav1d_get_bit(gb) != 0 {
                                                            (*hdr)
                                                                .loopfilter
                                                                .mode_ref_deltas
                                                                .mode_delta[i_11 as usize] = dav1d_get_sbits(
                                                                gb,
                                                                7 as core::ffi::c_int,
                                                            ) as int8_t;
                                                        }
                                                        i_11 += 1;
                                                    }
                                                }
                                            }
                                            current_block = 13;
                                        }
                                    }
                                }
                                match current_block {
                                    5 => {}
                                    _ => {
                                        if (*hdr).all_lossless == 0
                                            && (*seqhdr).cdef as core::ffi::c_int != 0
                                            && (*hdr).allow_intrabc == 0
                                        {
                                            (*hdr).cdef.damping = (dav1d_get_bits(
                                                gb,
                                                2 as core::ffi::c_int,
                                            ))
                                                .wrapping_add(3 as core::ffi::c_uint) as uint8_t;
                                            (*hdr).cdef.n_bits = dav1d_get_bits(
                                                gb,
                                                2 as core::ffi::c_int,
                                            ) as uint8_t;
                                            let mut i_12: core::ffi::c_int = 0 as core::ffi::c_int;
                                            while i_12
                                                < (1 as core::ffi::c_int)
                                                    << (*hdr).cdef.n_bits as core::ffi::c_int
                                            {
                                                (*hdr).cdef.y_strength[i_12 as usize] = dav1d_get_bits(
                                                    gb,
                                                    6 as core::ffi::c_int,
                                                ) as uint8_t;
                                                if (*seqhdr).monochrome == 0 {
                                                    (*hdr).cdef.uv_strength[i_12 as usize] = dav1d_get_bits(
                                                        gb,
                                                        6 as core::ffi::c_int,
                                                    ) as uint8_t;
                                                }
                                                i_12 += 1;
                                            }
                                        }
                                        if ((*hdr).all_lossless == 0
                                            || (*hdr).super_res.enabled as core::ffi::c_int != 0)
                                            && (*seqhdr).restoration as core::ffi::c_int != 0
                                            && (*hdr).allow_intrabc == 0
                                        {
                                            (*hdr).restoration.type_0[0 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                gb,
                                                2 as core::ffi::c_int,
                                            ) as Dav1dRestorationType;
                                            if (*seqhdr).monochrome == 0 {
                                                (*hdr).restoration.type_0[1 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                    gb,
                                                    2 as core::ffi::c_int,
                                                ) as Dav1dRestorationType;
                                                (*hdr).restoration.type_0[2 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                    gb,
                                                    2 as core::ffi::c_int,
                                                ) as Dav1dRestorationType;
                                            }
                                            if (*hdr).restoration.type_0[0 as core::ffi::c_int as usize]
                                                as core::ffi::c_uint != 0
                                                || (*hdr).restoration.type_0[1 as core::ffi::c_int as usize]
                                                    as core::ffi::c_uint != 0
                                                || (*hdr).restoration.type_0[2 as core::ffi::c_int as usize]
                                                    as core::ffi::c_uint != 0
                                            {
                                                (*hdr)
                                                    .restoration
                                                    .unit_size[0 as core::ffi::c_int as usize] = (6
                                                    as core::ffi::c_int + (*seqhdr).sb128 as core::ffi::c_int)
                                                    as uint8_t;
                                                if dav1d_get_bit(gb) != 0 {
                                                    (*hdr)
                                                        .restoration
                                                        .unit_size[0 as core::ffi::c_int as usize] = ((*hdr)
                                                        .restoration
                                                        .unit_size[0 as core::ffi::c_int as usize])
                                                        .wrapping_add(1);
                                                    if (*seqhdr).sb128 == 0 {
                                                        (*hdr)
                                                            .restoration
                                                            .unit_size[0 as core::ffi::c_int as usize] = ((*hdr)
                                                            .restoration
                                                            .unit_size[0 as core::ffi::c_int as usize]
                                                            as core::ffi::c_uint)
                                                            .wrapping_add(dav1d_get_bit(gb)) as uint8_t as uint8_t;
                                                    }
                                                }
                                                (*hdr)
                                                    .restoration
                                                    .unit_size[1 as core::ffi::c_int as usize] = (*hdr)
                                                    .restoration
                                                    .unit_size[0 as core::ffi::c_int as usize];
                                                if ((*hdr)
                                                    .restoration
                                                    .type_0[1 as core::ffi::c_int as usize] as core::ffi::c_uint
                                                    != 0
                                                    || (*hdr).restoration.type_0[2 as core::ffi::c_int as usize]
                                                        as core::ffi::c_uint != 0)
                                                    && (*seqhdr).ss_hor as core::ffi::c_int
                                                        == 1 as core::ffi::c_int
                                                    && (*seqhdr).ss_ver as core::ffi::c_int
                                                        == 1 as core::ffi::c_int
                                                {
                                                    (*hdr)
                                                        .restoration
                                                        .unit_size[1 as core::ffi::c_int as usize] = ((*hdr)
                                                        .restoration
                                                        .unit_size[1 as core::ffi::c_int as usize]
                                                        as core::ffi::c_uint)
                                                        .wrapping_sub(dav1d_get_bit(gb)) as uint8_t as uint8_t;
                                                }
                                            } else {
                                                (*hdr)
                                                    .restoration
                                                    .unit_size[0 as core::ffi::c_int as usize] = 8 as uint8_t;
                                            }
                                        }
                                        if (*hdr).all_lossless == 0 {
                                            (*hdr).txfm_mode = (if dav1d_get_bit(gb) != 0 {
                                                DAV1D_TX_SWITCHABLE as core::ffi::c_int
                                            } else {
                                                DAV1D_TX_LARGEST as core::ffi::c_int
                                            }) as Dav1dTxfmMode;
                                        }
                                        if (*hdr).frame_type as core::ffi::c_uint
                                            & 1 as core::ffi::c_uint != 0
                                        {
                                            (*hdr).switchable_comp_refs = dav1d_get_bit(gb) as uint8_t;
                                        }
                                        if (*hdr).switchable_comp_refs as core::ffi::c_int != 0
                                            && (*hdr).frame_type as core::ffi::c_uint
                                                & 1 as core::ffi::c_uint != 0
                                            && (*seqhdr).order_hint as core::ffi::c_int != 0
                                        {
                                            let poc: core::ffi::c_int = (*hdr).frame_offset
                                                as core::ffi::c_int;
                                            let mut off_before: core::ffi::c_int = -(1
                                                as core::ffi::c_int);
                                            let mut off_after: core::ffi::c_int = -(1
                                                as core::ffi::c_int);
                                            let mut off_before_idx: core::ffi::c_int = 0;
                                            let mut off_after_idx: core::ffi::c_int = 0;
                                            let mut i_13: core::ffi::c_int = 0 as core::ffi::c_int;
                                            loop {
                                                if !(i_13 < 7 as core::ffi::c_int) {
                                                    current_block = 15;
                                                    break;
                                                }
                                                if ((*c)
                                                    .refs[(*hdr).refidx[i_13 as usize] as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .is_null()
                                                {
                                                    current_block = 5;
                                                    break;
                                                }
                                                let refpoc: core::ffi::c_int = (*(*c)
                                                    .refs[(*hdr).refidx[i_13 as usize] as usize]
                                                    .p
                                                    .p
                                                    .frame_hdr)
                                                    .frame_offset as core::ffi::c_int;
                                                let diff_0: core::ffi::c_int = get_poc_diff(
                                                    (*seqhdr).order_hint_n_bits as core::ffi::c_int,
                                                    refpoc,
                                                    poc,
                                                ) as core::ffi::c_int;
                                                if diff_0 > 0 as core::ffi::c_int {
                                                    if off_after < 0 as core::ffi::c_int
                                                        || get_poc_diff(
                                                            (*seqhdr).order_hint_n_bits as core::ffi::c_int,
                                                            off_after,
                                                            refpoc,
                                                        ) > 0 as core::ffi::c_int
                                                    {
                                                        off_after = refpoc;
                                                        off_after_idx = i_13;
                                                    }
                                                } else if diff_0 < 0 as core::ffi::c_int
                                                    && (off_before < 0 as core::ffi::c_int
                                                        || get_poc_diff(
                                                            (*seqhdr).order_hint_n_bits as core::ffi::c_int,
                                                            refpoc,
                                                            off_before,
                                                        ) > 0 as core::ffi::c_int)
                                                {
                                                    off_before = refpoc;
                                                    off_before_idx = i_13;
                                                }
                                                i_13 += 1;
                                            }
                                            match current_block {
                                                5 => {}
                                                _ => {
                                                    if off_before | off_after >= 0 as core::ffi::c_int {
                                                        (*hdr).skip_mode_refs[0 as core::ffi::c_int as usize] = imin(
                                                            off_before_idx,
                                                            off_after_idx,
                                                        ) as int8_t;
                                                        (*hdr).skip_mode_refs[1 as core::ffi::c_int as usize] = imax(
                                                            off_before_idx,
                                                            off_after_idx,
                                                        ) as int8_t;
                                                        (*hdr).skip_mode_allowed = 1 as uint8_t;
                                                        current_block = 16;
                                                    } else if off_before >= 0 as core::ffi::c_int {
                                                        let mut off_before2: core::ffi::c_int = -(1
                                                            as core::ffi::c_int);
                                                        let mut off_before2_idx: core::ffi::c_int = 0;
                                                        let mut i_14: core::ffi::c_int = 0 as core::ffi::c_int;
                                                        loop {
                                                            if !(i_14 < 7 as core::ffi::c_int) {
                                                                current_block = 17;
                                                                break;
                                                            }
                                                            if ((*c)
                                                                .refs[(*hdr).refidx[i_14 as usize] as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .is_null()
                                                            {
                                                                current_block = 5;
                                                                break;
                                                            }
                                                            let refpoc_0: core::ffi::c_int = (*(*c)
                                                                .refs[(*hdr).refidx[i_14 as usize] as usize]
                                                                .p
                                                                .p
                                                                .frame_hdr)
                                                                .frame_offset as core::ffi::c_int;
                                                            if get_poc_diff(
                                                                (*seqhdr).order_hint_n_bits as core::ffi::c_int,
                                                                refpoc_0,
                                                                off_before,
                                                            ) < 0 as core::ffi::c_int
                                                            {
                                                                if off_before2 < 0 as core::ffi::c_int
                                                                    || get_poc_diff(
                                                                        (*seqhdr).order_hint_n_bits as core::ffi::c_int,
                                                                        refpoc_0,
                                                                        off_before2,
                                                                    ) > 0 as core::ffi::c_int
                                                                {
                                                                    off_before2 = refpoc_0;
                                                                    off_before2_idx = i_14;
                                                                }
                                                            }
                                                            i_14 += 1;
                                                        }
                                                        match current_block {
                                                            5 => {}
                                                            _ => {
                                                                if off_before2 >= 0 as core::ffi::c_int {
                                                                    (*hdr).skip_mode_refs[0 as core::ffi::c_int as usize] = imin(
                                                                        off_before_idx,
                                                                        off_before2_idx,
                                                                    ) as int8_t;
                                                                    (*hdr).skip_mode_refs[1 as core::ffi::c_int as usize] = imax(
                                                                        off_before_idx,
                                                                        off_before2_idx,
                                                                    ) as int8_t;
                                                                    (*hdr).skip_mode_allowed = 1 as uint8_t;
                                                                }
                                                                current_block = 16;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 16;
                                                    }
                                                }
                                            }
                                        } else {
                                            current_block = 16;
                                        }
                                        match current_block {
                                            5 => {}
                                            _ => {
                                                if (*hdr).skip_mode_allowed != 0 {
                                                    (*hdr).skip_mode_enabled = dav1d_get_bit(gb) as uint8_t;
                                                }
                                                if (*hdr).error_resilient_mode == 0
                                                    && (*hdr).frame_type as core::ffi::c_uint
                                                        & 1 as core::ffi::c_uint != 0
                                                    && (*seqhdr).warped_motion as core::ffi::c_int != 0
                                                {
                                                    (*hdr).warp_motion = dav1d_get_bit(gb) as uint8_t;
                                                }
                                                (*hdr).reduced_txtp_set = dav1d_get_bit(gb) as uint8_t;
                                                let mut i_15: core::ffi::c_int = 0 as core::ffi::c_int;
                                                while i_15 < 7 as core::ffi::c_int {
                                                    (*hdr).gmv[i_15 as usize] = dav1d_default_wm_params;
                                                    i_15 += 1;
                                                }
                                                if (*hdr).frame_type as core::ffi::c_uint
                                                    & 1 as core::ffi::c_uint != 0
                                                {
                                                    let mut i_16: core::ffi::c_int = 0 as core::ffi::c_int;
                                                    loop {
                                                        if !(i_16 < 7 as core::ffi::c_int) {
                                                            current_block = 18;
                                                            break;
                                                        }
                                                        (*hdr).gmv[i_16 as usize].type_0 = (if dav1d_get_bit(gb)
                                                            == 0
                                                        {
                                                            DAV1D_WM_TYPE_IDENTITY as core::ffi::c_int
                                                        } else if dav1d_get_bit(gb) != 0 {
                                                            DAV1D_WM_TYPE_ROT_ZOOM as core::ffi::c_int
                                                        } else if dav1d_get_bit(gb) != 0 {
                                                            DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                                        } else {
                                                            DAV1D_WM_TYPE_AFFINE as core::ffi::c_int
                                                        }) as Dav1dWarpedMotionType;
                                                        if !((*hdr).gmv[i_16 as usize].type_0 as core::ffi::c_uint
                                                            == DAV1D_WM_TYPE_IDENTITY as core::ffi::c_int
                                                                as core::ffi::c_uint)
                                                        {
                                                            let mut ref_gmv: *const Dav1dWarpedMotionParams = 0
                                                                as *const Dav1dWarpedMotionParams;
                                                            if (*hdr).primary_ref_frame as core::ffi::c_int
                                                                == DAV1D_PRIMARY_REF_NONE
                                                            {
                                                                ref_gmv = &dav1d_default_wm_params;
                                                            } else {
                                                                let pri_ref_0: core::ffi::c_int = (*hdr)
                                                                    .refidx[(*hdr).primary_ref_frame as usize]
                                                                    as core::ffi::c_int;
                                                                if ((*c).refs[pri_ref_0 as usize].p.p.frame_hdr).is_null() {
                                                                    current_block = 5;
                                                                    break;
                                                                }
                                                                ref_gmv = &raw mut *((*(*((*c).refs)
                                                                    .as_mut_ptr()
                                                                    .offset(pri_ref_0 as isize))
                                                                    .p
                                                                    .p
                                                                    .frame_hdr)
                                                                    .gmv)
                                                                    .as_mut_ptr()
                                                                    .offset(i_16 as isize) as *mut Dav1dWarpedMotionParams;
                                                            }
                                                            let mat: *mut int32_t = ((*hdr).gmv[i_16 as usize].matrix)
                                                                .as_mut_ptr();
                                                            let ref_mat: *const int32_t = ((*ref_gmv).matrix).as_ptr();
                                                            let mut bits: core::ffi::c_int = 0;
                                                            let mut shift: core::ffi::c_int = 0;
                                                            if (*hdr).gmv[i_16 as usize].type_0 as core::ffi::c_uint
                                                                >= DAV1D_WM_TYPE_ROT_ZOOM as core::ffi::c_int
                                                                    as core::ffi::c_uint
                                                            {
                                                                *mat.offset(2 as core::ffi::c_int as isize) = (((1
                                                                    as core::ffi::c_int) << 16 as core::ffi::c_int)
                                                                    + 2 as core::ffi::c_int
                                                                        * dav1d_get_bits_subexp(
                                                                            gb,
                                                                            *ref_mat.offset(2 as core::ffi::c_int as isize)
                                                                                - ((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                                                                                >> 1 as core::ffi::c_int,
                                                                            12 as core::ffi::c_uint,
                                                                        )) as int32_t;
                                                                *mat.offset(3 as core::ffi::c_int as isize) = (2
                                                                    as core::ffi::c_int
                                                                    * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(3 as core::ffi::c_int as isize)
                                                                            >> 1 as core::ffi::c_int,
                                                                        12 as core::ffi::c_uint,
                                                                    )) as int32_t;
                                                                bits = 12 as core::ffi::c_int;
                                                                shift = 10 as core::ffi::c_int;
                                                            } else {
                                                                bits = 9 as core::ffi::c_int
                                                                    - ((*hdr).hp == 0) as core::ffi::c_int;
                                                                shift = 13 as core::ffi::c_int
                                                                    + ((*hdr).hp == 0) as core::ffi::c_int;
                                                            }
                                                            if (*hdr).gmv[i_16 as usize].type_0 as core::ffi::c_uint
                                                                == DAV1D_WM_TYPE_AFFINE as core::ffi::c_int
                                                                    as core::ffi::c_uint
                                                            {
                                                                *mat.offset(4 as core::ffi::c_int as isize) = (2
                                                                    as core::ffi::c_int
                                                                    * dav1d_get_bits_subexp(
                                                                        gb,
                                                                        *ref_mat.offset(4 as core::ffi::c_int as isize)
                                                                            >> 1 as core::ffi::c_int,
                                                                        12 as core::ffi::c_uint,
                                                                    )) as int32_t;
                                                                *mat.offset(5 as core::ffi::c_int as isize) = (((1
                                                                    as core::ffi::c_int) << 16 as core::ffi::c_int)
                                                                    + 2 as core::ffi::c_int
                                                                        * dav1d_get_bits_subexp(
                                                                            gb,
                                                                            *ref_mat.offset(5 as core::ffi::c_int as isize)
                                                                                - ((1 as core::ffi::c_int) << 16 as core::ffi::c_int)
                                                                                >> 1 as core::ffi::c_int,
                                                                            12 as core::ffi::c_uint,
                                                                        )) as int32_t;
                                                            } else {
                                                                *mat.offset(4 as core::ffi::c_int as isize) = -*mat
                                                                    .offset(3 as core::ffi::c_int as isize);
                                                                *mat.offset(5 as core::ffi::c_int as isize) = *mat
                                                                    .offset(2 as core::ffi::c_int as isize);
                                                            }
                                                            *mat.offset(0 as core::ffi::c_int as isize) = (dav1d_get_bits_subexp(
                                                                gb,
                                                                *ref_mat.offset(0 as core::ffi::c_int as isize) >> shift,
                                                                bits as core::ffi::c_uint,
                                                            ) * ((1 as core::ffi::c_int) << shift)) as int32_t;
                                                            *mat.offset(1 as core::ffi::c_int as isize) = (dav1d_get_bits_subexp(
                                                                gb,
                                                                *ref_mat.offset(1 as core::ffi::c_int as isize) >> shift,
                                                                bits as core::ffi::c_uint,
                                                            ) * ((1 as core::ffi::c_int) << shift)) as int32_t;
                                                        }
                                                        i_16 += 1;
                                                    }
                                                } else {
                                                    current_block = 18;
                                                }
                                                match current_block {
                                                    5 => {}
                                                    _ => {
                                                        if (*seqhdr).film_grain_present as core::ffi::c_int != 0
                                                            && ((*hdr).show_frame as core::ffi::c_int != 0
                                                                || (*hdr).showable_frame as core::ffi::c_int != 0)
                                                        {
                                                            (*hdr).film_grain.present = dav1d_get_bit(gb) as uint8_t;
                                                            if (*hdr).film_grain.present != 0 {
                                                                let seed: core::ffi::c_uint = dav1d_get_bits(
                                                                    gb,
                                                                    16 as core::ffi::c_int,
                                                                ) as core::ffi::c_uint;
                                                                (*hdr).film_grain.update = ((*hdr).frame_type
                                                                    as core::ffi::c_uint
                                                                    != DAV1D_FRAME_TYPE_INTER as core::ffi::c_int
                                                                        as core::ffi::c_uint || dav1d_get_bit(gb) != 0)
                                                                    as core::ffi::c_int as uint8_t;
                                                                if (*hdr).film_grain.update == 0 {
                                                                    let refidx_0: core::ffi::c_int = dav1d_get_bits(
                                                                        gb,
                                                                        3 as core::ffi::c_int,
                                                                    ) as core::ffi::c_int;
                                                                    let mut i_17: core::ffi::c_int = 0;
                                                                    i_17 = 0 as core::ffi::c_int;
                                                                    while i_17 < 7 as core::ffi::c_int {
                                                                        if (*hdr).refidx[i_17 as usize] as core::ffi::c_int
                                                                            == refidx_0
                                                                        {
                                                                            break;
                                                                        }
                                                                        i_17 += 1;
                                                                    }
                                                                    if i_17 == 7 as core::ffi::c_int
                                                                        || ((*c).refs[refidx_0 as usize].p.p.frame_hdr).is_null()
                                                                    {
                                                                        current_block = 5;
                                                                    } else {
                                                                        (*hdr).film_grain.data = (*(*c)
                                                                            .refs[refidx_0 as usize]
                                                                            .p
                                                                            .p
                                                                            .frame_hdr)
                                                                            .film_grain
                                                                            .data;
                                                                        (*hdr).film_grain.data.seed = seed;
                                                                        current_block = 19;
                                                                    }
                                                                } else {
                                                                    let fgd: *mut Dav1dFilmGrainData = &raw mut (*hdr)
                                                                        .film_grain
                                                                        .data;
                                                                    (*fgd).seed = seed;
                                                                    (*fgd).num_y_points = dav1d_get_bits(
                                                                        gb,
                                                                        4 as core::ffi::c_int,
                                                                    ) as core::ffi::c_int;
                                                                    if (*fgd).num_y_points > 14 as core::ffi::c_int {
                                                                        current_block = 5;
                                                                    } else {
                                                                        let mut i_18: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                        loop {
                                                                            if !(i_18 < (*fgd).num_y_points) {
                                                                                current_block = 20;
                                                                                break;
                                                                            }
                                                                            (*fgd)
                                                                                .y_points[i_18 as usize][0 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                                                gb,
                                                                                8 as core::ffi::c_int,
                                                                            ) as uint8_t;
                                                                            if i_18 != 0
                                                                                && (*fgd)
                                                                                    .y_points[(i_18 - 1 as core::ffi::c_int)
                                                                                    as usize][0 as core::ffi::c_int as usize]
                                                                                    as core::ffi::c_int
                                                                                    >= (*fgd)
                                                                                        .y_points[i_18 as usize][0 as core::ffi::c_int as usize]
                                                                                        as core::ffi::c_int
                                                                            {
                                                                                current_block = 5;
                                                                                break;
                                                                            }
                                                                            (*fgd)
                                                                                .y_points[i_18 as usize][1 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                                                gb,
                                                                                8 as core::ffi::c_int,
                                                                            ) as uint8_t;
                                                                            i_18 += 1;
                                                                        }
                                                                        match current_block {
                                                                            5 => {}
                                                                            _ => {
                                                                                if (*seqhdr).monochrome == 0 {
                                                                                    (*fgd).chroma_scaling_from_luma = dav1d_get_bit(gb)
                                                                                        as core::ffi::c_int;
                                                                                }
                                                                                if (*seqhdr).monochrome as core::ffi::c_int != 0
                                                                                    || (*fgd).chroma_scaling_from_luma != 0
                                                                                    || (*seqhdr).ss_ver as core::ffi::c_int
                                                                                        == 1 as core::ffi::c_int
                                                                                        && (*seqhdr).ss_hor as core::ffi::c_int
                                                                                            == 1 as core::ffi::c_int && (*fgd).num_y_points == 0
                                                                                {
                                                                                    (*fgd).num_uv_points[1 as core::ffi::c_int as usize] = 0
                                                                                        as core::ffi::c_int;
                                                                                    (*fgd).num_uv_points[0 as core::ffi::c_int as usize] = (*fgd)
                                                                                        .num_uv_points[1 as core::ffi::c_int as usize];
                                                                                    current_block = 21;
                                                                                } else {
                                                                                    let mut pl: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                    's_1943: loop {
                                                                                        if !(pl < 2 as core::ffi::c_int) {
                                                                                            current_block = 21;
                                                                                            break;
                                                                                        }
                                                                                        (*fgd).num_uv_points[pl as usize] = dav1d_get_bits(
                                                                                            gb,
                                                                                            4 as core::ffi::c_int,
                                                                                        ) as core::ffi::c_int;
                                                                                        if (*fgd).num_uv_points[pl as usize]
                                                                                            > 10 as core::ffi::c_int
                                                                                        {
                                                                                            current_block = 5;
                                                                                            break;
                                                                                        }
                                                                                        let mut i_19: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                        while i_19 < (*fgd).num_uv_points[pl as usize] {
                                                                                            (*fgd)
                                                                                                .uv_points[pl
                                                                                                as usize][i_19 as usize][0 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                                                                gb,
                                                                                                8 as core::ffi::c_int,
                                                                                            ) as uint8_t;
                                                                                            if i_19 != 0
                                                                                                && (*fgd)
                                                                                                    .uv_points[pl
                                                                                                    as usize][(i_19 - 1 as core::ffi::c_int)
                                                                                                    as usize][0 as core::ffi::c_int as usize]
                                                                                                    as core::ffi::c_int
                                                                                                    >= (*fgd)
                                                                                                        .uv_points[pl
                                                                                                        as usize][i_19 as usize][0 as core::ffi::c_int as usize]
                                                                                                        as core::ffi::c_int
                                                                                            {
                                                                                                current_block = 5;
                                                                                                break 's_1943;
                                                                                            }
                                                                                            (*fgd)
                                                                                                .uv_points[pl
                                                                                                as usize][i_19 as usize][1 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                                                                gb,
                                                                                                8 as core::ffi::c_int,
                                                                                            ) as uint8_t;
                                                                                            i_19 += 1;
                                                                                        }
                                                                                        pl += 1;
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    5 => {}
                                                                                    _ => {
                                                                                        if (*seqhdr).ss_hor as core::ffi::c_int
                                                                                            == 1 as core::ffi::c_int
                                                                                            && (*seqhdr).ss_ver as core::ffi::c_int
                                                                                                == 1 as core::ffi::c_int
                                                                                            && ((*fgd).num_uv_points[0 as core::ffi::c_int as usize]
                                                                                                != 0) as core::ffi::c_int
                                                                                                != ((*fgd).num_uv_points[1 as core::ffi::c_int as usize]
                                                                                                    != 0) as core::ffi::c_int
                                                                                        {
                                                                                            current_block = 5;
                                                                                        } else {
                                                                                            (*fgd).scaling_shift = (dav1d_get_bits(
                                                                                                gb,
                                                                                                2 as core::ffi::c_int,
                                                                                            ))
                                                                                                .wrapping_add(8 as core::ffi::c_uint) as core::ffi::c_int;
                                                                                            (*fgd).ar_coeff_lag = dav1d_get_bits(
                                                                                                gb,
                                                                                                2 as core::ffi::c_int,
                                                                                            ) as core::ffi::c_int;
                                                                                            let num_y_pos: core::ffi::c_int = 2 as core::ffi::c_int
                                                                                                * (*fgd).ar_coeff_lag
                                                                                                * ((*fgd).ar_coeff_lag + 1 as core::ffi::c_int);
                                                                                            if (*fgd).num_y_points != 0 {
                                                                                                let mut i_20: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                                while i_20 < num_y_pos {
                                                                                                    (*fgd).ar_coeffs_y[i_20 as usize] = (dav1d_get_bits(
                                                                                                        gb,
                                                                                                        8 as core::ffi::c_int,
                                                                                                    ))
                                                                                                        .wrapping_sub(128 as core::ffi::c_uint) as int8_t;
                                                                                                    i_20 += 1;
                                                                                                }
                                                                                            }
                                                                                            let mut pl_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                            while pl_0 < 2 as core::ffi::c_int {
                                                                                                if (*fgd).num_uv_points[pl_0 as usize] != 0
                                                                                                    || (*fgd).chroma_scaling_from_luma != 0
                                                                                                {
                                                                                                    let num_uv_pos: core::ffi::c_int = num_y_pos
                                                                                                        + ((*fgd).num_y_points != 0) as core::ffi::c_int;
                                                                                                    let mut i_21: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                                    while i_21 < num_uv_pos {
                                                                                                        (*fgd).ar_coeffs_uv[pl_0 as usize][i_21 as usize] = (dav1d_get_bits(
                                                                                                            gb,
                                                                                                            8 as core::ffi::c_int,
                                                                                                        ))
                                                                                                            .wrapping_sub(128 as core::ffi::c_uint) as int8_t;
                                                                                                        i_21 += 1;
                                                                                                    }
                                                                                                    if (*fgd).num_y_points == 0 {
                                                                                                        (*fgd).ar_coeffs_uv[pl_0 as usize][num_uv_pos as usize] = 0
                                                                                                            as int8_t;
                                                                                                    }
                                                                                                }
                                                                                                pl_0 += 1;
                                                                                            }
                                                                                            (*fgd).ar_coeff_shift = (dav1d_get_bits(
                                                                                                gb,
                                                                                                2 as core::ffi::c_int,
                                                                                            ))
                                                                                                .wrapping_add(6 as core::ffi::c_uint) as uint64_t;
                                                                                            (*fgd).grain_scale_shift = dav1d_get_bits(
                                                                                                gb,
                                                                                                2 as core::ffi::c_int,
                                                                                            ) as core::ffi::c_int;
                                                                                            let mut pl_1: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                            while pl_1 < 2 as core::ffi::c_int {
                                                                                                if (*fgd).num_uv_points[pl_1 as usize] != 0 {
                                                                                                    (*fgd).uv_mult[pl_1 as usize] = (dav1d_get_bits(
                                                                                                        gb,
                                                                                                        8 as core::ffi::c_int,
                                                                                                    ))
                                                                                                        .wrapping_sub(128 as core::ffi::c_uint) as core::ffi::c_int;
                                                                                                    (*fgd).uv_luma_mult[pl_1 as usize] = (dav1d_get_bits(
                                                                                                        gb,
                                                                                                        8 as core::ffi::c_int,
                                                                                                    ))
                                                                                                        .wrapping_sub(128 as core::ffi::c_uint) as core::ffi::c_int;
                                                                                                    (*fgd).uv_offset[pl_1 as usize] = (dav1d_get_bits(
                                                                                                        gb,
                                                                                                        9 as core::ffi::c_int,
                                                                                                    ))
                                                                                                        .wrapping_sub(256 as core::ffi::c_uint) as core::ffi::c_int;
                                                                                                }
                                                                                                pl_1 += 1;
                                                                                            }
                                                                                            (*fgd).overlap_flag = dav1d_get_bit(gb) as core::ffi::c_int;
                                                                                            (*fgd).clip_to_restricted_range = dav1d_get_bit(gb)
                                                                                                as core::ffi::c_int;
                                                                                            current_block = 19;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                current_block = 19;
                                                            }
                                                        } else {
                                                            current_block = 19;
                                                        }
                                                        match current_block {
                                                            5 => {}
                                                            _ => return 0 as core::ffi::c_int,
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dav1d_log(
        c,
        b"Error parsing frame header\n\0" as *const u8 as *const core::ffi::c_char,
    );
    return -(22 as core::ffi::c_int);
}
unsafe extern "C" fn parse_tile_hdr(c: *mut Dav1dContext, gb: *mut GetBits) {
    let n_tiles: core::ffi::c_int = (*(*c).frame_hdr).tiling.cols as core::ffi::c_int
        * (*(*c).frame_hdr).tiling.rows as core::ffi::c_int;
    let have_tile_pos: core::ffi::c_int = (if n_tiles > 1 as core::ffi::c_int {
        dav1d_get_bit(gb)
    } else {
        0 as core::ffi::c_uint
    }) as core::ffi::c_int;
    if have_tile_pos != 0 {
        let n_bits: core::ffi::c_int = (*(*c).frame_hdr).tiling.log2_cols
            as core::ffi::c_int + (*(*c).frame_hdr).tiling.log2_rows as core::ffi::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).start = dav1d_get_bits(
            gb,
            n_bits,
        ) as core::ffi::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).end = dav1d_get_bits(gb, n_bits)
            as core::ffi::c_int;
    } else {
        (*((*c).tile).offset((*c).n_tile_data as isize)).start = 0 as core::ffi::c_int;
        (*((*c).tile).offset((*c).n_tile_data as isize)).end = n_tiles
            - 1 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_obus(
    c: *mut Dav1dContext,
    in_0: *mut Dav1dData,
) -> ptrdiff_t {
    let mut type_0: Dav1dObuType = 0 as Dav1dObuType;
    let mut has_extension: core::ffi::c_int = 0;
    let mut has_length_field: core::ffi::c_int = 0;
    let mut temporal_id: core::ffi::c_int = 0;
    let mut spatial_id: core::ffi::c_int = 0;
    let mut current_block: u64;
    let mut gb: GetBits = GetBits {
        state: 0,
        bits_left: 0,
        error: 0,
        ptr: 0 as *const uint8_t,
        ptr_start: 0 as *const uint8_t,
        ptr_end: 0 as *const uint8_t,
    };
    let mut res: core::ffi::c_int = 0;
    dav1d_init_get_bits(&raw mut gb, (*in_0).data, (*in_0).sz);
    let obu_forbidden_bit: core::ffi::c_int = dav1d_get_bit(&raw mut gb) as core::ffi::c_int;
    if !((*c).strict_std_compliance != 0 && obu_forbidden_bit != 0) {
        type_0 = dav1d_get_bits(&raw mut gb, 4 as core::ffi::c_int) as Dav1dObuType;
        has_extension = dav1d_get_bit(&raw mut gb) as core::ffi::c_int;
        has_length_field = dav1d_get_bit(&raw mut gb) as core::ffi::c_int;
        dav1d_get_bit(&raw mut gb);
        temporal_id = 0 as core::ffi::c_int;
        spatial_id = 0 as core::ffi::c_int;
        if has_extension != 0 {
            temporal_id = dav1d_get_bits(&raw mut gb, 3 as core::ffi::c_int)
                as core::ffi::c_int;
            spatial_id = dav1d_get_bits(&raw mut gb, 2 as core::ffi::c_int)
                as core::ffi::c_int;
            dav1d_get_bits(&raw mut gb, 3 as core::ffi::c_int);
        }
        if has_length_field != 0 {
            let len: size_t = dav1d_get_uleb128(&raw mut gb) as size_t;
            if len > (gb.ptr_end).offset_from(gb.ptr) as core::ffi::c_long as size_t {
                current_block = 22;
            } else {
                gb.ptr_end = (gb.ptr).offset(len as isize);
                current_block = 23;
            }
        } else {
            current_block = 23;
        }
        match current_block {
            22 => {}
            _ => {
                if !(gb.error != 0) {
                    if !(gb.bits_left == 0 as core::ffi::c_int) {
                        unreachable!();
                    }
                    if type_0 as core::ffi::c_uint
                        != DAV1D_OBU_SEQ_HDR as core::ffi::c_int as core::ffi::c_uint
                        && type_0 as core::ffi::c_uint
                            != DAV1D_OBU_TD as core::ffi::c_int as core::ffi::c_uint
                        && has_extension != 0
                        && (*c).operating_point_idc != 0 as core::ffi::c_uint
                    {
                        let in_temporal_layer: core::ffi::c_int = ((*c)
                            .operating_point_idc >> temporal_id & 1 as core::ffi::c_uint)
                            as core::ffi::c_int;
                        let in_spatial_layer: core::ffi::c_int = ((*c)
                            .operating_point_idc >> spatial_id + 8 as core::ffi::c_int
                            & 1 as core::ffi::c_uint) as core::ffi::c_int;
                        if in_temporal_layer == 0 || in_spatial_layer == 0 {
                            return (gb.ptr_end).offset_from(gb.ptr_start) as ptrdiff_t;
                        }
                    }
                    match type_0 as core::ffi::c_uint {
                        1 => {
                            let mut ref_0: *mut Dav1dRef = dav1d_ref_create_using_pool(
                                (*c).seq_hdr_pool,
                                ::core::mem::size_of::<Dav1dSequenceHeader>() as size_t,
                            );
                            if ref_0.is_null() {
                                return -(12 as core::ffi::c_int) as ptrdiff_t;
                            }
                            let mut seq_hdr: *mut Dav1dSequenceHeader = (*ref_0).data
                                as *mut Dav1dSequenceHeader;
                            res = parse_seq_hdr(
                                seq_hdr,
                                &raw mut gb,
                                (*c).strict_std_compliance,
                            );
                            if res < 0 as core::ffi::c_int {
                                dav1d_log(
                                    c,
                                    b"Error parsing sequence header\n\0" as *const u8
                                        as *const core::ffi::c_char,
                                );
                                dav1d_ref_dec(&raw mut ref_0);
                                current_block = 22;
                            } else {
                                let op_idx: core::ffi::c_int = if (*c).operating_point
                                    < (*seq_hdr).num_operating_points as core::ffi::c_int
                                {
                                    (*c).operating_point
                                } else {
                                    0 as core::ffi::c_int
                                };
                                (*c).operating_point_idc = (*seq_hdr)
                                    .operating_points[op_idx as usize]
                                    .idc as core::ffi::c_uint;
                                let spatial_mask: core::ffi::c_uint = (*c)
                                    .operating_point_idc >> 8 as core::ffi::c_int;
                                (*c).max_spatial_id = if spatial_mask != 0 {
                                    ulog2(spatial_mask)
                                } else {
                                    0 as core::ffi::c_int
                                };
                                if ((*c).seq_hdr).is_null() {
                                    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                    (*c).frame_flags = ::core::mem::transmute::<
                                        core::ffi::c_uint,
                                        PictureFlags,
                                    >(
                                        (*c).frame_flags as core::ffi::c_uint
                                            | PICTURE_FLAG_NEW_SEQUENCE as core::ffi::c_int
                                                as core::ffi::c_uint,
                                    );
                                } else if memcmp(
                                    seq_hdr as *const core::ffi::c_void,
                                    (*c).seq_hdr as *const core::ffi::c_void,
                                    424 as size_t,
                                ) != 0
                                {
                                    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                    (*c).mastering_display = 0 as *mut Dav1dMasteringDisplay;
                                    (*c).content_light = 0 as *mut Dav1dContentLightLevel;
                                    dav1d_ref_dec(&raw mut (*c).mastering_display_ref);
                                    dav1d_ref_dec(&raw mut (*c).content_light_ref);
                                    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                                    while i < 8 as core::ffi::c_int {
                                        if !((*c).refs[i as usize].p.p.frame_hdr).is_null() {
                                            dav1d_thread_picture_unref(
                                                &raw mut (*((*c).refs).as_mut_ptr().offset(i as isize)).p,
                                            );
                                        }
                                        dav1d_ref_dec(
                                            &raw mut (*((*c).refs).as_mut_ptr().offset(i as isize)).segmap,
                                        );
                                        dav1d_ref_dec(
                                            &raw mut (*((*c).refs).as_mut_ptr().offset(i as isize)).refmvs,
                                        );
                                        dav1d_cdf_thread_unref(
                                            &raw mut *((*c).cdf).as_mut_ptr().offset(i as isize),
                                        );
                                        i += 1;
                                    }
                                    (*c).frame_flags = ::core::mem::transmute::<
                                        core::ffi::c_uint,
                                        PictureFlags,
                                    >(
                                        (*c).frame_flags as core::ffi::c_uint
                                            | PICTURE_FLAG_NEW_SEQUENCE as core::ffi::c_int
                                                as core::ffi::c_uint,
                                    );
                                } else if memcmp(
                                    ((*seq_hdr).operating_parameter_info).as_mut_ptr()
                                        as *const core::ffi::c_void,
                                    ((*(*c).seq_hdr).operating_parameter_info).as_mut_ptr()
                                        as *const core::ffi::c_void,
                                    ::core::mem::size_of::<
                                        [Dav1dSequenceHeaderOperatingParameterInfo; 32],
                                    >() as size_t,
                                ) != 0
                                {
                                    (*c).frame_flags = ::core::mem::transmute::<
                                        core::ffi::c_uint,
                                        PictureFlags,
                                    >(
                                        (*c).frame_flags as core::ffi::c_uint
                                            | PICTURE_FLAG_NEW_OP_PARAMS_INFO as core::ffi::c_int
                                                as core::ffi::c_uint,
                                    );
                                }
                                dav1d_ref_dec(&raw mut (*c).seq_hdr_ref);
                                (*c).seq_hdr_ref = ref_0;
                                (*c).seq_hdr = seq_hdr;
                                current_block = 24;
                            }
                        }
                        7 => {
                            if !((*c).frame_hdr).is_null() {
                                current_block = 24;
                            } else {
                                current_block = 25;
                            }
                        }
                        6 | 3 => {
                            current_block = 25;
                        }
                        4 => {
                            current_block = 26;
                        }
                        5 => {
                            let meta_type: ObuMetaType = dav1d_get_uleb128(&raw mut gb)
                                as ObuMetaType;
                            if gb.error != 0 {
                                current_block = 22;
                            } else {
                                match meta_type as core::ffi::c_uint {
                                    1 => {
                                        let mut ref_1: *mut Dav1dRef = dav1d_ref_create(
                                            ::core::mem::size_of::<Dav1dContentLightLevel>() as size_t,
                                        );
                                        if ref_1.is_null() {
                                            return -(12 as core::ffi::c_int) as ptrdiff_t;
                                        }
                                        let content_light: *mut Dav1dContentLightLevel = (*ref_1)
                                            .data as *mut Dav1dContentLightLevel;
                                        (*content_light).max_content_light_level = dav1d_get_bits(
                                            &raw mut gb,
                                            16 as core::ffi::c_int,
                                        ) as uint16_t;
                                        (*content_light).max_frame_average_light_level = dav1d_get_bits(
                                            &raw mut gb,
                                            16 as core::ffi::c_int,
                                        ) as uint16_t;
                                        if check_trailing_bits(&raw mut gb, (*c).strict_std_compliance)
                                            < 0 as core::ffi::c_int
                                        {
                                            dav1d_ref_dec(&raw mut ref_1);
                                            current_block = 22;
                                        } else {
                                            dav1d_ref_dec(&raw mut (*c).content_light_ref);
                                            (*c).content_light = content_light;
                                            (*c).content_light_ref = ref_1;
                                            current_block = 24;
                                        }
                                    }
                                    2 => {
                                        let mut ref_2: *mut Dav1dRef = dav1d_ref_create(
                                            ::core::mem::size_of::<Dav1dMasteringDisplay>() as size_t,
                                        );
                                        if ref_2.is_null() {
                                            return -(12 as core::ffi::c_int) as ptrdiff_t;
                                        }
                                        let mastering_display: *mut Dav1dMasteringDisplay = (*ref_2)
                                            .data as *mut Dav1dMasteringDisplay;
                                        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
                                        while i_1 < 3 as core::ffi::c_int {
                                            (*mastering_display)
                                                .primaries[i_1 as usize][0 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                &raw mut gb,
                                                16 as core::ffi::c_int,
                                            ) as uint16_t;
                                            (*mastering_display)
                                                .primaries[i_1 as usize][1 as core::ffi::c_int as usize] = dav1d_get_bits(
                                                &raw mut gb,
                                                16 as core::ffi::c_int,
                                            ) as uint16_t;
                                            i_1 += 1;
                                        }
                                        (*mastering_display)
                                            .white_point[0 as core::ffi::c_int as usize] = dav1d_get_bits(
                                            &raw mut gb,
                                            16 as core::ffi::c_int,
                                        ) as uint16_t;
                                        (*mastering_display)
                                            .white_point[1 as core::ffi::c_int as usize] = dav1d_get_bits(
                                            &raw mut gb,
                                            16 as core::ffi::c_int,
                                        ) as uint16_t;
                                        (*mastering_display).max_luminance = dav1d_get_bits(
                                            &raw mut gb,
                                            32 as core::ffi::c_int,
                                        ) as uint32_t;
                                        (*mastering_display).min_luminance = dav1d_get_bits(
                                            &raw mut gb,
                                            32 as core::ffi::c_int,
                                        ) as uint32_t;
                                        if check_trailing_bits(&raw mut gb, (*c).strict_std_compliance)
                                            < 0 as core::ffi::c_int
                                        {
                                            dav1d_ref_dec(&raw mut ref_2);
                                            current_block = 22;
                                        } else {
                                            dav1d_ref_dec(&raw mut (*c).mastering_display_ref);
                                            (*c).mastering_display = mastering_display;
                                            (*c).mastering_display_ref = ref_2;
                                            current_block = 24;
                                        }
                                    }
                                    4 => {
                                        let mut payload_size: ptrdiff_t = (gb.ptr_end)
                                            .offset_from(gb.ptr) as ptrdiff_t;
                                        while payload_size > 0 as ptrdiff_t
                                            && *(gb.ptr)
                                                .offset((payload_size - 1 as ptrdiff_t) as isize) == 0
                                        {
                                            payload_size -= 1;
                                        }
                                        payload_size -= 1;
                                        let mut country_code_extension_byte: core::ffi::c_int = 0
                                            as core::ffi::c_int;
                                        let country_code: core::ffi::c_int = dav1d_get_bits(
                                            &raw mut gb,
                                            8 as core::ffi::c_int,
                                        ) as core::ffi::c_int;
                                        payload_size -= 1;
                                        if country_code == 0xff as core::ffi::c_int {
                                            country_code_extension_byte = dav1d_get_bits(
                                                &raw mut gb,
                                                8 as core::ffi::c_int,
                                            ) as core::ffi::c_int;
                                            payload_size -= 1;
                                        }
                                        if payload_size <= 0 as ptrdiff_t
                                            || *(gb.ptr).offset(payload_size as isize)
                                                as core::ffi::c_int != 0x80 as core::ffi::c_int
                                        {
                                            dav1d_log(
                                                c,
                                                b"Malformed ITU-T T.35 metadata message format\n\0"
                                                    as *const u8 as *const core::ffi::c_char,
                                            );
                                            current_block = 24;
                                        } else if (*c).n_itut_t35 + 1 as core::ffi::c_int
                                            > INT_MAX
                                                / ::core::mem::size_of::<Dav1dITUTT35>() as core::ffi::c_int
                                        {
                                            current_block = 22;
                                        } else {
                                            let mut itut_t35: *mut Dav1dITUTT35 = realloc(
                                                (*c).itut_t35 as *mut core::ffi::c_void,
                                                (((*c).n_itut_t35 + 1 as core::ffi::c_int) as size_t)
                                                    .wrapping_mul(
                                                        ::core::mem::size_of::<Dav1dITUTT35>() as size_t,
                                                    ),
                                            ) as *mut Dav1dITUTT35;
                                            if itut_t35.is_null() {
                                                current_block = 22;
                                            } else {
                                                (*c).itut_t35 = itut_t35 as *mut Dav1dITUTT35;
                                                memset(
                                                    ((*c).itut_t35).offset((*c).n_itut_t35 as isize)
                                                        as *mut core::ffi::c_void,
                                                    0 as core::ffi::c_int,
                                                    ::core::mem::size_of::<Dav1dITUTT35>() as size_t,
                                                );
                                                let mut itut_t35_ctx: *mut itut_t35_ctx_context = 0
                                                    as *mut itut_t35_ctx_context;
                                                if (*c).n_itut_t35 == 0 {
                                                    if !((*c).itut_t35_ref).is_null() {
                                                        unreachable!();
                                                    }
                                                    itut_t35_ctx = malloc(
                                                        ::core::mem::size_of::<itut_t35_ctx_context>() as size_t,
                                                    ) as *mut itut_t35_ctx_context;
                                                    if itut_t35_ctx.is_null() {
                                                        current_block = 22;
                                                    } else {
                                                        (*c).itut_t35_ref = dav1d_ref_init(
                                                            &raw mut (*itut_t35_ctx).ref_0,
                                                            (*c).itut_t35 as *const core::ffi::c_void,
                                                            Some(
                                                                dav1d_picture_free_itut_t35
                                                                    as unsafe extern "C" fn(
                                                                        *const uint8_t,
                                                                        *mut core::ffi::c_void,
                                                                    ) -> (),
                                                            ),
                                                            itut_t35_ctx as *mut core::ffi::c_void,
                                                            0 as core::ffi::c_int,
                                                        );
                                                        current_block = 27;
                                                    }
                                                } else {
                                                    if !(!((*c).itut_t35_ref).is_null()
                                                        && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                            &raw mut (*(*c).itut_t35_ref).ref_cnt as *mut atomic_int,
                                                        ) == 1 as core::ffi::c_int)
                                                    {
                                                        unreachable!();
                                                    }
                                                    itut_t35_ctx = (*(*c).itut_t35_ref).user_data
                                                        as *mut itut_t35_ctx_context;
                                                    (*(*c).itut_t35_ref).const_data = (*c).itut_t35
                                                        as *mut uint8_t as *const core::ffi::c_void;
                                                    current_block = 27;
                                                }
                                                match current_block {
                                                    22 => {}
                                                    _ => {
                                                        (*itut_t35_ctx).itut_t35 = (*c).itut_t35;
                                                        (*itut_t35_ctx).n_itut_t35 = ((*c).n_itut_t35
                                                            + 1 as core::ffi::c_int) as size_t;
                                                        let itut_t35_metadata: *mut Dav1dITUTT35 = &raw mut *((*c)
                                                            .itut_t35)
                                                            .offset((*c).n_itut_t35 as isize) as *mut Dav1dITUTT35;
                                                        (*itut_t35_metadata).payload = malloc(
                                                            payload_size as size_t,
                                                        ) as *mut uint8_t;
                                                        if ((*itut_t35_metadata).payload).is_null() {
                                                            current_block = 22;
                                                        } else {
                                                            (*itut_t35_metadata).country_code = country_code as uint8_t;
                                                            (*itut_t35_metadata).country_code_extension_byte = country_code_extension_byte
                                                                as uint8_t;
                                                            (*itut_t35_metadata).payload_size = payload_size as size_t;
                                                            if !(gb.bits_left == 0 as core::ffi::c_int) {
                                                                unreachable!();
                                                            }
                                                            memcpy(
                                                                (*itut_t35_metadata).payload as *mut core::ffi::c_void,
                                                                gb.ptr as *const core::ffi::c_void,
                                                                payload_size as size_t,
                                                            );
                                                            (*c).n_itut_t35 += 1;
                                                            current_block = 24;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    3 | 5 => {
                                        current_block = 24;
                                    }
                                    _ => {
                                        if meta_type as core::ffi::c_uint > 31 as core::ffi::c_uint
                                        {
                                            dav1d_log(
                                                c,
                                                b"Unknown Metadata OBU type %d\n\0" as *const u8
                                                    as *const core::ffi::c_char,
                                                meta_type as core::ffi::c_uint,
                                            );
                                        }
                                        current_block = 24;
                                    }
                                }
                            }
                        }
                        2 => {
                            (*c).frame_flags = ::core::mem::transmute::<
                                core::ffi::c_uint,
                                PictureFlags,
                            >(
                                (*c).frame_flags as core::ffi::c_uint
                                    | PICTURE_FLAG_NEW_TEMPORAL_UNIT as core::ffi::c_int
                                        as core::ffi::c_uint,
                            );
                            current_block = 24;
                        }
                        15 => {
                            current_block = 24;
                        }
                        _ => {
                            dav1d_log(
                                c,
                                b"Unknown OBU type %d of size %td\n\0" as *const u8
                                    as *const core::ffi::c_char,
                                type_0 as core::ffi::c_uint,
                                (gb.ptr_end).offset_from(gb.ptr) as core::ffi::c_long,
                            );
                            current_block = 24;
                        }
                    }
                    match current_block {
                        22 => {}
                        _ => {
                            match current_block {
                                25 => {
                                    if ((*c).seq_hdr).is_null() {
                                        current_block = 22;
                                    } else {
                                        if ((*c).frame_hdr_ref).is_null() {
                                            (*c).frame_hdr_ref = dav1d_ref_create_using_pool(
                                                (*c).frame_hdr_pool,
                                                ::core::mem::size_of::<Dav1dFrameHeader>() as size_t,
                                            );
                                            if ((*c).frame_hdr_ref).is_null() {
                                                return -(12 as core::ffi::c_int) as ptrdiff_t;
                                            }
                                        }
                                        (*c).frame_hdr = (*(*c).frame_hdr_ref).data
                                            as *mut Dav1dFrameHeader;
                                        memset(
                                            (*c).frame_hdr as *mut core::ffi::c_void,
                                            0 as core::ffi::c_int,
                                            ::core::mem::size_of::<Dav1dFrameHeader>() as size_t,
                                        );
                                        (*(*c).frame_hdr).temporal_id = temporal_id as uint8_t;
                                        (*(*c).frame_hdr).spatial_id = spatial_id as uint8_t;
                                        res = parse_frame_hdr(c, &raw mut gb);
                                        if res < 0 as core::ffi::c_int {
                                            (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                            current_block = 22;
                                        } else {
                                            let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
                                            while n < (*c).n_tile_data {
                                                dav1d_data_unref_internal(
                                                    &raw mut (*((*c).tile).offset(n as isize)).data,
                                                );
                                                n += 1;
                                            }
                                            (*c).n_tile_data = 0 as core::ffi::c_int;
                                            (*c).n_tiles = 0 as core::ffi::c_int;
                                            if type_0 as core::ffi::c_uint
                                                != DAV1D_OBU_FRAME as core::ffi::c_int as core::ffi::c_uint
                                            {
                                                if check_trailing_bits(&raw mut gb, (*c).strict_std_compliance)
                                                    < 0 as core::ffi::c_int
                                                {
                                                    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                    current_block = 22;
                                                } else {
                                                    current_block = 28;
                                                }
                                            } else {
                                                current_block = 28;
                                            }
                                            match current_block {
                                                22 => {}
                                                _ => {
                                                    if (*c).frame_size_limit != 0
                                                        && (*(*c).frame_hdr).width[1 as core::ffi::c_int as usize]
                                                            as int64_t * (*(*c).frame_hdr).height as int64_t
                                                            > (*c).frame_size_limit as int64_t
                                                    {
                                                        dav1d_log(
                                                            c,
                                                            b"Frame size %dx%d exceeds limit %u\n\0" as *const u8
                                                                as *const core::ffi::c_char,
                                                            (*(*c).frame_hdr).width[1 as core::ffi::c_int as usize],
                                                            (*(*c).frame_hdr).height,
                                                            (*c).frame_size_limit,
                                                        );
                                                        (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                        return -(34 as core::ffi::c_int) as ptrdiff_t;
                                                    }
                                                    if type_0 as core::ffi::c_uint
                                                        != DAV1D_OBU_FRAME as core::ffi::c_int as core::ffi::c_uint
                                                    {
                                                        current_block = 24;
                                                    } else if (*(*c).frame_hdr).show_existing_frame != 0 {
                                                        (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                        current_block = 22;
                                                    } else {
                                                        dav1d_bytealign_get_bits(&raw mut gb);
                                                        current_block = 26;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                            match current_block {
                                22 => {}
                                _ => {
                                    match current_block {
                                        26 => {
                                            if ((*c).frame_hdr).is_null() {
                                                current_block = 22;
                                            } else {
                                                if (*c).n_tile_data_alloc
                                                    < (*c).n_tile_data + 1 as core::ffi::c_int
                                                {
                                                    if (*c).n_tile_data + 1 as core::ffi::c_int
                                                        > INT_MAX
                                                            / ::core::mem::size_of::<Dav1dTileGroup>()
                                                                as core::ffi::c_int
                                                    {
                                                        current_block = 22;
                                                    } else {
                                                        let mut tile: *mut Dav1dTileGroup = realloc(
                                                            (*c).tile as *mut core::ffi::c_void,
                                                            (((*c).n_tile_data + 1 as core::ffi::c_int) as size_t)
                                                                .wrapping_mul(
                                                                    ::core::mem::size_of::<Dav1dTileGroup>() as size_t,
                                                                ),
                                                        ) as *mut Dav1dTileGroup;
                                                        if tile.is_null() {
                                                            current_block = 22;
                                                        } else {
                                                            (*c).tile = tile;
                                                            memset(
                                                                ((*c).tile).offset((*c).n_tile_data as isize)
                                                                    as *mut core::ffi::c_void,
                                                                0 as core::ffi::c_int,
                                                                ::core::mem::size_of::<Dav1dTileGroup>() as size_t,
                                                            );
                                                            (*c).n_tile_data_alloc = (*c).n_tile_data
                                                                + 1 as core::ffi::c_int;
                                                            current_block = 29;
                                                        }
                                                    }
                                                } else {
                                                    current_block = 29;
                                                }
                                                match current_block {
                                                    22 => {}
                                                    _ => {
                                                        parse_tile_hdr(c, &raw mut gb);
                                                        dav1d_bytealign_get_bits(&raw mut gb);
                                                        if gb.error != 0 {
                                                            current_block = 22;
                                                        } else {
                                                            dav1d_data_ref(
                                                                &raw mut (*((*c).tile).offset((*c).n_tile_data as isize)).data,
                                                                in_0,
                                                            );
                                                            let fresh0 = &raw mut (*((*c).tile)
                                                                .offset((*c).n_tile_data as isize))
                                                                .data
                                                                .data;
                                                            *fresh0 = gb.ptr;
                                                            (*((*c).tile).offset((*c).n_tile_data as isize)).data.sz = (gb
                                                                .ptr_end)
                                                                .offset_from(gb.ptr) as core::ffi::c_long as size_t;
                                                            if (*((*c).tile).offset((*c).n_tile_data as isize)).start
                                                                > (*((*c).tile).offset((*c).n_tile_data as isize)).end
                                                                || (*((*c).tile).offset((*c).n_tile_data as isize)).start
                                                                    != (*c).n_tiles
                                                            {
                                                                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                while i_0 <= (*c).n_tile_data {
                                                                    dav1d_data_unref_internal(
                                                                        &raw mut (*((*c).tile).offset(i_0 as isize)).data,
                                                                    );
                                                                    i_0 += 1;
                                                                }
                                                                (*c).n_tile_data = 0 as core::ffi::c_int;
                                                                (*c).n_tiles = 0 as core::ffi::c_int;
                                                                current_block = 22;
                                                            } else {
                                                                (*c).n_tiles
                                                                    += 1 as core::ffi::c_int
                                                                        + (*((*c).tile).offset((*c).n_tile_data as isize)).end
                                                                        - (*((*c).tile).offset((*c).n_tile_data as isize)).start;
                                                                (*c).n_tile_data += 1;
                                                                current_block = 24;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        22 => {}
                                        _ => {
                                            if !((*c).seq_hdr).is_null() && !((*c).frame_hdr).is_null()
                                            {
                                                if (*(*c).frame_hdr).show_existing_frame != 0 {
                                                    if ((*c)
                                                        .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                        .p
                                                        .p
                                                        .frame_hdr)
                                                        .is_null()
                                                    {
                                                        current_block = 22;
                                                    } else {
                                                        match (*(*c)
                                                            .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                            .p
                                                            .p
                                                            .frame_hdr)
                                                            .frame_type as core::ffi::c_uint
                                                        {
                                                            1 | 3 => {
                                                                if (*c).decode_frame_type as core::ffi::c_uint
                                                                    > DAV1D_DECODEFRAMETYPE_REFERENCE as core::ffi::c_int
                                                                        as core::ffi::c_uint
                                                                {
                                                                    current_block = 30;
                                                                } else {
                                                                    current_block = 31;
                                                                }
                                                            }
                                                            2 => {
                                                                if (*c).decode_frame_type as core::ffi::c_uint
                                                                    > DAV1D_DECODEFRAMETYPE_INTRA as core::ffi::c_int
                                                                        as core::ffi::c_uint
                                                                {
                                                                    current_block = 30;
                                                                } else {
                                                                    current_block = 31;
                                                                }
                                                            }
                                                            _ => {
                                                                current_block = 31;
                                                            }
                                                        }
                                                        match current_block {
                                                            30 => {}
                                                            _ => {
                                                                if ((*c)
                                                                    .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                                    .p
                                                                    .p
                                                                    .data[0 as core::ffi::c_int as usize])
                                                                    .is_null()
                                                                {
                                                                    current_block = 22;
                                                                } else if (*c).strict_std_compliance != 0
                                                                    && (*c)
                                                                        .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                                        .p
                                                                        .showable == 0
                                                                {
                                                                    current_block = 22;
                                                                } else {
                                                                    if (*c).n_fc == 1 as core::ffi::c_uint {
                                                                        dav1d_thread_picture_ref(
                                                                            &raw mut (*c).out,
                                                                            &raw mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset((*(*c).frame_hdr).existing_frame_idx as isize))
                                                                                .p,
                                                                        );
                                                                        dav1d_picture_copy_props(
                                                                            &raw mut (*c).out.p,
                                                                            (*c).content_light,
                                                                            (*c).content_light_ref,
                                                                            (*c).mastering_display,
                                                                            (*c).mastering_display_ref,
                                                                            (*c).itut_t35,
                                                                            (*c).itut_t35_ref,
                                                                            (*c).n_itut_t35 as size_t,
                                                                            &raw mut (*in_0).m,
                                                                        );
                                                                        dav1d_ref_dec(&raw mut (*c).itut_t35_ref);
                                                                        (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
                                                                        (*c).n_itut_t35 = 0 as core::ffi::c_int;
                                                                        (*c).event_flags = ::core::mem::transmute::<
                                                                            core::ffi::c_uint,
                                                                            Dav1dEventFlags,
                                                                        >(
                                                                            (*c).event_flags as core::ffi::c_uint
                                                                                | dav1d_picture_get_event_flags(
                                                                                    &raw mut (*((*c).refs)
                                                                                        .as_mut_ptr()
                                                                                        .offset((*(*c).frame_hdr).existing_frame_idx as isize))
                                                                                        .p,
                                                                                ) as core::ffi::c_uint,
                                                                        );
                                                                    } else {
                                                                        pthread_mutex_lock(&raw mut (*c).task_thread.lock);
                                                                        let fresh1 = (*c).frame_thread.next;
                                                                        (*c).frame_thread.next = ((*c).frame_thread.next)
                                                                            .wrapping_add(1);
                                                                        let next: core::ffi::c_uint = fresh1;
                                                                        if (*c).frame_thread.next == (*c).n_fc {
                                                                            (*c).frame_thread.next = 0 as core::ffi::c_uint;
                                                                        }
                                                                        let f: *mut Dav1dFrameContext = &raw mut *((*c).fc)
                                                                            .offset(next as isize) as *mut Dav1dFrameContext;
                                                                        while (*f).n_tile_data > 0 as core::ffi::c_int {
                                                                            pthread_cond_wait(
                                                                                &raw mut (*f).task_thread.cond,
                                                                                &raw mut (*(*f).task_thread.ttd).lock,
                                                                            );
                                                                        }
                                                                        let out_delayed: *mut Dav1dThreadPicture = &raw mut *((*c)
                                                                            .frame_thread
                                                                            .out_delayed)
                                                                            .offset(next as isize) as *mut Dav1dThreadPicture;
                                                                        if !((*out_delayed).p.data[0 as core::ffi::c_int as usize])
                                                                            .is_null()
                                                                            || ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                                                &raw mut (*f).task_thread.error as *mut atomic_int,
                                                                            ) != 0
                                                                        {
                                                                            let mut first: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                                                &raw mut (*c).task_thread.first,
                                                                            );
                                                                            if first.wrapping_add(1 as core::ffi::c_uint) < (*c).n_fc {
                                                                                ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                                                                    &raw mut (*c).task_thread.first,
                                                                                    1 as core::ffi::c_uint,
                                                                                );
                                                                            } else {
                                                                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                                                    &raw mut (*c).task_thread.first,
                                                                                    0 as core::ffi::c_int as core::ffi::c_uint,
                                                                                );
                                                                            }
                                                                            let fresh2 = ::core::intrinsics::atomic_cxchg::<_, {core::intrinsics::AtomicOrdering::SeqCst}, {core::intrinsics::AtomicOrdering::SeqCst}>(
                                                                                &raw mut (*c).task_thread.reset_task_cur,
                                                                                *&raw mut first,
                                                                                (2147483647 as core::ffi::c_uint)
                                                                                    .wrapping_mul(2 as core::ffi::c_uint)
                                                                                    .wrapping_add(1 as core::ffi::c_uint),
                                                                            );
                                                                            *&raw mut first = fresh2.0;
                                                                            fresh2.1;
                                                                            if (*c).task_thread.cur != 0
                                                                                && (*c).task_thread.cur < (*c).n_fc
                                                                            {
                                                                                (*c).task_thread.cur = ((*c).task_thread.cur)
                                                                                    .wrapping_sub(1);
                                                                            }
                                                                        }
                                                                        let error: core::ffi::c_int = (*f).task_thread.retval;
                                                                        if error != 0 {
                                                                            (*c).cached_error = error;
                                                                            (*f).task_thread.retval = 0 as core::ffi::c_int;
                                                                            dav1d_data_props_copy(
                                                                                &raw mut (*c).cached_error_props,
                                                                                &raw mut (*out_delayed).p.m,
                                                                            );
                                                                            dav1d_thread_picture_unref(out_delayed);
                                                                        } else if !((*out_delayed)
                                                                            .p
                                                                            .data[0 as core::ffi::c_int as usize])
                                                                            .is_null()
                                                                        {
                                                                            let progress: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::Relaxed}, false>(
                                                                                &raw mut *((*out_delayed).progress)
                                                                                    .offset(1 as core::ffi::c_int as isize) as *mut atomic_uint,
                                                                            );
                                                                            if ((*out_delayed).visible != 0
                                                                                || (*c).output_invisible_frames != 0)
                                                                                && progress != FRAME_ERROR
                                                                            {
                                                                                dav1d_thread_picture_ref(&raw mut (*c).out, out_delayed);
                                                                                (*c).event_flags = ::core::mem::transmute::<
                                                                                    core::ffi::c_uint,
                                                                                    Dav1dEventFlags,
                                                                                >(
                                                                                    (*c).event_flags as core::ffi::c_uint
                                                                                        | dav1d_picture_get_event_flags(out_delayed)
                                                                                            as core::ffi::c_uint,
                                                                                );
                                                                            }
                                                                            dav1d_thread_picture_unref(out_delayed);
                                                                        }
                                                                        dav1d_thread_picture_ref(
                                                                            out_delayed,
                                                                            &raw mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset((*(*c).frame_hdr).existing_frame_idx as isize))
                                                                                .p,
                                                                        );
                                                                        (*out_delayed).visible = 1 as core::ffi::c_int;
                                                                        dav1d_picture_copy_props(
                                                                            &raw mut (*out_delayed).p,
                                                                            (*c).content_light,
                                                                            (*c).content_light_ref,
                                                                            (*c).mastering_display,
                                                                            (*c).mastering_display_ref,
                                                                            (*c).itut_t35,
                                                                            (*c).itut_t35_ref,
                                                                            (*c).n_itut_t35 as size_t,
                                                                            &raw mut (*in_0).m,
                                                                        );
                                                                        dav1d_ref_dec(&raw mut (*c).itut_t35_ref);
                                                                        (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
                                                                        (*c).n_itut_t35 = 0 as core::ffi::c_int;
                                                                        pthread_mutex_unlock(&raw mut (*c).task_thread.lock);
                                                                    }
                                                                    if (*(*c)
                                                                        .refs[(*(*c).frame_hdr).existing_frame_idx as usize]
                                                                        .p
                                                                        .p
                                                                        .frame_hdr)
                                                                        .frame_type as core::ffi::c_uint
                                                                        == DAV1D_FRAME_TYPE_KEY as core::ffi::c_int
                                                                            as core::ffi::c_uint
                                                                    {
                                                                        let r: core::ffi::c_int = (*(*c).frame_hdr)
                                                                            .existing_frame_idx as core::ffi::c_int;
                                                                        (*c).refs[r as usize].p.showable = 0 as core::ffi::c_int;
                                                                        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                        while i_2 < 8 as core::ffi::c_int {
                                                                            if !(i_2 == r) {
                                                                                if !((*c).refs[i_2 as usize].p.p.frame_hdr).is_null() {
                                                                                    dav1d_thread_picture_unref(
                                                                                        &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).p,
                                                                                    );
                                                                                }
                                                                                dav1d_thread_picture_ref(
                                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).p,
                                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(r as isize)).p,
                                                                                );
                                                                                dav1d_cdf_thread_unref(
                                                                                    &raw mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                                );
                                                                                dav1d_cdf_thread_ref(
                                                                                    &raw mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                                    &raw mut *((*c).cdf).as_mut_ptr().offset(r as isize),
                                                                                );
                                                                                dav1d_ref_dec(
                                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).segmap,
                                                                                );
                                                                                (*c).refs[i_2 as usize].segmap = (*c)
                                                                                    .refs[r as usize]
                                                                                    .segmap;
                                                                                if !((*c).refs[r as usize].segmap).is_null() {
                                                                                    dav1d_ref_inc((*c).refs[r as usize].segmap);
                                                                                }
                                                                                dav1d_ref_dec(
                                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).refmvs,
                                                                                );
                                                                            }
                                                                            i_2 += 1;
                                                                        }
                                                                    }
                                                                    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                                    current_block = 32;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else if (*c).n_tiles
                                                    == (*(*c).frame_hdr).tiling.cols as core::ffi::c_int
                                                        * (*(*c).frame_hdr).tiling.rows as core::ffi::c_int
                                                {
                                                    match (*(*c).frame_hdr).frame_type as core::ffi::c_uint {
                                                        1 | 3 => {
                                                            if (*c).decode_frame_type as core::ffi::c_uint
                                                                > DAV1D_DECODEFRAMETYPE_REFERENCE as core::ffi::c_int
                                                                    as core::ffi::c_uint
                                                                || (*c).decode_frame_type as core::ffi::c_uint
                                                                    == DAV1D_DECODEFRAMETYPE_REFERENCE as core::ffi::c_int
                                                                        as core::ffi::c_uint
                                                                    && (*(*c).frame_hdr).refresh_frame_flags == 0
                                                            {
                                                                current_block = 30;
                                                            } else {
                                                                current_block = 33;
                                                            }
                                                        }
                                                        2 => {
                                                            if (*c).decode_frame_type as core::ffi::c_uint
                                                                > DAV1D_DECODEFRAMETYPE_INTRA as core::ffi::c_int
                                                                    as core::ffi::c_uint
                                                                || (*c).decode_frame_type as core::ffi::c_uint
                                                                    == DAV1D_DECODEFRAMETYPE_REFERENCE as core::ffi::c_int
                                                                        as core::ffi::c_uint
                                                                    && (*(*c).frame_hdr).refresh_frame_flags == 0
                                                            {
                                                                current_block = 30;
                                                            } else {
                                                                current_block = 33;
                                                            }
                                                        }
                                                        _ => {
                                                            current_block = 33;
                                                        }
                                                    }
                                                    match current_block {
                                                        30 => {}
                                                        _ => {
                                                            if (*c).n_tile_data == 0 {
                                                                current_block = 22;
                                                            } else {
                                                                res = dav1d_submit_frame(c);
                                                                if res < 0 as core::ffi::c_int {
                                                                    return res as ptrdiff_t;
                                                                }
                                                                if (*c).n_tile_data != 0 {
                                                                    unreachable!();
                                                                }
                                                                (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                                (*c).n_tiles = 0 as core::ffi::c_int;
                                                                current_block = 32;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    current_block = 32;
                                                }
                                                match current_block {
                                                    22 => {}
                                                    32 => {}
                                                    _ => {
                                                        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
                                                        while i_3 < 8 as core::ffi::c_int {
                                                            if (*(*c).frame_hdr).refresh_frame_flags as core::ffi::c_int
                                                                & (1 as core::ffi::c_int) << i_3 != 0
                                                            {
                                                                dav1d_thread_picture_unref(
                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).p,
                                                                );
                                                                (*c).refs[i_3 as usize].p.p.frame_hdr = (*c).frame_hdr;
                                                                (*c).refs[i_3 as usize].p.p.seq_hdr = (*c).seq_hdr;
                                                                (*c).refs[i_3 as usize].p.p.frame_hdr_ref = (*c)
                                                                    .frame_hdr_ref as *mut Dav1dRef;
                                                                (*c).refs[i_3 as usize].p.p.seq_hdr_ref = (*c).seq_hdr_ref
                                                                    as *mut Dav1dRef;
                                                                dav1d_ref_inc((*c).frame_hdr_ref);
                                                                dav1d_ref_inc((*c).seq_hdr_ref);
                                                            }
                                                            i_3 += 1;
                                                        }
                                                        dav1d_ref_dec(&raw mut (*c).frame_hdr_ref);
                                                        (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
                                                        (*c).n_tiles = 0 as core::ffi::c_int;
                                                        return (gb.ptr_end).offset_from(gb.ptr_start) as ptrdiff_t;
                                                    }
                                                }
                                            } else {
                                                current_block = 32;
                                            }
                                            match current_block {
                                                22 => {}
                                                _ => {
                                                    return (gb.ptr_end).offset_from(gb.ptr_start) as ptrdiff_t;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dav1d_data_props_copy(&raw mut (*c).cached_error_props, &raw mut (*in_0).m);
    dav1d_log(
        c,
        if gb.error != 0 {
            b"Overrun in OBU bit buffer\n\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"Error parsing OBU data\n\0" as *const u8 as *const core::ffi::c_char
        },
    );
    return -(22 as core::ffi::c_int) as ptrdiff_t;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: core::ffi::c_int = -__INT_MAX__ - 1 as core::ffi::c_int;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
