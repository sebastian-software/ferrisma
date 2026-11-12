//! djxl-compatible decoder front end for the external JPEG XL conformance
//! runner (`libjxl/conformance` `scripts/conformance.py`).
//!
//! It mirrors the public-API call sequence of the pinned
//! `lib/extras/dec/jxl.cc` `DecodeImageJXL` with the options `djxl` uses for
//! `.npy` output (`--norender_spotcolors`, float color channels, every extra
//! channel as a separate float plane, coalesced frames, upright orientation)
//! and for `.jpg` JPEG reconstruction. Only the Rust port is linked.
//!
//! ```text
//! conformance_decode input.jxl out.npy [--icc_out f] [--orig_icc_out f]
//!     [--metadata_out f] [--norender_spotcolors] [--preview_out f]
//! conformance_decode input.jxl out.jpg
//! ```
//!
//! Differences from djxl: the image goes to an output buffer instead of the
//! image-out callback (the API defines identical samples), and metadata keeps
//! the codestream bit depths instead of djxl's float-format rewrite. The
//! conformance script's metadata check only reaches the first frame name.

use jxl::jxl::codestream_header::{
    JxlBasicInfo, JxlExtraChannelInfo, JxlExtraChannelType, JxlFrameHeader,
};
use jxl::jxl::decode::{self as api, JxlColorProfileTarget, JxlDecoderStatus as Status};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::process::ExitCode;

struct Args {
    input: String,
    output: String,
    icc_out: Option<String>,
    orig_icc_out: Option<String>,
    metadata_out: Option<String>,
    render_spotcolors: bool,
}

fn parse() -> Result<Args, String> {
    let mut positional = Vec::new();
    let mut args = Args {
        input: String::new(),
        output: String::new(),
        icc_out: None,
        orig_icc_out: None,
        metadata_out: None,
        render_spotcolors: true,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--icc_out" => args.icc_out = it.next(),
            "--orig_icc_out" => args.orig_icc_out = it.next(),
            "--metadata_out" => args.metadata_out = it.next(),
            "--norender_spotcolors" => args.render_spotcolors = false,
            "--preview_out" => return Err("preview output is not implemented".into()),
            _ if arg.starts_with("--") => return Err(format!("unknown option {arg}")),
            _ => positional.push(arg),
        }
    }
    if positional.len() != 2 {
        return Err("usage: conformance_decode input.jxl output.(npy|jpg) [options]".into());
    }
    args.output = positional.pop().unwrap();
    args.input = positional.pop().unwrap();
    Ok(args)
}

/// Owns one decoder created by `jxl_decoder_create`.
struct Decoder(*mut api::JxlDecoder);

impl Decoder {
    fn new() -> Result<Self, String> {
        // SAFETY: A null manager selects the default allocator.
        let raw = unsafe { api::jxl_decoder_create(std::ptr::null()) };
        if raw.is_null() {
            return Err("JxlDecoderCreate failed".into());
        }
        Ok(Self(raw))
    }

    fn get(&mut self) -> &mut api::JxlDecoder {
        // SAFETY: The pointer is live and uniquely owned until Drop.
        unsafe { &mut *self.0 }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        // SAFETY: Sole owner of the created pointer.
        unsafe { api::jxl_decoder_destroy(self.0) };
    }
}

fn check(status: Status, what: &str) -> Result<(), String> {
    if status == Status::SUCCESS {
        Ok(())
    } else {
        Err(format!("{what} failed: {}", status.0))
    }
}

fn float_format(channels: u32) -> JxlPixelFormat {
    JxlPixelFormat {
        num_channels: channels,
        data_type: JxlDataType::FLOAT,
        endianness: JxlEndianness::LITTLE,
        align: 0,
    }
}

fn icc(dec: &mut Decoder, target: JxlColorProfileTarget) -> Result<Vec<u8>, String> {
    let mut size = 0;
    if api::jxl_decoder_get_icc_profile_size(dec.get(), target, Some(&mut size)) != Status::SUCCESS
    {
        eprintln!("JxlDecoderGetICCProfileSize failed");
        return Ok(Vec::new());
    }
    let mut profile = vec![0u8; size];
    if size != 0 {
        // SAFETY: `profile` is writable for `size` bytes during the call.
        let status = unsafe {
            api::jxl_decoder_get_color_as_icc_profile(dec.get(), target, profile.as_mut_ptr(), size)
        };
        check(status, "JxlDecoderGetColorAsICCProfile")?;
    }
    Ok(profile)
}

fn channel_type_name(kind: JxlExtraChannelType) -> &'static str {
    match kind {
        JxlExtraChannelType::ALPHA => "Alpha",
        JxlExtraChannelType::DEPTH => "Depth",
        JxlExtraChannelType::SPOT_COLOR => "SpotColor",
        JxlExtraChannelType::SELECTION_MASK => "SelectionMask",
        JxlExtraChannelType::BLACK => "Black",
        JxlExtraChannelType::CFA => "CFA",
        JxlExtraChannelType::THERMAL => "Thermal",
        _ => "UNKNOWN",
    }
}

struct Frame {
    name: String,
    duration: u32,
    timecode: u32,
    color: Vec<f32>,
    extra: Vec<Vec<f32>>,
}

fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn metadata(info: &JxlBasicInfo, extra: &[JxlExtraChannelInfo], frames: &[Frame]) -> String {
    let mut out = String::from("{\n  \"frames\": [");
    for (i, frame) in frames.iter().enumerate() {
        out.push_str(if i == 0 { "\n    {" } else { ",\n    {" });
        if info.have_animation != 0 {
            let seconds = f64::from(frame.duration) * f64::from(info.animation.tps_denominator)
                / f64::from(info.animation.tps_numerator);
            out.push_str(&format!("\"duration\": {seconds:.6}, "));
        }
        out.push_str(&format!("\"name\": {}", json_string(&frame.name)));
        if info.animation.have_timecodes != 0 {
            out.push_str(&format!(", \"timecode\": {}", frame.timecode));
        }
        out.push('}');
    }
    out.push_str("\n  ],\n");
    out.push_str(&format!(
        "  \"intensity_target\": {:.6},\n  \"min_nits\": {:.6},\n  \"relative_to_max_display\": {},\n  \"linear_below\": {:.6},\n",
        info.intensity_target, info.min_nits, info.relative_to_max_display, info.linear_below
    ));
    if info.have_preview != 0 {
        out.push_str("  \"preview\": {},\n");
    }
    let types: Vec<String> = extra
        .iter()
        .map(|e| json_string(channel_type_name(e.channel_type)))
        .collect();
    let mut bits = vec![info.bits_per_sample.to_string()];
    let mut exp = vec![info.exponent_bits_per_sample.to_string()];
    for e in extra {
        bits.push(e.bits_per_sample.to_string());
        exp.push(e.exponent_bits_per_sample.to_string());
    }
    out.push_str(&format!(
        "  \"extra_channel_type\": [{}],\n  \"bits_per_sample\": [{}],\n  \"exp_bits_per_sample\": [{}]\n}}\n",
        types.join(", "),
        bits.join(", "),
        exp.join(", ")
    ));
    out
}

fn npy(info: &JxlBasicInfo, extra_count: usize, frames: &[Frame]) -> Vec<u8> {
    let (w, h) = (info.xsize as usize, info.ysize as usize);
    let color = info.num_color_channels as usize;
    let channels = color + extra_count;
    let header = format!(
        "{{'descr': '<f4', 'fortran_order': False, 'shape': ({}, {h}, {w}, {channels}), }}\n",
        frames.len()
    );
    let mut out = b"\x93NUMPY\x01\x00".to_vec();
    out.extend_from_slice(&u16::try_from(header.len()).unwrap().to_le_bytes());
    out.extend_from_slice(header.as_bytes());
    out.reserve(frames.len() * w * h * channels * 4);
    for frame in frames {
        for p in 0..w * h {
            for c in 0..color {
                out.extend_from_slice(&frame.color[p * color + c].to_le_bytes());
            }
            for plane in &frame.extra {
                out.extend_from_slice(&plane[p].to_le_bytes());
            }
        }
    }
    out
}

fn write_optional(path: Option<&String>, bytes: &[u8]) -> Result<(), String> {
    if let Some(path) = path {
        std::fs::write(path, bytes).map_err(|e| format!("write {path}: {e}"))?;
    }
    Ok(())
}

fn decode_pixels(bytes: &[u8], args: &Args) -> Result<(), String> {
    let mut dec = Decoder::new()?;
    let events = (Status::BASIC_INFO.0
        | Status::FULL_IMAGE.0
        | Status::COLOR_ENCODING.0
        | Status::FRAME.0
        | Status::PREVIEW_IMAGE.0
        | Status::BOX.0) as i32;
    check(
        api::jxl_decoder_subscribe_events(dec.get(), events),
        "JxlDecoderSubscribeEvents",
    )?;
    check(
        api::jxl_decoder_set_render_spotcolors(dec.get(), i32::from(args.render_spotcolors)),
        "JxlDecoderSetRenderSpotcolors",
    )?;
    check(
        api::jxl_decoder_set_keep_orientation(dec.get(), 0),
        "JxlDecoderSetKeepOrientation",
    )?;
    check(
        api::jxl_decoder_set_unpremultiply_alpha(dec.get(), 0),
        "JxlDecoderSetUnpremultiplyAlpha",
    )?;
    check(
        api::jxl_decoder_set_coalescing(dec.get(), 1),
        "JxlDecoderSetCoalescing",
    )?;
    check(
        api::jxl_decoder_set_decompress_boxes(dec.get(), 1),
        "JxlDecoderSetDecompressBoxes",
    )?;
    // SAFETY: `bytes` outlives the decoder, which is dropped in this function.
    check(
        unsafe { api::jxl_decoder_set_input(dec.get(), bytes.as_ptr(), bytes.len()) },
        "JxlDecoderSetInput",
    )?;
    let mut info = JxlBasicInfo::zeroed();
    let mut extra: Vec<(u32, JxlExtraChannelInfo)> = Vec::new();
    let mut data_icc = Vec::new();
    let mut orig_icc = Vec::new();
    let mut frames: Vec<Frame> = Vec::new();
    let mut last_frame = false;
    let mut done = false;
    loop {
        // SAFETY: input and every registered buffer stay live until the
        // decoder is dropped; frame buffers are never reallocated after
        // registration because each frame is complete before the next push.
        let status = unsafe { api::jxl_decoder_process_input(dec.get()) };
        match status {
            Status::ERROR => return Err("Failed to decode image".into()),
            Status::NEED_MORE_INPUT => {
                if done {
                    break;
                }
                return Err("truncated input".into());
            }
            Status::BOX | Status::BOX_COMPLETE | Status::PREVIEW_IMAGE => {}
            Status::BASIC_INFO => {
                check(
                    api::jxl_decoder_get_basic_info(dec.get(), Some(&mut info)),
                    "JxlDecoderGetBasicInfo",
                )?;
                for index in 0..info.num_extra_channels {
                    let mut eci = JxlExtraChannelInfo::default();
                    check(
                        api::jxl_decoder_get_extra_channel_info(
                            dec.get(),
                            index as usize,
                            Some(&mut eci),
                        ),
                        "JxlDecoderGetExtraChannelInfo",
                    )?;
                    extra.push((index, eci));
                }
            }
            Status::COLOR_ENCODING => {
                data_icc = icc(&mut dec, JxlColorProfileTarget::DATA)?;
                orig_icc = icc(&mut dec, JxlColorProfileTarget::ORIGINAL)?;
            }
            Status::FRAME => {
                let mut header = JxlFrameHeader::default();
                check(
                    api::jxl_decoder_get_frame_header(dec.get(), Some(&mut header)),
                    "JxlDecoderGetFrameHeader",
                )?;
                let mut name = vec![0u8; header.name_length as usize + 1];
                // SAFETY: `name` is writable for its full length.
                check(
                    unsafe {
                        api::jxl_decoder_get_frame_name(dec.get(), name.as_mut_ptr(), name.len())
                    },
                    "JxlDecoderGetFrameName",
                )?;
                name.truncate(header.name_length as usize);
                let pixels = header.layer_info.xsize as usize * header.layer_info.ysize as usize;
                frames.push(Frame {
                    name: String::from_utf8_lossy(&name).into_owned(),
                    duration: header.duration,
                    timecode: header.timecode,
                    color: vec![0f32; pixels * info.num_color_channels as usize],
                    extra: vec![vec![0f32; pixels]; extra.len()],
                });
                last_frame = header.is_last != 0;
            }
            Status::NEED_IMAGE_OUT_BUFFER => {
                let format = float_format(info.num_color_channels);
                let frame = frames.last_mut().ok_or("no frame header")?;
                let mut size = 0;
                check(
                    api::jxl_decoder_image_out_buffer_size(
                        dec.get(),
                        Some(&format),
                        Some(&mut size),
                    ),
                    "JxlDecoderImageOutBufferSize",
                )?;
                if size != frame.color.len() * 4 {
                    return Err(format!("Invalid out buffer size {size}"));
                }
                // SAFETY: the frame's color vector is exactly `size` bytes and
                // is not moved or freed before the decoder is dropped.
                check(
                    unsafe {
                        api::jxl_decoder_set_image_out_buffer(
                            dec.get(),
                            Some(&format),
                            frame.color.as_mut_ptr().cast(),
                            size,
                        )
                    },
                    "JxlDecoderSetImageOutBuffer",
                )?;
                let ec_format = float_format(1);
                for ((index, _), plane) in extra.iter().zip(&mut frame.extra) {
                    let mut size = 0;
                    check(
                        api::jxl_decoder_extra_channel_buffer_size(
                            dec.get(),
                            Some(&ec_format),
                            Some(&mut size),
                            *index,
                        ),
                        "JxlDecoderExtraChannelBufferSize",
                    )?;
                    if size != plane.len() * 4 {
                        return Err(format!("Invalid extra channel buffer size {size}"));
                    }
                    // SAFETY: as for the color buffer above.
                    check(
                        unsafe {
                            api::jxl_decoder_set_extra_channel_buffer(
                                dec.get(),
                                Some(&ec_format),
                                plane.as_mut_ptr().cast(),
                                size,
                                *index,
                            )
                        },
                        "JxlDecoderSetExtraChannelBuffer",
                    )?;
                }
            }
            Status::FULL_IMAGE => {
                // djxl: the codestream is done once the last frame is complete.
                done |= last_frame;
            }
            Status::SUCCESS => break,
            other => return Err(format!("Error: unexpected status: {}", other.0)),
        }
    }
    drop(dec);
    let extra_info: Vec<JxlExtraChannelInfo> = extra.iter().map(|(_, e)| *e).collect();
    std::fs::write(&args.output, npy(&info, extra.len(), &frames))
        .map_err(|e| format!("write {}: {e}", args.output))?;
    write_optional(args.icc_out.as_ref(), &data_icc)?;
    write_optional(args.orig_icc_out.as_ref(), &orig_icc)?;
    write_optional(
        args.metadata_out.as_ref(),
        metadata(&info, &extra_info, &frames).as_bytes(),
    )?;
    Ok(())
}

fn decode_jpeg(bytes: &[u8], args: &Args) -> Result<(), String> {
    let mut dec = Decoder::new()?;
    let events =
        (Status::BASIC_INFO.0 | Status::FULL_IMAGE.0 | Status::JPEG_RECONSTRUCTION.0) as i32;
    check(
        api::jxl_decoder_subscribe_events(dec.get(), events),
        "JxlDecoderSubscribeEvents",
    )?;
    // SAFETY: `bytes` outlives the decoder.
    check(
        unsafe { api::jxl_decoder_set_input(dec.get(), bytes.as_ptr(), bytes.len()) },
        "JxlDecoderSetInput",
    )?;
    let mut chunk = vec![0u8; bytes.len() * 3 / 2 + 1024];
    let mut jpeg = Vec::new();
    let mut can_reconstruct = false;
    let mut done = false;
    loop {
        // SAFETY: input and the registered JPEG chunk stay live; the chunk is
        // only resized after the decoder released it.
        let status = unsafe { api::jxl_decoder_process_input(dec.get()) };
        match status {
            Status::ERROR => return Err("Failed to decode image".into()),
            Status::NEED_MORE_INPUT => {
                if done {
                    break;
                }
                return Err("truncated input".into());
            }
            Status::BASIC_INFO => {}
            Status::JPEG_RECONSTRUCTION => {
                can_reconstruct = true;
                // SAFETY: `chunk` stays live and unmoved until released.
                check(
                    unsafe {
                        api::jxl_decoder_set_jpeg_buffer(dec.get(), chunk.as_mut_ptr(), chunk.len())
                    },
                    "JxlDecoderSetJPEGBuffer",
                )?;
            }
            Status::JPEG_NEED_MORE_OUTPUT => {
                let used = chunk.len() - api::jxl_decoder_release_jpeg_buffer(dec.get());
                jpeg.extend_from_slice(&chunk[..used]);
                if used == 0 {
                    let len = chunk.len() * 2;
                    chunk.resize(len, 0);
                }
                // SAFETY: as above, after the previous chunk was released.
                check(
                    unsafe {
                        api::jxl_decoder_set_jpeg_buffer(dec.get(), chunk.as_mut_ptr(), chunk.len())
                    },
                    "JxlDecoderSetJPEGBuffer",
                )?;
            }
            Status::NEED_IMAGE_OUT_BUFFER => break,
            Status::FULL_IMAGE => done = true,
            Status::SUCCESS => break,
            other => return Err(format!("Error: unexpected status: {}", other.0)),
        }
    }
    if !can_reconstruct {
        return Err("no JPEG reconstruction data".into());
    }
    let used = chunk.len() - api::jxl_decoder_release_jpeg_buffer(dec.get());
    jpeg.extend_from_slice(&chunk[..used]);
    drop(dec);
    std::fs::write(&args.output, jpeg).map_err(|e| format!("write {}: {e}", args.output))
}

fn main() -> ExitCode {
    let result = parse().and_then(|args| {
        let bytes = std::fs::read(&args.input).map_err(|e| format!("read {}: {e}", args.input))?;
        if api::jxl_signature_check(&bytes) == api::JxlSignature::INVALID {
            return Err("not a JPEG XL file".into());
        }
        if args.output.ends_with(".jpg") || args.output.ends_with(".jpeg") {
            decode_jpeg(&bytes, &args)
        } else if args.output.ends_with(".npy") {
            decode_pixels(&bytes, &args)
        } else {
            Err("output must be .npy or .jpg".into())
        }
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
