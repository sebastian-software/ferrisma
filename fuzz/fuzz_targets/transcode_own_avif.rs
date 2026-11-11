#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|bytes: &[u8]| {
    for depth in [8, 10] {
        let _ = ferrixel_fuzz::run_own_avif_case(bytes, depth);
    }
});
