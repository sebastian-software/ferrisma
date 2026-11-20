// Supplementary transcription of upstream/aom/test/hadamard_test.cc for the
// NEON, SSE2 and AVX2 instantiations: HadamardLowbdTest
// (CompareReferenceRandom, CompareReferenceExtreme, VaryStride),
// HadamardHighbdTest (CompareReferenceRandom, VaryStride),
// HadamardLowbdLPTest (CompareReferenceRandom, VaryStride) and
// HadamardLowbdLP8x8DualTest (CompareReferenceRandom, VaryStride). As in the
// original, each kernel is compared with the test's own reference transform
// after sorting both outputs (the kernels order their coefficients
// differently from `_c`; the order they share with the original's SIMD build
// is checked by tools/check_aom_simd_original.py, ADR-0041). Each TEST_P
// starts from a fresh ACMRandom with the deterministic seed. The x86 tests
// skip on a CPU without the feature. The speed tests are not transcribed.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

use super::acm_random::ACMRandom;

type HadamardFunc = unsafe extern "C-unwind" fn(*const i16, isize, *mut i32);
type HadamardLPFunc = unsafe extern "C-unwind" fn(*const i16, isize, *mut i16);

/// The test's OutputType: C computes in `int` and stores into the type.
trait Out: Copy + Ord + Default {
    fn of(v: i32) -> Self;
    fn i(self) -> i32;
}
impl Out for i32 {
    fn of(v: i32) -> Self { v }
    fn i(self) -> i32 { self }
}
impl Out for i16 {
    fn of(v: i32) -> Self { v as i16 }
    fn i(self) -> i32 { self as i32 }
}

fn hadamard4x4<T: Out>(a: &[T], out: &mut [T]) {
    let mut b = [T::default(); 8];
    let mut i = 0;
    while i < 4 {
        b[i] = T::of((a[i * 4].i() + a[(i + 1) * 4].i()) >> 1);
        b[i + 1] = T::of((a[i * 4].i() - a[(i + 1) * 4].i()) >> 1);
        i += 2;
    }
    out[0] = T::of(b[0].i() + b[2].i());
    out[1] = T::of(b[1].i() + b[3].i());
    out[2] = T::of(b[0].i() - b[2].i());
    out[3] = T::of(b[1].i() - b[3].i());
}

fn transpose_square<T: Out>(b: &mut [T], n: usize) {
    for i in 0..n {
        for j in i + 1..n {
            b.swap(j * n + i, i * n + j);
        }
    }
}

fn reference_hadamard4x4<T: Out>(a: &[i16], a_stride: usize, b: &mut [T]) {
    let mut input = [T::default(); 16];
    let mut buf = [T::default(); 16];
    for i in 0..4 {
        for j in 0..4 {
            input[i * 4 + j] = T::of(a[i * a_stride + j] as i32);
        }
    }
    for i in 0..4 { hadamard4x4(&input[i..], &mut buf[i * 4..]); }
    for i in 0..4 { hadamard4x4(&buf[i..], &mut b[i * 4..]); }
    // Extra transpose to match C and SSE2 behavior(i.e., aom_hadamard_4x4).
    transpose_square(b, 4);
}

fn hadamard_loop<T: Out>(a: &[T], out: &mut [T]) {
    let mut b = [0i32; 8];
    let mut i = 0;
    while i < 8 {
        b[i] = T::of(a[i * 8].i() + a[(i + 1) * 8].i()).i();
        b[i + 1] = T::of(a[i * 8].i() - a[(i + 1) * 8].i()).i();
        i += 2;
    }
    let mut c = [0i32; 8];
    let mut i = 0;
    while i < 8 {
        c[i] = T::of(b[i] + b[i + 2]).i();
        c[i + 1] = T::of(b[i + 1] + b[i + 3]).i();
        c[i + 2] = T::of(b[i] - b[i + 2]).i();
        c[i + 3] = T::of(b[i + 1] - b[i + 3]).i();
        i += 4;
    }
    out[0] = T::of(c[0] + c[4]);
    out[7] = T::of(c[1] + c[5]);
    out[3] = T::of(c[2] + c[6]);
    out[4] = T::of(c[3] + c[7]);
    out[2] = T::of(c[0] - c[4]);
    out[6] = T::of(c[1] - c[5]);
    out[1] = T::of(c[2] - c[6]);
    out[5] = T::of(c[3] - c[7]);
}

fn reference_hadamard8x8<T: Out>(a: &[i16], a_stride: usize, b: &mut [T]) {
    let mut input = [T::default(); 64];
    let mut buf = [T::default(); 64];
    for i in 0..8 {
        for j in 0..8 {
            input[i * 8 + j] = T::of(a[i * a_stride + j] as i32);
        }
    }
    for i in 0..8 { hadamard_loop(&input[i..], &mut buf[i * 8..]); }
    for i in 0..8 { hadamard_loop(&buf[i..], &mut b[i * 8..]); }
    // Extra transpose to match SSE2 behavior (i.e., aom_hadamard_8x8 and
    // aom_hadamard_lp_8x8).
    transpose_square(b, 8);
}

fn reference_hadamard8x8_dual<T: Out>(a: &[i16], a_stride: usize, b: &mut [T]) {
    reference_hadamard8x8(a, a_stride, b);
    reference_hadamard8x8(&a[8..], a_stride, &mut b[64..]);
}

fn reference_hadamard16x16<T: Out>(a: &[i16], a_stride: usize, b: &mut [T], shift: bool) {
    reference_hadamard8x8(a, a_stride, &mut b[0..]);
    reference_hadamard8x8(&a[8..], a_stride, &mut b[64..]);
    reference_hadamard8x8(&a[8 * a_stride..], a_stride, &mut b[128..]);
    reference_hadamard8x8(&a[8 + 8 * a_stride..], a_stride, &mut b[192..]);
    for i in 0..64 {
        let (a0, a1, a2, a3) = (b[i].i(), b[i + 64].i(), b[i + 128].i(), b[i + 192].i());
        let b0 = T::of((a0 + a1) >> 1).i();
        let b1 = T::of((a0 - a1) >> 1).i();
        let b2 = T::of((a2 + a3) >> 1).i();
        let b3 = T::of((a2 - a3) >> 1).i();
        b[i] = T::of(b0 + b2);
        b[i + 64] = T::of(b1 + b3);
        b[i + 128] = T::of(b0 - b2);
        b[i + 192] = T::of(b1 - b3);
    }
    if shift {
        // Extra shift to match aom_hadamard_16x16_c and aom_hadamard_16x16_avx2.
        for i in 0..16 {
            for j in 0..4 {
                b.swap(i * 16 + 4 + j, i * 16 + 8 + j);
            }
        }
    }
}

fn reference_hadamard32x32<T: Out>(a: &[i16], a_stride: usize, b: &mut [T], shift: bool) {
    reference_hadamard16x16(a, a_stride, &mut b[0..], shift);
    reference_hadamard16x16(&a[16..], a_stride, &mut b[256..], shift);
    reference_hadamard16x16(&a[16 * a_stride..], a_stride, &mut b[512..], shift);
    reference_hadamard16x16(&a[16 + 16 * a_stride..], a_stride, &mut b[768..], shift);
    for i in 0..256 {
        let (a0, a1, a2, a3) = (b[i].i(), b[i + 256].i(), b[i + 512].i(), b[i + 768].i());
        let b0 = T::of((a0 + a1) >> 2).i();
        let b1 = T::of((a0 - a1) >> 2).i();
        let b2 = T::of((a2 + a3) >> 2).i();
        let b3 = T::of((a2 - a3) >> 2).i();
        b[i] = T::of(b0 + b2);
        b[i + 256] = T::of(b1 + b3);
        b[i + 512] = T::of(b0 - b2);
        b[i + 768] = T::of(b1 - b3);
    }
}

fn reference_hadamard<T: Out>(a: &[i16], a_stride: usize, b: &mut [T], bw: usize, bh: usize,
                              shift: bool) {
    match (bw, bh) {
        (32, 32) => reference_hadamard32x32(a, a_stride, b, shift),
        (16, 16) => reference_hadamard16x16(a, a_stride, b, shift),
        (8, 8) => reference_hadamard8x8(a, a_stride, b),
        (4, 4) => reference_hadamard4x4(a, a_stride, b),
        (8, 16) => reference_hadamard8x8_dual(a, a_stride, b),
        _ => panic!("Invalid Hadamard transform size {bw}{bh}"),
    }
}

const K_MAX_BLOCK_SIZE: usize = 32 * 32;

/// One instantiation: the kernel as a closure over a typed output buffer.
struct Case<T: Out> {
    run: Box<dyn Fn(&[i16], usize, &mut [T])>,
    bw: usize,
    bh: usize,
    shift: bool,
    // Rand8 (8-bit residuals) or Rand12 (high bit depth).
    bits12: bool,
}

impl<T: Out> Case<T> {
    fn rand(&self, rnd: &mut ACMRandom) -> i16 {
        if self.bits12 {
            let src = rnd.rand12() as i16;
            let pred = rnd.rand12() as i16;
            src - pred
        } else {
            let src = rnd.rand8() as i16;
            let pred = rnd.rand8() as i16;
            src - pred
        }
    }

    fn check(&self, a: &[i16], stride: usize, what: &str) {
        let block_size = self.bw * self.bh;
        let mut b = vec![T::default(); K_MAX_BLOCK_SIZE];
        let mut b_ref = vec![T::default(); K_MAX_BLOCK_SIZE];
        reference_hadamard(a, stride, &mut b_ref, self.bw, self.bh, self.shift);
        (self.run)(a, stride, &mut b);
        // The order of the output is not important. Sort before checking.
        b[..block_size].sort();
        b_ref[..block_size].sort();
        assert!(b == b_ref, "{what} {}x{} stride {stride}", self.bw, self.bh);
    }

    fn compare_reference_random(&self) {
        let mut rnd = ACMRandom::new();
        let mut a = vec![0i16; K_MAX_BLOCK_SIZE];
        for v in a[..self.bw * self.bh].iter_mut() {
            *v = self.rand(&mut rnd);
        }
        self.check(&a, self.bw, "CompareReferenceRandom");
    }

    fn compare_reference_extreme(&self) {
        let k_bit_depth = 8;
        let mut a = vec![0i16; K_MAX_BLOCK_SIZE];
        for i in 0..2 {
            let sign = if i == 0 { 1 } else { -1 };
            for v in a[..self.bw * self.bh].iter_mut() {
                *v = sign * ((1 << k_bit_depth) - 1);
            }
            self.check(&a, self.bw, "CompareReferenceExtreme");
        }
    }

    fn vary_stride(&self) {
        let mut rnd = ACMRandom::new();
        let mut a = vec![0i16; K_MAX_BLOCK_SIZE * 8];
        for v in a[..self.bw * self.bh * 8].iter_mut() {
            *v = self.rand(&mut rnd);
        }
        let mut i = 8;
        while i < 64 {
            self.check(&a, i, "VaryStride");
            i += 8;
        }
    }
}

fn tran_case(f: HadamardFunc, bw: usize, bh: usize, bits12: bool) -> Case<i32> {
    Case { run: Box::new(move |a, stride, b| unsafe { f(a.as_ptr(), stride as isize, b.as_mut_ptr()) }),
           bw, bh, shift: true, bits12 }
}

fn lp_case(f: HadamardLPFunc, bw: usize, bh: usize) -> Case<i16> {
    Case { run: Box::new(move |a, stride, b| unsafe { f(a.as_ptr(), stride as isize, b.as_mut_ptr()) }),
           bw, bh, shift: false, bits12: false }
}

fn lowbd(f: HadamardFunc, bw: usize) {
    let case = tran_case(f, bw, bw, false);
    case.compare_reference_random();
    case.compare_reference_extreme();
    case.vary_stride();
}

fn highbd(f: HadamardFunc, bw: usize) {
    let case = tran_case(f, bw, bw, true);
    case.compare_reference_random();
    case.vary_stride();
}

fn lowbd_lp(f: HadamardLPFunc, bw: usize, bh: usize) {
    let case = lp_case(f, bw, bh);
    case.compare_reference_random();
    case.vary_stride();
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use aom::simd::aom_dsp__arm__hadamard_neon::*;
    use aom::simd::aom_dsp__arm__highbd_hadamard_neon::*;

    #[test]
    fn hadamard_lowbd() {
        super::lowbd(aom_hadamard_4x4_neon, 4);
        super::lowbd(aom_hadamard_8x8_neon, 8);
        super::lowbd(aom_hadamard_16x16_neon, 16);
        super::lowbd(aom_hadamard_32x32_neon, 32);
    }

    #[test]
    fn hadamard_highbd() {
        super::highbd(aom_highbd_hadamard_8x8_neon, 8);
        super::highbd(aom_highbd_hadamard_16x16_neon, 16);
        super::highbd(aom_highbd_hadamard_32x32_neon, 32);
    }

    #[test]
    fn hadamard_lowbd_lp() {
        super::lowbd_lp(aom_hadamard_lp_8x8_neon, 8, 8);
        super::lowbd_lp(aom_hadamard_lp_16x16_neon, 16, 16);
        super::lowbd_lp(aom_hadamard_lp_8x8_dual_neon, 8, 16);
    }
}

#[cfg(target_arch = "x86_64")]
mod x86 {
    use aom::simd::aom_dsp__x86__avg_intrin_avx2::*;
    use aom::simd::aom_dsp__x86__avg_intrin_sse2::*;

    #[test]
    fn hadamard_lowbd() {
        super::lowbd(aom_hadamard_4x4_sse2, 4);
        super::lowbd(aom_hadamard_8x8_sse2, 8);
        super::lowbd(aom_hadamard_16x16_sse2, 16);
        super::lowbd(aom_hadamard_32x32_sse2, 32);
        if std::arch::is_x86_feature_detected!("avx2") {
            super::lowbd(aom_hadamard_16x16_avx2, 16);
            super::lowbd(aom_hadamard_32x32_avx2, 32);
        }
    }

    #[test]
    fn hadamard_highbd() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        super::highbd(aom_highbd_hadamard_8x8_avx2, 8);
        super::highbd(aom_highbd_hadamard_16x16_avx2, 16);
        super::highbd(aom_highbd_hadamard_32x32_avx2, 32);
    }

    #[test]
    fn hadamard_lowbd_lp() {
        super::lowbd_lp(aom_hadamard_lp_8x8_sse2, 8, 8);
        super::lowbd_lp(aom_hadamard_lp_16x16_sse2, 16, 16);
        super::lowbd_lp(aom_hadamard_lp_8x8_dual_sse2, 8, 16);
        if std::arch::is_x86_feature_detected!("avx2") {
            super::lowbd_lp(aom_hadamard_lp_16x16_avx2, 16, 16);
            super::lowbd_lp(aom_hadamard_lp_8x8_dual_avx2, 8, 16);
        }
    }
}
