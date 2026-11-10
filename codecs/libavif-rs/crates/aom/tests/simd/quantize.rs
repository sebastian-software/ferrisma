// Supplementary transcription of upstream/aom/test/quantize_func_test.cc
// (FullPrecisionQuantizeTest and LowPrecisionQuantizeTest) for the Neon, SSE2
// and AVX2 instantiations, and of upstream/aom/test/av1_quantize_test.cc
// (AV1QuantizeTest) for its AVX2 instantiation. Inputs, quantizer tables, scan
// orders, and the exact-match assertions follow the original; only the
// parameter tuples of kernels that the port implements are instantiated.
use aom::av1__common__scan::av1_scan_orders;
use aom::av1__encoder__av1_quantize::{av1_build_quantizer, Dequants, QUANTS};

use super::acm_random::ACMRandom;

pub type QuantizeFunc = unsafe extern "C-unwind" fn(
    *const i32, isize, *const i16, *const i16, *const i16, *const i16, *mut i32, *mut i32,
    *const i16, *mut u16, *const i16, *const i16);
pub type QuantizeFuncHbd = unsafe extern "C-unwind" fn(
    *const i32, isize, *const i16, *const i16, *const i16, *const i16, *mut i32, *mut i32,
    *const i16, *mut u16, *const i16, *const i16, i32);

#[derive(Clone, Copy, PartialEq)]
pub enum QuantType {
    B,
    Fp,
}

#[derive(Clone, Copy)]
pub enum Kernel {
    Plain(QuantizeFunc),
    Hbd(QuantizeFuncHbd, i32),
}

impl Kernel {
    #[allow(clippy::too_many_arguments)]
    unsafe fn call(self, coeff: *const i32, n: isize, zbin: *const i16, round: *const i16,
                   quant: *const i16, quant_shift: *const i16, qcoeff: *mut i32,
                   dqcoeff: *mut i32, dequant: *const i16, eob: *mut u16, scan: *const i16,
                   iscan: *const i16) {
        match self {
            Kernel::Plain(f) => f(coeff, n, zbin, round, quant, quant_shift, qcoeff, dqcoeff,
                                  dequant, eob, scan, iscan),
            Kernel::Hbd(f, log_scale) => f(coeff, n, zbin, round, quant, quant_shift, qcoeff,
                                           dqcoeff, dequant, eob, scan, iscan, log_scale),
        }
    }
}

pub const TX_4X4: usize = 0;
pub const TX_8X8: usize = 1;
pub const TX_16X16: usize = 2;
pub const TX_32X32: usize = 3;
pub const TX_64X64: usize = 4;
pub const TX_4X16: usize = 13;
pub const TX_16X4: usize = 14;
pub const TX_8X32: usize = 15;
pub const TX_32X8: usize = 16;

const QINDEX_RANGE: usize = 256;
const K_TEST_NUM: usize = 1000;
const DCT_DCT: usize = 0;

/// av1_get_max_eob() for the transform sizes used here.
fn coeff_num(tx_size: usize) -> usize {
    match tx_size {
        TX_4X4 => 16,
        TX_8X8 => 64,
        TX_16X16 => 256,
        TX_32X32 | TX_64X64 => 1024,
        TX_4X16 | TX_16X4 => 64,
        TX_8X32 | TX_32X8 => 256,
        _ => unreachable!(),
    }
}

#[repr(C, align(32))]
struct Aligned<T>(T);

pub struct FullPrecisionQuantizeTest {
    quant_ref: Kernel,
    quant: Kernel,
    tx_size: usize,
    kind: QuantType,
    bd: u32,
    qtab_quant: Box<QUANTS>,
    qtab_dequant: Box<Dequants>,
    coeff: Vec<Aligned<[i32; 6 * 1024]>>,
    rnd: ACMRandom,
}

impl FullPrecisionQuantizeTest {
    pub fn new(quant_ref: Kernel, quant: Kernel, tx_size: usize, kind: QuantType, bd: u32) -> Self {
        // SAFETY: QUANTS and Dequants are plain integer tables; zero is a valid
        // value, and av1_build_quantizer fills them below.
        let mut qtab_quant: Box<QUANTS> = unsafe { Box::new(core::mem::zeroed()) };
        let mut qtab_dequant: Box<Dequants> = unsafe { Box::new(core::mem::zeroed()) };
        // SAFETY: both tables are valid, exclusively borrowed outputs.
        unsafe { av1_build_quantizer(bd, 0, 0, 0, 0, 0, &mut *qtab_quant, &mut *qtab_dequant, 0) };
        Self {
            quant_ref,
            quant,
            tx_size,
            kind,
            bd,
            qtab_quant,
            qtab_dequant,
            coeff: vec![Aligned([0; 6 * 1024])],
            rnd: ACMRandom::new(),
        }
    }

    fn n(&self) -> usize {
        coeff_num(self.tx_size)
    }

    fn fill_coeff(&mut self, c: i32) {
        let n = self.n();
        self.coeff[0].0[..n].fill(c);
    }

    fn get_random_coeff(&mut self) -> i32 {
        if self.bd == 8 {
            (self.rnd.rand16() as i16).clamp(i16::MIN + 1, i16::MAX) as i32
        } else {
            let min = -(1i32 << (7 + self.bd));
            let max = -min - 1;
            (self.rnd.rand31() as i32).clamp(min, max)
        }
    }

    fn fill_coeff_random(&mut self) {
        let n = self.n();
        self.fill_coeff(0);
        let num = self.rnd.rand16() as usize % n;
        // Randomize the first non zero coeff position.
        let start = self.rnd.rand16() as usize % n;
        let end = (start + num).min(n);
        for i in start..end {
            self.coeff[0].0[i] = self.get_random_coeff();
        }
    }

    pub fn quantize_run(&mut self, is_loop: bool, q: usize, test_num: usize) {
        let n = self.n();
        let sc = unsafe { av1_scan_orders[self.tx_size][DCT_DCT] };
        let zbin = self.qtab_quant.y_zbin[q].as_ptr();
        let (round, quant) = match self.kind {
            QuantType::B => (self.qtab_quant.y_round[q].as_ptr(), self.qtab_quant.y_quant[q].as_ptr()),
            QuantType::Fp => (self.qtab_quant.y_round_fp[q].as_ptr(),
                              self.qtab_quant.y_quant_fp[q].as_ptr()),
        };
        let quant_shift = self.qtab_quant.y_quant_shift[q].as_ptr();
        let dequant = self.qtab_dequant.y_dequant_QTX[q].as_ptr();
        for i in 0..test_num {
            if is_loop {
                self.fill_coeff_random();
            }
            let buf = &mut self.coeff[0].0;
            buf[n..6 * n].fill(0);
            let base = buf.as_mut_ptr();
            // SAFETY: the five output regions and the eob pair lie within the
            // 6 * n element buffer; the tables are 8-entry quantizer rows and the
            // scan tables cover n coefficients.
            unsafe {
                let coeff = base as *const i32;
                let qcoeff_ref = base.add(n);
                let dqcoeff_ref = qcoeff_ref.add(n);
                let qcoeff = dqcoeff_ref.add(n);
                let dqcoeff = qcoeff.add(n);
                let eob = dqcoeff.add(n) as *mut u16;
                self.quant_ref.call(coeff, n as isize, zbin, round, quant, quant_shift, qcoeff_ref,
                                    dqcoeff_ref, dequant, eob, sc.scan, sc.iscan);
                self.quant.call(coeff, n as isize, zbin, round, quant, quant_shift, qcoeff,
                                dqcoeff, dequant, eob.add(1), sc.scan, sc.iscan);
                for j in 0..n {
                    assert_eq!(*qcoeff_ref.add(j), *qcoeff.add(j),
                               "Q mismatch on test: {i} at position: {j} Q: {q} coeff: {}",
                               *coeff.add(j));
                }
                for j in 0..n {
                    assert_eq!(*dqcoeff_ref.add(j), *dqcoeff.add(j),
                               "Dq mismatch on test: {i} at position: {j} Q: {q} coeff: {}",
                               *coeff.add(j));
                }
                assert_eq!(*eob, *eob.add(1), "eobs mismatch on test: {i} Q: {q}");
            }
        }
    }

    /// Runs every exact-match case of the original FullPrecisionQuantizeTest.
    /// Each original TEST_P runs on a fresh fixture, whose ACMRandom starts
    /// from the deterministic seed, so the generator is reset before each case.
    pub fn run_all(&mut self) {
        // ZeroInput
        self.rnd.reset(ACMRandom::deterministic_seed());
        self.fill_coeff(0);
        self.quantize_run(false, 0, 1);
        // LargeNegativeInput
        self.rnd.reset(ACMRandom::deterministic_seed());
        self.fill_coeff(0);
        self.coeff[0].0[0] = -8191;
        self.quantize_run(false, 0, 1);
        // DcOnlyInput
        self.rnd.reset(ACMRandom::deterministic_seed());
        self.fill_coeff(0);
        self.coeff[0].0[0] = self.get_random_coeff();
        self.quantize_run(false, 0, 1);
        // RandomInput
        self.rnd.reset(ACMRandom::deterministic_seed());
        self.quantize_run(true, 0, K_TEST_NUM);
        // MultipleQ
        self.rnd.reset(ACMRandom::deterministic_seed());
        for q in 0..QINDEX_RANGE {
            self.quantize_run(true, q, K_TEST_NUM);
        }
        // CoeffHalfDequant
        self.rnd.reset(ACMRandom::deterministic_seed());
        self.fill_coeff(16);
        self.quantize_run(false, 25, 1);
    }
}

macro_rules! quantize_c {
    ($($name:ident),*) => {
        extern "C-unwind" {
            $(fn $name(coeff_ptr: *const i32, n_coeffs: isize, zbin_ptr: *const i16,
                       round_ptr: *const i16, quant_ptr: *const i16, quant_shift_ptr: *const i16,
                       qcoeff_ptr: *mut i32, dqcoeff_ptr: *mut i32, dequant_ptr: *const i16,
                       eob_ptr: *mut u16, scan: *const i16, iscan: *const i16);)*
        }
    };
}

quantize_c!(aom_highbd_quantize_b_c, aom_highbd_quantize_b_32x32_c, aom_highbd_quantize_b_64x64_c,
            aom_highbd_quantize_b_adaptive_c, aom_highbd_quantize_b_32x32_adaptive_c,
            aom_highbd_quantize_b_64x64_adaptive_c);

extern "C-unwind" {
    fn av1_highbd_quantize_fp_c(coeff_ptr: *const i32, n_coeffs: isize, zbin_ptr: *const i16,
                                round_ptr: *const i16, quant_ptr: *const i16,
                                quant_shift_ptr: *const i16, qcoeff_ptr: *mut i32,
                                dqcoeff_ptr: *mut i32, dequant_ptr: *const i16, eob_ptr: *mut u16,
                                scan: *const i16, iscan: *const i16, log_scale: i32);
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::aom_dsp__arm__highbd_quantize_neon::*;
    use aom::simd::av1__encoder__arm__av1_highbd_quantize_neon::av1_highbd_quantize_fp_neon;

    fn run(c: Kernel, neon: Kernel, tx_size: usize, kind: QuantType, bd: u32) {
        FullPrecisionQuantizeTest::new(c, neon, tx_size, kind, bd).run_all();
    }

    #[test]
    fn highbd_quantize_fp_16x16() {
        run(Kernel::Hbd(av1_highbd_quantize_fp_c, 0), Kernel::Hbd(av1_highbd_quantize_fp_neon, 0),
            TX_16X16, QuantType::Fp, 12);
    }

    #[test]
    fn highbd_quantize_fp_32x32() {
        run(Kernel::Hbd(av1_highbd_quantize_fp_c, 1), Kernel::Hbd(av1_highbd_quantize_fp_neon, 1),
            TX_32X32, QuantType::Fp, 12);
    }

    #[test]
    fn highbd_quantize_fp_64x64() {
        run(Kernel::Hbd(av1_highbd_quantize_fp_c, 2), Kernel::Hbd(av1_highbd_quantize_fp_neon, 2),
            TX_64X64, QuantType::Fp, 12);
    }

    macro_rules! plain {
        ($test:ident, $c:ident, $neon:ident, $tx:expr, $kind:expr, $bd:expr) => {
            #[test]
            fn $test() {
                run(Kernel::Plain($c), Kernel::Plain($neon), $tx, $kind, $bd);
            }
        };
    }

    plain!(highbd_quantize_b, aom_highbd_quantize_b_c, aom_highbd_quantize_b_neon, TX_16X16,
           QuantType::B, 12);
    plain!(highbd_quantize_b_32x32, aom_highbd_quantize_b_32x32_c,
           aom_highbd_quantize_b_32x32_neon, TX_32X32, QuantType::B, 12);
    plain!(highbd_quantize_b_64x64, aom_highbd_quantize_b_64x64_c,
           aom_highbd_quantize_b_64x64_neon, TX_64X64, QuantType::B, 12);
    plain!(highbd_quantize_b_adaptive, aom_highbd_quantize_b_adaptive_c,
           aom_highbd_quantize_b_adaptive_neon, TX_16X16, QuantType::B, 12);
    plain!(highbd_quantize_b_32x32_adaptive, aom_highbd_quantize_b_32x32_adaptive_c,
           aom_highbd_quantize_b_32x32_adaptive_neon, TX_32X32, QuantType::B, 12);
    plain!(highbd_quantize_b_64x64_adaptive, aom_highbd_quantize_b_64x64_adaptive_c,
           aom_highbd_quantize_b_64x64_adaptive_neon, TX_64X64, QuantType::B, 12);
}

/// av1_quantize_test.cc AV1QuantizeTest: BitExactCheck (RunQuantizeTest) and
/// EobVerify (RunEobTest) for one coefficient count.
#[cfg(target_arch = "x86_64")]
mod av1_quantize_test {
    use super::*;

    const NUM_TESTS: usize = 1000;
    const MAX_SIZE: usize = 1024;
    const ROUND_FACTOR_RANGE: i32 = 127;
    const DEQUANT_RANGE: i32 = 32768;
    const COEFF_RANGE: i32 = (1 << 20) - 1;

    #[repr(C, align(16))]
    struct Aligned16<T>(T);

    fn get_tx_size(count: usize) -> usize {
        match count {
            16 => TX_4X4,
            64 => TX_8X8,
            256 => TX_16X16,
            1024 => TX_32X32,
            _ => TX_4X4,
        }
    }

    // The DECLARE_ALIGNED(16, ...) locals of RunQuantizeTest and RunEobTest.
    struct Buffers {
        coeff: Aligned16<[i32; MAX_SIZE]>,
        zbin: Aligned16<[i16; 8]>,
        round: Aligned16<[i16; 8]>,
        quant: Aligned16<[i16; 8]>,
        quant_shift: Aligned16<[i16; 8]>,
        qcoeff: Aligned16<[i32; MAX_SIZE]>,
        dqcoeff: Aligned16<[i32; MAX_SIZE]>,
        ref_qcoeff: Aligned16<[i32; MAX_SIZE]>,
        ref_dqcoeff: Aligned16<[i32; MAX_SIZE]>,
        dequant: Aligned16<[i16; 8]>,
    }

    impl Buffers {
        fn new() -> Box<Self> {
            Box::new(Self {
                coeff: Aligned16([0; MAX_SIZE]),
                zbin: Aligned16([0; 8]),
                round: Aligned16([0; 8]),
                quant: Aligned16([0; 8]),
                quant_shift: Aligned16([0; 8]),
                qcoeff: Aligned16([0; MAX_SIZE]),
                dqcoeff: Aligned16([0; MAX_SIZE]),
                ref_qcoeff: Aligned16([0; MAX_SIZE]),
                ref_dqcoeff: Aligned16([0; MAX_SIZE]),
                dequant: Aligned16([0; 8]),
            })
        }

        fn fill_quantizer(&mut self, rnd: &mut ACMRandom) {
            for j in 0..2 {
                self.zbin.0[j] = rnd.rand16_signed();
                self.quant_shift.0[j] = rnd.rand16_signed();
                // int16_t positive
                self.dequant.0[j] = rnd.pseudo_uniform(DEQUANT_RANGE).abs() as i16;
                // Both tests truncate (1 << 16) / dequant to int16_t.
                self.quant.0[j] = ((1i32 << 16) / self.dequant.0[j] as i32) as i16;
                self.round.0[j] = ((rnd.pseudo_uniform(ROUND_FACTOR_RANGE).abs()
                                    * self.dequant.0[j] as i32) >> 7) as i16;
            }
            for j in 2..8 {
                self.zbin.0[j] = self.zbin.0[1];
                self.quant_shift.0[j] = self.quant_shift.0[1];
                self.dequant.0[j] = self.dequant.0[1];
                self.quant.0[j] = self.quant.0[1];
                self.round.0[j] = self.round.0[1];
            }
        }

        /// Runs the reference and the optimized function; returns (ref_eob, eob).
        unsafe fn run(&mut self, func: QuantizeFuncHbd, func_ref: QuantizeFuncHbd, count: usize,
                      log_scale: i32, scan: *const i16, iscan: *const i16) -> (u16, u16) {
            let mut eob: u16 = u16::MAX;
            let mut ref_eob: u16 = u16::MAX;
            func_ref(self.coeff.0.as_ptr(), count as isize, self.zbin.0.as_ptr(),
                     self.round.0.as_ptr(), self.quant.0.as_ptr(), self.quant_shift.0.as_ptr(),
                     self.ref_qcoeff.0.as_mut_ptr(), self.ref_dqcoeff.0.as_mut_ptr(),
                     self.dequant.0.as_ptr(), &mut ref_eob, scan, iscan, log_scale);
            func(self.coeff.0.as_ptr(), count as isize, self.zbin.0.as_ptr(),
                 self.round.0.as_ptr(), self.quant.0.as_ptr(), self.quant_shift.0.as_ptr(),
                 self.qcoeff.0.as_mut_ptr(), self.dqcoeff.0.as_mut_ptr(),
                 self.dequant.0.as_ptr(), &mut eob, scan, iscan, log_scale);
            (ref_eob, eob)
        }
    }

    pub fn run_quantize_test(func: QuantizeFuncHbd, func_ref: QuantizeFuncHbd, count: usize) {
        let mut rnd = ACMRandom::new();
        let mut b = Buffers::new();
        let tx_size = get_tx_size(count);
        let log_scale = (tx_size == TX_32X32) as i32;
        // SAFETY: av1_scan_orders is an immutable table after static initialization.
        let sc = unsafe { av1_scan_orders[tx_size][DCT_DCT] };
        for i in 0..NUM_TESTS {
            for j in 0..count {
                b.coeff.0[j] = rnd.pseudo_uniform(COEFF_RANGE);
            }
            b.fill_quantizer(&mut rnd);
            // SAFETY: the buffers hold `count` coefficients and 8-entry
            // quantizer rows; the scan tables cover `count` coefficients.
            let (ref_eob, eob) = unsafe { b.run(func, func_ref, count, log_scale, sc.scan,
                                                sc.iscan) };
            for j in 0..count {
                assert_eq!(b.ref_qcoeff.0[j], b.qcoeff.0[j], "qcoeff error: i = {i} j = {j}");
                assert_eq!(b.ref_dqcoeff.0[j], b.dqcoeff.0[j], "dqcoeff error: i = {i} j = {j}");
            }
            assert_eq!(ref_eob, eob, "eob error: i = {i}");
        }
    }

    pub fn run_eob_test(func: QuantizeFuncHbd, func_ref: QuantizeFuncHbd, count: usize) {
        let mut rnd = ACMRandom::new();
        let mut b = Buffers::new();
        let tx_size = get_tx_size(count);
        let log_scale = (tx_size == TX_32X32) as i32;
        // SAFETY: as in run_quantize_test.
        let sc = unsafe { av1_scan_orders[tx_size][DCT_DCT] };
        for i in 0..NUM_TESTS {
            b.coeff.0[..count].fill(0);
            // coeff_ptr[rnd(count)] = rnd(coeffRange); C++17 sequences the
            // right operand of an assignment before the left one.
            for _ in 0..3 {
                let value = rnd.pseudo_uniform(COEFF_RANGE);
                let index = rnd.pseudo_uniform(count as i32) as usize;
                b.coeff.0[index] = value;
            }
            b.fill_quantizer(&mut rnd);
            // SAFETY: as in run_quantize_test.
            let (ref_eob, eob) = unsafe { b.run(func, func_ref, count, log_scale, sc.scan,
                                                sc.iscan) };
            assert_eq!(ref_eob, eob, "eob error: i = {i}");
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use super::*;
    use aom::simd::aom_dsp__x86__highbd_adaptive_quantize_avx2::*;
    use aom::simd::aom_dsp__x86__highbd_quantize_intrin_avx2::*;
    use aom::simd::av1__encoder__x86__av1_highbd_quantize_avx2::av1_highbd_quantize_fp_avx2;

    fn run(c: Kernel, avx2: Kernel, tx_size: usize, kind: QuantType, bd: u32) {
        assert!(std::arch::is_x86_feature_detected!("avx2"), "AVX2 instantiation needs an AVX2 CPU");
        FullPrecisionQuantizeTest::new(c, avx2, tx_size, kind, bd).run_all();
    }

    // kQParamArrayAvx2: the highbd_quan{16x16,32x32,64x64}_wrapper tuples.
    macro_rules! hbd {
        ($test:ident, $log_scale:expr, $tx:expr, $bd:expr) => {
            #[test]
            fn $test() {
                run(Kernel::Hbd(av1_highbd_quantize_fp_c, $log_scale),
                    Kernel::Hbd(av1_highbd_quantize_fp_avx2, $log_scale), $tx, QuantType::Fp,
                    $bd);
            }
        };
    }

    hbd!(highbd_quantize_fp_16x16_8, 0, TX_16X16, 8);
    hbd!(highbd_quantize_fp_16x16_10, 0, TX_16X16, 10);
    hbd!(highbd_quantize_fp_16x16_12, 0, TX_16X16, 12);
    hbd!(highbd_quantize_fp_32x32_8, 1, TX_32X32, 8);
    hbd!(highbd_quantize_fp_32x32_10, 1, TX_32X32, 10);
    hbd!(highbd_quantize_fp_32x32_12, 1, TX_32X32, 12);
    hbd!(highbd_quantize_fp_64x64_8, 2, TX_64X64, 8);
    hbd!(highbd_quantize_fp_64x64_10, 2, TX_64X64, 10);
    hbd!(highbd_quantize_fp_64x64_12, 2, TX_64X64, 12);

    // kQParamArrayAvx2: the aom_highbd_quantize_b* tuples.
    macro_rules! plain {
        ($test:ident, $c:ident, $avx2:ident, $tx:expr, $kind:expr, $bd:expr) => {
            #[test]
            fn $test() {
                run(Kernel::Plain($c), Kernel::Plain($avx2), $tx, $kind, $bd);
            }
        };
    }

    plain!(highbd_quantize_b_8, aom_highbd_quantize_b_c, aom_highbd_quantize_b_avx2, TX_16X16,
           QuantType::B, 8);
    plain!(highbd_quantize_b_10, aom_highbd_quantize_b_c, aom_highbd_quantize_b_avx2, TX_16X16,
           QuantType::B, 10);
    plain!(highbd_quantize_b_12, aom_highbd_quantize_b_c, aom_highbd_quantize_b_avx2, TX_16X16,
           QuantType::B, 12);
    plain!(highbd_quantize_b_32x32_12, aom_highbd_quantize_b_32x32_c,
           aom_highbd_quantize_b_32x32_avx2, TX_32X32, QuantType::B, 12);
    plain!(highbd_quantize_b_64x64_12, aom_highbd_quantize_b_64x64_c,
           aom_highbd_quantize_b_64x64_avx2, TX_64X64, QuantType::B, 12);
    plain!(highbd_quantize_b_adaptive_8, aom_highbd_quantize_b_adaptive_c,
           aom_highbd_quantize_b_adaptive_avx2, TX_16X16, QuantType::B, 8);
    plain!(highbd_quantize_b_adaptive_10, aom_highbd_quantize_b_adaptive_c,
           aom_highbd_quantize_b_adaptive_avx2, TX_16X16, QuantType::B, 10);
    plain!(highbd_quantize_b_adaptive_12, aom_highbd_quantize_b_adaptive_c,
           aom_highbd_quantize_b_adaptive_avx2, TX_16X16, QuantType::B, 12);
    plain!(highbd_quantize_b_32x32_adaptive_8, aom_highbd_quantize_b_32x32_adaptive_c,
           aom_highbd_quantize_b_32x32_adaptive_avx2, TX_32X32, QuantType::B, 8);
    plain!(highbd_quantize_b_32x32_adaptive_10, aom_highbd_quantize_b_32x32_adaptive_c,
           aom_highbd_quantize_b_32x32_adaptive_avx2, TX_32X32, QuantType::B, 10);
    plain!(highbd_quantize_b_32x32_adaptive_12, aom_highbd_quantize_b_32x32_adaptive_c,
           aom_highbd_quantize_b_32x32_adaptive_avx2, TX_32X32, QuantType::B, 12);

    // av1_quantize_test.cc qfps_avx2: BitExactCheck and EobVerify.
    const AV1_QUANTIZE_COUNTS: [usize; 4] = [16, 64, 256, 1024];

    #[test]
    fn av1_quantize_bit_exact_check() {
        assert!(std::arch::is_x86_feature_detected!("avx2"), "AVX2 instantiation needs an AVX2 CPU");
        for count in AV1_QUANTIZE_COUNTS {
            av1_quantize_test::run_quantize_test(av1_highbd_quantize_fp_avx2,
                                                 av1_highbd_quantize_fp_c, count);
        }
    }

    #[test]
    fn av1_quantize_eob_verify() {
        assert!(std::arch::is_x86_feature_detected!("avx2"), "AVX2 instantiation needs an AVX2 CPU");
        for count in AV1_QUANTIZE_COUNTS {
            av1_quantize_test::run_eob_test(av1_highbd_quantize_fp_avx2,
                                            av1_highbd_quantize_fp_c, count);
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod sse2 {
    use super::*;
    use aom::simd::aom_dsp__x86__highbd_adaptive_quantize_sse2::*;

    fn run(c: Kernel, sse2: Kernel, tx_size: usize, kind: QuantType, bd: u32) {
        assert!(std::arch::is_x86_feature_detected!("sse2"), "SSE2 instantiation needs an SSE2 CPU");
        FullPrecisionQuantizeTest::new(c, sse2, tx_size, kind, bd).run_all();
    }

    // kQParamArraySSE2: the aom_highbd_quantize_b_64x64_adaptive tuples.
    macro_rules! plain {
        ($test:ident, $c:ident, $sse2:ident, $tx:expr, $kind:expr, $bd:expr) => {
            #[test]
            fn $test() {
                run(Kernel::Plain($c), Kernel::Plain($sse2), $tx, $kind, $bd);
            }
        };
    }

    plain!(highbd_quantize_b_64x64_adaptive_8, aom_highbd_quantize_b_64x64_adaptive_c,
           aom_highbd_quantize_b_64x64_adaptive_sse2, TX_64X64, QuantType::B, 8);
    plain!(highbd_quantize_b_64x64_adaptive_10, aom_highbd_quantize_b_64x64_adaptive_c,
           aom_highbd_quantize_b_64x64_adaptive_sse2, TX_64X64, QuantType::B, 10);
    plain!(highbd_quantize_b_64x64_adaptive_12, aom_highbd_quantize_b_64x64_adaptive_c,
           aom_highbd_quantize_b_64x64_adaptive_sse2, TX_64X64, QuantType::B, 12);
}
