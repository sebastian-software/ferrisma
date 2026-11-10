// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_external_image::{
    convert_uint_channels_to_buffer, convert_uint_to_buffer, float_to_u32,
};
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{ImageMetadata, Orientation};
use jxl::jxl::types::JxlEndianness;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ue-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "ue-kernel" {
        let bits = number::<usize>(args);
        let n = number::<usize>(args);
        let input: Vec<_> = (0..n).map(|_| f32::from_bits(number(args))).collect();
        let mut output = vec![0xdeadbeef; n + 5];
        float_to_u32(&input, &mut output, n, ((1u64 << bits) - 1) as f32, bits);
        let mut out = String::new();
        for v in output {
            write!(out, "{v} ").unwrap();
        }
        return out;
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
    let status = if kind == "ue-channels" {
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
        convert_uint_channels_to_buffer(
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
        let status = convert_uint_to_buffer(
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
#[test]
fn scalar_integer_export_rounding_halfway_neighbors_clamp_and_suffix_match_original() {
    let mut cases = Vec::new();
    for bits in 1..=16 {
        let scale = ((1u64 << bits) - 1) as f32;
        let mut values = vec![
            -1.0,
            -0.0,
            0.0,
            1.0,
            1.001,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::from_bits(1),
            f32::from_bits(0x7fa12345),
            f32::from_bits(0xffa12345),
            f32::from_bits(0x7fc98765),
        ];
        for n in 0..256 {
            let x = (n as f32 + 0.5) / scale;
            for word in [x.to_bits() - 1, x.to_bits(), x.to_bits() + 1] {
                values.push(f32::from_bits(word));
            }
        }
        for row in values.chunks(97) {
            let words = row
                .iter()
                .map(|v| v.to_bits().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            cases.push(format!("ue-kernel {bits} {} {words}", row.len()));
        }
        cases.push(format!("ue-kernel {bits} 0"));
    }
    compare(cases);
}
#[test]
fn integer_channel_buffer_export_matches_orientation_null_channels_endianness_and_guard_order() {
    let mut cases = Vec::new();
    for n in 1..=4 {
        for bits in [1, 7, 8, 9, 12, 16] {
            for orientation in 1..=8 {
                // JxlEndianness's C++ enum represents 0..=3. The unlisted
                // value 3 exercises its defined fall-through branch; 9 would
                // be an undefined enum load rather than a codec observation.
                for endian in [0, 1, 2, 3] {
                    for missing in [0, 2, 8] {
                        for mode in [1, 3] {
                            cases.push(format!("ue-channels 5 3 {n} {missing} {bits} {orientation} {endian} 64 1024 {mode}"));
                        }
                    }
                }
            }
        }
    }
    for n in [0, 1, 4, 5] {
        for missing in [0, 1] {
            for bits in [0, 8, 16, 17] {
                for stride in [0, 1, 7, 64, usize::MAX] {
                    for size in [0, 1, 64, 1024] {
                        cases.push(format!(
                            "ue-channels 5 3 {n} {missing} {bits} 1 0 {stride} {size} 1"
                        ));
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn integer_bundle_export_preserves_alpha_unpremultiply_gray_layout_and_metadata_checks() {
    let mut cases = Vec::new();
    for gray in [0, 1] {
        for alpha in [0, 1] {
            for associated in [0, 1] {
                for unpremul in [0, 1] {
                    for n in 0..=5 {
                        for bits in [0, 8, 12, 16, 17] {
                            for orientation in [1, 2, 5, 6, 7, 8] {
                                for mode in [1, 3] {
                                    cases.push(format!("ue-bundle 5 3 {n} 0 {bits} {orientation} 0 64 1024 {mode} {gray} {alpha} {associated} {unpremul}"));
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
