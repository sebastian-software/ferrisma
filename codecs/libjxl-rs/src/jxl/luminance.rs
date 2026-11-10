// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! SetIntensityTarget from `lib/jxl/luminance.{h,cc}`, with
//! kDefaultIntensityTarget from `lib/jxl/base/common.h`.

use super::image_metadata::ImageMetadata;

pub const DEFAULT_INTENSITY_TARGET: f32 = 255.0;

pub fn set_intensity_target(m: &mut ImageMetadata) {
    if m.color_encoding.tf().is_pq() {
        // Peak luminance of PQ as defined by SMPTE ST 2084:2014.
        m.set_intensity_target(10000.0);
    } else if m.color_encoding.tf().is_hlg() {
        // Nominal display peak luminance used as a reference by
        // Rec. ITU-R BT.2100-2.
        m.set_intensity_target(1000.0);
    } else {
        // SDR
        m.set_intensity_target(DEFAULT_INTENSITY_TARGET);
    }
}
