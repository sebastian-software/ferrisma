// Copyright © 2018, VideoLAN and dav1d authors
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `upstream/dav1d/src/arm/ipred.h`: the C wrappers `ipred_z1_neon`,
//! `ipred_z2_neon` and `ipred_z3_neon` (edge preparation in C, then the
//! assembly's fill kernels; AArch64 only in the original as well) and
//! `intra_pred_dsp_init_arm`: under `DAV1D_ARM_CPU_FLAG_NEON` the fourteen
//! `intra_pred` slots, the four `cfl_pred` slots, the three `cfl_ac` slots
//! and `pal_pred` get the rebuilt kernels (`simd/arm64/ipred.rs` at 8 bpc,
//! `ipred16.rs` at 16 bpc, for every high bit depth as in the original);
//! with the flag masked off the C kernels stay.
//!
//! `get_filter_strength` and `get_upsample` are the two static functions of
//! `src/ipred_tmpl.c` that the header uses (it is included into that file);
//! the translated unit keeps them private, so they are translated here by
//! hand, statement for statement.
//!
//! The wrappers' stack buffers, which C leaves uninitialized: the kernels
//! load whole blocks of pixels, so they read elements nothing has written
//! (the values never reach the output), and `ipred_z1_filter_edge` at
//! strength 3 loads from `in - 1`. Therefore `buf` (z2) and `flipped` (z3)
//! are zero-initialized and, being passed to `ipred_z1_filter_edge` as
//! `in`, have one element in front of them (`buf` four, for the z2 fill
//! kernel of width 64); `top_out` (z1) and `left_out` (z3) are zeroed whole
//! as well (zero-initialized arrays), before anything is written to them. No
//! kernel therefore loads an element of a wrapper's buffer that has no
//! value, whatever it loads; what the kernels are known to load:
//! - z1 `top_out`: the edge kernels or `memcpy` write `[0, max_base_x]`
//!   and `ipred_pixel_set` the `pad_pixels * base_inc` elements behind it
//!   (more, in whole blocks). `ipred_z1_fill1`/`fill2` load whole blocks at
//!   a row's `base`, which reach past the padded range (by up to 15
//!   elements, as enumerated when the kernels were translated);
//! - z2 `buf`: `flipped[-1]` is `buf[-1]` in C. The fill kernels load whole
//!   vectors around what the wrapper writes: below `top[0]` down to
//!   `top[-69]` (width 64; `top[-65]` is `buf[0]` in C, hence four elements
//!   in front instead of one), past `top[width]` up to `top[width + 15]`,
//!   past `left[height]` up to `left[64]`, and at 16 bpc single pixels below
//!   `left[0]` down to `left[-81]`;
//! - z3 `flipped`: `ipred_reverse` writes `flipped[1..]` in blocks of 16
//!   pixels (8 at 16 bpc); `ipred_z1_filter_edge` and
//!   `ipred_z1_upsample_edge` read up to 15 elements more, and
//!   `ipred_z1_filter_edge` reads `flipped[-1]`;
//! - z3 `left_out`: the edge kernels write `[0, max_base_y]` and
//!   `ipred_pixel_set` the `pad_pixels * base_inc` elements behind it.
//!   At 8 bpc with `max_base_y > 64`, `ipred_z3_fill1` loads
//!   `height + 16` pixels at the base of each column, and the second
//!   column of a pair, whose base is not compared with `max_base_y`,
//!   starts above it (by up to 15 elements, as enumerated when the kernels
//!   were translated).
//!
//! Outside the wrappers' buffers: Z1 at 64x64 with the intra edge filter
//! makes `ipred_z1_filter_edge` load up to `topleft_in[142]` at 8 bpc and
//! `topleft_in[134]` at 16 bpc, 14 and 6 pixels past the caller's
//! `edge[257]` array (`topleft_in` is `edge + 128`). Not in the original:
//! the wrapper passes it a zero-initialized local copy of the pixels the C
//! filter reads instead (`crate::simd::copy_edge`), so every load stays
//! inside memory the wrapper owns or the caller's array; test
//! `the_kernels_read_only_the_edge_array`.
use core::ffi::{c_int, c_void};

use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};
use crate::common::tables::dav1d_dr_intra_derivative;

// `enum IntraPredMode` (src/levels.h), the indices of `intra_pred` and
// `cfl_pred`.
const DC_PRED: usize = 0;
const VERT_PRED: usize = 1;
const HOR_PRED: usize = 2;
const LEFT_DC_PRED: usize = 3;
const TOP_DC_PRED: usize = 4;
const DC_128_PRED: usize = 5;
const Z1_PRED: usize = 6;
const Z2_PRED: usize = 7;
const Z3_PRED: usize = 8;
const SMOOTH_PRED: usize = 9;
const SMOOTH_V_PRED: usize = 10;
const SMOOTH_H_PRED: usize = 11;
const PAETH_PRED: usize = 12;
const FILTER_PRED: usize = 13;
// `enum Dav1dPixelLayout` (include/dav1d/picture.h).
const DAV1D_PIXEL_LAYOUT_I420: usize = 1;
const DAV1D_PIXEL_LAYOUT_I422: usize = 2;
const DAV1D_PIXEL_LAYOUT_I444: usize = 3;

/// `angular_ipred_fn` (src/ipred.h) at 8 bpc.
pub type AngularIpredFn = unsafe extern "C" fn(*mut u8, isize, *const u8, c_int, c_int, c_int, c_int, c_int);
/// `cfl_ac_fn` at 8 bpc.
pub type CflAcFn = unsafe extern "C" fn(*mut i16, *const u8, isize, c_int, c_int, c_int, c_int);
/// `cfl_pred_fn` at 8 bpc.
pub type CflPredFn = unsafe extern "C" fn(*mut u8, isize, *const u8, c_int, c_int, *const i16, c_int);
/// `pal_pred_fn` at 8 bpc.
pub type PalPredFn = unsafe extern "C" fn(*mut u8, isize, *const u8, *const u8, c_int, c_int);

/// `Dav1dIntraPredDSPContext` (src/ipred.h) at 8 bpc.
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [Option<AngularIpredFn>; 14],
    pub cfl_ac: [Option<CflAcFn>; 3],
    pub cfl_pred: [Option<CflPredFn>; 6],
    pub pal_pred: Option<PalPredFn>,
}

/// `angular_ipred_fn` at 16 bpc (with `bitdepth_max`).
pub type AngularIpredFn16 = unsafe extern "C" fn(*mut u16, isize, *const u16, c_int, c_int, c_int, c_int, c_int, c_int);
/// `cfl_ac_fn` at 16 bpc.
pub type CflAcFn16 = unsafe extern "C" fn(*mut i16, *const u16, isize, c_int, c_int, c_int, c_int);
/// `cfl_pred_fn` at 16 bpc (with `bitdepth_max`).
pub type CflPredFn16 = unsafe extern "C" fn(*mut u16, isize, *const u16, c_int, c_int, *const i16, c_int, c_int);
/// `pal_pred_fn` at 16 bpc.
pub type PalPredFn16 = unsafe extern "C" fn(*mut u16, isize, *const u16, *const u8, c_int, c_int);

/// `Dav1dIntraPredDSPContext` at 16 bpc.
#[repr(C)]
pub struct Dav1dIntraPredDSPContext16 {
    pub intra_pred: [Option<AngularIpredFn16>; 14],
    pub cfl_ac: [Option<CflAcFn16>; 3],
    pub cfl_pred: [Option<CflPredFn16>; 6],
    pub pal_pred: Option<PalPredFn16>,
}

#[inline(always)]
fn imin(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn imax(a: c_int, b: c_int) -> c_int {
    if a > b {
        a
    } else {
        b
    }
}

/// `get_filter_strength` (src/ipred_tmpl.c, `NOINLINE` there).
#[inline(never)]
fn get_filter_strength(wh: c_int, angle: c_int, is_sm: c_int) -> c_int {
    if is_sm != 0 {
        if wh <= 8 {
            if angle >= 64 {
                return 2;
            }
            if angle >= 40 {
                return 1;
            }
        } else if wh <= 16 {
            if angle >= 48 {
                return 2;
            }
            if angle >= 20 {
                return 1;
            }
        } else if wh <= 24 {
            if angle >= 4 {
                return 3;
            }
        } else {
            return 3;
        }
    } else {
        if wh <= 8 {
            if angle >= 56 {
                return 1;
            }
        } else if wh <= 16 {
            if angle >= 40 {
                return 1;
            }
        } else if wh <= 24 {
            if angle >= 32 {
                return 3;
            }
            if angle >= 16 {
                return 2;
            }
            if angle >= 8 {
                return 1;
            }
        } else if wh <= 32 {
            if angle >= 32 {
                return 3;
            }
            if angle >= 4 {
                return 2;
            }
            return 1;
        } else {
            return 3;
        }
    }
    0
}

/// `get_upsample` (src/ipred_tmpl.c).
#[inline]
fn get_upsample(wh: c_int, angle: c_int, is_sm: c_int) -> c_int {
    (angle < 40 && wh <= 16 >> is_sm) as c_int
}

/// The three wrappers of `src/arm/ipred.h` for one bit depth. `$pixel` is
/// `pixel`; `$bd` is `HIGHBD_DECL_SUFFIX`/`HIGHBD_TAIL_SUFFIX` (nothing at
/// 8 bpc, `bitdepth_max` at 16 bpc). The kernel names are those of the
/// invoking module (`z1_upsample_edge` is
/// `dav1d_ipred_z1_upsample_edge_<bpc>bpc_neon`, and so on).
macro_rules! z_wrappers {
    ($pixel:ty $(, $bd:ident)?) => {
        /// `ipred_z1_neon`
        pub(super) unsafe extern "C" fn ipred_z1_neon(
            dst: *mut $pixel,
            stride: isize,
            topleft_in: *const $pixel,
            width: c_int,
            height: c_int,
            mut angle: c_int,
            _max_width: c_int,
            _max_height: c_int,
            $($bd: c_int,)?
        ) {
            let is_sm = (angle >> 9) & 0x1;
            let enable_intra_edge_filter = angle >> 10;
            angle &= 511;
            let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as c_int;
            // Zero-initialized (uninitialized in the original): the kernels load whole blocks of pixels.
            let mut top_out_buf: [$pixel; 64 + 64 + (64 + 15) * 2 + 16] = [0; 64 + 64 + (64 + 15) * 2 + 16];
            let top_out = top_out_buf.as_mut_ptr();
            let max_base_x;
            let upsample_above =
                if enable_intra_edge_filter != 0 { get_upsample(width + height, 90 - angle, is_sm) } else { 0 };
            if upsample_above != 0 {
                z1_upsample_edge(top_out, width + height, topleft_in, width + imin(width, height) $(, $bd)?);
                max_base_x = 2 * (width + height) - 2;
                dx <<= 1;
            } else {
                let filter_strength = if enable_intra_edge_filter != 0 {
                    get_filter_strength(width + height, 90 - angle, is_sm)
                } else {
                    0
                };
                if filter_strength != 0 {
                    // Not in the original: `ipred_z1_filter_edge` loads whole blocks around the edge, at 64x64 up to
                    // `topleft_in[142]` (8 bpc) or `[134]` (16 bpc), behind the caller's array. It runs on a copy of
                    // the pixels the C filter reads, `topleft_in[0..=end]`, with zeros around them (`copy_edge`).
                    let end = width + imin(width, height);
                    let mut edge: [$pixel; 32 + 129 + 48] = [0; 32 + 129 + 48];
                    let topleft = crate::simd::copy_edge(&mut edge, 32, topleft_in, 0, end as isize);
                    z1_filter_edge(top_out, width + height, topleft, end, filter_strength);
                    max_base_x = width + height - 1;
                } else {
                    max_base_x = width + imin(width, height) - 1;
                    core::ptr::copy_nonoverlapping(topleft_in.add(1), top_out, (max_base_x + 1) as usize);
                }
            }
            let base_inc = 1 + upsample_above;
            let pad_pixels = width + 15; // max(dx >> 6) == 15
            pixel_set(top_out.add(max_base_x as usize + 1), *top_out.add(max_base_x as usize), pad_pixels * base_inc);
            if upsample_above != 0 {
                z1_fill2(dst, stride, top_out, width, height, dx, max_base_x);
            } else {
                z1_fill1(dst, stride, top_out, width, height, dx, max_base_x);
            }
        }

        /// `ipred_z2_neon`
        pub(super) unsafe extern "C" fn ipred_z2_neon(
            dst: *mut $pixel,
            stride: isize,
            topleft_in: *const $pixel,
            width: c_int,
            height: c_int,
            mut angle: c_int,
            max_width: c_int,
            max_height: c_int,
            $($bd: c_int,)?
        ) {
            let is_sm = (angle >> 9) & 0x1;
            let enable_intra_edge_filter = angle >> 10;
            angle &= 511;
            debug_assert!(angle > 90 && angle < 180);
            let mut dy = dav1d_dr_intra_derivative[((angle - 90) >> 1) as usize] as c_int;
            let mut dx = dav1d_dr_intra_derivative[((180 - angle) >> 1) as usize] as c_int;
            let upsample_left =
                if enable_intra_edge_filter != 0 { get_upsample(width + height, 180 - angle, is_sm) } else { 0 };
            let upsample_above =
                if enable_intra_edge_filter != 0 { get_upsample(width + height, angle - 90, is_sm) } else { 0 };
            // Four elements in front (the original has `pixel buf[3*(64+1)]`):
            // `ipred_z1_filter_edge` reads `flipped[-1]`, and for width 64 the fill
            // kernel loads the second row of a pair from `top[base_x]` with `base_x`
            // down to -69 (the pair is still predicted from `top` while the first
            // row's `base_x` is above -64, and the second row lies up to `dx >> 6`
            // further left: `dx` 372 gives -63 and -69 at rows 10 and 11), which is
            // four pixels below `top[-65]`, the start of the original's buffer. The
            // values are not used (every pixel of that row comes from `left`), but
            // the load must stay inside the buffer.
            let mut buf: [$pixel; 4 + 3 * (64 + 1)] = [0; 4 + 3 * (64 + 1)];
            let buf = buf.as_mut_ptr().add(4);
            let left = buf.add(2 * (64 + 1));
            // The asm can underread below the start of top[] and left[]; to avoid
            // surprising behaviour, make sure this is within the allocated stack space.
            let top = buf.add(64 + 1);
            let flipped = buf;

            if upsample_above != 0 {
                z2_upsample_edge(top, width, topleft_in $(, $bd)?);
                dx <<= 1;
            } else {
                let filter_strength = if enable_intra_edge_filter != 0 {
                    get_filter_strength(width + height, angle - 90, is_sm)
                } else {
                    0
                };

                if filter_strength != 0 {
                    z1_filter_edge(top.add(1), imin(max_width, width), topleft_in, width, filter_strength);
                    if max_width < width {
                        core::ptr::copy_nonoverlapping(
                            topleft_in.add(1 + max_width as usize),
                            top.add(1 + max_width as usize),
                            (width - max_width) as usize,
                        );
                    }
                } else {
                    core::ptr::copy_nonoverlapping(topleft_in.add(1), top.add(1), width as usize);
                }
            }
            if upsample_left != 0 {
                *flipped = *topleft_in;
                reverse(flipped.add(1), topleft_in, height);
                z2_upsample_edge(left, height, flipped $(, $bd)?);
                dy <<= 1;
            } else {
                let filter_strength = if enable_intra_edge_filter != 0 {
                    get_filter_strength(width + height, 180 - angle, is_sm)
                } else {
                    0
                };

                if filter_strength != 0 {
                    *flipped = *topleft_in;
                    reverse(flipped.add(1), topleft_in, height);
                    z1_filter_edge(left.add(1), imin(max_height, height), flipped, height, filter_strength);
                    if max_height < height {
                        core::ptr::copy_nonoverlapping(
                            flipped.add(1 + max_height as usize),
                            left.add(1 + max_height as usize),
                            (height - max_height) as usize,
                        );
                    }
                } else {
                    reverse(left.add(1), topleft_in, height);
                }
            }
            *left = *topleft_in;
            *top = *topleft_in;

            debug_assert!(!(upsample_above != 0 && upsample_left != 0));
            if upsample_above == 0 && upsample_left == 0 {
                z2_fill1(dst, stride, top, left, width, height, dx, dy);
            } else if upsample_above != 0 {
                z2_fill2(dst, stride, top, left, width, height, dx, dy);
            } else {
                z2_fill3(dst, stride, top, left, width, height, dx, dy);
            }
        }

        /// `ipred_z3_neon`
        pub(super) unsafe extern "C" fn ipred_z3_neon(
            dst: *mut $pixel,
            stride: isize,
            topleft_in: *const $pixel,
            width: c_int,
            height: c_int,
            mut angle: c_int,
            _max_width: c_int,
            _max_height: c_int,
            $($bd: c_int,)?
        ) {
            let is_sm = (angle >> 9) & 0x1;
            let enable_intra_edge_filter = angle >> 10;
            angle &= 511;
            debug_assert!(angle > 180);
            let mut dy = dav1d_dr_intra_derivative[((270 - angle) >> 1) as usize] as c_int;
            // One element in front: `ipred_z1_filter_edge` reads `flipped[-1]`.
            let mut flipped_buf: [$pixel; 1 + 64 + 64 + 16] = [0; 1 + 64 + 64 + 16];
            let flipped = flipped_buf.as_mut_ptr().add(1);
            // Zero-initialized (uninitialized in the original): the kernels load whole blocks of pixels.
            let mut left_out_buf: [$pixel; 64 + 64 + (64 + 15) * 2] = [0; 64 + 64 + (64 + 15) * 2];
            let left_out = left_out_buf.as_mut_ptr();
            let max_base_y;
            let upsample_left =
                if enable_intra_edge_filter != 0 { get_upsample(width + height, angle - 180, is_sm) } else { 0 };
            if upsample_left != 0 {
                *flipped = *topleft_in;
                reverse(flipped.add(1), topleft_in, height + imax(width, height));
                z1_upsample_edge(left_out, width + height, flipped, height + imin(width, height) $(, $bd)?);
                max_base_y = 2 * (width + height) - 2;
                dy <<= 1;
            } else {
                let filter_strength = if enable_intra_edge_filter != 0 {
                    get_filter_strength(width + height, angle - 180, is_sm)
                } else {
                    0
                };

                if filter_strength != 0 {
                    *flipped = *topleft_in;
                    reverse(flipped.add(1), topleft_in, height + imax(width, height));
                    z1_filter_edge(left_out, width + height, flipped, height + imin(width, height), filter_strength);
                    max_base_y = width + height - 1;
                } else {
                    reverse(left_out, topleft_in, height + imin(width, height));
                    max_base_y = height + imin(width, height) - 1;
                }
            }
            let base_inc = 1 + upsample_left;
            // The tbx based implementation needs left[] to have 64 bytes intitialized,
            // the other implementation can read height + max(dy >> 6) past the end.
            let pad_pixels = imax(64 - max_base_y - 1, height + 15);

            pixel_set(left_out.add(max_base_y as usize + 1), *left_out.add(max_base_y as usize), pad_pixels * base_inc);
            if upsample_left != 0 {
                z3_fill2(dst, stride, left_out, width, height, dy, max_base_y);
            } else {
                z3_fill1(dst, stride, left_out, width, height, dy, max_base_y);
            }
        }
    };
}

/// The 8 bpc compilation of `src/arm/ipred.h`.
mod bpc8 {
    use super::super::super::arm64::ipred::{
        dav1d_ipred_cfl_128_8bpc_neon, dav1d_ipred_cfl_8bpc_neon, dav1d_ipred_cfl_ac_420_8bpc_neon,
        dav1d_ipred_cfl_ac_422_8bpc_neon, dav1d_ipred_cfl_ac_444_8bpc_neon, dav1d_ipred_cfl_left_8bpc_neon,
        dav1d_ipred_cfl_top_8bpc_neon, dav1d_ipred_dc_128_8bpc_neon, dav1d_ipred_dc_8bpc_neon,
        dav1d_ipred_dc_left_8bpc_neon, dav1d_ipred_dc_top_8bpc_neon, dav1d_ipred_filter_8bpc_neon,
        dav1d_ipred_h_8bpc_neon, dav1d_ipred_paeth_8bpc_neon, dav1d_ipred_pixel_set_8bpc_neon as pixel_set,
        dav1d_ipred_reverse_8bpc_neon as reverse, dav1d_ipred_smooth_8bpc_neon, dav1d_ipred_smooth_h_8bpc_neon,
        dav1d_ipred_smooth_v_8bpc_neon, dav1d_ipred_v_8bpc_neon, dav1d_ipred_z1_fill1_8bpc_neon as z1_fill1,
        dav1d_ipred_z1_fill2_8bpc_neon as z1_fill2, dav1d_ipred_z1_filter_edge_8bpc_neon as z1_filter_edge,
        dav1d_ipred_z1_upsample_edge_8bpc_neon as z1_upsample_edge, dav1d_ipred_z2_fill1_8bpc_neon as z2_fill1,
        dav1d_ipred_z2_fill2_8bpc_neon as z2_fill2, dav1d_ipred_z2_fill3_8bpc_neon as z2_fill3,
        dav1d_ipred_z2_upsample_edge_8bpc_neon as z2_upsample_edge, dav1d_ipred_z3_fill1_8bpc_neon as z3_fill1,
        dav1d_ipred_z3_fill2_8bpc_neon as z3_fill2, dav1d_pal_pred_8bpc_neon,
    };
    use super::*;

    z_wrappers!(u8);

    /// `intra_pred_dsp_init_arm` at 8 bpc.
    pub(super) unsafe fn intra_pred_dsp_init_arm(c: &mut Dav1dIntraPredDSPContext) {
        let flags = dav1d_get_cpu_flags();

        if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
            return;
        }

        c.intra_pred[DC_PRED] = Some(dav1d_ipred_dc_8bpc_neon);
        c.intra_pred[DC_128_PRED] = Some(dav1d_ipred_dc_128_8bpc_neon);
        c.intra_pred[TOP_DC_PRED] = Some(dav1d_ipred_dc_top_8bpc_neon);
        c.intra_pred[LEFT_DC_PRED] = Some(dav1d_ipred_dc_left_8bpc_neon);
        c.intra_pred[HOR_PRED] = Some(dav1d_ipred_h_8bpc_neon);
        c.intra_pred[VERT_PRED] = Some(dav1d_ipred_v_8bpc_neon);
        c.intra_pred[PAETH_PRED] = Some(dav1d_ipred_paeth_8bpc_neon);
        c.intra_pred[SMOOTH_PRED] = Some(dav1d_ipred_smooth_8bpc_neon);
        c.intra_pred[SMOOTH_V_PRED] = Some(dav1d_ipred_smooth_v_8bpc_neon);
        c.intra_pred[SMOOTH_H_PRED] = Some(dav1d_ipred_smooth_h_8bpc_neon);
        c.intra_pred[Z1_PRED] = Some(ipred_z1_neon);
        c.intra_pred[Z2_PRED] = Some(ipred_z2_neon);
        c.intra_pred[Z3_PRED] = Some(ipred_z3_neon);
        c.intra_pred[FILTER_PRED] = Some(dav1d_ipred_filter_8bpc_neon);

        c.cfl_pred[DC_PRED] = Some(dav1d_ipred_cfl_8bpc_neon);
        c.cfl_pred[DC_128_PRED] = Some(dav1d_ipred_cfl_128_8bpc_neon);
        c.cfl_pred[TOP_DC_PRED] = Some(dav1d_ipred_cfl_top_8bpc_neon);
        c.cfl_pred[LEFT_DC_PRED] = Some(dav1d_ipred_cfl_left_8bpc_neon);

        c.cfl_ac[DAV1D_PIXEL_LAYOUT_I420 - 1] = Some(dav1d_ipred_cfl_ac_420_8bpc_neon);
        c.cfl_ac[DAV1D_PIXEL_LAYOUT_I422 - 1] = Some(dav1d_ipred_cfl_ac_422_8bpc_neon);
        c.cfl_ac[DAV1D_PIXEL_LAYOUT_I444 - 1] = Some(dav1d_ipred_cfl_ac_444_8bpc_neon);

        c.pal_pred = Some(dav1d_pal_pred_8bpc_neon);
    }
}

/// The 16 bpc compilation of `src/arm/ipred.h`.
mod bpc16 {
    use super::super::super::arm64::ipred16::{
        dav1d_ipred_cfl_128_16bpc_neon, dav1d_ipred_cfl_16bpc_neon, dav1d_ipred_cfl_ac_420_16bpc_neon,
        dav1d_ipred_cfl_ac_422_16bpc_neon, dav1d_ipred_cfl_ac_444_16bpc_neon, dav1d_ipred_cfl_left_16bpc_neon,
        dav1d_ipred_cfl_top_16bpc_neon, dav1d_ipred_dc_128_16bpc_neon, dav1d_ipred_dc_16bpc_neon,
        dav1d_ipred_dc_left_16bpc_neon, dav1d_ipred_dc_top_16bpc_neon, dav1d_ipred_filter_16bpc_neon,
        dav1d_ipred_h_16bpc_neon, dav1d_ipred_paeth_16bpc_neon, dav1d_ipred_pixel_set_16bpc_neon as pixel_set,
        dav1d_ipred_reverse_16bpc_neon as reverse, dav1d_ipred_smooth_16bpc_neon, dav1d_ipred_smooth_h_16bpc_neon,
        dav1d_ipred_smooth_v_16bpc_neon, dav1d_ipred_v_16bpc_neon, dav1d_ipred_z1_fill1_16bpc_neon as z1_fill1,
        dav1d_ipred_z1_fill2_16bpc_neon as z1_fill2, dav1d_ipred_z1_filter_edge_16bpc_neon as z1_filter_edge,
        dav1d_ipred_z1_upsample_edge_16bpc_neon as z1_upsample_edge, dav1d_ipred_z2_fill1_16bpc_neon as z2_fill1,
        dav1d_ipred_z2_fill2_16bpc_neon as z2_fill2, dav1d_ipred_z2_fill3_16bpc_neon as z2_fill3,
        dav1d_ipred_z2_upsample_edge_16bpc_neon as z2_upsample_edge, dav1d_ipred_z3_fill1_16bpc_neon as z3_fill1,
        dav1d_ipred_z3_fill2_16bpc_neon as z3_fill2, dav1d_pal_pred_16bpc_neon,
    };
    use super::*;

    z_wrappers!(u16, bitdepth_max);

    /// `intra_pred_dsp_init_arm` at 16 bpc.
    pub(super) unsafe fn intra_pred_dsp_init_arm(c: &mut Dav1dIntraPredDSPContext16) {
        let flags = dav1d_get_cpu_flags();

        if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
            return;
        }

        c.intra_pred[DC_PRED] = Some(dav1d_ipred_dc_16bpc_neon);
        c.intra_pred[DC_128_PRED] = Some(dav1d_ipred_dc_128_16bpc_neon);
        c.intra_pred[TOP_DC_PRED] = Some(dav1d_ipred_dc_top_16bpc_neon);
        c.intra_pred[LEFT_DC_PRED] = Some(dav1d_ipred_dc_left_16bpc_neon);
        c.intra_pred[HOR_PRED] = Some(dav1d_ipred_h_16bpc_neon);
        c.intra_pred[VERT_PRED] = Some(dav1d_ipred_v_16bpc_neon);
        c.intra_pred[PAETH_PRED] = Some(dav1d_ipred_paeth_16bpc_neon);
        c.intra_pred[SMOOTH_PRED] = Some(dav1d_ipred_smooth_16bpc_neon);
        c.intra_pred[SMOOTH_V_PRED] = Some(dav1d_ipred_smooth_v_16bpc_neon);
        c.intra_pred[SMOOTH_H_PRED] = Some(dav1d_ipred_smooth_h_16bpc_neon);
        c.intra_pred[Z1_PRED] = Some(ipred_z1_neon);
        c.intra_pred[Z2_PRED] = Some(ipred_z2_neon);
        c.intra_pred[Z3_PRED] = Some(ipred_z3_neon);
        c.intra_pred[FILTER_PRED] = Some(dav1d_ipred_filter_16bpc_neon);

        c.cfl_pred[DC_PRED] = Some(dav1d_ipred_cfl_16bpc_neon);
        c.cfl_pred[DC_128_PRED] = Some(dav1d_ipred_cfl_128_16bpc_neon);
        c.cfl_pred[TOP_DC_PRED] = Some(dav1d_ipred_cfl_top_16bpc_neon);
        c.cfl_pred[LEFT_DC_PRED] = Some(dav1d_ipred_cfl_left_16bpc_neon);

        c.cfl_ac[DAV1D_PIXEL_LAYOUT_I420 - 1] = Some(dav1d_ipred_cfl_ac_420_16bpc_neon);
        c.cfl_ac[DAV1D_PIXEL_LAYOUT_I422 - 1] = Some(dav1d_ipred_cfl_ac_422_16bpc_neon);
        c.cfl_ac[DAV1D_PIXEL_LAYOUT_I444 - 1] = Some(dav1d_ipred_cfl_ac_444_16bpc_neon);

        c.pal_pred = Some(dav1d_pal_pred_16bpc_neon);
    }
}

/// Called at the end of the translated `dav1d_intra_pred_dsp_init_8bpc`,
/// where the original calls `intra_pred_dsp_init_arm(c)`.
///
/// # Safety
/// `c` points to the translated `Dav1dIntraPredDSPContext` being
/// initialized (same layout, checked by a test), not otherwise accessed
/// during the call.
pub unsafe fn intra_pred_dsp_init_arm_8bpc(c: *mut c_void) {
    bpc8::intra_pred_dsp_init_arm(&mut *(c as *mut Dav1dIntraPredDSPContext))
}

/// Called at the end of the translated `dav1d_intra_pred_dsp_init_16bpc`.
///
/// # Safety
/// As for `intra_pred_dsp_init_arm_8bpc`.
pub unsafe fn intra_pred_dsp_init_arm_16bpc(c: *mut c_void) {
    bpc16::intra_pred_dsp_init_arm(&mut *(c as *mut Dav1dIntraPredDSPContext16))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_layout_matches_the_translation() {
        use crate::b8::ipred_tmpl::Dav1dIntraPredDSPContext as Translated;
        use core::mem::{offset_of, size_of};
        assert_eq!(size_of::<Translated>(), size_of::<Dav1dIntraPredDSPContext>());
        assert_eq!(
            offset_of!(Translated, intra_pred),
            offset_of!(Dav1dIntraPredDSPContext, intra_pred)
        );
        assert_eq!(
            offset_of!(Translated, cfl_ac),
            offset_of!(Dav1dIntraPredDSPContext, cfl_ac)
        );
        assert_eq!(
            offset_of!(Translated, cfl_pred),
            offset_of!(Dav1dIntraPredDSPContext, cfl_pred)
        );
        assert_eq!(
            offset_of!(Translated, pal_pred),
            offset_of!(Dav1dIntraPredDSPContext, pal_pred)
        );
        assert_eq!(size_of::<Dav1dIntraPredDSPContext>(), 24 * size_of::<usize>());
        use crate::b16::ipred_tmpl::Dav1dIntraPredDSPContext as Translated16;
        assert_eq!(size_of::<Translated16>(), size_of::<Dav1dIntraPredDSPContext16>());
        assert_eq!(
            offset_of!(Translated16, intra_pred),
            offset_of!(Dav1dIntraPredDSPContext16, intra_pred)
        );
        assert_eq!(
            offset_of!(Translated16, cfl_ac),
            offset_of!(Dav1dIntraPredDSPContext16, cfl_ac)
        );
        assert_eq!(
            offset_of!(Translated16, cfl_pred),
            offset_of!(Dav1dIntraPredDSPContext16, cfl_pred)
        );
        assert_eq!(
            offset_of!(Translated16, pal_pred),
            offset_of!(Dav1dIntraPredDSPContext16, pal_pred)
        );
    }

    /// The edge-filter decisions of `src/ipred_tmpl.c`, at the thresholds.
    #[test]
    fn filter_strength_and_upsample_follow_the_original() {
        assert_eq!(get_upsample(16, 39, 0), 1);
        assert_eq!(get_upsample(16, 40, 0), 0);
        assert_eq!(get_upsample(16, 39, 1), 0);
        assert_eq!(get_upsample(8, 39, 1), 1);
        assert_eq!(get_filter_strength(8, 56, 0), 1);
        assert_eq!(get_filter_strength(8, 55, 0), 0);
        assert_eq!(get_filter_strength(24, 16, 0), 2);
        assert_eq!(get_filter_strength(32, 3, 0), 1);
        assert_eq!(get_filter_strength(33, 0, 0), 3);
        assert_eq!(get_filter_strength(8, 64, 1), 2);
        assert_eq!(get_filter_strength(16, 20, 1), 1);
        assert_eq!(get_filter_strength(24, 3, 1), 0);
        assert_eq!(get_filter_strength(25, 0, 1), 3);
    }

    /// The intra prediction slots load the edge only inside the caller's array: every slot, every block size,
    /// the directional modes over their angles and flags, the edge array (257 pixels, `topleft` at index 128, or
    /// at 32 for the inter-intra modes) placed directly after and directly before a page without access, so that
    /// a load outside the array faults (`crate::simd::edge_test::check`). The kernels whose loads would leave the
    /// array run on a copy of the pixels the C kernel reads (`crate::simd::copy_edge`). Each block also equals the
    /// C kernel's.
    #[test]
    fn the_kernels_read_only_the_edge_array() {
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        {
            use crate::b8::ipred_tmpl::{dav1d_intra_pred_dsp_init_8bpc, Dav1dIntraPredDSPContext as Translated};
            let init = |mask: u32| unsafe {
                dav1d_init_cpu();
                dav1d_set_cpu_flags_mask(mask);
                let mut c: Translated = core::mem::zeroed();
                dav1d_intra_pred_dsp_init_8bpc(&mut c);
                dav1d_set_cpu_flags_mask(!0);
                c
            };
            let (port, c) = (init(!0), init(0));
            assert_ne!(
                port.intra_pred[Z1_PRED].map(|f| f as usize),
                c.intra_pred[Z1_PRED].map(|f| f as usize)
            );
            crate::simd::edge_test::check(
                |v| v as u8,
                |_| true,
                |is_port, mode, dst, stride, topleft, w, h, angle| unsafe {
                    let f = if is_port {
                        port.intra_pred[mode]
                    } else {
                        c.intra_pred[mode]
                    };
                    f.unwrap()(dst, stride, topleft, w, h, angle, w, h)
                },
            );
        }
        {
            use crate::b16::ipred_tmpl::{dav1d_intra_pred_dsp_init_16bpc, Dav1dIntraPredDSPContext as Translated};
            let init = |mask: u32| unsafe {
                dav1d_init_cpu();
                dav1d_set_cpu_flags_mask(mask);
                let mut c: Translated = core::mem::zeroed();
                dav1d_intra_pred_dsp_init_16bpc(&mut c);
                dav1d_set_cpu_flags_mask(!0);
                c
            };
            let (port, c) = (init(!0), init(0));
            assert_ne!(
                port.intra_pred[Z1_PRED].map(|f| f as usize),
                c.intra_pred[Z1_PRED].map(|f| f as usize)
            );
            for bitdepth_max in [1023, 4095] {
                crate::simd::edge_test::check(
                    |v| (v & bitdepth_max) as u16,
                    |_| true,
                    |is_port, mode, dst, stride, topleft, w, h, angle| unsafe {
                        let f = if is_port {
                            port.intra_pred[mode]
                        } else {
                            c.intra_pred[mode]
                        };
                        f.unwrap()(dst, stride, topleft, w, h, angle, w, h, bitdepth_max as c_int)
                    },
                );
            }
        }
    }

    /// With NEON allowed every slot `intra_pred_dsp_init_arm` assigns holds a
    /// rebuilt kernel, and with `dav1d_set_cpu_flags_mask(0)` none does.
    #[test]
    fn neon_selects_the_rebuilt_kernels_and_mask_zero_keeps_c() {
        use crate::b8::ipred_tmpl::{dav1d_intra_pred_dsp_init_8bpc, Dav1dIntraPredDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        use crate::simd::arm64::ipred as kernels;
        let rebuilt = [
            kernels::dav1d_ipred_dc_8bpc_neon as AngularIpredFn as usize,
            kernels::dav1d_ipred_cfl_ac_420_8bpc_neon as CflAcFn as usize,
            kernels::dav1d_ipred_cfl_8bpc_neon as CflPredFn as usize,
            kernels::dav1d_pal_pred_8bpc_neon as PalPredFn as usize,
        ];
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let slots = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_intra_pred_dsp_init_8bpc(&mut c);
            dav1d_set_cpu_flags_mask(0);
            dav1d_intra_pred_dsp_init_8bpc(&mut reference);
            dav1d_set_cpu_flags_mask(!0);
            let all = |c: &Translated| {
                let mut v: Vec<Option<usize>> = c.intra_pred.iter().map(|f| f.map(|f| f as usize)).collect();
                v.extend(c.cfl_ac.iter().map(|f| f.map(|f| f as usize)));
                v.extend(c.cfl_pred.iter().map(|f| f.map(|f| f as usize)));
                v.push(c.pal_pred.map(|f| f as usize));
                v
            };
            let (new, old) = (all(&c), all(&reference));
            let changed = new.iter().zip(&old).filter(|(a, b)| a != b).count();
            // DC_PRED, cfl_ac[4:2:0], cfl_pred[DC_PRED], pal_pred
            (changed, [new[0], new[14], new[17], new[23]])
        };
        // 14 intra_pred, 3 cfl_ac, 4 cfl_pred (two of its six slots are unused), pal_pred.
        assert_eq!(slots(!0), (22, rebuilt.map(Some)));
        let (changed, selected) = slots(0);
        assert_eq!(changed, 0);
        assert!(selected.iter().zip(&rebuilt).all(|(s, r)| *s != Some(*r)));
    }

    /// The same at 16 bpc.
    #[test]
    fn neon_selects_the_rebuilt_16bpc_kernels_and_mask_zero_keeps_c() {
        use crate::b16::ipred_tmpl::{dav1d_intra_pred_dsp_init_16bpc, Dav1dIntraPredDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        use crate::simd::arm64::ipred16 as kernels;
        let rebuilt = [
            kernels::dav1d_ipred_dc_16bpc_neon as AngularIpredFn16 as usize,
            kernels::dav1d_ipred_cfl_ac_420_16bpc_neon as CflAcFn16 as usize,
            kernels::dav1d_ipred_cfl_16bpc_neon as CflPredFn16 as usize,
            kernels::dav1d_pal_pred_16bpc_neon as PalPredFn16 as usize,
        ];
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let slots = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_intra_pred_dsp_init_16bpc(&mut c);
            dav1d_set_cpu_flags_mask(0);
            dav1d_intra_pred_dsp_init_16bpc(&mut reference);
            dav1d_set_cpu_flags_mask(!0);
            let all = |c: &Translated| {
                let mut v: Vec<Option<usize>> = c.intra_pred.iter().map(|f| f.map(|f| f as usize)).collect();
                v.extend(c.cfl_ac.iter().map(|f| f.map(|f| f as usize)));
                v.extend(c.cfl_pred.iter().map(|f| f.map(|f| f as usize)));
                v.push(c.pal_pred.map(|f| f as usize));
                v
            };
            let (new, old) = (all(&c), all(&reference));
            let changed = new.iter().zip(&old).filter(|(a, b)| a != b).count();
            // DC_PRED, cfl_ac[4:2:0], cfl_pred[DC_PRED], pal_pred
            (changed, [new[0], new[14], new[17], new[23]])
        };
        // 14 intra_pred, 3 cfl_ac, 4 cfl_pred (two of its six slots are unused), pal_pred.
        assert_eq!(slots(!0), (22, rebuilt.map(Some)));
        let (changed, selected) = slots(0);
        assert_eq!(changed, 0);
        assert!(selected.iter().zip(&rebuilt).all(|(s, r)| *s != Some(*r)));
    }
}
