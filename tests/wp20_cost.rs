//! WP20 parity test: the NEON variants of `src/dsp/cost_neon.c` against the C
//! translations they replace (`src/dsp/cost.rs`), Rust against Rust: the
//! position of the last non-zero level and the residual cost, for the four
//! coefficient types, the three start contexts, the default and random
//! probabilities, and levels from empty blocks to full blocks of the
//! largest magnitude (`MAX_LEVEL`, 2047), as the encoder passes them. The
//! dispatch functions, which select the NEON variants by the query's `kNEON`
//! answer (`Features::query_neon`), are compared with the C functions too.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::dsp::cost::{
    get_residual_cost_c, set_residual_coeffs_c, vp8_get_residual_cost, vp8_set_residual_coeffs,
};
/// The NEON level of `src/dsp/cost_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::dsp::cost::vp8_get_residual_cost as get_residual_cost;
    pub use webp::dsp::cost::vp8_set_residual_coeffs as set_residual_coeffs;
}
use webp::enc::cost_enc::{vp8_calculate_level_costs, vp8_init_residual};
use webp::enc::picture_enc::webp_picture_alloc;
use webp::enc::vp8i_enc::VP8Encoder;
use webp::enc::webp_enc::init_vp8_encoder;
use webp::webp::encode::{WebPConfig, WebPPicture, webp_config_init, webp_picture_init};

/// An encoder from `InitVP8Encoder`, which calls `VP8DefaultProbas`.
fn encoder() -> Box<VP8Encoder> {
    let mut config = WebPConfig::default();
    assert!(webp_config_init(&mut config));
    let mut pic = WebPPicture::default();
    assert!(webp_picture_init(&mut pic));
    pic.width = 16;
    pic.height = 16;
    webp_picture_alloc(&mut pic).expect("the picture allocates");
    init_vp8_encoder(&config, &mut pic).expect("the encoder initializes")
}

/// The levels of one block: random as the encoder's, or an edge shape.
fn levels(rng: &mut Rng, first: usize) -> [i16; 16] {
    let mut coeffs = [0_i16; 16];
    match rng.below(8) {
        // Empty.
        0 => {}
        // Only the last position.
        1 => coeffs[15] = if rng.below(2) == 0 { 1 } else { -2047 },
        // Every position at the largest magnitude.
        2 => {
            for value in &mut coeffs[first..] {
                *value = if rng.below(2) == 0 { 2047 } else { -2047 };
            }
        }
        // Every position at 1 or 2 in magnitude, the context edges.
        3 => {
            for value in &mut coeffs[first..] {
                *value = [1, -1, 2, -2, 0][rng.below(5) as usize];
            }
        }
        _ => {
            let density = rng.below(17);
            for value in &mut coeffs[first..] {
                if rng.below(16) >= density {
                    continue;
                }
                let magnitude = match rng.below(10) {
                    0..=5 => 1 + rng.below(2),
                    6 | 7 => 3 + rng.below(8),
                    8 => 11 + rng.below(80),
                    _ => 67 + rng.below(1981),
                } as i16;
                *value = if rng.below(2) == 0 {
                    magnitude
                } else {
                    -magnitude
                };
            }
        }
    }
    coeffs
}

#[test]
fn residual_coefficients_and_costs_match_c() {
    let mut rng = Rng(0x2020_c001);
    let mut cases = 0;
    for random_probas in [false, true, true] {
        let mut enc = encoder();
        if random_probas {
            for value in enc
                .proba
                .coeffs
                .as_flattened_mut()
                .as_flattened_mut()
                .as_flattened_mut()
            {
                *value = rng.byte();
            }
        }
        vp8_calculate_level_costs(&mut enc.proba);
        for _ in 0..4000 {
            let coeff_type = rng.below(4) as i32;
            let first = i32::from(coeff_type == 0);
            let coeffs = levels(&mut rng, first as usize);
            let offset = rng.below(4) as usize;
            let mut buffer = vec![0x5a5a_i16; offset];
            buffer.extend_from_slice(&coeffs);
            let coeffs = &buffer[offset..];

            let mut res = vp8_init_residual(first, coeff_type, &mut enc.proba);
            set_residual_coeffs_c(coeffs, &mut res);
            let last_c = res.last;
            res.last = 77;
            res.coeffs = &[];
            neon::set_residual_coeffs(coeffs, &mut res);
            assert_eq!(res.last, last_c, "coeffs={coeffs:?}");
            assert!(std::ptr::eq(res.coeffs, coeffs));
            res.last = 77;
            vp8_set_residual_coeffs(coeffs, &mut res);
            assert_eq!(res.last, last_c, "dispatch coeffs={coeffs:?}");

            for ctx in 0..3 {
                let expected = get_residual_cost_c(ctx, &res);
                assert_eq!(neon::get_residual_cost(ctx, &res), expected, "ctx={ctx}");
                assert_eq!(
                    vp8_get_residual_cost(ctx, &res),
                    expected,
                    "dispatch ctx={ctx}"
                );
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 3 * 4000 * 3);
}
