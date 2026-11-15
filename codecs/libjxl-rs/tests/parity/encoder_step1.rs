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
use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::coeff_order::decode_permutation;
use jxl::jxl::coeff_order_fwd::CoeffOrder;
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::enc_coeff_order::encode_permutation;
use jxl::jxl::enc_quant_weights::{
    dequant_matrices_encode_default, dequant_matrices_set_custom_dc,
};
use jxl::jxl::encode::{self as api, JxlEncoderAdmission, JxlEncoderStatus};
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::quant_weights::{DequantMatrices, NUM_QUANT_TABLES, QuantEncoding};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

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

// ---------------------------------------------------------------------------
// roundtrip_test.cc Float/Uint16/Uint8FrameRoundtripTest, lossy, 1 and 3
// channels, no extra channels, no container.

/// roundtrip_test.cc GetTestImage<T> for `bytes`-wide samples converted by
/// ConvertTestPixel<T>, as native-endian bytes.
fn roundtrip_test_image(xsize: usize, ysize: usize, channels: usize, data_type: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    for y in 0..ysize {
        for x in 0..xsize {
            for chan in 0..channels {
                let val = match chan % 4 {
                    0 => y as f32 / ysize as f32,
                    1 => x as f32 / xsize as f32,
                    2 => (x + y) as f32 / (xsize + ysize) as f32,
                    _ => (x * y) as f32 / (xsize * ysize) as f32,
                };
                match data_type {
                    0 => bytes.extend_from_slice(&val.to_ne_bytes()),
                    3 => {
                        bytes.extend_from_slice(&((val * f32::from(u16::MAX)) as u16).to_ne_bytes())
                    }
                    _ => bytes.push((val * f32::from(u8::MAX)) as u8),
                }
            }
        }
    }
    bytes
}

/// The port's run of the oracle's `es1-rt` request: the tokens up to the
/// encoder output and, unless ProcessOutput returned an admission result, the
/// held expectation and the compressed bytes.
fn roundtrip_encode_half(request: &str) -> (String, Option<JxlEncoderAdmission>) {
    let mut args = request.split_whitespace();
    args.next();
    let data_type: u32 = number(&mut args);
    let channels: u32 = number(&mut args);
    let lossless: i32 = number(&mut args);
    let container: i32 = number(&mut args);
    let (xsize, ysize) = (63usize, 129usize);
    let format = JxlPixelFormat {
        num_channels: channels,
        data_type: JxlDataType(data_type),
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    let original_bytes = roundtrip_test_image(xsize, ysize, channels as usize, data_type);
    api::take_admissions();
    // SAFETY: The default manager.
    let enc_ptr = unsafe { api::jxl_encoder_create(std::ptr::null()) };
    let mut out = format!("R {}", u8::from(!enc_ptr.is_null()));
    // SAFETY: The live encoder; no other reference while these calls run.
    let enc = unsafe { &mut *enc_ptr };
    write!(out, " {}", api::jxl_encoder_set_codestream_level(enc, 10).0).unwrap();
    write!(out, " {}", api::jxl_encoder_use_container(enc, container).0).unwrap();
    // test_utils.cc JxlBasicInfoSetFromPixelFormat for these formats.
    let mut basic_info = JxlBasicInfo::zeroed();
    api::jxl_encoder_init_basic_info(&mut basic_info);
    (
        basic_info.bits_per_sample,
        basic_info.exponent_bits_per_sample,
    ) = match data_type {
        0 => (32, 8),
        5 => (16, 5),
        2 => (8, 0),
        _ => (16, 0),
    };
    basic_info.num_color_channels = if channels < 3 { 1 } else { 3 };
    if channels == 2 || channels == 4 {
        basic_info.alpha_exponent_bits = basic_info.exponent_bits_per_sample;
        basic_info.alpha_bits = basic_info.bits_per_sample;
        basic_info.num_extra_channels = 1;
    } else {
        basic_info.alpha_exponent_bits = 0;
        basic_info.alpha_bits = 0;
    }
    basic_info.xsize = xsize as u32;
    basic_info.ysize = ysize as u32;
    basic_info.uses_original_profile = lossless;
    let has_interleaved_alpha = u32::from(channels == 2 || channels == 4);
    basic_info.num_extra_channels = has_interleaved_alpha;
    write!(
        out,
        " {}",
        api::jxl_encoder_set_basic_info(enc, &basic_info).0
    )
    .unwrap();
    write!(out, " {}", enc.metadata().m.num_extra_channels).unwrap();
    let mut color_encoding = JxlColorEncoding::default();
    if data_type == 0 {
        api::jxl_color_encoding_set_to_linear_srgb(&mut color_encoding, i32::from(channels < 3));
    } else {
        api::jxl_color_encoding_set_to_srgb(&mut color_encoding, i32::from(channels < 3));
    }
    write!(
        out,
        " {}",
        api::jxl_encoder_set_color_encoding(enc, &color_encoding).0
    )
    .unwrap();
    write!(
        out,
        " {}",
        api::jxl_encoder_set_upsampling_mode(enc, 1, -1).0
    )
    .unwrap();
    // SAFETY: The live encoder and a null source.
    let settings = unsafe { api::jxl_encoder_frame_settings_create(enc_ptr, std::ptr::null()) };
    write!(out, " {}", u8::from(!settings.is_null())).unwrap();
    // SAFETY: The live settings of the live encoder.
    let status = unsafe { api::jxl_encoder_set_frame_lossless(settings, lossless) };
    write!(out, " {}", status.0).unwrap();
    // SAFETY: As above; the buffer holds the test image.
    let status =
        unsafe { api::jxl_encoder_add_image_frame(settings, &format, Some(&original_bytes)) };
    write!(out, " {}", status.0).unwrap();
    // SAFETY: The settings calls have returned.
    let enc = unsafe { &mut *enc_ptr };
    write!(out, " {}", u8::from(enc.input_queue().is_empty())).unwrap();
    api::jxl_encoder_close_input(enc);
    // EncodeWithEncoder: a 64-byte buffer that doubles.
    let mut compressed = vec![0u8; 64];
    let mut next_out: *mut u8 = compressed.as_mut_ptr();
    let mut avail_out = compressed.len();
    let mut process_result = JxlEncoderStatus::NEED_MORE_OUTPUT;
    let mut admission = None;
    while process_result == JxlEncoderStatus::NEED_MORE_OUTPUT {
        // SAFETY: next_out points into `compressed` with avail_out bytes.
        process_result =
            unsafe { api::jxl_encoder_process_output(enc, &mut next_out, &mut avail_out) };
        if let Some(&checkpoint) = api::take_admissions().first() {
            admission = Some(checkpoint);
            break;
        }
        if process_result == JxlEncoderStatus::NEED_MORE_OUTPUT {
            // SAFETY: next_out points into `compressed`.
            let offset = unsafe { next_out.offset_from(compressed.as_ptr()) } as usize;
            compressed.resize(compressed.len() * 2, 0);
            // SAFETY: Within the resized buffer.
            next_out = unsafe { compressed.as_mut_ptr().add(offset) };
            avail_out = compressed.len() - offset;
        }
    }
    if admission.is_none() {
        // SAFETY: next_out points into `compressed`.
        let written = unsafe { next_out.offset_from(compressed.as_ptr()) } as usize;
        compressed.truncate(written);
        write!(
            out,
            " {} {}",
            u8::from(process_result == JxlEncoderStatus::SUCCESS),
            hex(&compressed)
        )
        .unwrap();
    }
    // SAFETY: Not used again.
    unsafe { api::jxl_encoder_destroy(enc_ptr) };
    (out, admission)
}

/// The admitted instances of the three frame roundtrip tests: lossy, 1 and 3
/// channels, no extra channels, no container. Each sets
/// JxlEncoderSetCodestreamLevel(10), and the original writes the level in a
/// jxll box, so its output is a container even with JxlEncoderUseContainer
/// false. Containers are outside the step 1 profile (raw codestream output;
/// step 3), so the port returns its EnqueuedInput admission result at the
/// first ProcessOutput: these histories compare exactly up to that call, the
/// original's bytes must be a container with a jxll box (the branch evidence),
/// and the decoder half then runs on the original's bytes through both public
/// decoders with the test's events and format.
#[test]
fn upstream_roundtrip_test_frame_roundtrips_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut requests = Vec::new();
    for data_type in [0u32, 3, 2] {
        for channels in [1u32, 3] {
            requests.push(format!("es1-rt {data_type} {channels} 0 0"));
        }
    }
    let originals = super::original_observations(&requests);
    let mut decodes = Vec::new();
    let (mut exact, mut admitted) = (0, 0);
    for (request, cpp) in requests.iter().zip(&originals) {
        let (rust, admission) = roundtrip_encode_half(request);
        let c: Vec<&str> = cpp.split(' ').collect();
        let r: Vec<&str> = rust.split(' ').collect();
        match admission {
            None => {
                assert_eq!(rust, *cpp, "{request}");
                exact += 1;
            }
            Some(checkpoint) => {
                assert_eq!(checkpoint, JxlEncoderAdmission::EnqueuedInput, "{request}");
                assert_eq!(
                    r[..],
                    c[..r.len()],
                    "prefix before the admission: {request}"
                );
                assert_eq!(r.len(), 12, "the admission is at ProcessOutput: {request}");
                let bytes = c[13];
                assert!(
                    c[12] == "1"
                        && bytes.starts_with("0000000c4a584c200d0a870a")
                        && bytes.contains("6a786c6c"),
                    "the original writes a container with a jxll box: {request}"
                );
                admitted += 1;
            }
        }
        let data_type = request.split(' ').nth(1).unwrap();
        let nc = request.split(' ').nth(2).unwrap();
        decodes.push(format!(
            "dpub-run 0 0 {} s4416 F{nc}.{data_type}.0.0.0 L1,1000000,0,1000,1",
            c[13]
        ));
    }
    println!(
        "es1-roundtrip: {exact} C++/Rust histories matched exactly; {admitted} admission \
         histories matched up to the admission call with original branch evidence (not parity)"
    );
    assert_eq!(exact + admitted, 6);
    super::decoder_public::compare("es1-roundtrip-decode", decodes);
}
