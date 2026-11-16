// Supplementary transcription of upstream/aom/test/horver_correlation_test.cc
// for the NEON, SSE4_1 and AVX2 instantiations: HorverTest
// (av1_get_horver_correlation_full_neon, _sse4_1 and _avx2 against
// av1_get_horver_correlation_full_c: RandomValues, ExtremeValues). The x86
// tests skip on a CPU without the feature. Each
// original TEST_P starts from a fresh fixture whose ACMRandom has the
// deterministic seed; the MAX_SB_SQUARE buffer, the MAX_SB_SIZE stride, the
// block sizes, the 1000 iterations and the value ranges follow the original.
// The speed test is not transcribed. Besides the original's tolerance of
// 1e-6, each result must have the same bits as `_c`: the port dispatches this
// kernel only if it returns exactly what `_c` returns (ADR 0056).
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

#[cfg(target_arch = "aarch64")]
use aom::simd::av1__encoder__arm__rdopt_neon::av1_get_horver_correlation_full_neon;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__rdopt_avx2::av1_get_horver_correlation_full_avx2;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__rdopt_sse4::av1_get_horver_correlation_full_sse4_1;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_get_horver_correlation_full_c(diff: *const i16, stride: i32, width: i32, height: i32,
                                         hcorr: *mut f32, vcorr: *mut f32);
}

type HorverFunc = unsafe extern "C-unwind" fn(*const i16, i32, i32, i32, *mut f32, *mut f32);

const MAX_SB_SIZE: i32 = 128;
const MAX_SB_SQUARE: usize = (MAX_SB_SIZE * MAX_SB_SIZE) as usize;
// av1/common/common_data.h, BLOCK_SIZES_ALL entries.
const BLOCK_SIZE_WIDE: [i32; 22] = [4, 4, 8, 8, 8, 16, 16, 16, 32, 32, 32, 64, 64, 64, 128, 128, 4,
                                    16, 8, 32, 16, 64];
const BLOCK_SIZE_HIGH: [i32; 22] = [4, 8, 4, 8, 16, 8, 16, 32, 16, 32, 64, 32, 64, 128, 64, 128,
                                    16, 4, 32, 8, 64, 16];

fn compare(target: HorverFunc, data: &[i16], w: i32, h: i32) {
    let (mut hcorr_ref, mut vcorr_ref) = (0.0f32, 0.0f32);
    let (mut hcorr_test, mut vcorr_test) = (0.0f32, 0.0f32);
    // SAFETY: the buffer holds MAX_SB_SIZE rows of MAX_SB_SIZE values.
    unsafe {
        av1_get_horver_correlation_full_c(data.as_ptr(), MAX_SB_SIZE, w, h, &mut hcorr_ref,
                                          &mut vcorr_ref);
        target(data.as_ptr(), MAX_SB_SIZE, w, h, &mut hcorr_test, &mut vcorr_test);
    }
    assert!(((hcorr_ref - hcorr_test) as f64).abs() <= 1e-6, "hcorr incorrect ({w}x{h})");
    assert!(((vcorr_ref - vcorr_test) as f64).abs() <= 1e-6, "vcorr incorrect ({w}x{h})");
    assert_eq!(hcorr_ref.to_bits(), hcorr_test.to_bits(), "hcorr differs from _c ({w}x{h})");
    assert_eq!(vcorr_ref.to_bits(), vcorr_test.to_bits(), "vcorr differs from _c ({w}x{h})");
}

/// HorverTest.RandomValues for one instantiation.
fn random_values(target: HorverFunc) {
    let mut rng = ACMRandom::new();
    let mut data_buf = vec![0i16; MAX_SB_SQUARE];
    for block_size in 0..BLOCK_SIZE_WIDE.len() {
        let (w, h) = (BLOCK_SIZE_WIDE[block_size], BLOCK_SIZE_HIGH[block_size]);
        for _ in 0..1000 {
            for value in data_buf.iter_mut() {
                *value = ((rng.rand16() as i32 % (1 << 12)) - (1 << 11)) as i16;
            }
            compare(target, &data_buf, w, h);
        }
    }
}

/// HorverTest.ExtremeValues for one instantiation.
fn extreme_values(target: HorverFunc) {
    // Most of get_horver_test is squaring and summing, so simply saturating
    // the whole buffer is mostly likely to cause an overflow.
    let data_buf = vec![((1 << 12) - 1) as i16; MAX_SB_SQUARE];
    for block_size in 0..BLOCK_SIZE_WIDE.len() {
        compare(target, &data_buf, BLOCK_SIZE_WIDE[block_size], BLOCK_SIZE_HIGH[block_size]);
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn horver_random_values() {
    random_values(av1_get_horver_correlation_full_neon);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn horver_extreme_values() {
    extreme_values(av1_get_horver_correlation_full_neon);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn horver_random_values_sse4_1() {
    if !std::is_x86_feature_detected!("sse4.1") {
        return;
    }
    random_values(av1_get_horver_correlation_full_sse4_1);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn horver_extreme_values_sse4_1() {
    if !std::is_x86_feature_detected!("sse4.1") {
        return;
    }
    extreme_values(av1_get_horver_correlation_full_sse4_1);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn horver_random_values_avx2() {
    if !std::is_x86_feature_detected!("avx2") {
        return;
    }
    random_values(av1_get_horver_correlation_full_avx2);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn horver_extreme_values_avx2() {
    if !std::is_x86_feature_detected!("avx2") {
        return;
    }
    extreme_values(av1_get_horver_correlation_full_avx2);
}
