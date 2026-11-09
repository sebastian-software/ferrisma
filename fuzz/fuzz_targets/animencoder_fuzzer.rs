// libwebp v1.6.0, tests/fuzzer/animencoder_fuzzer.cc:
//
// Copyright 2018 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

//! Port of `tests/fuzzer/animencoder_fuzzer.cc`: an animation of up to 15
//! frames from the three built-in pictures, the first cropped or scaled and the
//! others rescaled to it, each encoded with `WebPAnimEncoderAdd` and a fuzzed
//! configuration and timestamp step, then the final `Add` and `Assemble`.
//!
//! The FuzzTest domains become one `arbitrary` tuple: `minimize_size`, the two
//! values of `ArbitraryKMinKMax` (kmax in `0..=15`, kmin in its range, by
//! modulo), `allow_mixed`, and the frames: `use_argb`, the timestamp step (in
//! `0..=131073`, by modulo), the 24 configuration values of
//! `ArbitraryWebPConfig`, the two optional ratio pairs of
//! `ArbitraryCropOrScaleParams` and the source picture (modulo the three
//! pictures). An empty frame list returns at once (C's domain has at least
//! one frame) and frames past 15 are ignored. The fuzzed input bytes, which
//! the C target passes on unused, and the optimization index of
//! `SetOptimization` are dropped: the reference configuration compiles no
//! SIMD.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use libfuzzer_sys::fuzz_target;
use webp::enc::picture_enc::webp_picture_free;
use webp::enc::picture_rescale_enc::webp_picture_rescale;
use webp::mux::anim_encode::{
    WebPAnimEncoder, webp_anim_encoder_add, webp_anim_encoder_assemble, webp_anim_encoder_delete,
    webp_anim_encoder_get_error,
};
use webp::webp::encode::{WebPConfig, WebPEncodingError};
use webp::webp::mux::{
    WebPAnimEncoderOptions, webp_anim_encoder_new, webp_anim_encoder_options_init,
};
use webp::webp::mux_types::{WebPData, webp_data_clear};

// upstream: tests/fuzzer/animencoder_fuzzer.cc FrameConfig
struct FrameConfig {
    use_argb: bool,
    timestamp: i32,
    webp_config: WebPConfig,
    crop_or_scale_params: fuzz_utils::CropOrScaleParams,
    source_image_index: usize,
}

// upstream: tests/fuzzer/animencoder_fuzzer.cc ArbitraryKMinKMax
/// kmax in `0..=15`; kmin in `kmax / 2..=kmax - 1` for kmax above 1, else 0.
fn arbitrary_kmin_kmax(kmax: u8, kmin: u8) -> (i32, i32) {
    let kmax = i32::from(kmax) % 16;
    let min_kmin = if kmax > 1 { kmax / 2 } else { 0 };
    let max_kmin = if kmax > 1 { kmax - 1 } else { 0 };
    (min_kmin + i32::from(kmin) % (max_kmin - min_kmin + 1), kmax)
}

// upstream: tests/fuzzer/animencoder_fuzzer.cc AddFrame
/// Returns false where C returns 0, after deleting the encoder as C does.
fn add_frame<'a>(
    enc: &mut Option<WebPAnimEncoder<'a>>,
    anim_config: &WebPAnimEncoderOptions,
    width: &mut i32,
    height: &mut i32,
    timestamp_ms: i32,
    frame_config: &FrameConfig,
) -> bool {
    // Init the source picture.
    let mut pic =
        fuzz_utils::get_source_picture(frame_config.source_image_index, frame_config.use_argb);

    // Crop and scale.
    if enc.is_none() {
        // First frame will set canvas width and height.
        if !fuzz_utils::crop_or_scale(&mut pic, &frame_config.crop_or_scale_params) {
            let error_code = pic.error_code;
            webp_picture_free(&mut pic);
            if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY {
                return false;
            }
            panic!("ExtractAndCropOrScale failed. Error code: {}", error_code.0);
        }
    } else if let Err(error_code) = webp_picture_rescale(&mut pic, *width, *height) {
        // Other frames will be resized to the first frame's dimensions.
        if let Some(enc) = enc.take() {
            webp_anim_encoder_delete(enc);
        }
        webp_picture_free(&mut pic);
        if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY {
            return false;
        }
        panic!(
            "WebPPictureRescale failed. Size: {},{}. Error code: {}",
            *width, *height, error_code.0
        );
    }

    // Create encoder if it doesn't exist.
    if enc.is_none() {
        *width = pic.width;
        *height = pic.height;
        *enc = webp_anim_encoder_new(*width, *height, Some(anim_config));
        if enc.is_none() {
            webp_picture_free(&mut pic);
            return false;
        }
    }

    // Create frame encoding config.
    let mut config = frame_config.webp_config;
    // Skip slow settings on big images, it's likely to timeout.
    if pic.width * pic.height > 32 * 32 {
        config.method = if config.method > 4 { 4 } else { config.method };
        config.quality = if config.quality > 99.0 { 99.0 } else { config.quality };
        config.alpha_quality = if config.alpha_quality > 99 { 99 } else { config.alpha_quality };
    }

    // Encode.
    let encoder = enc.as_mut().expect("created above");
    if !webp_anim_encoder_add(encoder, Some(&mut pic), timestamp_ms, Some(&config)) {
        let error_code = pic.error_code;
        if let Some(enc) = enc.take() {
            webp_anim_encoder_delete(enc);
        }
        webp_picture_free(&mut pic);
        // Tolerate failures when running under the nallocfuzz engine as
        // WebPAnimEncoderAdd() may fail due to memory allocation errors outside
        // of the encoder; in muxer functions that return booleans for instance.
        if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY
            || error_code == WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE
            || std::env::var_os("NALLOC_FUZZ_VERSION").is_some()
        {
            return false;
        }
        panic!("WebPEncode failed. Error code: {}", error_code.0);
    }

    webp_picture_free(&mut pic);
    true
}

// upstream: tests/fuzzer/animencoder_fuzzer.cc AnimEncoderTest
fn anim_encoder_test(
    minimize_size: bool,
    kmin_kmax: (i32, i32),
    allow_mixed: bool,
    frame_configs: &[FrameConfig],
) {
    let mut enc: Option<WebPAnimEncoder<'_>> = None;
    let (mut width, mut height, mut timestamp_ms) = (0, 0, 0);

    // Extract a configuration from the packed bits.
    let mut anim_config = WebPAnimEncoderOptions::default();
    assert!(
        webp_anim_encoder_options_init(&mut anim_config),
        "WebPAnimEncoderOptionsInit failed."
    );
    anim_config.minimize_size = i32::from(minimize_size);
    anim_config.kmin = kmin_kmax.0;
    anim_config.kmax = kmin_kmax.1;
    anim_config.allow_mixed = i32::from(allow_mixed);
    anim_config.verbose = 0;

    // For each frame.
    for frame_config in frame_configs {
        if !add_frame(
            &mut enc,
            &anim_config,
            &mut width,
            &mut height,
            timestamp_ms,
            frame_config,
        ) {
            return;
        }

        timestamp_ms += frame_config.timestamp;
    }

    // Assemble.
    let mut enc = enc.expect("the first frame created the encoder");
    if !webp_anim_encoder_add(&mut enc, None, timestamp_ms, None) {
        panic!(
            "Last WebPAnimEncoderAdd failed: {}.",
            webp_anim_encoder_get_error(&enc)
        );
    }
    let mut webp_data = WebPData::default();
    // Tolerate failures when running under the nallocfuzz engine as allocations
    // during assembly may fail.
    if !webp_anim_encoder_assemble(&mut enc, &mut webp_data)
        && std::env::var_os("NALLOC_FUZZ_VERSION").is_none()
    {
        panic!(
            "WebPAnimEncoderAssemble failed: {}.",
            webp_anim_encoder_get_error(&enc)
        );
    }

    webp_anim_encoder_delete(enc);
    webp_data_clear(&mut webp_data);
}

/// `use_argb`, the timestamp step, the configuration values, the two ratio
/// pairs and the source picture of one frame.
type FrameInput = (bool, u32, [u8; 24], Option<(u8, u8)>, Option<(u8, u8)>, u8);

fuzz_target!(|input: (bool, (u8, u8), bool, Vec<FrameInput>)| {
    let (minimize_size, (kmax, kmin), allow_mixed, frames) = input;
    if frames.is_empty() {
        return;
    }
    let frame_configs: Vec<FrameConfig> = frames
        .into_iter()
        .take(15)
        .map(|(use_argb, timestamp, config, width_height, left_top, source)| FrameConfig {
            use_argb,
            timestamp: (timestamp % 131_074) as i32,
            webp_config: fuzz_utils::arbitrary_webp_config(config),
            crop_or_scale_params: fuzz_utils::arbitrary_crop_or_scale_params(width_height, left_top),
            source_image_index: usize::from(source) % fuzz_utils::NUM_SOURCE_IMAGES,
        })
        .collect();
    anim_encoder_test(
        minimize_size,
        arbitrary_kmin_kmax(kmax, kmin),
        allow_mixed,
        &frame_configs,
    );
});
