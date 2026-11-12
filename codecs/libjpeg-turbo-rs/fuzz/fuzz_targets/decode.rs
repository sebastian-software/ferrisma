//! The safe decoding API on arbitrary bytes (ADR 0009); `src/lib.rs` holds
//! the cases.

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| libjpeg_turbo_rs_fuzz::fuzz(data));
