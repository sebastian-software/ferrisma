// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_frame::{ModularOutput, decode_modular};

struct Capture {
    wire: &'static [u8],
    pixels: &'static [u8],
    float_pixels: &'static [u8],
    size: (usize, usize),
    bits: u32,
    exponent: u32,
    alpha: bool,
    float_output: bool,
}
macro_rules! capture {
    ($name:literal, $w:literal, $h:literal, $bits:literal, $exp:literal, $alpha:literal, $float:literal) => {
        Capture {
            wire: include_bytes!(concat!("data/", $name, ".jxl")),
            pixels: include_bytes!(concat!("data/", $name, ".pixels")),
            float_pixels: include_bytes!(concat!("data/", $name, ".float-pixels")),
            size: ($w, $h),
            bits: $bits,
            exponent: $exp,
            alpha: $alpha,
            float_output: $float,
        }
    };
}
#[test]
fn captured_original_general_modular_files_decode_without_cpp() {
    // Pinned original WriteCodestreamHeaders + EFVarDctOneShot, Modular mode,
    // followed by its actual DecodeFile/external converter. Palette16 contains
    // a genuine Palette; rct-squeeze8 contains RCT28 + default Squeeze and two
    // groups. Binary32 captures use little-endian words for portable fixtures.
    for c in [
        capture!("modular-rgb1", 3, 8, 1, 0, false, false),
        capture!("modular-gray-alpha12", 3, 8, 12, 0, true, false),
        capture!("modular-rgb31", 3, 8, 31, 0, false, true),
        capture!("modular-rgb-alpha-float24e7", 3, 8, 24, 7, true, true),
        capture!("modular-palette16", 17, 13, 16, 0, false, false),
        capture!("modular-rct-squeeze8", 129, 3, 8, 0, false, false),
    ] {
        let decoded = decode_modular(
            c.wire,
            if c.float_output {
                ModularOutput::Float32
            } else {
                ModularOutput::SourceInteger
            },
        );
        assert_eq!(decoded.status, Ok(()));
        assert_eq!(decoded.frame_size, c.size);
        assert_eq!(decoded.reader_bits, c.wire.len() * 8);
        assert_eq!(decoded.metadata.m.bit_depth.bits_per_sample, c.bits);
        assert_eq!(
            decoded.metadata.m.bit_depth.exponent_bits_per_sample,
            c.exponent
        );
        assert_eq!(
            decoded.metadata.m.bit_depth.floating_point_sample,
            c.exponent != 0
        );
        assert_eq!(decoded.metadata.m.has_alpha(), c.alpha);
        let width = if c.float_output {
            4
        } else {
            c.bits.div_ceil(8) as usize
        };
        let expected: Vec<u8> = c
            .pixels
            .chunks_exact(width)
            .flat_map(|bytes| match width {
                1 => bytes.to_vec(),
                2 => u16::from_le_bytes(bytes.try_into().unwrap())
                    .to_ne_bytes()
                    .to_vec(),
                4 => u32::from_le_bytes(bytes.try_into().unwrap())
                    .to_ne_bytes()
                    .to_vec(),
                _ => unreachable!("captured original output widths"),
            })
            .collect();
        assert_eq!(decoded.pixels, expected);
        let expected_float: Vec<_> = c
            .float_pixels
            .as_chunks::<4>()
            .0
            .iter()
            .map(|bytes| u32::from_le_bytes(*bytes))
            .collect();
        assert_eq!(
            decoded
                .float_pixels
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_float
        );
    }
}

#[test]
fn explicit_integer_export_retains_original_failure_after_decoding_large_depth() {
    let wire = include_bytes!("data/modular-rgb31.jxl");
    let decoded = decode_modular(wire, ModularOutput::SourceInteger);
    assert_eq!(decoded.status, Err(StatusCode::GenericError));
    assert_eq!(decoded.metadata.m.bit_depth.bits_per_sample, 31);
    assert_eq!(decoded.frame_size, (3, 8));
    assert_eq!(decoded.reader_bits, wire.len() * 8);
    assert_eq!(decoded.pixels, vec![0; 3 * 8 * 3 * 4]);
    assert!(decoded.float_pixels.is_empty());
}
