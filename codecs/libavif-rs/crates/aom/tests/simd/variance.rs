// Supplementary transcription of upstream/aom/test/variance_test.cc for the
// block variance, block MSE and SSE/sum kernels the still-image encoder
// dispatches: AvxVarianceTest (Zero, Ref, RefStride, OneQuarter), AvxMseTest
// (RefMse, MaxMse), GetSseSum8x8QuadTest and GetSseSum16x16DualTest (RefMseSum,
// MinSseSum, MaxMseSum), AvxHBDVarianceTest (Zero, Ref, RefStride, OneQuarter)
// and AvxHBDMseTest (RefMse, MaxMse), for the NEON, SSE2, SSE4_1 and AVX2
// instantiations of the functions the port dispatches. Seeds, parameter
// lists (in the original order, including the NEON list's repeated 64x64
// entry), buffer sizes, strides and the exact checks follow the original;
// every TEST_P starts from a fresh SetUp (zeroed buffers, ACMRandom reset to
// the deterministic seed). The speed tests are not transcribed, nor the
// SSE2 high-bitdepth instantiations: those kernels call the NASM functions of
// highbd_variance_impl_sse2.asm and stay on `_c` in the port.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

use super::acm_random::ACMRandom;

type VarianceMxNFunc = unsafe extern "C-unwind" fn(*const u8, i32, *const u8, i32, *mut u32) -> u32;
type GetSseSum8x8QuadFunc =
    unsafe extern "C-unwind" fn(*const u8, i32, *const u8, i32, *mut u32, *mut i32, *mut u32, *mut i32, *mut u32);
type GetSseSum16x16DualFunc =
    unsafe extern "C-unwind" fn(*const u8, i32, *const u8, i32, *mut u32, *mut u32, *mut i32, *mut u32);

extern "C-unwind" {
    fn aom_get_var_sse_sum_8x8_quad_c(a: *const u8, a_stride: i32, b: *const u8, b_stride: i32,
                                      sse8x8: *mut u32, sum8x8: *mut i32, tot_sse: *mut u32,
                                      tot_sum: *mut i32, var8x8: *mut u32);
    fn aom_get_var_sse_sum_16x16_dual_c(a: *const u8, a_stride: i32, b: *const u8, b_stride: i32,
                                        sse16x16: *mut u32, tot_sse: *mut u32, tot_sum: *mut i32,
                                        var16x16: *mut u32);
}

/// CONVERT_TO_BYTEPTR (aom_dsp/aom_dsp_common.h).
fn convert_to_byteptr(p: *const u16) -> *const u8 {
    ((p as usize) >> 1) as *const u8
}

/// TestParams<Func>: a bit depth of 0 means the 8-bit (low bit depth) path.
#[derive(Clone, Copy)]
struct Params<F> {
    log2width: i32,
    log2height: i32,
    width: i32,
    height: i32,
    block_size: i32,
    func: F,
    bit_depth: i32,
    use_high_bit_depth: bool,
    mask: u32,
}

impl<F> Params<F> {
    const fn new(log2w: i32, log2h: i32, func: F, bit_depth_value: i32) -> Self {
        let use_high_bit_depth = bit_depth_value > 0;
        let bit_depth = if use_high_bit_depth { bit_depth_value } else { 8 };
        Params {
            log2width: log2w,
            log2height: log2h,
            width: 1 << log2w,
            height: 1 << log2h,
            block_size: (1 << log2w) * (1 << log2h),
            func,
            bit_depth,
            use_high_bit_depth,
            mask: (1u32 << bit_depth) - 1,
        }
    }
}

/// RoundHighBitDepth().
fn round_high_bit_depth(bit_depth: i32, se: &mut i64, sse: &mut u64) {
    match bit_depth {
        12 => {
            *sse = (*sse + 128) >> 8;
            *se = (*se + 8) >> 4;
        }
        10 => {
            *sse = (*sse + 8) >> 4;
            *se = (*se + 2) >> 2;
        }
        _ => {}
    }
}

/// MainTestClass: the source and reference blocks of one parameter set.
struct Main<F> {
    p: Params<F>,
    rnd: ACMRandom,
    src8: Vec<u8>,
    ref8: Vec<u8>,
    src16: Vec<u16>,
    ref16: Vec<u16>,
}

impl<F: Copy> Main<F> {
    /// SetUp().
    fn new(p: Params<F>) -> Self {
        let n = p.block_size as usize;
        Main {
            p,
            rnd: ACMRandom::with_seed(ACMRandom::deterministic_seed()),
            src8: vec![0; n],
            ref8: vec![0; n],
            src16: vec![0; n],
            ref16: vec![0; n],
        }
    }

    fn byte_shift(&self) -> i32 {
        self.p.bit_depth - 8
    }

    /// src_ and ref_ as the functions receive them (CONVERT_TO_BYTEPTR form
    /// at high bit depth).
    fn ptrs(&self) -> (*const u8, *const u8) {
        if self.p.use_high_bit_depth {
            (convert_to_byteptr(self.src16.as_ptr()), convert_to_byteptr(self.ref16.as_ptr()))
        } else {
            (self.src8.as_ptr(), self.ref8.as_ptr())
        }
    }

    fn fill_random(&mut self, src_ind: usize, ref_ind: usize) {
        if !self.p.use_high_bit_depth {
            self.src8[src_ind] = self.rnd.rand8();
            self.ref8[ref_ind] = self.rnd.rand8();
        } else {
            self.src16[src_ind] = (self.rnd.rand16() as u32 & self.p.mask) as u16;
            self.ref16[ref_ind] = (self.rnd.rand16() as u32 & self.p.mask) as u16;
        }
    }

    /// variance_ref().
    fn variance_ref(&self, src_stride: i32, ref_stride: i32, sse_ptr: &mut u32) -> u32 {
        let mut se: i64 = 0;
        let mut sse: u64 = 0;
        for y in 0..self.p.height {
            for x in 0..self.p.width {
                let si = (y * src_stride + x) as usize;
                let ri = (y * ref_stride + x) as usize;
                let diff: i32 = if !self.p.use_high_bit_depth {
                    self.src8[si] as i32 - self.ref8[ri] as i32
                } else {
                    self.src16[si] as i32 - self.ref16[ri] as i32
                };
                se += diff as i64;
                sse = sse.wrapping_add((diff * diff) as u32 as u64);
            }
        }
        round_high_bit_depth(self.p.bit_depth, &mut se, &mut sse);
        *sse_ptr = sse as u32;
        sse.wrapping_sub(((se * se) >> (self.p.log2width + self.p.log2height)) as u64) as u32
    }
}

impl Main<VarianceMxNFunc> {
    fn call(&self, src_stride: i32, ref_stride: i32, sse: &mut u32) -> u32 {
        let (s, r) = self.ptrs();
        // SAFETY: width x height samples at the given strides lie in the
        // block_size buffers (a stride of 0 repeats the first row).
        unsafe { (self.p.func)(s, src_stride, r, ref_stride, sse) }
    }

    /// ZeroTest().
    fn zero_test(&mut self) {
        let w = self.p.width;
        for i in 0..=255i32 {
            if !self.p.use_high_bit_depth {
                self.src8.fill(i as u8);
            } else {
                let v = (i << self.byte_shift()) as u16;
                self.src16.fill(v);
            }
            for j in 0..=255i32 {
                if !self.p.use_high_bit_depth {
                    self.ref8.fill(j as u8);
                } else {
                    let v = (j << self.byte_shift()) as u16;
                    self.ref16.fill(v);
                }
                let mut sse = 0u32;
                let var = self.call(w, w, &mut sse);
                assert_eq!(0, var, "src values: {i} ref values: {j}");
            }
        }
    }

    /// RefTest().
    fn ref_test(&mut self) {
        for i in 0..10 {
            for j in 0..self.p.block_size as usize {
                self.fill_random(j, j);
            }
            let (mut sse1, mut sse2) = (0u32, 0u32);
            let stride = self.p.width;
            let var1 = self.call(stride, stride, &mut sse1);
            let var2 = self.variance_ref(stride, stride, &mut sse2);
            assert_eq!(sse1, sse2, "Error at test index: {i}");
            assert_eq!(var1, var2, "Error at test index: {i}");
        }
    }

    /// RefStrideTest().
    fn ref_stride_test(&mut self) {
        let w = self.p.width;
        for i in 0..10 {
            let ref_stride = (i & 1) * w;
            let src_stride = ((i >> 1) & 1) * w;
            for j in 0..self.p.block_size {
                let ref_ind = ((j / w) * ref_stride + j % w) as usize;
                let src_ind = ((j / w) * src_stride + j % w) as usize;
                self.fill_random(src_ind, ref_ind);
            }
            let (mut sse1, mut sse2) = (0u32, 0u32);
            let var1 = self.call(src_stride, ref_stride, &mut sse1);
            let var2 = self.variance_ref(src_stride, ref_stride, &mut sse2);
            assert_eq!(sse1, sse2, "Error at test index: {i}");
            assert_eq!(var1, var2, "Error at test index: {i}");
        }
    }

    /// OneQuarterTest().
    fn one_quarter_test(&mut self) {
        let half = (self.p.block_size / 2) as usize;
        if !self.p.use_high_bit_depth {
            self.src8.fill(255);
            self.ref8[..half].fill(255);
            self.ref8[half..2 * half].fill(0);
        } else {
            let v = (255 << self.byte_shift()) as u16;
            self.src16.fill(v);
            self.ref16[..half].fill(v);
            self.ref16[half..2 * half].fill(0);
        }
        let mut sse = 0u32;
        let w = self.p.width;
        let var = self.call(w, w, &mut sse);
        let expected = (self.p.block_size * 255 * 255 / 4) as u32;
        assert_eq!(expected, var);
    }

    /// RefTestMse().
    fn ref_test_mse(&mut self) {
        for _ in 0..10 {
            for j in 0..self.p.block_size as usize {
                self.fill_random(j, j);
            }
            let (mut sse1, mut sse2) = (0u32, 0u32);
            let stride = self.p.width;
            self.call(stride, stride, &mut sse1);
            self.variance_ref(stride, stride, &mut sse2);
            assert_eq!(sse1, sse2);
        }
    }

    /// MaxTestMse().
    fn max_test_mse(&mut self) {
        let max_value: i32 = (1 << self.p.bit_depth) - 1;
        if !self.p.use_high_bit_depth {
            self.src8.fill(max_value as u8);
            self.ref8.fill(0);
        } else {
            self.src16.fill(max_value as u16);
            self.ref16.fill(0);
        }
        let mut sse = 0u32;
        let w = self.p.width;
        self.call(w, w, &mut sse);
        let mut expected: u32 =
            (self.p.block_size as u32).wrapping_mul(max_value as u32).wrapping_mul(max_value as u32);
        match self.p.bit_depth {
            12 => expected = (expected + 128) >> 8,
            10 => expected = (expected + 8) >> 4,
            _ => {}
        }
        assert_eq!(expected, sse);
    }
}

fn variance_tests(params: &[Params<VarianceMxNFunc>]) {
    for &p in params {
        let ctx = format!("width/height:{}/{} bit-depth:{}", p.width, p.height, p.bit_depth);
        let run = |f: fn(&mut Main<VarianceMxNFunc>)| {
            let r = std::panic::catch_unwind(|| f(&mut Main::new(p)));
            if let Err(e) = r {
                eprintln!("failing parameters: {ctx}");
                std::panic::resume_unwind(e);
            }
        };
        run(Main::zero_test);
        run(Main::ref_test);
        run(Main::ref_stride_test);
        run(Main::one_quarter_test);
    }
}

fn mse_tests(params: &[Params<VarianceMxNFunc>]) {
    for &p in params {
        let ctx = format!("width/height:{}/{} bit-depth:{}", p.width, p.height, p.bit_depth);
        let run = |f: fn(&mut Main<VarianceMxNFunc>)| {
            let r = std::panic::catch_unwind(|| f(&mut Main::new(p)));
            if let Err(e) = r {
                eprintln!("failing parameters: {ctx}");
                std::panic::resume_unwind(e);
            }
        };
        run(Main::ref_test_mse);
        run(Main::max_test_mse);
    }
}

#[derive(Clone, Copy)]
enum Fill {
    Random,
    Min,
    Max,
}

impl<F: Copy> Main<F> {
    fn fill_sse_sum(&mut self, fill: Fill) {
        match fill {
            Fill::Random => {
                for j in 0..self.p.block_size as usize {
                    self.src8[j] = self.rnd.rand8();
                    self.ref8[j] = self.rnd.rand8();
                }
            }
            Fill::Min => {
                self.src8.fill(0);
                self.ref8.fill(255);
            }
            Fill::Max => {
                self.src8.fill(255);
                self.ref8.fill(0);
            }
        }
    }
}

impl Main<GetSseSum8x8QuadFunc> {
    /// RefTestSseSum() (10 random rounds), MinTestSseSum() and MaxTestSseSum().
    fn sse_sum(&mut self, fill: Fill) {
        let rounds = if matches!(fill, Fill::Random) { 10 } else { 1 };
        for _ in 0..rounds {
            self.fill_sse_sum(fill);
            let mut sse1 = [0u32; 256];
            let mut sse2 = [0u32; 256];
            let mut var1 = [0u32; 256];
            let mut var2 = [0u32; 256];
            let mut sum1 = [0i32; 256];
            let mut sum2 = [0i32; 256];
            let mut sse_tot_c = 0u32;
            let mut sse_tot_simd = 0u32;
            let mut sum_tot_c = 0i32;
            let mut sum_tot_simd = 0i32;
            let stride = self.p.width;
            let mut k = 0usize;
            let (s, r) = (self.src8.as_ptr(), self.ref8.as_ptr());
            let mut row = 0;
            while row < self.p.height {
                let mut col = 0;
                while col < self.p.width {
                    let off = (stride * row + col) as isize;
                    // SAFETY: an 8x32 block inside the width x height buffers,
                    // four results from index k (k + 4 <= 256).
                    unsafe {
                        (self.p.func)(s.offset(off), stride, r.offset(off), stride, sse1.as_mut_ptr().add(k),
                                      sum1.as_mut_ptr().add(k), &mut sse_tot_simd, &mut sum_tot_simd,
                                      var1.as_mut_ptr().add(k));
                        aom_get_var_sse_sum_8x8_quad_c(s.offset(off), stride, r.offset(off), stride,
                                                       sse2.as_mut_ptr().add(k), sum2.as_mut_ptr().add(k),
                                                       &mut sse_tot_c, &mut sum_tot_c, var2.as_mut_ptr().add(k));
                    }
                    k += 4;
                    col += 32;
                }
                row += 8;
            }
            assert_eq!(sse_tot_c, sse_tot_simd);
            assert_eq!(sum_tot_c, sum_tot_simd);
            for q in 0..256 {
                assert_eq!(sse1[q], sse2[q]);
                assert_eq!(sum1[q], sum2[q]);
                assert_eq!(var1[q], var2[q]);
            }
        }
    }
}

impl Main<GetSseSum16x16DualFunc> {
    /// RefTestSseSumDual() (10 random rounds), MinTestSseSumDual() and
    /// MaxTestSseSumDual().
    fn sse_sum_dual(&mut self, fill: Fill) {
        let rounds = if matches!(fill, Fill::Random) { 10 } else { 1 };
        for _ in 0..rounds {
            self.fill_sse_sum(fill);
            let mut sse1 = [0u32; 64];
            let mut sse2 = [0u32; 64];
            let mut var1 = [0u32; 64];
            let mut var2 = [0u32; 64];
            let mut sse_tot_c = 0u32;
            let mut sse_tot_simd = 0u32;
            let mut sum_tot_c = 0i32;
            let mut sum_tot_simd = 0i32;
            let stride = self.p.width;
            let mut k = 0usize;
            let (s, r) = (self.src8.as_ptr(), self.ref8.as_ptr());
            let mut row = 0;
            while row < self.p.height {
                let mut col = 0;
                while col < self.p.width {
                    let off = (stride * row + col) as isize;
                    // SAFETY: a 16x32 block inside the width x height buffers,
                    // two results from index k (k + 2 <= 64).
                    unsafe {
                        (self.p.func)(s.offset(off), stride, r.offset(off), stride, sse1.as_mut_ptr().add(k),
                                      &mut sse_tot_simd, &mut sum_tot_simd, var1.as_mut_ptr().add(k));
                        aom_get_var_sse_sum_16x16_dual_c(s.offset(off), stride, r.offset(off), stride,
                                                         sse2.as_mut_ptr().add(k), &mut sse_tot_c,
                                                         &mut sum_tot_c, var2.as_mut_ptr().add(k));
                    }
                    k += 2;
                    col += 32;
                }
                row += 16;
            }
            assert_eq!(sse_tot_c, sse_tot_simd);
            assert_eq!(sum_tot_c, sum_tot_simd);
            for q in 0..64 {
                assert_eq!(sse1[q], sse2[q]);
                assert_eq!(var1[q], var2[q]);
            }
        }
    }
}

fn sse_sum_quad_tests(params: &[Params<GetSseSum8x8QuadFunc>]) {
    for &p in params {
        for fill in [Fill::Random, Fill::Min, Fill::Max] {
            Main::new(p).sse_sum(fill);
        }
    }
}

fn sse_sum_dual_tests(params: &[Params<GetSseSum16x16DualFunc>]) {
    for &p in params {
        for fill in [Fill::Random, Fill::Min, Fill::Max] {
            Main::new(p).sse_sum_dual(fill);
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::aom_dsp__arm__variance_neon::*;
    use aom::simd::aom_dsp__arm__highbd_variance_neon::*;

    fn check_cpu() {}

    const K_MSE_NEON: &[Params<VarianceMxNFunc>] = &[
        Params::new(3, 3, aom_mse8x8_neon, 0),
        Params::new(3, 4, aom_mse8x16_neon, 0),
        Params::new(4, 4, aom_mse16x16_neon, 0),
        Params::new(4, 3, aom_mse16x8_neon, 0),
    ];

    /// INSTANTIATE_TEST_SUITE_P(NEON, AvxMseTest, ...): RefMse, MaxMse.
    #[test]
    fn mse() {
        check_cpu();
        mse_tests(K_MSE_NEON);
    }

    const K_ARRAY_VARIANCE_NEON: &[Params<VarianceMxNFunc>] = &[
        Params::new(7, 7, aom_variance128x128_neon, 0),
        Params::new(6, 6, aom_variance64x64_neon, 0),
        Params::new(7, 6, aom_variance128x64_neon, 0),
        Params::new(6, 7, aom_variance64x128_neon, 0),
        Params::new(6, 6, aom_variance64x64_neon, 0),
        Params::new(6, 5, aom_variance64x32_neon, 0),
        Params::new(5, 6, aom_variance32x64_neon, 0),
        Params::new(5, 5, aom_variance32x32_neon, 0),
        Params::new(5, 4, aom_variance32x16_neon, 0),
        Params::new(4, 5, aom_variance16x32_neon, 0),
        Params::new(4, 4, aom_variance16x16_neon, 0),
        Params::new(4, 3, aom_variance16x8_neon, 0),
        Params::new(3, 4, aom_variance8x16_neon, 0),
        Params::new(3, 3, aom_variance8x8_neon, 0),
        Params::new(3, 2, aom_variance8x4_neon, 0),
        Params::new(2, 3, aom_variance4x8_neon, 0),
        Params::new(2, 2, aom_variance4x4_neon, 0),
        Params::new(2, 4, aom_variance4x16_neon, 0),
        Params::new(4, 2, aom_variance16x4_neon, 0),
        Params::new(3, 5, aom_variance8x32_neon, 0),
        Params::new(5, 3, aom_variance32x8_neon, 0),
        Params::new(4, 6, aom_variance16x64_neon, 0),
        Params::new(6, 4, aom_variance64x16_neon, 0),
    ];

    /// kArrayVariance_neon, AvxVarianceTest: Zero, Ref, RefStride, OneQuarter.
    #[test]
    fn var() {
        check_cpu();
        variance_tests(K_ARRAY_VARIANCE_NEON);
    }

    const K_ARRAY_GET_SSE_SUM_8X8_QUAD_NEON: &[Params<GetSseSum8x8QuadFunc>] = &[
        Params::new(7, 7, aom_get_var_sse_sum_8x8_quad_neon, 0),
        Params::new(6, 6, aom_get_var_sse_sum_8x8_quad_neon, 0),
        Params::new(5, 5, aom_get_var_sse_sum_8x8_quad_neon, 0),
        Params::new(5, 4, aom_get_var_sse_sum_8x8_quad_neon, 0),
    ];

    /// kArrayGetSseSum8x8Quad_neon, GetSseSum8x8QuadTest: RefMseSum, MinSseSum, MaxMseSum.
    #[test]
    fn quad() {
        check_cpu();
        sse_sum_quad_tests(K_ARRAY_GET_SSE_SUM_8X8_QUAD_NEON);
    }

    const K_ARRAY_GET_SSE_SUM_16X16_DUAL_NEON: &[Params<GetSseSum16x16DualFunc>] = &[
        Params::new(7, 7, aom_get_var_sse_sum_16x16_dual_neon, 0),
        Params::new(6, 6, aom_get_var_sse_sum_16x16_dual_neon, 0),
        Params::new(5, 5, aom_get_var_sse_sum_16x16_dual_neon, 0),
        Params::new(5, 4, aom_get_var_sse_sum_16x16_dual_neon, 0),
    ];

    /// kArrayGetSseSum16x16Dual_neon, GetSseSum16x16DualTest: RefMseSum, MinSseSum, MaxMseSum.
    #[test]
    fn dual() {
        check_cpu();
        sse_sum_dual_tests(K_ARRAY_GET_SSE_SUM_16X16_DUAL_NEON);
    }

    const K_ARRAY_HBD_VARIANCE_NEON: &[Params<VarianceMxNFunc>] = &[
        Params::new(7, 7, aom_highbd_12_variance128x128_neon, 12),
        Params::new(7, 6, aom_highbd_12_variance128x64_neon, 12),
        Params::new(6, 7, aom_highbd_12_variance64x128_neon, 12),
        Params::new(6, 6, aom_highbd_12_variance64x64_neon, 12),
        Params::new(6, 5, aom_highbd_12_variance64x32_neon, 12),
        Params::new(5, 6, aom_highbd_12_variance32x64_neon, 12),
        Params::new(5, 5, aom_highbd_12_variance32x32_neon, 12),
        Params::new(5, 4, aom_highbd_12_variance32x16_neon, 12),
        Params::new(4, 5, aom_highbd_12_variance16x32_neon, 12),
        Params::new(4, 4, aom_highbd_12_variance16x16_neon, 12),
        Params::new(4, 3, aom_highbd_12_variance16x8_neon, 12),
        Params::new(3, 4, aom_highbd_12_variance8x16_neon, 12),
        Params::new(3, 3, aom_highbd_12_variance8x8_neon, 12),
        Params::new(3, 2, aom_highbd_12_variance8x4_neon, 12),
        Params::new(2, 3, aom_highbd_12_variance4x8_neon, 12),
        Params::new(2, 2, aom_highbd_12_variance4x4_neon, 12),
        Params::new(7, 7, aom_highbd_10_variance128x128_neon, 10),
        Params::new(7, 6, aom_highbd_10_variance128x64_neon, 10),
        Params::new(6, 7, aom_highbd_10_variance64x128_neon, 10),
        Params::new(6, 6, aom_highbd_10_variance64x64_neon, 10),
        Params::new(6, 5, aom_highbd_10_variance64x32_neon, 10),
        Params::new(5, 6, aom_highbd_10_variance32x64_neon, 10),
        Params::new(5, 5, aom_highbd_10_variance32x32_neon, 10),
        Params::new(5, 4, aom_highbd_10_variance32x16_neon, 10),
        Params::new(4, 5, aom_highbd_10_variance16x32_neon, 10),
        Params::new(4, 4, aom_highbd_10_variance16x16_neon, 10),
        Params::new(4, 3, aom_highbd_10_variance16x8_neon, 10),
        Params::new(3, 4, aom_highbd_10_variance8x16_neon, 10),
        Params::new(3, 3, aom_highbd_10_variance8x8_neon, 10),
        Params::new(3, 2, aom_highbd_10_variance8x4_neon, 10),
        Params::new(2, 3, aom_highbd_10_variance4x8_neon, 10),
        Params::new(2, 2, aom_highbd_10_variance4x4_neon, 10),
        Params::new(7, 7, aom_highbd_8_variance128x128_neon, 8),
        Params::new(7, 6, aom_highbd_8_variance128x64_neon, 8),
        Params::new(6, 7, aom_highbd_8_variance64x128_neon, 8),
        Params::new(6, 6, aom_highbd_8_variance64x64_neon, 8),
        Params::new(6, 5, aom_highbd_8_variance64x32_neon, 8),
        Params::new(5, 6, aom_highbd_8_variance32x64_neon, 8),
        Params::new(5, 5, aom_highbd_8_variance32x32_neon, 8),
        Params::new(5, 4, aom_highbd_8_variance32x16_neon, 8),
        Params::new(4, 5, aom_highbd_8_variance16x32_neon, 8),
        Params::new(4, 4, aom_highbd_8_variance16x16_neon, 8),
        Params::new(4, 3, aom_highbd_8_variance16x8_neon, 8),
        Params::new(3, 4, aom_highbd_8_variance8x16_neon, 8),
        Params::new(3, 3, aom_highbd_8_variance8x8_neon, 8),
        Params::new(3, 2, aom_highbd_8_variance8x4_neon, 8),
        Params::new(2, 3, aom_highbd_8_variance4x8_neon, 8),
        Params::new(2, 2, aom_highbd_8_variance4x4_neon, 8),
        Params::new(6, 4, aom_highbd_12_variance64x16_neon, 12),
        Params::new(4, 6, aom_highbd_12_variance16x64_neon, 12),
        Params::new(5, 3, aom_highbd_12_variance32x8_neon, 12),
        Params::new(3, 5, aom_highbd_12_variance8x32_neon, 12),
        Params::new(4, 2, aom_highbd_12_variance16x4_neon, 12),
        Params::new(2, 4, aom_highbd_12_variance4x16_neon, 12),
        Params::new(6, 4, aom_highbd_10_variance64x16_neon, 10),
        Params::new(4, 6, aom_highbd_10_variance16x64_neon, 10),
        Params::new(5, 3, aom_highbd_10_variance32x8_neon, 10),
        Params::new(3, 5, aom_highbd_10_variance8x32_neon, 10),
        Params::new(4, 2, aom_highbd_10_variance16x4_neon, 10),
        Params::new(2, 4, aom_highbd_10_variance4x16_neon, 10),
        Params::new(6, 4, aom_highbd_8_variance64x16_neon, 8),
        Params::new(4, 6, aom_highbd_8_variance16x64_neon, 8),
        Params::new(5, 3, aom_highbd_8_variance32x8_neon, 8),
        Params::new(3, 5, aom_highbd_8_variance8x32_neon, 8),
        Params::new(4, 2, aom_highbd_8_variance16x4_neon, 8),
        Params::new(2, 4, aom_highbd_8_variance4x16_neon, 8),
    ];

    /// kArrayHBDVariance_neon, AvxHBDVarianceTest: Zero, Ref, RefStride, OneQuarter.
    #[test]
    fn hbd_var() {
        check_cpu();
        variance_tests(K_ARRAY_HBD_VARIANCE_NEON);
    }

    const K_HBD_MSE_NEON: &[Params<VarianceMxNFunc>] = &[
        Params::new(4, 4, aom_highbd_12_mse16x16_neon, 12),
        Params::new(4, 3, aom_highbd_12_mse16x8_neon, 12),
        Params::new(3, 4, aom_highbd_12_mse8x16_neon, 12),
        Params::new(3, 3, aom_highbd_12_mse8x8_neon, 12),
        Params::new(4, 4, aom_highbd_10_mse16x16_neon, 10),
        Params::new(4, 3, aom_highbd_10_mse16x8_neon, 10),
        Params::new(3, 4, aom_highbd_10_mse8x16_neon, 10),
        Params::new(3, 3, aom_highbd_10_mse8x8_neon, 10),
        Params::new(4, 4, aom_highbd_8_mse16x16_neon, 8),
        Params::new(4, 3, aom_highbd_8_mse16x8_neon, 8),
        Params::new(3, 4, aom_highbd_8_mse8x16_neon, 8),
        Params::new(3, 3, aom_highbd_8_mse8x8_neon, 8),
    ];

    /// INSTANTIATE_TEST_SUITE_P(NEON, AvxHBDMseTest, ...): RefMse, MaxMse.
    #[test]
    fn hbd_mse() {
        check_cpu();
        mse_tests(K_HBD_MSE_NEON);
    }
}

#[cfg(target_arch = "x86_64")]
mod sse2 {
    use super::*;
    use aom::simd::aom_dsp__x86__variance_sse2::*;

    fn check_cpu() {
        assert!(std::arch::is_x86_feature_detected!("sse2"), "SSE2 instantiation needs a CPU with sse2");
    }

    const K_MSE_SSE2: &[Params<VarianceMxNFunc>] = &[
        Params::new(4, 4, aom_mse16x16_sse2, 0),
        Params::new(4, 3, aom_mse16x8_sse2, 0),
        Params::new(3, 4, aom_mse8x16_sse2, 0),
        Params::new(3, 3, aom_mse8x8_sse2, 0),
    ];

    /// INSTANTIATE_TEST_SUITE_P(SSE2, AvxMseTest, ...): RefMse, MaxMse.
    #[test]
    fn mse() {
        check_cpu();
        mse_tests(K_MSE_SSE2);
    }

    const K_ARRAY_VARIANCE_SSE2: &[Params<VarianceMxNFunc>] = &[
        Params::new(7, 7, aom_variance128x128_sse2, 0),
        Params::new(7, 6, aom_variance128x64_sse2, 0),
        Params::new(6, 7, aom_variance64x128_sse2, 0),
        Params::new(6, 6, aom_variance64x64_sse2, 0),
        Params::new(6, 5, aom_variance64x32_sse2, 0),
        Params::new(5, 6, aom_variance32x64_sse2, 0),
        Params::new(5, 5, aom_variance32x32_sse2, 0),
        Params::new(5, 4, aom_variance32x16_sse2, 0),
        Params::new(4, 5, aom_variance16x32_sse2, 0),
        Params::new(4, 4, aom_variance16x16_sse2, 0),
        Params::new(4, 3, aom_variance16x8_sse2, 0),
        Params::new(3, 4, aom_variance8x16_sse2, 0),
        Params::new(3, 3, aom_variance8x8_sse2, 0),
        Params::new(3, 2, aom_variance8x4_sse2, 0),
        Params::new(2, 3, aom_variance4x8_sse2, 0),
        Params::new(2, 2, aom_variance4x4_sse2, 0),
        Params::new(6, 4, aom_variance64x16_sse2, 0),
        Params::new(5, 3, aom_variance32x8_sse2, 0),
        Params::new(4, 6, aom_variance16x64_sse2, 0),
        Params::new(4, 2, aom_variance16x4_sse2, 0),
        Params::new(3, 5, aom_variance8x32_sse2, 0),
        Params::new(2, 4, aom_variance4x16_sse2, 0),
    ];

    /// kArrayVariance_sse2, AvxVarianceTest: Zero, Ref, RefStride, OneQuarter.
    #[test]
    fn var() {
        check_cpu();
        variance_tests(K_ARRAY_VARIANCE_SSE2);
    }

    const K_ARRAY_GET_SSE_SUM_8X8_QUAD_SSE2: &[Params<GetSseSum8x8QuadFunc>] = &[
        Params::new(7, 7, aom_get_var_sse_sum_8x8_quad_sse2, 0),
        Params::new(6, 6, aom_get_var_sse_sum_8x8_quad_sse2, 0),
        Params::new(5, 5, aom_get_var_sse_sum_8x8_quad_sse2, 0),
        Params::new(5, 4, aom_get_var_sse_sum_8x8_quad_sse2, 0),
    ];

    /// kArrayGetSseSum8x8Quad_sse2, GetSseSum8x8QuadTest: RefMseSum, MinSseSum, MaxMseSum.
    #[test]
    fn quad() {
        check_cpu();
        sse_sum_quad_tests(K_ARRAY_GET_SSE_SUM_8X8_QUAD_SSE2);
    }

    const K_ARRAY_GET_SSE_SUM_16X16_DUAL_SSE2: &[Params<GetSseSum16x16DualFunc>] = &[
        Params::new(7, 7, aom_get_var_sse_sum_16x16_dual_sse2, 0),
        Params::new(6, 6, aom_get_var_sse_sum_16x16_dual_sse2, 0),
        Params::new(5, 5, aom_get_var_sse_sum_16x16_dual_sse2, 0),
        Params::new(5, 4, aom_get_var_sse_sum_16x16_dual_sse2, 0),
    ];

    /// kArrayGetSseSum16x16Dual_sse2, GetSseSum16x16DualTest: RefMseSum, MinSseSum, MaxMseSum.
    #[test]
    fn dual() {
        check_cpu();
        sse_sum_dual_tests(K_ARRAY_GET_SSE_SUM_16X16_DUAL_SSE2);
    }
}

#[cfg(target_arch = "x86_64")]
mod sse4_1 {
    use super::*;
    use aom::simd::aom_dsp__x86__highbd_variance_sse4::*;

    fn check_cpu() {
        assert!(std::arch::is_x86_feature_detected!("sse4.1"), "SSE4_1 instantiation needs a CPU with sse4.1");
    }

    const K_HBD_VARIANCE_SSE4_1: &[Params<VarianceMxNFunc>] = &[
        Params::new(2, 2, aom_highbd_8_variance4x4_sse4_1, 8),
        Params::new(2, 2, aom_highbd_10_variance4x4_sse4_1, 10),
        Params::new(2, 2, aom_highbd_12_variance4x4_sse4_1, 12),
    ];

    /// INSTANTIATE_TEST_SUITE_P(SSE4_1, AvxHBDVarianceTest, ...): Zero, Ref, RefStride, OneQuarter.
    #[test]
    fn hbd_var() {
        check_cpu();
        variance_tests(K_HBD_VARIANCE_SSE4_1);
    }
}

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use super::*;
    use aom::simd::aom_dsp__x86__variance_avx2::*;
    use aom::simd::aom_dsp__x86__highbd_variance_avx2::*;

    fn check_cpu() {
        assert!(std::arch::is_x86_feature_detected!("avx2"), "AVX2 instantiation needs a CPU with avx2");
    }

    const K_MSE_AVX2: &[Params<VarianceMxNFunc>] = &[
        Params::new(4, 4, aom_mse16x16_avx2, 0),
    ];

    /// INSTANTIATE_TEST_SUITE_P(AVX2, AvxMseTest, ...): RefMse, MaxMse.
    #[test]
    fn mse() {
        check_cpu();
        mse_tests(K_MSE_AVX2);
    }

    const K_ARRAY_VARIANCE_AVX2: &[Params<VarianceMxNFunc>] = &[
        Params::new(7, 7, aom_variance128x128_avx2, 0),
        Params::new(7, 6, aom_variance128x64_avx2, 0),
        Params::new(6, 7, aom_variance64x128_avx2, 0),
        Params::new(6, 6, aom_variance64x64_avx2, 0),
        Params::new(6, 5, aom_variance64x32_avx2, 0),
        Params::new(5, 6, aom_variance32x64_avx2, 0),
        Params::new(5, 5, aom_variance32x32_avx2, 0),
        Params::new(5, 4, aom_variance32x16_avx2, 0),
        Params::new(4, 5, aom_variance16x32_avx2, 0),
        Params::new(4, 4, aom_variance16x16_avx2, 0),
        Params::new(4, 3, aom_variance16x8_avx2, 0),
        Params::new(6, 4, aom_variance64x16_avx2, 0),
        Params::new(4, 6, aom_variance16x64_avx2, 0),
        Params::new(5, 3, aom_variance32x8_avx2, 0),
        Params::new(4, 2, aom_variance16x4_avx2, 0),
    ];

    /// kArrayVariance_avx2, AvxVarianceTest: Zero, Ref, RefStride, OneQuarter.
    #[test]
    fn var() {
        check_cpu();
        variance_tests(K_ARRAY_VARIANCE_AVX2);
    }

    const K_ARRAY_GET_SSE_SUM_8X8_QUAD_AVX2: &[Params<GetSseSum8x8QuadFunc>] = &[
        Params::new(7, 7, aom_get_var_sse_sum_8x8_quad_avx2, 0),
        Params::new(6, 6, aom_get_var_sse_sum_8x8_quad_avx2, 0),
        Params::new(5, 5, aom_get_var_sse_sum_8x8_quad_avx2, 0),
        Params::new(5, 4, aom_get_var_sse_sum_8x8_quad_avx2, 0),
    ];

    /// kArrayGetSseSum8x8Quad_avx2, GetSseSum8x8QuadTest: RefMseSum, MinSseSum, MaxMseSum.
    #[test]
    fn quad() {
        check_cpu();
        sse_sum_quad_tests(K_ARRAY_GET_SSE_SUM_8X8_QUAD_AVX2);
    }

    const K_ARRAY_GET_SSE_SUM_16X16_DUAL_AVX2: &[Params<GetSseSum16x16DualFunc>] = &[
        Params::new(7, 7, aom_get_var_sse_sum_16x16_dual_avx2, 0),
        Params::new(6, 6, aom_get_var_sse_sum_16x16_dual_avx2, 0),
        Params::new(5, 5, aom_get_var_sse_sum_16x16_dual_avx2, 0),
        Params::new(5, 4, aom_get_var_sse_sum_16x16_dual_avx2, 0),
    ];

    /// kArrayGetSseSum16x16Dual_avx2, GetSseSum16x16DualTest: RefMseSum, MinSseSum, MaxMseSum.
    #[test]
    fn dual() {
        check_cpu();
        sse_sum_dual_tests(K_ARRAY_GET_SSE_SUM_16X16_DUAL_AVX2);
    }

    const K_HBD_MSE_AVX2: &[Params<VarianceMxNFunc>] = &[
        Params::new(4, 4, aom_highbd_10_mse16x16_avx2, 10),
    ];

    /// INSTANTIATE_TEST_SUITE_P(AVX2, AvxHBDMseTest, ...): RefMse, MaxMse.
    #[test]
    fn hbd_mse() {
        check_cpu();
        mse_tests(K_HBD_MSE_AVX2);
    }

    const K_ARRAY_HBD_VARIANCE_AVX2: &[Params<VarianceMxNFunc>] = &[
        Params::new(7, 7, aom_highbd_10_variance128x128_avx2, 10),
        Params::new(7, 6, aom_highbd_10_variance128x64_avx2, 10),
        Params::new(6, 7, aom_highbd_10_variance64x128_avx2, 10),
        Params::new(6, 6, aom_highbd_10_variance64x64_avx2, 10),
        Params::new(6, 5, aom_highbd_10_variance64x32_avx2, 10),
        Params::new(5, 6, aom_highbd_10_variance32x64_avx2, 10),
        Params::new(5, 5, aom_highbd_10_variance32x32_avx2, 10),
        Params::new(5, 4, aom_highbd_10_variance32x16_avx2, 10),
        Params::new(4, 5, aom_highbd_10_variance16x32_avx2, 10),
        Params::new(4, 4, aom_highbd_10_variance16x16_avx2, 10),
        Params::new(4, 3, aom_highbd_10_variance16x8_avx2, 10),
        Params::new(3, 4, aom_highbd_10_variance8x16_avx2, 10),
        Params::new(3, 3, aom_highbd_10_variance8x8_avx2, 10),
        Params::new(6, 4, aom_highbd_10_variance64x16_avx2, 10),
        Params::new(5, 3, aom_highbd_10_variance32x8_avx2, 10),
        Params::new(4, 6, aom_highbd_10_variance16x64_avx2, 10),
        Params::new(3, 5, aom_highbd_10_variance8x32_avx2, 10),
    ];

    /// kArrayHBDVariance_avx2, AvxHBDVarianceTest: Zero, Ref, RefStride, OneQuarter.
    #[test]
    fn hbd_var() {
        check_cpu();
        variance_tests(K_ARRAY_HBD_VARIANCE_AVX2);
    }
}
