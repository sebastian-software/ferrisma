// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original DC storage, serialization, partial parsing, custom, and scaling APIs.
use super::*;
use jxl::jxl::enc_quant_weights::{
    dequant_matrices_encode_dc, dequant_matrices_scale_dc, dequant_matrices_set_custom_dc,
};
use jxl::jxl::quant_weights::DequantMatrices;

pub(super) fn handles(kind: &str) -> bool {
    kind == "qdc-chain"
}
fn read_dc(args: &mut std::str::SplitWhitespace<'_>) -> [f32; 3] {
    std::array::from_fn(|_| f32::from_bits(number(args)))
}
fn state(out: &mut String, matrices: &DequantMatrices) {
    for c in 0..3 {
        write!(
            out,
            " {},{}",
            matrices.dc_quant(c).to_bits(),
            matrices.inv_dc_quant(c).to_bits()
        )
        .unwrap();
    }
}
pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut matrices = DequantMatrices::default();
    if number::<u32>(args) != 0 {
        matrices.set_dc_quant(&read_dc(args));
    }
    let prefix_bits = number(args);
    let count = number(args);
    let mut writer = BitWriter::new();
    let mut aux = AuxOut::default();
    prefix(&mut writer, prefix_bits);
    let mut out = "Q".to_owned();
    state(&mut out, &matrices);
    modular_frame_state::writer_output(&mut out, &writer, &aux);
    for _ in 0..count {
        let result = match args.next().unwrap() {
            "S" => {
                matrices.set_dc_quant(&read_dc(args));
                Ok(())
            }
            "C" => dequant_matrices_set_custom_dc(&mut matrices, &read_dc(args)),
            "M" => dequant_matrices_scale_dc(&mut matrices, f32::from_bits(number(args))),
            "E" => dequant_matrices_encode_dc(
                &matrices,
                &mut writer,
                modular_frame_state::layer(number(args)),
                Some(&mut aux),
            ),
            "D" => {
                let offset = number(args);
                let bytes = unhex(args.next().unwrap());
                let mut reader = BitReader::new(&bytes);
                reader.skip_bits(offset);
                let result = matrices.decode_dc(&mut reader);
                write!(
                    out,
                    " R {} {}",
                    reader.total_bits_consumed(),
                    status_code(&reader.close())
                )
                .unwrap();
                result
            }
            _ => panic!("DC fixture action"),
        };
        write!(out, " A {}", status_code(&result)).unwrap();
        state(&mut out, &matrices);
        modular_frame_state::writer_output(&mut out, &writer, &aux);
    }
    out
}
fn bits(values: [f32; 3]) -> String {
    format!(
        "{} {} {}",
        values[0].to_bits(),
        values[1].to_bits(),
        values[2].to_bits()
    )
}
fn request(seed: Option<[f32; 3]>, prefix: usize, actions: &[String]) -> String {
    let seed = seed.map_or_else(|| "0".to_owned(), |s| format!("1 {}", bits(s)));
    format!(
        "qdc-chain {seed} {prefix} {} {}",
        actions.len(),
        actions.join(" ")
    )
}

#[test]
fn dc_quantization_encoding_custom_roundtrip_scaling_and_error_prefixes_match_source() {
    let mut cases = Vec::new();
    let mut seed = 0x915d_3487_0c62_efab;
    let values = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        1e-30,
        1e-8,
        0.01,
        128.0,
        256.0,
        512.0,
        4096.0,
        65504.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::from_bits(0x7fc12345),
        f32::from_bits(0x7fa23456),
    ];
    for trial in 0..700 {
        let dc = if trial < values.len() * 3 {
            let mut dc = [4096.0, 512.0, 256.0];
            dc[trial % 3] = values[trial / 3];
            dc
        } else {
            std::array::from_fn(|_| f32::from_bits(next(&mut seed) as u32))
        };
        let scale = values[trial % values.len()];
        cases.push(request(
            None,
            trial % 8,
            &[
                "E 5".to_owned(),
                format!("S {}", bits(dc)),
                "E 0".to_owned(),
                format!("C {}", bits(dc)),
                "E 5".to_owned(),
                format!("M {}", scale.to_bits()),
                "E 14".to_owned(),
                format!("C {}", bits([4096.0, 512.0, 256.0])),
                "E 5".to_owned(),
            ],
        ));
    }
    compare(cases);
}

#[test]
fn dc_quantization_decoding_keeps_reused_values_partial_fields_bounds_and_close_status() {
    let mut cases = Vec::new();
    let mut seed = 0xa953_7f20_168b_42cd;
    let initial = Some([3333.3, -7.0, 900.0]);
    for len in 0..9 {
        for trial in 0..120 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            cases.push(request(
                initial,
                trial % 8,
                &[
                    format!("D {} {}", trial % 8, hex(&bytes)),
                    "D 0 01".to_owned(),
                    "E 5".to_owned(),
                ],
            ));
        }
    }
    for half in [
        0u16, 1, 2, 21, 22, 23, 1023, 1024, 0x3c00, 0x7bff, 0x7c00, 0x7fff, 0x8000, 0x8001, 0xbc00,
        0xffff,
    ] {
        for c in 0..3 {
            for offset in 0..8 {
                let mut writer = BitWriter::new();
                writer
                    .with_max_bits(
                        64,
                        LayerType::Header,
                        None,
                        |w, _| {
                            if offset != 0 {
                                w.write(offset, (1 << offset) - 1);
                            }
                            w.write(1, 0);
                            for i in 0..3 {
                                w.write(16, if i == c { u64::from(half) } else { 0x3c00 });
                            }
                            w.zero_pad_to_byte();
                            Ok(())
                        },
                        false,
                    )
                    .unwrap();
                let bytes = writer.get_span();
                for limit in [bytes.len(), bytes.len() - 1] {
                    cases.push(request(
                        initial,
                        offset,
                        &[
                            format!("D {offset} {}", hex(&bytes[..limit])),
                            "D 0 01".to_owned(),
                            "E 5".to_owned(),
                        ],
                    ));
                }
            }
        }
    }
    compare(cases);
}
