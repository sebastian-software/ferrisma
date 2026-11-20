// Supplementary transcription of upstream/aom/test/av1_nn_predict_test.cc
// (NnPredictTest.RandomValues for the NEON, SSE3 and AVX2 instantiations of
// av1_nn_predict) and upstream/aom/test/av1_softmax_test.cc
// (FastSoftmaxTest.RandomValues for the SSE3 instantiation of
// av1_nn_fast_softmax_16). As in the originals, every output must lie within
// the tests' tolerances of `_c` (av1_nn_predict: relative 1e-3, or near zero
// when the reference is; softmax: against av1_nn_softmax, relative 5e-2 and
// absolute 5e-3). The kernels add in another order than `_c`, so equality
// with `_c` is not required; equality with the original's SIMD build is
// checked by tools/check_aom_simd_original.py (ADR-0041). Each TEST_P starts
// from a fresh ACMRandom with the deterministic seed; the shapes, the 10000
// iterations and the value ranges follow the originals. The x86 tests skip on
// a CPU without the feature. The speed tests are not transcribed.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

use aom::av1__encoder__ml::{av1_nn_predict_c, NN_CONFIG};
#[cfg(target_arch = "aarch64")]
use aom::simd::av1__encoder__arm__ml_neon::av1_nn_predict_neon;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__ml_avx2::av1_nn_predict_avx2;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__ml_sse3::{av1_nn_fast_softmax_16_sse3, av1_nn_predict_sse3};

use super::acm_random::ACMRandom;

type NnPredictFunc = unsafe extern "C-unwind" fn(*const f32, *const NN_CONFIG, i32, *mut f32);

const NN_MAX_HIDDEN_LAYERS: usize = 10;
const NN_MAX_NODES_PER_LAYER: usize = 128;
const EPSILON: f32 = 1e-3; // Error threshold for functional equivalence

fn rand_float(rng: &mut ACMRandom) -> f32 {
    (rng.rand31() as f32 - (1u32 << 30) as f32) / (1u32 << 31) as f32
}

// All the neural network shapes observed executed in a few different runs of
// the encoder: (num_inputs, num_outputs, num_hidden_layers, num_hidden_nodes).
const K_SHAPES: [(i32, i32, i32, &[i32]); 17] = [
    (37, 1, 2, &[16, 24]), (24, 24, 1, &[12]), (10, 16, 1, &[64]), (12, 1, 1, &[12]),
    (12, 1, 1, &[24]), (12, 1, 1, &[32]), (18, 4, 1, &[24]), (18, 4, 1, &[32]),
    (4, 1, 1, &[16]), (8, 1, 0, &[0]), (8, 4, 1, &[16]), (8, 1, 1, &[32]), (9, 3, 1, &[32]),
    (8, 4, 0, &[0]), (8, 8, 0, &[0]), (4, 4, 1, &[8]), (4, 3, 0, &[64]),
];

/// NnPredictTest.RandomValues for one instantiation.
fn random_values(target: NnPredictFunc) {
    const MAX_NODES2: usize = NN_MAX_NODES_PER_LAYER * NN_MAX_NODES_PER_LAYER;
    let mut weights_buf = vec![0f32; MAX_NODES2 * (NN_MAX_HIDDEN_LAYERS + 1)];
    let mut bias_buf = vec![0f32; NN_MAX_NODES_PER_LAYER * (NN_MAX_HIDDEN_LAYERS + 1)];
    // The config and the fill below both go through these pointers.
    let (weights, bias) = (weights_buf.as_mut_ptr(), bias_buf.as_mut_ptr());
    let mut rng = ACMRandom::new();
    for &(num_inputs, num_outputs, num_hidden_layers, hidden) in K_SHAPES.iter() {
        // SAFETY: NN_CONFIG is a C record of integers and pointers.
        let mut nn_config: NN_CONFIG = unsafe { core::mem::zeroed() };
        nn_config.num_inputs = num_inputs;
        nn_config.num_outputs = num_outputs;
        nn_config.num_hidden_layers = num_hidden_layers;
        for (i, &n) in hidden.iter().enumerate() {
            nn_config.num_hidden_nodes[i] = n;
        }
        for i in 0..NN_MAX_HIDDEN_LAYERS + 1 {
            // SAFETY: inside the buffers.
            nn_config.weights[i] = unsafe { weights.add(i * MAX_NODES2) };
            nn_config.bias[i] = unsafe { bias.add(i * NN_MAX_NODES_PER_LAYER) };
        }
        let mut inputs = [0f32; NN_MAX_NODES_PER_LAYER];
        let mut outputs_test = [0f32; NN_MAX_NODES_PER_LAYER];
        let mut outputs_ref = [0f32; NN_MAX_NODES_PER_LAYER];
        for _ in 0..10000 {
            for node in 0..num_inputs as usize {
                inputs[node] = rand_float(&mut rng);
            }
            // The hidden layers, then the outputs.
            for layer in 0..=num_hidden_layers as usize {
                // SAFETY: inside the buffers.
                for node in 0..NN_MAX_NODES_PER_LAYER {
                    unsafe { *bias.add(layer * NN_MAX_NODES_PER_LAYER + node) = rand_float(&mut rng) };
                }
                for node in 0..MAX_NODES2 {
                    unsafe { *weights.add(layer * MAX_NODES2 + node) = rand_float(&mut rng) };
                }
            }
            // SAFETY: the config points into the weight and bias buffers, which
            // hold every layer the shape uses.
            unsafe {
                av1_nn_predict_c(inputs.as_ptr(), &nn_config, 0, outputs_ref.as_mut_ptr());
                target(inputs.as_ptr(), &nn_config, 0, outputs_test.as_mut_ptr());
            }
            for node in 0..num_outputs as usize {
                if outputs_ref[node] < EPSILON {
                    assert!(outputs_test[node] <= EPSILON,
                            "Reference output was near-zero, test output was not ({num_inputs})");
                } else {
                    let error = outputs_ref[node] - outputs_test[node];
                    let relative_error = (error / outputs_ref[node]).abs();
                    assert!(relative_error <= EPSILON,
                            "Excessive relative error between reference and test ({num_inputs})");
                }
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn neon_nn_predict_random_values() {
    random_values(av1_nn_predict_neon);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn sse3_nn_predict_random_values() {
    if !std::arch::is_x86_feature_detected!("sse3") {
        return;
    }
    random_values(av1_nn_predict_sse3);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn avx2_nn_predict_random_values() {
    if !std::arch::is_x86_feature_detected!("avx2") {
        return;
    }
    random_values(av1_nn_predict_avx2);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn sse3_fast_softmax_16_random_values() {
    use aom::av1__encoder__ml::av1_nn_softmax;
    // Error thresholds for functional equivalence
    const K_REL_EPSILON: f32 = 5e-2;
    const K_ABS_EPSILON: f32 = 5e-3;
    if !std::arch::is_x86_feature_detected!("sse3") {
        return;
    }
    let num_classes = 16;
    let mut rng = ACMRandom::new();
    let mut input = [0f32; 16];
    for value in input.iter_mut() {
        *value = (rng.rand31() as f32 - (1u32 << 30) as f32) / (1u32 << 30) as f32;
    }
    let (mut ref_buf, mut dst_buf) = ([0f32; 16], [0f32; 16]);
    // SAFETY: the buffers hold 16 values.
    unsafe {
        av1_nn_softmax(input.as_ptr(), ref_buf.as_mut_ptr(), num_classes);
        av1_nn_fast_softmax_16_sse3(input.as_ptr(), dst_buf.as_mut_ptr());
    }
    for idx in 0..16 {
        if ref_buf[idx] < K_ABS_EPSILON {
            assert!(dst_buf[idx] <= K_ABS_EPSILON,
                    "Reference output was near-zero, test output was not");
        } else {
            let error = dst_buf[idx] - ref_buf[idx];
            let relative_error = (error / ref_buf[idx]).abs();
            assert!(relative_error <= K_REL_EPSILON,
                    "Excessive relative error between reference and test output");
            assert!(error <= K_ABS_EPSILON,
                    "Excessive absolute error between reference and test output");
        }
    }
}
