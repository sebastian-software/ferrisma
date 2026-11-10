//! WP11 differential test: the image transforms of the lossless encoder,
//! `VP8LResidualImage`, `VP8LColorSpaceTransform` and `VP8LOptimizeSampling`
//! (`src/enc/predictor_enc.c`) and `VP8ApplyNearLossless`
//! (`src/enc/near_lossless_enc.c`). The C probe
//! `tools/oracle/probes/wp11/predictor.c` and the Rust translation transform
//! the same seeded images: the residuals, the predictor or color-transform
//! image, the chosen bits, the return value, the error code and the progress
//! are compared. The residual images cover every pair of minimum and maximum
//! bits from 2 to 6 (so up to four subsampling levels of the zigzag search),
//! low effort, near-lossless qualities from 0 to 100, `exact`, the
//! subtract-green flag, transparent pixels and the right and bottom edges of
//! partial tiles. The probe fills the scratch rows with 0xa5 bytes and the
//! Rust code with zeros, so equal results also show that no stale scratch
//! value is read.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::dsp::lossless_common::vp8l_sub_sample_size;
use webp::enc::near_lossless_enc::vp8_apply_near_lossless;
use webp::enc::predictor_enc::{
    vp8l_color_space_transform, vp8l_optimize_sampling, vp8l_residual_image,
};
use webp::webp::encode::{WebPPicture, webp_picture_init};
use webp::webp::types::{PlaneOffset, PlaneStorage};
use wp11_images::{Kind, Rng, image};

fn hex32(values: &[u32]) -> String {
    if values.is_empty() {
        return "-".to_owned();
    }
    values.iter().map(|value| format!("{value:08x}")).collect()
}

fn sub(size: i32, bits: i32) -> usize {
    vp8l_sub_sample_size(size as u32, bits as u32) as usize
}

fn picture() -> WebPPicture<'static> {
    let mut picture = WebPPicture::default();
    assert!(webp_picture_init(&mut picture));
    picture
}

#[test]
fn image_transforms_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("image_transforms_match_the_c_functions") else {
        return;
    };
    let mut rng = Rng(0x9e1c_7012);
    let mut script = String::new();
    let mut expected = Fields::new();
    let sizes: [(i32, i32); 10] = [
        (1, 1),
        (2, 3),
        (5, 4),
        (9, 9),
        (16, 16),
        (17, 23),
        (33, 31),
        (64, 40),
        (70, 66),
        (130, 20),
    ];

    for &(width, height) in &sizes {
        for &kind in &Kind::ALL {
            let mut argb = image(
                &mut rng,
                kind,
                width as usize,
                height as usize,
                width as usize,
            );
            // VP8LResidualImage.
            let max_bits = 2 + rng.below(5) as i32;
            let min_bits = 2 + rng.below((max_bits - 1) as u64) as i32;
            let low_effort = rng.below(5) == 0;
            let near_lossless = [100, 100, 99, 80, 60, 40, 20, 0][rng.below(8) as usize];
            let exact = rng.below(2) == 0;
            let used_subtract_green = rng.below(2) == 0;
            writeln!(
                script,
                "residual {width} {height} {min_bits} {max_bits} {} {near_lossless} {} {} {}",
                i32::from(low_effort),
                i32::from(exact),
                i32::from(used_subtract_green),
                hex32(&argb)
            )
            .unwrap();
            let scratch_size = (width as usize + 1) * 2 + (width as usize * 2).div_ceil(4);
            let mut scratch = vec![0_u32; scratch_size];
            let mut transform = vec![0_u32; sub(width, 2) * sub(height, 2)];
            let mut pic = picture();
            let mut percent = 0;
            let mut best_bits = -1;
            let ret = vp8l_residual_image(
                width,
                height,
                min_bits,
                max_bits,
                low_effort,
                &mut argb,
                &mut scratch,
                &mut transform,
                near_lossless,
                exact,
                used_subtract_green,
                &mut pic,
                40,
                &mut percent,
                &mut best_bits,
            );
            let count = sub(width, best_bits) * sub(height, best_bits);
            expected = expected
                .with("ret", i32::from(ret))
                .with("error", pic.error_code.0)
                .with("percent", percent)
                .with("best_bits", best_bits)
                .with("argb", hex32(&argb))
                .with("image", hex32(&transform[..count]));

            // VP8LColorSpaceTransform.
            let mut argb = image(
                &mut rng,
                kind,
                width as usize,
                height as usize,
                width as usize,
            );
            let bits = 2 + rng.below(5) as i32;
            let quality = [0, 24, 25, 50, 51, 75, 100][rng.below(7) as usize];
            writeln!(
                script,
                "cross {width} {height} {bits} {quality} {}",
                hex32(&argb)
            )
            .unwrap();
            let mut transform = vec![0_u32; sub(width, bits) * sub(height, bits)];
            let mut pic = picture();
            let mut percent = 0;
            let mut best_bits = -1;
            let ret = vp8l_color_space_transform(
                width,
                height,
                bits,
                quality,
                &mut argb,
                &mut transform,
                &mut pic,
                40,
                &mut percent,
                &mut best_bits,
            );
            let count = sub(width, best_bits) * sub(height, best_bits);
            expected = expected
                .with("ret", i32::from(ret.is_ok()))
                .with("error", pic.error_code.0)
                .with("percent", percent)
                .with("best_bits", best_bits)
                .with("argb", hex32(&argb))
                .with("image", hex32(&transform[..count]));

            // VP8ApplyNearLossless.
            let stride = width + rng.below(3) as i32;
            let mut source = image(
                &mut rng,
                kind,
                width as usize,
                height as usize,
                stride as usize,
            );
            let quality = rng.below(100) as i32;
            writeln!(
                script,
                "near {width} {height} {stride} {quality} {}",
                hex32(&source)
            )
            .unwrap();
            let mut dst = vec![0_u32; (width * height) as usize];
            let ret = {
                let mut pic = picture();
                pic.use_argb = 1;
                pic.width = width;
                pic.height = height;
                pic.argb = Some(PlaneOffset {
                    region: 0,
                    offset: 0,
                });
                pic.argb_stride = stride;
                pic.argb_storage = PlaneStorage::External(vec![source.as_mut_slice()]);
                vp8_apply_near_lossless(&pic, quality, &mut dst)
            };
            expected = expected
                .with("ret", i32::from(ret))
                .with("dst", hex32(&dst));
        }
    }

    // VP8LOptimizeSampling on images made of uniform squares, some broken.
    for _ in 0..150 {
        let full_width = 1 + rng.below(200) as i32;
        let full_height = 1 + rng.below(200) as i32;
        let bits = 2 + rng.below(4) as i32;
        let max_bits = bits + rng.below(5) as i32;
        let (width, height) = (sub(full_width, bits), sub(full_height, bits));
        let square = 1_usize << rng.below(4);
        let colors: Vec<u32> = (0..4).map(|_| rng.u32()).collect();
        let mut img = vec![0_u32; width * height];
        for y in 0..height {
            for x in 0..width {
                img[y * width + x] = colors[((x / square) + (y / square) * 3) % 4];
            }
        }
        if rng.below(3) == 0 && !img.is_empty() {
            let at = rng.below(img.len() as u64) as usize;
            img[at] ^= 1;
        }
        writeln!(
            script,
            "sampling {full_width} {full_height} {bits} {max_bits} {}",
            hex32(&img)
        )
        .unwrap();
        let mut best_bits = -1;
        vp8l_optimize_sampling(
            &mut img,
            full_width,
            full_height,
            bits,
            max_bits,
            &mut best_bits,
        );
        expected = expected
            .with("best_bits", best_bits)
            .with("image", hex32(&img));
    }

    let output = oracle
        .probe("wp11", "predictor")
        .run::<&str>(&[], script.as_bytes());
    let actual = output.success().fields();
    assert_eq!(actual.0.len(), expected.0.len(), "number of observations");
    for (index, (a, e)) in actual.0.iter().zip(&expected.0).enumerate() {
        assert!(
            a == e,
            "observation {index}: C {}={} Rust {}={}",
            a.0,
            a.1.chars().take(200).collect::<String>(),
            e.0,
            e.1.chars().take(200).collect::<String>()
        );
    }
}
