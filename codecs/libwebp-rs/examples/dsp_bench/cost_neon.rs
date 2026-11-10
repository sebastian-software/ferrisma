//! Cases of the residual cost functions that `src/dsp/cost_neon.rs` serves
//! (WP20): the C translations of `src/dsp/cost.rs` and, with `simd` on
//! 64-bit ARM, the NEON variants through the dispatch functions, over the default probabilities and a
//! block of typical levels.

use std::hint::black_box;

use super::{Case, Rng};
use webp::dsp::cost::{get_residual_cost_c, set_residual_coeffs_c};
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/dsp/cost_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub use webp::dsp::cost::vp8_get_residual_cost as get_residual_cost;
    pub use webp::dsp::cost::vp8_set_residual_coeffs as set_residual_coeffs;
}
use webp::enc::cost_enc::{VP8Residual, vp8_calculate_level_costs, vp8_init_residual};
use webp::enc::picture_enc::webp_picture_alloc;
use webp::enc::vp8i_enc::VP8Encoder;
use webp::enc::webp_enc::init_vp8_encoder;
use webp::webp::encode::{WebPConfig, WebPPicture, webp_config_init, webp_picture_init};

/// An encoder with the default probabilities and their level costs.
fn encoder() -> Box<VP8Encoder> {
    let mut config = WebPConfig::default();
    assert!(webp_config_init(&mut config));
    let mut pic = WebPPicture::default();
    assert!(webp_picture_init(&mut pic));
    pic.width = 16;
    pic.height = 16;
    webp_picture_alloc(&mut pic).expect("the picture allocates");
    let mut enc = init_vp8_encoder(&config, &mut pic).expect("the encoder initializes");
    vp8_calculate_level_costs(&mut enc.proba);
    enc
}

/// The registered cases.
#[must_use]
pub fn cases() -> Vec<Case> {
    let mut rng = Rng::new(0x2020_c0b1);
    // Levels as the encoder produces them: mostly small, the tail zero.
    let mut coeffs = [0_i16; 16];
    for (n, value) in coeffs.iter_mut().enumerate().take(11) {
        let v = rng.next_u64();
        let magnitude = if n < 3 {
            (v % 40) as i16
        } else {
            (v % 3) as i16
        };
        *value = if v & (1 << 40) == 0 {
            magnitude
        } else {
            -magnitude
        };
    }
    coeffs[10] = 1;

    let mut cases = Vec::new();
    let set_c: for<'a> fn(&'a [i16], &mut VP8Residual<'a>) = set_residual_coeffs_c;
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    let set_neon: for<'a> fn(&'a [i16], &mut VP8Residual<'a>) = neon::set_residual_coeffs;
    #[cfg_attr(
        not(all(feature = "simd", target_arch = "aarch64")),
        expect(unused_mut, reason = "only the NEON variant is pushed")
    )]
    let mut variants = vec![("C", set_c)];
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    variants.push(("NEON", set_neon));
    for (variant, set) in variants {
        let mut enc = encoder();
        cases.push(Case {
            module: "cost",
            function: "VP8SetResidualCoeffs",
            variant,
            run: Box::new(move || {
                let mut res = vp8_init_residual(0, 3, &mut enc.proba);
                set(black_box(&coeffs), &mut res);
                black_box(res.last);
            }),
        });
    }

    let get_c: fn(i32, &VP8Residual<'_>) -> i32 = get_residual_cost_c;
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    let get_neon: fn(i32, &VP8Residual<'_>) -> i32 = neon::get_residual_cost;
    #[cfg_attr(
        not(all(feature = "simd", target_arch = "aarch64")),
        expect(unused_mut, reason = "only the NEON variant is pushed")
    )]
    let mut variants = vec![("C", get_c)];
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    variants.push(("NEON", get_neon));
    for (variant, get) in variants {
        let mut enc = encoder();
        cases.push(Case {
            module: "cost",
            function: "VP8GetResidualCost",
            variant,
            run: Box::new(move || {
                let mut res = vp8_init_residual(0, 3, &mut enc.proba);
                set_residual_coeffs_c(&coeffs, &mut res);
                black_box(get(black_box(1), &res));
            }),
        });
    }
    cases
}
