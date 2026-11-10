// libwebp v1.6.0, src/enc/config_enc.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Coding tools configuration
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/config_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Version checks.** `WebPConfigInitInternal` and
//!    `WebPPictureInitInternal` keep their version parameter and its ABI check.
//!    The wrappers pass the crate's version, so the check always succeeds
//!    through them.
//! 2. **`use_delta_palette`.** `WebPConfigInit` does not write this reserved
//!    field in C, so it keeps the caller's value. The Rust translation also
//!    leaves it alone, and the test (`tests/wp9_config.rs`) compares it as
//!    unchanged.

use crate::webp::encode::{WEBP_ENCODER_ABI_VERSION, WebPConfig, WebPImageHint, WebPPreset};
use crate::webp::types::webp_abi_is_incompatible;

// upstream: src/enc/config_enc.c WebPConfigInitInternal
/// Initializes `config` with the defaults, applies `preset` and validates the
/// result (C: `WebPConfigInitInternal`). A version mismatch returns `false` and
/// leaves `config` untouched.
pub fn webp_config_init_internal(
    config: &mut WebPConfig,
    preset: WebPPreset,
    quality: f32,
    version: i32,
) -> bool {
    if webp_abi_is_incompatible(version, WEBP_ENCODER_ABI_VERSION) {
        return false; // caller/system version mismatch!
    }

    config.quality = quality;
    config.target_size = 0;
    config.target_psnr = 0.0;
    config.method = 4;
    config.sns_strength = 50;
    config.filter_strength = 60; // mid-filtering
    config.filter_sharpness = 0;
    config.filter_type = 1; // default: strong (so U/V is filtered too)
    config.partitions = 0;
    config.segments = 4;
    config.pass = 1;
    config.qmin = 0;
    config.qmax = 100;
    config.show_compressed = 0;
    config.preprocessing = 0;
    config.autofilter = 0;
    config.partition_limit = 0;
    config.alpha_compression = 1;
    config.alpha_filtering = 1;
    config.alpha_quality = 100;
    config.lossless = 0;
    config.exact = 0;
    config.image_hint = WebPImageHint::WEBP_HINT_DEFAULT;
    config.emulate_jpeg_size = 0;
    config.thread_level = 0;
    config.low_memory = 0;
    config.near_lossless = 100;
    config.use_sharp_yuv = 0;

    // TODO(skal): tune.
    match preset {
        WebPPreset::WEBP_PRESET_PICTURE => {
            config.sns_strength = 80;
            config.filter_sharpness = 4;
            config.filter_strength = 35;
            config.preprocessing &= !2; // no dithering
        }
        WebPPreset::WEBP_PRESET_PHOTO => {
            config.sns_strength = 80;
            config.filter_sharpness = 3;
            config.filter_strength = 30;
            config.preprocessing |= 2;
        }
        WebPPreset::WEBP_PRESET_DRAWING => {
            config.sns_strength = 25;
            config.filter_sharpness = 6;
            config.filter_strength = 10;
        }
        WebPPreset::WEBP_PRESET_ICON => {
            config.sns_strength = 0;
            config.filter_strength = 0; // disable filtering to retain sharpness
            config.preprocessing &= !2; // no dithering
        }
        WebPPreset::WEBP_PRESET_TEXT => {
            config.sns_strength = 0;
            config.filter_strength = 0; // disable filtering to retain sharpness
            config.preprocessing &= !2; // no dithering
            config.segments = 2;
        }
        // WEBP_PRESET_DEFAULT and every other value: no change
        _ => {}
    }
    webp_validate_config(config)
}

// upstream: src/enc/config_enc.c WebPValidateConfig
/// Reports whether every field of `config` is within its documented range (C:
/// `WebPValidateConfig`). The comparisons are written as in C, so a NaN `quality`
/// or `target_psnr` passes the float tests, as it does in C.
#[must_use]
pub fn webp_validate_config(config: &WebPConfig) -> bool {
    if config.quality < 0.0 || config.quality > 100.0 {
        return false;
    }
    if config.target_size < 0 {
        return false;
    }
    if config.target_psnr < 0.0 {
        return false;
    }
    if config.method < 0 || config.method > 6 {
        return false;
    }
    if config.segments < 1 || config.segments > 4 {
        return false;
    }
    if config.sns_strength < 0 || config.sns_strength > 100 {
        return false;
    }
    if config.filter_strength < 0 || config.filter_strength > 100 {
        return false;
    }
    if config.filter_sharpness < 0 || config.filter_sharpness > 7 {
        return false;
    }
    if config.filter_type < 0 || config.filter_type > 1 {
        return false;
    }
    if config.autofilter < 0 || config.autofilter > 1 {
        return false;
    }
    if config.pass < 1 || config.pass > 10 {
        return false;
    }
    if config.qmin < 0 || config.qmax > 100 || config.qmin > config.qmax {
        return false;
    }
    if config.show_compressed < 0 || config.show_compressed > 1 {
        return false;
    }
    if config.preprocessing < 0 || config.preprocessing > 7 {
        return false;
    }
    if config.partitions < 0 || config.partitions > 3 {
        return false;
    }
    if config.partition_limit < 0 || config.partition_limit > 100 {
        return false;
    }
    if config.alpha_compression < 0 {
        return false;
    }
    if config.alpha_filtering < 0 {
        return false;
    }
    if config.alpha_quality < 0 || config.alpha_quality > 100 {
        return false;
    }
    if config.lossless < 0 || config.lossless > 1 {
        return false;
    }
    if config.near_lossless < 0 || config.near_lossless > 100 {
        return false;
    }
    if config.image_hint >= WebPImageHint::WEBP_HINT_LAST {
        return false;
    }
    if config.emulate_jpeg_size < 0 || config.emulate_jpeg_size > 1 {
        return false;
    }
    if config.thread_level < 0 || config.thread_level > 1 {
        return false;
    }
    if config.low_memory < 0 || config.low_memory > 1 {
        return false;
    }
    if config.exact < 0 || config.exact > 1 {
        return false;
    }
    if config.use_sharp_yuv < 0 || config.use_sharp_yuv > 1 {
        return false;
    }

    true
}

//------------------------------------------------------------------------------

// upstream: src/enc/config_enc.c MAX_LEVEL
const MAX_LEVEL: i32 = 9;

// Mapping between -z level and -m / -q parameter settings.
// upstream: src/enc/config_enc.c kLosslessPresets
// Each entry is `(method, quality)`, the two `uint8_t` fields of the C struct.
const LOSSLESS_PRESETS: [(u8, u8); (MAX_LEVEL + 1) as usize] = [
    (0, 0),
    (1, 20),
    (2, 25),
    (3, 30),
    (3, 50),
    (4, 50),
    (4, 75),
    (4, 90),
    (5, 90),
    (6, 100),
];

// upstream: src/enc/config_enc.c WebPConfigLosslessPreset
/// Sets the lossless flag and the method and quality of compression level `level`
/// (C: `WebPConfigLosslessPreset`). A level outside `0..=9` returns `false` and
/// leaves `config` untouched.
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does"
)]
pub fn webp_config_lossless_preset(config: &mut WebPConfig, level: i32) -> bool {
    if !(0..=MAX_LEVEL).contains(&level) {
        return false;
    }
    config.lossless = 1;
    let (method, quality) = LOSSLESS_PRESETS[level as usize];
    config.method = i32::from(method);
    config.quality = f32::from(quality);
    true
}
