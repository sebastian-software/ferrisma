// Supplementary transcription of upstream/aom/test/model_rd_test.cc for the
// SSE2 instantiation: InterpCubicTest.CheckOutput
// (av1_interp_cubic_rate_dist_sse2 against av1_interp_cubic_rate_dist_c).
// The fixture's ACMRandom has the deterministic seed; the 10000 iterations,
// the value ranges of GenerateRandomDouble and the exact equality checks
// follow the original. The x87 precision switch of the original is not needed
// on x86-64, where both functions compute in SSE2 double precision. The speed
// test is not transcribed; the test skips on a CPU without SSE2.
#![cfg(target_arch = "x86_64")]

use aom::simd::av1__encoder__x86__model_rd_sse2::av1_interp_cubic_rate_dist_sse2;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_interp_cubic_rate_dist_c(p1: *const f64, p2: *const f64, x: f64, rate_dist_f: *mut f64);
}

fn generate_random_double(rnd: &mut ACMRandom, min: f64, max: f64) -> f64 {
    min + (rnd.rand31() as f64 / ((1u32 << 31) - 1) as f64) * (max - min)
}

#[test]
fn interp_cubic_check_output_sse2() {
    if !std::is_x86_feature_detected!("sse2") {
        return;
    }
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let (mut p1, mut p2) = ([0f64; 4], [0f64; 4]);
    let (mut out_ref, mut out_mod) = ([0f64; 2], [0f64; 2]);
    for _ in 0..10000 {
        for i in 0..4 {
            p1[i] = generate_random_double(&mut rnd, 0.0000, 4096.000000);
            p2[i] = generate_random_double(&mut rnd, 0.0000, 16.0000);
        }
        let x = generate_random_double(&mut rnd, 0.0000, 1.0000);
        // SAFETY: p1 and p2 hold 4 values, the outputs 2.
        unsafe {
            av1_interp_cubic_rate_dist_c(p1.as_ptr(), p2.as_ptr(), x, out_ref.as_mut_ptr());
            av1_interp_cubic_rate_dist_sse2(p1.as_ptr(), p2.as_ptr(), x, out_mod.as_mut_ptr());
        }
        assert_eq!(out_ref[0], out_mod[0], "Error: rate_f value mismatch");
        assert_eq!(out_ref[1], out_mod[1], "Error: distbysse_f value mismatch");
    }
}
