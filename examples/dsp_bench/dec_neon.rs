//! Benchmark cases of the decoder DSP of `src/dsp/dec.rs`: the C translation
//! of each function that `VP8DspInitNEON` replaces and, with the `simd`
//! feature on 64-bit ARM, its NEON variant from `src/dsp/dec_neon.rs` (WP20).
//! The filters and predictors run in place on their buffers, as the decoder
//! calls them.

use super::{Case, Rng};
use webp::dsp::dec as c;
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
/// The NEON level of `src/dsp/dec_neon.rs`, reached through the dispatch
/// functions, which select its crate-private entry points with `simd` on
/// 64-bit ARM (the residual costs while the `VP8GetCPUInfo` query answers
/// `kNEON`, as it does by default); named as the entry points.
mod neon {
    pub fn dc16_no_left(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(5, buf, dst);
    }
    pub fn dc16_no_top(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(4, buf, dst);
    }
    pub fn dc16_no_top_left(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(6, buf, dst);
    }
    pub fn dc16_top_left(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(0, buf, dst);
    }
    pub fn dc4(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma4(0, buf, dst);
    }
    pub fn dc8uv(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(0, buf, dst);
    }
    pub fn dc8uv_no_left(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(5, buf, dst);
    }
    pub fn dc8uv_no_top(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(4, buf, dst);
    }
    pub fn dc8uv_no_top_left(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(6, buf, dst);
    }
    pub use webp::dsp::dec::vp8h_filter8 as h_filter8;
    pub use webp::dsp::dec::vp8h_filter8i as h_filter8i;
    pub use webp::dsp::dec::vp8h_filter16 as h_filter16;
    pub use webp::dsp::dec::vp8h_filter16i as h_filter16i;
    pub fn he16(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(3, buf, dst);
    }
    pub fn he8uv(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(3, buf, dst);
    }
    pub fn ld4(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma4(6, buf, dst);
    }
    pub fn rd4(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma4(4, buf, dst);
    }
    pub use webp::dsp::dec::vp8_simple_h_filter16 as simple_h_filter16;
    pub use webp::dsp::dec::vp8_simple_h_filter16i as simple_h_filter16i;
    pub use webp::dsp::dec::vp8_simple_v_filter16 as simple_v_filter16;
    pub use webp::dsp::dec::vp8_simple_v_filter16i as simple_v_filter16i;
    pub fn tm16(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(1, buf, dst);
    }
    pub fn tm4(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma4(1, buf, dst);
    }
    pub fn tm8uv(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(1, buf, dst);
    }
    pub use webp::dsp::dec::vp8_transform as transform_two;
    pub use webp::dsp::dec::vp8_transform_ac3 as transform_ac3;
    pub use webp::dsp::dec::vp8_transform_dc as transform_dc;
    pub use webp::dsp::dec::vp8_transform_wht as transform_wht;
    pub use webp::dsp::dec::vp8v_filter8 as v_filter8;
    pub use webp::dsp::dec::vp8v_filter8i as v_filter8i;
    pub use webp::dsp::dec::vp8v_filter16 as v_filter16;
    pub use webp::dsp::dec::vp8v_filter16i as v_filter16i;
    pub fn ve16(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma16(2, buf, dst);
    }
    pub fn ve4(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_luma4(2, buf, dst);
    }
    pub fn ve8uv(buf: &mut [u8], dst: usize) {
        webp::dsp::dec::vp8_pred_chroma8(2, buf, dst);
    }
}

const BPS: usize = 32;

type Transform = fn(&[i16], &mut [u8]);
type Pred = fn(&mut [u8], usize);
type Simple = fn(&mut [u8], usize, i32, i32);
type Luma = fn(&mut [u8], usize, i32, i32, i32, i32);
type Chroma = fn(&mut [u8], usize, &mut [u8], usize, i32, i32, i32, i32);

/// Coefficients as a decoded block holds them: mostly small, some zero.
fn coefficients(rng: &mut Rng, count: usize) -> Vec<i16> {
    rng.bytes(count)
        .into_iter()
        .map(|b| match b % 4 {
            0 => 0,
            _ => (i16::from(b) - 128) * 8,
        })
        .collect()
}

/// A plane with small steps, so that the filters filter most samples.
fn plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    rng.bytes(size).into_iter().map(|b| 100 + b % 24).collect()
}

fn case(function: &'static str, variant: &'static str, run: Box<dyn FnMut()>) -> Case {
    Case {
        module: "dec",
        function,
        variant,
        run,
    }
}

/// The C case and, with NEON, the NEON case of a function of the type `F`,
/// each with its own copy of the inputs that `make` builds.
fn pair<F: Copy + 'static>(
    cases: &mut Vec<Case>,
    function: &'static str,
    c_fn: F,
    neon_fn: F,
    make: impl Fn(F) -> Box<dyn FnMut()>,
) {
    cases.push(case(function, "C", make(c_fn)));
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    cases.push(case(function, "NEON", make(neon_fn)));
    #[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
    let _ = neon_fn;
}

/// The cases of `src/dsp/dec.rs`.
#[must_use]
#[expect(
    clippy::too_many_lines,
    reason = "one entry per dispatch function and table entry"
)]
pub fn cases() -> Vec<Case> {
    #[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
    mod neon {
        pub use webp::dsp::dec::{
            dc4_c as dc4, dc8uv_c as dc8uv, dc8uv_no_left_c as dc8uv_no_left,
            dc8uv_no_top_c as dc8uv_no_top, dc8uv_no_top_left_c as dc8uv_no_top_left,
            dc16_c as dc16_top_left, dc16_no_left_c as dc16_no_left, dc16_no_top_c as dc16_no_top,
            dc16_no_top_left_c as dc16_no_top_left, h_filter8_c as h_filter8,
            h_filter8i_c as h_filter8i, h_filter16_c as h_filter16, h_filter16i_c as h_filter16i,
            he8uv_c as he8uv, he16_c as he16, ld4_c as ld4, rd4_c as rd4,
            simple_h_filter16_c as simple_h_filter16, simple_h_filter16i_c as simple_h_filter16i,
            simple_v_filter16_c as simple_v_filter16, simple_v_filter16i_c as simple_v_filter16i,
            tm4_c as tm4, tm8uv_c as tm8uv, tm16_c as tm16, transform_ac3_c as transform_ac3,
            transform_dc_c as transform_dc, transform_two_c as transform_two,
            transform_wht_c as transform_wht, v_filter8_c as v_filter8, v_filter8i_c as v_filter8i,
            v_filter16_c as v_filter16, v_filter16i_c as v_filter16i, ve4_c as ve4,
            ve8uv_c as ve8uv, ve16_c as ve16,
        };
    }

    let mut cases = Vec::new();
    let mut rng = Rng::new(0x5750_3230_4445);

    // Transforms, added in place to a block as the decoder adds them; the
    // decoder calls `VP8Transform` for pairs of blocks.
    let input = coefficients(&mut rng, 32);
    let dst = rng.bytes(4 * BPS);
    {
        type Two = fn(&[i16], &mut [u8], bool);
        let (input, dst) = (input.clone(), dst.clone());
        pair(
            &mut cases,
            "VP8Transform",
            c::transform_two_c as Two,
            neon::transform_two as Two,
            move |f: Two| {
                let (input, mut dst) = (input.clone(), dst.clone());
                Box::new(move || {
                    f(&input, &mut dst, true);
                    std::hint::black_box(&dst);
                })
            },
        );
    }
    let transforms: [(&'static str, Transform, Transform); 2] = [
        ("VP8TransformAC3", c::transform_ac3_c, neon::transform_ac3),
        ("VP8TransformDC", c::transform_dc_c, neon::transform_dc),
    ];
    for (name, c_fn, neon_fn) in transforms {
        let (input, dst) = (input.clone(), dst.clone());
        pair(&mut cases, name, c_fn, neon_fn, move |f: Transform| {
            let (input, mut dst) = (input.clone(), dst.clone());
            Box::new(move || {
                f(&input, &mut dst);
                std::hint::black_box(&dst);
            })
        });
    }
    {
        let wht = coefficients(&mut rng, 16);
        type Wht = fn(&[i16], &mut [i16]);
        pair(
            &mut cases,
            "VP8TransformWHT",
            c::transform_wht_c as Wht,
            neon::transform_wht as Wht,
            move |f: Wht| {
                let wht = wht.clone();
                let mut out = vec![0i16; 256];
                Box::new(move || {
                    f(&wht, &mut out);
                    std::hint::black_box(&out);
                })
            },
        );
    }

    // Loop filters on a plane of stride 64 at an inner macroblock edge.
    let stride = 64_usize;
    let luma = plane(&mut rng, stride * 24);
    let pos = 4 * stride + 8;
    let simple: [(&'static str, Simple, Simple); 4] = [
        (
            "VP8SimpleVFilter16",
            c::simple_v_filter16_c,
            neon::simple_v_filter16,
        ),
        (
            "VP8SimpleHFilter16",
            c::simple_h_filter16_c,
            neon::simple_h_filter16,
        ),
        (
            "VP8SimpleVFilter16i",
            c::simple_v_filter16i_c,
            neon::simple_v_filter16i,
        ),
        (
            "VP8SimpleHFilter16i",
            c::simple_h_filter16i_c,
            neon::simple_h_filter16i,
        ),
    ];
    for (name, c_fn, neon_fn) in simple {
        let luma = luma.clone();
        pair(&mut cases, name, c_fn, neon_fn, move |f: Simple| {
            let mut buf = luma.clone();
            Box::new(move || {
                f(&mut buf, pos, stride as i32, 60);
                std::hint::black_box(&buf);
            })
        });
    }
    let complex: [(&'static str, Luma, Luma); 4] = [
        ("VP8VFilter16", c::v_filter16_c, neon::v_filter16),
        ("VP8HFilter16", c::h_filter16_c, neon::h_filter16),
        ("VP8VFilter16i", c::v_filter16i_c, neon::v_filter16i),
        ("VP8HFilter16i", c::h_filter16i_c, neon::h_filter16i),
    ];
    for (name, c_fn, neon_fn) in complex {
        let luma = luma.clone();
        pair(&mut cases, name, c_fn, neon_fn, move |f: Luma| {
            let mut buf = luma.clone();
            Box::new(move || {
                f(&mut buf, pos, stride as i32, 60, 20, 1);
                std::hint::black_box(&buf);
            })
        });
    }
    let chroma: [(&'static str, Chroma, Chroma); 4] = [
        ("VP8VFilter8", c::v_filter8_c, neon::v_filter8),
        ("VP8HFilter8", c::h_filter8_c, neon::h_filter8),
        ("VP8VFilter8i", c::v_filter8i_c, neon::v_filter8i),
        ("VP8HFilter8i", c::h_filter8i_c, neon::h_filter8i),
    ];
    let u_plane = plane(&mut rng, stride * 16);
    let v_plane = plane(&mut rng, stride * 16);
    for (name, c_fn, neon_fn) in chroma {
        let (u_plane, v_plane) = (u_plane.clone(), v_plane.clone());
        pair(&mut cases, name, c_fn, neon_fn, move |f: Chroma| {
            let (mut u, mut v) = (u_plane.clone(), v_plane.clone());
            Box::new(move || {
                f(&mut u, pos, &mut v, pos, stride as i32, 60, 20, 1);
                std::hint::black_box((&u, &v));
            })
        });
    }

    // Predictors in a BPS-strided work buffer, as the decoder's `yuv_b`.
    let work = rng.bytes(BPS * 18);
    let dst = BPS + 8;
    let predictors: [(&'static str, Pred, Pred); 19] = [
        ("VP8PredLuma4[0]", c::dc4_c, neon::dc4),
        ("VP8PredLuma4[1]", c::tm4_c, neon::tm4),
        ("VP8PredLuma4[2]", c::ve4_c, neon::ve4),
        ("VP8PredLuma4[4]", c::rd4_c, neon::rd4),
        ("VP8PredLuma4[6]", c::ld4_c, neon::ld4),
        ("VP8PredLuma16[0]", c::dc16_c, neon::dc16_top_left),
        ("VP8PredLuma16[1]", c::tm16_c, neon::tm16),
        ("VP8PredLuma16[2]", c::ve16_c, neon::ve16),
        ("VP8PredLuma16[3]", c::he16_c, neon::he16),
        ("VP8PredLuma16[4]", c::dc16_no_top_c, neon::dc16_no_top),
        ("VP8PredLuma16[5]", c::dc16_no_left_c, neon::dc16_no_left),
        (
            "VP8PredLuma16[6]",
            c::dc16_no_top_left_c,
            neon::dc16_no_top_left,
        ),
        ("VP8PredChroma8[0]", c::dc8uv_c, neon::dc8uv),
        ("VP8PredChroma8[1]", c::tm8uv_c, neon::tm8uv),
        ("VP8PredChroma8[2]", c::ve8uv_c, neon::ve8uv),
        ("VP8PredChroma8[3]", c::he8uv_c, neon::he8uv),
        ("VP8PredChroma8[4]", c::dc8uv_no_top_c, neon::dc8uv_no_top),
        ("VP8PredChroma8[5]", c::dc8uv_no_left_c, neon::dc8uv_no_left),
        (
            "VP8PredChroma8[6]",
            c::dc8uv_no_top_left_c,
            neon::dc8uv_no_top_left,
        ),
    ];
    for (name, c_fn, neon_fn) in predictors {
        let work = work.clone();
        pair(&mut cases, name, c_fn, neon_fn, move |f: Pred| {
            let mut buf = work.clone();
            Box::new(move || {
                f(&mut buf, dst);
                std::hint::black_box(&buf);
            })
        });
    }
    cases
}
