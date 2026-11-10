// libwebp v1.6.0, tests/fuzzer/fuzz_utils.h:
//
// Copyright 2018-2024 Google LLC
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

//! The parts of `tests/fuzzer/fuzz_utils.h` that the decoder targets use.
//! Include it with `#[path = "fuzz_utils.rs"] mod fuzz_utils;`.

#![allow(dead_code, reason = "each target uses a different subset")]

// upstream: tests/fuzzer/fuzz_utils.h kFuzzPxLimit
/// Limit on the pixels of a decoding: the value for a build without
/// AddressSanitizer or MemorySanitizer (cargo fuzz builds with AddressSanitizer
/// by default, where C uses 1024 * 1024 / 18; the targets keep the plain
/// limit, which only bounds their run time).
pub const FUZZ_PX_LIMIT: usize = 1024 * 1024;

// upstream: tests/fuzzer/fuzz_utils.h kFuzzFrameLimit
/// Limit on the frames that the animation targets read (WP15).
pub const FUZZ_FRAME_LIMIT: u32 = 3;

// upstream: tests/fuzzer/fuzz_utils.h FuzzHash
/// Reads and sums (up to) 128 spread-out bytes.
pub fn fuzz_hash(data: &[u8]) -> u8 {
    let mut value = 0u8;
    let mut incr = data.len() / 128;
    if incr == 0 {
        incr = 1;
    }
    let mut i = 0;
    while i < data.len() {
        value = value.wrapping_add(data[i]);
        i += incr;
    }
    value
}

//------------------------------------------------------------------------------
// The encoder parts of tests/fuzzer/fuzz_utils.h and fuzz_utils.cc (WP13).

use std::cell::RefCell;
use std::sync::LazyLock;

use webp::dec::webp_dec::{webp_decode, webp_get_info};
use webp::enc::config_enc::webp_validate_config;
use webp::enc::picture_csp_enc::webp_picture_import_rgba;
use webp::enc::picture_enc::{webp_memory_write, webp_picture_alloc, webp_picture_free};
use webp::enc::picture_rescale_enc::{webp_picture_crop, webp_picture_rescale};
use webp::webp::decode::{WEBP_CSP_MODE, WebPDecBufferPlanes, WebPDecoderConfig};
use webp::webp::encode::{
    WebPConfig, WebPEncCSP, WebPImageHint, WebPMemoryWriter, WebPPicture, WebPWriterFunction,
    webp_config_init, webp_picture_init,
};
use webp::webp::types::PlaneStorage;

/// The bytes of a `static const uint8_t k...Data[] = { 0x.., ... };` array.
fn header_bytes(header: &str) -> Vec<u8> {
    let start = header.find("Data[] = {").expect("the data array");
    header[start..]
        .split(|c: char| c == ',' || c == '{' || c == '}' || c.is_whitespace())
        .filter_map(|token| token.strip_prefix("0x"))
        .map(|hex| u8::from_str_radix(hex, 16).expect("a hex byte"))
        .collect()
}

// upstream: tests/fuzzer/fuzz_utils.h kImagesData
/// The three source images of `img_alpha.h`, `img_grid.h` and `img_peak.h`
/// (RGBA rows), with their widths and heights.
static IMAGES: LazyLock<[(i32, i32, Vec<u8>); 3]> = LazyLock::new(|| {
    [
        (
            32,
            32,
            header_bytes(include_str!(
                "../../upstream/libwebp/tests/fuzzer/img_alpha.h"
            )),
        ),
        (
            16,
            16,
            header_bytes(include_str!(
                "../../upstream/libwebp/tests/fuzzer/img_grid.h"
            )),
        ),
        (
            128,
            128,
            header_bytes(include_str!(
                "../../upstream/libwebp/tests/fuzzer/img_peak.h"
            )),
        ),
    ]
});

// upstream: tests/fuzzer/fuzz_utils.h kNumSourceImages
pub const NUM_SOURCE_IMAGES: usize = 3;

// upstream: tests/fuzzer/fuzz_utils.cc GetSourcePicture
/// The source picture `image_index`, imported from RGBA.
pub fn get_source_picture<'a>(image_index: usize, use_argb: bool) -> WebPPicture<'a> {
    let mut pic = WebPPicture::default();
    assert!(webp_picture_init(&mut pic));
    pic.use_argb = i32::from(use_argb);

    // Pick a source picture.
    let (width, height, data) = &IMAGES[image_index];
    assert_eq!(data.len(), (width * height * 4) as usize, "the image data");
    pic.width = *width;
    pic.height = *height;
    pic.argb_stride = pic.width * 4;

    // Read the bytes.
    let stride = pic.argb_stride;
    assert!(webp_picture_import_rgba(&mut pic, data, stride));
    pic
}

// upstream: tests/fuzzer/fuzz_utils.h ArbitraryWebPConfig
/// The configuration of `ArbitraryWebPConfig`: each of the 24 bytes picks a
/// value of its field's range (the FuzzTest domains, by modulo).
pub fn arbitrary_webp_config(values: [u8; 24]) -> WebPConfig {
    let pick = |i: usize, low: i32, high: i32| low + i32::from(values[i]) % (high - low + 1);
    let mut config = WebPConfig::default();
    assert!(webp_config_init(&mut config));
    config.lossless = pick(0, 0, 1);
    config.quality = pick(1, 0, 100) as f32;
    config.method = pick(2, 0, 6);
    config.image_hint =
        WebPImageHint(pick(3, 0, WebPImageHint::WEBP_HINT_LAST.0 as i32 - 1) as u32);
    config.segments = pick(4, 1, 4);
    config.sns_strength = pick(5, 0, 100);
    config.filter_strength = pick(6, 0, 100);
    config.filter_sharpness = pick(7, 0, 7);
    config.filter_type = pick(8, 0, 1);
    config.autofilter = pick(9, 0, 1);
    config.alpha_compression = pick(10, 0, 1);
    config.alpha_filtering = pick(11, 0, 2);
    config.alpha_quality = pick(12, 0, 100);
    config.pass = pick(13, 1, 10);
    config.show_compressed = 1;
    config.preprocessing = pick(14, 0, 2);
    config.partitions = pick(15, 0, 3);
    config.partition_limit = 10 * pick(16, 0, 10);
    config.emulate_jpeg_size = pick(17, 0, 1);
    config.thread_level = pick(18, 0, 1);
    config.low_memory = pick(19, 0, 1);
    config.near_lossless = 20 * pick(20, 0, 5);
    config.exact = pick(21, 0, 1);
    config.use_delta_palette = pick(22, 0, 1);
    config.use_sharp_yuv = pick(23, 0, 1);
    assert!(webp_validate_config(&config));
    config
}

// upstream: tests/fuzzer/fuzz_utils.h CropOrScaleParams
/// Whether and how to crop or scale the source picture.
#[derive(Clone, Copy, Debug, Default)]
pub struct CropOrScaleParams {
    pub alter_input: bool,
    pub crop_or_scale: bool,
    pub width_ratio: i32,
    pub height_ratio: i32,
    pub left_ratio: i32,
    pub top_ratio: i32,
}

// upstream: tests/fuzzer/fuzz_utils.h ArbitraryCropOrScaleParams
/// The parameters of `ArbitraryCropOrScaleParams`: two optional pairs of
/// ratios in `1..=8`.
pub fn arbitrary_crop_or_scale_params(
    width_height_ratio: Option<(u8, u8)>,
    left_top_ratio: Option<(u8, u8)>,
) -> CropOrScaleParams {
    let ratio = |value: u8| 1 + i32::from(value) % 8;
    let mut params = CropOrScaleParams {
        alter_input: width_height_ratio.is_some(),
        ..CropOrScaleParams::default()
    };
    if let Some((width, height)) = width_height_ratio {
        params.width_ratio = ratio(width);
        params.height_ratio = ratio(height);
        params.crop_or_scale = left_top_ratio.is_some();
        if let Some((left, top)) = left_top_ratio {
            params.left_ratio = ratio(left);
            params.top_ratio = ratio(top);
        }
    }
    params
}

// upstream: tests/fuzzer/fuzz_utils.cc CropOrScale
/// Crops or scales a picture according to the given params.
pub fn crop_or_scale(pic: &mut WebPPicture<'_>, params: &CropOrScaleParams) -> bool {
    if params.alter_input {
        if params.crop_or_scale {
            let cropped_width = 1.max(pic.width / params.width_ratio);
            let cropped_height = 1.max(pic.height / params.height_ratio);
            let cropped_left = (pic.width - cropped_width) / params.left_ratio;
            let cropped_top = (pic.height - cropped_height) / params.top_ratio;
            return webp_picture_crop(
                pic,
                cropped_left,
                cropped_top,
                cropped_width,
                cropped_height,
            );
        }
        let scaled_width = 1 + (pic.width * params.width_ratio) / 8;
        let scaled_height = 1 + (pic.height * params.height_ratio) / 8;
        return webp_picture_rescale(pic, scaled_width, scaled_height).is_ok();
    }
    true
}

// upstream: tests/fuzzer/fuzz_utils.cc IsImageTooBig
/// Whether the WebP image is too big for the fuzzer. The JPEG half of the C
/// function (`SjpegDimensions`) has no counterpart: the Rust targets read no
/// JPEG.
pub fn is_image_too_big(data: &[u8]) -> bool {
    let (mut width, mut height) = (0, 0);
    if webp_get_info(data, Some(&mut width), Some(&mut height)) {
        // Look at the number of 8x8px blocks rather than the overall pixel count
        // when comparing to memory and duration thresholds.
        let ceiled_width = (width as usize).div_ceil(8) * 8;
        let ceiled_height = (height as usize).div_ceil(8) * 8;
        // Threshold to avoid out-of-memory and timeout issues.
        if ceiled_width * ceiled_height > FUZZ_PX_LIMIT {
            return true;
        }
    }
    false
}

/// The reader of `imageio/webpdec.c` (`ReadWebP` with `keep_alpha`, no
/// metadata), with the translation's decoder: it decodes into the library's
/// memory and copies the samples into the planes that `WebPPictureAlloc` gave
/// the picture, where C decodes into them directly. imageio's PNG, JPEG, TIFF
/// and PNM readers have no Rust counterpart, so other inputs are not read.
pub fn read_webp(data: &[u8], pic: &mut WebPPicture<'_>) -> bool {
    let mut config = WebPDecoderConfig::default();
    if data.is_empty() || webp::webp::decode::webp_get_features(data, &mut config.input).is_err() {
        return false;
    }
    let has_alpha = config.input.has_alpha != 0;
    pic.width = config.input.width;
    pic.height = config.input.height;
    if pic.use_argb == 0 {
        pic.colorspace = if has_alpha {
            WebPEncCSP::WEBP_YUV420A
        } else {
            WebPEncCSP::WEBP_YUV420
        };
    }
    if webp_picture_alloc(pic).is_err() {
        return false;
    }
    config.output.colorspace = if pic.use_argb != 0 {
        if cfg!(target_endian = "big") {
            WEBP_CSP_MODE::MODE_ARGB
        } else {
            WEBP_CSP_MODE::MODE_BGRA
        }
    } else if has_alpha {
        WEBP_CSP_MODE::MODE_YUVA
    } else {
        WEBP_CSP_MODE::MODE_YUV
    };
    if webp_decode(data, &mut config).is_err() {
        webp_picture_free(pic);
        return false;
    }
    let PlaneStorage::Internal(samples) = &config.output.storage else {
        unreachable!("library memory");
    };
    let (w, h) = (pic.width as usize, pic.height as usize);
    match &config.output.u {
        WebPDecBufferPlanes::RGBA(rgba) => {
            let PlaneStorage::Internal(argb) = &mut pic.argb_storage else {
                unreachable!("allocated");
            };
            let base = pic.argb.expect("allocated").offset;
            for y in 0..h {
                for x in 0..w {
                    let at = rgba.rgba.offset + y * rgba.stride as usize + 4 * x;
                    argb[base + y * pic.argb_stride as usize + x] =
                        u32::from_ne_bytes(samples[at..at + 4].try_into().unwrap());
                }
            }
        }
        WebPDecBufferPlanes::YUVA(yuva) => {
            let PlaneStorage::Internal(planes) = &mut pic.yuva else {
                unreachable!("allocated");
            };
            let mut copy =
                |from: usize, from_stride: i32, to: usize, to_stride: i32, w: usize, h: usize| {
                    for y in 0..h {
                        let src = from + y * from_stride as usize;
                        let dst = to + y * to_stride as usize;
                        planes[dst..dst + w].copy_from_slice(&samples[src..src + w]);
                    }
                };
            let (uv_w, uv_h) = (w.div_ceil(2), h.div_ceil(2));
            copy(
                yuva.y.offset,
                yuva.y_stride,
                pic.y.unwrap().offset,
                pic.y_stride,
                w,
                h,
            );
            copy(
                yuva.u.offset,
                yuva.u_stride,
                pic.u.unwrap().offset,
                pic.uv_stride,
                uv_w,
                uv_h,
            );
            copy(
                yuva.v.offset,
                yuva.v_stride,
                pic.v.unwrap().offset,
                pic.uv_stride,
                uv_w,
                uv_h,
            );
            if let (Some(a), Some(pic_a)) = (yuva.a, pic.a) {
                copy(a.offset, yuva.a_stride, pic_a.offset, pic.a_stride, w, h);
            }
        }
    }
    true
}

/// A `WebPMemoryWriter` and the writer closure that fills it (C: `pic.writer =
/// WebPMemoryWrite; pic.custom_ptr = &memory_writer;`).
pub fn memory_writer(writer: &RefCell<WebPMemoryWriter>) -> WebPWriterFunction<'_> {
    Box::new(move |data: &[u8]| webp_memory_write(&mut writer.borrow_mut(), data))
}
