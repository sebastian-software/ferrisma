// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe complete encoder entry points and cross-decode their output.

use super::*;
use jxl::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use jxl::jxl::enc_ans::{
    EntropyEncodingData, build_and_encode_histograms, encode_histograms,
    write_tokens_with_allotment,
};
use jxl::jxl::enc_ans_params::{
    ANSHistogramStrategy, ClusteringType, HistogramParams, HybridUintMethod,
};
use jxl::jxl::enc_context_map::encode_context_map;

pub(super) fn handles(kind: &str) -> bool {
    matches!(
        kind,
        "entropy-build" | "context-write" | "entropy-read" | "entropy-batches"
    )
}

fn config(out: &mut String, cfg: jxl::jxl::dec_ans::HybridUintConfig) {
    write!(
        out,
        " {},{},{},{}",
        cfg.split_exponent, cfg.split_token, cfg.msb_in_token, cfg.lsb_in_token
    )
    .unwrap();
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "entropy-batches" {
        let contexts = number(args);
        let batches: usize = number(args);
        let force: u32 = number(args);
        let fixed: u32 = number(args);
        let mut codes = EntropyEncodingData::default();
        let mut params = HistogramParams {
            streaming_mode: true,
            force_huffman: force != 0,
            lz77_method: jxl::jxl::enc_ans_params::LZ77Method::None,
            ..HistogramParams::default()
        };
        let mut streams = Vec::new();
        let mut offsets = Vec::new();
        let mut aux = AuxOut::default();
        let mut out = String::new();
        for batch in 0..batches {
            params.initialize_global_state = batch == 0;
            params.add_fixed_histograms = fixed != 0 && batch == 0;
            let mut tokens = lz77::token_streams(args, false);
            offsets.push(codes.context_map.len());
            let cost = build_and_encode_histograms(
                &params,
                contexts,
                &mut tokens,
                &mut codes,
                None,
                LayerType::Ac,
                Some(&mut aux),
            )
            .unwrap();
            write!(out, "{cost} {} ", codes.encoding_info.len()).unwrap();
            streams.push(tokens.remove(0));
        }
        let mut writer = BitWriter::new();
        let mut status = encode_histograms(&mut codes, &mut writer, LayerType::Ac, Some(&mut aux));
        for (stream, &offset) in streams.iter().zip(&offsets) {
            if status.is_err() {
                break;
            }
            status = write_tokens_with_allotment(
                stream,
                &codes,
                offset,
                &mut writer,
                LayerType::Ac,
                Some(&mut aux),
            );
        }
        write!(out, "{} {}", status_code(&status), writer.bits_written()).unwrap();
        pad(&mut writer);
        write!(
            out,
            " {} {}",
            hex(writer.get_span()),
            codes.context_map.len()
        )
        .unwrap();
        for c in &codes.context_map {
            write!(out, " {c}").unwrap();
        }
        let a = aux.layer(LayerType::Ac);
        write!(
            out,
            " {} {} {} {} {}",
            a.num_clustered_histograms,
            a.extra_bits,
            a.histogram_bits,
            a.total_bits,
            a.clustered_entropy.to_bits()
        )
        .unwrap();
        return out;
    }
    if kind == "entropy-read" {
        let contexts = number(args);
        let width = number(args);
        let offset = number(args);
        let bytes = unhex(args.next().unwrap());
        let streams = lz77::token_streams(args, false);
        let mut br = BitReader::new(&bytes);
        br.skip_bits(offset);
        let mut code = ANSCode::default();
        let mut map = Vec::new();
        let status = decode_histograms(&mut br, contexts, &mut code, &mut map, false);
        let mut out = format!("{} {}", status_code(&status), br.total_bits_consumed());
        if status.is_ok() {
            for stream in streams {
                let mut reader = ANSSymbolReader::create(&code, &mut br, width).unwrap();
                for token in stream {
                    write!(
                        out,
                        " {}",
                        reader.read_hybrid_uint(token.context() as usize, &mut br, &map)
                    )
                    .unwrap();
                }
                write!(
                    out,
                    " {} {}",
                    u32::from(reader.check_ans_final_state()),
                    br.total_bits_consumed()
                )
                .unwrap();
            }
        }
        write!(out, " {}", status_code(&br.close())).unwrap();
        return out;
    }
    if kind == "context-write" {
        let histograms = number(args);
        let offset = number(args);
        let size: usize = number(args);
        let map: Vec<u8> = (0..size).map(|_| number(args)).collect();
        let mut writer = BitWriter::new();
        prefix(&mut writer, offset);
        let mut aux = AuxOut::default();
        let status = writer.with_max_bits(
            3 + size * 24 + 32768,
            LayerType::Ac,
            Some(&mut aux),
            |w, aux| encode_context_map(&map, histograms, w, LayerType::Ac, aux),
            false,
        );
        let bits = writer.bits_written();
        pad(&mut writer);
        let a = aux.layer(LayerType::Ac);
        return format!(
            "{} {bits} {} {} {} {} {}",
            status_code(&status),
            hex(writer.get_span()),
            a.num_clustered_histograms,
            a.extra_bits,
            a.histogram_bits,
            a.total_bits
        );
    }
    let mode: u32 = number(args);
    let streaming: u32 = number(args);
    let force: u32 = number(args);
    let cluster: u32 = number(args);
    let uint: u32 = number(args);
    let lz = lz77::method(number(args));
    let strategy: u32 = number(args);
    let missing: u32 = number(args);
    let fixed: u32 = number(args);
    let contexts = number(args);
    let width = number(args);
    let offset = number(args);
    let mut tokens = lz77::token_streams(args, false);
    let params = HistogramParams {
        streaming_mode: streaming != 0,
        force_huffman: force != 0,
        clustering: match cluster {
            0 => ClusteringType::Fastest,
            1 => ClusteringType::Fast,
            _ => ClusteringType::Best,
        },
        uint_method: match uint {
            0 => HybridUintMethod::None,
            1 => HybridUintMethod::Uint000,
            2 => HybridUintMethod::Fast,
            3 => HybridUintMethod::ContextMap,
            _ => HybridUintMethod::Best,
        },
        lz77_method: lz,
        ans_histogram_strategy: match strategy {
            0 => ANSHistogramStrategy::Fast,
            1 => ANSHistogramStrategy::Approximate,
            _ => ANSHistogramStrategy::Precise,
        },
        add_missing_symbols: missing != 0,
        add_fixed_histograms: fixed != 0,
        image_widths: vec![width; tokens.len()],
        ..HistogramParams::default()
    };
    let mut codes = EntropyEncodingData::default();
    let mut aux = AuxOut::default();
    let mut writer = BitWriter::new();
    prefix(&mut writer, offset);
    let cost = build_and_encode_histograms(
        &params,
        contexts,
        &mut tokens,
        &mut codes,
        if mode == 1 { Some(&mut writer) } else { None },
        LayerType::Ac,
        Some(&mut aux),
    );
    let mut status = cost.map(|_| ());
    let mut out = format!(
        "{} {}",
        status_code(&status),
        cost.map_or_else(|_| "-".to_owned(), |v| v.to_string())
    );
    if status.is_ok() && mode == 2 {
        status = encode_histograms(&mut codes, &mut writer, LayerType::Ac, Some(&mut aux));
    }
    if status.is_ok() && mode != 0 {
        for stream in &tokens {
            status = write_tokens_with_allotment(
                stream,
                &codes,
                0,
                &mut writer,
                LayerType::Ac,
                Some(&mut aux),
            );
            if status.is_err() {
                break;
            }
        }
    }
    write!(out, " {} {}", status_code(&status), writer.bits_written()).unwrap();
    pad(&mut writer);
    write!(
        out,
        " {} {} {} {} {} {} {}",
        hex(writer.get_span()),
        u32::from(codes.use_prefix_code),
        codes.log_alpha_size,
        u32::from(codes.lz77.enabled),
        codes.lz77.min_symbol,
        codes.lz77.min_length,
        codes.lz77.nonserialized_distance_context
    )
    .unwrap();
    config(&mut out, codes.lz77.length_uint_config);
    write!(out, " {}", codes.uint_config.len()).unwrap();
    for &cfg in &codes.uint_config {
        config(&mut out, cfg);
    }
    write!(out, " {}", codes.context_map.len()).unwrap();
    for c in &codes.context_map {
        write!(out, " {c}").unwrap();
    }
    write!(out, " {}", codes.encoding_info.len()).unwrap();
    for histo in &codes.encoding_info {
        write!(out, " {}", histo.len()).unwrap();
        for info in histo {
            write!(out, " {},{},{}", info.freq, info.depth, info.bits).unwrap();
        }
    }
    write!(out, " {}", codes.encoded_histograms.len()).unwrap();
    for histo in &mut codes.encoded_histograms {
        write!(out, " {}", histo.bits_written()).unwrap();
        pad(histo);
        write!(out, " {}", hex(histo.get_span())).unwrap();
    }
    let a = aux.layer(LayerType::Ac);
    write!(
        out,
        " {} {} {} {} {}",
        a.num_clustered_histograms,
        a.extra_bits,
        a.histogram_bits,
        a.total_bits,
        a.clustered_entropy.to_bits()
    )
    .unwrap();
    write!(out, " {}", tokens.len()).unwrap();
    for stream in &tokens {
        write!(out, " {}", stream.len()).unwrap();
        for token in stream {
            write!(
                out,
                " {},{},{}",
                token.context(),
                token.value,
                u32::from(token.is_lz77_length())
            )
            .unwrap();
        }
    }
    out
}

fn streams(size: usize, contexts: usize, trial: usize, seed: &mut u64) -> String {
    let mut out = "2".to_owned();
    for stream in 0..2 {
        write!(out, " {size}").unwrap();
        for i in 0..size {
            let context = (i + stream) % contexts;
            let value = match trial {
                0 => 0,
                1 => (i % 13) as u32,
                2 => ((i / 17) % 13) as u32,
                3 => (next(seed) % 8) as u32,
                4 => next(seed) as u32,
                _ => (context * 257 + (i % 3)) as u32,
            };
            write!(out, " {context} {value}").unwrap();
        }
    }
    out
}

#[test]
fn full_entropy_encoder_matches_source_costs_bytes_codes_and_accounting() {
    let mut cases = Vec::new();
    let mut seed = 731;
    for contexts in [1, 3, 17] {
        for size in [0, 1, 7, 99, 100, 257, 1024] {
            for trial in 0..6 {
                let input = streams(size, contexts, trial, &mut seed);
                for force in [0, 1] {
                    for mode in [0, 1] {
                        cases.push(format!("entropy-build {mode} 0 {force} {} {} {} {} 0 0 {contexts} 7 {} {input}",trial%3,(trial+size)%5,[0,1,14][trial%3],(trial+contexts)%3,size%8));
                    }
                }
            }
        }
    }
    // Explicitly cover every uint policy and clustering policy independently.
    let input = streams(257, 3, 4, &mut seed);
    for uint in 0..5 {
        for cluster in 0..3 {
            for strategy in 0..3 {
                cases.push(format!(
                    "entropy-build 1 0 0 {cluster} {uint} 0 {strategy} 0 0 3 0 3 {input}"
                ));
            }
        }
    }
    // The source rejects adding a fixed prior histogram to the single-context
    // builder. Preserve its returned error and partially populated codes.
    let input = streams(257, 1, 4, &mut seed);
    for mode in [0, 1] {
        for streaming in [0, 1] {
            for force in [0, 1] {
                cases.push(format!(
                    "entropy-build {mode} {streaming} {force} 1 0 0 1 0 1 1 0 5 {input}"
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn streaming_entropy_and_fixed_histograms_match_original() {
    let mut cases = Vec::new();
    let mut seed = 910;
    for contexts in [3, 17] {
        for trial in 0..6 {
            for force in [0, 1] {
                for fixed in [0, 1] {
                    for missing in [0, 1] {
                        let input = streams(257, contexts, trial, &mut seed);
                        cases.push(format!("entropy-build 2 1 {force} 1 {} {} 1 {missing} {fixed} {contexts} 3 5 {input}",trial%5,if trial%2==0 { 0 } else { 1 }));
                    }
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn context_map_cost_choices_and_serialized_bytes_match_original() {
    let mut cases = Vec::new();
    let mut seed = 833;
    for histograms in [1, 2, 3, 7, 8, 17, 128] {
        for size in [1, 7, 31, 257, 1024] {
            for trial in 0..4 {
                let mut line = format!("context-write {histograms} {} {size}", size % 8);
                for i in 0..size {
                    let value = match trial {
                        0 => i % histograms,
                        1 => (i / 31) % histograms,
                        2 => (next(&mut seed) as usize) % histograms,
                        _ => 0,
                    };
                    write!(line, " {value}").unwrap();
                }
                cases.push(line);
            }
        }
    }
    compare(cases);
}

#[test]
fn streaming_batches_keep_previous_histograms_and_context_offsets() {
    let mut cases = Vec::new();
    let mut seed = 511;
    for contexts in [3, 17] {
        for batches in [2, 3, 7] {
            for force in [0, 1] {
                for fixed in [0, 1] {
                    let mut line = format!("entropy-batches {contexts} {batches} {force} {fixed}");
                    for batch in 0..batches {
                        let input = streams(257, contexts, 1 + batch % 5, &mut seed);
                        write!(line, " {input}").unwrap();
                    }
                    cases.push(line);
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn complete_entropy_streams_cross_decode_without_adapter_framing() {
    let mut requests = Vec::new();
    let mut inputs = Vec::new();
    let mut seed = 1913;
    for contexts in [3, 17] {
        for size in [1, 7, 99, 100, 257, 1024] {
            for trial in 0..6 {
                let input = streams(size, contexts, trial, &mut seed);
                for streaming in [0, 1] {
                    for force in [0, 1] {
                        let mode = if streaming == 0 { 1 } else { 2 };
                        let width = [0, 1, 7][trial % 3];
                        let offset = size % 8;
                        requests.push(format!("entropy-build {mode} {streaming} {force} 2 {} {} 2 0 0 {contexts} {width} {offset} {input}",trial%5,[0,1,14][trial%3]));
                        inputs.push((contexts, width, offset, input.clone()));
                    }
                }
            }
        }
    }
    let cpp = original_observations(&requests);
    let mut read_requests = Vec::new();
    for ((request, cpp), (contexts, width, offset, input)) in requests.iter().zip(&cpp).zip(&inputs)
    {
        let rust = super::observe(request);
        assert_eq!(&rust, cpp, "request: {request}");
        let bytes = cpp.split_whitespace().nth(4).unwrap();
        read_requests.push(format!(
            "entropy-read {contexts} {width} {offset} {bytes} {input}"
        ));
    }
    println!("{} C++/Rust differential cases matched", requests.len());
    let cpp_read = original_observations(&read_requests);
    for ((request, cpp), (_, _, _, input)) in read_requests.iter().zip(&cpp_read).zip(&inputs) {
        let rust = super::observe(request);
        assert_eq!(&rust, cpp, "request: {request}");
        let mut decoded = rust.split_whitespace();
        assert_eq!(number::<u32>(&mut decoded), 0);
        let _: usize = number(&mut decoded);
        let mut args = input.split_whitespace();
        for stream in lz77::token_streams(&mut args, false) {
            for token in stream {
                assert_eq!(
                    number::<u32>(&mut decoded),
                    token.value,
                    "request: {request}"
                );
            }
            assert_eq!(number::<u32>(&mut decoded), 1, "final entropy state");
            let _: usize = number(&mut decoded);
        }
        assert_eq!(number::<u32>(&mut decoded), 0, "reader close");
        assert!(decoded.next().is_none());
    }
    println!(
        "{} C++/Rust differential cases matched",
        read_requests.len()
    );
}
