// libwebp v1.6.0, tests/fuzzer/mux_demux_api_fuzzer.cc:
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

//! Port of `tests/fuzzer/mux_demux_api_fuzzer.cc`: arbitrary bytes through the
//! mux API (`WebPMuxCreate` with copying when bit 1 of the size is set, the
//! chunk, feature, animation and frame getters) or the demux API (`WebPDemux`,
//! or `WebPDemuxPartial` when bit 1 of the size is set, and the chunk and frame
//! iterators). The FuzzTest domains (a string of up to `kMaxWebPFileSize + 1`
//! bytes and the boolean) become one `arbitrary` tuple; libFuzzer's `-max_len`
//! bounds the string.

#![no_main]

#[path = "fuzz_utils.rs"]
mod fuzz_utils;

use std::borrow::Cow;

use libfuzzer_sys::fuzz_target;
use webp::demux::demux::{
    webp_demux_delete, webp_demux_get_chunk, webp_demux_get_frame, webp_demux_next_chunk,
    webp_demux_next_frame, webp_demux_prev_chunk, webp_demux_release_chunk_iterator,
    webp_demux_release_iterator,
};
use webp::mux::muxedit::webp_mux_delete;
use webp::mux::muxread::{
    webp_mux_get_animation_params, webp_mux_get_chunk, webp_mux_get_features, webp_mux_get_frame,
};
use webp::webp::demux::{WebPDemuxState, webp_demux, webp_demux_partial};
use webp::webp::mux::{WebPMuxError, webp_mux_create};
use webp::webp::mux_types::{WebPData, webp_data_clear};

// upstream: tests/fuzzer/mux_demux_api_fuzzer.cc MuxDemuxApiTest
fn mux_demux_api_test(data_in: &[u8], use_mux_api: bool) {
    let size = data_in.len();
    let webp_data = WebPData {
        bytes: Cow::Borrowed(data_in),
    };

    // Extracted chunks and frames are not processed or decoded,
    // which is already covered extensively by the other fuzz targets.

    if use_mux_api {
        // Mux API
        let Some(mux) = webp_mux_create(&webp_data, (size & 2) as i32) else {
            return;
        };

        let _ = webp_mux_get_chunk(&mux, b"EXIF");
        let _ = webp_mux_get_chunk(&mux, b"ICCP");
        let _ = webp_mux_get_chunk(&mux, b"FUZZ"); // unknown

        let _ = webp_mux_get_features(&mux);

        let _ = webp_mux_get_animation_params(&mux);

        for i in 0..fuzz_utils::FUZZ_FRAME_LIMIT {
            match webp_mux_get_frame(&mux, i + 1) {
                Err(WebPMuxError::WEBP_MUX_NOT_FOUND) => break,
                Ok(mut info) => webp_data_clear(&mut info.bitstream),
                Err(_) => {}
            }
        }

        webp_mux_delete(mux);
    } else {
        // Demux API
        let demux = if size & 2 != 0 {
            let (demux, state) = webp_demux_partial(&webp_data);
            if state < WebPDemuxState::WEBP_DEMUX_PARSED_HEADER {
                if let Some(demux) = demux {
                    webp_demux_delete(demux);
                }
                return;
            }
            demux
        } else {
            webp_demux(&webp_data)
        };
        let Some(demux) = demux else {
            return;
        };

        if let Some(mut chunk_iter) = webp_demux_get_chunk(&demux, *b"EXIF", 1) {
            let _ = webp_demux_next_chunk(&mut chunk_iter);
            webp_demux_release_chunk_iterator(&mut chunk_iter);
        }
        if let Some(mut chunk_iter) = webp_demux_get_chunk(&demux, *b"ICCP", 0) {
            // 0 == last
            let _ = webp_demux_prev_chunk(&mut chunk_iter);
            webp_demux_release_chunk_iterator(&mut chunk_iter);
        }
        // Skips FUZZ because the Demux API has no concept of (un)known chunks.

        if let Some(mut iter) = webp_demux_get_frame(&demux, 1) {
            for _ in 1..fuzz_utils::FUZZ_FRAME_LIMIT {
                if !webp_demux_next_frame(&mut iter) {
                    break;
                }
            }
            webp_demux_release_iterator(&mut iter);
        }
        webp_demux_delete(demux);
    }
}

fuzz_target!(|input: (bool, &[u8])| {
    let (use_mux_api, data) = input;
    mux_demux_api_test(data, use_mux_api);
});
