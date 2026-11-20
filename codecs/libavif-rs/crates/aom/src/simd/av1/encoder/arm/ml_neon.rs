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

// Translated by hand from upstream/aom/av1/encoder/arm/ml_neon.c (ADR 0056).
// Function names, helper structure and intrinsic sequences follow the
// original; only the AOM_ARCH_AARCH64 branches are retained. `vmlaq_f32` is a
// separate multiply and add in both languages (the original is built with
// -ffp-contract=off, ADR 0024; Rust never contracts), and each scalar `*` and
// `+` rounds on its own as in C. CLAMP_0 assigns its operand, as the C macro
// does. The hidden-layer buffers start zeroed (uninitialized in C; every
// element read is written first).
#![allow(unused_mut, clippy::all)]

use core::arch::aarch64::*;

use crate::av1__encoder__ml::{av1_nn_output_prec_reduce, NN_CONFIG};

// av1/encoder/ml.h
const NN_MAX_NODES_PER_LAYER: usize = 128;

#[inline(always)]
unsafe fn nn_activate8(out_h: *mut float32x4_t, out_l: *mut float32x4_t, zero: *const float32x4_t) {
    *out_h = vmaxq_f32(*out_h, *zero);
    *out_l = vmaxq_f32(*out_l, *zero);
}

#[inline(always)]
unsafe fn nn_activate4(x: *mut float32x4_t, zero: *const float32x4_t) {
    *x = vmaxq_f32(*x, *zero);
}

// #define CLAMP_0(x) (x = x > 0 ? x : 0)
macro_rules! clamp_0 {
    ($x:expr) => {{
        $x = if $x > 0.0 { $x } else { 0.0 };
        $x
    }};
}

#[inline(always)]
unsafe fn nn_propagate_8to1(num_inputs: i32, inputs: *const f32, weights: *const f32,
                            layer_bias: *const f32, output_nodes: *mut f32, output_layer: bool) {
    let zero: float32x4_t = vdupq_n_f32(0.0);
    let mut vadd: float32x4_t = zero;
    let mut total: f32 = *layer_bias;

    let mut in_ = 0;
    while in_ < num_inputs {
        let inputs_h: float32x4_t = vld1q_f32(inputs.offset((in_ + 4) as isize));
        let inputs_l: float32x4_t = vld1q_f32(inputs.offset(in_ as isize));

        let weights_h: float32x4_t = vld1q_f32(weights.offset((in_ + 4) as isize));
        let weights_l: float32x4_t = vld1q_f32(weights.offset(in_ as isize));

        vadd = vmlaq_f32(vadd, inputs_h, weights_h);
        vadd = vmlaq_f32(vadd, inputs_l, weights_l);
        in_ += 8;
    }
    total += vaddvq_f32(vadd);

    if !output_layer {
        clamp_0!(total);
    }
    *output_nodes = total;
}

#[inline(always)]
unsafe fn nn_propagate_xto1(num_inputs: i32, inputs: *const f32, weights: *const f32,
                            layer_bias: *const f32, output_nodes: *mut f32) {
    let mut vadd: float32x4_t = vdupq_n_f32(0.0);

    let mut total: f32 = *layer_bias;
    let mut j = num_inputs;
    let mut in_ = 0;
    while j > 7 {
        let inputs_h: float32x4_t = vld1q_f32(inputs.offset((in_ + 4) as isize));
        let inputs_l: float32x4_t = vld1q_f32(inputs.offset(in_ as isize));

        let weights_h: float32x4_t = vld1q_f32(weights.offset((in_ + 4) as isize));
        let weights_l: float32x4_t = vld1q_f32(weights.offset(in_ as isize));

        vadd = vmlaq_f32(vadd, inputs_h, weights_h);
        vadd = vmlaq_f32(vadd, inputs_l, weights_l);
        in_ += 8;
        j -= 8;
    }

    total += vaddvq_f32(vadd);
    while in_ < num_inputs {
        total += *weights.offset(in_ as isize) * *inputs.offset(in_ as isize);
        in_ += 1;
    }

    *output_nodes = clamp_0!(total);
}

#[inline(always)]
unsafe fn nn_propagate_xsto1(num_inputs: i32, inputs: *const f32, weights: *const f32,
                             layer_bias: *const f32, output_nodes: *mut f32) {
    let mut total: f32 = *layer_bias;
    let v_inputs: float32x4_t = vld1q_f32(inputs);
    let v_weights: float32x4_t = vld1q_f32(weights);
    let vadd: float32x4_t = vmulq_f32(v_inputs, v_weights);
    total += vaddvq_f32(vadd);
    let mut in_ = 4;
    while in_ < num_inputs {
        total += *weights.offset(in_ as isize) * *inputs.offset(in_ as isize);
        in_ += 1;
    }

    *output_nodes = clamp_0!(total);
}

#[inline(always)]
unsafe fn nn_propagate_4to1(num_inputs: i32, inputs: *const f32, weights: *const f32,
                            layer_bias: *const f32, output_nodes: *mut f32, output_layer: bool) {
    let zero: float32x4_t = vdupq_n_f32(0.0);
    let mut vadd: float32x4_t = zero;
    let mut total: f32 = *layer_bias;

    let mut in_ = 0;
    while in_ < num_inputs {
        let v_inputs: float32x4_t = vld1q_f32(inputs.offset(in_ as isize));
        let v_weights: float32x4_t = vld1q_f32(weights.offset(in_ as isize));
        vadd = vmlaq_f32(vadd, v_inputs, v_weights);
        in_ += 4;
    }

    total += vaddvq_f32(vadd);

    if !output_layer {
        clamp_0!(total);
    }
    *output_nodes = total;
}

#[inline(always)]
unsafe fn nn_propagate_4to4(num_inputs: i32, inputs: *const f32, weights: *const f32,
                            layer_bias: *const f32, output_nodes: *mut f32, output_layer: bool) {
    let mut outputs: float32x4_t = vld1q_f32(layer_bias);
    let zero: float32x4_t = vdupq_n_f32(0.0);

    let mut mul0: [float32x4_t; 2] = [zero, zero];
    let mut mul1: [float32x4_t; 2] = [zero, zero];
    let mut in_ = 0;
    while in_ < num_inputs {
        let v_input: float32x4_t = vld1q_f32(inputs.offset(in_ as isize));

        for i in 0..2 {
            let weight0: float32x4_t =
                vld1q_f32(weights.offset((in_ + 2 * i * num_inputs) as isize));
            mul0[i as usize] = vmlaq_f32(mul0[i as usize], weight0, v_input);
            let weight1: float32x4_t =
                vld1q_f32(weights.offset((in_ + (2 * i + 1) * num_inputs) as isize));
            mul1[i as usize] = vmlaq_f32(mul1[i as usize], weight1, v_input);
        }
        in_ += 4;
    }
    for i in 0..2 {
        mul0[i] = vpaddq_f32(mul0[i], mul1[i]);
    }
    let hh: float32x4_t = vpaddq_f32(mul0[0], mul0[1]);

    outputs = vaddq_f32(outputs, hh);
    if !output_layer {
        nn_activate4(&mut outputs, &zero);
    }
    vst1q_f32(output_nodes, outputs);
}

#[inline(always)]
unsafe fn nn_propagate_4to8(num_inputs: i32, inputs: *const f32, weights: *const f32,
                            layer_bias: *const f32, output_nodes: *mut f32, output_layer: bool) {
    let mut out_h: float32x4_t = vld1q_f32(layer_bias.offset(4));
    let mut out_l: float32x4_t = vld1q_f32(layer_bias);
    let zero: float32x4_t = vdupq_n_f32(0.0);
    let mut mul0: [float32x4_t; 4] = [zero, zero, zero, zero];
    let mut mul1: [float32x4_t; 4] = [zero, zero, zero, zero];

    let mut in_ = 0;
    while in_ < num_inputs {
        let v_input: float32x4_t = vld1q_f32(inputs.offset(in_ as isize));
        for i in 0..4 {
            let weight0: float32x4_t =
                vld1q_f32(weights.offset((in_ + 2 * i * num_inputs) as isize));
            let weight1: float32x4_t =
                vld1q_f32(weights.offset((in_ + (2 * i + 1) * num_inputs) as isize));
            mul0[i as usize] = vmlaq_f32(mul0[i as usize], v_input, weight0);
            mul1[i as usize] = vmlaq_f32(mul1[i as usize], v_input, weight1);
        }
        in_ += 4;
    }
    for i in 0..4 {
        mul0[i] = vpaddq_f32(mul0[i], mul1[i]);
    }
    let hh0: float32x4_t = vpaddq_f32(mul0[0], mul0[1]);
    let hh1: float32x4_t = vpaddq_f32(mul0[2], mul0[3]);

    out_h = vaddq_f32(out_h, hh1);
    out_l = vaddq_f32(out_l, hh0);

    if !output_layer {
        nn_activate8(&mut out_h, &mut out_l, &zero);
    }
    vst1q_f32(output_nodes.offset(4), out_h);
    vst1q_f32(output_nodes, out_l);
}

#[inline(always)]
unsafe fn nn_propagate_8to4(num_inputs: i32, inputs: *const f32, weights: *const f32,
                            layer_bias: *const f32, output_nodes: *mut f32, output_layer: bool) {
    let mut outputs: float32x4_t = vld1q_f32(layer_bias);
    let zero: float32x4_t = vdupq_n_f32(0.0);
    let mut add: [float32x4_t; 4] = [zero, zero, zero, zero];
    let mut in_ = 0;
    while in_ < num_inputs {
        let inputs_l: float32x4_t = vld1q_f32(inputs.offset(in_ as isize));
        let inputs_h: float32x4_t = vld1q_f32(inputs.offset((in_ + 4) as isize));

        for i in 0..4 {
            let weight_l: float32x4_t = vld1q_f32(weights.offset((in_ + i * num_inputs) as isize));
            let weight_h: float32x4_t =
                vld1q_f32(weights.offset((in_ + i * num_inputs + 4) as isize));
            add[i as usize] = vmlaq_f32(add[i as usize], inputs_l, weight_l);
            add[i as usize] = vmlaq_f32(add[i as usize], inputs_h, weight_h);
        }
        in_ += 8;
    }
    let hadd_h: float32x4_t = vpaddq_f32(add[2], add[3]);
    let hadd_l: float32x4_t = vpaddq_f32(add[0], add[1]);
    let haddhadd: float32x4_t = vpaddq_f32(hadd_l, hadd_h);

    outputs = vaddq_f32(outputs, haddhadd);
    if !output_layer {
        nn_activate4(&mut outputs, &zero);
    }
    vst1q_f32(output_nodes, outputs);
}

// Calculate prediction based on the given input features and neural net config.
// Assume there are no more than NN_MAX_NODES_PER_LAYER nodes in each hidden
// layer.
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nn_predict_neon(mut input_nodes: *const f32,
                                                    nn_config: *const NN_CONFIG, reduce_prec: i32,
                                                    output: *mut f32) {
    let mut buf: [[f32; NN_MAX_NODES_PER_LAYER]; 2] = [[0.0; NN_MAX_NODES_PER_LAYER]; 2];
    let mut buf_index: usize = 0;
    let mut num_inputs: i32 = (*nn_config).num_inputs;
    // Hidden layers, except the final iteration is the output layer.
    for layer in 0..=(*nn_config).num_hidden_layers {
        let layer_weights: *const f32 = (*nn_config).weights[layer as usize];
        let layer_bias: *const f32 = (*nn_config).bias[layer as usize];
        let output_layer: bool = layer == (*nn_config).num_hidden_layers;
        let output_nodes: *mut f32 =
            if output_layer { output } else { buf[buf_index].as_mut_ptr() };
        let num_outputs: i32 = if output_layer {
            (*nn_config).num_outputs
        } else {
            (*nn_config).num_hidden_nodes[layer as usize]
        };

        if num_inputs % 4 == 0 && num_outputs % 8 == 0 {
            let mut out = 0;
            while out < num_outputs {
                nn_propagate_4to8(num_inputs, input_nodes,
                                  layer_weights.offset((out * num_inputs) as isize),
                                  layer_bias.offset(out as isize), output_nodes.offset(out as isize),
                                  output_layer);
                out += 8;
            }
        } else if num_inputs % 8 == 0 && num_outputs % 4 == 0 {
            let mut out = 0;
            while out < num_outputs {
                nn_propagate_8to4(num_inputs, input_nodes,
                                  layer_weights.offset((out * num_inputs) as isize),
                                  layer_bias.offset(out as isize), output_nodes.offset(out as isize),
                                  output_layer);
                out += 4;
            }
        } else if num_inputs % 4 == 0 && num_outputs % 4 == 0 {
            let mut out = 0;
            while out < num_outputs {
                nn_propagate_4to4(num_inputs, input_nodes,
                                  layer_weights.offset((out * num_inputs) as isize),
                                  layer_bias.offset(out as isize), output_nodes.offset(out as isize),
                                  output_layer);
                out += 4;
            }
        } else if num_inputs % 8 == 0 {
            for out in 0..num_outputs {
                nn_propagate_8to1(num_inputs, input_nodes,
                                  layer_weights.offset((out * num_inputs) as isize),
                                  layer_bias.offset(out as isize), output_nodes.offset(out as isize),
                                  output_layer);
            }
        } else if num_inputs % 4 == 0 {
            for out in 0..num_outputs {
                nn_propagate_4to1(num_inputs, input_nodes,
                                  layer_weights.offset((out * num_inputs) as isize),
                                  layer_bias.offset(out as isize), output_nodes.offset(out as isize),
                                  output_layer);
            }
        } else if num_inputs > 8 {
            for out in 0..num_outputs {
                nn_propagate_xto1(num_inputs, input_nodes,
                                  layer_weights.offset((out * num_inputs) as isize),
                                  layer_bias.offset(out as isize), output_nodes.offset(out as isize));
            }
        } else if num_inputs >= 4 {
            for out in 0..num_outputs {
                nn_propagate_xsto1(num_inputs, input_nodes,
                                   layer_weights.offset((out * num_inputs) as isize),
                                   layer_bias.offset(out as isize),
                                   output_nodes.offset(out as isize));
            }
        } else {
            for node in 0..num_outputs {
                let mut val: f32 = *layer_bias.offset(node as isize);
                for i in 0..num_inputs {
                    val += *layer_weights.offset((node * num_inputs + i) as isize)
                        * *input_nodes.offset(i as isize);
                }
                // ReLU as activation function.
                val = if val > 0.0f32 { val } else { 0.0f32 }; // Could use AOMMAX().
                *output_nodes.offset(node as isize) = val;
            }
        }
        input_nodes = output_nodes;
        num_inputs = num_outputs;
        buf_index = 1 - buf_index;
    }
    if reduce_prec != 0 {
        av1_nn_output_prec_reduce(output, (*nn_config).num_outputs);
    }
}
