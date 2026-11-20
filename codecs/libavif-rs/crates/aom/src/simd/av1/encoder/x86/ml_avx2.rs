/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/ml_avx2.c (ADR 0056, ADR 0058).
// Upstream compiles it with -mavx2. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Each `_mm*_mul_ps` and `_mm*_add_ps` and each scalar `*` and `+` is a
// separate rounding, as in the original built with -ffp-contract=off
// (ADR 0024); `float` literals stay single precision.
// The statement macro CALC_OUTPUT_FOR_2ROWS was expanded with the C
// preprocessor before printing; AOMMAX(bias_val, 0) is written out.
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

use crate::av1__encoder__ml::{av1_nn_output_prec_reduce, NN_CONFIG};
use crate::simd::av1__encoder__x86__ml_sse3::{
    av1_nn_propagate_4to1_sse3, av1_nn_propagate_4to4_sse3, av1_nn_propagate_4to8_sse3,
};
use core::arch::x86_64::*;

// av1/encoder/ml.h
const NN_MAX_NODES_PER_LAYER: i32 = 128;

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn nn_propagate_8to1(
    inputs: *const f32,
    weights: *const f32,
    bias: *const f32,
    num_inputs_to_process: i32,
    tot_num_inputs: i32,
    num_outputs: i32,
    output_nodes: *mut f32,
    is_clip_required: i32,
) {
    // Process one output row at a time.
    for out in 0..num_outputs {
        let mut in_result: __m256 = _mm256_setzero_ps();
        let mut bias_val: f32 = *bias.offset(out as isize);
        for in_ in (0..num_inputs_to_process).step_by(8) {
            let inputs256: __m256 = _mm256_loadu_ps(inputs.offset(in_ as isize));
            let weight_idx: i32 = in_ + out * tot_num_inputs;
            let weight0: __m256 = _mm256_loadu_ps(weights.offset(weight_idx as isize));
            let mul0: __m256 = _mm256_mul_ps(inputs256, weight0);
            in_result = _mm256_add_ps(in_result, mul0);
        }
        let low_128: __m128 = _mm256_castps256_ps128(in_result);
        let high_128: __m128 = _mm256_extractf128_ps::<1>(in_result);
        let sum_par_0: __m128 = _mm_add_ps(low_128, high_128);
        let sum_par_1: __m128 = _mm_hadd_ps(sum_par_0, sum_par_0);
        let sum_tot: __m128 = _mm_add_ps(_mm_shuffle_ps::<0x99>(sum_par_1, sum_par_1), sum_par_1);
        bias_val += _mm_cvtss_f32(sum_tot);
        if is_clip_required != 0 {
            bias_val = if bias_val > 0 as f32 {
                bias_val
            } else {
                0 as f32
            };
        }
        *output_nodes.offset(out as isize) = bias_val;
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn nn_propagate_8to4(
    inputs: *const f32,
    weights: *const f32,
    bias: *const f32,
    num_inputs_to_process: i32,
    tot_num_inputs: i32,
    num_outputs: i32,
    output_nodes: *mut f32,
    is_clip_required: i32,
) {
    let mut hadd: [__m256; 2] = core::mem::zeroed();
    for out in (0..num_outputs).step_by(4) {
        let mut bias_reg: __m128 = _mm_loadu_ps(bias.offset(out as isize));
        let mut in_result: __m128 = _mm_setzero_ps();
        for in_ in (0..num_inputs_to_process).step_by(8) {
            let inputs256: __m256 = _mm256_loadu_ps(inputs.offset(in_ as isize));
            let weight_idx: i32 = in_ + out * tot_num_inputs;
            // Process two output row at a time.
            for i in 0..2 {
                let index: i32 = weight_idx + 2 * i * tot_num_inputs;
                let weight0: __m256 = _mm256_loadu_ps(weights.offset(index as isize));
                let weight1: __m256 =
                    _mm256_loadu_ps(weights.offset((index + tot_num_inputs) as isize));
                let mul0: __m256 = _mm256_mul_ps(inputs256, weight0);
                let mul1: __m256 = _mm256_mul_ps(inputs256, weight1);
                hadd[i as usize] = _mm256_hadd_ps(mul0, mul1);
            }
            let sum_par: __m256 = _mm256_hadd_ps(hadd[0], hadd[1]);
            let low_128: __m128 = _mm256_castps256_ps128(sum_par);
            let high_128: __m128 = _mm256_extractf128_ps::<1>(sum_par);
            let result: __m128 = _mm_add_ps(low_128, high_128);
            in_result = _mm_add_ps(in_result, result);
        }
        in_result = _mm_add_ps(in_result, bias_reg);
        if is_clip_required != 0 {
            in_result = _mm_max_ps(in_result, _mm_setzero_ps());
        }
        _mm_storeu_ps(output_nodes.offset(out as isize), in_result);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn nn_propagate_8to8(
    inputs: *const f32,
    weights: *const f32,
    bias: *const f32,
    num_inputs_to_process: i32,
    tot_num_inputs: i32,
    num_outputs: i32,
    output_nodes: *mut f32,
    is_clip_required: i32,
) {
    let mut hadd: [__m256; 4] = core::mem::zeroed();
    for out in (0..num_outputs).step_by(8) {
        let mut bias_reg: __m256 = _mm256_loadu_ps(bias.offset(out as isize));
        let mut in_result: __m256 = _mm256_setzero_ps();
        for in_ in (0..num_inputs_to_process).step_by(8) {
            let inputs256: __m256 = _mm256_loadu_ps(inputs.offset(in_ as isize));
            let weight_idx: i32 = in_ + out * tot_num_inputs;
            // Process two output rows at a time.
            for i in 0..4 {
                let index: i32 = weight_idx + 2 * i * tot_num_inputs;
                let weight0: __m256 = _mm256_loadu_ps(weights.offset(index as isize));
                let weight1: __m256 =
                    _mm256_loadu_ps(weights.offset((index + tot_num_inputs) as isize));
                let mul0: __m256 = _mm256_mul_ps(inputs256, weight0);
                let mul1: __m256 = _mm256_mul_ps(inputs256, weight1);
                hadd[i as usize] = _mm256_hadd_ps(mul0, mul1);
            }
            let hh0: __m256 = _mm256_hadd_ps(hadd[0], hadd[1]);
            let hh1: __m256 = _mm256_hadd_ps(hadd[2], hadd[3]);
            let mut ht_0: __m256 = _mm256_permute2f128_ps::<0x20>(hh0, hh1);
            let mut ht_1: __m256 = _mm256_permute2f128_ps::<0x31>(hh0, hh1);
            let mut result: __m256 = _mm256_add_ps(ht_0, ht_1);
            in_result = _mm256_add_ps(in_result, result);
        }
        in_result = _mm256_add_ps(in_result, bias_reg);
        if is_clip_required != 0 {
            in_result = _mm256_max_ps(in_result, _mm256_setzero_ps());
        }
        _mm256_storeu_ps(output_nodes.offset(out as isize), in_result);
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn nn_propagate_input_multiple_of_8(
    inputs: *const f32,
    weights: *const f32,
    bias: *const f32,
    num_inputs_to_process: i32,
    tot_num_inputs: i32,
    is_output_layer: bool,
    num_outputs: i32,
    output_nodes: *mut f32,
) {
    // The saturation of output is considered for hidden layer which is not equal
    // to final hidden layer.
    let is_clip_required: i32 =
        (!is_output_layer && num_inputs_to_process == tot_num_inputs) as i32;
    if num_outputs % 8 == 0 {
        nn_propagate_8to8(
            inputs,
            weights,
            bias,
            num_inputs_to_process,
            tot_num_inputs,
            num_outputs,
            output_nodes,
            is_clip_required,
        );
    } else if num_outputs % 4 == 0 {
        nn_propagate_8to4(
            inputs,
            weights,
            bias,
            num_inputs_to_process,
            tot_num_inputs,
            num_outputs,
            output_nodes,
            is_clip_required,
        );
    } else {
        nn_propagate_8to1(
            inputs,
            weights,
            bias,
            num_inputs_to_process,
            tot_num_inputs,
            num_outputs,
            output_nodes,
            is_clip_required,
        );
    }
}

#[no_mangle]
#[target_feature(enable = "avx2")]
pub unsafe extern "C-unwind" fn av1_nn_predict_avx2(
    mut input_nodes: *const f32,
    nn_config: *const NN_CONFIG,
    reduce_prec: i32,
    output: *mut f32,
) {
    let mut buf: [[f32; (NN_MAX_NODES_PER_LAYER) as usize]; 2] = core::mem::zeroed();
    let mut buf_index: i32 = 0;
    let mut num_inputs: i32 = (*nn_config).num_inputs;
    debug_assert!(num_inputs > 0 && num_inputs <= NN_MAX_NODES_PER_LAYER);
    for layer in 0..=(*nn_config).num_hidden_layers {
        let mut layer_weights: *const f32 = (*nn_config).weights[layer as usize];
        let mut layer_bias: *const f32 = (*nn_config).bias[layer as usize];
        let mut is_output_layer: bool = layer == (*nn_config).num_hidden_layers;
        let mut output_nodes: *mut f32 = if is_output_layer {
            output
        } else {
            buf[buf_index as usize].as_mut_ptr()
        };
        let num_outputs: i32 = if is_output_layer {
            (*nn_config).num_outputs
        } else {
            (*nn_config).num_hidden_nodes[layer as usize]
        };
        debug_assert!(num_outputs > 0 && num_outputs <= NN_MAX_NODES_PER_LAYER);
        // Process input multiple of 8 using AVX2 intrinsic.
        if num_inputs % 8 == 0 {
            nn_propagate_input_multiple_of_8(
                input_nodes,
                layer_weights,
                layer_bias,
                num_inputs,
                num_inputs,
                is_output_layer as i32 != 0,
                num_outputs,
                output_nodes,
            );
        } else {
            // When number of inputs is not multiple of 8, use hybrid approach of AVX2
            // and SSE3 based on the need.
            let in_mul_8: i32 = num_inputs / 8;
            let num_inputs_to_process: i32 = in_mul_8 * 8;
            let mut bias_is_considered: i32 = 0;
            if in_mul_8 != 0 {
                nn_propagate_input_multiple_of_8(
                    input_nodes,
                    layer_weights,
                    layer_bias,
                    num_inputs_to_process,
                    num_inputs,
                    is_output_layer as i32 != 0,
                    num_outputs,
                    output_nodes,
                );
                bias_is_considered = 1;
            }
            let mut out_temp: *const f32 = (if bias_is_considered != 0 {
                output_nodes
            } else {
                layer_bias as *mut f32
            }) as *const f32;
            let input_remaining: i32 = num_inputs % 8;
            if input_remaining % 4 == 0 && num_outputs % 8 == 0 {
                for out in (0..num_outputs).step_by(8) {
                    let mut out_h: __m128 = _mm_loadu_ps(out_temp.offset((out + 4) as isize));
                    let mut out_l: __m128 = _mm_loadu_ps(out_temp.offset(out as isize));
                    for in_ in (in_mul_8 * 8..num_inputs).step_by(4) {
                        av1_nn_propagate_4to8_sse3(
                            input_nodes.offset(in_ as isize),
                            layer_weights.offset((out * num_inputs + in_) as isize),
                            &raw mut out_h,
                            &raw mut out_l,
                            num_inputs,
                        );
                    }
                    if !is_output_layer {
                        let zero: __m128 = _mm_setzero_ps();
                        out_h = _mm_max_ps(out_h, zero);
                        out_l = _mm_max_ps(out_l, zero);
                    }
                    _mm_storeu_ps(output_nodes.offset((out + 4) as isize), out_h);
                    _mm_storeu_ps(output_nodes.offset(out as isize), out_l);
                }
            } else if input_remaining % 4 == 0 && num_outputs % 4 == 0 {
                for out in (0..num_outputs).step_by(4) {
                    let mut outputs: __m128 = _mm_loadu_ps(out_temp.offset(out as isize));
                    for in_ in (in_mul_8 * 8..num_inputs).step_by(4) {
                        av1_nn_propagate_4to4_sse3(
                            input_nodes.offset(in_ as isize),
                            layer_weights.offset((out * num_inputs + in_) as isize),
                            &raw mut outputs,
                            num_inputs,
                        );
                    }
                    if !is_output_layer {
                        outputs = _mm_max_ps(outputs, _mm_setzero_ps());
                    }
                    _mm_storeu_ps(output_nodes.offset(out as isize), outputs);
                }
            } else if input_remaining % 4 == 0 {
                for out in 0..num_outputs {
                    let mut outputs: __m128 = _mm_load1_ps(out_temp.offset(out as isize));
                    for in_ in (in_mul_8 * 8..num_inputs).step_by(4) {
                        av1_nn_propagate_4to1_sse3(
                            input_nodes.offset(in_ as isize),
                            layer_weights.offset((out * num_inputs + in_) as isize),
                            &raw mut outputs,
                        );
                    }
                    if !is_output_layer {
                        outputs = _mm_max_ps(outputs, _mm_setzero_ps());
                    }
                    *output_nodes.offset(out as isize) = _mm_cvtss_f32(outputs);
                }
            } else {
                // Use SSE instructions for scalar operations to avoid the latency
                // of swapping between SIMD and FPU modes.
                for out in 0..num_outputs {
                    let mut outputs: __m128 = _mm_load1_ps(out_temp.offset(out as isize));
                    for in_node in in_mul_8 * 8..num_inputs {
                        let mut input: __m128 = _mm_load1_ps(input_nodes.offset(in_node as isize));
                        let mut weight: __m128 = _mm_load1_ps(
                            layer_weights.offset((num_inputs * out + in_node) as isize),
                        );
                        outputs = _mm_add_ps(outputs, _mm_mul_ps(input, weight));
                    }
                    if !is_output_layer {
                        outputs = _mm_max_ps(outputs, _mm_setzero_ps());
                    }
                    *output_nodes.offset(out as isize) = _mm_cvtss_f32(outputs);
                }
            }
        }
        // Before processing the next layer, treat the output of current layer as
        // input to next layer.
        input_nodes = output_nodes as *const f32;
        num_inputs = num_outputs;
        buf_index = 1 - buf_index;
    }
    if reduce_prec != 0 {
        av1_nn_output_prec_reduce(output, (*nn_config).num_outputs);
    }
}
