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

// Mechanically translated from upstream/dav1d/src/lib.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type __sFILEX;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn posix_memalign(
        __memptr: *mut *mut core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> core::ffi::c_int;
    fn abort() -> !;
    fn dav1d_init_cpu();
    fn dav1d_num_logical_processors(c: *mut Dav1dContext) -> core::ffi::c_int;
    fn dav1d_apply_grain_16bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
    );
    fn dav1d_apply_grain_8bpc(
        dsp: *const Dav1dFilmGrainDSPContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
    );
    fn pthread_attr_destroy(_: *mut pthread_attr_t) -> core::ffi::c_int;
    fn pthread_attr_init(_: *mut pthread_attr_t) -> core::ffi::c_int;
    fn pthread_attr_setstacksize(_: *mut pthread_attr_t, _: size_t) -> core::ffi::c_int;
    fn pthread_cond_broadcast(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_init(
        _: *mut pthread_cond_t,
        _: *const pthread_condattr_t,
    ) -> core::ffi::c_int;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<
            unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
        >,
        _: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn pthread_join(_: pthread_t, _: *mut *mut core::ffi::c_void) -> core::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> core::ffi::c_int;
    fn dav1d_mem_pool_init(pool: *mut *mut Dav1dMemPool) -> core::ffi::c_int;
    fn dav1d_mem_pool_end(pool: *mut Dav1dMemPool);
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_data_ref(dst: *mut Dav1dData, src: *const Dav1dData);
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_props_set_defaults(props: *mut Dav1dDataProps);
    fn dav1d_data_create_internal(buf: *mut Dav1dData, sz: size_t) -> *mut uint8_t;
    fn dav1d_data_wrap_internal(
        buf: *mut Dav1dData,
        ptr: *const uint8_t,
        sz: size_t,
        free_callback: Option<
            unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
        >,
        user_data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn dav1d_data_wrap_user_data_internal(
        buf: *mut Dav1dData,
        user_data: *const uint8_t,
        free_callback: Option<
            unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
        >,
        cookie: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    fn dav1d_data_props_unref_internal(props: *mut Dav1dDataProps);
    fn dav1d_init_intra_edge_tree();
    fn dav1d_refmvs_dsp_init(dsp: *mut Dav1dRefmvsDSPContext);
    fn dav1d_pal_dsp_init(dsp: *mut Dav1dPalDSPContext);
    fn dav1d_picture_alloc_copy(
        c: *mut Dav1dContext,
        dst: *mut Dav1dPicture,
        w: core::ffi::c_int,
        src: *const Dav1dPicture,
    ) -> core::ffi::c_int;
    fn dav1d_picture_ref(dst: *mut Dav1dPicture, src: *const Dav1dPicture);
    fn dav1d_thread_picture_ref(
        dst: *mut Dav1dThreadPicture,
        src: *const Dav1dThreadPicture,
    );
    fn dav1d_thread_picture_move_ref(
        dst: *mut Dav1dThreadPicture,
        src: *mut Dav1dThreadPicture,
    );
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_picture_move_ref(dst: *mut Dav1dPicture, src: *mut Dav1dPicture);
    fn dav1d_default_picture_alloc(
        p: *mut Dav1dPicture,
        cookie: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn dav1d_default_picture_release(
        p: *mut Dav1dPicture,
        cookie: *mut core::ffi::c_void,
    );
    fn dav1d_picture_unref_internal(p: *mut Dav1dPicture);
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    fn dav1d_log_default_callback(
        cookie: *mut core::ffi::c_void,
        format: *const core::ffi::c_char,
        ap: va_list,
    );
    fn dav1d_log(c: *mut Dav1dContext, format: *const core::ffi::c_char, ...);
    fn dav1d_parse_obus(c: *mut Dav1dContext, in_0: *mut Dav1dData) -> ptrdiff_t;
    fn dav1d_init_qm_tables();
    fn dav1d_task_delayed_fg(
        c: *mut Dav1dContext,
        out: *mut Dav1dPicture,
        in_0: *const Dav1dPicture,
    );
    fn dav1d_worker_task(data: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn dav1d_decode_frame_exit(f: *mut Dav1dFrameContext, retval: core::ffi::c_int);
    fn dav1d_init_ii_wedge_masks();
}
pub type __builtin_va_list = *mut core::ffi::c_char;
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> ()>,
    pub __arg: *mut core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_once_t {
    pub __sig: core::ffi::c_long,
    pub __opaque: [core::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [core::ffi::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type size_t = __darwin_size_t;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type va_list<'a> = core::ffi::VaList<'a>;
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
pub struct Dav1dData {
    pub data: *const uint8_t,
    pub sz: size_t,
    pub ref_0: *mut Dav1dRef,
    pub m: Dav1dDataProps,
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
    pub frame_thread: C2RustUnnamed_20,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [C2RustUnnamed_15; 8],
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
pub type pthread_mutex_t = libc::pthread_mutex_t;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub ref_0: uint8_t,
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
    pub sgr: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}
pub type pixel = uint8_t;
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
pub type coef = ();
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
    pub data: C2RustUnnamed_14,
    pub progress: *mut atomic_uint,
}
pub type atomic_uint = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub cdf: *mut CdfContext,
    pub qcat: core::ffi::c_uint,
}
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
pub struct C2RustUnnamed_15 {
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
pub struct TaskThreadData {
    pub lock: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub first: atomic_uint,
    pub cur: core::ffi::c_uint,
    pub reset_task_cur: atomic_uint,
    pub cond_signaled: atomic_int,
    pub delayed_fg: C2RustUnnamed_16,
    pub inited: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub exec: core::ffi::c_int,
    pub finished: core::ffi::c_int,
    pub cond: pthread_cond_t,
    pub in_0: *const Dav1dPicture,
    pub out: *mut Dav1dPicture,
    pub type_0: TaskType,
    pub progress: [atomic_int; 2],
    pub c2rust_unnamed: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub c2rust_unnamed: C2RustUnnamed_19,
    pub c2rust_unnamed_0: C2RustUnnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub __dav1d_align_grain_lut_16bpc: [crate::platform::Align16; 0],

    pub grain_lut_16bpc: [[[int16_t; 82]; 74]; 3],
    pub __dav1d_align_scaling_16bpc: [crate::platform::Align64; 0],
    pub scaling_16bpc: [[uint8_t; 4096]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub __dav1d_align_grain_lut_8bpc: [crate::platform::Align16; 0],

    pub grain_lut_8bpc: [[[int8_t; 82]; 74]; 3],
    pub __dav1d_align_scaling_8bpc: [crate::platform::Align64; 0],
    pub scaling_8bpc: [[uint8_t; 256]; 3],
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
pub type pthread_cond_t = libc::pthread_cond_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
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
    pub c2rust_unnamed: C2RustUnnamed_36,
    pub c2rust_unnamed_0: C2RustUnnamed_35,
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub scratch: C2RustUnnamed_23,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: core::ffi::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: C2RustUnnamed_22,
    pub task_thread: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub td: thread_data,
    pub ttd: *mut TaskThreadData,
    pub fttd: *mut FrameTileThreadData,
    pub flushed: core::ffi::c_int,
    pub die: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FrameTileThreadData {
    pub lowest_pixel_mem: *mut [[core::ffi::c_int; 2]; 7],
    pub lowest_pixel_mem_sz: core::ffi::c_int,
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
pub struct C2RustUnnamed_22 {
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
#[repr(C)]
pub struct Av1Filter {
    pub filter_y: [[[[uint16_t; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[uint16_t; 2]; 2]; 32]; 2],
    pub cdef_idx: [int8_t; 4],
    pub noskip_mask: [[uint16_t; 2]; 16],
}
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union C2RustUnnamed_23 {
    pub c2rust_unnamed: C2RustUnnamed_31,
    pub c2rust_unnamed_0: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub c2rust_unnamed: C2RustUnnamed_29,
    pub c2rust_unnamed_0: C2RustUnnamed_28,
    pub pal_idx_y: [uint8_t; 2048],
    pub pal_idx_uv: [uint8_t; 4096],
    pub c2rust_unnamed_1: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub c2rust_unnamed: C2RustUnnamed_27,
    pub c2rust_unnamed_0: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub interintra_16bpc: [uint16_t; 4096],
    pub edge_16bpc: [uint16_t; 257],
    pub __dav1d_align_pal_16bpc: [crate::platform::Align16; 0],
    pub pal_16bpc: [[uint16_t; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub interintra_8bpc: [uint8_t; 4096],
    pub edge_8bpc: [uint8_t; 257],
    pub __dav1d_align_pal_8bpc: [crate::platform::Align8; 0],
    pub pal_8bpc: [[uint8_t; 8]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_28 {
    pub ac: [int16_t; 1024],
    pub txtp_map: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_29 {
    pub levels: [uint8_t; 1088],
    pub c2rust_unnamed: C2RustUnnamed_30,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub pal_order: [[uint8_t; 8]; 64],
    pub pal_ctx: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub c2rust_unnamed: C2RustUnnamed_33,
    pub c2rust_unnamed_0: C2RustUnnamed_32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub emu_edge_8bpc: [uint8_t; 84160],
    pub emu_edge_16bpc: [uint16_t; 84160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub lap_8bpc: [uint8_t; 4096],
    pub lap_16bpc: [uint16_t; 4096],
    pub c2rust_unnamed: C2RustUnnamed_34,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub compinter: [[int16_t; 16384]; 2],
    pub seg_mask: [uint8_t; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
    pub al_pal_8bpc: [[[[uint8_t; 8]; 3]; 32]; 2],
    pub al_pal_16bpc: [[[[uint16_t; 8]; 3]; 32]; 2],
}
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub union C2RustUnnamed_36 {
    pub cf_8bpc: [int16_t; 1024],
    pub cf_16bpc: [int32_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: C2RustUnnamed_37,
    pub tile_row: C2RustUnnamed_37,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub start: core::ffi::c_int,
    pub end: core::ffi::c_int,
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
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: C2RustUnnamed_40,
    pub progress: [atomic_int; 2],
    pub frame_thread: [C2RustUnnamed_39; 2],
    pub lowest_pixel: *mut [[core::ffi::c_int; 2]; 7],
    pub dqmem: [[[uint16_t; 2]; 3]; 8],
    pub dq: *const [[uint16_t; 2]; 3],
    pub last_qidx: core::ffi::c_int,
    pub last_delta_lf: C2RustUnnamed_38,
    pub __dav1d_align_lflvlmem: [crate::platform::Align16; 0],
    pub lflvlmem: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[uint8_t; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
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
pub union C2RustUnnamed_38 {
    pub i8_0: [int8_t; 4],
    pub u32_0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub pal_idx: *mut uint8_t,
    pub cbi: *mut int16_t,
    pub cf: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
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
    pub bd_fn: C2RustUnnamed_51,
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
    pub frame_thread: C2RustUnnamed_44,
    pub lf: C2RustUnnamed_43,
    pub task_thread: C2RustUnnamed_41,
    pub tile_thread: FrameTileThreadData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
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
    pub pending_tasks: C2RustUnnamed_42,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
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
pub struct Av1Restoration {
    pub lr: [[Av1RestorationUnit; 4]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_44 {
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
    pub c2rust_unnamed: C2RustUnnamed_45,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_45 {
    pub c2rust_unnamed: C2RustUnnamed_50,
    pub c2rust_unnamed_0: C2RustUnnamed_46,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_46 {
    pub c2rust_unnamed: C2RustUnnamed_47,
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
pub union C2RustUnnamed_47 {
    pub c2rust_unnamed: C2RustUnnamed_49,
    pub c2rust_unnamed_0: C2RustUnnamed_48,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_48 {
    pub mv2d: mv,
    pub matrix: [int16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_49 {
    pub mv: [mv; 2],
    pub wedge_idx: uint8_t,
    pub mask_sign: uint8_t,
    pub interintra_mode: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_50 {
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
pub struct C2RustUnnamed_51 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSettings {
    pub n_threads: core::ffi::c_int,
    pub max_frame_delay: core::ffi::c_int,
    pub apply_grain: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub all_layers: core::ffi::c_int,
    pub frame_size_limit: core::ffi::c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: core::ffi::c_int,
    pub output_invisible_frames: core::ffi::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub reserved: [uint8_t; 16],
}
pub type pthread_attr_t = libc::pthread_attr_t;
pub type pthread_condattr_t = libc::pthread_condattr_t;
pub type pthread_mutexattr_t = libc::pthread_mutexattr_t;
pub type FILE = libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut core::ffi::c_uchar,
    pub _r: core::ffi::c_int,
    pub _w: core::ffi::c_int,
    pub _flags: core::ffi::c_short,
    pub _file: core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: core::ffi::c_int,
    pub _cookie: *mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut core::ffi::c_void, fpos_t, core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: core::ffi::c_int,
    pub _ubuf: [core::ffi::c_uchar; 3],
    pub _nbuf: [core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut core::ffi::c_uchar,
    pub _size: core::ffi::c_int,
}
pub type pthread_once_t = libc::pthread_once_t;
pub const memory_order_relaxed: memory_order = 0;
pub type memory_order = core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const __ATOMIC_SEQ_CST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const DAV1D_VERSION: [core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [core::ffi::c_char; 6]>(*b"1.5.4\0")
};
pub const DAV1D_API_VERSION_MAJOR: core::ffi::c_int = 7 as core::ffi::c_int;
pub const DAV1D_API_VERSION_MINOR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const DAV1D_API_VERSION_PATCH: core::ffi::c_int = 0 as core::ffi::c_int;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const DAV1D_MAX_THREADS: core::ffi::c_int = 256 as core::ffi::c_int;
pub const _PTHREAD_ONCE_SIG_init: core::ffi::c_int = 0x30b1bcba as core::ffi::c_int;
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
#[inline]
unsafe extern "C" fn dav1d_alloc_aligned_internal(
    sz: size_t,
    align: size_t,
) -> *mut core::ffi::c_void {
    if (align & align.wrapping_sub(1 as size_t) != 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 29],
                [core::ffi::c_char; 29],
            >(*b"dav1d_alloc_aligned_internal\0"))
                .as_ptr(),
            b"mem.h\0" as *const u8 as *const core::ffi::c_char,
            90 as core::ffi::c_int,
            b"!(align & (align - 1))\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
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
unsafe extern "C" fn dav1d_freep_aligned(mut ptr: *mut core::ffi::c_void) {
    let mut mem: *mut *mut core::ffi::c_void = ptr as *mut *mut core::ffi::c_void;
    if !(*mem).is_null() {
        dav1d_free_aligned_internal(*mem);
        *mem = NULL;
    }
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
pub const FRAME_ERROR: core::ffi::c_uint = UINT_MAX.wrapping_sub(1 as core::ffi::c_uint);
#[cold]
unsafe extern "C" fn init_internal() {
    dav1d_init_cpu();
    dav1d_init_ii_wedge_masks();
    dav1d_init_intra_edge_tree();
    dav1d_init_qm_tables();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_version() -> *const core::ffi::c_char {
    return DAV1D_VERSION.as_ptr();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_version_api() -> core::ffi::c_uint {
    return (DAV1D_API_VERSION_MAJOR << 16 as core::ffi::c_int
        | DAV1D_API_VERSION_MINOR << 8 as core::ffi::c_int
        | DAV1D_API_VERSION_PATCH << 0 as core::ffi::c_int) as core::ffi::c_uint;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: *mut Dav1dSettings) {
    (*s).n_threads = 0 as core::ffi::c_int;
    (*s).max_frame_delay = 0 as core::ffi::c_int;
    (*s).apply_grain = 1 as core::ffi::c_int;
    (*s).allocator.cookie = NULL;
    (*s).allocator.alloc_picture_callback = Some(
        dav1d_default_picture_alloc
            as unsafe extern "C" fn(
                *mut Dav1dPicture,
                *mut core::ffi::c_void,
            ) -> core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut Dav1dPicture,
                *mut core::ffi::c_void,
            ) -> core::ffi::c_int,
        >;
    (*s).allocator.release_picture_callback = Some(
        dav1d_default_picture_release
            as unsafe extern "C" fn(*mut Dav1dPicture, *mut core::ffi::c_void) -> (),
    ) as Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut core::ffi::c_void) -> ()>;
    (*s).logger.cookie = NULL;
    (*s).logger.callback = Some(
        dav1d_log_default_callback
            as unsafe extern "C" fn(
                *mut core::ffi::c_void,
                *const core::ffi::c_char,
                va_list,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut core::ffi::c_void,
                *const core::ffi::c_char,
                va_list,
            ) -> (),
        >;
    (*s).operating_point = 0 as core::ffi::c_int;
    (*s).all_layers = 1 as core::ffi::c_int;
    (*s).frame_size_limit = 0 as core::ffi::c_uint;
    (*s).strict_std_compliance = 0 as core::ffi::c_int;
    (*s).output_invisible_frames = 0 as core::ffi::c_int;
    (*s).inloop_filters = DAV1D_INLOOPFILTER_ALL;
    (*s).decode_frame_type = DAV1D_DECODEFRAMETYPE_ALL;
}
#[cold]
unsafe extern "C" fn get_num_threads(
    c: *mut Dav1dContext,
    s: *const Dav1dSettings,
    mut n_tc: *mut core::ffi::c_uint,
    mut n_fc: *mut core::ffi::c_uint,
) {
    static mut fc_lut: [uint8_t; 49] = [
        1 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
    ];
    *n_tc = (if (*s).n_threads != 0 {
        (*s).n_threads
    } else {
        iclip(dav1d_num_logical_processors(c), 1 as core::ffi::c_int, DAV1D_MAX_THREADS)
    }) as core::ffi::c_uint;
    *n_fc = if (*s).max_frame_delay != 0 {
        umin((*s).max_frame_delay as core::ffi::c_uint, *n_tc)
    } else {
        (if *n_tc < 50 as core::ffi::c_uint {
            fc_lut[(*n_tc).wrapping_sub(1 as core::ffi::c_uint) as usize]
                as core::ffi::c_int
        } else {
            8 as core::ffi::c_int
        }) as core::ffi::c_uint
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_frame_delay(
    s: *const Dav1dSettings,
) -> core::ffi::c_int {
    let mut n_tc: core::ffi::c_uint = 0;
    let mut n_fc: core::ffi::c_uint = 0;
    if s.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*s).n_threads >= 0 as core::ffi::c_int
        && (*s).n_threads <= 256 as core::ffi::c_int)
    {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->n_threads >= 0 && s->n_threads <= DAV1D_MAX_THREADS\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*s).max_frame_delay >= 0 as core::ffi::c_int
        && (*s).max_frame_delay <= 256 as core::ffi::c_int)
    {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->max_frame_delay >= 0 && s->max_frame_delay <= DAV1D_MAX_FRAME_DELAY\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        abort();
    }
    get_num_threads(0 as *mut Dav1dContext, s, &raw mut n_tc, &raw mut n_fc);
    return n_fc as core::ffi::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_open(
    c_out: *mut *mut Dav1dContext,
    s: *const Dav1dSettings,
) -> core::ffi::c_int {
    let mut current_block: u64;
    static mut initted: pthread_once_t = libc::PTHREAD_ONCE_INIT;
    pthread_once(&raw mut initted, Some(init_internal as unsafe extern "C" fn() -> ()));
    if c_out.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c_out != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if s.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*s).n_threads >= 0 as core::ffi::c_int
        && (*s).n_threads <= 256 as core::ffi::c_int)
    {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->n_threads >= 0 && s->n_threads <= DAV1D_MAX_THREADS\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*s).max_frame_delay >= 0 as core::ffi::c_int
        && (*s).max_frame_delay <= 256 as core::ffi::c_int)
    {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->max_frame_delay >= 0 && s->max_frame_delay <= DAV1D_MAX_FRAME_DELAY\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if ((*s).allocator.alloc_picture_callback).is_none() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->allocator.alloc_picture_callback != NULL\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if ((*s).allocator.release_picture_callback).is_none() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->allocator.release_picture_callback != NULL\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*s).operating_point >= 0 as core::ffi::c_int
        && (*s).operating_point <= 31 as core::ffi::c_int)
    {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->operating_point >= 0 && s->operating_point <= 31\0" as *const u8
                as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*s).decode_frame_type as core::ffi::c_uint
        >= DAV1D_DECODEFRAMETYPE_ALL as core::ffi::c_int as core::ffi::c_uint
        && (*s).decode_frame_type as core::ffi::c_uint
            <= DAV1D_DECODEFRAMETYPE_KEY as core::ffi::c_int as core::ffi::c_uint)
    {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"s->decode_frame_type >= DAV1D_DECODEFRAMETYPE_ALL && s->decode_frame_type <= DAV1D_DECODEFRAMETYPE_KEY\0"
                as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 11],
                [core::ffi::c_char; 11],
            >(*b"dav1d_open\0"))
                .as_ptr(),
        );
        abort();
    }
    let mut thread_attr: pthread_attr_t = core::mem::zeroed();
    if pthread_attr_init(&raw mut thread_attr) != 0 {
        return -(12 as core::ffi::c_int);
    }
    let mut stack_size: size_t = (1024 as core::ffi::c_int * 1024 as core::ffi::c_int
        + 0 as core::ffi::c_int) as size_t;
    pthread_attr_setstacksize(&raw mut thread_attr, stack_size);
    *c_out = dav1d_alloc_aligned_internal(
        ::core::mem::size_of::<Dav1dContext>() as size_t,
        64 as size_t,
    ) as *mut Dav1dContext;
    let c: *mut Dav1dContext = *c_out;
    if !c.is_null() {
        memset(
            c as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<Dav1dContext>() as size_t,
        );
        (*c).allocator = (*s).allocator;
        (*c).logger = (*s).logger;
        (*c).apply_grain = (*s).apply_grain;
        (*c).operating_point = (*s).operating_point;
        (*c).all_layers = (*s).all_layers;
        (*c).frame_size_limit = (*s).frame_size_limit;
        (*c).strict_std_compliance = (*s).strict_std_compliance;
        (*c).output_invisible_frames = (*s).output_invisible_frames;
        (*c).inloop_filters = (*s).inloop_filters;
        (*c).decode_frame_type = (*s).decode_frame_type;
        dav1d_data_props_set_defaults(&raw mut (*c).cached_error_props);
        if !(dav1d_mem_pool_init(&raw mut (*c).seq_hdr_pool) != 0
            || dav1d_mem_pool_init(&raw mut (*c).frame_hdr_pool) != 0
            || dav1d_mem_pool_init(&raw mut (*c).segmap_pool) != 0
            || dav1d_mem_pool_init(&raw mut (*c).refmvs_pool) != 0
            || dav1d_mem_pool_init(&raw mut (*c).pic_ctx_pool) != 0
            || dav1d_mem_pool_init(&raw mut (*c).cdf_pool) != 0)
        {
            if (*c).allocator.alloc_picture_callback
                == Some(
                    dav1d_default_picture_alloc
                        as unsafe extern "C" fn(
                            *mut Dav1dPicture,
                            *mut core::ffi::c_void,
                        ) -> core::ffi::c_int,
                )
                && (*c).allocator.release_picture_callback
                    == Some(
                        dav1d_default_picture_release
                            as unsafe extern "C" fn(
                                *mut Dav1dPicture,
                                *mut core::ffi::c_void,
                            ) -> (),
                    )
            {
                if !((*c).allocator.cookie).is_null() {
                    current_block = 0;
                } else if dav1d_mem_pool_init(&raw mut (*c).picture_pool) != 0 {
                    current_block = 0;
                } else {
                    (*c).allocator.cookie = (*c).picture_pool as *mut core::ffi::c_void;
                    current_block = 1;
                }
            } else if (*c).allocator.alloc_picture_callback
                == Some(
                    dav1d_default_picture_alloc
                        as unsafe extern "C" fn(
                            *mut Dav1dPicture,
                            *mut core::ffi::c_void,
                        ) -> core::ffi::c_int,
                )
                || (*c).allocator.release_picture_callback
                    == Some(
                        dav1d_default_picture_release
                            as unsafe extern "C" fn(
                                *mut Dav1dPicture,
                                *mut core::ffi::c_void,
                            ) -> (),
                    )
            {
                current_block = 0;
            } else {
                current_block = 1;
            }
            match current_block {
                0 => {}
                _ => {
                    if (::core::mem::size_of::<size_t>() as usize) < 8 as usize
                        && ((*s).frame_size_limit).wrapping_sub(1 as core::ffi::c_uint)
                            >= (8192 as core::ffi::c_int * 8192 as core::ffi::c_int)
                                as core::ffi::c_uint
                    {
                        (*c).frame_size_limit = (8192 as core::ffi::c_int
                            * 8192 as core::ffi::c_int) as core::ffi::c_uint;
                        if (*s).frame_size_limit != 0 {
                            dav1d_log(
                                c,
                                b"Frame size limit reduced from %u to %u.\n\0" as *const u8
                                    as *const core::ffi::c_char,
                                (*s).frame_size_limit,
                                (*c).frame_size_limit,
                            );
                        }
                    }
                    (*c).flush = &raw mut (*c).flush_mem;
                    *(*c).flush = 0 as core::ffi::c_int;
                    get_num_threads(c, s, &raw mut (*c).n_tc, &raw mut (*c).n_fc);
                    (*c).fc = dav1d_alloc_aligned_internal(
                        (::core::mem::size_of::<Dav1dFrameContext>() as size_t)
                            .wrapping_mul((*c).n_fc as size_t),
                        32 as size_t,
                    ) as *mut Dav1dFrameContext;
                    if !((*c).fc).is_null() {
                        memset(
                            (*c).fc as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            (::core::mem::size_of::<Dav1dFrameContext>() as size_t)
                                .wrapping_mul((*c).n_fc as size_t),
                        );
                        (*c).tc = dav1d_alloc_aligned_internal(
                            (::core::mem::size_of::<Dav1dTaskContext>() as size_t)
                                .wrapping_mul((*c).n_tc as size_t),
                            64 as size_t,
                        ) as *mut Dav1dTaskContext;
                        if !((*c).tc).is_null() {
                            memset(
                                (*c).tc as *mut core::ffi::c_void,
                                0 as core::ffi::c_int,
                                (::core::mem::size_of::<Dav1dTaskContext>() as size_t)
                                    .wrapping_mul((*c).n_tc as size_t),
                            );
                            if (*c).n_tc > 1 as core::ffi::c_uint {
                                if pthread_mutex_init(
                                    &raw mut (*c).task_thread.lock,
                                    0 as *const pthread_mutexattr_t,
                                ) != 0
                                {
                                    current_block = 0;
                                } else if pthread_cond_init(
                                    &raw mut (*c).task_thread.cond,
                                    0 as *const pthread_condattr_t,
                                ) != 0
                                {
                                    pthread_mutex_destroy(&raw mut (*c).task_thread.lock);
                                    current_block = 0;
                                } else if pthread_cond_init(
                                    &raw mut (*c).task_thread.delayed_fg.cond,
                                    0 as *const pthread_condattr_t,
                                ) != 0
                                {
                                    pthread_cond_destroy(&raw mut (*c).task_thread.cond);
                                    pthread_mutex_destroy(&raw mut (*c).task_thread.lock);
                                    current_block = 0;
                                } else {
                                    (*c).task_thread.cur = (*c).n_fc;
                                    *&raw mut (*c).task_thread.reset_task_cur = UINT_MAX;
                                    *&raw mut (*c).task_thread.cond_signaled = 0
                                        as core::ffi::c_int;
                                    (*c).task_thread.inited = 1 as core::ffi::c_int;
                                    current_block = 2;
                                }
                            } else {
                                current_block = 2;
                            }
                            match current_block {
                                0 => {}
                                _ => {
                                    if (*c).n_fc > 1 as core::ffi::c_uint {
                                        let out_delayed_sz: size_t = (::core::mem::size_of::<
                                            Dav1dThreadPicture,
                                        >() as size_t)
                                            .wrapping_mul((*c).n_fc as size_t);
                                        (*c).frame_thread.out_delayed = malloc(out_delayed_sz)
                                            as *mut Dav1dThreadPicture;
                                        if ((*c).frame_thread.out_delayed).is_null() {
                                            current_block = 0;
                                        } else {
                                            memset(
                                                (*c).frame_thread.out_delayed as *mut core::ffi::c_void,
                                                0 as core::ffi::c_int,
                                                out_delayed_sz,
                                            );
                                            current_block = 3;
                                        }
                                    } else {
                                        current_block = 3;
                                    }
                                    match current_block {
                                        0 => {}
                                        _ => {
                                            let mut n: core::ffi::c_uint = 0 as core::ffi::c_uint;
                                            loop {
                                                if !(n < (*c).n_fc) {
                                                    current_block = 4;
                                                    break;
                                                }
                                                let f: *mut Dav1dFrameContext = &raw mut *((*c).fc)
                                                    .offset(n as isize) as *mut Dav1dFrameContext;
                                                if (*c).n_tc > 1 as core::ffi::c_uint {
                                                    if pthread_mutex_init(
                                                        &raw mut (*f).task_thread.lock,
                                                        0 as *const pthread_mutexattr_t,
                                                    ) != 0
                                                    {
                                                        current_block = 0;
                                                        break;
                                                    }
                                                    if pthread_cond_init(
                                                        &raw mut (*f).task_thread.cond,
                                                        0 as *const pthread_condattr_t,
                                                    ) != 0
                                                    {
                                                        pthread_mutex_destroy(&raw mut (*f).task_thread.lock);
                                                        current_block = 0;
                                                        break;
                                                    } else if pthread_mutex_init(
                                                        &raw mut (*f).task_thread.pending_tasks.lock,
                                                        0 as *const pthread_mutexattr_t,
                                                    ) != 0
                                                    {
                                                        pthread_cond_destroy(&raw mut (*f).task_thread.cond);
                                                        pthread_mutex_destroy(&raw mut (*f).task_thread.lock);
                                                        current_block = 0;
                                                        break;
                                                    }
                                                }
                                                (*f).c = c;
                                                (*f).task_thread.ttd = &raw mut (*c).task_thread
                                                    as *mut TaskThreadData;
                                                (*f).lf.last_sharpness = -(1 as core::ffi::c_int);
                                                n = n.wrapping_add(1);
                                            }
                                            match current_block {
                                                0 => {}
                                                _ => {
                                                    let mut m: core::ffi::c_uint = 0 as core::ffi::c_uint;
                                                    loop {
                                                        if !(m < (*c).n_tc) {
                                                            current_block = 5;
                                                            break;
                                                        }
                                                        let t: *mut Dav1dTaskContext = &raw mut *((*c).tc)
                                                            .offset(m as isize) as *mut Dav1dTaskContext;
                                                        (*t).f = &raw mut *((*c).fc)
                                                            .offset(0 as core::ffi::c_int as isize)
                                                            as *mut Dav1dFrameContext;
                                                        (*t).task_thread.ttd = &raw mut (*c).task_thread
                                                            as *mut TaskThreadData;
                                                        (*t).c = c;
                                                        memset(
                                                            ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr()
                                                                as *mut core::ffi::c_void,
                                                            0 as core::ffi::c_int,
                                                            ::core::mem::size_of::<[int32_t; 1024]>() as size_t,
                                                        );
                                                        if (*c).n_tc > 1 as core::ffi::c_uint {
                                                            if pthread_mutex_init(
                                                                &raw mut (*t).task_thread.td.lock,
                                                                0 as *const pthread_mutexattr_t,
                                                            ) != 0
                                                            {
                                                                current_block = 0;
                                                                break;
                                                            }
                                                            if pthread_cond_init(
                                                                &raw mut (*t).task_thread.td.cond,
                                                                0 as *const pthread_condattr_t,
                                                            ) != 0
                                                            {
                                                                pthread_mutex_destroy(&raw mut (*t).task_thread.td.lock);
                                                                current_block = 0;
                                                                break;
                                                            } else if pthread_create(
                                                                &raw mut (*t).task_thread.td.thread,
                                                                &raw mut thread_attr,
                                                                Some(
                                                                    dav1d_worker_task
                                                                        as unsafe extern "C" fn(
                                                                            *mut core::ffi::c_void,
                                                                        ) -> *mut core::ffi::c_void,
                                                                ),
                                                                t as *mut core::ffi::c_void,
                                                            ) != 0
                                                            {
                                                                pthread_cond_destroy(&raw mut (*t).task_thread.td.cond);
                                                                pthread_mutex_destroy(&raw mut (*t).task_thread.td.lock);
                                                                current_block = 0;
                                                                break;
                                                            } else {
                                                                (*t).task_thread.td.inited = 1 as core::ffi::c_int;
                                                            }
                                                        }
                                                        m = m.wrapping_add(1);
                                                    }
                                                    match current_block {
                                                        0 => {}
                                                        _ => {
                                                            dav1d_pal_dsp_init(&raw mut (*c).pal_dsp);
                                                            dav1d_refmvs_dsp_init(&raw mut (*c).refmvs_dsp);
                                                            pthread_attr_destroy(&raw mut thread_attr);
                                                            return 0 as core::ffi::c_int;
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
    if !c.is_null() {
        close_internal(c_out, 0 as core::ffi::c_int);
    }
    pthread_attr_destroy(&raw mut thread_attr);
    return -(12 as core::ffi::c_int);
}
unsafe extern "C" fn has_grain(pic: *const Dav1dPicture) -> core::ffi::c_int {
    let mut fgdata: *const Dav1dFilmGrainData = &raw mut (*(*pic).frame_hdr).film_grain.data;
    return ((*fgdata).num_y_points != 0
        || (*fgdata).num_uv_points[0 as core::ffi::c_int as usize] != 0
        || (*fgdata).num_uv_points[1 as core::ffi::c_int as usize] != 0
        || (*fgdata).clip_to_restricted_range != 0
            && (*fgdata).chroma_scaling_from_luma != 0) as core::ffi::c_int;
}
unsafe extern "C" fn output_image(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
) -> core::ffi::c_int {
    let mut res: core::ffi::c_int = 0 as core::ffi::c_int;
    let in_0: *mut Dav1dThreadPicture = if (*c).all_layers != 0
        || (*c).max_spatial_id == 0
    {
        &raw mut (*c).out
    } else {
        &raw mut (*c).cache
    };
    if (*c).apply_grain == 0 || has_grain(&raw mut (*in_0).p) == 0 {
        dav1d_picture_move_ref(out, &raw mut (*in_0).p);
        dav1d_thread_picture_unref(in_0);
    } else {
        res = dav1d_apply_grain(c, out, &raw mut (*in_0).p);
        dav1d_thread_picture_unref(in_0);
    }
    if (*c).all_layers == 0 && (*c).max_spatial_id != 0
        && !((*c).out.p.data[0 as core::ffi::c_int as usize]).is_null()
    {
        dav1d_thread_picture_move_ref(in_0, &raw mut (*c).out);
    }
    return res;
}
unsafe extern "C" fn output_picture_ready(
    c: *mut Dav1dContext,
    drain: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*c).cached_error != 0 {
        return 1 as core::ffi::c_int;
    }
    if (*c).all_layers == 0 && (*c).max_spatial_id != 0 {
        if !((*c).out.p.data[0 as core::ffi::c_int as usize]).is_null()
            && !((*c).cache.p.data[0 as core::ffi::c_int as usize]).is_null()
        {
            if (*c).max_spatial_id
                == (*(*c).cache.p.frame_hdr).spatial_id as core::ffi::c_int
                || (*c).out.flags as core::ffi::c_uint
                    & PICTURE_FLAG_NEW_TEMPORAL_UNIT as core::ffi::c_int
                        as core::ffi::c_uint != 0
            {
                return 1 as core::ffi::c_int;
            }
            dav1d_thread_picture_unref(&raw mut (*c).cache);
            dav1d_thread_picture_move_ref(&raw mut (*c).cache, &raw mut (*c).out);
            return 0 as core::ffi::c_int;
        } else if !((*c).cache.p.data[0 as core::ffi::c_int as usize]).is_null()
            && drain != 0
        {
            return 1 as core::ffi::c_int
        } else if !((*c).out.p.data[0 as core::ffi::c_int as usize]).is_null() {
            dav1d_thread_picture_move_ref(&raw mut (*c).cache, &raw mut (*c).out);
            return 0 as core::ffi::c_int;
        }
    }
    return !((*c).out.p.data[0 as core::ffi::c_int as usize]).is_null()
        as core::ffi::c_int;
}
unsafe extern "C" fn drain_picture(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
) -> core::ffi::c_int {
    let mut drain_count: core::ffi::c_uint = 0 as core::ffi::c_uint;
    let mut drained: core::ffi::c_int = 0 as core::ffi::c_int;
    loop {
        let next: core::ffi::c_uint = (*c).frame_thread.next;
        let f: *mut Dav1dFrameContext = &raw mut *((*c).fc).offset(next as isize)
            as *mut Dav1dFrameContext;
        pthread_mutex_lock(&raw mut (*c).task_thread.lock);
        while (*f).n_tile_data > 0 as core::ffi::c_int {
            pthread_cond_wait(
                &raw mut (*f).task_thread.cond,
                &raw mut (*(*f).task_thread.ttd).lock,
            );
        }
        let out_delayed: *mut Dav1dThreadPicture = &raw mut *((*c).frame_thread.out_delayed)
            .offset(next as isize) as *mut Dav1dThreadPicture;
        if !((*out_delayed).p.data[0 as core::ffi::c_int as usize]).is_null()
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
            let fresh5 = ::core::intrinsics::atomic_cxchg::<_, {core::intrinsics::AtomicOrdering::SeqCst}, {core::intrinsics::AtomicOrdering::SeqCst}>(
                &raw mut (*c).task_thread.reset_task_cur,
                *&raw mut first,
                (2147483647 as core::ffi::c_uint)
                    .wrapping_mul(2 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint),
            );
            *&raw mut first = fresh5.0;
            fresh5.1;
            if (*c).task_thread.cur != 0 && (*c).task_thread.cur < (*c).n_fc {
                (*c).task_thread.cur = ((*c).task_thread.cur).wrapping_sub(1);
            }
            drained = 1 as core::ffi::c_int;
        } else if drained != 0 {
            pthread_mutex_unlock(&raw mut (*c).task_thread.lock);
            break;
        }
        (*c).frame_thread.next = ((*c).frame_thread.next).wrapping_add(1);
        if (*c).frame_thread.next == (*c).n_fc {
            (*c).frame_thread.next = 0 as core::ffi::c_uint;
        }
        pthread_mutex_unlock(&raw mut (*c).task_thread.lock);
        let error: core::ffi::c_int = (*f).task_thread.retval;
        if error != 0 {
            (*f).task_thread.retval = 0 as core::ffi::c_int;
            dav1d_data_props_copy(&raw mut (*c).cached_error_props, &raw mut (*out_delayed).p.m);
            dav1d_thread_picture_unref(out_delayed);
            return error;
        }
        if !((*out_delayed).p.data[0 as core::ffi::c_int as usize]).is_null() {
            let progress: core::ffi::c_uint = ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::Relaxed}, false>(
                &raw mut *((*out_delayed).progress).offset(1 as core::ffi::c_int as isize)
                    as *mut atomic_uint,
            );
            if ((*out_delayed).visible != 0 || (*c).output_invisible_frames != 0)
                && progress != FRAME_ERROR
            {
                dav1d_thread_picture_ref(&raw mut (*c).out, out_delayed);
                (*c).event_flags = ::core::mem::transmute::<
                    core::ffi::c_uint,
                    Dav1dEventFlags,
                >(
                    (*c).event_flags as core::ffi::c_uint
                        | dav1d_picture_get_event_flags(out_delayed) as core::ffi::c_uint,
                );
            }
            dav1d_thread_picture_unref(out_delayed);
            if output_picture_ready(c, 0 as core::ffi::c_int) != 0 {
                return output_image(c, out);
            }
        }
        drain_count = drain_count.wrapping_add(1);
        if !(drain_count < (*c).n_fc) {
            break;
        }
    }
    if output_picture_ready(c, 1 as core::ffi::c_int) != 0 {
        return output_image(c, out);
    }
    return -(35 as core::ffi::c_int);
}
unsafe extern "C" fn gen_picture(c: *mut Dav1dContext) -> core::ffi::c_int {
    let in_0: *mut Dav1dData = &raw mut (*c).in_0;
    if output_picture_ready(c, 0 as core::ffi::c_int) != 0 {
        return 0 as core::ffi::c_int;
    }
    while (*in_0).sz > 0 as size_t {
        let res: ptrdiff_t = dav1d_parse_obus(c, in_0) as ptrdiff_t;
        if res < 0 as ptrdiff_t {
            dav1d_data_unref_internal(in_0);
        } else {
            if !(res as size_t <= (*in_0).sz) as core::ffi::c_int as core::ffi::c_long
                != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 12],
                        [core::ffi::c_char; 12],
                    >(*b"gen_picture\0"))
                        .as_ptr(),
                    b"lib.c\0" as *const u8 as *const core::ffi::c_char,
                    424 as core::ffi::c_int,
                    b"(size_t)res <= in->sz\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            (*in_0).sz = ((*in_0).sz).wrapping_sub(res as size_t);
            (*in_0).data = ((*in_0).data).offset(res as isize);
            if (*in_0).sz == 0 {
                dav1d_data_unref_internal(in_0);
            }
        }
        if output_picture_ready(c, 0 as core::ffi::c_int) != 0 {
            break;
        }
        if res < 0 as ptrdiff_t {
            return res as core::ffi::c_int;
        }
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_send_data(
    c: *mut Dav1dContext,
    in_0: *mut Dav1dData,
) -> core::ffi::c_int {
    if c.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"dav1d_send_data\0"))
                .as_ptr(),
        );
        abort();
    }
    if in_0.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"in != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"dav1d_send_data\0"))
                .as_ptr(),
        );
        abort();
    }
    if !((*in_0).data).is_null() {
        if !((*in_0).sz > 0 as size_t
            && (*in_0).sz <= (18446744073709551615 as size_t).wrapping_div(2 as size_t))
        {
            fprintf(
                __stderrp,
                b"Input validation check '%s' failed in %s!\n\0" as *const u8
                    as *const core::ffi::c_char,
                b"in->sz > 0 && in->sz <= SIZE_MAX / 2\0" as *const u8
                    as *const core::ffi::c_char,
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"dav1d_send_data\0"))
                    .as_ptr(),
            );
            abort();
        }
        (*c).drain = 0 as core::ffi::c_int;
    }
    if !((*c).in_0.data).is_null() {
        return -(35 as core::ffi::c_int);
    }
    dav1d_data_ref(&raw mut (*c).in_0, in_0);
    let mut res: core::ffi::c_int = gen_picture(c);
    if res == 0 {
        dav1d_data_unref_internal(in_0);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_picture(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
) -> core::ffi::c_int {
    if c.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"dav1d_get_picture\0"))
                .as_ptr(),
        );
        abort();
    }
    if out.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"out != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"dav1d_get_picture\0"))
                .as_ptr(),
        );
        abort();
    }
    let drain: core::ffi::c_int = (*c).drain;
    (*c).drain = 1 as core::ffi::c_int;
    let mut res: core::ffi::c_int = gen_picture(c);
    if res < 0 as core::ffi::c_int {
        return res;
    }
    if (*c).cached_error != 0 {
        let res_0: core::ffi::c_int = (*c).cached_error;
        (*c).cached_error = 0 as core::ffi::c_int;
        return res_0;
    }
    if output_picture_ready(c, ((*c).n_fc == 1 as core::ffi::c_uint) as core::ffi::c_int)
        != 0
    {
        return output_image(c, out);
    }
    if (*c).n_fc > 1 as core::ffi::c_uint && drain != 0 {
        return drain_picture(c, out);
    }
    return -(35 as core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) -> core::ffi::c_int {
    if c.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"dav1d_apply_grain\0"))
                .as_ptr(),
        );
        abort();
    }
    if out.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"out != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"dav1d_apply_grain\0"))
                .as_ptr(),
        );
        abort();
    }
    if in_0.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"in != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 18],
                [core::ffi::c_char; 18],
            >(*b"dav1d_apply_grain\0"))
                .as_ptr(),
        );
        abort();
    }
    if has_grain(in_0) == 0 {
        dav1d_picture_ref(out, in_0);
        return 0 as core::ffi::c_int;
    }
    let mut res: core::ffi::c_int = dav1d_picture_alloc_copy(c, out, (*in_0).p.w, in_0);
    if res < 0 as core::ffi::c_int {
        dav1d_picture_unref_internal(out);
        return res;
    } else {
        if (*c).n_tc > 1 as core::ffi::c_uint {
            dav1d_task_delayed_fg(c, out, in_0);
        } else {
            match (*out).p.bpc {
                8 => {
                    dav1d_apply_grain_8bpc(
                        &raw mut (*((*c).dsp)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .fg,
                        out,
                        in_0,
                    );
                }
                10 | 12 => {
                    dav1d_apply_grain_16bpc(
                        &raw mut (*((*c).dsp)
                            .as_mut_ptr()
                            .offset(
                                (((*out).p.bpc >> 1 as core::ffi::c_int)
                                    - 4 as core::ffi::c_int) as isize,
                            ))
                            .fg,
                        out,
                        in_0,
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        return 0 as core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: *mut Dav1dContext) {
    dav1d_data_unref_internal(&raw mut (*c).in_0);
    if !((*c).out.p.frame_hdr).is_null() {
        dav1d_thread_picture_unref(&raw mut (*c).out);
    }
    if !((*c).cache.p.frame_hdr).is_null() {
        dav1d_thread_picture_unref(&raw mut (*c).cache);
    }
    (*c).drain = 0 as core::ffi::c_int;
    (*c).cached_error = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        if !((*c).refs[i as usize].p.p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(
                &raw mut (*((*c).refs).as_mut_ptr().offset(i as isize)).p,
            );
        }
        dav1d_ref_dec(&raw mut (*((*c).refs).as_mut_ptr().offset(i as isize)).segmap);
        dav1d_ref_dec(&raw mut (*((*c).refs).as_mut_ptr().offset(i as isize)).refmvs);
        dav1d_cdf_thread_unref(&raw mut *((*c).cdf).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
    (*c).seq_hdr = 0 as *mut Dav1dSequenceHeader;
    dav1d_ref_dec(&raw mut (*c).seq_hdr_ref);
    (*c).mastering_display = 0 as *mut Dav1dMasteringDisplay;
    (*c).content_light = 0 as *mut Dav1dContentLightLevel;
    (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
    (*c).n_itut_t35 = 0 as core::ffi::c_int;
    dav1d_ref_dec(&raw mut (*c).mastering_display_ref);
    dav1d_ref_dec(&raw mut (*c).content_light_ref);
    dav1d_ref_dec(&raw mut (*c).itut_t35_ref);
    dav1d_data_props_unref_internal(&raw mut (*c).cached_error_props);
    if (*c).n_fc == 1 as core::ffi::c_uint && (*c).n_tc == 1 as core::ffi::c_uint {
        return;
    }
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*c).flush, 1 as core::ffi::c_int);
    if (*c).n_tc > 1 as core::ffi::c_uint {
        pthread_mutex_lock(&raw mut (*c).task_thread.lock);
        let mut i_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
        while i_0 < (*c).n_tc {
            let tc: *mut Dav1dTaskContext = &raw mut *((*c).tc).offset(i_0 as isize)
                as *mut Dav1dTaskContext;
            while (*tc).task_thread.flushed == 0 {
                pthread_cond_wait(
                    &raw mut (*tc).task_thread.td.cond,
                    &raw mut (*c).task_thread.lock,
                );
            }
            i_0 = i_0.wrapping_add(1);
        }
        let mut i_1: core::ffi::c_uint = 0 as core::ffi::c_uint;
        while i_1 < (*c).n_fc {
            let fresh0 = &raw mut (*((*c).fc).offset(i_1 as isize)).task_thread.task_head;
            *fresh0 = 0 as *mut Dav1dTask;
            let fresh1 = &raw mut (*((*c).fc).offset(i_1 as isize)).task_thread.task_tail;
            *fresh1 = 0 as *mut Dav1dTask;
            let fresh2 = &raw mut (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .task_cur_prev;
            *fresh2 = 0 as *mut Dav1dTask;
            let fresh3 = &raw mut (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .head;
            *fresh3 = 0 as *mut Dav1dTask;
            let fresh4 = &raw mut (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .tail;
            *fresh4 = 0 as *mut Dav1dTask;
            *&raw mut (*((*c).fc).offset(i_1 as isize)).task_thread.pending_tasks.merge = 0
                as core::ffi::c_int;
            i_1 = i_1.wrapping_add(1);
        }
        *&raw mut (*c).task_thread.first = 0 as core::ffi::c_int as core::ffi::c_uint;
        (*c).task_thread.cur = (*c).n_fc;
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*c).task_thread.reset_task_cur,
            (2147483647 as core::ffi::c_uint)
                .wrapping_mul(2 as core::ffi::c_uint)
                .wrapping_add(1 as core::ffi::c_uint),
        );
        ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
            &raw mut (*c).task_thread.cond_signaled,
            0 as core::ffi::c_int,
        );
        pthread_mutex_unlock(&raw mut (*c).task_thread.lock);
    }
    if (*c).n_fc > 1 as core::ffi::c_uint {
        let mut n: core::ffi::c_uint = 0 as core::ffi::c_uint;
        let mut next: core::ffi::c_uint = (*c).frame_thread.next;
        while n < (*c).n_fc {
            if next == (*c).n_fc {
                next = 0 as core::ffi::c_uint;
            }
            let f: *mut Dav1dFrameContext = &raw mut *((*c).fc).offset(next as isize)
                as *mut Dav1dFrameContext;
            dav1d_decode_frame_exit(f, -(1 as core::ffi::c_int));
            (*f).n_tile_data = 0 as core::ffi::c_int;
            (*f).task_thread.retval = 0 as core::ffi::c_int;
            (*f).task_thread.error = 0 as core::ffi::c_int;
            let mut out_delayed: *mut Dav1dThreadPicture = &raw mut *((*c)
                .frame_thread
                .out_delayed)
                .offset(next as isize) as *mut Dav1dThreadPicture;
            if !((*out_delayed).p.frame_hdr).is_null() {
                dav1d_thread_picture_unref(out_delayed);
            }
            n = n.wrapping_add(1);
            next = next.wrapping_add(1);
        }
        (*c).frame_thread.next = 0 as core::ffi::c_uint;
    }
    ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>((*c).flush, 0 as core::ffi::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_close(c_out: *mut *mut Dav1dContext) {
    if c_out.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c_out != ((void*)0)\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 12],
                [core::ffi::c_char; 12],
            >(*b"dav1d_close\0"))
                .as_ptr(),
        );
        abort();
    }
    close_internal(c_out, 1 as core::ffi::c_int);
}
#[cold]
unsafe extern "C" fn close_internal(
    c_out: *mut *mut Dav1dContext,
    mut flush: core::ffi::c_int,
) {
    let c: *mut Dav1dContext = *c_out;
    if c.is_null() {
        return;
    }
    if flush != 0 {
        dav1d_flush(c);
    }
    if !((*c).tc).is_null() {
        let mut ttd: *mut TaskThreadData = &raw mut (*c).task_thread;
        if (*ttd).inited != 0 {
            pthread_mutex_lock(&raw mut (*ttd).lock);
            let mut n: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while n < (*c).n_tc
                && (*((*c).tc).offset(n as isize)).task_thread.td.inited != 0
            {
                (*((*c).tc).offset(n as isize)).task_thread.die = 1 as core::ffi::c_int;
                n = n.wrapping_add(1);
            }
            pthread_cond_broadcast(&raw mut (*ttd).cond);
            pthread_mutex_unlock(&raw mut (*ttd).lock);
            let mut n_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
            while n_0 < (*c).n_tc {
                let pf: *mut Dav1dTaskContext = &raw mut *((*c).tc).offset(n_0 as isize)
                    as *mut Dav1dTaskContext;
                if (*pf).task_thread.td.inited == 0 {
                    break;
                }
                pthread_join(
                    (*pf).task_thread.td.thread as pthread_t,
                    0 as *mut *mut core::ffi::c_void,
                );
                pthread_cond_destroy(&raw mut (*pf).task_thread.td.cond);
                pthread_mutex_destroy(&raw mut (*pf).task_thread.td.lock);
                n_0 = n_0.wrapping_add(1);
            }
            pthread_cond_destroy(&raw mut (*ttd).delayed_fg.cond);
            pthread_cond_destroy(&raw mut (*ttd).cond);
            pthread_mutex_destroy(&raw mut (*ttd).lock);
        }
        dav1d_free_aligned_internal((*c).tc as *mut core::ffi::c_void);
    }
    let mut n_1: core::ffi::c_uint = 0 as core::ffi::c_uint;
    while !((*c).fc).is_null() && n_1 < (*c).n_fc {
        let f: *mut Dav1dFrameContext = &raw mut *((*c).fc).offset(n_1 as isize)
            as *mut Dav1dFrameContext;
        if (*c).n_fc > 1 as core::ffi::c_uint {
            free((*f).tile_thread.lowest_pixel_mem as *mut core::ffi::c_void);
            free((*f).frame_thread.b as *mut core::ffi::c_void);
            dav1d_free_aligned_internal((*f).frame_thread.cbi as *mut core::ffi::c_void);
            dav1d_free_aligned_internal(
                (*f).frame_thread.pal_idx as *mut core::ffi::c_void,
            );
            dav1d_free_aligned_internal((*f).frame_thread.cf as *mut core::ffi::c_void);
            free((*f).frame_thread.tile_start_off as *mut core::ffi::c_void);
            dav1d_free_aligned_internal((*f).frame_thread.pal as *mut core::ffi::c_void);
        }
        if (*c).n_tc > 1 as core::ffi::c_uint {
            pthread_mutex_destroy(&raw mut (*f).task_thread.pending_tasks.lock);
            pthread_cond_destroy(&raw mut (*f).task_thread.cond);
            pthread_mutex_destroy(&raw mut (*f).task_thread.lock);
        }
        free((*f).frame_thread.frame_progress as *mut core::ffi::c_void);
        free((*f).task_thread.tasks as *mut core::ffi::c_void);
        free(
            (*f).task_thread.tile_tasks[0 as core::ffi::c_int as usize]
                as *mut core::ffi::c_void,
        );
        dav1d_free_aligned_internal((*f).ts as *mut core::ffi::c_void);
        dav1d_free_aligned_internal(
            (*f).ipred_edge[0 as core::ffi::c_int as usize] as *mut core::ffi::c_void,
        );
        free((*f).a as *mut core::ffi::c_void);
        free((*f).tile as *mut core::ffi::c_void);
        free((*f).lf.mask as *mut core::ffi::c_void);
        free((*f).lf.level as *mut core::ffi::c_void);
        free((*f).lf.lr_mask as *mut core::ffi::c_void);
        free(
            (*f).lf.tx_lpf_right_edge[0 as core::ffi::c_int as usize]
                as *mut core::ffi::c_void,
        );
        free((*f).lf.start_of_tile_row as *mut core::ffi::c_void);
        dav1d_free_aligned_internal((*f).rf.r as *mut core::ffi::c_void);
        dav1d_free_aligned_internal((*f).lf.cdef_line_buf as *mut core::ffi::c_void);
        dav1d_free_aligned_internal((*f).lf.lr_line_buf as *mut core::ffi::c_void);
        n_1 = n_1.wrapping_add(1);
    }
    dav1d_free_aligned_internal((*c).fc as *mut core::ffi::c_void);
    if (*c).n_fc > 1 as core::ffi::c_uint && !((*c).frame_thread.out_delayed).is_null() {
        let mut n_2: core::ffi::c_uint = 0 as core::ffi::c_uint;
        while n_2 < (*c).n_fc {
            if !((*((*c).frame_thread.out_delayed).offset(n_2 as isize)).p.frame_hdr)
                .is_null()
            {
                dav1d_thread_picture_unref(
                    &raw mut *((*c).frame_thread.out_delayed).offset(n_2 as isize),
                );
            }
            n_2 = n_2.wrapping_add(1);
        }
        free((*c).frame_thread.out_delayed as *mut core::ffi::c_void);
    }
    let mut n_3: core::ffi::c_int = 0 as core::ffi::c_int;
    while n_3 < (*c).n_tile_data {
        dav1d_data_unref_internal(&raw mut (*((*c).tile).offset(n_3 as isize)).data);
        n_3 += 1;
    }
    free((*c).tile as *mut core::ffi::c_void);
    let mut n_4: core::ffi::c_int = 0 as core::ffi::c_int;
    while n_4 < 8 as core::ffi::c_int {
        dav1d_cdf_thread_unref(&raw mut *((*c).cdf).as_mut_ptr().offset(n_4 as isize));
        if !((*c).refs[n_4 as usize].p.p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(
                &raw mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).p,
            );
        }
        dav1d_ref_dec(&raw mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).refmvs);
        dav1d_ref_dec(&raw mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).segmap);
        n_4 += 1;
    }
    dav1d_ref_dec(&raw mut (*c).seq_hdr_ref);
    dav1d_ref_dec(&raw mut (*c).frame_hdr_ref);
    dav1d_ref_dec(&raw mut (*c).mastering_display_ref);
    dav1d_ref_dec(&raw mut (*c).content_light_ref);
    dav1d_ref_dec(&raw mut (*c).itut_t35_ref);
    dav1d_mem_pool_end((*c).seq_hdr_pool);
    dav1d_mem_pool_end((*c).frame_hdr_pool);
    dav1d_mem_pool_end((*c).segmap_pool);
    dav1d_mem_pool_end((*c).refmvs_pool);
    dav1d_mem_pool_end((*c).cdf_pool);
    dav1d_mem_pool_end((*c).picture_pool);
    dav1d_mem_pool_end((*c).pic_ctx_pool);
    dav1d_freep_aligned(c_out as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_event_flags(
    c: *mut Dav1dContext,
    flags: *mut Dav1dEventFlags,
) -> core::ffi::c_int {
    if c.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_get_event_flags\0"))
                .as_ptr(),
        );
        abort();
    }
    if flags.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"flags != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"dav1d_get_event_flags\0"))
                .as_ptr(),
        );
        abort();
    }
    *flags = (*c).event_flags;
    (*c).event_flags = 0 as Dav1dEventFlags;
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_decode_error_data_props(
    c: *mut Dav1dContext,
    out: *mut Dav1dDataProps,
) -> core::ffi::c_int {
    if c.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"c != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"dav1d_get_decode_error_data_props\0"))
                .as_ptr(),
        );
        abort();
    }
    if out.is_null() {
        fprintf(
            __stderrp,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const core::ffi::c_char,
            b"out != NULL\0" as *const u8 as *const core::ffi::c_char,
            (::core::mem::transmute::<
                [u8; 34],
                [core::ffi::c_char; 34],
            >(*b"dav1d_get_decode_error_data_props\0"))
                .as_ptr(),
        );
        abort();
    }
    dav1d_data_props_unref_internal(out);
    *out = (*c).cached_error_props;
    dav1d_data_props_set_defaults(&raw mut (*c).cached_error_props);
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: *mut Dav1dPicture) {
    dav1d_picture_unref_internal(p);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(
    buf: *mut Dav1dData,
    sz: size_t,
) -> *mut uint8_t {
    return dav1d_data_create_internal(buf, sz);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap(
    buf: *mut Dav1dData,
    ptr: *const uint8_t,
    sz: size_t,
    free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    user_data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    return dav1d_data_wrap_internal(buf, ptr, sz, free_callback, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data(
    buf: *mut Dav1dData,
    user_data: *const uint8_t,
    free_callback: Option<
        unsafe extern "C" fn(*const uint8_t, *mut core::ffi::c_void) -> (),
    >,
    cookie: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    return dav1d_data_wrap_user_data_internal(buf, user_data, free_callback, cookie);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref(buf: *mut Dav1dData) {
    dav1d_data_unref_internal(buf);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref(props: *mut Dav1dDataProps) {
    dav1d_data_props_unref_internal(props);
}
