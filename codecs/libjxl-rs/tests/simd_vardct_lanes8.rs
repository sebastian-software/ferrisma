// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The 8-lane structure of the VarDCT SIMD kernels, exercised natively on
//! AArch64 (port ADR 0004). `Neon8` is an 8-lane descriptor made of two
//! NEON vectors: every lane-wise operation runs the NEON instruction on each
//! half, so each lane computes exactly what a NEON lane computes. The VarDCT
//! kernels use only lane-wise operations, so with eight lanes they must give
//! the same words as with NEON's four: any difference is a defect of the
//! code that depends on the lane count (capped/partial vectors such as the
//! 4-column `Part<4>` transforms, loop bounds and tails, one-lane fallbacks,
//! scratch strides) and not of arithmetic. This is not AVX2 parity: AVX2's
//! own instructions (rcpps, its min/max and conversion rules) are compared
//! only with the original's AVX2 build. Scratch words are not compared
//! here, because their layout follows the lane count by design (the
//! original's `SZ`-strided temporaries).
#![cfg(target_arch = "aarch64")]

use core::arch::aarch64::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::compressed_dc_simd::{DequantRow, SmoothRow};
use jxl::jxl::dct_simd::{compute_scaled_dct_simd, compute_scaled_idct_simd};
use jxl::jxl::dct_util::ConstAcPtr;
use jxl::jxl::dec_group_simd::DequantBlockArgs;
use jxl::jxl::dec_transforms_simd::{lowest_frequencies_from_dc_d, transform_to_pixels_d};
use jxl::jxl::hwy::{Kernel, Neon, SimdF32};
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quant_weights_simd::{Powf, Weights};
use jxl::jxl::quantizer::Quantizer;

#[derive(Clone, Copy, Debug)]
struct Neon8(Neon);

macro_rules! halves {
    ($s:ident, $f:ident, $($a:ident),*) => {
        [$s.0.$f($($a[0]),*), $s.0.$f($($a[1]),*)]
    };
}

impl SimdF32 for Neon8 {
    type V = [float32x4_t; 2];
    type I = [int32x4_t; 2];
    type M = [uint32x4_t; 2];
    const LANES: usize = 8;
    const NAME: &'static str = "NEON x2";
    fn set(self, v: f32) -> Self::V {
        [self.0.set(v), self.0.set(v)]
    }
    fn zero(self) -> Self::V {
        [self.0.zero(), self.0.zero()]
    }
    fn load(self, src: &[f32]) -> Self::V {
        let src = &src[..8];
        [self.0.load(src), self.0.load(&src[4..])]
    }
    fn store(self, v: Self::V, dst: &mut [f32]) {
        let dst = &mut dst[..8];
        self.0.store(v[0], dst);
        self.0.store(v[1], &mut dst[4..]);
    }
    fn load_bits(self, src: &[u32]) -> Self::V {
        let src = &src[..8];
        [self.0.load_bits(src), self.0.load_bits(&src[4..])]
    }
    fn store_bits(self, v: Self::V, dst: &mut [u32]) {
        let dst = &mut dst[..8];
        self.0.store_bits(v[0], dst);
        self.0.store_bits(v[1], &mut dst[4..]);
    }
    fn store_i32(self, v: Self::I, dst: &mut [i32]) {
        let dst = &mut dst[..8];
        self.0.store_i32(v[0], dst);
        self.0.store_i32(v[1], &mut dst[4..]);
    }
    fn get_lane(self, v: Self::V) -> f32 {
        self.0.get_lane(v[0])
    }
    fn add(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, add, a, b)
    }
    fn sub(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, sub, a, b)
    }
    fn mul(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, mul, a, b)
    }
    fn div(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, div, a, b)
    }
    fn mul_add(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
        halves!(self, mul_add, m, x, a)
    }
    fn neg_mul_add(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
        halves!(self, neg_mul_add, m, x, a)
    }
    fn mul_sub(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
        halves!(self, mul_sub, m, x, a)
    }
    fn neg_mul_sub(self, m: Self::V, x: Self::V, a: Self::V) -> Self::V {
        halves!(self, neg_mul_sub, m, x, a)
    }
    fn min(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, min, a, b)
    }
    fn max(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, max, a, b)
    }
    fn abs(self, v: Self::V) -> Self::V {
        halves!(self, abs, v)
    }
    fn neg(self, v: Self::V) -> Self::V {
        halves!(self, neg, v)
    }
    fn abs_diff(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, abs_diff, a, b)
    }
    fn sqrt(self, v: Self::V) -> Self::V {
        halves!(self, sqrt, v)
    }
    fn approximate_reciprocal(self, v: Self::V) -> Self::V {
        halves!(self, approximate_reciprocal, v)
    }
    fn approximate_reciprocal_sqrt(self, v: Self::V) -> Self::V {
        halves!(self, approximate_reciprocal_sqrt, v)
    }
    fn floor(self, v: Self::V) -> Self::V {
        halves!(self, floor, v)
    }
    fn ceil(self, v: Self::V) -> Self::V {
        halves!(self, ceil, v)
    }
    fn round(self, v: Self::V) -> Self::V {
        halves!(self, round, v)
    }
    fn trunc(self, v: Self::V) -> Self::V {
        halves!(self, trunc, v)
    }
    fn zero_if_negative(self, v: Self::V) -> Self::V {
        halves!(self, zero_if_negative, v)
    }
    fn copy_sign(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, copy_sign, a, b)
    }
    fn copy_sign_to_abs(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, copy_sign_to_abs, a, b)
    }
    fn and(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, and, a, b)
    }
    fn or(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, or, a, b)
    }
    fn xor(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, xor, a, b)
    }
    fn and_not(self, a: Self::V, b: Self::V) -> Self::V {
        halves!(self, and_not, a, b)
    }
    fn lt(self, a: Self::V, b: Self::V) -> Self::M {
        halves!(self, lt, a, b)
    }
    fn le(self, a: Self::V, b: Self::V) -> Self::M {
        halves!(self, le, a, b)
    }
    fn gt(self, a: Self::V, b: Self::V) -> Self::M {
        halves!(self, gt, a, b)
    }
    fn ge(self, a: Self::V, b: Self::V) -> Self::M {
        halves!(self, ge, a, b)
    }
    fn eq(self, a: Self::V, b: Self::V) -> Self::M {
        halves!(self, eq, a, b)
    }
    fn ne(self, a: Self::V, b: Self::V) -> Self::M {
        halves!(self, ne, a, b)
    }
    fn vec_from_mask(self, m: Self::M) -> Self::V {
        halves!(self, vec_from_mask, m)
    }
    fn if_then_else(self, m: Self::M, y: Self::V, n: Self::V) -> Self::V {
        halves!(self, if_then_else, m, y, n)
    }
    fn if_then_zero_else(self, m: Self::M, n: Self::V) -> Self::V {
        halves!(self, if_then_zero_else, m, n)
    }
    fn if_then_else_zero(self, m: Self::M, y: Self::V) -> Self::V {
        halves!(self, if_then_else_zero, m, y)
    }
    fn convert_to_i32(self, v: Self::V) -> Self::I {
        halves!(self, convert_to_i32, v)
    }
    fn convert_to_f32(self, v: Self::I) -> Self::V {
        halves!(self, convert_to_f32, v)
    }
    fn nearest_int(self, v: Self::V) -> Self::I {
        halves!(self, nearest_int, v)
    }
    fn bitcast_to_i32(self, v: Self::V) -> Self::I {
        halves!(self, bitcast_to_i32, v)
    }
    fn bitcast_to_f32(self, v: Self::I) -> Self::V {
        halves!(self, bitcast_to_f32, v)
    }
    fn add_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, add_i32, a, b)
    }
    fn sub_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, sub_i32, a, b)
    }
    fn min_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, min_i32, a, b)
    }
    fn max_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, max_i32, a, b)
    }
    fn lt_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, lt_i32, a, b)
    }
    fn eq_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, eq_i32, a, b)
    }
    fn gt_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, gt_i32, a, b)
    }
    fn shift_left_i32<const B: i32>(self, v: Self::I) -> Self::I {
        [
            self.0.shift_left_i32::<B>(v[0]),
            self.0.shift_left_i32::<B>(v[1]),
        ]
    }
    fn shift_right_i32<const B: i32>(self, v: Self::I) -> Self::I {
        [
            self.0.shift_right_i32::<B>(v[0]),
            self.0.shift_right_i32::<B>(v[1]),
        ]
    }
    fn shift_right_u32<const B: i32>(self, v: Self::I) -> Self::I {
        [
            self.0.shift_right_u32::<B>(v[0]),
            self.0.shift_right_u32::<B>(v[1]),
        ]
    }
    // Lane-crossing operations: the VarDCT kernels do not use them.
    fn sum_of_lanes(self, _: Self::V) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn max_of_lanes(self, _: Self::V) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn min_of_lanes(self, _: Self::V) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn interleave_lower(self, _: Self::V, _: Self::V) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn interleave_upper(self, _: Self::V, _: Self::V) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn store_interleaved2(self, _: Self::V, _: Self::V, _: &mut [f32]) {
        unimplemented!("lane-crossing")
    }
    fn store_interleaved3(self, _: Self::V, _: Self::V, _: Self::V, _: &mut [f32]) {
        unimplemented!("lane-crossing")
    }
    fn load_dup128(self, _: &[f32]) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn broadcast<const LANE: i32>(self, _: Self::V) -> Self::V {
        unimplemented!("lane-crossing")
    }
    fn store_demote_to_f16(self, _: Self::V, _: &mut [u16]) {
        unimplemented!("not used by the VarDCT kernels")
    }
    // Signed 32-bit operations of the modular and stages2 packages, each half
    // through the NEON implementation. The VarDCT kernels do not use them;
    // they are here because the layer trait requires them.
    fn set_i32(self, v: i32) -> Self::I {
        [self.0.set_i32(v), self.0.set_i32(v)]
    }
    fn load_i32(self, src: &[i32]) -> Self::I {
        let src = &src[..8];
        [self.0.load_i32(src), self.0.load_i32(&src[4..])]
    }
    fn and_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, and_i32, a, b)
    }
    fn xor_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, xor_i32, a, b)
    }
    fn abs_i32(self, v: Self::I) -> Self::I {
        halves!(self, abs_i32, v)
    }
    fn neg_i32(self, v: Self::I) -> Self::I {
        halves!(self, neg_i32, v)
    }
    fn ne_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, ne_i32, a, b)
    }
    fn if_then_else_i32(self, m: Self::I, yes: Self::I, no: Self::I) -> Self::I {
        halves!(self, if_then_else_i32, m, yes, no)
    }
    fn if_then_zero_else_i32(self, m: Self::I, no: Self::I) -> Self::I {
        halves!(self, if_then_zero_else_i32, m, no)
    }
    fn mul_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, mul_i32, a, b)
    }
    // Block-wise on AVX2, so one NEON operation per 128-bit half gives the
    // AVX2 block structure (Highway x86_256-inl.h at the pinned revision):
    // MulEven is `_mm256_mul_epi32`, which multiplies the low 32 bits of each
    // 64-bit element (lanes 0, 2 | 4, 6); MulOdd for 32-bit lanes is
    // `MulEven(DupOdd(a), DupOdd(b))` (x86_128-inl.h), DupOdd shuffling
    // within each block; InterleaveOdd (non-AVX3 path) is two
    // `_mm256_shuffle_ps`, each per 128-bit lane, giving a1 b1 a3 b3 | a5 b5
    // a7 b7; TableLookupBytes is `_mm256_shuffle_epi8`, which looks up within
    // each 128-bit lane. Per half, the byte-index rule stays NEON's
    // (`vqtbl1q_u8`), as every operation of this descriptor does.
    fn mul_even_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, mul_even_i32, a, b)
    }
    fn mul_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, mul_odd_i32, a, b)
    }
    fn interleave_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
        halves!(self, interleave_odd_i32, a, b)
    }
    fn table_lookup_bytes(self, bytes: Self::I, from: Self::I) -> Self::I {
        halves!(self, table_lookup_bytes, bytes, from)
    }
}

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

const SPECIAL: [u32; 12] = [
    1, 0x80000001, 0x007fffff, 0x00800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
    0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
];

/// Values of mode `mode`: random finite, random words, specials spread.
fn values(n: usize, mode: usize, seed: &mut u64) -> Vec<f32> {
    (0..n)
        .map(|i| match mode {
            0 => (next(seed) as i32 % 16384) as f32 / 32.0,
            1 => f32::from_bits(next(seed) as u32),
            2 => f32::from_bits((next(seed) as u32 & 0x807f_ffff) | 1),
            _ => {
                if i % 5 == 0 {
                    f32::from_bits(SPECIAL[(i / 5) % SPECIAL.len()])
                } else {
                    (next(seed) as i32 % 4096) as f32 / 64.0
                }
            }
        })
        .collect()
}

/// Bit-identical words, except that any NaN equals any NaN (decision D8:
/// which competing NaN payload survives depends on operand order that the
/// compiler may commute, and is not gated).
fn assert_same(what: &str, four: &[f32], eight: &[f32]) {
    assert_eq!(four.len(), eight.len(), "{what}: lengths");
    for (i, (a, b)) in four.iter().zip(eight).enumerate() {
        if a.to_bits() != b.to_bits() && !(a.is_nan() && b.is_nan()) {
            panic!(
                "{what}: 4 and 8 lanes differ at word {i}: {:#010x} != {:#010x}",
                a.to_bits(),
                b.to_bits()
            );
        }
    }
}

/// One inverse (coefficients, pixels, scratch) or forward (samples,
/// coefficients, scratch) transform run.
type InverseRun<'a> = dyn Fn(&mut [f32], &mut [f32], &mut [f32]) + 'a;
type ForwardRun<'a> = dyn Fn(&[f32], &mut [f32], &mut [f32]) + 'a;

fn n4() -> Neon {
    Neon::new()
}

#[test]
fn eight_lane_transforms_equal_four_lane_transforms() {
    let mut seed = 0x8888_4444;
    let mut cases = 0;
    let shapes: Vec<(usize, usize)> = [1usize, 2, 4, 8, 16, 32]
        .iter()
        .flat_map(|&r| [1usize, 2, 4, 8, 16, 32].map(move |c| (r, c)))
        .chain([
            (64, 32),
            (32, 64),
            (64, 64),
            (128, 64),
            (256, 256),
            (1, 256),
            (256, 1),
        ])
        .collect();
    for (rows, cols) in shapes {
        for mode in 0..4 {
            for pad in [0, 3] {
                let stride = cols + pad;
                // Inverse: coefficients rows x cols into rows of `stride`.
                let input = values(rows * cols, mode, &mut seed);
                let run_i = |d: &InverseRun| {
                    let mut i = input.clone();
                    let mut o = vec![-1.5f32; rows * stride + 4];
                    let mut s = vec![-2.5f32; rows * cols * 4 + 3 * (rows + cols) + 4];
                    d(&mut i, &mut o, &mut s);
                    (i, o)
                };
                let (i4, o4) =
                    run_i(&|i, o, s| compute_scaled_idct_simd(n4(), rows, cols, i, o, stride, s));
                let (i8, o8) = run_i(&|i, o, s| {
                    compute_scaled_idct_simd(Neon8(n4()), rows, cols, i, o, stride, s)
                });
                assert_same(&format!("idct {rows}x{cols} mode {mode}"), &i4, &i8);
                assert_same(&format!("idct {rows}x{cols} mode {mode}"), &o4, &o8);
                // Forward: samples rows of `stride` into rows x cols.
                let input = values(rows * stride, mode, &mut seed);
                let run_f = |d: &ForwardRun| {
                    let mut o = vec![-1.5f32; rows * cols + 4];
                    let mut s = vec![-2.5f32; rows * cols * 4 + 3 * (rows + cols) + 4];
                    d(&input, &mut o, &mut s);
                    o
                };
                let f4 =
                    run_f(&|i, o, s| compute_scaled_dct_simd(n4(), rows, cols, i, stride, o, s));
                let f8 = run_f(&|i, o, s| {
                    compute_scaled_dct_simd(Neon8(n4()), rows, cols, i, stride, o, s)
                });
                assert_same(&format!("dct {rows}x{cols} mode {mode}"), &f4, &f8);
                cases += 2;
            }
        }
    }
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let (rows, cols) = (acs.covered_blocks_y() * 8, acs.covered_blocks_x() * 8);
        for mode in 0..4 {
            for pad in [0, 5] {
                let stride = cols + pad;
                let input = values(rows * cols, mode, &mut seed);
                let run = |eight: bool| {
                    let mut i = input.clone();
                    let mut o = vec![-1.5f32; rows * stride + 4];
                    let mut s = vec![-2.5f32; rows * cols * 4 + 3 * (rows + cols) + 4];
                    if eight {
                        transform_to_pixels_d(
                            Neon8(n4()),
                            AcStrategyType(raw),
                            &mut i,
                            &mut o,
                            stride,
                            &mut s,
                        );
                    } else {
                        transform_to_pixels_d(
                            n4(),
                            AcStrategyType(raw),
                            &mut i,
                            &mut o,
                            stride,
                            &mut s,
                        );
                    }
                    (i, o)
                };
                let (i4, o4) = run(false);
                let (i8, o8) = run(true);
                assert_same(&format!("pixels {raw} mode {mode}"), &i4, &i8);
                assert_same(&format!("pixels {raw} mode {mode}"), &o4, &o8);
                // LowestFrequenciesFromDC from a DC block of `stride` rows.
                let (r, c) = (acs.covered_blocks_y(), acs.covered_blocks_x());
                let dc = values(r * (c + pad), mode, &mut seed);
                let llf = |eight: bool| {
                    let mut o = vec![-1.5f32; r * c * 64 + 4];
                    let mut s = vec![-2.5f32; r * c * 64 * 4 + 260];
                    if eight {
                        lowest_frequencies_from_dc_d(
                            Neon8(n4()),
                            AcStrategyType(raw),
                            &dc,
                            c + pad,
                            &mut o,
                            &mut s,
                        );
                    } else {
                        lowest_frequencies_from_dc_d(
                            n4(),
                            AcStrategyType(raw),
                            &dc,
                            c + pad,
                            &mut o,
                            &mut s,
                        );
                    }
                    o
                };
                assert_same(&format!("llf {raw} mode {mode}"), &llf(false), &llf(true));
                cases += 2;
            }
        }
    }
    println!("8-lane transforms: {cases} cases equal the 4-lane results");
}

#[test]
fn eight_lane_dequantization_equals_four_lane_dequantization() {
    let mut seed = 0x0001_2345;
    let biases = [0.94f32, 0.93, 0.95, 0.145];
    let mut cases = 0;
    for raw in 0..27u32 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let size = acs.covered_blocks_x() * acs.covered_blocks_y() * 64;
        let mut matrices = DequantMatrices::default();
        matrices.ensure_computed(1 << raw).unwrap();
        let quantizer = Quantizer::new(&matrices);
        for ty in 0..2 {
            for mode in 0..3 {
                let q32: [Vec<i32>; 3] = std::array::from_fn(|_| {
                    (0..size)
                        .map(|k| match mode {
                            0 => [0, 1, -1, 2, -2, 3, -3, 32767, -32768][k % 9],
                            1 => (next(&mut seed) as i32) >> (next(&mut seed) % 32),
                            _ => [i32::MIN, i32::MAX, 16777217, -16777217][k % 4],
                        })
                        .collect()
                });
                let q16: [Vec<i16>; 3] = q32
                    .each_ref()
                    .map(|v| v.iter().map(|&x| x as i16).collect());
                let stride = acs.covered_blocks_x() + 3;
                let dc: [Vec<f32>; 3] = std::array::from_fn(|_| {
                    values(acs.covered_blocks_y() * stride + 4, mode, &mut seed)
                });
                let run = |eight: bool| {
                    let blocks = if ty == 0 {
                        q16.each_ref().map(|v| ConstAcPtr::I16(v))
                    } else {
                        q32.each_ref().map(|v| ConstAcPtr::I32(v))
                    };
                    let mut block = vec![-1.5f32; size * 3 + 4];
                    let mut scratch = vec![-2.5f32; size * 4 + 260];
                    let args = DequantBlockArgs {
                        inv_global_scale: 0.125,
                        quant: 3,
                        x_dm_multiplier: 0.8,
                        b_dm_multiplier: 1.25,
                        x_cc_mul: -0.125,
                        b_cc_mul: 0.375,
                        kind: acs.strategy(),
                        size,
                        quantizer: &quantizer,
                        matrices: &matrices,
                        covered_blocks: acs.covered_blocks_x() * acs.covered_blocks_y(),
                        sbx: [1, 2, 0],
                        dc_rows: dc.each_ref().map(|v| &v[..]),
                        dc_stride: stride,
                        biases: &biases,
                        qblock: &blocks,
                        block: &mut block,
                        scratch: &mut scratch,
                    };
                    if eight {
                        args.run(Neon8(n4()));
                    } else {
                        args.run(n4());
                    }
                    block
                };
                assert_same(
                    &format!("dq-block {raw} {ty} {mode}"),
                    &run(false),
                    &run(true),
                );
                cases += 1;
            }
        }
    }
    println!("8-lane DequantBlock: {cases} cases equal the 4-lane results");
}

#[test]
fn eight_lane_dc_and_quant_weights_equal_four_lane_results() {
    let mut seed = 0xdc_dc;
    let mut cases = 0;
    // AdaptiveDCSmoothing rows: widths around both lane counts.
    for width in 3..42 {
        for mode in 0..4 {
            let rows_data: Vec<[Vec<f32>; 3]> = (0..3)
                .map(|_| std::array::from_fn(|_| values(width, mode, &mut seed)))
                .collect();
            let factors = [0.125f32, 0.5, 2.0];
            for channels in [1usize, 3] {
                let rows: Vec<[&[f32]; 3]> = rows_data[..channels]
                    .iter()
                    .map(|r| [&r[0][..], &r[1][..], &r[2][..]])
                    .collect();
                let run = |eight: bool| {
                    let mut out: Vec<Vec<f32>> = rows.iter().map(|r| r[1].to_vec()).collect();
                    let k = SmoothRow {
                        dc_factors: &factors,
                        rows: &rows,
                        xsize: width,
                        out: &mut out,
                    };
                    if eight {
                        k.run(Neon8(n4()));
                    } else {
                        k.run(n4());
                    }
                    out.concat()
                };
                assert_same(
                    &format!("smooth {width} {channels}"),
                    &run(false),
                    &run(true),
                );
                cases += 1;
            }
        }
    }
    // DequantDC rows: lengths around both lane counts.
    for n in 1..42 {
        let quant: [Vec<i32>; 3] = std::array::from_fn(|_| {
            (0..n)
                .map(|_| (next(&mut seed) as i32) >> (next(&mut seed) % 32))
                .collect()
        });
        for cfl in [Some([-0.75f32, 1.25]), None] {
            let channels = if cfl.is_some() { 3 } else { 1 };
            let run = |eight: bool| {
                let mut o: [Vec<f32>; 3] = std::array::from_fn(|_| vec![-1.5f32; n]);
                let [a, b, c] = &mut o;
                let k = DequantRow {
                    cfl,
                    factors: [0.25, 0.5, 1.75],
                    quant: [&quant[0], &quant[1], &quant[2]],
                    out: [a, b, c],
                    n,
                    channels,
                };
                if eight {
                    k.run(Neon8(n4()));
                } else {
                    k.run(n4());
                }
                o.concat()
            };
            assert_same(&format!("dequant dc {n}"), &run(false), &run(true));
            cases += 1;
        }
    }
    // GetQuantWeights channels and the one-lane FastPowf.
    for (rows, cols) in [
        (4, 4),
        (4, 8),
        (8, 8),
        (16, 8),
        (8, 32),
        (64, 64),
        (256, 128),
    ] {
        for num_bands in [1usize, 2, 4, 8, 16] {
            let mut bands = [0.0f32; 17];
            bands[0] = 1.0 + (next(&mut seed) % 4000) as f32 / 8.0;
            for i in 1..num_bands {
                bands[i] = bands[i - 1] * (1.0 + (next(&mut seed) % 1000) as f32 / 1000.0);
            }
            let run = |eight: bool| {
                let mut out = vec![-1.5f32; rows * cols];
                let k = Weights {
                    rows,
                    cols,
                    bands: &bands,
                    num_bands,
                    out: &mut out,
                };
                if eight {
                    k.run(Neon8(n4()));
                } else {
                    k.run(n4());
                }
                out
            };
            assert_same(
                &format!("weights {rows}x{cols} {num_bands}"),
                &run(false),
                &run(true),
            );
            cases += 1;
        }
    }
    for _ in 0..500 {
        let base = (next(&mut seed) % 100_000) as f32 / 1000.0 + 0.001;
        let exponent = (next(&mut seed) % 2000) as f32 / 1000.0;
        let p4 = Powf { base, exponent }.run(n4());
        let p8 = Powf { base, exponent }.run(Neon8(n4()));
        assert_eq!(p4.to_bits(), p8.to_bits(), "FastPowf({base}, {exponent})");
        cases += 1;
    }
    println!(
        "8-lane DC smoothing, DequantDC and quant weights: {cases} cases equal the 4-lane results"
    );
}

/// The transforms with exactly the buffer sizes of the `dct-*` parity
/// requests (tools/oracle/dct.cc, tests/simd_vardct.rs) and DequantBlock's
/// `dq-block` scratch, with eight lanes: 8-lane column groups need more
/// transform temporaries than 4 lanes (`SZ * (3N - 4)` floats for an N-point
/// DCT), so a request scratch sized for NEON overflows on AVX2. The first
/// hosted AVX2 run panicked here (`mid > len`) for DCT128X64/DCT64X128,
/// whose `dct-llf` scratch was then sized from block counts (588 floats,
/// 608 needed) instead of the caller's 4 * max_block_area.
#[test]
fn eight_lane_transforms_fit_the_parity_requests_scratch() {
    let mut seed = 0x0bad_5eed;
    let scratch_size = |kind: &str, rows: usize, cols: usize| {
        if kind == "dct-llf" {
            rows * cols * 64 * 4 + 4
        } else {
            rows * cols * 4 + 3 * (rows + cols) + 4
        }
    };
    let mut cases = 0;
    for raw in 0..27 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        let (rows, cols) = (acs.covered_blocks_y(), acs.covered_blocks_x());
        for pad in [0, 3] {
            let stride = cols + pad;
            let dc = values(rows * stride, 0, &mut seed);
            let run = |eight: bool| {
                let mut o = vec![-1.5f32; rows * cols * 64 + 4];
                let mut s = vec![-2.5f32; scratch_size("dct-llf", rows, cols)];
                if eight {
                    lowest_frequencies_from_dc_d(
                        Neon8(n4()),
                        AcStrategyType(raw),
                        &dc,
                        stride,
                        &mut o,
                        &mut s,
                    );
                } else {
                    lowest_frequencies_from_dc_d(
                        n4(),
                        AcStrategyType(raw),
                        &dc,
                        stride,
                        &mut o,
                        &mut s,
                    );
                }
                o
            };
            assert_same(&format!("llf {raw} pad {pad}"), &run(false), &run(true));
            // dct-pixels with the pixel dimensions of the strategy.
            let (prows, pcols) = (rows * 8, cols * 8);
            let pstride = pcols + pad;
            let input = values(prows * pcols, 0, &mut seed);
            let run = |eight: bool| {
                let mut i = input.clone();
                let mut o = vec![-1.5f32; prows * pstride + 4];
                let mut s = vec![-2.5f32; scratch_size("dct-pixels", prows, pcols)];
                if eight {
                    transform_to_pixels_d(
                        Neon8(n4()),
                        AcStrategyType(raw),
                        &mut i,
                        &mut o,
                        pstride,
                        &mut s,
                    );
                } else {
                    transform_to_pixels_d(
                        n4(),
                        AcStrategyType(raw),
                        &mut i,
                        &mut o,
                        pstride,
                        &mut s,
                    );
                }
                o
            };
            assert_same(&format!("pixels {raw} pad {pad}"), &run(false), &run(true));
            cases += 2;
        }
    }
    // dct-scaled: every shape of the parity requests, both directions.
    let shapes: Vec<(usize, usize)> = [1usize, 2, 4, 8, 16, 32]
        .iter()
        .flat_map(|&r| [1usize, 2, 4, 8, 16, 32].map(move |c| (r, c)))
        .chain([
            (64, 32),
            (32, 64),
            (64, 64),
            (128, 64),
            (64, 128),
            (128, 128),
            (256, 128),
            (128, 256),
            (256, 256),
            (1, 256),
            (256, 1),
            (2, 128),
            (128, 4),
        ])
        .collect();
    for (rows, cols) in shapes {
        for pad in [0, 3, 7] {
            let stride = cols + pad;
            let size = scratch_size("dct-scaled", rows, cols);
            let input = values(rows * cols, 0, &mut seed);
            let inv = |eight: bool| {
                let mut i = input.clone();
                let mut o = vec![-1.5f32; rows * stride + 4];
                let mut s = vec![-2.5f32; size];
                if eight {
                    compute_scaled_idct_simd(
                        Neon8(n4()),
                        rows,
                        cols,
                        &mut i,
                        &mut o,
                        stride,
                        &mut s,
                    );
                } else {
                    compute_scaled_idct_simd(n4(), rows, cols, &mut i, &mut o, stride, &mut s);
                }
                o
            };
            assert_same(&format!("idct {rows}x{cols}"), &inv(false), &inv(true));
            let samples = values(rows * stride, 0, &mut seed);
            let fwd = |eight: bool| {
                let mut o = vec![-1.5f32; rows * cols + 4];
                let mut s = vec![-2.5f32; size];
                if eight {
                    compute_scaled_dct_simd(
                        Neon8(n4()),
                        rows,
                        cols,
                        &samples,
                        stride,
                        &mut o,
                        &mut s,
                    );
                } else {
                    compute_scaled_dct_simd(n4(), rows, cols, &samples, stride, &mut o, &mut s);
                }
                o
            };
            assert_same(&format!("dct {rows}x{cols}"), &fwd(false), &fwd(true));
            cases += 2;
        }
    }
    println!("8-lane transforms with the parity requests' buffers: {cases} cases fit and match");
}

// encoder simd: the 8-lane structure of the encoder's SIMD kernels (port
// ADR 0004, "Encoder path"). Their lane-count-dependent parts are the
// `Lanes`-strided constants of ComputePremulAbsorb and the whole-vector row
// loop; the arithmetic is lane-wise.
mod encoder_simd {
    use super::{Neon8, assert_same, n4, next};
    use jxl::jxl::enc_xyb_simd::{premul_absorb_for_lanes, xyb_row_d};

    #[test]
    fn eight_lane_xyb_rows_equal_four_lane_xyb_rows() {
        let mut seed = 0x0e0c_5eed;
        let mut cases = 0;
        for xsize in 1..=41usize {
            for srgb in [false, true] {
                for intensity in [255.0f32, 80.0, 10000.0] {
                    let padded = xsize.div_ceil(8) * 8;
                    let input: Vec<Vec<f32>> = (0..3)
                        .map(|_| {
                            (0..padded + 4)
                                .map(|_| {
                                    (next(&mut seed) % (3 << 22)) as f32 / (1u32 << 23) as f32
                                        - 0.25
                                })
                                .collect()
                        })
                        .collect();
                    let run = |eight: bool| {
                        let lanes = if eight { 8 } else { 4 };
                        let mut premul = vec![0.0f32; 12 * lanes];
                        premul_absorb_for_lanes(lanes, intensity, &mut premul);
                        let mut rows = input.clone();
                        let [r0, r1, r2] = &mut rows[..] else {
                            unreachable!()
                        };
                        let rows3 = [&mut r0[..], &mut r1[..], &mut r2[..]];
                        if eight {
                            xyb_row_d(Neon8(n4()), rows3, &premul, xsize, srgb);
                        } else {
                            xyb_row_d(n4(), rows3, &premul, xsize, srgb);
                        }
                        rows.concat()
                    };
                    let (four, eight) = (run(false), run(true));
                    // Four lanes leave the samples from RoundUpTo(xsize, 4)
                    // on untouched where eight lanes convert them; compare
                    // the converted prefix and the untouched tail.
                    let row = padded + 4;
                    for c in 0..3 {
                        let done = xsize.div_ceil(4) * 4;
                        assert_same(
                            &format!("xyb row {xsize} srgb {srgb} plane {c}"),
                            &four[c * row..c * row + done],
                            &eight[c * row..c * row + done],
                        );
                        assert_same(
                            &format!("xyb row tail {xsize} plane {c}"),
                            &eight[c * row + padded..(c + 1) * row],
                            &input[c][padded..],
                        );
                    }
                    cases += 1;
                }
            }
        }
        println!("8-lane XYB rows: {cases} cases match");
    }
}
// end encoder simd
