#![no_main]

// The harness runs *outside* the safe core and requires sandbox/process
// memory limits. Codec scratch remains outside the engine-managed budget.
use libfuzzer_sys::fuzz_target;

// One input runs at every AVIF depth its selector admits (10 and 8 for AVIF
// output); the former `transcode_own_avif` lane is merged here (2026-11-15).
fuzz_target!(|bytes: &[u8]| {
    for &depth in ferrisma_fuzz::avif_depths(bytes) {
        let _ = ferrisma_fuzz::run_case_at_depth(bytes, depth);
    }
});
