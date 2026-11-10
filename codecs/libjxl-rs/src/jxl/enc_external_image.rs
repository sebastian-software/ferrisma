// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Plane input conversion from lib/jxl/enc_external_image.cc, with the original
//! null-pool row schedule. ImageBundle/BufferToImage wrappers remain unported.
//! Byte slices retain the source buffer validity precondition; the explicit
//! size argument preserves the original size=0 validation behavior.

use super::base::byte_order::is_little_endian;
use super::base::float::load_float_row;
use super::base::status::{Status, StatusCode};
use super::image::Plane;
use super::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

fn jxl_data_type_bytes(data_type: JxlDataType) -> usize {
    match data_type {
        JxlDataType::UINT8 => 1,
        JxlDataType::UINT16 | JxlDataType::FLOAT16 => 2,
        JxlDataType::FLOAT => 4,
        _ => 0,
    }
}

// Retain the original conversion boundary's separate dimensions/stride/format.
#[allow(clippy::too_many_arguments)]
pub fn convert_from_external_plane_no_size_check(
    data: &[u8],
    xsize: usize,
    ysize: usize,
    stride: usize,
    bits_per_sample: usize,
    format: JxlPixelFormat,
    c: usize,
    channel: &mut Plane<f32>,
) -> Status {
    if format.data_type == JxlDataType::UINT8 {
        if bits_per_sample == 0 || bits_per_sample > 8 {
            return Err(StatusCode::GenericError);
        }
    } else if format.data_type == JxlDataType::UINT16 {
        if bits_per_sample <= 8 || bits_per_sample > 16 {
            return Err(StatusCode::GenericError);
        }
    } else if format.data_type != JxlDataType::FLOAT16 && format.data_type != JxlDataType::FLOAT {
        return Err(StatusCode::GenericError);
    }
    if channel.xsize() != xsize || channel.ysize() != ysize {
        return Err(StatusCode::GenericError);
    }
    let bytes_per_channel = jxl_data_type_bytes(format.data_type);
    let bytes_per_pixel = (format.num_channels as usize).wrapping_mul(bytes_per_channel);
    let bytes_per_row = xsize
        .checked_mul(bytes_per_pixel)
        .ok_or(StatusCode::GenericError)?;
    if bytes_per_row > stride {
        return Err(StatusCode::GenericError);
    }
    let pixel_offset = c.wrapping_mul(bytes_per_channel);
    assert!(bits_per_sample < 64, "source scale shift must be below 64");
    let scale = 1.0f32 / ((1u64 << bits_per_sample) - 1) as f32;
    let little_endian = format.endianness == JxlEndianness::LITTLE
        || (format.endianness == JxlEndianness::NATIVE && is_little_endian());
    for task in 0..ysize as u32 {
        let y = task as usize;
        let offset = y.wrapping_mul(stride).wrapping_add(pixel_offset);
        let row_out = channel.row_mut(y);
        load_float_row(
            &data[offset..],
            xsize,
            bytes_per_pixel,
            format.data_type,
            little_endian,
            scale,
            |index, value| row_out[index] = value,
        )
        .map_err(|_| StatusCode::GenericError)?;
    }
    Ok(())
}

// The source size argument is independent of the borrowed allocation extent.
#[allow(clippy::too_many_arguments)]
pub fn convert_from_external(
    data: &[u8],
    size: usize,
    xsize: usize,
    ysize: usize,
    bits_per_sample: usize,
    format: JxlPixelFormat,
    c: usize,
    channel: &mut Plane<f32>,
) -> Status {
    let bytes_per_channel = jxl_data_type_bytes(format.data_type);
    let bytes_per_pixel = (format.num_channels as usize)
        .checked_mul(bytes_per_channel)
        .ok_or(StatusCode::GenericError)?;
    let last_row_size = xsize
        .checked_mul(bytes_per_pixel)
        .ok_or(StatusCode::GenericError)?;
    let mut row_size = last_row_size;
    // SafeRoundUpTo accepts arbitrary alignments, with 0/1 requiring no padding.
    if format.align >= 2 {
        let reminder = row_size % format.align;
        if reminder != 0 {
            row_size = row_size
                .checked_add(format.align - reminder)
                .ok_or(StatusCode::GenericError)?;
        }
    }
    if xsize == 0 || ysize == 0 {
        return Err(StatusCode::GenericError);
    }
    let bytes_to_read = row_size
        .checked_mul(ysize - 1)
        .and_then(|v| v.checked_add(last_row_size))
        .ok_or(StatusCode::GenericError)?;
    if size > 0 && size < bytes_to_read {
        return Err(StatusCode::GenericError);
    }
    if size > row_size.wrapping_mul(ysize) {
        return Err(StatusCode::GenericError);
    }
    convert_from_external_plane_no_size_check(
        data,
        xsize,
        ysize,
        row_size,
        bits_per_sample,
        format,
        c,
        channel,
    )
}
