// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Sample conversion from lib/jxl/enc_modular.cc. Frame encoding is unported.
//! Disjoint slices replace the source's row pointers. Source-undefined floating
//! conversions and shifts remain caller preconditions, checked with assertions.
//! A failed custom-float conversion retains its already written output prefix.

use super::base::float::{mul_add_f32, mul_add_f64};
use super::base::status::{Status, StatusCode};

/// Convert normalized integer samples or exactly representable custom floats.
///
/// Both rows must contain `xsize` samples. Integer conversion requires finite
/// products whose rounded, truncated result fits i32. Custom floats other than
/// binary32 require 1..=8 exponent bits and 0..=23 mantissa bits.
pub fn float_to_int(
    row_in: &[f32],
    row_out: &mut [i32],
    xsize: usize,
    bits: u32,
    exp_bits: u32,
    fp: bool,
    dfactor: f64,
) -> Status {
    if bits > 32 {
        return Err(StatusCode::GenericError);
    }
    if !fp {
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        if bits > 22 {
            for x in 0..xsize {
                let value = mul_add_f64(
                    row_in[x] as f64,
                    dfactor,
                    if row_in[x] < 0.0 { -0.5 } else { 0.5 },
                );
                assert!(value.trunc() >= i32::MIN as f64 && value.trunc() <= i32::MAX as f64);
                row_out[x] = value as i32;
            }
        } else {
            let factor = dfactor as f32;
            for x in 0..xsize {
                let value =
                    mul_add_f32(row_in[x], factor, if row_in[x] < 0.0 { -0.5 } else { 0.5 });
                assert!(
                    value.trunc() as f64 >= i32::MIN as f64
                        && value.trunc() as f64 <= i32::MAX as f64
                );
                row_out[x] = value as i32;
            }
        }
        return Ok(());
    }
    if bits == 32 {
        if exp_bits != 8 {
            return Err(StatusCode::GenericError);
        }
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        for x in 0..xsize {
            row_out[x] = row_in[x].to_bits() as i32;
        }
        return Ok(());
    }
    if bits == 0 {
        return Err(StatusCode::GenericError);
    }
    assert!((1..=8).contains(&exp_bits) && bits > exp_bits && bits - exp_bits - 1 <= 23);
    let exp_bias = (1i32 << (exp_bits - 1)) - 1;
    let max_exp = (1i32 << exp_bits) - 1;
    let sign = 1u32 << (bits - 1);
    let mant_bits = (bits - exp_bits - 1) as i32;
    let mant_shift = 23 - mant_bits;
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let mut f = row_in[x].to_bits();
        let signbit = f >> 31;
        f &= 0x7fff_ffff;
        if f == 0 {
            row_out[x] = if signbit != 0 { sign as i32 } else { 0 };
            continue;
        }
        let mut exp = (f >> 23) as i32 - 127;
        let mut mantissa = (f & 0x007f_ffff) as i32;
        if exp == 128 {
            f = if signbit != 0 { sign } else { 0 };
            f |= (((1i32 << exp_bits) - 1) << mant_bits) as u32;
            f |= (mantissa >> mant_shift) as u32;
            row_out[x] = f as i32;
            continue;
        }
        exp += exp_bias;
        if exp <= 0 {
            mantissa |= 0x0080_0000;
            if exp < -mant_bits {
                return Err(StatusCode::GenericError);
            }
            mantissa >>= 1 - exp;
            exp = 0;
        }
        if exp >= max_exp {
            return Err(StatusCode::GenericError);
        }
        if mantissa & ((1 << mant_shift) - 1) != 0 {
            return Err(StatusCode::GenericError);
        }
        mantissa >>= mant_shift;
        f = if signbit != 0 { sign } else { 0 };
        f |= (exp << mant_bits) as u32;
        f |= mantissa as u32;
        row_out[x] = f as i32;
    }
    Ok(())
}
