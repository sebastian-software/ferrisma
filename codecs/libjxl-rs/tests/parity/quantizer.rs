// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_strategy::AcStrategyType;
use jxl::jxl::enc_fields::write_quantizer_params;
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::{
    BIAS_NUMERATOR, DEFAULT_QUANT_BIAS, GLOBAL_SCALE_DENOM, GLOBAL_SCALE_NUMERATOR, Quantizer,
    QuantizerParams, ZERO_BIAS_DEFAULT,
};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("qz-")
}
fn params(out: &mut String, p: &QuantizerParams) {
    write!(out, " P {} {}", p.global_scale, p.quant_dc).unwrap();
}
fn quant(out: &mut String, q: &Quantizer, d: &DequantMatrices, slots: usize) {
    write!(
        out,
        " Q {} {} {} {} {}",
        q.global_scale(),
        q.raw_dc(),
        q.scale().to_bits(),
        q.inv_global_scale().to_bits(),
        q.inv_quant_dc().to_bits()
    )
    .unwrap();
    for c in 0..slots {
        write!(
            out,
            " {} {}",
            q.mul_dc()[c].to_bits(),
            q.inv_mul_dc()[c].to_bits()
        )
        .unwrap();
    }
    for value in q.zero_bias() {
        write!(out, " {}", value.to_bits()).unwrap();
    }
    for c in 0..3 {
        write!(
            out,
            " {} {}",
            q.dc_step(d, c).to_bits(),
            q.inv_dc_step(d, c).to_bits()
        )
        .unwrap();
    }
    for n in [0, 1, -1, 64, i32::MIN, i32::MAX] {
        write!(out, " {}", q.inv_quant_ac(n).to_bits()).unwrap();
    }
}
fn read_dc(args: &mut std::str::SplitWhitespace<'_>) -> [f32; 3] {
    std::array::from_fn(|_| f32::from_bits(number(args)))
}
fn params_observe(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let prefix_bits = number(args);
    let count = number(args);
    let mut p = QuantizerParams::default();
    let mut writer = BitWriter::new();
    let mut aux = AuxOut::default();
    prefix(&mut writer, prefix_bits);
    let mut out = String::new();
    params(&mut out, &p);
    modular_frame_state::writer_output(&mut out, &writer, &aux);
    for _ in 0..count {
        let result = match args.next().unwrap() {
            "S" => {
                p.global_scale = number(args);
                p.quant_dc = number(args);
                Ok(())
            }
            "I" => {
                bundle::init(&mut p);
                Ok(())
            }
            "W" => {
                let layer = modular_frame_state::layer(number(args));
                let can = bundle::can_encode(&mut p);
                let (extension, total) = can.as_ref().copied().unwrap_or((0, 0));
                write!(
                    out,
                    " C {} {extension} {total}",
                    status_code(&can.map(|_| ()))
                )
                .unwrap();
                write_quantizer_params(&mut p, &mut writer, layer, Some(&mut aux))
            }
            "D" => {
                let offset = number(args);
                let bytes = unhex(args.next().unwrap());
                let mut reader = BitReader::new(&bytes);
                reader.skip_bits(offset);
                let result = bundle::read(&mut reader, &mut p);
                write!(
                    out,
                    " R {} {}",
                    reader.total_bits_consumed(),
                    status_code(&reader.close())
                )
                .unwrap();
                result
            }
            _ => panic!("quantizer parameter fixture action"),
        };
        write!(out, " A {}", status_code(&result)).unwrap();
        params(&mut out, &p);
        modular_frame_state::writer_output(&mut out, &writer, &aux);
    }
    out
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "qz-params" {
        return params_observe(args);
    }
    let mut out = String::new();
    if kind == "qz-constants" {
        write!(
            out,
            "{GLOBAL_SCALE_DENOM} {GLOBAL_SCALE_NUMERATOR} {} {}",
            Quantizer::QUANT_MAX,
            BIAS_NUMERATOR.to_bits()
        )
        .unwrap();
        for value in ZERO_BIAS_DEFAULT.into_iter().chain(DEFAULT_QUANT_BIAS) {
            write!(out, " {}", value.to_bits()).unwrap();
        }
        return out;
    }
    if kind == "qz-clamp" {
        let count = number(args);
        out.push('C');
        for _ in 0..count {
            write!(
                out,
                " {}",
                Quantizer::clamp_val(f32::from_bits(number(args)))
            )
            .unwrap();
        }
        return out;
    }
    if kind == "qz-matrix" {
        let strategy = AcStrategyType(number(args));
        let mut matrices = DequantMatrices::default();
        let quantizer = Quantizer::new(&matrices);
        let result = matrices.ensure_computed(1 << strategy.0);
        write!(out, "{}", status_code(&result)).unwrap();
        if result.is_ok() {
            for c in 0..3 {
                let m = quantizer.dequant_matrix(&matrices, strategy, c);
                let inv = quantizer.inv_dequant_matrix(&matrices, strategy, c);
                write!(
                    out,
                    " {} {} {}",
                    u32::from(std::ptr::eq(m, matrices.matrix(strategy, c))),
                    u32::from(std::ptr::eq(inv, matrices.inv_matrix(strategy, c))),
                    m.len()
                )
                .unwrap();
                for (&v, &i) in m.iter().zip(inv) {
                    write!(out, " {},{}", v.to_bits(), i.to_bits()).unwrap();
                }
            }
        }
        return out;
    }
    assert_eq!(kind, "qz-chain");
    let dc = number(args);
    let global = number(args);
    let mut matrices = DequantMatrices::default();
    matrices.set_dc_quant(&read_dc(args));
    let mut q = Quantizer::with_values(&matrices, dc, global);
    let prefix_bits = number(args);
    let count = number(args);
    let mut writer = BitWriter::new();
    let mut aux = AuxOut::default();
    prefix(&mut writer, prefix_bits);
    let mut slots = 3;
    quant(&mut out, &q, &matrices, slots);
    params(&mut out, &q.get_params());
    modular_frame_state::writer_output(&mut out, &writer, &aux);
    for _ in 0..count {
        let result = match args.next().unwrap() {
            action @ ("D" | "R") => {
                let offset = number(args);
                let bytes = if action == "D" {
                    unhex(args.next().unwrap())
                } else {
                    let mut copy = BitWriter::new();
                    copy.append_unaligned(&writer).unwrap();
                    pad(&mut copy);
                    copy.get_span().to_vec()
                };
                let mut reader = BitReader::new(&bytes);
                reader.skip_bits(offset);
                let result = q.decode(&matrices, &mut reader);
                write!(
                    out,
                    " R {} {}",
                    reader.total_bits_consumed(),
                    status_code(&reader.close())
                )
                .unwrap();
                result
            }
            "E" => write_quantizer_params(
                &mut q.get_params(),
                &mut writer,
                modular_frame_state::layer(number(args)),
                Some(&mut aux),
            ),
            "G" => {
                let scale = q.scale_global_scale(&matrices, f32::from_bits(number(args)));
                write!(out, " G {}", scale.to_bits()).unwrap();
                Ok(())
            }
            "S" => {
                matrices.set_dc_quant(&read_dc(args));
                Ok(())
            }
            "C" => {
                q.clear_dc_mul();
                slots = 4;
                Ok(())
            }
            _ => panic!("quantizer fixture action"),
        };
        write!(out, " A {}", status_code(&result)).unwrap();
        quant(&mut out, &q, &matrices, slots);
        params(&mut out, &q.get_params());
        modular_frame_state::writer_output(&mut out, &writer, &aux);
    }
    out
}
fn request(dc: i32, global: i32, matrix: [f32; 3], prefix: usize, actions: &[String]) -> String {
    format!(
        "qz-chain {dc} {global} {} {} {} {prefix} {} {}",
        matrix[0].to_bits(),
        matrix[1].to_bits(),
        matrix[2].to_bits(),
        actions.len(),
        actions.join(" ")
    )
}
fn encoded(global: u32, dc: u32, offset: usize) -> Vec<u8> {
    let mut writer = BitWriter::new();
    prefix(&mut writer, offset);
    write_quantizer_params(
        &mut QuantizerParams {
            global_scale: global,
            quant_dc: dc,
        },
        &mut writer,
        LayerType::Header,
        None,
    )
    .unwrap();
    pad(&mut writer);
    writer.get_span().to_vec()
}

#[test]
fn quantizer_constants_and_literal_min_max_clamping_match_the_original() {
    let mut cases = vec!["qz-constants".to_owned()];
    let mut words = vec![
        0, 0x80000000, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc23456,
    ];
    for value in [-1.0f32, 1.0, 2.0, 255.0, 256.0, 257.0] {
        words.extend([value.to_bits() - 1, value.to_bits(), value.to_bits() + 1]);
    }
    let mut seed = 0xa813_52fb_719d_60e4;
    // The original optimized AArch64 FMAXNM returns a quiet NaN for an sNaN,
    // making ClampVal's final float-to-int cast undefined (verified by UBSan).
    // Generate only the defined finite/infinity/quiet-NaN ClampVal domain.
    words.extend((0..4096).map(|_| next(&mut seed) as u32).filter(|word| {
        !(word & 0x7f80_0000 == 0x7f80_0000 && word & 0x007f_ffff != 0 && word & 0x0040_0000 == 0)
    }));
    for chunk in words.chunks(64) {
        let mut line = format!("qz-clamp {}", chunk.len());
        for word in chunk {
            write!(line, " {word}").unwrap();
        }
        cases.push(line);
    }
    compare(cases);
}

#[test]
fn quantizer_parameter_defaults_serialization_selector_boundaries_and_accounting_match_source() {
    let globals = [
        0u32,
        1,
        2048,
        2049,
        4096,
        4097,
        8192,
        8193,
        73728,
        73729,
        u32::MAX,
    ];
    let dcs = [0u32, 1, 16, 17, 32, 33, 256, 257, 65536, 65537, u32::MAX];
    let mut cases = Vec::new();
    for global in globals {
        for dc in dcs {
            for prefix in 0..8 {
                cases.push(format!(
                    "qz-params {prefix} 5 W 0 S {global} {dc} W 5 I W 14"
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn quantizer_bundle_partial_fields_and_owner_decode_commit_after_success_match_source() {
    let mut cases = Vec::new();
    let mut seed = 0x3902_a8fc_6b51_d7e4;
    for len in 0..8 {
        for trial in 0..160 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            let offset = trial % 8;
            cases.push(format!(
                "qz-params 0 5 S 54321 12345 D {offset} {} W 5 D 0 0000 W 0",
                hex(&bytes)
            ));
            cases.push(request(
                123,
                54321,
                [3.0, -7.0, 11.0],
                offset,
                &[
                    "C".to_owned(),
                    format!("D {offset} {}", hex(&bytes)),
                    "E 5".to_owned(),
                    "D 0 0000".to_owned(),
                    "E 0".to_owned(),
                ],
            ));
        }
    }
    for (global, dc) in [
        (1, 1),
        (2048, 16),
        (2049, 32),
        (4097, 256),
        (8193, 257),
        (73728, 65536),
    ] {
        for offset in 0..8 {
            let bytes = encoded(global, dc, offset);
            for len in 0..=bytes.len() {
                cases.push(request(
                    -7,
                    -1103,
                    [2.0, 4.0, 8.0],
                    offset,
                    &[
                        "C".to_owned(),
                        format!("D {offset} {}", hex(&bytes[..len])),
                        "E 14".to_owned(),
                    ],
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn quantizer_original_writer_decode_roundtrips_reuse_the_matrix_owner_and_cleared_lane() {
    let mut cases = Vec::new();
    let matrix_words = [
        0u32, 0x80000000, 0x3f800000, 0xbf800000, 1, 0x7f800000, 0xff800000, 0x7fc12345,
        0xffc23456, 0x7fa12345, 0xffa23456,
    ];
    for (global, dc) in [(1, 1), (1024, 64), (4096, 16), (8192, 256), (73728, 65536)] {
        for offset in 0..8 {
            for &word in &matrix_words {
                cases.push(request(
                    dc,
                    global,
                    [2.0, 4.0, f32::from_bits(word)],
                    offset,
                    &[
                        "E 5".to_owned(),
                        "C".to_owned(),
                        format!("S {} {} {}", 3.0f32.to_bits(), 7.0f32.to_bits(), word),
                        format!("R {offset}"),
                        "E 14".to_owned(),
                    ],
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn quantizer_scale_global_scale_keeps_binary32_product_lround_and_recomputation_order() {
    let mut cases = Vec::new();
    let globals = [
        i32::MIN,
        -65536,
        -4097,
        -1024,
        -3,
        -1,
        0,
        1,
        3,
        1024,
        65536,
        16777215,
        16777216,
        16777217,
        i32::MAX,
    ];
    let scales = [
        -2.0f32,
        -1.5,
        -1.0,
        -0.75,
        -0.5,
        -0.0,
        0.0,
        f32::from_bits(1),
        0.25,
        1.0 / 3.0,
        0.5,
        0.75,
        1.0,
        1.5,
        2.0,
    ];
    let long_range_scales = [
        0x4eff_ffff,
        0x4f00_0000,
        0x4f00_0001,
        0x4f7f_ffff,
        0x4f80_0000,
        0x4f80_0001,
        0xceff_ffff,
        0xcf00_0000,
        0xcf00_0001,
        0xcf7f_ffff,
        0xcf80_0000,
        0xcf80_0001,
        0x5eff_ffff,
        0x5f00_0000,
        0xdeff_ffff,
        0xdf00_0000,
    ];
    // Only std::lround's nonfinite/out-of-C-long domain is excluded. Preserve
    // the source's selected low-bit long-to-int narrowing outside i32 range.
    for global in globals {
        for scale in scales
            .into_iter()
            .chain(long_range_scales.map(f32::from_bits))
        {
            let product = global as f32 * scale;
            if !product.is_finite()
                || product < std::ffi::c_long::MIN as f32
                || product >= -(std::ffi::c_long::MIN as f32)
            {
                continue;
            }
            for dc in [0, -1, 1, 64, i32::MIN, i32::MAX] {
                cases.push(request(
                    dc,
                    global,
                    [2.0, -7.0, 11.0],
                    3,
                    &[
                        "C".to_owned(),
                        format!("G {}", scale.to_bits()),
                        "E 5".to_owned(),
                    ],
                ));
            }
        }
    }
    for dc_word in [0x7fc12345, 0xffc23456, 0x7fa12345, 0xffa23456] {
        cases.push(request(
            64,
            1024,
            [1.0, 1.0, f32::from_bits(dc_word)],
            0,
            &[
                "C".to_owned(),
                format!("G {}", 1.5f32.to_bits()),
                format!("G {}", 0.5f32.to_bits()),
            ],
        ));
    }
    compare(cases);
}

#[test]
fn quantizer_matrix_getters_borrow_each_original_computed_strategy_table() {
    compare(
        (0..27)
            .map(|strategy| format!("qz-matrix {strategy}"))
            .collect(),
    );
}
