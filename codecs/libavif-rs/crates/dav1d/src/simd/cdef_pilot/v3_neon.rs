// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Variant 3 on AArch64: `cdef_tmpl.c` with NEON intrinsics, written from
//! the algorithm. Values have names, constants are typed Rust constants,
//! and `unsafe` is limited to the loads and stores of local arrays (NEON is
//! the baseline, so the value intrinsics are safe to call).
//!
//! The filter works on 8 pixels at once (one row of an 8-wide block, two
//! rows of a 4-wide one). The direction search keeps each partial-sum line
//! in two registers and builds the diagonal ones by "shift up one lane, add
//! the next row" (`vextq_s16` by 7).

use core::arch::aarch64::*;
use core::ffi::{c_int, c_uint};

use super::{
    best_direction, DirBlock, Filtered, Padded, Params, Variant, ALT_WEIGHTS, DIAGONAL_WEIGHTS,
    LINE_WEIGHT,
};

pub struct Neon;

impl Variant for Neon {
    #[target_feature(enable = "neon")]
    unsafe fn filter<const W: usize, const H: usize>(input: &Padded, params: &Params) -> Filtered {
        match (params.pri_strength != 0, params.sec_strength != 0) {
            (true, true) => filter::<W, H, true, true>(input, params),
            (true, false) => filter::<W, H, true, false>(input, params),
            _ => filter::<W, H, false, true>(input, params),
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
        find_dir(block)
    }
}

#[inline]
#[target_feature(enable = "neon")]
fn load(lanes: &[i16; 8]) -> int16x8_t {
    // SAFETY: reads the 8 values of the array.
    unsafe { vld1q_s16(lanes.as_ptr()) }
}

#[inline]
#[target_feature(enable = "neon")]
fn load4(lanes: &[i16; 4]) -> int16x4_t {
    // SAFETY: reads the 4 values of the array.
    unsafe { vld1_s16(lanes.as_ptr()) }
}

/// The pixels of rows `y..y + 8 / W` at offset `(dy, dx)`: one row of 8, or
/// two rows of 4.
#[inline]
#[target_feature(enable = "neon")]
fn taps<const W: usize>(input: &Padded, y: usize, off: isize) -> int16x8_t {
    if W == 8 {
        load(input.at::<8>(y, off))
    } else {
        vcombine_s16(load4(input.at::<4>(y, off)), load4(input.at::<4>(y + 1, off)))
    }
}

#[inline]
#[target_feature(enable = "neon")]
fn store(v: int16x8_t) -> [i16; 8] {
    let mut lanes = [0; 8];
    // SAFETY: writes the 8 values of the array.
    unsafe { vst1q_s16(lanes.as_mut_ptr(), v) };
    lanes
}

/// `constrain` of cdef_tmpl.c: `|p - px|` (exact as unsigned), limited to
/// `max(0, strength - (|p - px| >> shift))`, with the sign of `p - px`.
#[inline]
#[target_feature(enable = "neon")]
fn constrain(p: int16x8_t, px: int16x8_t, strength: uint16x8_t, right_shift: int16x8_t) -> int16x8_t {
    let adiff = vreinterpretq_u16_s16(vabdq_s16(p, px));
    let limit = vqsubq_u16(strength, vshlq_u16(adiff, right_shift));
    let c = vreinterpretq_s16_u16(vminq_u16(adiff, limit));
    vbslq_s16(vcltq_s16(p, px), vnegq_s16(c), c)
}

/// The tap's strength as the filter needs it.
struct Tap {
    strength: uint16x8_t,
    /// A negative left shift: NEON's variable right shift.
    right_shift: int16x8_t,
}

impl Tap {
    #[target_feature(enable = "neon")]
    fn new(strength: u16, shift: u32) -> Self {
        Tap { strength: vdupq_n_u16(strength), right_shift: vdupq_n_s16(-(shift as i16)) }
    }
}

/// `cdef_filter_block_c`, for the strengths that are not zero.
#[target_feature(enable = "neon")]
fn filter<const W: usize, const H: usize, const PRI: bool, const SEC: bool>(input: &Padded, params: &Params) -> Filtered {
    let load = |y: usize, off| taps::<W>(input, y, off);
    let pri = Tap::new(params.pri_strength, params.pri_shift);
    let sec = Tap::new(params.sec_strength, params.sec_shift);
    let mut out = [[0; 8]; 8];
    for y in (0..H).step_by(8 / W) {
        let px = load(y, 0);
        let mut sum = vdupq_n_s16(0);
        let (mut min, mut max) = (vreinterpretq_u16_s16(px), px);
        for k in 0..2 {
            if PRI {
                let off = params.primary(k);
                for off in [off, -off] {
                    let p = load(y, off);
                    sum = vmlaq_n_s16(sum, constrain(p, px, pri.strength, pri.right_shift), params.pri_taps[k]);
                    if SEC {
                        min = vminq_u16(min, vreinterpretq_u16_s16(p));
                        max = vmaxq_s16(max, p);
                    }
                }
            }
            if SEC {
                // "sec_tap starts at 2 and becomes 1"
                let tap = 2 - k as i16;
                for off in params.secondary(k) {
                    for off in [off, -off] {
                        let s = load(y, off);
                        sum = vmlaq_n_s16(sum, constrain(s, px, sec.strength, sec.right_shift), tap);
                        if PRI {
                            min = vminq_u16(min, vreinterpretq_u16_s16(s));
                            max = vmaxq_s16(max, s);
                        }
                    }
                }
            }
        }
        // px + ((sum - (sum < 0) + 8) >> 4): the comparison is -1 where true,
        // and a rounding shift adds the 8.
        let sum = vaddq_s16(sum, vreinterpretq_s16_u16(vcltzq_s16(sum)));
        let filtered = vaddq_s16(px, vrshrq_n_s16::<4>(sum));
        let result = if PRI && SEC {
            vminq_s16(vmaxq_s16(filtered, vreinterpretq_s16_u16(min)), max)
        } else {
            filtered
        };
        for (row, values) in out[y..].iter_mut().zip(store(result).chunks_exact(W)) {
            row[..W].copy_from_slice(values);
        }
    }
    out
}

/// A partial-sum line of up to 16 entries, in two registers.
#[derive(Clone, Copy)]
struct Partial {
    lo: int16x8_t,
    hi: int16x8_t,
}

impl Partial {
    #[target_feature(enable = "neon")]
    fn new() -> Self {
        Partial { lo: vdupq_n_s16(0), hi: vdupq_n_s16(0) }
    }

    /// Every entry to the next index.
    #[inline]
    #[target_feature(enable = "neon")]
    fn shift_up(self) -> Self {
        Partial { lo: vextq_s16::<7>(vdupq_n_s16(0), self.lo), hi: vextq_s16::<7>(self.lo, self.hi) }
    }

    /// Adds `v` to entries 0..8.
    #[inline]
    #[target_feature(enable = "neon")]
    fn add(self, v: int16x8_t) -> Self {
        Partial { lo: vaddq_s16(self.lo, v), hi: self.hi }
    }

    /// `sum(entry^2 * weight)`.
    #[inline]
    #[target_feature(enable = "neon")]
    fn cost(self, weights: &[[i32; 4]; 4]) -> u32 {
        // SAFETY: reads the 4 values of each array.
        let w = |i: usize| unsafe { vld1q_s32(weights[i].as_ptr()) };
        let mut acc = vmulq_s32(vmull_s16(vget_low_s16(self.lo), vget_low_s16(self.lo)), w(0));
        acc = vmlaq_s32(acc, vmull_high_s16(self.lo, self.lo), w(1));
        acc = vmlaq_s32(acc, vmull_s16(vget_low_s16(self.hi), vget_low_s16(self.hi)), w(2));
        acc = vmlaq_s32(acc, vmull_high_s16(self.hi, self.hi), w(3));
        vaddvq_s32(acc) as u32
    }
}

/// The weights of a line, zero beyond its length, in groups of four.
const fn weights<const N: usize>(w: [u32; N]) -> [[i32; 4]; 4] {
    let mut out = [[0; 4]; 4];
    let mut i = 0;
    while i < N {
        out[i / 4][i % 4] = w[i] as i32;
        i += 1;
    }
    out
}

const DIAGONAL: [[i32; 4]; 4] = weights(DIAGONAL_WEIGHTS);
const ALT: [[i32; 4]; 4] = weights(ALT_WEIGHTS);
const LINE: [[i32; 4]; 4] = weights([LINE_WEIGHT; 8]);

/// All eight lanes in reverse order.
#[inline]
#[target_feature(enable = "neon")]
fn reverse(v: int16x8_t) -> int16x8_t {
    let halves_reversed = vrev64q_s16(v);
    vextq_s16::<4>(halves_reversed, halves_reversed)
}

/// `cdef_find_dir_c`.
#[target_feature(enable = "neon")]
fn find_dir(block: &DirBlock) -> (c_int, c_uint) {
    let rows = block.map(|row| load(&row));
    let zero = vdupq_n_s16(0);
    // [r0 + r1, r2 + r3, r4 + r5, r6 + r7, 0, 0, 0, 0]
    let pairs = |r| vpaddq_s16(r, zero);
    // [r6 + r7, r4 + r5, r2 + r3, r0 + r1, 0, 0, 0, 0]
    let reversed_pairs = |r| vrev64q_s16(pairs(r));

    // diag[0][y + x], diag[1][7 + y - x], alt[0][y + x / 2],
    // alt[1][3 + y - x / 2]: row y at offset y, so the rows from the
    // last up, shifting the sums by one lane before each.
    let (mut diag0, mut diag1, mut alt0, mut alt1) = (Partial::new(), Partial::new(), Partial::new(), Partial::new());
    for &row in rows.iter().rev() {
        diag0 = diag0.shift_up().add(row);
        diag1 = diag1.shift_up().add(reverse(row));
        alt0 = alt0.shift_up().add(pairs(row));
        alt1 = alt1.shift_up().add(reversed_pairs(row));
    }
    // alt[2][3 - y / 2 + x] and alt[3][y / 2 + x]: rows in pairs, at offset
    // 3 - y / 2 and y / 2.
    let (mut alt2, mut alt3) = (Partial::new(), Partial::new());
    for y in (0..8).step_by(2) {
        alt2 = alt2.shift_up().add(vaddq_s16(rows[y], rows[y + 1]));
        alt3 = alt3.shift_up().add(vaddq_s16(rows[6 - y], rows[7 - y]));
    }
    // hv[0][y]: the row sums (a pairwise-add tree); hv[1][x]: the column sums.
    let quads = |a, b, c, d| vpaddq_s16(vpaddq_s16(a, b), vpaddq_s16(c, d));
    let row_sums = vpaddq_s16(quads(rows[0], rows[1], rows[2], rows[3]), quads(rows[4], rows[5], rows[6], rows[7]));
    let column_sums = rows.iter().fold(zero, |acc, &r| vaddq_s16(acc, r));
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
