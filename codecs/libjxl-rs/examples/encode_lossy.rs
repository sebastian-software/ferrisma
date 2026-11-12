// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! cjxl-equivalent lossy VarDCT encode of a binary 8-bit RGB PPM through the
//! port's public encoder API, for the CI performance comparison
//! (`scripts/bench/ci-perf/codec_jxl.sh`).
//!
//! ```text
//! encode_lossy input.ppm output.jxl [effort] [distance]
//! ```
//!
//! It mirrors the call sequence of the pinned `tools/cjxl_main.cc` and
//! `lib/extras/enc/jxl.cc` `EncodeImageJXL` for a PPM input without flags
//! other than `-e`/`-d`: the PNM decoder's zero-initialized basic info and
//! frame header, sRGB color encoding with a zero rendering intent, the
//! options cjxl always sets (at their default values) in cjxl's order except
//! the explicit `PROGRESSIVE_AC=0` (see `encode`), a bare
//! codestream, `JxlEncoderAddImageFrame` with big-endian UINT8 samples, and
//! cjxl's doubling ProcessOutput loop. cjxl's parallel runner has no port
//! counterpart: the port runs every unit serially, as the original does
//! without a runner, so compare it with `cjxl --num_threads=0`.
//!
//! The timer covers the encoder calls from creation to the last output byte,
//! like cjxl's timer around `EncodeImageJXL`; reading and parsing the PPM and
//! writing the file are excluded. The public encoder is the translated C API,
//! so raw encoder and settings pointers are unavoidable here; each unsafe
//! call states the contract it relies on. Settings outside the admitted
//! profile (effort 7, distance below 3.0, 8-bit RGB without ICC, alpha or
//! container) end with an error status.

use jxl::jxl::codestream_header::{JxlBasicInfo, JxlFrameHeader, JxlOrientation};
use jxl::jxl::color_encoding::{
    ColorSpace, JxlColorEncoding, Primaries, TransferFunction, WhitePoint,
};
use jxl::jxl::encode::{
    self as api, JxlEncoder, JxlEncoderFrameSettingId as Id, JxlEncoderStatus as Status,
};
use jxl::jxl::types::{JxlBitDepth, JxlDataType, JxlEndianness, JxlPixelFormat};
use std::process::ExitCode;
use std::time::Instant;

/// Parses a binary P6 PPM with maxval 255 into (width, height, pixels).
fn read_ppm(bytes: &[u8]) -> Result<(u32, u32, &[u8]), String> {
    let mut pos = 0;
    let mut fields = Vec::new();
    while fields.len() < 4 {
        while pos < bytes.len() && (bytes[pos].is_ascii_whitespace() || bytes[pos] == b'#') {
            if bytes[pos] == b'#' {
                while pos < bytes.len() && bytes[pos] != b'\n' {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }
        let start = pos;
        while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        fields.push(std::str::from_utf8(&bytes[start..pos]).map_err(|e| e.to_string())?);
    }
    if fields[0] != "P6" || fields[3] != "255" {
        return Err("only binary 8-bit P6 PPM input is supported".into());
    }
    let width: u32 = fields[1].parse().map_err(|_| "bad PPM width")?;
    let height: u32 = fields[2].parse().map_err(|_| "bad PPM height")?;
    let size = width as usize * height as usize * 3;
    let pixels = bytes
        .get(pos + 1..pos + 1 + size)
        .ok_or("truncated PPM raster")?;
    Ok((width, height, pixels))
}

fn check(enc: *mut JxlEncoder, status: Status, call: &str) -> Result<(), String> {
    if status == Status::SUCCESS {
        return Ok(());
    }
    // SAFETY: `enc` is the live encoder; no other reference to it is live.
    let error = api::jxl_encoder_get_error(unsafe { &*enc }).0;
    Err(format!(
        "{call} failed: status {} error {error:#x}",
        status.0
    ))
}

/// Runs cjxl's encoder call sequence; returns the codestream.
fn encode(
    enc: *mut JxlEncoder,
    width: u32,
    height: u32,
    pixels: &[u8],
    effort: i64,
    distance: f32,
) -> Result<Vec<u8>, String> {
    // SAFETY (all calls below): `enc` is a live encoder from
    // jxl_encoder_create that the caller destroys only after this function;
    // no Rust reference to it outlives a single call. `settings` is owned by
    // that encoder and used only while it lives.
    let settings = unsafe { api::jxl_encoder_frame_settings_create(enc, std::ptr::null()) };
    if settings.is_null() {
        return Err("JxlEncoderFrameSettingsCreate failed".into());
    }
    // cjxl's ProcessFlags order; every value but the effort is cjxl's default.
    let int_options: [(Id, i64); 6] = [
        (Id::EFFORT, effort),
        (Id::BROTLI_EFFORT, -1),
        (Id::BUFFERING, -1),
        (Id::EPF, -1),
        (Id::DECODING_SPEED, 0),
        (Id::RESAMPLING, -1),
    ];
    for (id, value) in int_options {
        let status = unsafe { api::jxl_encoder_frame_settings_set_option(settings, id, value) };
        check(enc, status, "JxlEncoderFrameSettingsSetOption")?;
    }
    // cjxl also sets PROGRESSIVE_AC to 0, which stores an explicit "off"
    // override. The port admits only the default override (the frame would
    // end with a StreamingFrame/FrameProfile admission), so it is omitted;
    // at effort 7 both resolve to a non-progressive frame, and the CI checks
    // compare the resulting bytes with cjxl's.
    let rest: [(Id, f32, bool); 16] = [
        (Id::EXTRA_CHANNEL_RESAMPLING, -1.0, false),
        (Id::PHOTON_NOISE, 0.0, true),
        (Id::ALREADY_DOWNSAMPLED, 0.0, false),
        (Id::GROUP_ORDER_CENTER_X, -1.0, false),
        (Id::GROUP_ORDER_CENTER_Y, -1.0, false),
        (Id::PROGRESSIVE_DC, -1.0, false),
        (Id::MODULAR_GROUP_SIZE, -1.0, false),
        (Id::MODULAR_PREDICTOR, -1.0, false),
        (Id::MODULAR_COLOR_SPACE, -1.0, false),
        (Id::MODULAR_MA_TREE_LEARNING_PERCENT, -1.0, true),
        (Id::MODULAR_NB_PREV_CHANNELS, -1.0, false),
        (Id::LOSSY_PALETTE, 0.0, false),
        (Id::PALETTE_COLORS, -1.0, false),
        (Id::CHANNEL_COLORS_GLOBAL_PERCENT, -1.0, true),
        (Id::CHANNEL_COLORS_GROUP_PERCENT, -1.0, true),
        (Id::OUTPUT_MODE, -1.0, false),
    ];
    for (id, value, is_float) in rest {
        let status = if is_float {
            unsafe { api::jxl_encoder_frame_settings_set_float_option(settings, id, value) }
        } else {
            unsafe { api::jxl_encoder_frame_settings_set_option(settings, id, value as i64) }
        };
        check(enc, status, "JxlEncoderFrameSettingsSetOption")?;
    }
    let status = unsafe { api::jxl_encoder_set_frame_distance(settings, distance) };
    check(enc, status, "JxlEncoderSetFrameDistance")?;
    let status = api::jxl_encoder_use_container(unsafe { &mut *enc }, 0);
    check(enc, status, "JxlEncoderUseContainer")?;

    // The PNM decoder's basic info: zero-initialized, then its fields.
    let info = JxlBasicInfo {
        xsize: width,
        ysize: height,
        bits_per_sample: 8,
        orientation: JxlOrientation::IDENTITY,
        num_color_channels: 3,
        ..JxlBasicInfo::zeroed()
    };
    let status = api::jxl_encoder_set_codestream_level(unsafe { &mut *enc }, -1);
    check(enc, status, "JxlEncoderSetCodestreamLevel")?;
    let status = api::jxl_encoder_set_basic_info(unsafe { &mut *enc }, &info);
    check(enc, status, "JxlEncoderSetBasicInfo")?;
    let status = api::jxl_encoder_set_upsampling_mode(unsafe { &mut *enc }, 1, -1);
    check(enc, status, "JxlEncoderSetUpsamplingMode")?;
    let status = unsafe { api::jxl_encoder_set_frame_bit_depth(settings, &JxlBitDepth::default()) };
    check(enc, status, "JxlEncoderSetFrameBitDepth")?;
    // ApplyColorHints without hints: sRGB, rendering intent left at zero.
    let color = JxlColorEncoding {
        color_space: ColorSpace::RGB,
        white_point: WhitePoint::D65,
        primaries: Primaries::SRGB,
        transfer_function: TransferFunction::SRGB,
        ..JxlColorEncoding::default()
    };
    let status = api::jxl_encoder_set_color_encoding(unsafe { &mut *enc }, &color);
    check(enc, status, "JxlEncoderSetColorEncoding")?;

    let status = unsafe { api::jxl_encoder_set_frame_header(settings, &JxlFrameHeader::default()) };
    check(enc, status, "JxlEncoderSetFrameHeader")?;
    let format = JxlPixelFormat {
        num_channels: 3,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::BIG,
        align: 0,
    };
    let status = unsafe { api::jxl_encoder_add_image_frame(settings, &format, Some(pixels)) };
    check(enc, status, "JxlEncoderAddImageFrame")?;
    api::jxl_encoder_close_input(unsafe { &mut *enc });

    // ReadCompressedOutput: start at 4096 bytes and double while needed.
    let mut compressed = vec![0u8; 4096];
    let mut next_out = compressed.as_mut_ptr();
    let mut avail_out = compressed.len();
    loop {
        // SAFETY: `next_out` points into `compressed` with `avail_out`
        // writable bytes after it; both variables outlive the call.
        let status =
            unsafe { api::jxl_encoder_process_output(&mut *enc, &mut next_out, &mut avail_out) };
        // SAFETY: the encoder advanced `next_out` within `compressed`.
        let offset = unsafe { next_out.offset_from(compressed.as_ptr()) } as usize;
        if status == Status::NEED_MORE_OUTPUT {
            compressed.resize(compressed.len() * 2, 0);
            // SAFETY: `offset` is within the resized buffer.
            next_out = unsafe { compressed.as_mut_ptr().add(offset) };
            avail_out = compressed.len() - offset;
            continue;
        }
        check(enc, status, "JxlEncoderProcessOutput")?;
        compressed.truncate(offset);
        return Ok(compressed);
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 5 {
        eprintln!("usage: encode_lossy input.ppm output.jxl [effort=7] [distance=1.0]");
        return ExitCode::from(2);
    }
    let effort: i64 = args.get(3).map_or(Ok(7), |s| s.parse()).expect("effort");
    let distance: f32 = args
        .get(4)
        .map_or(Ok(1.0), |s| s.parse())
        .expect("distance");
    let bytes = std::fs::read(&args[1]).expect("read input");
    let (width, height, pixels) = match read_ppm(&bytes) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let begin = Instant::now();
    // SAFETY: a null memory manager selects the default allocator.
    let enc = unsafe { api::jxl_encoder_create(std::ptr::null()) };
    if enc.is_null() {
        eprintln!("JxlEncoderCreate failed");
        return ExitCode::FAILURE;
    }
    let result = encode(enc, width, height, pixels, effort, distance);
    // SAFETY: the encoder and its settings are not used again.
    unsafe { api::jxl_encoder_destroy(enc) };
    let elapsed = begin.elapsed();
    let compressed = match result {
        Ok(compressed) => compressed,
        Err(e) => {
            eprintln!("{e}");
            // With the cpp-oracle feature, name the admission checkpoint.
            #[cfg(feature = "cpp-oracle")]
            eprintln!("admissions: {:?}", api::take_admissions());
            return ExitCode::FAILURE;
        }
    };
    std::fs::write(&args[2], &compressed).expect("write output");
    println!(
        "dimensions={width}x{height} effort={effort} distance={distance} bytes={} elapsed_ms={:.3}",
        compressed.len(),
        elapsed.as_secs_f64() * 1000.0
    );
    ExitCode::SUCCESS
}
