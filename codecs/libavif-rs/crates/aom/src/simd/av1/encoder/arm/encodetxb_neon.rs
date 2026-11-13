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

// Translated by hand from upstream/aom/av1/encoder/arm/encodetxb_neon.c
// (ADR 0056). Intrinsic sequences, helper structure, and names follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. C locals that the
// original leaves uninitialized start zeroed here. The original's lane loads
// through (uint32_t *) / (uint64_t *) casts of byte addresses use
// read_unaligned for the lane value (as `vld1q_lane_u32_unaligned` in
// mem_neon.rs does), and chained C assignments are split right to left.
#![allow(unused_mut, unused_parens, unused_variables, unused_braces, clippy::all)]

use core::arch::aarch64::*;

use crate::simd::aom_dsp__arm__mem_neon::{load_tran_low_to_s16q, vld1q_lane_u32_unaligned};
use crate::simd::types::tran_low_t;

// av1/common/enums.h and av1/common/entropy.h
type TX_SIZE = u8;
type TX_CLASS = u8;
const TX_64X64: TX_SIZE = 4;
const TX_32X32: TX_SIZE = 3;
const TX_32X64: TX_SIZE = 11;
const TX_64X32: TX_SIZE = 12;
const TX_32X16: TX_SIZE = 10;
const TX_16X32: TX_SIZE = 9;
const TX_16X64: TX_SIZE = 17;
const TX_64X16: TX_SIZE = 18;
const TX_PAD_HOR: i32 = 4;
const TX_PAD_TOP: i32 = 0;
const TX_PAD_BOTTOM: i32 = 4;
const TX_PAD_END: i32 = 16;
const SIG_COEF_CONTEXTS_2D: u8 = 26;
const TX_CLASS_2D: TX_CLASS = 0;
const TX_CLASS_HORIZ: TX_CLASS = 1;
const TX_CLASS_VERT: TX_CLASS = 2;

// av1/common/common_data.h
static tx_size_wide: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
static tx_size_high: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// av1/common/blockd.h
#[inline(always)]
fn av1_get_adjusted_tx_size(tx_size: TX_SIZE) -> TX_SIZE {
    match tx_size {
        TX_64X64 | TX_64X32 | TX_32X64 => TX_32X32,
        TX_64X16 => TX_32X16,
        TX_16X64 => TX_16X32,
        _ => tx_size,
    }
}

// av1/common/txb_common.h
#[inline(always)]
fn get_txb_bhl(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[inline(always)]
fn get_txb_wide(tx_size: TX_SIZE) -> i32 {
    tx_size_wide[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[inline(always)]
fn get_txb_high(tx_size: TX_SIZE) -> i32 {
    tx_size_high[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[repr(C, align(16))]
struct Aligned16<T>(T);

#[no_mangle]
pub unsafe extern "C-unwind" fn av1_txb_init_levels_neon(coeff: *const tran_low_t, width: i32,
                                                         height: i32, levels: *mut u8) {
    let stride: i32 = height + TX_PAD_HOR;
    core::ptr::write_bytes(levels.offset(-(TX_PAD_TOP * stride) as isize), 0,
                           (TX_PAD_TOP * stride) as usize);
    core::ptr::write_bytes(levels.offset((stride * width) as isize), 0,
                           (TX_PAD_BOTTOM * stride + TX_PAD_END) as usize);

    let zeros: int32x4_t = vdupq_n_s32(0);
    let mut i: i32 = 0;
    let mut ls: *mut u8 = levels;
    let mut cf: *const tran_low_t = coeff;
    if height == 4 {
        loop {
            let coeffA: int32x4_t = vld1q_s32(cf);
            let coeffB: int32x4_t = vld1q_s32(cf.offset(height as isize));
            let coeffAB: int16x8_t = vcombine_s16(vqmovn_s32(coeffA), vqmovn_s32(coeffB));
            let absAB: int16x8_t = vqabsq_s16(coeffAB);
            let absABs: int8x8_t = vqmovn_s16(absAB);
            let absAB8: int8x16_t = vcombine_s8(absABs, vreinterpret_s8_s32(vget_low_s32(zeros)));
            let lsAB: uint8x16_t =
                vreinterpretq_u8_s32(vzip1q_s32(vreinterpretq_s32_s8(absAB8), zeros));
            vst1q_u8(ls, lsAB);
            ls = ls.offset((stride << 1) as isize);
            cf = cf.offset((height << 1) as isize);
            i += 2;
            if !(i < width) {
                break;
            }
        }
    } else if height == 8 {
        loop {
            let coeffAB: int16x8_t = load_tran_low_to_s16q(cf);
            let absAB: int16x8_t = vqabsq_s16(coeffAB);
            let absAB8: uint8x16_t = vreinterpretq_u8_s8(vcombine_s8(
                vqmovn_s16(absAB),
                vreinterpret_s8_s32(vget_low_s32(zeros)),
            ));
            vst1q_u8(ls, absAB8);
            ls = ls.offset(stride as isize);
            cf = cf.offset(height as isize);
            i += 1;
            if !(i < width) {
                break;
            }
        }
    } else {
        loop {
            let mut j: i32 = 0;
            loop {
                let coeffAB: int16x8_t = load_tran_low_to_s16q(cf);
                let coeffCD: int16x8_t = load_tran_low_to_s16q(cf.add(8));
                let absAB: int16x8_t = vqabsq_s16(coeffAB);
                let absCD: int16x8_t = vqabsq_s16(coeffCD);
                let absABCD: uint8x16_t =
                    vreinterpretq_u8_s8(vcombine_s8(vqmovn_s16(absAB), vqmovn_s16(absCD)));
                vst1q_u8(ls.offset(j as isize), absABCD);
                j += 16;
                cf = cf.add(16);
                if !(j < height) {
                    break;
                }
            }
            core::ptr::write_unaligned(ls.offset(height as isize) as *mut i32, 0);
            ls = ls.offset(stride as isize);
            i += 1;
            if !(i < width) {
                break;
            }
        }
    }
}

// get_4_nz_map_contexts_2d coefficients:
static c_4_po_2d: Aligned16<[[u8; 16]; 2]> = Aligned16([
    [0, 1, 6, 6, 1, 6, 6, 21, 6, 6, 21, 21, 6, 21, 21, 21],
    [0, 16, 16, 16, 16, 16, 16, 16, 6, 6, 21, 21, 6, 21, 21, 21],
]);

// get_4_nz_map_contexts_hor coefficients:
const SIG_COEF_CONTEXTS_2D_X4_051010: u32 = SIG_COEF_CONTEXTS_2D as u32
    + ((SIG_COEF_CONTEXTS_2D as u32 + 5) << 8)
    + ((SIG_COEF_CONTEXTS_2D as u32 + 10) << 16)
    + ((SIG_COEF_CONTEXTS_2D as u32 + 10) << 24);

const S: u8 = SIG_COEF_CONTEXTS_2D;

// get_4_nz_map_contexts_ver coefficients:
static c_4_po_hor: Aligned16<[u8; 16]> = Aligned16([
    S + 0, S + 0, S + 0, S + 0, S + 5, S + 5, S + 5, S + 5, S + 10, S + 10, S + 10, S + 10,
    S + 10, S + 10, S + 10, S + 10,
]);

// get_8_coeff_contexts_2d coefficients:
// if (width == 8)
static c_8_po_2d_8: Aligned16<[[u8; 16]; 2]> = Aligned16([
    [0, 1, 6, 6, 21, 21, 21, 21, 1, 6, 6, 21, 21, 21, 21, 21],
    [6, 6, 21, 21, 21, 21, 21, 21, 6, 21, 21, 21, 21, 21, 21, 21],
]);
// if (width < 8)
static c_8_po_2d_l: Aligned16<[[u8; 16]; 2]> = Aligned16([
    [0, 11, 6, 6, 21, 21, 21, 21, 11, 11, 6, 21, 21, 21, 21, 21],
    [11, 11, 21, 21, 21, 21, 21, 21, 11, 11, 21, 21, 21, 21, 21, 21],
]);

// if (width > 8)
static c_8_po_2d_g: Aligned16<[[u8; 16]; 2]> = Aligned16([
    [0, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16],
    [6, 6, 21, 21, 21, 21, 21, 21, 6, 21, 21, 21, 21, 21, 21, 21],
]);

// get_4_nz_map_contexts_ver coefficients:
static c_8_po_ver: Aligned16<[u8; 16]> = Aligned16([
    S + 0, S + 5, S + 10, S + 10, S + 10, S + 10, S + 10, S + 10, S + 0, S + 5, S + 10, S + 10,
    S + 10, S + 10, S + 10, S + 10,
]);

// get_16n_coeff_contexts_2d coefficients:
// real_width == real_height
static c_16_po_2d_e: Aligned16<[[u8; 16]; 4]> = Aligned16([
    [0, 1, 6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
    [1, 6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
    [6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
    [6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
]);

// real_width < real_height
static c_16_po_2d_g: Aligned16<[[u8; 16]; 3]> = Aligned16([
    [0, 11, 6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
    [11, 11, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
    [11, 11, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
]);

// real_width > real_height
static c_16_po_2d_l: Aligned16<[[u8; 16]; 3]> = Aligned16([
    [16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16],
    [6, 6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
    [6, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21],
]);

// get_16n_coeff_contexts_hor coefficients:
static c_16_po_ver: Aligned16<[u8; 16]> = Aligned16([
    S + 0, S + 5, S + 10, S + 10, S + 10, S + 10, S + 10, S + 10, S + 10, S + 10, S + 10, S + 10,
    S + 10, S + 10, S + 10, S + 10,
]);

// end of coefficients declaration area

#[inline(always)]
unsafe fn load_8bit_4x4_to_1_reg(src: *const u8, byte_stride: i32) -> uint8x16_t {
    let mut v_data: uint32x4_t = vld1q_u32(src as *const u32);
    v_data = vld1q_lane_u32_unaligned::<1>(src.offset((1 * byte_stride) as isize) as *const u32,
                                           v_data);
    v_data = vld1q_lane_u32_unaligned::<2>(src.offset((2 * byte_stride) as isize) as *const u32,
                                           v_data);
    v_data = vld1q_lane_u32_unaligned::<3>(src.offset((3 * byte_stride) as isize) as *const u32,
                                           v_data);

    vreinterpretq_u8_u32(v_data)
}

#[inline(always)]
unsafe fn load_8bit_8x2_to_1_reg(src: *const u8, byte_stride: i32) -> uint8x16_t {
    let mut v_data: uint64x2_t = vld1q_u64(src as *const u64);
    v_data = vsetq_lane_u64::<1>(
        core::ptr::read_unaligned(src.offset((1 * byte_stride) as isize) as *const u64), v_data);

    vreinterpretq_u8_u64(v_data)
}

#[inline(always)]
unsafe fn load_8bit_16x1_to_1_reg(src: *const u8, _byte_stride: i32) -> uint8x16_t {
    vld1q_u8(src)
}

#[inline(always)]
unsafe fn load_levels_4x4x5(mut src: *const u8, mut stride: i32, mut offsets: *const isize, mut level: *mut uint8x16_t) {
    *level.add(0) = load_8bit_4x4_to_1_reg(src.add(1), stride);
    *level.add(1) = load_8bit_4x4_to_1_reg(src.offset(stride as isize), stride);
    *level.add(2) = load_8bit_4x4_to_1_reg(src.offset((*offsets.add(0)) as isize), stride);
    *level.add(3) = load_8bit_4x4_to_1_reg(src.offset((*offsets.add(1)) as isize), stride);
    *level.add(4) = load_8bit_4x4_to_1_reg(src.offset((*offsets.add(2)) as isize), stride);
}

#[inline(always)]
unsafe fn load_levels_8x2x5(mut src: *const u8, mut stride: i32, mut offsets: *const isize, mut level: *mut uint8x16_t) {
    *level.add(0) = load_8bit_8x2_to_1_reg(src.add(1), stride);
    *level.add(1) = load_8bit_8x2_to_1_reg(src.offset(stride as isize), stride);
    *level.add(2) = load_8bit_8x2_to_1_reg(src.offset((*offsets.add(0)) as isize), stride);
    *level.add(3) = load_8bit_8x2_to_1_reg(src.offset((*offsets.add(1)) as isize), stride);
    *level.add(4) = load_8bit_8x2_to_1_reg(src.offset((*offsets.add(2)) as isize), stride);
}

#[inline(always)]
unsafe fn load_levels_16x1x5(mut src: *const u8, mut stride: i32, mut offsets: *const isize, mut level: *mut uint8x16_t) {
    *level.add(0) = load_8bit_16x1_to_1_reg(src.add(1), stride);
    *level.add(1) = load_8bit_16x1_to_1_reg(src.offset(stride as isize), stride);
    *level.add(2) = load_8bit_16x1_to_1_reg(src.offset((*offsets.add(0)) as isize), stride);
    *level.add(3) = load_8bit_16x1_to_1_reg(src.offset((*offsets.add(1)) as isize), stride);
    *level.add(4) = load_8bit_16x1_to_1_reg(src.offset((*offsets.add(2)) as isize), stride);
}

#[inline(always)]
unsafe fn get_coeff_contexts_kernel(mut level: *mut uint8x16_t) -> uint8x16_t {
    let mut const_3: uint8x16_t = vdupq_n_u8(3);
    let mut const_4: uint8x16_t = vdupq_n_u8(4);
    let mut count: uint8x16_t = core::mem::zeroed();
    count = vminq_u8(*level.add(0), const_3);
    *level.add(1) = vminq_u8(*level.add(1), const_3);
    *level.add(2) = vminq_u8(*level.add(2), const_3);
    *level.add(3) = vminq_u8(*level.add(3), const_3);
    *level.add(4) = vminq_u8(*level.add(4), const_3);
    count = vaddq_u8(count, *level.add(1));
    count = vaddq_u8(count, *level.add(2));
    count = vaddq_u8(count, *level.add(3));
    count = vaddq_u8(count, *level.add(4));
    count = vrshrq_n_u8::<1>(count);
    count = vminq_u8(count, const_4);
    return count;
}

#[inline(always)]
unsafe fn get_4_nz_map_contexts_2d(mut levels: *const u8, mut width: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = 4 + TX_PAD_HOR;
    let mut pos_to_offset_large: uint8x16_t = vdupq_n_u8(21);
    let mut pos_to_offset: uint8x16_t =
        if width == 4 { vld1q_u8(c_4_po_2d.0[0].as_ptr()) } else { vld1q_u8(c_4_po_2d.0[1].as_ptr()) };
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    let mut cc: *mut u8 = coeff_contexts;
    debug_assert!(width % 4 == 0);
    let mut col: i32 = width;
    loop {
        load_levels_4x4x5(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel(level.as_mut_ptr());
        count = vaddq_u8(count, pos_to_offset);
        vst1q_u8(cc, count);
        pos_to_offset = pos_to_offset_large;
        levels = levels.offset(((4 * stride)) as isize);
        cc = cc.offset(((16)) as isize);
        col -= 4;
        if !(col != 0) { break; }
    }
    *coeff_contexts.add(0) = 0;
}

#[inline(always)]
unsafe fn get_4_nz_map_contexts_ver(mut levels: *const u8, mut width: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = 4 + TX_PAD_HOR;
    let mut pos_to_offset: uint8x16_t = vreinterpretq_u8_u32(vdupq_n_u32(SIG_COEF_CONTEXTS_2D_X4_051010));
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(width % 4 == 0);
    let mut col: i32 = width;
    loop {
        load_levels_4x4x5(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel(level.as_mut_ptr());
        count = vaddq_u8(count, pos_to_offset);
        vst1q_u8(coeff_contexts, count);
        levels = levels.offset(((4 * stride)) as isize);
        coeff_contexts = coeff_contexts.offset(((16)) as isize);
        col -= 4;
        if !(col != 0) { break; }
    }
}

#[inline(always)]
unsafe fn get_4_nz_map_contexts_hor(mut levels: *const u8, mut width: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = 4 + TX_PAD_HOR;
    let mut pos_to_offset_large: uint8x16_t = vdupq_n_u8(SIG_COEF_CONTEXTS_2D + 10);
    let mut pos_to_offset: uint8x16_t = vld1q_u8(c_4_po_hor.0.as_ptr());
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(width % 4 == 0);
    let mut col: i32 = width;
    loop {
        load_levels_4x4x5(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel(level.as_mut_ptr());
        count = vaddq_u8(count, pos_to_offset);
        vst1q_u8(coeff_contexts, count);
        pos_to_offset = pos_to_offset_large;
        levels = levels.offset(((4 * stride)) as isize);
        coeff_contexts = coeff_contexts.offset(((16)) as isize);
        col -= 4;
        if !(col != 0) { break; }
    }
}

#[inline(always)]
unsafe fn get_8_coeff_contexts_2d(mut levels: *const u8, mut width: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = 8 + TX_PAD_HOR;
    let mut cc: *mut u8 = coeff_contexts;
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    let mut pos_to_offset: [uint8x16_t; 3] = core::mem::zeroed();
    debug_assert!(width % 2 == 0);
    if width == 8 {
        pos_to_offset[0] = vld1q_u8(c_8_po_2d_8.0[0].as_ptr());
        pos_to_offset[1] = vld1q_u8(c_8_po_2d_8.0[1].as_ptr());
    } else if width < 8 {
        pos_to_offset[0] = vld1q_u8(c_8_po_2d_l.0[0].as_ptr());
        pos_to_offset[1] = vld1q_u8(c_8_po_2d_l.0[1].as_ptr());
    } else {
        pos_to_offset[0] = vld1q_u8(c_8_po_2d_g.0[0].as_ptr());
        pos_to_offset[1] = vld1q_u8(c_8_po_2d_g.0[1].as_ptr());
    }
    pos_to_offset[2] = vdupq_n_u8(21);
    let mut col: i32 = width;
    loop {
        load_levels_8x2x5(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel(level.as_mut_ptr());
        count = vaddq_u8(count, pos_to_offset[0]);
        vst1q_u8(cc, count);
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[1] = pos_to_offset[2];
        levels = levels.offset(((2 * stride)) as isize);
        cc = cc.offset(((16)) as isize);
        col -= 2;
        if !(col != 0) { break; }
    }
    *coeff_contexts.add(0) = 0;
}

#[inline(always)]
unsafe fn get_8_coeff_contexts_ver(mut levels: *const u8, mut width: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = 8 + TX_PAD_HOR;
    let mut pos_to_offset: uint8x16_t = vld1q_u8(c_8_po_ver.0.as_ptr());
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(width % 2 == 0);
    let mut col: i32 = width;
    loop {
        load_levels_8x2x5(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel(level.as_mut_ptr());
        count = vaddq_u8(count, pos_to_offset);
        vst1q_u8(coeff_contexts, count);
        levels = levels.offset(((2 * stride)) as isize);
        coeff_contexts = coeff_contexts.offset(((16)) as isize);
        col -= 2;
        if !(col != 0) { break; }
    }
}

#[inline(always)]
unsafe fn get_8_coeff_contexts_hor(mut levels: *const u8, mut width: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = 8 + TX_PAD_HOR;
    let mut pos_to_offset_large: uint8x16_t = vdupq_n_u8(SIG_COEF_CONTEXTS_2D + 10);
    let mut pos_to_offset: uint8x16_t = vcombine_u8(vdup_n_u8(SIG_COEF_CONTEXTS_2D + 0), vdup_n_u8(SIG_COEF_CONTEXTS_2D + 5));
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(width % 2 == 0);
    let mut col: i32 = width;
    loop {
        load_levels_8x2x5(levels, stride, offsets, level.as_mut_ptr());
        count = get_coeff_contexts_kernel(level.as_mut_ptr());
        count = vaddq_u8(count, pos_to_offset);
        vst1q_u8(coeff_contexts, count);
        pos_to_offset = pos_to_offset_large;
        levels = levels.offset(((2 * stride)) as isize);
        coeff_contexts = coeff_contexts.offset(((16)) as isize);
        col -= 2;
        if !(col != 0) { break; }
    }
}

#[inline(always)]
unsafe fn get_16n_coeff_contexts_2d(mut levels: *const u8, mut real_width: i32, mut real_height: i32, mut width: i32, mut height: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = height + TX_PAD_HOR;
    let mut cc: *mut u8 = coeff_contexts;
    let mut col: i32 = width;
    let mut pos_to_offset: [uint8x16_t; 5] = core::mem::zeroed();
    let mut pos_to_offset_large: [uint8x16_t; 3] = core::mem::zeroed();
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(height % 16 == 0);
    pos_to_offset_large[2] = vdupq_n_u8(21);
    if real_width == real_height {
        pos_to_offset[0] = vld1q_u8(c_16_po_2d_e.0[0].as_ptr());
        pos_to_offset[1] = vld1q_u8(c_16_po_2d_e.0[1].as_ptr());
        pos_to_offset[2] = vld1q_u8(c_16_po_2d_e.0[2].as_ptr());
        pos_to_offset[3] = vld1q_u8(c_16_po_2d_e.0[3].as_ptr());
        pos_to_offset_large[1] = pos_to_offset_large[2];
        pos_to_offset_large[0] = pos_to_offset_large[1];
        pos_to_offset[4] = pos_to_offset_large[0];
    } else if real_width < real_height {
        pos_to_offset[0] = vld1q_u8(c_16_po_2d_g.0[0].as_ptr());
        pos_to_offset[1] = vld1q_u8(c_16_po_2d_g.0[1].as_ptr());
        pos_to_offset[4] = vld1q_u8(c_16_po_2d_g.0[2].as_ptr());
        pos_to_offset[3] = pos_to_offset[4];
        pos_to_offset[2] = pos_to_offset[3];
        pos_to_offset_large[1] = pos_to_offset_large[2];
        pos_to_offset_large[0] = pos_to_offset_large[1];
    } else {
        // real_width > real_height
        pos_to_offset[1] = vld1q_u8(c_16_po_2d_l.0[0].as_ptr());
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[2] = vld1q_u8(c_16_po_2d_l.0[1].as_ptr());
        pos_to_offset[3] = vld1q_u8(c_16_po_2d_l.0[2].as_ptr());
        pos_to_offset[4] = pos_to_offset_large[2];
        pos_to_offset_large[1] = vdupq_n_u8(16);
        pos_to_offset_large[0] = pos_to_offset_large[1];
    }
    loop {
        let mut h: i32 = height;
        loop {
            load_levels_16x1x5(levels, stride, offsets, level.as_mut_ptr());
            count = get_coeff_contexts_kernel(level.as_mut_ptr());
            count = vaddq_u8(count, pos_to_offset[0]);
            vst1q_u8(cc, count);
            levels = levels.offset(((16)) as isize);
            cc = cc.offset(((16)) as isize);
            h -= 16;
            pos_to_offset[0] = pos_to_offset_large[0];
            if !(h != 0) { break; }
        }
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[1] = pos_to_offset[2];
        pos_to_offset[2] = pos_to_offset[3];
        pos_to_offset[3] = pos_to_offset[4];
        pos_to_offset_large[0] = pos_to_offset_large[1];
        pos_to_offset_large[1] = pos_to_offset_large[2];
        levels = levels.offset(((TX_PAD_HOR)) as isize);
        if !({ col -= 1; col } != 0) { break; }
    }
    *coeff_contexts.add(0) = 0;
}

#[inline(always)]
unsafe fn get_16n_coeff_contexts_ver(mut levels: *const u8, mut width: i32, mut height: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = height + TX_PAD_HOR;
    let mut pos_to_offset_large: uint8x16_t = vdupq_n_u8(SIG_COEF_CONTEXTS_2D + 10);
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(height % 16 == 0);
    let mut col: i32 = width;
    loop {
        let mut pos_to_offset: uint8x16_t = vld1q_u8(c_16_po_ver.0.as_ptr());
        let mut h: i32 = height;
        loop {
            load_levels_16x1x5(levels, stride, offsets, level.as_mut_ptr());
            count = get_coeff_contexts_kernel(level.as_mut_ptr());
            count = vaddq_u8(count, pos_to_offset);
            vst1q_u8(coeff_contexts, count);
            pos_to_offset = pos_to_offset_large;
            levels = levels.offset(((16)) as isize);
            coeff_contexts = coeff_contexts.offset(((16)) as isize);
            h -= 16;
            if !(h != 0) { break; }
        }
        levels = levels.offset(((TX_PAD_HOR)) as isize);
        if !({ col -= 1; col } != 0) { break; }
    }
}

#[inline(always)]
unsafe fn get_16n_coeff_contexts_hor(mut levels: *const u8, mut width: i32, mut height: i32, mut offsets: *const isize, mut coeff_contexts: *mut u8) {
    let mut stride: i32 = height + TX_PAD_HOR;
    let mut pos_to_offset: [uint8x16_t; 3] = core::mem::zeroed();
    let mut count: uint8x16_t = core::mem::zeroed();
    let mut level: [uint8x16_t; 5] = core::mem::zeroed();
    debug_assert!(height % 16 == 0);
    pos_to_offset[0] = vdupq_n_u8(SIG_COEF_CONTEXTS_2D + 0);
    pos_to_offset[1] = vdupq_n_u8(SIG_COEF_CONTEXTS_2D + 5);
    pos_to_offset[2] = vdupq_n_u8(SIG_COEF_CONTEXTS_2D + 10);
    let mut col: i32 = width;
    loop {
        let mut h: i32 = height;
        loop {
            load_levels_16x1x5(levels, stride, offsets, level.as_mut_ptr());
            count = get_coeff_contexts_kernel(level.as_mut_ptr());
            count = vaddq_u8(count, pos_to_offset[0]);
            vst1q_u8(coeff_contexts, count);
            levels = levels.offset(((16)) as isize);
            coeff_contexts = coeff_contexts.offset(((16)) as isize);
            h -= 16;
            if !(h != 0) { break; }
        }
        pos_to_offset[0] = pos_to_offset[1];
        pos_to_offset[1] = pos_to_offset[2];
        levels = levels.offset(((TX_PAD_HOR)) as isize);
        if !({ col -= 1; col } != 0) { break; }
    }
}

// Note: levels[] must be in the range [0, 127], inclusive.
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_get_nz_map_contexts_neon(mut levels: *const u8, mut scan: *const i16, mut eob: i32, mut tx_size: TX_SIZE, mut tx_class: TX_CLASS, mut coeff_contexts: *mut i8) {
    let mut last_idx: i32 = eob - 1;
    if last_idx == 0 {
        *coeff_contexts.add(0) = 0;
        return;
    }
    let mut coefficients: *mut u8 = coeff_contexts as *mut u8;
    let mut real_width: i32 = tx_size_wide[tx_size as usize];
    let mut real_height: i32 = tx_size_high[tx_size as usize];
    let mut width: i32 = get_txb_wide(tx_size);
    let mut height: i32 = get_txb_high(tx_size);
    let mut stride: i32 = height + TX_PAD_HOR;
    let mut offsets: [isize; 3] = core::mem::zeroed();
    /* coeff_contexts must be 16 byte aligned. */
    debug_assert!(coeff_contexts as isize & 0xf == 0);
    if tx_class == TX_CLASS_2D {
        offsets[0] = (0 * stride + 2) as isize;
        offsets[1] = (1 * stride + 1) as isize;
        offsets[2] = (2 * stride + 0) as isize;
        if height == 4 {
            get_4_nz_map_contexts_2d(levels, width, offsets.as_mut_ptr(), coefficients);
        } else if height == 8 {
            get_8_coeff_contexts_2d(levels, width, offsets.as_mut_ptr(), coefficients);
        } else {
            get_16n_coeff_contexts_2d(levels, real_width, real_height, width, height, offsets.as_mut_ptr(), coefficients);
        }
    } else if tx_class == TX_CLASS_HORIZ {
        offsets[0] = (2 * stride) as isize;
        offsets[1] = (3 * stride) as isize;
        offsets[2] = (4 * stride) as isize;
        if height == 4 {
            get_4_nz_map_contexts_hor(levels, width, offsets.as_mut_ptr(), coefficients);
        } else if height == 8 {
            get_8_coeff_contexts_hor(levels, width, offsets.as_mut_ptr(), coefficients);
        } else {
            get_16n_coeff_contexts_hor(levels, width, height, offsets.as_mut_ptr(), coefficients);
        }
    } else {
        // TX_CLASS_VERT
        offsets[0] = (2) as isize;
        offsets[1] = (3) as isize;
        offsets[2] = (4) as isize;
        if height == 4 {
            get_4_nz_map_contexts_ver(levels, width, offsets.as_mut_ptr(), coefficients);
        } else if height == 8 {
            get_8_coeff_contexts_ver(levels, width, offsets.as_mut_ptr(), coefficients);
        } else {
            get_16n_coeff_contexts_ver(levels, width, height, offsets.as_mut_ptr(), coefficients);
        }
    }
    let mut bhl: i32 = get_txb_bhl(tx_size);
    let mut pos: i32 = *scan.offset(last_idx as isize) as i32;
    if last_idx <= (width << bhl) / 8 {
        *coeff_contexts.offset(pos as isize) = 1;
    } else if last_idx <= (width << bhl) / 4 {
        *coeff_contexts.offset(pos as isize) = 2;
    } else {
        *coeff_contexts.offset(pos as isize) = 3;
    }
}

