// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_external_image::{
    convert_float_channels_to_buffer, convert_float_to_buffer, float_to_f16,
};
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{ImageMetadata, Orientation};
use jxl::jxl::types::JxlEndianness;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("fe-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "fe-kernel" {
        let n = number::<usize>(args);
        let input: Vec<_> = (0..n).map(|_| f32::from_bits(number(args))).collect();
        let mut output = vec![0xdeadu16; n + 5];
        float_to_f16(&input, &mut output, n);
        let mut out = String::new();
        for v in output {
            write!(out, "{v} ").unwrap();
        }
        return out;
    }
    if kind == "fe-row" {
        let n = number::<usize>(args);
        let bits = number(args);
        let endian = JxlEndianness(number(args));
        let w = number(args);
        let stride = number(args);
        let size = number(args);
        let mut planes = Vec::new();
        for _ in 0..n {
            let mut plane = Plane::create(w, 1).unwrap();
            for x in 0..w {
                plane.row_mut(0)[x] = f32::from_bits(number(args));
            }
            planes.push(plane);
        }
        let channels: Vec<_> = planes.iter().map(Some).collect();
        let mut output = vec![0xa7; 1024];
        let status = convert_float_channels_to_buffer(
            &channels,
            bits,
            endian,
            stride,
            &mut output,
            size,
            Orientation::IDENTITY,
        );
        return format!(" S {} {}", status_code(&status), hex(&output));
    }
    let w = number(args);
    let h = number(args);
    let n = number::<usize>(args);
    let missing = number::<u32>(args);
    let bits = number(args);
    let orientation = Orientation(number(args));
    let endian = JxlEndianness(number(args));
    let stride = number(args);
    let out_size = number(args);
    let mode = number(args);
    let mut output = vec![0xa7; 1024];
    let mut out = String::new();
    let status = if kind == "fe-channels" {
        let planes: Vec<_> = (0..n)
            .map(|c| {
                let mut p = Plane::create(w, h).unwrap();
                for y in 0..h {
                    for x in 0..w {
                        p.row_mut(y)[x] = render_pipeline::value(c, x, y, 0, mode);
                    }
                }
                p
            })
            .collect();
        let channels: Vec<_> = planes
            .iter()
            .enumerate()
            .map(|(c, p)| {
                if missing & (1 << c) != 0 {
                    None
                } else {
                    Some(p)
                }
            })
            .collect();
        convert_float_channels_to_buffer(
            &channels,
            bits,
            endian,
            stride,
            &mut output,
            out_size,
            orientation,
        )
    } else {
        let gray = number::<u32>(args) != 0;
        let alpha = number::<u32>(args) != 0;
        let associated = number::<u32>(args) != 0;
        let unpremul = number::<u32>(args) != 0;
        let mut metadata = ImageMetadata::default();
        metadata.set_alpha_bits(if alpha { 8 } else { 0 }, associated);
        metadata
            .color_encoding
            .set_srgb(
                if gray {
                    ColorSpace::GRAY
                } else {
                    ColorSpace::RGB
                },
                RenderingIntent::RELATIVE,
            )
            .unwrap();
        let mut image = ImageBundle::new(Some(&metadata));
        let mut color = Image3::create(w, h).unwrap();
        for c in 0..3 {
            for y in 0..h {
                for x in 0..w {
                    color.plane_row_mut(c, y)[x] =
                        render_pipeline::value(if gray { 0 } else { c }, x, y, 0, mode);
                }
            }
        }
        image
            .set_from_image(&mut Some(color), &metadata.color_encoding)
            .unwrap();
        if alpha {
            let mut p = Plane::create(w, h).unwrap();
            for y in 0..h {
                for x in 0..w {
                    p.row_mut(y)[x] = render_pipeline::value(3, x, y, 0, mode);
                }
            }
            image.extra_channels_mut().push(p);
        }
        let status = convert_float_to_buffer(
            &image,
            bits,
            n,
            endian,
            stride,
            &mut output,
            out_size,
            orientation,
            unpremul,
        );
        write!(
            out,
            " M {} {} {} {}",
            status_code(&image.verify_metadata()),
            u32::from(image.has_alpha()),
            u32::from(image.alpha_is_premultiplied()),
            u32::from(image.alpha().is_some())
        )
        .unwrap();
        status
    };
    write!(out, " S {} {}", status_code(&status), hex(&output)).unwrap();
    out
}

fn kernel_cases(values: &[u32]) -> Vec<String> {
    values
        .chunks(97)
        .map(|row| {
            format!(
                "fe-kernel {} {}",
                row.len(),
                row.iter().map(u32::to_string).collect::<Vec<_>>().join(" ")
            )
        })
        .collect()
}

#[test]
fn scalar_half_export_matches_every_half_word_and_nearest_even_threshold_neighbors() {
    use jxl::jxl::base::float::load_float16;
    let mut values: Vec<u32> = (0..=u16::MAX)
        .map(|word| load_float16(word).to_bits())
        .collect();
    // Adjacent finite positive halves have exactly representable binary32
    // midpoints. Include both signs and the immediately neighboring floats.
    for word in 0..0x7bff {
        let midpoint = (load_float16(word) + load_float16(word + 1)) * 0.5;
        for bits in [
            midpoint.to_bits() - 1,
            midpoint.to_bits(),
            midpoint.to_bits() + 1,
        ] {
            values.extend([bits, bits | 0x80000000]);
        }
    }
    for bits in [
        65520.0f32.to_bits() - 1,
        65520.0f32.to_bits(),
        65520.0f32.to_bits() + 1,
    ] {
        values.extend([bits, bits | 0x80000000]);
    }
    let mut cases = kernel_cases(&values);
    cases.push("fe-kernel 0".into());
    compare(cases);
}

#[test]
fn scalar_half_export_matches_nan_payloads_exponents_and_generated_binary32_words() {
    let mut values = Vec::new();
    for exponent in 0..=255 {
        for mantissa in [
            0, 1, 0xfff, 0x1000, 0x1001, 0x1fff, 0x2000, 0x2001, 0x3fffff, 0x400000, 0x400001,
            0x7fdfff, 0x7fe000, 0x7fe001, 0x7ffffe, 0x7fffff,
        ] {
            let bits = (exponent << 23) | mantissa;
            values.extend([bits, bits | 0x80000000]);
        }
    }
    for mantissa in (1..=0x7fffff).step_by(257) {
        values.extend([0x7f800000 | mantissa, 0xff800000 | mantissa]);
    }
    let mut seed = 0x52ac18e7u32;
    for _ in 0..65536 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        values.push(seed);
    }
    compare(kernel_cases(&values));
}

#[test]
fn floating_channel_export_matches_raw_bits_null_channels_orientation_endianness_and_guards() {
    let mut cases = Vec::new();
    for n in 1..=4 {
        for bits in [16, 32] {
            for orientation in 1..=8 {
                for endian in 0..=3 {
                    for missing in [0, 2, 8] {
                        for mode in [1, 3] {
                            cases.push(format!("fe-channels 5 3 {n} {missing} {bits} {orientation} {endian} 96 1024 {mode}"));
                        }
                    }
                }
            }
        }
    }
    for n in [0, 1, 4, 5] {
        for missing in [0, 1] {
            for bits in [0, 8, 15, 16, 17, 31, 32, 33] {
                for stride in [0, 1, 7, 96, usize::MAX] {
                    for size in [0, 1, 96, 1024] {
                        cases.push(format!(
                            "fe-channels 5 3 {n} {missing} {bits} 1 0 {stride} {size} 3"
                        ));
                    }
                }
            }
        }
    }
    for (w, h) in [(0, 0), (3, 0)] {
        for bits in [16, 32] {
            for orientation in [0, 1, 5, 9] {
                for stride in [0, 96] {
                    cases.push(format!(
                        "fe-channels {w} {h} 1 0 {bits} {orientation} 0 {stride} 1024 3"
                    ));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn floating_buffer_stores_match_binary32_payload_bits_and_half_casts() {
    let mut values = vec![
        0, 0x80000000, 1, 0x80000001, 0x7f800000, 0xff800000, 0x7f800001, 0xff800001, 0x7fa12345,
        0xffa12345, 0x7fc98765, 0xffc98765, 0x7fffffff, 0xffffffff, 0x33000000, 0x33000001,
        0x477fefff, 0x477ff000,
    ];
    let mut seed = 0x5ab81234u32;
    for _ in 0..4096 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        values.push(seed);
    }
    let mut cases = Vec::new();
    for n in 1..=4 {
        for row in values.chunks(n * 31) {
            let w = row.len() / n;
            assert!(w > 0, "every generated row supplies a complete pixel");
            let words = row[..w * n]
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(" ");
            for bits in [16, 32] {
                for endian in 0..=3 {
                    let stride = w * n * (bits / 8) + 6;
                    cases.push(format!(
                        "fe-row {n} {bits} {endian} {w} {stride} 1024 {words}"
                    ));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn floating_bundle_export_matches_alpha_unpremultiply_and_gray_layout() {
    let mut cases = Vec::new();
    for gray in [0, 1] {
        for alpha in [0, 1] {
            for associated in [0, 1] {
                for unpremul in [0, 1] {
                    for n in 0..=5 {
                        for bits in [0, 16, 32, 33] {
                            for orientation in [1, 2, 5, 6, 7, 8] {
                                for endian in [0, 1, 2, 3] {
                                    for mode in [1, 3] {
                                        cases.push(format!("fe-bundle 5 3 {n} 0 {bits} {orientation} {endian} 96 1024 {mode} {gray} {alpha} {associated} {unpremul}"));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    compare(cases);
}
