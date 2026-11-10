// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Profile generation from `lib/jxl/cms/jxl_cms_internal.h`, with the source's
//! default 3D tone-mapping setting. Owned byte vectors replace source pointers.
//! Header date, tag order, aliases, padding, and profile-ID rules remain fixed.

use super::super::base::status::{Status, StatusCode};
use super::super::color_encoding::{
    ColorSpace, JxlColorEncoding, RenderingIntent, TransferFunction,
};
pub use super::icc_description::{color_encoding_description, source_float_to_string};
pub use super::icc_math::{
    adapt_to_xyz_d50, cie_xyz_from_white_cie_xy, create_table_curve, primaries_to_xyz,
    primaries_to_xyz_d50, tone_map_pixel,
};
pub use super::icc_md5::icc_compute_md5;
use super::icc_tags::*;

pub fn maybe_create_profile(c: &JxlColorEncoding, icc: &mut Vec<u8>) -> Status {
    let mut header = Vec::new();
    let mut table = Vec::new();
    let mut tags = Vec::new();
    let tf = c.transfer_function;
    if c.color_space == ColorSpace::UNKNOWN || tf == TransferFunction::UNKNOWN {
        return Err(StatusCode::GenericError);
    }
    if !matches!(
        c.color_space,
        ColorSpace::RGB | ColorSpace::GRAY | ColorSpace::XYB
    ) {
        return Err(StatusCode::GenericError);
    }
    if c.color_space == ColorSpace::XYB && c.rendering_intent != RenderingIntent::PERCEPTUAL {
        return Err(StatusCode::GenericError);
    }
    create_icc_header(c, &mut header)?;
    let mut offsets = Vec::new();
    write_icc_uint32(0, table.len(), &mut table);
    let mut tag_offset = 0;
    let mut tag_size = 0;
    create_icc_mluc_tag(&color_encoding_description(c, false), &mut tags);
    finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
    add_to_icc_tag_table(b"desc", tag_offset, tag_size, &mut table, &mut offsets);
    create_icc_mluc_tag("CC0", &mut tags);
    finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
    add_to_icc_tag_table(b"cprt", tag_offset, tag_size, &mut table, &mut offsets);
    if c.color_space == ColorSpace::GRAY {
        let mut wtpt = [0.0; 3];
        cie_xyz_from_white_cie_xy(c.white_point_xy[0], c.white_point_xy[1], &mut wtpt)?;
        create_icc_xyz_tag(wtpt, &mut tags)?;
    } else {
        create_icc_xyz_tag([0.964203, 1.0, 0.824905], &mut tags)?;
    }
    finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
    add_to_icc_tag_table(b"wtpt", tag_offset, tag_size, &mut table, &mut offsets);
    if c.color_space != ColorSpace::GRAY {
        let mut chad = [[0.0; 3]; 3];
        if c.white_point_xy[1] == 0.0 {
            return Err(StatusCode::GenericError);
        }
        adapt_to_xyz_d50(
            c.white_point_xy[0] as f32,
            c.white_point_xy[1] as f32,
            &mut chad,
        )?;
        create_icc_chad_tag(&chad, &mut tags)?;
        finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
        add_to_icc_tag_table(b"chad", tag_offset, tag_size, &mut table, &mut offsets);
    }
    if c.color_space == ColorSpace::RGB {
        maybe_create_icc_cicp_tag(
            c,
            &mut tags,
            &mut tag_offset,
            &mut tag_size,
            &mut table,
            &mut offsets,
        );
        let mut m = [[0.0; 3]; 3];
        primaries_to_xyz_d50(
            c.primaries_red_xy[0] as f32,
            c.primaries_red_xy[1] as f32,
            c.primaries_green_xy[0] as f32,
            c.primaries_green_xy[1] as f32,
            c.primaries_blue_xy[0] as f32,
            c.primaries_blue_xy[1] as f32,
            c.white_point_xy[0] as f32,
            c.white_point_xy[1] as f32,
            &mut m,
        )?;
        for (col, tag) in [b"rXYZ", b"gXYZ", b"bXYZ"].iter().enumerate() {
            create_icc_xyz_tag([m[0][col], m[1][col], m[2][col]], &mut tags)?;
            finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
            add_to_icc_tag_table(tag, tag_offset, tag_size, &mut table, &mut offsets);
        }
    }
    if c.color_space == ColorSpace::XYB {
        create_icc_lut_atob_tag_for_xyb(&mut tags)?;
        finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
        add_to_icc_tag_table(b"A2B0", tag_offset, tag_size, &mut table, &mut offsets);
        create_icc_no_op_btoa_tag(&mut tags)?;
        finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
        add_to_icc_tag_table(b"B2A0", tag_offset, tag_size, &mut table, &mut offsets);
    } else if ENABLE_3D_TONE_MAPPING && can_tone_map(c) {
        create_icc_lut_atob_tag_for_hdr(c, &mut tags)?;
        finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
        add_to_icc_tag_table(b"A2B0", tag_offset, tag_size, &mut table, &mut offsets);
        create_icc_no_op_btoa_tag(&mut tags)?;
        finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
        add_to_icc_tag_table(b"B2A0", tag_offset, tag_size, &mut table, &mut offsets);
    } else {
        match tf {
            TransferFunction::GAMMA => {
                create_icc_curv_para_tag(&[(1.0 / c.gamma) as f32], 0, &mut tags)?
            }
            TransferFunction::HLG => create_icc_curv_curv_tag(
                &create_table_curve::<64>(false, can_tone_map(c)),
                &mut tags,
            ),
            TransferFunction::PQ => create_icc_curv_curv_tag(
                &create_table_curve::<64>(true, can_tone_map(c)),
                &mut tags,
            ),
            TransferFunction::SRGB => create_icc_curv_para_tag(
                &[
                    2.4,
                    (1.0f64 / 1.055) as f32,
                    (0.055f64 / 1.055) as f32,
                    (1.0f64 / 12.92) as f32,
                    0.04045,
                ],
                3,
                &mut tags,
            )?,
            TransferFunction::BT709 => create_icc_curv_para_tag(
                &[
                    (1.0f64 / 0.45) as f32,
                    (1.0f64 / 1.099) as f32,
                    (0.099f64 / 1.099) as f32,
                    (1.0f64 / 4.5) as f32,
                    0.081,
                ],
                3,
                &mut tags,
            )?,
            TransferFunction::LINEAR => {
                create_icc_curv_para_tag(&[1.0, 1.0, 0.0, 1.0, 0.0], 3, &mut tags)?
            }
            TransferFunction::DCI => {
                create_icc_curv_para_tag(&[2.6, 1.0, 0.0, 1.0, 0.0], 3, &mut tags)?
            }
            _ => return Err(StatusCode::GenericError),
        }
        finalize_icc_tag(&mut tags, &mut tag_offset, &mut tag_size);
        if c.color_space == ColorSpace::GRAY {
            add_to_icc_tag_table(b"kTRC", tag_offset, tag_size, &mut table, &mut offsets);
        } else {
            for tag in [b"rTRC", b"gTRC", b"bTRC"] {
                add_to_icc_tag_table(tag, tag_offset, tag_size, &mut table, &mut offsets);
            }
        }
    }
    write_icc_uint32(offsets.len() as u32, 0, &mut table);
    for (i, &offset) in offsets.iter().enumerate() {
        let pos = offset + header.len() + table.len();
        write_icc_uint32(pos as u32, 4 + 12 * i + 4, &mut table);
    }
    let size = header.len() + table.len() + tags.len();
    write_icc_uint32(size as u32, 0, &mut header);
    *icc = header;
    icc.extend_from_slice(&table);
    icc.extend_from_slice(&tags);
    let mut icc_sum = icc.clone();
    if icc_sum.len() >= 68 {
        icc_sum[44..48].fill(0);
        icc_sum[64..68].fill(0);
    }
    let checksum = icc_compute_md5(&icc_sum);
    icc[84..100].copy_from_slice(&checksum);
    Ok(())
}
