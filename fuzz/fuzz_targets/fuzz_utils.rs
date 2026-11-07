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
