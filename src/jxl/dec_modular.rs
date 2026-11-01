// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ModularStreamId storage, constructors, and index mapping from
//! lib/jxl/dec_modular.h, and sample conversion from lib/jxl/dec_modular.cc.
//! The frame decoder and debug descriptions are unported. The three Highway
//! kernels retain the one-lane scalar configuration; disjoint row slices replace
//! restrict pointers. Custom-float shifts retain source caller preconditions.
//! Kind retains raw fixed-underlying-type enum values: the source switch leaves
//! an unknown kind's initialized index at zero. All size_t arithmetic wraps.

use super::base::status::{Status, StatusCode, StatusOr};
use super::frame_dimensions::FrameDimensions;
use super::quant_weights::NUM_QUANT_TABLES;

pub fn multiply_sum(
    xsize: usize,
    row_in: &[i32],
    row_in_y: &[i32],
    factor: f32,
    row_out: &mut [f32],
) {
    assert!(row_in.len() >= xsize && row_in_y.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let input = row_in[x].wrapping_add(row_in_y[x]);
        row_out[x] = input as f32 * factor;
    }
}

pub fn rgb_from_single(
    xsize: usize,
    row_in: &[i32],
    factor: f32,
    out_r: &mut [f32],
    out_g: &mut [f32],
    out_b: &mut [f32],
) {
    assert!(
        row_in.len() >= xsize
            && out_r.len() >= xsize
            && out_g.len() >= xsize
            && out_b.len() >= xsize
    );
    for x in 0..xsize {
        let output = row_in[x] as f32 * factor;
        out_r[x] = output;
        out_g[x] = output;
        out_b[x] = output;
    }
}

pub fn single_from_single(xsize: usize, row_in: &[i32], factor: f32, row_out: &mut [f32]) {
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        row_out[x] = row_in[x] as f32 * factor;
    }
}

pub fn single_from_single_accurate(xsize: usize, row_in: &[i32], factor: f64, row_out: &mut [f32]) {
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        row_out[x] = (row_in[x] as f64 * factor) as f32;
    }
}

/// Convert a custom floating-point bit pattern to binary32 without rounding.
///
/// Both rows must contain `xsize` samples. Except for binary32, the source
/// requires positive width, 1..=8 exponent bits, and 0..=23 mantissa bits.
pub fn int_to_float(
    row_in: &[i32],
    row_out: &mut [f32],
    xsize: usize,
    bits: i32,
    exp_bits: i32,
) -> Status {
    if bits == 32 {
        if exp_bits != 8 {
            return Err(StatusCode::GenericError);
        }
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        for x in 0..xsize {
            row_out[x] = f32::from_bits(row_in[x] as u32);
        }
        return Ok(());
    }
    assert!(
        (1..=31).contains(&bits)
            && (1..=8).contains(&exp_bits)
            && bits > exp_bits
            && bits - exp_bits - 1 <= 23
    );
    let exp_bias = (1 << (exp_bits - 1)) - 1;
    let sign_shift = bits - 1;
    let mant_bits = bits - exp_bits - 1;
    let mant_shift = 23 - mant_bits;
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let mut f = row_in[x] as u32;
        let signbit = f >> sign_shift;
        f &= (1 << sign_shift) - 1;
        if f == 0 {
            row_out[x] = if signbit != 0 { -0.0 } else { 0.0 };
            continue;
        }
        let mut exp = (f >> mant_bits) as i32;
        let mut mantissa = (f & ((1 << mant_bits) - 1)) as i32;
        if exp == (1 << exp_bits) - 1 {
            f = if signbit != 0 { 0x8000_0000 } else { 0 };
            f |= 0b1111_1111 << 23;
            f |= (mantissa << mant_shift) as u32;
            row_out[x] = f32::from_bits(f);
            continue;
        }
        mantissa <<= mant_shift;
        if exp == 0 && exp_bits < 8 {
            while mantissa & 0x80_0000 == 0 {
                mantissa <<= 1;
                exp -= 1;
            }
            exp += 1;
            mantissa &= 0x7f_ffff;
        }
        exp -= exp_bias;
        exp += 127;
        if exp < 0 {
            return Err(StatusCode::GenericError);
        }
        f = if signbit != 0 { 0x8000_0000 } else { 0 };
        f |= (exp << 23) as u32;
        f |= mantissa as u32;
        row_out[x] = f32::from_bits(f);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ModularStreamKind(pub i32);
impl ModularStreamKind {
    pub const GLOBAL_DATA: Self = Self(0);
    pub const VAR_DCT_DC: Self = Self(1);
    pub const MODULAR_DC: Self = Self(2);
    pub const AC_METADATA: Self = Self(3);
    pub const QUANT_TABLE: Self = Self(4);
    pub const MODULAR_AC: Self = Self(5);
}
#[derive(Clone, Copy, Debug)]
pub struct ModularStreamId {
    pub kind: ModularStreamKind,
    pub quant_table_id: usize,
    pub group_id: usize,
    pub pass_id: usize,
}
impl ModularStreamId {
    pub fn id(&self, frame_dim: &FrameDimensions) -> usize {
        match self.kind {
            ModularStreamKind::GLOBAL_DATA => 0,
            ModularStreamKind::VAR_DCT_DC => 1usize.wrapping_add(self.group_id),
            ModularStreamKind::MODULAR_DC => 1usize
                .wrapping_add(frame_dim.num_dc_groups)
                .wrapping_add(self.group_id),
            ModularStreamKind::AC_METADATA => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(2))
                .wrapping_add(self.group_id),
            ModularStreamKind::QUANT_TABLE => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(3))
                .wrapping_add(self.quant_table_id),
            ModularStreamKind::MODULAR_AC => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(3))
                .wrapping_add(NUM_QUANT_TABLES as usize)
                .wrapping_add(frame_dim.num_groups.wrapping_mul(self.pass_id))
                .wrapping_add(self.group_id),
            _ => 0,
        }
    }
    pub fn global() -> Self {
        Self {
            kind: ModularStreamKind::GLOBAL_DATA,
            quant_table_id: 0,
            group_id: 0,
            pass_id: 0,
        }
    }
    pub fn var_dct_dc(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::VAR_DCT_DC,
            group_id,
            ..Self::global()
        }
    }
    pub fn modular_dc(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::MODULAR_DC,
            group_id,
            ..Self::global()
        }
    }
    pub fn ac_metadata(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::AC_METADATA,
            group_id,
            ..Self::global()
        }
    }
    pub fn quant_table(quant_table_id: usize) -> StatusOr<Self> {
        if quant_table_id >= NUM_QUANT_TABLES as usize {
            return Err(StatusCode::GenericError);
        }
        Ok(Self {
            kind: ModularStreamKind::QUANT_TABLE,
            quant_table_id,
            ..Self::global()
        })
    }
    pub fn modular_ac(group_id: usize, pass_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::MODULAR_AC,
            group_id,
            pass_id,
            ..Self::global()
        }
    }
    pub fn num(frame_dim: &FrameDimensions, passes: usize) -> usize {
        Self::modular_ac(0, passes).id(frame_dim)
    }
}
