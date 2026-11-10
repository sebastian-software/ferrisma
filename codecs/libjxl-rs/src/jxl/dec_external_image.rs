// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Buffer-backed, null-pool specializations of dec_external_image.cc: original
//! FloatToU32/FloatToF16, UndoOrientation, row stores, ConvertChannelsToExternal,
//! and ConvertToExternal. Separate function types select integer or floating
//! output, a non-null output buffer, and no callback. Callback output and custom
//! thread runners remain separate boundaries.
use super::base::float::max_f32;
use super::base::hwy_scalar::{clamp, nearest_int};
use super::base::status::{Status, StatusCode};
use super::image::{Image3, Plane, Sample};
use super::image_bundle::ImageBundle;
use super::image_metadata::Orientation;
use super::image_ops::{copy_image3_to, fill_image};
use super::types::JxlEndianness;
pub const CONVERT_MAX_CHANNELS: usize = 4;

// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Derived from hwy/base.h F16FromF32 and scalar-inl.h DemoteTo. Highway's
// native-F16-operator branch is selected by the reference Clang/GCC builds.
// Stable Rust has no binary16 type. The finite rounding below uses the source's
// explicit bit algorithm; native casts retain/quiet NaN payloads instead of the
// fallback algorithm's canonical NaN. The default IEEE nearest-even environment
// and the selected native-cast mapping are recorded in ADR 0003.
fn native_f16_from_f32(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = bits >> 31;
    let biased_exp = (bits >> 23) & 255;
    let mantissa = bits & 0x7fffff;
    if biased_exp == 255 && mantissa != 0 {
        return ((sign << 15) | 0x7c00 | (mantissa >> 13) | 0x200) as u16;
    }
    let ulp_bit = (126 - biased_exp as i32).clamp(13, 24) as u32;
    let odd = ((mantissa | 0x800000) >> ulp_bit) & 1;
    let rounded = mantissa + odd + (1 << (ulp_bit - 1)) - 1;
    let carry = u32::from(rounded >= (1 << 23));
    let exp = biased_exp as i32 - 127 + carry as i32;
    if exp < -24 {
        return (sign << 15) as u16;
    }
    let overflowed = exp >= 16;
    let biased_exp16 = (exp + 15).clamp(0, 31) as u32;
    let sub_exp = (-14 - exp).max(0) as u32;
    let shifted = (rounded & 0x7fffff) >> (13 + sub_exp);
    let leading = if sub_exp == 0 { 0 } else { 1024 >> sub_exp };
    let mantissa16 = if overflowed { 0 } else { leading + shifted };
    ((sign << 15) | (biased_exp16 << 10) | mantissa16) as u16
}

pub fn float_to_f16(input: &[f32], output: &mut [u16], num: usize) {
    // Scalar Lanes(d)=1 gives no MSan tail. Each source load/store must fit.
    assert!(
        num <= input.len() && num <= output.len(),
        "source row-span precondition"
    );
    for x in 0..num {
        output[x] = native_f16_from_f32(input[x]);
    }
}
pub fn float_to_u32(
    input: &[f32],
    output: &mut [u32],
    num: usize,
    mul: f32,
    _bits_per_sample: usize,
) {
    // Scalar Lanes(d)=1 gives no MSan tail. Each source load/store must fit.
    assert!(
        num <= input.len() && num <= output.len(),
        "source row-span precondition"
    );
    for x in 0..num {
        output[x] = nearest_int(clamp(input[x], 0.0, 1.0) * mul) as u32;
    }
}
pub fn undo_orientation<T: Sample>(
    orientation: Orientation,
    image: &Plane<T>,
    out: &mut Plane<T>,
) -> Status {
    let w = image.xsize();
    let h = image.ysize();
    if orientation == Orientation::FLIP_HORIZONTAL {
        *out = Plane::create(w, h)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(y)[w - x - 1] = image.row(y)[x];
            }
        }
    } else if orientation == Orientation::ROTATE_180 {
        *out = Plane::create(w, h)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(h - y - 1)[w - x - 1] = image.row(y)[x];
            }
        }
    } else if orientation == Orientation::FLIP_VERTICAL {
        *out = Plane::create(w, h)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(h - y - 1)[x] = image.row(y)[x];
            }
        }
    } else if orientation == Orientation::TRANSPOSE {
        *out = Plane::create(h, w)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(x)[y] = image.row(y)[x];
            }
        }
    } else if orientation == Orientation::ROTATE_90 {
        *out = Plane::create(h, w)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(x)[h - y - 1] = image.row(y)[x];
            }
        }
    } else if orientation == Orientation::ANTI_TRANSPOSE {
        *out = Plane::create(h, w)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(w - x - 1)[h - y - 1] = image.row(y)[x];
            }
        }
    } else if orientation == Orientation::ROTATE_270 {
        *out = Plane::create(h, w)?;
        for y in 0..h as u32 {
            let y = y as usize;
            for x in 0..w {
                out.row_mut(w - x - 1)[y] = image.row(y)[x];
            }
        }
    }
    Ok(())
}
pub fn convert_uint_channels_to_buffer(
    input: &[Option<&Plane<f32>>],
    bits: usize,
    endianness: JxlEndianness,
    stride: usize,
    out: &mut [u8],
    out_size: usize,
    orientation: Orientation,
) -> Status {
    let n = input.len();
    if n == 0 || n > CONVERT_MAX_CHANNELS {
        return Err(StatusCode::GenericError);
    }
    if input[0].is_none() {
        return Err(StatusCode::GenericError);
    }
    if bits == 0 || bits > 16 {
        return Err(StatusCode::GenericError);
    }
    // The non-null buffer borrow and absent callback satisfy the source's XOR
    // guard. out_size describes a real span within that borrowed allocation.
    assert!(
        out_size <= out.len(),
        "source output allocation precondition"
    );
    let mut channels = input.to_vec();
    let bytes = (bits.wrapping_add(7)) / 8;
    let pixel_bytes = n * bytes;
    // Callback row owners/context are empty in the selected source branch.
    let mut temp: [Plane<f32>; CONVERT_MAX_CHANNELS] = std::array::from_fn(|_| Plane::default());
    if orientation != Orientation::IDENTITY {
        for (channel, temp) in channels.iter_mut().zip(temp.iter_mut()) {
            if let Some(ch) = *channel {
                undo_orientation(orientation, ch, temp)?;
                *channel = Some(&*temp);
            }
        }
    }
    let w = channels[0].unwrap().xsize();
    let h = channels[0].unwrap().ysize();
    let row_size = pixel_bytes.checked_mul(w).ok_or(StatusCode::GenericError)?;
    if stride < row_size {
        return Err(StatusCode::GenericError);
    }
    let total = h
        .wrapping_sub(1)
        .checked_mul(stride)
        .and_then(|v| v.checked_add(row_size))
        .ok_or(StatusCode::GenericError)?;
    if out_size < total {
        return Err(StatusCode::GenericError);
    }
    let little = endianness == JxlEndianness::LITTLE
        || (endianness == JxlEndianness::NATIVE && cfg!(target_endian = "little"));
    let mut ones = Plane::default();
    for ch in &channels {
        if ch.is_none() {
            ones = Plane::create(w, 1)?;
            fill_image(1.0, &mut ones);
            break;
        }
    }
    let mul = ((1u64 << bits) - 1) as f32;
    let mut cache = Plane::<u32>::default();
    // RunOnPool does not invoke its initializer when uint32_t(ysize)==0.
    if h as u32 != 0 {
        cache = Plane::create(w, n)?;
        // InitOutCallback is reached and returns true without side effects.
    }
    for y in 0..h as u32 {
        let y = y as usize;
        let row = &mut out[stride * y..stride * y + row_size];
        let inputs: [&[f32]; CONVERT_MAX_CHANNELS] = std::array::from_fn(|c| {
            if c >= n {
                &[]
            } else {
                channels[c].map_or_else(|| ones.row(0), |ch| ch.row(y))
            }
        });
        for (c, input) in inputs.iter().enumerate().take(n) {
            float_to_u32(input, cache.row_mut(c), w, mul, bits);
        }
        // StoreUintRow retains its original x-major/channel-minor order.
        for x in 0..w {
            for c in 0..n {
                let at = (n * x + c) * bytes;
                let value = cache.row(c)[x];
                if bits <= 8 {
                    row[at] = (value & 255) as u8;
                } else {
                    let value = value as u16;
                    let bytes = if little {
                        value.to_le_bytes()
                    } else {
                        value.to_be_bytes()
                    };
                    row[at..at + 2].copy_from_slice(&bytes);
                }
            }
        }
    }
    Ok(())
}
#[expect(
    clippy::too_many_arguments,
    reason = "Original ConvertToExternal boundary with fixed integer/buffer/null-pool branches"
)]
pub fn convert_uint_to_buffer(
    image: &ImageBundle<'_>,
    bits: usize,
    n: usize,
    endianness: JxlEndianness,
    stride: usize,
    out: &mut [u8],
    out_size: usize,
    orientation: Orientation,
    unpremul_alpha: bool,
) -> Status {
    let want_alpha = n == 2 || n == 4;
    let colors = if n <= 2 { 1 } else { 3 };
    let mut color = image.color();
    #[expect(
        unused_assignments,
        reason = "Preserve original empty owner construction before its conditional assignment"
    )]
    let mut unpremul = Image3::default();
    if image.alpha_is_premultiplied() && image.has_alpha() && unpremul_alpha {
        unpremul = Image3::create(color.xsize(), color.ysize())?;
        copy_image3_to(color, &mut unpremul)?;
        let alpha = image.alpha().expect("source requires alpha storage");
        for y in 0..unpremul.ysize() {
            // alpha.cc UnpremultiplyAlpha: std::max keeps its first operand on
            // unordered comparison, unlike Highway Min/Max in FloatToU32.
            for x in 0..unpremul.xsize() {
                let multiplier = 1.0 / max_f32(1.0f32 / (1u32 << 26) as f32, alpha.row(y)[x]);
                for c in 0..3 {
                    unpremul.plane_row_mut(c, y)[x] *= multiplier;
                }
            }
        }
        color = &unpremul;
    }
    let mut channels: [Option<&Plane<f32>>; CONVERT_MAX_CHANNELS] = [None; CONVERT_MAX_CHANNELS];
    let mut c = 0;
    while c < colors {
        channels[c] = Some(color.plane(c));
        c += 1;
    }
    if want_alpha {
        channels[c] = image.alpha();
        c += 1;
    }
    if n != c {
        return Err(StatusCode::GenericError);
    }
    convert_uint_channels_to_buffer(
        &channels[..n],
        bits,
        endianness,
        stride,
        out,
        out_size,
        orientation,
    )
}

pub fn convert_float_channels_to_buffer(
    input: &[Option<&Plane<f32>>],
    bits: usize,
    endianness: JxlEndianness,
    stride: usize,
    out: &mut [u8],
    out_size: usize,
    orientation: Orientation,
) -> Status {
    let n = input.len();
    if n == 0 || n > CONVERT_MAX_CHANNELS {
        return Err(StatusCode::GenericError);
    }
    if input[0].is_none() {
        return Err(StatusCode::GenericError);
    }
    if bits != 16 && bits != 32 {
        return Err(StatusCode::GenericError);
    }
    // The non-null buffer borrow and absent callback satisfy the source's XOR
    // guard. out_size describes a real span within that borrowed allocation.
    assert!(
        out_size <= out.len(),
        "source output allocation precondition"
    );
    let mut channels = input.to_vec();
    let bytes = (bits.wrapping_add(7)) / 8;
    let pixel_bytes = n * bytes;
    // Callback row owners/context are empty in the selected source branch.
    let mut temp: [Plane<f32>; CONVERT_MAX_CHANNELS] = std::array::from_fn(|_| Plane::default());
    if orientation != Orientation::IDENTITY {
        for (channel, temp) in channels.iter_mut().zip(temp.iter_mut()) {
            if let Some(ch) = *channel {
                undo_orientation(orientation, ch, temp)?;
                *channel = Some(&*temp);
            }
        }
    }
    let w = channels[0].unwrap().xsize();
    let h = channels[0].unwrap().ysize();
    let row_size = pixel_bytes.checked_mul(w).ok_or(StatusCode::GenericError)?;
    if stride < row_size {
        return Err(StatusCode::GenericError);
    }
    let total = h
        .wrapping_sub(1)
        .checked_mul(stride)
        .and_then(|v| v.checked_add(row_size))
        .ok_or(StatusCode::GenericError)?;
    if out_size < total {
        return Err(StatusCode::GenericError);
    }
    let little = endianness == JxlEndianness::LITTLE
        || (endianness == JxlEndianness::NATIVE && cfg!(target_endian = "little"));
    let mut ones = Plane::default();
    for ch in &channels {
        if ch.is_none() {
            ones = Plane::create(w, 1)?;
            fill_image(1.0, &mut ones);
            break;
        }
    }
    if bits == 16 {
        let swap_endianness = little != cfg!(target_endian = "little");
        let mut cache = Plane::<u16>::default();
        // RunOnPool skips initialization entirely when uint32_t(ysize)==0.
        if h as u32 != 0 {
            cache = Plane::create(w, n)?;
            // InitOutCallback is reached and returns true without side effects.
        }
        for y in 0..h as u32 {
            let y = y as usize;
            let inputs: [&[f32]; CONVERT_MAX_CHANNELS] = std::array::from_fn(|c| {
                if c >= n {
                    &[]
                } else {
                    channels[c].map_or_else(|| ones.row(0), |ch| ch.row(y))
                }
            });
            for (c, input) in inputs.iter().enumerate().take(n) {
                float_to_f16(input, cache.row_mut(c), w);
            }
            let row = &mut out[stride * y..stride * y + row_size];
            // The original writes native half words first, then swaps every
            // pair in a second pass. Byte stores need no typed-pointer cast.
            for x in 0..w {
                for c in 0..n {
                    let at = (n * x + c) * 2;
                    row[at..at + 2].copy_from_slice(&cache.row(c)[x].to_ne_bytes());
                }
            }
            if swap_endianness {
                for i in (0..w * n * 2).step_by(2) {
                    row.swap(i, i + 1);
                }
            }
        }
    } else {
        // The float32 initializer only calls the absent callback initializer.
        for y in 0..h as u32 {
            let y = y as usize;
            let row = &mut out[stride * y..stride * y + row_size];
            let inputs: [&[f32]; CONVERT_MAX_CHANNELS] = std::array::from_fn(|c| {
                if c >= n {
                    &[]
                } else {
                    channels[c].map_or_else(|| ones.row(0), |ch| ch.row(y))
                }
            });
            // StoreFloatRow retains x-major/channel-minor raw-bit copies.
            for x in 0..w {
                for (c, input) in inputs.iter().enumerate().take(n) {
                    let at = (n * x + c) * 4;
                    let bits = input[x].to_bits();
                    let bytes = if little {
                        bits.to_le_bytes()
                    } else {
                        bits.to_be_bytes()
                    };
                    row[at..at + 4].copy_from_slice(&bytes);
                }
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "Original ConvertToExternal boundary with fixed float/buffer/null-pool branches"
)]
pub fn convert_float_to_buffer(
    image: &ImageBundle<'_>,
    bits: usize,
    n: usize,
    endianness: JxlEndianness,
    stride: usize,
    out: &mut [u8],
    out_size: usize,
    orientation: Orientation,
    unpremul_alpha: bool,
) -> Status {
    let want_alpha = n == 2 || n == 4;
    let colors = if n <= 2 { 1 } else { 3 };
    let mut color = image.color();
    #[expect(
        unused_assignments,
        reason = "Preserve original empty owner construction before its conditional assignment"
    )]
    let mut unpremul = Image3::default();
    if image.alpha_is_premultiplied() && image.has_alpha() && unpremul_alpha {
        unpremul = Image3::create(color.xsize(), color.ysize())?;
        copy_image3_to(color, &mut unpremul)?;
        let alpha = image.alpha().expect("source requires alpha storage");
        for y in 0..unpremul.ysize() {
            // alpha.cc UnpremultiplyAlpha: std::max keeps its first operand on
            // unordered comparison, unlike Highway Min/Max in FloatToU32.
            for x in 0..unpremul.xsize() {
                let multiplier = 1.0 / max_f32(1.0f32 / (1u32 << 26) as f32, alpha.row(y)[x]);
                for c in 0..3 {
                    unpremul.plane_row_mut(c, y)[x] *= multiplier;
                }
            }
        }
        color = &unpremul;
    }
    let mut channels: [Option<&Plane<f32>>; CONVERT_MAX_CHANNELS] = [None; CONVERT_MAX_CHANNELS];
    let mut c = 0;
    while c < colors {
        channels[c] = Some(color.plane(c));
        c += 1;
    }
    if want_alpha {
        channels[c] = image.alpha();
        c += 1;
    }
    if n != c {
        return Err(StatusCode::GenericError);
    }
    convert_float_channels_to_buffer(
        &channels[..n],
        bits,
        endianness,
        stride,
        out,
        out_size,
        orientation,
    )
}
