//! WP9 differential test: `WebPPlaneDistortion` (`src/enc/picture_psnr_enc.c`) for the
//! three metrics, PSNR (0), SSIM (1) and the local-minimum measure (2). The probe
//! `tools/oracle/probes/wp9/distortion.c` runs the C function and prints the return
//! value and the bits of both floats. The test compares the bits, so the floating
//! point operations must match in order and rounding, not only in value.
//!
//! The cases are planes of 1 to 17 samples a side, unpacked (`x_step` 1) and packed
//! ARGB-style (`x_step` 4), with padded strides, identical planes, and strides too
//! short for a row, which fail without a result.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp9_pictures.rs"]
mod pictures;

use pictures::{Case, Rng, hex, with_picture};

use oracle::{Fields, Oracle};
use webp::enc::picture_psnr_enc::{webp_picture_distortion, webp_plane_distortion};

#[test]
fn plane_distortion_matches_the_c_metrics_bit_for_bit() {
    let Some(oracle) = Oracle::locate("plane_distortion_matches_the_c_metrics_bit_for_bit") else {
        return;
    };
    let probe = oracle.probe("wp9", "distortion");

    let mut rng = Rng(0x5eed_0050);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut succeeded = 0;
    let mut nonzero = 0;
    for kind in 0..3_i32 {
        for x_step in [1_usize, 4] {
            for (width, height) in [(1_usize, 1_usize), (2, 3), (5, 4), (8, 8), (9, 7), (17, 5)] {
                for pad in [0_usize, 3] {
                    // The short stride holds one byte less than a row: the call fails.
                    for short in [false, true] {
                        let stride = if short {
                            x_step * width - 1
                        } else {
                            x_step * width + pad
                        };
                        if stride == 0 {
                            continue;
                        }
                        let src: Vec<u8> = (0..stride * height).map(|_| rng.byte()).collect();
                        // The reference is the source with small changes, or the same plane.
                        let reference: Vec<u8> = if rng.next_u64().is_multiple_of(3) {
                            src.clone()
                        } else {
                            src.iter()
                                .map(|sample| sample.wrapping_add(rng.byte() % 9))
                                .collect()
                        };
                        stdin.push_str(&format!(
                            "plane {kind} {width} {height} {x_step} {stride} {stride} {} {}\n",
                            hex(&src),
                            hex(&reference)
                        ));
                        let mut distortion = -1.0_f32;
                        let mut result = -1.0_f32;
                        let ok = webp_plane_distortion(
                            &src,
                            stride,
                            &reference,
                            stride,
                            i32::try_from(width).unwrap(),
                            i32::try_from(height).unwrap(),
                            x_step,
                            kind,
                            &mut distortion,
                            &mut result,
                        );
                        succeeded += usize::from(ok);
                        nonzero += usize::from(ok && result.to_bits() != 0x42c6_0000);
                        expected = expected
                            .with("ret", i32::from(ok))
                            .with("distortion", format!("{:08x}", distortion.to_bits()))
                            .with("result", format!("{:08x}", result.to_bits()));
                    }
                }
            }
        }
    }
    assert!(succeeded > 0, "no metric succeeded");
    assert!(nonzero > 0, "every metric result was the maximum");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn picture_distortion_matches_the_c_distortion() {
    let Some(oracle) = Oracle::locate("picture_distortion_matches_the_c_distortion") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_distortion");

    let mut rng = Rng(0x5eed_0051);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut succeeded = 0;
    let mut refused = 0;
    for index in 0..48_u32 {
        let kind = (index % 3) as i32;
        let a_argb = i32::from(index % 2 == 0);
        let b_argb = i32::from(index % 4 < 2);
        let a_colorspace = if index % 5 == 0 { 0 } else { 4 };
        let b_colorspace = if index % 3 == 0 { 0 } else { 4 };
        let mut a = Case::random(&mut rng, a_argb, a_colorspace);
        let (width, height) = (a.width, a.height);
        // Every eighth pair has different widths, and the call refuses it.
        let b_width = if index % 8 == 7 { width + 1 } else { width };
        let mut b = Case::with_size(&mut rng, b_argb, b_colorspace, b_width, height);
        stdin.push_str(&format!(
            "distortion {kind} {} {} {} {}\n",
            a.command_fields(),
            a.plane_arguments(),
            b.command_fields(),
            b.plane_arguments()
        ));
        let mut results = [-1.0_f32; 5];
        let ok = with_picture(&mut a, |picture_a| {
            with_picture(&mut b, |picture_b| {
                webp_picture_distortion(picture_a, picture_b, kind, &mut results)
            })
        });
        if ok {
            succeeded += 1;
        } else {
            refused += 1;
        }
        let mut block = Fields::new().with("ret", i32::from(ok));
        for (channel, value) in results.iter().enumerate() {
            block = block.with(
                &format!("result{channel}"),
                format!("{:08x}", value.to_bits()),
            );
        }
        expected.0.extend(block.0);
    }
    // The cases must include pictures that measure and pairs that the call refuses.
    assert!(succeeded > 0, "no picture pair was measured");
    assert!(refused > 0, "no picture pair was refused");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}
