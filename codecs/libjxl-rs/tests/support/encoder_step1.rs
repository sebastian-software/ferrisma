// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder step 1 (#118): the public-API call sequences of the upstream test
// helpers that the step's differential cases run, translated onto the port's
// public encoder. The oracle runs the original helpers themselves
// (tools/oracle/encoder_step1.cc: the pristine lib/extras/enc/jxl.cc for
// EncodeImageJXL); this file only calls the translated public entry points
// in the same order with the same arguments and implements no encoder
// algorithm.
#![allow(dead_code)]
use jxl::jxl::codestream_header::{JxlBasicInfo, JxlFrameHeader};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::encode::{
    self as api, JxlEncoder, JxlEncoderAdmission, JxlEncoderFrameSettingId, JxlEncoderStatus,
};
use jxl::jxl::types::{JxlBitDepth, JxlBitDepthType, JxlPixelFormat};

/// The color representation of an extras PackedPixelFile.
pub enum PackedColor {
    /// kColorEncodingIsPrimary.
    Encoding(JxlColorEncoding),
    /// kIccIsPrimary.
    Icc(Vec<u8>),
}

/// The parts of extras::PackedPixelFile that EncodeImageJXL reads for one
/// still frame without extra channels, metadata boxes or a preview.
pub struct PackedPixelFile {
    pub info: JxlBasicInfo,
    pub color: PackedColor,
    pub input_bitdepth: JxlBitDepth,
    pub frame_info: JxlFrameHeader,
    pub format: JxlPixelFormat,
    pub pixels: Vec<u8>,
}

impl PackedPixelFile {
    /// PackedPixelFile(): JxlEncoderInitBasicInfo, the default input bit
    /// depth and a zero-initialized frame header (PackedFrame's `= {}`).
    pub fn new(format: JxlPixelFormat, pixels: Vec<u8>, color: PackedColor) -> Self {
        let mut info = JxlBasicInfo::zeroed();
        api::jxl_encoder_init_basic_info(&mut info);
        Self {
            info,
            color,
            input_bitdepth: JxlBitDepth {
                depth_type: JxlBitDepthType::FROM_PIXEL_FORMAT,
                bits_per_sample: 0,
                exponent_bits_per_sample: 0,
            },
            frame_info: JxlFrameHeader::default(),
            format,
            pixels,
        }
    }
}

/// The parts of extras::JXLCompressParams these cases set; every other field
/// keeps its default (no runner, no output processor, no container, level
/// -1, intensity target 0, upsampling 1/-1, no stats or debug image).
pub struct JxlCompressParams {
    pub distance: f32,
    pub allow_expert_options: bool,
    /// (id, value) integer options of frame index 0 (AddOption).
    pub options: Vec<(u32, i64)>,
}

/// How a translated helper ended.
#[derive(Debug, PartialEq, Eq)]
pub enum Encoded {
    /// The helper returned true with these bytes.
    Bytes(Vec<u8>),
    /// The helper returned false (a call failed without an admission).
    Failed,
    /// A call returned the port's admission result at this checkpoint, at
    /// the named call.
    Admission(&'static str, JxlEncoderAdmission),
}

fn admitted(call: &'static str, status: JxlEncoderStatus) -> Result<(), Encoded> {
    let admissions = api::take_admissions();
    if let Some(&checkpoint) = admissions.first() {
        return Err(Encoded::Admission(call, checkpoint));
    }
    if status == JxlEncoderStatus::SUCCESS {
        Ok(())
    } else {
        Err(Encoded::Failed)
    }
}

/// ReadCompressedOutput of lib/extras/enc/jxl.cc: a 4096-byte buffer that
/// doubles while ProcessOutput needs more output.
fn read_compressed_output(enc: &mut JxlEncoder) -> Result<Vec<u8>, Encoded> {
    let mut compressed = vec![0u8; 4096];
    let mut next_out: *mut u8 = compressed.as_mut_ptr();
    let mut avail_out: usize = compressed.len();
    let mut result = JxlEncoderStatus::NEED_MORE_OUTPUT;
    while result == JxlEncoderStatus::NEED_MORE_OUTPUT {
        // SAFETY: next_out points into `compressed` with avail_out writable
        // bytes; both caller variables outlive the call.
        result = unsafe { api::jxl_encoder_process_output(enc, &mut next_out, &mut avail_out) };
        if let Some(&checkpoint) = api::take_admissions().first() {
            return Err(Encoded::Admission("JxlEncoderProcessOutput", checkpoint));
        }
        if result == JxlEncoderStatus::NEED_MORE_OUTPUT {
            // SAFETY: next_out still points into `compressed`.
            let offset = unsafe { next_out.offset_from(compressed.as_ptr()) } as usize;
            compressed.resize(compressed.len() * 2, 0);
            // SAFETY: offset is within the resized buffer.
            next_out = unsafe { compressed.as_mut_ptr().add(offset) };
            avail_out = compressed.len() - offset;
        }
    }
    // SAFETY: next_out points into `compressed`.
    let written = unsafe { next_out.offset_from(compressed.as_ptr()) } as usize;
    compressed.truncate(written);
    if result != JxlEncoderStatus::SUCCESS {
        return Err(Encoded::Failed);
    }
    Ok(compressed)
}

/// SetFrameOptions for frame index 0: every option, in order.
///
/// # Safety
/// `settings` is a live frame-settings object of a live encoder.
unsafe fn set_frame_options(
    options: &[(u32, i64)],
    settings: *mut api::JxlEncoderFrameSettings,
) -> Result<(), Encoded> {
    for &(id, value) in options {
        // SAFETY: As for the function.
        let status = unsafe {
            api::jxl_encoder_frame_settings_set_option(
                settings,
                JxlEncoderFrameSettingId(id),
                value,
            )
        };
        admitted("JxlEncoderFrameSettingsSetOption", status)?;
    }
    Ok(())
}

/// EncodeImageJXL of lib/extras/enc/jxl.cc for `ppf` without JPEG bytes,
/// boxes, extra channels or chunked frames. The reached calls in their order;
/// the ones these inputs skip are noted.
pub fn encode_image_jxl(params: &JxlCompressParams, ppf: &PackedPixelFile) -> Encoded {
    api::take_admissions();
    // JxlEncoderMake(params.memory_manager = nullptr).
    // SAFETY: A null manager selects the default allocator.
    let enc_ptr = unsafe { api::jxl_encoder_create(std::ptr::null()) };
    assert!(!enc_ptr.is_null());
    let result = (|| -> Result<Vec<u8>, Encoded> {
        // SAFETY: The live encoder; no other reference to it exists.
        let enc = unsafe { &mut *enc_ptr };
        if params.allow_expert_options {
            api::jxl_encoder_allow_expert_options(enc);
        }
        // No runner (runner_opaque is null) and no output processor.
        // SAFETY: The live encoder and a null source.
        let settings = unsafe { api::jxl_encoder_frame_settings_create(enc_ptr, std::ptr::null()) };
        if settings.is_null() {
            return Err(Encoded::Failed);
        }
        // SAFETY: The live settings of the live encoder.
        unsafe { set_frame_options(&params.options, settings)? };
        // SAFETY: As above.
        let status = unsafe { api::jxl_encoder_set_frame_distance(settings, params.distance) };
        admitted("JxlEncoderSetFrameDistance", status)?;
        // No debug image callback and no stats.
        // use_boxes is false (no metadata) and use_container is false.
        // SAFETY: The settings call above has returned; no other reference.
        let enc = unsafe { &mut *enc_ptr };
        admitted(
            "JxlEncoderUseContainer",
            api::jxl_encoder_use_container(enc, 0),
        )?;
        let mut basic_info = ppf.info;
        // already_downsampled is 1; alpha_bits is 0, so no alpha channel.
        let num_alpha_channels = 0u32;
        // params.intensity_target is 0: keep the file's.
        basic_info.num_extra_channels = num_alpha_channels.max(ppf.info.num_extra_channels);
        basic_info.num_color_channels = ppf.info.num_color_channels;
        let lossless = params.distance == 0.0;
        let non_perceptual = params
            .options
            .iter()
            .find(|(id, _)| *id == DISABLE_PERCEPTUAL_HEURISTICS)
            .is_some_and(|(_, value)| *value == 1);
        basic_info.uses_original_profile = i32::from(lossless || non_perceptual);
        // override_bitdepth is 0.
        admitted(
            "JxlEncoderSetCodestreamLevel",
            api::jxl_encoder_set_codestream_level(enc, -1),
        )?;
        admitted(
            "JxlEncoderSetBasicInfo",
            api::jxl_encoder_set_basic_info(enc, &basic_info),
        )?;
        admitted(
            "JxlEncoderSetUpsamplingMode",
            api::jxl_encoder_set_upsampling_mode(enc, 1, -1),
        )?;
        // SAFETY: The live settings of the live encoder.
        let status = unsafe { api::jxl_encoder_set_frame_bit_depth(settings, &ppf.input_bitdepth) };
        admitted("JxlEncoderSetFrameBitDepth", status)?;
        if lossless {
            // SAFETY: As above.
            let status = unsafe { api::jxl_encoder_set_frame_lossless(settings, 1) };
            admitted("JxlEncoderSetFrameLossless", status)?;
        }
        // SAFETY: The settings calls have returned; no other reference.
        let enc = unsafe { &mut *enc_ptr };
        match &ppf.color {
            PackedColor::Icc(icc) => admitted(
                "JxlEncoderSetICCProfile",
                api::jxl_encoder_set_icc_profile(enc, icc),
            )?,
            PackedColor::Encoding(color) => admitted(
                "JxlEncoderSetColorEncoding",
                api::jxl_encoder_set_color_encoding(enc, color),
            )?,
        }
        // SetupFrame for frame 0: the frame header, the options of frame
        // index 0 (already consumed: option_idx is past them), strip_alpha
        // -1, no frame name, no alpha or extra channels.
        // SAFETY: The live settings of the live encoder.
        let status = unsafe { api::jxl_encoder_set_frame_header(settings, &ppf.frame_info) };
        admitted("JxlEncoderSetFrameHeader", status)?;
        // SAFETY: As above; the buffer holds pixels_size bytes.
        let status =
            unsafe { api::jxl_encoder_add_image_frame(settings, &ppf.format, Some(&ppf.pixels)) };
        admitted("JxlEncoderAddImageFrame", status)?;
        // SAFETY: The settings calls have returned; no other reference.
        let enc = unsafe { &mut *enc_ptr };
        api::jxl_encoder_close_input(enc);
        read_compressed_output(enc)
    })();
    // SAFETY: The encoder is not used again.
    unsafe { api::jxl_encoder_destroy(enc_ptr) };
    match result {
        Ok(bytes) => Encoded::Bytes(bytes),
        Err(encoded) => encoded,
    }
}

/// JXL_ENC_FRAME_SETTING_DISABLE_PERCEPTUAL_HEURISTICS.
pub const DISABLE_PERCEPTUAL_HEURISTICS: u32 = 39;
