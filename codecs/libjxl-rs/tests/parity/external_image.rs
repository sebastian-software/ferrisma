// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::float::load_float_row;
use jxl::jxl::enc_external_image::{
    convert_from_external, convert_from_external_plane_no_size_check,
};
use jxl::jxl::image::Plane;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::mem::{align_of, offset_of, size_of};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ex-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "ex-layout" {
        return format!(
            "{} {} {} {} {} {}",
            size_of::<JxlPixelFormat>(),
            align_of::<JxlPixelFormat>(),
            offset_of!(JxlPixelFormat, num_channels),
            offset_of!(JxlPixelFormat, data_type),
            offset_of!(JxlPixelFormat, endianness),
            offset_of!(JxlPixelFormat, align)
        );
    }
    if kind == "ex-row" {
        let dtype = JxlDataType(number(args));
        let little = number::<u32>(args) != 0;
        let count = number(args);
        let stride = number(args);
        let scale = f32::from_bits(number(args));
        let data = unhex(args.next().unwrap());
        let sentinel = f32::from_bits(number(args));
        let mut output = vec![sentinel; count + 3];
        let mut calls = 0;
        let status = load_float_row(&data, count, stride, dtype, little, scale, |i, v| {
            output[i] = v;
            calls += 1;
        });
        let mut out = format!("{} {}", status_code(&status), calls);
        for value in output {
            write!(out, " {}", value.to_bits()).unwrap();
        }
        return out;
    }
    assert_eq!(kind, "ex-plane");
    let checked = number::<u32>(args) != 0;
    let w = number(args);
    let h = number(args);
    let pw = number(args);
    let ph = number(args);
    let stride = number(args);
    let bits = number(args);
    let num_channels = number(args);
    let dtype = JxlDataType(number(args));
    let endian = JxlEndianness(number(args));
    let align = number(args);
    let c = number(args);
    let size = number(args);
    let data = unhex(args.next().unwrap());
    let format = JxlPixelFormat {
        num_channels,
        data_type: dtype,
        endianness: endian,
        align,
    };
    let mut plane = Plane::<f32>::create(pw, ph).unwrap();
    for y in 0..ph {
        plane.row_mut(y).fill(f32::from_bits(0x7fa12345));
    }
    let status = if checked {
        convert_from_external(&data, size, w, h, bits, format, c, &mut plane)
    } else {
        convert_from_external_plane_no_size_check(&data, w, h, stride, bits, format, c, &mut plane)
    };
    let mut out = format!(
        "{} {} {} {}",
        status_code(&status),
        plane.xsize(),
        plane.ysize(),
        plane.bytes_per_row()
    );
    for y in 0..ph {
        for value in plane.row(y) {
            write!(out, " {}", value.to_bits()).unwrap();
        }
    }
    out
}

#[test]
fn external_rows_preserve_binary16_patterns_float_copies_integer_scaling_stride_and_callback_order()
{
    let mut cases = vec!["ex-layout".to_owned()];
    for little in [false, true] {
        for dtype in [3, 5] {
            let data: Vec<_> = (0..=u16::MAX)
                .flat_map(|v| {
                    if little {
                        v.to_le_bytes()
                    } else {
                        v.to_be_bytes()
                    }
                })
                .collect();
            cases.push(format!(
                "ex-row {dtype} {} 65536 2 {} {} 2141266757",
                u32::from(little),
                (1.0f32 / 65535.0).to_bits(),
                hex(&data)
            ));
        }
    }
    let floats = [
        0u32, 0x80000000, 0x00000001, 0x7f800000, 0xff800000, 0x7fa12345, 0xffa98765, 0x7fc12345,
        0xffc98765, 0x3f800000,
    ];
    for little in [0, 1] {
        let data: Vec<_> = floats
            .into_iter()
            .flat_map(|v| {
                if little != 0 {
                    v.to_le_bytes()
                } else {
                    v.to_be_bytes()
                }
            })
            .collect();
        cases.push(format!(
            "ex-row 0 {little} 10 4 2141266757 {} 2141266757",
            hex(&data)
        ));
    }
    for dtype in 0..8 {
        for little in [0, 1] {
            for count in [0, 1, 17] {
                for stride in [0, 1, 7] {
                    for scale in [
                        0u32, 0x80000000, 0x3f800000, 0xbf800000, 0x3b808081, 0x7f800000,
                        0xff800000, 0x7fa12345, 0xffc98765,
                    ] {
                        let data: Vec<_> = (0..count * stride + 4)
                            .map(|i| (i * 137 + 59) as u8)
                            .collect();
                        cases.push(format!(
                            "ex-row {dtype} {little} {count} {stride} {scale} {} 2141266757",
                            hex(&data)
                        ));
                    }
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn external_planes_preserve_stride_size_error_order_endianness_samples_and_untouched_padding() {
    let mut cases = Vec::new();
    for (dtype, bpc, bits) in [(0, 4, 32), (2, 1, 8), (3, 2, 16), (5, 2, 16)] {
        for (w, h) in [(1usize, 1usize), (3, 2), (17, 3)] {
            for channels in 1..=4 {
                for endian in 0..4 {
                    for align in [0usize, 1, 3, 16] {
                        let row = w * channels * bpc;
                        let stride = if align < 2 {
                            row
                        } else {
                            row.div_ceil(align) * align
                        };
                        let min = stride * (h - 1) + row;
                        let max = stride * h;
                        let data: Vec<_> = (0..max + 16).map(|i| (i * 173 + 61) as u8).collect();
                        for c in 0..channels {
                            let common = format!(
                                "{w} {h} {w} {h} {stride} {bits} {channels} {dtype} {endian} {align} {c}"
                            );
                            cases.push(format!("ex-plane 0 {common} 0 {}", hex(&data)));
                            for size in [0, min.saturating_sub(1), min, max, max + 1] {
                                cases.push(format!("ex-plane 1 {common} {size} {}", hex(&data)));
                            }
                        }
                    }
                }
            }
        }
    }
    let data = hex(&[0x69; 128]);
    for checked in [0, 1] {
        for dtype in 0..8 {
            for bits in [0, 1, 7, 8, 9, 15, 16, 17, 32, 63] {
                for (w, h, pw, ph, stride) in [
                    (3, 2, 3, 2, 64),
                    (3, 2, 2, 2, 64),
                    (3, 2, 3, 1, 64),
                    (3, 2, 3, 2, 1),
                    (0, 0, 0, 0, 0),
                ] {
                    cases.push(format!("ex-plane {checked} {w} {h} {pw} {ph} {stride} {bits} 3 {dtype} 0 1 0 0 {data}"));
                }
            }
        }
    }
    for (w, h, align, channels, dtype) in [
        (usize::MAX, 2, 1, 4, 0),
        (1, usize::MAX, 16, 3, 3),
        (usize::MAX / 2, 3, 1, 1, 2),
        (usize::MAX, 1, 2, 1, 2),
    ] {
        cases.push(format!(
            "ex-plane 1 {w} {h} 1 1 0 8 {channels} {dtype} 0 {align} 0 0 -"
        ));
    }
    compare(cases);
}
