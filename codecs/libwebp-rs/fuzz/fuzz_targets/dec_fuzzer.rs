// libwebp v1.6.0, tests/fuzzer/dec_fuzzer.cc:
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

//! Port of `tests/fuzzer/dec_fuzzer.cc`: `WebPDecode` with the default
//! configuration on arbitrary bytes.

#![no_main]

use libfuzzer_sys::fuzz_target;
use webp::dec::buffer_dec::webp_free_dec_buffer;
use webp::dec::webp_dec::webp_decode;
use webp::webp::decode::{VP8StatusCode, WebPDecoderConfig, webp_init_decoder_config};

// upstream: tests/fuzzer/dec_fuzzer.cc DecodeWebP
fuzz_target!(|arbitrary_bytes: &[u8]| {
    let mut decoder_config = WebPDecoderConfig::default();
    assert!(
        webp_init_decoder_config(&mut decoder_config),
        "WebPInitDecoderConfig failed."
    );
    let status = webp_decode(arbitrary_bytes, &mut decoder_config);
    webp_free_dec_buffer(&mut decoder_config.output);
    // The decoding may fail (because the fuzzed input can be anything) but not
    // for these reasons.
    assert!(
        status != Err(VP8StatusCode::VP8_STATUS_SUSPENDED)
            && status != Err(VP8StatusCode::VP8_STATUS_USER_ABORT),
        "unexpected status {status:?}"
    );
});
