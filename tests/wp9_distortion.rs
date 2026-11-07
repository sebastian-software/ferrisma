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

use oracle::{Fields, Oracle};
use webp::enc::picture_psnr_enc::webp_plane_distortion;

/// SplitMix64, a small deterministic generator for the cases.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

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
