//! The pilot against the translated C kernels (`itx_tmpl.c`), slot by slot,
//! on random blocks: every coefficient the C reads is drawn from the full
//! range the decoder stores (`cf_max` of recon_tmpl.c), so the clipping
//! steps are reached as well. A development check; the gate is checkasm
//! against the original's assembly and the whole-image decodes.

use super::*;

struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        // xorshift32
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }

    /// A value in `-max..=max`.
    fn signed(&mut self, max: i32) -> i32 {
        (self.next() % (2 * max as u32 + 1)) as i32 - max
    }
}

/// The C kernels of the translation: the DSP tables with the CPU flags
/// masked off (the C then also builds its `last_nonzero_col` tables).
fn c_tables(bpc: c_int) -> ([[Option<ItxFn>; 17]; 19], [[Option<ItxFn16>; 17]; 19]) {
    use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
    let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe {
        dav1d_init_cpu();
        dav1d_set_cpu_flags_mask(0);
        let mut c8: crate::b8::itx_tmpl::Dav1dInvTxfmDSPContext = core::mem::zeroed();
        crate::b8::itx_tmpl::dav1d_itx_dsp_init_8bpc(&mut c8, 8);
        let mut c16: crate::b16::itx_tmpl::Dav1dInvTxfmDSPContext = core::mem::zeroed();
        crate::b16::itx_tmpl::dav1d_itx_dsp_init_16bpc(&mut c16, bpc);
        dav1d_set_cpu_flags_mask(!0);
        (core::mem::transmute(c8.itxfm_add), core::mem::transmute(c16.itxfm_add))
    }
}

/// The rows the C's first pass reads for `eob` (`last_nonzero_col + 1`) in
/// DSP-table slot `tx_type`, from the C's own table where it uses one.
fn c_coded_rows(tx: usize, tx_type: usize, eob: usize) -> usize {
    use transforms::Kind::Identity;
    let (w, h) = SIZES[tx];
    if tx_type == WHT_WHT {
        return 4;
    }
    let [first, second] = blocks::TX1D_TYPES[blocks::kernel_type(tx_type)];
    let last = if second == Identity && first != Identity {
        (h.min(32) - 1).min(eob)
    } else if first == Identity && second != Identity {
        eob >> w.trailing_zeros()
    } else {
        // SAFETY: built by the C init above (`all_simd` was 0).
        unsafe {
            let tables = *core::ptr::addr_of!(crate::common::scan::dav1d_last_nonzero_col_from_eob);
            *tables[tx].add(eob) as usize
        }
    };
    last + 1
}

/// One random block of `tx`: pixels, and coefficients in the rows the C
/// reads (only the DC one for `eob` 0).
fn draw(rng: &mut Rng, tx: usize, tx_type: usize, pixel_max: i32, cf_max: i32) -> (Vec<i32>, Vec<i32>, usize) {
    let (w, h) = SIZES[tx];
    let (sw, sh) = (w.min(32), h.min(32));
    let eob = match rng.next() % 4 {
        0 => 0,
        1 => (rng.next() as usize) % 8,
        _ => (rng.next() as usize) % (sw * sh),
    };
    let eob = if tx_type == WHT_WHT { 15 } else { eob };
    let mut coeff = vec![0; sw * sh];
    if eob == 0 {
        coeff[0] = rng.signed(cf_max);
    } else {
        let rows = c_coded_rows(tx, tx_type, eob);
        let small = rng.next() % 2 == 0;
        for x in 0..sw {
            for y in 0..rows {
                coeff[y + x * sh] = if small { rng.signed(cf_max >> 6) } else { rng.signed(cf_max) };
            }
        }
    }
    let pixels = (0..w * h).map(|_| (rng.next() % (pixel_max as u32 + 1)) as i32).collect();
    (pixels, coeff, eob)
}

fn check_8bpc(draws: usize) {
    let (c, _) = c_tables(10);
    let pilot = table_8bpc();
    let mut rng = Rng(0x2545_f491);
    let mut checked = 0;
    for tx in 0..19 {
        let (w, h) = SIZES[tx];
        for tx_type in 0..17 {
            let Some(c_fn) = c[tx][tx_type] else { continue };
            let pilot_fn: ItxFn = if tx_type == WHT_WHT { wht_8bpc } else { pilot[tx][tx_type] };
            for _ in 0..draws {
                let (pixels, coeff, eob) = draw(&mut rng, tx, tx_type, 255, i16::MAX.into());
                let mut outputs = [pixels.clone(), pixels].map(|p| p.into_iter().map(|v| v as u8).collect::<Vec<_>>());
                let mut coeffs = [coeff.clone(), coeff].map(|c| c.into_iter().map(|v| v as i16).collect::<Vec<_>>());
                for (f, (out, cf)) in [c_fn, pilot_fn].into_iter().zip(outputs.iter_mut().zip(coeffs.iter_mut())) {
                    unsafe { f(out.as_mut_ptr(), w as isize, cf.as_mut_ptr(), eob as c_int) };
                }
                assert_eq!(outputs[0], outputs[1], "8 bpc {w}x{h} type {tx_type} eob {eob}: pixels");
                assert_eq!(coeffs[0], coeffs[1], "8 bpc {w}x{h} type {tx_type} eob {eob}: coefficients");
                checked += 1;
            }
        }
    }
    assert!(checked > 0);
}

fn check_16bpc(bpc: c_int, draws: usize) {
    let (_, c) = c_tables(bpc);
    let pilot = table_16bpc();
    let pixel_max = (1 << bpc) - 1;
    let cf_max = (128 << bpc) - 1;
    let mut rng = Rng(0x1234_5678 ^ bpc as u32);
    for tx in 0..19 {
        let (w, h) = SIZES[tx];
        for tx_type in 0..17 {
            let Some(c_fn) = c[tx][tx_type] else { continue };
            let pilot_fn: ItxFn16 = if tx_type == WHT_WHT { wht_16bpc } else { pilot[tx][tx_type] };
            for _ in 0..draws {
                let (pixels, coeff, eob) = draw(&mut rng, tx, tx_type, pixel_max, cf_max);
                let mut outputs = [pixels.clone(), pixels].map(|p| p.into_iter().map(|v| v as u16).collect::<Vec<_>>());
                let mut coeffs = [coeff.clone(), coeff];
                for (f, (out, cf)) in [c_fn, pilot_fn].into_iter().zip(outputs.iter_mut().zip(coeffs.iter_mut())) {
                    unsafe { f(out.as_mut_ptr(), 2 * w as isize, cf.as_mut_ptr(), eob as c_int, pixel_max) };
                }
                assert_eq!(outputs[0], outputs[1], "{bpc} bits {w}x{h} type {tx_type} eob {eob}: pixels");
                assert_eq!(coeffs[0], coeffs[1], "{bpc} bits {w}x{h} type {tx_type} eob {eob}: coefficients");
            }
        }
    }
}

#[test]
fn pilot_matches_c_8bpc() {
    if available() {
        check_8bpc(300);
    }
}

#[test]
fn pilot_matches_c_10bpc() {
    if available() {
        check_16bpc(10, 300);
    }
}

#[test]
fn pilot_matches_c_12bpc() {
    if available() {
        check_16bpc(12, 300);
    }
}

/// The pilot's table of `eob` to coded rows is the C's.
#[test]
fn last_nonzero_rows_match_c() {
    let _ = c_tables(10);
    for tx in 0..19 {
        let (w, h) = SIZES[tx];
        for eob in 0..w.min(32) * h.min(32) {
            assert_eq!(coded_rows(tx, DCT_DCT_FOR_TESTS, eob), c_coded_rows(tx, DCT_DCT_FOR_TESTS, eob), "{w}x{h} eob {eob}");
        }
    }
}

const DCT_DCT_FOR_TESTS: usize = blocks::DCT_DCT;
