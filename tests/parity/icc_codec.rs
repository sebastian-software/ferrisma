// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::{icc_codec as codec, icc_codec_common as common, padded_bytes::PaddedBytes};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ic-")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    match kind {
        "ic-contexts" => {
            let i = number(args);
            for a in 0..256 {
                for b in 0..256 {
                    write!(out, " {}", common::icc_ans_context(i, a, b)).unwrap();
                }
            }
        }
        "ic-bounds" => {
            let a = number(args);
            let b = number(args);
            let size = number(args);
            let value = number(args);
            write!(
                out,
                "{} {}",
                status_code(&common::check_out_of_bounds(a, b, size)),
                status_code(&common::check_is_32_bit(value))
            )
            .unwrap();
        }
        "ic-varint" => {
            let mut pos = number(args);
            let mut value = number(args);
            let data = unhex(args.next().unwrap());
            let s = codec::decode_var_int(&data, &mut pos, &mut value);
            write!(out, "{} {pos} {value}", status_code(&s)).unwrap();
        }
        "ic-header" => {
            let size = number(args);
            let pos = number(args);
            let data = unhex(args.next().unwrap());
            let mut header = common::icc_initial_header_prediction(size);
            common::icc_predict_header(&data, &mut header, pos);
            out = hex(&header);
        }
        "ic-access" => {
            let pos = number(args);
            let mut data = unhex(args.next().unwrap());
            let tag: [u8; 4] = unhex(args.next().unwrap()).try_into().unwrap();
            write!(
                out,
                "{} {} ",
                common::decode_uint32(&data, pos),
                hex(&common::decode_keyword(&data, pos))
            )
            .unwrap();
            common::encode_keyword(&tag, &mut data, pos);
            out.push_str(&hex(&data));
            let mut result = PaddedBytes::default();
            let s = common::append_uint32(0x89abcdef, &mut result)
                .and_then(|()| common::append_keyword(&tag, &mut result));
            write!(out, " {} {}", status_code(&s), hex(result.as_slice())).unwrap();
        }
        "ic-linear" => {
            let start = number(args);
            let stride = number(args);
            let width = number(args);
            let order = number(args);
            let count = number(args);
            let data = unhex(args.next().unwrap());
            for i in 0..count {
                write!(
                    out,
                    " {}",
                    common::linear_predict_icc_value(&data, start, i, stride, width, order)
                )
                .unwrap();
            }
        }
        "ic-shuffle" => {
            let width = number(args);
            let mut data = unhex(args.next().unwrap());
            let s = codec::shuffle(&mut data, width);
            write!(out, "{} {}", status_code(&s), hex(&data)).unwrap();
        }
        "ic-preamble" => {
            let size = number(args);
            let data = unhex(args.next().unwrap());
            write!(out, "{}", status_code(&codec::check_preamble(&data, size))).unwrap();
        }
        "ic-unpredict" => {
            let data = unhex(args.next().unwrap());
            let seed = unhex(args.next().unwrap());
            let mut result = PaddedBytes::default();
            result.append(&seed).unwrap();
            let s = codec::unpredict_icc(&data, &mut result);
            write!(out, "{} {}", status_code(&s), hex(result.as_slice())).unwrap();
        }
        _ => panic!("unknown ICC observer command {kind}"),
    }
    out
}
fn exact(cases: Vec<String>) {
    let len = cases.len();
    for chunk in cases.chunks(256) {
        let source = original_observations(chunk);
        for (request, actual) in chunk.iter().zip(source) {
            assert_eq!(super::observe(request), actual, "request: {request}");
        }
    }
    println!("{len} C++/Rust differential cases matched");
}
fn varint(mut value: u64) -> Vec<u8> {
    let mut data = Vec::new();
    while value > 127 {
        data.push((value as u8 & 127) | 128);
        value >>= 7;
    }
    data.push(value as u8);
    data
}
fn command_stream(size: u64, commands: &[u8], data: &[u8]) -> Vec<u8> {
    let mut encoded = varint(size);
    encoded.extend(varint(commands.len() as u64));
    encoded.extend(commands);
    encoded.extend(data);
    encoded
}
fn unpredict(data: &[u8]) -> String {
    format!("ic-unpredict {} -", hex(data))
}
#[test]
fn icc_common_byte_classes_endian_tags_prediction_and_unsigned_guards_match_original() {
    let mut cases = vec![0, 128, 129, 130, usize::MAX]
        .into_iter()
        .map(|i| format!("ic-contexts {i}"))
        .collect::<Vec<_>>();
    let values = [
        0,
        1,
        3,
        4,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        u64::MAX - 3,
        u64::MAX,
    ];
    for a in values {
        for b in values {
            for size in values {
                cases.push(format!("ic-bounds {a} {b} {size} {}", a.wrapping_add(b)));
            }
        }
    }
    for len in [0, 1, 3, 4, 5, 15, 127, 128, 129] {
        let data = (0..len).map(|i| (i * 73 + 19) as u8).collect::<Vec<_>>();
        for pos in 0..len + 5 {
            cases.push(format!("ic-access {pos} {} 897a00ff", hex(&data)));
        }
    }
    for size in [0, 1, 128, u32::MAX] {
        for len in [0, 7, 8, 40, 41, 42, 83, 128] {
            for pos in [0, 8, 40, 41, 42, 43, 127] {
                for pair in [*b"AP", *b"MS", *b"SG", *b"SU", *b"zz"] {
                    let mut data = (0..len).map(|i| (i * 31) as u8).collect::<Vec<_>>();
                    if len > 40 {
                        data[40] = pair[0];
                    }
                    if len > 41 {
                        data[41] = pair[1];
                    }
                    cases.push(format!("ic-header {size} {pos} {}", hex(&data)));
                }
            }
        }
    }
    let data = (0..256)
        .map(|i| ((i * 71) ^ (i * 13)) as u8)
        .collect::<Vec<_>>();
    for width in [1, 2, 4] {
        for stride in [width, width * 2, width * 3, width * 7] {
            for order in [-1, 0, 1, 2, 3, i32::MAX] {
                cases.push(format!(
                    "ic-linear 128 {stride} {width} {order} 65 {}",
                    hex(&data)
                ));
            }
        }
    }
    let history = (0..64).collect::<Vec<u8>>();
    for start in [64, 65, 66, 67, 68] {
        cases.push(format!("ic-linear {start} 8 4 2 1 {}", hex(&history)));
    }
    exact(cases);
}
#[test]
fn icc_varints_keep_consumed_prefix_and_unchanged_output_on_every_failure() {
    let mut cases = Vec::new();
    for value in [
        0,
        1,
        127,
        128,
        129,
        16383,
        16384,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        1 << 63,
        u64::MAX,
    ] {
        let bytes = varint(value);
        for prefix in 0..4 {
            for end in 0..=bytes.len() {
                let mut data = vec![0x5a; prefix];
                data.extend(&bytes[..end]);
                cases.push(format!(
                    "ic-varint {prefix} 1234605616436508552 {}",
                    hex(&data)
                ));
            }
        }
    }
    for tenth in 0..=255 {
        let mut data = vec![0xff; 9];
        data.push(tenth);
        data.push(0);
        cases.push(format!("ic-varint 0 987654321 {}", hex(&data)));
    }
    for len in 0..=12 {
        for byte in [0, 1, 127, 128, 255] {
            let data = vec![byte; len];
            for pos in 0..=len + 2 {
                cases.push(format!("ic-varint {pos} 777 {}", hex(&data)));
            }
        }
    }
    exact(cases);
}
#[test]
fn icc_shuffle_and_all_command_types_preserve_exact_partial_output() {
    let mut cases = Vec::new();
    for width in [1, 2, 3, 4, 7, 17] {
        for len in 0..=65 {
            let data = (0..len).map(|i| (i * 29 + 3) as u8).collect::<Vec<_>>();
            cases.push(format!("ic-shuffle {width} {}", hex(&data)));
        }
    }
    // Synthetic command streams target the real original command parser. They
    // are component inputs, rather than invented complete JPEG XL files.
    for command in 0..=255u8 {
        for num in [0, 1, 3, 4, 9, 17] {
            let mut commands = vec![0, command];
            commands.extend(varint(num));
            let mut data = vec![0; 128];
            data.extend((0..num).map(|i| (i * 37) as u8));
            cases.push(unpredict(&command_stream(128 + num, &commands, &data)));
        }
    }
    for flags in 0..=255u8 {
        let mut commands = vec![0, 4, flags];
        if flags & 16 != 0 {
            commands.extend(varint(16));
        }
        commands.extend(varint(3));
        let mut data = vec![0; 128];
        data.extend([19, 71, 143]);
        cases.push(unpredict(&command_stream(131, &commands, &data)));
    }
    for width in [1u8, 2, 4] {
        for order in 0..=3 {
            for stride in [0, 1, 2, 4, 8, 31, 32, 33, u64::MAX] {
                for num in [0, 1, 3, 7, 19] {
                    let flags = (width - 1) | (order << 2) | 16;
                    let mut commands = vec![0, 4, flags];
                    commands.extend(varint(stride));
                    commands.extend(varint(num));
                    let mut data = vec![0; 128];
                    data.extend((0..num).map(|i| (i * 67 + 21) as u8));
                    cases.push(unpredict(&command_stream(128 + num, &commands, &data)));
                }
            }
        }
    }
    // All known and unknown tag codes, explicit/implicit offset/size, and the
    // TRC/XYZ triplet expansion are observed through the original decoder.
    for code in 0..64u8 {
        for flags in [0, 64, 128, 192] {
            let mut commands = vec![4, code | flags];
            if flags & 64 != 0 {
                commands.extend(varint(168));
            }
            if flags & 128 != 0 {
                commands.extend(varint(20));
            }
            commands.push(0);
            let mut data = vec![0; 128];
            if code == 1 {
                data.extend(b"test");
            }
            cases.push(unpredict(&command_stream(168, &commands, &data)));
        }
    }
    exact(cases);
}
#[test]
fn icc_original_predictor_outputs_roundtrip_and_keep_truncated_mutated_state() {
    let mut profiles = Vec::new();
    use jxl::jxl::color_encoding_internal::ColorEncoding;
    for gray in [false, true] {
        profiles.push(ColorEncoding::srgb(gray).icc().to_vec());
        profiles.push(ColorEncoding::linear_srgb(gray).icc().to_vec());
    }
    for size in [
        0, 1, 7, 8, 40, 41, 42, 127, 128, 129, 132, 144, 256, 513, 2048,
    ] {
        for mode in 0..3 {
            profiles.push(
                (0..size)
                    .map(|i| match mode {
                        0 => 0,
                        1 => (i * 31 + 7) as u8,
                        _ => (i * i + 13 * i) as u8,
                    })
                    .collect(),
            );
        }
    }
    let encoded = original_observations(
        &profiles
            .iter()
            .map(|p| format!("ic-encode {}", hex(p)))
            .collect::<Vec<_>>(),
    );
    let mut cases = Vec::new();
    for (profile, line) in profiles.iter().zip(encoded) {
        let mut fields = line.split_whitespace();
        assert_eq!(fields.next(), Some("0"));
        let data = unhex(fields.next().unwrap());
        let request = unpredict(&data);
        let original = original_observations(std::slice::from_ref(&request));
        assert_eq!(
            original[0],
            format!("0 {}", hex(profile)),
            "actual original predictor/decoder roundtrip"
        );
        cases.push(request.clone());
        cases.push(format!("ic-unpredict {} 1234", hex(&data)));
        for end in 0..=data.len().min(32) {
            cases.push(unpredict(&data[..end]));
        }
        for end in [data.len() / 2, data.len().saturating_sub(1), data.len()] {
            cases.push(unpredict(&data[..end]));
        }
        for index in 0..data.len().min(32) {
            for mask in [1, 0x80, 0xff] {
                let mut changed = data.clone();
                changed[index] ^= mask;
                cases.push(unpredict(&changed));
            }
        }
        let mut extra = data.clone();
        extra.push(0x5a);
        cases.push(unpredict(&extra));
    }
    exact(cases);
}
#[test]
fn icc_preamble_and_malformed_command_varints_preserve_source_checks() {
    let mut cases = Vec::new();
    for osize in [
        0,
        1,
        127,
        128,
        65536,
        (1 << 28) - 1,
        1 << 28,
        (1 << 28) + 1,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        u64::MAX,
    ] {
        for csize in [0, 1, 20, 40, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX] {
            let mut data = varint(osize);
            data.extend(varint(csize));
            data.resize(32, 0);
            for enc_size in [
                0,
                osize as usize,
                osize.wrapping_add(65536) as usize,
                osize.wrapping_add(65537) as usize,
                usize::MAX,
            ] {
                cases.push(format!("ic-preamble {enc_size} {}", hex(&data)));
            }
            cases.push(unpredict(&data));
        }
    }
    for len in 0..=20 {
        for byte in [0, 1, 127, 128, 255] {
            let data = vec![byte; len];
            cases.push(unpredict(&data));
            cases.push(format!("ic-preamble 128 {}", hex(&data)));
        }
    }
    exact(cases);
}
