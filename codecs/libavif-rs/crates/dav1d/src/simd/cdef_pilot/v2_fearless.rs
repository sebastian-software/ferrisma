// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Variant 2: `cdef_tmpl.c` on the portable vector types of
//! `fearless_simd` (1.2, Apache-2.0 OR MIT, stable Rust): one generic
//! implementation, compiled for NEON on AArch64 and for its `Avx2` level
//! (x86-64-v3) on x86-64.
//!
//! The filter works on 16 pixels at once (`i16x16`: one AVX2 register, two
//! NEON registers). The direction search keeps each partial-sum line in two
//! `i16x8` halves and builds the diagonal ones by "shift up one lane, add the
//! next row" (`Partial::shift_up`), which places row `y` at offset `y`
//! without a per-row shuffle. As the library asks, every generic function
//! is `#[inline(always)]` and the entry runs inside `Simd::vectorize`, so the
//! whole kernel is compiled with the level's target features.

use core::ffi::{c_int, c_uint};

use fearless_simd::prelude::*;
use fearless_simd::{i16x16, i16x8, i32x4, u16x16};

use super::{
    best_direction, opposite, DirBlock, Filtered, Padded, Params, Variant, ALT_WEIGHTS, DIAGONAL_WEIGHTS,
    LINE_WEIGHT,
};

#[cfg(target_arch = "aarch64")]
type Level = fearless_simd::aarch64::Neon;
#[cfg(target_arch = "x86_64")]
type Level = fearless_simd::x86::Avx2;

/// Whether the CPU has the level this variant is compiled for (on x86-64
/// the library's `Avx2` needs x86-64-v3, a little more than dav1d's AVX2
/// flag; without it the shipped kernels stay).
pub fn available() -> bool {
    #[cfg(target_arch = "aarch64")]
    return fearless_simd::Level::new().as_neon().is_some();
    #[cfg(target_arch = "x86_64")]
    return fearless_simd::Level::new().as_avx2().is_some();
}

/// The level's token.
///
/// # Safety
/// `available()` is true (the variant is installed only then).
#[inline(always)]
unsafe fn level() -> Level {
    // SAFETY: the caller's contract.
    unsafe { Level::assume_supported() }
}

pub struct Fearless;

impl Variant for Fearless {
    unsafe fn filter<const W: usize, const H: usize>(input: &Padded, params: &Params) -> Filtered {
        let simd = level();
        simd.vectorize(
            #[inline(always)]
            || match (params.pri_strength != 0, params.sec_strength != 0) {
                (true, true) => filter::<_, W, H, true, true>(simd, input, params),
                (true, false) => filter::<_, W, H, true, false>(simd, input, params),
                _ => filter::<_, W, H, false, true>(simd, input, params),
            },
        )
    }

    unsafe fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
        let simd = level();
        simd.vectorize(
            #[inline(always)]
            || find_dir(simd, block),
        )
    }
}

/// `constrain` of cdef_tmpl.c, lane by lane.
#[inline(always)]
fn constrain<S: Simd>(p: i16x16<S>, px: i16x16<S>, strength: u16x16<S>, shift: u32) -> i16x16<S> {
    let adiff: u16x16<S> = (p.max(px) - p.min(px)).bitcast();
    let c: i16x16<S> = adiff.min(strength.saturating_sub(adiff >> shift)).bitcast();
    p.simd_lt(px).select(-c, c)
}

/// The C's `umin`: the padding marker, unsigned, never wins.
#[inline(always)]
fn umin<S: Simd>(a: i16x16<S>, b: i16x16<S>) -> i16x16<S> {
    a.bitcast::<u16x16<S>>().min(b.bitcast::<u16x16<S>>()).bitcast()
}

/// `cdef_filter_block_c`, for the strengths that are not zero.
#[inline(always)]
fn filter<S: Simd, const W: usize, const H: usize, const PRI: bool, const SEC: bool>(
    simd: S,
    input: &Padded,
    params: &Params,
) -> Filtered {
    let load = |y: usize, off| i16x16::from_slice(simd, &input.lanes::<W, 16>(y, off));
    let pri_strength = u16x16::splat(simd, params.pri_strength);
    let sec_strength = u16x16::splat(simd, params.sec_strength);
    let mut out = [[0; 8]; 8];
    for y in (0..H).step_by(16 / W) {
        let px = load(y, (0, 0));
        let mut sum = i16x16::splat(simd, 0);
        let (mut min, mut max) = (px, px);
        for k in 0..2 {
            if PRI {
                let tap = i16x16::splat(simd, params.pri_taps[k]);
                let off = params.primary(k);
                for off in [off, opposite(off)] {
                    let p = load(y, off);
                    sum = sum + constrain(p, px, pri_strength, params.pri_shift) * tap;
                    if SEC {
                        min = umin(min, p);
                        max = max.max(p);
                    }
                }
            }
            if SEC {
                let tap = i16x16::splat(simd, 2 - k as i16);
                for off in params.secondary(k) {
                    for off in [off, opposite(off)] {
                        let s = load(y, off);
                        sum = sum + constrain(s, px, sec_strength, params.sec_shift) * tap;
                        if PRI {
                            min = umin(min, s);
                            max = max.max(s);
                        }
                    }
                }
            }
        }
        // px + ((sum - (sum < 0) + 8) >> 4)
        let sum = sum.simd_lt(0).select(sum - 1, sum);
        let filtered = px + ((sum + 8) >> 4);
        let result = if PRI && SEC { filtered.max(min).min(max) } else { filtered };
        for (row, values) in out[y..].iter_mut().zip(result.as_slice().chunks_exact(W)) {
            row[..W].copy_from_slice(values);
        }
    }
    out
}

/// A partial-sum line of up to 16 entries, in two halves.
#[derive(Clone, Copy)]
struct Partial<S: Simd> {
    simd: S,
    lo: i16x8<S>,
    hi: i16x8<S>,
}

impl<S: Simd> Partial<S> {
    #[inline(always)]
    fn new(simd: S) -> Self {
        Partial { simd, lo: i16x8::splat(simd, 0), hi: i16x8::splat(simd, 0) }
    }

    /// Every entry to the next index.
    #[inline(always)]
    fn shift_up(self) -> Self {
        let zero = i16x8::splat(self.simd, 0);
        Partial { lo: zero.slide::<7>(self.lo), hi: self.lo.slide::<7>(self.hi), ..self }
    }

    /// Adds `v` to entries 0..8.
    #[inline(always)]
    fn add(self, v: i16x8<S>) -> Self {
        Partial { lo: self.lo + v, ..self }
    }

    /// `sum(entry^2 * weight)`.
    #[inline(always)]
    fn cost(self, weights: &[i32; 16]) -> u32 {
        let simd = self.simd;
        let w = |i: usize| i32x4::from_slice(simd, &weights[i..i + 4]);
        let (a, b) = self.lo.widening_mul(self.lo);
        let (c, d) = self.hi.widening_mul(self.hi);
        (a * w(0) + b * w(4) + c * w(8) + d * w(12)).reduce_sum() as u32
    }
}

/// The weights of a line, zero beyond its length.
const fn weights<const N: usize>(w: [u32; N]) -> [i32; 16] {
    let mut out = [0; 16];
    let mut i = 0;
    while i < N {
        out[i] = w[i] as i32;
        i += 1;
    }
    out
}

const DIAGONAL: [i32; 16] = weights(DIAGONAL_WEIGHTS);
const ALT: [i32; 16] = weights(ALT_WEIGHTS);
const LINE: [i32; 16] = weights([LINE_WEIGHT; 8]);

/// `cdef_find_dir_c`.
#[inline(always)]
fn find_dir<S: Simd>(simd: S, block: &DirBlock) -> (c_int, c_uint) {
    let rows = block.map(|row| i16x8::from_slice(simd, &row));
    let zero = i16x8::splat(simd, 0);
    // [r0 + r1, r2 + r3, r4 + r5, r6 + r7, 0, 0, 0, 0]
    let pairs = |r: i16x8<S>| r.unzip_low(zero) + r.unzip_high(zero);
    // [r6 + r7, r4 + r5, r2 + r3, r0 + r1, 0, 0, 0, 0]
    let reversed_pairs = |r: i16x8<S>| pairs(r).reverse().slide::<4>(zero);

    // diag[0][y + x], diag[1][7 + y - x], alt[0][y + x / 2],
    // alt[1][3 + y - x / 2]: row y at offset y, so the rows from the
    // last up, shifting the sums by one lane before each.
    let (mut diag0, mut diag1, mut alt0, mut alt1) =
        (Partial::new(simd), Partial::new(simd), Partial::new(simd), Partial::new(simd));
    for &row in rows.iter().rev() {
        diag0 = diag0.shift_up().add(row);
        diag1 = diag1.shift_up().add(row.reverse());
        alt0 = alt0.shift_up().add(pairs(row));
        alt1 = alt1.shift_up().add(reversed_pairs(row));
    }
    // alt[2][3 - y / 2 + x] and alt[3][y / 2 + x]: rows in pairs, at offset
    // 3 - y / 2 and y / 2.
    let (mut alt2, mut alt3) = (Partial::new(simd), Partial::new(simd));
    for y in (0..8).step_by(2) {
        alt2 = alt2.shift_up().add(rows[y] + rows[y + 1]);
        alt3 = alt3.shift_up().add(rows[6 - y] + rows[7 - y]);
    }
    // hv[0][y]: the row sums; hv[1][x]: the column sums.
    let hv0 = Partial { simd, lo: i16x8::from_fn(simd, |y| rows[y].reduce_sum()), hi: zero };
    let hv1 = Partial { simd, lo: rows.iter().fold(zero, |acc, &r| acc + r), hi: zero };

    best_direction([
        diag0.cost(&DIAGONAL),
        alt0.cost(&ALT),
        hv0.cost(&LINE),
        alt1.cost(&ALT),
        diag1.cost(&DIAGONAL),
        alt2.cost(&ALT),
        hv1.cost(&LINE),
        alt3.cost(&ALT),
    ])
}
