// libwebp v1.6.0, tests/fuzzer/animation_api_fuzzer.cc:
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

//! Port of `tests/fuzzer/animation_api_fuzzer.cc`: the animation decoder with
//! fuzzed threading and color mode on arbitrary bytes, after upstream's limits
//! on the canvas and frame sizes, decoding up to `kFuzzFrameLimit` frames. The
//! FuzzTest domains (the string, the boolean and one of the four animation
//! color modes) become one `arbitrary` tuple, with the mode chosen by modulo.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use std::borrow::Cow;

use libfuzzer_sys::fuzz_target;
use webp::dec::webp_dec::webp_get_info;
use webp::demux::anim_decode::{webp_anim_decoder_delete, webp_anim_decoder_get_next};
use webp::demux::demux::{
    webp_demux_delete, webp_demux_get_frame, webp_demux_get_i, webp_demux_release_iterator,
};
use webp::webp::decode::WEBP_CSP_MODE;
use webp::webp::demux::{
    WebPAnimDecoderOptions, WebPFormatFeature, webp_anim_decoder_new,
    webp_anim_decoder_options_init, webp_demux,
};
use webp::webp::mux_types::WebPData;

/// Animations only support 4 (out of 12) modes.
const MODES: [WEBP_CSP_MODE; 4] = [
    WEBP_CSP_MODE::MODE_RGBA,
    WEBP_CSP_MODE::MODE_BGRA,
    WEBP_CSP_MODE::MODE_rgbA,
    WEBP_CSP_MODE::MODE_bgrA,
];

// upstream: tests/fuzzer/animation_api_fuzzer.cc AnimationApiTest
fn animation_api_test(blob: &[u8], use_threads: bool, color_mode: WEBP_CSP_MODE) {
    let webp_data = WebPData {
        bytes: Cow::Borrowed(blob),
    };

    // WebPAnimDecoderNew uses WebPDemux internally to calloc canvas size.
    let Some(demux) = webp_demux(&webp_data) else {
        return;
    };
    let cw = webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_CANVAS_WIDTH);
    let ch = webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT);
    if cw as usize * ch as usize > fuzz_utils::FUZZ_PX_LIMIT {
        webp_demux_delete(demux);
        return;
    }

    // In addition to canvas size, check each frame separately.
    for i in 0..fuzz_utils::FUZZ_FRAME_LIMIT {
        let Some(mut iter) = webp_demux_get_frame(&demux, i as i32 + 1) else {
            break;
        };
        let (mut w, mut h) = (0, 0);
        if webp_get_info(&iter.fragment.bytes, Some(&mut w), Some(&mut h))
            && w as usize * h as usize > fuzz_utils::FUZZ_PX_LIMIT
        {
            // image size of the frame payload
            webp_demux_release_iterator(&mut iter);
            webp_demux_delete(demux);
            return;
        }
        webp_demux_release_iterator(&mut iter);
    }

    webp_demux_delete(demux);

    let mut dec_options = WebPAnimDecoderOptions::default();
    if !webp_anim_decoder_options_init(&mut dec_options) {
        return;
    }

    dec_options.use_threads = i32::from(use_threads);
    dec_options.color_mode = color_mode;

    let Some(mut dec) = webp_anim_decoder_new(&webp_data, Some(&dec_options)) else {
        return;
    };

    for _ in 0..fuzz_utils::FUZZ_FRAME_LIMIT {
        if webp_anim_decoder_get_next(&mut dec).is_none() {
            break;
        }
    }

    webp_anim_decoder_delete(dec);
}

fuzz_target!(|input: (bool, u8, &[u8])| {
    let (use_threads, mode, blob) = input;
    animation_api_test(blob, use_threads, MODES[usize::from(mode) % MODES.len()]);
});
