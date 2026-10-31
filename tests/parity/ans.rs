// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Boundary adapters for source entropy streams. Fixture generation calls the
// original encoder; it is never a dependency or fallback of the Rust library.

use super::*;
use jxl::jxl::dec_ans::{self, ANSCode, ANSSymbolReader, HybridUintConfig};
use jxl::jxl::dec_context_map::decode_context_map;
use jxl::jxl::enc_ans::{
    ANSEncSymbolInfo, EntropyEncodingData, Token, encode_uint_configs, write_tokens_with_allotment,
};
use jxl::jxl::inverse_mtf::inverse_move_to_front_transform;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ans-")
        || matches!(
            kind,
            "hybrid"
                | "uint-config"
                | "uint-config-read"
                | "context-map"
                | "inverse-mtf"
                | "special-distance"
        )
}

fn config(args: &mut std::str::SplitWhitespace<'_>) -> HybridUintConfig {
    HybridUintConfig::new(number(args), number(args), number(args))
}

fn config_observation(result: &mut String, cfg: HybridUintConfig) {
    write!(
        result,
        " {},{},{},{}",
        cfg.split_exponent, cfg.split_token, cfg.msb_in_token, cfg.lsb_in_token
    )
    .unwrap();
}

fn code_observation(result: &mut String, code: &ANSCode, initialized_tables: bool) {
    write!(
        result,
        " {} {} {} {} {} {} {}",
        u32::from(code.use_prefix_code),
        code.log_alpha_size,
        code.max_num_bits,
        u32::from(code.lz77.enabled),
        code.lz77.min_symbol,
        code.lz77.min_length,
        code.lz77.nonserialized_distance_context
    )
    .unwrap();
    config_observation(result, code.lz77.length_uint_config);
    write!(result, " {}", code.uint_config.len()).unwrap();
    for &cfg in &code.uint_config {
        config_observation(result, cfg);
    }
    write!(result, " {}", code.degenerate_symbols.len()).unwrap();
    for symbol in &code.degenerate_symbols {
        write!(result, " {symbol}").unwrap();
    }
    write!(result, " {}", code.huffman_data.len()).unwrap();
    for data in &code.huffman_data {
        write!(result, " {}", data.table.len()).unwrap();
        for h in &data.table {
            write!(result, " {},{}", h.bits, h.value).unwrap();
        }
    }
    if initialized_tables && !code.use_prefix_code {
        write!(result, " {}", code.alias_tables().len()).unwrap();
        for &e in code.alias_tables() {
            let (a, b, c, d, f) = (
                e.cutoff,
                e.right_value,
                e.freq0,
                e.offsets1,
                e.freq1_xor_freq0,
            );
            write!(result, " {a},{b},{c},{d},{f}").unwrap();
        }
    }
}

fn ans_read(args: &mut std::str::SplitWhitespace<'_>, mode: &str) -> String {
    let stream = mode != "ans-hist";
    let contexts = number(args);
    let disallow: u32 = number(args);
    let bytes = unhex(args.next().unwrap());
    let offset = number(args);
    let (count, multiplier) = if stream {
        (number(args), number(args))
    } else {
        (0, 0)
    };
    let (checkpoint_at, interval): (usize, usize) = if mode == "ans-checkpoint" {
        (number(args), number(args))
    } else {
        (0, 0)
    };
    let mut br = BitReader::new(&bytes);
    br.skip_bits(offset);
    let mut code = ANSCode::default();
    let mut context_map = Vec::new();
    let status = dec_ans::decode_histograms(
        &mut br,
        contexts,
        &mut code,
        &mut context_map,
        disallow != 0,
    );
    let mut result = format!(
        "{} {} {}",
        status_code(&status),
        br.total_bits_consumed(),
        context_map.len()
    );
    for ctx in &context_map {
        write!(result, " {ctx}").unwrap();
    }
    code_observation(&mut result, &code, status.is_ok());
    if stream && status.is_ok() {
        let mut reader = ANSSymbolReader::create(&code, &mut br, multiplier).unwrap();
        write!(
            result,
            " {} {}",
            u32::from(reader.uses_lz77()),
            u32::from(reader.is_huff_rle_only())
        )
        .unwrap();
        if mode == "ans-single" {
            let mut value = 123456;
            let single =
                reader.is_single_value_and_advance(usize::from(context_map[0]), &mut value, count);
            write!(result, " {},{value}", u32::from(single)).unwrap();
        }
        let (mut value, mut run) = (99, 17);
        let mut i = 0;
        while i < count {
            if mode == "ans-rle" {
                reader.read_hybrid_uint_clustered_huff_rle_only(
                    usize::from(context_map[0]),
                    &mut br,
                    &mut value,
                    &mut run,
                );
                write!(result, " {value},{run}").unwrap();
                i += 1;
            } else if mode == "ans-checkpoint" && i == checkpoint_at {
                let mut checkpoint = dec_ans::Checkpoint::default();
                reader.save(&mut checkpoint);
                let pos = br.total_bits_consumed();
                write!(
                    result,
                    " C{},{},{},{}",
                    checkpoint.state,
                    checkpoint.num_decoded,
                    checkpoint.num_to_copy,
                    checkpoint.copy_pos
                )
                .unwrap();
                let values: Vec<_> = (0..interval)
                    .map(|_| reader.read_hybrid_uint(0, &mut br, &context_map))
                    .collect();
                reader.restore(&checkpoint);
                write!(result, ",{}", status_code(&br.close())).unwrap();
                br = BitReader::new(&bytes);
                br.skip_bits(pos);
                for expected in values {
                    let replay = reader.read_hybrid_uint(0, &mut br, &context_map);
                    assert_eq!(replay, expected, "checkpoint replay");
                    write!(result, " {replay}").unwrap();
                }
                i += interval;
            } else {
                let value = if mode == "ans-nolz" {
                    reader.read_hybrid_uint_inlined::<false>(0, &mut br, &context_map)
                } else {
                    reader.read_hybrid_uint(0, &mut br, &context_map)
                };
                write!(result, " {}", value).unwrap();
                i += 1;
            }
        }
        write!(result, " {}", u32::from(reader.check_ans_final_state())).unwrap();
    }
    write!(
        result,
        " {} {}",
        br.total_bits_consumed(),
        status_code(&br.close())
    )
    .unwrap();
    result
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "ans-hist" | "ans-stream" | "ans-checkpoint" | "ans-single" | "ans-rle" | "ans-nolz" => {
            ans_read(args, kind)
        }
        "hybrid" => {
            let cfg = config(args);
            let (value, offset) = (number(args), number(args));
            let (token, nbits, bits) = cfg.encode(value);
            let mut writer = BitWriter::new();
            writer
                .with_max_bits(
                    64,
                    LayerType::Header,
                    None,
                    |w, _| {
                        w.write(offset, (1 << offset) - 1);
                        w.write(nbits as usize, u64::from(bits));
                        Ok(())
                    },
                    false,
                )
                .unwrap();
            pad(&mut writer);
            let mut reader = BitReader::new(writer.get_span());
            reader.skip_bits(offset);
            reader.refill();
            let decoded =
                ANSSymbolReader::read_hybrid_uint_config(cfg, token as usize, &mut reader);
            format!(
                "{token} {nbits} {bits} {} {decoded} {} {}",
                cfg.lsb_mask(),
                reader.total_bits_consumed(),
                status_code(&reader.close())
            )
        }
        "uint-config" => {
            let (log_alpha, offset) = (number(args), number(args));
            let cfg = config(args);
            let mut writer = BitWriter::new();
            prefix(&mut writer, offset);
            writer
                .with_max_bits(
                    32,
                    LayerType::Header,
                    None,
                    |w, _| {
                        encode_uint_configs(&[cfg], w, log_alpha);
                        Ok(())
                    },
                    false,
                )
                .unwrap();
            let bits = writer.bits_written();
            pad(&mut writer);
            let mut reader = BitReader::new(writer.get_span());
            reader.skip_bits(offset);
            let mut decoded = [HybridUintConfig::default()];
            let status = dec_ans::decode_uint_configs(log_alpha, &mut decoded, &mut reader);
            let mut result = format!("{bits} {} {}", hex(writer.get_span()), status_code(&status));
            config_observation(&mut result, decoded[0]);
            write!(
                result,
                " {} {}",
                reader.total_bits_consumed(),
                status_code(&reader.close())
            )
            .unwrap();
            result
        }
        "uint-config-read" => {
            let log_alpha = number(args);
            let bytes = unhex(args.next().unwrap());
            let (offset, count) = (number(args), number(args));
            let mut reader = BitReader::new(&bytes);
            reader.skip_bits(offset);
            let mut configs = vec![HybridUintConfig::default(); count];
            let status = dec_ans::decode_uint_configs(log_alpha, &mut configs, &mut reader);
            let mut result = status_code(&status).to_string();
            for cfg in configs {
                config_observation(&mut result, cfg);
            }
            write!(
                result,
                " {} {}",
                reader.total_bits_consumed(),
                status_code(&reader.close())
            )
            .unwrap();
            result
        }
        "context-map" => {
            let size = number(args);
            let bytes = unhex(args.next().unwrap());
            let offset = number(args);
            let mut map = vec![0xA5; size];
            let mut num_htrees = 99;
            let mut reader = BitReader::new(&bytes);
            reader.skip_bits(offset);
            let status = decode_context_map(&mut map, &mut num_htrees, &mut reader);
            let mut result = format!("{} {num_htrees}", status_code(&status));
            for value in map {
                write!(result, " {value}").unwrap();
            }
            write!(
                result,
                " {} {}",
                reader.total_bits_consumed(),
                status_code(&reader.close())
            )
            .unwrap();
            result
        }
        "inverse-mtf" => {
            let mut bytes = unhex(args.next().unwrap());
            inverse_move_to_front_transform(&mut bytes);
            hex(&bytes)
        }
        "special-distance" => dec_ans::special_distance(number(args), number(args)).to_string(),
        _ => panic!("unknown ANS command: {kind}"),
    }
}

#[test]
fn hybrid_uint_configs_and_extreme_values_match_original() {
    let mut cases = Vec::new();
    let mut seed = 193;
    let mut values = vec![0, 1, u32::MAX];
    for shift in 0..32 {
        let value = 1u32 << shift;
        values.extend([value - 1, value, value.saturating_add(1)]);
    }
    for _ in 0..32 {
        values.push(next(&mut seed) as u32);
    }
    for e in 0..=15 {
        for m in 0..=e {
            for l in 0..=e - m {
                for &value in &values {
                    cases.push(format!(
                        "hybrid {e} {m} {l} {value} {}",
                        next(&mut seed) % 8
                    ));
                }
                for log_alpha in [5, 6, 7, 8, 15] {
                    if e <= log_alpha {
                        cases.push(format!(
                            "uint-config {log_alpha} {} {e} {m} {l}",
                            next(&mut seed) % 8
                        ));
                    }
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn malformed_hybrid_configs_histograms_and_context_maps_preserve_source_state() {
    let mut cases = Vec::new();
    let mut seed = 3942;
    for offset in 0..8 {
        for length in 0..24 {
            for _ in 0..8 {
                let bytes: Vec<_> = (0..length).map(|_| next(&mut seed) as u8).collect();
                let hex = hex(&bytes);
                for log_alpha in [5, 6, 7, 8, 15] {
                    cases.push(format!("uint-config-read {log_alpha} {hex} {offset} 3"));
                }
                for contexts in [1, 2, 3, 8, 18] {
                    cases.push(format!("ans-hist {contexts} {} {hex} {offset}", length % 2));
                    cases.push(format!("context-map {contexts} {hex} {offset}"));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn inverse_mtf_and_special_distances_match_source_helpers() {
    let mut cases = Vec::new();
    let mut seed = 31415;
    for multiplier in [0, 1, 2, 7, 255, 4096, 1 << 20] {
        for i in 0..120 {
            cases.push(format!("special-distance {i} {multiplier}"));
        }
    }
    for length in [0, 1, 2, 3, 4, 63, 64, 65, 255, 256, 257, 4096] {
        for trial in 0..8 {
            let bytes: Vec<_> = (0..length)
                .map(|i| match trial {
                    0 => 0,
                    1 => 1,
                    2 => 255,
                    3 => i as u8,
                    _ => next(&mut seed) as u8,
                })
                .collect();
            cases.push(format!("inverse-mtf {}", hex(&bytes)));
        }
    }
    compare(cases);
}

#[test]
fn compressed_context_maps_match_original_ans_and_prefix_streams() {
    let mut fixtures = Vec::new();
    let mut sizes = Vec::new();
    let mut seed = 7929;
    for prefix in [0, 1] {
        for mtf in [0, 1] {
            for size in [1, 2, 3, 18, 256, 1024] {
                for trial in 0..4 {
                    let mut line = format!("context-fixture {prefix} {mtf} {size}");
                    for i in 0..size {
                        let value = match trial {
                            0 => 0,
                            1 => i % 256,
                            2 => (next(&mut seed) % 8) as usize,
                            _ => {
                                if i == 0 {
                                    256
                                } else {
                                    0
                                }
                            }
                        };
                        write!(line, " {value}").unwrap();
                    }
                    fixtures.push(line);
                    sizes.push(size);
                }
            }
        }
    }
    let originals = original_observations(&fixtures);
    let mut cases = Vec::new();
    for (size, original) in sizes.into_iter().zip(originals) {
        let stream = original.split_whitespace().nth(3).unwrap();
        cases.push(format!("context-map {size} {stream} 0"));
    }
    compare(cases);
}

#[test]
fn original_encoder_payloads_and_streams_match_rust_tokens_and_decoder() {
    let mut fixtures = Vec::new();
    let mut seed = 9797;
    for prefix in [0, 1] {
        for offset in 0..8 {
            for trial in 0..12 {
                let cfg = if trial % 2 == 0 {
                    HybridUintConfig::new(0, 0, 0)
                } else {
                    HybridUintConfig::default()
                };
                let lz77 = trial >= 8;
                let mut tokens = Vec::new();
                let count = if trial == 0 { 0 } else { 8 + trial * 17 };
                let mut decoded = 0;
                for i in 0..count {
                    let value = match trial % 4 {
                        0 => (i % 7) as u32,
                        1 => 3,
                        2 => next(&mut seed) as u32,
                        _ => (next(&mut seed) % 256) as u32,
                    };
                    let value = if lz77 { value % 16 } else { value };
                    tokens.push((0, value, 0));
                    decoded += 1;
                    if lz77 && i % 8 == 0 {
                        let length = (next(&mut seed) % 30) as u32;
                        tokens.push((0, length, 1));
                        tokens.push((1, (i % 7) as u32, 0));
                        decoded += length as usize + 3;
                    }
                }
                let mut line = format!(
                    "ans-fixture {prefix} 8 {} {} {} {} {offset} {}",
                    u32::from(lz77),
                    cfg.split_exponent,
                    cfg.msb_in_token,
                    cfg.lsb_in_token,
                    tokens.len()
                );
                for &(ctx, value, length) in &tokens {
                    write!(line, " {ctx} {value} {length}").unwrap();
                }
                fixtures.push((line, cfg, lz77, offset, tokens, decoded));
            }
        }
    }
    let requests: Vec<_> = fixtures.iter().map(|f| f.0.clone()).collect();
    let originals = original_observations(&requests);
    let mut cases = Vec::new();
    for ((request, cfg, lz77, offset, tokens, decoded), original) in
        fixtures.into_iter().zip(originals)
    {
        let mut args = original.split_whitespace();
        let _header_bits: usize = number(&mut args);
        let payload_bits: usize = number(&mut args);
        let payload = args.next().unwrap();
        let stream = unhex(args.next().unwrap());
        let num_histograms = number(&mut args);
        let mut codes = EntropyEncodingData {
            use_prefix_code: request.starts_with("ans-fixture 1"),
            log_alpha_size: 8,
            uint_config: vec![cfg],
            context_map: vec![0],
            ..EntropyEncodingData::default()
        };
        codes.lz77.enabled = lz77;
        codes.lz77.min_symbol = 224;
        codes.lz77.min_length = 3;
        if lz77 {
            codes.uint_config.push(HybridUintConfig::new(0, 0, 0));
            codes.context_map.push(1);
        }
        for _ in 0..num_histograms {
            let size: usize = number(&mut args);
            let mut histogram = Vec::new();
            for _ in 0..size {
                let freq = number(&mut args);
                let ifreq = number(&mut args);
                let depth = number(&mut args);
                let bits = number(&mut args);
                let length: usize = number(&mut args);
                let reverse_map = (0..length).map(|_| number(&mut args)).collect();
                histogram.push(ANSEncSymbolInfo {
                    freq,
                    ifreq,
                    depth,
                    bits,
                    reverse_map,
                });
            }
            codes.encoding_info.push(histogram);
        }
        assert!(args.next().is_none());
        let tokens: Vec<_> = tokens
            .into_iter()
            .map(|(ctx, value, length)| {
                let mut t = Token::new(ctx, value);
                t.set_is_lz77_length(length != 0);
                t
            })
            .collect();
        let mut writer = BitWriter::new();
        super::prefix(&mut writer, offset);
        write_tokens_with_allotment(&tokens, &codes, 0, &mut writer, LayerType::Header, None)
            .unwrap();
        assert_eq!(writer.bits_written(), payload_bits, "{request}");
        pad(&mut writer);
        assert_eq!(hex(writer.get_span()), payload, "{request}");
        cases.push(format!(
            "ans-stream 1 0 {} {offset} {decoded} 0",
            hex(&stream)
        ));
        if !lz77 {
            cases.push(format!(
                "ans-nolz 1 0 {} {offset} {decoded} 0",
                hex(&stream)
            ));
        }
        cases.push(format!("ans-single 1 0 {} {offset} 20 0", hex(&stream)));
        if decoded >= 19 {
            cases.push(format!(
                "ans-checkpoint 1 0 {} {offset} {decoded} 0 2 17",
                hex(&stream)
            ));
        }
        for length in 0..=stream.len().min(16) {
            cases.push(format!(
                "ans-stream 1 0 {} {offset} 16 1",
                hex(&stream[..length])
            ));
        }
        for _ in 0..4 {
            let mut damaged = stream.clone();
            let i = next(&mut seed) as usize % (damaged.len() * 8);
            damaged[i / 8] ^= 1 << (i % 8);
            cases.push(format!("ans-stream 1 0 {} {offset} 16 16", hex(&damaged)));
        }
    }
    println!("{} C++/Rust differential cases matched", requests.len());
    compare(cases);
}

#[test]
fn lz77_checkpoint_window_wrap_and_prefix_rle_shortcut_match_source() {
    let mut fixture_requests = vec![
        "ans-wrap-fixture 0".to_owned(),
        "ans-wrap-fixture 1".to_owned(),
        "ans-fixture 1 8 1 4 2 0 0 6 0 3 0 0 5 1 1 1 0 0 4 0 0 8 1 1 1 0".to_owned(),
    ];
    for value in [u32::MAX - 2, u32::MAX - 1, u32::MAX] {
        fixture_requests.push(format!("ans-fixture 1 8 1 4 2 0 0 2 0 {value} 1 1 0 0"));
    }
    let originals = original_observations(&fixture_requests);
    let mut cases = Vec::new();
    for original in &originals[..2] {
        let stream = original.split_whitespace().nth(3).unwrap();
        let at = dec_ans::WINDOW_SIZE + 10;
        cases.push(format!(
            "ans-checkpoint 1 0 {stream} 0 {} 0 {at} 510",
            at + 510
        ));
    }
    let rle_stream = originals[2].split_whitespace().nth(3).unwrap();
    cases.push(format!("ans-rle 1 0 {rle_stream} 0 4 1"));
    for original in &originals[3..] {
        let stream = original.split_whitespace().nth(3).unwrap();
        cases.push(format!("ans-stream 1 0 {stream} 0 1 0"));
    }
    compare(cases);
}
