// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder step 1 (#118): the upstream test cases of the step's gate in
// differential form. Each test runs an upstream case's own inputs and
// settings through the original (the `es1-*` requests of
// tools/oracle/encoder_step1.cc) and through the port, and compares every
// output exactly; where the upstream test asserts a property, the property is
// also asserted on the port's output. Exact bytes replace the upstream size,
// Butteraugli and distance expectations (port docs/ENCODER_EXPANSION.md,
// "Upstream tests"). Every expected value comes from the original.
use super::*;
use jxl::jxl::base::random::Rng;
use jxl::jxl::coeff_order::decode_permutation;
use jxl::jxl::coeff_order_fwd::CoeffOrder;
use jxl::jxl::enc_coeff_order::encode_permutation;
use jxl::jxl::enc_quant_weights::{
    dequant_matrices_encode_default, dequant_matrices_set_custom_dc,
};
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::quant_weights::{DequantMatrices, NUM_QUANT_TABLES, QuantEncoding};

/// Compares the port's observations with the original's, request by request.
fn compare_step1(family: &str, requests: &[String], rust: impl Fn(&str) -> String) -> usize {
    let originals = super::original_observations(requests);
    for (request, cpp) in requests.iter().zip(&originals) {
        assert_eq!(&rust(request), cpp, "{family}: {request}");
    }
    println!(
        "{family}: {} C++/Rust observations matched exactly",
        requests.len()
    );
    requests.len()
}

// ---------------------------------------------------------------------------
// coeff_order_test.cc, encode halves.

/// TestPermutation's permutation of `kind` and RoundtripPermutation, as the
/// oracle prints them.
fn permutation(kind: u32, len: usize) -> String {
    const SWAPS: usize = 32;
    let mut perm: Vec<CoeffOrder> = (0..len as CoeffOrder).collect();
    let mut rng = Rng::new(0);
    if kind == 1 {
        for _ in 0..SWAPS {
            let a = rng.uniform_u(0, len as u64 - 1) as usize;
            let b = rng.uniform_u(0, len as u64 - 1) as usize;
            perm.swap(a, b);
        }
    }
    if kind == 2 {
        for _ in 0..SWAPS {
            let a = rng.uniform_u(0, len as u64 - 1) as usize;
            let b = rng.uniform_u(0, len as u64 - 1) as usize;
            let from = a.min(b);
            let to = a.max(b);
            let start = perm[from];
            for j in from..to {
                perm[j] = perm[j + 1];
            }
            perm[to] = start;
        }
    }
    if kind == 3 {
        rng.shuffle(&mut perm, len);
    }
    let mut out = format!("P {len}");
    for v in &perm {
        write!(out, " {v}").unwrap();
    }
    let mut writer = BitWriter::new();
    let status = encode_permutation(&perm, 0, len, &mut writer, LayerType::Header, None);
    write!(out, " W {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    pad(&mut writer);
    let bytes = writer.get_span().to_vec();
    write!(out, " {} {}", hex(&bytes), bytes.len()).unwrap();
    let mut decoded: Vec<CoeffOrder> = vec![0; len];
    let mut reader = BitReader::new(&bytes);
    let read = decode_permutation(
        MemoryManagerPtr::NATIVE,
        0,
        len,
        Some(&mut decoded),
        &mut reader,
    );
    let close = reader.close();
    write!(out, " R {} {}", status_code(&read), status_code(&close)).unwrap();
    for v in &decoded {
        write!(out, " {v}").unwrap();
    }
    // The upstream property: the permutation roundtrips.
    assert_eq!(decoded, perm, "permutation {kind} {len} roundtrips");
    out
}

/// All twelve TEST cases of coeff_order_test.cc: Identity, FewSlides,
/// FewSwaps and Random at 256, 1 << 12 and 1 << 16 entries.
#[test]
fn upstream_coeff_order_test_encode_halves_match_original() {
    let mut requests = Vec::new();
    for len in [256usize, 1 << 12, 1 << 16] {
        for kind in [0u32, 2, 1, 3] {
            requests.push(format!("es1-perm {kind} {len}"));
        }
    }
    let count = compare_step1("es1-coeff-order", &requests, |request| {
        let mut args = request.split_whitespace();
        args.next();
        let kind = number(&mut args);
        let len = number(&mut args);
        permutation(kind, len)
    });
    assert_eq!(count, 12);
}

// ---------------------------------------------------------------------------
// quant_weights_test.cc, encode halves.

fn quant_weights(request: &str) -> String {
    match request {
        "es1-qw-dc" => {
            let mut mat = DequantMatrices::default();
            let dc_quant = [1e+5f32, 1e+3, 1e+1];
            let status = dequant_matrices_set_custom_dc(&mut mat, &dc_quant);
            let mut out = format!("D {}", status_code(&status));
            for (c, &expected) in dc_quant.iter().enumerate() {
                let inv = mat.inv_dc_quant(c);
                write!(out, " {} {}", inv.to_bits(), mat.dc_quant(c).to_bits()).unwrap();
                // The upstream property (CheckSimilar): 10 bits of precision.
                let m = inv.abs().max(expected.abs());
                assert!(
                    (inv - expected).abs() <= m / 1024.0,
                    "DC {c}: {inv} {expected}"
                );
            }
            out
        }
        "es1-qw-default" => {
            // DequantMatricesSetCustom with all-default encodings: SetEncodings,
            // no RAW table for the modular encoder, then DequantMatricesRoundtrip
            // (DequantMatricesEncode, ZeroPadToByte, Decode, Close).
            let encodings = vec![QuantEncoding::library::<0>(); NUM_QUANT_TABLES as usize];
            let mut mat = DequantMatrices::default();
            mat.set_encodings(&encodings);
            let mut status = (|| {
                let mut writer = BitWriter::new();
                dequant_matrices_encode_default(&mat, &mut writer, LayerType::Header, None)?;
                pad(&mut writer);
                let bytes = writer.get_span().to_vec();
                let mut reader = BitReader::new(&bytes);
                mat.decode(&mut reader, None)?;
                reader.close()
            })();
            let mut out = format!("S {} {}", status_code(&status), mat.encodings().len());
            for (e, d) in encodings.iter().zip(mat.encodings()) {
                write!(
                    out,
                    " {},{},{}",
                    d.mode() as i32,
                    d.predefined,
                    d.dct_params.num_distance_bands
                )
                .unwrap();
                // The upstream properties of RoundtripMatrices.
                assert_eq!(e.mode() as i32, d.mode() as i32);
                assert_eq!(e.predefined, d.predefined);
                assert_eq!(
                    e.dct_params.num_distance_bands,
                    d.dct_params.num_distance_bands
                );
            }
            let mut writer = BitWriter::new();
            status = dequant_matrices_encode_default(&mat, &mut writer, LayerType::Header, None);
            pad(&mut writer);
            write!(
                out,
                " W {} {}",
                status_code(&status),
                hex(writer.get_span())
            )
            .unwrap();
            out
        }
        _ => panic!("quant weights request"),
    }
}

/// The admitted encode halves of quant_weights_test.cc: DC and AllDefault.
/// Not admitted (custom AC matrices are outside the step 1 profile, whose
/// encoder writes only the default matrices; port ENCODER_EXPANSION.md step
/// 1 "Closes": "AC DequantMatricesEncode for default matrices"): the
/// single-table cases DCT to AFV0 (parametric library encodings), RAW
/// (ModularFrameEncoder::AddQuantTable) and DCTUniform (a custom DCT
/// encoding). Invariant has no encode half.
#[test]
fn upstream_quant_weights_test_encode_halves_match_original() {
    let requests = vec!["es1-qw-dc".to_string(), "es1-qw-default".to_string()];
    let count = compare_step1("es1-quant-weights", &requests, quant_weights);
    assert_eq!(count, 2);
}
