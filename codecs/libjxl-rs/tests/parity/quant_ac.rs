// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::quant_weights::{
    AC_STRATEGY_TO_QUANT_TABLE_MAP, DctQuantWeightParams, DequantMatrices, QuantEncoding,
    QuantEncodingData,
};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("qac-")
}
fn state(out: &mut String, m: &DequantMatrices, strategy: u8) {
    write!(
        out,
        " {} {}",
        m.computed_mask,
        u8::from(m.have_table_storage())
    )
    .unwrap();
    if m.computed_mask & (1 << strategy) == 0 {
        return;
    }
    for c in 0..3 {
        for (&value, &inverse) in m
            .matrix(jxl::jxl::ac_strategy::AcStrategyType(strategy as u32), c)
            .iter()
            .zip(m.inv_matrix(jxl::jxl::ac_strategy::AcStrategyType(strategy as u32), c))
        {
            write!(out, " {},{}", value.to_bits(), inverse.to_bits()).unwrap();
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    if kind == "qac-library" {
        out.push('L');
        for q in DequantMatrices::library() {
            quant_storage::encoding(&mut out, q);
        }
    } else if kind == "qac-compute" {
        let strategy: u8 = number(args);
        let q = quant_storage::read(args).0;
        let mut encodings = vec![QuantEncoding::library::<0>(); 17];
        encodings[AC_STRATEGY_TO_QUANT_TABLE_MAP[strategy as usize] as usize] = q;
        let mut m = DequantMatrices::default();
        m.set_encodings(&encodings);
        let n = number(args);
        out.push('C');
        for _ in 0..n {
            let status = m.ensure_computed(number(args));
            write!(out, " {}", status_code(&status) as i32).unwrap();
            state(&mut out, &m, strategy);
        }
    } else {
        let strategy: u8 = number(args);
        let seed = number(args);
        let offset = number(args);
        let bytes = unhex(args.next().unwrap());
        let mut m = DequantMatrices::default();
        if seed != 0 {
            m.ensure_computed(seed).unwrap();
        }
        let mut reader = BitReader::new(&bytes);
        reader.skip_bits(offset);
        let mut metadata = jxl::jxl::image_metadata::CodecMetadata::default();
        metadata.size.set(8, 8).unwrap();
        let mut header = jxl::jxl::frame_header::FrameHeader::new(Some(&metadata));
        header.encoding = jxl::jxl::frame_header::FrameEncoding::VAR_DCT;
        let mut decoder = jxl::jxl::dec_modular::ModularFrameDecoder::new();
        decoder.init(&header.to_frame_dimensions());
        if kind == "qac-decode-context" {
            let global = unhex(args.next().unwrap());
            let mut gr = BitReader::new(&global);
            let result = decoder.decode_global_info(&mut gr, &header, false);
            let closed = gr.close();
            result.unwrap();
            closed.unwrap();
        }
        let status = m.decode(
            &mut reader,
            (kind == "qac-decode-context").then_some(&decoder),
        );
        let bits = reader.total_bits_consumed();
        let closed = reader.close();
        write!(
            out,
            "D {} {bits} {}",
            status_code(&status) as i32,
            status_code(&closed) as i32
        )
        .unwrap();
        for q in m.encodings() {
            quant_storage::encoding(&mut out, q);
        }
        state(&mut out, &m, strategy);
        if status.is_ok() {
            let computed = m.ensure_computed(1 << strategy);
            write!(out, " C {}", status_code(&computed) as i32).unwrap();
            state(&mut out, &m, strategy);
        }
    }
    out
}
fn recipe(q: &QuantEncoding) -> String {
    let mut out = (q.mode() as u32).to_string();
    for p in [&q.dct_params, &q.dct_params_afv_4x4] {
        write!(out, " 1 {}", p.num_distance_bands).unwrap();
        for row in p.distance_bands {
            for v in row {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
    }
    let mut weights = [0.0f32; 27];
    match q.data() {
        QuantEncodingData::Identity(w) => {
            for c in 0..3 {
                weights[c * 3..c * 3 + 3].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Dct2(w) => {
            for c in 0..3 {
                weights[c * 6..c * 6 + 6].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Dct4(w) => {
            for c in 0..3 {
                weights[c * 2..c * 2 + 2].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Dct4X8(w) => weights[..3].copy_from_slice(w),
        QuantEncodingData::Afv(w) => {
            for c in 0..3 {
                weights[c * 9..c * 9 + 9].copy_from_slice(&w[c]);
            }
        }
        QuantEncodingData::Raw { .. } => {}
    }
    for v in weights {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    if let QuantEncodingData::Raw {
        qtable: Some(table),
        ..
    } = q.data()
    {
        write!(out, " 0 {}", table.len()).unwrap();
        for v in table {
            write!(out, " {v}").unwrap();
        }
    } else {
        out.push_str(" 0 0");
    }
    out
}
#[test]
fn ac_quantization_library_all_strategies_and_shared_kind_masks() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = vec!["qac-library".into()];
    for strategy in 0..27 {
        cases.push(format!(
            "qac-compute {strategy} {} 3 0 {} {}",
            recipe(&QuantEncoding::library::<0>()),
            1u32 << strategy,
            1u32 << strategy
        ));
    }
    for (a, b) in [
        (6, 7),
        (8, 9),
        (10, 11),
        (12, 13),
        (14, 17),
        (19, 20),
        (22, 23),
        (25, 26),
    ] {
        cases.push(format!(
            "qac-compute {a} {} 3 {} {} {}",
            recipe(&QuantEncoding::library::<0>()),
            1u32 << b,
            1u32 << a,
            (1u32 << a) | (1u32 << b)
        ));
    }
    compare(cases);
}
#[test]
fn ac_quantization_custom_modes_ranges_and_partial_failure_masks() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let bands = DctQuantWeightParams::from_bands(&[
        [128.0, 0.25, -0.5],
        [256.0, -0.25, 0.5],
        [64.0, 1.0, -1.0],
    ]);
    let encodings = [
        QuantEncoding::identity(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]),
        QuantEncoding::dct2(&[[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 3]),
        QuantEncoding::dct4(&bands, &[[0.5, 2.0], [2.0, 0.5], [1.0, 1.0]]),
        QuantEncoding::dct4x8(&bands, &[0.5, 1.0, 2.0]),
        QuantEncoding::afv(
            &bands,
            &bands,
            &[[1.0, 2.0, 3.0, 4.0, 5.0, 10.0, 0.5, -0.5, 1.0]; 3],
        ),
        QuantEncoding::dct(&bands),
    ];
    for strategy in [0, 1, 2, 3, 4, 6, 8, 10, 12, 14, 18, 19, 21, 22, 24, 25] {
        for q in &encodings {
            if strategy >= 4 && strategy != 12 && strategy != 14 && q.mode() as u32 != 6 {
                continue;
            }
            // AFV, ID and small DCT modes have a source caller precondition of
            // a one-block kind; ComputeQuantTable rejects incompatible sizes.
            if strategy == 14 && q.mode() as u32 != 5 && q.mode() as u32 != 6 {
                continue;
            }
            cases.push(format!(
                "qac-compute {strategy} {} 1 {}",
                recipe(q),
                1u32 << strategy
            ));
        }
    }
    for value in [
        0.0,
        -0.0,
        -1.0,
        1e-9,
        1e-8,
        1e8,
        f32::INFINITY,
        f32::from_bits(0x7fc12345),
    ] {
        let q = QuantEncoding::identity(&[[value, 2.0, 3.0]; 3]);
        cases.push(format!("qac-compute 1 {} 2 1 3", recipe(&q)));
    }
    for len in [0, 1, 191, 192, 193] {
        let q = QuantEncoding::raw(&vec![3; len], 0);
        cases.push(format!("qac-compute 0 {} 1 1", recipe(&q)));
    }
    for value in [0, -1, 1, i32::MAX] {
        let mut table = vec![2; 192];
        table[77] = value;
        let q = QuantEncoding::raw(&table, 0);
        cases.push(format!("qac-compute 0 {} 1 1", recipe(&q)));
    }
    for channel in 0..3 {
        let mut bad = bands;
        bad.distance_bands[channel][0] = 0.0;
        cases.push(format!(
            "qac-compute 0 {} 1 1",
            recipe(&QuantEncoding::dct(&bad))
        ));
    }
    for count in [1, 2, 4, 8, 16, 17] {
        let mut bands = [[0.0f32; 17]; 3];
        for (c, row) in bands.iter_mut().enumerate() {
            row[0] = 64.0 * (c + 1) as f32;
            for (i, value) in row.iter_mut().enumerate().take(count).skip(1) {
                *value = (i as f32 - 8.0) * 0.125;
            }
        }
        let params = DctQuantWeightParams::new(bands, count);
        for strategy in [0, 6, 18] {
            cases.push(format!(
                "qac-compute {strategy} {} 1 {}",
                recipe(&QuantEncoding::dct(&params)),
                1u32 << strategy
            ));
        }
    }
    compare(cases);
}
fn encoded_tables(mode: u32, variant: u32, prefix: usize, selected_kind: usize) -> Vec<u8> {
    let mut w = BitWriter::new();
    if prefix > 0 {
        w.with_max_bits(
            prefix,
            LayerType::Header,
            None,
            |w, _| {
                w.write(prefix, 0);
                Ok(())
            },
            false,
        )
        .unwrap();
    }
    w.with_max_bits(
        16000,
        LayerType::Header,
        None,
        |w, _| {
            w.write(1, 0);
            for kind in 0..17 {
                let selected = if kind == selected_kind { mode } else { 0 };
                w.write(3, selected as u64);
                if selected == 0 {
                    continue;
                }
                let n = match selected {
                    1 => 3,
                    2 => 6,
                    3 => 2,
                    4 => 1,
                    5 => 9,
                    _ => 0,
                };
                for c in 0..3 {
                    for i in 0..n {
                        let bits = if variant == 1 && c == 1 && i == 0 {
                            0
                        } else if variant == 2 && c == 1 && i == 0 {
                            0x7c00
                        } else {
                            0x3c00
                        };
                        w.write(16, bits);
                    }
                }
                if (3..=6).contains(&selected) {
                    for _ in 0..if selected == 5 { 2 } else { 1 } {
                        w.write(4, 2);
                        for c in 0..3 {
                            for i in 0..3 {
                                let bits = if variant == 3 && c == 1 && i == 0 {
                                    0
                                } else {
                                    0x3c00
                                };
                                w.write(16, bits);
                            }
                        }
                    }
                }
                if selected == 7 {
                    w.write(16, if variant == 1 { 0 } else { 0x7c00 });
                }
            }
            w.zero_pad_to_byte();
            Ok(())
        },
        false,
    )
    .unwrap();
    w.get_span().to_vec()
}
#[test]
fn ac_quantization_table_decoding_unaligned_truncated_and_partial_state() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for mode in 0..8 {
        for variant in 0..4 {
            for prefix in [0, 3, 7] {
                let bytes = encoded_tables(mode, variant, prefix, 0);
                cases.push(format!("qac-decode 0 1 {prefix} {}", hex(&bytes)));
                for len in [
                    0,
                    1,
                    2,
                    3,
                    7,
                    bytes.len() / 2,
                    bytes.len().saturating_sub(1),
                ] {
                    cases.push(format!(
                        "qac-decode 0 1 {prefix} {}",
                        hex(&bytes[..len.min(bytes.len())])
                    ));
                }
            }
        }
    }
    for kind in [4, 6, 16] {
        for mode in 1..=5 {
            let bytes = encoded_tables(mode, 0, 3, kind);
            cases.push(format!("qac-decode 0 1 3 {}", hex(&bytes)));
        }
    }
    cases.push("qac-decode 1 2 0 01".into());
    compare(cases);
}

#[test]
fn ac_raw_quantization_tables_decode_original_local_and_shared_modular_streams() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut sources = Vec::new();
    let mut settings = Vec::new();
    for strategy in [0, 4, 6, 12, 14, 19] {
        for variant in 0..4 {
            for shared in [false, true] {
                for prefix in [0, 5] {
                    sources.push(format!(
                        "qac-source {strategy} {variant} {} {prefix}",
                        u8::from(shared)
                    ));
                    settings.push((strategy, shared, prefix));
                }
            }
        }
    }
    let mut cases = Vec::new();
    for ((strategy, shared, prefix), output) in
        settings.into_iter().zip(original_observations(&sources))
    {
        let mut fields = output.split_whitespace();
        let global = fields.next().unwrap();
        let data = unhex(fields.next().unwrap());
        let mut lengths = vec![
            0,
            1,
            2,
            data.len() / 2,
            data.len().saturating_sub(1),
            data.len(),
        ];
        lengths.sort_unstable();
        lengths.dedup();
        for len in lengths {
            let command = if shared {
                "qac-decode-context"
            } else {
                "qac-decode"
            };
            let mut case = format!("{command} {strategy} 1 {prefix} {}", hex(&data[..len]));
            if shared {
                write!(case, " {global}").unwrap();
            }
            cases.push(case);
        }
    }
    compare(cases);
}

#[test]
fn ac_quantization_single_band_and_afv_direct_nan_words_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    // A single distance band bypasses FastPowf. Multi-band NaN interpolation
    // would violate the original FastLog2f negative/NaN input precondition.
    let pairs = [
        (0x7fc1_2345, 0x7fc2_3456),
        (0xffc2_3456, 0x7fc1_2345),
        (0x7fa1_2345, 0xffc2_3456),
        (0xffc2_3456, 0x7fa1_2345),
    ];
    let mut cases = Vec::new();
    for (seed, multiplier) in pairs {
        let seed = f32::from_bits(seed);
        let multiplier = f32::from_bits(multiplier);
        let bands = DctQuantWeightParams::from_bands(&[[seed]; 3]);
        for q in [
            QuantEncoding::dct4(&bands, &[[multiplier, multiplier]; 3]),
            QuantEncoding::dct4x8(&bands, &[multiplier; 3]),
        ] {
            cases.push(format!("qac-compute 0 {} 1 1", recipe(&q)));
        }
    }
    for (seed, direct) in [pairs[0], pairs[2]] {
        let seed = f32::from_bits(seed);
        let direct = f32::from_bits(direct);
        let bands = DctQuantWeightParams::from_bands(&[[seed]; 3]);
        let q = QuantEncoding::afv(
            &bands,
            &bands,
            &[[seed, direct, -seed, -direct, seed, 10.0, 0.0, 0.0, 0.0]; 3],
        );
        cases.push(format!("qac-compute 14 {} 1 {}", recipe(&q), 1u32 << 14));
    }
    compare(cases);
}
