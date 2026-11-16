// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Numeric representation helpers for the reference Clang configuration.
//! Ordinary scalar expressions contract on AArch64, with auto-vectorization
//! disabled by upstream. These helpers mark source contraction sites, not new
//! optimizations. Highway's explicit non-fused operations do not use them.
//! LoadFloat16/LoadFloatRow below translate lib/jxl/base/float.h directly.

use super::byte_order::{load_be_float, load_be16, load_le_float, load_le16};
use super::status::{Status, StatusCode};
use crate::jxl::types::JxlDataType;

pub fn load_float16(bits16: u16) -> f32 {
    let sign = u32::from(bits16 >> 15);
    let biased_exp = u32::from((bits16 >> 10) & 0x1f);
    let mantissa = u32::from(bits16 & 0x3ff);
    if biased_exp == 0 {
        let subnormal = (1.0f32 / 16384.0) * (mantissa as f32 * (1.0f32 / 1024.0));
        return if sign != 0 { -subnormal } else { subnormal };
    }
    let biased_exp32 = if biased_exp == 0b11111 {
        0b11111111
    } else {
        biased_exp + (127 - 15)
    };
    f32::from_bits((sign << 31) | (biased_exp32 << 23) | (mantissa << (23 - 10)))
}

/// Input bytes must cover each source load. Unsupported types fail before any
/// access or callback. The callback retains the original increasing index order.
pub fn load_float_row(
    src: &[u8],
    count: usize,
    stride: usize,
    data_type: JxlDataType,
    little_endian: bool,
    scale: f32,
    mut callback: impl FnMut(usize, f32),
) -> Status {
    match data_type {
        JxlDataType::FLOAT => {
            if little_endian {
                for i in 0..count {
                    callback(i, load_le_float(&src[stride.wrapping_mul(i)..]));
                }
            } else {
                for i in 0..count {
                    callback(i, load_be_float(&src[stride.wrapping_mul(i)..]));
                }
            }
        }
        JxlDataType::UINT8 => {
            for i in 0..count {
                callback(i, f32::from(src[stride.wrapping_mul(i)]) * scale);
            }
        }
        JxlDataType::UINT16 => {
            if little_endian {
                for i in 0..count {
                    callback(i, load_le16(&src[stride.wrapping_mul(i)..]) as f32 * scale);
                }
            } else {
                for i in 0..count {
                    callback(i, load_be16(&src[stride.wrapping_mul(i)..]) as f32 * scale);
                }
            }
        }
        JxlDataType::FLOAT16 => {
            if little_endian {
                for i in 0..count {
                    callback(
                        i,
                        load_float16(load_le16(&src[stride.wrapping_mul(i)..]) as u16),
                    );
                }
            } else {
                for i in 0..count {
                    callback(
                        i,
                        load_float16(load_be16(&src[stride.wrapping_mul(i)..]) as u16),
                    );
                }
            }
        }
        _ => return Err(StatusCode::GenericError),
    }
    Ok(())
}

pub(crate) fn mul_add_f32(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}
pub(crate) fn mul_add_f64(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}

// C++ min/max retain the first operand on unordered comparison, unlike Rust's
// floating-point min/max. Clamp1 is from lib/jxl/base/common.h.
//
// std::max(a, b) is `(a < b) ? b : a` and std::min(a, b) is `(b < a) ? b : a`:
// a NaN `b` makes the comparison false and returns `a` unchanged, signaling
// NaN included. The plain compare-and-select is not enough to keep that on
// every compiler: when `a` is known not to be a NaN (a constant, for example),
// LLVM 20 to 22 (Rust 1.88 to 1.98) lower it on AArch64 to FMAXNM/FMINNM,
// which return a quiet NaN for a signaling NaN `b` instead of `a`; LLVM 23
// (the pinned nightly, Rust 1.100) keeps FCMP+FCSEL. The explicit test of
// `b` spells out the unordered case so that the result no longer depends on
// that lowering (port ADR 0002, "Results that depend on the Rust compiler",
// 2026-11-15). x86-64's MAXSS/MINSS already return the second source on an
// unordered comparison, so the test changes no x86-64 result either.
pub(crate) fn max_f32(a: f32, b: f32) -> f32 {
    if b.is_nan() {
        a
    } else if a < b {
        b
    } else {
        a
    }
}
pub(crate) fn min_f32(a: f32, b: f32) -> f32 {
    if b.is_nan() {
        a
    } else if b < a {
        b
    } else {
        a
    }
}
pub(crate) fn max_f64(a: f64, b: f64) -> f64 {
    if b.is_nan() {
        a
    } else if a < b {
        b
    } else {
        a
    }
}
pub(crate) fn clamp_f32(x: f32, low: f32, high: f32) -> f32 {
    if x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}
pub(crate) fn clamp_f64(x: f64, low: f64, high: f64) -> f64 {
    if x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}

// Decision D8 leaves NaN payloads out of the gate, but a NaN's sign can reach
// a sign test (Highway's ZeroIfNegative, `is_sign_negative`) and so decide
// between a finite and a NaN result. LLVM may commute the operands of an
// inlined `fadd`/`fmul`, while the original's scalar FADD/ADDSS/FMUL/MULSS
// keep the NaN of their first source when both sources are quiet NaNs. These
// helpers compute the source operation unchanged and only select the first
// operand's quieted NaN when it is a NaN, so they inline without letting the
// compiler's operand order pick the sign. Finite and infinite results, and
// the default NaN of an invalid operation, are the plain operation's.
/// The source's binary32 `a + b`, with first-operand NaN selection.
#[inline]
pub(crate) fn add_first_nan(a: f32, b: f32) -> f32 {
    let sum = a + b;
    if a.is_nan() { quiet_f32(a) } else { sum }
}
/// The source's binary32 `a - b`, with first-operand NaN selection.
#[inline]
pub(crate) fn sub_first_nan(a: f32, b: f32) -> f32 {
    let difference = a - b;
    if a.is_nan() { quiet_f32(a) } else { difference }
}
/// The source's binary32 `a * b`, with first-operand NaN selection.
#[inline]
pub(crate) fn mul_first_nan(a: f32, b: f32) -> f32 {
    let product = a * b;
    if a.is_nan() { quiet_f32(a) } else { product }
}
/// Highway scalar `operator*`: both operands promoted to binary64, the
/// product narrowed to binary32, with first-operand NaN selection.
#[inline]
pub(crate) fn mul_promoted_first_nan(a: f32, b: f32) -> f32 {
    let product = (f64::from(a) * f64::from(b)) as f32;
    if a.is_nan() { quiet_f32(a) } else { product }
}
#[inline]
fn quiet_f32(a: f32) -> f32 {
    f32::from_bits(a.to_bits() | 0x0040_0000)
}

#[cfg(test)]
mod tests {
    // Regression tests for port ADR 0002, "Results that depend on the Rust
    // compiler" (2026-11-15). Each NaN word reaches the helper through
    // black_box, so no constant folding decides it, and the other operand is
    // a constant the backend knows is not a NaN: the shape that LLVM 20 to 22
    // lower to AArch64 FMAXNM/FMINNM. Every asserted result is finite or an
    // operand returned unchanged, never a computed NaN.
    use super::{max_f32, max_f64, min_f32};
    use std::hint::black_box;

    const NAN_WORDS: [u32; 4] = [0x7fc1_2345, 0xffc9_8765, 0x7fa1_2345, 0xffa9_8765];
    const NAN_WORDS_64: [u64; 4] = [
        0x7ff8_1234_5678_9abc,
        0xfff8_9876_5432_1000,
        0x7ff4_1234_5678_9abc,
        0xfff4_9876_5432_1000,
    ];
    const SMALL_ALPHA: f32 = 1.0 / (1u32 << 26) as f32;

    #[test]
    fn std_max_min_keep_the_first_operand_for_a_nan_second_operand() {
        for word in NAN_WORDS {
            let nan = black_box(f32::from_bits(word));
            assert_eq!(
                max_f32(SMALL_ALPHA, nan).to_bits(),
                SMALL_ALPHA.to_bits(),
                "{word:#x}"
            );
            assert_eq!(
                max_f32(-1.0, nan).to_bits(),
                (-1.0f32).to_bits(),
                "{word:#x}"
            );
            assert_eq!(min_f32(1.0, nan).to_bits(), 1.0f32.to_bits(), "{word:#x}");
            assert_eq!(min_f32(5.0, nan).to_bits(), 5.0f32.to_bits(), "{word:#x}");
        }
        for word in NAN_WORDS_64 {
            let nan64 = black_box(f64::from_bits(word));
            assert_eq!(
                max_f64(0.25, nan64).to_bits(),
                0.25f64.to_bits(),
                "{word:#x}"
            );
        }
    }

    #[test]
    fn std_max_min_return_a_nan_first_operand_unchanged() {
        for word in NAN_WORDS {
            let nan = black_box(f32::from_bits(word));
            assert_eq!(max_f32(nan, SMALL_ALPHA).to_bits(), word, "{word:#x}");
            assert_eq!(min_f32(nan, 1.0).to_bits(), word, "{word:#x}");
        }
    }

    #[test]
    fn std_max_min_order_finite_operands_as_the_source() {
        let values = [
            -0.0f32,
            0.0,
            SMALL_ALPHA,
            0.5,
            1.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        for &a in &values {
            for &b in &values {
                let (x, y) = (black_box(a), black_box(b));
                let max = if x < y { y } else { x };
                let min = if y < x { y } else { x };
                assert_eq!(max_f32(x, y).to_bits(), max.to_bits(), "{a} {b}");
                assert_eq!(min_f32(x, y).to_bits(), min.to_bits(), "{a} {b}");
            }
        }
    }
}
