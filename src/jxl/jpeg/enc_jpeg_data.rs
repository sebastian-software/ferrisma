// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Frame subsampling/color selection from lib/jxl/jpeg/enc_jpeg_data.cc.
//! ICC handling, JPEG serialization, and the remaining encoder are unported.

use super::jpeg_data::JpegData;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::frame_header::{ColorTransform, YCbCrChromaSubsampling};

pub fn set_chroma_subsampling_from_jpeg_data(
    jpg: &JpegData,
    cs: &mut YCbCrChromaSubsampling,
) -> Status {
    let n = jpg.components.len();
    if n != 1 && n != 3 {
        return Err(StatusCode::GenericError);
    }
    let h = std::array::from_fn(|i| jpg.components[if n == 1 { 0 } else { i }].h_samp_factor as u8);
    let v = std::array::from_fn(|i| jpg.components[if n == 1 { 0 } else { i }].v_samp_factor as u8);
    cs.set(&h, &v)
}
pub fn set_color_transform_from_jpeg_data(
    jpg: &JpegData,
    transform: &mut ColorTransform,
) -> Status {
    let n = jpg.components.len();
    if n != 1 && n != 3 {
        return Err(StatusCode::Unsupported);
    }
    let mut rgb = false;
    if !jpg.marker_order.contains(&0xE0) {
        let mut app_markers = 0;
        let mut i = 0;
        while i < jpg.marker_order.len() {
            if jpg.marker_order[i] & 0xF0 == 0xE0 {
                if app_markers >= jpg.app_data.len() {
                    return Err(StatusCode::GenericError);
                }
                if jpg.marker_order[i] == 0xEE {
                    let data = &jpg.app_data[app_markers];
                    if data.len() == 15 && data[3..8] == *b"Adobe" {
                        rgb = data[14] == 0;
                        break;
                    }
                }
                app_markers += 1;
            }
            i += 1;
        }
        if i == jpg.marker_order.len() {
            rgb = n == 3
                && jpg.components[0].id == u32::from(b'R')
                && jpg.components[1].id == u32::from(b'G')
                && jpg.components[2].id == u32::from(b'B');
        }
    }
    *transform = if !rgb || n == 1 {
        ColorTransform::YCBCR
    } else {
        ColorTransform::NONE
    };
    Ok(())
}
