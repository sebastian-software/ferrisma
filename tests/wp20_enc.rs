//! WP20 parity test: the NEON variants of `src/dsp/enc_neon.c` and the NEON
//! `IsFlat` of `src/dsp/quant.h` against the C translations they replace
//! (`src/dsp/enc.rs`, `src/dsp/quant.rs`), Rust against Rust, and each
//! dispatch function against its C translation. Blocks start at offsets 0 to
//! 3 of their buffers, and the buffers end at the last byte the C function
//! reads or writes (except where the NEON function reads further, as
//! `Intra4Preds_NEON` does), so an access outside the C range panics.
//!
//! The inputs are the ones the encoder passes: any samples for the
//! transforms, metrics and predictors; dequantized coefficients up to 4096
//! in magnitude for the inverse transform, whose NEON form saturates 16-bit
//! sums that C computes in `int` (the encoder's stay below about 2300);
//! symmetric weight matrices for the texture distortion, which the NEON
//! form computes transposed (`kWeightY`, `kWeightTrellis` and random
//! symmetric ones); and quantization matrices as `ExpandMatrix` builds them
//! from every step 4 to 440, for any coefficient.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::dsp::dsp::{BPS, VP8Histogram};
use webp::dsp::enc::{self as c, PredLeft};
use webp::dsp::enc_neon as neon;
use webp::dsp::quant;
use webp::enc::vp8i_enc::{PRED_SIZE_ENC, VP8Matrix};

const BPS_U: usize = BPS as usize;

/// The weights of the texture distortion in `quant_enc.c`.
const WEIGHT_Y: [u16; 16] = [38, 32, 20, 9, 32, 28, 17, 7, 20, 17, 10, 4, 9, 7, 4, 2];
const WEIGHT_TRELLIS: [u16; 16] = [30, 27, 19, 11, 27, 24, 17, 10, 19, 17, 12, 8, 11, 10, 8, 6];

/// `kBiasMatrices` and `kFreqSharpening` of `quant_enc.c`.
const BIAS_MATRICES: [[u32; 2]; 3] = [[96, 110], [96, 108], [110, 115]];
const FREQ_SHARPENING: [u32; 16] = [
    0, 30, 60, 90, 30, 60, 90, 90, 60, 90, 90, 90, 90, 90, 90, 90,
];

/// A block area: random samples, a flat value, the extremes, a checkerboard
/// of 0 and 255, or a ramp.
fn area(rng: &mut Rng, len: usize) -> Vec<u8> {
    match rng.below(6) {
        0 => vec![0; len],
        1 => vec![255; len],
        2 => vec![rng.byte(); len],
        3 => (0..len)
            .map(|i| {
                if (i + i / BPS_U).is_multiple_of(2) {
                    0
                } else {
                    255
                }
            })
            .collect(),
        4 => (0..len)
            .map(|i| ((i % BPS_U) * 7 + (i / BPS_U) * 5) as u8)
            .collect(),
        _ => rng.edge_bytes(len),
    }
}

/// `len` bytes of `area` after `offset` bytes of padding.
fn padded(rng: &mut Rng, offset: usize, len: usize) -> Vec<u8> {
    let mut buffer = rng.bytes(offset);
    buffer.extend(area(rng, len));
    buffer
}

/// The extent of a `w` by `h` block with stride `BPS`.
fn extent(w: usize, h: usize) -> usize {
    (h - 1) * BPS_U + w
}

#[test]
fn forward_transform_matches_c() {
    let mut rng = Rng(0x2020_e001);
    let len = extent(4, 4);
    let mut cases = 0;
    for _ in 0..20_000 {
        let offset = rng.below(4) as usize;
        let src = padded(&mut rng, offset, len);
        let ref_ = padded(&mut rng, offset, len);
        let mut expected = [0_i16; 16];
        let mut actual = [0x55_i16; 16];
        let mut dispatched = [0x33_i16; 16];
        c::f_transform_c(&src[offset..], &ref_[offset..], &mut expected);
        neon::f_transform(&src[offset..], &ref_[offset..], &mut actual);
        c::vp8f_transform(&src[offset..], &ref_[offset..], &mut dispatched);
        assert_eq!(actual, expected, "src={src:?} ref={ref_:?}");
        assert_eq!(dispatched, expected);
        cases += 1;
    }
    // FTransform2_C goes through the dispatch function.
    for _ in 0..2000 {
        let src = area(&mut rng, extent(8, 4));
        let ref_ = area(&mut rng, extent(8, 4));
        let mut expected = [0_i16; 32];
        let mut actual = [0_i16; 32];
        c::f_transform_c(&src, &ref_, &mut expected[..16]);
        c::f_transform_c(&src[4..], &ref_[4..], &mut expected[16..]);
        c::vp8f_transform2(&src, &ref_, &mut actual);
        assert_eq!(actual, expected);
    }
    assert_eq!(cases, 20_000);
}

#[test]
fn inverse_transform_matches_c() {
    let mut rng = Rng(0x2020_e002);
    let mut cases = 0;
    for bound in [0_i32, 1, 16, 255, 1024, 2048, 2300, 4096] {
        for round in 0..3000 {
            let do_two = round % 2 == 1;
            let width = if do_two { 8 } else { 4 };
            let offset = rng.below(4) as usize;
            let ref_ = padded(&mut rng, offset, extent(width, 4));
            let dst = padded(&mut rng, offset, extent(width, 4));
            let input: Vec<i16> = match round % 5 {
                // The extremes in every coefficient.
                0 => (0..32)
                    .map(|_| if rng.below(2) == 0 { bound } else { -bound } as i16)
                    .collect(),
                // A DC-only block.
                1 => {
                    let mut v = vec![0_i16; 32];
                    v[0] = rng.i16s(1, bound)[0];
                    v[16] = rng.i16s(1, bound)[0];
                    v
                }
                _ => rng.i16s(32, bound),
            };
            let input = &input[..if do_two { 32 } else { 16 }];
            let mut expected = dst.clone();
            let mut actual = dst.clone();
            let mut dispatched = dst;
            c::i_transform_c(&ref_[offset..], input, &mut expected[offset..], do_two);
            neon::i_transform(&ref_[offset..], input, &mut actual[offset..], do_two);
            c::vp8i_transform(&ref_[offset..], input, &mut dispatched[offset..], do_two);
            assert_eq!(actual, expected, "bound={bound} input={input:?}");
            assert_eq!(dispatched, expected);
            cases += 1;
        }
    }
    assert_eq!(cases, 24_000);
}

#[test]
fn walsh_hadamard_transform_matches_c() {
    let mut rng = Rng(0x2020_e003);
    for round in 0..20_000 {
        // The transform reads in[0], in[16], ..., in[240].
        let range = [1, 255, 2048, 4096, 16_383, 32_767][round % 6];
        let input: Vec<i16> = match round % 7 {
            0 => (0..241)
                .map(|_| {
                    if rng.below(2) == 0 {
                        i16::MIN
                    } else {
                        i16::MAX
                    }
                })
                .collect(),
            _ => rng.i16s(241, range),
        };
        let mut expected = [0_i16; 16];
        let mut actual = [1_i16; 16];
        let mut dispatched = [2_i16; 16];
        c::f_transform_wht_c(&input, &mut expected);
        neon::f_transform_wht(&input, &mut actual);
        c::vp8f_transform_wht(&input, &mut dispatched);
        assert_eq!(actual, expected);
        assert_eq!(dispatched, expected);
    }
}

#[test]
fn squared_errors_match_c() {
    let mut rng = Rng(0x2020_e004);
    type Pair = (
        fn(&[u8], &[u8]) -> i32,
        fn(&[u8], &[u8]) -> i32,
        fn(&[u8], &[u8]) -> i32,
    );
    let functions: [(usize, usize, Pair); 4] = [
        (16, 16, (c::sse16x16_c, neon::sse16x16, c::vp8sse16x16)),
        (16, 8, (c::sse16x8_c, neon::sse16x8, c::vp8sse16x8)),
        (8, 8, (c::sse8x8_c, neon::sse8x8, c::vp8sse8x8)),
        (4, 4, (c::sse4x4_c, neon::sse4x4, c::vp8sse4x4)),
    ];
    for (w, h, (c_fn, neon_fn, dispatch)) in functions {
        for _ in 0..5000 {
            let offset = rng.below(4) as usize;
            let a = padded(&mut rng, offset, extent(w, h));
            let b = padded(&mut rng, offset, extent(w, h));
            let expected = c_fn(&a[offset..], &b[offset..]);
            assert_eq!(neon_fn(&a[offset..], &b[offset..]), expected, "{w}x{h}");
            assert_eq!(dispatch(&a[offset..], &b[offset..]), expected);
        }
        // The largest error.
        let a = vec![0_u8; extent(w, h)];
        let b = vec![255_u8; extent(w, h)];
        assert_eq!(neon_fn(&a, &b), c_fn(&a, &b));
        assert_eq!(c_fn(&a, &b), (w * h * 255 * 255) as i32);
    }
}

/// A random symmetric weight matrix with entries up to `max`.
fn symmetric_weights(rng: &mut Rng, max: u64) -> [u16; 16] {
    let mut w = [0_u16; 16];
    for i in 0..4 {
        for j in i..4 {
            let v = rng.below(max + 1) as u16;
            w[4 * i + j] = v;
            w[4 * j + i] = v;
        }
    }
    w
}

#[test]
fn texture_distortion_matches_c_for_symmetric_weights() {
    let mut rng = Rng(0x2020_e005);
    for round in 0..6000 {
        let w = match round % 4 {
            0 => WEIGHT_Y,
            1 => WEIGHT_TRELLIS,
            2 => symmetric_weights(&mut rng, 64),
            _ => symmetric_weights(&mut rng, 4095),
        };
        let offset = rng.below(4) as usize;
        let a = padded(&mut rng, offset, extent(4, 4));
        let b = padded(&mut rng, offset, extent(4, 4));
        let expected = c::disto4x4_c(&a[offset..], &b[offset..], &w);
        assert_eq!(neon::disto4x4(&a[offset..], &b[offset..], &w), expected);
        assert_eq!(c::vp8t_disto4x4(&a[offset..], &b[offset..], &w), expected);

        let a = padded(&mut rng, offset, extent(16, 16));
        let b = padded(&mut rng, offset, extent(16, 16));
        let expected = c::disto16x16_c(&a[offset..], &b[offset..], &w);
        assert_eq!(neon::disto16x16(&a[offset..], &b[offset..], &w), expected);
        assert_eq!(c::vp8t_disto16x16(&a[offset..], &b[offset..], &w), expected);
    }
}

#[test]
fn histogram_matches_c() {
    let mut rng = Rng(0x2020_e006);
    for _ in 0..4000 {
        let offset = rng.below(4) as usize;
        let ref_ = padded(&mut rng, offset, 16 * BPS_U);
        let pred = padded(&mut rng, offset, 16 * BPS_U);
        let start = rng.below(25) as i32;
        let end = start + rng.below(25 - start as u64) as i32;
        let mut expected = VP8Histogram {
            max_value: 7,
            last_non_zero: 9,
        };
        let mut actual = expected;
        let mut dispatched = expected;
        c::collect_histogram_c(&ref_[offset..], &pred[offset..], start, end, &mut expected);
        neon::collect_histogram(&ref_[offset..], &pred[offset..], start, end, &mut actual);
        c::vp8_collect_histogram(
            &ref_[offset..],
            &pred[offset..],
            start,
            end,
            &mut dispatched,
        );
        assert_eq!(actual, expected, "{start}..{end}");
        assert_eq!(dispatched, expected);
    }
}

/// A matrix as `ExpandMatrix` builds it for the coefficient type `type_`
/// (0: luma AC, 1: luma DC, 2: chroma) from the steps `dc` and `ac`.
fn matrix(type_: usize, dc: u16, ac: u16) -> VP8Matrix {
    let mut m = VP8Matrix::default();
    for i in 0..16 {
        m.q[i] = if i == 0 { dc } else { ac };
        m.iq[i] = ((1_u32 << 17) / u32::from(m.q[i])) as u16;
        m.bias[i] = BIAS_MATRICES[type_][usize::from(i > 0)] << 9;
        m.zthresh[i] = ((1_u32 << 17) - 1 - m.bias[i]) / u32::from(m.iq[i]);
        m.sharpen[i] = if type_ == 0 {
            ((FREQ_SHARPENING[i] * u32::from(m.q[i])) >> 11) as u16
        } else {
            0
        };
    }
    m
}

/// Coefficients: transform-sized, the full `int16_t` range, the extremes, or
/// sparse.
fn coefficients(rng: &mut Rng, count: usize) -> Vec<i16> {
    match rng.below(5) {
        0 => rng.i16s(count, 2048),
        1 => (0..count).map(|_| rng.next_u64() as i16).collect(),
        2 => (0..count)
            .map(|_| [i16::MIN, i16::MAX, -1, 0, 1][rng.below(5) as usize])
            .collect(),
        3 => (0..count)
            .map(|_| {
                if rng.below(4) == 0 {
                    rng.i16s(1, 64)[0]
                } else {
                    0
                }
            })
            .collect(),
        _ => rng.i16s(count, 300),
    }
}

#[test]
fn quantizers_match_c_for_the_encoder_matrices() {
    let mut rng = Rng(0x2020_e007);
    let mut cases = 0;
    for step in 4..=440_u16 {
        for type_ in 0..3 {
            for _ in 0..8 {
                let other = 4 + rng.below(437) as u16;
                let (dc, ac) = if rng.below(2) == 0 {
                    (step, other)
                } else {
                    (other, step)
                };
                let mtx = matrix(type_, dc, ac);
                let input = coefficients(&mut rng, 32);

                let mut in_c = input.clone();
                let mut in_n = input.clone();
                let mut in_d = input.clone();
                let mut out_c = [0x11_i16; 16];
                let mut out_n = [0x22_i16; 16];
                let mut out_d = [0x33_i16; 16];
                let r_c = c::quantize_block_c(&mut in_c[..16], &mut out_c, &mtx);
                let r_n = neon::quantize_block(&mut in_n[..16], &mut out_n, &mtx);
                let r_d = c::vp8_enc_quantize_block(&mut in_d[..16], &mut out_d, &mtx);
                assert_eq!((r_n, &in_n, out_n), (r_c, &in_c, out_c), "dc={dc} ac={ac}");
                assert_eq!((r_d, &in_d, out_d), (r_c, &in_c, out_c));

                let mut in_d = input.clone();
                let mut out_d = [0x33_i16; 16];
                let r_d = c::vp8_enc_quantize_block_wht(&mut in_d[..16], &mut out_d, &mtx);
                assert_eq!((r_d, &in_d, out_d), (r_c, &in_c, out_c));

                let mut in_c = input.clone();
                let mut in_n = input.clone();
                let mut in_d = input;
                let mut out_c = [0x11_i16; 32];
                let mut out_n = [0x22_i16; 32];
                let mut out_d = [0x33_i16; 32];
                let r_c = c::quantize2_blocks_c(&mut in_c, &mut out_c, &mtx);
                let r_n = neon::quantize2_blocks(&mut in_n, &mut out_n, &mtx);
                let r_d = c::vp8_enc_quantize2_blocks(&mut in_d, &mut out_d, &mtx);
                assert_eq!((r_n, &in_n, out_n), (r_c, &in_c, out_c));
                assert_eq!((r_d, &in_d, out_d), (r_c, &in_c, out_c));
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 437 * 3 * 8);
}

#[test]
fn intra4_predictions_match_c() {
    let mut rng = Rng(0x2020_e008);
    // The iterator's i4_boundary has 40 entries, of which Intra4Preds_NEON
    // reads the 16 from top[-5].
    for top_pos in 5..=24 {
        for _ in 0..500 {
            let top = area(&mut rng, 40);
            let dst = rng.bytes(PRED_SIZE_ENC as usize);
            let mut expected = dst.clone();
            let mut actual = dst.clone();
            let mut dispatched = dst;
            c::intra4_preds_c(&mut expected, &top, top_pos);
            neon::intra4_preds(&mut actual, &top, top_pos);
            c::vp8_enc_pred_luma4(&mut dispatched, &top, top_pos);
            assert_eq!(actual, expected, "top_pos={top_pos} top={top:?}");
            assert_eq!(dispatched, expected);
        }
    }
}

#[test]
fn intra16_predictions_match_c() {
    let mut rng = Rng(0x2020_e009);
    for has_left in [false, true] {
        for has_top in [false, true] {
            for _ in 0..3000 {
                let left_pos = 1 + rng.below(8) as usize;
                // left[-1] is the top-left sample.
                let left_buf = area(&mut rng, left_pos + 16);
                let top_offset = rng.below(4) as usize;
                let top_buf = padded(&mut rng, top_offset, 16);
                let left: PredLeft<'_> = has_left.then_some((left_buf.as_slice(), left_pos));
                let top = has_top.then(|| &top_buf[top_offset..]);
                let dst = rng.bytes(PRED_SIZE_ENC as usize);
                let mut expected = dst.clone();
                let mut actual = dst.clone();
                let mut dispatched = dst;
                c::intra16_preds_c(&mut expected, left, top);
                neon::intra16_preds(&mut actual, left, top);
                c::vp8_enc_pred_luma16(&mut dispatched, left, top);
                assert_eq!(actual, expected, "left={has_left} top={has_top}");
                assert_eq!(dispatched, expected);
            }
        }
    }
}

/// The encoder passes the thresholds 0, 2 and 3 (`FLATNESS_LIMIT_*`) with 1,
/// 8 or 16 blocks. A negative threshold with no block is the one input where
/// the two forms differ: `IsFlat_C` answers 1 without looking, `IsFlat_NEON`
/// compares the threshold with a zero count.
#[test]
fn is_flat_matches_c() {
    let mut rng = Rng(0x2020_e00a);
    for num_blocks in 0..=16_i32 {
        for thresh in [0, 1, 2, 3, 5, 10, 16, 100, 240] {
            for _ in 0..200 {
                let len = 16 * num_blocks as usize;
                let levels: Vec<i16> = (0..len)
                    .map(|_| match rng.below(10) {
                        0 => i16::MIN,
                        1 => rng.i16s(1, 2047)[0],
                        _ => 0,
                    })
                    .collect();
                let expected = quant::is_flat_c(&levels, num_blocks, thresh);
                assert_eq!(neon::is_flat(&levels, num_blocks, thresh), expected);
                assert_eq!(quant::is_flat(&levels, num_blocks, thresh), expected);
            }
        }
    }
}
