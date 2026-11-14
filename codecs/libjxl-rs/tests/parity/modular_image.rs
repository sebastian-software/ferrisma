// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_frame::{ModularOutput, decode_modular};
pub(super) fn handles(kind: &str) -> bool {
    kind == "mi-decode" || kind == "mi-depth-domain"
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "mi-depth-domain" {
        use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
        use jxl::jxl::image_metadata::CodecMetadata;
        let bits = number(args);
        let exponent = number(args);
        let floating = number::<u8>(args) != 0;
        let mut metadata = CodecMetadata::default();
        metadata.size.set(3, 8).unwrap();
        if floating {
            metadata.m.set_float32_samples();
            metadata.m.bit_depth.bits_per_sample = bits;
            metadata.m.bit_depth.exponent_bits_per_sample = exponent;
        } else {
            metadata.m.set_uint_samples(bits);
        }
        metadata.m.xyb_encoded = false;
        metadata
            .m
            .color_encoding
            .set_srgb(ColorSpace::RGB, RenderingIntent::RELATIVE)
            .unwrap();
        let can = bundle::can_encode(&mut metadata.m.bit_depth).map(|_| ());
        let mut writer = BitWriter::new();
        let status =
            jxl::jxl::enc_fields::write_codestream_headers(&mut metadata, &mut writer, None);
        let written = writer.bits_written();
        pad(&mut writer);
        let mut out = format!(
            "{} W {} {} 0 {} M",
            status_code(&can),
            status_code(&status),
            written,
            hex(writer.get_span())
        );
        metadata::metadata(&mut out, &metadata.m);
        metadata::transform(&mut out, &metadata.transform_data);
        return out;
    }
    let format: u32 = number(args);
    let decoded = decode_modular(
        &unhex(args.next().unwrap()),
        if format == 0 {
            ModularOutput::SourceInteger
        } else {
            ModularOutput::Float32
        },
    );
    let mut out = format!(
        "{} M {} {}",
        status_code(&decoded.status),
        decoded.metadata.xsize(),
        decoded.metadata.ysize()
    );
    metadata::metadata(&mut out, &decoded.metadata.m);
    metadata::transform(&mut out, &decoded.metadata.transform_data);
    write!(
        out,
        " F {} {} {} O {} R",
        decoded.frame_size.0,
        decoded.frame_size.1,
        decoded.frame_decoded_bytes,
        hex(&decoded.pixels)
    )
    .unwrap();
    for v in decoded.float_pixels {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    out
}
#[derive(Clone, Copy)]
struct Config {
    w: usize,
    h: usize,
    gray: bool,
    alpha: bool,
    bits: usize,
    exponent: usize,
    xyb: bool,
    effort: usize,
    group: usize,
    responsive: usize,
    rct: usize,
    predictor: usize,
    palette: bool,
    epf: usize,
    gab: bool,
    noise: bool,
    pattern: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            w: 17,
            h: 13,
            gray: false,
            alpha: false,
            bits: 8,
            exponent: 0,
            xyb: false,
            effort: 7,
            group: 0,
            responsive: 0,
            rct: 0,
            predictor: 5,
            palette: false,
            epf: 0,
            gab: false,
            noise: false,
            pattern: 0,
        }
    }
}
fn pixels(c: Config) -> Vec<u8> {
    let n = (if c.gray { 1 } else { 3 }) + usize::from(c.alpha);
    let mut seed = 0x7513ab4345u64;
    (0..c.w * c.h * n)
        .flat_map(|i| {
            let value = match c.pattern {
                0 => ((i * 13 + i / n * 7) % 61) as f32 / 60.0,
                1 => [0.0f32, 0.25, 0.5, 0.75][(i / n + i % n) % 4],
                2 => ((i / c.w / n + i / n % 7) % 2) as f32,
                3 => (next(&mut seed) % 65536) as f32 / 65535.0,
                _ => [0.0, -0.0, 0.125, 0.5, 1.0, -0.25, 0.75][i % 7],
            };
            value.to_ne_bytes()
        })
        .collect()
}
fn encode(c: Config) -> Vec<u8> {
    let req = format!(
        "mi-encode {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        c.w,
        c.h,
        u8::from(c.gray),
        u8::from(c.alpha),
        c.bits,
        c.exponent,
        u8::from(c.xyb),
        c.effort,
        c.group,
        c.responsive,
        c.rct,
        c.predictor,
        u8::from(c.palette),
        c.epf,
        u8::from(c.gab),
        u8::from(c.noise),
        hex(&pixels(c))
    );
    let result = original_observations(&[req]);
    let words: Vec<_> = result[0].split_whitespace().collect();
    assert_eq!(
        words[0], "0",
        "original fixture encoding: bits={}, exp={}, gray={}, alpha={}, xyb={}",
        c.bits, c.exponent, c.gray, c.alpha, c.xyb
    );
    unhex(words[1])
}
#[derive(Debug)]
struct Reached {
    color_transform: u32,
    group_shift: usize,
    flags: u64,
    epf: usize,
    gab: bool,
    groups: usize,
    dc_groups: usize,
    transforms: Vec<(u32, u32, usize)>,
    predictors: u32,
}
fn inspect(bytes: &[u8]) -> Reached {
    let result = original_observations(&[format!("mi-inspect {}", hex(bytes))]);
    let mut fields = result[0].split_whitespace();
    assert_eq!(fields.next(), Some("0"), "{}", result[0]);
    assert_eq!(fields.next(), Some("H"));
    assert_eq!(number::<u32>(&mut fields), 1, "genuine Modular header");
    let color_transform = number(&mut fields);
    let group_shift = number(&mut fields);
    let flags = number(&mut fields);
    let epf = number(&mut fields);
    let gab = number::<u32>(&mut fields) != 0;
    assert_eq!(number::<usize>(&mut fields), 1, "single-pass source frame");
    assert_eq!(fields.next(), Some("G"));
    let groups = number(&mut fields);
    let dc_groups = number(&mut fields);
    assert_eq!(fields.next(), Some("D"));
    assert_eq!(number::<i32>(&mut fields), 0, "original ProcessDCGlobal");
    assert_eq!(fields.next(), Some("T"));
    let transforms = (0..number(&mut fields))
        .map(|_| {
            (
                number(&mut fields),
                number(&mut fields),
                number(&mut fields),
            )
        })
        .collect();
    assert_eq!(fields.next(), Some("P"));
    let predictors = number(&mut fields);
    assert_eq!(fields.next(), Some("C"));
    assert_eq!(
        number::<i32>(&mut fields),
        0,
        "original global reader close"
    );
    assert_eq!(fields.next(), Some("R"));
    assert_eq!(
        number::<i32>(&mut fields),
        0,
        "original header reader close"
    );
    assert_eq!(fields.next(), None);
    Reached {
        color_transform,
        group_shift,
        flags,
        epf,
        gab,
        groups,
        dc_groups,
        transforms,
        predictors,
    }
}

fn request(bytes: &[u8], format: usize) -> String {
    format!("mi-decode {format} {}", hex(bytes))
}
fn chunks(cases: Vec<String>) {
    for chunk in cases.chunks(4) {
        compare(chunk.to_vec());
    }
}
#[test]
fn modular_complete_integer_depths_and_explicit_output_precision() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for bits in [1, 7, 8, 9, 10, 12, 15, 16, 17, 22, 23, 24, 30, 31] {
        for (gray, alpha) in [(false, false), (true, false), (false, true), (true, true)] {
            let c = Config {
                bits,
                gray,
                alpha,
                ..Config::default()
            };
            let bytes = encode(c);
            let reached = inspect(&bytes);
            assert_eq!(reached.color_transform, 1, "NONE source transform");
            cases.push(request(&bytes, 1));
            // Larger source depths exercise the original integer export's
            // failure after decode/allocation, retaining partial output/state.
            cases.push(request(&bytes, 0));
        }
    }
    chunks(cases);
}
#[test]
fn modular_complete_custom_float_depths_and_xyb() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for (bits, exponent) in [(16, 5), (24, 7), (24, 8), (32, 8)] {
        for (gray, alpha) in [(false, false), (true, false), (false, true), (true, true)] {
            let bytes = encode(Config {
                bits,
                exponent,
                gray,
                alpha,
                pattern: 4,
                predictor: if bits == 32 { 0 } else { 5 },
                ..Config::default()
            });
            inspect(&bytes);
            cases.push(request(&bytes, 1));
            cases.push(request(&bytes, 0));
        }
    }
    for bits in [8, 16] {
        for gray in [false, true] {
            let bytes = encode(Config {
                bits,
                gray,
                xyb: true,
                pattern: 1,
                ..Config::default()
            });
            let reached = inspect(&bytes);
            assert_eq!(reached.color_transform, 0, "XYB source transform");
            cases.push(request(&bytes, 1));
            cases.push(request(&bytes, 0));
        }
    }
    chunks(cases);
}
#[test]
fn modular_complete_original_predictors_palette_rct_squeeze_and_groups() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut reached_predictors = 0;
    for predictor in 0..16 {
        let bytes = encode(Config {
            predictor,
            ..Config::default()
        });
        let reached = inspect(&bytes);
        assert_ne!(reached.predictors, 0, "{reached:?}");
        if predictor < 14 {
            assert_eq!(reached.predictors, 1 << predictor, "{reached:?}");
            reached_predictors |= reached.predictors;
        } else if predictor == 14 {
            // Best is an encoder choice between Gradient and Weighted, not a
            // serialized decoder predictor numbered14.
            assert_eq!(reached.predictors & !((1 << 5) | (1 << 6)), 0);
        } else {
            assert_eq!(reached.predictors & !((1 << 14) - 1), 0);
        }
        cases.push(request(&bytes, 0));
    }
    assert_eq!(reached_predictors, (1 << 14) - 1);
    for responsive in [0, 1] {
        for palette in [false, true] {
            for rct in [0, 6, 28] {
                let bytes = encode(Config {
                    w: 67,
                    h: 33,
                    rct,
                    responsive,
                    palette,
                    pattern: 1,
                    ..Config::default()
                });
                let reached = inspect(&bytes);
                let expected: Vec<_> = if responsive == 0 && palette {
                    vec![(1, 6, 0)] // actual original global Palette
                } else {
                    let mut expected = Vec::new();
                    if rct != 0 {
                        expected.push((0, rct as u32, 0));
                    }
                    if responsive != 0 {
                        expected.push((2, 6, 0));
                    }
                    expected
                };
                assert_eq!(reached.transforms, expected, "{reached:?}");
                // An empty explicit squeeze parameter list is the original
                // default Squeeze stack, not the absence of the transform.
                assert_eq!(reached.color_transform, 1);
                cases.push(request(&bytes, 0));
            }
        }
    }
    for group in 0..=2 {
        let g = 128usize << group;
        for (w, h) in [
            (g - 1, 3),
            (g, 3),
            (g + 1, 3),
            (3, g - 1),
            (3, g + 1),
            (8 * g + 1, 1),
            (1, 8 * g + 1),
        ] {
            let bytes = encode(Config {
                w,
                h,
                group,
                rct: 6,
                pattern: 2,
                ..Config::default()
            });
            let reached = inspect(&bytes);
            assert_eq!(reached.group_shift, group);
            assert_eq!(reached.groups, w.div_ceil(g) * h.div_ceil(g), "{reached:?}");
            assert_eq!(
                reached.dc_groups,
                w.div_ceil(8 * g) * h.div_ceil(8 * g),
                "{reached:?}"
            );
            cases.push(request(&bytes, 0));
        }
    }
    chunks(cases);
}
#[test]
fn modular_complete_filters_noise_and_malformed_file_boundaries() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for epf in 0..=3 {
        for noise in [false, true] {
            let bytes = encode(Config {
                xyb: true,
                epf,
                noise,
                gab: epf % 2 == 1,
                responsive: 1,
                pattern: 1,
                ..Config::default()
            });
            let reached = inspect(&bytes);
            assert_eq!(reached.color_transform, 0);
            assert_eq!(reached.epf, epf);
            assert_eq!(reached.gab, epf % 2 == 1);
            assert_eq!(reached.flags, u64::from(noise));
            assert!(reached.transforms.iter().any(|t| t.0 == 2), "{reached:?}");
            cases.push(request(&bytes, 1));
        }
    }
    // Cross a DC-group boundary with source filters and noise simultaneously:
    // Modular sigma fill and full-image storage must apply to every group.
    for (w, h) in [(1025, 3), (3, 1025)] {
        let bytes = encode(Config {
            w,
            h,
            xyb: true,
            epf: 3,
            gab: true,
            noise: true,
            responsive: 1,
            pattern: 1,
            ..Config::default()
        });
        let reached = inspect(&bytes);
        assert_eq!((reached.groups, reached.dc_groups), (9, 2));
        assert_eq!((reached.epf, reached.gab, reached.flags), (3, true, 1));
        cases.push(request(&bytes, 1));
    }
    for c in [
        Config {
            w: 7,
            h: 5,
            bits: 1,
            ..Config::default()
        },
        Config {
            w: 13,
            h: 9,
            bits: 23,
            ..Config::default()
        },
        Config {
            w: 17,
            h: 13,
            bits: 24,
            exponent: 7,
            alpha: true,
            pattern: 4,
            ..Config::default()
        },
        Config {
            w: 129,
            h: 3,
            responsive: 1,
            rct: 6,
            ..Config::default()
        },
    ] {
        let bytes = encode(c);
        for len in (0..=bytes.len().min(100)).chain([
            bytes.len() / 2,
            bytes.len().saturating_sub(1),
            bytes.len(),
        ]) {
            cases.push(request(&bytes[..len], 1));
        }
        for index in [
            bytes.len() / 3,
            bytes.len() / 2,
            bytes.len().saturating_sub(1),
        ] {
            for bit in [0, 3, 7] {
                let mut changed = bytes.clone();
                changed[index] ^= 1 << bit;
                cases.push(request(&changed, 1));
            }
        }
        for trailing in [0, 1, 255] {
            let mut changed = bytes.clone();
            changed.push(trailing);
            cases.push(request(&changed, 1));
        }
    }
    chunks(cases);
}

#[test]
fn modular_original_depth_writer_domain_and_partial_header_state() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    // The original public writer rejects integer32 during CanEncode, before
    // emitting metadata. Retain its actual partial signature/size header and
    // metadata state; do not manufacture a32-bit integer codestream.
    let cases: Vec<_> = [
        (0, 0, 0),
        (1, 0, 0),
        (31, 0, 0),
        (32, 0, 0),
        (64, 0, 0),
        (5, 2, 1),
        (16, 5, 1),
        (24, 7, 1),
        (24, 8, 1),
        (32, 8, 1),
        (32, 9, 1),
        (4, 2, 1),
        (16, 1, 1),
        (40, 8, 1),
    ]
    .into_iter()
    .map(|(b, e, f)| format!("mi-depth-domain {b} {e} {f}"))
    .collect();
    let rejected = original_observations(&["mi-depth-domain 32 0 0".into()]);
    assert!(rejected[0].starts_with("1 W 1 "), "{}", rejected[0]);
    chunks(cases);
}
