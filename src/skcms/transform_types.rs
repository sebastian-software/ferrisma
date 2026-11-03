// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Format and stage carriers from pinned src/skcms_public.h and skcms_Transform.h.

/// Original numeric format carrier. Unnamed source-defined enum values retain
/// the source's Release default rejection instead of creating an invalid Rust enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PixelFormat(pub i32);
#[expect(non_upper_case_globals, reason = "Preserve source format names.")]
impl PixelFormat {
    pub const A_8: Self = Self(0);
    pub const A_8_: Self = Self(1);
    pub const G_8: Self = Self(2);
    pub const G_8_: Self = Self(3);
    pub const GA_88: Self = Self(4);
    pub const GA_88_: Self = Self(5);
    pub const RGB_565: Self = Self(6);
    pub const BGR_565: Self = Self(7);
    pub const ABGR_4444: Self = Self(8);
    pub const ARGB_4444: Self = Self(9);
    pub const RGB_888: Self = Self(10);
    pub const BGR_888: Self = Self(11);
    pub const RGBA_8888: Self = Self(12);
    pub const BGRA_8888: Self = Self(13);
    pub const RGBA_8888_sRGB: Self = Self(14);
    pub const BGRA_8888_sRGB: Self = Self(15);
    pub const RGBA_1010102: Self = Self(16);
    pub const BGRA_1010102: Self = Self(17);
    pub const RGB_161616LE: Self = Self(18);
    pub const BGR_161616LE: Self = Self(19);
    pub const RGBA_16161616LE: Self = Self(20);
    pub const BGRA_16161616LE: Self = Self(21);
    pub const RGB_161616BE: Self = Self(22);
    pub const BGR_161616BE: Self = Self(23);
    pub const RGBA_16161616BE: Self = Self(24);
    pub const BGRA_16161616BE: Self = Self(25);
    pub const RGB_hhh_Norm: Self = Self(26);
    pub const BGR_hhh_Norm: Self = Self(27);
    pub const RGBA_hhhh_Norm: Self = Self(28);
    pub const BGRA_hhhh_Norm: Self = Self(29);
    pub const RGB_hhh: Self = Self(30);
    pub const BGR_hhh: Self = Self(31);
    pub const RGBA_hhhh: Self = Self(32);
    pub const BGRA_hhhh: Self = Self(33);
    pub const RGB_fff: Self = Self(34);
    pub const BGR_fff: Self = Self(35);
    pub const RGBA_ffff: Self = Self(36);
    pub const BGRA_ffff: Self = Self(37);
    pub const RGB_101010x_XR: Self = Self(38);
    pub const BGR_101010x_XR: Self = Self(39);
    pub const RGBA_10101010_XR: Self = Self(40);
    pub const BGRA_10101010_XR: Self = Self(41);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct AlphaFormat(pub i32);
#[expect(non_upper_case_globals, reason = "Preserve source alpha names.")]
impl AlphaFormat {
    pub const Opaque: Self = Self(0);
    pub const Unpremul: Self = Self(1);
    pub const PremulAsEncoded: Self = Self(2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
#[expect(non_camel_case_types, reason = "Preserve source stage names.")]
pub(crate) enum Op {
    load_a8,
    load_g8,
    load_ga88,
    load_4444,
    load_565,
    load_888,
    load_8888,
    load_1010102,
    load_101010x_XR,
    load_10101010_XR,
    load_161616LE,
    load_16161616LE,
    load_161616BE,
    load_16161616BE,
    load_hhh,
    load_hhhh,
    load_fff,
    load_ffff,
    swap_rb,
    clamp,
    invert,
    force_opaque,
    premul,
    unpremul,
    matrix_3x3,
    matrix_3x4,
    lab_to_xyz,
    xyz_to_lab,
    gamma_r,
    gamma_g,
    gamma_b,
    gamma_a,
    gamma_rgb,
    tf_r,
    tf_g,
    tf_b,
    tf_a,
    tf_rgb,
    pq_r,
    pq_g,
    pq_b,
    pq_a,
    pq_rgb,
    hlg_r,
    hlg_g,
    hlg_b,
    hlg_a,
    hlg_rgb,
    hlg_ootf_scale,
    hlginv_r,
    hlginv_g,
    hlginv_b,
    hlginv_a,
    hlginv_rgb,
    hlginv_ootf_scale,
    table_r,
    table_g,
    table_b,
    table_a,
    clut_A2B,
    clut_B2A,
    store_a8,
    store_g8,
    store_ga88,
    store_4444,
    store_565,
    store_888,
    store_8888,
    store_1010102,
    store_161616LE,
    store_16161616LE,
    store_161616BE,
    store_16161616BE,
    store_101010x_XR,
    store_10101010_XR,
    store_hhh,
    store_hhhh,
    store_fff,
    store_ffff,
}
