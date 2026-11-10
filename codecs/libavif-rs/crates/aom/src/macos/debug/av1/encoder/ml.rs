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

// Mechanically translated from upstream/aom/av1/encoder/ml.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub as_float: core::ffi::c_float,
    pub as_int32: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NN_CONFIG {
    pub num_inputs: core::ffi::c_int,
    pub num_outputs: core::ffi::c_int,
    pub num_hidden_layers: core::ffi::c_int,
    pub num_hidden_nodes: [core::ffi::c_int; 10],
    pub weights: [*const core::ffi::c_float; 11],
    pub bias: [*const core::ffi::c_float; 11],
}
#[inline]
unsafe extern "C-unwind" fn approx_exp(mut y: core::ffi::c_float) -> core::ffi::c_float {
    let mut container: C2RustUnnamed = C2RustUnnamed { as_float: 0. };
    container.as_int32 = (y * A) as int32_t
        + (((B as int32_t) << 23 as core::ffi::c_int) - C as int32_t);
    return container.as_float;
}
pub const A: core::ffi::c_float = ((1 as core::ffi::c_int) << 23 as core::ffi::c_int)
    as core::ffi::c_float / 0.69314718056f32;
pub const B: core::ffi::c_int = 127 as core::ffi::c_int;
pub const C: core::ffi::c_int = 60801 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nn_output_prec_reduce(
    output: *mut core::ffi::c_float,
    mut num_output: core::ffi::c_int,
) {
    let prec_bits: core::ffi::c_int = 9 as core::ffi::c_int;
    let prec: core::ffi::c_int = (1 as core::ffi::c_int) << prec_bits;
    let inv_prec: core::ffi::c_float = (1.0f64 / prec as core::ffi::c_double)
        as core::ffi::c_float;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < num_output {
        *output.offset(i as isize) = ((*output.offset(i as isize)
            * prec as core::ffi::c_float) as core::ffi::c_double + 0.5f64)
            as core::ffi::c_int as core::ffi::c_float * inv_prec;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nn_predict_c(
    mut input_nodes: *const core::ffi::c_float,
    nn_config: *const NN_CONFIG,
    mut reduce_prec: core::ffi::c_int,
    output: *mut core::ffi::c_float,
) {
    let mut num_input_nodes: core::ffi::c_int = (*nn_config).num_inputs;
    let mut buf_index: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut buf: [[core::ffi::c_float; 128]; 2] = [[0.; 128]; 2];
    let num_layers: core::ffi::c_int = (*nn_config).num_hidden_layers;
    if !(num_layers <= 10 as core::ffi::c_int) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"av1_nn_predict_c\0"))
                .as_ptr(),
            b"ml.c\0" as *const u8 as *const core::ffi::c_char,
            40 as core::ffi::c_int,
            b"num_layers <= NN_MAX_HIDDEN_LAYERS\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    let mut layer: core::ffi::c_int = 0 as core::ffi::c_int;
    while layer < num_layers {
        let mut layer_weights: *const core::ffi::c_float = (*nn_config)
            .weights[layer as usize];
        let mut layer_bias: *const core::ffi::c_float = (*nn_config)
            .bias[layer as usize];
        let mut output_nodes: *mut core::ffi::c_float = (buf[buf_index as usize])
            .as_mut_ptr();
        let num_output_nodes: core::ffi::c_int = (*nn_config)
            .num_hidden_nodes[layer as usize];
        if !(num_output_nodes < 128 as core::ffi::c_int) as core::ffi::c_int
            as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 17],
                    [core::ffi::c_char; 17],
                >(*b"av1_nn_predict_c\0"))
                    .as_ptr(),
                b"ml.c\0" as *const u8 as *const core::ffi::c_char,
                46 as core::ffi::c_int,
                b"num_output_nodes < NN_MAX_NODES_PER_LAYER\0" as *const u8
                    as *const core::ffi::c_char,
            );
        } else {};
        let mut node: core::ffi::c_int = 0 as core::ffi::c_int;
        while node < num_output_nodes {
            let mut val: core::ffi::c_float = *layer_bias.offset(node as isize);
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < num_input_nodes {
                val
                    += *layer_weights.offset((node * num_input_nodes + i) as isize)
                        * *input_nodes.offset(i as isize);
                i += 1;
            }
            val = if val > 0.0f32 { val } else { 0.0f32 };
            *output_nodes.offset(node as isize) = val;
            node += 1;
        }
        num_input_nodes = num_output_nodes;
        input_nodes = output_nodes;
        buf_index = 1 as core::ffi::c_int - buf_index;
        layer += 1;
    }
    let mut layer_weights_0: *const core::ffi::c_float = (*nn_config)
        .weights[num_layers as usize];
    let mut layer_bias_0: *const core::ffi::c_float = (*nn_config)
        .bias[num_layers as usize];
    let mut node_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while node_0 < (*nn_config).num_outputs {
        let mut val_0: core::ffi::c_float = *layer_bias_0.offset(node_0 as isize);
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < num_input_nodes {
            val_0
                += *layer_weights_0.offset((node_0 * num_input_nodes + i_0) as isize)
                    * *input_nodes.offset(i_0 as isize);
            i_0 += 1;
        }
        *output.offset(node_0 as isize) = val_0;
        node_0 += 1;
    }
    if reduce_prec != 0 {
        av1_nn_output_prec_reduce(output, (*nn_config).num_outputs);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nn_softmax(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
    mut n: core::ffi::c_int,
) {
    let mut max_input: core::ffi::c_float = *input
        .offset(0 as core::ffi::c_int as isize);
    let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
    while i < n {
        max_input = if max_input > *input.offset(i as isize) {
            max_input
        } else {
            *input.offset(i as isize)
        };
        i += 1;
    }
    let mut sum_out: core::ffi::c_float = 0.0f32;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < n {
        let normalized_input: core::ffi::c_float = if *input.offset(i_0 as isize)
            - max_input > -10.0f32
        {
            *input.offset(i_0 as isize) - max_input
        } else {
            -10.0f32
        };
        *output.offset(i_0 as isize) = expf(normalized_input);
        sum_out += *output.offset(i_0 as isize);
        i_0 += 1;
    }
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < n {
        *output.offset(i_1 as isize) /= sum_out;
        i_1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_nn_fast_softmax_16_c(
    mut input: *const core::ffi::c_float,
    mut output: *mut core::ffi::c_float,
) {
    let kNumClasses: core::ffi::c_int = 16 as core::ffi::c_int;
    let mut max_input: core::ffi::c_float = *input
        .offset(0 as core::ffi::c_int as isize);
    let mut i: core::ffi::c_int = 1 as core::ffi::c_int;
    while i < kNumClasses {
        max_input = if max_input > *input.offset(i as isize) {
            max_input
        } else {
            *input.offset(i as isize)
        };
        i += 1;
    }
    let mut sum_out: core::ffi::c_float = 0.0f32;
    let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_0 < kNumClasses {
        let normalized_input: core::ffi::c_float = if *input.offset(i_0 as isize)
            - max_input > -10.0f32
        {
            *input.offset(i_0 as isize) - max_input
        } else {
            -10.0f32
        };
        *output.offset(i_0 as isize) = approx_exp(normalized_input);
        sum_out += *output.offset(i_0 as isize);
        i_0 += 1;
    }
    let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while i_1 < kNumClasses {
        *output.offset(i_1 as isize) /= sum_out;
        i_1 += 1;
    }
}

extern "C" {
    fn expf(_: core::ffi::c_float) -> core::ffi::c_float;
}
