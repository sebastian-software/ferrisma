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
pub(crate) fn max_f32(a: f32, b: f32) -> f32 {
    if a < b { b } else { a }
}
pub(crate) fn min_f32(a: f32, b: f32) -> f32 {
    if b < a { b } else { a }
}
pub(crate) fn max_f64(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
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
