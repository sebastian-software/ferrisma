/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/encoder/x86/ml_sse3.c (ADR 0056, ADR 0058).
// Upstream compiles it with -msse3. The functions were
// transliterated with a typing-aware C-subset printer (not C2Rust) and
// reviewed against the C function by function: the same sequence of intrinsic
// and helper calls, constants and operators in order (ADR 0058).
// Each `_mm*_mul_ps` and `_mm*_add_ps` and each scalar `*` and `+` is a
// separate rounding, as in the original built with -ffp-contract=off
// (ADR 0024); `float` literals stay single precision.
// approx_exp's object-like macros A, B and C are written out in place.
//
// Translation notes:
// - Every function carries `sse3`, the flag upstream compiles the file with.
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
use core::arch::x86_64::*;

// av1/encoder/ml.h
const NN_MAX_NODES_PER_LAYER: i32 = 128;

const C: i32 = 60801;

// In order to avoid the high-latency of swapping between FPU and SIMD
// operations, we keep the result in a 128-bit register even though we only
// care about a single value.
#[inline]
#[target_feature(enable = "sse3")]
unsafe fn nn_propagate_8to1(inputs: *const f32, weights: *const f32, output: *mut __m128) {
    let inputs_h: __m128 = _mm_loadu_ps(inputs.offset(4));
    let inputs_l: __m128 = _mm_loadu_ps(inputs);
    let weights_h: __m128 = _mm_loadu_ps(weights.offset(4));
    let weights_l: __m128 = _mm_loadu_ps(weights);
    let mul_h: __m128 = _mm_mul_ps(inputs_h, weights_h);
    let mul_l: __m128 = _mm_mul_ps(inputs_l, weights_l);
    // [7 6 5 4] [3 2 1 0] (weight and input indices)
    let vadd: __m128 = _mm_add_ps(mul_l, mul_h);
    // [7+3 6+2 5+1 4+0]
    let hadd1: __m128 = _mm_hadd_ps(vadd, vadd);
    // [7+6+3+2 5+4+1+0 7+6+3+2 5+4+1+0]
    let hadd2: __m128 = _mm_hadd_ps(hadd1, hadd1);
    // [7+6+5+4+3+2+1+0 7+6+5+4+3+2+1+0 7+6+5+4+3+2+1+0 7+6+5+4+3+2+1+0]
    *output = _mm_add_ps(*output, hadd2);
}

#[target_feature(enable = "sse3")]
pub unsafe fn av1_nn_propagate_4to1_sse3(
    inputs: *const f32,
    weights: *const f32,
    output: *mut __m128,
) {
    let inputs128: __m128 = _mm_loadu_ps(inputs);
    let weights128: __m128 = _mm_loadu_ps(weights);
    let mul: __m128 = _mm_mul_ps(inputs128, weights128);
    // [3 2 1 0] (weight and input indices)
    let hadd1: __m128 = _mm_hadd_ps(mul, mul);
    // [3+2 1+0 3+2 1+0]
    let hadd2: __m128 = _mm_hadd_ps(hadd1, hadd1);
    // [3+2+1+0 3+2+1+0 3+2+1+0 3+2+1+0]
    *output = _mm_add_ps(*output, hadd2);
}

#[target_feature(enable = "sse3")]
pub unsafe fn av1_nn_propagate_4to4_sse3(
    inputs: *const f32,
    weights: *const f32,
    outputs: *mut __m128,
    num_inputs: i32,
) {
    let inputs128: __m128 = _mm_loadu_ps(inputs);
    let mut hadd: [__m128; 2] = core::mem::zeroed();
    for i in 0..2 {
        // For each pair of outputs
        let weight0: __m128 = _mm_loadu_ps(weights.offset((2 * i * num_inputs) as isize));
        let mul0: __m128 = _mm_mul_ps(weight0, inputs128);
        let weight1: __m128 = _mm_loadu_ps(weights.offset(((2 * i + 1) * num_inputs) as isize));
        let mul1: __m128 = _mm_mul_ps(weight1, inputs128);
        hadd[i as usize] = _mm_hadd_ps(mul0, mul1);
    }
    // hadd[0] = [7+6 5+4 3+2 1+0] (weight indices)
    // hadd[1] = [15+14 13+12 11+10 9+8]
    let hh: __m128 = _mm_hadd_ps(hadd[0], hadd[1]);
    // [15+14+13+12 11+10+9+8 7+6+5+4 3+2+1+0]
    *outputs = _mm_add_ps(*outputs, hh);
}

#[target_feature(enable = "sse3")]
pub unsafe fn av1_nn_propagate_4to8_sse3(
    inputs: *const f32,
    weights: *const f32,
    out_h: *mut __m128,
    out_l: *mut __m128,
    num_inputs: i32,
) {
    let inputs128: __m128 = _mm_loadu_ps(inputs);
    let mut hadd: [__m128; 4] = core::mem::zeroed();
    for i in 0..4 {
        // For each pair of outputs
        let weight0: __m128 = _mm_loadu_ps(weights.offset((2 * i * num_inputs) as isize));
        let weight1: __m128 = _mm_loadu_ps(weights.offset(((2 * i + 1) * num_inputs) as isize));
        let mul0: __m128 = _mm_mul_ps(inputs128, weight0);
        let mul1: __m128 = _mm_mul_ps(inputs128, weight1);
        hadd[i as usize] = _mm_hadd_ps(mul0, mul1);
    }
    // hadd[0] = [7+6 5+4 3+2 1+0] (weight indices)
    // hadd[1] = [15+14 13+12 11+10 9+8]
    // hadd[2] = [23+22 21+20 19+18 17+16]
    // hadd[3] = [31+30 29+28 27+26 25+24]
    let hh0: __m128 = _mm_hadd_ps(hadd[0], hadd[1]);
    // [15+14+13+12 11+10+9+8 7+6+5+4 3+2+1+0]
    let hh1: __m128 = _mm_hadd_ps(hadd[2], hadd[3]);
    // [31+30+29+28 27+26+25+24 23+22+21+20 19+18+17+16]
    *out_h = _mm_add_ps(*out_h, hh1);
    *out_l = _mm_add_ps(*out_l, hh0);
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn nn_propagate_8to4(
    inputs: *const f32,
    weights: *const f32,
    outputs: *mut __m128,
    num_inputs: i32,
) {
    let inputs_h: __m128 = _mm_loadu_ps(inputs.offset(4));
    let inputs_l: __m128 = _mm_loadu_ps(inputs);
    // [7 6 5 4] [3 2 1 0] (input indices)
    let mut add: [__m128; 4] = core::mem::zeroed();
    for i in 0..4 {
        // For each output:
        let weight_h: __m128 = _mm_loadu_ps(weights.offset((i * num_inputs + 4) as isize));
        let weight_l: __m128 = _mm_loadu_ps(weights.offset((i * num_inputs) as isize));
        let mul_h: __m128 = _mm_mul_ps(inputs_h, weight_h);
        let mul_l: __m128 = _mm_mul_ps(inputs_l, weight_l);
        add[i as usize] = _mm_add_ps(mul_l, mul_h);
    }
    // add[0] = [7+3 6+2 5+1 4+0]
    // add[1] = [15+11 14+10 13+9 12+8]
    // add[2] = [23+19 22+18 21+17 20+16]
    // add[3] = [31+27 30+26 29+25 28+24]
    let hadd_h: __m128 = _mm_hadd_ps(add[2], add[3]);
    // [31+30+27+26 29+28+25+24 23+22+19+18 21+20+17+16]
    let hadd_l: __m128 = _mm_hadd_ps(add[0], add[1]);
    // [15+14+11+10 13+12+9+8 7+6+3+2 5+4+1+0]
    let haddhadd: __m128 = _mm_hadd_ps(hadd_l, hadd_h);
    // [31+30+29+28+27+26+25+24 23+22+21+20+19+18+17+16
    //  15+14+13+12+11+10+9+8 7+6+5+4+3+2+1+0]
    *outputs = _mm_add_ps(*outputs, haddhadd);
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn nn_activate8(out_h: *mut __m128, out_l: *mut __m128) {
    let zero: __m128 = _mm_setzero_ps();
    *out_h = _mm_max_ps(*out_h, zero);
    *out_l = _mm_max_ps(*out_l, zero);
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn nn_activate4(x: *mut __m128) {
    *x = _mm_max_ps(*x, _mm_setzero_ps());
}

// Calculate prediction based on the given input features and neural net config.
// Assume there are no more than NN_MAX_NODES_PER_LAYER nodes in each hidden
// layer.
#[no_mangle]
#[target_feature(enable = "sse3")]
pub unsafe extern "C-unwind" fn av1_nn_predict_sse3(
    mut input_nodes: *const f32,
    nn_config: *const NN_CONFIG,
    reduce_prec: i32,
    output: *mut f32,
) {
    let mut buf: [[f32; (NN_MAX_NODES_PER_LAYER) as usize]; 2] = core::mem::zeroed();
    let mut buf_index: i32 = 0;
    let mut num_inputs: i32 = (*nn_config).num_inputs;
    // Hidden layers, except the final iteration is the output layer.
    for layer in 0..=(*nn_config).num_hidden_layers {
        let mut layer_weights: *const f32 = (*nn_config).weights[layer as usize];
        let mut layer_bias: *const f32 = (*nn_config).bias[layer as usize];
        let mut output_layer: bool = layer == (*nn_config).num_hidden_layers;
        let mut output_nodes: *mut f32 = if output_layer {
            output
        } else {
            buf[buf_index as usize].as_mut_ptr()
        };
        let num_outputs: i32 = if output_layer {
            (*nn_config).num_outputs
        } else {
            (*nn_config).num_hidden_nodes[layer as usize]
        };
        if num_inputs % 4 == 0 && num_outputs % 8 == 0 {
            for out in (0..num_outputs).step_by(8) {
                let mut out_h: __m128 = _mm_loadu_ps(layer_bias.offset((out + 4) as isize));
                let mut out_l: __m128 = _mm_loadu_ps(layer_bias.offset(out as isize));
                for in_ in (0..num_inputs).step_by(4) {
                    av1_nn_propagate_4to8_sse3(
                        input_nodes.offset(in_ as isize),
                        layer_weights.offset((out * num_inputs + in_) as isize),
                        &raw mut out_h,
                        &raw mut out_l,
                        num_inputs,
                    );
                }
                if !output_layer {
                    nn_activate8(&raw mut out_h, &raw mut out_l);
                }
                _mm_storeu_ps(output_nodes.offset((out + 4) as isize), out_h);
                _mm_storeu_ps(output_nodes.offset(out as isize), out_l);
            }
        } else if num_inputs % 8 == 0 && num_outputs % 4 == 0 {
            for out in (0..num_outputs).step_by(4) {
                let mut outputs: __m128 = _mm_loadu_ps(layer_bias.offset(out as isize));
                for in_ in (0..num_inputs).step_by(8) {
                    nn_propagate_8to4(
                        input_nodes.offset(in_ as isize),
                        layer_weights.offset((out * num_inputs + in_) as isize),
                        &raw mut outputs,
                        num_inputs,
                    );
                }
                if !output_layer {
                    nn_activate4(&raw mut outputs);
                }
                _mm_storeu_ps(output_nodes.offset(out as isize), outputs);
            }
        } else if num_inputs % 4 == 0 && num_outputs % 4 == 0 {
            for out in (0..num_outputs).step_by(4) {
                let mut outputs: __m128 = _mm_loadu_ps(layer_bias.offset(out as isize));
                for in_ in (0..num_inputs).step_by(4) {
                    av1_nn_propagate_4to4_sse3(
                        input_nodes.offset(in_ as isize),
                        layer_weights.offset((out * num_inputs + in_) as isize),
                        &raw mut outputs,
                        num_inputs,
                    );
                }
                if !output_layer {
                    nn_activate4(&raw mut outputs);
                }
                _mm_storeu_ps(output_nodes.offset(out as isize), outputs);
            }
        } else if num_inputs % 8 == 0 {
            for out in 0..num_outputs {
                let mut total: __m128 = _mm_load1_ps(layer_bias.offset(out as isize));
                for in_ in (0..num_inputs).step_by(8) {
                    nn_propagate_8to1(
                        input_nodes.offset(in_ as isize),
                        layer_weights.offset((out * num_inputs + in_) as isize),
                        &raw mut total,
                    );
                }
                if !output_layer {
                    nn_activate4(&raw mut total);
                }
                *output_nodes.offset(out as isize) = _mm_cvtss_f32(total);
            }
        } else if num_inputs % 4 == 0 {
            for out in 0..num_outputs {
                let mut total: __m128 = _mm_load1_ps(layer_bias.offset(out as isize));
                for in_ in (0..num_inputs).step_by(4) {
                    av1_nn_propagate_4to1_sse3(
                        input_nodes.offset(in_ as isize),
                        layer_weights.offset((out * num_inputs + in_) as isize),
                        &raw mut total,
                    );
                }
                if !output_layer {
                    nn_activate4(&raw mut total);
                }
                *output_nodes.offset(out as isize) = _mm_cvtss_f32(total);
            }
        } else {
            // Use SSE instructions for scalar operations to avoid the latency of
            // swapping between SIMD and FPU modes.
            for out in 0..num_outputs {
                let mut total: __m128 = _mm_load1_ps(layer_bias.offset(out as isize));
                for in_node in 0..num_inputs {
                    let mut input: __m128 = _mm_load1_ps(input_nodes.offset(in_node as isize));
                    let mut weight: __m128 =
                        _mm_load1_ps(layer_weights.offset((num_inputs * out + in_node) as isize));
                    total = _mm_add_ps(total, _mm_mul_ps(input, weight));
                }
                if !output_layer {
                    nn_activate4(&raw mut total);
                }
                *output_nodes.offset(out as isize) = _mm_cvtss_f32(total);
            }
        }
        input_nodes = output_nodes as *const f32;
        num_inputs = num_outputs;
        buf_index = 1 - buf_index;
    }
    if reduce_prec != 0 {
        av1_nn_output_prec_reduce(output, (*nn_config).num_outputs);
    }
}

// Based on N. N. Schraudolph. A Fast, Compact Approximation of the Exponential
// Function. Neural Computation, 11(4):853–862, 1999.
#[inline]
#[target_feature(enable = "sse3")]
unsafe fn approx_exp(mut y: __m128) -> __m128 {
    // (1 << 23) / ln(2)
    // Offset for the exponent according to IEEE floating point standard.
    // Magic number controls the accuracy of approximation
    let multiplier: __m128 = _mm_set1_ps((1 << 23) as f32 / 0.69314718056f32);
    let offset: __m128i = _mm_set1_epi32(127 * (1 << 23) - 60801);
    y = _mm_mul_ps(y, multiplier);
    y = _mm_castsi128_ps(_mm_add_epi32(_mm_cvtps_epi32(y), offset));
    return y;
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn reduce_max(mut reg: __m128) -> __m128 {
    let mut tmp_reg: __m128;
    tmp_reg = _mm_shuffle_ps::<0x4e>(reg, reg); // 01 00 11 10
    reg = _mm_max_ps(reg, tmp_reg);
    tmp_reg = _mm_shuffle_ps::<0xb1>(reg, reg); // 10 11 00 01
    reg = _mm_max_ps(reg, tmp_reg);
    return reg;
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn reduce_sum(mut reg: __m128) -> __m128 {
    let mut tmp_reg: __m128;
    tmp_reg = _mm_shuffle_ps::<0x4e>(reg, reg); // 01 00 11 10
    reg = _mm_add_ps(reg, tmp_reg);
    tmp_reg = _mm_shuffle_ps::<0xb1>(reg, reg); // 10 11 00 01
    reg = _mm_add_ps(reg, tmp_reg);
    return reg;
}

#[no_mangle]
#[target_feature(enable = "sse3")]
pub unsafe extern "C-unwind" fn av1_nn_fast_softmax_16_sse3(input: *const f32, output: *mut f32) {
    // Clips at -10 to avoid underflowing
    let clipper: __m128 = _mm_set1_ps(-10.0f32);
    // Load in 16 values
    let mut in_0: __m128 = _mm_loadu_ps(input);
    let mut in_1: __m128 = _mm_loadu_ps(input.offset(4));
    let mut in_2: __m128 = _mm_loadu_ps(input.offset(8));
    let mut in_3: __m128 = _mm_loadu_ps(input.offset(12));
    // Get the max
    let mut max_0: __m128 = _mm_max_ps(in_0, in_1);
    let mut max_1: __m128 = _mm_max_ps(in_2, in_3);
    max_0 = _mm_max_ps(max_0, max_1);
    max_0 = reduce_max(max_0);
    // Subtract the max off and clip
    in_0 = _mm_sub_ps(in_0, max_0);
    in_1 = _mm_sub_ps(in_1, max_0);
    in_2 = _mm_sub_ps(in_2, max_0);
    in_3 = _mm_sub_ps(in_3, max_0);
    in_0 = _mm_max_ps(in_0, clipper);
    in_1 = _mm_max_ps(in_1, clipper);
    in_2 = _mm_max_ps(in_2, clipper);
    in_3 = _mm_max_ps(in_3, clipper);
    // Exponentiate and compute the denominator
    in_0 = approx_exp(in_0);
    let mut sum: __m128 = in_0;
    in_1 = approx_exp(in_1);
    sum = _mm_add_ps(sum, in_1);
    in_2 = approx_exp(in_2);
    sum = _mm_add_ps(sum, in_2);
    in_3 = approx_exp(in_3);
    sum = _mm_add_ps(sum, in_3);
    sum = reduce_sum(sum);
    // Divide to get the probability
    in_0 = _mm_div_ps(in_0, sum);
    in_1 = _mm_div_ps(in_1, sum);
    in_2 = _mm_div_ps(in_2, sum);
    in_3 = _mm_div_ps(in_3, sum);
    _mm_storeu_ps(output, in_0);
    _mm_storeu_ps(output.offset(4), in_1);
    _mm_storeu_ps(output.offset(8), in_2);
    _mm_storeu_ps(output.offset(12), in_3);
}
