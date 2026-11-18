// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Variant 3 on x86-64: `cdef_tmpl.c` with AVX2 intrinsics, written from
//! the algorithm. Values have names, constants are typed Rust constants,
//! and `unsafe` is limited to the loads and stores of local arrays (every
//! function carries `target_feature(enable = "avx2")`, so the value
//! intrinsics are safe inside; the variant is installed only on a CPU with
//! AVX2).
//!
//! The filter works on 16 pixels at once (two rows of an 8-wide block, four
//! rows of a 4-wide one). The direction search keeps each partial-sum line
//! in two 128-bit registers and builds the diagonal ones by "shift up one
//! lane, add the next row" (`_mm_alignr_epi8` by 14 bytes).

use core::arch::x86_64::*;
use core::ffi::{c_int, c_uint};

use super::{
    best_direction, opposite, DirBlock, Filtered, Padded, Params, Variant, ALT_WEIGHTS, DIAGONAL_WEIGHTS,
    LINE_WEIGHT,
};

pub struct Avx2;

impl Variant for Avx2 {
    #[target_feature(enable = "avx2")]
    unsafe fn filter<const W: usize, const H: usize>(input: &Padded, params: &Params) -> Filtered {
        match (params.pri_strength != 0, params.sec_strength != 0) {
            (true, true) => filter::<W, H, true, true>(input, params),
            (true, false) => filter::<W, H, true, false>(input, params),
            _ => filter::<W, H, false, true>(input, params),
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
        find_dir(block)
    }
}

#[inline]
#[target_feature(enable = "avx2")]
fn load16(lanes: &[i16; 16]) -> __m256i {
    // SAFETY: reads the 16 values of the array.
    unsafe { _mm256_loadu_si256(lanes.as_ptr().cast()) }
}

#[inline]
#[target_feature(enable = "avx2")]
fn load8(lanes: &[i16; 8]) -> __m128i {
    // SAFETY: reads the 8 values of the array.
    unsafe { _mm_loadu_si128(lanes.as_ptr().cast()) }
}

#[inline]
#[target_feature(enable = "avx2")]
fn store16(v: __m256i) -> [i16; 16] {
    let mut lanes = [0; 16];
    // SAFETY: writes the 16 values of the array.
    unsafe { _mm256_storeu_si256(lanes.as_mut_ptr().cast(), v) };
    lanes
}

/// `constrain` of cdef_tmpl.c: `|p - px|` (exact as unsigned), limited to
/// `max(0, strength - (|p - px| >> shift))`, with the sign of `p - px`.
#[inline]
#[target_feature(enable = "avx2")]
fn constrain(p: __m256i, px: __m256i, strength: __m256i, shift: __m128i) -> __m256i {
    let adiff = _mm256_sub_epi16(_mm256_max_epi16(p, px), _mm256_min_epi16(p, px));
    let limit = _mm256_subs_epu16(strength, _mm256_srl_epi16(adiff, shift));
    let c = _mm256_min_epu16(adiff, limit);
    // -c where p < px: (c ^ -1) - (-1).
    let negative = _mm256_cmpgt_epi16(px, p);
    _mm256_sub_epi16(_mm256_xor_si256(c, negative), negative)
}

/// The tap's strength as the filter needs it.
struct Tap {
    strength: __m256i,
    shift: __m128i,
}

impl Tap {
    #[target_feature(enable = "avx2")]
    fn new(strength: u16, shift: u32) -> Self {
        Tap { strength: _mm256_set1_epi16(strength as i16), shift: _mm_cvtsi32_si128(shift as i32) }
    }
}

/// `cdef_filter_block_c`, for the strengths that are not zero.
#[target_feature(enable = "avx2")]
fn filter<const W: usize, const H: usize, const PRI: bool, const SEC: bool>(input: &Padded, params: &Params) -> Filtered {
    let load = |y: usize, off| load16(&input.lanes::<W, 16>(y, off));
    let pri = Tap::new(params.pri_strength, params.pri_shift);
    let sec = Tap::new(params.sec_strength, params.sec_shift);
    let mut out = [[0; 8]; 8];
    for y in (0..H).step_by(16 / W) {
        let px = load(y, (0, 0));
        let mut sum = _mm256_setzero_si256();
        let (mut min, mut max) = (px, px);
        for k in 0..2 {
            if PRI {
                let tap = _mm256_set1_epi16(params.pri_taps[k]);
                let off = params.primary(k);
                for off in [off, opposite(off)] {
                    let p = load(y, off);
                    sum = _mm256_add_epi16(sum, _mm256_mullo_epi16(constrain(p, px, pri.strength, pri.shift), tap));
                    if SEC {
                        min = _mm256_min_epu16(min, p);
                        max = _mm256_max_epi16(max, p);
                    }
                }
            }
            if SEC {
                // "sec_tap starts at 2 and becomes 1"
                let tap = _mm256_set1_epi16(2 - k as i16);
                for off in params.secondary(k) {
                    for off in [off, opposite(off)] {
                        let s = load(y, off);
                        sum = _mm256_add_epi16(sum, _mm256_mullo_epi16(constrain(s, px, sec.strength, sec.shift), tap));
                        if PRI {
                            min = _mm256_min_epu16(min, s);
                            max = _mm256_max_epi16(max, s);
                        }
                    }
                }
            }
        }
        // px + ((sum - (sum < 0) + 8) >> 4): the comparison is -1 where true.
        let sum = _mm256_add_epi16(sum, _mm256_cmpgt_epi16(_mm256_setzero_si256(), sum));
        let rounded = _mm256_srai_epi16::<4>(_mm256_add_epi16(sum, _mm256_set1_epi16(8)));
        let filtered = _mm256_add_epi16(px, rounded);
        let result = if PRI && SEC { _mm256_min_epi16(_mm256_max_epi16(filtered, min), max) } else { filtered };
        for (row, values) in out[y..].iter_mut().zip(store16(result).chunks_exact(W)) {
            row[..W].copy_from_slice(values);
        }
    }
    out
}

/// A partial-sum line of up to 16 entries, in two registers.
#[derive(Clone, Copy)]
struct Partial {
    lo: __m128i,
    hi: __m128i,
}

impl Partial {
    #[target_feature(enable = "avx2")]
    fn new() -> Self {
        Partial { lo: _mm_setzero_si128(), hi: _mm_setzero_si128() }
    }

    /// Every entry to the next index.
    #[inline]
    #[target_feature(enable = "avx2")]
    fn shift_up(self) -> Self {
        Partial { lo: _mm_alignr_epi8::<14>(self.lo, _mm_setzero_si128()), hi: _mm_alignr_epi8::<14>(self.hi, self.lo) }
    }

    /// Adds `v` to entries 0..8.
    #[inline]
    #[target_feature(enable = "avx2")]
    fn add(self, v: __m128i) -> Self {
        Partial { lo: _mm_add_epi16(self.lo, v), hi: self.hi }
    }

    /// `sum(entry^2 * weight)`.
    #[inline]
    #[target_feature(enable = "avx2")]
    fn cost(self, weights: &[[i32; 8]; 2]) -> u32 {
        let weighted_squares = |half: __m128i, w: &[i32; 8]| {
            let wide = _mm256_cvtepi16_epi32(half);
            // SAFETY: reads the 8 values of the array.
            let w = unsafe { _mm256_loadu_si256(w.as_ptr().cast()) };
            _mm256_mullo_epi32(_mm256_mullo_epi32(wide, wide), w)
        };
        let sum = _mm256_add_epi32(weighted_squares(self.lo, &weights[0]), weighted_squares(self.hi, &weights[1]));
        let sum = _mm_add_epi32(_mm256_castsi256_si128(sum), _mm256_extracti128_si256::<1>(sum));
        let sum = _mm_add_epi32(sum, _mm_unpackhi_epi64(sum, sum));
        let sum = _mm_add_epi32(sum, _mm_shuffle_epi32::<0b01>(sum));
        _mm_cvtsi128_si32(sum) as u32
    }
}

/// The weights of a line, zero beyond its length, in groups of eight.
const fn weights<const N: usize>(w: [u32; N]) -> [[i32; 8]; 2] {
    let mut out = [[0; 8]; 2];
    let mut i = 0;
    while i < N {
        out[i / 8][i % 8] = w[i] as i32;
        i += 1;
    }
    out
}

const DIAGONAL: [[i32; 8]; 2] = weights(DIAGONAL_WEIGHTS);
const ALT: [[i32; 8]; 2] = weights(ALT_WEIGHTS);
const LINE: [[i32; 8]; 2] = weights([LINE_WEIGHT; 8]);

/// The byte shuffle that reverses eight 16-bit lanes.
const REVERSE_LANES: [i8; 16] = [14, 15, 12, 13, 10, 11, 8, 9, 6, 7, 4, 5, 2, 3, 0, 1];

/// `cdef_find_dir_c`.
#[target_feature(enable = "avx2")]
fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
    let rows = block.map(|row| load8(&row));
    let zero = _mm_setzero_si128();
    // SAFETY: reads the 16 values of the array.
    let reverse_lanes = unsafe { _mm_loadu_si128(REVERSE_LANES.as_ptr().cast()) };
    // [r0 + r1, r2 + r3, r4 + r5, r6 + r7, 0, 0, 0, 0]
    let pairs = |r| _mm_hadd_epi16(r, zero);
    // [r6 + r7, r4 + r5, r2 + r3, r0 + r1, 0, 0, 0, 0]
    let reversed_pairs = |r| _mm_shufflelo_epi16::<0b00_01_10_11>(pairs(r));

    // diag[0][y + x], diag[1][7 + y - x], alt[0][y + x / 2],
    // alt[1][3 + y - x / 2]: row y at offset y, so the rows from the
    // last up, shifting the sums by one lane before each.
    let (mut diag0, mut diag1, mut alt0, mut alt1) = (Partial::new(), Partial::new(), Partial::new(), Partial::new());
    for &row in rows.iter().rev() {
        diag0 = diag0.shift_up().add(row);
        diag1 = diag1.shift_up().add(_mm_shuffle_epi8(row, reverse_lanes));
        alt0 = alt0.shift_up().add(pairs(row));
        alt1 = alt1.shift_up().add(reversed_pairs(row));
    }
    // alt[2][3 - y / 2 + x] and alt[3][y / 2 + x]: rows in pairs, at offset
    // 3 - y / 2 and y / 2.
    let (mut alt2, mut alt3) = (Partial::new(), Partial::new());
    for y in (0..8).step_by(2) {
        alt2 = alt2.shift_up().add(_mm_add_epi16(rows[y], rows[y + 1]));
        alt3 = alt3.shift_up().add(_mm_add_epi16(rows[6 - y], rows[7 - y]));
    }
    // hv[0][y]: the row sums (a horizontal-add tree); hv[1][x]: the column sums.
    let quads = |a, b, c, d| _mm_hadd_epi16(_mm_hadd_epi16(a, b), _mm_hadd_epi16(c, d));
    let row_sums = _mm_hadd_epi16(quads(rows[0], rows[1], rows[2], rows[3]), quads(rows[4], rows[5], rows[6], rows[7]));
    let column_sums = rows.iter().fold(zero, |acc, &r| _mm_add_epi16(acc, r));
    let hv0 = Partial { lo: row_sums, hi: zero };
    let hv1 = Partial { lo: column_sums, hi: zero };

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
