//! WP19 parity test (ADR 0008, rule 1): the x86 encoder SIMD variants against
//! the C translations they replace, Rust against Rust. Each record runs a
//! dispatch function twice on the same input, once with the CPU query
//! disabled (`-noasm`: the C translation) and once with the built-in query
//! (the SSE2, SSE4.1 or AVX2 variant that the host, or `LIBWEBP_SIMD_LEVEL`
//! with the `oracle` feature, selects), and compares every output. The inputs
//! stay in the domain the encoder passes, where upstream's SIMD variants equal
//! upstream's C functions; outside it the differential gate against the
//! `release-simd` oracle compares the SIMD variants with upstream's.
//!
//! One test function toggles the process-wide query, so no other test in this
//! binary dispatches concurrently.

#![cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "seeded test inputs in known ranges"
)]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::abi::decode::{disable_cpu_query, use_builtin_cpu_query};
use webp::dsp::dsp::{BPS, VP8Histogram};
use webp::dsp::enc::{
    vp8_collect_histogram, vp8_enc_pred_chroma8, vp8_enc_pred_luma4, vp8_enc_pred_luma16,
    vp8_enc_quantize_block, vp8_enc_quantize_block_wht, vp8_enc_quantize2_blocks, vp8_mean16x4,
    vp8f_transform, vp8f_transform_wht, vp8f_transform2, vp8i_transform, vp8sse4x4, vp8sse8x8,
    vp8sse16x8, vp8sse16x16, vp8t_disto4x4, vp8t_disto16x16,
};
use webp::dsp::lossless::VP8LMultipliers;
use webp::dsp::lossless_enc::{
    vp8l_add_vector, vp8l_add_vector_eq, vp8l_bundle_color_map, vp8l_collect_color_blue_transforms,
    vp8l_collect_color_red_transforms, vp8l_combined_shannon_entropy, vp8l_extra_cost,
    vp8l_predictors_sub, vp8l_subtract_green_from_blue_and_red, vp8l_transform_color,
    vp8l_vector_mismatch,
};
use webp::dsp::ssim::{vp8_accumulate_sse, vp8ssim_get};
use webp::enc::vp8i_enc::VP8Matrix;
use webp::sharpyuv::sharpyuv_dsp::{
    sharp_yuv_filter_row, sharp_yuv_update_rgb, sharp_yuv_update_y,
};

const BPS_U: usize = BPS as usize;

/// Runs `f` with the C translation and with the SIMD variant and asserts that
/// both give the same value.
fn same<T: PartialEq + std::fmt::Debug>(what: &str, mut f: impl FnMut() -> T) {
    disable_cpu_query();
    let c = f();
    use_builtin_cpu_query();
    let simd = f();
    assert_eq!(
        c, simd,
        "{what}: the SIMD variant differs from the C translation"
    );
}

fn lossless(rng: &mut Rng) {
    for n in 0..70 {
        let data = rng.words(n);
        same("subtract green", || {
            let mut d = data.clone();
            vp8l_subtract_green_from_blue_and_red(&mut d, n as i32);
            d
        });
        let m = VP8LMultipliers {
            green_to_red: rng.byte(),
            green_to_blue: rng.byte(),
            red_to_blue: rng.byte(),
        };
        same("transform color", || {
            let mut d = data.clone();
            vp8l_transform_color(&m, &mut d, n as i32);
            d
        });
    }
    for _ in 0..200 {
        let w = 1 + rng.below(40) as i32;
        let h = 1 + rng.below(5) as i32;
        let stride = w + rng.below(4) as i32;
        let argb = rng.words((stride * h) as usize);
        let (g2b, r2b, g2r) = (
            rng.below(256) as i32,
            rng.below(256) as i32,
            rng.below(256) as i32,
        );
        same("collect blue", || {
            let mut histo = vec![0_u32; 256];
            vp8l_collect_color_blue_transforms(&argb, stride, w, h, g2b, r2b, &mut histo);
            histo
        });
        same("collect red", || {
            let mut histo = vec![0_u32; 256];
            vp8l_collect_color_red_transforms(&argb, stride, w, h, g2r, &mut histo);
            histo
        });
    }
    for length in [24, 40] {
        for _ in 0..50 {
            let pop: Vec<u32> = (0..length).map(|_| rng.below(5000) as u32).collect();
            same("extra cost", || vp8l_extra_cost(&pop, length));
        }
    }
    for _ in 0..200 {
        let sparse = rng.below(4) + 1;
        let x: Vec<u32> = (0..256)
            .map(|_| {
                if rng.below(sparse) == 0 {
                    rng.below(1000) as u32
                } else {
                    0
                }
            })
            .collect();
        let y: Vec<u32> = (0..256)
            .map(|_| {
                if rng.below(sparse) == 0 {
                    rng.below(1000) as u32
                } else {
                    0
                }
            })
            .collect();
        same("combined entropy", || vp8l_combined_shannon_entropy(&x, &y));
    }
    for size in [
        40, 280, 282, 284, 288, 296, 312, 344, 408, 536, 792, 1304, 2328,
    ] {
        let a = rng.words(size);
        let b = rng.words(size);
        same("add vector", || {
            let mut out = vec![0_u32; size];
            vp8l_add_vector(&a, &b, &mut out, size as i32);
            out
        });
        same("add vector eq", || {
            let mut out = b.clone();
            vp8l_add_vector_eq(&a, &mut out, size as i32);
            out
        });
    }
    for length in 0..100 {
        let a = rng.words(length + 8);
        let mut b = a.clone();
        let cut = rng.below(length as u64 + 1) as usize;
        if cut < length {
            b[cut] ^= 1;
        }
        same("vector mismatch", || {
            vp8l_vector_mismatch(&a, &b, length as i32)
        });
    }
    for xbits in 0..=3 {
        for width in 1..80 {
            let bits = 8 >> xbits;
            let row: Vec<u8> = (0..width).map(|_| rng.below(1 << bits) as u8).collect();
            let size = ((width + (1 << xbits) - 1) >> xbits) as usize;
            same("bundle color map", || {
                let mut dst = vec![0_u32; size];
                vp8l_bundle_color_map(&row, width, xbits, &mut dst);
                dst
            });
        }
    }
    for mode in 0..16 {
        for n in 1..40 {
            // The scratch rows: the row above and the input row, with the
            // pixel left of each and the top-right pixel past the end.
            let width = n + 2;
            let data = rng.words(2 * width + 1);
            let upper_pos = 1;
            let in_pos = width + 1;
            same("predictor sub", || {
                let mut out = vec![0_u32; n];
                vp8l_predictors_sub(mode, &data, in_pos, upper_pos, n as i32, &mut out);
                out
            });
        }
    }
}

/// A quantization matrix as `ExpandMatrix` (`quant_enc.c`) builds one: the
/// bias `BIAS(b) = b << (QFIX - 8)` with `b` from 96 to 115, `zthresh` from
/// it, and the sharpening `(kFreqSharpening[i] * q) >> SHARPEN_BITS` with the
/// factor at most 24, so that sharpening alone never quantizes to a nonzero
/// level (where `_mm_sign_epi16` of `QuantizeBlock_SSE41` would keep a zero
/// input at zero and the C function would not).
fn matrix(rng: &mut Rng) -> VP8Matrix {
    let mut mtx = VP8Matrix::default();
    for i in 0..16 {
        let q = 8 + rng.below(120) as u32;
        let iq = (1_u32 << 17) / q;
        mtx.q[i] = q as u16;
        mtx.iq[i] = iq as u16;
        mtx.bias[i] = (96 + rng.below(20) as u32) << (17 - 8);
        mtx.zthresh[i] = ((1_u32 << 17) - 1 - mtx.bias[i]) / iq;
        mtx.sharpen[i] = ((rng.below(25) as u32 * q) >> 11) as u16;
    }
    mtx
}

fn lossy(rng: &mut Rng) {
    for _ in 0..300 {
        let src = rng.edge_bytes(16 * BPS_U + 16);
        let reference = rng.edge_bytes(16 * BPS_U + 16);
        let coeffs = rng.i16s(32, 1024);
        let do_two = rng.below(2) == 1;
        same("ITransform", || {
            let mut dst = rng_copy(&src);
            vp8i_transform(&reference, &coeffs, &mut dst, do_two);
            dst
        });
        same("FTransform", || {
            let mut out = [0_i16; 16];
            vp8f_transform(&src, &reference, &mut out);
            out
        });
        same("FTransform2", || {
            let mut out = [0_i16; 32];
            vp8f_transform2(&src, &reference, &mut out);
            out
        });
        let wht_in = rng.i16s(256, 2047);
        same("FTransformWHT", || {
            let mut out = [0_i16; 16];
            vp8f_transform_wht(&wht_in, &mut out);
            out
        });
        same("SSE", || {
            (
                vp8sse16x16(&src, &reference),
                vp8sse16x8(&src, &reference),
                vp8sse8x8(&src, &reference),
                vp8sse4x4(&src, &reference),
            )
        });
        // Symmetric weights, as the encoder's kWeightY and kWeightTrellis.
        let mut w = [0_u16; 16];
        for i in 0..4 {
            for j in i..4 {
                let v = rng.below(40) as u16;
                w[4 * i + j] = v;
                w[4 * j + i] = v;
            }
        }
        same("TDisto", || {
            (
                vp8t_disto4x4(&src, &reference, &w),
                vp8t_disto16x16(&src, &reference, &w),
            )
        });
        same("Mean16x4", || {
            let mut dc = [0_u32; 4];
            vp8_mean16x4(&src, &mut dc);
            dc
        });
        let mtx = matrix(rng);
        // The second-transform matrix (y2) has no sharpening: `QuantizeBlockWHT`
        // is `QuantizeBlock_C` with `sharpen`, `QuantizeBlockWHT_SSE2` without.
        let mtx_wht = VP8Matrix {
            sharpen: [0; 16],
            ..mtx
        };
        let input = rng.i16s(32, 2047);
        same("Quantize", || {
            let mut a = input.clone();
            let mut out = [0_i16; 32];
            let nz1 = vp8_enc_quantize_block(&mut a, &mut out, &mtx);
            let mut b = input.clone();
            let mut out2 = [0_i16; 32];
            let nz2 = vp8_enc_quantize2_blocks(&mut b, &mut out2, &mtx);
            let mut c = input.clone();
            let mut out3 = [0_i16; 16];
            let nz3 = vp8_enc_quantize_block_wht(&mut c, &mut out3, &mtx_wht);
            (a, out, nz1, b, out2, nz2, c, out3, nz3)
        });
    }
    // Histograms over a 16x16 macroblock: 16 luma blocks.
    for _ in 0..50 {
        let reference = rng.edge_bytes(16 * BPS_U);
        let pred = rng.edge_bytes(16 * BPS_U);
        same("CollectHistogram", || {
            let mut histo = VP8Histogram::default();
            vp8_collect_histogram(&reference, &pred, 0, 16, &mut histo);
            (histo.max_value, histo.last_non_zero)
        });
    }
    // Predictions: every combination of missing left and top samples.
    for _ in 0..50 {
        let left = rng.bytes(33);
        let top = rng.bytes(32);
        for (has_left, has_top) in [(false, false), (false, true), (true, false), (true, true)] {
            let left_arg = has_left.then_some((left.as_slice(), 1));
            let top_arg = has_top.then_some(top.as_slice());
            same("Intra16Preds", || {
                let mut dst = vec![0_u8; 4 * 16 * BPS_U];
                vp8_enc_pred_luma16(&mut dst, left_arg, top_arg);
                dst
            });
            same("IntraChromaPreds", || {
                let mut dst = vec![0_u8; 4 * 16 * BPS_U];
                vp8_enc_pred_chroma8(&mut dst, left_arg, top_arg);
                dst
            });
        }
        let top4 = rng.bytes(16);
        same("Intra4Preds", || {
            let mut dst = vec![0_u8; 4 * 16 * BPS_U];
            vp8_enc_pred_luma4(&mut dst, &top4, 5);
            dst
        });
    }
}

fn rng_copy(src: &[u8]) -> Vec<u8> {
    src.to_vec()
}

fn ssim_and_sharpyuv(rng: &mut Rng) {
    for len in 0..100 {
        let a = rng.bytes(len);
        let b = rng.bytes(len);
        same("AccumulateSSE", || vp8_accumulate_sse(&a, &b, len as i32));
    }
    for _ in 0..100 {
        let stride1 = 8 + rng.below(8) as usize;
        let stride2 = 8 + rng.below(8) as usize;
        let a = rng.bytes(7 * stride1 + 8);
        let b = rng.bytes(7 * stride2 + 8);
        same("SSIMGet", || {
            vp8ssim_get(&a, stride1 as i32, &b, stride2 as i32).to_bits()
        });
    }
    for bit_depth in [8, 10, 12] {
        let max = (1_i32 << bit_depth) - 1;
        for len in 0..40 {
            let r: Vec<u16> = (0..len).map(|_| rng.below(max as u64 + 1) as u16).collect();
            let s: Vec<u16> = (0..len).map(|_| rng.below(max as u64 + 1) as u16).collect();
            let d: Vec<u16> = (0..len).map(|_| rng.below(max as u64 + 1) as u16).collect();
            same("SharpYuvUpdateY", || {
                let mut dst = d.clone();
                let diff = sharp_yuv_update_y(&r, &s, &mut dst, len as i32, bit_depth);
                (dst, diff)
            });
            // Chroma and luma values in the converter's range: the RGB planes
            // and their differences stay within the bit depth plus sign.
            let ri: Vec<i16> = (0..len)
                .map(|_| (rng.below(2 * max as u64) as i32 - max) as i16)
                .collect();
            let si: Vec<i16> = (0..len)
                .map(|_| (rng.below(2 * max as u64) as i32 - max) as i16)
                .collect();
            let di: Vec<i16> = (0..len)
                .map(|_| (rng.below(max as u64) as i32) as i16)
                .collect();
            same("SharpYuvUpdateRGB", || {
                let mut dst = di.clone();
                sharp_yuv_update_rgb(&ri, &si, &mut dst, len as i32);
                dst
            });
            let a: Vec<i16> = (0..=len)
                .map(|_| (rng.below(max as u64) as i32 - max / 2) as i16)
                .collect();
            let b: Vec<i16> = (0..=len)
                .map(|_| (rng.below(max as u64) as i32 - max / 2) as i16)
                .collect();
            let best_y: Vec<u16> = (0..2 * len + 2)
                .map(|_| rng.below(max as u64 + 1) as u16)
                .collect();
            same("SharpYuvFilterRow", || {
                let mut out = vec![0_u16; 2 * len + 2];
                sharp_yuv_filter_row(&a, &b, len as i32, &best_y, &mut out, bit_depth);
                out
            });
        }
    }
}

#[test]
fn encoder_simd_variants_match_the_c_translations() {
    let mut rng = Rng(0x1919_0001);
    lossless(&mut rng);
    lossy(&mut rng);
    ssim_and_sharpyuv(&mut rng);
    use_builtin_cpu_query();
}
