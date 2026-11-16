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
// channels, no extra channels, with and without a container.

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
/// channels, no extra channels, with and without JxlEncoderUseContainer.
/// Each sets JxlEncoderSetCodestreamLevel(10), and the original writes the
/// level in a jxll box, so its output is a container even with
/// JxlEncoderUseContainer false. Encoder container (step 3): every history,
/// container output included, must match the original exactly, and the
/// decoder half runs on the bytes through both public decoders with the
/// test's events and format.
#[test]
fn upstream_roundtrip_test_frame_roundtrips_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let mut requests = Vec::new();
    for data_type in [0u32, 3, 2] {
        for channels in [1u32, 3] {
            for container in 0..2 {
                requests.push(format!("es1-rt {data_type} {channels} 0 {container}"));
            }
        }
    }
    let originals = super::original_observations(&requests);
    let mut decodes = Vec::new();
    let mut exact = 0;
    for (request, cpp) in requests.iter().zip(&originals) {
        let (rust, admission) = roundtrip_encode_half(request);
        assert_eq!(admission, None, "no admission result: {request}");
        assert_eq!(rust, *cpp, "{request}");
        let c: Vec<&str> = cpp.split(' ').collect();
        assert!(
            c[12] == "1"
                && c[13].starts_with("0000000c4a584c200d0a870a")
                && c[13].contains("6a786c6c"),
            "the original writes a container with a jxll box: {request}"
        );
        exact += 1;
        let data_type = request.split(' ').nth(1).unwrap();
        let nc = request.split(' ').nth(2).unwrap();
        decodes.push(format!(
            "dpub-run 0 0 {} s4416 F{nc}.{data_type}.0.0.0 L1,1000000,0,1000,1",
            c[13]
        ));
    }
    println!("es1-roundtrip: {exact} C++/Rust histories matched exactly");
    assert_eq!(exact, 12);
    super::decoder_public::compare("es1-roundtrip-decode", decodes);
}

// ---------------------------------------------------------------------------
// jxl_test.cc and speed_tier_test.cc: test::Roundtrip's EncodeImageJXL.

#[path = "../support/encoder_step1.rs"]
mod helpers;

use helpers::{Encoded, JxlCompressParams, PackedColor, PackedPixelFile, encode_image_jxl};
use jxl::jxl::codestream_header::{JxlBlendMode, JxlFrameHeader};
use jxl::jxl::color_encoding::{
    ColorSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};

/// The raw fixtures of tools/encoder_step1_fixtures.py.
fn fixture(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/encoder-step1")
        .join(name)
}

/// The pixel source of an EncodeImageJXL case.
#[derive(Clone, Copy)]
enum Source {
    /// TestImage(): 8-bit RGB, little endian, "RGB_D65_SRG_Rel_SRG", one
    /// AddFrame with a zero frame header, ZeroFill.
    Zero,
    /// A testdata PNG through DecodeFromBytes and ClearMetadata, cropped by
    /// SetDimensions: the raw fixture (8-bit RGB, big endian, its iCCP
    /// profile primary, one frame with the PNG decoder's frame header).
    Png(&'static str, usize),
}

/// One test::Roundtrip call: its EncodeImageJXL inputs.
#[derive(Clone)]
struct EjxlCase {
    source: Source,
    xsize: usize,
    ysize: usize,
    intensity_target: f32,
    distance: f32,
    allow_expert_options: bool,
    options: Vec<(u32, i64)>,
}

impl EjxlCase {
    fn new(source: Source, xsize: usize, ysize: usize, distance: f32) -> Self {
        Self {
            source,
            xsize,
            ysize,
            // PackedPixelFile's JxlEncoderInitBasicInfo, and DecodeImageAPNG
            // resets it for a non-PQ transfer function.
            intensity_target: 0.0,
            distance,
            allow_expert_options: false,
            options: Vec::new(),
        }
    }

    /// The frame header the case's PackedFrame carries: zero for
    /// TestImage::AddFrame; DecodeImageAPNG's for a still PNG (default
    /// delay 1/10 s, DisposeOp::NONE so save_as_reference, BlendOp::SOURCE so
    /// kReplace, full size, last), cropped by TestImage::CropLayerInfo.
    fn frame_info(&self) -> JxlFrameHeader {
        let mut h = JxlFrameHeader::default();
        if let Source::Png(..) = self.source {
            h.duration = 100;
            h.is_last = 1;
            h.layer_info.xsize = self.xsize as u32;
            h.layer_info.ysize = self.ysize as u32;
            h.layer_info.blend_info.blendmode = JxlBlendMode(0);
            h.layer_info.blend_info.source = 1;
            h.layer_info.save_as_reference = 1;
        }
        h
    }

    fn request(&self) -> String {
        let mut out = format!(
            "es1-ejxl {} {} {}",
            u8::from(self.allow_expert_options),
            self.distance.to_bits(),
            self.options.len()
        );
        for (id, value) in &self.options {
            write!(out, " {id} {value}").unwrap();
        }
        let endianness = if matches!(self.source, Source::Zero) {
            1
        } else {
            2
        };
        write!(
            out,
            " {} {} 3 8 2 {endianness} {}",
            self.xsize,
            self.ysize,
            self.intensity_target.to_bits()
        )
        .unwrap();
        match self.source {
            Source::Zero => out.push_str(" e 0 1 1 13 1 z"),
            Source::Png(name, source_xsize) => write!(
                out,
                " i {} f {} {source_xsize}",
                fixture(&format!("{name}.icc")).display(),
                fixture(&format!("{name}.u8")).display()
            )
            .unwrap(),
        }
        let h = self.frame_info();
        let l = &h.layer_info;
        write!(
            out,
            " {} {} {} {} {} {} {} {} {} {} {} {}",
            h.duration,
            h.is_last,
            l.have_crop,
            l.crop_x0,
            l.crop_y0,
            l.xsize,
            l.ysize,
            l.blend_info.blendmode.0,
            l.blend_info.source,
            l.blend_info.alpha,
            l.blend_info.clamp,
            l.save_as_reference
        )
        .unwrap();
        out
    }

    /// The same PackedPixelFile for the port's EncodeImageJXL translation.
    fn ppf(&self) -> PackedPixelFile {
        let channels = 3usize;
        let (color, pixels, endianness) = match self.source {
            Source::Zero => (
                PackedColor::Encoding(JxlColorEncoding {
                    color_space: ColorSpace(0),
                    white_point: WhitePoint(1),
                    primaries: Primaries(1),
                    transfer_function: TransferFunction(13),
                    rendering_intent: RenderingIntent(1),
                    ..JxlColorEncoding::default()
                }),
                vec![0u8; self.xsize * self.ysize * channels],
                JxlEndianness(1),
            ),
            Source::Png(name, source_xsize) => {
                let icc = std::fs::read(fixture(&format!("{name}.icc"))).unwrap();
                let source = std::fs::read(fixture(&format!("{name}.u8"))).unwrap();
                let row = self.xsize * channels;
                let mut pixels = Vec::with_capacity(row * self.ysize);
                for y in 0..self.ysize {
                    let start = y * source_xsize * channels;
                    pixels.extend_from_slice(&source[start..start + row]);
                }
                (PackedColor::Icc(icc), pixels, JxlEndianness(2))
            }
        };
        let format = JxlPixelFormat {
            num_channels: channels as u32,
            data_type: JxlDataType::UINT8,
            endianness,
            align: 0,
        };
        let mut ppf = PackedPixelFile::new(format, pixels, color);
        ppf.info.xsize = self.xsize as u32;
        ppf.info.ysize = self.ysize as u32;
        ppf.info.num_color_channels = 3;
        ppf.info.num_extra_channels = 0;
        ppf.info.bits_per_sample = 8;
        ppf.info.exponent_bits_per_sample = 0;
        ppf.info.alpha_bits = 0;
        ppf.info.alpha_exponent_bits = 0;
        ppf.info.intensity_target = self.intensity_target;
        ppf.frame_info = self.frame_info();
        ppf
    }

    fn params(&self) -> JxlCompressParams {
        JxlCompressParams {
            distance: self.distance,
            allow_expert_options: self.allow_expert_options,
            options: self.options.clone(),
        }
    }
}

/// A case the step 1 profile does not admit, with the reason.
struct NotAdmitted {
    case: EjxlCase,
    reason: &'static str,
}

/// Runs EncodeImageJXL cases on both encoders. Admitted cases must match the
/// original's result and bytes exactly; every port result that is an
/// admission must be listed in `not_admitted` (and vice versa) and must
/// coincide with an original that succeeds. The exact codestreams are then
/// decoded by both public decoders (float and 8-bit output, whole and
/// 64-byte input), which replaces the tests' Butteraugli and distance checks
/// and compares the decoded basic info (intensity target included).
fn compare_ejxl(family: &str, cases: &[EjxlCase], not_admitted: &[NotAdmitted]) -> usize {
    let mut all: Vec<&EjxlCase> = cases.iter().collect();
    all.extend(not_admitted.iter().map(|n| &n.case));
    let requests: Vec<String> = all.iter().map(|case| case.request()).collect();
    let originals = super::original_observations(&requests);
    let mut decodes = Vec::new();
    for (index, ((case, request), cpp)) in all.iter().zip(&requests).zip(&originals).enumerate() {
        let rust = encode_image_jxl(&case.params(), &case.ppf());
        let expected_admission = index >= cases.len();
        match rust {
            Encoded::Bytes(bytes) => {
                assert!(
                    !expected_admission,
                    "{family}: listed as not admitted: {request}"
                );
                assert_eq!(format!("J 1 {}", hex(&bytes)), *cpp, "{family}: {request}");
                let file = hex(&bytes);
                decodes.push(format!(
                    "dpub-run 0 0 {file} s5184 F3.0.0.0.0 L1,1000000,0,1000,1"
                ));
                decodes.push(format!(
                    "dpub-run 0 0 {file} s5120 F3.2.0.0.0 L2,64,0,1000,1"
                ));
            }
            Encoded::Failed => panic!("{family}: the port failed: {request}"),
            Encoded::Admission(call, checkpoint) => {
                assert!(
                    expected_admission,
                    "{family}: admission {checkpoint:?} at {call}: {request}"
                );
                assert!(
                    cpp.starts_with("J 1 "),
                    "{family}: the original encodes: {request}"
                );
                println!(
                    "{family}: not admitted ({}): admission {checkpoint:?} at {call}",
                    not_admitted[index - cases.len()].reason
                );
            }
        }
    }
    println!(
        "{family}: {} C++/Rust EncodeImageJXL results matched exactly; {} not admitted",
        cases.len(),
        not_admitted.len()
    );
    super::decoder_public::compare(&format!("{family}-decode"), decodes);
    cases.len()
}

/// jxl_test.cc RoundtripSinglePixel, RoundtripSmallD1 (both intensity
/// targets), RoundtripUnalignedD2 and RoundtripD2Consistent (serial: the
/// upstream 8-thread pool does not change the bytes, and the test's property
/// is that two encodes of the same input are identical, which the port must
/// show as well).
#[test]
fn upstream_jxl_test_roundtrips_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let bliznaca = Source::Png("bliznaca", 500);
    let mut cases = vec![
        EjxlCase::new(Source::Zero, 1, 1, 1.0),
        EjxlCase::new(bliznaca, 500 / 8, 500 / 8, 1.0),
    ];
    let mut small_d1_100 = EjxlCase::new(bliznaca, 500 / 8, 500 / 8, 1.0);
    small_d1_100.intensity_target = 100.0;
    cases.push(small_d1_100);
    cases.push(EjxlCase::new(bliznaca, 500 / 12, 500 / 7, 2.0));
    let flower = Source::Png("flower", 48);
    for xsize in (41..=48).rev() {
        let mut case = EjxlCase::new(flower, xsize, 15, 2.0);
        case.options = vec![(0, 7)];
        // Roundtrip twice.
        cases.push(case.clone());
        cases.push(case);
    }
    for xsize in (41..=48).rev() {
        let mut case = EjxlCase::new(flower, xsize, 15, 2.0);
        case.options = vec![(0, 7)];
        let ppf = case.ppf();
        // The test's property on the port alone: identical results.
        assert_eq!(
            encode_image_jxl(&case.params(), &ppf),
            encode_image_jxl(&case.params(), &ppf),
            "RoundtripD2Consistent {xsize}"
        );
    }
    assert_eq!(compare_ejxl("es1-jxl-test", &cases, &[]), 20);
}

/// speed_tier_test.cc, the kSquirrel instances (effort 10 - 3 = 7) with
/// shrink8 true and false. The shrink8 instance's second, lossless roundtrip
/// is not admitted.
#[test]
fn upstream_speed_tier_test_squirrel_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let bliznaca = Source::Png("bliznaca", 500);
    let squirrel = |xsize, ysize, distance| {
        let mut case = EjxlCase::new(bliznaca, xsize, ysize, distance);
        case.allow_expert_options = true;
        case.options = vec![(0, 7)];
        case
    };
    let cases = vec![squirrel(500 / 8, 500 / 8, 1.0), squirrel(500, 500, 1.0)];
    let not_admitted = [NotAdmitted {
        case: squirrel(500 / 8, 500 / 8, 0.0),
        reason: "lossless (distance 0, uses_original_profile), outside the step 1 profile",
    }];
    assert_eq!(compare_ejxl("es1-speed-tier", &cases, &not_admitted), 2);
}
