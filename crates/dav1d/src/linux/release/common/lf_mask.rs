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

// Mechanically translated from upstream/dav1d/src/lf_mask.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
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
    static dav1d_memset_pow2: [dav1d_memset_pow2_fn; 6];
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
    static dav1d_txfm_dimensions: [TxfmInfo; 19];
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ptrdiff_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias64 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias32 {
    pub u32_0: uint32_t,
    pub u8_0: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union alias8 {
    pub u8_0: uint8_t,
}
pub type dav1d_memset_pow2_fn = Option<
    unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_int) -> (),
>;
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
pub type TxfmSize = core::ffi::c_uint;
pub const N_TX_SIZES: TxfmSize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;
pub type RectTxfmSize = core::ffi::c_uint;
pub const N_RECT_TX_SIZES: RectTxfmSize = 19;
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;
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
pub struct Av1Filter {
    pub filter_y: [[[[uint16_t; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[uint16_t; 2]; 2]; 32]; 2],
    pub cdef_idx: [int8_t; 4],
    pub noskip_mask: [[uint16_t; 2]; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxfmInfo {
    pub w: uint8_t,
    pub h: uint8_t,
    pub lw: uint8_t,
    pub lh: uint8_t,
    pub min: uint8_t,
    pub max: uint8_t,
    pub sub: uint8_t,
    pub ctx: uint8_t,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
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
unsafe extern "C" fn ulog2(v: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ clz(v);
}
#[inline]
unsafe extern "C" fn dav1d_memset_likely_pow2(
    ptr: *mut core::ffi::c_void,
    value: core::ffi::c_int,
    n: core::ffi::c_int,
) {
    if !(n >= 1 as core::ffi::c_int && n <= 32 as core::ffi::c_int) {
        unreachable!();
    }
    if n & n - 1 as core::ffi::c_int == 0 as core::ffi::c_int {
        (dav1d_memset_pow2[ulog2(n as core::ffi::c_uint) as usize])
            .expect("non-null function pointer")(ptr, value);
    } else {
        memset(ptr, value, n as size_t);
    };
}
unsafe extern "C" fn decomp_tx(
    txa: *mut [[[uint8_t; 32]; 32]; 2],
    from: RectTxfmSize,
    depth: core::ffi::c_int,
    y_off: core::ffi::c_int,
    x_off: core::ffi::c_int,
    tx_masks: *const uint16_t,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(from as isize)
        as *const TxfmInfo;
    let is_split: core::ffi::c_int = if from as core::ffi::c_uint
        == TX_4X4 as core::ffi::c_int as core::ffi::c_uint
        || depth > 1 as core::ffi::c_int
    {
        0 as core::ffi::c_int
    } else {
        *tx_masks.offset(depth as isize) as core::ffi::c_int
            >> y_off * 4 as core::ffi::c_int + x_off & 1 as core::ffi::c_int
    };
    if is_split != 0 {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let htw4: core::ffi::c_int = (*t_dim).w as core::ffi::c_int
            >> 1 as core::ffi::c_int;
        let hth4: core::ffi::c_int = (*t_dim).h as core::ffi::c_int
            >> 1 as core::ffi::c_int;
        decomp_tx(
            txa,
            sub,
            depth + 1 as core::ffi::c_int,
            y_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
            x_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
            tx_masks,
        );
        if (*t_dim).w as core::ffi::c_int >= (*t_dim).h as core::ffi::c_int {
            decomp_tx(
                &raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(htw4 as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                sub,
                depth + 1 as core::ffi::c_int,
                y_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
                x_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                tx_masks,
            );
        }
        if (*t_dim).h as core::ffi::c_int >= (*t_dim).w as core::ffi::c_int {
            decomp_tx(
                &raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(hth4 as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                sub,
                depth + 1 as core::ffi::c_int,
                y_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                x_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
                tx_masks,
            );
            if (*t_dim).w as core::ffi::c_int >= (*t_dim).h as core::ffi::c_int {
                decomp_tx(
                    &raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(hth4 as isize))
                        .as_mut_ptr()
                        .offset(htw4 as isize) as *mut uint8_t
                        as *mut [[[uint8_t; 32]; 32]; 2],
                    sub,
                    depth + 1 as core::ffi::c_int,
                    y_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                    x_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                    tx_masks,
                );
            }
        }
    } else {
        let lw: core::ffi::c_int = imin(
            2 as core::ffi::c_int,
            (*t_dim).lw as core::ffi::c_int,
        ) as core::ffi::c_int;
        let lh: core::ffi::c_int = imin(
            2 as core::ffi::c_int,
            (*t_dim).lh as core::ffi::c_int,
        ) as core::ffi::c_int;
        match (*t_dim).lw as core::ffi::c_int {
            0 => {
                let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
                while y < (*t_dim).h as core::ffi::c_int {
                    (*(&raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias8))
                        .u8_0 = (lw * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *(*(*(*txa.offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias8))
                        .u8_0 = (lh * 0x1 as core::ffi::c_int) as uint8_t;
                    (*txa
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[1 as core::ffi::c_int
                        as usize][y as usize][0 as core::ffi::c_int as usize] = (*t_dim)
                        .w;
                    y += 1;
                }
            }
            1 => {
                let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_0 < (*t_dim).h as core::ffi::c_int {
                    (*(&raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias16))
                        .u16_0 = (lw * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *(*(*(*txa.offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias16))
                        .u16_0 = (lh * 0x101 as core::ffi::c_int) as uint16_t;
                    (*txa
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[1 as core::ffi::c_int
                        as usize][y_0 as usize][0 as core::ffi::c_int as usize] = (*t_dim)
                        .w;
                    y_0 += 1;
                }
            }
            2 => {
                let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_1 < (*t_dim).h as core::ffi::c_int {
                    (*(&raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_1 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias32))
                        .u32_0 = (lw as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*(*(*txa.offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_1 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias32))
                        .u32_0 = (lh as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*txa
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[1 as core::ffi::c_int
                        as usize][y_1 as usize][0 as core::ffi::c_int as usize] = (*t_dim)
                        .w;
                    y_1 += 1;
                }
            }
            3 => {
                let mut y_2: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_2 < (*t_dim).h as core::ffi::c_int {
                    (*(&raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_2 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = (lw as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*(*(*txa.offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(y_2 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = (lh as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*txa
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[1 as core::ffi::c_int
                        as usize][y_2 as usize][0 as core::ffi::c_int as usize] = (*t_dim)
                        .w;
                    y_2 += 1;
                }
            }
            4 => {
                let mut y_3: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_3 < (*t_dim).h as core::ffi::c_int {
                    memset(
                        &raw mut *(*(*(*txa.offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(y_3 as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        lw,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*(*(*txa.offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(y_3 as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        lh,
                        16 as size_t,
                    );
                    (*txa
                        .offset(
                            0 as core::ffi::c_int as isize,
                        ))[1 as core::ffi::c_int
                        as usize][y_3 as usize][0 as core::ffi::c_int as usize] = (*t_dim)
                        .w;
                    y_3 += 1;
                }
            }
            _ => {
                if 0 as core::ffi::c_int == 0 {
                    unreachable!();
                }
            }
        }
        (dav1d_memset_pow2[(*t_dim).lw as usize])
            .expect(
                "non-null function pointer",
            )(
            ((*txa
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize])
                .as_mut_ptr() as *mut core::ffi::c_void,
            (*t_dim).h as core::ffi::c_int,
        );
    };
}
#[inline]
unsafe extern "C" fn mask_edges_inter(
    masks: *mut [[[uint16_t; 2]; 3]; 32],
    by4: core::ffi::c_int,
    bx4: core::ffi::c_int,
    w4: core::ffi::c_int,
    h4: core::ffi::c_int,
    skip: core::ffi::c_int,
    max_tx: RectTxfmSize,
    tx_masks: *const uint16_t,
    a: *mut uint8_t,
    l: *mut uint8_t,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(max_tx as isize)
        as *const TxfmInfo;
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut txa: crate::platform::Aligned16<[[[[uint8_t; 32]; 32]; 2]; 2]> = crate::platform::Aligned16([[[[0; 32]; 32]; 2]; 2]);
    let mut y_off: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < h4 {
        let mut x_off: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < w4 {
            decomp_tx(
                &raw mut *(*(*(*txa.0.as_mut_ptr().offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(y_0 as isize))
                    .as_mut_ptr()
                    .offset(x_0 as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                max_tx,
                0 as core::ffi::c_int,
                y_off,
                x_off,
                tx_masks,
            );
            x_0 += (*t_dim).w as core::ffi::c_int;
            x_off += 1;
        }
        y_0 += (*t_dim).h as core::ffi::c_int;
        y_off += 1;
    }
    let mut mask: core::ffi::c_uint = (1 as core::ffi::c_uint) << by4;
    y = 0 as core::ffi::c_int;
    while y < h4 {
        let sidx: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
            as core::ffi::c_int;
        let smask: core::ffi::c_uint = mask >> (sidx << 4 as core::ffi::c_int);
        let fresh12 = &raw mut (*masks
            .offset(
                0 as core::ffi::c_int as isize,
            ))[bx4
            as usize][imin(
            txa.0[0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int
                as usize][y as usize][0 as core::ffi::c_int as usize]
                as core::ffi::c_int,
            *l.offset(y as isize) as core::ffi::c_int,
        ) as usize][sidx as usize];
        *fresh12 = (*fresh12 as core::ffi::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1 as core::ffi::c_int;
    }
    x = 0 as core::ffi::c_int;
    mask = (1 as core::ffi::c_uint) << bx4;
    while x < w4 {
        let sidx_0: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
            as core::ffi::c_int;
        let smask_0: core::ffi::c_uint = mask >> (sidx_0 << 4 as core::ffi::c_int);
        let fresh13 = &raw mut (*masks
            .offset(
                1 as core::ffi::c_int as isize,
            ))[by4
            as usize][imin(
            txa.0[1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize][x as usize]
                as core::ffi::c_int,
            *a.offset(x as isize) as core::ffi::c_int,
        ) as usize][sidx_0 as usize];
        *fresh13 = (*fresh13 as core::ffi::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1 as core::ffi::c_int;
    }
    if skip == 0 {
        y = 0 as core::ffi::c_int;
        mask = (1 as core::ffi::c_uint) << by4;
        while y < h4 {
            let sidx_1: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
                as core::ffi::c_int;
            let smask_1: core::ffi::c_uint = mask >> (sidx_1 << 4 as core::ffi::c_int);
            let mut ltx: core::ffi::c_int = txa.0[0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int
                as usize][y as usize][0 as core::ffi::c_int as usize]
                as core::ffi::c_int;
            let mut step: core::ffi::c_int = txa.0[0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int
                as usize][y as usize][0 as core::ffi::c_int as usize]
                as core::ffi::c_int;
            x = step;
            while x < w4 {
                let rtx: core::ffi::c_int = txa.0[0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize][y as usize][x as usize]
                    as core::ffi::c_int;
                let fresh14 = &raw mut (*masks
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[(bx4 + x) as usize][imin(rtx, ltx) as usize][sidx_1 as usize];
                *fresh14 = (*fresh14 as core::ffi::c_uint | smask_1) as uint16_t;
                ltx = rtx;
                step = txa.0[0 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize][y as usize][x as usize]
                    as core::ffi::c_int;
                x += step;
            }
            y += 1;
            mask <<= 1 as core::ffi::c_int;
        }
        x = 0 as core::ffi::c_int;
        mask = (1 as core::ffi::c_uint) << bx4;
        while x < w4 {
            let sidx_2: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
                as core::ffi::c_int;
            let smask_2: core::ffi::c_uint = mask >> (sidx_2 << 4 as core::ffi::c_int);
            let mut ttx: core::ffi::c_int = txa.0[1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize][x as usize]
                as core::ffi::c_int;
            let mut step_0: core::ffi::c_int = txa.0[1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize][x as usize]
                as core::ffi::c_int;
            y = step_0;
            while y < h4 {
                let btx: core::ffi::c_int = txa.0[1 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize][y as usize][x as usize]
                    as core::ffi::c_int;
                let fresh15 = &raw mut (*masks
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[(by4 + y) as usize][imin(ttx, btx) as usize][sidx_2 as usize];
                *fresh15 = (*fresh15 as core::ffi::c_uint | smask_2) as uint16_t;
                ttx = btx;
                step_0 = txa.0[1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize][y as usize][x as usize]
                    as core::ffi::c_int;
                y += step_0;
            }
            x += 1;
            mask <<= 1 as core::ffi::c_int;
        }
    }
    y = 0 as core::ffi::c_int;
    while y < h4 {
        *l.offset(y as isize) = txa.0[0 as core::ffi::c_int
            as usize][0 as core::ffi::c_int
            as usize][y as usize][(w4 - 1 as core::ffi::c_int) as usize];
        y += 1;
    }
    memcpy(
        a as *mut core::ffi::c_void,
        (txa.0[1 as core::ffi::c_int
            as usize][0 as core::ffi::c_int
            as usize][(h4 - 1 as core::ffi::c_int) as usize])
            .as_mut_ptr() as *const core::ffi::c_void,
        w4 as size_t,
    );
}
#[inline]
unsafe extern "C" fn mask_edges_intra(
    masks: *mut [[[uint16_t; 2]; 3]; 32],
    by4: core::ffi::c_int,
    bx4: core::ffi::c_int,
    w4: core::ffi::c_int,
    h4: core::ffi::c_int,
    tx: RectTxfmSize,
    a: *mut uint8_t,
    l: *mut uint8_t,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize)
        as *const TxfmInfo;
    let twl4: core::ffi::c_int = (*t_dim).lw as core::ffi::c_int;
    let thl4: core::ffi::c_int = (*t_dim).lh as core::ffi::c_int;
    let twl4c: core::ffi::c_int = imin(2 as core::ffi::c_int, twl4) as core::ffi::c_int;
    let thl4c: core::ffi::c_int = imin(2 as core::ffi::c_int, thl4) as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let mut mask: core::ffi::c_uint = (1 as core::ffi::c_uint) << by4;
    y = 0 as core::ffi::c_int;
    while y < h4 {
        let sidx: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
            as core::ffi::c_int;
        let smask: core::ffi::c_uint = mask >> (sidx << 4 as core::ffi::c_int);
        let fresh6 = &raw mut (*masks
            .offset(
                0 as core::ffi::c_int as isize,
            ))[bx4
            as usize][imin(twl4c, *l.offset(y as isize) as core::ffi::c_int)
            as usize][sidx as usize];
        *fresh6 = (*fresh6 as core::ffi::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1 as core::ffi::c_int;
    }
    x = 0 as core::ffi::c_int;
    mask = (1 as core::ffi::c_uint) << bx4;
    while x < w4 {
        let sidx_0: core::ffi::c_int = (mask >= 0x10000 as core::ffi::c_uint)
            as core::ffi::c_int;
        let smask_0: core::ffi::c_uint = mask >> (sidx_0 << 4 as core::ffi::c_int);
        let fresh7 = &raw mut (*masks
            .offset(
                1 as core::ffi::c_int as isize,
            ))[by4
            as usize][imin(thl4c, *a.offset(x as isize) as core::ffi::c_int)
            as usize][sidx_0 as usize];
        *fresh7 = (*fresh7 as core::ffi::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1 as core::ffi::c_int;
    }
    let hstep: core::ffi::c_int = (*t_dim).w as core::ffi::c_int;
    let mut t: core::ffi::c_uint = (1 as core::ffi::c_uint) << by4;
    let mut inner: core::ffi::c_uint = ((t as uint64_t) << h4)
        .wrapping_sub(t as uint64_t) as core::ffi::c_uint;
    let mut inner1: core::ffi::c_uint = inner & 0xffff as core::ffi::c_uint;
    let mut inner2: core::ffi::c_uint = inner >> 16 as core::ffi::c_int;
    x = hstep;
    while x < w4 {
        if inner1 != 0 {
            let fresh8 = &raw mut (*masks
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[(bx4 + x) as usize][twl4c as usize][0 as core::ffi::c_int as usize];
            *fresh8 = (*fresh8 as core::ffi::c_uint | inner1) as uint16_t;
        }
        if inner2 != 0 {
            let fresh9 = &raw mut (*masks
                .offset(
                    0 as core::ffi::c_int as isize,
                ))[(bx4 + x) as usize][twl4c as usize][1 as core::ffi::c_int as usize];
            *fresh9 = (*fresh9 as core::ffi::c_uint | inner2) as uint16_t;
        }
        x += hstep;
    }
    let vstep: core::ffi::c_int = (*t_dim).h as core::ffi::c_int;
    t = (1 as core::ffi::c_uint) << bx4;
    inner = ((t as uint64_t) << w4).wrapping_sub(t as uint64_t) as core::ffi::c_uint;
    inner1 = inner & 0xffff as core::ffi::c_uint;
    inner2 = inner >> 16 as core::ffi::c_int;
    y = vstep;
    while y < h4 {
        if inner1 != 0 {
            let fresh10 = &raw mut (*masks
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[(by4 + y) as usize][thl4c as usize][0 as core::ffi::c_int as usize];
            *fresh10 = (*fresh10 as core::ffi::c_uint | inner1) as uint16_t;
        }
        if inner2 != 0 {
            let fresh11 = &raw mut (*masks
                .offset(
                    1 as core::ffi::c_int as isize,
                ))[(by4 + y) as usize][thl4c as usize][1 as core::ffi::c_int as usize];
            *fresh11 = (*fresh11 as core::ffi::c_uint | inner2) as uint16_t;
        }
        y += vstep;
    }
    dav1d_memset_likely_pow2(a as *mut core::ffi::c_void, thl4c, w4);
    dav1d_memset_likely_pow2(l as *mut core::ffi::c_void, twl4c, h4);
}
unsafe extern "C" fn mask_edges_chroma(
    masks: *mut [[[uint16_t; 2]; 2]; 32],
    cby4: core::ffi::c_int,
    cbx4: core::ffi::c_int,
    cw4: core::ffi::c_int,
    ch4: core::ffi::c_int,
    skip_inter: core::ffi::c_int,
    tx: RectTxfmSize,
    a: *mut uint8_t,
    l: *mut uint8_t,
    ss_hor: core::ffi::c_int,
    ss_ver: core::ffi::c_int,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize)
        as *const TxfmInfo;
    let twl4: core::ffi::c_int = (*t_dim).lw as core::ffi::c_int;
    let thl4: core::ffi::c_int = (*t_dim).lh as core::ffi::c_int;
    let twl4c: core::ffi::c_int = (twl4 != 0) as core::ffi::c_int;
    let thl4c: core::ffi::c_int = (thl4 != 0) as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    let vbits: core::ffi::c_int = 4 as core::ffi::c_int - ss_ver;
    let hbits: core::ffi::c_int = 4 as core::ffi::c_int - ss_hor;
    let vmask: core::ffi::c_int = 16 as core::ffi::c_int >> ss_ver;
    let hmask: core::ffi::c_int = 16 as core::ffi::c_int >> ss_hor;
    let vmax: core::ffi::c_uint = ((1 as core::ffi::c_int) << vmask)
        as core::ffi::c_uint;
    let hmax: core::ffi::c_uint = ((1 as core::ffi::c_int) << hmask)
        as core::ffi::c_uint;
    let mut mask: core::ffi::c_uint = (1 as core::ffi::c_uint) << cby4;
    y = 0 as core::ffi::c_int;
    while y < ch4 {
        let sidx: core::ffi::c_int = (mask >= vmax) as core::ffi::c_int;
        let smask: core::ffi::c_uint = mask >> (sidx << vbits);
        let fresh0 = &raw mut (*masks
            .offset(
                0 as core::ffi::c_int as isize,
            ))[cbx4
            as usize][imin(twl4c, *l.offset(y as isize) as core::ffi::c_int)
            as usize][sidx as usize];
        *fresh0 = (*fresh0 as core::ffi::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1 as core::ffi::c_int;
    }
    x = 0 as core::ffi::c_int;
    mask = (1 as core::ffi::c_uint) << cbx4;
    while x < cw4 {
        let sidx_0: core::ffi::c_int = (mask >= hmax) as core::ffi::c_int;
        let smask_0: core::ffi::c_uint = mask >> (sidx_0 << hbits);
        let fresh1 = &raw mut (*masks
            .offset(
                1 as core::ffi::c_int as isize,
            ))[cby4
            as usize][imin(thl4c, *a.offset(x as isize) as core::ffi::c_int)
            as usize][sidx_0 as usize];
        *fresh1 = (*fresh1 as core::ffi::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1 as core::ffi::c_int;
    }
    if skip_inter == 0 {
        let hstep: core::ffi::c_int = (*t_dim).w as core::ffi::c_int;
        let mut t: core::ffi::c_uint = (1 as core::ffi::c_uint) << cby4;
        let mut inner: core::ffi::c_uint = ((t as uint64_t) << ch4)
            .wrapping_sub(t as uint64_t) as core::ffi::c_uint;
        let mut inner1: core::ffi::c_uint = inner
            & (((1 as core::ffi::c_int) << vmask) - 1 as core::ffi::c_int)
                as core::ffi::c_uint;
        let mut inner2: core::ffi::c_uint = inner >> vmask;
        x = hstep;
        while x < cw4 {
            if inner1 != 0 {
                let fresh2 = &raw mut (*masks
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[(cbx4 + x)
                    as usize][twl4c as usize][0 as core::ffi::c_int as usize];
                *fresh2 = (*fresh2 as core::ffi::c_uint | inner1) as uint16_t;
            }
            if inner2 != 0 {
                let fresh3 = &raw mut (*masks
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[(cbx4 + x)
                    as usize][twl4c as usize][1 as core::ffi::c_int as usize];
                *fresh3 = (*fresh3 as core::ffi::c_uint | inner2) as uint16_t;
            }
            x += hstep;
        }
        let vstep: core::ffi::c_int = (*t_dim).h as core::ffi::c_int;
        t = (1 as core::ffi::c_uint) << cbx4;
        inner = ((t as uint64_t) << cw4).wrapping_sub(t as uint64_t)
            as core::ffi::c_uint;
        inner1 = inner
            & (((1 as core::ffi::c_int) << hmask) - 1 as core::ffi::c_int)
                as core::ffi::c_uint;
        inner2 = inner >> hmask;
        y = vstep;
        while y < ch4 {
            if inner1 != 0 {
                let fresh4 = &raw mut (*masks
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[(cby4 + y)
                    as usize][thl4c as usize][0 as core::ffi::c_int as usize];
                *fresh4 = (*fresh4 as core::ffi::c_uint | inner1) as uint16_t;
            }
            if inner2 != 0 {
                let fresh5 = &raw mut (*masks
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[(cby4 + y)
                    as usize][thl4c as usize][1 as core::ffi::c_int as usize];
                *fresh5 = (*fresh5 as core::ffi::c_uint | inner2) as uint16_t;
            }
            y += vstep;
        }
    }
    dav1d_memset_likely_pow2(a as *mut core::ffi::c_void, thl4c, cw4);
    dav1d_memset_likely_pow2(l as *mut core::ffi::c_void, twl4c, ch4);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_create_lf_mask_intra(
    lflvl: *mut Av1Filter,
    level_cache: *mut [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[uint8_t; 2]; 8],
    bx: core::ffi::c_int,
    by: core::ffi::c_int,
    iw: core::ffi::c_int,
    ih: core::ffi::c_int,
    bs: BlockSize,
    ytx: RectTxfmSize,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut uint8_t,
    ly: *mut uint8_t,
    auv: *mut uint8_t,
    luv: *mut uint8_t,
) {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: core::ffi::c_int = imin(
        iw - bx,
        *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
    ) as core::ffi::c_int;
    let bh4: core::ffi::c_int = imin(
        ih - by,
        *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
    ) as core::ffi::c_int;
    let bx4: core::ffi::c_int = bx & 31 as core::ffi::c_int;
    let by4: core::ffi::c_int = by & 31 as core::ffi::c_int;
    if !(bw4 >= 0 as core::ffi::c_int && bh4 >= 0 as core::ffi::c_int) {
        unreachable!();
    }
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [uint8_t; 4] = level_cache
            .offset((by as ptrdiff_t * b4_stride) as isize)
            .offset(bx as isize);
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < bh4 {
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < bw4 {
                (*level_cache_ptr.offset(x as isize))[0 as core::ffi::c_int as usize] = (*filter_level
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
                (*level_cache_ptr.offset(x as isize))[1 as core::ffi::c_int as usize] = (*filter_level
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
                x += 1;
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
            y += 1;
        }
        mask_edges_intra(
            ((*lflvl).filter_y).as_mut_ptr() as *mut [[[uint16_t; 2]; 3]; 32],
            by4,
            bx4,
            bw4,
            bh4,
            ytx,
            ay,
            ly,
        );
    }
    if auv.is_null() {
        return;
    }
    let ss_ver: core::ffi::c_int = (layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = (layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let cbw4: core::ffi::c_int = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int + ss_hor
            >> ss_hor,
    ) as core::ffi::c_int;
    let cbh4: core::ffi::c_int = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + ss_ver
            >> ss_ver,
    ) as core::ffi::c_int;
    if !(cbw4 >= 0 as core::ffi::c_int && cbh4 >= 0 as core::ffi::c_int) {
        unreachable!();
    }
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4: core::ffi::c_int = bx4 >> ss_hor;
    let cby4: core::ffi::c_int = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [uint8_t; 4] = level_cache
        .offset(((by >> ss_ver) as ptrdiff_t * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < cbh4 {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < cbw4 {
            (*level_cache_ptr_0.offset(x_0 as isize))[2 as core::ffi::c_int as usize] = (*filter_level
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
            (*level_cache_ptr_0.offset(x_0 as isize))[3 as core::ffi::c_int as usize] = (*filter_level
                .offset(
                    3 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
            x_0 += 1;
        }
        level_cache_ptr_0 = level_cache_ptr_0.offset(b4_stride as isize);
        y_0 += 1;
    }
    mask_edges_chroma(
        ((*lflvl).filter_uv).as_mut_ptr() as *mut [[[uint16_t; 2]; 2]; 32],
        cby4,
        cbx4,
        cbw4,
        cbh4,
        0 as core::ffi::c_int,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_create_lf_mask_inter(
    lflvl: *mut Av1Filter,
    level_cache: *mut [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[uint8_t; 2]; 8],
    bx: core::ffi::c_int,
    by: core::ffi::c_int,
    iw: core::ffi::c_int,
    ih: core::ffi::c_int,
    skip: core::ffi::c_int,
    bs: BlockSize,
    max_ytx: RectTxfmSize,
    tx_masks: *const uint16_t,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut uint8_t,
    ly: *mut uint8_t,
    auv: *mut uint8_t,
    luv: *mut uint8_t,
) {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: core::ffi::c_int = imin(
        iw - bx,
        *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
    ) as core::ffi::c_int;
    let bh4: core::ffi::c_int = imin(
        ih - by,
        *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
    ) as core::ffi::c_int;
    let bx4: core::ffi::c_int = bx & 31 as core::ffi::c_int;
    let by4: core::ffi::c_int = by & 31 as core::ffi::c_int;
    if !(bw4 >= 0 as core::ffi::c_int && bh4 >= 0 as core::ffi::c_int) {
        unreachable!();
    }
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [uint8_t; 4] = level_cache
            .offset((by as ptrdiff_t * b4_stride) as isize)
            .offset(bx as isize);
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < bh4 {
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < bw4 {
                (*level_cache_ptr.offset(x as isize))[0 as core::ffi::c_int as usize] = (*filter_level
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
                (*level_cache_ptr.offset(x as isize))[1 as core::ffi::c_int as usize] = (*filter_level
                    .offset(
                        1 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
                x += 1;
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
            y += 1;
        }
        mask_edges_inter(
            ((*lflvl).filter_y).as_mut_ptr() as *mut [[[uint16_t; 2]; 3]; 32],
            by4,
            bx4,
            bw4,
            bh4,
            skip,
            max_ytx,
            tx_masks,
            ay,
            ly,
        );
    }
    if auv.is_null() {
        return;
    }
    let ss_ver: core::ffi::c_int = (layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = (layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let cbw4: core::ffi::c_int = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int + ss_hor
            >> ss_hor,
    ) as core::ffi::c_int;
    let cbh4: core::ffi::c_int = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int + ss_ver
            >> ss_ver,
    ) as core::ffi::c_int;
    if !(cbw4 >= 0 as core::ffi::c_int && cbh4 >= 0 as core::ffi::c_int) {
        unreachable!();
    }
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4: core::ffi::c_int = bx4 >> ss_hor;
    let cby4: core::ffi::c_int = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [uint8_t; 4] = level_cache
        .offset(((by >> ss_ver) as ptrdiff_t * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < cbh4 {
        let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_0 < cbw4 {
            (*level_cache_ptr_0.offset(x_0 as isize))[2 as core::ffi::c_int as usize] = (*filter_level
                .offset(
                    2 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
            (*level_cache_ptr_0.offset(x_0 as isize))[3 as core::ffi::c_int as usize] = (*filter_level
                .offset(
                    3 as core::ffi::c_int as isize,
                ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize];
            x_0 += 1;
        }
        level_cache_ptr_0 = level_cache_ptr_0.offset(b4_stride as isize);
        y_0 += 1;
    }
    mask_edges_chroma(
        ((*lflvl).filter_uv).as_mut_ptr() as *mut [[[uint16_t; 2]; 2]; 32],
        cby4,
        cbx4,
        cbw4,
        cbh4,
        skip,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_calc_eih(
    lim_lut: *mut Av1FilterLUT,
    filter_sharpness: core::ffi::c_int,
) {
    let sharp: core::ffi::c_int = filter_sharpness;
    let mut level: core::ffi::c_int = 0 as core::ffi::c_int;
    while level < 64 as core::ffi::c_int {
        let mut limit: core::ffi::c_int = level;
        if sharp > 0 as core::ffi::c_int {
            limit >>= sharp + 3 as core::ffi::c_int >> 2 as core::ffi::c_int;
            limit = imin(limit, 9 as core::ffi::c_int - sharp);
        }
        limit = imax(limit, 1 as core::ffi::c_int);
        (*lim_lut).i[level as usize] = limit as uint8_t;
        (*lim_lut).e[level as usize] = (2 as core::ffi::c_int
            * (level + 2 as core::ffi::c_int) + limit) as uint8_t;
        level += 1;
    }
    (*lim_lut).sharp[0 as core::ffi::c_int as usize] = (sharp + 3 as core::ffi::c_int
        >> 2 as core::ffi::c_int) as uint64_t;
    (*lim_lut).sharp[1 as core::ffi::c_int as usize] = (if sharp != 0 {
        9 as core::ffi::c_int - sharp
    } else {
        0xff as core::ffi::c_int
    }) as uint64_t;
}
unsafe extern "C" fn calc_lf_value(
    lflvl_values: *mut [uint8_t; 2],
    base_lvl: core::ffi::c_int,
    lf_delta: core::ffi::c_int,
    seg_delta: core::ffi::c_int,
    mr_delta: *const Dav1dLoopfilterModeRefDeltas,
) {
    let base: core::ffi::c_int = iclip(
        iclip(base_lvl + lf_delta, 0 as core::ffi::c_int, 63 as core::ffi::c_int)
            + seg_delta,
        0 as core::ffi::c_int,
        63 as core::ffi::c_int,
    ) as core::ffi::c_int;
    if mr_delta.is_null() {
        memset(
            lflvl_values as *mut core::ffi::c_void,
            base,
            (::core::mem::size_of::<[uint8_t; 2]>() as size_t).wrapping_mul(8 as size_t),
        );
    } else {
        let sh: core::ffi::c_int = (base >= 32 as core::ffi::c_int) as core::ffi::c_int;
        let fresh16 = &raw mut (*lflvl_values
            .offset(0 as core::ffi::c_int as isize))[1 as core::ffi::c_int as usize];
        *fresh16 = iclip(
            base
                + (*mr_delta).ref_delta[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int * ((1 as core::ffi::c_int) << sh),
            0 as core::ffi::c_int,
            63 as core::ffi::c_int,
        ) as uint8_t;
        (*lflvl_values
            .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize] = *fresh16;
        let mut r: core::ffi::c_int = 1 as core::ffi::c_int;
        while r < 8 as core::ffi::c_int {
            let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
            while m < 2 as core::ffi::c_int {
                let delta: core::ffi::c_int = (*mr_delta).mode_delta[m as usize]
                    as core::ffi::c_int
                    + (*mr_delta).ref_delta[r as usize] as core::ffi::c_int;
                (*lflvl_values.offset(r as isize))[m as usize] = iclip(
                    base + delta * ((1 as core::ffi::c_int) << sh),
                    0 as core::ffi::c_int,
                    63 as core::ffi::c_int,
                ) as uint8_t;
                m += 1;
            }
            r += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn calc_lf_value_chroma(
    lflvl_values: *mut [uint8_t; 2],
    base_lvl: core::ffi::c_int,
    lf_delta: core::ffi::c_int,
    seg_delta: core::ffi::c_int,
    mr_delta: *const Dav1dLoopfilterModeRefDeltas,
) {
    if base_lvl == 0 {
        memset(
            lflvl_values as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<[uint8_t; 2]>() as size_t).wrapping_mul(8 as size_t),
        );
    } else {
        calc_lf_value(lflvl_values, base_lvl, lf_delta, seg_delta, mr_delta);
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_calc_lf_values(
    lflvl_values: *mut [[[uint8_t; 2]; 8]; 4],
    hdr: *const Dav1dFrameHeader,
    mut lf_delta: *const int8_t,
) {
    let n_seg: core::ffi::c_int = if (*hdr).segmentation.enabled as core::ffi::c_int != 0
    {
        8 as core::ffi::c_int
    } else {
        1 as core::ffi::c_int
    };
    if (*hdr).loopfilter.level_y[0 as core::ffi::c_int as usize] == 0
        && (*hdr).loopfilter.level_y[1 as core::ffi::c_int as usize] == 0
    {
        memset(
            lflvl_values as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<[[[uint8_t; 2]; 8]; 4]>() as size_t)
                .wrapping_mul(n_seg as size_t),
        );
        return;
    }
    let mr_deltas: *const Dav1dLoopfilterModeRefDeltas = if (*hdr)
        .loopfilter
        .mode_ref_delta_enabled as core::ffi::c_int != 0
    {
        &(*hdr).loopfilter.mode_ref_deltas
    } else {
        0 as *const Dav1dLoopfilterModeRefDeltas
    };
    let mut s: core::ffi::c_int = 0 as core::ffi::c_int;
    while s < n_seg {
        let segd: *const Dav1dSegmentationData = if (*hdr).segmentation.enabled
            as core::ffi::c_int != 0
        {
            &*((*hdr).segmentation.seg_data.d).as_ptr().offset(s as isize)
                as *const Dav1dSegmentationData
        } else {
            0 as *const Dav1dSegmentationData
        };
        calc_lf_value(
            ((*lflvl_values.offset(s as isize))[0 as core::ffi::c_int as usize])
                .as_mut_ptr() as *mut [uint8_t; 2],
            (*hdr).loopfilter.level_y[0 as core::ffi::c_int as usize]
                as core::ffi::c_int,
            *lf_delta.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_y_v as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            },
            mr_deltas,
        );
        calc_lf_value(
            ((*lflvl_values.offset(s as isize))[1 as core::ffi::c_int as usize])
                .as_mut_ptr() as *mut [uint8_t; 2],
            (*hdr).loopfilter.level_y[1 as core::ffi::c_int as usize]
                as core::ffi::c_int,
            *lf_delta
                .offset(
                    (if (*hdr).delta.lf.multi as core::ffi::c_int != 0 {
                        1 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as isize,
                ) as core::ffi::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_y_h as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            },
            mr_deltas,
        );
        calc_lf_value_chroma(
            ((*lflvl_values.offset(s as isize))[2 as core::ffi::c_int as usize])
                .as_mut_ptr() as *mut [uint8_t; 2],
            (*hdr).loopfilter.level_u as core::ffi::c_int,
            *lf_delta
                .offset(
                    (if (*hdr).delta.lf.multi as core::ffi::c_int != 0 {
                        2 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as isize,
                ) as core::ffi::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_u as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            },
            mr_deltas,
        );
        calc_lf_value_chroma(
            ((*lflvl_values.offset(s as isize))[3 as core::ffi::c_int as usize])
                .as_mut_ptr() as *mut [uint8_t; 2],
            (*hdr).loopfilter.level_v as core::ffi::c_int,
            *lf_delta
                .offset(
                    (if (*hdr).delta.lf.multi as core::ffi::c_int != 0 {
                        3 as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as isize,
                ) as core::ffi::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_v as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            },
            mr_deltas,
        );
        s += 1;
    }
}
