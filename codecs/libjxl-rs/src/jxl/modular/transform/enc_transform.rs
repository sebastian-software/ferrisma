// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Forward dispatch and min/max scan from modular/transform/enc_transform.cc.
//! The original null-pool configuration borrows the transform and image owners.

use super::enc_palette::fwd_palette;
use super::enc_rct::fwd_rct;
use super::enc_squeeze::fwd_squeeze;
use super::transform::{Transform, TransformId};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::modular::encoding::context_predict::weighted;
use crate::jxl::modular::modular_image::{Channel, Image};

pub fn transform_forward(
    t: &mut Transform,
    image: &mut Image,
    header: &weighted::Header,
) -> Status {
    match t.id {
        TransformId::RCT => fwd_rct(image, t.begin_c as usize, t.rct_type as usize),
        TransformId::SQUEEZE => fwd_squeeze(image, t.squeezes.clone()),
        TransformId::PALETTE => fwd_palette(
            image,
            t.begin_c,
            t.begin_c.wrapping_add(t.num_c).wrapping_sub(1),
            &mut t.nb_colors,
            &mut t.nb_deltas,
            t.ordered_palette,
            t.lossy_palette,
            &mut t.predictor,
            header,
        ),
        _ => Err(StatusCode::GenericError),
    }
}
pub fn compute_minmax(channel: &Channel, min: Option<&mut i32>, max: Option<&mut i32>) {
    let (mut real_min, mut real_max) = (i32::MAX, i32::MIN);
    for y in 0..channel.h {
        for &v in &channel.row(y)[..channel.w] {
            if v < real_min {
                real_min = v;
            }
            if v > real_max {
                real_max = v;
            }
        }
    }
    if let Some(min) = min {
        *min = real_min;
    }
    if let Some(max) = max {
        *max = real_max;
    }
}
