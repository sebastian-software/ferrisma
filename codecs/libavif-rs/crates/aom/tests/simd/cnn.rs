// Supplementary transcription of upstream/aom/test/cnn_test.cc for the NEON
// and AVX2 instantiations of CNNConvolveTest.CheckOutput
// (av1_cnn_convolve_no_maxpool_padding_valid_neon and _avx2 against
// av1_cnn_convolve_no_maxpool_padding_valid_c). The x86 test skips on a CPU
// without AVX2. As in the original, the five layers of
// av1_intra_mode_cnn_partition_cnn_config run in turn from a 65x65 input, each
// on fresh ACMRandom values in [-0.5, 0.5), and every output must lie within
// the original's pixel-wise tolerance CNN_CONVOLVE_PIXELWISE_FLOAT_TOL (1e-3,
// relative, or near zero when the reference is). The kernels add in another
// order than `_c`, so equality with `_c` is not required; equality with the
// original's SIMD build is checked by tools/check_aom_simd_original.py
// (ADR-0041). The layer configs are read from the original header
// av1/encoder/partition_cnn_weights.h, which the test includes. The speed
// test is not transcribed.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

use aom::av1__encoder__cnn::{av1_cnn_convolve_no_maxpool_padding_valid_c,
                             av1_find_cnn_layer_output_size, CNN_LAYER_CONFIG, PADDING_VALID,
                             RELU};
#[cfg(target_arch = "aarch64")]
use aom::simd::av1__encoder__arm__cnn_neon::av1_cnn_convolve_no_maxpool_padding_valid_neon;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__cnn_avx2::av1_cnn_convolve_no_maxpool_padding_valid_avx2;

use super::acm_random::ACMRandom;

type ConvolveFunc = unsafe extern "C-unwind" fn(*mut *const f32, i32, i32, i32,
                                                 *const CNN_LAYER_CONFIG, *mut *mut f32, i32,
                                                 i32, i32, i32);

const HEADER: &str = include_str!("../../../../upstream/aom/av1/encoder/partition_cnn_weights.h");
const CNN_CONVOLVE_PIXELWISE_FLOAT_TOL: f32 = 1e-3;
// av1_intra_mode_cnn_partition_cnn_config's output_num per layer.
const OUTPUT_NUM: [i32; 5] = [-1, 3, 2, 1, 0];

fn define(name: &str) -> i32 {
    let key = format!("#define {name} ");
    let line = HEADER.lines().find(|line| line.starts_with(&key)).expect(name);
    line[key.len()..].trim().parse().expect(name)
}

fn table(name: &str) -> Vec<f32> {
    let start = HEADER.find(&format!("static const float {name}[] = {{")).expect(name);
    let body = &HEADER[start..];
    let body = &body[body.find('{').unwrap() + 1..body.find("};").unwrap()];
    body.split(',').map(str::trim).filter(|value| !value.is_empty())
        .map(|value| value.trim_end_matches('f').parse::<f32>().expect(name)).collect()
}

/// The layer configs of av1_intra_mode_cnn_partition_cnn_config with their
/// weight and bias tables, which must outlive the configs.
fn layer_configs() -> (Vec<CNN_LAYER_CONFIG>, Vec<(Vec<f32>, Vec<f32>)>) {
    let tables: Vec<(Vec<f32>, Vec<f32>)> = (0..5).map(|layer| {
        (table(&format!("av1_intra_mode_cnn_partition_cnn_layer_{layer}_kernel")),
         table(&format!("av1_intra_mode_cnn_partition_cnn_layer_{layer}_bias")))
    }).collect();
    let configs = (0..5).map(|layer| {
        // SAFETY: CNN_LAYER_CONFIG is a C record of integers and pointers; all
        // fields the original sets to 0 or NULL stay zero.
        let mut config: CNN_LAYER_CONFIG = unsafe { core::mem::zeroed() };
        config.in_channels = define(&format!("CNN_LAYER_{layer}_IN_CH"));
        config.filter_width = define(&format!("CNN_LAYER_{layer}_WIDTH"));
        config.filter_height = define(&format!("CNN_LAYER_{layer}_WIDTH"));
        config.out_channels = define(&format!("CNN_LAYER_{layer}_OUT_CH"));
        config.skip_width = define(&format!("CNN_LAYER_{layer}_HORZ_STRIDE"));
        config.skip_height = define(&format!("CNN_LAYER_{layer}_VERT_STRIDE"));
        config.weights = tables[layer].0.as_ptr();
        config.bias = tables[layer].1.as_ptr();
        config.pad = PADDING_VALID as _;
        config.activation = RELU as _;
        config.output_num = OUTPUT_NUM[layer];
        assert_eq!(tables[layer].0.len() as i32,
                   config.in_channels * config.out_channels * config.filter_width
                   * config.filter_height);
        assert_eq!(tables[layer].1.len() as i32, config.out_channels);
        config
    }).collect();
    (configs, tables)
}

/// CNNConvolveTest.CheckOutput for one instantiation.
fn check_output(target: ConvolveFunc) {
    let (configs, _tables) = layer_configs();
    let mut rng = ACMRandom::new();
    let (mut in_width, mut in_height) = (65, 65);
    for (layer, config) in configs.iter().enumerate() {
        let (mut out_width, mut out_height) = (0, 0);
        let in_size = (in_width * in_height) as usize;
        // SAFETY: the config is a valid layer config.
        unsafe { av1_find_cnn_layer_output_size(in_height, in_width, config, &mut out_width,
                                                &mut out_height) };
        let out_size = (out_width * out_height) as usize;
        let mut input_data = vec![0f32; in_size * config.in_channels as usize];
        for value in input_data.iter_mut() {
            *value = (rng.rand31() as f32 - (1u32 << 30) as f32) / (1u32 << 31) as f32;
        }
        let mut input: Vec<*const f32> = input_data.chunks(in_size).map(|c| c.as_ptr()).collect();
        let mut out_ref = vec![0f32; out_size * config.out_channels as usize];
        let mut out_mod = vec![0f32; out_size * config.out_channels as usize];
        let mut output_ref: Vec<*mut f32> =
            out_ref.chunks_mut(out_size).map(|c| c.as_mut_ptr()).collect();
        let mut output_mod: Vec<*mut f32> =
            out_mod.chunks_mut(out_size).map(|c| c.as_mut_ptr()).collect();
        let cstep = config.in_channels * config.out_channels;
        // SAFETY: input and output planes have the sizes the layer needs.
        unsafe {
            av1_cnn_convolve_no_maxpool_padding_valid_c(input.as_mut_ptr(), in_width, in_height,
                                                        in_width, config,
                                                        output_ref.as_mut_ptr(), out_width, 0,
                                                        cstep, 1);
            target(input.as_mut_ptr(), in_width, in_height, in_width, config,
                   output_mod.as_mut_ptr(), out_width, 0, cstep, 1);
        }
        for (channel, (buf_ref, buf_mod)) in
            out_ref.chunks(out_size).zip(out_mod.chunks(out_size)).enumerate()
        {
            for i in 0..out_size {
                if buf_ref[i] < CNN_CONVOLVE_PIXELWISE_FLOAT_TOL {
                    assert!(buf_ref[i] <= CNN_CONVOLVE_PIXELWISE_FLOAT_TOL,
                            "Reference output was near-zero, test output was not ({})",
                            buf_mod[i]);
                } else {
                    let error = buf_ref[i] - buf_mod[i];
                    let relative_error = (error / buf_ref[i]).abs();
                    assert!(relative_error <= CNN_CONVOLVE_PIXELWISE_FLOAT_TOL,
                            "layer {layer} channel {channel} pixel {i}: {}/{}", buf_ref[i],
                            buf_mod[i]);
                }
            }
        }
        in_width = out_width;
        in_height = out_height;
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn neon_cnn_convolve_check_output() {
    check_output(av1_cnn_convolve_no_maxpool_padding_valid_neon);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn avx2_cnn_convolve_check_output() {
    if !std::arch::is_x86_feature_detected!("avx2") {
        return;
    }
    check_output(av1_cnn_convolve_no_maxpool_padding_valid_avx2);
}
