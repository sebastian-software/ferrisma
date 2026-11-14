// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! SIMD parity of the encoder's chroma-from-luma Highway exports (port ADR
//! 0004, "Encoder path"): the port's NEON/AVX2 instantiations against the
//! original built for the same single Highway target (`JXL_CPP_ORACLE_SIMD`),
//! through the `ecf-*` requests of tools/oracle/encoder_cfl_simd.cc. Every
//! word the original defines is compared bit for bit, except that any NaN
//! equals any NaN (decision D8). The requests go through the dispatching
//! entry points with the encoder's selection (`hwy::encoder_target`) forced
//! to the SIMD target; each family also shows that the scalar path gives
//! different words for some of its requests, so the comparison tells the two
//! paths apart.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::base::rect::Rect;
use jxl::jxl::base::status::status_code;
use jxl::jxl::enc_chroma_from_luma::CfLHeuristics;
use jxl::jxl::hwy;
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use simd_common::{check_rows, target};
use std::fmt::Write as _;
use std::sync::{Mutex, MutexGuard};

type Args<'a> = std::str::SplitWhitespace<'a>;

/// The encoder's dispatch override is process-wide: tests that set it run
/// one at a time.
fn dispatch_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn number<T: std::str::FromStr>(args: &mut Args<'_>) -> T
where
    T::Err: std::fmt::Debug,
{
    args.next().unwrap().parse().unwrap()
}
fn word(out: &mut String, v: f32) {
    write!(out, " {}", v.to_bits()).unwrap();
}
fn all_float(_: usize) -> bool {
    true
}
fn all_defined(_: usize) -> bool {
    true
}

/// Runs `requests` through `observe` with the scalar and then the SIMD
/// encoder selection, asserts that the scalar path differs somewhere, and
/// compares the SIMD observations with the original's target.
fn check_through_dispatch(family: &str, requests: &[String], observe: fn(&str) -> String) {
    let _lock = dispatch_lock();
    hwy::set_encoder_simd_for_test(Some(false));
    let scalar: Vec<String> = requests.iter().map(|r| observe(r)).collect();
    hwy::set_encoder_simd_for_test(Some(true));
    let simd: Vec<String> = requests.iter().map(|r| observe(r)).collect();
    hwy::set_encoder_simd_for_test(None);
    let differing = scalar.iter().zip(&simd).filter(|(a, b)| a != b).count();
    assert!(
        differing > 0,
        "{family}: the scalar and SIMD paths gave the same words everywhere"
    );
    println!(
        "{family}: the scalar path differs in {differing} of {} requests",
        requests.len()
    );
    let index = std::cell::Cell::new(0);
    check_rows(
        family,
        requests,
        |_| {
            let observed = simd[index.get()].clone();
            index.set(index.get() + 1);
            (observed, Box::new(all_defined))
        },
        all_float,
    );
}

// InitDCStorage --------------------------------------------------------------

/// The port's observation of an `ecf-init` request through the dispatching
/// `CfLHeuristics::init` (the caller sets the encoder's selection).
fn observe_init(request: &str) -> String {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("ecf-init"));
    let xsize: usize = number(&mut args);
    let ysize: usize = number(&mut args);
    let mut cfl = CfLHeuristics::new(MemoryManagerPtr::NATIVE);
    let status = cfl.init(Rect::new(0, 0, xsize, ysize));
    // MaxVectorSize() / sizeof(float) of the original's single target.
    let lanes = target().lanes();
    let mut out = format!("I {} {lanes}", status_code(&status));
    if status.is_err() {
        return out;
    }
    let dc = &cfl.dc_values;
    write!(out, " {} {}", dc.xsize(), dc.ysize()).unwrap();
    for y in 0..dc.ysize() {
        out.push_str(" R");
        for &v in &dc.row(y)[dc.xsize() - dc.xsize().min(lanes)..dc.xsize()] {
            word(&mut out, v);
        }
    }
    out
}

#[test]
fn enc_cfl_simd_init_dc_storage_through_the_encoder_dispatch_matches_the_original_target() {
    let mut requests = Vec::new();
    // Every block count from 0 to 60 in several shapes, pixel sizes that are
    // and are not multiples of the block size, and photograph-sized frames.
    for xb in 0..=12usize {
        for yb in 0..=5usize {
            for extra in [0usize, 7] {
                requests.push(format!(
                    "ecf-init {} {}",
                    xb * 8 + extra,
                    yb * 8 + extra % 3
                ));
            }
        }
    }
    for (x, y) in [
        (1, 1),
        (7, 9),
        (9, 7),
        (257, 31),
        (403, 302),
        (2048, 2048),
        (2268, 1512),
        (4033, 9),
    ] {
        requests.push(format!("ecf-init {x} {y}"));
    }
    check_through_dispatch("ecf-init", &requests, observe_init);
}

// The 8-lane structure, natively on AArch64 -------------------------------

/// The 8-lane structure of the chroma-from-luma kernels (port ADR 0004):
/// `Neon8` is an 8-lane descriptor made of two NEON vectors, as in
/// tests/simd_vardct_lanes8.rs, whose SumOfLanes adds in the operand order
/// of Highway's AVX2 reduction. Lane-wise results must equal the 4-lane
/// ones; the reductions are checked against a model of the original's
/// per-lane accumulation with 4 lanes and NEON's `vaddv` order, which must
/// give the NEON kernel's words, and with 8 lanes and the AVX2 order, which
/// must give `Neon8`'s. This is not AVX2 parity (that is the AVX2 oracle
/// run); it exercises the lane-count-dependent paths: `RoundUpTo(num_blocks,
/// Lanes)` and the zeroed last lanes of InitDCStorage, the `Lanes`-strided
/// accumulators and the scratch sized by `MaxVectorSize()`.
#[cfg(target_arch = "aarch64")]
mod lanes8 {
    use core::arch::aarch64::*;
    use jxl::jxl::enc_chroma_from_luma_simd::init_dc_storage_d;
    use jxl::jxl::hwy::{Neon, SimdF32};
    use jxl::jxl::image::ImageF;
    use jxl::jxl::memory_manager_internal::MemoryManagerPtr;

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
        // SumOfLanes in the operand order of Highway's AVX2 reduction (the
        // port's `avx2::reduce`): s = v + SwapAdjacentBlocks(v) gives s_i =
        // v_i + v_(i+4) in the lower block, then (s0 + s3) + (s1 + s2), the same
        // value in every lane. The other lane-crossing operations are not used
        // by the chroma-from-luma kernels.
        fn sum_of_lanes(self, v: Self::V) -> Self::V {
            let mut s = [0.0f32; 4];
            self.0.store(self.0.add(v[0], v[1]), &mut s);
            let sum = (s[0] + s[3]) + (s[1] + s[2]);
            [self.0.set(sum), self.0.set(sum)]
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
            unimplemented!("not used by the chroma-from-luma kernels")
        }
        // Signed 32-bit operations, each half through the NEON implementation,
        // as tests/simd_vardct_lanes8.rs has them.
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
    fn n4() -> Neon {
        Neon::new()
    }

    #[test]
    fn eight_lane_init_dc_storage_rounds_and_zeroes_eight_lanes() {
        for num_blocks in 0..=70usize {
            let mut four = ImageF::default();
            let mut eight = ImageF::default();
            let s4 = init_dc_storage_d(n4(), MemoryManagerPtr::NATIVE, num_blocks, &mut four);
            let s8 = init_dc_storage_d(
                Neon8(n4()),
                MemoryManagerPtr::NATIVE,
                num_blocks,
                &mut eight,
            );
            assert_eq!(s4.is_ok(), num_blocks != 0, "4 lanes, {num_blocks} blocks");
            assert_eq!(s8.is_ok(), num_blocks != 0, "8 lanes, {num_blocks} blocks");
            if num_blocks == 0 {
                continue;
            }
            assert_eq!(four.xsize(), num_blocks.div_ceil(4) * 4);
            assert_eq!(eight.xsize(), num_blocks.div_ceil(8) * 8);
            assert_eq!(eight.ysize(), 4);
            for y in 0..4 {
                let row = &eight.row(y)[..eight.xsize()];
                assert!(
                    row[row.len() - 8..].iter().all(|v| v.to_bits() == 0),
                    "8 lanes, {num_blocks} blocks, row {y}: last lanes not zero"
                );
            }
        }
    }
}
