// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! EstimateTokenCost of lib/jxl/enc_ans_simd.cc for the NEON and AVX2
//! targets (port ADR 0004, "Encoder path"): `EstimateTokenCostImpl<E, M, L>`
//! written once against [`SimdF32`] (`HWY_FULL(uint32_t)` and
//! `HWY_FULL(float)`: four NEON, eight AVX2 lanes) and the source's dispatch
//! over the hybrid-uint configurations, including its fallthrough branches
//! and the `~0` result outside them. `enc_entropy` keeps the `HWY_SCALAR`
//! specialization (`HybridUintConfig::Encode` per value) and calls
//! [`estimate_token_cost_simd`] when the encoder selected a SIMD target, as
//! `HWY_DYNAMIC_DISPATCH(EstimateTokenCost)` does.
//!
//! The unsigned lanes are the signed lanes reinterpreted: `Add`, `Sub`, the
//! low half of `Mul`, `And`/`Or` and the logical `ShiftRight` are the same
//! words; the unsigned comparisons `Gt`, `Ge` and `Lt` flip both operands'
//! sign bits and compare signed (as Highway's AVX2 `Gt` does), which gives
//! the exact unsigned mask words of NEON's `vcgtq_u32`/`vcgeq_u32`/
//! `vcltq_u32` and of AVX2 alike. `ConvertTo(df, val_fixed)` converts values below
//! 2^22 (`val > 2^22 - 1` is shifted right by 10), which every conversion
//! gives exactly, so the signed conversion is used. The last, partial vector
//! loads and stores a whole vector at `last_full`, as the source does: the
//! caller pads `values` and `out` by `MaxVectorSize()` words (counted as
//! words, as in `enc_ans.cc`), and the extra lanes' bits are masked by
//! `take`; their tokens are stored but never read.

use super::dec_ans::HybridUintConfig;
use super::hwy::{Kernel, SimdF32, Target, dispatch};

/// `Gt(a, b)` of unsigned lanes, as a mask vector.
#[inline(always)]
pub(crate) fn gt_u32<D: SimdF32>(d: D, a: D::I, b: D::I) -> D::I {
    let msb = d.set_i32(i32::MIN);
    d.gt_i32(d.xor_i32(a, msb), d.xor_i32(b, msb))
}
/// `Lt(a, b)` of unsigned lanes.
#[inline(always)]
pub(crate) fn lt_u32<D: SimdF32>(d: D, a: D::I, b: D::I) -> D::I {
    gt_u32(d, b, a)
}
/// `Ge(a, b) = Not(Lt(a, b))` of unsigned lanes.
#[inline(always)]
pub(crate) fn ge_u32<D: SimdF32>(d: D, a: D::I, b: D::I) -> D::I {
    d.xor_i32(lt_u32(d, a, b), d.set_i32(-1))
}
#[inline(always)]
pub(crate) fn or_i32<D: SimdF32>(d: D, a: D::I, b: D::I) -> D::I {
    d.bitcast_to_i32(d.or(d.bitcast_to_f32(a), d.bitcast_to_f32(b)))
}
#[inline(always)]
pub(crate) fn load_u32<D: SimdF32>(d: D, src: &[u32]) -> D::I {
    d.bitcast_to_i32(d.load_bits(src))
}
#[inline(always)]
pub(crate) fn store_u32<D: SimdF32>(d: D, v: D::I, dst: &mut [u32]) {
    d.store_bits(d.bitcast_to_f32(v), dst);
}
/// `GetLane(SumOfLanes(du, v))` of 32-bit lanes (wrapping, exact in any
/// order).
#[inline(always)]
fn sum_of_lanes_u32<D: SimdF32>(d: D, v: D::I) -> u32 {
    let mut lanes = [0i32; 16];
    d.store_i32(v, &mut lanes[..D::LANES]);
    lanes[..D::LANES]
        .iter()
        .fold(0u32, |sum, &x| sum.wrapping_add(x as u32))
}

/// One vector of `EstimateTokenCostImpl`'s loop body: the token vector and
/// `eb_fixed`.
#[inline(always)]
fn token_vector<D: SimdF32, const E: u32, const M: u32, const L: u32>(
    du: D,
    val: D::I,
) -> (D::I, D::I) {
    let k_zero = du.set_i32(0);
    let k_split = du.set_i32(1 << E);
    let k_exp_offset = du.set_i32(127);
    let k_eb_offset = du.set_i32((127 + M + L) as i32);
    let k_base = du.set_i32(((1u32 << E).wrapping_sub(E << (M + L))) as i32);
    let k_mul_n = du.set_i32(1 << (M + L));
    let k_mask_l = du.set_i32((1 << L) - 1);
    let k_mask_m = du.set_i32(((1 << M) - 1) << L);
    let k_large_threshold = du.set_i32((1 << 22) - 1);
    const LARGE_SHIFT_VAL: i32 = 10;
    let k_large_shift = du.set_i32(LARGE_SHIFT_VAL);

    let is_large = gt_u32(du, val, k_large_threshold);
    let val_shifted = du.shift_right_u32::<LARGE_SHIFT_VAL>(val);
    let not_literal = ge_u32(du, val, k_split);
    let val_fixed = du.if_then_else_i32(is_large, val_shifted, val);
    let b = du.bitcast_to_i32(du.convert_to_f32(val_fixed));
    let l = du.and_i32(val, k_mask_l);
    let exp = du.shift_right_u32::<23>(b);
    let exp_fixed = du.if_then_else_i32(is_large, du.add_i32(exp, k_large_shift), exp);
    let n = du.sub_i32(exp_fixed, k_exp_offset);
    let eb = du.sub_i32(exp_fixed, k_eb_offset);
    let m = shift_right_u32_by(du, b, 23 - M - L);
    let a = du.add_i32(k_base, du.mul_i32(n, k_mul_n));
    let d = du.and_i32(m, k_mask_m);
    let eb_fixed = du.if_then_else_i32(not_literal, eb, k_zero);
    let c = or_i32(du, a, l);
    let t = or_i32(du, c, d);
    let t_fixed = du.if_then_else_i32(not_literal, t, val);
    (t_fixed, eb_fixed)
}

/// `ShiftRight<23 - M - L>` of unsigned lanes; the count is a template
/// constant in the source (16 to 23 here).
#[inline(always)]
fn shift_right_u32_by<D: SimdF32>(d: D, v: D::I, bits: u32) -> D::I {
    match bits {
        16 => d.shift_right_u32::<16>(v),
        17 => d.shift_right_u32::<17>(v),
        18 => d.shift_right_u32::<18>(v),
        19 => d.shift_right_u32::<19>(v),
        20 => d.shift_right_u32::<20>(v),
        21 => d.shift_right_u32::<21>(v),
        22 => d.shift_right_u32::<22>(v),
        23 => d.shift_right_u32::<23>(v),
        _ => unreachable!("M + L of a hybrid-uint configuration is at most 7"),
    }
}

/// `EstimateTokenCostImpl<E, M, L>(values, len, out)`.
#[inline(always)]
pub fn estimate_token_cost_impl<D: SimdF32, const E: u32, const M: u32, const L: u32>(
    du: D,
    values: &[u32],
    len: usize,
    out: &mut [u32],
) -> u32 {
    let mut extra_bits = du.set_i32(0);
    let last_full = D::LANES * (len / D::LANES);
    let mut i = 0;
    while i < last_full {
        let val = load_u32(du, &values[i..]);
        let (t_fixed, eb_fixed) = token_vector::<D, E, M, L>(du, val);
        extra_bits = du.add_i32(extra_bits, eb_fixed);
        store_u32(du, t_fixed, &mut out[i..]);
        i += D::LANES;
    }
    if last_full < len {
        let stop = du.set_i32(len as u32 as i32);
        let mut iota = [0i32; 16];
        for (k, lane) in iota[..D::LANES].iter_mut().enumerate() {
            *lane = (last_full + k) as u32 as i32;
        }
        let fence = du.load_i32(&iota[..D::LANES]);
        let take = lt_u32(du, fence, stop);
        let val = load_u32(du, &values[last_full..]);
        let (t_fixed, eb_fixed) = token_vector::<D, E, M, L>(du, val);
        let eb_masked = du.if_then_else_i32(take, eb_fixed, du.set_i32(0));
        extra_bits = du.add_i32(extra_bits, eb_masked);
        store_u32(du, t_fixed, &mut out[last_full..]);
    }
    sum_of_lanes_u32(du, extra_bits)
}

/// `EstimateTokenCost(values, len, cfg, tokens)` of a SIMD target.
#[inline(always)]
pub fn estimate_token_cost_d<D: SimdF32>(
    d: D,
    values: &[u32],
    len: usize,
    cfg: HybridUintConfig,
    out: &mut [u32],
) -> u32 {
    let (e, m, l) = (cfg.split_exponent, cfg.msb_in_token, cfg.lsb_in_token);
    macro_rules! imp {
        ($e:literal, $m:literal, $l:literal) => {
            estimate_token_cost_impl::<D, $e, $m, $l>(d, values, len, out)
        };
    }
    if e == 0 {
        imp!(0, 0, 0)
    } else if e == 2 {
        imp!(2, 0, 1)
    } else if e == 3 {
        if m == 1 {
            if l == 0 { imp!(3, 1, 0) } else { imp!(3, 1, 2) }
        } else if l == 0 {
            imp!(3, 2, 0)
        } else {
            imp!(3, 2, 1)
        }
    } else if e == 4 {
        if m == 1 {
            if l == 0 {
                imp!(4, 1, 0)
            } else if l == 2 {
                imp!(4, 1, 2)
            } else {
                imp!(4, 1, 3)
            }
        } else if l == 0 {
            imp!(4, 2, 0)
        } else if l == 1 {
            imp!(4, 2, 1)
        } else {
            imp!(4, 2, 2)
        }
    } else if e == 5 {
        if m == 1 {
            if l == 0 {
                imp!(5, 1, 0)
            } else if l == 2 {
                imp!(5, 1, 2)
            } else {
                imp!(5, 1, 4)
            }
        } else if l == 0 {
            imp!(5, 2, 0)
        } else if l == 1 {
            imp!(5, 2, 1)
        } else if l == 2 {
            imp!(5, 2, 2)
        } else {
            imp!(5, 2, 3)
        }
    } else if e == 6 {
        if m == 0 {
            imp!(6, 0, 0)
        } else if m == 1 {
            imp!(6, 1, 5)
        } else {
            imp!(6, 2, 4)
        }
    } else if (7..=12).contains(&e) {
        match e {
            7 => imp!(7, 0, 0),
            8 => imp!(8, 0, 0),
            9 => imp!(9, 0, 0),
            10 => imp!(10, 0, 0),
            11 => imp!(11, 0, 0),
            _ => imp!(12, 0, 0),
        }
    } else {
        // JXL_DASSERT(false).
        !0
    }
}

struct Estimate<'a> {
    values: &'a [u32],
    len: usize,
    cfg: HybridUintConfig,
    out: &'a mut [u32],
}
impl Kernel for Estimate<'_> {
    type Output = u32;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> u32 {
        estimate_token_cost_d(d, self.values, self.len, self.cfg, self.out)
    }
}

/// `HWY_DYNAMIC_DISPATCH(EstimateTokenCost)` at a SIMD target. `values`
/// and `out` start at the source's pointers and extend over at least
/// `RoundUpTo(len, Lanes)` words.
pub fn estimate_token_cost_simd(
    target: Target,
    values: &[u32],
    len: usize,
    cfg: HybridUintConfig,
    out: &mut [u32],
) -> u32 {
    dispatch(
        target,
        Estimate {
            values,
            len,
            cfg,
            out,
        },
    )
}
