// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this source code is governed by Highway's offered BSD alternative in
// licenses/HIGHWAY-BSD-3-Clause.txt. Modified: translated the used one-lane
// Min/Max/MulAdd/Floor/Round/ConvertTo/NearestInt/sign operations from hwy/ops/scalar-inl.h and
// generic_ops-inl.h. Other operations and SIMD targets remain separate work.
pub(crate) fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}
pub(crate) fn min(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a < b { a } else { b }
}
pub(crate) fn max(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return b;
    }
    if b.is_nan() {
        return a;
    }
    if a > b { a } else { b }
}
pub(crate) fn clamp(x: f32, lo: f32, hi: f32) -> f32 {
    min(max(lo, x), hi)
}
pub(crate) fn zero_if_negative(x: f32) -> f32 {
    if x.to_bits() >> 31 != 0 { 0.0 } else { x }
}
pub(crate) fn floor(x: f32) -> f32 {
    let negative = x < 0.0;
    let mut bits = x.to_bits();
    let exponent = ((bits >> 23) & 255) as i32 - 127;
    if exponent >= 23 {
        return x;
    }
    if exponent < 0 {
        return if negative { -1.0 } else { 0.0 };
    }
    let mask = 0x7fffffu32 >> exponent;
    if bits & mask == 0 {
        return x;
    }
    if negative {
        bits = bits.wrapping_add(0x800000u32 >> exponent);
    }
    bits &= !mask;
    f32::from_bits(bits)
}
// scalar-inl.h Round: huge and NaN values are returned unchanged; zero
// results keep the input's sign; ties round to even.
pub(crate) fn round(v: f32) -> f32 {
    if v.is_nan() || v.abs() >= 8388608.0 {
        return v;
    }
    let bias = if v < 0.0 { -0.5f32 } else { 0.5f32 };
    let rounded = (v + bias) as i32;
    if rounded == 0 {
        return 0.0f32.copysign(v);
    }
    let mut offset = 0;
    if rounded & 1 != 0 && ((rounded as f32) - v).abs() == 0.5 {
        offset = if v < 0.0 { -1 } else { 1 };
    }
    (rounded - offset) as f32
}
pub(crate) fn to_i32(x: f32) -> i32 {
    if (x.to_bits() & 0x7fffffff) >> 23 < 158 {
        x as i32
    } else if x.to_bits() >> 31 != 0 {
        i32::MIN
    } else {
        i32::MAX
    }
}
// scalar-inl.h NearestInt. The integer export's clamped values times its
// <=16-bit scale stay in the source's defined conversion range.
pub(crate) fn nearest_int(x: f32) -> i32 {
    let abs = x.abs();
    let sign = x.to_bits() >> 31 != 0;
    let in_mantissa_range = abs < 8388608.0;
    if !in_mantissa_range {
        let within_int_range = abs <= i32::MAX as f32;
        if !within_int_range {
            return if sign { i32::MIN } else { i32::MAX };
        }
        return x as i32;
    }
    let bias = if x < 0.0 { -0.5 } else { 0.5 };
    let rounded = (x + bias) as i32;
    if rounded == 0 {
        return 0;
    }
    let mut offset = 0;
    if rounded & 1 != 0 && ((rounded as f32) - x).abs() == 0.5 {
        offset = if sign { -1 } else { 1 };
    }
    rounded - offset
}
