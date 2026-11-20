/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/cnn_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// The two statement macros of the file (PERFORM_CONVOLVE_FOR_3_5X5_BLOCKS,
// PERFORM_CONVOLVE_FOR_1_5X5_BLOCK) were expanded with the C preprocessor
// before printing; their `do { ... } while (0)` stays a loop that runs once.
// Each `_mm*_mul_ps` and `_mm*_add_ps` and each scalar `*` and `+` is a
// separate rounding, as in the original built with -ffp-contract=off
// (ADR 0024); Rust never contracts.
//
// Translation notes:
// - Every function carries `avx2`, the flag upstream compiles the file with.
//   The RTCD specializations are exported under their upstream names.
// - Pointer and array parameters stay raw pointers; local arrays are Rust
//   arrays and start zeroed. DECLARE_ALIGNED arrays keep their alignment
//   through `Aligned16`/`Aligned32`.
// - C integer conversions are explicit (`as`); `assert` becomes
//   `debug_assert!`. Immediate operands become const generics; shift counts
//   that are run-time `int` values use the count-register intrinsics, which is
//   what GCC and Clang emit for a non-constant count.
// - Scalar accesses through cast pointers (`*(int *)p`) are unaligned reads
//   and writes, as x86 compilers emit them.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens
)]
#![allow(unused_variables, dead_code, clippy::all)]

use crate::av1__encoder__cnn::{av1_cnn_convolve_no_maxpool_padding_valid_c, CNN_LAYER_CONFIG};
use core::arch::x86_64::*;

#[repr(C, align(16))]
struct Aligned16<T>(T);
#[repr(C, align(32))]
struct Aligned32<T>(T);

// This mask rearranges source pixels in the order shown below.
// shuffle_src_layer0[0][8]: applied on source pixels 0 to 7.
// shuffle_src_layer0[1][8]: applied on source pixels 7 to 14.
// This shuffling is needed to process 3 5x5 blocks which need
// source pixels in the following order.
// 1st 5x5 block: source pixels needed are 0 to 4,
// 2nd 5x5 block: source pixels needed are 4 to 8,
// 3rd 5x5 block: source pixels needed are 8 to 12.
// Source pixels are loaded like mentioned below.
// load_src0 : 0, 1, 2, 3, 4, 5, 6, 7
// load_src1 : 7, 8, 9, 10, 11, 12, 13, 14
// After applying masks, source bytes will be in the order:
// load_src0 : 0, 1, 2, 3, 4, 4, 5, 6
//             consists 5 pixels needed for 1st 5x5 block and
//             first 3 pixels needed for 2nd 5x5 block.
// load_src1 : 7, 8, 8, 9, 10, 11, 12, x
//             consists last 2 pixels needed for 2nd 5x5 block and
//             5 pixels needed for 3rd 5x5 block.
static shuffle_src_layer0: Aligned32<[[u32; 8]; 2]> =
    Aligned32([[0, 1, 2, 3, 4, 4, 5, 6], [0, 1, 1, 2, 3, 4, 5, 0]]);

// This mask rearrange the weights to match shuffled source pixels order.
static shuffle_weight_layer0: Aligned32<[[u32; 8]; 2]> =
    Aligned32([[0, 1, 2, 3, 4, 0, 1, 2], [3, 4, 0, 1, 2, 3, 4, 0]]);

// Shuffle mask used to rearrange weights corresponding to layer 1 and layer 2.
// For layer 1 and layer 2, convolution happens at 2x2 as filter_width and
// filter_height are equal to 2. So rearranging the weights in the
// order shown below to match source pixels. Basically this mask replicates
// the weights across the width of 2.
static shuffle_weight_layer_1_and_2: Aligned32<[[u32; 8]; 2]> =
    Aligned32([[0, 1, 0, 1, 0, 1, 0, 1], [2, 3, 2, 3, 2, 3, 2, 3]]);

// After the stages of multiplication and accumulation, the output values
// in the register will be jumbled. In order to store register into
// output buffer in a proper way, the following mask is applied on output
// register.
static shuffle_output_layer_1_and_2: Aligned32<[u32; 8]> = Aligned32([0, 1, 4, 5, 2, 3, 6, 7]);

// Load weights needed for layer 0 (for 5x5 block processing),
// and fill the registers appropriately to match source pixel mapping.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn prepare_weights_for_5x5_convolve(
    layer_config_weights: *const f32,
    mut off: i32,
    weight: *mut [f32; 8],
    cstep: i32,
    shuffle_weight: *mut __m256,
    weight_mask_0: __m256i,
    weight_mask_1: __m256i,
) {
    for row in 0..5 {
        for col in 0..5 {
            (*weight.offset(row as isize))[col as usize] =
                *layer_config_weights.offset(off as isize);
            off += cstep;
        }
    }
    *shuffle_weight = _mm256_loadu_ps((*weight).as_mut_ptr() as *const f32);
    *shuffle_weight.offset(1) = _mm256_loadu_ps((*weight.offset(1)).as_mut_ptr() as *const f32);
    *shuffle_weight.offset(2) = _mm256_loadu_ps((*weight.offset(2)).as_mut_ptr() as *const f32);
    *shuffle_weight.offset(3) = _mm256_loadu_ps((*weight.offset(3)).as_mut_ptr() as *const f32);
    *shuffle_weight.offset(4) = _mm256_loadu_ps((*weight.offset(4)).as_mut_ptr() as *const f32);
    *shuffle_weight = _mm256_permutevar8x32_ps(*shuffle_weight, weight_mask_0);
    *shuffle_weight.offset(1) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(1), weight_mask_0);
    *shuffle_weight.offset(2) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(2), weight_mask_0);
    *shuffle_weight.offset(3) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(3), weight_mask_0);
    *shuffle_weight.offset(4) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(4), weight_mask_0);
    *shuffle_weight.offset(5) = _mm256_permutevar8x32_ps(*shuffle_weight, weight_mask_1);
    *shuffle_weight.offset(6) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(1), weight_mask_1);
    *shuffle_weight.offset(7) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(2), weight_mask_1);
    *shuffle_weight.offset(8) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(3), weight_mask_1);
    *shuffle_weight.offset(9) = _mm256_permutevar8x32_ps(*shuffle_weight.offset(4), weight_mask_1);
}

// For each row, loads source pixels 0 to 7(load_src_0), 7 to 14(load_src_1) and
// arranges them appropriately to process 3 blocks.
// Load masks needed for shuffling of output and weights.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load_shuffle_masks_for_2x2_convolve(
    output_mask: *mut __m256i,
    weight_mask: *mut __m256i,
) {
    // Load shuffle buffer needed to sort the output.
    *output_mask = _mm256_load_si256(shuffle_output_layer_1_and_2.0.as_ptr() as *const __m256i);
    // Load shuffle buffers needed for weight.
    *weight_mask = _mm256_load_si256(shuffle_weight_layer_1_and_2.0[0].as_ptr() as *const __m256i);
    *weight_mask.offset(1) =
        _mm256_load_si256(shuffle_weight_layer_1_and_2.0[1].as_ptr() as *const __m256i);
}

// Load weights needed for layer 1 and 2 (for 2x2 block processing),
// and fill the registers appropriately to match source pixel mapping.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn prepare_weights_for_2x2_convolve(
    layer_config_weights: *const f32,
    mut off: i32,
    cstep: i32,
    shuffle_weight: *mut __m256,
    weight_mask: *mut __m256i,
) {
    // Weights needed for 2x2 block.
    let mut weight: [f32; 4] = [0 as f32, 0.0, 0.0, 0.0];
    for i in 0..4 {
        weight[i as usize] = *layer_config_weights.offset(off as isize);
        off += cstep;
    }
    let weight_vec: __m256 =
        _mm256_castps128_ps256(_mm_loadu_ps(weight.as_mut_ptr() as *const f32));
    *shuffle_weight = _mm256_permutevar8x32_ps(weight_vec, *weight_mask);
    *shuffle_weight.offset(1) = _mm256_permutevar8x32_ps(weight_vec, *weight_mask.offset(1));
}

// Do convolution of one 5x5 block.
// Do convolution on 8 horizontal 2x2 blocks.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn perform_convolve_for_8h_2x2_blocks(
    input_ptr: *const f32,
    in_stride: i32,
    weight: *mut __m256,
    out_accum: *mut __m256,
    shuffle_output_mask: __m256i,
) {
    let mut load_src: [__m256; 4] = core::mem::zeroed();
    // Load input into source registers.
    load_src[0] = _mm256_loadu_ps(input_ptr);
    load_src[1] = _mm256_loadu_ps(input_ptr.offset(8));
    load_src[2] = _mm256_loadu_ps(input_ptr.offset(in_stride as isize));
    load_src[3] = _mm256_loadu_ps(input_ptr.offset(in_stride as isize).offset(8));
    // Multiply the loaded input with corresponding weights.
    load_src[0] = _mm256_mul_ps(load_src[0], *weight);
    load_src[1] = _mm256_mul_ps(load_src[1], *weight);
    load_src[2] = _mm256_mul_ps(load_src[2], *weight.offset(1));
    load_src[3] = _mm256_mul_ps(load_src[3], *weight.offset(1));
    // Accumulate across 2x2 blocks.
    load_src[0] = _mm256_add_ps(load_src[0], load_src[2]);
    load_src[1] = _mm256_add_ps(load_src[1], load_src[3]);
    load_src[0] = _mm256_hadd_ps(load_src[0], load_src[1]);
    // Sort the output in order to store into output buffer.
    load_src[0] = _mm256_permutevar8x32_ps(load_src[0], shuffle_output_mask);
    *out_accum = _mm256_add_ps(*out_accum, load_src[0]);
}

// Do convolution on 8 (4 horizontal x 2 vertical) 2x2 blocks.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn perform_convolve_for_4hx2v_2x2_blocks(
    input_ptr: *const f32,
    in_stride: i32,
    weight: *mut __m256,
    out_accum: *mut __m256,
    shuffle_output_mask: __m256i,
) {
    let mut load_src: [__m256; 4] = core::mem::zeroed();
    // Load input into source registers.
    load_src[0] = _mm256_loadu_ps(input_ptr);
    load_src[1] = _mm256_loadu_ps(input_ptr.offset(in_stride as isize));
    load_src[2] = _mm256_loadu_ps(input_ptr.offset((in_stride * 2) as isize));
    load_src[3] = _mm256_loadu_ps(input_ptr.offset((in_stride * 3) as isize));
    // Multiply the loaded input with corresponding weights.
    load_src[0] = _mm256_mul_ps(load_src[0], *weight);
    load_src[1] = _mm256_mul_ps(load_src[1], *weight.offset(1));
    load_src[2] = _mm256_mul_ps(load_src[2], *weight);
    load_src[3] = _mm256_mul_ps(load_src[3], *weight.offset(1));
    // Accumulate across 2x2 blocks.
    load_src[0] = _mm256_add_ps(load_src[0], load_src[1]);
    load_src[2] = _mm256_add_ps(load_src[2], load_src[3]);
    load_src[0] = _mm256_hadd_ps(load_src[0], load_src[2]);
    // Sort the output in order to store into output buffer.
    load_src[0] = _mm256_permutevar8x32_ps(load_src[0], shuffle_output_mask);
    *out_accum = _mm256_add_ps(*out_accum, load_src[0]);
}

// AVX2 variant of av1_cnn_convolve_no_maxpool_padding_valid_c(), when
// filter_width and filter_height are equal to 5.
// CNN convolve parsing is based on av1_intra_mode_cnn_partition_cnn_config.
// Based on the configuration set for each layer, the current encoder
// always chooses the case of no_maxpool_padding_valid.
// And also for layer 0 convolution happens at 5x5 level as the
// filter_width and filter_height are set as 5.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cnn_convolve_no_maxpool_padding_valid_5x5_avx2(
    input: *mut *const f32,
    in_width: i32,
    in_height: i32,
    in_stride: i32,
    layer_config: *const CNN_LAYER_CONFIG,
    output: *mut *mut f32,
    out_stride: i32,
    start_idx: i32,
    cstep: i32,
    channel_step: i32,
) {
    let kFilterWidth: i32 = 5;
    let kFilterHeight: i32 = 5;
    let kSkipWidth: i32 = 4;
    let kSkipHeight: i32 = 4;
    debug_assert!(
        (*layer_config).filter_width == kFilterWidth
            && (*layer_config).filter_height == kFilterHeight
    );
    debug_assert!(
        (*layer_config).skip_width == kSkipWidth && (*layer_config).skip_height == kSkipHeight
    );
    // Load shuffle buffers needed for source.
    let block0_1: __m256i = _mm256_load_si256(shuffle_src_layer0.0[0].as_ptr() as *const __m256i);
    let block1_2: __m256i = _mm256_load_si256(shuffle_src_layer0.0[1].as_ptr() as *const __m256i);
    // Load shuffle buffers needed for weight.
    let weight_mask_0: __m256i =
        _mm256_load_si256(shuffle_weight_layer0.0[0].as_ptr() as *const __m256i);
    let weight_mask_1: __m256i =
        _mm256_load_si256(shuffle_weight_layer0.0[1].as_ptr() as *const __m256i);
    // Width needs to be moved to go to next iteration of processing 3 5x5 blocks.
    let kSkipWidthForNextIter: i32 = kSkipWidth * 3;
    // Minimum width required to process 3 5x5 blocks at a time.
    // min width (for processing 3 5x5 block) = 2*skip_width + filter_width
    // Here, skip_width specifies how much width we should move while processing
    // next block convolution and filter_width specifies for how many pixels
    // filter needs to be applied.
    let kMinWidthFor3_5x5Blocks: i32 = kSkipWidth * 2 + kFilterWidth;
    {
        let mut i: i32 = start_idx;
        while i < (*layer_config).out_channels {
            let out_ch_bias: f32 = *(*layer_config).bias.offset(i as isize);
            for k in 0..(*layer_config).in_channels {
                let mut shuffle_weight: [__m256; 10] = core::mem::zeroed();
                // Weights needed are 5x5, for SIMD purpose made this array as 5x8.
                let mut weight: [[f32; 8]; 5] = [
                    [0 as f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    [0.0; 8],
                    [0.0; 8],
                    [0.0; 8],
                    [0.0; 8],
                ];
                let mut off: i32 = k * (*layer_config).out_channels + i;
                // In layer 0, the convolution process happens at 5x5.
                // The weights needed for 5x5 block are same across the in-channels,
                // which is why the load of weights happens once for each in-channel.
                prepare_weights_for_5x5_convolve(
                    (*layer_config).weights,
                    off,
                    weight.as_mut_ptr(),
                    cstep,
                    shuffle_weight.as_mut_ptr(),
                    weight_mask_0,
                    weight_mask_1,
                );
                {
                    let mut h: i32 = 0;
                    let mut u: i32 = 0;
                    while h < in_height - kFilterHeight + 1 {
                        let out_h: i32 = u * out_stride;
                        let mut v: i32 = 0;
                        let mut w: i32 = 0;
                        let mut rem_width: i32 = in_width;
                        // Processing 3 5x5 blocks at a time, if sufficient width is present.
                        while rem_width >= kMinWidthFor3_5x5Blocks {
                            let mut load_src_0: __m256;
                            let mut load_src_1: __m256;
                            let mut accum_src_0: __m256 = _mm256_setzero_ps();
                            let mut accum_src_1: __m256 = _mm256_setzero_ps();
                            let mut input_ptr: *const f32 =
                                (*input.offset(k as isize)).offset((h * in_stride + w) as isize);
                            loop {
                                for row in 0..5 {
                                    load_src_0 = _mm256_loadu_ps(input_ptr);
                                    load_src_1 = _mm256_loadu_ps(input_ptr.offset(7));
                                    load_src_0 = _mm256_permutevar8x32_ps(load_src_0, block0_1);
                                    load_src_1 = _mm256_permutevar8x32_ps(load_src_1, block1_2);
                                    load_src_0 = _mm256_mul_ps(
                                        load_src_0,
                                        shuffle_weight[(0 + row) as usize],
                                    );
                                    load_src_1 = _mm256_mul_ps(
                                        load_src_1,
                                        shuffle_weight[(5 + row) as usize],
                                    );
                                    accum_src_0 = _mm256_add_ps(load_src_0, accum_src_0);
                                    accum_src_1 = _mm256_add_ps(load_src_1, accum_src_1);
                                    input_ptr = input_ptr.offset(in_stride as isize);
                                }
                                if !(0 != 0) {
                                    break;
                                }
                            }
                            // Accumulate across column.
                            let mut accum: __m256 = _mm256_hadd_ps(accum_src_0, accum_src_1);
                            let mut tmp_reg_0: __m128 = _mm256_extractf128_ps::<1>(accum_src_0);
                            let mut tmp_reg_1: __m128 = _mm256_extractf128_ps::<1>(accum_src_1);
                            let mut accum_l: __m128 = _mm256_castps256_ps128(accum);
                            let mut accum_h: __m128 = _mm256_extractf128_ps::<1>(accum);
                            let mut tmp_reg_2: __m128 = _mm_add_ps(accum_l, tmp_reg_0);
                            let mut tmp_reg_3: __m128 = _mm_add_ps(tmp_reg_0, accum_h);
                            let mut tmp_reg_4: __m128 = _mm_add_ps(tmp_reg_1, accum_h);
                            // 1st 5x5 block output.
                            *(*output.offset(i as isize)).offset((out_h + v) as isize) = out_ch_bias
                                + _mm_cvtss_f32(tmp_reg_2)
                                + _mm_cvtss_f32(_mm_shuffle_ps::<1>(accum_l, accum_l));
                            // 2nd 5x5 block output.
                            *(*output.offset(i as isize)).offset((out_h + v + 1) as isize) =
                                out_ch_bias
                                    + _mm_cvtss_f32(_mm_shuffle_ps::<1>(tmp_reg_3, tmp_reg_3))
                                    + _mm_cvtss_f32(_mm_shuffle_ps::<2>(accum_l, accum_l));
                            // 3rd 5x5 block output.
                            *(*output.offset(i as isize)).offset((out_h + v + 2) as isize) =
                                out_ch_bias
                                    + _mm_cvtss_f32(_mm_shuffle_ps::<2>(tmp_reg_4, tmp_reg_4))
                                    + _mm_cvtss_f32(_mm_shuffle_ps::<3>(accum_l, accum_l));
                            v += 3;
                            w += kSkipWidthForNextIter;
                            rem_width -= kSkipWidthForNextIter;
                        }
                        // Process remaining blocks as single 5x5 block at a time.
                        while rem_width >= kFilterWidth {
                            let mut last_column_sum: f32 = 0 as f32;
                            let mut accum: __m128 = _mm_setzero_ps();
                            let mut input_ptr: *const f32 =
                                (*input.offset(k as isize)).offset((h * in_stride + w) as isize);
                            loop {
                                let mut load_src: [__m128; 5] = core::mem::zeroed();
                                load_src[0] = _mm_loadu_ps(input_ptr);
                                last_column_sum += *input_ptr.offset(4) * weight[0][4];
                                input_ptr = input_ptr.offset(in_stride as isize);
                                load_src[1] = _mm_loadu_ps(input_ptr);
                                last_column_sum += *input_ptr.offset(4) * weight[1][4];
                                input_ptr = input_ptr.offset(in_stride as isize);
                                load_src[2] = _mm_loadu_ps(input_ptr);
                                last_column_sum += *input_ptr.offset(4) * weight[2][4];
                                input_ptr = input_ptr.offset(in_stride as isize);
                                load_src[3] = _mm_loadu_ps(input_ptr);
                                last_column_sum += *input_ptr.offset(4) * weight[3][4];
                                input_ptr = input_ptr.offset(in_stride as isize);
                                load_src[4] = _mm_loadu_ps(input_ptr);
                                last_column_sum += *input_ptr.offset(4) * weight[4][4];
                                load_src[0] = _mm_mul_ps(
                                    load_src[0],
                                    _mm256_castps256_ps128(shuffle_weight[0]),
                                );
                                load_src[1] = _mm_mul_ps(
                                    load_src[1],
                                    _mm256_castps256_ps128(shuffle_weight[1]),
                                );
                                load_src[2] = _mm_mul_ps(
                                    load_src[2],
                                    _mm256_castps256_ps128(shuffle_weight[2]),
                                );
                                load_src[3] = _mm_mul_ps(
                                    load_src[3],
                                    _mm256_castps256_ps128(shuffle_weight[3]),
                                );
                                load_src[4] = _mm_mul_ps(
                                    load_src[4],
                                    _mm256_castps256_ps128(shuffle_weight[4]),
                                );
                                accum = _mm_add_ps(load_src[0], accum);
                                load_src[1] = _mm_add_ps(load_src[1], load_src[2]);
                                load_src[3] = _mm_add_ps(load_src[3], load_src[4]);
                                load_src[1] = _mm_add_ps(load_src[1], load_src[3]);
                                accum = _mm_add_ps(accum, load_src[1]);
                                if !(0 != 0) {
                                    break;
                                }
                            }
                            // Accumulate across column.
                            accum = _mm_hadd_ps(accum, accum);
                            *(*output.offset(i as isize)).offset((out_h + v) as isize) = out_ch_bias
                                + last_column_sum
                                + _mm_cvtss_f32(accum)
                                + _mm_cvtss_f32(_mm_shuffle_ps::<1>(accum, accum));
                            v += 1;
                            w += kSkipWidth;
                            rem_width -= kSkipWidth;
                        }
                        h += kSkipHeight;
                        u += 1;
                    }
                }
            }
            i += channel_step;
        }
    }
}

// AVX2 implementation for layer 1.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cnn_convolve_no_maxpool_padding_valid_layer1_avx2(
    input: *mut *const f32,
    in_stride: i32,
    layer_config: *const CNN_LAYER_CONFIG,
    output: *mut *mut f32,
    out_stride: i32,
    start_idx: i32,
    cstep: i32,
    channel_step: i32,
) {
    let mut weight_mask: [__m256i; 2] = core::mem::zeroed();
    let mut shuffle_output_mask: __m256i = core::mem::zeroed();
    load_shuffle_masks_for_2x2_convolve(&raw mut shuffle_output_mask, weight_mask.as_mut_ptr());
    let kInHeight: i32 = 16;
    let kFilterHeight: i32 = 2;
    let kSkipHeight: i32 = 2;
    {
        let mut i: i32 = start_idx;
        while i < (*layer_config).out_channels {
            let mut bias_reg: __m256 = _mm256_set1_ps(*(*layer_config).bias.offset(i as isize));
            // out_accum registers are used to store the 2x2 convolve outputs
            // (calculated over input block size), which are accumulated across the
            // in_channels. As per the design, each iteration of for loop processes 8
            // (horizontal) 2x2 blocks and stores in corresponding out_accum register
            // (as input size is 16x16, a total of 64 2x2 blocks are present and 8
            // out_accum registers are enough to store the outputs).
            // Hence for loops corresponding to 'j' and 'h', below, run over the number
            // of out_accum registers.
            let mut out_accum: [__m256; 8] = core::mem::zeroed();
            for j in 0..8 {
                out_accum[j as usize] = bias_reg;
            }
            for k in 0..(*layer_config).in_channels {
                let mut shuffle_weight: [__m256; 2] = core::mem::zeroed();
                let mut off: i32 = k * (*layer_config).out_channels + i;
                // In layer 1, the convolution process happens at 2x2.
                // The weights needed for 2x2 block are same across the in-channels,
                // which is why the load of weights happens once for each in-channel.
                prepare_weights_for_2x2_convolve(
                    (*layer_config).weights,
                    off,
                    cstep,
                    shuffle_weight.as_mut_ptr(),
                    weight_mask.as_mut_ptr(),
                );
                {
                    let mut h: i32 = 0;
                    let mut u: i32 = 0;
                    while h < kInHeight - kFilterHeight + 1 {
                        let mut input_ptr: *const f32 =
                            (*input.offset(k as isize)).offset((h * in_stride) as isize);
                        perform_convolve_for_8h_2x2_blocks(
                            input_ptr,
                            in_stride,
                            shuffle_weight.as_mut_ptr(),
                            out_accum.as_mut_ptr().offset(u as isize),
                            shuffle_output_mask,
                        );
                        h += kSkipHeight;
                        u += 1;
                    }
                }
            }
            // Store output of layer 1.
            for j in 0..8 {
                _mm256_storeu_ps(
                    (*output.offset(i as isize)).offset((j * out_stride) as isize),
                    out_accum[j as usize],
                );
            }
            i += channel_step;
        }
    }
}

// AVX2 implementation for layer 2.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cnn_convolve_no_maxpool_padding_valid_layer2_avx2(
    input: *mut *const f32,
    in_stride: i32,
    layer_config: *const CNN_LAYER_CONFIG,
    output: *mut *mut f32,
    out_stride: i32,
    start_idx: i32,
    cstep: i32,
    channel_step: i32,
) {
    let mut weight_mask: [__m256i; 2] = core::mem::zeroed();
    let mut shuffle_output_mask: __m256i = core::mem::zeroed();
    load_shuffle_masks_for_2x2_convolve(&raw mut shuffle_output_mask, weight_mask.as_mut_ptr());
    let kInHeight: i32 = 8;
    let kFilterHeight: i32 = 2;
    let kSkipHeight: i32 = 2;
    {
        let mut i: i32 = start_idx;
        while i < (*layer_config).out_channels {
            let mut bias_reg: __m256 = _mm256_set1_ps(*(*layer_config).bias.offset(i as isize));
            // out_accum registers are used to store the 2x2 convolve outputs
            // (calculated over input block size), which are accumulated across the
            // in_channels. As per the design, each iteration of for loop processes 8
            // (4 horizontal x 2 vertical) 2x2 blocks and stores in corresponding
            // out_accum register (as input size is 8x8, a total of 16 2x2 blocks are
            // present and 2 out_accum registers are enough to store the outputs).
            // Hence for loops corresponding to 'j' and 'h', below, run over the number
            // of out_accum registers.
            let mut out_accum: [__m256; 2] = core::mem::zeroed();
            // Height needs to be moved to go to next iteration of processing
            // while processing 2 2x2 blocks vertically.
            let kSkipHeightForNextIter: i32 = kSkipHeight * 2;
            for j in 0..2 {
                out_accum[j as usize] = bias_reg;
            }
            for k in 0..(*layer_config).in_channels {
                let mut shuffle_weight: [__m256; 2] = core::mem::zeroed();
                let mut off: i32 = k * (*layer_config).out_channels + i;
                // In layer 2, the convolution process happens at 2x2.
                // The weights needed for 2x2 block are same across the in-channels,
                // which is why the load of weights happens once for each in-channel.
                prepare_weights_for_2x2_convolve(
                    (*layer_config).weights,
                    off,
                    cstep,
                    shuffle_weight.as_mut_ptr(),
                    weight_mask.as_mut_ptr(),
                );
                {
                    let mut h: i32 = 0;
                    let mut u: i32 = 0;
                    while h < kInHeight - kFilterHeight + 1 {
                        let mut input_ptr: *const f32 =
                            (*input.offset(k as isize)).offset((h * in_stride) as isize);
                        perform_convolve_for_4hx2v_2x2_blocks(
                            input_ptr,
                            in_stride,
                            shuffle_weight.as_mut_ptr(),
                            out_accum.as_mut_ptr().offset(u as isize),
                            shuffle_output_mask,
                        );
                        h += kSkipHeightForNextIter;
                        u += 1;
                    }
                }
            }
            // Store output of layer 2.
            for j in 0..2 {
                _mm256_storeu_ps(
                    (*output.offset(i as isize)).offset((j * out_stride * 2) as isize),
                    out_accum[j as usize],
                );
            }
            i += channel_step;
        }
    }
}

// AVX2 variant of av1_cnn_convolve_no_maxpool_padding_valid_c(), when
// filter_width and filter_height are equal to 2.
// As per the layer config set by av1_intra_mode_cnn_partition_cnn_config,
// the filter_width and filter_height are equal to 2 for layer >= 1. So
// convolution happens at 2x2 for layer >= 1.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn cnn_convolve_no_maxpool_padding_valid_2x2_avx2(
    input: *mut *const f32,
    in_width: i32,
    in_height: i32,
    in_stride: i32,
    layer_config: *const CNN_LAYER_CONFIG,
    output: *mut *mut f32,
    out_stride: i32,
    start_idx: i32,
    cstep: i32,
    channel_step: i32,
) {
    debug_assert!((*layer_config).filter_width == 2 && (*layer_config).filter_height == 2);
    debug_assert!((*layer_config).skip_width == 2 && (*layer_config).skip_height == 2);
    if in_width == 16 && in_height == 16 {
        // This case of in_width and in_height equal to 16 corresponds to layer 1.
        // The output size of this layer is 8x8.
        cnn_convolve_no_maxpool_padding_valid_layer1_avx2(
            input,
            in_stride,
            layer_config,
            output,
            out_stride,
            start_idx,
            cstep,
            channel_step,
        );
    } else if in_width == 8 && in_height == 8 {
        // This case of in_width and in_height equal to 8 corresponds to layer 2.
        // The output size of this layer is 4x4.
        cnn_convolve_no_maxpool_padding_valid_layer2_avx2(
            input,
            in_stride,
            layer_config,
            output,
            out_stride,
            start_idx,
            cstep,
            channel_step,
        );
    } else {
        // For layer equal to 3 and 4, the input is of size 4x4 and 2x2
        // respectively. Implementing SIMD for these cases might not be optimal,
        // which is why we call C path for layer >= 3.
        av1_cnn_convolve_no_maxpool_padding_valid_c(
            input,
            in_width,
            in_height,
            in_stride,
            layer_config,
            output,
            out_stride,
            start_idx,
            cstep,
            channel_step,
        );
    }
}

// AVX2 variant of av1_cnn_convolve_no_maxpool_padding_valid_c().
// As per the current encoder, av1_cnn_convolve function gets called for
// block size equal to 64x64. av1_cnn_convolve() uses layer config values
// set by av1_intra_mode_cnn_partition_cnn_config. The following are a few
// details related to each layer's config parameters.
// Layer_Number in_size out_size filter_wd filter_ht skip_wd skip_ht
//     0         64x64    16x16      5         5         4       4
//     1         16x16    8x8        2         2         2       2
//     2         8x8      4x4        2         2         2       2
//     3         4x4      2x2        2         2         2       2
//     4         2x2      1x1        2         2         2       2
// Here,
// filter_wd = filter_width and filter_ht = filter_height,
// skip_wd = skip_width and skip_ht = skip_height.
#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_cnn_convolve_no_maxpool_padding_valid_avx2(
    input: *mut *const f32,
    in_width: i32,
    in_height: i32,
    in_stride: i32,
    layer_config: *const CNN_LAYER_CONFIG,
    output: *mut *mut f32,
    out_stride: i32,
    start_idx: i32,
    cstep: i32,
    channel_step: i32,
) {
    if (*layer_config).filter_width == 5
        && (*layer_config).filter_height == 5
        && (*layer_config).skip_width == 4
        && (*layer_config).skip_height == 4
    {
        cnn_convolve_no_maxpool_padding_valid_5x5_avx2(
            input,
            in_width,
            in_height,
            in_stride,
            layer_config,
            output,
            out_stride,
            start_idx,
            cstep,
            channel_step,
        );
    } else if (*layer_config).filter_width == 2
        && (*layer_config).filter_height == 2
        && (*layer_config).skip_width == 2
        && (*layer_config).skip_height == 2
    {
        cnn_convolve_no_maxpool_padding_valid_2x2_avx2(
            input,
            in_width,
            in_height,
            in_stride,
            layer_config,
            output,
            out_stride,
            start_idx,
            cstep,
            channel_step,
        );
    } else {
        av1_cnn_convolve_no_maxpool_padding_valid_c(
            input,
            in_width,
            in_height,
            in_stride,
            layer_config,
            output,
            out_stride,
            start_idx,
            cstep,
            channel_step,
        );
    }
}
