#![no_main]

// The harness runs *outside* the safe core and requires sandbox/process
// memory limits. Codec scratch remains outside the engine-managed budget.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    let _ = ferrixel_fuzz::run_case(bytes);
});
