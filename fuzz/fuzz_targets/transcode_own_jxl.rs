#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    let _ = ferrisma_fuzz::run_own_jxl_case(bytes);
});
