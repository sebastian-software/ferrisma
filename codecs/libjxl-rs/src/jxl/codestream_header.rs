// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Public metadata structures from `lib/include/jxl/codestream_header.h`.
//! C enums use the existing transparent-u32 public-value convention, so any
//! value the source can store remains representable. `JXL_BOOL` is `i32`.
//! These are C-layout Rust values; the C ABI entry points remain separate.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlOrientation(pub u32);
impl JxlOrientation {
    pub const IDENTITY: Self = Self(1);
    pub const FLIP_HORIZONTAL: Self = Self(2);
    pub const ROTATE_180: Self = Self(3);
    pub const FLIP_VERTICAL: Self = Self(4);
    pub const TRANSPOSE: Self = Self(5);
    pub const ROTATE_90_CW: Self = Self(6);
    pub const ANTI_TRANSPOSE: Self = Self(7);
    pub const ROTATE_90_CCW: Self = Self(8);
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlExtraChannelType(pub u32);
impl JxlExtraChannelType {
    pub const ALPHA: Self = Self(0);
    pub const DEPTH: Self = Self(1);
    pub const SPOT_COLOR: Self = Self(2);
    pub const SELECTION_MASK: Self = Self(3);
    pub const BLACK: Self = Self(4);
    pub const CFA: Self = Self(5);
    pub const THERMAL: Self = Self(6);
    pub const RESERVED0: Self = Self(7);
    pub const RESERVED1: Self = Self(8);
    pub const RESERVED2: Self = Self(9);
    pub const RESERVED3: Self = Self(10);
    pub const RESERVED4: Self = Self(11);
    pub const RESERVED5: Self = Self(12);
    pub const RESERVED6: Self = Self(13);
    pub const RESERVED7: Self = Self(14);
    pub const UNKNOWN: Self = Self(15);
    pub const OPTIONAL: Self = Self(16);
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct JxlPreviewHeader {
    pub xsize: u32,
    pub ysize: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct JxlAnimationHeader {
    pub tps_numerator: u32,
    pub tps_denominator: u32,
    pub num_loops: u32,
    pub have_timecodes: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct JxlBasicInfo {
    pub have_container: i32,
    pub xsize: u32,
    pub ysize: u32,
    pub bits_per_sample: u32,
    pub exponent_bits_per_sample: u32,
    pub intensity_target: f32,
    pub min_nits: f32,
    pub relative_to_max_display: i32,
    pub linear_below: f32,
    pub uses_original_profile: i32,
    pub have_preview: i32,
    pub have_animation: i32,
    pub orientation: JxlOrientation,
    pub num_color_channels: u32,
    pub num_extra_channels: u32,
    pub alpha_bits: u32,
    pub alpha_exponent_bits: u32,
    pub alpha_premultiplied: i32,
    pub preview: JxlPreviewHeader,
    pub animation: JxlAnimationHeader,
    pub intrinsic_xsize: u32,
    pub intrinsic_ysize: u32,
    pub padding: [u8; 100],
}
const _: () = assert!(std::mem::size_of::<JxlBasicInfo>() == 204);
impl JxlBasicInfo {
    /// The all-zero object the source produces with memset.
    pub const fn zeroed() -> Self {
        Self {
            have_container: 0,
            xsize: 0,
            ysize: 0,
            bits_per_sample: 0,
            exponent_bits_per_sample: 0,
            intensity_target: 0.0,
            min_nits: 0.0,
            relative_to_max_display: 0,
            linear_below: 0.0,
            uses_original_profile: 0,
            have_preview: 0,
            have_animation: 0,
            orientation: JxlOrientation(0),
            num_color_channels: 0,
            num_extra_channels: 0,
            alpha_bits: 0,
            alpha_exponent_bits: 0,
            alpha_premultiplied: 0,
            preview: JxlPreviewHeader { xsize: 0, ysize: 0 },
            animation: JxlAnimationHeader {
                tps_numerator: 0,
                tps_denominator: 0,
                num_loops: 0,
                have_timecodes: 0,
            },
            intrinsic_xsize: 0,
            intrinsic_ysize: 0,
            padding: [0; 100],
        }
    }
}
impl Default for JxlBasicInfo {
    fn default() -> Self {
        Self::zeroed()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct JxlExtraChannelInfo {
    pub channel_type: JxlExtraChannelType,
    pub bits_per_sample: u32,
    pub exponent_bits_per_sample: u32,
    pub dim_shift: u32,
    pub name_length: u32,
    pub alpha_premultiplied: i32,
    pub spot_color: [f32; 4],
    pub cfa_channel: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct JxlHeaderExtensions {
    pub extensions: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlBlendMode(pub u32);
impl JxlBlendMode {
    pub const REPLACE: Self = Self(0);
    pub const ADD: Self = Self(1);
    pub const BLEND: Self = Self(2);
    pub const MULADD: Self = Self(3);
    pub const MUL: Self = Self(4);
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct JxlBlendInfo {
    pub blendmode: JxlBlendMode,
    pub source: u32,
    pub alpha: u32,
    pub clamp: i32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct JxlLayerInfo {
    pub have_crop: i32,
    pub crop_x0: i32,
    pub crop_y0: i32,
    pub xsize: u32,
    pub ysize: u32,
    pub blend_info: JxlBlendInfo,
    pub save_as_reference: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct JxlFrameHeader {
    pub duration: u32,
    pub timecode: u32,
    pub name_length: u32,
    pub is_last: i32,
    pub layer_info: JxlLayerInfo,
}
