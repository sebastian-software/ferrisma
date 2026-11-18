// Copyright © 2018, VideoLAN and dav1d authors
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `upstream/dav1d/src/arm/looprestoration.h` as compiled for AArch64: the
//! C side of the self-guided filters (`sgr_filter_5x5_neon`,
//! `sgr_filter_3x3_neon`, `sgr_filter_mix_neon` with their helpers
//! `rotate_neon`, `rotate5_x2_neon`, `sgr_box{3,5}_vert_neon`,
//! `sgr_box3_hv_neon`, `sgr_finish{1,2,_mix}_neon`), which keeps the rings
//! of row pointers and calls the assembly's row kernels, and
//! `loop_restoration_dsp_init_arm`: under `DAV1D_ARM_CPU_FLAG_NEON` the two
//! `wiener` slots get the rebuilt kernels at every bit depth, and the three
//! `sgr` slots get the wrappers at 8 bpc and, at 16 bpc, only for 10-bit
//! pictures (`BITDEPTH == 8 || bpc == 10`; 12-bit keeps the C kernels, as
//! in the original). With the flag masked off the C kernels stay. The
//! header's `wiener_filter_neon` and the row kernels declared under
//! `ARCH_ARM` belong to the 32-bit build and have no counterpart here.
//!
//! The wrappers are translated statement for statement; the C `goto` tails
//! (`vert_1`, `vert_2`, `odd`, `output_1`, `output_2`) are a `match` on the
//! label the main part leaves through. `params->sgr.{s0,s1,w0,w1}` are read
//! once at the top.
//!
//! One deviation, for memory safety: the row buffers (`sumsq*_buf`,
//! `sum*_buf`, `A*_buf`, `B*_buf`) and the two `tmp` buffers of
//! `sgr_finish_mix_neon`, which `ALIGN_STK_16` leaves uninitialized, are
//! zero-initialized arrays. The kernels
//! work in blocks of 8 or 16 elements and load up to the block's end behind
//! the range an earlier kernel has written; zeroed whole, no load can find
//! an element without a value, whatever the kernels load. The `tmp` buffers
//! are declared in `sgr_filter_mix_neon` and passed to
//! `sgr_finish_mix_neon`, so that they are zeroed once per call instead of
//! once per pair of rows. All buffers are reached through raw pointers
//! only.
use core::ffi::{c_int, c_uint, c_void};
use core::ptr::{null, null_mut};

use super::super::arm64::looprestoration_common::{dav1d_sgr_box3_vert_neon, dav1d_sgr_box5_vert_neon};
use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};

// `enum LrEdgeFlags` (src/looprestoration.h).
#[allow(dead_code)]
const LR_HAVE_LEFT: c_uint = 1 << 0;
#[allow(dead_code)]
const LR_HAVE_RIGHT: c_uint = 1 << 1;
const LR_HAVE_TOP: c_uint = 1 << 2;
const LR_HAVE_BOTTOM: c_uint = 1 << 3;

/// The `sgr` member of `LooprestorationParams`.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SgrParams {
    pub s0: u32,
    pub s1: u32,
    pub w0: i16,
    pub w1: i16,
}

/// `LooprestorationParams` (src/looprestoration.h).
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub union LooprestorationParams {
    pub filter: [[i16; 8]; 2],
    pub sgr: SgrParams,
}

/// `looprestorationfilter_fn` (src/looprestoration.h) at 8 bpc.
pub type LrFilterFn =
    unsafe extern "C" fn(*mut u8, isize, *const [u8; 4], *const u8, c_int, c_int, *const LooprestorationParams, c_uint);

/// `Dav1dLoopRestorationDSPContext` (src/looprestoration.h) at 8 bpc.
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [Option<LrFilterFn>; 2],
    pub sgr: [Option<LrFilterFn>; 3],
}

/// `looprestorationfilter_fn` at 16 bpc (with `bitdepth_max`).
pub type LrFilterFn16 = unsafe extern "C" fn(
    *mut u16,
    isize,
    *const [u16; 4],
    *const u16,
    c_int,
    c_int,
    *const LooprestorationParams,
    c_uint,
    c_int,
);

/// `Dav1dLoopRestorationDSPContext` at 16 bpc.
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext16 {
    pub wiener: [Option<LrFilterFn16>; 2],
    pub sgr: [Option<LrFilterFn16>; 3],
}

/// `ALIGN_STK_16`
#[repr(C, align(16))]
struct Align16<T>(T);

const ARM_FILTER_OUT_STRIDE: usize = 384;
const ARM_BUF_STRIDE: usize = 384 + 16;

/// `rotate_neon`
#[inline(always)]
fn rotate_neon(sumsq_ptrs: &mut [*mut i32], sum_ptrs: &mut [*mut i16], n: usize) {
    let tmp32 = sumsq_ptrs[0];
    let tmp16 = sum_ptrs[0];
    for i in 0..n - 1 {
        sumsq_ptrs[i] = sumsq_ptrs[i + 1];
        sum_ptrs[i] = sum_ptrs[i + 1];
    }
    sumsq_ptrs[n - 1] = tmp32;
    sum_ptrs[n - 1] = tmp16;
}

/// `rotate5_x2_neon`
#[inline(always)]
fn rotate5_x2_neon(sumsq_ptrs: &mut [*mut i32; 5], sum_ptrs: &mut [*mut i16; 5]) {
    let mut tmp32 = [null_mut::<i32>(); 2];
    let mut tmp16 = [null_mut::<i16>(); 2];
    for i in 0..2 {
        tmp32[i] = sumsq_ptrs[i];
        tmp16[i] = sum_ptrs[i];
    }
    for i in 0..3 {
        sumsq_ptrs[i] = sumsq_ptrs[i + 2];
        sum_ptrs[i] = sum_ptrs[i + 2];
    }
    for i in 0..2 {
        sumsq_ptrs[3 + i] = tmp32[i];
        sum_ptrs[3 + i] = tmp16[i];
    }
}

/// `sgr_box3_vert_neon`
///
/// # Safety
/// As for the kernel it calls: three readable rows, a writable output row.
#[inline(always)]
unsafe fn sgr_box3_vert_neon(
    sumsq: &mut [*mut i32; 3],
    sum: &mut [*mut i16; 3],
    sumsq_out: *mut i32,
    sum_out: *mut i16,
    w: c_int,
    s: c_int,
    bitdepth_max: c_int,
) {
    // box3_v + calc_ab1
    dav1d_sgr_box3_vert_neon(
        sumsq.as_mut_ptr(),
        sum.as_mut_ptr(),
        sumsq_out,
        sum_out,
        w,
        s,
        bitdepth_max,
    );
    rotate_neon(sumsq, sum, 3);
}

/// `sgr_box5_vert_neon`
///
/// # Safety
/// As for the kernel it calls: five readable rows, a writable output row.
#[inline(always)]
unsafe fn sgr_box5_vert_neon(
    sumsq: &mut [*mut i32; 5],
    sum: &mut [*mut i16; 5],
    sumsq_out: *mut i32,
    sum_out: *mut i16,
    w: c_int,
    s: c_int,
    bitdepth_max: c_int,
) {
    // box5_v + calc_ab2
    dav1d_sgr_box5_vert_neon(
        sumsq.as_mut_ptr(),
        sum.as_mut_ptr(),
        sumsq_out,
        sum_out,
        w,
        s,
        bitdepth_max,
    );
    rotate5_x2_neon(sumsq, sum);
}

/// The bit-depth dependent part of `src/arm/looprestoration.h` for one bit
/// depth. `$pixel` is `pixel`; `$bitdepth_max` is `BITDEPTH_MAX` (`0xff` at
/// 8 bpc, the parameter at 16 bpc); `$bd` is `HIGHBD_DECL_SUFFIX` /
/// `HIGHBD_TAIL_SUFFIX` (nothing at 8 bpc, `bitdepth_max` at 16 bpc). The
/// kernel names and `pxstride` (`PXSTRIDE`) are those of the invoking module
/// (`sgr_box3_row_h` is `dav1d_sgr_box3_row_h_<bpc>bpc_neon`, and so on).
macro_rules! sgr_wrappers {
    ($pixel:ty, $bitdepth_max:expr $(, $bd:ident)?) => {
        /// `sgr_box3_hv_neon`
        #[inline(always)]
        unsafe fn sgr_box3_hv_neon(
            sumsq: &mut [*mut i32; 3],
            sum: &mut [*mut i16; 3],
            aa: *mut i32,
            bb: *mut i16,
            left: *const [$pixel; 4],
            src: *const $pixel,
            w: c_int,
            s: c_int,
            edges: c_uint,
            bitdepth_max: c_int,
        ) {
            sgr_box3_row_h(sumsq[2], sum[2], left, src, w, edges);
            sgr_box3_vert_neon(sumsq, sum, aa, bb, w, s, bitdepth_max);
        }

        /// `sgr_finish1_neon`
        #[inline(always)]
        unsafe fn sgr_finish1_neon(
            dst: &mut *mut $pixel,
            stride: isize,
            a_ptrs: &mut [*mut i32; 3],
            b_ptrs: &mut [*mut i16; 3],
            w: c_int,
            w1: c_int,
            $($bd: c_int,)?
        ) {
            sgr_finish_weighted1(*dst, a_ptrs.as_mut_ptr(), b_ptrs.as_mut_ptr(), w, w1 $(, $bd)?);
            *dst = dst.wrapping_offset(pxstride(stride));
            rotate_neon(a_ptrs, b_ptrs, 3);
        }

        /// `sgr_finish2_neon`
        #[inline(always)]
        unsafe fn sgr_finish2_neon(
            dst: &mut *mut $pixel,
            stride: isize,
            a_ptrs: &mut [*mut i32; 2],
            b_ptrs: &mut [*mut i16; 2],
            w: c_int,
            h: c_int,
            w1: c_int,
            $($bd: c_int,)?
        ) {
            sgr_finish_weighted2(*dst, stride, a_ptrs.as_mut_ptr(), b_ptrs.as_mut_ptr(), w, h, w1 $(, $bd)?);
            *dst = dst.wrapping_offset(2 * pxstride(stride));
            rotate_neon(a_ptrs, b_ptrs, 2);
        }

        /// `sgr_finish_mix_neon`
        #[inline(always)]
        unsafe fn sgr_finish_mix_neon(
            dst: &mut *mut $pixel,
            stride: isize,
            a5_ptrs: &mut [*mut i32; 2],
            b5_ptrs: &mut [*mut i16; 2],
            a3_ptrs: &mut [*mut i32; 4],
            b3_ptrs: &mut [*mut i16; 4],
            w: c_int,
            h: c_int,
            w0: c_int,
            w1: c_int,
            tmp5: *mut i16,
            tmp3: *mut i16,
            $($bd: c_int,)?
        ) {
            sgr_finish_filter2_2rows(tmp5, *dst, stride, a5_ptrs.as_mut_ptr(), b5_ptrs.as_mut_ptr(), w, h);
            sgr_finish_filter1_2rows(tmp3, *dst, stride, a3_ptrs.as_mut_ptr(), b3_ptrs.as_mut_ptr(), w, h);
            let wt: [i16; 2] = [w0 as i16, w1 as i16];
            sgr_weighted2(*dst, stride, tmp5, tmp3, w, h, wt.as_ptr() $(, $bd)?);
            *dst = dst.wrapping_offset(h as isize * pxstride(stride));
            rotate_neon(a5_ptrs, b5_ptrs, 2);
            rotate_neon(a3_ptrs, b3_ptrs, 4);
        }

        /// `sgr_filter_3x3_neon`
        #[allow(unused_assignments)]
        pub(super) unsafe extern "C" fn sgr_filter_3x3_neon(
            mut dst: *mut $pixel,
            stride: isize,
            mut left: *const [$pixel; 4],
            mut lpf: *const $pixel,
            w: c_int,
            mut h: c_int,
            params: *const LooprestorationParams,
            edges: c_uint,
            $($bd: c_int,)?
        ) {
            let s1 = (*params).sgr.s1 as c_int;
            let w1 = (*params).sgr.w1 as c_int;
            // Zeroed whole (uninitialized in the original): see the module documentation.
            let mut sumsq_buf = Align16([0i32; ARM_BUF_STRIDE * 3 + 16]);
            let mut sum_buf = Align16([0i16; ARM_BUF_STRIDE * 3 + 16]);
            let sumsq_buf = sumsq_buf.0.as_mut_ptr();
            let sum_buf = sum_buf.0.as_mut_ptr();
            let mut sumsq_ptrs = [null_mut::<i32>(); 3];
            let mut sumsq_rows = [null_mut::<i32>(); 3];
            let mut sum_ptrs = [null_mut::<i16>(); 3];
            let mut sum_rows = [null_mut::<i16>(); 3];
            for i in 0..3 {
                sumsq_rows[i] = sumsq_buf.add(i * ARM_BUF_STRIDE);
                sum_rows[i] = sum_buf.add(i * ARM_BUF_STRIDE);
            }

            let mut a_buf = Align16([0i32; ARM_BUF_STRIDE * 3 + 16]);
            let mut b_buf = Align16([0i16; ARM_BUF_STRIDE * 3 + 16]);
            let a_buf = a_buf.0.as_mut_ptr();
            let b_buf = b_buf.0.as_mut_ptr();
            let mut a_ptrs = [null_mut::<i32>(); 3];
            let mut b_ptrs = [null_mut::<i16>(); 3];
            for i in 0..3 {
                a_ptrs[i] = a_buf.add(i * ARM_BUF_STRIDE);
                b_ptrs[i] = b_buf.add(i * ARM_BUF_STRIDE);
            }
            let mut src: *const $pixel = dst;
            let mut lpf_bottom = lpf.wrapping_offset(6 * pxstride(stride));

            enum Tail {
                Vert1,
                Vert2,
            }
            let tail = 'body: {
                if edges & LR_HAVE_TOP != 0 {
                    sumsq_ptrs[0] = sumsq_rows[0];
                    sumsq_ptrs[1] = sumsq_rows[1];
                    sumsq_ptrs[2] = sumsq_rows[2];
                    sum_ptrs[0] = sum_rows[0];
                    sum_ptrs[1] = sum_rows[1];
                    sum_ptrs[2] = sum_rows[2];

                    sgr_box3_row_h(sumsq_rows[0], sum_rows[0], null(), lpf, w, edges);
                    lpf = lpf.wrapping_offset(pxstride(stride));
                    sgr_box3_row_h(sumsq_rows[1], sum_rows[1], null(), lpf, w, edges);

                    sgr_box3_hv_neon(
                        &mut sumsq_ptrs,
                        &mut sum_ptrs,
                        a_ptrs[2],
                        b_ptrs[2],
                        left,
                        src,
                        w,
                        s1,
                        edges,
                        $bitdepth_max,
                    );
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 3);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert1;
                    }

                    sgr_box3_hv_neon(
                        &mut sumsq_ptrs,
                        &mut sum_ptrs,
                        a_ptrs[2],
                        b_ptrs[2],
                        left,
                        src,
                        w,
                        s1,
                        edges,
                        $bitdepth_max,
                    );
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 3);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }
                } else {
                    sumsq_ptrs[0] = sumsq_rows[0];
                    sumsq_ptrs[1] = sumsq_rows[0];
                    sumsq_ptrs[2] = sumsq_rows[0];
                    sum_ptrs[0] = sum_rows[0];
                    sum_ptrs[1] = sum_rows[0];
                    sum_ptrs[2] = sum_rows[0];

                    sgr_box3_row_h(sumsq_rows[0], sum_rows[0], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box3_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[2], b_ptrs[2], w, s1, $bitdepth_max);
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 3);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert1;
                    }

                    sumsq_ptrs[2] = sumsq_rows[1];
                    sum_ptrs[2] = sum_rows[1];

                    sgr_box3_hv_neon(
                        &mut sumsq_ptrs,
                        &mut sum_ptrs,
                        a_ptrs[2],
                        b_ptrs[2],
                        left,
                        src,
                        w,
                        s1,
                        edges,
                        $bitdepth_max,
                    );
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 3);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    sumsq_ptrs[2] = sumsq_rows[2];
                    sum_ptrs[2] = sum_rows[2];
                }

                loop {
                    sgr_box3_hv_neon(
                        &mut sumsq_ptrs,
                        &mut sum_ptrs,
                        a_ptrs[2],
                        b_ptrs[2],
                        left,
                        src,
                        w,
                        s1,
                        edges,
                        $bitdepth_max,
                    );
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_finish1_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, w1 $(, $bd)?);
                    h -= 1;
                    if h <= 0 {
                        break;
                    }
                }

                if edges & LR_HAVE_BOTTOM == 0 {
                    break 'body Tail::Vert2;
                }

                sgr_box3_hv_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    null(),
                    lpf_bottom,
                    w,
                    s1,
                    edges,
                    $bitdepth_max,
                );
                lpf_bottom = lpf_bottom.wrapping_offset(pxstride(stride));

                sgr_finish1_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, w1 $(, $bd)?);

                sgr_box3_hv_neon(
                    &mut sumsq_ptrs,
                    &mut sum_ptrs,
                    a_ptrs[2],
                    b_ptrs[2],
                    null(),
                    lpf_bottom,
                    w,
                    s1,
                    edges,
                    $bitdepth_max,
                );

                sgr_finish1_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, w1 $(, $bd)?);
                return;
            };

            match tail {
                // vert_2:
                Tail::Vert2 => {
                    sumsq_ptrs[2] = sumsq_ptrs[1];
                    sum_ptrs[2] = sum_ptrs[1];
                    sgr_box3_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[2], b_ptrs[2], w, s1, $bitdepth_max);

                    sgr_finish1_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, w1 $(, $bd)?);
                }
                // vert_1:
                Tail::Vert1 => {
                    sumsq_ptrs[2] = sumsq_ptrs[1];
                    sum_ptrs[2] = sum_ptrs[1];
                    sgr_box3_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[2], b_ptrs[2], w, s1, $bitdepth_max);
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 3);
                }
            }

            // output_1:
            sumsq_ptrs[2] = sumsq_ptrs[1];
            sum_ptrs[2] = sum_ptrs[1];
            sgr_box3_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[2], b_ptrs[2], w, s1, $bitdepth_max);

            sgr_finish1_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, w1 $(, $bd)?);
        }

        /// `sgr_filter_5x5_neon`
        #[allow(unused_assignments)]
        pub(super) unsafe extern "C" fn sgr_filter_5x5_neon(
            mut dst: *mut $pixel,
            stride: isize,
            mut left: *const [$pixel; 4],
            mut lpf: *const $pixel,
            w: c_int,
            mut h: c_int,
            params: *const LooprestorationParams,
            edges: c_uint,
            $($bd: c_int,)?
        ) {
            let s0 = (*params).sgr.s0 as c_int;
            let w0 = (*params).sgr.w0 as c_int;
            // Zeroed whole (uninitialized in the original): see the module documentation.
            let mut sumsq_buf = Align16([0i32; ARM_BUF_STRIDE * 5 + 16]);
            let mut sum_buf = Align16([0i16; ARM_BUF_STRIDE * 5 + 16]);
            let sumsq_buf = sumsq_buf.0.as_mut_ptr();
            let sum_buf = sum_buf.0.as_mut_ptr();
            let mut sumsq_ptrs = [null_mut::<i32>(); 5];
            let mut sumsq_rows = [null_mut::<i32>(); 5];
            let mut sum_ptrs = [null_mut::<i16>(); 5];
            let mut sum_rows = [null_mut::<i16>(); 5];
            for i in 0..5 {
                sumsq_rows[i] = sumsq_buf.add(i * ARM_BUF_STRIDE);
                sum_rows[i] = sum_buf.add(i * ARM_BUF_STRIDE);
            }

            let mut a_buf = Align16([0i32; ARM_BUF_STRIDE * 2 + 16]);
            let mut b_buf = Align16([0i16; ARM_BUF_STRIDE * 2 + 16]);
            let a_buf = a_buf.0.as_mut_ptr();
            let b_buf = b_buf.0.as_mut_ptr();
            let mut a_ptrs = [null_mut::<i32>(); 2];
            let mut b_ptrs = [null_mut::<i16>(); 2];
            for i in 0..2 {
                a_ptrs[i] = a_buf.add(i * ARM_BUF_STRIDE);
                b_ptrs[i] = b_buf.add(i * ARM_BUF_STRIDE);
            }
            let mut src: *const $pixel = dst;
            let mut lpf_bottom = lpf.wrapping_offset(6 * pxstride(stride));

            enum Tail {
                Vert1,
                Vert2,
                Odd,
                Output2,
            }
            let tail = 'body: {
                if edges & LR_HAVE_TOP != 0 {
                    sumsq_ptrs[0] = sumsq_rows[0];
                    sumsq_ptrs[1] = sumsq_rows[0];
                    sumsq_ptrs[2] = sumsq_rows[1];
                    sumsq_ptrs[3] = sumsq_rows[2];
                    sumsq_ptrs[4] = sumsq_rows[3];
                    sum_ptrs[0] = sum_rows[0];
                    sum_ptrs[1] = sum_rows[0];
                    sum_ptrs[2] = sum_rows[1];
                    sum_ptrs[3] = sum_rows[2];
                    sum_ptrs[4] = sum_rows[3];

                    sgr_box5_row_h(sumsq_rows[0], sum_rows[0], null(), lpf, w, edges);
                    lpf = lpf.wrapping_offset(pxstride(stride));
                    sgr_box5_row_h(sumsq_rows[1], sum_rows[1], null(), lpf, w, edges);

                    sgr_box5_row_h(sumsq_rows[2], sum_rows[2], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert1;
                    }

                    sgr_box5_row_h(sumsq_rows[3], sum_rows[3], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));
                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 2);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                    // one of them to point at the previously unused rows[4].
                    sumsq_ptrs[3] = sumsq_rows[4];
                    sum_ptrs[3] = sum_rows[4];
                } else {
                    sumsq_ptrs[0] = sumsq_rows[0];
                    sumsq_ptrs[1] = sumsq_rows[0];
                    sumsq_ptrs[2] = sumsq_rows[0];
                    sumsq_ptrs[3] = sumsq_rows[0];
                    sumsq_ptrs[4] = sumsq_rows[0];
                    sum_ptrs[0] = sum_rows[0];
                    sum_ptrs[1] = sum_rows[0];
                    sum_ptrs[2] = sum_rows[0];
                    sum_ptrs[3] = sum_rows[0];
                    sum_ptrs[4] = sum_rows[0];

                    sgr_box5_row_h(sumsq_rows[0], sum_rows[0], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert1;
                    }

                    sumsq_ptrs[4] = sumsq_rows[1];
                    sum_ptrs[4] = sum_rows[1];

                    sgr_box5_row_h(sumsq_rows[1], sum_rows[1], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 2);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    sumsq_ptrs[3] = sumsq_rows[2];
                    sumsq_ptrs[4] = sumsq_rows[3];
                    sum_ptrs[3] = sum_rows[2];
                    sum_ptrs[4] = sum_rows[3];

                    sgr_box5_row_h(sumsq_rows[2], sum_rows[2], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Odd;
                    }

                    sgr_box5_row_h(sumsq_rows[3], sum_rows[3], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    sgr_finish2_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, 2, w0 $(, $bd)?);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                    // one of them to point at the previously unused rows[4].
                    sumsq_ptrs[3] = sumsq_rows[4];
                    sum_ptrs[3] = sum_rows[4];
                }

                loop {
                    sgr_box5_row_h(sumsq_ptrs[3], sum_ptrs[3], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Odd;
                    }

                    sgr_box5_row_h(sumsq_ptrs[4], sum_ptrs[4], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    sgr_finish2_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, 2, w0 $(, $bd)?);
                    h -= 1;
                    if h <= 0 {
                        break;
                    }
                }

                if edges & LR_HAVE_BOTTOM == 0 {
                    break 'body Tail::Vert2;
                }

                sgr_box5_row_h(sumsq_ptrs[3], sum_ptrs[3], null(), lpf_bottom, w, edges);
                lpf_bottom = lpf_bottom.wrapping_offset(pxstride(stride));
                sgr_box5_row_h(sumsq_ptrs[4], sum_ptrs[4], null(), lpf_bottom, w, edges);

                Tail::Output2
            };

            match tail {
                Tail::Vert2 | Tail::Output2 => {
                    if let Tail::Vert2 = tail {
                        // vert_2:
                        // Duplicate the last row twice more
                        sumsq_ptrs[3] = sumsq_ptrs[2];
                        sumsq_ptrs[4] = sumsq_ptrs[2];
                        sum_ptrs[3] = sum_ptrs[2];
                        sum_ptrs[4] = sum_ptrs[2];
                    }
                    // output_2:
                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    sgr_finish2_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, 2, w0 $(, $bd)?);
                    return;
                }
                // odd:
                Tail::Odd => {
                    // Copy the last row as padding once
                    sumsq_ptrs[4] = sumsq_ptrs[3];
                    sum_ptrs[4] = sum_ptrs[3];

                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    sgr_finish2_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, 2, w0 $(, $bd)?);
                }
                // vert_1:
                Tail::Vert1 => {
                    // Copy the last row as padding once
                    sumsq_ptrs[4] = sumsq_ptrs[3];
                    sum_ptrs[4] = sum_ptrs[3];

                    sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
                    rotate_neon(&mut a_ptrs, &mut b_ptrs, 2);
                }
            }

            // output_1:
            // Duplicate the last row twice more
            sumsq_ptrs[3] = sumsq_ptrs[2];
            sumsq_ptrs[4] = sumsq_ptrs[2];
            sum_ptrs[3] = sum_ptrs[2];
            sum_ptrs[4] = sum_ptrs[2];

            sgr_box5_vert_neon(&mut sumsq_ptrs, &mut sum_ptrs, a_ptrs[1], b_ptrs[1], w, s0, $bitdepth_max);
            // Output only one row
            sgr_finish2_neon(&mut dst, stride, &mut a_ptrs, &mut b_ptrs, w, 1, w0 $(, $bd)?);
        }

        /// `sgr_filter_mix_neon`
        #[allow(unused_assignments)]
        pub(super) unsafe extern "C" fn sgr_filter_mix_neon(
            mut dst: *mut $pixel,
            stride: isize,
            mut left: *const [$pixel; 4],
            mut lpf: *const $pixel,
            w: c_int,
            mut h: c_int,
            params: *const LooprestorationParams,
            edges: c_uint,
            $($bd: c_int,)?
        ) {
            let s0 = (*params).sgr.s0 as c_int;
            let s1 = (*params).sgr.s1 as c_int;
            let w0 = (*params).sgr.w0 as c_int;
            let w1 = (*params).sgr.w1 as c_int;
            // Zeroed whole (uninitialized in the original): see the module documentation.
            let mut sumsq5_buf = Align16([0i32; ARM_BUF_STRIDE * 5 + 16]);
            let mut sum5_buf = Align16([0i16; ARM_BUF_STRIDE * 5 + 16]);
            let sumsq5_buf = sumsq5_buf.0.as_mut_ptr();
            let sum5_buf = sum5_buf.0.as_mut_ptr();
            let mut sumsq5_ptrs = [null_mut::<i32>(); 5];
            let mut sumsq5_rows = [null_mut::<i32>(); 5];
            let mut sum5_ptrs = [null_mut::<i16>(); 5];
            let mut sum5_rows = [null_mut::<i16>(); 5];
            for i in 0..5 {
                sumsq5_rows[i] = sumsq5_buf.add(i * ARM_BUF_STRIDE);
                sum5_rows[i] = sum5_buf.add(i * ARM_BUF_STRIDE);
            }
            let mut sumsq3_buf = Align16([0i32; ARM_BUF_STRIDE * 3 + 16]);
            let mut sum3_buf = Align16([0i16; ARM_BUF_STRIDE * 3 + 16]);
            let sumsq3_buf = sumsq3_buf.0.as_mut_ptr();
            let sum3_buf = sum3_buf.0.as_mut_ptr();
            let mut sumsq3_ptrs = [null_mut::<i32>(); 3];
            let mut sumsq3_rows = [null_mut::<i32>(); 3];
            let mut sum3_ptrs = [null_mut::<i16>(); 3];
            let mut sum3_rows = [null_mut::<i16>(); 3];
            for i in 0..3 {
                sumsq3_rows[i] = sumsq3_buf.add(i * ARM_BUF_STRIDE);
                sum3_rows[i] = sum3_buf.add(i * ARM_BUF_STRIDE);
            }

            let mut a5_buf = Align16([0i32; ARM_BUF_STRIDE * 2 + 16]);
            let mut b5_buf = Align16([0i16; ARM_BUF_STRIDE * 2 + 16]);
            let a5_buf = a5_buf.0.as_mut_ptr();
            let b5_buf = b5_buf.0.as_mut_ptr();
            let mut a5_ptrs = [null_mut::<i32>(); 2];
            let mut b5_ptrs = [null_mut::<i16>(); 2];
            for i in 0..2 {
                a5_ptrs[i] = a5_buf.add(i * ARM_BUF_STRIDE);
                b5_ptrs[i] = b5_buf.add(i * ARM_BUF_STRIDE);
            }
            let mut a3_buf = Align16([0i32; ARM_BUF_STRIDE * 4 + 16]);
            let mut b3_buf = Align16([0i16; ARM_BUF_STRIDE * 4 + 16]);
            let a3_buf = a3_buf.0.as_mut_ptr();
            let b3_buf = b3_buf.0.as_mut_ptr();
            let mut a3_ptrs = [null_mut::<i32>(); 4];
            let mut b3_ptrs = [null_mut::<i16>(); 4];
            for i in 0..4 {
                a3_ptrs[i] = a3_buf.add(i * ARM_BUF_STRIDE);
                b3_ptrs[i] = b3_buf.add(i * ARM_BUF_STRIDE);
            }
            // `tmp5` and `tmp3` of `sgr_finish_mix_neon` (two rows of `ARM_FILTER_OUT_STRIDE` each), which the
            // original declares there: here, so that they are zeroed once per call and not once per pair of rows.
            let mut tmp5 = Align16([0i16; 2 * ARM_FILTER_OUT_STRIDE]);
            let mut tmp3 = Align16([0i16; 2 * ARM_FILTER_OUT_STRIDE]);
            let tmp5 = tmp5.0.as_mut_ptr();
            let tmp3 = tmp3.0.as_mut_ptr();
            let mut src: *const $pixel = dst;
            let mut lpf_bottom = lpf.wrapping_offset(6 * pxstride(stride));

            enum Tail {
                Vert1,
                Vert2,
                Odd,
                Output2,
            }
            let tail = 'body: {
                if edges & LR_HAVE_TOP != 0 {
                    sumsq5_ptrs[0] = sumsq5_rows[0];
                    sumsq5_ptrs[1] = sumsq5_rows[0];
                    sumsq5_ptrs[2] = sumsq5_rows[1];
                    sumsq5_ptrs[3] = sumsq5_rows[2];
                    sumsq5_ptrs[4] = sumsq5_rows[3];
                    sum5_ptrs[0] = sum5_rows[0];
                    sum5_ptrs[1] = sum5_rows[0];
                    sum5_ptrs[2] = sum5_rows[1];
                    sum5_ptrs[3] = sum5_rows[2];
                    sum5_ptrs[4] = sum5_rows[3];

                    sumsq3_ptrs[0] = sumsq3_rows[0];
                    sumsq3_ptrs[1] = sumsq3_rows[1];
                    sumsq3_ptrs[2] = sumsq3_rows[2];
                    sum3_ptrs[0] = sum3_rows[0];
                    sum3_ptrs[1] = sum3_rows[1];
                    sum3_ptrs[2] = sum3_rows[2];

                    sgr_box35_row_h(sumsq3_rows[0], sum3_rows[0], sumsq5_rows[0], sum5_rows[0], null(), lpf, w, edges);
                    lpf = lpf.wrapping_offset(pxstride(stride));
                    sgr_box35_row_h(sumsq3_rows[1], sum3_rows[1], sumsq5_rows[1], sum5_rows[1], null(), lpf, w, edges);

                    sgr_box35_row_h(sumsq3_rows[2], sum3_rows[2], sumsq5_rows[2], sum5_rows[2], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert1;
                    }

                    sgr_box35_row_h(sumsq3_ptrs[2], sum3_ptrs[2], sumsq5_rows[3], sum5_rows[3], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));
                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    rotate_neon(&mut a5_ptrs, &mut b5_ptrs, 2);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                    // one of them to point at the previously unused rows[4].
                    sumsq5_ptrs[3] = sumsq5_rows[4];
                    sum5_ptrs[3] = sum5_rows[4];
                } else {
                    sumsq5_ptrs[0] = sumsq5_rows[0];
                    sumsq5_ptrs[1] = sumsq5_rows[0];
                    sumsq5_ptrs[2] = sumsq5_rows[0];
                    sumsq5_ptrs[3] = sumsq5_rows[0];
                    sumsq5_ptrs[4] = sumsq5_rows[0];
                    sum5_ptrs[0] = sum5_rows[0];
                    sum5_ptrs[1] = sum5_rows[0];
                    sum5_ptrs[2] = sum5_rows[0];
                    sum5_ptrs[3] = sum5_rows[0];
                    sum5_ptrs[4] = sum5_rows[0];

                    sumsq3_ptrs[0] = sumsq3_rows[0];
                    sumsq3_ptrs[1] = sumsq3_rows[0];
                    sumsq3_ptrs[2] = sumsq3_rows[0];
                    sum3_ptrs[0] = sum3_rows[0];
                    sum3_ptrs[1] = sum3_rows[0];
                    sum3_ptrs[2] = sum3_rows[0];

                    sgr_box35_row_h(sumsq3_rows[0], sum3_rows[0], sumsq5_rows[0], sum5_rows[0], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert1;
                    }

                    sumsq5_ptrs[4] = sumsq5_rows[1];
                    sum5_ptrs[4] = sum5_rows[1];

                    sumsq3_ptrs[2] = sumsq3_rows[1];
                    sum3_ptrs[2] = sum3_rows[1];

                    sgr_box35_row_h(sumsq3_rows[1], sum3_rows[1], sumsq5_rows[1], sum5_rows[1], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    rotate_neon(&mut a5_ptrs, &mut b5_ptrs, 2);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    sumsq5_ptrs[3] = sumsq5_rows[2];
                    sumsq5_ptrs[4] = sumsq5_rows[3];
                    sum5_ptrs[3] = sum5_rows[2];
                    sum5_ptrs[4] = sum5_rows[3];

                    sumsq3_ptrs[2] = sumsq3_rows[2];
                    sum3_ptrs[2] = sum3_rows[2];

                    sgr_box35_row_h(sumsq3_rows[2], sum3_rows[2], sumsq5_rows[2], sum5_rows[2], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Odd;
                    }

                    sgr_box35_row_h(sumsq3_ptrs[2], sum3_ptrs[2], sumsq5_rows[3], sum5_rows[3], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    sgr_finish_mix_neon(
                        &mut dst,
                        stride,
                        &mut a5_ptrs,
                        &mut b5_ptrs,
                        &mut a3_ptrs,
                        &mut b3_ptrs,
                        w,
                        2,
                        w0,
                        w1,
                        tmp5,
                        tmp3,
                        $($bd,)?
                    );

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Vert2;
                    }

                    // ptrs are rotated by 2; both [3] and [4] now point at rows[0]; set
                    // one of them to point at the previously unused rows[4].
                    sumsq5_ptrs[3] = sumsq5_rows[4];
                    sum5_ptrs[3] = sum5_rows[4];
                }

                loop {
                    sgr_box35_row_h(sumsq3_ptrs[2], sum3_ptrs[2], sumsq5_ptrs[3], sum5_ptrs[3], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                    h -= 1;
                    if h <= 0 {
                        break 'body Tail::Odd;
                    }

                    sgr_box35_row_h(sumsq3_ptrs[2], sum3_ptrs[2], sumsq5_ptrs[4], sum5_ptrs[4], left, src, w, edges);
                    left = left.wrapping_add(1);
                    src = src.wrapping_offset(pxstride(stride));

                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    sgr_finish_mix_neon(
                        &mut dst,
                        stride,
                        &mut a5_ptrs,
                        &mut b5_ptrs,
                        &mut a3_ptrs,
                        &mut b3_ptrs,
                        w,
                        2,
                        w0,
                        w1,
                        tmp5,
                        tmp3,
                        $($bd,)?
                    );
                    h -= 1;
                    if h <= 0 {
                        break;
                    }
                }

                if edges & LR_HAVE_BOTTOM == 0 {
                    break 'body Tail::Vert2;
                }

                sgr_box35_row_h(sumsq3_ptrs[2], sum3_ptrs[2], sumsq5_ptrs[3], sum5_ptrs[3], null(), lpf_bottom, w, edges);
                lpf_bottom = lpf_bottom.wrapping_offset(pxstride(stride));
                sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                sgr_box35_row_h(sumsq3_ptrs[2], sum3_ptrs[2], sumsq5_ptrs[4], sum5_ptrs[4], null(), lpf_bottom, w, edges);

                Tail::Output2
            };

            match tail {
                Tail::Vert2 | Tail::Output2 => {
                    if let Tail::Vert2 = tail {
                        // vert_2:
                        // Duplicate the last row twice more
                        sumsq5_ptrs[3] = sumsq5_ptrs[2];
                        sumsq5_ptrs[4] = sumsq5_ptrs[2];
                        sum5_ptrs[3] = sum5_ptrs[2];
                        sum5_ptrs[4] = sum5_ptrs[2];

                        sumsq3_ptrs[2] = sumsq3_ptrs[1];
                        sum3_ptrs[2] = sum3_ptrs[1];
                        sgr_box3_vert_neon(
                            &mut sumsq3_ptrs,
                            &mut sum3_ptrs,
                            a3_ptrs[3],
                            b3_ptrs[3],
                            w,
                            s1,
                            $bitdepth_max,
                        );
                        rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);

                        sumsq3_ptrs[2] = sumsq3_ptrs[1];
                        sum3_ptrs[2] = sum3_ptrs[1];
                    }
                    // output_2:
                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    sgr_finish_mix_neon(
                        &mut dst,
                        stride,
                        &mut a5_ptrs,
                        &mut b5_ptrs,
                        &mut a3_ptrs,
                        &mut b3_ptrs,
                        w,
                        2,
                        w0,
                        w1,
                        tmp5,
                        tmp3,
                        $($bd,)?
                    );
                    return;
                }
                // odd:
                Tail::Odd => {
                    // Copy the last row as padding once
                    sumsq5_ptrs[4] = sumsq5_ptrs[3];
                    sum5_ptrs[4] = sum5_ptrs[3];

                    sumsq3_ptrs[2] = sumsq3_ptrs[1];
                    sum3_ptrs[2] = sum3_ptrs[1];

                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    sgr_finish_mix_neon(
                        &mut dst,
                        stride,
                        &mut a5_ptrs,
                        &mut b5_ptrs,
                        &mut a3_ptrs,
                        &mut b3_ptrs,
                        w,
                        2,
                        w0,
                        w1,
                        tmp5,
                        tmp3,
                        $($bd,)?
                    );
                }
                // vert_1:
                Tail::Vert1 => {
                    // Copy the last row as padding once
                    sumsq5_ptrs[4] = sumsq5_ptrs[3];
                    sum5_ptrs[4] = sum5_ptrs[3];

                    sumsq3_ptrs[2] = sumsq3_ptrs[1];
                    sum3_ptrs[2] = sum3_ptrs[1];

                    sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
                    rotate_neon(&mut a5_ptrs, &mut b5_ptrs, 2);
                    sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
                    rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);
                }
            }

            // output_1:
            // Duplicate the last row twice more
            sumsq5_ptrs[3] = sumsq5_ptrs[2];
            sumsq5_ptrs[4] = sumsq5_ptrs[2];
            sum5_ptrs[3] = sum5_ptrs[2];
            sum5_ptrs[4] = sum5_ptrs[2];

            sumsq3_ptrs[2] = sumsq3_ptrs[1];
            sum3_ptrs[2] = sum3_ptrs[1];

            sgr_box5_vert_neon(&mut sumsq5_ptrs, &mut sum5_ptrs, a5_ptrs[1], b5_ptrs[1], w, s0, $bitdepth_max);
            sgr_box3_vert_neon(&mut sumsq3_ptrs, &mut sum3_ptrs, a3_ptrs[3], b3_ptrs[3], w, s1, $bitdepth_max);
            rotate_neon(&mut a3_ptrs, &mut b3_ptrs, 4);
            // Output only one row
            sgr_finish_mix_neon(
                &mut dst,
                stride,
                &mut a5_ptrs,
                &mut b5_ptrs,
                &mut a3_ptrs,
                &mut b3_ptrs,
                w,
                1,
                w0,
                w1,
                tmp5,
                tmp3,
                $($bd,)?
            );
        }
    };
}

/// The 8 bpc compilation of `src/arm/looprestoration.h`.
mod bpc8 {
    use super::super::super::arm64::looprestoration::{
        dav1d_sgr_box35_row_h_8bpc_neon as sgr_box35_row_h, dav1d_sgr_box3_row_h_8bpc_neon as sgr_box3_row_h,
        dav1d_sgr_box5_row_h_8bpc_neon as sgr_box5_row_h,
        dav1d_sgr_finish_filter1_2rows_8bpc_neon as sgr_finish_filter1_2rows,
        dav1d_sgr_finish_filter2_2rows_8bpc_neon as sgr_finish_filter2_2rows,
        dav1d_sgr_finish_weighted1_8bpc_neon as sgr_finish_weighted1,
        dav1d_sgr_finish_weighted2_8bpc_neon as sgr_finish_weighted2, dav1d_sgr_weighted2_8bpc_neon as sgr_weighted2,
        dav1d_wiener_filter5_8bpc_neon, dav1d_wiener_filter7_8bpc_neon,
    };
    use super::*;

    const BITDEPTH: c_int = 8;

    /// `PXSTRIDE`
    #[inline(always)]
    fn pxstride(x: isize) -> isize {
        x
    }

    sgr_wrappers!(u8, 0xff);

    /// `loop_restoration_dsp_init_arm` at 8 bpc.
    pub(super) unsafe fn loop_restoration_dsp_init_arm(c: &mut Dav1dLoopRestorationDSPContext, bpc: c_int) {
        let flags = dav1d_get_cpu_flags();

        if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
            return;
        }

        c.wiener[0] = Some(dav1d_wiener_filter7_8bpc_neon);
        c.wiener[1] = Some(dav1d_wiener_filter5_8bpc_neon);
        if BITDEPTH == 8 || bpc == 10 {
            c.sgr[0] = Some(sgr_filter_5x5_neon);
            c.sgr[1] = Some(sgr_filter_3x3_neon);
            c.sgr[2] = Some(sgr_filter_mix_neon);
        }
    }
}

/// The 16 bpc compilation of `src/arm/looprestoration.h`.
mod bpc16 {
    use super::super::super::arm64::looprestoration16::{
        dav1d_sgr_box35_row_h_16bpc_neon as sgr_box35_row_h, dav1d_sgr_box3_row_h_16bpc_neon as sgr_box3_row_h,
        dav1d_sgr_box5_row_h_16bpc_neon as sgr_box5_row_h,
        dav1d_sgr_finish_filter1_2rows_16bpc_neon as sgr_finish_filter1_2rows,
        dav1d_sgr_finish_filter2_2rows_16bpc_neon as sgr_finish_filter2_2rows,
        dav1d_sgr_finish_weighted1_16bpc_neon as sgr_finish_weighted1,
        dav1d_sgr_finish_weighted2_16bpc_neon as sgr_finish_weighted2, dav1d_sgr_weighted2_16bpc_neon as sgr_weighted2,
        dav1d_wiener_filter5_16bpc_neon, dav1d_wiener_filter7_16bpc_neon,
    };
    use super::*;

    const BITDEPTH: c_int = 16;

    /// `PXSTRIDE`
    #[inline(always)]
    fn pxstride(x: isize) -> isize {
        x >> 1
    }

    sgr_wrappers!(u16, bitdepth_max, bitdepth_max);

    /// `loop_restoration_dsp_init_arm` at 16 bpc.
    pub(super) unsafe fn loop_restoration_dsp_init_arm(c: &mut Dav1dLoopRestorationDSPContext16, bpc: c_int) {
        let flags = dav1d_get_cpu_flags();

        if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
            return;
        }

        c.wiener[0] = Some(dav1d_wiener_filter7_16bpc_neon);
        c.wiener[1] = Some(dav1d_wiener_filter5_16bpc_neon);
        if BITDEPTH == 8 || bpc == 10 {
            c.sgr[0] = Some(sgr_filter_5x5_neon);
            c.sgr[1] = Some(sgr_filter_3x3_neon);
            c.sgr[2] = Some(sgr_filter_mix_neon);
        }
    }
}

/// Called at the end of the translated `dav1d_loop_restoration_dsp_init_8bpc`,
/// where the original calls `loop_restoration_dsp_init_arm(c, bpc)`.
///
/// # Safety
/// `c` points to the translated `Dav1dLoopRestorationDSPContext` being
/// initialized (same layout, checked by a test), not otherwise accessed
/// during the call.
pub unsafe fn loop_restoration_dsp_init_arm_8bpc(c: *mut c_void, bpc: c_int) {
    bpc8::loop_restoration_dsp_init_arm(&mut *(c as *mut Dav1dLoopRestorationDSPContext), bpc)
}

/// Called at the end of the translated `dav1d_loop_restoration_dsp_init_16bpc`.
///
/// # Safety
/// As for `loop_restoration_dsp_init_arm_8bpc`.
pub unsafe fn loop_restoration_dsp_init_arm_16bpc(c: *mut c_void, bpc: c_int) {
    bpc16::loop_restoration_dsp_init_arm(&mut *(c as *mut Dav1dLoopRestorationDSPContext16), bpc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_and_parameter_layouts_match_the_translation() {
        use crate::b8::looprestoration_tmpl::{
            Dav1dLoopRestorationDSPContext as Translated, LooprestorationParams as TranslatedParams,
        };
        use core::mem::{align_of, offset_of, size_of};
        assert_eq!(size_of::<Translated>(), size_of::<Dav1dLoopRestorationDSPContext>());
        assert_eq!(
            offset_of!(Translated, wiener),
            offset_of!(Dav1dLoopRestorationDSPContext, wiener)
        );
        assert_eq!(
            offset_of!(Translated, sgr),
            offset_of!(Dav1dLoopRestorationDSPContext, sgr)
        );
        assert_eq!(size_of::<Dav1dLoopRestorationDSPContext>(), 5 * size_of::<usize>());
        use crate::b16::looprestoration_tmpl::{
            Dav1dLoopRestorationDSPContext as Translated16, LooprestorationParams as TranslatedParams16,
        };
        assert_eq!(size_of::<Translated16>(), size_of::<Dav1dLoopRestorationDSPContext16>());
        assert_eq!(
            offset_of!(Translated16, wiener),
            offset_of!(Dav1dLoopRestorationDSPContext16, wiener)
        );
        assert_eq!(
            offset_of!(Translated16, sgr),
            offset_of!(Dav1dLoopRestorationDSPContext16, sgr)
        );
        // `LooprestorationParams`: `filter[2][8]` aligned to 16, and the `sgr` member over its start.
        assert_eq!(size_of::<TranslatedParams>(), size_of::<LooprestorationParams>());
        assert_eq!(align_of::<TranslatedParams>(), align_of::<LooprestorationParams>());
        assert_eq!(size_of::<TranslatedParams16>(), size_of::<LooprestorationParams>());
        assert_eq!(align_of::<TranslatedParams16>(), align_of::<LooprestorationParams>());
        assert_eq!(
            (size_of::<LooprestorationParams>(), align_of::<LooprestorationParams>()),
            (32, 16)
        );
        let translated = TranslatedParams {
            filter: [[1, 2, 3, 4, 5, 6, 7, 8], [9, 10, 11, 12, 13, 14, 15, 16]],
        };
        // SAFETY: both unions are plain integers of the same size and alignment.
        let owned: LooprestorationParams = unsafe { core::mem::transmute(translated) };
        // SAFETY: every member is plain integers.
        unsafe {
            assert_eq!(owned.filter, translated.filter);
            assert_eq!(owned.sgr.s0, translated.sgr.s0);
            assert_eq!(owned.sgr.s1, translated.sgr.s1);
            assert_eq!(owned.sgr.w0, translated.sgr.w0);
            assert_eq!(owned.sgr.w1, translated.sgr.w1);
            assert_eq!((owned.sgr.w0, owned.sgr.w1), (5, 6));
        }
    }

    /// With NEON allowed every slot `loop_restoration_dsp_init_arm` assigns
    /// holds a rebuilt kernel or wrapper, and with
    /// `dav1d_set_cpu_flags_mask(0)` none does.
    #[test]
    fn neon_selects_the_rebuilt_kernels_and_mask_zero_keeps_c() {
        use crate::b8::looprestoration_tmpl::{
            dav1d_loop_restoration_dsp_init_8bpc, Dav1dLoopRestorationDSPContext as Translated,
        };
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        use crate::simd::arm64::looprestoration as kernels;
        let rebuilt = [
            kernels::dav1d_wiener_filter7_8bpc_neon as LrFilterFn as usize,
            kernels::dav1d_wiener_filter5_8bpc_neon as LrFilterFn as usize,
            bpc8::sgr_filter_5x5_neon as LrFilterFn as usize,
            bpc8::sgr_filter_3x3_neon as LrFilterFn as usize,
            bpc8::sgr_filter_mix_neon as LrFilterFn as usize,
        ];
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let slots = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_loop_restoration_dsp_init_8bpc(&mut c, 8);
            dav1d_set_cpu_flags_mask(0);
            dav1d_loop_restoration_dsp_init_8bpc(&mut reference, 8);
            dav1d_set_cpu_flags_mask(!0);
            let all = |c: &Translated| {
                let mut v: Vec<Option<usize>> = c.wiener.iter().map(|f| f.map(|f| f as usize)).collect();
                v.extend(c.sgr.iter().map(|f| f.map(|f| f as usize)));
                v
            };
            let (new, old) = (all(&c), all(&reference));
            (new.iter().zip(&old).filter(|(a, b)| a != b).count(), new)
        };
        assert_eq!(slots(!0), (5, rebuilt.map(Some).to_vec()));
        let (changed, selected) = slots(0);
        assert_eq!(changed, 0);
        assert!(selected.iter().zip(&rebuilt).all(|(s, r)| *s != Some(*r)));
    }

    /// The same at 16 bpc: the Wiener slots at 10 and at 12 bit, the
    /// self-guided slots at 10 bit only (`BITDEPTH == 8 || bpc == 10`).
    #[test]
    fn neon_selects_the_rebuilt_16bpc_kernels_and_mask_zero_keeps_c() {
        use crate::b16::looprestoration_tmpl::{
            dav1d_loop_restoration_dsp_init_16bpc, Dav1dLoopRestorationDSPContext as Translated,
        };
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        use crate::simd::arm64::looprestoration16 as kernels;
        let rebuilt = [
            kernels::dav1d_wiener_filter7_16bpc_neon as LrFilterFn16 as usize,
            kernels::dav1d_wiener_filter5_16bpc_neon as LrFilterFn16 as usize,
            bpc16::sgr_filter_5x5_neon as LrFilterFn16 as usize,
            bpc16::sgr_filter_3x3_neon as LrFilterFn16 as usize,
            bpc16::sgr_filter_mix_neon as LrFilterFn16 as usize,
        ];
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let slots = |mask: u32, bpc: c_int| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_loop_restoration_dsp_init_16bpc(&mut c, bpc);
            dav1d_set_cpu_flags_mask(0);
            dav1d_loop_restoration_dsp_init_16bpc(&mut reference, bpc);
            dav1d_set_cpu_flags_mask(!0);
            let all = |c: &Translated| {
                let mut v: Vec<Option<usize>> = c.wiener.iter().map(|f| f.map(|f| f as usize)).collect();
                v.extend(c.sgr.iter().map(|f| f.map(|f| f as usize)));
                v
            };
            let (new, old) = (all(&c), all(&reference));
            let changed: Vec<bool> = new.iter().zip(&old).map(|(a, b)| a != b).collect();
            (changed, new)
        };
        assert_eq!(slots(!0, 10), (vec![true; 5], rebuilt.map(Some).to_vec()));
        // 12 bit: the two Wiener slots only; the self-guided slots keep the C kernels.
        let (changed, selected) = slots(!0, 12);
        assert_eq!(changed, [true, true, false, false, false]);
        assert_eq!(selected[..2], rebuilt.map(Some)[..2]);
        assert!(selected[2..].iter().zip(&rebuilt[2..]).all(|(s, r)| *s != Some(*r)));
        for bpc in [10, 12] {
            let (changed, selected) = slots(0, bpc);
            assert_eq!(changed, [false; 5]);
            assert!(selected.iter().zip(&rebuilt).all(|(s, r)| *s != Some(*r)));
        }
    }
}
