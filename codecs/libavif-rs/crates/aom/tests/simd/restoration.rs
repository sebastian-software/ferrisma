// Supplementary transcription of the loop-restoration apply tests for the
// Neon instantiations:
// - upstream/aom/test/hiprec_convolve_test.cc with
//   test/hiprec_convolve_test_util.cc (AV1HiprecConvolveTest.CheckOutput and
//   AV1HighbdHiprecConvolveTest.CheckOutput): av1_wiener_convolve_add_src_neon
//   and av1_highbd_wiener_convolve_add_src_neon against their `_c` versions,
//   with the original parameter lists, kernel types, seeds, iteration counts,
//   buffer layout and exact-match checks.
// - upstream/aom/test/selfguided_filter_test.cc
//   (AV1SelfguidedFilterTest.CorrectnessTest and
//   AV1HighbdSelfguidedFilterTest.CorrectnessTest at 8, 10 and 12 bits):
//   av1_apply_selfguided_restoration_neon against
//   av1_apply_selfguided_restoration_c.
// The DISABLED_SpeedTest cases are not transcribed. No original test
// instantiates av1_selfguided_restoration_neon; `selfguided_restoration_*`
// below is a supplementary exact comparison against
// av1_selfguided_restoration_c on the inputs of the selfguided correctness
// test (not part of the original suite).
#![cfg(target_arch = "aarch64")]
use aom::av1__common__restoration::WienerConvolveParams;
use aom::simd::av1__common__arm__highbd_wiener_convolve_neon::av1_highbd_wiener_convolve_add_src_neon;
use aom::simd::av1__common__arm__selfguided_neon::{
    av1_apply_selfguided_restoration_neon, av1_selfguided_restoration_neon,
};
use aom::simd::av1__common__arm__wiener_convolve_neon::av1_wiener_convolve_add_src_neon;

use super::acm_random::ACMRandom;

type HiprecConvolveFunc = unsafe extern "C-unwind" fn(
    *const u8, isize, *mut u8, isize, *const i16, i32, *const i16, i32, i32, i32,
    *const WienerConvolveParams,
);
type HighbdHiprecConvolveFunc = unsafe extern "C-unwind" fn(
    *const u8, isize, *mut u8, isize, *const i16, i32, *const i16, i32, i32, i32,
    *const WienerConvolveParams, i32,
);
type SgrFunc = unsafe extern "C-unwind" fn(
    *const u8, i32, i32, i32, i32, *const i32, *mut u8, i32, *mut i32, i32, i32,
) -> i32;
type SelfguidedFunc = unsafe extern "C-unwind" fn(
    *const u8, i32, i32, i32, *mut i32, *mut i32, i32, i32, i32, i32,
) -> i32;

extern "C-unwind" {
    fn av1_wiener_convolve_add_src_c(src: *const u8, src_stride: isize, dst: *mut u8,
                                     dst_stride: isize, filter_x: *const i16, x_step_q4: i32,
                                     filter_y: *const i16, y_step_q4: i32, w: i32, h: i32,
                                     conv_params: *const WienerConvolveParams);
    fn av1_highbd_wiener_convolve_add_src_c(src: *const u8, src_stride: isize, dst: *mut u8,
                                            dst_stride: isize, filter_x: *const i16,
                                            x_step_q4: i32, filter_y: *const i16,
                                            y_step_q4: i32, w: i32, h: i32,
                                            conv_params: *const WienerConvolveParams, bd: i32);
    fn av1_apply_selfguided_restoration_c(dat8: *const u8, width: i32, height: i32, stride: i32,
                                          eps: i32, xqd: *const i32, dst8: *mut u8,
                                          dst_stride: i32, tmpbuf: *mut i32, bit_depth: i32,
                                          highbd: i32) -> i32;
    fn av1_selfguided_restoration_c(dgd8: *const u8, width: i32, height: i32, dgd_stride: i32,
                                    flt0: *mut i32, flt1: *mut i32, flt_stride: i32,
                                    sgr_params_idx: i32, bit_depth: i32, highbd: i32) -> i32;
    fn av1_loop_restoration_precal();
}

// av1/common/restoration.h and av1/common/convolve.h
const WIENER_ROUND0_BITS: i32 = 3;
const FILTER_BITS: i32 = 7;
const WIENER_FILT_TAP0_MINV: i16 = 3 - (1 << 4) / 2;
const WIENER_FILT_TAP1_MINV: i16 = -7 - (1 << 5) / 2;
const WIENER_FILT_TAP2_MINV: i16 = 15 - (1 << 6) / 2;
const WIENER_FILT_TAP0_MAXV: i16 = 3 - 1 + (1 << 4) / 2;
const WIENER_FILT_TAP1_MAXV: i16 = -7 - 1 + (1 << 5) / 2;
const WIENER_FILT_TAP2_MAXV: i16 = 15 - 1 + (1 << 6) / 2;
const RESTORATION_PROC_UNIT_SIZE: i32 = 64;
const RESTORATION_UNITPELS_MAX: usize = (256 * 3 / 2 + 2 * 3 + 16) * (256 * 3 / 2 + 2 * 3 + 8);
const SGRPROJ_PARAMS_BITS: i32 = 4;
const SGRPROJ_PRJ_BITS: i32 = 7;
const SGRPROJ_PRJ_MIN0: i32 = -(1 << SGRPROJ_PRJ_BITS) * 3 / 4;
const SGRPROJ_PRJ_MAX0: i32 = SGRPROJ_PRJ_MIN0 + (1 << SGRPROJ_PRJ_BITS) - 1;
const SGRPROJ_PRJ_MIN1: i32 = -(1 << SGRPROJ_PRJ_BITS) / 4;
const SGRPROJ_PRJ_MAX1: i32 = SGRPROJ_PRJ_MIN1 + (1 << SGRPROJ_PRJ_BITS) - 1;

/// get_conv_params_wiener (av1/common/convolve.h).
fn get_conv_params_wiener(bd: i32) -> WienerConvolveParams {
    let mut conv_params = WienerConvolveParams { round_0: WIENER_ROUND0_BITS, round_1: 0 };
    conv_params.round_1 = 2 * FILTER_BITS - conv_params.round_0;
    let intbufrange = bd + FILTER_BITS - conv_params.round_0 + 2;
    if intbufrange > 16 {
        conv_params.round_0 += intbufrange - 16;
        conv_params.round_1 -= intbufrange - 16;
    }
    conv_params
}

fn convert_to_byteptr<T>(p: *const T) -> *mut u8 {
    ((p as usize) >> 1) as *mut u8
}

#[repr(C, align(16))]
struct Aligned16<T>(T);

/// generate_kernels (test/hiprec_convolve_test_util.cc).
fn generate_kernels(rnd: &mut ACMRandom, hkernel: &mut [i16; 8], vkernel: &mut [i16; 8],
                    kernel_type: i32) {
    let pu = |rnd: &mut ACMRandom, range: i16| rnd.pseudo_uniform(range as i32) as i16;
    if kernel_type == 0 {
        // Low possible values for filter coefficients, 7-tap kernel
        for k in [&mut *hkernel, &mut *vkernel] {
            k[0] = WIENER_FILT_TAP0_MINV;
            k[6] = WIENER_FILT_TAP0_MINV;
            k[1] = WIENER_FILT_TAP1_MINV;
            k[5] = WIENER_FILT_TAP1_MINV;
            k[2] = WIENER_FILT_TAP2_MINV;
            k[4] = WIENER_FILT_TAP2_MINV;
        }
        hkernel[3] = -2 * (hkernel[0] + hkernel[1] + hkernel[2]);
        vkernel[3] = hkernel[3];
        hkernel[7] = 0;
        vkernel[7] = 0;
    } else if kernel_type == 1 {
        // Max possible values for filter coefficients, 7-tap kernel
        for k in [&mut *hkernel, &mut *vkernel] {
            k[0] = WIENER_FILT_TAP0_MAXV;
            k[6] = WIENER_FILT_TAP0_MAXV;
            k[1] = WIENER_FILT_TAP1_MAXV;
            k[5] = WIENER_FILT_TAP1_MAXV;
            k[2] = WIENER_FILT_TAP2_MAXV;
            k[4] = WIENER_FILT_TAP2_MAXV;
        }
        hkernel[3] = -2 * (hkernel[0] + hkernel[1] + hkernel[2]);
        vkernel[3] = hkernel[3];
        hkernel[7] = 0;
        vkernel[7] = 0;
    } else if kernel_type == 2 {
        // Randomly generated values for filter coefficients, 7-tap kernel
        hkernel[0] = WIENER_FILT_TAP0_MINV
            + pu(rnd, WIENER_FILT_TAP0_MAXV + 1 - WIENER_FILT_TAP0_MINV);
        hkernel[6] = hkernel[0];
        hkernel[1] = WIENER_FILT_TAP1_MINV
            + pu(rnd, WIENER_FILT_TAP1_MAXV + 1 - WIENER_FILT_TAP1_MINV);
        hkernel[5] = hkernel[1];
        hkernel[2] = WIENER_FILT_TAP2_MINV
            + pu(rnd, WIENER_FILT_TAP2_MAXV + 1 - WIENER_FILT_TAP2_MINV);
        hkernel[4] = hkernel[2];
        hkernel[3] = -2 * (hkernel[0] + hkernel[1] + hkernel[2]);
        hkernel[7] = 0;

        vkernel[0] = WIENER_FILT_TAP0_MINV
            + pu(rnd, WIENER_FILT_TAP0_MAXV + 2 - WIENER_FILT_TAP0_MINV);
        vkernel[6] = vkernel[0];
        vkernel[1] = WIENER_FILT_TAP1_MINV
            + pu(rnd, WIENER_FILT_TAP1_MAXV + 2 - WIENER_FILT_TAP1_MINV);
        vkernel[5] = vkernel[1];
        vkernel[2] = WIENER_FILT_TAP2_MINV
            + pu(rnd, WIENER_FILT_TAP2_MAXV + 2 - WIENER_FILT_TAP2_MINV);
        vkernel[4] = vkernel[2];
        vkernel[3] = -2 * (vkernel[0] + vkernel[1] + vkernel[2]);
        vkernel[7] = 0;
    } else if kernel_type == 3 {
        // Low possible values for filter coefficients, 5-tap kernel
        for k in [&mut *hkernel, &mut *vkernel] {
            k[0] = 0;
            k[6] = 0;
            k[1] = WIENER_FILT_TAP1_MINV;
            k[5] = WIENER_FILT_TAP1_MINV;
            k[2] = WIENER_FILT_TAP2_MINV;
            k[4] = WIENER_FILT_TAP2_MINV;
        }
        hkernel[3] = -2 * (hkernel[0] + hkernel[1] + hkernel[2]);
        vkernel[3] = hkernel[3];
        hkernel[7] = 0;
        vkernel[7] = 0;
    } else if kernel_type == 4 {
        // Max possible values for filter coefficients, 5-tap kernel
        for k in [&mut *hkernel, &mut *vkernel] {
            k[0] = 0;
            k[6] = 0;
            k[1] = WIENER_FILT_TAP1_MAXV;
            k[5] = WIENER_FILT_TAP1_MAXV;
            k[2] = WIENER_FILT_TAP2_MAXV;
            k[4] = WIENER_FILT_TAP2_MAXV;
        }
        hkernel[3] = -2 * (hkernel[0] + hkernel[1] + hkernel[2]);
        vkernel[3] = hkernel[3];
        hkernel[7] = 0;
        vkernel[7] = 0;
    } else {
        // Randomly generated values for filter coefficients, 5-tap kernel
        hkernel[0] = 0;
        hkernel[6] = 0;
        hkernel[1] = WIENER_FILT_TAP1_MINV
            + pu(rnd, WIENER_FILT_TAP1_MAXV + 1 - WIENER_FILT_TAP1_MINV);
        hkernel[5] = hkernel[1];
        hkernel[2] = WIENER_FILT_TAP2_MINV
            + pu(rnd, WIENER_FILT_TAP2_MAXV + 1 - WIENER_FILT_TAP2_MINV);
        hkernel[4] = hkernel[2];
        hkernel[3] = -2 * (hkernel[0] + hkernel[1] + hkernel[2]);
        hkernel[7] = 0;

        vkernel[0] = 0;
        vkernel[6] = 0;
        vkernel[1] = WIENER_FILT_TAP1_MINV
            + pu(rnd, WIENER_FILT_TAP1_MAXV + 2 - WIENER_FILT_TAP1_MINV);
        vkernel[5] = vkernel[1];
        vkernel[2] = WIENER_FILT_TAP2_MINV
            + pu(rnd, WIENER_FILT_TAP2_MAXV + 2 - WIENER_FILT_TAP2_MINV);
        vkernel[4] = vkernel[2];
        vkernel[3] = -2 * (vkernel[0] + vkernel[1] + vkernel[2]);
        vkernel[7] = 0;
    }
}

fn align_power_of_two(value: i32, n: i32) -> i32 {
    (value + (1 << n) - 1) & !((1 << n) - 1)
}

/// AV1HiprecConvolveTest::RunCheckOutput for one parameter; returns the
/// number of compared blocks.
fn hiprec_check_output(out_w: i32, out_h: i32, num_iters: i32,
                       test_impl: HiprecConvolveFunc) -> usize {
    let (w, h) = (128i32, 128i32);
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let conv_params = get_conv_params_wiener(8);
    let mut input = vec![0u8; (h * w) as usize];
    let output_n = (align_power_of_two(out_w, 4) * out_h) as usize;
    let mut output = vec![0u8; output_n];
    let mut output2 = vec![0u8; output_n];
    let mut hkernel = Aligned16([0i16; 8]);
    let mut vkernel = Aligned16([0i16; 8]);
    let mut blocks = 0;
    for kernel_type in 0..6 {
        generate_kernels(&mut rnd, &mut hkernel.0, &mut vkernel.0, kernel_type);
        for i in 0..num_iters {
            for v in input.iter_mut() {
                *v = rnd.rand8();
            }
            // Choose random locations within the source block
            let offset_r = 3 + rnd.pseudo_uniform(h - out_h - 7);
            let offset_c = 3 + rnd.pseudo_uniform(w - out_w - 7);
            // SAFETY: the source position keeps the 7-tap support inside the
            // 128x128 input; the outputs hold out_w x out_h samples.
            unsafe {
                let src = input.as_ptr().add((offset_r * w + offset_c) as usize);
                av1_wiener_convolve_add_src_c(src, w as isize, output.as_mut_ptr(), out_w as isize,
                                              hkernel.0.as_ptr(), 16, vkernel.0.as_ptr(), 16,
                                              out_w, out_h, &conv_params);
                test_impl(src, w as isize, output2.as_mut_ptr(), out_w as isize,
                          hkernel.0.as_ptr(), 16, vkernel.0.as_ptr(), 16, out_w, out_h,
                          &conv_params);
            }
            for j in 0..(out_w * out_h) as usize {
                assert_eq!(output[j], output2[j],
                           "{}x{} kernel_type {}: Pixel mismatch at index {} = ({}, {}) on iteration {}",
                           out_w, out_h, kernel_type, j, j as i32 % out_w, j as i32 / out_w, i);
            }
            blocks += 1;
        }
    }
    blocks
}

/// AV1HighbdHiprecConvolveTest::RunCheckOutput for one parameter.
fn highbd_hiprec_check_output(out_w: i32, out_h: i32, num_iters: i32, bd: i32,
                              test_impl: HighbdHiprecConvolveFunc) -> usize {
    let (w, h) = (128i32, 128i32);
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let conv_params = get_conv_params_wiener(bd);
    let mut input = vec![0u16; (h * w) as usize];
    let output_n = (align_power_of_two(out_w, 4) * out_h) as usize;
    let mut output = vec![0u16; output_n];
    let mut output2 = vec![0u16; output_n];
    let mut hkernel = Aligned16([0i16; 8]);
    let mut vkernel = Aligned16([0i16; 8]);

    for v in input.iter_mut() {
        *v = rnd.rand16() & ((1u16 << bd) - 1);
    }
    let input_ptr = convert_to_byteptr(input.as_ptr());
    let output_ptr = convert_to_byteptr(output.as_mut_ptr());
    let output2_ptr = convert_to_byteptr(output2.as_mut_ptr());
    let mut blocks = 0;
    for kernel_type in 0..6 {
        generate_kernels(&mut rnd, &mut hkernel.0, &mut vkernel.0, kernel_type);
        for i in 0..num_iters {
            // Choose random locations within the source block
            let offset_r = 3 + rnd.pseudo_uniform(h - out_h - 7);
            let offset_c = 3 + rnd.pseudo_uniform(w - out_w - 7);
            // SAFETY: as above, with CONVERT_TO_BYTEPTR pointers that the
            // kernels convert back before any access.
            unsafe {
                let src = input_ptr.wrapping_add((offset_r * w + offset_c) as usize);
                av1_highbd_wiener_convolve_add_src_c(src, w as isize, output_ptr, out_w as isize,
                                                     hkernel.0.as_ptr(), 16, vkernel.0.as_ptr(),
                                                     16, out_w, out_h, &conv_params, bd);
                test_impl(src, w as isize, output2_ptr, out_w as isize, hkernel.0.as_ptr(), 16,
                          vkernel.0.as_ptr(), 16, out_w, out_h, &conv_params, bd);
            }
            for j in 0..(out_w * out_h) as usize {
                assert_eq!(output[j], output2[j],
                           "{}x{} bd {} kernel_type {}: Pixel mismatch at index {} = ({}, {}) on iteration {}",
                           out_w, out_h, bd, kernel_type, j, j as i32 % out_w, j as i32 / out_w, i);
            }
            blocks += 1;
        }
    }
    blocks
}

#[test]
fn hiprec_convolve_neon_check_output() {
    let func: HiprecConvolveFunc = av1_wiener_convolve_add_src_neon;
    // libaom_test::AV1HiprecConvolve::BuildParams
    let params: [(i32, i32, i32); 14] = [
        (8, 8, 50000), (8, 4, 50000), (64, 24, 1000), (64, 64, 1000), (64, 56, 1000),
        (32, 8, 10000), (32, 28, 10000), (32, 32, 10000), (16, 34, 10000), (32, 34, 10000),
        (64, 34, 1000), (8, 17, 10000), (16, 17, 10000), (32, 17, 10000),
    ];
    let mut blocks = 0;
    for (out_w, out_h, iters) in params {
        blocks += hiprec_check_output(out_w, out_h, iters, func);
    }
    assert!(blocks > 0);
}

#[test]
fn highbd_hiprec_convolve_neon_check_output() {
    let func: HighbdHiprecConvolveFunc = av1_highbd_wiener_convolve_add_src_neon;
    // libaom_test::AV1HighbdHiprecConvolve::BuildParams
    let params: [(i32, i32, i32, i32); 9] = [
        (8, 8, 50000, 8), (64, 64, 1000, 8), (32, 8, 10000, 8), (8, 8, 50000, 10),
        (64, 64, 1000, 10), (32, 8, 10000, 10), (8, 8, 50000, 12), (64, 64, 1000, 12),
        (32, 8, 10000, 12),
    ];
    let mut blocks = 0;
    for (out_w, out_h, iters, bd) in params {
        blocks += highbd_hiprec_check_output(out_w, out_h, iters, bd, func);
    }
    assert!(blocks > 0);
}

/// AV1SelfguidedFilterTest::RunCorrectnessTest (bit_depth None) and
/// AV1HighbdSelfguidedFilterTest::RunCorrectnessTest (Some(bd)).
fn selfguided_correctness(tst_fun: SgrFunc, bit_depth: Option<i32>) {
    let pu_width = RESTORATION_PROC_UNIT_SIZE;
    let pu_height = RESTORATION_PROC_UNIT_SIZE;
    let (max_w, max_h, stride, out_stride) = (260i32, 260i32, 672i32, 672i32);
    const NUM_ITERS: i32 = 81;
    let highbd = bit_depth.is_some();
    let bd = bit_depth.unwrap_or(8);
    let mask: u16 = if highbd { ((1i32 << bd) - 1) as u16 } else { 0xFF };

    let mut input_ = vec![0u16; (stride * (max_h + 32)) as usize];
    let mut output_ = vec![0u16; (out_stride * (max_h + 32)) as usize];
    let mut output2_ = vec![0u16; (out_stride * (max_h + 32)) as usize];
    let mut input8_ = vec![0u8; (stride * (max_h + 32)) as usize];
    let mut output8_ = vec![0u8; (out_stride * (max_h + 32)) as usize];
    let mut output28_ = vec![0u8; (out_stride * (max_h + 32)) as usize];
    let mut tmpbuf = vec![0i32; RESTORATION_UNITPELS_MAX * 2];
    let input = (stride * 16 + 16) as usize;
    let output = (out_stride * 16 + 16) as usize;

    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    // SAFETY: av1_loop_restoration_precal has no effect in this configuration.
    unsafe { av1_loop_restoration_precal() };

    for i in 0..NUM_ITERS {
        for j in -16..(max_h + 16) {
            for k in -16..(max_w + 16) {
                let idx = (input as i32 + j * stride + k) as usize;
                let v = rnd.rand16() & mask;
                if highbd {
                    input_[idx] = v;
                } else {
                    input8_[idx] = v as u8;
                }
            }
        }
        let xqd: [i32; 2] = [
            SGRPROJ_PRJ_MIN0 + rnd.pseudo_uniform(SGRPROJ_PRJ_MAX0 + 1 - SGRPROJ_PRJ_MIN0),
            SGRPROJ_PRJ_MIN1 + rnd.pseudo_uniform(SGRPROJ_PRJ_MAX1 + 1 - SGRPROJ_PRJ_MIN1),
        ];
        let eps = rnd.pseudo_uniform(1 << SGRPROJ_PARAMS_BITS);

        // Test various tile sizes around 256x256
        let test_w = max_w - (i / 9);
        let test_h = max_h - (i % 9);

        let mut k = 0;
        while k < test_h {
            let mut j = 0;
            while j < test_w {
                let w = pu_width.min(test_w - j);
                let h = pu_height.min(test_h - k);
                let in_off = input + (k * stride + j) as usize;
                let out_off = output + (k * out_stride + j) as usize;
                // SAFETY: the tiles and their 3-pixel borders lie inside the
                // padded buffers, as in the original test.
                unsafe {
                    let (inp, out, out2) = if highbd {
                        (convert_to_byteptr(input_.as_ptr().add(in_off)),
                         convert_to_byteptr(output_.as_mut_ptr().add(out_off)),
                         convert_to_byteptr(output2_.as_mut_ptr().add(out_off)))
                    } else {
                        (input8_.as_mut_ptr().add(in_off), output8_.as_mut_ptr().add(out_off),
                         output28_.as_mut_ptr().add(out_off))
                    };
                    let ret_tst = tst_fun(inp, w, h, stride, eps, xqd.as_ptr(), out, out_stride,
                                          tmpbuf.as_mut_ptr(), bd, highbd as i32);
                    assert_eq!(ret_tst, 0);
                    let ret_c = av1_apply_selfguided_restoration_c(
                        inp, w, h, stride, eps, xqd.as_ptr(), out2, out_stride,
                        tmpbuf.as_mut_ptr(), bd, highbd as i32);
                    assert_eq!(ret_c, 0);
                }
                j += pu_width;
            }
            k += pu_height;
        }

        for j in 0..test_h {
            for k in 0..test_w {
                let idx = output + (j * out_stride + k) as usize;
                if highbd {
                    assert_eq!(output_[idx], output2_[idx],
                               "bd {} iteration {} eps {} at ({}, {})", bd, i, eps, k, j);
                } else {
                    assert_eq!(output8_[idx], output28_[idx],
                               "8-bit iteration {} eps {} at ({}, {})", i, eps, k, j);
                }
            }
        }
    }
}

#[test]
fn selfguided_filter_neon_correctness() {
    selfguided_correctness(av1_apply_selfguided_restoration_neon, None);
}

#[test]
fn highbd_selfguided_filter_neon_correctness() {
    for bd in [8, 10, 12] {
        selfguided_correctness(av1_apply_selfguided_restoration_neon, Some(bd));
    }
}

/// Supplementary (not in the original suite): av1_selfguided_restoration_neon
/// against av1_selfguided_restoration_c on the inputs and tiles of the
/// selfguided correctness test, comparing every flt0/flt1 sample the
/// parameter set produces.
fn selfguided_restoration_compare(tst_fun: SelfguidedFunc, bit_depth: Option<i32>) {
    let pu_width = RESTORATION_PROC_UNIT_SIZE;
    let pu_height = RESTORATION_PROC_UNIT_SIZE;
    let (max_w, max_h, stride) = (260i32, 260i32, 672i32);
    const NUM_ITERS: i32 = 81;
    let highbd = bit_depth.is_some();
    let bd = bit_depth.unwrap_or(8);
    let mask: u16 = if highbd { ((1i32 << bd) - 1) as u16 } else { 0xFF };

    let mut input_ = vec![0u16; (stride * (max_h + 32)) as usize];
    let mut input8_ = vec![0u8; (stride * (max_h + 32)) as usize];
    let n = (pu_width * pu_height) as usize;
    let input = (stride * 16 + 16) as usize;

    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    for i in 0..NUM_ITERS {
        for j in -16..(max_h + 16) {
            for k in -16..(max_w + 16) {
                let idx = (input as i32 + j * stride + k) as usize;
                let v = rnd.rand16() & mask;
                if highbd {
                    input_[idx] = v;
                } else {
                    input8_[idx] = v as u8;
                }
            }
        }
        let _xqd0 = rnd.pseudo_uniform(SGRPROJ_PRJ_MAX0 + 1 - SGRPROJ_PRJ_MIN0);
        let _xqd1 = rnd.pseudo_uniform(SGRPROJ_PRJ_MAX1 + 1 - SGRPROJ_PRJ_MIN1);
        let eps = rnd.pseudo_uniform(1 << SGRPROJ_PARAMS_BITS);
        let test_w = max_w - (i / 9);
        let test_h = max_h - (i % 9);

        let mut k = 0;
        while k < test_h {
            let mut j = 0;
            while j < test_w {
                let w = pu_width.min(test_w - j);
                let h = pu_height.min(test_h - k);
                let in_off = input + (k * stride + j) as usize;
                let mut flt = [vec![0i32; n], vec![0i32; n], vec![0i32; n], vec![0i32; n]];
                // SAFETY: as in selfguided_correctness; flt buffers hold w x h.
                unsafe {
                    let inp = if highbd {
                        convert_to_byteptr(input_.as_ptr().add(in_off))
                    } else {
                        input8_.as_mut_ptr().add(in_off)
                    };
                    let [a0, a1, b0, b1] = &mut flt;
                    assert_eq!(tst_fun(inp, w, h, stride, a0.as_mut_ptr(), a1.as_mut_ptr(), w, eps,
                                       bd, highbd as i32), 0);
                    assert_eq!(av1_selfguided_restoration_c(inp, w, h, stride, b0.as_mut_ptr(),
                                                            b1.as_mut_ptr(), w, eps, bd,
                                                            highbd as i32), 0);
                }
                for idx in 0..(w * h) as usize {
                    assert_eq!(flt[0][idx], flt[2][idx], "flt0 bd {} iter {} eps {} at {}", bd, i, eps, idx);
                    assert_eq!(flt[1][idx], flt[3][idx], "flt1 bd {} iter {} eps {} at {}", bd, i, eps, idx);
                }
                j += pu_width;
            }
            k += pu_height;
        }
    }
}

#[test]
fn selfguided_restoration_neon_supplementary() {
    selfguided_restoration_compare(av1_selfguided_restoration_neon, None);
    for bd in [8, 10, 12] {
        selfguided_restoration_compare(av1_selfguided_restoration_neon, Some(bd));
    }
}
