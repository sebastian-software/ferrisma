// libwebp v1.6.0, tests/fuzzer/enc_dec_fuzzer.cc:
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

//! Port of `tests/fuzzer/enc_dec_fuzzer.cc`: one of the three built-in source
//! pictures, cropped or scaled, encoded with `WebPEncode` and a fuzzed
//! configuration, decoded with `WebPDecode` into BGRA and, for exact lossless
//! encodings, compared pixel for pixel.
//!
//! The FuzzTest domains of the C target become one `arbitrary` tuple:
//! `use_argb`, the source image index (modulo the three images), the 24
//! configuration values of `ArbitraryWebPConfig` and the two optional ratio
//! pairs of `ArbitraryCropOrScaleParams`. The optimization index of
//! `SetOptimization` is dropped: the reference configuration compiles no SIMD.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use std::cell::RefCell;

use libfuzzer_sys::fuzz_target;
use webp::dec::buffer_dec::webp_free_dec_buffer;
use webp::dec::webp_dec::webp_decode;
use webp::enc::picture_enc::{
    webp_memory_writer_clear, webp_memory_writer_init, webp_picture_free,
};
use webp::enc::webp_enc::webp_encode;
use webp::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPDecBufferPlanes, WebPDecoderConfig, webp_init_decoder_config,
};
use webp::webp::encode::{WebPConfig, WebPEncodingError, WebPMemoryWriter};

// upstream: tests/fuzzer/enc_dec_fuzzer.cc EncDecTest
fn enc_dec_test(
    use_argb: bool,
    source_image_index: usize,
    mut config: WebPConfig,
    crop_or_scale_params: &fuzz_utils::CropOrScaleParams,
) {
    // The memory writer outlives the picture whose writer borrows it.
    let memory_writer = RefCell::new(WebPMemoryWriter::default());
    // Init the source picture.
    let mut pic = fuzz_utils::get_source_picture(source_image_index, use_argb);

    // Crop and scale.
    if !fuzz_utils::crop_or_scale(&mut pic, crop_or_scale_params) {
        let error_code = pic.error_code;
        webp_picture_free(&mut pic);
        if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY {
            return;
        }
        panic!("ExtractAndCropOrScale failed. Error code: {}", error_code.0);
    }

    // Skip slow settings on big images, it's likely to timeout.
    if pic.width * pic.height > 32 * 32 {
        if config.lossless != 0 {
            if config.quality > 99.0 && config.method >= 5 {
                config.quality = 99.0;
                config.method = 5;
            }
        } else if config.quality > 99.0 && config.method == 6 {
            config.quality = 99.0;
        }
        if config.alpha_quality == 100 && config.method == 6 {
            config.alpha_quality = 99;
        }
    }

    // Encode.
    webp_memory_writer_init(&mut memory_writer.borrow_mut());
    pic.writer = Some(fuzz_utils::memory_writer(&memory_writer));
    if !webp_encode(&config, &mut pic) {
        let error_code = pic.error_code;
        webp_memory_writer_clear(&mut memory_writer.borrow_mut());
        webp_picture_free(&mut pic);
        if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY
            || error_code == WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE
        {
            return;
        }
        panic!("WebPEncode failed. Error code: {}", error_code.0);
    }

    // Try decoding the result.
    let mut dec_config = WebPDecoderConfig::default();
    assert!(
        webp_init_decoder_config(&mut dec_config),
        "WebPInitDecoderConfig failed."
    );

    dec_config.output.colorspace = WEBP_CSP_MODE::MODE_BGRA;
    let status = {
        let writer = memory_writer.borrow();
        webp_decode(&writer.mem[..writer.size], &mut dec_config)
    };
    let status = status.err().unwrap_or(VP8StatusCode::VP8_STATUS_OK);
    if (status != VP8StatusCode::VP8_STATUS_OK
        && status != VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY
        && status != VP8StatusCode::VP8_STATUS_USER_ABORT)
        || (status == VP8StatusCode::VP8_STATUS_OK
            && (dec_config.output.width != pic.width || dec_config.output.height != pic.height))
    {
        panic!("WebPDecode failed. status: {}.", status.0);
    }

    if status == VP8StatusCode::VP8_STATUS_OK {
        let WebPDecBufferPlanes::RGBA(buffer) = &dec_config.output.u else {
            unreachable!("an RGBA colorspace");
        };
        let rgba = dec_config
            .output
            .storage
            .region(buffer.rgba.region)
            .expect("the samples");
        let rgba = &rgba[buffer.rgba.offset..];
        let w = dec_config.output.width as usize;
        let h = dec_config.output.height as usize;

        // Compare the results if exact encoding.
        if pic.use_argb != 0 && config.lossless != 0 && config.near_lossless == 100 {
            let argb = pic.argb_storage.region(0).expect("the ARGB plane");
            let base = pic.argb.expect("the ARGB plane").offset;
            for y in 0..h {
                for x in 0..w {
                    let at = y * buffer.stride as usize + 4 * x;
                    let mut v1 = u32::from_ne_bytes(rgba[at..at + 4].try_into().unwrap());
                    let mut v2 = argb[base + y * pic.argb_stride as usize + x];
                    if config.exact == 0 && ((v1 & 0xff00_0000) == 0 || (v2 & 0xff00_0000) == 0) {
                        // Only keep alpha for comparison of fully transparent area.
                        v1 &= 0xff00_0000;
                        v2 &= 0xff00_0000;
                    }
                    assert!(v1 == v2, "Lossless compression failed pixel-exactness.");
                }
            }
        }
    }

    webp_free_dec_buffer(&mut dec_config.output);
    pic.writer = None;
    webp_memory_writer_clear(&mut memory_writer.borrow_mut());
    webp_picture_free(&mut pic);
}

/// `use_argb`, the source image, the configuration and the two ratio pairs.
type Input = (bool, u8, [u8; 24], Option<(u8, u8)>, Option<(u8, u8)>);

fuzz_target!(|input: Input| {
    let (use_argb, source_image_index, config, width_height_ratio, left_top_ratio) = input;
    let source_image_index = usize::from(source_image_index) % fuzz_utils::NUM_SOURCE_IMAGES;
    let config = fuzz_utils::arbitrary_webp_config(config);
    let params = fuzz_utils::arbitrary_crop_or_scale_params(width_height_ratio, left_top_ratio);
    enc_dec_test(use_argb, source_image_index, config, &params);
});
