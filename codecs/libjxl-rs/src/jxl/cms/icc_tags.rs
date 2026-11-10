// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ICC tag writers from `lib/jxl/cms/jxl_cms_internal.h`.
//! Grow initialized byte vectors at the same write offsets as the source.

use super::super::base::float::max_f32;
use super::super::base::matrix_ops::Matrix3x3;
use super::super::base::status::{Status, StatusCode};
use super::super::color_encoding::{
    ColorSpace, JxlColorEncoding, Primaries, TransferFunction, WhitePoint,
};
use super::tone_mapping::Color;
use super::{icc_math, opsin_params};

pub fn write_icc_uint32(value: u32, pos: usize, icc: &mut Vec<u8>) {
    if icc.len() < pos + 4 {
        icc.resize(pos + 4, 0);
    }
    icc[pos..pos + 4].copy_from_slice(&value.to_be_bytes());
}
pub fn write_icc_uint16(value: u16, pos: usize, icc: &mut Vec<u8>) {
    if icc.len() < pos + 2 {
        icc.resize(pos + 2, 0);
    }
    icc[pos..pos + 2].copy_from_slice(&value.to_be_bytes());
}
pub fn write_icc_uint8(value: u8, pos: usize, icc: &mut Vec<u8>) {
    if icc.len() < pos + 1 {
        icc.resize(pos + 1, 0);
    }
    icc[pos] = value;
}
pub fn write_icc_tag(value: &[u8; 4], pos: usize, icc: &mut Vec<u8>) {
    if icc.len() < pos + 4 {
        icc.resize(pos + 4, 0);
    }
    icc[pos..pos + 4].copy_from_slice(value);
}
pub fn write_icc_s15_fixed16(value: f32, pos: usize, icc: &mut Vec<u8>) -> Status {
    if !(-32767.995..=32767.995).contains(&value) {
        return Err(StatusCode::GenericError);
    }
    let i = (value * 65536.0).round() as i32;
    write_icc_uint32(i as u32, pos, icc);
    Ok(())
}
pub fn can_tone_map(c: &JxlColorEncoding) -> bool {
    c.color_space == ColorSpace::RGB
        && (c.transfer_function == TransferFunction::PQ
            || c.transfer_function == TransferFunction::HLG)
        && ((c.primaries == Primaries::P3
            && (c.white_point == WhitePoint::D65 || c.white_point == WhitePoint::DCI))
            || (c.primaries != Primaries::CUSTOM && c.white_point == WhitePoint::D65))
}
// The pinned source defaults JXL_ENABLE_3D_ICC_TONEMAPPING to 1.
pub const ENABLE_3D_TONE_MAPPING: bool = true;
pub fn create_icc_header(c: &JxlColorEncoding, header: &mut Vec<u8>) -> Status {
    header.resize(128, 0);
    write_icc_uint32(0, 0, header);
    write_icc_tag(b"jxl ", 4, header);
    write_icc_uint32(0x04400000, 8, header);
    write_icc_tag(
        if c.color_space == ColorSpace::XYB {
            b"scnr"
        } else {
            b"mntr"
        },
        12,
        header,
    );
    write_icc_tag(
        if c.color_space == ColorSpace::GRAY {
            b"GRAY"
        } else {
            b"RGB "
        },
        16,
        header,
    );
    write_icc_tag(
        if ENABLE_3D_TONE_MAPPING && can_tone_map(c) {
            b"Lab "
        } else {
            b"XYZ "
        },
        20,
        header,
    );
    // This frozen date is the source value, not the current wall clock.
    for (pos, value) in [(24, 2019), (26, 12), (28, 1), (30, 0), (32, 0), (34, 0)] {
        write_icc_uint16(value, pos, header);
    }
    write_icc_tag(b"acsp", 36, header);
    write_icc_tag(b"APPL", 40, header);
    for pos in [44, 48, 52, 56, 60] {
        write_icc_uint32(0, pos, header);
    }
    write_icc_uint32(c.rendering_intent.0, 64, header);
    write_icc_uint32(0x0000f6d6, 68, header);
    write_icc_uint32(0x00010000, 72, header);
    write_icc_uint32(0x0000d32d, 76, header);
    write_icc_tag(b"jxl ", 80, header);
    Ok(())
}
pub fn add_to_icc_tag_table(
    tag: &[u8; 4],
    offset: usize,
    size: usize,
    table: &mut Vec<u8>,
    offsets: &mut Vec<usize>,
) {
    write_icc_tag(tag, table.len(), table);
    write_icc_uint32(0, table.len(), table);
    offsets.push(offset);
    write_icc_uint32(size as u32, table.len(), table);
}
pub fn finalize_icc_tag(tags: &mut Vec<u8>, offset: &mut usize, size: &mut usize) {
    while tags.len() & 3 != 0 {
        tags.push(0);
    }
    *offset += *size;
    *size = tags.len() - *offset;
}
pub fn create_icc_mluc_tag(text: &str, tags: &mut Vec<u8>) {
    write_icc_tag(b"mluc", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    write_icc_uint32(1, tags.len(), tags);
    write_icc_uint32(12, tags.len(), tags);
    write_icc_tag(b"enUS", tags.len(), tags);
    write_icc_uint32((text.len() * 2) as u32, tags.len(), tags);
    write_icc_uint32(28, tags.len(), tags);
    for &c in text.as_bytes() {
        tags.push(0);
        tags.push(c);
    }
}
pub fn create_icc_xyz_tag(xyz: Color, tags: &mut Vec<u8>) -> Status {
    write_icc_tag(b"XYZ ", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    for value in xyz {
        write_icc_s15_fixed16(value, tags.len(), tags)?;
    }
    Ok(())
}
pub fn create_icc_chad_tag(chad: &Matrix3x3, tags: &mut Vec<u8>) -> Status {
    write_icc_tag(b"sf32", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    for row in chad {
        for &value in row {
            write_icc_s15_fixed16(value, tags.len(), tags)?;
        }
    }
    Ok(())
}
pub fn maybe_create_icc_cicp_tag(
    c: &JxlColorEncoding,
    tags: &mut Vec<u8>,
    offset: &mut usize,
    size: &mut usize,
    table: &mut Vec<u8>,
    offsets: &mut Vec<usize>,
) {
    if c.color_space != ColorSpace::RGB {
        return;
    }
    let primaries = if c.primaries == Primaries::P3 {
        if c.white_point == WhitePoint::D65 {
            12
        } else if c.white_point == WhitePoint::DCI {
            11
        } else {
            return;
        }
    } else if c.primaries != Primaries::CUSTOM && c.white_point == WhitePoint::D65 {
        c.primaries.0 as u8
    } else {
        return;
    };
    if c.transfer_function == TransferFunction::UNKNOWN
        || c.transfer_function == TransferFunction::GAMMA
    {
        return;
    }
    write_icc_tag(b"cicp", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    write_icc_uint8(primaries, tags.len(), tags);
    write_icc_uint8(c.transfer_function.0 as u8, tags.len(), tags);
    write_icc_uint8(0, tags.len(), tags);
    write_icc_uint8(1, tags.len(), tags);
    finalize_icc_tag(tags, offset, size);
    add_to_icc_tag_table(b"cicp", *offset, *size, table, offsets);
}
pub fn create_icc_curv_curv_tag(curve: &[u16], tags: &mut Vec<u8>) {
    let pos = tags.len();
    tags.resize(pos + 12 + curve.len() * 2, 0);
    write_icc_tag(b"curv", pos, tags);
    write_icc_uint32(0, pos + 4, tags);
    write_icc_uint32(curve.len() as u32, pos + 8, tags);
    for (i, &value) in curve.iter().enumerate() {
        write_icc_uint16(value, pos + 12 + i * 2, tags);
    }
}
pub fn create_icc_curv_para_tag(params: &[f32], curve_type: usize, tags: &mut Vec<u8>) -> Status {
    write_icc_tag(b"para", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    write_icc_uint16(curve_type as u16, tags.len(), tags);
    write_icc_uint16(0, tags.len(), tags);
    for &param in params {
        write_icc_s15_fixed16(param, tags.len(), tags)?;
    }
    Ok(())
}
pub fn create_icc_lut_atob_tag_for_xyb(tags: &mut Vec<u8>) -> Status {
    write_icc_tag(b"mAB ", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    write_icc_uint8(3, tags.len(), tags);
    write_icc_uint8(3, tags.len(), tags);
    write_icc_uint16(0, tags.len(), tags);
    for offset in [32, 244, 148, 80, 32] {
        write_icc_uint32(offset, tags.len(), tags);
    }
    for _ in 0..3 {
        create_icc_curv_para_tag(&[1.0], 0, tags)?;
    }
    for i in 0..16 {
        write_icc_uint8(if i < 3 { 2 } else { 0 }, tags.len(), tags);
    }
    write_icc_uint8(2, tags.len(), tags);
    write_icc_uint8(0, tags.len(), tags);
    write_icc_uint16(0, tags.len(), tags);
    let cube = opsin_params::unscaled_a2b_cube();
    for plane in cube {
        for row in plane {
            for point in row {
                for out in point {
                    let val = (65535.0 * out).round() as i32;
                    debug_assert!((0..=65535).contains(&val));
                    write_icc_uint16(val as u16, tags.len(), tags);
                }
            }
        }
    }
    for i in 0..3 {
        let b =
            -opsin_params::XYB_OFFSET[i] - opsin_params::NEG_OPSIN_ABSORBANCE_BIAS_RGB[i].cbrt();
        create_icc_curv_para_tag(
            &[
                3.0,
                1.0 / opsin_params::XYB_SCALE[i],
                b,
                0.0,
                max_f32(0.0, -b * opsin_params::XYB_SCALE[i]),
            ],
            3,
            tags,
        )?;
    }
    let matrix: [f64; 9] = [
        1.5170095, -1.1065225, 0.071623, -0.050022, 0.5683655, -0.018344, -1.387676, 1.1145555,
        0.6857255,
    ];
    for value in matrix {
        write_icc_s15_fixed16(value as f32, tags.len(), tags)?;
    }
    for i in 0..3 {
        let mut intercept = 0.0f32;
        for j in 0..3 {
            // The compound assignment promotes the float accumulator to double
            // for each source addition, then narrows it back to float.
            intercept = super::super::base::float::mul_add_f64(
                matrix[i * 3 + j],
                f64::from(opsin_params::NEG_OPSIN_ABSORBANCE_BIAS_RGB[j]),
                f64::from(intercept),
            ) as f32;
        }
        write_icc_s15_fixed16(intercept, tags.len(), tags)?;
    }
    Ok(())
}
pub fn create_icc_lut_atob_tag_for_hdr(c: &JxlColorEncoding, tags: &mut Vec<u8>) -> Status {
    const DIM: usize = 9;
    write_icc_tag(b"mft1", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    write_icc_uint8(3, tags.len(), tags);
    write_icc_uint8(3, tags.len(), tags);
    write_icc_uint8(DIM as u8, tags.len(), tags);
    write_icc_uint8(0, tags.len(), tags);
    for i in 0..3 {
        for j in 0..3 {
            write_icc_s15_fixed16(if i == j { 1.0 } else { 0.0 }, tags.len(), tags)?;
        }
    }
    for _ in 0..3 {
        for i in 0..256 {
            write_icc_uint8(i as u8, tags.len(), tags);
        }
    }
    for ix in 0..DIM {
        for iy in 0..DIM {
            for ib in 0..DIM {
                let input = [
                    ix as f32 * (1.0 / (DIM - 1) as f32),
                    iy as f32 * (1.0 / (DIM - 1) as f32),
                    ib as f32 * (1.0 / (DIM - 1) as f32),
                ];
                let mut out = [0; 3];
                icc_math::tone_map_pixel(c, input, &mut out)?;
                for value in out {
                    write_icc_uint8(value, tags.len(), tags);
                }
            }
        }
    }
    for _ in 0..3 {
        for i in 0..256 {
            write_icc_uint8(i as u8, tags.len(), tags);
        }
    }
    Ok(())
}
pub fn create_icc_no_op_btoa_tag(tags: &mut Vec<u8>) -> Status {
    write_icc_tag(b"mBA ", tags.len(), tags);
    write_icc_uint32(0, tags.len(), tags);
    write_icc_uint8(3, tags.len(), tags);
    write_icc_uint8(3, tags.len(), tags);
    write_icc_uint16(0, tags.len(), tags);
    for offset in [32, 0, 0, 0, 0] {
        write_icc_uint32(offset, tags.len(), tags);
    }
    for _ in 0..3 {
        create_icc_curv_para_tag(&[1.0], 0, tags)?;
    }
    Ok(())
}
