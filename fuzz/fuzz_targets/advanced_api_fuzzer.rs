// libwebp v1.6.0, tests/fuzzer/advanced_api_fuzzer.cc:
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

//! Port of `tests/fuzzer/advanced_api_fuzzer.cc`: `WebPDecode` and the
//! incremental decoder with options drawn from the fuzzer and, in a second
//! pass, from the bytes of the input.
//!
//! The FuzzTest domains of the C target become one `arbitrary` tuple: the
//! cropping and scaling factor, the seven option flags, the colorspace in
//! `0..MODE_LAST` and the incremental flag, followed by the bitstream.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use libfuzzer_sys::fuzz_target;
use webp::dec::buffer_dec::webp_free_dec_buffer;
use webp::dec::idec_dec::{webp_i_append, webp_i_decode, webp_i_update};
use webp::dec::webp_dec::{webp_check_crop_dimensions, webp_decode};
use webp::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;
use webp::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPDecoderConfig, WebPDecoderOptions, webp_get_features,
    webp_init_decoder_config,
};

/// `sizeof(WebPDecoderOptions)`.
const OPTIONS_SIZE: usize = size_of::<WebPDecoderOptions>();

/// `memcpy(&config.options, bytes, sizeof(config.options))`: the 14 `int`
/// fields and the 5 `uint32_t` of padding in native byte order.
fn options_from_bytes(bytes: &[u8]) -> WebPDecoderOptions {
    let field = |i: usize| i32::from_ne_bytes(bytes[4 * i..4 * i + 4].try_into().unwrap());
    let mut pad = [0u32; 5];
    for (k, value) in pad.iter_mut().enumerate() {
        *value = u32::from_ne_bytes(bytes[56 + 4 * k..60 + 4 * k].try_into().unwrap());
    }
    WebPDecoderOptions {
        bypass_filtering: field(0),
        no_fancy_upsampling: field(1),
        use_cropping: field(2),
        crop_left: field(3),
        crop_top: field(4),
        crop_width: field(5),
        crop_height: field(6),
        use_scaling: field(7),
        scaled_width: field(8),
        scaled_height: field(9),
        use_threads: field(10),
        dithering_strength: field(11),
        flip: field(12),
        alpha_dithering_strength: field(13),
        pad,
    }
}

// upstream: tests/fuzzer/advanced_api_fuzzer.cc AdvancedApiTest
#[expect(clippy::too_many_lines, reason = "the C function")]
fn advanced_api_test(
    blob: &[u8],
    factor_u8: u8,
    flags: [bool; 7],
    colorspace: u8,
    incremental: bool,
) {
    let [
        flip,
        bypass_filtering,
        no_fancy_upsampling,
        use_threads,
        use_cropping,
        use_scaling,
        use_dithering,
    ] = flags;
    let mut config = WebPDecoderConfig::default();
    if !webp_init_decoder_config(&mut config) {
        return;
    }
    let data = blob;
    let size = data.len();
    if webp_get_features(data, &mut config.input).is_err() {
        return;
    }
    if config.input.width as usize * config.input.height as usize > fuzz_utils::FUZZ_PX_LIMIT {
        return;
    }

    // Using two independent criteria ensures that all combinations of options
    // can reach each path at the decoding stage, with meaningful differences.

    let value = fuzz_utils::fuzz_hash(data);
    let factor = f32::from(factor_u8) / 255.0; // 0-1

    config.options.flip = i32::from(flip);
    config.options.bypass_filtering = i32::from(bypass_filtering);
    config.options.no_fancy_upsampling = i32::from(no_fancy_upsampling);
    config.options.use_threads = i32::from(use_threads);
    if use_cropping {
        config.options.use_cropping = 1;
        config.options.crop_width = (config.input.width as f32 * (1.0 - factor)) as i32;
        config.options.crop_height = (config.input.height as f32 * (1.0 - factor)) as i32;
        config.options.crop_left = config.input.width - config.options.crop_width;
        config.options.crop_top = config.input.height - config.options.crop_height;
    }
    if use_dithering {
        let strength = (factor * 100.0) as i32;
        config.options.dithering_strength = strength;
        config.options.alpha_dithering_strength = 100 - strength;
    }
    if use_scaling {
        config.options.use_scaling = 1;
        config.options.scaled_width = (config.input.width as f32 * factor * 2.0) as i32;
        config.options.scaled_height = (config.input.height as f32 * factor * 2.0) as i32;
    }
    // The C domain is InRange(0, MODE_LAST - 1).
    config.output.colorspace = WEBP_CSP_MODE(u32::from(colorspace) % WEBP_CSP_MODE::MODE_LAST.0);

    for i in 0..2 {
        if i == 1 {
            // Use the bitstream data to generate extreme ranges for the options. An
            // alternative approach would be to use a custom corpus containing webp
            // files prepended with sizeof(config.options) zeroes to allow the fuzzer
            // to modify these independently.
            let data_offset = 50;
            if data_offset + OPTIONS_SIZE >= size {
                break;
            }
            config.options = options_from_bytes(&data[data_offset..data_offset + OPTIONS_SIZE]);

            // Skip easily avoidable out-of-memory fuzzing errors.
            if config.options.use_scaling != 0 {
                let mut input_width = config.input.width;
                let mut input_height = config.input.height;
                if config.options.use_cropping != 0 {
                    let cw = config.options.crop_width;
                    let ch = config.options.crop_height;
                    let x = config.options.crop_left & !1;
                    let y = config.options.crop_top & !1;
                    if webp_check_crop_dimensions(input_width, input_height, x, y, cw, ch) {
                        input_width = cw;
                        input_height = ch;
                    }
                }

                let mut scaled_width = config.options.scaled_width;
                let mut scaled_height = config.options.scaled_height;
                if webp_rescaler_get_scaled_dimensions(
                    input_width,
                    input_height,
                    &mut scaled_width,
                    &mut scaled_height,
                ) {
                    let mut fuzz_px_limit = fuzz_utils::FUZZ_PX_LIMIT as u64;
                    if scaled_width != config.input.width || scaled_height != config.input.height {
                        // Using the WebPRescalerImport internally can significantly slow
                        // down the execution. Avoid timeouts due to that.
                        fuzz_px_limit /= 2;
                    }
                    // A big output canvas can lead to out-of-memory and timeout issues,
                    // but a big internal working buffer can too. Also, rescaling from a
                    // very wide input image to a very tall canvas can be as slow as
                    // decoding a huge number of pixels. Avoid timeouts due to these.
                    let max_num_operations = scaled_width.max(config.input.width) as u64
                        * scaled_height.max(config.input.height) as u64;
                    if max_num_operations > fuzz_px_limit {
                        break;
                    }
                }
            }
        }
        if incremental {
            // Decodes incrementally in chunks of increasing size.
            let Some(mut idec) = webp_i_decode(&[], Some(&mut config)) else {
                return;
            };
            if size & 8 != 0 {
                let mut available_size = usize::from(value) + 1;
                loop {
                    if available_size > size {
                        available_size = size;
                    }
                    let status = webp_i_update(&mut idec, &data[..available_size]);
                    if status != Err(VP8StatusCode::VP8_STATUS_SUSPENDED) || available_size == size
                    {
                        break;
                    }
                    available_size *= 2;
                }
            } else {
                // WebPIAppend expects new data and its size with each call.
                // Implemented here by simply advancing the position in data.
                let mut new_data = 0;
                let mut new_size = usize::from(value) + 1;
                loop {
                    if new_data + new_size > size {
                        new_size = size - new_data;
                    }
                    let status = webp_i_append(&mut idec, &data[new_data..new_data + new_size]);
                    if status != Err(VP8StatusCode::VP8_STATUS_SUSPENDED) || new_size == 0 {
                        break;
                    }
                    new_data += new_size;
                    new_size *= 2;
                }
            }
            // C: WebPIDelete(idec).
            drop(idec);
        } else {
            let _ = webp_decode(data, &mut config);
        }

        webp_free_dec_buffer(&mut config.output);
    }
}

fuzz_target!(|input: (u8, [bool; 7], u8, bool, &[u8])| {
    let (factor_u8, flags, colorspace, incremental, blob) = input;
    advanced_api_test(blob, factor_u8, flags, colorspace, incremental);
});
