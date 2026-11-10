//! Cases of the encoder DSP functions that `src/dsp/enc_neon.rs` serves
//! (WP20): the C translation of each dispatch function of `src/dsp/enc.rs`
//! and `IsFlat` of `src/dsp/quant.rs`, and with `simd` on 64-bit ARM the
//! NEON variant through the dispatch function, over the same seeded blocks.

use std::hint::black_box;

use super::{Case, Rng};
use webp::dsp::dsp::{BPS, VP8Histogram};
use webp::dsp::enc as c;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/dsp/enc_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::dsp::enc::vp8_collect_histogram as collect_histogram;
    pub use webp::dsp::enc::vp8_enc_pred_luma4 as intra4_preds;
    pub use webp::dsp::enc::vp8_enc_pred_luma16 as intra16_preds;
    pub use webp::dsp::enc::vp8_enc_quantize_block as quantize_block;
    pub use webp::dsp::enc::vp8_enc_quantize2_blocks as quantize2_blocks;
    pub use webp::dsp::enc::vp8f_transform as f_transform;
    pub use webp::dsp::enc::vp8f_transform_wht as f_transform_wht;
    pub use webp::dsp::enc::vp8i_transform as i_transform;
    pub use webp::dsp::enc::vp8sse4x4 as sse4x4;
    pub use webp::dsp::enc::vp8sse8x8 as sse8x8;
    pub use webp::dsp::enc::vp8sse16x8 as sse16x8;
    pub use webp::dsp::enc::vp8sse16x16 as sse16x16;
    pub use webp::dsp::enc::vp8t_disto4x4 as disto4x4;
    pub use webp::dsp::enc::vp8t_disto16x16 as disto16x16;
    pub use webp::dsp::quant::is_flat;
}
use webp::dsp::quant;
use webp::enc::vp8i_enc::{PRED_SIZE_ENC, VP8Matrix};

const AREA: usize = 16 * BPS as usize;

/// `kWeightY` of `quant_enc.c`.
const WEIGHT_Y: [u16; 16] = [38, 32, 20, 9, 32, 28, 17, 7, 20, 17, 10, 4, 9, 7, 4, 2];

/// A luma AC matrix as `ExpandMatrix` builds it for the step 36.
fn matrix() -> VP8Matrix {
    let mut m = VP8Matrix::default();
    let sharpening = [
        0, 30, 60, 90, 30, 60, 90, 90, 60, 90, 90, 90, 90, 90, 90, 90_u32,
    ];
    for (i, &sharpen) in sharpening.iter().enumerate() {
        m.q[i] = 36;
        m.iq[i] = ((1_u32 << 17) / 36) as u16;
        m.bias[i] = (if i == 0 { 96 } else { 110 }) << 9;
        m.zthresh[i] = ((1_u32 << 17) - 1 - m.bias[i]) / u32::from(m.iq[i]);
        m.sharpen[i] = ((sharpen * 36) >> 11) as u16;
    }
    m
}

/// Coefficients of transform size, half of them zero.
fn coefficients(rng: &mut Rng, count: usize) -> Vec<i16> {
    (0..count)
        .map(|_| {
            let v = rng.next_u64();
            if v & 1 == 0 {
                0
            } else {
                ((v >> 8) % 4096) as i16 - 2048
            }
        })
        .collect()
}

/// The C case and, with `simd` on 64-bit ARM, the NEON case of one function.
fn push(
    cases: &mut Vec<Case>,
    module: &'static str,
    function: &'static str,
    c_run: Box<dyn FnMut()>,
    #[cfg_attr(
        not(all(feature = "simd", target_arch = "aarch64")),
        expect(
            unused_variables,
            reason = "the NEON case exists only with simd on 64-bit ARM"
        )
    )]
    neon_run: Box<dyn FnMut()>,
) {
    cases.push(Case {
        module,
        function,
        variant: "C",
        run: c_run,
    });
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.push(Case {
        module,
        function,
        variant: "NEON",
        run: neon_run,
    });
}

/// Builds the C and NEON variants of one function from a constructor that
/// receives the function to call.
macro_rules! both {
    ($cases:expr, $module:expr, $name:expr, $c:expr, $neon:path, $make:expr) => {{
        #[cfg(all(feature = "simd", target_arch = "aarch64"))]
        let neon_run = $make($neon);
        #[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
        let neon_run = $make($c);
        push($cases, $module, $name, $make($c), neon_run);
    }};
}

/// The registered cases.
#[must_use]
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_e0b1);
    let a = rng.bytes(AREA);
    let b = rng.bytes(AREA);
    let input = coefficients(&mut rng, 256);
    let mtx = matrix();
    let top = rng.bytes(40);
    let left = rng.bytes(17);
    let mut cases = Vec::new();

    both!(
        &mut cases,
        "enc",
        "VP8ITransform",
        c::i_transform_c,
        neon::i_transform,
        {
            let (a, b, input) = (a.clone(), b.clone(), input.clone());
            move |f: fn(&[u8], &[i16], &mut [u8], bool)| -> Box<dyn FnMut()> {
                let (a, input, mut dst) = (a.clone(), input.clone(), b.clone());
                Box::new(move || f(black_box(&a), black_box(&input), black_box(&mut dst), true))
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8FTransform",
        c::f_transform_c,
        neon::f_transform,
        {
            let (a, b) = (a.clone(), b.clone());
            move |f: fn(&[u8], &[u8], &mut [i16])| -> Box<dyn FnMut()> {
                let (a, b) = (a.clone(), b.clone());
                let mut out = [0_i16; 16];
                Box::new(move || f(black_box(&a), black_box(&b), black_box(&mut out)))
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8FTransformWHT",
        c::f_transform_wht_c,
        neon::f_transform_wht,
        {
            let input = input.clone();
            move |f: fn(&[i16], &mut [i16])| -> Box<dyn FnMut()> {
                let input = input.clone();
                let mut out = [0_i16; 16];
                Box::new(move || f(black_box(&input), black_box(&mut out)))
            }
        }
    );
    let sse = |f: fn(&[u8], &[u8]) -> i32| -> Box<dyn FnMut()> {
        let (a, b) = (a.clone(), b.clone());
        Box::new(move || {
            black_box(f(black_box(&a), black_box(&b)));
        })
    };
    both!(
        &mut cases,
        "enc",
        "VP8SSE16x16",
        c::sse16x16_c,
        neon::sse16x16,
        sse
    );
    both!(
        &mut cases,
        "enc",
        "VP8SSE16x8",
        c::sse16x8_c,
        neon::sse16x8,
        sse
    );
    both!(
        &mut cases,
        "enc",
        "VP8SSE8x8",
        c::sse8x8_c,
        neon::sse8x8,
        sse
    );
    both!(
        &mut cases,
        "enc",
        "VP8SSE4x4",
        c::sse4x4_c,
        neon::sse4x4,
        sse
    );
    both!(
        &mut cases,
        "enc",
        "VP8TDisto4x4",
        c::disto4x4_c,
        neon::disto4x4,
        {
            let (a, b) = (a.clone(), b.clone());
            move |f: fn(&[u8], &[u8], &[u16; 16]) -> i32| -> Box<dyn FnMut()> {
                let (a, b) = (a.clone(), b.clone());
                Box::new(move || {
                    black_box(f(black_box(&a), black_box(&b), black_box(&WEIGHT_Y)));
                })
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8TDisto16x16",
        c::disto16x16_c,
        neon::disto16x16,
        {
            let (a, b) = (a.clone(), b.clone());
            move |f: fn(&[u8], &[u8], &[u16; 16]) -> i32| -> Box<dyn FnMut()> {
                let (a, b) = (a.clone(), b.clone());
                Box::new(move || {
                    black_box(f(black_box(&a), black_box(&b), black_box(&WEIGHT_Y)));
                })
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8CollectHistogram",
        c::collect_histogram_c,
        neon::collect_histogram,
        {
            let (a, b) = (a.clone(), b.clone());
            move |f: fn(&[u8], &[u8], i32, i32, &mut VP8Histogram)| -> Box<dyn FnMut()> {
                let (a, b) = (a.clone(), b.clone());
                let mut histo = VP8Histogram::default();
                Box::new(move || f(black_box(&a), black_box(&b), 0, 16, black_box(&mut histo)))
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8EncQuantizeBlock",
        c::quantize_block_c,
        neon::quantize_block,
        {
            let input = input.clone();
            move |f: fn(&mut [i16], &mut [i16], &VP8Matrix) -> i32| -> Box<dyn FnMut()> {
                let input: [i16; 16] = input[..16].try_into().unwrap();
                let mut out = [0_i16; 16];
                Box::new(move || {
                    let mut block = black_box(input);
                    black_box(f(&mut block, black_box(&mut out), black_box(&mtx)));
                })
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8EncQuantize2Blocks",
        c::quantize2_blocks_c,
        neon::quantize2_blocks,
        {
            let input = input.clone();
            move |f: fn(&mut [i16], &mut [i16], &VP8Matrix) -> i32| -> Box<dyn FnMut()> {
                let input: [i16; 32] = input[..32].try_into().unwrap();
                let mut out = [0_i16; 32];
                Box::new(move || {
                    let mut block = black_box(input);
                    black_box(f(&mut block, black_box(&mut out), black_box(&mtx)));
                })
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8EncPredLuma4",
        c::intra4_preds_c,
        neon::intra4_preds,
        {
            let top = top.clone();
            move |f: fn(&mut [u8], &[u8], usize)| -> Box<dyn FnMut()> {
                let top = top.clone();
                let mut dst = vec![0_u8; PRED_SIZE_ENC as usize];
                Box::new(move || f(black_box(&mut dst), black_box(&top), 17))
            }
        }
    );
    both!(
        &mut cases,
        "enc",
        "VP8EncPredLuma16",
        c::intra16_preds_c,
        neon::intra16_preds,
        {
            let (top, left) = (top.clone(), left.clone());
            move |f: fn(&mut [u8], c::PredLeft<'_>, Option<&[u8]>)| -> Box<dyn FnMut()> {
                let (top, left) = (top.clone(), left.clone());
                let mut dst = vec![0_u8; PRED_SIZE_ENC as usize];
                Box::new(move || {
                    f(
                        black_box(&mut dst),
                        Some((black_box(&left), 1)),
                        Some(black_box(&top)),
                    );
                })
            }
        }
    );
    both!(
        &mut cases,
        "quant",
        "IsFlat",
        quant::is_flat_c,
        neon::is_flat,
        {
            let mut levels = input.clone();
            for (i, level) in levels.iter_mut().enumerate() {
                if i % 16 > 2 {
                    *level = 0;
                }
            }
            move |f: fn(&[i16], i32, i32) -> bool| -> Box<dyn FnMut()> {
                let levels = levels.clone();
                Box::new(move || {
                    black_box(f(black_box(&levels), 16, 40));
                })
            }
        }
    );
    cases
}
