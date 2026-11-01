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

// Mechanically translated from upstream/dav1d/src/decode.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
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
    fn printf(_: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    static dav1d_memset_pow2: [dav1d_memset_pow2_fn; 6];
    fn dav1d_cdef_dsp_init_8bpc(c: *mut Dav1dCdefDSPContext);
    fn dav1d_cdef_dsp_init_16bpc(c: *mut Dav1dCdefDSPContext);
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn posix_memalign(
        __memptr: *mut *mut core::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> core::ffi::c_int;
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn llabs(_: core::ffi::c_longlong) -> core::ffi::c_longlong;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> core::ffi::c_int;
    fn pthread_cond_wait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
    ) -> core::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> core::ffi::c_int;
    fn dav1d_ref_create_using_pool(
        pool: *mut Dav1dMemPool,
        size: size_t,
    ) -> *mut Dav1dRef;
    fn dav1d_ref_dec(ref_0: *mut *mut Dav1dRef);
    fn dav1d_cdf_thread_init_static(cdf: *mut CdfThreadContext, qidx: core::ffi::c_uint);
    fn dav1d_cdf_thread_alloc(
        c: *mut Dav1dContext,
        cdf: *mut CdfThreadContext,
        have_frame_mt: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn dav1d_cdf_thread_copy(dst: *mut CdfContext, src: *const CdfThreadContext);
    fn dav1d_cdf_thread_ref(dst: *mut CdfThreadContext, src: *mut CdfThreadContext);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_cdf_thread_update(
        hdr: *const Dav1dFrameHeader,
        dst: *mut CdfContext,
        src: *const CdfContext,
    );
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    static mut dav1d_intra_edge_tree: [*const EdgeNode; 2];
    static dav1d_al_part_ctx: [[[uint8_t; 10]; 5]; 2];
    static dav1d_block_sizes: [[[uint8_t; 2]; 10]; 5];
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
    static dav1d_txfm_dimensions: [TxfmInfo; 19];
    static dav1d_max_txfm_size_for_bs: [[uint8_t; 4]; 22];
    static dav1d_comp_inter_pred_modes: [[uint8_t; 2]; 8];
    static dav1d_partition_type_count: [uint8_t; 5];
    static dav1d_ymode_size_context: [uint8_t; 22];
    static dav1d_filter_2d: [[uint8_t; 4]; 4];
    static dav1d_filter_dir: [[uint8_t; 2]; 10];
    static dav1d_intra_mode_context: [uint8_t; 13];
    static dav1d_wedge_ctx_lut: [uint8_t; 22];
    static dav1d_sgr_params: [[uint16_t; 2]; 16];
    fn dav1d_refmvs_init_frame(
        rf: *mut refmvs_frame,
        seq_hdr: *const Dav1dSequenceHeader,
        frm_hdr: *const Dav1dFrameHeader,
        ref_poc: *const uint8_t,
        rp: *mut refmvs_temporal_block,
        ref_ref_poc: *const [uint8_t; 7],
        rp_ref: *const *mut refmvs_temporal_block,
        n_tile_threads: core::ffi::c_int,
        n_frame_threads: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn dav1d_refmvs_tile_sbrow_init(
        rt: *mut refmvs_tile,
        rf: *const refmvs_frame,
        tile_col_start4: core::ffi::c_int,
        tile_col_end4: core::ffi::c_int,
        tile_row_start4: core::ffi::c_int,
        tile_row_end4: core::ffi::c_int,
        sby: core::ffi::c_int,
        tile_row_idx: core::ffi::c_int,
        pass: core::ffi::c_int,
    );
    fn dav1d_refmvs_find(
        rt: *const refmvs_tile,
        mvstack: *mut refmvs_candidate,
        cnt: *mut core::ffi::c_int,
        ctx: *mut core::ffi::c_int,
        ref_0: refmvs_refpair,
        bs: BlockSize,
        edge_flags: EdgeFlags,
        by4: core::ffi::c_int,
        bx4: core::ffi::c_int,
    );
    fn dav1d_film_grain_dsp_init_8bpc(c: *mut Dav1dFilmGrainDSPContext);
    fn dav1d_film_grain_dsp_init_16bpc(c: *mut Dav1dFilmGrainDSPContext);
    fn dav1d_intra_pred_dsp_init_8bpc(c: *mut Dav1dIntraPredDSPContext);
    fn dav1d_intra_pred_dsp_init_16bpc(c: *mut Dav1dIntraPredDSPContext);
    fn dav1d_itx_dsp_init_8bpc(c: *mut Dav1dInvTxfmDSPContext, bpc: core::ffi::c_int);
    fn dav1d_itx_dsp_init_16bpc(c: *mut Dav1dInvTxfmDSPContext, bpc: core::ffi::c_int);
    fn dav1d_create_lf_mask_intra(
        lflvl: *mut Av1Filter,
        level_cache: *mut [uint8_t; 4],
        b4_stride: ptrdiff_t,
        level: *const [[uint8_t; 2]; 8],
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
    );
    fn dav1d_create_lf_mask_inter(
        lflvl: *mut Av1Filter,
        level_cache: *mut [uint8_t; 4],
        b4_stride: ptrdiff_t,
        level: *const [[uint8_t; 2]; 8],
        bx: core::ffi::c_int,
        by: core::ffi::c_int,
        iw: core::ffi::c_int,
        ih: core::ffi::c_int,
        skip_inter: core::ffi::c_int,
        bs: BlockSize,
        max_ytx: RectTxfmSize,
        tx_mask: *const uint16_t,
        uvtx: RectTxfmSize,
        layout: Dav1dPixelLayout,
        ay: *mut uint8_t,
        ly: *mut uint8_t,
        auv: *mut uint8_t,
        luv: *mut uint8_t,
    );
    fn dav1d_calc_eih(lim_lut: *mut Av1FilterLUT, filter_sharpness: core::ffi::c_int);
    fn dav1d_calc_lf_values(
        values: *mut [[[uint8_t; 2]; 8]; 4],
        hdr: *const Dav1dFrameHeader,
        lf_delta: *const int8_t,
    );
    fn dav1d_loop_filter_dsp_init_8bpc(c: *mut Dav1dLoopFilterDSPContext);
    fn dav1d_loop_filter_dsp_init_16bpc(c: *mut Dav1dLoopFilterDSPContext);
    fn dav1d_loop_restoration_dsp_init_16bpc(
        c: *mut Dav1dLoopRestorationDSPContext,
        bpc: core::ffi::c_int,
    );
    fn dav1d_loop_restoration_dsp_init_8bpc(
        c: *mut Dav1dLoopRestorationDSPContext,
        bpc: core::ffi::c_int,
    );
    fn dav1d_mc_dsp_init_8bpc(c: *mut Dav1dMCDSPContext);
    fn dav1d_mc_dsp_init_16bpc(c: *mut Dav1dMCDSPContext);
    fn dav1d_msac_init(
        s: *mut MsacContext,
        data: *const uint8_t,
        sz: size_t,
        disable_cdf_update_flag: core::ffi::c_int,
    );
    fn dav1d_msac_decode_symbol_adapt_c(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
        n_symbols: size_t,
    ) -> core::ffi::c_uint;
    fn dav1d_msac_decode_bool_adapt_c(
        s: *mut MsacContext,
        cdf: *mut uint16_t,
    ) -> core::ffi::c_uint;
    fn dav1d_msac_decode_bool_equi_c(s: *mut MsacContext) -> core::ffi::c_uint;
    fn dav1d_msac_decode_bool_c(
        s: *mut MsacContext,
        f: core::ffi::c_uint,
    ) -> core::ffi::c_uint;
    fn dav1d_msac_decode_subexp(
        s: *mut MsacContext,
        ref_0: core::ffi::c_int,
        n: core::ffi::c_int,
        k: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn dav1d_thread_picture_alloc(
        c: *mut Dav1dContext,
        f: *mut Dav1dFrameContext,
        bpc: core::ffi::c_int,
    ) -> core::ffi::c_int;
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
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_picture_unref_internal(p: *mut Dav1dPicture);
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    fn dav1d_recon_b_intra_8bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        intra_edge_flags: EdgeFlags,
        b: *const Av1Block,
    );
    fn dav1d_recon_b_intra_16bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        intra_edge_flags: EdgeFlags,
        b: *const Av1Block,
    );
    fn dav1d_recon_b_inter_8bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        b: *const Av1Block,
    ) -> core::ffi::c_int;
    fn dav1d_recon_b_inter_16bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        b: *const Av1Block,
    ) -> core::ffi::c_int;
    fn dav1d_filter_sbrow_8bpc(f: *mut Dav1dFrameContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_16bpc(f: *mut Dav1dFrameContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_deblock_cols_8bpc(
        f: *mut Dav1dFrameContext,
        sby: core::ffi::c_int,
    );
    fn dav1d_filter_sbrow_deblock_cols_16bpc(
        f: *mut Dav1dFrameContext,
        sby: core::ffi::c_int,
    );
    fn dav1d_filter_sbrow_deblock_rows_8bpc(
        f: *mut Dav1dFrameContext,
        sby: core::ffi::c_int,
    );
    fn dav1d_filter_sbrow_deblock_rows_16bpc(
        f: *mut Dav1dFrameContext,
        sby: core::ffi::c_int,
    );
    fn dav1d_filter_sbrow_cdef_8bpc(tc: *mut Dav1dTaskContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_cdef_16bpc(tc: *mut Dav1dTaskContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_resize_8bpc(f: *mut Dav1dFrameContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_resize_16bpc(f: *mut Dav1dFrameContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_lr_8bpc(f: *mut Dav1dFrameContext, sby: core::ffi::c_int);
    fn dav1d_filter_sbrow_lr_16bpc(f: *mut Dav1dFrameContext, sby: core::ffi::c_int);
    fn dav1d_backup_ipred_edge_8bpc(t: *mut Dav1dTaskContext);
    fn dav1d_backup_ipred_edge_16bpc(t: *mut Dav1dTaskContext);
    fn dav1d_read_coef_blocks_8bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        b: *const Av1Block,
    );
    fn dav1d_read_coef_blocks_16bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        b: *const Av1Block,
    );
    fn dav1d_copy_pal_block_y_8bpc(
        t: *mut Dav1dTaskContext,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
        bw4: core::ffi::c_int,
        bh4: core::ffi::c_int,
    );
    fn dav1d_copy_pal_block_y_16bpc(
        t: *mut Dav1dTaskContext,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
        bw4: core::ffi::c_int,
        bh4: core::ffi::c_int,
    );
    fn dav1d_copy_pal_block_uv_8bpc(
        t: *mut Dav1dTaskContext,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
        bw4: core::ffi::c_int,
        bh4: core::ffi::c_int,
    );
    fn dav1d_copy_pal_block_uv_16bpc(
        t: *mut Dav1dTaskContext,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
        bw4: core::ffi::c_int,
        bh4: core::ffi::c_int,
    );
    fn dav1d_read_pal_plane_8bpc(
        t: *mut Dav1dTaskContext,
        b: *mut Av1Block,
        pl: core::ffi::c_int,
        sz_ctx: core::ffi::c_int,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
    );
    fn dav1d_read_pal_plane_16bpc(
        t: *mut Dav1dTaskContext,
        b: *mut Av1Block,
        pl: core::ffi::c_int,
        sz_ctx: core::ffi::c_int,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
    );
    fn dav1d_read_pal_uv_8bpc(
        t: *mut Dav1dTaskContext,
        b: *mut Av1Block,
        sz_ctx: core::ffi::c_int,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
    );
    fn dav1d_read_pal_uv_16bpc(
        t: *mut Dav1dTaskContext,
        b: *mut Av1Block,
        sz_ctx: core::ffi::c_int,
        bx4: core::ffi::c_int,
        by4: core::ffi::c_int,
    );
    static dav1d_dq_tbl: [[[uint16_t; 2]; 256]; 3];
    fn dav1d_log(c: *mut Dav1dContext, format: *const core::ffi::c_char, ...);
    static mut dav1d_qm_tbl: [[[*const uint8_t; 19]; 2]; 16];
    fn dav1d_task_create_tile_sbrow(
        f: *mut Dav1dFrameContext,
        pass: core::ffi::c_int,
        cond_signal: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn dav1d_task_frame_init(f: *mut Dav1dFrameContext);
    fn dav1d_get_shear_params(wm: *mut Dav1dWarpedMotionParams) -> core::ffi::c_int;
    fn dav1d_find_affine_int(
        pts: *const [[core::ffi::c_int; 2]; 2],
        np: core::ffi::c_int,
        bw4: core::ffi::c_int,
        bh4: core::ffi::c_int,
        mv: mv,
        wm: *mut Dav1dWarpedMotionParams,
        bx: core::ffi::c_int,
        by: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn dav1d_set_affine_mv2d(
        bw4: core::ffi::c_int,
        bh4: core::ffi::c_int,
        mv: mv,
        wm: *mut Dav1dWarpedMotionParams,
        bx: core::ffi::c_int,
        by: core::ffi::c_int,
    );
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
pub type size_t = __darwin_size_t;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type va_list = __darwin_va_list;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
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
pub type TxfmSize = core::ffi::c_uint;
pub const N_TX_SIZES: TxfmSize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;
pub type BlockLevel = core::ffi::c_uint;
pub const N_BL_LEVELS: BlockLevel = 5;
pub const BL_8X8: BlockLevel = 4;
pub const BL_16X16: BlockLevel = 3;
pub const BL_32X32: BlockLevel = 2;
pub const BL_64X64: BlockLevel = 1;
pub const BL_128X128: BlockLevel = 0;
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
pub type IntraPredMode = core::ffi::c_uint;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: IntraPredMode = 14;
pub const N_UV_INTRA_PRED_MODES: IntraPredMode = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: IntraPredMode = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;
pub type InterIntraPredMode = core::ffi::c_uint;
pub const N_INTER_INTRA_PRED_MODES: InterIntraPredMode = 4;
pub const II_SMOOTH_PRED: InterIntraPredMode = 3;
pub const II_HOR_PRED: InterIntraPredMode = 2;
pub const II_VERT_PRED: InterIntraPredMode = 1;
pub const II_DC_PRED: InterIntraPredMode = 0;
pub type BlockPartition = core::ffi::c_uint;
pub const N_SUB8X8_PARTITIONS: BlockPartition = 4;
pub const N_PARTITIONS: BlockPartition = 10;
pub const PARTITION_V4: BlockPartition = 9;
pub const PARTITION_H4: BlockPartition = 8;
pub const PARTITION_T_RIGHT_SPLIT: BlockPartition = 7;
pub const PARTITION_T_LEFT_SPLIT: BlockPartition = 6;
pub const PARTITION_T_BOTTOM_SPLIT: BlockPartition = 5;
pub const PARTITION_T_TOP_SPLIT: BlockPartition = 4;
pub const PARTITION_SPLIT: BlockPartition = 3;
pub const PARTITION_V: BlockPartition = 2;
pub const PARTITION_H: BlockPartition = 1;
pub const PARTITION_NONE: BlockPartition = 0;
pub type MVJoint = core::ffi::c_uint;
pub const N_MV_JOINTS: MVJoint = 4;
pub const MV_JOINT_HV: MVJoint = 3;
pub const MV_JOINT_V: MVJoint = 2;
pub const MV_JOINT_H: MVJoint = 1;
pub const MV_JOINT_ZERO: MVJoint = 0;
pub type InterPredMode = core::ffi::c_uint;
pub const N_INTER_PRED_MODES: InterPredMode = 4;
pub const NEWMV: InterPredMode = 3;
pub const GLOBALMV: InterPredMode = 2;
pub const NEARMV: InterPredMode = 1;
pub const NEARESTMV: InterPredMode = 0;
pub type DRL_PROXIMITY = core::ffi::c_uint;
pub const NEARISH_DRL: DRL_PROXIMITY = 3;
pub const NEAR_DRL: DRL_PROXIMITY = 2;
pub const NEARER_DRL: DRL_PROXIMITY = 1;
pub const NEAREST_DRL: DRL_PROXIMITY = 0;
pub type CompInterPredMode = core::ffi::c_uint;
pub const N_COMP_INTER_PRED_MODES: CompInterPredMode = 8;
pub const NEWMV_NEWMV: CompInterPredMode = 7;
pub const GLOBALMV_GLOBALMV: CompInterPredMode = 6;
pub const NEWMV_NEARMV: CompInterPredMode = 5;
pub const NEARMV_NEWMV: CompInterPredMode = 4;
pub const NEWMV_NEARESTMV: CompInterPredMode = 3;
pub const NEARESTMV_NEWMV: CompInterPredMode = 2;
pub const NEARMV_NEARMV: CompInterPredMode = 1;
pub const NEARESTMV_NEARESTMV: CompInterPredMode = 0;
pub type CompInterType = core::ffi::c_uint;
pub const COMP_INTER_WEDGE: CompInterType = 4;
pub const COMP_INTER_SEG: CompInterType = 3;
pub const COMP_INTER_AVG: CompInterType = 2;
pub const COMP_INTER_WEIGHTED_AVG: CompInterType = 1;
pub const COMP_INTER_NONE: CompInterType = 0;
pub type InterIntraType = core::ffi::c_uint;
pub const INTER_INTRA_WEDGE: InterIntraType = 2;
pub const INTER_INTRA_BLEND: InterIntraType = 1;
pub const INTER_INTRA_NONE: InterIntraType = 0;
pub type MotionMode = core::ffi::c_uint;
pub const MM_WARP: MotionMode = 2;
pub const MM_OBMC: MotionMode = 1;
pub const MM_TRANSLATION: MotionMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeNode {
    pub o: uint8_t,
    pub h: [uint8_t; 2],
    pub v: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeBranch {
    pub node: EdgeNode,
    pub h4: uint8_t,
    pub v4: uint8_t,
    pub split_offset: [uint16_t; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_candidate {
    pub mv: refmvs_mvpair,
    pub weight: core::ffi::c_int,
}
pub const __ATOMIC_SEQ_CST: core::ffi::c_int = 5 as core::ffi::c_int;
pub const INT16_MIN: core::ffi::c_int = -(32768 as core::ffi::c_int);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C" fn ctz(mask: core::ffi::c_uint) -> core::ffi::c_int {
    return mask.trailing_zeros() as i32;
}
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
unsafe extern "C" fn apply_sign(
    v: core::ffi::c_int,
    s: core::ffi::c_int,
) -> core::ffi::c_int {
    return if s < 0 as core::ffi::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign64(v: core::ffi::c_int, s: int64_t) -> core::ffi::c_int {
    return if s < 0 as int64_t { -v } else { v };
}
#[inline]
unsafe extern "C" fn ulog2(v: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ clz(v);
}
pub const DAV1D_MAX_SEGMENTS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const DAV1D_PRIMARY_REF_NONE: core::ffi::c_int = 7 as core::ffi::c_int;
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
unsafe extern "C" fn dav1d_ref_inc(ref_0: *mut Dav1dRef) {
    ::core::intrinsics::atomic_xadd::<_, _, {core::intrinsics::AtomicOrdering::Relaxed}>(
        &raw mut (*ref_0).ref_cnt,
        1 as core::ffi::c_int,
    );
}
static mut cfl_allowed_mask: core::ffi::c_uint = ((1 as core::ffi::c_int)
    << BS_32x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_32x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_32x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x4 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x4 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_4x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_4x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_4x4 as core::ffi::c_int) as core::ffi::c_uint;
static mut wedge_allowed_mask: core::ffi::c_uint = ((1 as core::ffi::c_int)
    << BS_32x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_32x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_32x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x8 as core::ffi::c_int) as core::ffi::c_uint;
static mut interintra_allowed_mask: core::ffi::c_uint = ((1 as core::ffi::c_int)
    << BS_32x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_32x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x32 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_16x8 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x16 as core::ffi::c_int
    | (1 as core::ffi::c_int) << BS_8x8 as core::ffi::c_int) as core::ffi::c_uint;
pub const INVALID_MV: core::ffi::c_uint = 0x80008000 as core::ffi::c_uint;
#[inline]
unsafe extern "C" fn dav1d_refmvs_save_tmvs(
    dsp: *const Dav1dRefmvsDSPContext,
    rt: *const refmvs_tile,
    col_start8: core::ffi::c_int,
    mut col_end8: core::ffi::c_int,
    row_start8: core::ffi::c_int,
    mut row_end8: core::ffi::c_int,
) {
    let rf: *const refmvs_frame = (*rt).rf;
    if !(row_start8 >= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"dav1d_refmvs_save_tmvs\0"))
                .as_ptr(),
            b"refmvs.h\0" as *const u8 as *const core::ffi::c_char,
            148 as core::ffi::c_int,
            b"row_start8 >= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !((row_end8 - row_start8) as core::ffi::c_uint <= 16 as core::ffi::c_uint)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 23],
                [core::ffi::c_char; 23],
            >(*b"dav1d_refmvs_save_tmvs\0"))
                .as_ptr(),
            b"refmvs.h\0" as *const u8 as *const core::ffi::c_char,
            149 as core::ffi::c_int,
            b"(unsigned) (row_end8 - row_start8) <= 16U\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    row_end8 = imin(row_end8, (*rf).ih8);
    col_end8 = imin(col_end8, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let ref_sign: *const uint8_t = ((*rf).mfmv_sign).as_ptr();
    let mut rp: *mut refmvs_temporal_block = &raw mut *((*rf).rp)
        .offset((row_start8 as ptrdiff_t * stride) as isize)
        as *mut refmvs_temporal_block;
    ((*dsp).save_tmvs)
        .expect(
            "non-null function pointer",
        )(
        rp,
        stride,
        ((*rt).r).as_ptr().offset(6 as core::ffi::c_int as isize),
        ref_sign,
        col_end8,
        row_end8,
        col_start8,
        row_start8,
    );
}
#[inline]
unsafe extern "C" fn get_intra_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    if have_left != 0 {
        if have_top != 0 {
            let ctx: core::ffi::c_int = (*l).intra[yb4 as usize] as core::ffi::c_int
                + (*a).intra[xb4 as usize] as core::ffi::c_int;
            return ctx + (ctx == 2 as core::ffi::c_int) as core::ffi::c_int;
        } else {
            return (*l).intra[yb4 as usize] as core::ffi::c_int * 2 as core::ffi::c_int
        }
    } else {
        return if have_top != 0 {
            (*a).intra[xb4 as usize] as core::ffi::c_int * 2 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }
    };
}
#[inline]
unsafe extern "C" fn get_tx_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    max_tx: *const TxfmInfo,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
) -> core::ffi::c_int {
    return ((*l).tx_intra[yb4 as usize] as core::ffi::c_int
        >= (*max_tx).lh as core::ffi::c_int) as core::ffi::c_int
        + ((*a).tx_intra[xb4 as usize] as core::ffi::c_int
            >= (*max_tx).lw as core::ffi::c_int) as core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_partition_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    bl: BlockLevel,
    yb8: core::ffi::c_int,
    xb8: core::ffi::c_int,
) -> core::ffi::c_int {
    return ((*a).partition[xb8 as usize] as core::ffi::c_int
        >> (4 as core::ffi::c_uint).wrapping_sub(bl as core::ffi::c_uint)
        & 1 as core::ffi::c_int)
        + (((*l).partition[yb8 as usize] as core::ffi::c_int
            >> (4 as core::ffi::c_uint).wrapping_sub(bl as core::ffi::c_uint)
            & 1 as core::ffi::c_int) << 1 as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn gather_left_partition_prob(
    in_0: *const uint16_t,
    bl: BlockLevel,
) -> core::ffi::c_uint {
    let mut out: core::ffi::c_uint = (*in_0
        .offset((PARTITION_H as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
        as core::ffi::c_int
        - *in_0.offset(PARTITION_H as core::ffi::c_int as isize) as core::ffi::c_int)
        as core::ffi::c_uint;
    out = out
        .wrapping_add(
            (*in_0
                .offset(
                    (PARTITION_SPLIT as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_int
                - *in_0.offset(PARTITION_T_LEFT_SPLIT as core::ffi::c_int as isize)
                    as core::ffi::c_int) as core::ffi::c_uint,
        );
    if bl as core::ffi::c_uint != BL_128X128 as core::ffi::c_int as core::ffi::c_uint {
        out = out
            .wrapping_add(
                (*in_0
                    .offset(
                        (PARTITION_H4 as core::ffi::c_int - 1 as core::ffi::c_int)
                            as isize,
                    ) as core::ffi::c_int
                    - *in_0.offset(PARTITION_H4 as core::ffi::c_int as isize)
                        as core::ffi::c_int) as core::ffi::c_uint,
            );
    }
    return out;
}
#[inline]
unsafe extern "C" fn gather_top_partition_prob(
    in_0: *const uint16_t,
    bl: BlockLevel,
) -> core::ffi::c_uint {
    let mut out: core::ffi::c_uint = (*in_0
        .offset((PARTITION_V as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
        as core::ffi::c_int
        - *in_0.offset(PARTITION_T_TOP_SPLIT as core::ffi::c_int as isize)
            as core::ffi::c_int) as core::ffi::c_uint;
    out = out
        .wrapping_add(
            *in_0
                .offset(
                    (PARTITION_T_LEFT_SPLIT as core::ffi::c_int - 1 as core::ffi::c_int)
                        as isize,
                ) as core::ffi::c_uint,
        );
    if bl as core::ffi::c_uint != BL_128X128 as core::ffi::c_int as core::ffi::c_uint {
        out = out
            .wrapping_add(
                (*in_0
                    .offset(
                        (PARTITION_V4 as core::ffi::c_int - 1 as core::ffi::c_int)
                            as isize,
                    ) as core::ffi::c_int
                    - *in_0.offset(PARTITION_T_RIGHT_SPLIT as core::ffi::c_int as isize)
                        as core::ffi::c_int) as core::ffi::c_uint,
            );
    }
    return out;
}
#[inline]
unsafe extern "C" fn get_filter_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    comp: core::ffi::c_int,
    dir: core::ffi::c_int,
    ref_0: core::ffi::c_int,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
) -> core::ffi::c_int {
    let a_filter: core::ffi::c_int = if (*a)
        .ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int == ref_0
        || (*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int
            == ref_0
    {
        (*a).filter[dir as usize][xb4 as usize] as core::ffi::c_int
    } else {
        DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int as core::ffi::c_int
    };
    let l_filter: core::ffi::c_int = if (*l)
        .ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int == ref_0
        || (*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int
            == ref_0
    {
        (*l).filter[dir as usize][yb4 as usize] as core::ffi::c_int
    } else {
        DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int as core::ffi::c_int
    };
    if a_filter == l_filter {
        return comp * 4 as core::ffi::c_int + a_filter
    } else if a_filter == DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int {
        return comp * 4 as core::ffi::c_int + l_filter
    } else if l_filter == DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int {
        return comp * 4 as core::ffi::c_int + a_filter
    } else {
        return comp * 4 as core::ffi::c_int
            + DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn get_comp_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    if have_top != 0 {
        if have_left != 0 {
            if (*a).comp_type[xb4 as usize] != 0 {
                if (*l).comp_type[yb4 as usize] != 0 {
                    return 4 as core::ffi::c_int
                } else {
                    return 2 as core::ffi::c_int
                        + ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
                            as core::ffi::c_uint >= 4 as core::ffi::c_uint)
                            as core::ffi::c_int
                }
            } else if (*l).comp_type[yb4 as usize] != 0 {
                return 2 as core::ffi::c_int
                    + ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                        as core::ffi::c_uint >= 4 as core::ffi::c_uint)
                        as core::ffi::c_int
            } else {
                return ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
                    as core::ffi::c_int >= 4 as core::ffi::c_int) as core::ffi::c_int
                    ^ ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                        as core::ffi::c_int >= 4 as core::ffi::c_int) as core::ffi::c_int
            }
        } else {
            return if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0 {
                3 as core::ffi::c_int
            } else {
                ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                    as core::ffi::c_int >= 4 as core::ffi::c_int) as core::ffi::c_int
            }
        }
    } else if have_left != 0 {
        return if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0 {
            3 as core::ffi::c_int
        } else {
            ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int
                >= 4 as core::ffi::c_int) as core::ffi::c_int
        }
    } else {
        return 1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn get_comp_dir_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    if have_top != 0 && have_left != 0 {
        let a_intra: core::ffi::c_int = (*a).intra[xb4 as usize] as core::ffi::c_int;
        let l_intra: core::ffi::c_int = (*l).intra[yb4 as usize] as core::ffi::c_int;
        if a_intra != 0 && l_intra != 0 {
            return 2 as core::ffi::c_int;
        }
        if a_intra != 0 || l_intra != 0 {
            let edge: *const BlockContext = if a_intra != 0 { l } else { a };
            let off: core::ffi::c_int = if a_intra != 0 { yb4 } else { xb4 };
            if (*edge).comp_type[off as usize] as core::ffi::c_int
                == COMP_INTER_NONE as core::ffi::c_int
            {
                return 2 as core::ffi::c_int;
            }
            return 1 as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * ((((*edge).ref_0[0 as core::ffi::c_int as usize][off as usize]
                        as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int
                        == (((*edge).ref_0[1 as core::ffi::c_int as usize][off as usize]
                            as core::ffi::c_int) < 4 as core::ffi::c_int)
                            as core::ffi::c_int) as core::ffi::c_int;
        }
        let a_comp: core::ffi::c_int = ((*a).comp_type[xb4 as usize] as core::ffi::c_int
            != COMP_INTER_NONE as core::ffi::c_int) as core::ffi::c_int;
        let l_comp: core::ffi::c_int = ((*l).comp_type[yb4 as usize] as core::ffi::c_int
            != COMP_INTER_NONE as core::ffi::c_int) as core::ffi::c_int;
        let a_ref0: core::ffi::c_int = (*a)
            .ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int;
        let l_ref0: core::ffi::c_int = (*l)
            .ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int;
        if a_comp == 0 && l_comp == 0 {
            return 1 as core::ffi::c_int
                + 2 as core::ffi::c_int
                    * ((a_ref0 >= 4 as core::ffi::c_int) as core::ffi::c_int
                        == (l_ref0 >= 4 as core::ffi::c_int) as core::ffi::c_int)
                        as core::ffi::c_int
        } else if a_comp == 0 || l_comp == 0 {
            let edge_0: *const BlockContext = if a_comp != 0 { a } else { l };
            let off_0: core::ffi::c_int = if a_comp != 0 { xb4 } else { yb4 };
            if !((((*edge_0).ref_0[0 as core::ffi::c_int as usize][off_0 as usize]
                as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int
                == (((*edge_0).ref_0[1 as core::ffi::c_int as usize][off_0 as usize]
                    as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int)
            {
                return 1 as core::ffi::c_int;
            }
            return 3 as core::ffi::c_int
                + ((a_ref0 >= 4 as core::ffi::c_int) as core::ffi::c_int
                    == (l_ref0 >= 4 as core::ffi::c_int) as core::ffi::c_int)
                    as core::ffi::c_int;
        } else {
            let a_uni: core::ffi::c_int = ((((*a)
                .ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int)
                < 4 as core::ffi::c_int) as core::ffi::c_int
                == (((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                    as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int)
                as core::ffi::c_int;
            let l_uni: core::ffi::c_int = ((((*l)
                .ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int)
                < 4 as core::ffi::c_int) as core::ffi::c_int
                == (((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                    as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int)
                as core::ffi::c_int;
            if a_uni == 0 && l_uni == 0 {
                return 0 as core::ffi::c_int;
            }
            if a_uni == 0 || l_uni == 0 {
                return 2 as core::ffi::c_int;
            }
            return 3 as core::ffi::c_int
                + ((a_ref0 == 4 as core::ffi::c_int) as core::ffi::c_int
                    == (l_ref0 == 4 as core::ffi::c_int) as core::ffi::c_int)
                    as core::ffi::c_int;
        }
    } else if have_top != 0 || have_left != 0 {
        let edge_1: *const BlockContext = if have_left != 0 { l } else { a };
        let off_1: core::ffi::c_int = if have_left != 0 { yb4 } else { xb4 };
        if (*edge_1).intra[off_1 as usize] != 0 {
            return 2 as core::ffi::c_int;
        }
        if (*edge_1).comp_type[off_1 as usize] as core::ffi::c_int
            == COMP_INTER_NONE as core::ffi::c_int
        {
            return 2 as core::ffi::c_int;
        }
        return 4 as core::ffi::c_int
            * ((((*edge_1).ref_0[0 as core::ffi::c_int as usize][off_1 as usize]
                as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int
                == (((*edge_1).ref_0[1 as core::ffi::c_int as usize][off_1 as usize]
                    as core::ffi::c_int) < 4 as core::ffi::c_int) as core::ffi::c_int)
                as core::ffi::c_int;
    } else {
        return 2 as core::ffi::c_int
    };
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
unsafe extern "C" fn get_jnt_comp_ctx(
    order_hint_n_bits: core::ffi::c_int,
    poc: core::ffi::c_int,
    ref0poc: core::ffi::c_int,
    ref1poc: core::ffi::c_int,
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
) -> core::ffi::c_int {
    let d0: core::ffi::c_int = abs(get_poc_diff(order_hint_n_bits, ref0poc, poc))
        as core::ffi::c_int;
    let d1: core::ffi::c_int = abs(get_poc_diff(order_hint_n_bits, poc, ref1poc))
        as core::ffi::c_int;
    let offset: core::ffi::c_int = (d0 == d1) as core::ffi::c_int;
    let a_ctx: core::ffi::c_int = ((*a).comp_type[xb4 as usize] as core::ffi::c_int
        >= COMP_INTER_AVG as core::ffi::c_int
        || (*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int
            == 6 as core::ffi::c_int) as core::ffi::c_int;
    let l_ctx: core::ffi::c_int = ((*l).comp_type[yb4 as usize] as core::ffi::c_int
        >= COMP_INTER_AVG as core::ffi::c_int
        || (*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int
            == 6 as core::ffi::c_int) as core::ffi::c_int;
    return 3 as core::ffi::c_int * offset + a_ctx + l_ctx;
}
#[inline]
unsafe extern "C" fn get_mask_comp_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
) -> core::ffi::c_int {
    let a_ctx: core::ffi::c_int = if (*a).comp_type[xb4 as usize] as core::ffi::c_int
        >= COMP_INTER_SEG as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else if (*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
        as core::ffi::c_int == 6 as core::ffi::c_int
    {
        3 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    let l_ctx: core::ffi::c_int = if (*l).comp_type[yb4 as usize] as core::ffi::c_int
        >= COMP_INTER_SEG as core::ffi::c_int
    {
        1 as core::ffi::c_int
    } else if (*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
        as core::ffi::c_int == 6 as core::ffi::c_int
    {
        3 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
    return imin(a_ctx + l_ctx, 5 as core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn av1_get_ref_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    mut have_top: core::ffi::c_int,
    mut have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        cnt[((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int
            >= 4 as core::ffi::c_int) as core::ffi::c_int as usize] += 1;
        if (*a).comp_type[xb4 as usize] != 0 {
            cnt[((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int >= 4 as core::ffi::c_int) as core::ffi::c_int
                as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        cnt[((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int
            >= 4 as core::ffi::c_int) as core::ffi::c_int as usize] += 1;
        if (*l).comp_type[yb4 as usize] != 0 {
            cnt[((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int >= 4 as core::ffi::c_int) as core::ffi::c_int
                as usize] += 1;
        }
    }
    return if cnt[0 as core::ffi::c_int as usize] == cnt[1 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[0 as core::ffi::c_int as usize] < cnt[1 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn av1_get_fwd_ref_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 4] = [0 as core::ffi::c_int; 4];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        if ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int)
            < 4 as core::ffi::c_int
        {
            cnt[(*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as usize] += 1;
        }
        if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0
            && ((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int) < 4 as core::ffi::c_int
        {
            cnt[(*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize] as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        if ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int)
            < 4 as core::ffi::c_int
        {
            cnt[(*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as usize] += 1;
        }
        if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0
            && ((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int) < 4 as core::ffi::c_int
        {
            cnt[(*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize] as usize] += 1;
        }
    }
    cnt[0 as core::ffi::c_int as usize] += cnt[1 as core::ffi::c_int as usize];
    cnt[2 as core::ffi::c_int as usize] += cnt[3 as core::ffi::c_int as usize];
    return if cnt[0 as core::ffi::c_int as usize] == cnt[2 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[0 as core::ffi::c_int as usize] < cnt[2 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn av1_get_fwd_ref_1_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        if ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int)
            < 2 as core::ffi::c_int
        {
            cnt[(*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as usize] += 1;
        }
        if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0
            && ((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int) < 2 as core::ffi::c_int
        {
            cnt[(*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize] as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        if ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int)
            < 2 as core::ffi::c_int
        {
            cnt[(*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as usize] += 1;
        }
        if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0
            && ((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int) < 2 as core::ffi::c_int
        {
            cnt[(*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize] as usize] += 1;
        }
    }
    return if cnt[0 as core::ffi::c_int as usize] == cnt[1 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[0 as core::ffi::c_int as usize] < cnt[1 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn av1_get_fwd_ref_2_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 2] = [0 as core::ffi::c_int; 2];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        if ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_uint
            ^ 2 as core::ffi::c_uint) < 2 as core::ffi::c_uint
        {
            cnt[((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 2 as core::ffi::c_int) as usize] += 1;
        }
        if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0
            && ((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_uint ^ 2 as core::ffi::c_uint) < 2 as core::ffi::c_uint
        {
            cnt[((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 2 as core::ffi::c_int) as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        if ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_uint
            ^ 2 as core::ffi::c_uint) < 2 as core::ffi::c_uint
        {
            cnt[((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 2 as core::ffi::c_int) as usize] += 1;
        }
        if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0
            && ((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_uint ^ 2 as core::ffi::c_uint) < 2 as core::ffi::c_uint
        {
            cnt[((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 2 as core::ffi::c_int) as usize] += 1;
        }
    }
    return if cnt[0 as core::ffi::c_int as usize] == cnt[1 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[0 as core::ffi::c_int as usize] < cnt[1 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn av1_get_bwd_ref_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 3] = [0 as core::ffi::c_int; 3];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        if (*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int
            >= 4 as core::ffi::c_int
        {
            cnt[((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
        if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0
            && (*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int >= 4 as core::ffi::c_int
        {
            cnt[((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        if (*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int
            >= 4 as core::ffi::c_int
        {
            cnt[((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
        if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0
            && (*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int >= 4 as core::ffi::c_int
        {
            cnt[((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
    }
    cnt[1 as core::ffi::c_int as usize] += cnt[0 as core::ffi::c_int as usize];
    return if cnt[2 as core::ffi::c_int as usize] == cnt[1 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[1 as core::ffi::c_int as usize] < cnt[2 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn av1_get_bwd_ref_1_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 3] = [0 as core::ffi::c_int; 3];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        if (*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize] as core::ffi::c_int
            >= 4 as core::ffi::c_int
        {
            cnt[((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
        if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0
            && (*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int >= 4 as core::ffi::c_int
        {
            cnt[((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        if (*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize] as core::ffi::c_int
            >= 4 as core::ffi::c_int
        {
            cnt[((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
        if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0
            && (*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int >= 4 as core::ffi::c_int
        {
            cnt[((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 4 as core::ffi::c_int) as usize] += 1;
        }
    }
    return if cnt[0 as core::ffi::c_int as usize] == cnt[1 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[0 as core::ffi::c_int as usize] < cnt[1 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn av1_get_uni_p1_ctx(
    a: *const BlockContext,
    l: *const BlockContext,
    yb4: core::ffi::c_int,
    xb4: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: [core::ffi::c_int; 3] = [0 as core::ffi::c_int; 3];
    if have_top != 0 && (*a).intra[xb4 as usize] == 0 {
        if ((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
            as core::ffi::c_uint)
            .wrapping_sub(1 as core::ffi::c_uint) < 3 as core::ffi::c_uint
        {
            cnt[((*a).ref_0[0 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 1 as core::ffi::c_int) as usize] += 1;
        }
        if (*a).comp_type[xb4 as usize] as core::ffi::c_int != 0
            && ((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_uint)
                .wrapping_sub(1 as core::ffi::c_uint) < 3 as core::ffi::c_uint
        {
            cnt[((*a).ref_0[1 as core::ffi::c_int as usize][xb4 as usize]
                as core::ffi::c_int - 1 as core::ffi::c_int) as usize] += 1;
        }
    }
    if have_left != 0 && (*l).intra[yb4 as usize] == 0 {
        if ((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
            as core::ffi::c_uint)
            .wrapping_sub(1 as core::ffi::c_uint) < 3 as core::ffi::c_uint
        {
            cnt[((*l).ref_0[0 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 1 as core::ffi::c_int) as usize] += 1;
        }
        if (*l).comp_type[yb4 as usize] as core::ffi::c_int != 0
            && ((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_uint)
                .wrapping_sub(1 as core::ffi::c_uint) < 3 as core::ffi::c_uint
        {
            cnt[((*l).ref_0[1 as core::ffi::c_int as usize][yb4 as usize]
                as core::ffi::c_int - 1 as core::ffi::c_int) as usize] += 1;
        }
    }
    cnt[1 as core::ffi::c_int as usize] += cnt[2 as core::ffi::c_int as usize];
    return if cnt[0 as core::ffi::c_int as usize] == cnt[1 as core::ffi::c_int as usize]
    {
        1 as core::ffi::c_int
    } else if cnt[0 as core::ffi::c_int as usize] < cnt[1 as core::ffi::c_int as usize] {
        0 as core::ffi::c_int
    } else {
        2 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn get_drl_context(
    ref_mv_stack: *const refmvs_candidate,
    ref_idx: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*ref_mv_stack.offset(ref_idx as isize)).weight >= 640 as core::ffi::c_int {
        return ((*ref_mv_stack.offset((ref_idx + 1 as core::ffi::c_int) as isize)).weight
            < 640 as core::ffi::c_int) as core::ffi::c_int;
    }
    return if (*ref_mv_stack.offset((ref_idx + 1 as core::ffi::c_int) as isize)).weight
        < 640 as core::ffi::c_int
    {
        2 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn get_cur_frame_segid(
    by: core::ffi::c_int,
    bx: core::ffi::c_int,
    have_top: core::ffi::c_int,
    have_left: core::ffi::c_int,
    seg_ctx: *mut core::ffi::c_int,
    mut cur_seg_map: *const uint8_t,
    stride: ptrdiff_t,
) -> core::ffi::c_uint {
    cur_seg_map = cur_seg_map
        .offset((bx as ptrdiff_t + by as ptrdiff_t * stride) as isize);
    if have_left != 0 && have_top != 0 {
        let l: core::ffi::c_int = *cur_seg_map.offset(-(1 as core::ffi::c_int) as isize)
            as core::ffi::c_int;
        let a: core::ffi::c_int = *cur_seg_map.offset(-stride as isize)
            as core::ffi::c_int;
        let al: core::ffi::c_int = *cur_seg_map
            .offset(-(stride + 1 as ptrdiff_t) as isize) as core::ffi::c_int;
        if l == a && al == l {
            *seg_ctx = 2 as core::ffi::c_int;
        } else if l == a || al == l || a == al {
            *seg_ctx = 1 as core::ffi::c_int;
        } else {
            *seg_ctx = 0 as core::ffi::c_int;
        }
        return (if a == al { a } else { l }) as core::ffi::c_uint;
    } else {
        *seg_ctx = 0 as core::ffi::c_int;
        return (if have_left != 0 {
            *cur_seg_map.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
        } else if have_top != 0 {
            *cur_seg_map.offset(-stride as isize) as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as core::ffi::c_uint;
    };
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
                == (*gmv).matrix[2 as core::ffi::c_int as usize]) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 11],
                        [core::ffi::c_char; 11],
                    >(*b"get_gmv_2d\0"))
                        .as_ptr(),
                    b"env.h\0" as *const u8 as *const core::ffi::c_char,
                    485 as core::ffi::c_int,
                    b"gmv->matrix[5] == gmv->matrix[2]\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
            if !((*gmv).matrix[4 as core::ffi::c_int as usize]
                == -(*gmv).matrix[3 as core::ffi::c_int as usize]) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 11],
                        [core::ffi::c_char; 11],
                    >(*b"get_gmv_2d\0"))
                        .as_ptr(),
                    b"env.h\0" as *const u8 as *const core::ffi::c_char,
                    486 as core::ffi::c_int,
                    b"gmv->matrix[4] == -gmv->matrix[3]\0" as *const u8
                        as *const core::ffi::c_char,
                );
            } else {};
        }
        1 => {
            let mut res_0: mv = mv {
                c2rust_unnamed: {
                    let mut init = C2RustUnnamed_24 {
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
                    let mut init = C2RustUnnamed_24 {
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
            let mut init = C2RustUnnamed_24 {
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
#[inline]
unsafe extern "C" fn dav1d_msac_decode_bools(
    s: *mut MsacContext,
    mut n: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut v: core::ffi::c_uint = 0 as core::ffi::c_uint;
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        v = v << 1 as core::ffi::c_int | dav1d_msac_decode_bool_equi_c(s);
    }
    return v;
}
#[inline]
unsafe extern "C" fn dav1d_msac_decode_uniform(
    s: *mut MsacContext,
    n: core::ffi::c_uint,
) -> core::ffi::c_int {
    if !(n > 0 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"dav1d_msac_decode_uniform\0"))
                .as_ptr(),
            b"msac.h\0" as *const u8 as *const core::ffi::c_char,
            102 as core::ffi::c_int,
            b"n > 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let l: core::ffi::c_int = ulog2(n) as core::ffi::c_int + 1 as core::ffi::c_int;
    if !(l > 1 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 26],
                [core::ffi::c_char; 26],
            >(*b"dav1d_msac_decode_uniform\0"))
                .as_ptr(),
            b"msac.h\0" as *const u8 as *const core::ffi::c_char,
            104 as core::ffi::c_int,
            b"l > 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let m: core::ffi::c_uint = (((1 as core::ffi::c_int) << l) as core::ffi::c_uint)
        .wrapping_sub(n);
    let v: core::ffi::c_uint = dav1d_msac_decode_bools(
        s,
        (l - 1 as core::ffi::c_int) as core::ffi::c_uint,
    ) as core::ffi::c_uint;
    return (if v < m {
        v
    } else {
        (v << 1 as core::ffi::c_int)
            .wrapping_sub(m)
            .wrapping_add(dav1d_msac_decode_bool_equi_c(s))
    }) as core::ffi::c_int;
}
pub const FRAME_ERROR: core::ffi::c_uint = UINT_MAX.wrapping_sub(1 as core::ffi::c_uint);
unsafe extern "C" fn init_quant_tables(
    seq_hdr: *const Dav1dSequenceHeader,
    frame_hdr: *const Dav1dFrameHeader,
    qidx: core::ffi::c_int,
    mut dq: *mut [[uint16_t; 2]; 3],
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i
        < (if (*frame_hdr).segmentation.enabled as core::ffi::c_int != 0 {
            8 as core::ffi::c_int
        } else {
            1 as core::ffi::c_int
        })
    {
        let yac: core::ffi::c_int = if (*frame_hdr).segmentation.enabled
            as core::ffi::c_int != 0
        {
            iclip_u8(
                qidx
                    + (*frame_hdr).segmentation.seg_data.d[i as usize].delta_q
                        as core::ffi::c_int,
            ) as core::ffi::c_int
        } else {
            qidx
        };
        let ydc: core::ffi::c_int = iclip_u8(
            yac + (*frame_hdr).quant.ydc_delta as core::ffi::c_int,
        ) as core::ffi::c_int;
        let uac: core::ffi::c_int = iclip_u8(
            yac + (*frame_hdr).quant.uac_delta as core::ffi::c_int,
        ) as core::ffi::c_int;
        let udc: core::ffi::c_int = iclip_u8(
            yac + (*frame_hdr).quant.udc_delta as core::ffi::c_int,
        ) as core::ffi::c_int;
        let vac: core::ffi::c_int = iclip_u8(
            yac + (*frame_hdr).quant.vac_delta as core::ffi::c_int,
        ) as core::ffi::c_int;
        let vdc: core::ffi::c_int = iclip_u8(
            yac + (*frame_hdr).quant.vdc_delta as core::ffi::c_int,
        ) as core::ffi::c_int;
        (*dq
            .offset(
                i as isize,
            ))[0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = dav1d_dq_tbl[(*seq_hdr)
            .hbd as usize][ydc as usize][0 as core::ffi::c_int as usize];
        (*dq
            .offset(
                i as isize,
            ))[0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = dav1d_dq_tbl[(*seq_hdr)
            .hbd as usize][yac as usize][1 as core::ffi::c_int as usize];
        (*dq
            .offset(
                i as isize,
            ))[1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = dav1d_dq_tbl[(*seq_hdr)
            .hbd as usize][udc as usize][0 as core::ffi::c_int as usize];
        (*dq
            .offset(
                i as isize,
            ))[1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = dav1d_dq_tbl[(*seq_hdr)
            .hbd as usize][uac as usize][1 as core::ffi::c_int as usize];
        (*dq
            .offset(
                i as isize,
            ))[2 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = dav1d_dq_tbl[(*seq_hdr)
            .hbd as usize][vdc as usize][0 as core::ffi::c_int as usize];
        (*dq
            .offset(
                i as isize,
            ))[2 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = dav1d_dq_tbl[(*seq_hdr)
            .hbd as usize][vac as usize][1 as core::ffi::c_int as usize];
        i += 1;
    }
}
unsafe extern "C" fn read_mv_component_diff(
    msac: *mut MsacContext,
    mv_comp: *mut CdfMvComponent,
    mv_prec: core::ffi::c_int,
) -> core::ffi::c_int {
    let sign: core::ffi::c_int = dav1d_msac_decode_bool_adapt_c(
        msac,
        ((*mv_comp).sign).as_mut_ptr(),
    ) as core::ffi::c_int;
    let cl: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
        msac,
        ((*mv_comp).classes).as_mut_ptr(),
        10 as size_t,
    ) as core::ffi::c_int;
    let mut up: core::ffi::c_int = 0;
    let mut fp: core::ffi::c_int = 3 as core::ffi::c_int;
    let mut hp: core::ffi::c_int = 1 as core::ffi::c_int;
    if cl == 0 {
        up = dav1d_msac_decode_bool_adapt_c(msac, ((*mv_comp).class0).as_mut_ptr())
            as core::ffi::c_int;
        if mv_prec >= 0 as core::ffi::c_int {
            fp = dav1d_msac_decode_symbol_adapt_c(
                msac,
                ((*mv_comp).class0_fp[up as usize]).as_mut_ptr(),
                3 as size_t,
            ) as core::ffi::c_int;
            if mv_prec > 0 as core::ffi::c_int {
                hp = dav1d_msac_decode_bool_adapt_c(
                    msac,
                    ((*mv_comp).class0_hp).as_mut_ptr(),
                ) as core::ffi::c_int;
            }
        }
    } else {
        up = (1 as core::ffi::c_int) << cl;
        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
        while n < cl {
            up = (up as core::ffi::c_uint
                | dav1d_msac_decode_bool_adapt_c(
                    msac,
                    ((*mv_comp).classN[n as usize]).as_mut_ptr(),
                ) << n) as core::ffi::c_int;
            n += 1;
        }
        if mv_prec >= 0 as core::ffi::c_int {
            fp = dav1d_msac_decode_symbol_adapt_c(
                msac,
                ((*mv_comp).classN_fp).as_mut_ptr(),
                3 as size_t,
            ) as core::ffi::c_int;
            if mv_prec > 0 as core::ffi::c_int {
                hp = dav1d_msac_decode_bool_adapt_c(
                    msac,
                    ((*mv_comp).classN_hp).as_mut_ptr(),
                ) as core::ffi::c_int;
            }
        }
    }
    let diff: core::ffi::c_int = (up << 3 as core::ffi::c_int
        | fp << 1 as core::ffi::c_int | hp) + 1 as core::ffi::c_int;
    return if sign != 0 { -diff } else { diff };
}
unsafe extern "C" fn read_mv_residual(
    ts: *mut Dav1dTileState,
    ref_mv: *mut mv,
    mv_prec: core::ffi::c_int,
) {
    let msac: *mut MsacContext = &raw mut (*ts).msac;
    let mv_joint: MVJoint = dav1d_msac_decode_symbol_adapt_c(
        msac,
        ((*ts).cdf.mv.joint).as_mut_ptr(),
        (N_MV_JOINTS as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
    ) as MVJoint;
    if mv_joint as core::ffi::c_uint
        & MV_JOINT_V as core::ffi::c_int as core::ffi::c_uint != 0
    {
        (*ref_mv).c2rust_unnamed.y = ((*ref_mv).c2rust_unnamed.y as core::ffi::c_int
            + read_mv_component_diff(
                msac,
                &raw mut *((*ts).cdf.mv.comp)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                mv_prec,
            )) as int16_t;
    }
    if mv_joint as core::ffi::c_uint
        & MV_JOINT_H as core::ffi::c_int as core::ffi::c_uint != 0
    {
        (*ref_mv).c2rust_unnamed.x = ((*ref_mv).c2rust_unnamed.x as core::ffi::c_int
            + read_mv_component_diff(
                msac,
                &raw mut *((*ts).cdf.mv.comp)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize),
                mv_prec,
            )) as int16_t;
    }
}
unsafe extern "C" fn read_tx_tree(
    t: *mut Dav1dTaskContext,
    from: RectTxfmSize,
    depth: core::ffi::c_int,
    masks: *mut uint16_t,
    x_off: core::ffi::c_int,
    y_off: core::ffi::c_int,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let bx4: core::ffi::c_int = (*t).bx & 31 as core::ffi::c_int;
    let by4: core::ffi::c_int = (*t).by & 31 as core::ffi::c_int;
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(from as isize)
        as *const TxfmInfo;
    let txw: core::ffi::c_int = (*t_dim).lw as core::ffi::c_int;
    let txh: core::ffi::c_int = (*t_dim).lh as core::ffi::c_int;
    let mut is_split: core::ffi::c_int = 0;
    if depth < 2 as core::ffi::c_int
        && from as core::ffi::c_uint > TX_4X4 as core::ffi::c_int as core::ffi::c_uint
    {
        let cat: core::ffi::c_int = 2 as core::ffi::c_int
            * (TX_64X64 as core::ffi::c_int as core::ffi::c_int
                - (*t_dim).max as core::ffi::c_int) - depth;
        let a: core::ffi::c_int = (((*(*t).a).tx[bx4 as usize] as core::ffi::c_int)
            < txw) as core::ffi::c_int;
        let l: core::ffi::c_int = (((*t).l.tx[by4 as usize] as core::ffi::c_int) < txh)
            as core::ffi::c_int;
        is_split = dav1d_msac_decode_bool_adapt_c(
            &raw mut (*(*t).ts).msac,
            ((*(*t).ts).cdf.m.txpart[cat as usize][(a + l) as usize]).as_mut_ptr(),
        ) as core::ffi::c_int;
        if is_split != 0 {
            let fresh8 = &raw mut *masks.offset(depth as isize);
            *fresh8 = (*fresh8 as core::ffi::c_int
                | (1 as core::ffi::c_int) << y_off * 4 as core::ffi::c_int + x_off)
                as uint16_t;
        }
    } else {
        is_split = 0 as core::ffi::c_int;
    }
    if is_split != 0 && (*t_dim).max as core::ffi::c_int > TX_8X8 as core::ffi::c_int {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let sub_t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions
            .as_ptr()
            .offset(sub as isize) as *const TxfmInfo;
        let txsw: core::ffi::c_int = (*sub_t_dim).w as core::ffi::c_int;
        let txsh: core::ffi::c_int = (*sub_t_dim).h as core::ffi::c_int;
        read_tx_tree(
            t,
            sub,
            depth + 1 as core::ffi::c_int,
            masks,
            x_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
            y_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
        );
        (*t).bx += txsw;
        if txw >= txh && (*t).bx < (*f).bw {
            read_tx_tree(
                t,
                sub,
                depth + 1 as core::ffi::c_int,
                masks,
                x_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                y_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
            );
        }
        (*t).bx -= txsw;
        (*t).by += txsh;
        if txh >= txw && (*t).by < (*f).bh {
            read_tx_tree(
                t,
                sub,
                depth + 1 as core::ffi::c_int,
                masks,
                x_off * 2 as core::ffi::c_int + 0 as core::ffi::c_int,
                y_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
            );
            (*t).bx += txsw;
            if txw >= txh && (*t).bx < (*f).bw {
                read_tx_tree(
                    t,
                    sub,
                    depth + 1 as core::ffi::c_int,
                    masks,
                    x_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                    y_off * 2 as core::ffi::c_int + 1 as core::ffi::c_int,
                );
            }
            (*t).bx -= txsw;
        }
        (*t).by -= txsh;
    } else {
        (dav1d_memset_pow2[(*t_dim).lw as usize])
            .expect(
                "non-null function pointer",
            )(
            &raw mut *((*(*t).a).tx).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                as *mut core::ffi::c_void,
            if is_split != 0 { TX_4X4 as core::ffi::c_int } else { txw },
        );
        (dav1d_memset_pow2[(*t_dim).lh as usize])
            .expect(
                "non-null function pointer",
            )(
            &raw mut *((*t).l.tx).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                as *mut core::ffi::c_void,
            if is_split != 0 { TX_4X4 as core::ffi::c_int } else { txh },
        );
    };
}
unsafe extern "C" fn neg_deinterleave(
    mut diff: core::ffi::c_int,
    mut ref_0: core::ffi::c_int,
    mut max: core::ffi::c_int,
) -> core::ffi::c_int {
    if ref_0 == 0 {
        return diff;
    }
    if ref_0 >= max - 1 as core::ffi::c_int {
        return max - diff - 1 as core::ffi::c_int;
    }
    if 2 as core::ffi::c_int * ref_0 < max {
        if diff <= 2 as core::ffi::c_int * ref_0 {
            if diff & 1 as core::ffi::c_int != 0 {
                return ref_0 + (diff + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            } else {
                return ref_0 - (diff >> 1 as core::ffi::c_int)
            }
        }
        return diff;
    } else {
        if diff <= 2 as core::ffi::c_int * (max - ref_0 - 1 as core::ffi::c_int) {
            if diff & 1 as core::ffi::c_int != 0 {
                return ref_0 + (diff + 1 as core::ffi::c_int >> 1 as core::ffi::c_int)
            } else {
                return ref_0 - (diff >> 1 as core::ffi::c_int)
            }
        }
        return max - (diff + 1 as core::ffi::c_int);
    };
}
unsafe extern "C" fn find_matching_ref(
    t: *const Dav1dTaskContext,
    intra_edge_flags: EdgeFlags,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
    w4: core::ffi::c_int,
    h4: core::ffi::c_int,
    have_left: core::ffi::c_int,
    have_top: core::ffi::c_int,
    ref_0: core::ffi::c_int,
    mut masks: *mut uint64_t,
) {
    let mut r: *const *mut refmvs_block = &*((*t).rt.r)
        .as_ptr()
        .offset((((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize)
        as *const *mut refmvs_block;
    let mut count: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut have_topleft: core::ffi::c_int = (have_top != 0 && have_left != 0)
        as core::ffi::c_int;
    let mut have_topright: core::ffi::c_int = (imax(bw4, bh4) < 32 as core::ffi::c_int
        && have_top != 0 && (*t).bx + bw4 < (*(*t).ts).tiling.col_end
        && intra_edge_flags as core::ffi::c_uint
            & EDGE_I444_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint != 0)
        as core::ffi::c_int;
    if have_top != 0 {
        let mut r2: *const refmvs_block = &raw mut *(*r
            .offset(-(1 as core::ffi::c_int) as isize))
            .offset((*t).bx as isize) as *mut refmvs_block;
        if (*r2).ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == ref_0 + 1 as core::ffi::c_int
            && (*r2).ref_0.ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == -(1 as core::ffi::c_int)
        {
            *masks.offset(0 as core::ffi::c_int as isize) |= 1 as uint64_t;
            count = 1 as core::ffi::c_int;
        }
        let mut aw4: core::ffi::c_int = dav1d_block_dimensions[(*r2).bs
            as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int;
        if aw4 >= bw4 {
            let off: core::ffi::c_int = (*t).bx & aw4 - 1 as core::ffi::c_int;
            if off != 0 {
                have_topleft = 0 as core::ffi::c_int;
            }
            if aw4 - off > bw4 {
                have_topright = 0 as core::ffi::c_int;
            }
        } else {
            let mut mask: core::ffi::c_uint = ((1 as core::ffi::c_int) << aw4)
                as core::ffi::c_uint;
            let mut x: core::ffi::c_int = aw4;
            while x < w4 {
                r2 = r2.offset(aw4 as isize);
                if (*r2).ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == ref_0 + 1 as core::ffi::c_int
                    && (*r2).ref_0.ref_0[1 as core::ffi::c_int as usize]
                        as core::ffi::c_int == -(1 as core::ffi::c_int)
                {
                    *masks.offset(0 as core::ffi::c_int as isize) |= mask as uint64_t;
                    count += 1;
                    if count >= 8 as core::ffi::c_int {
                        return;
                    }
                }
                aw4 = dav1d_block_dimensions[(*r2).bs
                    as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int;
                mask <<= aw4;
                x += aw4;
            }
        }
    }
    if have_left != 0 {
        let mut r2_0: *const *mut refmvs_block = r;
        if (*(*r2_0.offset(0 as core::ffi::c_int as isize))
            .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
            .ref_0
            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == ref_0 + 1 as core::ffi::c_int
            && (*(*r2_0.offset(0 as core::ffi::c_int as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .ref_0
                .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == -(1 as core::ffi::c_int)
        {
            *masks.offset(1 as core::ffi::c_int as isize) |= 1 as uint64_t;
            count += 1;
            if count >= 8 as core::ffi::c_int {
                return;
            }
        }
        let mut lh4: core::ffi::c_int = dav1d_block_dimensions[(*(*r2_0
            .offset(0 as core::ffi::c_int as isize))
            .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
            .bs as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int;
        if lh4 >= bh4 {
            if (*t).by & lh4 - 1 as core::ffi::c_int != 0 {
                have_topleft = 0 as core::ffi::c_int;
            }
        } else {
            let mut mask_0: core::ffi::c_uint = ((1 as core::ffi::c_int) << lh4)
                as core::ffi::c_uint;
            let mut y: core::ffi::c_int = lh4;
            while y < h4 {
                r2_0 = r2_0.offset(lh4 as isize);
                if (*(*r2_0.offset(0 as core::ffi::c_int as isize))
                    .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                    .ref_0
                    .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == ref_0 + 1 as core::ffi::c_int
                    && (*(*r2_0.offset(0 as core::ffi::c_int as isize))
                        .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                        .ref_0
                        .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                        == -(1 as core::ffi::c_int)
                {
                    *masks.offset(1 as core::ffi::c_int as isize) |= mask_0 as uint64_t;
                    count += 1;
                    if count >= 8 as core::ffi::c_int {
                        return;
                    }
                }
                lh4 = dav1d_block_dimensions[(*(*r2_0
                    .offset(0 as core::ffi::c_int as isize))
                    .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                    .bs as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int;
                mask_0 <<= lh4;
                y += lh4;
            }
        }
    }
    if have_topleft != 0
        && ((*(*r.offset(-(1 as core::ffi::c_int) as isize))
            .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
            .ref_0
            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == ref_0 + 1 as core::ffi::c_int
            && (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .ref_0
                .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == -(1 as core::ffi::c_int))
    {
        let fresh9 = &raw mut *masks.offset(1 as core::ffi::c_int as isize);
        *fresh9 = (*fresh9 as core::ffi::c_ulonglong
            | (1 as core::ffi::c_ulonglong) << 32 as core::ffi::c_int) as uint64_t;
        count += 1;
        if count >= 8 as core::ffi::c_int {
            return;
        }
    }
    if have_topright != 0
        && ((*(*r.offset(-(1 as core::ffi::c_int) as isize))
            .offset(((*t).bx + bw4) as isize))
            .ref_0
            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
            == ref_0 + 1 as core::ffi::c_int
            && (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx + bw4) as isize))
                .ref_0
                .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                == -(1 as core::ffi::c_int))
    {
        let fresh10 = &raw mut *masks.offset(0 as core::ffi::c_int as isize);
        *fresh10 = (*fresh10 as core::ffi::c_ulonglong
            | (1 as core::ffi::c_ulonglong) << 32 as core::ffi::c_int) as uint64_t;
    }
}
unsafe extern "C" fn derive_warpmv(
    t: *const Dav1dTaskContext,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
    mut masks: *const uint64_t,
    mv: mv,
    wmp: *mut Dav1dWarpedMotionParams,
) {
    let mut pts: [[[core::ffi::c_int; 2]; 2]; 8] = [[[0; 2]; 2]; 8];
    let mut np: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut r: *const *mut refmvs_block = &*((*t).rt.r)
        .as_ptr()
        .offset((((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize)
        as *const *mut refmvs_block;
    if *masks.offset(0 as core::ffi::c_int as isize) as core::ffi::c_uint
        == 1 as core::ffi::c_uint
        && *masks.offset(1 as core::ffi::c_int as isize) >> 32 as core::ffi::c_int == 0
    {
        let off: core::ffi::c_int = (*t).bx
            & dav1d_block_dimensions[(*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset((*t).bx as isize))
                .bs as usize][0 as core::ffi::c_int as usize] as core::ffi::c_int
                - 1 as core::ffi::c_int;
        pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * -off
                + 1 as core::ffi::c_int
                    * dav1d_block_dimensions[(*(*r
                        .offset(-(1 as core::ffi::c_int) as isize))
                        .offset((*t).bx as isize))
                        .bs as usize][0 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                + -(1 as core::ffi::c_int)
                    * dav1d_block_dimensions[(*(*r
                        .offset(-(1 as core::ffi::c_int) as isize))
                        .offset((*t).bx as isize))
                        .bs as usize][1 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            + (*(*r.offset(-(1 as core::ffi::c_int) as isize)).offset((*t).bx as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
            + (*(*r.offset(-(1 as core::ffi::c_int) as isize)).offset((*t).bx as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y as core::ffi::c_int;
        np += 1;
    } else {
        let mut off_0: core::ffi::c_uint = 0 as core::ffi::c_uint;
        let mut xmask: core::ffi::c_uint = *masks.offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_uint;
        while np < 8 as core::ffi::c_int && xmask != 0 {
            let tz: core::ffi::c_int = ctz(xmask) as core::ffi::c_int;
            off_0 = off_0.wrapping_add(tz as core::ffi::c_uint);
            xmask >>= tz;
            pts[np
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = (16 as core::ffi::c_uint)
                .wrapping_mul(
                    (2 as core::ffi::c_uint)
                        .wrapping_mul(off_0)
                        .wrapping_add(
                            (1 as core::ffi::c_int
                                * dav1d_block_dimensions[(*(*r
                                    .offset(-(1 as core::ffi::c_int) as isize))
                                    .offset(
                                        ((*t).bx as core::ffi::c_uint).wrapping_add(off_0) as isize,
                                    ))
                                    .bs as usize][0 as core::ffi::c_int as usize]
                                    as core::ffi::c_int) as core::ffi::c_uint,
                        ),
                )
                .wrapping_sub(8 as core::ffi::c_uint) as core::ffi::c_int;
            pts[np
                as usize][0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = 16 as core::ffi::c_int
                * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                    + -(1 as core::ffi::c_int)
                        * dav1d_block_dimensions[(*(*r
                            .offset(-(1 as core::ffi::c_int) as isize))
                            .offset(
                                ((*t).bx as core::ffi::c_uint).wrapping_add(off_0) as isize,
                            ))
                            .bs as usize][1 as core::ffi::c_int as usize]
                            as core::ffi::c_int) - 8 as core::ffi::c_int;
            pts[np
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = pts[np
                as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                + (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                    .offset(((*t).bx as core::ffi::c_uint).wrapping_add(off_0) as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int;
            pts[np
                as usize][1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = pts[np
                as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                + (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                    .offset(((*t).bx as core::ffi::c_uint).wrapping_add(off_0) as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int;
            np += 1;
            xmask &= !(1 as core::ffi::c_int) as core::ffi::c_uint;
        }
    }
    if np < 8 as core::ffi::c_int
        && *masks.offset(1 as core::ffi::c_int as isize) == 1 as uint64_t
    {
        let off_1: core::ffi::c_int = (*t).by
            & dav1d_block_dimensions[(*(*r.offset(0 as core::ffi::c_int as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .bs as usize][1 as core::ffi::c_int as usize] as core::ffi::c_int
                - 1 as core::ffi::c_int;
        pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                + -(1 as core::ffi::c_int)
                    * dav1d_block_dimensions[(*(*r.offset(-off_1 as isize))
                        .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                        .bs as usize][0 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * -off_1
                + 1 as core::ffi::c_int
                    * dav1d_block_dimensions[(*(*r.offset(-off_1 as isize))
                        .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                        .bs as usize][1 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            + (*(*r.offset(-off_1 as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
            + (*(*r.offset(-off_1 as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y as core::ffi::c_int;
        np += 1;
    } else {
        let mut off_2: core::ffi::c_uint = 0 as core::ffi::c_uint;
        let mut ymask: core::ffi::c_uint = *masks.offset(1 as core::ffi::c_int as isize)
            as core::ffi::c_uint;
        while np < 8 as core::ffi::c_int && ymask != 0 {
            let tz_0: core::ffi::c_int = ctz(ymask) as core::ffi::c_int;
            off_2 = off_2.wrapping_add(tz_0 as core::ffi::c_uint);
            ymask >>= tz_0;
            pts[np
                as usize][0 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = 16 as core::ffi::c_int
                * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                    + -(1 as core::ffi::c_int)
                        * dav1d_block_dimensions[(*(*r.offset(off_2 as isize))
                            .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                            .bs as usize][0 as core::ffi::c_int as usize]
                            as core::ffi::c_int) - 8 as core::ffi::c_int;
            pts[np
                as usize][0 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = (16 as core::ffi::c_uint)
                .wrapping_mul(
                    (2 as core::ffi::c_uint)
                        .wrapping_mul(off_2)
                        .wrapping_add(
                            (1 as core::ffi::c_int
                                * dav1d_block_dimensions[(*(*r.offset(off_2 as isize))
                                    .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                                    .bs as usize][1 as core::ffi::c_int as usize]
                                    as core::ffi::c_int) as core::ffi::c_uint,
                        ),
                )
                .wrapping_sub(8 as core::ffi::c_uint) as core::ffi::c_int;
            pts[np
                as usize][1 as core::ffi::c_int
                as usize][0 as core::ffi::c_int as usize] = pts[np
                as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                + (*(*r.offset(off_2 as isize))
                    .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int;
            pts[np
                as usize][1 as core::ffi::c_int
                as usize][1 as core::ffi::c_int as usize] = pts[np
                as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
                + (*(*r.offset(off_2 as isize))
                    .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int;
            np += 1;
            ymask &= !(1 as core::ffi::c_int) as core::ffi::c_uint;
        }
    }
    if np < 8 as core::ffi::c_int
        && *masks.offset(1 as core::ffi::c_int as isize) >> 32 as core::ffi::c_int != 0
    {
        pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                + -(1 as core::ffi::c_int)
                    * dav1d_block_dimensions[(*(*r
                        .offset(-(1 as core::ffi::c_int) as isize))
                        .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                        .bs as usize][0 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                + -(1 as core::ffi::c_int)
                    * dav1d_block_dimensions[(*(*r
                        .offset(-(1 as core::ffi::c_int) as isize))
                        .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                        .bs as usize][1 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            + (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
            + (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y as core::ffi::c_int;
        np += 1;
    }
    if np < 8 as core::ffi::c_int
        && *masks.offset(0 as core::ffi::c_int as isize) >> 32 as core::ffi::c_int != 0
    {
        pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * bw4
                + 1 as core::ffi::c_int
                    * dav1d_block_dimensions[(*(*r
                        .offset(-(1 as core::ffi::c_int) as isize))
                        .offset(((*t).bx + bw4) as isize))
                        .bs as usize][0 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = 16
            as core::ffi::c_int
            * (2 as core::ffi::c_int * 0 as core::ffi::c_int
                + -(1 as core::ffi::c_int)
                    * dav1d_block_dimensions[(*(*r
                        .offset(-(1 as core::ffi::c_int) as isize))
                        .offset(((*t).bx + bw4) as isize))
                        .bs as usize][1 as core::ffi::c_int as usize]
                        as core::ffi::c_int) - 8 as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
            + (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx + bw4) as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x as core::ffi::c_int;
        pts[np
            as usize][1 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize] = pts[np
            as usize][0 as core::ffi::c_int as usize][1 as core::ffi::c_int as usize]
            + (*(*r.offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx + bw4) as isize))
                .mv
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y as core::ffi::c_int;
        np += 1;
    }
    if !(np > 0 as core::ffi::c_int && np <= 8 as core::ffi::c_int) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"derive_warpmv\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            305 as core::ffi::c_int,
            b"np > 0 && np <= 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut mvd: [core::ffi::c_int; 8] = [0; 8];
    let mut ret: core::ffi::c_int = 0 as core::ffi::c_int;
    let thresh: core::ffi::c_int = 4 as core::ffi::c_int
        * iclip(imax(bw4, bh4), 4 as core::ffi::c_int, 28 as core::ffi::c_int)
            as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < np {
        mvd[i as usize] = abs(
            pts[i
                as usize][1 as core::ffi::c_int as usize][0 as core::ffi::c_int as usize]
                - pts[i
                    as usize][0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                - mv.c2rust_unnamed.x as core::ffi::c_int,
        )
            + abs(
                pts[i
                    as usize][1 as core::ffi::c_int
                    as usize][1 as core::ffi::c_int as usize]
                    - pts[i
                        as usize][0 as core::ffi::c_int
                        as usize][1 as core::ffi::c_int as usize]
                    - mv.c2rust_unnamed.y as core::ffi::c_int,
            );
        if mvd[i as usize] > thresh {
            mvd[i as usize] = -(1 as core::ffi::c_int);
        } else {
            ret += 1;
        }
        i += 1;
    }
    if ret == 0 {
        ret = 1 as core::ffi::c_int;
    } else {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut j: core::ffi::c_int = np - 1 as core::ffi::c_int;
        let mut k: core::ffi::c_int = 0 as core::ffi::c_int;
        while k < np - ret {
            while mvd[i_0 as usize] != -(1 as core::ffi::c_int) {
                i_0 += 1;
            }
            while mvd[j as usize] == -(1 as core::ffi::c_int) {
                j -= 1;
            }
            if !(i_0 != j) as core::ffi::c_int as core::ffi::c_long != 0 {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"derive_warpmv\0"))
                        .as_ptr(),
                    b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                    324 as core::ffi::c_int,
                    b"i != j\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            if i_0 > j {
                break;
            }
            mvd[i_0 as usize] = mvd[j as usize];
            memcpy(
                (pts[i_0 as usize]).as_mut_ptr() as *mut core::ffi::c_void,
                (pts[j as usize]).as_mut_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[[core::ffi::c_int; 2]; 2]>() as size_t,
            );
            k += 1;
            i_0 += 1;
            j -= 1;
        }
    }
    if dav1d_find_affine_int(
        pts.as_mut_ptr() as *const [[core::ffi::c_int; 2]; 2],
        ret,
        bw4,
        bh4,
        mv as mv,
        wmp,
        (*t).bx,
        (*t).by,
    ) == 0 && dav1d_get_shear_params(wmp) == 0
    {
        (*wmp).type_0 = DAV1D_WM_TYPE_AFFINE;
    } else {
        (*wmp).type_0 = DAV1D_WM_TYPE_IDENTITY;
    };
}
#[inline]
unsafe extern "C" fn findoddzero(
    mut buf: *const uint8_t,
    mut len: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < len {
        if *buf.offset((n * 2 as core::ffi::c_int) as isize) == 0 {
            return 1 as core::ffi::c_int;
        }
        n += 1;
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn order_palette(
    mut pal_idx: *const uint8_t,
    stride: ptrdiff_t,
    i: core::ffi::c_int,
    first: core::ffi::c_int,
    last: core::ffi::c_int,
    order: *mut [uint8_t; 8],
    ctx: *mut uint8_t,
) {
    let mut have_top: core::ffi::c_int = (i > first) as core::ffi::c_int;
    if pal_idx.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 14],
                [core::ffi::c_char; 14],
            >(*b"order_palette\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            359 as core::ffi::c_int,
            b"pal_idx\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    pal_idx = pal_idx
        .offset((first as ptrdiff_t + (i - first) as ptrdiff_t * stride) as isize);
    let mut j: core::ffi::c_int = first;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while j >= last {
        let have_left: core::ffi::c_int = (j > 0 as core::ffi::c_int)
            as core::ffi::c_int;
        if !(have_left != 0 || have_top != 0) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"order_palette\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                364 as core::ffi::c_int,
                b"have_left || have_top\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut mask: core::ffi::c_uint = 0 as core::ffi::c_uint;
        let mut o_idx: core::ffi::c_int = 0 as core::ffi::c_int;
        if have_left == 0 {
            *ctx.offset(n as isize) = 0 as uint8_t;
            let v: core::ffi::c_int = *pal_idx.offset(-stride as isize)
                as core::ffi::c_int;
            if !((v as core::ffi::c_uint) < 8 as core::ffi::c_uint) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"order_palette\0"))
                        .as_ptr(),
                    b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                    377 as core::ffi::c_int,
                    b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let fresh11 = o_idx;
            o_idx = o_idx + 1;
            (*order.offset(n as isize))[fresh11 as usize] = v as uint8_t;
            mask |= ((1 as core::ffi::c_int) << v) as core::ffi::c_uint;
        } else if have_top == 0 {
            *ctx.offset(n as isize) = 0 as uint8_t;
            let v_0: core::ffi::c_int = *pal_idx
                .offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int;
            if !((v_0 as core::ffi::c_uint) < 8 as core::ffi::c_uint) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 14],
                        [core::ffi::c_char; 14],
                    >(*b"order_palette\0"))
                        .as_ptr(),
                    b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                    380 as core::ffi::c_int,
                    b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            let fresh12 = o_idx;
            o_idx = o_idx + 1;
            (*order.offset(n as isize))[fresh12 as usize] = v_0 as uint8_t;
            mask |= ((1 as core::ffi::c_int) << v_0) as core::ffi::c_uint;
        } else {
            let l: core::ffi::c_int = *pal_idx.offset(-(1 as core::ffi::c_int) as isize)
                as core::ffi::c_int;
            let t: core::ffi::c_int = *pal_idx.offset(-stride as isize)
                as core::ffi::c_int;
            let tl: core::ffi::c_int = *pal_idx
                .offset(-(stride + 1 as ptrdiff_t) as isize) as core::ffi::c_int;
            let same_t_l: core::ffi::c_int = (t == l) as core::ffi::c_int;
            let same_t_tl: core::ffi::c_int = (t == tl) as core::ffi::c_int;
            let same_l_tl: core::ffi::c_int = (l == tl) as core::ffi::c_int;
            let same_all: core::ffi::c_int = same_t_l & same_t_tl & same_l_tl;
            if same_all != 0 {
                *ctx.offset(n as isize) = 4 as uint8_t;
                let v_1: core::ffi::c_int = t;
                if !((v_1 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        390 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh13 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh13 as usize] = v_1 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_1) as core::ffi::c_uint;
            } else if same_t_l != 0 {
                *ctx.offset(n as isize) = 3 as uint8_t;
                let v_2: core::ffi::c_int = t;
                if !((v_2 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        393 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh14 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh14 as usize] = v_2 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_2) as core::ffi::c_uint;
                let v_3: core::ffi::c_int = tl;
                if !((v_3 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        394 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh15 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh15 as usize] = v_3 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_3) as core::ffi::c_uint;
            } else if same_t_tl | same_l_tl != 0 {
                *ctx.offset(n as isize) = 2 as uint8_t;
                let v_4: core::ffi::c_int = tl;
                if !((v_4 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        397 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh16 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh16 as usize] = v_4 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_4) as core::ffi::c_uint;
                let v_5: core::ffi::c_int = if same_t_tl != 0 { l } else { t };
                if !((v_5 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        398 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh17 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh17 as usize] = v_5 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_5) as core::ffi::c_uint;
            } else {
                *ctx.offset(n as isize) = 1 as uint8_t;
                let v_6: core::ffi::c_int = imin(t, l) as core::ffi::c_int;
                if !((v_6 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        401 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh18 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh18 as usize] = v_6 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_6) as core::ffi::c_uint;
                let v_7: core::ffi::c_int = imax(t, l) as core::ffi::c_int;
                if !((v_7 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        402 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh19 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh19 as usize] = v_7 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_7) as core::ffi::c_uint;
                let v_8: core::ffi::c_int = tl;
                if !((v_8 as core::ffi::c_uint) < 8 as core::ffi::c_uint)
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 14],
                            [core::ffi::c_char; 14],
                        >(*b"order_palette\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        403 as core::ffi::c_int,
                        b"(unsigned)v < 8U\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                let fresh20 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh20 as usize] = v_8 as uint8_t;
                mask |= ((1 as core::ffi::c_int) << v_8) as core::ffi::c_uint;
            }
        }
        let mut m: core::ffi::c_uint = 1 as core::ffi::c_uint;
        let mut bit: core::ffi::c_uint = 0 as core::ffi::c_uint;
        while m < 0x100 as core::ffi::c_uint {
            if mask & m == 0 {
                let fresh21 = o_idx;
                o_idx = o_idx + 1;
                (*order.offset(n as isize))[fresh21 as usize] = bit as uint8_t;
            }
            m <<= 1 as core::ffi::c_int;
            bit = bit.wrapping_add(1);
        }
        if !(o_idx == 8 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 14],
                    [core::ffi::c_char; 14],
                >(*b"order_palette\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                409 as core::ffi::c_int,
                b"o_idx == 8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        have_top = 1 as core::ffi::c_int;
        j -= 1;
        n += 1;
        pal_idx = pal_idx.offset((stride - 1 as ptrdiff_t) as isize);
    }
}
unsafe extern "C" fn read_pal_indices(
    t: *mut Dav1dTaskContext,
    pal_idx: *mut uint8_t,
    pal_sz: core::ffi::c_int,
    pl: core::ffi::c_int,
    w4: core::ffi::c_int,
    h4: core::ffi::c_int,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
) {
    let ts: *mut Dav1dTileState = (*t).ts;
    let stride: ptrdiff_t = (bw4 * 4 as core::ffi::c_int) as ptrdiff_t;
    if pal_idx.is_null() as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"read_pal_indices\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            422 as core::ffi::c_int,
            b"pal_idx\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let pal_tmp: *mut uint8_t = ((*t).scratch.c2rust_unnamed_0.pal_idx_uv).as_mut_ptr();
    *pal_tmp.offset(0 as core::ffi::c_int as isize) = dav1d_msac_decode_uniform(
        &raw mut (*ts).msac,
        pal_sz as core::ffi::c_uint,
    ) as uint8_t;
    let color_map_cdf: *mut [uint16_t; 8] = ((*ts)
        .cdf
        .m
        .color_map[pl as usize][(pal_sz - 2 as core::ffi::c_int) as usize])
        .as_mut_ptr() as *mut [uint16_t; 8];
    let order: *mut [uint8_t; 8] = ((*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed
        .c2rust_unnamed
        .pal_order)
        .as_mut_ptr() as *mut [uint8_t; 8];
    let ctx: *mut uint8_t = ((*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed
        .c2rust_unnamed
        .pal_ctx)
        .as_mut_ptr();
    let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int * (w4 + h4) - 1 as core::ffi::c_int {
        let first: core::ffi::c_int = imin(
            i,
            w4 * 4 as core::ffi::c_int - 1 as core::ffi::c_int,
        ) as core::ffi::c_int;
        let last: core::ffi::c_int = imax(
            0 as core::ffi::c_int,
            i - h4 * 4 as core::ffi::c_int + 1 as core::ffi::c_int,
        ) as core::ffi::c_int;
        order_palette(pal_tmp, stride, i, first, last, order, ctx);
        let mut j: core::ffi::c_int = first;
        let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
        while j >= last {
            let color_idx: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
                &raw mut (*ts).msac,
                (*color_map_cdf.offset(*ctx.offset(m as isize) as isize)).as_mut_ptr(),
                (pal_sz - 1 as core::ffi::c_int) as size_t,
            ) as core::ffi::c_int;
            *pal_tmp.offset(((i - j) as ptrdiff_t * stride + j as ptrdiff_t) as isize) = (*order
                .offset(m as isize))[color_idx as usize];
            j -= 1;
            m += 1;
        }
        i += 1;
    }
    ((*(*t).c).pal_dsp.pal_idx_finish)
        .expect(
            "non-null function pointer",
        )(
        pal_idx,
        pal_tmp,
        bw4 * 4 as core::ffi::c_int,
        bh4 * 4 as core::ffi::c_int,
        w4 * 4 as core::ffi::c_int,
        h4 * 4 as core::ffi::c_int,
    );
}
unsafe extern "C" fn read_vartx_tree(
    t: *mut Dav1dTaskContext,
    b: *mut Av1Block,
    bs: BlockSize,
    bx4: core::ffi::c_int,
    by4: core::ffi::c_int,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: core::ffi::c_int = *b_dim.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let bh4: core::ffi::c_int = *b_dim.offset(1 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let mut tx_split: [uint16_t; 2] = [0 as core::ffi::c_int as uint16_t, 0];
    (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx = dav1d_max_txfm_size_for_bs[bs
        as usize][0 as core::ffi::c_int as usize];
    if (*b).skip == 0
        && ((*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize]
            as core::ffi::c_int != 0
            || (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as core::ffi::c_int
                == TX_4X4 as core::ffi::c_int)
    {
        (*b).uvtx = TX_4X4 as core::ffi::c_int as uint8_t;
        (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx = (*b).uvtx;
        if (*(*f).frame_hdr).txfm_mode as core::ffi::c_uint
            == DAV1D_TX_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
        {
            (dav1d_memset_pow2[*b_dim.offset(2 as core::ffi::c_int as isize) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*(*t).a).tx).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut core::ffi::c_void,
                TX_4X4 as core::ffi::c_int,
            );
            (dav1d_memset_pow2[*b_dim.offset(3 as core::ffi::c_int as isize) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*t).l.tx).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut core::ffi::c_void,
                TX_4X4 as core::ffi::c_int,
            );
        }
    } else if (*(*f).frame_hdr).txfm_mode as core::ffi::c_uint
        != DAV1D_TX_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
        || (*b).skip as core::ffi::c_int != 0
    {
        if (*(*f).frame_hdr).txfm_mode as core::ffi::c_uint
            == DAV1D_TX_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
        {
            (dav1d_memset_pow2[*b_dim.offset(2 as core::ffi::c_int as isize) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*(*t).a).tx).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut core::ffi::c_void,
                *b_dim.offset((2 as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
            );
            (dav1d_memset_pow2[*b_dim.offset(3 as core::ffi::c_int as isize) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*t).l.tx).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut core::ffi::c_void,
                *b_dim.offset((2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int,
            );
        }
        (*b).uvtx = dav1d_max_txfm_size_for_bs[bs as usize][(*f).cur.p.layout as usize];
    } else {
        if !(bw4 <= 16 as core::ffi::c_int || bh4 <= 16 as core::ffi::c_int
            || (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as core::ffi::c_int
                == TX_64X64 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 16],
                    [core::ffi::c_char; 16],
                >(*b"read_vartx_tree\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                471 as core::ffi::c_int,
                b"bw4 <= 16 || bh4 <= 16 || b->max_ytx == TX_64X64\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let mut y: core::ffi::c_int = 0;
        let mut x: core::ffi::c_int = 0;
        let mut y_off: core::ffi::c_int = 0;
        let mut x_off: core::ffi::c_int = 0;
        let ytx: *const TxfmInfo = &*dav1d_txfm_dimensions
            .as_ptr()
            .offset((*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as isize)
            as *const TxfmInfo;
        y = 0 as core::ffi::c_int;
        y_off = 0 as core::ffi::c_int;
        while y < bh4 {
            x = 0 as core::ffi::c_int;
            x_off = 0 as core::ffi::c_int;
            while x < bw4 {
                read_tx_tree(
                    t,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                    0 as core::ffi::c_int,
                    tx_split.as_mut_ptr(),
                    x_off,
                    y_off,
                );
                (*t).bx += (*ytx).w as core::ffi::c_int;
                x += (*ytx).w as core::ffi::c_int;
                x_off += 1;
            }
            (*t).bx -= x;
            (*t).by += (*ytx).h as core::ffi::c_int;
            y += (*ytx).h as core::ffi::c_int;
            y_off += 1;
        }
        (*t).by -= y;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-vartxtree[%x/%x]: r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                tx_split[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                tx_split[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*(*t).ts).msac.rng,
            );
        }
        (*b).uvtx = dav1d_max_txfm_size_for_bs[bs as usize][(*f).cur.p.layout as usize];
    }
    if (tx_split[0 as core::ffi::c_int as usize] as core::ffi::c_int
        & !(0x33 as core::ffi::c_int) != 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"read_vartx_tree\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            489 as core::ffi::c_int,
            b"!(tx_split[0] & ~0x33)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 = tx_split[0 as core::ffi::c_int
        as usize] as uint8_t;
    (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1 = tx_split[1 as core::ffi::c_int
        as usize];
}
#[inline]
unsafe extern "C" fn get_prev_frame_segid(
    f: *const Dav1dFrameContext,
    by: core::ffi::c_int,
    bx: core::ffi::c_int,
    w4: core::ffi::c_int,
    mut h4: core::ffi::c_int,
    mut ref_seg_map: *const uint8_t,
    stride: ptrdiff_t,
) -> core::ffi::c_uint {
    if !((*(*f).frame_hdr).primary_ref_frame as core::ffi::c_int
        != 7 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_prev_frame_segid\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            500 as core::ffi::c_int,
            b"f->frame_hdr->primary_ref_frame != DAV1D_PRIMARY_REF_NONE\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut seg_id: core::ffi::c_uint = 8 as core::ffi::c_uint;
    ref_seg_map = ref_seg_map
        .offset((by as ptrdiff_t * stride + bx as ptrdiff_t) as isize);
    loop {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w4 {
            seg_id = imin(
                seg_id as core::ffi::c_int,
                *ref_seg_map.offset(x as isize) as core::ffi::c_int,
            ) as core::ffi::c_uint;
            x += 1;
        }
        ref_seg_map = ref_seg_map.offset(stride as isize);
        h4 -= 1;
        if !(h4 > 0 as core::ffi::c_int && seg_id != 0) {
            break;
        }
    }
    if !(seg_id < 8 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 21],
                [core::ffi::c_char; 21],
            >(*b"get_prev_frame_segid\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            509 as core::ffi::c_int,
            b"seg_id < 8\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return seg_id;
}
#[inline]
unsafe extern "C" fn splat_oneref_mv(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
) {
    let mode: InterPredMode = (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
        as InterPredMode;
    let tmpl: refmvs_block = {
        let mut init = refmvs_block {
            mv: refmvs_mvpair {
                mv: [
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize],
                    mv {
                        c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                    },
                ],
            },
            ref_0: refmvs_refpair {
                ref_0: [
                    ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        + 1 as core::ffi::c_int) as int8_t,
                    (if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type
                        as core::ffi::c_int != 0
                    {
                        0 as core::ffi::c_int
                    } else {
                        -(1 as core::ffi::c_int)
                    }) as int8_t,
                ],
            },
            bs: bs as uint8_t,
            mf: ((mode as core::ffi::c_uint
                == GLOBALMV as core::ffi::c_int as core::ffi::c_uint
                && imin(bw4, bh4) >= 2 as core::ffi::c_int) as core::ffi::c_int
                | (mode as core::ffi::c_uint
                    == NEWMV as core::ffi::c_int as core::ffi::c_uint)
                    as core::ffi::c_int * 2 as core::ffi::c_int) as uint8_t,
        };
        init
    };
    ((*c).refmvs_dsp.splat_mv)
        .expect(
            "non-null function pointer",
        )(
        &raw mut *((*t).rt.r)
            .as_mut_ptr()
            .offset(
                (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize,
            ),
        &tmpl,
        (*t).bx,
        bw4,
        bh4,
    );
}
#[inline]
unsafe extern "C" fn splat_intrabc_mv(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
) {
    let tmpl: refmvs_block = {
        let mut init = refmvs_block {
            mv: refmvs_mvpair {
                mv: [
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize],
                    mv {
                        c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                    },
                ],
            },
            ref_0: refmvs_refpair {
                ref_0: [
                    0 as core::ffi::c_int as int8_t,
                    -(1 as core::ffi::c_int) as int8_t,
                ],
            },
            bs: bs as uint8_t,
            mf: 0 as uint8_t,
        };
        init
    };
    ((*c).refmvs_dsp.splat_mv)
        .expect(
            "non-null function pointer",
        )(
        &raw mut *((*t).rt.r)
            .as_mut_ptr()
            .offset(
                (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize,
            ),
        &tmpl,
        (*t).bx,
        bw4,
        bh4,
    );
}
#[inline]
unsafe extern "C" fn splat_tworef_mv(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
) {
    if !(bw4 >= 2 as core::ffi::c_int && bh4 >= 2 as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 16],
                [core::ffi::c_char; 16],
            >(*b"splat_tworef_mv\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            551 as core::ffi::c_int,
            b"bw4 >= 2 && bh4 >= 2\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mode: CompInterPredMode = (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
        as CompInterPredMode;
    let tmpl: refmvs_block = {
        let mut init = refmvs_block {
            mv: refmvs_mvpair {
                mv: [
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize],
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize],
                ],
            },
            ref_0: refmvs_refpair {
                ref_0: [
                    ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        + 1 as core::ffi::c_int) as int8_t,
                    ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                        + 1 as core::ffi::c_int) as int8_t,
                ],
            },
            bs: bs as uint8_t,
            mf: ((mode as core::ffi::c_uint
                == GLOBALMV_GLOBALMV as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int
                | ((1 as core::ffi::c_int) << mode as core::ffi::c_uint
                    & 0xbc as core::ffi::c_int != 0) as core::ffi::c_int
                    * 2 as core::ffi::c_int) as uint8_t,
        };
        init
    };
    ((*c).refmvs_dsp.splat_mv)
        .expect(
            "non-null function pointer",
        )(
        &raw mut *((*t).rt.r)
            .as_mut_ptr()
            .offset(
                (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize,
            ),
        &tmpl,
        (*t).bx,
        bw4,
        bh4,
    );
}
#[inline]
unsafe extern "C" fn splat_intraref(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    bw4: core::ffi::c_int,
    bh4: core::ffi::c_int,
) {
    let tmpl: refmvs_block = {
        let mut init = refmvs_block {
            mv: refmvs_mvpair {
                mv: [
                    mv { n: INVALID_MV },
                    mv {
                        c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                    },
                ],
            },
            ref_0: refmvs_refpair {
                ref_0: [
                    0 as core::ffi::c_int as int8_t,
                    -(1 as core::ffi::c_int) as int8_t,
                ],
            },
            bs: bs as uint8_t,
            mf: 0 as uint8_t,
        };
        init
    };
    ((*c).refmvs_dsp.splat_mv)
        .expect(
            "non-null function pointer",
        )(
        &raw mut *((*t).rt.r)
            .as_mut_ptr()
            .offset(
                (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize,
            ),
        &tmpl,
        (*t).bx,
        bw4,
        bh4,
    );
}
unsafe extern "C" fn mc_lowest_px(
    dst: *mut core::ffi::c_int,
    by4: core::ffi::c_int,
    bh4: core::ffi::c_int,
    mvy: core::ffi::c_int,
    ss_ver: core::ffi::c_int,
    smp: *const ScalableMotionParams,
) {
    let v_mul: core::ffi::c_int = 4 as core::ffi::c_int >> ss_ver;
    if (*smp).scale == 0 {
        let my: core::ffi::c_int = mvy >> 3 as core::ffi::c_int + ss_ver;
        let dy: core::ffi::c_int = mvy
            & 15 as core::ffi::c_int >> (ss_ver == 0) as core::ffi::c_int;
        *dst = imax(
            *dst,
            (by4 + bh4) * v_mul + my
                + 4 as core::ffi::c_int * (dy != 0) as core::ffi::c_int,
        );
    } else {
        let mut y: core::ffi::c_int = (by4 * v_mul << 4 as core::ffi::c_int)
            + mvy * ((1 as core::ffi::c_int) << (ss_ver == 0) as core::ffi::c_int);
        let tmp: int64_t = y as int64_t * (*smp).scale as int64_t
            + (((*smp).scale - 0x4000 as core::ffi::c_int) * 8 as core::ffi::c_int)
                as int64_t;
        y = apply_sign64(
            (llabs(tmp as core::ffi::c_longlong) + 128 as core::ffi::c_longlong
                >> 8 as core::ffi::c_int) as core::ffi::c_int,
            tmp,
        ) + 32 as core::ffi::c_int;
        let bottom: core::ffi::c_int = (y
            + (bh4 * v_mul - 1 as core::ffi::c_int) * (*smp).step
            >> 10 as core::ffi::c_int) + 1 as core::ffi::c_int + 4 as core::ffi::c_int;
        *dst = imax(*dst, bottom);
    };
}
#[inline(always)]
unsafe extern "C" fn affine_lowest_px(
    t: *mut Dav1dTaskContext,
    dst: *mut core::ffi::c_int,
    b_dim: *const uint8_t,
    wmp: *const Dav1dWarpedMotionParams,
    ss_ver: core::ffi::c_int,
    ss_hor: core::ffi::c_int,
) {
    let h_mul: core::ffi::c_int = 4 as core::ffi::c_int >> ss_hor;
    let v_mul: core::ffi::c_int = 4 as core::ffi::c_int >> ss_ver;
    if !(*b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * h_mul
        & 7 as core::ffi::c_int == 0
        && *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int * v_mul
            & 7 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"affine_lowest_px\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            599 as core::ffi::c_int,
            b"!((b_dim[0] * h_mul) & 7) && !((b_dim[1] * v_mul) & 7)\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mat: *const int32_t = ((*wmp).matrix).as_ptr();
    let y: core::ffi::c_int = *b_dim.offset(1 as core::ffi::c_int as isize)
        as core::ffi::c_int * v_mul - 8 as core::ffi::c_int;
    let src_y: core::ffi::c_int = (*t).by * 4 as core::ffi::c_int
        + ((y + 4 as core::ffi::c_int) << ss_ver);
    let mat5_y: int64_t = *mat.offset(5 as core::ffi::c_int as isize) as int64_t
        * src_y as int64_t + *mat.offset(1 as core::ffi::c_int as isize) as int64_t;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    while x < *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * h_mul {
        let src_x: core::ffi::c_int = (*t).bx * 4 as core::ffi::c_int
            + ((x + 4 as core::ffi::c_int) << ss_hor);
        let mvy: int64_t = *mat.offset(4 as core::ffi::c_int as isize) as int64_t
            * src_x as int64_t + mat5_y >> ss_ver;
        let dy: core::ffi::c_int = (mvy >> 16 as core::ffi::c_int) as core::ffi::c_int
            - 4 as core::ffi::c_int;
        *dst = imax(*dst, dy + 4 as core::ffi::c_int + 8 as core::ffi::c_int);
        x
            += imax(
                8 as core::ffi::c_int,
                *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * h_mul
                    - 8 as core::ffi::c_int,
            );
    }
}
#[inline(never)]
unsafe extern "C" fn affine_lowest_px_luma(
    t: *mut Dav1dTaskContext,
    dst: *mut core::ffi::c_int,
    b_dim: *const uint8_t,
    wmp: *const Dav1dWarpedMotionParams,
) {
    affine_lowest_px(t, dst, b_dim, wmp, 0 as core::ffi::c_int, 0 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn affine_lowest_px_chroma(
    t: *mut Dav1dTaskContext,
    dst: *mut core::ffi::c_int,
    b_dim: *const uint8_t,
    wmp: *const Dav1dWarpedMotionParams,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    if !((*f).cur.p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"affine_lowest_px_chroma\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            628 as core::ffi::c_int,
            b"f->cur.p.layout != DAV1D_PIXEL_LAYOUT_I400\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if (*f).cur.p.layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint
    {
        affine_lowest_px_luma(t, dst, b_dim, wmp);
    } else {
        affine_lowest_px(
            t,
            dst,
            b_dim,
            wmp,
            ((*f).cur.p.layout as core::ffi::c_uint
                & DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int,
            1 as core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn obmc_lowest_px(
    t: *mut Dav1dTaskContext,
    dst: *mut [core::ffi::c_int; 2],
    is_chroma: core::ffi::c_int,
    b_dim: *const uint8_t,
    bx4: core::ffi::c_int,
    by4: core::ffi::c_int,
    w4: core::ffi::c_int,
    h4: core::ffi::c_int,
) {
    if !((*t).bx & 1 as core::ffi::c_int == 0 && (*t).by & 1 as core::ffi::c_int == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 15],
                [core::ffi::c_char; 15],
            >(*b"obmc_lowest_px\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            640 as core::ffi::c_int,
            b"!(t->bx & 1) && !(t->by & 1)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let f: *const Dav1dFrameContext = (*t).f;
    let mut r: *mut *mut refmvs_block = &raw mut *((*t).rt.r)
        .as_mut_ptr()
        .offset((((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int) as isize)
        as *mut *mut refmvs_block;
    let ss_ver: core::ffi::c_int = (is_chroma != 0
        && (*f).cur.p.layout as core::ffi::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = (is_chroma != 0
        && (*f).cur.p.layout as core::ffi::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let h_mul: core::ffi::c_int = 4 as core::ffi::c_int >> ss_hor;
    let v_mul: core::ffi::c_int = 4 as core::ffi::c_int >> ss_ver;
    if (*t).by > (*(*t).ts).tiling.row_start
        && (is_chroma == 0
            || *b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int * h_mul
                + *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    * v_mul >= 16 as core::ffi::c_int)
    {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w4
            && i
                < imin(
                    *b_dim.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int,
                    4 as core::ffi::c_int,
                )
        {
            let a_r: *const refmvs_block = &raw mut *(*r
                .offset(-(1 as core::ffi::c_int) as isize))
                .offset(((*t).bx + x + 1 as core::ffi::c_int) as isize)
                as *mut refmvs_block;
            let a_b_dim: *const uint8_t = (dav1d_block_dimensions[(*a_r).bs as usize])
                .as_ptr();
            if (*a_r).ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            {
                let oh4: core::ffi::c_int = imin(
                    *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
                    16 as core::ffi::c_int,
                ) as core::ffi::c_int >> 1 as core::ffi::c_int;
                mc_lowest_px(
                    &raw mut *(*dst
                        .offset(
                            (*((*a_r).ref_0.ref_0)
                                .as_ptr()
                                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ))
                        .as_mut_ptr()
                        .offset(is_chroma as isize),
                    (*t).by,
                    oh4 * 3 as core::ffi::c_int + 3 as core::ffi::c_int
                        >> 2 as core::ffi::c_int,
                    (*a_r).mv.mv[0 as core::ffi::c_int as usize].c2rust_unnamed.y
                        as core::ffi::c_int,
                    ss_ver,
                    &*(*((*f).svc)
                        .as_ptr()
                        .offset(
                            (*((*a_r).ref_0.ref_0)
                                .as_ptr()
                                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ))
                        .as_ptr()
                        .offset(1 as core::ffi::c_int as isize),
                );
                i += 1;
            }
            x
                += imax(
                    *a_b_dim.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                    2 as core::ffi::c_int,
                );
        }
    }
    if (*t).bx > (*(*t).ts).tiling.col_start {
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < h4
            && i_0
                < imin(
                    *b_dim.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int,
                    4 as core::ffi::c_int,
                )
        {
            let l_r: *const refmvs_block = &raw mut *(*r
                .offset((y + 1 as core::ffi::c_int) as isize))
                .offset(((*t).bx - 1 as core::ffi::c_int) as isize) as *mut refmvs_block;
            let l_b_dim: *const uint8_t = (dav1d_block_dimensions[(*l_r).bs as usize])
                .as_ptr();
            if (*l_r).ref_0.ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                > 0 as core::ffi::c_int
            {
                let oh4_0: core::ffi::c_int = iclip(
                    *l_b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
                    2 as core::ffi::c_int,
                    *b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
                ) as core::ffi::c_int;
                mc_lowest_px(
                    &raw mut *(*dst
                        .offset(
                            (*((*l_r).ref_0.ref_0)
                                .as_ptr()
                                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ))
                        .as_mut_ptr()
                        .offset(is_chroma as isize),
                    (*t).by + y,
                    oh4_0,
                    (*l_r).mv.mv[0 as core::ffi::c_int as usize].c2rust_unnamed.y
                        as core::ffi::c_int,
                    ss_ver,
                    &*(*((*f).svc)
                        .as_ptr()
                        .offset(
                            (*((*l_r).ref_0.ref_0)
                                .as_ptr()
                                .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                - 1 as core::ffi::c_int) as isize,
                        ))
                        .as_ptr()
                        .offset(1 as core::ffi::c_int as isize),
                );
                i_0 += 1;
            }
            y
                += imax(
                    *l_b_dim.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int,
                    2 as core::ffi::c_int,
                );
        }
    }
}
unsafe extern "C" fn decode_b(
    t: *mut Dav1dTaskContext,
    bl: BlockLevel,
    bs: BlockSize,
    bp: BlockPartition,
    intra_edge_flags: EdgeFlags,
) -> core::ffi::c_int {
    let ts: *mut Dav1dTileState = (*t).ts;
    let f: *const Dav1dFrameContext = (*t).f;
    let mut b_mem: Av1Block = Av1Block {
        bl: 0,
        bs: 0,
        bp: 0,
        intra: 0,
        seg_id: 0,
        skip_mode: 0,
        skip: 0,
        uvtx: 0,
        c2rust_unnamed: C2RustUnnamed_20 {
            c2rust_unnamed: C2RustUnnamed_26 {
                y_mode: 0,
                uv_mode: 0,
                tx: 0,
                pal_sz: [0; 2],
                y_angle: 0,
                uv_angle: 0,
                cfl_alpha: [0; 2],
            },
        },
    };
    let b: *mut Av1Block = if (*t).frame_thread.pass != 0 {
        &raw mut *((*f).frame_thread.b)
            .offset(
                ((*t).by as ptrdiff_t * (*f).b4_stride + (*t).bx as ptrdiff_t) as isize,
            ) as *mut Av1Block
    } else {
        &raw mut b_mem
    };
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bx4: core::ffi::c_int = (*t).bx & 31 as core::ffi::c_int;
    let by4: core::ffi::c_int = (*t).by & 31 as core::ffi::c_int;
    let ss_ver: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let ss_hor: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    let cbx4: core::ffi::c_int = bx4 >> ss_hor;
    let cby4: core::ffi::c_int = by4 >> ss_ver;
    let bw4: core::ffi::c_int = *b_dim.offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let bh4: core::ffi::c_int = *b_dim.offset(1 as core::ffi::c_int as isize)
        as core::ffi::c_int;
    let w4: core::ffi::c_int = imin(bw4, (*f).bw - (*t).bx) as core::ffi::c_int;
    let h4: core::ffi::c_int = imin(bh4, (*f).bh - (*t).by) as core::ffi::c_int;
    let cbw4: core::ffi::c_int = bw4 + ss_hor >> ss_hor;
    let cbh4: core::ffi::c_int = bh4 + ss_ver >> ss_ver;
    let have_left: core::ffi::c_int = ((*t).bx > (*ts).tiling.col_start)
        as core::ffi::c_int;
    let have_top: core::ffi::c_int = ((*t).by > (*ts).tiling.row_start)
        as core::ffi::c_int;
    let has_chroma: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int as core::ffi::c_uint
        && (bw4 > ss_hor || (*t).bx & 1 as core::ffi::c_int != 0)
        && (bh4 > ss_ver || (*t).by & 1 as core::ffi::c_int != 0)) as core::ffi::c_int;
    if (*t).frame_thread.pass == 2 as core::ffi::c_int {
        if (*b).intra != 0 {
            ((*f).bd_fn.recon_b_intra)
                .expect("non-null function pointer")(t, bs, intra_edge_flags, b);
            let y_mode_nofilt: IntraPredMode = (if (*b)
                .c2rust_unnamed
                .c2rust_unnamed
                .y_mode as core::ffi::c_int == FILTER_PRED as core::ffi::c_int
            {
                DC_PRED as core::ffi::c_int
            } else {
                (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int
            }) as IntraPredMode;
            let mut edge: *mut BlockContext = (*t).a;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut off: core::ffi::c_int = bx4;
            while i < 2 as core::ffi::c_int {
                match *b_dim.offset((2 as core::ffi::c_int + i) as isize)
                    as core::ffi::c_int
                {
                    0 => {
                        (*(&raw mut *((*edge).mode).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias8))
                            .u8_0 = (y_mode_nofilt as core::ffi::c_uint)
                            .wrapping_mul(0x1 as core::ffi::c_uint) as uint8_t;
                        (*(&raw mut *((*edge).intra).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias8))
                            .u8_0 = (1 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                            as uint8_t;
                    }
                    1 => {
                        (*(&raw mut *((*edge).mode).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias16))
                            .u16_0 = (y_mode_nofilt as core::ffi::c_uint)
                            .wrapping_mul(0x101 as core::ffi::c_uint) as uint16_t;
                        (*(&raw mut *((*edge).intra).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias16))
                            .u16_0 = (1 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                            as uint16_t;
                    }
                    2 => {
                        (*(&raw mut *((*edge).mode).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias32))
                            .u32_0 = (y_mode_nofilt as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *((*edge).intra).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias32))
                            .u32_0 = (1 as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    }
                    3 => {
                        (*(&raw mut *((*edge).mode).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias64))
                            .u64_0 = (y_mode_nofilt as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *((*edge).intra).as_mut_ptr().offset(off as isize)
                            as *mut uint8_t as *mut alias64))
                            .u64_0 = (1 as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                    }
                    4 => {
                        memset(
                            &raw mut *((*edge).mode).as_mut_ptr().offset(off as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            y_mode_nofilt as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *((*edge).intra).as_mut_ptr().offset(off as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            1 as core::ffi::c_int,
                            16 as size_t,
                        );
                    }
                    5 => {
                        memset(
                            &raw mut *((*edge).mode).as_mut_ptr().offset(off as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            y_mode_nofilt as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *((*edge).intra).as_mut_ptr().offset(off as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            1 as core::ffi::c_int,
                            32 as size_t,
                        );
                    }
                    _ => {
                        if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 9],
                                    [core::ffi::c_char; 9],
                                >(*b"decode_b\0"))
                                    .as_ptr(),
                                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                                717 as core::ffi::c_int,
                                b"0\0" as *const u8 as *const core::ffi::c_char,
                            );
                        } else {};
                    }
                }
                i += 1;
                off = by4;
                edge = &raw mut (*t).l;
            }
            if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
                != 0
            {
                let r: *mut refmvs_block = &raw mut *(*((*t).rt.r)
                    .as_mut_ptr()
                    .offset(
                        (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int + bh4
                            - 1 as core::ffi::c_int) as isize,
                    ))
                    .offset((*t).bx as isize) as *mut refmvs_block;
                let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
                while x < bw4 {
                    (*r.offset(x as isize))
                        .ref_0
                        .ref_0[0 as core::ffi::c_int as usize] = 0 as int8_t;
                    (*r.offset(x as isize)).bs = bs as uint8_t;
                    x += 1;
                }
                let mut rr: *const *mut refmvs_block = &raw mut *((*t).rt.r)
                    .as_mut_ptr()
                    .offset(
                        (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int)
                            as isize,
                    ) as *mut *mut refmvs_block;
                let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
                while y < bh4 - 1 as core::ffi::c_int {
                    (*(*rr.offset(y as isize))
                        .offset(((*t).bx + bw4 - 1 as core::ffi::c_int) as isize))
                        .ref_0
                        .ref_0[0 as core::ffi::c_int as usize] = 0 as int8_t;
                    (*(*rr.offset(y as isize))
                        .offset(((*t).bx + bw4 - 1 as core::ffi::c_int) as isize))
                        .bs = bs as uint8_t;
                    y += 1;
                }
            }
            if has_chroma != 0 {
                let mut uv_mode: uint8_t = (*b).c2rust_unnamed.c2rust_unnamed.uv_mode;
                (dav1d_memset_pow2[ulog2(cbw4 as core::ffi::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut *((*(*t).a).uvmode).as_mut_ptr().offset(cbx4 as isize)
                        as *mut uint8_t as *mut core::ffi::c_void,
                    uv_mode as core::ffi::c_int,
                );
                (dav1d_memset_pow2[ulog2(cbh4 as core::ffi::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut *((*t).l.uvmode).as_mut_ptr().offset(cby4 as isize)
                        as *mut uint8_t as *mut core::ffi::c_void,
                    uv_mode as core::ffi::c_int,
                );
            }
        } else {
            if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
                != 0
                && (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as core::ffi::c_int
                    == COMP_INTER_NONE as core::ffi::c_int
                && (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as core::ffi::c_int
                    == MM_WARP as core::ffi::c_int
            {
                if (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .matrix[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    == INT16_MIN
                {
                    (*t).warpmv.type_0 = DAV1D_WM_TYPE_IDENTITY;
                } else {
                    (*t).warpmv.type_0 = DAV1D_WM_TYPE_AFFINE;
                    (*t).warpmv.matrix[2 as core::ffi::c_int as usize] = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .matrix[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        + 0x10000 as core::ffi::c_int) as int32_t;
                    (*t).warpmv.matrix[3 as core::ffi::c_int as usize] = (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .matrix[1 as core::ffi::c_int as usize] as int32_t;
                    (*t).warpmv.matrix[4 as core::ffi::c_int as usize] = (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .matrix[2 as core::ffi::c_int as usize] as int32_t;
                    (*t).warpmv.matrix[5 as core::ffi::c_int as usize] = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .matrix[3 as core::ffi::c_int as usize] as core::ffi::c_int
                        + 0x10000 as core::ffi::c_int) as int32_t;
                    dav1d_set_affine_mv2d(
                        bw4,
                        bh4,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .mv2d as mv,
                        &raw mut (*t).warpmv,
                        (*t).bx,
                        (*t).by,
                    );
                    dav1d_get_shear_params(&raw mut (*t).warpmv);
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"[ %c%x %c%x %c%x\n  %c%x %c%x %c%x ]\nalpha=%c%x, beta=%c%x, gamma=%c%x, delta=%c%x, mv=y:%d,x:%d\n\0"
                                as *const u8 as *const core::ffi::c_char,
                            if (*t).warpmv.matrix[0 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[0 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[1 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[1 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[2 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[2 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[3 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[3 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[4 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[4 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[5 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[5 as core::ffi::c_int as usize]),
                            if ((*t).warpmv.u.p.alpha as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.alpha as core::ffi::c_int),
                            if ((*t).warpmv.u.p.beta as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.beta as core::ffi::c_int),
                            if ((*t).warpmv.u.p.gamma as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.gamma as core::ffi::c_int),
                            if ((*t).warpmv.u.p.delta as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.delta as core::ffi::c_int),
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .mv2d
                                .c2rust_unnamed
                                .y as core::ffi::c_int,
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .mv2d
                                .c2rust_unnamed
                                .x as core::ffi::c_int,
                        );
                    }
                }
            }
            if ((*f).bd_fn.recon_b_inter).expect("non-null function pointer")(t, bs, b)
                != 0
            {
                return -(1 as core::ffi::c_int);
            }
            let filter: *const uint8_t = (dav1d_filter_dir[(*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .filter2d as usize])
                .as_ptr();
            let mut edge_0: *mut BlockContext = (*t).a;
            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
            let mut off_0: core::ffi::c_int = bx4;
            while i_0 < 2 as core::ffi::c_int {
                match *b_dim.offset((2 as core::ffi::c_int + i_0) as isize)
                    as core::ffi::c_int
                {
                    0 => {
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias8))
                            .u8_0 = (*filter.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias8))
                            .u8_0 = (*filter.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                        (*(&raw mut *((*edge_0).intra).as_mut_ptr().offset(off_0 as isize)
                            as *mut uint8_t as *mut alias8))
                            .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                            as uint8_t;
                    }
                    1 => {
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias16))
                            .u16_0 = (*filter.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias16))
                            .u16_0 = (*filter.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                        (*(&raw mut *((*edge_0).intra).as_mut_ptr().offset(off_0 as isize)
                            as *mut uint8_t as *mut alias16))
                            .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                            as uint16_t;
                    }
                    2 => {
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias32))
                            .u32_0 = (*filter.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias32))
                            .u32_0 = (*filter.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *((*edge_0).intra).as_mut_ptr().offset(off_0 as isize)
                            as *mut uint8_t as *mut alias32))
                            .u32_0 = (0 as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    }
                    3 => {
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias64))
                            .u64_0 = (*filter.offset(0 as core::ffi::c_int as isize)
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *(*((*edge_0).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_0 as isize) as *mut uint8_t as *mut alias64))
                            .u64_0 = (*filter.offset(1 as core::ffi::c_int as isize)
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *((*edge_0).intra).as_mut_ptr().offset(off_0 as isize)
                            as *mut uint8_t as *mut alias64))
                            .u64_0 = (0 as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                    }
                    4 => {
                        memset(
                            &raw mut *(*((*edge_0).filter)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_0 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            *filter.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_0).filter)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_0 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            *filter.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *((*edge_0).intra).as_mut_ptr().offset(off_0 as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            16 as size_t,
                        );
                    }
                    5 => {
                        memset(
                            &raw mut *(*((*edge_0).filter)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_0 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            *filter.offset(0 as core::ffi::c_int as isize)
                                as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_0).filter)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_0 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            *filter.offset(1 as core::ffi::c_int as isize)
                                as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *((*edge_0).intra).as_mut_ptr().offset(off_0 as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            32 as size_t,
                        );
                    }
                    _ => {
                        if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                            as core::ffi::c_long != 0
                        {
                            crate::platform::assertion(
                                (::core::mem::transmute::<
                                    [u8; 9],
                                    [core::ffi::c_char; 9],
                                >(*b"decode_b\0"))
                                    .as_ptr(),
                                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                                780 as core::ffi::c_int,
                                b"0\0" as *const u8 as *const core::ffi::c_char,
                            );
                        } else {};
                    }
                }
                i_0 += 1;
                off_0 = by4;
                edge_0 = &raw mut (*t).l;
            }
            if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
                != 0
            {
                let r_0: *mut refmvs_block = &raw mut *(*((*t).rt.r)
                    .as_mut_ptr()
                    .offset(
                        (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int + bh4
                            - 1 as core::ffi::c_int) as isize,
                    ))
                    .offset((*t).bx as isize) as *mut refmvs_block;
                let ref1: core::ffi::c_int = (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    + 1 as core::ffi::c_int;
                let mv1: mv = (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0 as core::ffi::c_int as usize];
                let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while x_0 < bw4 {
                    (*r_0.offset(x_0 as isize))
                        .ref_0
                        .ref_0[0 as core::ffi::c_int as usize] = ref1 as int8_t;
                    (*r_0.offset(x_0 as isize)).mv.mv[0 as core::ffi::c_int as usize] = mv1
                        as mv;
                    (*r_0.offset(x_0 as isize)).bs = bs as uint8_t;
                    x_0 += 1;
                }
                let mut rr_0: *const *mut refmvs_block = &raw mut *((*t).rt.r)
                    .as_mut_ptr()
                    .offset(
                        (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int)
                            as isize,
                    ) as *mut *mut refmvs_block;
                let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_0 < bh4 - 1 as core::ffi::c_int {
                    (*(*rr_0.offset(y_0 as isize))
                        .offset(((*t).bx + bw4 - 1 as core::ffi::c_int) as isize))
                        .ref_0
                        .ref_0[0 as core::ffi::c_int as usize] = ref1 as int8_t;
                    (*(*rr_0.offset(y_0 as isize))
                        .offset(((*t).bx + bw4 - 1 as core::ffi::c_int) as isize))
                        .mv
                        .mv[0 as core::ffi::c_int as usize] = mv1 as mv;
                    (*(*rr_0.offset(y_0 as isize))
                        .offset(((*t).bx + bw4 - 1 as core::ffi::c_int) as isize))
                        .bs = bs as uint8_t;
                    y_0 += 1;
                }
            }
            if has_chroma != 0 {
                (dav1d_memset_pow2[ulog2(cbw4 as core::ffi::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut *((*(*t).a).uvmode).as_mut_ptr().offset(cbx4 as isize)
                        as *mut uint8_t as *mut core::ffi::c_void,
                    DC_PRED as core::ffi::c_int,
                );
                (dav1d_memset_pow2[ulog2(cbh4 as core::ffi::c_uint) as usize])
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut *((*t).l.uvmode).as_mut_ptr().offset(cby4 as isize)
                        as *mut uint8_t as *mut core::ffi::c_void,
                    DC_PRED as core::ffi::c_int,
                );
            }
        }
        return 0 as core::ffi::c_int;
    }
    let cw4: core::ffi::c_int = w4 + ss_hor >> ss_hor;
    let ch4: core::ffi::c_int = h4 + ss_ver >> ss_ver;
    (*b).bl = bl as uint8_t;
    (*b).bp = bp as uint8_t;
    (*b).bs = bs as uint8_t;
    let mut seg: *const Dav1dSegmentationData = 0 as *const Dav1dSegmentationData;
    let mut seg_pred: core::ffi::c_int = 0 as core::ffi::c_int;
    if (*(*f).frame_hdr).segmentation.enabled != 0 {
        if (*(*f).frame_hdr).segmentation.update_map == 0 {
            if !((*f).prev_segmap).is_null() {
                let mut seg_id: core::ffi::c_uint = get_prev_frame_segid(
                    f,
                    (*t).by,
                    (*t).bx,
                    w4,
                    h4,
                    (*f).prev_segmap,
                    (*f).b4_stride,
                );
                if seg_id >= 8 as core::ffi::c_uint {
                    return -(1 as core::ffi::c_int);
                }
                (*b).seg_id = seg_id as uint8_t;
            } else {
                (*b).seg_id = 0 as uint8_t;
            }
            seg = &raw mut *((*(*f).frame_hdr).segmentation.seg_data.d)
                .as_mut_ptr()
                .offset((*b).seg_id as isize) as *mut Dav1dSegmentationData;
        } else if (*(*f).frame_hdr).segmentation.seg_data.preskip != 0 {
            if (*(*f).frame_hdr).segmentation.temporal as core::ffi::c_int != 0
                && {
                    seg_pred = dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .seg_pred[((*(*t).a).seg_pred[bx4 as usize]
                            as core::ffi::c_int
                            + (*t).l.seg_pred[by4 as usize] as core::ffi::c_int)
                            as usize])
                            .as_mut_ptr(),
                    ) as core::ffi::c_int;
                    seg_pred != 0
                }
            {
                if !((*f).prev_segmap).is_null() {
                    let mut seg_id_0: core::ffi::c_uint = get_prev_frame_segid(
                        f,
                        (*t).by,
                        (*t).bx,
                        w4,
                        h4,
                        (*f).prev_segmap,
                        (*f).b4_stride,
                    );
                    if seg_id_0 >= 8 as core::ffi::c_uint {
                        return -(1 as core::ffi::c_int);
                    }
                    (*b).seg_id = seg_id_0 as uint8_t;
                } else {
                    (*b).seg_id = 0 as uint8_t;
                }
            } else {
                let mut seg_ctx: core::ffi::c_int = 0;
                let pred_seg_id: core::ffi::c_uint = get_cur_frame_segid(
                    (*t).by,
                    (*t).bx,
                    have_top,
                    have_left,
                    &raw mut seg_ctx,
                    (*f).cur_segmap,
                    (*f).b4_stride,
                ) as core::ffi::c_uint;
                let diff: core::ffi::c_uint = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.seg_id[seg_ctx as usize]).as_mut_ptr(),
                    (DAV1D_MAX_SEGMENTS - 1 as core::ffi::c_int) as size_t,
                ) as core::ffi::c_uint;
                let last_active_seg_id: core::ffi::c_uint = (*(*f).frame_hdr)
                    .segmentation
                    .seg_data
                    .last_active_segid as core::ffi::c_uint;
                (*b).seg_id = neg_deinterleave(
                    diff as core::ffi::c_int,
                    pred_seg_id as core::ffi::c_int,
                    last_active_seg_id.wrapping_add(1 as core::ffi::c_uint)
                        as core::ffi::c_int,
                ) as uint8_t;
                if (*b).seg_id as core::ffi::c_uint > last_active_seg_id {
                    (*b).seg_id = 0 as uint8_t;
                }
                if (*b).seg_id as core::ffi::c_int >= DAV1D_MAX_SEGMENTS {
                    (*b).seg_id = 0 as uint8_t;
                }
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-segid[preskip;%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b).seg_id as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
            seg = &raw mut *((*(*f).frame_hdr).segmentation.seg_data.d)
                .as_mut_ptr()
                .offset((*b).seg_id as isize) as *mut Dav1dSegmentationData;
        }
    } else {
        (*b).seg_id = 0 as uint8_t;
    }
    if (seg.is_null()
        || (*seg).globalmv == 0
            && (*seg).ref_0 as core::ffi::c_int == -(1 as core::ffi::c_int)
            && (*seg).skip == 0)
        && (*(*f).frame_hdr).skip_mode_enabled as core::ffi::c_int != 0
        && imin(bw4, bh4) > 1 as core::ffi::c_int
    {
        let smctx: core::ffi::c_int = (*(*t).a).skip_mode[bx4 as usize]
            as core::ffi::c_int + (*t).l.skip_mode[by4 as usize] as core::ffi::c_int;
        (*b).skip_mode = dav1d_msac_decode_bool_adapt_c(
            &raw mut (*ts).msac,
            ((*ts).cdf.m.skip_mode[smctx as usize]).as_mut_ptr(),
        ) as uint8_t;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-skipmode[%d]: r=%d\n\0" as *const u8 as *const core::ffi::c_char,
                (*b).skip_mode as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
    } else {
        (*b).skip_mode = 0 as uint8_t;
    }
    if (*b).skip_mode as core::ffi::c_int != 0
        || !seg.is_null() && (*seg).skip as core::ffi::c_int != 0
    {
        (*b).skip = 1 as uint8_t;
    } else {
        let sctx: core::ffi::c_int = (*(*t).a).skip[bx4 as usize] as core::ffi::c_int
            + (*t).l.skip[by4 as usize] as core::ffi::c_int;
        (*b).skip = dav1d_msac_decode_bool_adapt_c(
            &raw mut (*ts).msac,
            ((*ts).cdf.m.skip[sctx as usize]).as_mut_ptr(),
        ) as uint8_t;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-skip[%d]: r=%d\n\0" as *const u8 as *const core::ffi::c_char,
                (*b).skip as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
    }
    if (*(*f).frame_hdr).segmentation.enabled as core::ffi::c_int != 0
        && (*(*f).frame_hdr).segmentation.update_map as core::ffi::c_int != 0
        && (*(*f).frame_hdr).segmentation.seg_data.preskip == 0
    {
        if (*b).skip == 0
            && (*(*f).frame_hdr).segmentation.temporal as core::ffi::c_int != 0
            && {
                seg_pred = dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts)
                        .cdf
                        .m
                        .seg_pred[((*(*t).a).seg_pred[bx4 as usize] as core::ffi::c_int
                        + (*t).l.seg_pred[by4 as usize] as core::ffi::c_int) as usize])
                        .as_mut_ptr(),
                ) as core::ffi::c_int;
                seg_pred != 0
            }
        {
            if !((*f).prev_segmap).is_null() {
                let mut seg_id_1: core::ffi::c_uint = get_prev_frame_segid(
                    f,
                    (*t).by,
                    (*t).bx,
                    w4,
                    h4,
                    (*f).prev_segmap,
                    (*f).b4_stride,
                );
                if seg_id_1 >= 8 as core::ffi::c_uint {
                    return -(1 as core::ffi::c_int);
                }
                (*b).seg_id = seg_id_1 as uint8_t;
            } else {
                (*b).seg_id = 0 as uint8_t;
            }
        } else {
            let mut seg_ctx_0: core::ffi::c_int = 0;
            let pred_seg_id_0: core::ffi::c_uint = get_cur_frame_segid(
                (*t).by,
                (*t).bx,
                have_top,
                have_left,
                &raw mut seg_ctx_0,
                (*f).cur_segmap,
                (*f).b4_stride,
            ) as core::ffi::c_uint;
            if (*b).skip != 0 {
                (*b).seg_id = pred_seg_id_0 as uint8_t;
            } else {
                let diff_0: core::ffi::c_uint = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.seg_id[seg_ctx_0 as usize]).as_mut_ptr(),
                    (DAV1D_MAX_SEGMENTS - 1 as core::ffi::c_int) as size_t,
                ) as core::ffi::c_uint;
                let last_active_seg_id_0: core::ffi::c_uint = (*(*f).frame_hdr)
                    .segmentation
                    .seg_data
                    .last_active_segid as core::ffi::c_uint;
                (*b).seg_id = neg_deinterleave(
                    diff_0 as core::ffi::c_int,
                    pred_seg_id_0 as core::ffi::c_int,
                    last_active_seg_id_0.wrapping_add(1 as core::ffi::c_uint)
                        as core::ffi::c_int,
                ) as uint8_t;
                if (*b).seg_id as core::ffi::c_uint > last_active_seg_id_0 {
                    (*b).seg_id = 0 as uint8_t;
                }
            }
            if (*b).seg_id as core::ffi::c_int >= DAV1D_MAX_SEGMENTS {
                (*b).seg_id = 0 as uint8_t;
            }
        }
        seg = &raw mut *((*(*f).frame_hdr).segmentation.seg_data.d)
            .as_mut_ptr()
            .offset((*b).seg_id as isize) as *mut Dav1dSegmentationData;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-segid[postskip;%d]: r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*b).seg_id as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
    }
    if (*b).skip == 0 {
        let idx: core::ffi::c_int = if (*(*f).seq_hdr).sb128 as core::ffi::c_int != 0 {
            (((*t).bx & 16 as core::ffi::c_int) >> 4 as core::ffi::c_int)
                + (((*t).by & 16 as core::ffi::c_int) >> 3 as core::ffi::c_int)
        } else {
            0 as core::ffi::c_int
        };
        if *((*t).cur_sb_cdef_idx_ptr).offset(idx as isize) as core::ffi::c_int
            == -(1 as core::ffi::c_int)
        {
            let v: core::ffi::c_int = dav1d_msac_decode_bools(
                &raw mut (*ts).msac,
                (*(*f).frame_hdr).cdef.n_bits as core::ffi::c_uint,
            ) as core::ffi::c_int;
            *((*t).cur_sb_cdef_idx_ptr).offset(idx as isize) = v as int8_t;
            if bw4 > 16 as core::ffi::c_int {
                *((*t).cur_sb_cdef_idx_ptr)
                    .offset((idx + 1 as core::ffi::c_int) as isize) = v as int8_t;
            }
            if bh4 > 16 as core::ffi::c_int {
                *((*t).cur_sb_cdef_idx_ptr)
                    .offset((idx + 2 as core::ffi::c_int) as isize) = v as int8_t;
            }
            if bw4 == 32 as core::ffi::c_int && bh4 == 32 as core::ffi::c_int {
                *((*t).cur_sb_cdef_idx_ptr)
                    .offset((idx + 3 as core::ffi::c_int) as isize) = v as int8_t;
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-cdef_idx[%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    *(*t).cur_sb_cdef_idx_ptr as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
        }
    }
    if ((*t).bx | (*t).by)
        & 31 as core::ffi::c_int >> ((*(*f).seq_hdr).sb128 == 0) as core::ffi::c_int == 0
    {
        let prev_qidx: core::ffi::c_int = (*ts).last_qidx;
        let have_delta_q: core::ffi::c_int = ((*(*f).frame_hdr).delta.q.present
            as core::ffi::c_int != 0
            && (bs as core::ffi::c_uint
                != (if (*(*f).seq_hdr).sb128 as core::ffi::c_int != 0 {
                    BS_128x128 as core::ffi::c_int
                } else {
                    BS_64x64 as core::ffi::c_int
                }) as core::ffi::c_uint || (*b).skip == 0)) as core::ffi::c_int;
        let mut prev_delta_lf: uint32_t = (*ts).last_delta_lf.u32_0;
        if have_delta_q != 0 {
            let mut delta_q: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
                &raw mut (*ts).msac,
                ((*ts).cdf.m.delta_q).as_mut_ptr(),
                3 as size_t,
            ) as core::ffi::c_int;
            if delta_q == 3 as core::ffi::c_int {
                let n_bits: core::ffi::c_int = (1 as core::ffi::c_uint)
                    .wrapping_add(
                        dav1d_msac_decode_bools(&raw mut (*ts).msac, 3 as core::ffi::c_uint),
                    ) as core::ffi::c_int;
                delta_q = (dav1d_msac_decode_bools(
                    &raw mut (*ts).msac,
                    n_bits as core::ffi::c_uint,
                ))
                    .wrapping_add(1 as core::ffi::c_uint)
                    .wrapping_add(
                        ((1 as core::ffi::c_int) << n_bits) as core::ffi::c_uint,
                    ) as core::ffi::c_int;
            }
            if delta_q != 0 {
                if dav1d_msac_decode_bool_equi_c(&raw mut (*ts).msac) != 0 {
                    delta_q = -delta_q;
                }
                delta_q
                    *= (1 as core::ffi::c_int)
                        << (*(*f).frame_hdr).delta.q.res_log2 as core::ffi::c_int;
            }
            (*ts).last_qidx = iclip(
                (*ts).last_qidx + delta_q,
                1 as core::ffi::c_int,
                255 as core::ffi::c_int,
            );
            if have_delta_q != 0 && 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-delta_q[%d->%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    delta_q,
                    (*ts).last_qidx,
                    (*ts).msac.rng,
                );
            }
            if (*(*f).frame_hdr).delta.lf.present != 0 {
                let n_lfs: core::ffi::c_int = if (*(*f).frame_hdr).delta.lf.multi
                    as core::ffi::c_int != 0
                {
                    if (*f).cur.p.layout as core::ffi::c_uint
                        != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int
                            as core::ffi::c_uint
                    {
                        4 as core::ffi::c_int
                    } else {
                        2 as core::ffi::c_int
                    }
                } else {
                    1 as core::ffi::c_int
                };
                let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_1 < n_lfs {
                    let mut delta_lf: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .delta_lf[(i_1
                            + (*(*f).frame_hdr).delta.lf.multi as core::ffi::c_int)
                            as usize])
                            .as_mut_ptr(),
                        3 as size_t,
                    ) as core::ffi::c_int;
                    if delta_lf == 3 as core::ffi::c_int {
                        let n_bits_0: core::ffi::c_int = (1 as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bools(
                                    &raw mut (*ts).msac,
                                    3 as core::ffi::c_uint,
                                ),
                            ) as core::ffi::c_int;
                        delta_lf = (dav1d_msac_decode_bools(
                            &raw mut (*ts).msac,
                            n_bits_0 as core::ffi::c_uint,
                        ))
                            .wrapping_add(1 as core::ffi::c_uint)
                            .wrapping_add(
                                ((1 as core::ffi::c_int) << n_bits_0) as core::ffi::c_uint,
                            ) as core::ffi::c_int;
                    }
                    if delta_lf != 0 {
                        if dav1d_msac_decode_bool_equi_c(&raw mut (*ts).msac) != 0 {
                            delta_lf = -delta_lf;
                        }
                        delta_lf
                            *= (1 as core::ffi::c_int)
                                << (*(*f).frame_hdr).delta.lf.res_log2 as core::ffi::c_int;
                    }
                    (*ts).last_delta_lf.i8_0[i_1 as usize] = iclip(
                        (*ts).last_delta_lf.i8_0[i_1 as usize] as core::ffi::c_int
                            + delta_lf,
                        -(63 as core::ffi::c_int),
                        63 as core::ffi::c_int,
                    ) as int8_t;
                    if have_delta_q != 0 && 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-delta_lf[%d:%d]: r=%d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            i_1,
                            delta_lf,
                            (*ts).msac.rng,
                        );
                    }
                    i_1 += 1;
                }
            }
        }
        if (*ts).last_qidx == (*(*f).frame_hdr).quant.yac as core::ffi::c_int {
            (*ts).dq = ((*f).dq).as_ptr() as *const [[uint16_t; 2]; 3];
        } else if (*ts).last_qidx != prev_qidx {
            init_quant_tables(
                (*f).seq_hdr,
                (*f).frame_hdr,
                (*ts).last_qidx,
                ((*ts).dqmem).as_mut_ptr() as *mut [[uint16_t; 2]; 3],
            );
            (*ts).dq = ((*ts).dqmem).as_mut_ptr() as *const [[uint16_t; 2]; 3];
        }
        if (*ts).last_delta_lf.u32_0 == 0 {
            (*ts).lflvl = ((*f).lf.lvl).as_ptr() as *const [[[uint8_t; 2]; 8]; 4];
        } else if (*ts).last_delta_lf.u32_0 != prev_delta_lf {
            (*ts).lflvl = ((*ts).lflvlmem).as_mut_ptr() as *const [[[uint8_t; 2]; 8]; 4];
            dav1d_calc_lf_values(
                ((*ts).lflvlmem).as_mut_ptr() as *mut [[[uint8_t; 2]; 8]; 4],
                (*f).frame_hdr,
                ((*ts).last_delta_lf.i8_0).as_mut_ptr() as *const int8_t,
            );
        }
    }
    if (*b).skip_mode != 0 {
        (*b).intra = 0 as uint8_t;
    } else if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
        != 0
    {
        if !seg.is_null()
            && ((*seg).ref_0 as core::ffi::c_int >= 0 as core::ffi::c_int
                || (*seg).globalmv as core::ffi::c_int != 0)
        {
            (*b).intra = ((*seg).ref_0 == 0) as core::ffi::c_int as uint8_t;
        } else {
            let ictx: core::ffi::c_int = get_intra_ctx(
                (*t).a,
                &raw mut (*t).l,
                by4,
                bx4,
                have_top,
                have_left,
            ) as core::ffi::c_int;
            (*b).intra = (dav1d_msac_decode_bool_adapt_c(
                &raw mut (*ts).msac,
                ((*ts).cdf.m.intra[ictx as usize]).as_mut_ptr(),
            ) == 0) as core::ffi::c_int as uint8_t;
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-intra[%d]: r=%d\n\0" as *const u8 as *const core::ffi::c_char,
                    (*b).intra as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
        }
    } else if (*(*f).frame_hdr).allow_intrabc != 0 {
        (*b).intra = (dav1d_msac_decode_bool_adapt_c(
            &raw mut (*ts).msac,
            ((*ts).cdf.m.intrabc).as_mut_ptr(),
        ) == 0) as core::ffi::c_int as uint8_t;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-intrabcflag[%d]: r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*b).intra as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
    } else {
        (*b).intra = 1 as uint8_t;
    }
    if (*b).intra != 0 {
        let ymode_cdf: *mut uint16_t = if (*(*f).frame_hdr).frame_type
            as core::ffi::c_uint & 1 as core::ffi::c_uint != 0
        {
            ((*ts).cdf.m.y_mode[dav1d_ymode_size_context[bs as usize] as usize])
                .as_mut_ptr()
        } else {
            ((*ts)
                .cdf
                .kfym[dav1d_intra_mode_context[(*(*t).a).mode[bx4 as usize] as usize]
                as usize][dav1d_intra_mode_context[(*t).l.mode[by4 as usize] as usize]
                as usize])
                .as_mut_ptr()
        };
        (*b).c2rust_unnamed.c2rust_unnamed.y_mode = dav1d_msac_decode_symbol_adapt_c(
            &raw mut (*ts).msac,
            ymode_cdf,
            (N_INTRA_PRED_MODES as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        ) as uint8_t;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-ymode[%d]: r=%d\n\0" as *const u8 as *const core::ffi::c_char,
                (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
        if *b_dim.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
            + *b_dim.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
            >= 2 as core::ffi::c_int
            && (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int
                >= VERT_PRED as core::ffi::c_int
            && (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int
                <= VERT_LEFT_PRED as core::ffi::c_int
        {
            let acdf: *mut uint16_t = ((*ts)
                .cdf
                .m
                .angle_delta[((*b).c2rust_unnamed.c2rust_unnamed.y_mode
                as core::ffi::c_int - VERT_PRED as core::ffi::c_int) as usize])
                .as_mut_ptr();
            let angle: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
                &raw mut (*ts).msac,
                acdf,
                6 as size_t,
            ) as core::ffi::c_int;
            (*b).c2rust_unnamed.c2rust_unnamed.y_angle = (angle - 3 as core::ffi::c_int)
                as int8_t;
        } else {
            (*b).c2rust_unnamed.c2rust_unnamed.y_angle = 0 as int8_t;
        }
        if has_chroma != 0 {
            let cfl_allowed: core::ffi::c_int = if (*(*f).frame_hdr)
                .segmentation
                .lossless[(*b).seg_id as usize] as core::ffi::c_int != 0
            {
                (cbw4 == 1 as core::ffi::c_int && cbh4 == 1 as core::ffi::c_int)
                    as core::ffi::c_int
            } else {
                (cfl_allowed_mask
                    & ((1 as core::ffi::c_int) << bs as core::ffi::c_uint)
                        as core::ffi::c_uint != 0) as core::ffi::c_int
            };
            let uvmode_cdf: *mut uint16_t = ((*ts)
                .cdf
                .m
                .uv_mode[cfl_allowed
                as usize][(*b).c2rust_unnamed.c2rust_unnamed.y_mode as usize])
                .as_mut_ptr();
            (*b).c2rust_unnamed.c2rust_unnamed.uv_mode = dav1d_msac_decode_symbol_adapt_c(
                &raw mut (*ts).msac,
                uvmode_cdf,
                (N_UV_INTRA_PRED_MODES as core::ffi::c_int - 1 as core::ffi::c_int
                    - (cfl_allowed == 0) as core::ffi::c_int) as size_t,
            ) as uint8_t;
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-uvmode[%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
            (*b).c2rust_unnamed.c2rust_unnamed.uv_angle = 0 as int8_t;
            if (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as core::ffi::c_int
                == CFL_PRED as core::ffi::c_int
            {
                let sign: core::ffi::c_int = (dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.cfl_sign).as_mut_ptr(),
                    7 as size_t,
                ))
                    .wrapping_add(1 as core::ffi::c_uint) as core::ffi::c_int;
                let sign_u: core::ffi::c_int = sign * 0x56 as core::ffi::c_int
                    >> 8 as core::ffi::c_int;
                let sign_v: core::ffi::c_int = sign - sign_u * 3 as core::ffi::c_int;
                if !(sign_u == sign / 3 as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 9],
                            [core::ffi::c_char; 9],
                        >(*b"decode_b\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        1088 as core::ffi::c_int,
                        b"sign_u == sign / 3\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                if sign_u != 0 {
                    let ctx: core::ffi::c_int = (sign_u == 2 as core::ffi::c_int)
                        as core::ffi::c_int * 3 as core::ffi::c_int + sign_v;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .cfl_alpha[0 as core::ffi::c_int as usize] = (dav1d_msac_decode_symbol_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts).cdf.m.cfl_alpha[ctx as usize]).as_mut_ptr(),
                        15 as size_t,
                    ))
                        .wrapping_add(1 as core::ffi::c_uint) as int8_t;
                    if sign_u == 1 as core::ffi::c_int {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .cfl_alpha[0 as core::ffi::c_int as usize] = -((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .cfl_alpha[0 as core::ffi::c_int as usize]
                            as core::ffi::c_int) as int8_t;
                    }
                } else {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .cfl_alpha[0 as core::ffi::c_int as usize] = 0 as int8_t;
                }
                if sign_v != 0 {
                    let ctx_0: core::ffi::c_int = (sign_v == 2 as core::ffi::c_int)
                        as core::ffi::c_int * 3 as core::ffi::c_int + sign_u;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .cfl_alpha[1 as core::ffi::c_int as usize] = (dav1d_msac_decode_symbol_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts).cdf.m.cfl_alpha[ctx_0 as usize]).as_mut_ptr(),
                        15 as size_t,
                    ))
                        .wrapping_add(1 as core::ffi::c_uint) as int8_t;
                    if sign_v == 1 as core::ffi::c_int {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .cfl_alpha[1 as core::ffi::c_int as usize] = -((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .cfl_alpha[1 as core::ffi::c_int as usize]
                            as core::ffi::c_int) as int8_t;
                    }
                } else {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .cfl_alpha[1 as core::ffi::c_int as usize] = 0 as int8_t;
                }
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-uvalphas[%d/%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .cfl_alpha[0 as core::ffi::c_int as usize]
                            as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .cfl_alpha[1 as core::ffi::c_int as usize]
                            as core::ffi::c_int,
                        (*ts).msac.rng,
                    );
                }
            } else if *b_dim.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int
                + *b_dim.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                >= 2 as core::ffi::c_int
                && (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as core::ffi::c_int
                    >= VERT_PRED as core::ffi::c_int
                && (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as core::ffi::c_int
                    <= VERT_LEFT_PRED as core::ffi::c_int
            {
                let acdf_0: *mut uint16_t = ((*ts)
                    .cdf
                    .m
                    .angle_delta[((*b).c2rust_unnamed.c2rust_unnamed.uv_mode
                    as core::ffi::c_int - VERT_PRED as core::ffi::c_int) as usize])
                    .as_mut_ptr();
                let angle_0: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    acdf_0,
                    6 as size_t,
                ) as core::ffi::c_int;
                (*b).c2rust_unnamed.c2rust_unnamed.uv_angle = (angle_0
                    - 3 as core::ffi::c_int) as int8_t;
            }
        }
        (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1 as core::ffi::c_int as usize] = 0
            as uint8_t;
        (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0 as core::ffi::c_int as usize] = (*b)
            .c2rust_unnamed
            .c2rust_unnamed
            .pal_sz[1 as core::ffi::c_int as usize];
        if (*(*f).frame_hdr).allow_screen_content_tools as core::ffi::c_int != 0
            && imax(bw4, bh4) <= 16 as core::ffi::c_int
            && bw4 + bh4 >= 4 as core::ffi::c_int
        {
            let sz_ctx: core::ffi::c_int = *b_dim.offset(2 as core::ffi::c_int as isize)
                as core::ffi::c_int
                + *b_dim.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int
                - 2 as core::ffi::c_int;
            if (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int
                == DC_PRED as core::ffi::c_int
            {
                let pal_ctx: core::ffi::c_int = ((*(*t).a).pal_sz[bx4 as usize]
                    as core::ffi::c_int > 0 as core::ffi::c_int) as core::ffi::c_int
                    + ((*t).l.pal_sz[by4 as usize] as core::ffi::c_int
                        > 0 as core::ffi::c_int) as core::ffi::c_int;
                let use_y_pal: core::ffi::c_int = dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.pal_y[sz_ctx as usize][pal_ctx as usize]).as_mut_ptr(),
                ) as core::ffi::c_int;
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-y_pal[%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        use_y_pal,
                        (*ts).msac.rng,
                    );
                }
                if use_y_pal != 0 {
                    ((*f).bd_fn.read_pal_plane)
                        .expect(
                            "non-null function pointer",
                        )(t, b, 0 as core::ffi::c_int, sz_ctx, bx4, by4);
                }
            }
            if has_chroma != 0
                && (*b).c2rust_unnamed.c2rust_unnamed.uv_mode as core::ffi::c_int
                    == DC_PRED as core::ffi::c_int
            {
                let pal_ctx_0: core::ffi::c_int = ((*b)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .pal_sz[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    > 0 as core::ffi::c_int) as core::ffi::c_int;
                let use_uv_pal: core::ffi::c_int = dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.pal_uv[pal_ctx_0 as usize]).as_mut_ptr(),
                ) as core::ffi::c_int;
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-uv_pal[%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        use_uv_pal,
                        (*ts).msac.rng,
                    );
                }
                if use_uv_pal != 0 {
                    ((*f).bd_fn.read_pal_uv)
                        .expect("non-null function pointer")(t, b, sz_ctx, bx4, by4);
                }
            }
        }
        if (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int
            == DC_PRED as core::ffi::c_int
            && (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0 as core::ffi::c_int as usize]
                == 0
            && imax(
                *b_dim.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int,
                *b_dim.offset(3 as core::ffi::c_int as isize) as core::ffi::c_int,
            ) <= 3 as core::ffi::c_int
            && (*(*f).seq_hdr).filter_intra as core::ffi::c_int != 0
        {
            let is_filter: core::ffi::c_int = dav1d_msac_decode_bool_adapt_c(
                &raw mut (*ts).msac,
                ((*ts).cdf.m.use_filter_intra[bs as usize]).as_mut_ptr(),
            ) as core::ffi::c_int;
            if is_filter != 0 {
                (*b).c2rust_unnamed.c2rust_unnamed.y_mode = FILTER_PRED
                    as core::ffi::c_int as uint8_t;
                (*b).c2rust_unnamed.c2rust_unnamed.y_angle = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.filter_intra).as_mut_ptr(),
                    4 as size_t,
                ) as int8_t;
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-filterintramode[%d/%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int,
                    (*b).c2rust_unnamed.c2rust_unnamed.y_angle as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
        }
        if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0 as core::ffi::c_int as usize] != 0
        {
            let mut pal_idx: *mut uint8_t = 0 as *mut uint8_t;
            if (*t).frame_thread.pass != 0 {
                let p: core::ffi::c_int = (*t).frame_thread.pass & 1 as core::ffi::c_int;
                if ((*ts).frame_thread[p as usize].pal_idx).is_null() as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 9],
                            [core::ffi::c_char; 9],
                        >(*b"decode_b\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        1163 as core::ffi::c_int,
                        b"ts->frame_thread[p].pal_idx\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                pal_idx = (*ts).frame_thread[p as usize].pal_idx;
                (*ts).frame_thread[p as usize].pal_idx = ((*ts)
                    .frame_thread[p as usize]
                    .pal_idx)
                    .offset((bw4 * bh4 * 8 as core::ffi::c_int) as isize);
            } else {
                pal_idx = ((*t).scratch.c2rust_unnamed_0.pal_idx_y).as_mut_ptr();
            }
            read_pal_indices(
                t,
                pal_idx,
                (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0 as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                0 as core::ffi::c_int,
                w4,
                h4,
                bw4,
                bh4,
            );
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-y-pal-indices: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*ts).msac.rng,
                );
            }
        }
        if has_chroma != 0
            && (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
        {
            let mut pal_idx_0: *mut uint8_t = 0 as *mut uint8_t;
            if (*t).frame_thread.pass != 0 {
                let p_0: core::ffi::c_int = (*t).frame_thread.pass
                    & 1 as core::ffi::c_int;
                if ((*ts).frame_thread[p_0 as usize].pal_idx).is_null()
                    as core::ffi::c_int as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 9],
                            [core::ffi::c_char; 9],
                        >(*b"decode_b\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        1177 as core::ffi::c_int,
                        b"ts->frame_thread[p].pal_idx\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                } else {};
                pal_idx_0 = (*ts).frame_thread[p_0 as usize].pal_idx;
                (*ts).frame_thread[p_0 as usize].pal_idx = ((*ts)
                    .frame_thread[p_0 as usize]
                    .pal_idx)
                    .offset((cbw4 * cbh4 * 8 as core::ffi::c_int) as isize);
            } else {
                pal_idx_0 = ((*t).scratch.c2rust_unnamed_0.pal_idx_uv).as_mut_ptr();
            }
            read_pal_indices(
                t,
                pal_idx_0,
                (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1 as core::ffi::c_int as usize]
                    as core::ffi::c_int,
                1 as core::ffi::c_int,
                cw4,
                ch4,
                cbw4,
                cbh4,
            );
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-uv-pal-indices: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*ts).msac.rng,
                );
            }
        }
        let mut t_dim: *const TxfmInfo = 0 as *const TxfmInfo;
        if (*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize] != 0 {
            (*b).uvtx = TX_4X4 as core::ffi::c_int as uint8_t;
            (*b).c2rust_unnamed.c2rust_unnamed.tx = (*b).uvtx;
            t_dim = &*dav1d_txfm_dimensions
                .as_ptr()
                .offset(TX_4X4 as core::ffi::c_int as isize) as *const TxfmInfo;
        } else {
            (*b).c2rust_unnamed.c2rust_unnamed.tx = dav1d_max_txfm_size_for_bs[bs
                as usize][0 as core::ffi::c_int as usize];
            (*b).uvtx = dav1d_max_txfm_size_for_bs[bs
                as usize][(*f).cur.p.layout as usize];
            t_dim = &*dav1d_txfm_dimensions
                .as_ptr()
                .offset((*b).c2rust_unnamed.c2rust_unnamed.tx as isize)
                as *const TxfmInfo;
            if (*(*f).frame_hdr).txfm_mode as core::ffi::c_uint
                == DAV1D_TX_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
                && (*t_dim).max as core::ffi::c_int > TX_4X4 as core::ffi::c_int
            {
                let tctx: core::ffi::c_int = get_tx_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    t_dim,
                    by4,
                    bx4,
                ) as core::ffi::c_int;
                let tx_cdf: *mut uint16_t = ((*ts)
                    .cdf
                    .m
                    .txsz[((*t_dim).max as core::ffi::c_int - 1 as core::ffi::c_int)
                    as usize][tctx as usize])
                    .as_mut_ptr();
                let mut depth: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    tx_cdf,
                    imin((*t_dim).max as core::ffi::c_int, 2 as core::ffi::c_int)
                        as size_t,
                ) as core::ffi::c_int;
                loop {
                    let fresh7 = depth;
                    depth = depth - 1;
                    if !(fresh7 != 0) {
                        break;
                    }
                    (*b).c2rust_unnamed.c2rust_unnamed.tx = (*t_dim).sub;
                    t_dim = &*dav1d_txfm_dimensions
                        .as_ptr()
                        .offset((*b).c2rust_unnamed.c2rust_unnamed.tx as isize)
                        as *const TxfmInfo;
                }
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-tx[%d]: r=%d\n\0" as *const u8 as *const core::ffi::c_char,
                    (*b).c2rust_unnamed.c2rust_unnamed.tx as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
        }
        if (*t).frame_thread.pass == 1 as core::ffi::c_int {
            ((*f).bd_fn.read_coef_blocks).expect("non-null function pointer")(t, bs, b);
        } else {
            ((*f).bd_fn.recon_b_intra)
                .expect("non-null function pointer")(t, bs, intra_edge_flags, b);
        }
        if (*(*f).frame_hdr).loopfilter.level_y[0 as core::ffi::c_int as usize]
            as core::ffi::c_int != 0
            || (*(*f).frame_hdr).loopfilter.level_y[1 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
        {
            dav1d_create_lf_mask_intra(
                (*t).lf_mask,
                (*f).lf.level,
                (*f).b4_stride,
                &*(*(*(*((*ts).lflvl).offset((*b).seg_id as isize))
                    .as_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .as_ptr()
                    .offset(0 as core::ffi::c_int as isize) as *const uint8_t
                    as *const [[uint8_t; 2]; 8],
                (*t).bx,
                (*t).by,
                (*f).w4,
                (*f).h4,
                bs,
                (*b).c2rust_unnamed.c2rust_unnamed.tx as RectTxfmSize,
                (*b).uvtx as RectTxfmSize,
                (*f).cur.p.layout,
                &raw mut *((*(*t).a).tx_lpf_y).as_mut_ptr().offset(bx4 as isize),
                &raw mut *((*t).l.tx_lpf_y).as_mut_ptr().offset(by4 as isize),
                if has_chroma != 0 {
                    &raw mut *((*(*t).a).tx_lpf_uv).as_mut_ptr().offset(cbx4 as isize)
                } else {
                    0 as *mut uint8_t
                },
                if has_chroma != 0 {
                    &raw mut *((*t).l.tx_lpf_uv).as_mut_ptr().offset(cby4 as isize)
                } else {
                    0 as *mut uint8_t
                },
            );
        }
        let y_mode_nofilt_0: IntraPredMode = (if (*b)
            .c2rust_unnamed
            .c2rust_unnamed
            .y_mode as core::ffi::c_int == FILTER_PRED as core::ffi::c_int
        {
            DC_PRED as core::ffi::c_int
        } else {
            (*b).c2rust_unnamed.c2rust_unnamed.y_mode as core::ffi::c_int
        }) as IntraPredMode;
        let mut edge_1: *mut BlockContext = (*t).a;
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut off_1: core::ffi::c_int = bx4;
        while i_2 < 2 as core::ffi::c_int {
            let mut t_lsz: core::ffi::c_int = *(&(*t_dim).lw as *const uint8_t
                as *mut uint8_t)
                .offset(i_2 as isize) as core::ffi::c_int;
            match *b_dim.offset((2 as core::ffi::c_int + i_2) as isize)
                as core::ffi::c_int
            {
                0 => {
                    (*(&raw mut *((*edge_1).tx_intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias8))
                        .u8_0 = (t_lsz * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_1).tx).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias8))
                        .u8_0 = (t_lsz * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_1).mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (y_mode_nofilt_0 as core::ffi::c_uint)
                        .wrapping_mul(0x1 as core::ffi::c_uint) as uint8_t;
                    (*(&raw mut *((*edge_1).pal_sz).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .pal_sz[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_1).seg_pred).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (seg_pred * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_1).skip_mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_1).intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (1 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_1).skip).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).skip as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_2 as isize))
                        .as_mut_ptr()
                        .offset(off_1 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = ((if has_chroma != 0 {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pal_sz[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) * 0x1 as core::ffi::c_int) as uint8_t;
                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                        & 1 as core::ffi::c_uint != 0
                    {
                        (*(&raw mut *((*edge_1).comp_type)
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias8))
                            .u8_0 = (COMP_INTER_NONE as core::ffi::c_int
                            * 0x1 as core::ffi::c_int) as uint8_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias8))
                            .u8_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias8))
                            .u8_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias8))
                            .u8_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            * 0x1 as core::ffi::c_int) as uint8_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias8))
                            .u8_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            * 0x1 as core::ffi::c_int) as uint8_t;
                    }
                }
                1 => {
                    (*(&raw mut *((*edge_1).tx_intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias16))
                        .u16_0 = (t_lsz * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_1).tx).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias16))
                        .u16_0 = (t_lsz * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_1).mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (y_mode_nofilt_0 as core::ffi::c_uint)
                        .wrapping_mul(0x101 as core::ffi::c_uint) as uint16_t;
                    (*(&raw mut *((*edge_1).pal_sz).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .pal_sz[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_1).seg_pred).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (seg_pred * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_1).skip_mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_1).intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (1 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_1).skip).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).skip as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_2 as isize))
                        .as_mut_ptr()
                        .offset(off_1 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = ((if has_chroma != 0 {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pal_sz[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) * 0x101 as core::ffi::c_int) as uint16_t;
                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                        & 1 as core::ffi::c_uint != 0
                    {
                        (*(&raw mut *((*edge_1).comp_type)
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias16))
                            .u16_0 = (COMP_INTER_NONE as core::ffi::c_int
                            * 0x101 as core::ffi::c_int) as uint16_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias16))
                            .u16_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias16))
                            .u16_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias16))
                            .u16_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            * 0x101 as core::ffi::c_int) as uint16_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias16))
                            .u16_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            * 0x101 as core::ffi::c_int) as uint16_t;
                    }
                }
                2 => {
                    (*(&raw mut *((*edge_1).tx_intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias32))
                        .u32_0 = (t_lsz as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).tx).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias32))
                        .u32_0 = (t_lsz as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (y_mode_nofilt_0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).pal_sz).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .pal_sz[0 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).seg_pred).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (seg_pred as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).skip_mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (1 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_1).skip).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).skip as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_2 as isize))
                        .as_mut_ptr()
                        .offset(off_1 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = ((if has_chroma != 0 {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pal_sz[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                        & 1 as core::ffi::c_uint != 0
                    {
                        (*(&raw mut *((*edge_1).comp_type)
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias32))
                            .u32_0 = (COMP_INTER_NONE as core::ffi::c_int
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias32))
                            .u32_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias32))
                            .u32_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias32))
                            .u32_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias32))
                            .u32_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            as core::ffi::c_uint)
                            .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    }
                }
                3 => {
                    (*(&raw mut *((*edge_1).tx_intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = (t_lsz as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).tx).as_mut_ptr().offset(off_1 as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = (t_lsz as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (y_mode_nofilt_0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).pal_sz).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .pal_sz[0 as core::ffi::c_int as usize]
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).seg_pred).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (seg_pred as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).skip_mode).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).intra).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (1 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_1).skip).as_mut_ptr().offset(off_1 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).skip as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_2 as isize))
                        .as_mut_ptr()
                        .offset(off_1 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = ((if has_chroma != 0 {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pal_sz[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                        & 1 as core::ffi::c_uint != 0
                    {
                        (*(&raw mut *((*edge_1).comp_type)
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias64))
                            .u64_0 = (COMP_INTER_NONE as core::ffi::c_int
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias64))
                            .u64_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *(*((*edge_1).ref_0)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut int8_t as *mut alias64))
                            .u64_0 = (-(1 as core::ffi::c_int) as uint8_t
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias64))
                            .u64_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                        (*(&raw mut *(*((*edge_1).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t as *mut alias64))
                            .u64_0 = (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            as core::ffi::c_ulonglong)
                            .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                            as uint64_t;
                    }
                }
                4 => {
                    memset(
                        &raw mut *((*edge_1).tx_intra).as_mut_ptr().offset(off_1 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        t_lsz,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).tx).as_mut_ptr().offset(off_1 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        t_lsz,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).mode).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        y_mode_nofilt_0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).pal_sz).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pal_sz[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).seg_pred).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        seg_pred,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).skip_mode).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).intra).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        1 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).skip).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_2 as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        if has_chroma != 0 {
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .pal_sz[1 as core::ffi::c_int as usize] as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        },
                        16 as size_t,
                    );
                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                        & 1 as core::ffi::c_uint != 0
                    {
                        memset(
                            &raw mut *((*edge_1).comp_type)
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            COMP_INTER_NONE as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).ref_0)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut int8_t
                                as *mut core::ffi::c_void,
                            -(1 as core::ffi::c_int) as uint8_t as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).ref_0)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut int8_t
                                as *mut core::ffi::c_void,
                            -(1 as core::ffi::c_int) as uint8_t as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).filter)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int,
                            16 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).filter)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int,
                            16 as size_t,
                        );
                    }
                }
                5 => {
                    memset(
                        &raw mut *((*edge_1).tx_intra).as_mut_ptr().offset(off_1 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        t_lsz,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).tx).as_mut_ptr().offset(off_1 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        t_lsz,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).mode).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        y_mode_nofilt_0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).pal_sz).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .pal_sz[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).seg_pred).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        seg_pred,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).skip_mode).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).intra).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        1 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_1).skip).as_mut_ptr().offset(off_1 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_2 as isize))
                            .as_mut_ptr()
                            .offset(off_1 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        if has_chroma != 0 {
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .pal_sz[1 as core::ffi::c_int as usize] as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        },
                        32 as size_t,
                    );
                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                        & 1 as core::ffi::c_uint != 0
                    {
                        memset(
                            &raw mut *((*edge_1).comp_type)
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            COMP_INTER_NONE as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).ref_0)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut int8_t
                                as *mut core::ffi::c_void,
                            -(1 as core::ffi::c_int) as uint8_t as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).ref_0)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut int8_t
                                as *mut core::ffi::c_void,
                            -(1 as core::ffi::c_int) as uint8_t as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).filter)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int,
                            32 as size_t,
                        );
                        memset(
                            &raw mut *(*((*edge_1).filter)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(off_1 as isize) as *mut uint8_t
                                as *mut core::ffi::c_void,
                            DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int,
                            32 as size_t,
                        );
                    }
                }
                _ => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1253 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
            }
            i_2 += 1;
            off_1 = by4;
            edge_1 = &raw mut (*t).l;
        }
        if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[0 as core::ffi::c_int as usize] != 0
        {
            ((*f).bd_fn.copy_pal_block_y)
                .expect("non-null function pointer")(t, bx4, by4, bw4, bh4);
        }
        if has_chroma != 0 {
            let mut uv_mode_0: uint8_t = (*b).c2rust_unnamed.c2rust_unnamed.uv_mode;
            (dav1d_memset_pow2[ulog2(cbw4 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*(*t).a).uvmode).as_mut_ptr().offset(cbx4 as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
                uv_mode_0 as core::ffi::c_int,
            );
            (dav1d_memset_pow2[ulog2(cbh4 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*t).l.uvmode).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                    as *mut core::ffi::c_void,
                uv_mode_0 as core::ffi::c_int,
            );
            if (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1 as core::ffi::c_int as usize]
                != 0
            {
                ((*f).bd_fn.copy_pal_block_uv)
                    .expect("non-null function pointer")(t, bx4, by4, bw4, bh4);
            }
        }
        if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
            != 0 || (*(*f).frame_hdr).allow_intrabc as core::ffi::c_int != 0
        {
            splat_intraref((*f).c, t, bs, bw4, bh4);
        }
    } else if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
        == 0
    {
        let mut mvstack: [refmvs_candidate; 8] = [refmvs_candidate {
            mv: refmvs_mvpair {
                mv: [mv {
                    c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                }; 2],
            },
            weight: 0,
        }; 8];
        let mut n_mvs: core::ffi::c_int = 0;
        let mut ctx_1: core::ffi::c_int = 0;
        dav1d_refmvs_find(
            &raw mut (*t).rt,
            mvstack.as_mut_ptr(),
            &raw mut n_mvs,
            &raw mut ctx_1,
            refmvs_refpair {
                ref_0: [
                    0 as core::ffi::c_int as int8_t,
                    -(1 as core::ffi::c_int) as int8_t,
                ],
            },
            bs,
            intra_edge_flags,
            (*t).by,
            (*t).bx,
        );
        if mvstack[0 as core::ffi::c_int as usize]
            .mv
            .mv[0 as core::ffi::c_int as usize]
            .n != 0
        {
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize] = mvstack[0 as core::ffi::c_int
                    as usize]
                .mv
                .mv[0 as core::ffi::c_int as usize] as mv;
        } else if mvstack[1 as core::ffi::c_int as usize]
            .mv
            .mv[0 as core::ffi::c_int as usize]
            .n != 0
        {
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize] = mvstack[1 as core::ffi::c_int
                    as usize]
                .mv
                .mv[0 as core::ffi::c_int as usize] as mv;
        } else if (*t).by
            - ((16 as core::ffi::c_int) << (*(*f).seq_hdr).sb128 as core::ffi::c_int)
            < (*ts).tiling.row_start
        {
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y = 0 as int16_t;
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x = (-((512 as core::ffi::c_int)
                << (*(*f).seq_hdr).sb128 as core::ffi::c_int) - 2048 as core::ffi::c_int)
                as int16_t;
        } else {
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y = -((512 as core::ffi::c_int)
                << (*(*f).seq_hdr).sb128 as core::ffi::c_int) as int16_t;
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x = 0 as int16_t;
        }
        let ref_0: mv = (*b)
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mv[0 as core::ffi::c_int as usize];
        read_mv_residual(
            ts,
            &raw mut *((*b).c2rust_unnamed.c2rust_unnamed_0.c2rust_unnamed.c2rust_unnamed.mv)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize),
            -(1 as core::ffi::c_int),
        );
        let mut border_left: core::ffi::c_int = (*ts).tiling.col_start
            * 4 as core::ffi::c_int;
        let mut border_top: core::ffi::c_int = (*ts).tiling.row_start
            * 4 as core::ffi::c_int;
        if has_chroma != 0 {
            if bw4 < 2 as core::ffi::c_int && ss_hor != 0 {
                border_left += 4 as core::ffi::c_int;
            }
            if bh4 < 2 as core::ffi::c_int && ss_ver != 0 {
                border_top += 4 as core::ffi::c_int;
            }
        }
        let mut src_left: core::ffi::c_int = (*t).bx * 4 as core::ffi::c_int
            + ((*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .x as core::ffi::c_int >> 3 as core::ffi::c_int);
        let mut src_top: core::ffi::c_int = (*t).by * 4 as core::ffi::c_int
            + ((*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize]
                .c2rust_unnamed
                .y as core::ffi::c_int >> 3 as core::ffi::c_int);
        let mut src_right: core::ffi::c_int = src_left + bw4 * 4 as core::ffi::c_int;
        let mut src_bottom: core::ffi::c_int = src_top + bh4 * 4 as core::ffi::c_int;
        let border_right: core::ffi::c_int = ((*ts).tiling.col_end
            + (bw4 - 1 as core::ffi::c_int) & !(bw4 - 1 as core::ffi::c_int))
            * 4 as core::ffi::c_int;
        if src_left < border_left {
            src_right += border_left - src_left;
            src_left += border_left - src_left;
        } else if src_right > border_right {
            src_left -= src_right - border_right;
            src_right -= src_right - border_right;
        }
        if src_top < border_top {
            src_bottom += border_top - src_top;
            src_top += border_top - src_top;
        }
        let sbx: core::ffi::c_int = ((*t).bx
            >> 4 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int)
            << 6 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int;
        let sby: core::ffi::c_int = ((*t).by
            >> 4 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int)
            << 6 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int;
        let sb_size: core::ffi::c_int = (1 as core::ffi::c_int)
            << 6 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int;
        if src_bottom > sby && src_right > sbx {
            if src_top - border_top >= src_bottom - sby {
                src_top -= src_bottom - sby;
                src_bottom -= src_bottom - sby;
            } else if src_left - border_left >= src_right - sbx {
                src_left -= src_right - sbx;
                src_right -= src_right - sbx;
            }
        }
        if src_bottom > sby + sb_size {
            src_top -= src_bottom - (sby + sb_size);
            src_bottom -= src_bottom - (sby + sb_size);
        }
        if src_bottom > sby && src_right > sbx {
            return -(1 as core::ffi::c_int);
        }
        (*b)
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mv[0 as core::ffi::c_int as usize]
            .c2rust_unnamed
            .x = ((src_left - (*t).bx * 4 as core::ffi::c_int) * 8 as core::ffi::c_int)
            as int16_t;
        (*b)
            .c2rust_unnamed
            .c2rust_unnamed_0
            .c2rust_unnamed
            .c2rust_unnamed
            .mv[0 as core::ffi::c_int as usize]
            .c2rust_unnamed
            .y = ((src_top - (*t).by * 4 as core::ffi::c_int) * 8 as core::ffi::c_int)
            as int16_t;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-dmv[%d/%d,ref=%d/%d|%d/%d]: r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int,
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int,
                ref_0.c2rust_unnamed.y as core::ffi::c_int,
                ref_0.c2rust_unnamed.x as core::ffi::c_int,
                mvstack[0 as core::ffi::c_int as usize]
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .y as core::ffi::c_int,
                mvstack[0 as core::ffi::c_int as usize]
                    .mv
                    .mv[0 as core::ffi::c_int as usize]
                    .c2rust_unnamed
                    .x as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
        read_vartx_tree(t, b, bs, bx4, by4);
        if (*t).frame_thread.pass == 1 as core::ffi::c_int {
            ((*f).bd_fn.read_coef_blocks).expect("non-null function pointer")(t, bs, b);
            (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d = FILTER_2D_BILINEAR
                as core::ffi::c_int as uint8_t;
        } else if ((*f).bd_fn.recon_b_inter)
            .expect("non-null function pointer")(t, bs, b) != 0
        {
            return -(1 as core::ffi::c_int)
        }
        splat_intrabc_mv((*f).c, t, bs, b, bw4, bh4);
        let mut edge_2: *mut BlockContext = (*t).a;
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut off_2: core::ffi::c_int = bx4;
        while i_3 < 2 as core::ffi::c_int {
            match *b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                as core::ffi::c_int
            {
                0 => {
                    (*(&raw mut *((*edge_2).tx_intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut int8_t as *mut alias8))
                        .u8_0 = (*b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                        as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_2).mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (DC_PRED as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_2).pal_sz).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_3 as isize))
                        .as_mut_ptr()
                        .offset(off_2 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_2).seg_pred).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (seg_pred * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_2).skip_mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_2).intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_2).skip).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).skip as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                }
                1 => {
                    (*(&raw mut *((*edge_2).tx_intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut int8_t as *mut alias16))
                        .u16_0 = (*b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                        as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_2).mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (DC_PRED as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_2).pal_sz).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_3 as isize))
                        .as_mut_ptr()
                        .offset(off_2 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_2).seg_pred).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (seg_pred * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_2).skip_mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_2).intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_2).skip).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).skip as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                }
                2 => {
                    (*(&raw mut *((*edge_2).tx_intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut int8_t as *mut alias32))
                        .u32_0 = (*b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_2).mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (DC_PRED as core::ffi::c_int as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_2).pal_sz).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_3 as isize))
                        .as_mut_ptr()
                        .offset(off_2 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_2).seg_pred).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (seg_pred as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_2).skip_mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_2).intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_2).skip).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).skip as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                }
                3 => {
                    (*(&raw mut *((*edge_2).tx_intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = (*b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_2).mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (DC_PRED as core::ffi::c_int as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_2).pal_sz).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_3 as isize))
                        .as_mut_ptr()
                        .offset(off_2 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_2).seg_pred).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (seg_pred as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_2).skip_mode).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_2).intra).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_2).skip).as_mut_ptr().offset(off_2 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).skip as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                }
                4 => {
                    memset(
                        &raw mut *((*edge_2).tx_intra).as_mut_ptr().offset(off_2 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        *b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                            as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).mode).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        DC_PRED as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).pal_sz).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_3 as isize))
                            .as_mut_ptr()
                            .offset(off_2 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).seg_pred).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        seg_pred,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).skip_mode).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).intra).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).skip).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip as core::ffi::c_int,
                        16 as size_t,
                    );
                }
                5 => {
                    memset(
                        &raw mut *((*edge_2).tx_intra).as_mut_ptr().offset(off_2 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        *b_dim.offset((2 as core::ffi::c_int + i_3) as isize)
                            as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).mode).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        DC_PRED as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).pal_sz).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_3 as isize))
                            .as_mut_ptr()
                            .offset(off_2 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).seg_pred).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        seg_pred,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).skip_mode).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).intra).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_2).skip).as_mut_ptr().offset(off_2 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip as core::ffi::c_int,
                        32 as size_t,
                    );
                }
                _ => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1375 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
            }
            i_3 += 1;
            off_2 = by4;
            edge_2 = &raw mut (*t).l;
        }
        if has_chroma != 0 {
            (dav1d_memset_pow2[ulog2(cbw4 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*(*t).a).uvmode).as_mut_ptr().offset(cbx4 as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
                DC_PRED as core::ffi::c_int,
            );
            (dav1d_memset_pow2[ulog2(cbh4 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*t).l.uvmode).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                    as *mut core::ffi::c_void,
                DC_PRED as core::ffi::c_int,
            );
        }
    } else {
        let mut is_comp: core::ffi::c_int = 0;
        let mut has_subpel_filter: core::ffi::c_int = 0;
        if (*b).skip_mode != 0 {
            is_comp = 1 as core::ffi::c_int;
        } else if (seg.is_null()
            || (*seg).ref_0 as core::ffi::c_int == -(1 as core::ffi::c_int)
                && (*seg).globalmv == 0 && (*seg).skip == 0)
            && (*(*f).frame_hdr).switchable_comp_refs as core::ffi::c_int != 0
            && imin(bw4, bh4) > 1 as core::ffi::c_int
        {
            let ctx_2: core::ffi::c_int = get_comp_ctx(
                (*t).a,
                &raw mut (*t).l,
                by4,
                bx4,
                have_top,
                have_left,
            ) as core::ffi::c_int;
            is_comp = dav1d_msac_decode_bool_adapt_c(
                &raw mut (*ts).msac,
                ((*ts).cdf.m.comp[ctx_2 as usize]).as_mut_ptr(),
            ) as core::ffi::c_int;
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-compflag[%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    is_comp,
                    (*ts).msac.rng,
                );
            }
        } else {
            is_comp = 0 as core::ffi::c_int;
        }
        if (*b).skip_mode != 0 {
            (*b).c2rust_unnamed.c2rust_unnamed_0.ref_0[0 as core::ffi::c_int as usize] = (*(*f)
                .frame_hdr)
                .skip_mode_refs[0 as core::ffi::c_int as usize];
            (*b).c2rust_unnamed.c2rust_unnamed_0.ref_0[1 as core::ffi::c_int as usize] = (*(*f)
                .frame_hdr)
                .skip_mode_refs[1 as core::ffi::c_int as usize];
            (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type = COMP_INTER_AVG
                as core::ffi::c_int as uint8_t;
            (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode = NEARESTMV_NEARESTMV
                as core::ffi::c_int as uint8_t;
            (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEAREST_DRL
                as core::ffi::c_int as uint8_t;
            has_subpel_filter = 0 as core::ffi::c_int;
            let mut mvstack_0: [refmvs_candidate; 8] = [refmvs_candidate {
                mv: refmvs_mvpair {
                    mv: [mv {
                        c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                    }; 2],
                },
                weight: 0,
            }; 8];
            let mut n_mvs_0: core::ffi::c_int = 0;
            let mut ctx_3: core::ffi::c_int = 0;
            dav1d_refmvs_find(
                &raw mut (*t).rt,
                mvstack_0.as_mut_ptr(),
                &raw mut n_mvs_0,
                &raw mut ctx_3,
                refmvs_refpair {
                    ref_0: [
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                            + 1 as core::ffi::c_int) as int8_t,
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                            + 1 as core::ffi::c_int) as int8_t,
                    ],
                },
                bs,
                intra_edge_flags,
                (*t).by,
                (*t).bx,
            );
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[0 as core::ffi::c_int as usize] = mvstack_0[0 as core::ffi::c_int
                    as usize]
                .mv
                .mv[0 as core::ffi::c_int as usize] as mv;
            (*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .c2rust_unnamed
                .c2rust_unnamed
                .mv[1 as core::ffi::c_int as usize] = mvstack_0[0 as core::ffi::c_int
                    as usize]
                .mv
                .mv[1 as core::ffi::c_int as usize] as mv;
            fix_mv_precision(
                (*f).frame_hdr,
                &raw mut *((*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
            );
            fix_mv_precision(
                (*f).frame_hdr,
                &raw mut *((*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mv)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize),
            );
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-skipmodeblock[mv=1:y=%d,x=%d,2:y=%d,x=%d,refs=%d+%d\n\0"
                        as *const u8 as *const core::ffi::c_char,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .y as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .x as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .y as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .x as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                );
            }
        } else if is_comp != 0 {
            let dir_ctx: core::ffi::c_int = get_comp_dir_ctx(
                (*t).a,
                &raw mut (*t).l,
                by4,
                bx4,
                have_top,
                have_left,
            ) as core::ffi::c_int;
            if dav1d_msac_decode_bool_adapt_c(
                &raw mut (*ts).msac,
                ((*ts).cdf.m.comp_dir[dir_ctx as usize]).as_mut_ptr(),
            ) != 0
            {
                let ctx1: core::ffi::c_int = av1_get_fwd_ref_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    by4,
                    bx4,
                    have_top,
                    have_left,
                ) as core::ffi::c_int;
                if dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts)
                        .cdf
                        .m
                        .comp_fwd_ref[0 as core::ffi::c_int as usize][ctx1 as usize])
                        .as_mut_ptr(),
                ) != 0
                {
                    let ctx2: core::ffi::c_int = av1_get_fwd_ref_2_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                        have_top,
                        have_left,
                    ) as core::ffi::c_int;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] = (2 as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts)
                                    .cdf
                                    .m
                                    .comp_fwd_ref[2 as core::ffi::c_int
                                    as usize][ctx2 as usize])
                                    .as_mut_ptr(),
                            ),
                        ) as int8_t;
                } else {
                    let ctx2_0: core::ffi::c_int = av1_get_fwd_ref_1_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                        have_top,
                        have_left,
                    ) as core::ffi::c_int;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] = dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .comp_fwd_ref[1 as core::ffi::c_int
                            as usize][ctx2_0 as usize])
                            .as_mut_ptr(),
                    ) as int8_t;
                }
                let ctx3: core::ffi::c_int = av1_get_bwd_ref_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    by4,
                    bx4,
                    have_top,
                    have_left,
                ) as core::ffi::c_int;
                if dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts)
                        .cdf
                        .m
                        .comp_bwd_ref[0 as core::ffi::c_int as usize][ctx3 as usize])
                        .as_mut_ptr(),
                ) != 0
                {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] = 6 as int8_t;
                } else {
                    let ctx4: core::ffi::c_int = av1_get_bwd_ref_1_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                        have_top,
                        have_left,
                    ) as core::ffi::c_int;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] = (4 as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts)
                                    .cdf
                                    .m
                                    .comp_bwd_ref[1 as core::ffi::c_int
                                    as usize][ctx4 as usize])
                                    .as_mut_ptr(),
                            ),
                        ) as int8_t;
                }
            } else {
                let uctx_p: core::ffi::c_int = av1_get_ref_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    by4,
                    bx4,
                    have_top,
                    have_left,
                ) as core::ffi::c_int;
                if dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts)
                        .cdf
                        .m
                        .comp_uni_ref[0 as core::ffi::c_int as usize][uctx_p as usize])
                        .as_mut_ptr(),
                ) != 0
                {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] = 4 as int8_t;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] = 6 as int8_t;
                } else {
                    let uctx_p1: core::ffi::c_int = av1_get_uni_p1_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                        have_top,
                        have_left,
                    ) as core::ffi::c_int;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] = 0 as int8_t;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] = (1 as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts)
                                    .cdf
                                    .m
                                    .comp_uni_ref[1 as core::ffi::c_int
                                    as usize][uctx_p1 as usize])
                                    .as_mut_ptr(),
                            ),
                        ) as int8_t;
                    if (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                        == 2 as core::ffi::c_int
                    {
                        let uctx_p2: core::ffi::c_int = av1_get_fwd_ref_2_ctx(
                            (*t).a,
                            &raw mut (*t).l,
                            by4,
                            bx4,
                            have_top,
                            have_left,
                        ) as core::ffi::c_int;
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[1 as core::ffi::c_int as usize] = ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bool_adapt_c(
                                    &raw mut (*ts).msac,
                                    ((*ts)
                                        .cdf
                                        .m
                                        .comp_uni_ref[2 as core::ffi::c_int
                                        as usize][uctx_p2 as usize])
                                        .as_mut_ptr(),
                                ),
                            ) as int8_t as int8_t;
                    }
                }
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-refs[%d/%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
            let mut mvstack_1: [refmvs_candidate; 8] = [refmvs_candidate {
                mv: refmvs_mvpair {
                    mv: [mv {
                        c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                    }; 2],
                },
                weight: 0,
            }; 8];
            let mut n_mvs_1: core::ffi::c_int = 0;
            let mut ctx_4: core::ffi::c_int = 0;
            dav1d_refmvs_find(
                &raw mut (*t).rt,
                mvstack_1.as_mut_ptr(),
                &raw mut n_mvs_1,
                &raw mut ctx_4,
                refmvs_refpair {
                    ref_0: [
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                            + 1 as core::ffi::c_int) as int8_t,
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[1 as core::ffi::c_int as usize] as core::ffi::c_int
                            + 1 as core::ffi::c_int) as int8_t,
                    ],
                },
                bs,
                intra_edge_flags,
                (*t).by,
                (*t).bx,
            );
            (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode = dav1d_msac_decode_symbol_adapt_c(
                &raw mut (*ts).msac,
                ((*ts).cdf.m.comp_inter_mode[ctx_4 as usize]).as_mut_ptr(),
                (N_COMP_INTER_PRED_MODES as core::ffi::c_int - 1 as core::ffi::c_int)
                    as size_t,
            ) as uint8_t;
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-compintermode[%d,ctx=%d,n_mvs=%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as core::ffi::c_int,
                    ctx_4,
                    n_mvs_1,
                    (*ts).msac.rng,
                );
            }
            let im: *const uint8_t = (dav1d_comp_inter_pred_modes[(*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .inter_mode as usize])
                .as_ptr();
            (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEAREST_DRL
                as core::ffi::c_int as uint8_t;
            if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as core::ffi::c_int
                == NEWMV_NEWMV as core::ffi::c_int
            {
                if n_mvs_1 > 1 as core::ffi::c_int {
                    let drl_ctx_v1: core::ffi::c_int = get_drl_context(
                        mvstack_1.as_mut_ptr(),
                        0 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .drl_idx as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts).cdf.m.drl_bit[drl_ctx_v1 as usize]).as_mut_ptr(),
                            ),
                        ) as uint8_t as uint8_t;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                        == NEARER_DRL as core::ffi::c_int
                        && n_mvs_1 > 2 as core::ffi::c_int
                    {
                        let drl_ctx_v2: core::ffi::c_int = get_drl_context(
                            mvstack_1.as_mut_ptr(),
                            1 as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bool_adapt_c(
                                    &raw mut (*ts).msac,
                                    ((*ts).cdf.m.drl_bit[drl_ctx_v2 as usize]).as_mut_ptr(),
                                ),
                            ) as uint8_t as uint8_t;
                    }
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-drlidx[%d,n_mvs=%d]: r=%d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx
                                as core::ffi::c_int,
                            n_mvs_1,
                            (*ts).msac.rng,
                        );
                    }
                }
            } else if *im.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                == NEARMV as core::ffi::c_int
                || *im.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
                    == NEARMV as core::ffi::c_int
            {
                (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEARER_DRL
                    as core::ffi::c_int as uint8_t;
                if n_mvs_1 > 2 as core::ffi::c_int {
                    let drl_ctx_v2_0: core::ffi::c_int = get_drl_context(
                        mvstack_1.as_mut_ptr(),
                        1 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .drl_idx as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts).cdf.m.drl_bit[drl_ctx_v2_0 as usize]).as_mut_ptr(),
                            ),
                        ) as uint8_t as uint8_t;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                        == NEAR_DRL as core::ffi::c_int
                        && n_mvs_1 > 3 as core::ffi::c_int
                    {
                        let drl_ctx_v3: core::ffi::c_int = get_drl_context(
                            mvstack_1.as_mut_ptr(),
                            2 as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bool_adapt_c(
                                    &raw mut (*ts).msac,
                                    ((*ts).cdf.m.drl_bit[drl_ctx_v3 as usize]).as_mut_ptr(),
                                ),
                            ) as uint8_t as uint8_t;
                    }
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-drlidx[%d,n_mvs=%d]: r=%d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx
                                as core::ffi::c_int,
                            n_mvs_1,
                            (*ts).msac.rng,
                        );
                    }
                }
            }
            if !((*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                >= NEAREST_DRL as core::ffi::c_int
                && (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                    <= NEARISH_DRL as core::ffi::c_int) as core::ffi::c_int
                as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 9],
                        [core::ffi::c_char; 9],
                    >(*b"decode_b\0"))
                        .as_ptr(),
                    b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                    1533 as core::ffi::c_int,
                    b"b->drl_idx >= NEAREST_DRL && b->drl_idx <= NEARISH_DRL\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            } else {};
            has_subpel_filter = (imin(bw4, bh4) == 1 as core::ffi::c_int
                || (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as core::ffi::c_int
                    != GLOBALMV_GLOBALMV as core::ffi::c_int) as core::ffi::c_int;
            match *im.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int {
                1 | 0 => {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = mvstack_1[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as usize]
                        .mv
                        .mv[0 as core::ffi::c_int as usize] as mv;
                    fix_mv_precision(
                        (*f).frame_hdr,
                        &raw mut *((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                    );
                }
                2 => {
                    has_subpel_filter
                        |= ((*(*f).frame_hdr)
                            .gmv[(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .ref_0[0 as core::ffi::c_int as usize] as usize]
                            .type_0 as core::ffi::c_uint
                            == DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                as core::ffi::c_uint) as core::ffi::c_int;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = get_gmv_2d(
                        &raw mut *((*(*f).frame_hdr).gmv)
                            .as_mut_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ),
                        (*t).bx,
                        (*t).by,
                        bw4,
                        bh4,
                        (*f).frame_hdr,
                    ) as mv;
                }
                3 => {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = mvstack_1[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as usize]
                        .mv
                        .mv[0 as core::ffi::c_int as usize] as mv;
                    let mv_prec: core::ffi::c_int = (*(*f).frame_hdr).hp
                        as core::ffi::c_int
                        - (*(*f).frame_hdr).force_integer_mv as core::ffi::c_int;
                    read_mv_residual(
                        ts,
                        &raw mut *((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        mv_prec,
                    );
                }
                _ => {}
            }
            match *im.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int {
                1 | 0 => {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize] = mvstack_1[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as usize]
                        .mv
                        .mv[1 as core::ffi::c_int as usize] as mv;
                    fix_mv_precision(
                        (*f).frame_hdr,
                        &raw mut *((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                    );
                }
                2 => {
                    has_subpel_filter
                        |= ((*(*f).frame_hdr)
                            .gmv[(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .ref_0[1 as core::ffi::c_int as usize] as usize]
                            .type_0 as core::ffi::c_uint
                            == DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                as core::ffi::c_uint) as core::ffi::c_int;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize] = get_gmv_2d(
                        &raw mut *((*(*f).frame_hdr).gmv)
                            .as_mut_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(1 as core::ffi::c_int as isize) as isize,
                            ),
                        (*t).bx,
                        (*t).by,
                        bw4,
                        bh4,
                        (*f).frame_hdr,
                    ) as mv;
                }
                3 => {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize] = mvstack_1[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as usize]
                        .mv
                        .mv[1 as core::ffi::c_int as usize] as mv;
                    let mv_prec_0: core::ffi::c_int = (*(*f).frame_hdr).hp
                        as core::ffi::c_int
                        - (*(*f).frame_hdr).force_integer_mv as core::ffi::c_int;
                    read_mv_residual(
                        ts,
                        &raw mut *((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                        mv_prec_0,
                    );
                }
                _ => {}
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"Post-residual_mv[1:y=%d,x=%d,2:y=%d,x=%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .y as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .x as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .y as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .x as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
            let mut is_segwedge: core::ffi::c_int = 0 as core::ffi::c_int;
            if (*(*f).seq_hdr).masked_compound != 0 {
                let mask_ctx: core::ffi::c_int = get_mask_comp_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    by4,
                    bx4,
                ) as core::ffi::c_int;
                is_segwedge = dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.mask_comp[mask_ctx as usize]).as_mut_ptr(),
                ) as core::ffi::c_int;
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-segwedge_vs_jntavg[%d,ctx=%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        is_segwedge,
                        mask_ctx,
                        (*ts).msac.rng,
                    );
                }
            }
            if is_segwedge == 0 {
                if (*(*f).seq_hdr).jnt_comp != 0 {
                    let jnt_ctx: core::ffi::c_int = get_jnt_comp_ctx(
                        (*(*f).seq_hdr).order_hint_n_bits as core::ffi::c_int,
                        (*(*f).cur.frame_hdr).frame_offset as core::ffi::c_int,
                        (*(*f)
                            .refp[(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .ref_0[0 as core::ffi::c_int as usize] as usize]
                            .p
                            .frame_hdr)
                            .frame_offset as core::ffi::c_int,
                        (*(*f)
                            .refp[(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .ref_0[1 as core::ffi::c_int as usize] as usize]
                            .p
                            .frame_hdr)
                            .frame_offset as core::ffi::c_int,
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                    ) as core::ffi::c_int;
                    (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type = (COMP_INTER_WEIGHTED_AVG
                        as core::ffi::c_int as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts).cdf.m.jnt_comp[jnt_ctx as usize]).as_mut_ptr(),
                            ),
                        ) as uint8_t;
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-jnt_comp[%d,ctx=%d[ac:%d,ar:%d,lc:%d,lr:%d]]: r=%d\n\0"
                                as *const u8 as *const core::ffi::c_char,
                            ((*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                                as core::ffi::c_int == COMP_INTER_AVG as core::ffi::c_int)
                                as core::ffi::c_int,
                            jnt_ctx,
                            (*(*t).a).comp_type[bx4 as usize] as core::ffi::c_int,
                            (*(*t).a).ref_0[0 as core::ffi::c_int as usize][bx4 as usize]
                                as core::ffi::c_int,
                            (*t).l.comp_type[by4 as usize] as core::ffi::c_int,
                            (*t).l.ref_0[0 as core::ffi::c_int as usize][by4 as usize]
                                as core::ffi::c_int,
                            (*ts).msac.rng,
                        );
                    }
                } else {
                    (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type = COMP_INTER_AVG
                        as core::ffi::c_int as uint8_t;
                }
            } else {
                if wedge_allowed_mask
                    & ((1 as core::ffi::c_int) << bs as core::ffi::c_uint)
                        as core::ffi::c_uint != 0
                {
                    let ctx_5: core::ffi::c_int = dav1d_wedge_ctx_lut[bs as usize]
                        as core::ffi::c_int;
                    (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type = (COMP_INTER_WEDGE
                        as core::ffi::c_int as core::ffi::c_uint)
                        .wrapping_sub(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts).cdf.m.wedge_comp[ctx_5 as usize]).as_mut_ptr(),
                            ),
                        ) as uint8_t;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as core::ffi::c_int
                        == COMP_INTER_WEDGE as core::ffi::c_int
                    {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .wedge_idx = dav1d_msac_decode_symbol_adapt_c(
                            &raw mut (*ts).msac,
                            ((*ts).cdf.m.wedge_idx[ctx_5 as usize]).as_mut_ptr(),
                            15 as size_t,
                        ) as uint8_t;
                    }
                } else {
                    (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type = COMP_INTER_SEG
                        as core::ffi::c_int as uint8_t;
                }
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .mask_sign = dav1d_msac_decode_bool_equi_c(&raw mut (*ts).msac)
                    as uint8_t;
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-seg/wedge[%d,wedge_idx=%d,sign=%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        ((*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                            as core::ffi::c_int == COMP_INTER_WEDGE as core::ffi::c_int)
                            as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .wedge_idx as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mask_sign as core::ffi::c_int,
                        (*ts).msac.rng,
                    );
                }
            }
        } else {
            (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type = COMP_INTER_NONE
                as core::ffi::c_int as uint8_t;
            if !seg.is_null() && (*seg).ref_0 as core::ffi::c_int > 0 as core::ffi::c_int
            {
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[0 as core::ffi::c_int as usize] = ((*seg).ref_0
                    as core::ffi::c_int - 1 as core::ffi::c_int) as int8_t;
            } else if !seg.is_null()
                && ((*seg).globalmv as core::ffi::c_int != 0
                    || (*seg).skip as core::ffi::c_int != 0)
            {
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .ref_0[0 as core::ffi::c_int as usize] = 0 as int8_t;
            } else {
                let ctx1_0: core::ffi::c_int = av1_get_ref_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    by4,
                    bx4,
                    have_top,
                    have_left,
                ) as core::ffi::c_int;
                if dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.ref_0[0 as core::ffi::c_int as usize][ctx1_0 as usize])
                        .as_mut_ptr(),
                ) != 0
                {
                    let ctx2_1: core::ffi::c_int = av1_get_bwd_ref_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                        have_top,
                        have_left,
                    ) as core::ffi::c_int;
                    if dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .ref_0[1 as core::ffi::c_int as usize][ctx2_1 as usize])
                            .as_mut_ptr(),
                    ) != 0
                    {
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] = 6 as int8_t;
                    } else {
                        let ctx3_0: core::ffi::c_int = av1_get_bwd_ref_1_ctx(
                            (*t).a,
                            &raw mut (*t).l,
                            by4,
                            bx4,
                            have_top,
                            have_left,
                        ) as core::ffi::c_int;
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] = (4
                            as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bool_adapt_c(
                                    &raw mut (*ts).msac,
                                    ((*ts)
                                        .cdf
                                        .m
                                        .ref_0[5 as core::ffi::c_int as usize][ctx3_0 as usize])
                                        .as_mut_ptr(),
                                ),
                            ) as int8_t;
                    }
                } else {
                    let ctx2_2: core::ffi::c_int = av1_get_fwd_ref_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        by4,
                        bx4,
                        have_top,
                        have_left,
                    ) as core::ffi::c_int;
                    if dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .ref_0[2 as core::ffi::c_int as usize][ctx2_2 as usize])
                            .as_mut_ptr(),
                    ) != 0
                    {
                        let ctx3_1: core::ffi::c_int = av1_get_fwd_ref_2_ctx(
                            (*t).a,
                            &raw mut (*t).l,
                            by4,
                            bx4,
                            have_top,
                            have_left,
                        ) as core::ffi::c_int;
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] = (2
                            as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bool_adapt_c(
                                    &raw mut (*ts).msac,
                                    ((*ts)
                                        .cdf
                                        .m
                                        .ref_0[4 as core::ffi::c_int as usize][ctx3_1 as usize])
                                        .as_mut_ptr(),
                                ),
                            ) as int8_t;
                    } else {
                        let ctx3_2: core::ffi::c_int = av1_get_fwd_ref_1_ctx(
                            (*t).a,
                            &raw mut (*t).l,
                            by4,
                            bx4,
                            have_top,
                            have_left,
                        ) as core::ffi::c_int;
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] = dav1d_msac_decode_bool_adapt_c(
                            &raw mut (*ts).msac,
                            ((*ts)
                                .cdf
                                .m
                                .ref_0[3 as core::ffi::c_int as usize][ctx3_2 as usize])
                                .as_mut_ptr(),
                        ) as int8_t;
                    }
                }
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-ref[%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        (*ts).msac.rng,
                    );
                }
            }
            (*b).c2rust_unnamed.c2rust_unnamed_0.ref_0[1 as core::ffi::c_int as usize] = -(1
                as core::ffi::c_int) as int8_t;
            let mut mvstack_2: [refmvs_candidate; 8] = [refmvs_candidate {
                mv: refmvs_mvpair {
                    mv: [mv {
                        c2rust_unnamed: C2RustUnnamed_24 { y: 0, x: 0 },
                    }; 2],
                },
                weight: 0,
            }; 8];
            let mut n_mvs_2: core::ffi::c_int = 0;
            let mut ctx_6: core::ffi::c_int = 0;
            dav1d_refmvs_find(
                &raw mut (*t).rt,
                mvstack_2.as_mut_ptr(),
                &raw mut n_mvs_2,
                &raw mut ctx_6,
                refmvs_refpair {
                    ref_0: [
                        ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                            + 1 as core::ffi::c_int) as int8_t,
                        -(1 as core::ffi::c_int) as int8_t,
                    ],
                },
                bs,
                intra_edge_flags,
                (*t).by,
                (*t).bx,
            );
            if !seg.is_null()
                && ((*seg).skip as core::ffi::c_int != 0
                    || (*seg).globalmv as core::ffi::c_int != 0)
                || dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.newmv_mode[(ctx_6 & 7 as core::ffi::c_int) as usize])
                        .as_mut_ptr(),
                ) != 0
            {
                if !seg.is_null()
                    && ((*seg).skip as core::ffi::c_int != 0
                        || (*seg).globalmv as core::ffi::c_int != 0)
                    || dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .globalmv_mode[(ctx_6 >> 3 as core::ffi::c_int
                            & 1 as core::ffi::c_int) as usize])
                            .as_mut_ptr(),
                    ) == 0
                {
                    (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode = GLOBALMV
                        as core::ffi::c_int as uint8_t;
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = get_gmv_2d(
                        &raw mut *((*(*f).frame_hdr).gmv)
                            .as_mut_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ),
                        (*t).bx,
                        (*t).by,
                        bw4,
                        bh4,
                        (*f).frame_hdr,
                    ) as mv;
                    has_subpel_filter = (imin(bw4, bh4) == 1 as core::ffi::c_int
                        || (*(*f).frame_hdr)
                            .gmv[(*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .ref_0[0 as core::ffi::c_int as usize] as usize]
                            .type_0 as core::ffi::c_uint
                            == DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                as core::ffi::c_uint) as core::ffi::c_int;
                } else {
                    has_subpel_filter = 1 as core::ffi::c_int;
                    if dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .refmv_mode[(ctx_6 >> 4 as core::ffi::c_int
                            & 15 as core::ffi::c_int) as usize])
                            .as_mut_ptr(),
                    ) != 0
                    {
                        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode = NEARMV
                            as core::ffi::c_int as uint8_t;
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEARER_DRL
                            as core::ffi::c_int as uint8_t;
                        if n_mvs_2 > 2 as core::ffi::c_int {
                            let drl_ctx_v2_1: core::ffi::c_int = get_drl_context(
                                mvstack_2.as_mut_ptr(),
                                1 as core::ffi::c_int,
                            ) as core::ffi::c_int;
                            (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .drl_idx as core::ffi::c_uint)
                                .wrapping_add(
                                    dav1d_msac_decode_bool_adapt_c(
                                        &raw mut (*ts).msac,
                                        ((*ts).cdf.m.drl_bit[drl_ctx_v2_1 as usize]).as_mut_ptr(),
                                    ),
                                ) as uint8_t as uint8_t;
                            if (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx
                                as core::ffi::c_int == NEAR_DRL as core::ffi::c_int
                                && n_mvs_2 > 3 as core::ffi::c_int
                            {
                                let drl_ctx_v3_0: core::ffi::c_int = get_drl_context(
                                    mvstack_2.as_mut_ptr(),
                                    2 as core::ffi::c_int,
                                ) as core::ffi::c_int;
                                (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                                    .c2rust_unnamed
                                    .c2rust_unnamed_0
                                    .drl_idx as core::ffi::c_uint)
                                    .wrapping_add(
                                        dav1d_msac_decode_bool_adapt_c(
                                            &raw mut (*ts).msac,
                                            ((*ts).cdf.m.drl_bit[drl_ctx_v3_0 as usize]).as_mut_ptr(),
                                        ),
                                    ) as uint8_t as uint8_t;
                            }
                        }
                    } else {
                        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode = NEARESTMV
                            as core::ffi::c_int as uint8_t;
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEAREST_DRL
                            as core::ffi::c_int as uint8_t;
                    }
                    if !((*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                        >= NEAREST_DRL as core::ffi::c_int
                        && (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx
                            as core::ffi::c_int <= NEARISH_DRL as core::ffi::c_int)
                        as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1704 as core::ffi::c_int,
                            b"b->drl_idx >= NEAREST_DRL && b->drl_idx <= NEARISH_DRL\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = mvstack_2[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as usize]
                        .mv
                        .mv[0 as core::ffi::c_int as usize] as mv;
                    if ((*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int)
                        < NEAR_DRL as core::ffi::c_int
                    {
                        fix_mv_precision(
                            (*f).frame_hdr,
                            &raw mut *((*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize),
                        );
                    }
                }
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-intermode[%d,drl=%d,mv=y:%d,x:%d,n_mvs=%d]: r=%d\n\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                            as core::ffi::c_int,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize]
                            .c2rust_unnamed
                            .y as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize]
                            .c2rust_unnamed
                            .x as core::ffi::c_int,
                        n_mvs_2,
                        (*ts).msac.rng,
                    );
                }
            } else {
                has_subpel_filter = 1 as core::ffi::c_int;
                (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode = NEWMV
                    as core::ffi::c_int as uint8_t;
                (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEAREST_DRL
                    as core::ffi::c_int as uint8_t;
                if n_mvs_2 > 1 as core::ffi::c_int {
                    let drl_ctx_v1_0: core::ffi::c_int = get_drl_context(
                        mvstack_2.as_mut_ptr(),
                        0 as core::ffi::c_int,
                    ) as core::ffi::c_int;
                    (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .drl_idx as core::ffi::c_uint)
                        .wrapping_add(
                            dav1d_msac_decode_bool_adapt_c(
                                &raw mut (*ts).msac,
                                ((*ts).cdf.m.drl_bit[drl_ctx_v1_0 as usize]).as_mut_ptr(),
                            ),
                        ) as uint8_t as uint8_t;
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                        == NEARER_DRL as core::ffi::c_int
                        && n_mvs_2 > 2 as core::ffi::c_int
                    {
                        let drl_ctx_v2_2: core::ffi::c_int = get_drl_context(
                            mvstack_2.as_mut_ptr(),
                            1 as core::ffi::c_int,
                        ) as core::ffi::c_int;
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx = ((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as core::ffi::c_uint)
                            .wrapping_add(
                                dav1d_msac_decode_bool_adapt_c(
                                    &raw mut (*ts).msac,
                                    ((*ts).cdf.m.drl_bit[drl_ctx_v2_2 as usize]).as_mut_ptr(),
                                ),
                            ) as uint8_t as uint8_t;
                    }
                }
                if !((*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                    >= NEAREST_DRL as core::ffi::c_int
                    && (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int
                        <= NEARISH_DRL as core::ffi::c_int) as core::ffi::c_int
                    as core::ffi::c_long != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 9],
                            [core::ffi::c_char; 9],
                        >(*b"decode_b\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        1728 as core::ffi::c_int,
                        b"b->drl_idx >= NEAREST_DRL && b->drl_idx <= NEARISH_DRL\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
                if n_mvs_2 > 1 as core::ffi::c_int {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = mvstack_2[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .drl_idx as usize]
                        .mv
                        .mv[0 as core::ffi::c_int as usize] as mv;
                } else {
                    if ((*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx != 0)
                        as core::ffi::c_int as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1732 as core::ffi::c_int,
                            b"!b->drl_idx\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize] = mvstack_2[0
                            as core::ffi::c_int as usize]
                        .mv
                        .mv[0 as core::ffi::c_int as usize] as mv;
                    fix_mv_precision(
                        (*f).frame_hdr,
                        &raw mut *((*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                    );
                }
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-intermode[%d,drl=%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                            as core::ffi::c_int,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.drl_idx as core::ffi::c_int,
                        (*ts).msac.rng,
                    );
                }
                let mv_prec_1: core::ffi::c_int = (*(*f).frame_hdr).hp
                    as core::ffi::c_int
                    - (*(*f).frame_hdr).force_integer_mv as core::ffi::c_int;
                read_mv_residual(
                    ts,
                    &raw mut *((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize),
                    mv_prec_1,
                );
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-residualmv[mv=y:%d,x:%d]: r=%d\n\0" as *const u8
                            as *const core::ffi::c_char,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize]
                            .c2rust_unnamed
                            .y as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize]
                            .c2rust_unnamed
                            .x as core::ffi::c_int,
                        (*ts).msac.rng,
                    );
                }
            }
            let ii_sz_grp: core::ffi::c_int = dav1d_ymode_size_context[bs as usize]
                as core::ffi::c_int;
            if (*(*f).seq_hdr).inter_intra as core::ffi::c_int != 0
                && interintra_allowed_mask
                    & ((1 as core::ffi::c_int) << bs as core::ffi::c_uint)
                        as core::ffi::c_uint != 0
                && dav1d_msac_decode_bool_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.interintra[ii_sz_grp as usize]).as_mut_ptr(),
                ) != 0
            {
                (*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.interintra_mode[ii_sz_grp as usize]).as_mut_ptr(),
                    (N_INTER_INTRA_PRED_MODES as core::ffi::c_int
                        - 1 as core::ffi::c_int) as size_t,
                ) as uint8_t;
                let wedge_ctx: core::ffi::c_int = dav1d_wedge_ctx_lut[bs as usize]
                    as core::ffi::c_int;
                (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type = (INTER_INTRA_BLEND
                    as core::ffi::c_int as core::ffi::c_uint)
                    .wrapping_add(
                        dav1d_msac_decode_bool_adapt_c(
                            &raw mut (*ts).msac,
                            ((*ts).cdf.m.interintra_wedge[wedge_ctx as usize])
                                .as_mut_ptr(),
                        ),
                    ) as uint8_t;
                if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type
                    as core::ffi::c_int == INTER_INTRA_WEDGE as core::ffi::c_int
                {
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx = dav1d_msac_decode_symbol_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts).cdf.m.wedge_idx[wedge_ctx as usize]).as_mut_ptr(),
                        15 as size_t,
                    ) as uint8_t;
                }
            } else {
                (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type = INTER_INTRA_NONE
                    as core::ffi::c_int as uint8_t;
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
                && (*(*f).seq_hdr).inter_intra as core::ffi::c_int != 0
                && interintra_allowed_mask
                    & ((1 as core::ffi::c_int) << bs as core::ffi::c_uint)
                        as core::ffi::c_uint != 0
            {
                printf(
                    b"Post-interintra[t=%d,m=%d,w=%d]: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type
                        as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .interintra_mode as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx as core::ffi::c_int,
                    (*ts).msac.rng,
                );
            }
            if (*(*f).frame_hdr).switchable_motion_mode as core::ffi::c_int != 0
                && (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type
                    as core::ffi::c_int == INTER_INTRA_NONE as core::ffi::c_int
                && imin(bw4, bh4) >= 2 as core::ffi::c_int
                && !((*(*f).frame_hdr).force_integer_mv == 0
                    && (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_int == GLOBALMV as core::ffi::c_int
                    && (*(*f).frame_hdr)
                        .gmv[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as usize]
                        .type_0 as core::ffi::c_uint
                        > DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                            as core::ffi::c_uint)
                && (have_left != 0
                    && findoddzero(
                        &raw mut *((*t).l.intra)
                            .as_mut_ptr()
                            .offset((by4 + 1 as core::ffi::c_int) as isize),
                        h4 >> 1 as core::ffi::c_int,
                    ) != 0
                    || have_top != 0
                        && findoddzero(
                            &raw mut *((*(*t).a).intra)
                                .as_mut_ptr()
                                .offset((bx4 + 1 as core::ffi::c_int) as isize),
                            w4 >> 1 as core::ffi::c_int,
                        ) != 0)
            {
                let mut mask: [uint64_t; 2] = [
                    0 as core::ffi::c_int as uint64_t,
                    0 as core::ffi::c_int as uint64_t,
                ];
                find_matching_ref(
                    t,
                    intra_edge_flags,
                    bw4,
                    bh4,
                    w4,
                    h4,
                    have_left,
                    have_top,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                    mask.as_mut_ptr(),
                );
                let allow_warp: core::ffi::c_int = ((*f)
                    .svc[(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize]
                        as usize][0 as core::ffi::c_int as usize]
                    .scale == 0 && (*(*f).frame_hdr).force_integer_mv == 0
                    && (*(*f).frame_hdr).warp_motion as core::ffi::c_int != 0
                    && mask[0 as core::ffi::c_int as usize]
                        | mask[1 as core::ffi::c_int as usize] != 0) as core::ffi::c_int;
                (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode = (if allow_warp != 0 {
                    dav1d_msac_decode_symbol_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts).cdf.m.motion_mode[bs as usize]).as_mut_ptr(),
                        2 as size_t,
                    )
                } else {
                    dav1d_msac_decode_bool_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts).cdf.m.obmc[bs as usize]).as_mut_ptr(),
                    )
                }) as uint8_t;
                if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as core::ffi::c_int
                    == MM_WARP as core::ffi::c_int
                {
                    has_subpel_filter = 0 as core::ffi::c_int;
                    derive_warpmv(
                        t,
                        bw4,
                        bh4,
                        mask.as_mut_ptr() as *const uint64_t,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize],
                        &raw mut (*t).warpmv,
                    );
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"[ %c%x %c%x %c%x\n  %c%x %c%x %c%x ]\nalpha=%c%x, beta=%c%x, gamma=%c%x, delta=%c%x, mv=y:%d,x:%d\n\0"
                                as *const u8 as *const core::ffi::c_char,
                            if (*t).warpmv.matrix[0 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[0 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[1 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[1 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[2 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[2 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[3 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[3 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[4 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[4 as core::ffi::c_int as usize]),
                            if (*t).warpmv.matrix[5 as core::ffi::c_int as usize]
                                < 0 as int32_t
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.matrix[5 as core::ffi::c_int as usize]),
                            if ((*t).warpmv.u.p.alpha as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.alpha as core::ffi::c_int),
                            if ((*t).warpmv.u.p.beta as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.beta as core::ffi::c_int),
                            if ((*t).warpmv.u.p.gamma as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.gamma as core::ffi::c_int),
                            if ((*t).warpmv.u.p.delta as core::ffi::c_int)
                                < 0 as core::ffi::c_int
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            abs((*t).warpmv.u.p.delta as core::ffi::c_int),
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0 as core::ffi::c_int as usize]
                                .c2rust_unnamed
                                .y as core::ffi::c_int,
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0 as core::ffi::c_int as usize]
                                .c2rust_unnamed
                                .x as core::ffi::c_int,
                        );
                    }
                    if (*t).frame_thread.pass != 0 {
                        if (*t).warpmv.type_0 as core::ffi::c_uint
                            == DAV1D_WM_TYPE_AFFINE as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[0 as core::ffi::c_int as usize] = ((*t)
                                .warpmv
                                .matrix[2 as core::ffi::c_int as usize]
                                - 0x10000 as int32_t) as int16_t;
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[1 as core::ffi::c_int as usize] = (*t)
                                .warpmv
                                .matrix[3 as core::ffi::c_int as usize] as int16_t;
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[2 as core::ffi::c_int as usize] = (*t)
                                .warpmv
                                .matrix[4 as core::ffi::c_int as usize] as int16_t;
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[3 as core::ffi::c_int as usize] = ((*t)
                                .warpmv
                                .matrix[5 as core::ffi::c_int as usize]
                                - 0x10000 as int32_t) as int16_t;
                        } else {
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[0 as core::ffi::c_int as usize] = INT16_MIN
                                as int16_t;
                        }
                    }
                }
                if 0 as core::ffi::c_int != 0
                    && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                        == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                    && (*t).by < 4 as core::ffi::c_int
                    && (*t).bx >= 8 as core::ffi::c_int
                    && (*t).bx < 12 as core::ffi::c_int
                {
                    printf(
                        b"Post-motionmode[%d]: r=%d [mask: 0x%llx/0x%llx]\n\0"
                            as *const u8 as *const core::ffi::c_char,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                            as core::ffi::c_int,
                        (*ts).msac.rng,
                        mask[0 as core::ffi::c_int as usize],
                        mask[1 as core::ffi::c_int as usize],
                    );
                }
            } else {
                (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode = MM_TRANSLATION
                    as core::ffi::c_int as uint8_t;
            }
        }
        let mut filter_0: [Dav1dFilterMode; 2] = [DAV1D_FILTER_8TAP_REGULAR; 2];
        if (*(*f).frame_hdr).subpel_filter_mode as core::ffi::c_uint
            == DAV1D_FILTER_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
        {
            if has_subpel_filter != 0 {
                let comp: core::ffi::c_int = ((*b)
                    .c2rust_unnamed
                    .c2rust_unnamed_0
                    .comp_type as core::ffi::c_int
                    != COMP_INTER_NONE as core::ffi::c_int) as core::ffi::c_int;
                let ctx1_1: core::ffi::c_int = get_filter_ctx(
                    (*t).a,
                    &raw mut (*t).l,
                    comp,
                    0 as core::ffi::c_int,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                    by4,
                    bx4,
                ) as core::ffi::c_int;
                filter_0[0 as core::ffi::c_int as usize] = dav1d_msac_decode_symbol_adapt_c(
                    &raw mut (*ts).msac,
                    ((*ts).cdf.m.filter[0 as core::ffi::c_int as usize][ctx1_1 as usize])
                        .as_mut_ptr(),
                    (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                        - 1 as core::ffi::c_int) as size_t,
                ) as Dav1dFilterMode;
                if (*(*f).seq_hdr).dual_filter != 0 {
                    let ctx2_3: core::ffi::c_int = get_filter_ctx(
                        (*t).a,
                        &raw mut (*t).l,
                        comp,
                        1 as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        by4,
                        bx4,
                    ) as core::ffi::c_int;
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-subpel_filter1[%d,ctx=%d]: r=%d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            filter_0[0 as core::ffi::c_int as usize]
                                as core::ffi::c_uint,
                            ctx1_1,
                            (*ts).msac.rng,
                        );
                    }
                    filter_0[1 as core::ffi::c_int as usize] = dav1d_msac_decode_symbol_adapt_c(
                        &raw mut (*ts).msac,
                        ((*ts)
                            .cdf
                            .m
                            .filter[1 as core::ffi::c_int as usize][ctx2_3 as usize])
                            .as_mut_ptr(),
                        (DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int
                            - 1 as core::ffi::c_int) as size_t,
                    ) as Dav1dFilterMode;
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-subpel_filter2[%d,ctx=%d]: r=%d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            filter_0[1 as core::ffi::c_int as usize]
                                as core::ffi::c_uint,
                            ctx2_3,
                            (*ts).msac.rng,
                        );
                    }
                } else {
                    filter_0[1 as core::ffi::c_int as usize] = filter_0[0
                        as core::ffi::c_int as usize];
                    if 0 as core::ffi::c_int != 0
                        && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                            == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                        && (*t).by < 4 as core::ffi::c_int
                        && (*t).bx >= 8 as core::ffi::c_int
                        && (*t).bx < 12 as core::ffi::c_int
                    {
                        printf(
                            b"Post-subpel_filter[%d,ctx=%d]: r=%d\n\0" as *const u8
                                as *const core::ffi::c_char,
                            filter_0[0 as core::ffi::c_int as usize]
                                as core::ffi::c_uint,
                            ctx1_1,
                            (*ts).msac.rng,
                        );
                    }
                }
            } else {
                filter_0[1 as core::ffi::c_int as usize] = DAV1D_FILTER_8TAP_REGULAR;
                filter_0[0 as core::ffi::c_int as usize] = filter_0[1 as core::ffi::c_int
                    as usize];
            }
        } else {
            filter_0[1 as core::ffi::c_int as usize] = (*(*f).frame_hdr)
                .subpel_filter_mode;
            filter_0[0 as core::ffi::c_int as usize] = filter_0[1 as core::ffi::c_int
                as usize];
        }
        (*b).c2rust_unnamed.c2rust_unnamed_0.filter2d = dav1d_filter_2d[filter_0[1
            as core::ffi::c_int as usize]
            as usize][filter_0[0 as core::ffi::c_int as usize] as usize];
        read_vartx_tree(t, b, bs, bx4, by4);
        if (*t).frame_thread.pass == 1 as core::ffi::c_int {
            ((*f).bd_fn.read_coef_blocks).expect("non-null function pointer")(t, bs, b);
        } else if ((*f).bd_fn.recon_b_inter)
            .expect("non-null function pointer")(t, bs, b) != 0
        {
            return -(1 as core::ffi::c_int)
        }
        if (*(*f).frame_hdr).loopfilter.level_y[0 as core::ffi::c_int as usize]
            as core::ffi::c_int != 0
            || (*(*f).frame_hdr).loopfilter.level_y[1 as core::ffi::c_int as usize]
                as core::ffi::c_int != 0
        {
            let is_globalmv: core::ffi::c_int = ((*b)
                .c2rust_unnamed
                .c2rust_unnamed_0
                .inter_mode as core::ffi::c_int
                == (if is_comp != 0 {
                    GLOBALMV_GLOBALMV as core::ffi::c_int
                } else {
                    GLOBALMV as core::ffi::c_int
                })) as core::ffi::c_int;
            let lf_lvls: *const [[uint8_t; 2]; 8] = &*(*(*(*((*ts).lflvl)
                .offset((*b).seg_id as isize))
                .as_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .as_ptr()
                .offset(
                    (*((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                        + 1 as core::ffi::c_int) as isize,
                ))
                .as_ptr()
                .offset((is_globalmv == 0) as core::ffi::c_int as isize)
                as *const uint8_t as *const [[uint8_t; 2]; 8];
            let tx_split: [uint16_t; 2] = [
                (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 as uint16_t,
                (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1,
            ];
            let mut ytx: RectTxfmSize = (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx
                as RectTxfmSize;
            let mut uvtx: RectTxfmSize = (*b).uvtx as RectTxfmSize;
            if (*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize] != 0 {
                ytx = TX_4X4 as core::ffi::c_int as RectTxfmSize;
                uvtx = TX_4X4 as core::ffi::c_int as RectTxfmSize;
            }
            dav1d_create_lf_mask_inter(
                (*t).lf_mask,
                (*f).lf.level,
                (*f).b4_stride,
                lf_lvls,
                (*t).bx,
                (*t).by,
                (*f).w4,
                (*f).h4,
                (*b).skip as core::ffi::c_int,
                bs,
                ytx,
                tx_split.as_ptr(),
                uvtx,
                (*f).cur.p.layout,
                &raw mut *((*(*t).a).tx_lpf_y).as_mut_ptr().offset(bx4 as isize),
                &raw mut *((*t).l.tx_lpf_y).as_mut_ptr().offset(by4 as isize),
                if has_chroma != 0 {
                    &raw mut *((*(*t).a).tx_lpf_uv).as_mut_ptr().offset(cbx4 as isize)
                } else {
                    0 as *mut uint8_t
                },
                if has_chroma != 0 {
                    &raw mut *((*t).l.tx_lpf_uv).as_mut_ptr().offset(cby4 as isize)
                } else {
                    0 as *mut uint8_t
                },
            );
        }
        if is_comp != 0 {
            splat_tworef_mv((*f).c, t, bs, b, bw4, bh4);
        } else {
            splat_oneref_mv((*f).c, t, bs, b, bw4, bh4);
        }
        let mut edge_3: *mut BlockContext = (*t).a;
        let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut off_3: core::ffi::c_int = bx4;
        while i_4 < 2 as core::ffi::c_int {
            match *b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                as core::ffi::c_int
            {
                0 => {
                    (*(&raw mut *((*edge_3).seg_pred).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (seg_pred * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_3).skip_mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).skip_mode as core::ffi::c_int
                        * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_3).intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_3).skip).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).skip as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_3).pal_sz).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_4 as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0 as core::ffi::c_int * 0x1 as core::ffi::c_int)
                        as uint8_t;
                    (*(&raw mut *((*edge_3).tx_intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut int8_t as *mut alias8))
                        .u8_0 = (*b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                        as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *((*edge_3).comp_type).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                        as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (filter_0[0 as core::ffi::c_int as usize]
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1 as core::ffi::c_uint) as uint8_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (filter_0[1 as core::ffi::c_int as usize]
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1 as core::ffi::c_uint) as uint8_t;
                    (*(&raw mut *((*edge_3).mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias8))
                        .u8_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        * 0x1 as core::ffi::c_int) as uint8_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias8))
                        .u8_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as uint8_t
                        as core::ffi::c_int * 0x1 as core::ffi::c_int) as uint8_t;
                }
                1 => {
                    (*(&raw mut *((*edge_3).seg_pred).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (seg_pred * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_3).skip_mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).skip_mode as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_3).intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_3).skip).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).skip as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_3).pal_sz).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_4 as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0 as core::ffi::c_int * 0x101 as core::ffi::c_int)
                        as uint16_t;
                    (*(&raw mut *((*edge_3).tx_intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut int8_t as *mut alias16))
                        .u16_0 = (*b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                        as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *((*edge_3).comp_type).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                        as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (filter_0[0 as core::ffi::c_int as usize]
                        as core::ffi::c_uint)
                        .wrapping_mul(0x101 as core::ffi::c_uint) as uint16_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (filter_0[1 as core::ffi::c_int as usize]
                        as core::ffi::c_uint)
                        .wrapping_mul(0x101 as core::ffi::c_uint) as uint16_t;
                    (*(&raw mut *((*edge_3).mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias16))
                        .u16_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias16))
                        .u16_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as uint8_t
                        as core::ffi::c_int * 0x101 as core::ffi::c_int) as uint16_t;
                }
                2 => {
                    (*(&raw mut *((*edge_3).seg_pred).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (seg_pred as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).skip_mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).skip_mode as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).skip).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).skip as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).pal_sz).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_4 as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0 as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).tx_intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut int8_t as *mut alias32))
                        .u32_0 = (*b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).comp_type).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (filter_0[0 as core::ffi::c_int as usize]
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (filter_0[1 as core::ffi::c_int as usize]
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *((*edge_3).mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias32))
                        .u32_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias32))
                        .u32_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as uint8_t
                        as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                }
                3 => {
                    (*(&raw mut *((*edge_3).seg_pred).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (seg_pred as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).skip_mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).skip_mode as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).skip).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).skip as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).pal_sz).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_4 as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0 as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).tx_intra).as_mut_ptr().offset(off_3 as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = (*b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).comp_type).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (filter_0[0 as core::ffi::c_int as usize]
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*edge_3).filter)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (filter_0[1 as core::ffi::c_int as usize]
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *((*edge_3).mode).as_mut_ptr().offset(off_3 as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias64))
                        .u64_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    (*(&raw mut *(*((*edge_3).ref_0)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(off_3 as isize) as *mut int8_t as *mut alias64))
                        .u64_0 = ((*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[1 as core::ffi::c_int as usize] as uint8_t
                        as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                }
                4 => {
                    memset(
                        &raw mut *((*edge_3).seg_pred).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        seg_pred,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).skip_mode).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip_mode as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).intra).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).skip).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).pal_sz).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_4 as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).tx_intra).as_mut_ptr().offset(off_3 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        *b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                            as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).comp_type).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                            as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        filter_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        filter_0[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).mode).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                            as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).ref_0)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut int8_t
                            as *mut core::ffi::c_void,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        16 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).ref_0)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut int8_t
                            as *mut core::ffi::c_void,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[1 as core::ffi::c_int as usize] as uint8_t
                            as core::ffi::c_int,
                        16 as size_t,
                    );
                }
                5 => {
                    memset(
                        &raw mut *((*edge_3).seg_pred).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        seg_pred,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).skip_mode).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip_mode as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).intra).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).skip).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).skip as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).pal_sz).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*t).pal_sz_uv).as_mut_ptr().offset(i_4 as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).tx_intra).as_mut_ptr().offset(off_3 as isize)
                            as *mut int8_t as *mut core::ffi::c_void,
                        *b_dim.offset((2 as core::ffi::c_int + i_4) as isize)
                            as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).comp_type).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type
                            as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).filter)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        filter_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).filter)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        filter_0[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *((*edge_3).mode).as_mut_ptr().offset(off_3 as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                            as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).ref_0)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut int8_t
                            as *mut core::ffi::c_void,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                        32 as size_t,
                    );
                    memset(
                        &raw mut *(*((*edge_3).ref_0)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(off_3 as isize) as *mut int8_t
                            as *mut core::ffi::c_void,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[1 as core::ffi::c_int as usize] as uint8_t
                            as core::ffi::c_int,
                        32 as size_t,
                    );
                }
                _ => {
                    if (0 as core::ffi::c_int == 0) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1926 as core::ffi::c_int,
                            b"0\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                }
            }
            i_4 += 1;
            off_3 = by4;
            edge_3 = &raw mut (*t).l;
        }
        if has_chroma != 0 {
            (dav1d_memset_pow2[ulog2(cbw4 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*(*t).a).uvmode).as_mut_ptr().offset(cbx4 as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
                DC_PRED as core::ffi::c_int,
            );
            (dav1d_memset_pow2[ulog2(cbh4 as core::ffi::c_uint) as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *((*t).l.uvmode).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                    as *mut core::ffi::c_void,
                DC_PRED as core::ffi::c_int,
            );
        }
    }
    if (*(*f).frame_hdr).segmentation.enabled as core::ffi::c_int != 0
        && (*(*f).frame_hdr).segmentation.update_map as core::ffi::c_int != 0
    {
        let mut seg_ptr: *mut uint8_t = &raw mut *((*f).cur_segmap)
            .offset(
                ((*t).by as ptrdiff_t * (*f).b4_stride + (*t).bx as ptrdiff_t) as isize,
            ) as *mut uint8_t;
        match *b_dim.offset(2 as core::ffi::c_int as isize) as core::ffi::c_int {
            0 => {
                let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_1 < bh4 {
                    (*(&raw mut *seg_ptr.offset(0 as core::ffi::c_int as isize)
                        as *mut uint8_t as *mut alias8))
                        .u8_0 = ((*b).seg_id as core::ffi::c_int
                        * 0x1 as core::ffi::c_int) as uint8_t;
                    seg_ptr = seg_ptr.offset((*f).b4_stride as isize);
                    y_1 += 1;
                }
            }
            1 => {
                let mut y_2: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_2 < bh4 {
                    (*(&raw mut *seg_ptr.offset(0 as core::ffi::c_int as isize)
                        as *mut uint8_t as *mut alias16))
                        .u16_0 = ((*b).seg_id as core::ffi::c_int
                        * 0x101 as core::ffi::c_int) as uint16_t;
                    seg_ptr = seg_ptr.offset((*f).b4_stride as isize);
                    y_2 += 1;
                }
            }
            2 => {
                let mut y_3: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_3 < bh4 {
                    (*(&raw mut *seg_ptr.offset(0 as core::ffi::c_int as isize)
                        as *mut uint8_t as *mut alias32))
                        .u32_0 = ((*b).seg_id as core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                    seg_ptr = seg_ptr.offset((*f).b4_stride as isize);
                    y_3 += 1;
                }
            }
            3 => {
                let mut y_4: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_4 < bh4 {
                    (*(&raw mut *seg_ptr.offset(0 as core::ffi::c_int as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = ((*b).seg_id as core::ffi::c_ulonglong)
                        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                        as uint64_t;
                    seg_ptr = seg_ptr.offset((*f).b4_stride as isize);
                    y_4 += 1;
                }
            }
            4 => {
                let mut y_5: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_5 < bh4 {
                    memset(
                        &raw mut *seg_ptr.offset(0 as core::ffi::c_int as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).seg_id as core::ffi::c_int,
                        16 as size_t,
                    );
                    seg_ptr = seg_ptr.offset((*f).b4_stride as isize);
                    y_5 += 1;
                }
            }
            5 => {
                let mut y_6: core::ffi::c_int = 0 as core::ffi::c_int;
                while y_6 < bh4 {
                    memset(
                        &raw mut *seg_ptr.offset(0 as core::ffi::c_int as isize)
                            as *mut uint8_t as *mut core::ffi::c_void,
                        (*b).seg_id as core::ffi::c_int,
                        32 as size_t,
                    );
                    seg_ptr = seg_ptr.offset((*f).b4_stride as isize);
                    y_6 += 1;
                }
            }
            _ => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 9],
                            [core::ffi::c_char; 9],
                        >(*b"decode_b\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        1945 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
    }
    if (*b).skip == 0 {
        let mut noskip_mask: *mut [uint16_t; 2] = &raw mut *((*(*t).lf_mask).noskip_mask)
            .as_mut_ptr()
            .offset((by4 >> 1 as core::ffi::c_int) as isize) as *mut [uint16_t; 2];
        let mask_0: core::ffi::c_uint = !(0 as core::ffi::c_uint)
            >> 32 as core::ffi::c_int - bw4 << (bx4 & 15 as core::ffi::c_int);
        let bx_idx: core::ffi::c_int = (bx4 & 16 as core::ffi::c_int)
            >> 4 as core::ffi::c_int;
        let mut y_7: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_7 < bh4 {
            (*noskip_mask)[bx_idx as usize] = ((*noskip_mask)[bx_idx as usize]
                as core::ffi::c_uint | mask_0) as uint16_t;
            if bw4 == 32 as core::ffi::c_int {
                (*noskip_mask)[1 as core::ffi::c_int as usize] = ((*noskip_mask)[1
                    as core::ffi::c_int as usize] as core::ffi::c_uint | mask_0)
                    as uint16_t;
            }
            y_7 += 2 as core::ffi::c_int;
            noskip_mask = noskip_mask.offset(1);
        }
    }
    if (*t).frame_thread.pass == 1 as core::ffi::c_int && (*b).intra == 0
        && (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
            != 0
    {
        let sby_0: core::ffi::c_int = (*t).by - (*ts).tiling.row_start >> (*f).sb_shift;
        let lowest_px: *mut [core::ffi::c_int; 2] = (*((*ts).lowest_pixel)
            .offset(sby_0 as isize))
            .as_mut_ptr() as *mut [core::ffi::c_int; 2];
        if (*b).c2rust_unnamed.c2rust_unnamed_0.comp_type as core::ffi::c_int
            == COMP_INTER_NONE as core::ffi::c_int
        {
            if imin(bw4, bh4) > 1 as core::ffi::c_int
                && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as core::ffi::c_int
                    == GLOBALMV as core::ffi::c_int
                    && (*f)
                        .gmv_warp_allowed[(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[0 as core::ffi::c_int as usize] as usize]
                        as core::ffi::c_int != 0
                    || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                        as core::ffi::c_int == MM_WARP as core::ffi::c_int
                        && (*t).warpmv.type_0 as core::ffi::c_uint
                            > DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                as core::ffi::c_uint)
            {
                affine_lowest_px_luma(
                    t,
                    &raw mut *(*lowest_px
                        .offset(
                            *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as isize,
                        ))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize),
                    b_dim,
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                        as core::ffi::c_int == MM_WARP as core::ffi::c_int
                    {
                        &raw mut (*t).warpmv
                    } else {
                        &raw mut *((*(*f).frame_hdr).gmv)
                            .as_mut_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            )
                    },
                );
            } else {
                mc_lowest_px(
                    &raw mut *(*lowest_px
                        .offset(
                            *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as isize,
                        ))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize),
                    (*t).by,
                    bh4,
                    (*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0 as core::ffi::c_int as usize]
                        .c2rust_unnamed
                        .y as core::ffi::c_int,
                    0 as core::ffi::c_int,
                    &*(*((*f).svc)
                        .as_ptr()
                        .offset(
                            *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as isize,
                        ))
                        .as_ptr()
                        .offset(1 as core::ffi::c_int as isize),
                );
                if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as core::ffi::c_int
                    == MM_OBMC as core::ffi::c_int
                {
                    obmc_lowest_px(
                        t,
                        lowest_px,
                        0 as core::ffi::c_int,
                        b_dim,
                        bx4,
                        by4,
                        w4,
                        h4,
                    );
                }
            }
            if has_chroma != 0 {
                let mut is_sub8x8: core::ffi::c_int = (bw4 == ss_hor || bh4 == ss_ver)
                    as core::ffi::c_int;
                let mut r_1: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
                if is_sub8x8 != 0 {
                    if !(ss_hor == 1 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1987 as core::ffi::c_int,
                            b"ss_hor == 1\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    r_1 = &raw mut *((*t).rt.r)
                        .as_mut_ptr()
                        .offset(
                            (((*t).by & 31 as core::ffi::c_int) + 5 as core::ffi::c_int)
                                as isize,
                        ) as *mut *mut refmvs_block;
                    if bw4 == 1 as core::ffi::c_int {
                        is_sub8x8
                            &= ((*(*r_1.offset(0 as core::ffi::c_int as isize))
                                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                                .ref_0
                                .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                > 0 as core::ffi::c_int) as core::ffi::c_int;
                    }
                    if bh4 == ss_ver {
                        is_sub8x8
                            &= ((*(*r_1.offset(-(1 as core::ffi::c_int) as isize))
                                .offset((*t).bx as isize))
                                .ref_0
                                .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                > 0 as core::ffi::c_int) as core::ffi::c_int;
                    }
                    if bw4 == 1 as core::ffi::c_int && bh4 == ss_ver {
                        is_sub8x8
                            &= ((*(*r_1.offset(-(1 as core::ffi::c_int) as isize))
                                .offset(((*t).bx - 1 as core::ffi::c_int) as isize))
                                .ref_0
                                .ref_0[0 as core::ffi::c_int as usize] as core::ffi::c_int
                                > 0 as core::ffi::c_int) as core::ffi::c_int;
                    }
                }
                if is_sub8x8 != 0 {
                    if !(ss_hor == 1 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 9],
                                [core::ffi::c_char; 9],
                            >(*b"decode_b\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            1997 as core::ffi::c_int,
                            b"ss_hor == 1\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    if bw4 == 1 as core::ffi::c_int && bh4 == ss_ver {
                        let rr_1: *const refmvs_block = &raw mut *(*r_1
                            .offset(-(1 as core::ffi::c_int) as isize))
                            .offset(((*t).bx - 1 as core::ffi::c_int) as isize)
                            as *mut refmvs_block;
                        mc_lowest_px(
                            &raw mut *(*lowest_px
                                .offset(
                                    (*((*rr_1).ref_0.ref_0)
                                        .as_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                            (*t).by - 1 as core::ffi::c_int,
                            bh4,
                            (*rr_1)
                                .mv
                                .mv[0 as core::ffi::c_int as usize]
                                .c2rust_unnamed
                                .y as core::ffi::c_int,
                            ss_ver,
                            &*(*((*f).svc)
                                .as_ptr()
                                .offset(
                                    (*((*rr_1).ref_0.ref_0)
                                        .as_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ))
                                .as_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                        );
                    }
                    if bw4 == 1 as core::ffi::c_int {
                        let rr_2: *const refmvs_block = &raw mut *(*r_1
                            .offset(0 as core::ffi::c_int as isize))
                            .offset(((*t).bx - 1 as core::ffi::c_int) as isize)
                            as *mut refmvs_block;
                        mc_lowest_px(
                            &raw mut *(*lowest_px
                                .offset(
                                    (*((*rr_2).ref_0.ref_0)
                                        .as_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                            (*t).by,
                            bh4,
                            (*rr_2)
                                .mv
                                .mv[0 as core::ffi::c_int as usize]
                                .c2rust_unnamed
                                .y as core::ffi::c_int,
                            ss_ver,
                            &*(*((*f).svc)
                                .as_ptr()
                                .offset(
                                    (*((*rr_2).ref_0.ref_0)
                                        .as_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ))
                                .as_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                        );
                    }
                    if bh4 == ss_ver {
                        let rr_3: *const refmvs_block = &raw mut *(*r_1
                            .offset(-(1 as core::ffi::c_int) as isize))
                            .offset((*t).bx as isize) as *mut refmvs_block;
                        mc_lowest_px(
                            &raw mut *(*lowest_px
                                .offset(
                                    (*((*rr_3).ref_0.ref_0)
                                        .as_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                            (*t).by - 1 as core::ffi::c_int,
                            bh4,
                            (*rr_3)
                                .mv
                                .mv[0 as core::ffi::c_int as usize]
                                .c2rust_unnamed
                                .y as core::ffi::c_int,
                            ss_ver,
                            &*(*((*f).svc)
                                .as_ptr()
                                .offset(
                                    (*((*rr_3).ref_0.ref_0)
                                        .as_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                ))
                                .as_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                        );
                    }
                    mc_lowest_px(
                        &raw mut *(*lowest_px
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ))
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                        (*t).by,
                        bh4,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize]
                            .c2rust_unnamed
                            .y as core::ffi::c_int,
                        ss_ver,
                        &*(*((*f).svc)
                            .as_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ))
                            .as_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                    );
                } else if imin(cbw4, cbh4) > 1 as core::ffi::c_int
                    && ((*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_int == GLOBALMV as core::ffi::c_int
                        && (*f)
                            .gmv_warp_allowed[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[0 as core::ffi::c_int as usize] as usize]
                            as core::ffi::c_int != 0
                        || (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                            as core::ffi::c_int == MM_WARP as core::ffi::c_int
                            && (*t).warpmv.type_0 as core::ffi::c_uint
                                > DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                    as core::ffi::c_uint)
                {
                    affine_lowest_px_chroma(
                        t,
                        &raw mut *(*lowest_px
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ))
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                        b_dim,
                        if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                            as core::ffi::c_int == MM_WARP as core::ffi::c_int
                        {
                            &raw mut (*t).warpmv
                        } else {
                            &raw mut *((*(*f).frame_hdr).gmv)
                                .as_mut_ptr()
                                .offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                        .as_mut_ptr()
                                        .offset(0 as core::ffi::c_int as isize) as isize,
                                )
                        },
                    );
                } else {
                    mc_lowest_px(
                        &raw mut *(*lowest_px
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ))
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                        (*t).by & !ss_ver,
                        bh4 << (bh4 == ss_ver) as core::ffi::c_int,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0 as core::ffi::c_int as usize]
                            .c2rust_unnamed
                            .y as core::ffi::c_int,
                        ss_ver,
                        &*(*((*f).svc)
                            .as_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as isize) as isize,
                            ))
                            .as_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                    );
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode
                        as core::ffi::c_int == MM_OBMC as core::ffi::c_int
                    {
                        obmc_lowest_px(
                            t,
                            lowest_px,
                            1 as core::ffi::c_int,
                            b_dim,
                            bx4,
                            by4,
                            w4,
                            h4,
                        );
                    }
                }
            }
        } else {
            let mut i_5: core::ffi::c_int = 0 as core::ffi::c_int;
            while i_5 < 2 as core::ffi::c_int {
                if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as core::ffi::c_int
                    == GLOBALMV_GLOBALMV as core::ffi::c_int
                    && (*f)
                        .gmv_warp_allowed[(*b)
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .ref_0[i_5 as usize] as usize] as core::ffi::c_int != 0
                {
                    affine_lowest_px_luma(
                        t,
                        &raw mut *(*lowest_px
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(i_5 as isize) as isize,
                            ))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        b_dim,
                        &raw mut *((*(*f).frame_hdr).gmv)
                            .as_mut_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(i_5 as isize) as isize,
                            ),
                    );
                } else {
                    mc_lowest_px(
                        &raw mut *(*lowest_px
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(i_5 as isize) as isize,
                            ))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize),
                        (*t).by,
                        bh4,
                        (*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[i_5 as usize]
                            .c2rust_unnamed
                            .y as core::ffi::c_int,
                        0 as core::ffi::c_int,
                        &*(*((*f).svc)
                            .as_ptr()
                            .offset(
                                *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                    .as_mut_ptr()
                                    .offset(i_5 as isize) as isize,
                            ))
                            .as_ptr()
                            .offset(1 as core::ffi::c_int as isize),
                    );
                }
                i_5 += 1;
            }
            if has_chroma != 0 {
                let mut i_6: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_6 < 2 as core::ffi::c_int {
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode
                        as core::ffi::c_int == GLOBALMV_GLOBALMV as core::ffi::c_int
                        && imin(cbw4, cbh4) > 1 as core::ffi::c_int
                        && (*f)
                            .gmv_warp_allowed[(*b)
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .ref_0[i_6 as usize] as usize] as core::ffi::c_int != 0
                    {
                        affine_lowest_px_chroma(
                            t,
                            &raw mut *(*lowest_px
                                .offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                        .as_mut_ptr()
                                        .offset(i_6 as isize) as isize,
                                ))
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                            b_dim,
                            &raw mut *((*(*f).frame_hdr).gmv)
                                .as_mut_ptr()
                                .offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                        .as_mut_ptr()
                                        .offset(i_6 as isize) as isize,
                                ),
                        );
                    } else {
                        mc_lowest_px(
                            &raw mut *(*lowest_px
                                .offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                        .as_mut_ptr()
                                        .offset(i_6 as isize) as isize,
                                ))
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                            (*t).by,
                            bh4,
                            (*b)
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[i_6 as usize]
                                .c2rust_unnamed
                                .y as core::ffi::c_int,
                            ss_ver,
                            &*(*((*f).svc)
                                .as_ptr()
                                .offset(
                                    *((*b).c2rust_unnamed.c2rust_unnamed_0.ref_0)
                                        .as_mut_ptr()
                                        .offset(i_6 as isize) as isize,
                                ))
                                .as_ptr()
                                .offset(1 as core::ffi::c_int as isize),
                        );
                    }
                    i_6 += 1;
                }
            }
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn decode_sb(
    t: *mut Dav1dTaskContext,
    bl: BlockLevel,
    node: *const EdgeNode,
) -> core::ffi::c_int {
    let f: *const Dav1dFrameContext = (*t).f;
    let ts: *mut Dav1dTileState = (*t).ts;
    let hsz: core::ffi::c_int = 16 as core::ffi::c_int >> bl as core::ffi::c_uint;
    let have_h_split: core::ffi::c_int = ((*f).bw > (*t).bx + hsz) as core::ffi::c_int;
    let have_v_split: core::ffi::c_int = ((*f).bh > (*t).by + hsz) as core::ffi::c_int;
    if have_h_split == 0 && have_v_split == 0 {
        if !((bl as core::ffi::c_uint) < BL_8X8 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 10],
                    [core::ffi::c_char; 10],
                >(*b"decode_sb\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                2129 as core::ffi::c_int,
                b"bl < BL_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        return decode_sb(
            t,
            (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint) as BlockLevel,
            (node as uintptr_t)
                .wrapping_add(
                    (*(node as *const EdgeBranch))
                        .split_offset[0 as core::ffi::c_int as usize] as uintptr_t,
                ) as *const EdgeNode,
        );
    }
    let mut pc: *mut uint16_t = 0 as *mut uint16_t;
    let mut bp: BlockPartition = PARTITION_NONE;
    let mut ctx: core::ffi::c_int = 0;
    let mut bx8: core::ffi::c_int = 0;
    let mut by8: core::ffi::c_int = 0;
    if (*t).frame_thread.pass != 2 as core::ffi::c_int {
        if 0 as core::ffi::c_int != 0
            && bl as core::ffi::c_uint
                == BL_64X64 as core::ffi::c_int as core::ffi::c_uint
        {
            printf(
                b"poc=%d,y=%d,x=%d,bl=%d,r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*(*f).frame_hdr).frame_offset as core::ffi::c_int,
                (*t).by,
                (*t).bx,
                bl as core::ffi::c_uint,
                (*ts).msac.rng,
            );
        }
        bx8 = ((*t).bx & 31 as core::ffi::c_int) >> 1 as core::ffi::c_int;
        by8 = ((*t).by & 31 as core::ffi::c_int) >> 1 as core::ffi::c_int;
        ctx = get_partition_ctx((*t).a, &raw mut (*t).l, bl, by8, bx8);
        pc = ((*ts).cdf.m.partition[bl as usize][ctx as usize]).as_mut_ptr();
    }
    if have_h_split != 0 && have_v_split != 0 {
        if (*t).frame_thread.pass == 2 as core::ffi::c_int {
            let b: *const Av1Block = &raw mut *((*f).frame_thread.b)
                .offset(
                    ((*t).by as ptrdiff_t * (*f).b4_stride + (*t).bx as ptrdiff_t)
                        as isize,
                ) as *mut Av1Block;
            bp = (if (*b).bl as core::ffi::c_uint == bl as core::ffi::c_uint {
                (*b).bp as core::ffi::c_int
            } else {
                PARTITION_SPLIT as core::ffi::c_int
            }) as BlockPartition;
        } else {
            bp = dav1d_msac_decode_symbol_adapt_c(
                &raw mut (*ts).msac,
                pc,
                dav1d_partition_type_count[bl as usize] as size_t,
            ) as BlockPartition;
            if (*f).cur.p.layout as core::ffi::c_uint
                == DAV1D_PIXEL_LAYOUT_I422 as core::ffi::c_int as core::ffi::c_uint
                && (bp as core::ffi::c_uint
                    == PARTITION_V as core::ffi::c_int as core::ffi::c_uint
                    || bp as core::ffi::c_uint
                        == PARTITION_V4 as core::ffi::c_int as core::ffi::c_uint
                    || bp as core::ffi::c_uint
                        == PARTITION_T_LEFT_SPLIT as core::ffi::c_int
                            as core::ffi::c_uint
                    || bp as core::ffi::c_uint
                        == PARTITION_T_RIGHT_SPLIT as core::ffi::c_int
                            as core::ffi::c_uint)
            {
                return 1 as core::ffi::c_int;
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"poc=%d,y=%d,x=%d,bl=%d,ctx=%d,bp=%d: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*(*f).frame_hdr).frame_offset as core::ffi::c_int,
                    (*t).by,
                    (*t).bx,
                    bl as core::ffi::c_uint,
                    ctx,
                    bp as core::ffi::c_uint,
                    (*ts).msac.rng,
                );
            }
        }
        let b_0: *const uint8_t = (dav1d_block_sizes[bl as usize][bp as usize]).as_ptr();
        match bp as core::ffi::c_uint {
            0 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_NONE,
                    (*node).o as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
            }
            1 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_H,
                    (*node).h[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_H,
                    (*node).h[1 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by -= hsz;
            }
            2 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_V,
                    (*node).v[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_V,
                    (*node).v[1 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx -= hsz;
            }
            3 => {
                if bl as core::ffi::c_uint
                    == BL_8X8 as core::ffi::c_int as core::ffi::c_uint
                {
                    let tip: *const EdgeTip = node as *const EdgeTip;
                    if !(hsz == 1 as core::ffi::c_int) as core::ffi::c_int
                        as core::ffi::c_long != 0
                    {
                        crate::platform::assertion(
                            (::core::mem::transmute::<
                                [u8; 10],
                                [core::ffi::c_char; 10],
                            >(*b"decode_sb\0"))
                                .as_ptr(),
                            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                            2190 as core::ffi::c_int,
                            b"hsz == 1\0" as *const u8 as *const core::ffi::c_char,
                        );
                    } else {};
                    if decode_b(t, bl, BS_4x4, PARTITION_SPLIT, EDGE_ALL_TR_AND_BL) != 0
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    let tl_filter: Filter2d = (*t).tl_4x4_filter;
                    (*t).bx += 1;
                    if decode_b(
                        t,
                        bl,
                        BS_4x4,
                        PARTITION_SPLIT,
                        (*tip).split[0 as core::ffi::c_int as usize] as EdgeFlags,
                    ) != 0
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    (*t).bx -= 1;
                    (*t).by += 1;
                    if decode_b(
                        t,
                        bl,
                        BS_4x4,
                        PARTITION_SPLIT,
                        (*tip).split[1 as core::ffi::c_int as usize] as EdgeFlags,
                    ) != 0
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    (*t).bx += 1;
                    (*t).tl_4x4_filter = tl_filter;
                    if decode_b(
                        t,
                        bl,
                        BS_4x4,
                        PARTITION_SPLIT,
                        (*tip).split[2 as core::ffi::c_int as usize] as EdgeFlags,
                    ) != 0
                    {
                        return -(1 as core::ffi::c_int);
                    }
                    (*t).bx -= 1;
                    (*t).by -= 1;
                } else {
                    if decode_sb(
                        t,
                        (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                            as BlockLevel,
                        (node as uintptr_t)
                            .wrapping_add(
                                (*(node as *const EdgeBranch))
                                    .split_offset[0 as core::ffi::c_int as usize] as uintptr_t,
                            ) as *const EdgeNode,
                    ) != 0
                    {
                        return 1 as core::ffi::c_int;
                    }
                    (*t).bx += hsz;
                    if decode_sb(
                        t,
                        (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                            as BlockLevel,
                        (node as uintptr_t)
                            .wrapping_add(
                                (*(node as *const EdgeBranch))
                                    .split_offset[1 as core::ffi::c_int as usize] as uintptr_t,
                            ) as *const EdgeNode,
                    ) != 0
                    {
                        return 1 as core::ffi::c_int;
                    }
                    (*t).bx -= hsz;
                    (*t).by += hsz;
                    if decode_sb(
                        t,
                        (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                            as BlockLevel,
                        (node as uintptr_t)
                            .wrapping_add(
                                (*(node as *const EdgeBranch))
                                    .split_offset[2 as core::ffi::c_int as usize] as uintptr_t,
                            ) as *const EdgeNode,
                    ) != 0
                    {
                        return 1 as core::ffi::c_int;
                    }
                    (*t).bx += hsz;
                    if decode_sb(
                        t,
                        (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                            as BlockLevel,
                        (node as uintptr_t)
                            .wrapping_add(
                                (*(node as *const EdgeBranch))
                                    .split_offset[3 as core::ffi::c_int as usize] as uintptr_t,
                            ) as *const EdgeNode,
                    ) != 0
                    {
                        return 1 as core::ffi::c_int;
                    }
                    (*t).bx -= hsz;
                    (*t).by -= hsz;
                }
            }
            4 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_TOP_SPLIT,
                    EDGE_ALL_TR_AND_BL,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_TOP_SPLIT,
                    (*node).v[1 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx -= hsz;
                (*t).by += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(1 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_TOP_SPLIT,
                    (*node).h[1 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by -= hsz;
            }
            5 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_BOTTOM_SPLIT,
                    (*node).h[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(1 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_BOTTOM_SPLIT,
                    (*node).v[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(1 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_BOTTOM_SPLIT,
                    0 as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx -= hsz;
                (*t).by -= hsz;
            }
            6 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_LEFT_SPLIT,
                    EDGE_ALL_TR_AND_BL,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_LEFT_SPLIT,
                    (*node).h[1 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by -= hsz;
                (*t).bx += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(1 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_LEFT_SPLIT,
                    (*node).v[1 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx -= hsz;
            }
            7 => {
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_RIGHT_SPLIT,
                    (*node).v[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(1 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_RIGHT_SPLIT,
                    (*node).h[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(1 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_T_RIGHT_SPLIT,
                    0 as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by -= hsz;
                (*t).bx -= hsz;
            }
            8 => {
                let branch: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_H4,
                    (*node).h[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz >> 1 as core::ffi::c_int;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_H4,
                    (*branch).h4 as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz >> 1 as core::ffi::c_int;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_H4,
                    EDGE_ALL_LEFT_HAS_BOTTOM,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).by += hsz >> 1 as core::ffi::c_int;
                if (*t).by < (*f).bh {
                    if decode_b(
                        t,
                        bl,
                        *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                        PARTITION_H4,
                        (*node).h[1 as core::ffi::c_int as usize] as EdgeFlags,
                    ) != 0
                    {
                        return -(1 as core::ffi::c_int);
                    }
                }
                (*t).by -= hsz * 3 as core::ffi::c_int >> 1 as core::ffi::c_int;
            }
            9 => {
                let branch_0: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_V4,
                    (*node).v[0 as core::ffi::c_int as usize] as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz >> 1 as core::ffi::c_int;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_V4,
                    (*branch_0).v4 as EdgeFlags,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz >> 1 as core::ffi::c_int;
                if decode_b(
                    t,
                    bl,
                    *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                    PARTITION_V4,
                    EDGE_ALL_TOP_HAS_RIGHT,
                ) != 0
                {
                    return -(1 as core::ffi::c_int);
                }
                (*t).bx += hsz >> 1 as core::ffi::c_int;
                if (*t).bx < (*f).bw {
                    if decode_b(
                        t,
                        bl,
                        *b_0.offset(0 as core::ffi::c_int as isize) as BlockSize,
                        PARTITION_V4,
                        (*node).v[1 as core::ffi::c_int as usize] as EdgeFlags,
                    ) != 0
                    {
                        return -(1 as core::ffi::c_int);
                    }
                }
                (*t).bx -= hsz * 3 as core::ffi::c_int >> 1 as core::ffi::c_int;
            }
            _ => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 10],
                            [core::ffi::c_char; 10],
                        >(*b"decode_sb\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        2320 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
    } else if have_h_split != 0 {
        let mut is_split: core::ffi::c_uint = 0;
        if (*t).frame_thread.pass == 2 as core::ffi::c_int {
            let b_1: *const Av1Block = &raw mut *((*f).frame_thread.b)
                .offset(
                    ((*t).by as ptrdiff_t * (*f).b4_stride + (*t).bx as ptrdiff_t)
                        as isize,
                ) as *mut Av1Block;
            is_split = ((*b_1).bl as core::ffi::c_uint != bl as core::ffi::c_uint)
                as core::ffi::c_int as core::ffi::c_uint;
        } else {
            is_split = dav1d_msac_decode_bool_c(
                &raw mut (*ts).msac,
                gather_top_partition_prob(pc, bl),
            );
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"poc=%d,y=%d,x=%d,bl=%d,ctx=%d,bp=%d: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*(*f).frame_hdr).frame_offset as core::ffi::c_int,
                    (*t).by,
                    (*t).bx,
                    bl as core::ffi::c_uint,
                    ctx,
                    if is_split != 0 {
                        PARTITION_SPLIT as core::ffi::c_int
                    } else {
                        PARTITION_H as core::ffi::c_int
                    },
                    (*ts).msac.rng,
                );
            }
        }
        if !((bl as core::ffi::c_uint) < BL_8X8 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 10],
                    [core::ffi::c_char; 10],
                >(*b"decode_sb\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                2336 as core::ffi::c_int,
                b"bl < BL_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if is_split != 0 {
            bp = PARTITION_SPLIT;
            if decode_sb(
                t,
                (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                    as BlockLevel,
                (node as uintptr_t)
                    .wrapping_add(
                        (*(node as *const EdgeBranch))
                            .split_offset[0 as core::ffi::c_int as usize] as uintptr_t,
                    ) as *const EdgeNode,
            ) != 0
            {
                return 1 as core::ffi::c_int;
            }
            (*t).bx += hsz;
            if decode_sb(
                t,
                (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                    as BlockLevel,
                (node as uintptr_t)
                    .wrapping_add(
                        (*(node as *const EdgeBranch))
                            .split_offset[1 as core::ffi::c_int as usize] as uintptr_t,
                    ) as *const EdgeNode,
            ) != 0
            {
                return 1 as core::ffi::c_int;
            }
            (*t).bx -= hsz;
        } else {
            bp = PARTITION_H;
            if decode_b(
                t,
                bl,
                dav1d_block_sizes[bl
                    as usize][PARTITION_H as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] as BlockSize,
                PARTITION_H,
                (*node).h[0 as core::ffi::c_int as usize] as EdgeFlags,
            ) != 0
            {
                return -(1 as core::ffi::c_int);
            }
        }
    } else {
        if (have_v_split == 0) as core::ffi::c_int as core::ffi::c_long != 0 {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 10],
                    [core::ffi::c_char; 10],
                >(*b"decode_sb\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                2350 as core::ffi::c_int,
                b"have_v_split\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        let mut is_split_0: core::ffi::c_uint = 0;
        if (*t).frame_thread.pass == 2 as core::ffi::c_int {
            let b_2: *const Av1Block = &raw mut *((*f).frame_thread.b)
                .offset(
                    ((*t).by as ptrdiff_t * (*f).b4_stride + (*t).bx as ptrdiff_t)
                        as isize,
                ) as *mut Av1Block;
            is_split_0 = ((*b_2).bl as core::ffi::c_uint != bl as core::ffi::c_uint)
                as core::ffi::c_int as core::ffi::c_uint;
        } else {
            is_split_0 = dav1d_msac_decode_bool_c(
                &raw mut (*ts).msac,
                gather_left_partition_prob(pc, bl),
            );
            if (*f).cur.p.layout as core::ffi::c_uint
                == DAV1D_PIXEL_LAYOUT_I422 as core::ffi::c_int as core::ffi::c_uint
                && is_split_0 == 0
            {
                return 1 as core::ffi::c_int;
            }
            if 0 as core::ffi::c_int != 0
                && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                    == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
                && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
                && (*t).bx < 12 as core::ffi::c_int
            {
                printf(
                    b"poc=%d,y=%d,x=%d,bl=%d,ctx=%d,bp=%d: r=%d\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*(*f).frame_hdr).frame_offset as core::ffi::c_int,
                    (*t).by,
                    (*t).bx,
                    bl as core::ffi::c_uint,
                    ctx,
                    if is_split_0 != 0 {
                        PARTITION_SPLIT as core::ffi::c_int
                    } else {
                        PARTITION_V as core::ffi::c_int
                    },
                    (*ts).msac.rng,
                );
            }
        }
        if !((bl as core::ffi::c_uint) < BL_8X8 as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 10],
                    [core::ffi::c_char; 10],
                >(*b"decode_sb\0"))
                    .as_ptr(),
                b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                2366 as core::ffi::c_int,
                b"bl < BL_8X8\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        if is_split_0 != 0 {
            bp = PARTITION_SPLIT;
            if decode_sb(
                t,
                (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                    as BlockLevel,
                (node as uintptr_t)
                    .wrapping_add(
                        (*(node as *const EdgeBranch))
                            .split_offset[0 as core::ffi::c_int as usize] as uintptr_t,
                    ) as *const EdgeNode,
            ) != 0
            {
                return 1 as core::ffi::c_int;
            }
            (*t).by += hsz;
            if decode_sb(
                t,
                (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                    as BlockLevel,
                (node as uintptr_t)
                    .wrapping_add(
                        (*(node as *const EdgeBranch))
                            .split_offset[2 as core::ffi::c_int as usize] as uintptr_t,
                    ) as *const EdgeNode,
            ) != 0
            {
                return 1 as core::ffi::c_int;
            }
            (*t).by -= hsz;
        } else {
            bp = PARTITION_V;
            if decode_b(
                t,
                bl,
                dav1d_block_sizes[bl
                    as usize][PARTITION_V as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize] as BlockSize,
                PARTITION_V,
                (*node).v[0 as core::ffi::c_int as usize] as EdgeFlags,
            ) != 0
            {
                return -(1 as core::ffi::c_int);
            }
        }
    }
    if (*t).frame_thread.pass != 2 as core::ffi::c_int
        && (bp as core::ffi::c_uint
            != PARTITION_SPLIT as core::ffi::c_int as core::ffi::c_uint
            || bl as core::ffi::c_uint
                == BL_8X8 as core::ffi::c_int as core::ffi::c_uint)
    {
        match ulog2(hsz as core::ffi::c_uint) {
            0 => {
                (*(&raw mut *((*(*t).a).partition).as_mut_ptr().offset(bx8 as isize)
                    as *mut uint8_t as *mut alias8))
                    .u8_0 = (dav1d_al_part_ctx[0 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_int
                    * 0x1 as core::ffi::c_int) as uint8_t;
                (*(&raw mut *((*t).l.partition).as_mut_ptr().offset(by8 as isize)
                    as *mut uint8_t as *mut alias8))
                    .u8_0 = (dav1d_al_part_ctx[1 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_int
                    * 0x1 as core::ffi::c_int) as uint8_t;
            }
            1 => {
                (*(&raw mut *((*(*t).a).partition).as_mut_ptr().offset(bx8 as isize)
                    as *mut uint8_t as *mut alias16))
                    .u16_0 = (dav1d_al_part_ctx[0 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_int
                    * 0x101 as core::ffi::c_int) as uint16_t;
                (*(&raw mut *((*t).l.partition).as_mut_ptr().offset(by8 as isize)
                    as *mut uint8_t as *mut alias16))
                    .u16_0 = (dav1d_al_part_ctx[1 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_int
                    * 0x101 as core::ffi::c_int) as uint16_t;
            }
            2 => {
                (*(&raw mut *((*(*t).a).partition).as_mut_ptr().offset(bx8 as isize)
                    as *mut uint8_t as *mut alias32))
                    .u32_0 = (dav1d_al_part_ctx[0 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_uint)
                    .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
                (*(&raw mut *((*t).l.partition).as_mut_ptr().offset(by8 as isize)
                    as *mut uint8_t as *mut alias32))
                    .u32_0 = (dav1d_al_part_ctx[1 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_uint)
                    .wrapping_mul(0x1010101 as core::ffi::c_uint) as uint32_t;
            }
            3 => {
                (*(&raw mut *((*(*t).a).partition).as_mut_ptr().offset(bx8 as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = (dav1d_al_part_ctx[0 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_ulonglong)
                    .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                    as uint64_t;
                (*(&raw mut *((*t).l.partition).as_mut_ptr().offset(by8 as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = (dav1d_al_part_ctx[1 as core::ffi::c_int
                    as usize][bl as usize][bp as usize] as core::ffi::c_ulonglong)
                    .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong)
                    as uint64_t;
            }
            4 => {
                memset(
                    &raw mut *((*(*t).a).partition).as_mut_ptr().offset(bx8 as isize)
                        as *mut uint8_t as *mut core::ffi::c_void,
                    dav1d_al_part_ctx[0 as core::ffi::c_int
                        as usize][bl as usize][bp as usize] as core::ffi::c_int,
                    16 as size_t,
                );
                memset(
                    &raw mut *((*t).l.partition).as_mut_ptr().offset(by8 as isize)
                        as *mut uint8_t as *mut core::ffi::c_void,
                    dav1d_al_part_ctx[1 as core::ffi::c_int
                        as usize][bl as usize][bp as usize] as core::ffi::c_int,
                    16 as size_t,
                );
            }
            _ => {
                if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
                    != 0
                {
                    crate::platform::assertion(
                        (::core::mem::transmute::<
                            [u8; 10],
                            [core::ffi::c_char; 10],
                        >(*b"decode_sb\0"))
                            .as_ptr(),
                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                        2385 as core::ffi::c_int,
                        b"0\0" as *const u8 as *const core::ffi::c_char,
                    );
                } else {};
            }
        }
    }
    return 0 as core::ffi::c_int;
}
unsafe extern "C" fn reset_context(
    ctx: *mut BlockContext,
    keyframe: core::ffi::c_int,
    pass: core::ffi::c_int,
) {
    memset(
        ((*ctx).intra).as_mut_ptr() as *mut core::ffi::c_void,
        keyframe,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).uvmode).as_mut_ptr() as *mut core::ffi::c_void,
        DC_PRED as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    if keyframe != 0 {
        memset(
            ((*ctx).mode).as_mut_ptr() as *mut core::ffi::c_void,
            DC_PRED as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
    }
    if pass == 2 as core::ffi::c_int {
        return;
    }
    memset(
        ((*ctx).partition).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        ((*ctx).skip).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).skip_mode).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).tx_lpf_y).as_mut_ptr() as *mut core::ffi::c_void,
        2 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).tx_lpf_uv).as_mut_ptr() as *mut core::ffi::c_void,
        1 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).tx_intra).as_mut_ptr() as *mut core::ffi::c_void,
        -(1 as core::ffi::c_int),
        ::core::mem::size_of::<[int8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).tx).as_mut_ptr() as *mut core::ffi::c_void,
        TX_64X64 as core::ffi::c_int,
        ::core::mem::size_of::<[int8_t; 32]>() as size_t,
    );
    if keyframe == 0 {
        memset(
            ((*ctx).ref_0).as_mut_ptr() as *mut core::ffi::c_void,
            -(1 as core::ffi::c_int),
            ::core::mem::size_of::<[[int8_t; 32]; 2]>() as size_t,
        );
        memset(
            ((*ctx).comp_type).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        memset(
            ((*ctx).mode).as_mut_ptr() as *mut core::ffi::c_void,
            NEARESTMV as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
    }
    memset(
        ((*ctx).lcoef).as_mut_ptr() as *mut core::ffi::c_void,
        0x40 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).ccoef).as_mut_ptr() as *mut core::ffi::c_void,
        0x40 as core::ffi::c_int,
        ::core::mem::size_of::<[[uint8_t; 32]; 2]>() as size_t,
    );
    memset(
        ((*ctx).filter).as_mut_ptr() as *mut core::ffi::c_void,
        DAV1D_N_SWITCHABLE_FILTERS as core::ffi::c_int,
        ::core::mem::size_of::<[[uint8_t; 32]; 2]>() as size_t,
    );
    memset(
        ((*ctx).seg_pred).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    memset(
        ((*ctx).pal_sz).as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
}
static mut ss_size_mul: [[uint8_t; 2]; 4] = [
    [4 as core::ffi::c_int as uint8_t, 4 as core::ffi::c_int as uint8_t],
    [6 as core::ffi::c_int as uint8_t, 5 as core::ffi::c_int as uint8_t],
    [8 as core::ffi::c_int as uint8_t, 6 as core::ffi::c_int as uint8_t],
    [12 as core::ffi::c_int as uint8_t, 8 as core::ffi::c_int as uint8_t],
];
unsafe extern "C" fn setup_tile(
    ts: *mut Dav1dTileState,
    f: *const Dav1dFrameContext,
    data: *const uint8_t,
    sz: size_t,
    tile_row: core::ffi::c_int,
    tile_col: core::ffi::c_int,
    tile_start_off: core::ffi::c_uint,
) {
    let col_sb_start: core::ffi::c_int = (*(*f).frame_hdr)
        .tiling
        .col_start_sb[tile_col as usize] as core::ffi::c_int;
    let col_sb128_start: core::ffi::c_int = col_sb_start
        >> ((*(*f).seq_hdr).sb128 == 0) as core::ffi::c_int;
    let col_sb_end: core::ffi::c_int = (*(*f).frame_hdr)
        .tiling
        .col_start_sb[(tile_col + 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
    let row_sb_start: core::ffi::c_int = (*(*f).frame_hdr)
        .tiling
        .row_start_sb[tile_row as usize] as core::ffi::c_int;
    let row_sb_end: core::ffi::c_int = (*(*f).frame_hdr)
        .tiling
        .row_start_sb[(tile_row + 1 as core::ffi::c_int) as usize] as core::ffi::c_int;
    let sb_shift: core::ffi::c_int = (*f).sb_shift;
    let size_mul: *const uint8_t = (ss_size_mul[(*f).cur.p.layout as usize]).as_ptr();
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    while p < 2 as core::ffi::c_int {
        (*ts).frame_thread[p as usize].pal_idx = if !((*f).frame_thread.pal_idx)
            .is_null()
        {
            &raw mut *((*f).frame_thread.pal_idx)
                .offset(
                    (tile_start_off as size_t)
                        .wrapping_mul(
                            *size_mul.offset(1 as core::ffi::c_int as isize) as size_t,
                        )
                        .wrapping_div(8 as size_t) as isize,
                ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        (*ts).frame_thread[p as usize].cbi = if !((*f).frame_thread.cbi).is_null() {
            &raw mut *((*f).frame_thread.cbi)
                .offset(
                    (tile_start_off as size_t)
                        .wrapping_mul(
                            *size_mul.offset(0 as core::ffi::c_int as isize) as size_t,
                        )
                        .wrapping_div(64 as size_t) as isize,
                ) as *mut int16_t
        } else {
            0 as *mut int16_t
        };
        (*ts).frame_thread[p as usize].cf = (if !((*f).frame_thread.cf).is_null() {
            ((*f).frame_thread.cf as *mut uint8_t)
                .offset(
                    ((tile_start_off as size_t)
                        .wrapping_mul(
                            *size_mul.offset(0 as core::ffi::c_int as isize) as size_t,
                        ) >> ((*(*f).seq_hdr).hbd == 0) as core::ffi::c_int) as isize,
                )
        } else {
            0 as *mut uint8_t
        }) as *mut core::ffi::c_void;
        p += 1;
    }
    dav1d_cdf_thread_copy(&raw mut (*ts).cdf, &(*f).in_cdf);
    (*ts).last_qidx = (*(*f).frame_hdr).quant.yac as core::ffi::c_int;
    (*ts).last_delta_lf.u32_0 = 0 as uint32_t;
    dav1d_msac_init(
        &raw mut (*ts).msac,
        data,
        sz,
        (*(*f).frame_hdr).disable_cdf_update as core::ffi::c_int,
    );
    (*ts).tiling.row = tile_row;
    (*ts).tiling.col = tile_col;
    (*ts).tiling.col_start = col_sb_start << sb_shift;
    (*ts).tiling.col_end = imin(col_sb_end << sb_shift, (*f).bw);
    (*ts).tiling.row_start = row_sb_start << sb_shift;
    (*ts).tiling.row_end = imin(row_sb_end << sb_shift, (*f).bh);
    let mut sb_idx: core::ffi::c_int = 0;
    let mut unit_idx: core::ffi::c_int = 0;
    if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
        != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
    {
        sb_idx = ((*ts).tiling.row_start >> 5 as core::ffi::c_int) * (*f).sr_sb128w;
        unit_idx = ((*ts).tiling.row_start & 16 as core::ffi::c_int)
            >> 3 as core::ffi::c_int;
    } else {
        sb_idx = ((*ts).tiling.row_start >> 5 as core::ffi::c_int) * (*f).sb128w
            + col_sb128_start;
        unit_idx = (((*ts).tiling.row_start & 16 as core::ffi::c_int)
            >> 3 as core::ffi::c_int)
            + (((*ts).tiling.col_start & 16 as core::ffi::c_int)
                >> 4 as core::ffi::c_int);
    }
    let mut current_block_32: u64;
    let mut p_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while p_0 < 3 as core::ffi::c_int {
        if !(((*f).lf.restore_planes >> p_0) as core::ffi::c_uint
            & 1 as core::ffi::c_uint == 0)
        {
            if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
            {
                let ss_hor: core::ffi::c_int = (p_0 != 0
                    && (*f).cur.p.layout as core::ffi::c_uint
                        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
                            as core::ffi::c_uint) as core::ffi::c_int;
                let d: core::ffi::c_int = (*(*f).frame_hdr)
                    .super_res
                    .width_scale_denominator as core::ffi::c_int;
                let unit_size_log2: core::ffi::c_int = (*(*f).frame_hdr)
                    .restoration
                    .unit_size[(p_0 != 0) as core::ffi::c_int as usize]
                    as core::ffi::c_int;
                let rnd: core::ffi::c_int = ((8 as core::ffi::c_int) << unit_size_log2)
                    - 1 as core::ffi::c_int;
                let shift: core::ffi::c_int = unit_size_log2 + 3 as core::ffi::c_int;
                let x: core::ffi::c_int = (4 as core::ffi::c_int * (*ts).tiling.col_start
                    * d >> ss_hor) + rnd >> shift;
                let px_x: core::ffi::c_int = x << unit_size_log2 + ss_hor;
                let u_idx: core::ffi::c_int = unit_idx
                    + ((px_x & 64 as core::ffi::c_int) >> 6 as core::ffi::c_int);
                let sb128x: core::ffi::c_int = px_x >> 7 as core::ffi::c_int;
                if sb128x >= (*f).sr_sb128w {
                    current_block_32 = 0;
                } else {
                    (*ts).lr_ref[p_0 as usize] = &raw mut *(*((*((*f).lf.lr_mask)
                        .offset((sb_idx + sb128x) as isize))
                        .lr)
                        .as_mut_ptr()
                        .offset(p_0 as isize))
                        .as_mut_ptr()
                        .offset(u_idx as isize) as *mut Av1RestorationUnit;
                    current_block_32 = 1;
                }
            } else {
                (*ts).lr_ref[p_0 as usize] = &raw mut *(*((*((*f).lf.lr_mask)
                    .offset(sb_idx as isize))
                    .lr)
                    .as_mut_ptr()
                    .offset(p_0 as isize))
                    .as_mut_ptr()
                    .offset(unit_idx as isize) as *mut Av1RestorationUnit;
                current_block_32 = 1;
            }
            match current_block_32 {
                0 => {}
                _ => {
                    (*(*ts).lr_ref[p_0 as usize])
                        .filter_v[0 as core::ffi::c_int as usize] = 3 as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .filter_v[1 as core::ffi::c_int as usize] = -(7
                        as core::ffi::c_int) as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .filter_v[2 as core::ffi::c_int as usize] = 15 as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .filter_h[0 as core::ffi::c_int as usize] = 3 as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .filter_h[1 as core::ffi::c_int as usize] = -(7
                        as core::ffi::c_int) as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .filter_h[2 as core::ffi::c_int as usize] = 15 as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .sgr_weights[0 as core::ffi::c_int as usize] = -(32
                        as core::ffi::c_int) as int8_t;
                    (*(*ts).lr_ref[p_0 as usize])
                        .sgr_weights[1 as core::ffi::c_int as usize] = 31 as int8_t;
                }
            }
        }
        p_0 += 1;
    }
    if (*(*f).c).n_tc > 1 as core::ffi::c_uint {
        let mut p_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while p_1 < 2 as core::ffi::c_int {
            *(&raw mut *((*ts).progress).as_mut_ptr().offset(p_1 as isize)
                as *mut atomic_int) = row_sb_start;
            p_1 += 1;
        }
    }
}
unsafe extern "C" fn read_restoration_info(
    t: *mut Dav1dTaskContext,
    lr: *mut Av1RestorationUnit,
    p: core::ffi::c_int,
    frame_type: Dav1dRestorationType,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let ts: *mut Dav1dTileState = (*t).ts;
    let lr_ref: *const Av1RestorationUnit = (*ts).lr_ref[p as usize];
    if frame_type as core::ffi::c_uint
        == DAV1D_RESTORATION_SWITCHABLE as core::ffi::c_int as core::ffi::c_uint
    {
        let filter: core::ffi::c_int = dav1d_msac_decode_symbol_adapt_c(
            &raw mut (*ts).msac,
            ((*ts).cdf.m.restore_switchable).as_mut_ptr(),
            2 as size_t,
        ) as core::ffi::c_int;
        (*lr).type_0 = (filter + (filter != 0) as core::ffi::c_int) as uint8_t;
    } else {
        let type_0: core::ffi::c_uint = dav1d_msac_decode_bool_adapt_c(
            &raw mut (*ts).msac,
            if frame_type as core::ffi::c_uint
                == DAV1D_RESTORATION_WIENER as core::ffi::c_int as core::ffi::c_uint
            {
                ((*ts).cdf.m.restore_wiener).as_mut_ptr()
            } else {
                ((*ts).cdf.m.restore_sgrproj).as_mut_ptr()
            },
        ) as core::ffi::c_uint;
        (*lr).type_0 = (if type_0 != 0 {
            frame_type as core::ffi::c_uint
        } else {
            DAV1D_RESTORATION_NONE as core::ffi::c_int as core::ffi::c_uint
        }) as uint8_t;
    }
    if (*lr).type_0 as core::ffi::c_int == DAV1D_RESTORATION_WIENER as core::ffi::c_int {
        (*lr).filter_v[0 as core::ffi::c_int as usize] = (if p != 0 {
            0 as core::ffi::c_int
        } else {
            dav1d_msac_decode_subexp(
                &raw mut (*ts).msac,
                (*lr_ref).filter_v[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    + 5 as core::ffi::c_int,
                16 as core::ffi::c_int,
                1 as core::ffi::c_uint,
            ) - 5 as core::ffi::c_int
        }) as int8_t;
        (*lr).filter_v[1 as core::ffi::c_int as usize] = (dav1d_msac_decode_subexp(
            &raw mut (*ts).msac,
            (*lr_ref).filter_v[1 as core::ffi::c_int as usize] as core::ffi::c_int
                + 23 as core::ffi::c_int,
            32 as core::ffi::c_int,
            2 as core::ffi::c_uint,
        ) - 23 as core::ffi::c_int) as int8_t;
        (*lr).filter_v[2 as core::ffi::c_int as usize] = (dav1d_msac_decode_subexp(
            &raw mut (*ts).msac,
            (*lr_ref).filter_v[2 as core::ffi::c_int as usize] as core::ffi::c_int
                + 17 as core::ffi::c_int,
            64 as core::ffi::c_int,
            3 as core::ffi::c_uint,
        ) - 17 as core::ffi::c_int) as int8_t;
        (*lr).filter_h[0 as core::ffi::c_int as usize] = (if p != 0 {
            0 as core::ffi::c_int
        } else {
            dav1d_msac_decode_subexp(
                &raw mut (*ts).msac,
                (*lr_ref).filter_h[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    + 5 as core::ffi::c_int,
                16 as core::ffi::c_int,
                1 as core::ffi::c_uint,
            ) - 5 as core::ffi::c_int
        }) as int8_t;
        (*lr).filter_h[1 as core::ffi::c_int as usize] = (dav1d_msac_decode_subexp(
            &raw mut (*ts).msac,
            (*lr_ref).filter_h[1 as core::ffi::c_int as usize] as core::ffi::c_int
                + 23 as core::ffi::c_int,
            32 as core::ffi::c_int,
            2 as core::ffi::c_uint,
        ) - 23 as core::ffi::c_int) as int8_t;
        (*lr).filter_h[2 as core::ffi::c_int as usize] = (dav1d_msac_decode_subexp(
            &raw mut (*ts).msac,
            (*lr_ref).filter_h[2 as core::ffi::c_int as usize] as core::ffi::c_int
                + 17 as core::ffi::c_int,
            64 as core::ffi::c_int,
            3 as core::ffi::c_uint,
        ) - 17 as core::ffi::c_int) as int8_t;
        memcpy(
            ((*lr).sgr_weights).as_mut_ptr() as *mut core::ffi::c_void,
            ((*lr_ref).sgr_weights).as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[int8_t; 2]>() as size_t,
        );
        (*ts).lr_ref[p as usize] = lr;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-lr_wiener[pl=%d,v[%d,%d,%d],h[%d,%d,%d]]: r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                p,
                (*lr).filter_v[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*lr).filter_v[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*lr).filter_v[2 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*lr).filter_h[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*lr).filter_h[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*lr).filter_h[2 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
    } else if (*lr).type_0 as core::ffi::c_int
        == DAV1D_RESTORATION_SGRPROJ as core::ffi::c_int
    {
        let idx: core::ffi::c_uint = dav1d_msac_decode_bools(
            &raw mut (*ts).msac,
            4 as core::ffi::c_uint,
        ) as core::ffi::c_uint;
        let sgr_params: *const uint16_t = (dav1d_sgr_params[idx as usize]).as_ptr();
        (*lr).type_0 = ((*lr).type_0 as core::ffi::c_uint).wrapping_add(idx) as uint8_t
            as uint8_t;
        (*lr).sgr_weights[0 as core::ffi::c_int as usize] = (if *sgr_params
            .offset(0 as core::ffi::c_int as isize) as core::ffi::c_int != 0
        {
            dav1d_msac_decode_subexp(
                &raw mut (*ts).msac,
                (*lr_ref).sgr_weights[0 as core::ffi::c_int as usize] as core::ffi::c_int
                    + 96 as core::ffi::c_int,
                128 as core::ffi::c_int,
                4 as core::ffi::c_uint,
            ) - 96 as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        }) as int8_t;
        (*lr).sgr_weights[1 as core::ffi::c_int as usize] = (if *sgr_params
            .offset(1 as core::ffi::c_int as isize) as core::ffi::c_int != 0
        {
            dav1d_msac_decode_subexp(
                &raw mut (*ts).msac,
                (*lr_ref).sgr_weights[1 as core::ffi::c_int as usize] as core::ffi::c_int
                    + 32 as core::ffi::c_int,
                128 as core::ffi::c_int,
                4 as core::ffi::c_uint,
            ) - 32 as core::ffi::c_int
        } else {
            95 as core::ffi::c_int
        }) as int8_t;
        memcpy(
            ((*lr).filter_v).as_mut_ptr() as *mut core::ffi::c_void,
            ((*lr_ref).filter_v).as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[int8_t; 3]>() as size_t,
        );
        memcpy(
            ((*lr).filter_h).as_mut_ptr() as *mut core::ffi::c_void,
            ((*lr_ref).filter_h).as_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[int8_t; 3]>() as size_t,
        );
        (*ts).lr_ref[p as usize] = lr;
        if 0 as core::ffi::c_int != 0
            && (*(*f).frame_hdr).frame_offset as core::ffi::c_int
                == 2 as core::ffi::c_int && (*t).by >= 0 as core::ffi::c_int
            && (*t).by < 4 as core::ffi::c_int && (*t).bx >= 8 as core::ffi::c_int
            && (*t).bx < 12 as core::ffi::c_int
        {
            printf(
                b"Post-lr_sgrproj[pl=%d,idx=%d,w[%d,%d]]: r=%d\n\0" as *const u8
                    as *const core::ffi::c_char,
                p,
                idx,
                (*lr).sgr_weights[0 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*lr).sgr_weights[1 as core::ffi::c_int as usize] as core::ffi::c_int,
                (*ts).msac.rng,
            );
        }
    }
}
unsafe extern "C" fn check_trailing_bits_after_symbol_coder(
    msac: *const MsacContext,
) -> core::ffi::c_int {
    let n_bits: core::ffi::c_int = -((*msac).cnt + 14 as core::ffi::c_int);
    if !(n_bits <= 0 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 39],
                [core::ffi::c_char; 39],
            >(*b"check_trailing_bits_after_symbol_coder\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            2582 as core::ffi::c_int,
            b"n_bits <= 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let n_bytes: core::ffi::c_int = n_bits + 7 as core::ffi::c_int
        >> 3 as core::ffi::c_int;
    let mut p: *const uint8_t = &*((*msac).buf_pos).offset(n_bytes as isize)
        as *const uint8_t;
    let pattern: core::ffi::c_int = 128 as core::ffi::c_int
        >> (n_bits - 1 as core::ffi::c_int & 7 as core::ffi::c_int);
    if *p.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
        & 2 as core::ffi::c_int * pattern - 1 as core::ffi::c_int != pattern
    {
        return 1 as core::ffi::c_int;
    }
    while p < (*msac).buf_end {
        if *p != 0 {
            return 1 as core::ffi::c_int;
        }
        p = p.offset(1);
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_tile_sbrow(
    t: *mut Dav1dTaskContext,
) -> core::ffi::c_int {
    let f: *const Dav1dFrameContext = (*t).f;
    let root_bl: BlockLevel = (if (*(*f).seq_hdr).sb128 as core::ffi::c_int != 0 {
        BL_128X128 as core::ffi::c_int
    } else {
        BL_64X64 as core::ffi::c_int
    }) as BlockLevel;
    let ts: *mut Dav1dTileState = (*t).ts;
    let c: *const Dav1dContext = (*f).c;
    let sb_step: core::ffi::c_int = (*f).sb_step;
    let tile_row: core::ffi::c_int = (*ts).tiling.row;
    let tile_col: core::ffi::c_int = (*ts).tiling.col;
    let col_sb_start: core::ffi::c_int = (*(*f).frame_hdr)
        .tiling
        .col_start_sb[tile_col as usize] as core::ffi::c_int;
    let col_sb128_start: core::ffi::c_int = col_sb_start
        >> ((*(*f).seq_hdr).sb128 == 0) as core::ffi::c_int;
    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint != 0
        || (*(*f).frame_hdr).allow_intrabc as core::ffi::c_int != 0
    {
        dav1d_refmvs_tile_sbrow_init(
            &raw mut (*t).rt,
            &(*f).rf,
            (*ts).tiling.col_start,
            (*ts).tiling.col_end,
            (*ts).tiling.row_start,
            (*ts).tiling.row_end,
            (*t).by >> (*f).sb_shift,
            (*ts).tiling.row,
            (*t).frame_thread.pass,
        );
    }
    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint != 0
        && (*c).n_fc > 1 as core::ffi::c_uint
    {
        let sby: core::ffi::c_int = (*t).by - (*ts).tiling.row_start >> (*f).sb_shift;
        let lowest_px: *mut [core::ffi::c_int; 2] = (*((*ts).lowest_pixel)
            .offset(sby as isize))
            .as_mut_ptr() as *mut [core::ffi::c_int; 2];
        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
        while n < 7 as core::ffi::c_int {
            let mut m: core::ffi::c_int = 0 as core::ffi::c_int;
            while m < 2 as core::ffi::c_int {
                (*lowest_px.offset(n as isize))[m as usize] = INT_MIN;
                m += 1;
            }
            n += 1;
        }
    }
    reset_context(
        &raw mut (*t).l,
        ((*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint == 0)
            as core::ffi::c_int,
        (*t).frame_thread.pass,
    );
    if (*t).frame_thread.pass == 2 as core::ffi::c_int {
        let off_2pass: core::ffi::c_int = if (*c).n_tc > 1 as core::ffi::c_uint {
            (*f).sb128w * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        };
        (*t).bx = (*ts).tiling.col_start;
        (*t).a = ((*f).a)
            .offset(off_2pass as isize)
            .offset(col_sb128_start as isize)
            .offset((tile_row * (*f).sb128w) as isize);
        while (*t).bx < (*ts).tiling.col_end {
            if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::Acquire}, false>((*c).flush) != 0 {
                return 1 as core::ffi::c_int;
            }
            if decode_sb(t, root_bl, dav1d_intra_edge_tree[root_bl as usize]) != 0 {
                return 1 as core::ffi::c_int;
            }
            if (*t).bx & 16 as core::ffi::c_int != 0
                || (*(*f).seq_hdr).sb128 as core::ffi::c_int != 0
            {
                (*t).a = ((*t).a).offset(1);
            }
            (*t).bx += sb_step;
        }
        ((*f).bd_fn.backup_ipred_edge).expect("non-null function pointer")(t);
        return 0 as core::ffi::c_int;
    }
    if (*(*f).c).n_tc > 1 as core::ffi::c_uint
        && (*(*f).frame_hdr).use_ref_frame_mvs as core::ffi::c_int != 0
    {
        ((*(*f).c).refmvs_dsp.load_tmvs)
            .expect(
                "non-null function pointer",
            )(
            &(*f).rf,
            (*ts).tiling.row,
            (*ts).tiling.col_start >> 1 as core::ffi::c_int,
            (*ts).tiling.col_end >> 1 as core::ffi::c_int,
            (*t).by >> 1 as core::ffi::c_int,
            (*t).by + sb_step >> 1 as core::ffi::c_int,
        );
    }
    memset(
        ((*t).pal_sz_uv[1 as core::ffi::c_int as usize]).as_mut_ptr()
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    let sb128y: core::ffi::c_int = (*t).by >> 5 as core::ffi::c_int;
    (*t).bx = (*ts).tiling.col_start;
    (*t).a = ((*f).a)
        .offset(col_sb128_start as isize)
        .offset((tile_row * (*f).sb128w) as isize);
    (*t).lf_mask = ((*f).lf.mask)
        .offset((sb128y * (*f).sb128w) as isize)
        .offset(col_sb128_start as isize);
    while (*t).bx < (*ts).tiling.col_end {
        if ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::Acquire}, false>((*c).flush) != 0 {
            return 1 as core::ffi::c_int;
        }
        if root_bl as core::ffi::c_uint
            == BL_128X128 as core::ffi::c_int as core::ffi::c_uint
        {
            (*t).cur_sb_cdef_idx_ptr = ((*(*t).lf_mask).cdef_idx).as_mut_ptr();
            *((*t).cur_sb_cdef_idx_ptr).offset(0 as core::ffi::c_int as isize) = -(1
                as core::ffi::c_int) as int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(1 as core::ffi::c_int as isize) = -(1
                as core::ffi::c_int) as int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(2 as core::ffi::c_int as isize) = -(1
                as core::ffi::c_int) as int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(3 as core::ffi::c_int as isize) = -(1
                as core::ffi::c_int) as int8_t;
        } else {
            (*t).cur_sb_cdef_idx_ptr = &raw mut *((*(*t).lf_mask).cdef_idx)
                .as_mut_ptr()
                .offset(
                    ((((*t).bx & 16 as core::ffi::c_int) >> 4 as core::ffi::c_int)
                        + (((*t).by & 16 as core::ffi::c_int) >> 3 as core::ffi::c_int))
                        as isize,
                ) as *mut int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(0 as core::ffi::c_int as isize) = -(1
                as core::ffi::c_int) as int8_t;
        }
        let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
        while p < 3 as core::ffi::c_int {
            if !(((*f).lf.restore_planes >> p) as core::ffi::c_uint
                & 1 as core::ffi::c_uint == 0)
            {
                let ss_ver: core::ffi::c_int = (p != 0
                    && (*f).cur.p.layout as core::ffi::c_uint
                        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int
                            as core::ffi::c_uint) as core::ffi::c_int;
                let ss_hor: core::ffi::c_int = (p != 0
                    && (*f).cur.p.layout as core::ffi::c_uint
                        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
                            as core::ffi::c_uint) as core::ffi::c_int;
                let unit_size_log2: core::ffi::c_int = (*(*f).frame_hdr)
                    .restoration
                    .unit_size[(p != 0) as core::ffi::c_int as usize]
                    as core::ffi::c_int;
                let y: core::ffi::c_int = (*t).by * 4 as core::ffi::c_int >> ss_ver;
                let h: core::ffi::c_int = (*f).cur.p.h + ss_ver >> ss_ver;
                let unit_size: core::ffi::c_int = (1 as core::ffi::c_int)
                    << unit_size_log2;
                let mask: core::ffi::c_uint = (unit_size - 1 as core::ffi::c_int)
                    as core::ffi::c_uint;
                if !(y as core::ffi::c_uint & mask != 0) {
                    let half_unit: core::ffi::c_int = unit_size >> 1 as core::ffi::c_int;
                    if !(y != 0 && y + half_unit > h) {
                        let frame_type: Dav1dRestorationType = (*(*f).frame_hdr)
                            .restoration
                            .type_0[p as usize];
                        if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                            != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
                        {
                            let w: core::ffi::c_int = (*f).sr_cur.p.p.w + ss_hor
                                >> ss_hor;
                            let n_units: core::ffi::c_int = imax(
                                1 as core::ffi::c_int,
                                w + half_unit >> unit_size_log2,
                            ) as core::ffi::c_int;
                            let d: core::ffi::c_int = (*(*f).frame_hdr)
                                .super_res
                                .width_scale_denominator as core::ffi::c_int;
                            let rnd: core::ffi::c_int = unit_size * 8 as core::ffi::c_int
                                - 1 as core::ffi::c_int;
                            let shift: core::ffi::c_int = unit_size_log2
                                + 3 as core::ffi::c_int;
                            let x0: core::ffi::c_int = (4 as core::ffi::c_int * (*t).bx
                                * d >> ss_hor) + rnd >> shift;
                            let x1: core::ffi::c_int = (4 as core::ffi::c_int
                                * ((*t).bx + sb_step) * d >> ss_hor) + rnd >> shift;
                            let mut x: core::ffi::c_int = x0;
                            while x < imin(x1, n_units) {
                                let px_x: core::ffi::c_int = x << unit_size_log2 + ss_hor;
                                let sb_idx: core::ffi::c_int = ((*t).by
                                    >> 5 as core::ffi::c_int) * (*f).sr_sb128w
                                    + (px_x >> 7 as core::ffi::c_int);
                                let unit_idx: core::ffi::c_int = (((*t).by
                                    & 16 as core::ffi::c_int) >> 3 as core::ffi::c_int)
                                    + ((px_x & 64 as core::ffi::c_int)
                                        >> 6 as core::ffi::c_int);
                                let lr: *mut Av1RestorationUnit = &raw mut *(*((*((*f)
                                    .lf
                                    .lr_mask)
                                    .offset(sb_idx as isize))
                                    .lr)
                                    .as_mut_ptr()
                                    .offset(p as isize))
                                    .as_mut_ptr()
                                    .offset(unit_idx as isize) as *mut Av1RestorationUnit;
                                read_restoration_info(t, lr, p, frame_type);
                                x += 1;
                            }
                        } else {
                            let x_0: core::ffi::c_int = 4 as core::ffi::c_int * (*t).bx
                                >> ss_hor;
                            if !(x_0 as core::ffi::c_uint & mask != 0) {
                                let w_0: core::ffi::c_int = (*f).cur.p.w + ss_hor >> ss_hor;
                                if !(x_0 != 0 && x_0 + half_unit > w_0) {
                                    let sb_idx_0: core::ffi::c_int = ((*t).by
                                        >> 5 as core::ffi::c_int) * (*f).sr_sb128w
                                        + ((*t).bx >> 5 as core::ffi::c_int);
                                    let unit_idx_0: core::ffi::c_int = (((*t).by
                                        & 16 as core::ffi::c_int) >> 3 as core::ffi::c_int)
                                        + (((*t).bx & 16 as core::ffi::c_int)
                                            >> 4 as core::ffi::c_int);
                                    let lr_0: *mut Av1RestorationUnit = &raw mut *(*((*((*f)
                                        .lf
                                        .lr_mask)
                                        .offset(sb_idx_0 as isize))
                                        .lr)
                                        .as_mut_ptr()
                                        .offset(p as isize))
                                        .as_mut_ptr()
                                        .offset(unit_idx_0 as isize) as *mut Av1RestorationUnit;
                                    read_restoration_info(t, lr_0, p, frame_type);
                                }
                            }
                        }
                    }
                }
            }
            p += 1;
        }
        if decode_sb(t, root_bl, dav1d_intra_edge_tree[root_bl as usize]) != 0 {
            return 1 as core::ffi::c_int;
        }
        if (*t).bx & 16 as core::ffi::c_int != 0
            || (*(*f).seq_hdr).sb128 as core::ffi::c_int != 0
        {
            (*t).a = ((*t).a).offset(1);
            (*t).lf_mask = ((*t).lf_mask).offset(1);
        }
        (*t).bx += sb_step;
    }
    if (*(*f).seq_hdr).ref_frame_mvs as core::ffi::c_int != 0
        && (*(*f).c).n_tc > 1 as core::ffi::c_uint
        && (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
            != 0
    {
        dav1d_refmvs_save_tmvs(
            &(*(*f).c).refmvs_dsp,
            &raw mut (*t).rt,
            (*ts).tiling.col_start >> 1 as core::ffi::c_int,
            (*ts).tiling.col_end >> 1 as core::ffi::c_int,
            (*t).by >> 1 as core::ffi::c_int,
            (*t).by + sb_step >> 1 as core::ffi::c_int,
        );
    }
    if (*t).frame_thread.pass != 1 as core::ffi::c_int {
        ((*f).bd_fn.backup_ipred_edge).expect("non-null function pointer")(t);
    }
    let mut align_h: core::ffi::c_int = (*f).bh + 31 as core::ffi::c_int
        & !(31 as core::ffi::c_int);
    memcpy(
        &raw mut *(*((*f).lf.tx_lpf_right_edge)
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .offset((align_h * tile_col + (*t).by) as isize) as *mut uint8_t
            as *mut core::ffi::c_void,
        &raw mut *((*t).l.tx_lpf_y)
            .as_mut_ptr()
            .offset(((*t).by & 16 as core::ffi::c_int) as isize) as *mut uint8_t
            as *const core::ffi::c_void,
        sb_step as size_t,
    );
    let ss_ver_0: core::ffi::c_int = ((*f).cur.p.layout as core::ffi::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;
    align_h >>= ss_ver_0;
    memcpy(
        &raw mut *(*((*f).lf.tx_lpf_right_edge)
            .as_ptr()
            .offset(1 as core::ffi::c_int as isize))
            .offset((align_h * tile_col + ((*t).by >> ss_ver_0)) as isize)
            as *mut uint8_t as *mut core::ffi::c_void,
        &raw mut *((*t).l.tx_lpf_uv)
            .as_mut_ptr()
            .offset((((*t).by & 16 as core::ffi::c_int) >> ss_ver_0) as isize)
            as *mut uint8_t as *const core::ffi::c_void,
        (sb_step >> ss_ver_0) as size_t,
    );
    if (*ts).msac.cnt <= -(15 as core::ffi::c_int) {
        return 1 as core::ffi::c_int;
    }
    return ((*c).strict_std_compliance != 0
        && ((*t).by >> (*f).sb_shift) + 1 as core::ffi::c_int
            >= (*(*f).frame_hdr)
                .tiling
                .row_start_sb[(tile_row + 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int
        && check_trailing_bits_after_symbol_coder(&raw mut (*ts).msac) != 0)
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_init(
    f: *mut Dav1dFrameContext,
) -> core::ffi::c_int {
    let mut sby: core::ffi::c_int = 0;
    let mut n_ts: core::ffi::c_int = 0;
    let mut a_sz: core::ffi::c_int = 0;
    let mut num_sb128: core::ffi::c_int = 0;
    let mut size_mul: *const uint8_t = 0 as *const uint8_t;
    let mut hbd: core::ffi::c_int = 0;
    let mut y_stride: ptrdiff_t = 0;
    let mut uv_stride: ptrdiff_t = 0;
    let mut has_resize: core::ffi::c_int = 0;
    let mut need_cdef_lpf_copy: core::ffi::c_int = 0;
    let mut sb128: core::ffi::c_int = 0;
    let mut num_lines: core::ffi::c_int = 0;
    let mut lr_mask_sz: core::ffi::c_int = 0;
    let mut ipred_edge_sz: core::ffi::c_int = 0;
    let mut re_sz: core::ffi::c_int = 0;
    let mut has_chroma: core::ffi::c_int = 0;
    let mut current_block: u64;
    let c: *const Dav1dContext = (*f).c;
    let mut retval: core::ffi::c_int = -(12 as core::ffi::c_int);
    if (*f).sbh > (*f).lf.start_of_tile_row_sz {
        free((*f).lf.start_of_tile_row as *mut core::ffi::c_void);
        (*f).lf.start_of_tile_row = malloc(
            ((*f).sbh as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        ) as *mut uint8_t;
        if ((*f).lf.start_of_tile_row).is_null() {
            (*f).lf.start_of_tile_row_sz = 0 as core::ffi::c_int;
            current_block = 2;
        } else {
            (*f).lf.start_of_tile_row_sz = (*f).sbh;
            current_block = 3;
        }
    } else {
        current_block = 3;
    }
    match current_block {
        3 => {
            sby = 0 as core::ffi::c_int;
            let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
            while tile_row < (*(*f).frame_hdr).tiling.rows as core::ffi::c_int {
                let fresh3 = sby;
                sby = sby + 1;
                *((*f).lf.start_of_tile_row).offset(fresh3 as isize) = tile_row
                    as uint8_t;
                while sby
                    < (*(*f).frame_hdr)
                        .tiling
                        .row_start_sb[(tile_row + 1 as core::ffi::c_int) as usize]
                        as core::ffi::c_int
                {
                    let fresh4 = sby;
                    sby = sby + 1;
                    *((*f).lf.start_of_tile_row).offset(fresh4 as isize) = 0 as uint8_t;
                }
                tile_row += 1;
            }
            n_ts = (*(*f).frame_hdr).tiling.cols as core::ffi::c_int
                * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int;
            if n_ts != (*f).n_ts {
                if (*c).n_fc > 1 as core::ffi::c_uint {
                    free((*f).frame_thread.tile_start_off as *mut core::ffi::c_void);
                    (*f).frame_thread.tile_start_off = malloc(
                        (::core::mem::size_of::<core::ffi::c_uint>() as size_t)
                            .wrapping_mul(n_ts as size_t),
                    ) as *mut core::ffi::c_uint;
                    if ((*f).frame_thread.tile_start_off).is_null() {
                        (*f).n_ts = 0 as core::ffi::c_int;
                        current_block = 2;
                    } else {
                        current_block = 4;
                    }
                } else {
                    current_block = 4;
                }
                match current_block {
                    2 => {}
                    _ => {
                        dav1d_free_aligned_internal((*f).ts as *mut core::ffi::c_void);
                        (*f).ts = dav1d_alloc_aligned_internal(
                            (::core::mem::size_of::<Dav1dTileState>() as size_t)
                                .wrapping_mul(n_ts as size_t),
                            32 as size_t,
                        ) as *mut Dav1dTileState;
                        if ((*f).ts).is_null() {
                            current_block = 2;
                        } else {
                            (*f).n_ts = n_ts;
                            current_block = 5;
                        }
                    }
                }
            } else {
                current_block = 5;
            }
            match current_block {
                2 => {}
                _ => {
                    a_sz = (*f).sb128w
                        * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int
                        * (1 as core::ffi::c_int
                            + ((*c).n_fc > 1 as core::ffi::c_uint
                                && (*c).n_tc > 1 as core::ffi::c_uint) as core::ffi::c_int);
                    if a_sz != (*f).a_sz {
                        free((*f).a as *mut core::ffi::c_void);
                        (*f).a = malloc(
                            (::core::mem::size_of::<BlockContext>() as size_t)
                                .wrapping_mul(a_sz as size_t),
                        ) as *mut BlockContext;
                        if ((*f).a).is_null() {
                            (*f).a_sz = 0 as core::ffi::c_int;
                            current_block = 2;
                        } else {
                            (*f).a_sz = a_sz;
                            current_block = 6;
                        }
                    } else {
                        current_block = 6;
                    }
                    match current_block {
                        2 => {}
                        _ => {
                            num_sb128 = (*f).sb128w * (*f).sb128h;
                            size_mul = (ss_size_mul[(*f).cur.p.layout as usize])
                                .as_ptr();
                            hbd = ((*(*f).seq_hdr).hbd != 0) as core::ffi::c_int;
                            if (*c).n_fc > 1 as core::ffi::c_uint {
                                let sb_step4: core::ffi::c_uint = ((*f).sb_step
                                    * 4 as core::ffi::c_int) as core::ffi::c_uint;
                                let mut tile_idx: core::ffi::c_int = 0 as core::ffi::c_int;
                                let mut tile_row_0: core::ffi::c_int = 0
                                    as core::ffi::c_int;
                                while tile_row_0
                                    < (*(*f).frame_hdr).tiling.rows as core::ffi::c_int
                                {
                                    let row_off: core::ffi::c_uint = ((*(*f).frame_hdr)
                                        .tiling
                                        .row_start_sb[tile_row_0 as usize] as core::ffi::c_uint)
                                        .wrapping_mul(sb_step4)
                                        .wrapping_mul((*f).sb128w as core::ffi::c_uint)
                                        .wrapping_mul(128 as core::ffi::c_uint);
                                    let b_diff: core::ffi::c_uint = (((*(*f).frame_hdr)
                                        .tiling
                                        .row_start_sb[(tile_row_0 + 1 as core::ffi::c_int) as usize]
                                        as core::ffi::c_int
                                        - (*(*f).frame_hdr).tiling.row_start_sb[tile_row_0 as usize]
                                            as core::ffi::c_int) as core::ffi::c_uint)
                                        .wrapping_mul(sb_step4);
                                    let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
                                    while tile_col
                                        < (*(*f).frame_hdr).tiling.cols as core::ffi::c_int
                                    {
                                        let fresh5 = tile_idx;
                                        tile_idx = tile_idx + 1;
                                        *((*f).frame_thread.tile_start_off)
                                            .offset(fresh5 as isize) = row_off
                                            .wrapping_add(
                                                b_diff
                                                    .wrapping_mul(
                                                        (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize]
                                                            as core::ffi::c_uint,
                                                    )
                                                    .wrapping_mul(sb_step4),
                                            );
                                        tile_col += 1;
                                    }
                                    tile_row_0 += 1;
                                }
                                let lowest_pixel_mem_sz: core::ffi::c_int = (*(*f)
                                    .frame_hdr)
                                    .tiling
                                    .cols as core::ffi::c_int * (*f).sbh;
                                if lowest_pixel_mem_sz
                                    != (*f).tile_thread.lowest_pixel_mem_sz
                                {
                                    free(
                                        (*f).tile_thread.lowest_pixel_mem as *mut core::ffi::c_void,
                                    );
                                    (*f).tile_thread.lowest_pixel_mem = malloc(
                                        (lowest_pixel_mem_sz as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<[[core::ffi::c_int; 2]; 7]>()
                                                    as size_t,
                                            ),
                                    ) as *mut [[core::ffi::c_int; 2]; 7];
                                    if ((*f).tile_thread.lowest_pixel_mem).is_null() {
                                        (*f).tile_thread.lowest_pixel_mem_sz = 0
                                            as core::ffi::c_int;
                                        current_block = 2;
                                    } else {
                                        (*f).tile_thread.lowest_pixel_mem_sz = lowest_pixel_mem_sz;
                                        current_block = 7;
                                    }
                                } else {
                                    current_block = 7;
                                }
                                match current_block {
                                    2 => {}
                                    _ => {
                                        let mut lowest_pixel_ptr: *mut [[core::ffi::c_int; 2]; 7] = (*f)
                                            .tile_thread
                                            .lowest_pixel_mem;
                                        let mut tile_row_1: core::ffi::c_int = 0
                                            as core::ffi::c_int;
                                        let mut tile_row_base: core::ffi::c_int = 0
                                            as core::ffi::c_int;
                                        while tile_row_1
                                            < (*(*f).frame_hdr).tiling.rows as core::ffi::c_int
                                        {
                                            let tile_row_sb_h: core::ffi::c_int = (*(*f).frame_hdr)
                                                .tiling
                                                .row_start_sb[(tile_row_1 + 1 as core::ffi::c_int) as usize]
                                                as core::ffi::c_int
                                                - (*(*f).frame_hdr).tiling.row_start_sb[tile_row_1 as usize]
                                                    as core::ffi::c_int;
                                            let mut tile_col_0: core::ffi::c_int = 0
                                                as core::ffi::c_int;
                                            while tile_col_0
                                                < (*(*f).frame_hdr).tiling.cols as core::ffi::c_int
                                            {
                                                let fresh6 = &raw mut (*((*f).ts)
                                                    .offset((tile_row_base + tile_col_0) as isize))
                                                    .lowest_pixel;
                                                *fresh6 = lowest_pixel_ptr;
                                                lowest_pixel_ptr = lowest_pixel_ptr
                                                    .offset(tile_row_sb_h as isize);
                                                tile_col_0 += 1;
                                            }
                                            tile_row_1 += 1;
                                            tile_row_base
                                                += (*(*f).frame_hdr).tiling.cols as core::ffi::c_int;
                                        }
                                        let cbi_sz: core::ffi::c_int = num_sb128
                                            * *size_mul.offset(0 as core::ffi::c_int as isize)
                                                as core::ffi::c_int;
                                        if cbi_sz != (*f).frame_thread.cbi_sz {
                                            dav1d_free_aligned_internal(
                                                (*f).frame_thread.cbi as *mut core::ffi::c_void,
                                            );
                                            (*f).frame_thread.cbi = dav1d_alloc_aligned_internal(
                                                (::core::mem::size_of::<int16_t>() as size_t)
                                                    .wrapping_mul(cbi_sz as size_t)
                                                    .wrapping_mul(32 as size_t)
                                                    .wrapping_mul(32 as size_t)
                                                    .wrapping_div(4 as size_t),
                                                64 as size_t,
                                            ) as *mut int16_t;
                                            if ((*f).frame_thread.cbi).is_null() {
                                                (*f).frame_thread.cbi_sz = 0 as core::ffi::c_int;
                                                current_block = 2;
                                            } else {
                                                (*f).frame_thread.cbi_sz = cbi_sz;
                                                current_block = 8;
                                            }
                                        } else {
                                            current_block = 8;
                                        }
                                        match current_block {
                                            2 => {}
                                            _ => {
                                                let cf_sz: core::ffi::c_int = (num_sb128
                                                    * *size_mul.offset(0 as core::ffi::c_int as isize)
                                                        as core::ffi::c_int) << hbd;
                                                if cf_sz != (*f).frame_thread.cf_sz {
                                                    dav1d_free_aligned_internal(
                                                        (*f).frame_thread.cf as *mut core::ffi::c_void,
                                                    );
                                                    (*f).frame_thread.cf = dav1d_alloc_aligned_internal(
                                                        (cf_sz as size_t)
                                                            .wrapping_mul(128 as size_t)
                                                            .wrapping_mul(128 as size_t)
                                                            .wrapping_div(2 as size_t),
                                                        64 as size_t,
                                                    ) as *mut core::ffi::c_void;
                                                    if ((*f).frame_thread.cf).is_null() {
                                                        (*f).frame_thread.cf_sz = 0 as core::ffi::c_int;
                                                        current_block = 2;
                                                    } else {
                                                        memset(
                                                            (*f).frame_thread.cf as *mut core::ffi::c_void,
                                                            0 as core::ffi::c_int,
                                                            (cf_sz as size_t)
                                                                .wrapping_mul(128 as size_t)
                                                                .wrapping_mul(128 as size_t)
                                                                .wrapping_div(2 as size_t),
                                                        );
                                                        (*f).frame_thread.cf_sz = cf_sz;
                                                        current_block = 9;
                                                    }
                                                } else {
                                                    current_block = 9;
                                                }
                                                match current_block {
                                                    2 => {}
                                                    _ => {
                                                        if (*(*f).frame_hdr).allow_screen_content_tools != 0 {
                                                            let pal_sz: core::ffi::c_int = num_sb128 << hbd;
                                                            if pal_sz != (*f).frame_thread.pal_sz {
                                                                dav1d_free_aligned_internal(
                                                                    (*f).frame_thread.pal as *mut core::ffi::c_void,
                                                                );
                                                                (*f).frame_thread.pal = dav1d_alloc_aligned_internal(
                                                                    (::core::mem::size_of::<[[pixel; 8]; 3]>() as size_t)
                                                                        .wrapping_mul(pal_sz as size_t)
                                                                        .wrapping_mul(16 as size_t)
                                                                        .wrapping_mul(16 as size_t),
                                                                    64 as size_t,
                                                                ) as *mut [[pixel; 8]; 3];
                                                                if ((*f).frame_thread.pal).is_null() {
                                                                    (*f).frame_thread.pal_sz = 0 as core::ffi::c_int;
                                                                    current_block = 2;
                                                                } else {
                                                                    (*f).frame_thread.pal_sz = pal_sz;
                                                                    current_block = 10;
                                                                }
                                                            } else {
                                                                current_block = 10;
                                                            }
                                                            match current_block {
                                                                2 => {}
                                                                _ => {
                                                                    let pal_idx_sz: core::ffi::c_int = num_sb128
                                                                        * *size_mul.offset(1 as core::ffi::c_int as isize)
                                                                            as core::ffi::c_int;
                                                                    if pal_idx_sz != (*f).frame_thread.pal_idx_sz {
                                                                        dav1d_free_aligned_internal(
                                                                            (*f).frame_thread.pal_idx as *mut core::ffi::c_void,
                                                                        );
                                                                        (*f).frame_thread.pal_idx = dav1d_alloc_aligned_internal(
                                                                            (::core::mem::size_of::<uint8_t>() as size_t)
                                                                                .wrapping_mul(pal_idx_sz as size_t)
                                                                                .wrapping_mul(128 as size_t)
                                                                                .wrapping_mul(128 as size_t)
                                                                                .wrapping_div(8 as size_t),
                                                                            64 as size_t,
                                                                        ) as *mut uint8_t;
                                                                        if ((*f).frame_thread.pal_idx).is_null() {
                                                                            (*f).frame_thread.pal_idx_sz = 0 as core::ffi::c_int;
                                                                            current_block = 2;
                                                                        } else {
                                                                            (*f).frame_thread.pal_idx_sz = pal_idx_sz;
                                                                            current_block = 11;
                                                                        }
                                                                    } else {
                                                                        current_block = 11;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if !((*f).frame_thread.pal).is_null() {
                                                                dav1d_freep_aligned(
                                                                    &raw mut (*f).frame_thread.pal as *mut *mut [[pixel; 8]; 3]
                                                                        as *mut core::ffi::c_void,
                                                                );
                                                                dav1d_freep_aligned(
                                                                    &raw mut (*f).frame_thread.pal_idx as *mut *mut uint8_t
                                                                        as *mut core::ffi::c_void,
                                                                );
                                                                (*f).frame_thread.pal_idx_sz = 0 as core::ffi::c_int;
                                                                (*f).frame_thread.pal_sz = (*f).frame_thread.pal_idx_sz;
                                                            }
                                                            current_block = 11;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                current_block = 11;
                            }
                            match current_block {
                                2 => {}
                                _ => {
                                    y_stride = (*f).cur.stride[0 as core::ffi::c_int as usize];
                                    uv_stride = (*f).cur.stride[1 as core::ffi::c_int as usize];
                                    has_resize = ((*(*f).frame_hdr)
                                        .width[0 as core::ffi::c_int as usize]
                                        != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize])
                                        as core::ffi::c_int;
                                    need_cdef_lpf_copy = ((*c).n_tc > 1 as core::ffi::c_uint
                                        && has_resize != 0) as core::ffi::c_int;
                                    if y_stride * (*f).sbh as ptrdiff_t * 4 as ptrdiff_t
                                        != (*f).lf.cdef_buf_plane_sz[0 as core::ffi::c_int as usize]
                                            as ptrdiff_t
                                        || uv_stride * (*f).sbh as ptrdiff_t * 8 as ptrdiff_t
                                            != (*f).lf.cdef_buf_plane_sz[1 as core::ffi::c_int as usize]
                                                as ptrdiff_t
                                        || need_cdef_lpf_copy != (*f).lf.need_cdef_lpf_copy
                                        || (*f).sbh != (*f).lf.cdef_buf_sbh
                                    {
                                        dav1d_free_aligned_internal(
                                            (*f).lf.cdef_line_buf as *mut core::ffi::c_void,
                                        );
                                        let mut alloc_sz: size_t = 64 as size_t;
                                        alloc_sz = alloc_sz
                                            .wrapping_add(
                                                (llabs(y_stride as core::ffi::c_longlong) as size_t)
                                                    .wrapping_mul(4 as size_t)
                                                    .wrapping_mul((*f).sbh as size_t) << need_cdef_lpf_copy,
                                            );
                                        alloc_sz = alloc_sz
                                            .wrapping_add(
                                                (llabs(uv_stride as core::ffi::c_longlong) as size_t)
                                                    .wrapping_mul(8 as size_t)
                                                    .wrapping_mul((*f).sbh as size_t) << need_cdef_lpf_copy,
                                            );
                                        (*f).lf.cdef_line_buf = dav1d_alloc_aligned_internal(
                                            alloc_sz,
                                            32 as size_t,
                                        ) as *mut uint8_t;
                                        let mut ptr: *mut uint8_t = (*f).lf.cdef_line_buf;
                                        if ptr.is_null() {
                                            (*f).lf.cdef_buf_plane_sz[1 as core::ffi::c_int as usize] = 0
                                                as core::ffi::c_int;
                                            (*f).lf.cdef_buf_plane_sz[0 as core::ffi::c_int as usize] = (*f)
                                                .lf
                                                .cdef_buf_plane_sz[1 as core::ffi::c_int as usize];
                                            current_block = 2;
                                        } else {
                                            ptr = ptr.offset(32 as core::ffi::c_int as isize);
                                            if y_stride < 0 as ptrdiff_t {
                                                (*f)
                                                    .lf
                                                    .cdef_line[0 as core::ffi::c_int
                                                    as usize][0 as core::ffi::c_int as usize] = ptr
                                                    .offset(
                                                        -((y_stride
                                                            * ((*f).sbh * 4 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                                as ptrdiff_t) as isize),
                                                    ) as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[1 as core::ffi::c_int
                                                    as usize][0 as core::ffi::c_int as usize] = ptr
                                                    .offset(
                                                        -((y_stride
                                                            * ((*f).sbh * 4 as core::ffi::c_int - 3 as core::ffi::c_int)
                                                                as ptrdiff_t) as isize),
                                                    ) as *mut pixel;
                                            } else {
                                                (*f)
                                                    .lf
                                                    .cdef_line[0 as core::ffi::c_int
                                                    as usize][0 as core::ffi::c_int as usize] = ptr
                                                    .offset((y_stride * 0 as ptrdiff_t) as isize) as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[1 as core::ffi::c_int
                                                    as usize][0 as core::ffi::c_int as usize] = ptr
                                                    .offset((y_stride * 2 as ptrdiff_t) as isize) as *mut pixel;
                                            }
                                            ptr = ptr
                                                .offset(
                                                    (llabs(y_stride as core::ffi::c_longlong)
                                                        * (*f).sbh as core::ffi::c_longlong
                                                        * 4 as core::ffi::c_longlong) as isize,
                                                );
                                            if uv_stride < 0 as ptrdiff_t {
                                                (*f)
                                                    .lf
                                                    .cdef_line[0 as core::ffi::c_int
                                                    as usize][1 as core::ffi::c_int as usize] = ptr
                                                    .offset(
                                                        -((uv_stride
                                                            * ((*f).sbh * 8 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                                as ptrdiff_t) as isize),
                                                    ) as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[0 as core::ffi::c_int
                                                    as usize][2 as core::ffi::c_int as usize] = ptr
                                                    .offset(
                                                        -((uv_stride
                                                            * ((*f).sbh * 8 as core::ffi::c_int - 3 as core::ffi::c_int)
                                                                as ptrdiff_t) as isize),
                                                    ) as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[1 as core::ffi::c_int
                                                    as usize][1 as core::ffi::c_int as usize] = ptr
                                                    .offset(
                                                        -((uv_stride
                                                            * ((*f).sbh * 8 as core::ffi::c_int - 5 as core::ffi::c_int)
                                                                as ptrdiff_t) as isize),
                                                    ) as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[1 as core::ffi::c_int
                                                    as usize][2 as core::ffi::c_int as usize] = ptr
                                                    .offset(
                                                        -((uv_stride
                                                            * ((*f).sbh * 8 as core::ffi::c_int - 7 as core::ffi::c_int)
                                                                as ptrdiff_t) as isize),
                                                    ) as *mut pixel;
                                            } else {
                                                (*f)
                                                    .lf
                                                    .cdef_line[0 as core::ffi::c_int
                                                    as usize][1 as core::ffi::c_int as usize] = ptr
                                                    .offset((uv_stride * 0 as ptrdiff_t) as isize)
                                                    as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[0 as core::ffi::c_int
                                                    as usize][2 as core::ffi::c_int as usize] = ptr
                                                    .offset((uv_stride * 2 as ptrdiff_t) as isize)
                                                    as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[1 as core::ffi::c_int
                                                    as usize][1 as core::ffi::c_int as usize] = ptr
                                                    .offset((uv_stride * 4 as ptrdiff_t) as isize)
                                                    as *mut pixel;
                                                (*f)
                                                    .lf
                                                    .cdef_line[1 as core::ffi::c_int
                                                    as usize][2 as core::ffi::c_int as usize] = ptr
                                                    .offset((uv_stride * 6 as ptrdiff_t) as isize)
                                                    as *mut pixel;
                                            }
                                            if need_cdef_lpf_copy != 0 {
                                                ptr = ptr
                                                    .offset(
                                                        (llabs(uv_stride as core::ffi::c_longlong)
                                                            * (*f).sbh as core::ffi::c_longlong
                                                            * 8 as core::ffi::c_longlong) as isize,
                                                    );
                                                if y_stride < 0 as ptrdiff_t {
                                                    (*f).lf.cdef_lpf_line[0 as core::ffi::c_int as usize] = ptr
                                                        .offset(
                                                            -((y_stride
                                                                * ((*f).sbh * 4 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                                    as ptrdiff_t) as isize),
                                                        ) as *mut pixel;
                                                } else {
                                                    (*f).lf.cdef_lpf_line[0 as core::ffi::c_int as usize] = ptr
                                                        as *mut pixel;
                                                }
                                                ptr = ptr
                                                    .offset(
                                                        (llabs(y_stride as core::ffi::c_longlong)
                                                            * (*f).sbh as core::ffi::c_longlong
                                                            * 4 as core::ffi::c_longlong) as isize,
                                                    );
                                                if uv_stride < 0 as ptrdiff_t {
                                                    (*f).lf.cdef_lpf_line[1 as core::ffi::c_int as usize] = ptr
                                                        .offset(
                                                            -((uv_stride
                                                                * ((*f).sbh * 4 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                                    as ptrdiff_t) as isize),
                                                        ) as *mut pixel;
                                                    (*f).lf.cdef_lpf_line[2 as core::ffi::c_int as usize] = ptr
                                                        .offset(
                                                            -((uv_stride
                                                                * ((*f).sbh * 8 as core::ffi::c_int - 1 as core::ffi::c_int)
                                                                    as ptrdiff_t) as isize),
                                                        ) as *mut pixel;
                                                } else {
                                                    (*f).lf.cdef_lpf_line[1 as core::ffi::c_int as usize] = ptr
                                                        as *mut pixel;
                                                    (*f).lf.cdef_lpf_line[2 as core::ffi::c_int as usize] = ptr
                                                        .offset(
                                                            (uv_stride * (*f).sbh as ptrdiff_t * 4 as ptrdiff_t)
                                                                as isize,
                                                        ) as *mut pixel;
                                                }
                                            }
                                            (*f).lf.cdef_buf_plane_sz[0 as core::ffi::c_int as usize] = y_stride
                                                as core::ffi::c_int * (*f).sbh * 4 as core::ffi::c_int;
                                            (*f).lf.cdef_buf_plane_sz[1 as core::ffi::c_int as usize] = uv_stride
                                                as core::ffi::c_int * (*f).sbh * 8 as core::ffi::c_int;
                                            (*f).lf.need_cdef_lpf_copy = need_cdef_lpf_copy;
                                            (*f).lf.cdef_buf_sbh = (*f).sbh;
                                            current_block = 12;
                                        }
                                    } else {
                                        current_block = 12;
                                    }
                                    match current_block {
                                        2 => {}
                                        _ => {
                                            sb128 = (*(*f).seq_hdr).sb128 as core::ffi::c_int;
                                            num_lines = if (*c).n_tc > 1 as core::ffi::c_uint {
                                                ((*f).sbh * 4 as core::ffi::c_int) << sb128
                                            } else {
                                                12 as core::ffi::c_int
                                            };
                                            y_stride = (*f)
                                                .sr_cur
                                                .p
                                                .stride[0 as core::ffi::c_int as usize];
                                            uv_stride = (*f)
                                                .sr_cur
                                                .p
                                                .stride[1 as core::ffi::c_int as usize];
                                            if y_stride * num_lines as ptrdiff_t
                                                != (*f).lf.lr_buf_plane_sz[0 as core::ffi::c_int as usize]
                                                    as ptrdiff_t
                                                || uv_stride * num_lines as ptrdiff_t * 2 as ptrdiff_t
                                                    != (*f).lf.lr_buf_plane_sz[1 as core::ffi::c_int as usize]
                                                        as ptrdiff_t
                                            {
                                                dav1d_free_aligned_internal(
                                                    (*f).lf.lr_line_buf as *mut core::ffi::c_void,
                                                );
                                                let mut alloc_sz_0: size_t = 128 as size_t;
                                                alloc_sz_0 = alloc_sz_0
                                                    .wrapping_add(
                                                        (llabs(y_stride as core::ffi::c_longlong) as size_t)
                                                            .wrapping_mul(num_lines as size_t),
                                                    );
                                                alloc_sz_0 = alloc_sz_0
                                                    .wrapping_add(
                                                        (llabs(uv_stride as core::ffi::c_longlong) as size_t)
                                                            .wrapping_mul(num_lines as size_t)
                                                            .wrapping_mul(2 as size_t),
                                                    );
                                                (*f).lf.lr_line_buf = dav1d_alloc_aligned_internal(
                                                    alloc_sz_0,
                                                    64 as size_t,
                                                ) as *mut uint8_t;
                                                let mut ptr_0: *mut uint8_t = (*f).lf.lr_line_buf;
                                                if ptr_0.is_null() {
                                                    (*f).lf.lr_buf_plane_sz[1 as core::ffi::c_int as usize] = 0
                                                        as core::ffi::c_int;
                                                    (*f).lf.lr_buf_plane_sz[0 as core::ffi::c_int as usize] = (*f)
                                                        .lf
                                                        .lr_buf_plane_sz[1 as core::ffi::c_int as usize];
                                                    current_block = 2;
                                                } else {
                                                    ptr_0 = ptr_0.offset(64 as core::ffi::c_int as isize);
                                                    if y_stride < 0 as ptrdiff_t {
                                                        (*f).lf.lr_lpf_line[0 as core::ffi::c_int as usize] = ptr_0
                                                            .offset(
                                                                -((y_stride
                                                                    * (num_lines - 1 as core::ffi::c_int) as ptrdiff_t)
                                                                    as isize),
                                                            ) as *mut pixel;
                                                    } else {
                                                        (*f).lf.lr_lpf_line[0 as core::ffi::c_int as usize] = ptr_0
                                                            as *mut pixel;
                                                    }
                                                    ptr_0 = ptr_0
                                                        .offset(
                                                            (llabs(y_stride as core::ffi::c_longlong)
                                                                * num_lines as core::ffi::c_longlong) as isize,
                                                        );
                                                    if uv_stride < 0 as ptrdiff_t {
                                                        (*f).lf.lr_lpf_line[1 as core::ffi::c_int as usize] = ptr_0
                                                            .offset(
                                                                -((uv_stride
                                                                    * (num_lines * 1 as core::ffi::c_int
                                                                        - 1 as core::ffi::c_int) as ptrdiff_t) as isize),
                                                            ) as *mut pixel;
                                                        (*f).lf.lr_lpf_line[2 as core::ffi::c_int as usize] = ptr_0
                                                            .offset(
                                                                -((uv_stride
                                                                    * (num_lines * 2 as core::ffi::c_int
                                                                        - 1 as core::ffi::c_int) as ptrdiff_t) as isize),
                                                            ) as *mut pixel;
                                                    } else {
                                                        (*f).lf.lr_lpf_line[1 as core::ffi::c_int as usize] = ptr_0
                                                            as *mut pixel;
                                                        (*f).lf.lr_lpf_line[2 as core::ffi::c_int as usize] = ptr_0
                                                            .offset((uv_stride * num_lines as ptrdiff_t) as isize)
                                                            as *mut pixel;
                                                    }
                                                    (*f).lf.lr_buf_plane_sz[0 as core::ffi::c_int as usize] = y_stride
                                                        as core::ffi::c_int * num_lines;
                                                    (*f).lf.lr_buf_plane_sz[1 as core::ffi::c_int as usize] = uv_stride
                                                        as core::ffi::c_int * num_lines * 2 as core::ffi::c_int;
                                                    current_block = 13;
                                                }
                                            } else {
                                                current_block = 13;
                                            }
                                            match current_block {
                                                2 => {}
                                                _ => {
                                                    if num_sb128 != (*f).lf.mask_sz {
                                                        free((*f).lf.mask as *mut core::ffi::c_void);
                                                        free((*f).lf.level as *mut core::ffi::c_void);
                                                        (*f).lf.mask = malloc(
                                                            (::core::mem::size_of::<Av1Filter>() as size_t)
                                                                .wrapping_mul(num_sb128 as size_t),
                                                        ) as *mut Av1Filter;
                                                        (*f).lf.level = malloc(
                                                            (::core::mem::size_of::<[uint8_t; 4]>() as size_t)
                                                                .wrapping_mul(num_sb128 as size_t)
                                                                .wrapping_mul(32 as size_t)
                                                                .wrapping_mul(32 as size_t)
                                                                .wrapping_add(3 as size_t),
                                                        ) as *mut [uint8_t; 4];
                                                        if ((*f).lf.mask).is_null() || ((*f).lf.level).is_null() {
                                                            (*f).lf.mask_sz = 0 as core::ffi::c_int;
                                                            current_block = 2;
                                                        } else {
                                                            if (*c).n_fc > 1 as core::ffi::c_uint {
                                                                free((*f).frame_thread.b as *mut core::ffi::c_void);
                                                                (*f).frame_thread.b = malloc(
                                                                    (::core::mem::size_of::<Av1Block>() as size_t)
                                                                        .wrapping_mul(num_sb128 as size_t)
                                                                        .wrapping_mul(32 as size_t)
                                                                        .wrapping_mul(32 as size_t),
                                                                ) as *mut Av1Block;
                                                                if ((*f).frame_thread.b).is_null() {
                                                                    (*f).lf.mask_sz = 0 as core::ffi::c_int;
                                                                    current_block = 2;
                                                                } else {
                                                                    current_block = 14;
                                                                }
                                                            } else {
                                                                current_block = 14;
                                                            }
                                                            match current_block {
                                                                2 => {}
                                                                _ => {
                                                                    (*f).lf.mask_sz = num_sb128;
                                                                    current_block = 15;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 15;
                                                    }
                                                    match current_block {
                                                        2 => {}
                                                        _ => {
                                                            (*f).sr_sb128w = (*f).sr_cur.p.p.w + 127 as core::ffi::c_int
                                                                >> 7 as core::ffi::c_int;
                                                            lr_mask_sz = (*f).sr_sb128w * (*f).sb128h;
                                                            if lr_mask_sz != (*f).lf.lr_mask_sz {
                                                                free((*f).lf.lr_mask as *mut core::ffi::c_void);
                                                                (*f).lf.lr_mask = malloc(
                                                                    (::core::mem::size_of::<Av1Restoration>() as size_t)
                                                                        .wrapping_mul(lr_mask_sz as size_t),
                                                                ) as *mut Av1Restoration;
                                                                if ((*f).lf.lr_mask).is_null() {
                                                                    (*f).lf.lr_mask_sz = 0 as core::ffi::c_int;
                                                                    current_block = 2;
                                                                } else {
                                                                    (*f).lf.lr_mask_sz = lr_mask_sz;
                                                                    current_block = 16;
                                                                }
                                                            } else {
                                                                current_block = 16;
                                                            }
                                                            match current_block {
                                                                2 => {}
                                                                _ => {
                                                                    (*f).lf.restore_planes = ((((*(*f).frame_hdr)
                                                                        .restoration
                                                                        .type_0[0 as core::ffi::c_int as usize] as core::ffi::c_uint
                                                                        != DAV1D_RESTORATION_NONE as core::ffi::c_int
                                                                            as core::ffi::c_uint) as core::ffi::c_int)
                                                                        << 0 as core::ffi::c_int)
                                                                        + ((((*(*f).frame_hdr)
                                                                            .restoration
                                                                            .type_0[1 as core::ffi::c_int as usize] as core::ffi::c_uint
                                                                            != DAV1D_RESTORATION_NONE as core::ffi::c_int
                                                                                as core::ffi::c_uint) as core::ffi::c_int)
                                                                            << 1 as core::ffi::c_int)
                                                                        + ((((*(*f).frame_hdr)
                                                                            .restoration
                                                                            .type_0[2 as core::ffi::c_int as usize] as core::ffi::c_uint
                                                                            != DAV1D_RESTORATION_NONE as core::ffi::c_int
                                                                                as core::ffi::c_uint) as core::ffi::c_int)
                                                                            << 2 as core::ffi::c_int);
                                                                    if (*(*f).frame_hdr).loopfilter.sharpness
                                                                        as core::ffi::c_int != (*f).lf.last_sharpness
                                                                    {
                                                                        dav1d_calc_eih(
                                                                            &raw mut (*f).lf.lim_lut,
                                                                            (*(*f).frame_hdr).loopfilter.sharpness as core::ffi::c_int,
                                                                        );
                                                                        (*f).lf.last_sharpness = (*(*f).frame_hdr)
                                                                            .loopfilter
                                                                            .sharpness as core::ffi::c_int;
                                                                    }
                                                                    dav1d_calc_lf_values(
                                                                        ((*f).lf.lvl).as_mut_ptr() as *mut [[[uint8_t; 2]; 8]; 4],
                                                                        (*f).frame_hdr,
                                                                        [
                                                                            0 as core::ffi::c_int as int8_t,
                                                                            0 as core::ffi::c_int as int8_t,
                                                                            0 as core::ffi::c_int as int8_t,
                                                                            0 as core::ffi::c_int as int8_t,
                                                                        ]
                                                                            .as_mut_ptr() as *const int8_t,
                                                                    );
                                                                    memset(
                                                                        (*f).lf.mask as *mut core::ffi::c_void,
                                                                        0 as core::ffi::c_int,
                                                                        (::core::mem::size_of::<Av1Filter>() as size_t)
                                                                            .wrapping_mul(num_sb128 as size_t),
                                                                    );
                                                                    ipred_edge_sz = (*f).sbh * (*f).sb128w << hbd;
                                                                    if ipred_edge_sz != (*f).ipred_edge_sz {
                                                                        dav1d_free_aligned_internal(
                                                                            (*f).ipred_edge[0 as core::ffi::c_int as usize]
                                                                                as *mut core::ffi::c_void,
                                                                        );
                                                                        (*f).ipred_edge[0 as core::ffi::c_int as usize] = dav1d_alloc_aligned_internal(
                                                                            (ipred_edge_sz * 128 as core::ffi::c_int
                                                                                * 3 as core::ffi::c_int) as size_t,
                                                                            64 as size_t,
                                                                        ) as *mut pixel;
                                                                        let mut ptr_1: *mut uint8_t = (*f)
                                                                            .ipred_edge[0 as core::ffi::c_int as usize];
                                                                        if ptr_1.is_null() {
                                                                            (*f).ipred_edge_sz = 0 as core::ffi::c_int;
                                                                            current_block = 2;
                                                                        } else {
                                                                            (*f).ipred_edge[1 as core::ffi::c_int as usize] = ptr_1
                                                                                .offset(
                                                                                    (ipred_edge_sz * 128 as core::ffi::c_int
                                                                                        * 1 as core::ffi::c_int) as isize,
                                                                                ) as *mut pixel;
                                                                            (*f).ipred_edge[2 as core::ffi::c_int as usize] = ptr_1
                                                                                .offset(
                                                                                    (ipred_edge_sz * 128 as core::ffi::c_int
                                                                                        * 2 as core::ffi::c_int) as isize,
                                                                                ) as *mut pixel;
                                                                            (*f).ipred_edge_sz = ipred_edge_sz;
                                                                            current_block = 17;
                                                                        }
                                                                    } else {
                                                                        current_block = 17;
                                                                    }
                                                                    match current_block {
                                                                        2 => {}
                                                                        _ => {
                                                                            re_sz = (*f).sb128h
                                                                                * (*(*f).frame_hdr).tiling.cols as core::ffi::c_int;
                                                                            if re_sz != (*f).lf.re_sz {
                                                                                free(
                                                                                    (*f).lf.tx_lpf_right_edge[0 as core::ffi::c_int as usize]
                                                                                        as *mut core::ffi::c_void,
                                                                                );
                                                                                (*f).lf.tx_lpf_right_edge[0 as core::ffi::c_int as usize] = malloc(
                                                                                    (re_sz * 32 as core::ffi::c_int * 2 as core::ffi::c_int)
                                                                                        as size_t,
                                                                                ) as *mut uint8_t;
                                                                                if ((*f)
                                                                                    .lf
                                                                                    .tx_lpf_right_edge[0 as core::ffi::c_int as usize])
                                                                                    .is_null()
                                                                                {
                                                                                    (*f).lf.re_sz = 0 as core::ffi::c_int;
                                                                                    current_block = 2;
                                                                                } else {
                                                                                    (*f).lf.tx_lpf_right_edge[1 as core::ffi::c_int as usize] = ((*f)
                                                                                        .lf
                                                                                        .tx_lpf_right_edge[0 as core::ffi::c_int as usize])
                                                                                        .offset((re_sz * 32 as core::ffi::c_int) as isize);
                                                                                    (*f).lf.re_sz = re_sz;
                                                                                    current_block = 18;
                                                                                }
                                                                            } else {
                                                                                current_block = 18;
                                                                            }
                                                                            match current_block {
                                                                                2 => {}
                                                                                _ => {
                                                                                    if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                                                                                        & 1 as core::ffi::c_uint != 0
                                                                                        || (*(*f).frame_hdr).allow_intrabc as core::ffi::c_int != 0
                                                                                    {
                                                                                        let ret: core::ffi::c_int = dav1d_refmvs_init_frame(
                                                                                            &raw mut (*f).rf,
                                                                                            (*f).seq_hdr,
                                                                                            (*f).frame_hdr,
                                                                                            ((*f).refpoc).as_mut_ptr() as *const uint8_t,
                                                                                            (*f).mvs,
                                                                                            ((*f).refrefpoc).as_mut_ptr() as *const [uint8_t; 7],
                                                                                            ((*f).ref_mvs).as_mut_ptr()
                                                                                                as *const *mut refmvs_temporal_block,
                                                                                            (*(*f).c).n_tc as core::ffi::c_int,
                                                                                            (*(*f).c).n_fc as core::ffi::c_int,
                                                                                        ) as core::ffi::c_int;
                                                                                        if ret < 0 as core::ffi::c_int {
                                                                                            current_block = 2;
                                                                                        } else {
                                                                                            current_block = 19;
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 19;
                                                                                    }
                                                                                    match current_block {
                                                                                        2 => {}
                                                                                        _ => {
                                                                                            init_quant_tables(
                                                                                                (*f).seq_hdr,
                                                                                                (*f).frame_hdr,
                                                                                                (*(*f).frame_hdr).quant.yac as core::ffi::c_int,
                                                                                                ((*f).dq).as_mut_ptr() as *mut [[uint16_t; 2]; 3],
                                                                                            );
                                                                                            if (*(*f).frame_hdr).quant.qm != 0 {
                                                                                                let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                                while i < N_RECT_TX_SIZES as core::ffi::c_int {
                                                                                                    (*f).qm[i as usize][0 as core::ffi::c_int as usize] = dav1d_qm_tbl[(*(*f)
                                                                                                        .frame_hdr)
                                                                                                        .quant
                                                                                                        .qm_y as usize][0 as core::ffi::c_int as usize][i as usize];
                                                                                                    (*f).qm[i as usize][1 as core::ffi::c_int as usize] = dav1d_qm_tbl[(*(*f)
                                                                                                        .frame_hdr)
                                                                                                        .quant
                                                                                                        .qm_u as usize][1 as core::ffi::c_int as usize][i as usize];
                                                                                                    (*f).qm[i as usize][2 as core::ffi::c_int as usize] = dav1d_qm_tbl[(*(*f)
                                                                                                        .frame_hdr)
                                                                                                        .quant
                                                                                                        .qm_v as usize][1 as core::ffi::c_int as usize][i as usize];
                                                                                                    i += 1;
                                                                                                }
                                                                                            } else {
                                                                                                memset(
                                                                                                    ((*f).qm).as_mut_ptr() as *mut core::ffi::c_void,
                                                                                                    0 as core::ffi::c_int,
                                                                                                    ::core::mem::size_of::<[[*const uint8_t; 3]; 19]>()
                                                                                                        as size_t,
                                                                                                );
                                                                                            }
                                                                                            if (*(*f).frame_hdr).switchable_comp_refs != 0 {
                                                                                                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                                                while i_0 < 7 as core::ffi::c_int {
                                                                                                    let ref0poc: core::ffi::c_uint = (*(*f)
                                                                                                        .refp[i_0 as usize]
                                                                                                        .p
                                                                                                        .frame_hdr)
                                                                                                        .frame_offset as core::ffi::c_uint;
                                                                                                    let mut j: core::ffi::c_int = i_0 + 1 as core::ffi::c_int;
                                                                                                    while j < 7 as core::ffi::c_int {
                                                                                                        let ref1poc: core::ffi::c_uint = (*(*f)
                                                                                                            .refp[j as usize]
                                                                                                            .p
                                                                                                            .frame_hdr)
                                                                                                            .frame_offset as core::ffi::c_uint;
                                                                                                        let d1: core::ffi::c_uint = imin(
                                                                                                            abs(
                                                                                                                get_poc_diff(
                                                                                                                    (*(*f).seq_hdr).order_hint_n_bits as core::ffi::c_int,
                                                                                                                    ref0poc as core::ffi::c_int,
                                                                                                                    (*(*f).cur.frame_hdr).frame_offset as core::ffi::c_int,
                                                                                                                ),
                                                                                                            ),
                                                                                                            31 as core::ffi::c_int,
                                                                                                        ) as core::ffi::c_uint;
                                                                                                        let d0: core::ffi::c_uint = imin(
                                                                                                            abs(
                                                                                                                get_poc_diff(
                                                                                                                    (*(*f).seq_hdr).order_hint_n_bits as core::ffi::c_int,
                                                                                                                    ref1poc as core::ffi::c_int,
                                                                                                                    (*(*f).cur.frame_hdr).frame_offset as core::ffi::c_int,
                                                                                                                ),
                                                                                                            ),
                                                                                                            31 as core::ffi::c_int,
                                                                                                        ) as core::ffi::c_uint;
                                                                                                        let order: core::ffi::c_int = (d0 <= d1)
                                                                                                            as core::ffi::c_int;
                                                                                                        static mut quant_dist_weight: [[uint8_t; 2]; 3] = [
                                                                                                            [
                                                                                                                2 as core::ffi::c_int as uint8_t,
                                                                                                                3 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                            [
                                                                                                                2 as core::ffi::c_int as uint8_t,
                                                                                                                5 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                            [
                                                                                                                2 as core::ffi::c_int as uint8_t,
                                                                                                                7 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                        ];
                                                                                                        static mut quant_dist_lookup_table: [[uint8_t; 2]; 4] = [
                                                                                                            [
                                                                                                                9 as core::ffi::c_int as uint8_t,
                                                                                                                7 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                            [
                                                                                                                11 as core::ffi::c_int as uint8_t,
                                                                                                                5 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                            [
                                                                                                                12 as core::ffi::c_int as uint8_t,
                                                                                                                4 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                            [
                                                                                                                13 as core::ffi::c_int as uint8_t,
                                                                                                                3 as core::ffi::c_int as uint8_t,
                                                                                                            ],
                                                                                                        ];
                                                                                                        let mut k: core::ffi::c_int = 0;
                                                                                                        k = 0 as core::ffi::c_int;
                                                                                                        while k < 3 as core::ffi::c_int {
                                                                                                            let c0: core::ffi::c_int = quant_dist_weight[k
                                                                                                                as usize][order as usize] as core::ffi::c_int;
                                                                                                            let c1: core::ffi::c_int = quant_dist_weight[k
                                                                                                                as usize][(order == 0) as core::ffi::c_int as usize]
                                                                                                                as core::ffi::c_int;
                                                                                                            let d0_c0: core::ffi::c_int = d0
                                                                                                                .wrapping_mul(c0 as core::ffi::c_uint) as core::ffi::c_int;
                                                                                                            let d1_c1: core::ffi::c_int = d1
                                                                                                                .wrapping_mul(c1 as core::ffi::c_uint) as core::ffi::c_int;
                                                                                                            if d0 > d1 && d0_c0 < d1_c1 || d0 <= d1 && d0_c0 > d1_c1 {
                                                                                                                break;
                                                                                                            }
                                                                                                            k += 1;
                                                                                                        }
                                                                                                        (*f).jnt_weights[i_0 as usize][j as usize] = quant_dist_lookup_table[k
                                                                                                            as usize][order as usize];
                                                                                                        j += 1;
                                                                                                    }
                                                                                                    i_0 += 1;
                                                                                                }
                                                                                            }
                                                                                            has_chroma = ((*f).cur.p.layout as core::ffi::c_uint
                                                                                                != DAV1D_PIXEL_LAYOUT_I400 as core::ffi::c_int
                                                                                                    as core::ffi::c_uint) as core::ffi::c_int;
                                                                                            (*f).lf.p[0 as core::ffi::c_int as usize] = (*f)
                                                                                                .cur
                                                                                                .data[0 as core::ffi::c_int as usize] as *mut pixel;
                                                                                            (*f).lf.p[1 as core::ffi::c_int as usize] = (*f)
                                                                                                .cur
                                                                                                .data[(if has_chroma != 0 {
                                                                                                1 as core::ffi::c_int
                                                                                            } else {
                                                                                                0 as core::ffi::c_int
                                                                                            }) as usize] as *mut pixel;
                                                                                            (*f).lf.p[2 as core::ffi::c_int as usize] = (*f)
                                                                                                .cur
                                                                                                .data[(if has_chroma != 0 {
                                                                                                2 as core::ffi::c_int
                                                                                            } else {
                                                                                                0 as core::ffi::c_int
                                                                                            }) as usize] as *mut pixel;
                                                                                            (*f).lf.sr_p[0 as core::ffi::c_int as usize] = (*f)
                                                                                                .sr_cur
                                                                                                .p
                                                                                                .data[0 as core::ffi::c_int as usize] as *mut pixel;
                                                                                            (*f).lf.sr_p[1 as core::ffi::c_int as usize] = (*f)
                                                                                                .sr_cur
                                                                                                .p
                                                                                                .data[(if has_chroma != 0 {
                                                                                                1 as core::ffi::c_int
                                                                                            } else {
                                                                                                0 as core::ffi::c_int
                                                                                            }) as usize] as *mut pixel;
                                                                                            (*f).lf.sr_p[2 as core::ffi::c_int as usize] = (*f)
                                                                                                .sr_cur
                                                                                                .p
                                                                                                .data[(if has_chroma != 0 {
                                                                                                2 as core::ffi::c_int
                                                                                            } else {
                                                                                                0 as core::ffi::c_int
                                                                                            }) as usize] as *mut pixel;
                                                                                            retval = 0 as core::ffi::c_int;
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
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_init_cdf(
    f: *mut Dav1dFrameContext,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let c: *const Dav1dContext = (*f).c;
    let mut retval: core::ffi::c_int = -(22 as core::ffi::c_int);
    if (*(*f).frame_hdr).refresh_context != 0 {
        dav1d_cdf_thread_copy((*f).out_cdf.data.cdf, &raw mut (*f).in_cdf);
    }
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
    (*f).task_thread.update_set = 0 as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    's_19: loop {
        if !(i < (*f).n_tile_data) {
            current_block = 20;
            break;
        }
        let mut data: *const uint8_t = (*((*f).tile).offset(i as isize)).data.data;
        let mut size: size_t = (*((*f).tile).offset(i as isize)).data.sz;
        let mut j: core::ffi::c_int = (*((*f).tile).offset(i as isize)).start;
        while j <= (*((*f).tile).offset(i as isize)).end {
            let mut tile_sz: size_t = 0;
            if j == (*((*f).tile).offset(i as isize)).end {
                tile_sz = size;
            } else {
                if (*(*f).frame_hdr).tiling.n_bytes as size_t > size {
                    current_block = 21;
                    break 's_19;
                }
                tile_sz = 0 as size_t;
                let mut k: core::ffi::c_uint = 0 as core::ffi::c_uint;
                while k < (*(*f).frame_hdr).tiling.n_bytes as core::ffi::c_uint {
                    let fresh22 = data;
                    data = data.offset(1);
                    tile_sz
                        |= ((*fresh22 as core::ffi::c_uint)
                            << k.wrapping_mul(8 as core::ffi::c_uint)) as size_t;
                    k = k.wrapping_add(1);
                }
                tile_sz = tile_sz.wrapping_add(1);
                size = size.wrapping_sub((*(*f).frame_hdr).tiling.n_bytes as size_t);
                if tile_sz > size {
                    current_block = 21;
                    break 's_19;
                }
            }
            let fresh23 = tile_col;
            tile_col = tile_col + 1;
            setup_tile(
                &raw mut *((*f).ts).offset(j as isize),
                f,
                data,
                tile_sz,
                tile_row,
                fresh23,
                if (*c).n_fc > 1 as core::ffi::c_uint {
                    *((*f).frame_thread.tile_start_off).offset(j as isize)
                } else {
                    0 as core::ffi::c_uint
                },
            );
            if tile_col == (*(*f).frame_hdr).tiling.cols as core::ffi::c_int {
                tile_col = 0 as core::ffi::c_int;
                tile_row += 1;
            }
            if j == (*(*f).frame_hdr).tiling.update as core::ffi::c_int
                && (*(*f).frame_hdr).refresh_context as core::ffi::c_int != 0
            {
                (*f).task_thread.update_set = 1 as core::ffi::c_int;
            }
            data = data.offset(tile_sz as isize);
            size = size.wrapping_sub(tile_sz);
            j += 1;
        }
        i += 1;
    }
    match current_block {
        20 => {
            if (*c).n_tc > 1 as core::ffi::c_uint {
                let uses_2pass: core::ffi::c_int = ((*c).n_fc > 1 as core::ffi::c_uint)
                    as core::ffi::c_int;
                let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
                while n
                    < (*f).sb128w * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int
                        * (1 as core::ffi::c_int + uses_2pass)
                {
                    reset_context(
                        &raw mut *((*f).a).offset(n as isize),
                        ((*(*f).frame_hdr).frame_type as core::ffi::c_uint
                            & 1 as core::ffi::c_uint == 0) as core::ffi::c_int,
                        if uses_2pass != 0 {
                            1 as core::ffi::c_int
                                + (n
                                    >= (*f).sb128w
                                        * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int)
                                    as core::ffi::c_int
                        } else {
                            0 as core::ffi::c_int
                        },
                    );
                    n += 1;
                }
            }
            retval = 0 as core::ffi::c_int;
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_main(
    f: *mut Dav1dFrameContext,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let c: *const Dav1dContext = (*f).c;
    let mut retval: core::ffi::c_int = -(22 as core::ffi::c_int);
    if !((*(*f).c).n_tc == 1 as core::ffi::c_uint) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 24],
                [core::ffi::c_char; 24],
            >(*b"dav1d_decode_frame_main\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            3203 as core::ffi::c_int,
            b"f->c->n_tc == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let t: *mut Dav1dTaskContext = &raw mut *((*c).tc)
        .offset(f.offset_from((*c).fc) as core::ffi::c_long as isize)
        as *mut Dav1dTaskContext;
    (*t).f = f;
    (*t).frame_thread.pass = 0 as core::ffi::c_int;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < (*f).sb128w * (*(*f).frame_hdr).tiling.rows as core::ffi::c_int {
        reset_context(
            &raw mut *((*f).a).offset(n as isize),
            ((*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
                == 0) as core::ffi::c_int,
            0 as core::ffi::c_int,
        );
        n += 1;
    }
    let mut tile_row: core::ffi::c_int = 0 as core::ffi::c_int;
    's_30: loop {
        if !(tile_row < (*(*f).frame_hdr).tiling.rows as core::ffi::c_int) {
            current_block = 22;
            break;
        }
        let sbh_end: core::ffi::c_int = imin(
            (*(*f).frame_hdr)
                .tiling
                .row_start_sb[(tile_row + 1 as core::ffi::c_int) as usize]
                as core::ffi::c_int,
            (*f).sbh,
        ) as core::ffi::c_int;
        let mut sby: core::ffi::c_int = (*(*f).frame_hdr)
            .tiling
            .row_start_sb[tile_row as usize] as core::ffi::c_int;
        while sby < sbh_end {
            (*t).by = sby
                << 4 as core::ffi::c_int + (*(*f).seq_hdr).sb128 as core::ffi::c_int;
            let by_end: core::ffi::c_int = (*t).by + (*f).sb_step
                >> 1 as core::ffi::c_int;
            if (*(*f).frame_hdr).use_ref_frame_mvs != 0 {
                ((*(*f).c).refmvs_dsp.load_tmvs)
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut (*f).rf,
                    tile_row,
                    0 as core::ffi::c_int,
                    (*f).bw >> 1 as core::ffi::c_int,
                    (*t).by >> 1 as core::ffi::c_int,
                    by_end,
                );
            }
            let mut tile_col: core::ffi::c_int = 0 as core::ffi::c_int;
            while tile_col < (*(*f).frame_hdr).tiling.cols as core::ffi::c_int {
                (*t).ts = &raw mut *((*f).ts)
                    .offset(
                        (tile_row * (*(*f).frame_hdr).tiling.cols as core::ffi::c_int
                            + tile_col) as isize,
                    ) as *mut Dav1dTileState;
                if dav1d_decode_tile_sbrow(t) != 0 {
                    current_block = 23;
                    break 's_30;
                }
                tile_col += 1;
            }
            if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
                != 0
            {
                dav1d_refmvs_save_tmvs(
                    &(*(*f).c).refmvs_dsp,
                    &raw mut (*t).rt,
                    0 as core::ffi::c_int,
                    (*f).bw >> 1 as core::ffi::c_int,
                    (*t).by >> 1 as core::ffi::c_int,
                    by_end,
                );
            }
            ((*f).bd_fn.filter_sbrow).expect("non-null function pointer")(f, sby);
            sby += 1;
        }
        tile_row += 1;
    }
    match current_block {
        22 => {
            retval = 0 as core::ffi::c_int;
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_exit(
    f: *mut Dav1dFrameContext,
    mut retval: core::ffi::c_int,
) {
    let c: *const Dav1dContext = (*f).c;
    if !((*f).sr_cur.p.data[0 as core::ffi::c_int as usize]).is_null() {
        *&raw mut (*f).task_thread.error = 0 as core::ffi::c_int;
    }
    if (*c).n_fc > 1 as core::ffi::c_uint && retval != 0
        && !((*f).frame_thread.cf).is_null()
    {
        memset(
            (*f).frame_thread.cf as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ((*f).frame_thread.cf_sz as size_t)
                .wrapping_mul(128 as size_t)
                .wrapping_mul(128 as size_t)
                .wrapping_div(2 as size_t),
        );
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < 7 as core::ffi::c_int {
        if !((*f).refp[i as usize].p.frame_hdr).is_null() {
            if retval == 0 && (*c).n_fc > 1 as core::ffi::c_uint
                && (*c).strict_std_compliance != 0
                && ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                    &raw mut *((*((*f).refp).as_mut_ptr().offset(i as isize)).progress)
                        .offset(1 as core::ffi::c_int as isize) as *mut atomic_uint,
                ) == FRAME_ERROR
            {
                retval = -(22 as core::ffi::c_int);
                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                    &raw mut (*f).task_thread.error,
                    1 as core::ffi::c_int,
                );
                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                    &raw mut *((*f).sr_cur.progress).offset(1 as core::ffi::c_int as isize)
                        as *mut atomic_uint,
                    (2147483647 as core::ffi::c_uint)
                        .wrapping_mul(2 as core::ffi::c_uint)
                        .wrapping_add(1 as core::ffi::c_uint)
                        .wrapping_sub(1 as core::ffi::c_uint),
                );
            }
            dav1d_thread_picture_unref(
                &raw mut *((*f).refp).as_mut_ptr().offset(i as isize),
            );
        }
        dav1d_ref_dec(&raw mut *((*f).ref_mvs_ref).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    dav1d_picture_unref_internal(&raw mut (*f).cur);
    dav1d_thread_picture_unref(&raw mut (*f).sr_cur);
    dav1d_cdf_thread_unref(&raw mut (*f).in_cdf);
    if !((*f).frame_hdr).is_null()
        && (*(*f).frame_hdr).refresh_context as core::ffi::c_int != 0
    {
        if !((*f).out_cdf.progress).is_null() {
            ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                (*f).out_cdf.progress,
                (if retval == 0 as core::ffi::c_int {
                    1 as core::ffi::c_int
                } else {
                    2147483647 as core::ffi::c_int - 1 as core::ffi::c_int
                }) as core::ffi::c_uint,
            );
        }
        dav1d_cdf_thread_unref(&raw mut (*f).out_cdf);
    }
    dav1d_ref_dec(&raw mut (*f).cur_segmap_ref);
    dav1d_ref_dec(&raw mut (*f).prev_segmap_ref);
    dav1d_ref_dec(&raw mut (*f).mvs_ref);
    dav1d_ref_dec(&raw mut (*f).seq_hdr_ref);
    dav1d_ref_dec(&raw mut (*f).frame_hdr_ref);
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < (*f).n_tile_data {
        dav1d_data_unref_internal(&raw mut (*((*f).tile).offset(i_0 as isize)).data);
        i_0 += 1;
    }
    (*f).task_thread.retval = retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame(
    f: *mut Dav1dFrameContext,
) -> core::ffi::c_int {
    if !((*(*f).c).n_fc == 1 as core::ffi::c_uint) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 19],
                [core::ffi::c_char; 19],
            >(*b"dav1d_decode_frame\0"))
                .as_ptr(),
            b"decode.c\0" as *const u8 as *const core::ffi::c_char,
            3289 as core::ffi::c_int,
            b"f->c->n_fc == 1\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut res: core::ffi::c_int = dav1d_decode_frame_init(f);
    if res == 0 {
        res = dav1d_decode_frame_init_cdf(f);
    }
    if res == 0 {
        if (*(*f).c).n_tc > 1 as core::ffi::c_uint {
            res = dav1d_task_create_tile_sbrow(
                f,
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
            );
            pthread_mutex_lock(&raw mut (*(*f).task_thread.ttd).lock);
            pthread_cond_signal(&raw mut (*(*f).task_thread.ttd).cond);
            if res == 0 {
                while (*f).task_thread.done[0 as core::ffi::c_int as usize] == 0
                    || ::core::intrinsics::atomic_load::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                        &raw mut (*f).task_thread.task_counter as *mut atomic_int,
                    ) > 0 as core::ffi::c_int
                {
                    pthread_cond_wait(
                        &raw mut (*f).task_thread.cond,
                        &raw mut (*(*f).task_thread.ttd).lock,
                    );
                }
            }
            pthread_mutex_unlock(&raw mut (*(*f).task_thread.ttd).lock);
            res = (*f).task_thread.retval;
        } else {
            res = dav1d_decode_frame_main(f);
            if res == 0 && (*(*f).frame_hdr).refresh_context as core::ffi::c_int != 0
                && (*f).task_thread.update_set != 0
            {
                dav1d_cdf_thread_update(
                    (*f).frame_hdr,
                    (*f).out_cdf.data.cdf,
                    &raw mut (*((*f).ts).offset((*(*f).frame_hdr).tiling.update as isize))
                        .cdf,
                );
            }
        }
    }
    dav1d_decode_frame_exit(f, res);
    res = (*f).task_thread.retval;
    (*f).n_tile_data = 0 as core::ffi::c_int;
    return res;
}
unsafe extern "C" fn get_upscale_x0(
    in_w: core::ffi::c_int,
    out_w: core::ffi::c_int,
    step: core::ffi::c_int,
) -> core::ffi::c_int {
    let err: core::ffi::c_int = out_w * step - (in_w << 14 as core::ffi::c_int);
    let x0: core::ffi::c_int = (-(out_w - in_w << 13 as core::ffi::c_int)
        + (out_w >> 1 as core::ffi::c_int)) / out_w + 128 as core::ffi::c_int
        - err / 2 as core::ffi::c_int;
    return x0 & 0x3fff as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_submit_frame(c: *mut Dav1dContext) -> core::ffi::c_int {
    let mut ref_coded_width: [core::ffi::c_int; 7] = [0; 7];
    let mut uses_2pass: core::ffi::c_int = 0;
    let mut cols: core::ffi::c_int = 0;
    let mut rows: core::ffi::c_int = 0;
    let mut refresh_frame_flags: core::ffi::c_uint = 0;
    let mut current_block: u64;
    let mut f: *mut Dav1dFrameContext = 0 as *mut Dav1dFrameContext;
    let mut res: core::ffi::c_int = -(1 as core::ffi::c_int);
    let mut out_delayed: *mut Dav1dThreadPicture = 0 as *mut Dav1dThreadPicture;
    if (*c).n_fc > 1 as core::ffi::c_uint {
        pthread_mutex_lock(&raw mut (*c).task_thread.lock);
        let fresh1 = (*c).frame_thread.next;
        (*c).frame_thread.next = ((*c).frame_thread.next).wrapping_add(1);
        let next: core::ffi::c_uint = fresh1;
        if (*c).frame_thread.next == (*c).n_fc {
            (*c).frame_thread.next = 0 as core::ffi::c_uint;
        }
        f = &raw mut *((*c).fc).offset(next as isize) as *mut Dav1dFrameContext;
        while (*f).n_tile_data > 0 as core::ffi::c_int {
            pthread_cond_wait(&raw mut (*f).task_thread.cond, &raw mut (*c).task_thread.lock);
        }
        out_delayed = &raw mut *((*c).frame_thread.out_delayed).offset(next as isize)
            as *mut Dav1dThreadPicture;
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
            let fresh2 = ::core::intrinsics::atomic_cxchg::<_, {core::intrinsics::AtomicOrdering::SeqCst}, {core::intrinsics::AtomicOrdering::SeqCst}>(
                &raw mut (*c).task_thread.reset_task_cur,
                *&raw mut first,
                (2147483647 as core::ffi::c_uint)
                    .wrapping_mul(2 as core::ffi::c_uint)
                    .wrapping_add(1 as core::ffi::c_uint),
            );
            *&raw mut first = fresh2.0;
            fresh2.1;
            if (*c).task_thread.cur != 0 && (*c).task_thread.cur < (*c).n_fc {
                (*c).task_thread.cur = ((*c).task_thread.cur).wrapping_sub(1);
            }
        }
        let error: core::ffi::c_int = (*f).task_thread.retval;
        if error != 0 {
            (*f).task_thread.retval = 0 as core::ffi::c_int;
            (*c).cached_error = error;
            dav1d_data_props_copy(&raw mut (*c).cached_error_props, &raw mut (*out_delayed).p.m);
            dav1d_thread_picture_unref(out_delayed);
        } else if !((*out_delayed).p.data[0 as core::ffi::c_int as usize]).is_null() {
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
        }
    } else {
        f = (*c).fc;
    }
    (*f).seq_hdr = (*c).seq_hdr;
    (*f).seq_hdr_ref = (*c).seq_hdr_ref;
    dav1d_ref_inc((*f).seq_hdr_ref);
    (*f).frame_hdr = (*c).frame_hdr;
    (*f).frame_hdr_ref = (*c).frame_hdr_ref;
    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
    (*c).frame_hdr_ref = 0 as *mut Dav1dRef;
    (*f).dsp = &raw mut *((*c).dsp).as_mut_ptr().offset((*(*f).seq_hdr).hbd as isize)
        as *mut Dav1dDSPContext;
    let bpc: core::ffi::c_int = 8 as core::ffi::c_int
        + 2 as core::ffi::c_int * (*(*f).seq_hdr).hbd as core::ffi::c_int;
    if ((*(*f).dsp).ipred.intra_pred[DC_PRED as core::ffi::c_int as usize]).is_none() {
        let dsp: *mut Dav1dDSPContext = &raw mut *((*c).dsp)
            .as_mut_ptr()
            .offset((*(*f).seq_hdr).hbd as isize) as *mut Dav1dDSPContext;
        match bpc {
            8 => {
                dav1d_cdef_dsp_init_8bpc(&raw mut (*dsp).cdef);
                dav1d_intra_pred_dsp_init_8bpc(&raw mut (*dsp).ipred);
                dav1d_itx_dsp_init_8bpc(&raw mut (*dsp).itx, bpc);
                dav1d_loop_filter_dsp_init_8bpc(&raw mut (*dsp).lf);
                dav1d_loop_restoration_dsp_init_8bpc(&raw mut (*dsp).lr, bpc);
                dav1d_mc_dsp_init_8bpc(&raw mut (*dsp).mc);
                dav1d_film_grain_dsp_init_8bpc(&raw mut (*dsp).fg);
                current_block = 24;
            }
            10 | 12 => {
                dav1d_cdef_dsp_init_16bpc(&raw mut (*dsp).cdef);
                dav1d_intra_pred_dsp_init_16bpc(&raw mut (*dsp).ipred);
                dav1d_itx_dsp_init_16bpc(&raw mut (*dsp).itx, bpc);
                dav1d_loop_filter_dsp_init_16bpc(&raw mut (*dsp).lf);
                dav1d_loop_restoration_dsp_init_16bpc(&raw mut (*dsp).lr, bpc);
                dav1d_mc_dsp_init_16bpc(&raw mut (*dsp).mc);
                dav1d_film_grain_dsp_init_16bpc(&raw mut (*dsp).fg);
                current_block = 24;
            }
            _ => {
                dav1d_log(
                    c,
                    b"Compiled without support for %d-bit decoding\n\0" as *const u8
                        as *const core::ffi::c_char,
                    8 as core::ffi::c_int
                        + 2 as core::ffi::c_int * (*(*f).seq_hdr).hbd as core::ffi::c_int,
                );
                res = -(42 as core::ffi::c_int);
                current_block = 25;
            }
        }
    } else {
        current_block = 24;
    }
    match current_block {
        24 => {
            if (*(*f).seq_hdr).hbd == 0 {
                (*f).bd_fn.recon_b_inter = Some(
                    dav1d_recon_b_inter_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            BlockSize,
                            *const Av1Block,
                        ) -> core::ffi::c_int,
                ) as recon_b_inter_fn;
                (*f).bd_fn.recon_b_intra = Some(
                    dav1d_recon_b_intra_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            BlockSize,
                            EdgeFlags,
                            *const Av1Block,
                        ) -> (),
                ) as recon_b_intra_fn;
                (*f).bd_fn.filter_sbrow = Some(
                    dav1d_filter_sbrow_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_deblock_cols = Some(
                    dav1d_filter_sbrow_deblock_cols_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_deblock_rows = Some(
                    dav1d_filter_sbrow_deblock_rows_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_cdef = Some(
                    dav1d_filter_sbrow_cdef_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                        ) -> (),
                    >;
                (*f).bd_fn.filter_sbrow_resize = Some(
                    dav1d_filter_sbrow_resize_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_lr = Some(
                    dav1d_filter_sbrow_lr_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.backup_ipred_edge = Some(
                    dav1d_backup_ipred_edge_8bpc
                        as unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
                ) as backup_ipred_edge_fn;
                (*f).bd_fn.read_coef_blocks = Some(
                    dav1d_read_coef_blocks_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            BlockSize,
                            *const Av1Block,
                        ) -> (),
                ) as read_coef_blocks_fn;
                (*f).bd_fn.copy_pal_block_y = Some(
                    dav1d_copy_pal_block_y_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as copy_pal_block_fn;
                (*f).bd_fn.copy_pal_block_uv = Some(
                    dav1d_copy_pal_block_uv_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as copy_pal_block_fn;
                (*f).bd_fn.read_pal_plane = Some(
                    dav1d_read_pal_plane_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            *mut Av1Block,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as read_pal_plane_fn;
                (*f).bd_fn.read_pal_uv = Some(
                    dav1d_read_pal_uv_8bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            *mut Av1Block,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as read_pal_uv_fn;
            } else {
                (*f).bd_fn.recon_b_inter = Some(
                    dav1d_recon_b_inter_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            BlockSize,
                            *const Av1Block,
                        ) -> core::ffi::c_int,
                ) as recon_b_inter_fn;
                (*f).bd_fn.recon_b_intra = Some(
                    dav1d_recon_b_intra_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            BlockSize,
                            EdgeFlags,
                            *const Av1Block,
                        ) -> (),
                ) as recon_b_intra_fn;
                (*f).bd_fn.filter_sbrow = Some(
                    dav1d_filter_sbrow_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_deblock_cols = Some(
                    dav1d_filter_sbrow_deblock_cols_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_deblock_rows = Some(
                    dav1d_filter_sbrow_deblock_rows_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_cdef = Some(
                    dav1d_filter_sbrow_cdef_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                        ) -> (),
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                        ) -> (),
                    >;
                (*f).bd_fn.filter_sbrow_resize = Some(
                    dav1d_filter_sbrow_resize_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.filter_sbrow_lr = Some(
                    dav1d_filter_sbrow_lr_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dFrameContext,
                            core::ffi::c_int,
                        ) -> (),
                ) as filter_sbrow_fn;
                (*f).bd_fn.backup_ipred_edge = Some(
                    dav1d_backup_ipred_edge_16bpc
                        as unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
                ) as backup_ipred_edge_fn;
                (*f).bd_fn.read_coef_blocks = Some(
                    dav1d_read_coef_blocks_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            BlockSize,
                            *const Av1Block,
                        ) -> (),
                ) as read_coef_blocks_fn;
                (*f).bd_fn.copy_pal_block_y = Some(
                    dav1d_copy_pal_block_y_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as copy_pal_block_fn;
                (*f).bd_fn.copy_pal_block_uv = Some(
                    dav1d_copy_pal_block_uv_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as copy_pal_block_fn;
                (*f).bd_fn.read_pal_plane = Some(
                    dav1d_read_pal_plane_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            *mut Av1Block,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as read_pal_plane_fn;
                (*f).bd_fn.read_pal_uv = Some(
                    dav1d_read_pal_uv_16bpc
                        as unsafe extern "C" fn(
                            *mut Dav1dTaskContext,
                            *mut Av1Block,
                            core::ffi::c_int,
                            core::ffi::c_int,
                            core::ffi::c_int,
                        ) -> (),
                ) as read_pal_uv_fn;
            }
            ref_coded_width = [0; 7];
            if (*(*f).frame_hdr).frame_type as core::ffi::c_uint & 1 as core::ffi::c_uint
                != 0
            {
                if (*(*f).frame_hdr).primary_ref_frame as core::ffi::c_int
                    != DAV1D_PRIMARY_REF_NONE
                {
                    let pri_ref: core::ffi::c_int = (*(*f).frame_hdr)
                        .refidx[(*(*f).frame_hdr).primary_ref_frame as usize]
                        as core::ffi::c_int;
                    if ((*c)
                        .refs[pri_ref as usize]
                        .p
                        .p
                        .data[0 as core::ffi::c_int as usize])
                        .is_null()
                    {
                        res = -(22 as core::ffi::c_int);
                        current_block = 25;
                    } else {
                        current_block = 26;
                    }
                } else {
                    current_block = 26;
                }
                match current_block {
                    25 => {}
                    _ => {
                        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
                        loop {
                            if !(i < 7 as core::ffi::c_int) {
                                current_block = 27;
                                break;
                            }
                            let refidx: core::ffi::c_int = (*(*f).frame_hdr)
                                .refidx[i as usize] as core::ffi::c_int;
                            if ((*c)
                                .refs[refidx as usize]
                                .p
                                .p
                                .data[0 as core::ffi::c_int as usize])
                                .is_null()
                                || ((*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                    * 2 as core::ffi::c_int)
                                    < (*c).refs[refidx as usize].p.p.p.w
                                || ((*(*f).frame_hdr).height * 2 as core::ffi::c_int)
                                    < (*c).refs[refidx as usize].p.p.p.h
                                || (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                    > (*c).refs[refidx as usize].p.p.p.w
                                        * 16 as core::ffi::c_int
                                || (*(*f).frame_hdr).height
                                    > (*c).refs[refidx as usize].p.p.p.h
                                        * 16 as core::ffi::c_int
                                || (*(*f).seq_hdr).layout as core::ffi::c_uint
                                    != (*c).refs[refidx as usize].p.p.p.layout
                                        as core::ffi::c_uint
                                || bpc != (*c).refs[refidx as usize].p.p.p.bpc
                            {
                                let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
                                while j < i {
                                    dav1d_thread_picture_unref(
                                        &raw mut *((*f).refp).as_mut_ptr().offset(j as isize),
                                    );
                                    j += 1;
                                }
                                res = -(22 as core::ffi::c_int);
                                current_block = 25;
                                break;
                            } else {
                                dav1d_thread_picture_ref(
                                    &raw mut *((*f).refp).as_mut_ptr().offset(i as isize),
                                    &raw mut (*((*c).refs).as_mut_ptr().offset(refidx as isize)).p,
                                );
                                ref_coded_width[i as usize] = (*(*c)
                                    .refs[refidx as usize]
                                    .p
                                    .p
                                    .frame_hdr)
                                    .width[0 as core::ffi::c_int as usize];
                                if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                    != (*c).refs[refidx as usize].p.p.p.w
                                    || (*(*f).frame_hdr).height
                                        != (*c).refs[refidx as usize].p.p.p.h
                                {
                                    (*f)
                                        .svc[i as usize][0 as core::ffi::c_int as usize]
                                        .scale = (((*c).refs[refidx as usize].p.p.p.w
                                        << 14 as core::ffi::c_int)
                                        + ((*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                            >> 1 as core::ffi::c_int))
                                        / (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize];
                                    (*f)
                                        .svc[i as usize][1 as core::ffi::c_int as usize]
                                        .scale = (((*c).refs[refidx as usize].p.p.p.h
                                        << 14 as core::ffi::c_int)
                                        + ((*(*f).frame_hdr).height >> 1 as core::ffi::c_int))
                                        / (*(*f).frame_hdr).height;
                                    (*f).svc[i as usize][0 as core::ffi::c_int as usize].step = (*f)
                                        .svc[i as usize][0 as core::ffi::c_int as usize]
                                        .scale + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
                                    (*f).svc[i as usize][1 as core::ffi::c_int as usize].step = (*f)
                                        .svc[i as usize][1 as core::ffi::c_int as usize]
                                        .scale + 8 as core::ffi::c_int >> 4 as core::ffi::c_int;
                                } else {
                                    (*f)
                                        .svc[i as usize][1 as core::ffi::c_int as usize]
                                        .scale = 0 as core::ffi::c_int;
                                    (*f)
                                        .svc[i as usize][0 as core::ffi::c_int as usize]
                                        .scale = (*f)
                                        .svc[i as usize][1 as core::ffi::c_int as usize]
                                        .scale;
                                }
                                (*f).gmv_warp_allowed[i as usize] = ((*(*f).frame_hdr)
                                    .gmv[i as usize]
                                    .type_0 as core::ffi::c_uint
                                    > DAV1D_WM_TYPE_TRANSLATION as core::ffi::c_int
                                        as core::ffi::c_uint
                                    && (*(*f).frame_hdr).force_integer_mv == 0
                                    && dav1d_get_shear_params(
                                        &raw mut *((*(*f).frame_hdr).gmv)
                                            .as_mut_ptr()
                                            .offset(i as isize),
                                    ) == 0
                                    && (*f)
                                        .svc[i as usize][0 as core::ffi::c_int as usize]
                                        .scale == 0) as core::ffi::c_int as uint8_t;
                                i += 1;
                            }
                        }
                    }
                }
            } else {
                current_block = 27;
            }
            match current_block {
                25 => {}
                _ => {
                    if (*(*f).frame_hdr).primary_ref_frame as core::ffi::c_int
                        == DAV1D_PRIMARY_REF_NONE
                    {
                        dav1d_cdf_thread_init_static(
                            &raw mut (*f).in_cdf,
                            (*(*f).frame_hdr).quant.yac as core::ffi::c_uint,
                        );
                    } else {
                        let pri_ref_0: core::ffi::c_int = (*(*f).frame_hdr)
                            .refidx[(*(*f).frame_hdr).primary_ref_frame as usize]
                            as core::ffi::c_int;
                        dav1d_cdf_thread_ref(
                            &raw mut (*f).in_cdf,
                            &raw mut *((*c).cdf).as_mut_ptr().offset(pri_ref_0 as isize),
                        );
                    }
                    if (*(*f).frame_hdr).refresh_context != 0 {
                        res = dav1d_cdf_thread_alloc(
                            c,
                            &raw mut (*f).out_cdf,
                            ((*c).n_fc > 1 as core::ffi::c_uint) as core::ffi::c_int,
                        );
                        if res < 0 as core::ffi::c_int {
                            current_block = 25;
                        } else {
                            current_block = 28;
                        }
                    } else {
                        current_block = 28;
                    }
                    match current_block {
                        25 => {}
                        _ => {
                            if (*f).n_tile_data_alloc < (*c).n_tile_data {
                                free((*f).tile as *mut core::ffi::c_void);
                                if !((*c).n_tile_data
                                    < 2147483647 as core::ffi::c_int
                                        / ::core::mem::size_of::<Dav1dTileGroup>()
                                            as core::ffi::c_int) as core::ffi::c_int
                                    as core::ffi::c_long != 0
                                {
                                    crate::platform::assertion(
                                        (::core::mem::transmute::<
                                            [u8; 19],
                                            [core::ffi::c_char; 19],
                                        >(*b"dav1d_submit_frame\0"))
                                            .as_ptr(),
                                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                                        3509 as core::ffi::c_int,
                                        b"c->n_tile_data < INT_MAX / (int)sizeof(*f->tile)\0"
                                            as *const u8 as *const core::ffi::c_char,
                                    );
                                } else {};
                                (*f).tile = malloc(
                                    ((*c).n_tile_data as size_t)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<Dav1dTileGroup>() as size_t,
                                        ),
                                ) as *mut Dav1dTileGroup;
                                if ((*f).tile).is_null() {
                                    (*f).n_tile_data = 0 as core::ffi::c_int;
                                    (*f).n_tile_data_alloc = (*f).n_tile_data;
                                    res = -(12 as core::ffi::c_int);
                                    current_block = 25;
                                } else {
                                    (*f).n_tile_data_alloc = (*c).n_tile_data;
                                    current_block = 29;
                                }
                            } else {
                                current_block = 29;
                            }
                            match current_block {
                                25 => {}
                                _ => {
                                    memcpy(
                                        (*f).tile as *mut core::ffi::c_void,
                                        (*c).tile as *const core::ffi::c_void,
                                        ((*c).n_tile_data as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<Dav1dTileGroup>() as size_t,
                                            ),
                                    );
                                    memset(
                                        (*c).tile as *mut core::ffi::c_void,
                                        0 as core::ffi::c_int,
                                        ((*c).n_tile_data as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<Dav1dTileGroup>() as size_t,
                                            ),
                                    );
                                    (*f).n_tile_data = (*c).n_tile_data;
                                    (*c).n_tile_data = 0 as core::ffi::c_int;
                                    res = dav1d_thread_picture_alloc(c, f, bpc);
                                    if !(res < 0 as core::ffi::c_int) {
                                        if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                            != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
                                        {
                                            res = dav1d_picture_alloc_copy(
                                                c,
                                                &raw mut (*f).cur,
                                                (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize],
                                                &raw mut (*f).sr_cur.p,
                                            );
                                            if res < 0 as core::ffi::c_int {
                                                current_block = 25;
                                            } else {
                                                current_block = 30;
                                            }
                                        } else {
                                            dav1d_picture_ref(&raw mut (*f).cur, &raw mut (*f).sr_cur.p);
                                            current_block = 30;
                                        }
                                        match current_block {
                                            25 => {}
                                            _ => {
                                                if (*(*f).frame_hdr).width[0 as core::ffi::c_int as usize]
                                                    != (*(*f).frame_hdr).width[1 as core::ffi::c_int as usize]
                                                {
                                                    (*f).resize_step[0 as core::ffi::c_int as usize] = (((*f)
                                                        .cur
                                                        .p
                                                        .w << 14 as core::ffi::c_int)
                                                        + ((*f).sr_cur.p.p.w >> 1 as core::ffi::c_int))
                                                        / (*f).sr_cur.p.p.w;
                                                    let ss_hor: core::ffi::c_int = ((*f).cur.p.layout
                                                        as core::ffi::c_uint
                                                        != DAV1D_PIXEL_LAYOUT_I444 as core::ffi::c_int
                                                            as core::ffi::c_uint) as core::ffi::c_int;
                                                    let in_cw: core::ffi::c_int = (*f).cur.p.w + ss_hor
                                                        >> ss_hor;
                                                    let out_cw: core::ffi::c_int = (*f).sr_cur.p.p.w + ss_hor
                                                        >> ss_hor;
                                                    (*f).resize_step[1 as core::ffi::c_int as usize] = ((in_cw
                                                        << 14 as core::ffi::c_int)
                                                        + (out_cw >> 1 as core::ffi::c_int)) / out_cw;
                                                    (*f).resize_start[0 as core::ffi::c_int as usize] = get_upscale_x0(
                                                        (*f).cur.p.w,
                                                        (*f).sr_cur.p.p.w,
                                                        (*f).resize_step[0 as core::ffi::c_int as usize],
                                                    );
                                                    (*f).resize_start[1 as core::ffi::c_int as usize] = get_upscale_x0(
                                                        in_cw,
                                                        out_cw,
                                                        (*f).resize_step[1 as core::ffi::c_int as usize],
                                                    );
                                                }
                                                if (*c).n_fc == 1 as core::ffi::c_uint {
                                                    if (*(*f).frame_hdr).show_frame as core::ffi::c_int != 0
                                                        || (*c).output_invisible_frames != 0
                                                    {
                                                        dav1d_thread_picture_ref(&raw mut (*c).out, &raw mut (*f).sr_cur);
                                                        (*c).event_flags = ::core::mem::transmute::<
                                                            core::ffi::c_uint,
                                                            Dav1dEventFlags,
                                                        >(
                                                            (*c).event_flags as core::ffi::c_uint
                                                                | dav1d_picture_get_event_flags(&raw mut (*f).sr_cur)
                                                                    as core::ffi::c_uint,
                                                        );
                                                    }
                                                } else {
                                                    dav1d_thread_picture_ref(out_delayed, &raw mut (*f).sr_cur);
                                                }
                                                (*f).w4 = (*(*f).frame_hdr)
                                                    .width[0 as core::ffi::c_int as usize]
                                                    + 3 as core::ffi::c_int >> 2 as core::ffi::c_int;
                                                (*f).h4 = (*(*f).frame_hdr).height + 3 as core::ffi::c_int
                                                    >> 2 as core::ffi::c_int;
                                                (*f).bw = ((*(*f).frame_hdr)
                                                    .width[0 as core::ffi::c_int as usize]
                                                    + 7 as core::ffi::c_int >> 3 as core::ffi::c_int)
                                                    << 1 as core::ffi::c_int;
                                                (*f).bh = ((*(*f).frame_hdr).height + 7 as core::ffi::c_int
                                                    >> 3 as core::ffi::c_int) << 1 as core::ffi::c_int;
                                                (*f).sb128w = (*f).bw + 31 as core::ffi::c_int
                                                    >> 5 as core::ffi::c_int;
                                                (*f).sb128h = (*f).bh + 31 as core::ffi::c_int
                                                    >> 5 as core::ffi::c_int;
                                                (*f).sb_shift = 4 as core::ffi::c_int
                                                    + (*(*f).seq_hdr).sb128 as core::ffi::c_int;
                                                (*f).sb_step = (16 as core::ffi::c_int)
                                                    << (*(*f).seq_hdr).sb128 as core::ffi::c_int;
                                                (*f).sbh = (*f).bh + (*f).sb_step - 1 as core::ffi::c_int
                                                    >> (*f).sb_shift;
                                                (*f).b4_stride = ((*f).bw + 31 as core::ffi::c_int
                                                    & !(31 as core::ffi::c_int)) as ptrdiff_t;
                                                (*f).bitdepth_max = ((1 as core::ffi::c_int)
                                                    << (*f).cur.p.bpc) - 1 as core::ffi::c_int;
                                                *&raw mut (*f).task_thread.error = 0 as core::ffi::c_int;
                                                uses_2pass = ((*c).n_fc > 1 as core::ffi::c_uint)
                                                    as core::ffi::c_int;
                                                cols = (*(*f).frame_hdr).tiling.cols as core::ffi::c_int;
                                                rows = (*(*f).frame_hdr).tiling.rows as core::ffi::c_int;
                                                ::core::intrinsics::atomic_store::<_, {core::intrinsics::AtomicOrdering::SeqCst}, false>(
                                                    &raw mut (*f).task_thread.task_counter,
                                                    cols * rows + (*f).sbh << uses_2pass,
                                                );
                                                if (*(*f).frame_hdr).frame_type as core::ffi::c_uint
                                                    & 1 as core::ffi::c_uint != 0
                                                    || (*(*f).frame_hdr).allow_intrabc as core::ffi::c_int != 0
                                                {
                                                    (*f).mvs_ref = dav1d_ref_create_using_pool(
                                                        (*c).refmvs_pool,
                                                        (::core::mem::size_of::<refmvs_temporal_block>() as size_t)
                                                            .wrapping_mul((*f).sb128h as size_t)
                                                            .wrapping_mul(16 as size_t)
                                                            .wrapping_mul(
                                                                ((*f).b4_stride >> 1 as core::ffi::c_int) as size_t,
                                                            ),
                                                    );
                                                    if ((*f).mvs_ref).is_null() {
                                                        res = -(12 as core::ffi::c_int);
                                                        current_block = 25;
                                                    } else {
                                                        (*f).mvs = (*(*f).mvs_ref).data
                                                            as *mut refmvs_temporal_block;
                                                        if (*(*f).frame_hdr).allow_intrabc == 0 {
                                                            let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                                                            while i_0 < 7 as core::ffi::c_int {
                                                                (*f).refpoc[i_0 as usize] = (*(*f)
                                                                    .refp[i_0 as usize]
                                                                    .p
                                                                    .frame_hdr)
                                                                    .frame_offset;
                                                                i_0 += 1;
                                                            }
                                                        } else {
                                                            memset(
                                                                ((*f).refpoc).as_mut_ptr() as *mut core::ffi::c_void,
                                                                0 as core::ffi::c_int,
                                                                ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
                                                            );
                                                        }
                                                        if (*(*f).frame_hdr).use_ref_frame_mvs != 0 {
                                                            let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
                                                            while i_1 < 7 as core::ffi::c_int {
                                                                let refidx_0: core::ffi::c_int = (*(*f).frame_hdr)
                                                                    .refidx[i_1 as usize] as core::ffi::c_int;
                                                                let ref_w: core::ffi::c_int = (ref_coded_width[i_1 as usize]
                                                                    + 7 as core::ffi::c_int >> 3 as core::ffi::c_int)
                                                                    << 1 as core::ffi::c_int;
                                                                let ref_h: core::ffi::c_int = ((*f).refp[i_1 as usize].p.p.h
                                                                    + 7 as core::ffi::c_int >> 3 as core::ffi::c_int)
                                                                    << 1 as core::ffi::c_int;
                                                                if !((*c).refs[refidx_0 as usize].refmvs).is_null()
                                                                    && ref_w == (*f).bw && ref_h == (*f).bh
                                                                {
                                                                    (*f).ref_mvs_ref[i_1 as usize] = (*c)
                                                                        .refs[refidx_0 as usize]
                                                                        .refmvs;
                                                                    dav1d_ref_inc((*f).ref_mvs_ref[i_1 as usize]);
                                                                    (*f).ref_mvs[i_1 as usize] = (*(*c)
                                                                        .refs[refidx_0 as usize]
                                                                        .refmvs)
                                                                        .data as *mut refmvs_temporal_block;
                                                                } else {
                                                                    (*f).ref_mvs[i_1 as usize] = 0
                                                                        as *mut refmvs_temporal_block;
                                                                    (*f).ref_mvs_ref[i_1 as usize] = 0 as *mut Dav1dRef;
                                                                }
                                                                memcpy(
                                                                    ((*f).refrefpoc[i_1 as usize]).as_mut_ptr()
                                                                        as *mut core::ffi::c_void,
                                                                    ((*c).refs[refidx_0 as usize].refpoc).as_mut_ptr()
                                                                        as *const core::ffi::c_void,
                                                                    ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
                                                                );
                                                                i_1 += 1;
                                                            }
                                                        } else {
                                                            memset(
                                                                ((*f).ref_mvs_ref).as_mut_ptr() as *mut core::ffi::c_void,
                                                                0 as core::ffi::c_int,
                                                                ::core::mem::size_of::<[*mut Dav1dRef; 7]>() as size_t,
                                                            );
                                                        }
                                                        current_block = 31;
                                                    }
                                                } else {
                                                    (*f).mvs_ref = 0 as *mut Dav1dRef;
                                                    memset(
                                                        ((*f).ref_mvs_ref).as_mut_ptr() as *mut core::ffi::c_void,
                                                        0 as core::ffi::c_int,
                                                        ::core::mem::size_of::<[*mut Dav1dRef; 7]>() as size_t,
                                                    );
                                                    current_block = 31;
                                                }
                                                match current_block {
                                                    25 => {}
                                                    _ => {
                                                        if (*(*f).frame_hdr).segmentation.enabled != 0 {
                                                            (*f).prev_segmap_ref = 0 as *mut Dav1dRef;
                                                            (*f).prev_segmap = 0 as *const uint8_t;
                                                            if (*(*f).frame_hdr).segmentation.temporal
                                                                as core::ffi::c_int != 0
                                                                || (*(*f).frame_hdr).segmentation.update_map == 0
                                                            {
                                                                let pri_ref_1: core::ffi::c_int = (*(*f).frame_hdr)
                                                                    .primary_ref_frame as core::ffi::c_int;
                                                                if !(pri_ref_1 != 7 as core::ffi::c_int) as core::ffi::c_int
                                                                    as core::ffi::c_long != 0
                                                                {
                                                                    crate::platform::assertion(
                                                                        (::core::mem::transmute::<
                                                                            [u8; 19],
                                                                            [core::ffi::c_char; 19],
                                                                        >(*b"dav1d_submit_frame\0"))
                                                                            .as_ptr(),
                                                                        b"decode.c\0" as *const u8 as *const core::ffi::c_char,
                                                                        3624 as core::ffi::c_int,
                                                                        b"pri_ref != DAV1D_PRIMARY_REF_NONE\0" as *const u8
                                                                            as *const core::ffi::c_char,
                                                                    );
                                                                } else {};
                                                                let ref_w_0: core::ffi::c_int = (ref_coded_width[pri_ref_1
                                                                    as usize] + 7 as core::ffi::c_int >> 3 as core::ffi::c_int)
                                                                    << 1 as core::ffi::c_int;
                                                                let ref_h_0: core::ffi::c_int = ((*f)
                                                                    .refp[pri_ref_1 as usize]
                                                                    .p
                                                                    .p
                                                                    .h + 7 as core::ffi::c_int >> 3 as core::ffi::c_int)
                                                                    << 1 as core::ffi::c_int;
                                                                if ref_w_0 == (*f).bw && ref_h_0 == (*f).bh {
                                                                    (*f).prev_segmap_ref = (*c)
                                                                        .refs[(*(*f).frame_hdr).refidx[pri_ref_1 as usize] as usize]
                                                                        .segmap;
                                                                    if !((*f).prev_segmap_ref).is_null() {
                                                                        dav1d_ref_inc((*f).prev_segmap_ref);
                                                                        (*f).prev_segmap = (*(*f).prev_segmap_ref).data
                                                                            as *const uint8_t;
                                                                    }
                                                                }
                                                            }
                                                            if (*(*f).frame_hdr).segmentation.update_map != 0 {
                                                                (*f).cur_segmap_ref = dav1d_ref_create_using_pool(
                                                                    (*c).segmap_pool,
                                                                    (::core::mem::size_of::<uint8_t>() as size_t)
                                                                        .wrapping_mul((*f).b4_stride as size_t)
                                                                        .wrapping_mul(32 as size_t)
                                                                        .wrapping_mul((*f).sb128h as size_t),
                                                                );
                                                                if ((*f).cur_segmap_ref).is_null() {
                                                                    dav1d_ref_dec(&raw mut (*f).prev_segmap_ref);
                                                                    res = -(12 as core::ffi::c_int);
                                                                    current_block = 25;
                                                                } else {
                                                                    (*f).cur_segmap = (*(*f).cur_segmap_ref).data
                                                                        as *mut uint8_t;
                                                                    current_block = 32;
                                                                }
                                                            } else if !((*f).prev_segmap_ref).is_null() {
                                                                (*f).cur_segmap_ref = (*f).prev_segmap_ref;
                                                                dav1d_ref_inc((*f).cur_segmap_ref);
                                                                (*f).cur_segmap = (*(*f).prev_segmap_ref).data
                                                                    as *mut uint8_t;
                                                                current_block = 32;
                                                            } else {
                                                                let segmap_size: size_t = (::core::mem::size_of::<uint8_t>()
                                                                    as size_t)
                                                                    .wrapping_mul((*f).b4_stride as size_t)
                                                                    .wrapping_mul(32 as size_t)
                                                                    .wrapping_mul((*f).sb128h as size_t);
                                                                (*f).cur_segmap_ref = dav1d_ref_create_using_pool(
                                                                    (*c).segmap_pool,
                                                                    segmap_size,
                                                                );
                                                                if ((*f).cur_segmap_ref).is_null() {
                                                                    res = -(12 as core::ffi::c_int);
                                                                    current_block = 25;
                                                                } else {
                                                                    (*f).cur_segmap = (*(*f).cur_segmap_ref).data
                                                                        as *mut uint8_t;
                                                                    memset(
                                                                        (*f).cur_segmap as *mut core::ffi::c_void,
                                                                        0 as core::ffi::c_int,
                                                                        segmap_size,
                                                                    );
                                                                    current_block = 32;
                                                                }
                                                            }
                                                        } else {
                                                            (*f).cur_segmap = 0 as *mut uint8_t;
                                                            (*f).cur_segmap_ref = 0 as *mut Dav1dRef;
                                                            (*f).prev_segmap_ref = 0 as *mut Dav1dRef;
                                                            current_block = 32;
                                                        }
                                                        match current_block {
                                                            25 => {}
                                                            _ => {
                                                                refresh_frame_flags = (*(*f).frame_hdr).refresh_frame_flags
                                                                    as core::ffi::c_uint;
                                                                let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                while i_2 < 8 as core::ffi::c_int {
                                                                    if refresh_frame_flags
                                                                        & ((1 as core::ffi::c_int) << i_2) as core::ffi::c_uint != 0
                                                                    {
                                                                        if !((*c).refs[i_2 as usize].p.p.frame_hdr).is_null() {
                                                                            dav1d_thread_picture_unref(
                                                                                &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).p,
                                                                            );
                                                                        }
                                                                        dav1d_thread_picture_ref(
                                                                            &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).p,
                                                                            &raw mut (*f).sr_cur,
                                                                        );
                                                                        dav1d_cdf_thread_unref(
                                                                            &raw mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                        );
                                                                        if (*(*f).frame_hdr).refresh_context != 0 {
                                                                            dav1d_cdf_thread_ref(
                                                                                &raw mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                                &raw mut (*f).out_cdf,
                                                                            );
                                                                        } else {
                                                                            dav1d_cdf_thread_ref(
                                                                                &raw mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                                &raw mut (*f).in_cdf,
                                                                            );
                                                                        }
                                                                        dav1d_ref_dec(
                                                                            &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).segmap,
                                                                        );
                                                                        (*c).refs[i_2 as usize].segmap = (*f).cur_segmap_ref;
                                                                        if !((*f).cur_segmap_ref).is_null() {
                                                                            dav1d_ref_inc((*f).cur_segmap_ref);
                                                                        }
                                                                        dav1d_ref_dec(
                                                                            &raw mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).refmvs,
                                                                        );
                                                                        if (*(*f).frame_hdr).allow_intrabc == 0 {
                                                                            (*c).refs[i_2 as usize].refmvs = (*f).mvs_ref;
                                                                            if !((*f).mvs_ref).is_null() {
                                                                                dav1d_ref_inc((*f).mvs_ref);
                                                                            }
                                                                        }
                                                                        memcpy(
                                                                            ((*c).refs[i_2 as usize].refpoc).as_mut_ptr()
                                                                                as *mut core::ffi::c_void,
                                                                            ((*f).refpoc).as_mut_ptr() as *const core::ffi::c_void,
                                                                            ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
                                                                        );
                                                                    }
                                                                    i_2 += 1;
                                                                }
                                                                if (*c).n_fc == 1 as core::ffi::c_uint {
                                                                    res = dav1d_decode_frame(f);
                                                                    if res < 0 as core::ffi::c_int {
                                                                        dav1d_thread_picture_unref(&raw mut (*c).out);
                                                                        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
                                                                        while i_3 < 8 as core::ffi::c_int {
                                                                            if refresh_frame_flags
                                                                                & ((1 as core::ffi::c_int) << i_3) as core::ffi::c_uint != 0
                                                                            {
                                                                                if !((*c).refs[i_3 as usize].p.p.frame_hdr).is_null() {
                                                                                    dav1d_thread_picture_unref(
                                                                                        &raw mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).p,
                                                                                    );
                                                                                }
                                                                                dav1d_cdf_thread_unref(
                                                                                    &raw mut *((*c).cdf).as_mut_ptr().offset(i_3 as isize),
                                                                                );
                                                                                dav1d_ref_dec(
                                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).segmap,
                                                                                );
                                                                                dav1d_ref_dec(
                                                                                    &raw mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).refmvs,
                                                                                );
                                                                            }
                                                                            i_3 += 1;
                                                                        }
                                                                        current_block = 25;
                                                                    } else {
                                                                        current_block = 33;
                                                                    }
                                                                } else {
                                                                    dav1d_task_frame_init(f);
                                                                    pthread_mutex_unlock(&raw mut (*c).task_thread.lock);
                                                                    current_block = 33;
                                                                }
                                                                match current_block {
                                                                    25 => {}
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
        }
        _ => {}
    }
    *&raw mut (*f).task_thread.error = 1 as core::ffi::c_int;
    dav1d_cdf_thread_unref(&raw mut (*f).in_cdf);
    if (*(*f).frame_hdr).refresh_context != 0 {
        dav1d_cdf_thread_unref(&raw mut (*f).out_cdf);
    }
    let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_4 < 7 as core::ffi::c_int {
        if !((*f).refp[i_4 as usize].p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(
                &raw mut *((*f).refp).as_mut_ptr().offset(i_4 as isize),
            );
        }
        dav1d_ref_dec(&raw mut *((*f).ref_mvs_ref).as_mut_ptr().offset(i_4 as isize));
        i_4 += 1;
    }
    if (*c).n_fc == 1 as core::ffi::c_uint {
        dav1d_thread_picture_unref(&raw mut (*c).out);
    } else {
        dav1d_thread_picture_unref(out_delayed);
    }
    dav1d_picture_unref_internal(&raw mut (*f).cur);
    dav1d_thread_picture_unref(&raw mut (*f).sr_cur);
    dav1d_ref_dec(&raw mut (*f).mvs_ref);
    dav1d_ref_dec(&raw mut (*f).seq_hdr_ref);
    dav1d_ref_dec(&raw mut (*f).frame_hdr_ref);
    dav1d_data_props_copy(&raw mut (*c).cached_error_props, &raw mut (*c).in_0.m);
    let mut i_5: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_5 < (*f).n_tile_data {
        dav1d_data_unref_internal(&raw mut (*((*f).tile).offset(i_5 as isize)).data);
        i_5 += 1;
    }
    (*f).n_tile_data = 0 as core::ffi::c_int;
    if (*c).n_fc > 1 as core::ffi::c_uint {
        pthread_mutex_unlock(&raw mut (*c).task_thread.lock);
    }
    return res;
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const INT_MIN: core::ffi::c_int = -__INT_MAX__ - 1 as core::ffi::c_int;
pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2 as core::ffi::c_uint)
    .wrapping_add(1 as core::ffi::c_uint);
