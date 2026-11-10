// libwebp v1.6.0, tests/fuzzer/simple_api_fuzzer.cc:
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

//! Port of `tests/fuzzer/simple_api_fuzzer.cc`: the simple decoding API and
//! the `...Into` functions, chosen by a hash of the input, sometimes with a
//! buffer that is too small.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use libfuzzer_sys::fuzz_target;
use webp::dec::webp_dec::{
    webp_decode_argb, webp_decode_argb_into, webp_decode_bgr, webp_decode_bgr_into,
    webp_decode_bgra, webp_decode_bgra_into, webp_decode_rgb, webp_decode_rgb_into,
    webp_decode_rgba, webp_decode_rgba_into, webp_decode_yuv, webp_decode_yuv_into, webp_get_info,
};

// upstream: tests/fuzzer/simple_api_fuzzer.cc SimpleApiTest
fuzz_target!(|data: &[u8]| {
    let size = data.len();
    let (mut w, mut h) = (0i32, 0i32);
    if !webp_get_info(data, Some(&mut w), Some(&mut h)) {
        return;
    }
    if w as usize * h as usize > fuzz_utils::FUZZ_PX_LIMIT {
        return;
    }

    let value = fuzz_utils::fuzz_hash(data);

    // For *Into functions, which decode into an external buffer, an
    // intentionally too small buffer can be given with low probability.
    // WEBP_REDUCE_CSP is not defined in the reference configuration.
    if value < 0x16 {
        let _ = webp_decode_rgba(data, Some(&mut w), Some(&mut h));
    } else if value < 0x2b {
        let _ = webp_decode_bgra(data, Some(&mut w), Some(&mut h));
    } else if value < 0x40 {
        let _ = webp_decode_argb(data, Some(&mut w), Some(&mut h));
    } else if value < 0x55 {
        let _ = webp_decode_rgb(data, Some(&mut w), Some(&mut h));
    } else if value < 0x6a {
        let _ = webp_decode_bgr(data, Some(&mut w), Some(&mut h));
    } else if value < 0x7f {
        // C: WebPDecodeYUV returns the luma pointer and writes u, v and the
        // strides; the buffer holds them here.
        let _ = webp_decode_yuv(data, Some(&mut w), Some(&mut h));
    } else if value < 0xe8 {
        let stride = (if value < 0xbe { 4 } else { 3 }) * w;
        let mut buf_size = (stride * h) as usize;
        if value % 0x10 == 0 {
            buf_size -= 1;
        }
        let mut ext_buf = vec![0u8; buf_size];
        if value < 0x94 {
            let _ = webp_decode_rgba_into(data, &mut ext_buf, stride);
        } else if value < 0xa9 {
            let _ = webp_decode_argb_into(data, &mut ext_buf, stride);
        } else if value < 0xbe {
            let _ = webp_decode_bgr_into(data, &mut ext_buf, stride);
        } else if value < 0xd3 {
            let _ = webp_decode_rgb_into(data, &mut ext_buf, stride);
        } else {
            let _ = webp_decode_bgra_into(data, &mut ext_buf, stride);
        }
    } else {
        let mut luma_size = (w * h) as usize;
        let uv_stride = (w + 1) / 2;
        let mut u_size = (uv_stride * (h + 1) / 2) as usize;
        let mut v_size = (uv_stride * (h + 1) / 2) as usize;
        if value % 0x10 == 0 {
            if size & 1 != 0 {
                luma_size -= 1;
            }
            if size & 2 != 0 {
                u_size -= 1;
            }
            if size & 4 != 0 {
                v_size -= 1;
            }
        }
        let mut luma_buf = vec![0u8; luma_size];
        let mut u_buf = vec![0u8; u_size];
        let mut v_buf = vec![0u8; v_size];
        let _ = webp_decode_yuv_into(
            data,
            &mut luma_buf,
            w, // luma_stride
            &mut u_buf,
            uv_stride,
            &mut v_buf,
            uv_stride,
        );
    }
});
