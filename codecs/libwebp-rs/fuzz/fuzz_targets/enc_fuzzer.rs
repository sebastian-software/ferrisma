// libwebp v1.6.0, tests/fuzzer/enc_fuzzer.cc:
//
// Copyright 2024 Google Inc.
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

//! Port of `tests/fuzzer/enc_fuzzer.cc`: reads an image file, crops or scales
//! it, encodes it with `WebPEncode` and a fuzzed configuration into a
//! `WebPMemoryWriter`, decodes the result and, for exact lossless encodings,
//! compares the pixels.
//!
//! The FuzzTest domains of the C target become one `arbitrary` tuple: `use_argb`,
//! the 24 configuration values of `ArbitraryWebPConfig`, the two optional ratio
//! pairs of `ArbitraryCropOrScaleParams`, followed by the file. The
//! optimization index of `SetOptimization` is dropped: the reference
//! configuration compiles no SIMD, so it selects nothing. The file is read with
//! imageio's WebP reader only (`fuzz_utils::read_webp`); its PNG, JPEG, TIFF
//! and PNM readers have no Rust counterpart.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use std::cell::RefCell;

use libfuzzer_sys::fuzz_target;
use webp::dec::webp_dec::webp_decode_bgra;
use webp::enc::picture_enc::{
    webp_memory_writer_clear, webp_memory_writer_init, webp_picture_free,
};
use webp::enc::webp_enc::webp_encode;
use webp::webp::encode::{
    WebPConfig, WebPEncodingError, WebPMemoryWriter, WebPPicture, webp_picture_init,
};

// upstream: tests/fuzzer/enc_fuzzer.cc EncTest
fn enc_test(
    file: &[u8],
    use_argb: bool,
    mut config: WebPConfig,
    crop_or_scale_params: &fuzz_utils::CropOrScaleParams,
) {
    // The memory writer outlives the picture whose writer borrows it.
    let memory_writer = RefCell::new(WebPMemoryWriter::default());
    // Init the source picture.
    let mut pic = WebPPicture::default();
    assert!(webp_picture_init(&mut pic), "WebPPictureInit failed.");
    pic.use_argb = i32::from(use_argb);

    if fuzz_utils::is_image_too_big(file) {
        return;
    }
    // WebPGuessImageReader: only the WebP reader exists in Rust.
    if !fuzz_utils::read_webp(file, &mut pic) {
        return;
    }

    // Crop and scale.
    if !fuzz_utils::crop_or_scale(&mut pic, crop_or_scale_params) {
        let error_code = pic.error_code;
        webp_picture_free(&mut pic);
        if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY {
            return;
        }
        panic!("CropOrScale failed. Error code: {}", error_code.0);
    }

    // Skip the cruncher except on small images, it's likely to timeout.
    if config.lossless != 0
        && config.quality == 100.
        && config.method == 6
        && pic.width * pic.height >= 16384
    {
        config.lossless = 0;
    }

    // Encode.
    webp_memory_writer_init(&mut memory_writer.borrow_mut());
    pic.writer = Some(fuzz_utils::memory_writer(&memory_writer));
    if !webp_encode(&config, &mut pic) {
        let error_code = pic.error_code;
        webp_memory_writer_clear(&mut memory_writer.borrow_mut());
        webp_picture_free(&mut pic);
        if error_code == WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY {
            return;
        }
        panic!(
            "WebPEncode failed. Error code: {} \nFile starts with: {:?}",
            error_code.0,
            &file[..file.len().min(20)]
        );
    }

    // Try decoding the result.
    let (mut w, mut h) = (0, 0);
    let rgba = {
        let writer = memory_writer.borrow();
        webp_decode_bgra(&writer.mem[..writer.size], Some(&mut w), Some(&mut h))
    };
    let Some(rgba) = rgba.filter(|_| w == pic.width && h == pic.height) else {
        panic!(
            "WebPDecodeBGRA failed.\nFile starts with: {:?}",
            &file[..file.len().min(20)]
        );
    };

    // Compare the results if exact encoding.
    if pic.use_argb != 0 && config.lossless != 0 && config.near_lossless == 100 {
        let argb = pic.argb_storage.region(0).expect("the ARGB plane");
        let base = pic.argb.expect("the ARGB plane").offset;
        for y in 0..h as usize {
            for x in 0..w as usize {
                let at = 4 * (y * w as usize + x);
                let mut v1 = u32::from_ne_bytes(rgba[at..at + 4].try_into().unwrap());
                let mut v2 = argb[base + y * pic.argb_stride as usize + x];
                if config.exact == 0 && ((v1 & 0xff00_0000) == 0 || (v2 & 0xff00_0000) == 0) {
                    // Only keep alpha for comparison of fully transparent area.
                    v1 &= 0xff00_0000;
                    v2 &= 0xff00_0000;
                }
                assert!(
                    v1 == v2,
                    "Lossless compression failed pixel-exactness.\nFile starts with: {:?}",
                    &file[..file.len().min(20)]
                );
            }
        }
    }

    pic.writer = None;
    webp_memory_writer_clear(&mut memory_writer.borrow_mut());
    webp_picture_free(&mut pic);
}

type Input<'a> = (bool, [u8; 24], Option<(u8, u8)>, Option<(u8, u8)>, &'a [u8]);

fuzz_target!(|input: Input<'_>| {
    let (use_argb, config, width_height_ratio, left_top_ratio, file) = input;
    let config = fuzz_utils::arbitrary_webp_config(config);
    let params = fuzz_utils::arbitrary_crop_or_scale_params(width_height_ratio, left_top_ratio);
    enc_test(file, use_argb, config, &params);
});
