#![no_main]

// JPEG XL output through the workspace libjxl-rs encoder (issues #27, #65).
// Every `Err` from the public API is an accepted outcome; a successful output
// must satisfy the invariants of `assert_jxl_output`.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    if let Some((request, Ok(output))) = ferrisma_fuzz::run_jxl_out_case(bytes) {
        ferrisma_fuzz::assert_jxl_output(&request, &output);
    }
});
