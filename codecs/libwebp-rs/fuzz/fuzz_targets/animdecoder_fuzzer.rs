// libwebp v1.6.0, tests/fuzzer/animdecoder_fuzzer.cc:
//
// Copyright 2020 Google Inc.
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

//! Port of `tests/fuzzer/animdecoder_fuzzer.cc`: the animation decoder with the
//! default options on arbitrary bytes, decoding every frame. The FuzzTest domain
//! (strings up to `kMaxWebPFileSize + 1` bytes, 4 GiB) is libFuzzer's input,
//! whose `-max_len` is far smaller.

#![no_main]

use std::borrow::Cow;

use libfuzzer_sys::fuzz_target;
use webp::demux::anim_decode::{
    webp_anim_decoder_delete, webp_anim_decoder_get_info, webp_anim_decoder_get_next,
    webp_anim_decoder_has_more_frames,
};
use webp::webp::decode::{WebPBitstreamFeatures, webp_get_features};
use webp::webp::demux::webp_anim_decoder_new;
use webp::webp::mux_types::WebPData;

// upstream: imageio/imageio_util.c ImgIoUtilCheckSizeArgumentsOverflow
/// Checks that `stride * height` fits in `size_t` and that `stride` is
/// representable as an `int` (`WEBP_MAX_IMAGE_SIZE` is not defined, as in
/// upstream's CMake build).
fn img_io_util_check_size_arguments_overflow(stride: u64, height: usize) -> bool {
    let total_size = stride.wrapping_mul(height as u64);
    let mut ok = usize::try_from(total_size).is_ok();
    // check that 'stride' is representable as int:
    ok = ok && i64::from(stride as i32) as u64 == stride;
    ok
}

// upstream: tests/fuzzer/animdecoder_fuzzer.cc AnimDecoderTest
fuzz_target!(|data: &[u8]| {
    // WebPAnimDecoderGetInfo() is too late to check the canvas size as
    // WebPAnimDecoderNew() will handle the allocations.
    const K_MAX_NUM_BYTES: usize = 2_684_354_560; // RSS (resident set size) limit.
    const K_MAX_NUM_PIXELS: usize = K_MAX_NUM_BYTES / 4; // At most ARGB.
    const K_MAX_NUM_PIXELS_SAFE: usize = K_MAX_NUM_PIXELS / 2; // Allow one buffer copy.
    let mut features = WebPBitstreamFeatures::default();
    if webp_get_features(data, &mut features).is_ok()
        && (!img_io_util_check_size_arguments_overflow(
            i64::from(features.width * 4) as u64,
            features.height as usize,
        ) || features.width as usize * features.height as usize > K_MAX_NUM_PIXELS_SAFE)
    {
        return;
    }

    // decode everything as an animation
    let webp_data = WebPData {
        bytes: Cow::Borrowed(data),
    };
    let Some(mut dec) = webp_anim_decoder_new(&webp_data, None) else {
        return;
    };

    let info = webp_anim_decoder_get_info(&dec);
    if img_io_util_check_size_arguments_overflow(
        u64::from(info.canvas_width.wrapping_mul(4)),
        info.canvas_height as usize,
    ) {
        while webp_anim_decoder_has_more_frames(&dec) {
            if webp_anim_decoder_get_next(&mut dec).is_none() {
                break;
            }
        }
    }
    webp_anim_decoder_delete(dec);
});
