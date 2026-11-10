/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated by hand from upstream/aom/aom_dsp/arm/subtract_neon.c (ADR 0056).
use core::arch::aarch64::*;

// aom_ports/mem.h
#[inline(always)]
fn CONVERT_TO_SHORTPTR(x: *const u8) -> *mut u16 {
    ((x as usize) << 1) as *mut u16
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_subtract_block_neon(
    rows: i32,
    cols: i32,
    mut diff: *mut i16,
    diff_stride: isize,
    mut src: *const u8,
    src_stride: isize,
    mut pred: *const u8,
    pred_stride: isize,
) {
    if cols > 16 {
        let mut r = rows;
        loop {
            let mut c: isize = 0;
            loop {
                let v_src_00 = vld1q_u8(src.offset(c + 0));
                let v_src_16 = vld1q_u8(src.offset(c + 16));
                let v_pred_00 = vld1q_u8(pred.offset(c + 0));
                let v_pred_16 = vld1q_u8(pred.offset(c + 16));
                let v_diff_lo_00 = vsubl_u8(vget_low_u8(v_src_00), vget_low_u8(v_pred_00));
                let v_diff_hi_00 = vsubl_u8(vget_high_u8(v_src_00), vget_high_u8(v_pred_00));
                let v_diff_lo_16 = vsubl_u8(vget_low_u8(v_src_16), vget_low_u8(v_pred_16));
                let v_diff_hi_16 = vsubl_u8(vget_high_u8(v_src_16), vget_high_u8(v_pred_16));
                vst1q_s16(diff.offset(c + 0), vreinterpretq_s16_u16(v_diff_lo_00));
                vst1q_s16(diff.offset(c + 8), vreinterpretq_s16_u16(v_diff_hi_00));
                vst1q_s16(diff.offset(c + 16), vreinterpretq_s16_u16(v_diff_lo_16));
                vst1q_s16(diff.offset(c + 24), vreinterpretq_s16_u16(v_diff_hi_16));
                c += 32;
                if c >= cols as isize {
                    break;
                }
            }
            diff = diff.offset(diff_stride);
            pred = pred.offset(pred_stride);
            src = src.offset(src_stride);
            r -= 1;
            if r == 0 {
                break;
            }
        }
    } else if cols > 8 {
        let mut r = rows;
        loop {
            let v_src = vld1q_u8(src);
            let v_pred = vld1q_u8(pred);
            let v_diff_lo = vsubl_u8(vget_low_u8(v_src), vget_low_u8(v_pred));
            let v_diff_hi = vsubl_u8(vget_high_u8(v_src), vget_high_u8(v_pred));
            vst1q_s16(diff, vreinterpretq_s16_u16(v_diff_lo));
            vst1q_s16(diff.add(8), vreinterpretq_s16_u16(v_diff_hi));
            diff = diff.offset(diff_stride);
            pred = pred.offset(pred_stride);
            src = src.offset(src_stride);
            r -= 1;
            if r == 0 {
                break;
            }
        }
    } else if cols > 4 {
        let mut r = rows;
        loop {
            let v_src = vld1_u8(src);
            let v_pred = vld1_u8(pred);
            let v_diff = vsubl_u8(v_src, v_pred);
            vst1q_s16(diff, vreinterpretq_s16_u16(v_diff));
            diff = diff.offset(diff_stride);
            pred = pred.offset(pred_stride);
            src = src.offset(src_stride);
            r -= 1;
            if r == 0 {
                break;
            }
        }
    } else {
        let mut r = rows;
        loop {
            let mut c: isize = 0;
            loop {
                *diff.offset(c) = (*src.offset(c) as i32 - *pred.offset(c) as i32) as i16;
                c += 1;
                if c >= cols as isize {
                    break;
                }
            }
            diff = diff.offset(diff_stride);
            pred = pred.offset(pred_stride);
            src = src.offset(src_stride);
            r -= 1;
            if r == 0 {
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_subtract_block_neon(
    rows: i32,
    cols: i32,
    mut diff: *mut i16,
    diff_stride: isize,
    src8: *const u8,
    src_stride: isize,
    pred8: *const u8,
    pred_stride: isize,
) {
    let mut src: *mut u16 = CONVERT_TO_SHORTPTR(src8);
    let mut pred: *mut u16 = CONVERT_TO_SHORTPTR(pred8);

    if cols > 16 {
        let mut r = rows;
        loop {
            let mut c: isize = 0;
            loop {
                let v_src_00 = vld1q_u16(src.offset(c + 0));
                let v_pred_00 = vld1q_u16(pred.offset(c + 0));
                let v_diff_00 = vsubq_u16(v_src_00, v_pred_00);
                let v_src_08 = vld1q_u16(src.offset(c + 8));
                let v_pred_08 = vld1q_u16(pred.offset(c + 8));
                let v_diff_08 = vsubq_u16(v_src_08, v_pred_08);
                vst1q_s16(diff.offset(c + 0), vreinterpretq_s16_u16(v_diff_00));
                vst1q_s16(diff.offset(c + 8), vreinterpretq_s16_u16(v_diff_08));
                c += 16;
                if c >= cols as isize {
                    break;
                }
            }
            diff = diff.offset(diff_stride);
            pred = pred.offset(pred_stride);
            src = src.offset(src_stride);
            r -= 1;
            if r == 0 {
                break;
            }
        }
    } else if cols > 8 {
        let mut r = rows;
        loop {
            let v_src_00 = vld1q_u16(src);
            let v_pred_00 = vld1q_u16(pred);
            let v_diff_00 = vsubq_u16(v_src_00, v_pred_00);
            let v_src_08 = vld1q_u16(src.add(8));
            let v_pred_08 = vld1q_u16(pred.add(8));
            let v_diff_08 = vsubq_u16(v_src_08, v_pred_08);
            vst1q_s16(diff, vreinterpretq_s16_u16(v_diff_00));
            vst1q_s16(diff.add(8), vreinterpretq_s16_u16(v_diff_08));
            diff = diff.offset(diff_stride);
            pred = pred.offset(pred_stride);
            src = src.offset(src_stride);
            r -= 1;
            if r == 0 {
                break;
            }
        }
    } else if cols > 4 {
        let mut r = rows;
        loop {
            let v_src_r0 = vld1q_u16(src);
            let v_src_r1 = vld1q_u16(src.offset(src_stride));
            let v_pred_r0 = vld1q_u16(pred);
            let v_pred_r1 = vld1q_u16(pred.offset(pred_stride));
            let v_diff_r0 = vsubq_u16(v_src_r0, v_pred_r0);
            let v_diff_r1 = vsubq_u16(v_src_r1, v_pred_r1);
            vst1q_s16(diff, vreinterpretq_s16_u16(v_diff_r0));
            vst1q_s16(diff.offset(diff_stride), vreinterpretq_s16_u16(v_diff_r1));
            diff = diff.offset(diff_stride << 1);
            pred = pred.offset(pred_stride << 1);
            src = src.offset(src_stride << 1);
            r -= 2;
            if r == 0 {
                break;
            }
        }
    } else {
        let mut r = rows;
        loop {
            let v_src_r0 = vld1_u16(src);
            let v_src_r1 = vld1_u16(src.offset(src_stride));
            let v_pred_r0 = vld1_u16(pred);
            let v_pred_r1 = vld1_u16(pred.offset(pred_stride));
            let v_diff_r0 = vsub_u16(v_src_r0, v_pred_r0);
            let v_diff_r1 = vsub_u16(v_src_r1, v_pred_r1);
            vst1_s16(diff, vreinterpret_s16_u16(v_diff_r0));
            vst1_s16(diff.offset(diff_stride), vreinterpret_s16_u16(v_diff_r1));
            diff = diff.offset(diff_stride << 1);
            pred = pred.offset(pred_stride << 1);
            src = src.offset(src_stride << 1);
            r -= 2;
            if r == 0 {
                break;
            }
        }
    }
}
