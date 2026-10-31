// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Raw color definitions from `lib/include/jxl/color_encoding.h` and the
//! corresponding enum masks in `lib/jxl/color_encoding_internal.h`.
//! Raw wrappers preserve invalid values without invalid Rust discriminants.
//! This does not yet establish the public C ABI.

use super::field_encodings::FieldEnum;

macro_rules! color_enum {
    ($name:ident, $bits:expr, {$($variant:ident = $value:expr),* $(,)?}) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $name(pub u32);
        impl $name { $(pub const $variant: Self = Self($value);)* }
        impl FieldEnum for $name {
            const BITS: u64 = $bits;
            const NAME: &'static str = stringify!($name);
            fn from_raw(value: u32) -> Self { Self(value) }
            fn raw(self) -> u32 { self.0 }
        }
    };
}
color_enum!(ColorSpace, 0xf, { RGB=0, GRAY=1, XYB=2, UNKNOWN=3 });
color_enum!(WhitePoint, (1<<1)|(1<<2)|(1<<10)|(1<<11), { D65=1, CUSTOM=2, E=10, DCI=11 });
color_enum!(Primaries, (1<<1)|(1<<2)|(1<<9)|(1<<11), { SRGB=1, CUSTOM=2, BT2100=9, P3=11 });
color_enum!(TransferFunction, (1<<1)|(1<<2)|(1<<8)|(1<<13)|(1<<16)|(1<<17)|(1<<18), {
    BT709=1, UNKNOWN=2, LINEAR=8, SRGB=13, PQ=16, DCI=17, HLG=18,
    // External-only value, deliberately excluded from the internal enum mask.
    GAMMA=65535
});
color_enum!(RenderingIntent, 0xf, { PERCEPTUAL=0, RELATIVE=1, SATURATION=2, ABSOLUTE=3 });

#[derive(Clone, Debug, Default)]
pub struct JxlColorEncoding {
    pub color_space: ColorSpace,
    pub white_point: WhitePoint,
    pub white_point_xy: [f64; 2],
    pub primaries: Primaries,
    pub primaries_red_xy: [f64; 2],
    pub primaries_green_xy: [f64; 2],
    pub primaries_blue_xy: [f64; 2],
    pub transfer_function: TransferFunction,
    pub gamma: f64,
    pub rendering_intent: RenderingIntent,
}
