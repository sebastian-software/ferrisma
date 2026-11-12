// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Row kernels for Highway's one-lane scalar Add, Sub and operator*.
//!
//! The render stages used to call per-element `#[inline(never)]` helpers
//! (`add`, `promote`, `product`, ...). Those boundaries exist for two reasons:
//! inlining may commute the operands of an addition, which changes the result
//! when both operands carry different NaN payloads; and inlining lets the
//! optimizer fold Highway's explicit float-to-double promotion into a binary32
//! product, although the promotion quiets a signaling NaN before the product
//! selects its payload.
//!
//! These kernels move the same boundary from the element to the row. Each
//! kernel is out of line and performs exactly one IEEE operation per lane;
//! both operands are loads from two slices, passed in the source operand
//! order, exactly as both operands of the per-element helper were opaque
//! arguments. Promotion and the product stay in different kernels, so no
//! promotion is visible to the product. A scalar operand (a weight, a matrix
//! entry or a bias) is broadcast into a row first, so it is a load like the
//! other operand and never a constant or argument that canonicalization
//! could reorder. Lanes are independent, so computing a stage pass by pass
//! instead of element by element changes no operation, operand or rounding.
//!
//! Every kernel processes `out.len()` lanes; the inputs must be at least as
//! long.
//!
//! Only AArch64 runs these loops as written. On every other target each
//! kernel calls the per-element `#[inline(never)]` helper below once per lane
//! instead, so every lane executes the previous per-element code by
//! construction. Reason: LLVM treats `fadd` as commutative and may pick
//! either operand as the first source of the emitted instruction. On x86-64
//! the fused `add_abs_diff` loop added the accumulator as the second operand
//! (`addps` keeps the first source's NaN payload), so a lane with two NaN
//! operands returned the payload of the absolute difference; the per-element
//! helper returns the accumulator's. Rust cannot pin the operand order of an
//! inlined `fadd`, only the out-of-line helper boundary can, so the other
//! targets keep that boundary per element and receive only the row-span
//! hoisting of the stages, not the arithmetic speedup of these loops.
// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// The one-lane operations derive from hwy/ops/scalar-inl.h. Use of this
// derived helper code is governed by Highway's offered BSD alternative in
// licenses/HIGHWAY-BSD-3-Clause.txt.

/// `out[i] = a[i] + b[i]` (Highway scalar Add, operand order retained).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub fn add(out: &mut [f32], a: &[f32], b: &[f32]) {
    let (a, b) = (&a[..out.len()], &b[..out.len()]);
    for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
        *o = a + b;
    }
}

/// `out[i] = a[i] - b[i]` (Highway scalar Sub).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub fn sub(out: &mut [f32], a: &[f32], b: &[f32]) {
    let (a, b) = (&a[..out.len()], &b[..out.len()]);
    for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
        *o = a - b;
    }
}

/// `out[i] = |a[i] - b[i]|`: Highway scalar AbsDiff, Sub followed by the
/// sign-bit clear of Abs (no addition, so no operand can be reordered).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub fn abs_diff(out: &mut [f32], a: &[f32], b: &[f32]) {
    let (a, b) = (&a[..out.len()], &b[..out.len()]);
    for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
        *o = (a - b).abs();
    }
}

/// `out[i] = acc[i] + |a[i] - b[i]|`: `add(acc, abs_diff(a, b))` with the
/// accumulator as the first addition operand. All three operands are loads.
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub fn add_abs_diff(out: &mut [f32], acc: &[f32], a: &[f32], b: &[f32]) {
    let (acc, a, b) = (&acc[..out.len()], &a[..out.len()], &b[..out.len()]);
    for (((o, &acc), &a), &b) in out.iter_mut().zip(acc).zip(a).zip(b) {
        *o = acc + (a - b).abs();
    }
}

/// `out[i] = f64::from(a[i])`: the promotion of Highway's scalar operator*.
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub fn promote(out: &mut [f64], a: &[f32]) {
    let a = &a[..out.len()];
    for (o, &a) in out.iter_mut().zip(a) {
        *o = f64::from(a);
    }
}

/// `out[i] = (a[i] * b[i]) as f32`: the double product and its narrowing.
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub fn product(out: &mut [f32], a: &[f64], b: &[f64]) {
    let (a, b) = (&a[..out.len()], &b[..out.len()]);
    for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
        *o = (a * b) as f32;
    }
}

// The previous per-element helpers of the stages, verbatim. Other targets
// call them once per lane; the tests compare every kernel against them.
#[cfg(any(test, not(target_arch = "aarch64")))]
#[inline(never)]
pub(crate) fn add1(a: f32, b: f32) -> f32 {
    a + b
}
#[cfg(any(test, not(target_arch = "aarch64")))]
#[inline(never)]
pub(crate) fn sub1(a: f32, b: f32) -> f32 {
    a - b
}
#[cfg(any(test, not(target_arch = "aarch64")))]
#[inline(never)]
pub(crate) fn promote1(a: f32) -> f64 {
    f64::from(a)
}
#[cfg(any(test, not(target_arch = "aarch64")))]
#[inline(never)]
pub(crate) fn product1(a: f64, b: f64) -> f32 {
    (a * b) as f32
}

/// The kernels for targets other than AArch64: the same signatures, each
/// lane one call of the previous per-element helper with the same operands
/// in the same order.
#[cfg(not(target_arch = "aarch64"))]
mod per_element {
    use super::{add1, product1, promote1, sub1};

    pub fn add(out: &mut [f32], a: &[f32], b: &[f32]) {
        let (a, b) = (&a[..out.len()], &b[..out.len()]);
        for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
            *o = add1(a, b);
        }
    }
    pub fn sub(out: &mut [f32], a: &[f32], b: &[f32]) {
        let (a, b) = (&a[..out.len()], &b[..out.len()]);
        for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
            *o = sub1(a, b);
        }
    }
    /// `diff(a, b) = sub(a, b).abs()` of the EPF stage.
    pub fn abs_diff(out: &mut [f32], a: &[f32], b: &[f32]) {
        let (a, b) = (&a[..out.len()], &b[..out.len()]);
        for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
            *o = sub1(a, b).abs();
        }
    }
    /// `add(acc, diff(a, b))` of the EPF stage.
    pub fn add_abs_diff(out: &mut [f32], acc: &[f32], a: &[f32], b: &[f32]) {
        let (acc, a, b) = (&acc[..out.len()], &a[..out.len()], &b[..out.len()]);
        for (((o, &acc), &a), &b) in out.iter_mut().zip(acc).zip(a).zip(b) {
            *o = add1(acc, sub1(a, b).abs());
        }
    }
    pub fn promote(out: &mut [f64], a: &[f32]) {
        let a = &a[..out.len()];
        for (o, &a) in out.iter_mut().zip(a) {
            *o = promote1(a);
        }
    }
    pub fn product(out: &mut [f32], a: &[f64], b: &[f64]) {
        let (a, b) = (&a[..out.len()], &b[..out.len()]);
        for ((o, &a), &b) in out.iter_mut().zip(a).zip(b) {
            *o = product1(a, b);
        }
    }
}
#[cfg(not(target_arch = "aarch64"))]
pub use per_element::{abs_diff, add, add_abs_diff, product, promote, sub};

/// A row of `value` with at least `len` lanes, refilled only when `value` or
/// the required length changes. Filling is a copy, not arithmetic.
#[derive(Default)]
pub struct Broadcast {
    value: u32,
    lanes: Vec<f32>,
    wide: Vec<f64>,
}
impl Broadcast {
    /// The scalar as a row, for kernels taking a binary32 operand.
    pub fn row(&mut self, value: f32, len: usize) -> &[f32] {
        self.fill(value, len);
        &self.lanes[..len]
    }
    /// The promoted scalar as a row, produced by the `promote` kernel.
    pub fn promoted(&mut self, value: f32, len: usize) -> &[f64] {
        self.fill(value, len);
        &self.wide[..len]
    }
    fn fill(&mut self, value: f32, len: usize) {
        if self.lanes.len() < len || self.value != value.to_bits() {
            self.value = value.to_bits();
            self.lanes.clear();
            self.lanes.resize(len.max(self.lanes.capacity()), value);
            self.wide.resize(self.lanes.len(), 0.0);
            promote(&mut self.wide, &self.lanes);
        }
    }
}

/// Resize a scratch row to `len` lanes; previous contents are overwritten by
/// the kernel that writes it.
pub fn lanes<T: Copy + Default>(row: &mut Vec<T>, len: usize) -> &mut [T] {
    row.resize(len, T::default());
    &mut row[..len]
}

#[cfg(test)]
pub(crate) mod tests {
    /// Adversarial binary32 words: NaNs with distinct payloads, signs and
    /// quiet/signaling states, infinities, signed zeros, denormals, extremes.
    pub(crate) fn adversarial() -> Vec<f32> {
        [
            0x7fc0_0000u32,
            0xffc0_0000,
            0x7fc1_2345,
            0xffc9_8765,
            0x7f80_0001,
            0xff80_0002,
            0x7fa5_5555,
            0xffbf_ffff,
            0x7f80_0000,
            0xff80_0000,
            0x0000_0000,
            0x8000_0000,
            0x0000_0001,
            0x8000_0001,
            0x007f_ffff,
            0x807f_ffff,
            0x0080_0000,
            0x7f7f_ffff,
            0xff7f_ffff,
            0x3f80_0000,
            0xbf80_0000,
            0x3eaa_aaab,
            0x4b80_0001,
            0x1f80_0000,
            0x5f00_0001,
        ]
        .into_iter()
        .map(f32::from_bits)
        .collect()
    }

    /// Every ordered pair of adversarial words, as two parallel rows.
    pub(crate) fn pairs() -> (Vec<f32>, Vec<f32>) {
        let v = adversarial();
        let mut a = Vec::new();
        let mut b = Vec::new();
        for &x in &v {
            for &y in &v {
                a.push(x);
                b.push(y);
            }
        }
        (a, b)
    }

    // The previous per-element helpers, verbatim.
    use super::{add1, product1, promote1, sub1};

    fn bits(v: &[f32]) -> Vec<u32> {
        v.iter().map(|x| x.to_bits()).collect()
    }

    #[test]
    fn row_kernels_match_per_element_helpers_bit_for_bit() {
        let (a, b) = pairs();
        let n = a.len();
        // The corpus observes operand order and promotion: swapped operands,
        // or a binary32 product without promotion, give different words.
        assert!(
            a.iter()
                .zip(&b)
                .any(|(&a, &b)| add1(a, b).to_bits() != add1(b, a).to_bits())
        );
        assert!(a.iter().zip(&b).any(|(&a, &b)| {
            product1(promote1(a), promote1(b)).to_bits() != (std::hint::black_box(a) * b).to_bits()
        }));
        let mut out = vec![0.0; n];
        super::add(&mut out, &a, &b);
        let want: Vec<_> = a.iter().zip(&b).map(|(&a, &b)| add1(a, b)).collect();
        assert_eq!(bits(&out), bits(&want));
        super::sub(&mut out, &a, &b);
        let want: Vec<_> = a.iter().zip(&b).map(|(&a, &b)| sub1(a, b)).collect();
        assert_eq!(bits(&out), bits(&want));
        super::abs_diff(&mut out, &a, &b);
        let want: Vec<_> = a.iter().zip(&b).map(|(&a, &b)| sub1(a, b).abs()).collect();
        assert_eq!(bits(&out), bits(&want));
        // add(acc, diff(a, b)) on every triple of adversarial words.
        let v = adversarial();
        let mut acc = Vec::new();
        let (mut ta, mut tb) = (Vec::new(), Vec::new());
        for &x in &v {
            for (&y, &z) in a.iter().zip(&b) {
                acc.push(x);
                ta.push(y);
                tb.push(z);
            }
        }
        let mut sum = vec![0.0; acc.len()];
        super::add_abs_diff(&mut sum, &acc, &ta, &tb);
        let want: Vec<_> = (0..acc.len())
            .map(|i| add1(acc[i], sub1(ta[i], tb[i]).abs()))
            .collect();
        assert_eq!(bits(&sum), bits(&want));
        let mut wa = vec![0.0; n];
        let mut wb = vec![0.0; n];
        super::promote(&mut wa, &a);
        super::promote(&mut wb, &b);
        let want: Vec<_> = a.iter().map(|&a| promote1(a).to_bits()).collect();
        assert_eq!(wa.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), want);
        super::product(&mut out, &wa, &wb);
        let want: Vec<_> = a
            .iter()
            .zip(&b)
            .map(|(&a, &b)| product1(promote1(a), promote1(b)))
            .collect();
        assert_eq!(bits(&out), bits(&want));
        // Broadcast rows promote through the same kernel.
        let mut bc = super::Broadcast::default();
        for &s in &adversarial() {
            let row = bc.promoted(s, 7).to_vec();
            assert!(row.iter().all(|w| w.to_bits() == promote1(s).to_bits()));
            assert!(bc.row(s, 9).iter().all(|w| w.to_bits() == s.to_bits()));
        }
    }
}
