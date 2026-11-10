// libwebp v1.6.0, src/enc/picture_csp_enc.c:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// WebPPicture utils for colorspace conversion
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/picture_csp_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! The C file is compiled in its reference configuration: gamma compression and the
//! inverse-alpha table are on (`USE_GAMMA_COMPRESSION`, `USE_INVERSE_ALPHA_TABLE`),
//! `WEBP_REDUCE_CSP` and `WEBP_MSAN` are off, and the bytes of ARGB samples are read
//! in native order, as C's casts read them.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Native-order byte views.** C reads and writes ARGB samples as bytes in
//!    five places: the ARGB-to-YUVA conversion, the YUVA-to-ARGB upsampler
//!    output, the RGBA import through `VP8LConvertBGRAToRGBA`, the rescaling of
//!    ARGB pictures, and the metric of `WebPPictureDistortion` on the ARGB
//!    views. The Rust code makes copies in native byte order (`to_ne_bytes`,
//!    `from_ne_bytes`), converts, and copies the words back where it writes.
//!    The values are the same; the copies cost one temporary of the plane size.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Misaligned `uint32_t` reads in the RGBA import.**
//!    `WebPPictureImportRGBA` with an ARGB picture reads each row through
//!    `VP8LConvertBGRAToRGBA((const uint32_t*)rgb, ...)`
//!    (`upstream/libwebp/src/dsp/lossless.c`). When the row stride is not a
//!    multiple of 4 and the picture has more than one row, the row starts off a
//!    4-byte boundary, and the `asan-ubsan` variant reports the misaligned
//!    load. The import test (`tests/wp9_picture.rs`) excludes these inputs
//!    (`kind == rgba`, ARGB, `stride % 4 != 0`, `height > 1`). The Rust
//!    translation reads the same bytes in native order, which is the value the
//!    C code computes where unaligned loads are tolerated; the Rust side
//!    defines that reading, and the exclusion keeps the oracle from proving it.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. The padding byte of `WebPPictureImportRGBX` and `BGRX` (not read), and
//!    the alignment of the ARGB plane (not observable, see
//!    `webp_picture_alloc_argb` in `src/enc/picture_enc.rs`).

use std::sync::LazyLock;

use crate::dsp::alpha_processing::{
    webp_extract_alpha, webp_has_alpha8b, webp_has_alpha32b, webp_pack_rgb,
};
use crate::dsp::lossless::vp8l_convert_bgra_to_rgba;
use crate::dsp::upsampling::{WebPUpsampleLinePairFunc, webp_get_line_pair_converter};
use crate::dsp::yuv::{
    YUV_FIX, YUV_HALF, vp8rgb_to_u, vp8rgb_to_v, vp8rgb_to_y, webp_convert_bgr24_to_y,
    webp_convert_rgb24_to_y, webp_convert_rgba32_to_uv,
};
use crate::enc::picture_enc::{
    argb_bytes, argb_samples, argb_samples_mut, offset_by, webp_picture_alloc,
    webp_picture_alloc_argb, webp_picture_alloc_yuva, yuva_planes_mut, yuva_samples,
};
use crate::enc::webp_enc::webp_encoding_set_error;
use crate::sharpyuv::sharpyuv::sharp_yuv_convert;
use crate::sharpyuv::sharpyuv_csp::{SharpYuvMatrixType, sharp_yuv_get_conversion_matrix};
use crate::utils::random_utils::{
    VP8_RANDOM_TABLE_SIZE, VP8Random, vp8_init_random, vp8_random_bits,
};
use crate::utils::utils::webp_safe_malloc;
use crate::webp::encode::{WebPEncCSP, WebPEncodingError, WebPPicture};

// upstream: src/enc/picture_csp_enc.c CHANNEL_OFFSET
/// The byte of a pixel that holds channel `i` (0 alpha, 1 red, 2 green, 3 blue): the
/// layout of a `uint32_t` in native byte order, as C's byte casts read it.
const fn channel_offset(i: usize) -> usize {
    if cfg!(target_endian = "big") {
        i
    } else {
        3 - i
    }
}

// upstream: src/enc/picture_csp_enc.c ALPHA_OFFSET
const ALPHA_OFFSET: usize = channel_offset(0);

/// A sample position as an index; a negative position is a precondition violation.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("the sample position lies in the plane")
}

/// A sample of a byte buffer at `pos`.
fn sample(src: &[u8], pos: usize) -> u8 {
    src[pos]
}

// upstream: src/enc/picture_csp_enc.c CheckNonOpaque
/// Returns `true` if alpha[] has non-0xff values. `alpha` is the position of the
/// first alpha sample in `src`; the samples of a row are `x_step` bytes apart and the
/// rows are `y_step` bytes apart. `None` stands for a `NULL` alpha pointer.
fn check_non_opaque(
    src: &[u8],
    alpha: Option<usize>,
    width: i32,
    height: i32,
    x_step: i32,
    y_step: i32,
) -> bool {
    let Some(alpha) = alpha else {
        return false;
    };
    // WebPInitAlphaProcessing() only assigns portable pointers (ADR 0003).
    for row in 0..height {
        let row_pos = offset_by(alpha, i64::from(row) * i64::from(y_step));
        // C only advances the pointer; with a width of 0 or less no sample is
        // read, so a row past the end of `src` is empty.
        let samples = src.get(row_pos..).unwrap_or_default();
        let found = if x_step == 1 {
            webp_has_alpha8b(samples, width)
        } else {
            webp_has_alpha32b(samples, width)
        };
        if found {
            return true;
        }
    }
    false
}

// upstream: src/enc/picture_csp_enc.c WebPPictureHasTransparency
/// Reports whether `picture` has a sample that is not opaque (C:
/// `WebPPictureHasTransparency`). An ARGB picture without a plane has none.
#[must_use]
pub fn webp_picture_has_transparency(picture: &WebPPicture<'_>) -> bool {
    if picture.use_argb != 0 {
        if let Some(plane) = picture.argb {
            let bytes = argb_bytes(argb_samples(&picture.argb_storage, plane));
            return check_non_opaque(
                &bytes,
                Some(ALPHA_OFFSET),
                picture.width,
                picture.height,
                4,
                picture.argb_stride * 4,
            );
        }
        return false;
    }
    match picture.a {
        None => false,
        Some(a) => check_non_opaque(
            yuva_samples(&picture.yuva, a),
            Some(0),
            picture.width,
            picture.height,
            1,
            picture.a_stride,
        ),
    }
}

//------------------------------------------------------------------------------
// Code for gamma correction

// Gamma correction compensates loss of resolution during chroma subsampling.
// upstream: src/enc/picture_csp_enc.c GAMMA_FIX
const GAMMA_FIX: i32 = 12; // fixed-point precision for linear values
// upstream: src/enc/picture_csp_enc.c GAMMA_TAB_FIX
const GAMMA_TAB_FIX: i32 = 7; // fixed-point fractional bits precision
// upstream: src/enc/picture_csp_enc.c GAMMA_TAB_SIZE
const GAMMA_TAB_SIZE: i32 = 1 << (GAMMA_FIX - GAMMA_TAB_FIX);
// upstream: src/enc/picture_csp_enc.c kGamma
const GAMMA: f64 = 0.80;
// upstream: src/enc/picture_csp_enc.c kGammaScale
const GAMMA_SCALE: i32 = (1 << GAMMA_FIX) - 1;
// upstream: src/enc/picture_csp_enc.c kGammaTabScale
const GAMMA_TAB_SCALE: i32 = 1 << GAMMA_TAB_FIX;
// upstream: src/enc/picture_csp_enc.c kGammaTabRounder
const GAMMA_TAB_ROUNDER: i32 = (1 << GAMMA_TAB_FIX) >> 1;

// upstream: src/enc/picture_csp_enc.c kGammaToLinearTab
// The table that C fills once in `InitGammaTables`, here at first use (ADR 0003).
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does; the C conversion truncates as the source does"
)]
static GAMMA_TO_LINEAR_TAB: LazyLock<[u16; 256]> = LazyLock::new(|| {
    let norm = 1.0_f64 / 255.0;
    let mut tab = [0_u16; 256];
    for (v, slot) in tab.iter_mut().enumerate() {
        let v = f64::from(u16::try_from(v).expect("the index is below 256"));
        *slot = ((norm * v).powf(GAMMA) * f64::from(GAMMA_SCALE) + 0.5) as u16;
    }
    tab
});

// upstream: src/enc/picture_csp_enc.c kLinearToGammaTab
// The table that C fills once in `InitGammaTables`, here at first use (ADR 0003).
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion truncates as the source does"
)]
static LINEAR_TO_GAMMA_TAB: LazyLock<[i32; (GAMMA_TAB_SIZE + 1) as usize]> = LazyLock::new(|| {
    let scale = f64::from(1 << GAMMA_TAB_FIX) / f64::from(GAMMA_SCALE);
    let mut tab = [0_i32; (GAMMA_TAB_SIZE + 1) as usize];
    for (v, slot) in tab.iter_mut().enumerate() {
        let v = f64::from(u16::try_from(v).expect("the index is at most 32"));
        *slot = (255.0 * (scale * v).powf(1.0 / GAMMA) + 0.5) as i32;
    }
    tab
});

// upstream: src/enc/picture_csp_enc.c InitGammaTables
/// Computes the two gamma tables now, as C's `InitGammaTables` does.
fn init_gamma_tables() {
    LazyLock::force(&GAMMA_TO_LINEAR_TAB);
    LazyLock::force(&LINEAR_TO_GAMMA_TAB);
}

// upstream: src/enc/picture_csp_enc.c GammaToLinear
fn gamma_to_linear(v: u8) -> u32 {
    u32::from(GAMMA_TO_LINEAR_TAB[usize::from(v)])
}

// upstream: src/enc/picture_csp_enc.c Interpolate
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does"
)]
fn interpolate(v: i32) -> i32 {
    let tab_pos = (v >> (GAMMA_TAB_FIX + 2)) as usize; // integer part
    let x = v & ((GAMMA_TAB_SCALE << 2) - 1); // fractional part
    let v0 = LINEAR_TO_GAMMA_TAB[tab_pos];
    let v1 = LINEAR_TO_GAMMA_TAB[tab_pos + 1];
    let y = v1 * x + v0 * ((GAMMA_TAB_SCALE << 2) - x); // interpolate
    debug_assert!(tab_pos + 1 < (GAMMA_TAB_SIZE + 1) as usize);
    y
}

// upstream: src/enc/picture_csp_enc.c LinearToGamma
/// Converts a linear value to the `YUV_FIX + 2` fixed-point precision U/V value that
/// the RGB to U/V calls take.
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C conversion wraps as the source does"
)]
fn linear_to_gamma(base_value: u32, shift: i32) -> i32 {
    let y = interpolate((base_value << shift) as i32); // final uplifted value
    (y + GAMMA_TAB_ROUNDER) >> GAMMA_TAB_FIX // descale
}

//------------------------------------------------------------------------------
// RGB -> YUV conversion

// upstream: src/enc/picture_csp_enc.c RGBToY
fn rgb_to_y(r: i32, g: i32, b: i32, rg: Option<&mut VP8Random>) -> i32 {
    match rg {
        None => vp8rgb_to_y(r, g, b, YUV_HALF),
        Some(rg) => vp8rgb_to_y(r, g, b, vp8_random_bits(rg, YUV_FIX)),
    }
}

// upstream: src/enc/picture_csp_enc.c RGBToU
fn rgb_to_u(r: i32, g: i32, b: i32, rg: Option<&mut VP8Random>) -> i32 {
    match rg {
        None => vp8rgb_to_u(r, g, b, YUV_HALF << 2),
        Some(rg) => vp8rgb_to_u(r, g, b, vp8_random_bits(rg, YUV_FIX + 2)),
    }
}

// upstream: src/enc/picture_csp_enc.c RGBToV
fn rgb_to_v(r: i32, g: i32, b: i32, rg: Option<&mut VP8Random>) -> i32 {
    match rg {
        None => vp8rgb_to_v(r, g, b, YUV_HALF << 2),
        Some(rg) => vp8rgb_to_v(r, g, b, vp8_random_bits(rg, YUV_FIX + 2)),
    }
}

//------------------------------------------------------------------------------
// Sharp RGB->YUV conversion

// upstream: src/enc/picture_csp_enc.c kMinDimensionIterativeConversion
const MIN_DIMENSION_ITERATIVE_CONVERSION: i32 = 4;

// upstream: src/enc/picture_csp_enc.c PreprocessARGB
/// Converts the RGB source to the Y, U and V planes with the sharp conversion, the
/// WebP matrix and 8-bit depths (C: `PreprocessARGB`).
fn preprocess_argb(
    src: &[u8],
    r_ptr: usize,
    g_ptr: usize,
    b_ptr: usize,
    step: i32,
    rgb_stride: i32,
    picture: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType::kSharpYuvMatrixWebp)
        .expect("the WebP conversion matrix is defined");
    let y_stride = picture.y_stride;
    let uv_stride = picture.uv_stride;
    let width = picture.width;
    let height = picture.height;
    let [y_plane, u_plane, v_plane, _] =
        yuva_planes_mut(&mut picture.yuva, [picture.y, picture.u, picture.v, None]);
    let y_plane = y_plane.expect("the luma plane is allocated");
    let u_plane = u_plane.expect("the U plane is allocated");
    let v_plane = v_plane.expect("the V plane is allocated");
    let ok = sharp_yuv_convert(
        src, r_ptr, src, g_ptr, src, b_ptr, step, rgb_stride, 8, y_plane, 0, y_stride, u_plane, 0,
        uv_stride, v_plane, 0, uv_stride, 8, width, height, matrix,
    );
    if !ok {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }
    Ok(())
}

//------------------------------------------------------------------------------
// "Fast" regular RGB->YUV

// upstream: src/enc/picture_csp_enc.c SUM4
/// The gamma-weighted average of the 2x2 block at `ptr` (C macro `SUM4`), with the
/// samples `step` bytes apart in a row and `rgb_stride` bytes between rows.
fn sum4(src: &[u8], ptr: usize, step: i32, rgb_stride: i32) -> i32 {
    linear_to_gamma(
        gamma_to_linear(sample(src, ptr))
            + gamma_to_linear(sample(src, offset_by(ptr, i64::from(step))))
            + gamma_to_linear(sample(src, offset_by(ptr, i64::from(rgb_stride))))
            + gamma_to_linear(sample(
                src,
                offset_by(ptr, i64::from(rgb_stride) + i64::from(step)),
            )),
        0,
    )
}

// upstream: src/enc/picture_csp_enc.c SUM2
/// The gamma-weighted average of the pair at `ptr` (C macro `SUM2`), with the
/// samples `rgb_stride` bytes apart.
fn sum2(src: &[u8], ptr: usize, rgb_stride: i32) -> i32 {
    linear_to_gamma(
        gamma_to_linear(sample(src, ptr))
            + gamma_to_linear(sample(src, offset_by(ptr, i64::from(rgb_stride)))),
        1,
    )
}

// upstream: src/enc/picture_csp_enc.c SUM2ALPHA
/// The sum of the two alpha samples of a column (C macro `SUM2ALPHA`).
fn sum2_alpha(src: &[u8], ptr: usize, rgb_stride: i32) -> u32 {
    u32::from(sample(src, ptr)) + u32::from(sample(src, offset_by(ptr, i64::from(rgb_stride))))
}

// upstream: src/enc/picture_csp_enc.c SUM4ALPHA
/// The sum of the four alpha samples of a 2x2 block (C macro `SUM4ALPHA`).
fn sum4_alpha(src: &[u8], ptr: usize, rgb_stride: i32) -> u32 {
    sum2_alpha(src, ptr, rgb_stride) + sum2_alpha(src, offset_by(ptr, 4), rgb_stride)
}

// upstream: src/enc/picture_csp_enc.c kAlphaFix
const ALPHA_FIX: i32 = 19;
// upstream: src/enc/picture_csp_enc.c ImportYUVAFromRGBA (assert kAlphaFix + GAMMA_FIX <= 31)
const _: () = assert!(ALPHA_FIX + GAMMA_FIX <= 31);

// upstream: src/enc/picture_csp_enc.c kInvAlpha
// Following table is (1 << kAlphaFix) / a. The (v * kInvAlpha[a]) >> kAlphaFix
// formula is then equal to v / a in most (99.6%) cases. Note that this table
// and constant are adjusted very tightly to fit 32b arithmetic.
// In particular, they use the fact that the operands for 'v / a' are actually
// derived as v = (a0.p0 + a1.p1 + a2.p2 + a3.p3) and a = a0 + a1 + a2 + a3
// with ai in [0..255] and pi in [0..1<<GAMMA_FIX). The constraint to avoid
// overflow is: GAMMA_FIX + kAlphaFix <= 31.
const INV_ALPHA: [u32; 4 * 0xff + 1] = [
    // alpha = 0 has no inverse; the entry is unused for it (C: 0)
    0, 524_288, 262_144, 174_762, 131_072, 104_857, 87_381, 74_898, 65_536, 58_254, 52_428, 47_662,
    43_690, 40_329, 37_449, 34_952, 32_768, 30_840, 29_127, 27_594, 26_214, 24_966, 23_831, 22_795,
    21_845, 20_971, 20_164, 19_418, 18_724, 18_078, 17_476, 16_912, 16_384, 15_887, 15_420, 14_979,
    14_563, 14_169, 13_797, 13_443, 13_107, 12_787, 12_483, 12_192, 11_915, 11_650, 11_397, 11_155,
    10_922, 10_699, 10_485, 10_280, 10_082, 9892, 9709, 9532, 9362, 9198, 9039, 8886, 8738, 8594,
    8456, 8322, 8192, 8065, 7943, 7825, 7710, 7598, 7489, 7384, 7281, 7182, 7084, 6990, 6898, 6808,
    6721, 6636, 6553, 6472, 6393, 6316, 6241, 6168, 6096, 6026, 5957, 5890, 5825, 5761, 5698, 5637,
    5577, 5518, 5461, 5405, 5349, 5295, 5242, 5190, 5140, 5090, 5041, 4993, 4946, 4899, 4854, 4809,
    4766, 4723, 4681, 4639, 4599, 4559, 4519, 4481, 4443, 4405, 4369, 4332, 4297, 4262, 4228, 4194,
    4161, 4128, 4096, 4064, 4032, 4002, 3971, 3942, 3912, 3883, 3855, 3826, 3799, 3771, 3744, 3718,
    3692, 3666, 3640, 3615, 3591, 3566, 3542, 3518, 3495, 3472, 3449, 3426, 3404, 3382, 3360, 3339,
    3318, 3297, 3276, 3256, 3236, 3216, 3196, 3177, 3158, 3139, 3120, 3102, 3084, 3066, 3048, 3030,
    3013, 2995, 2978, 2962, 2945, 2928, 2912, 2896, 2880, 2864, 2849, 2833, 2818, 2803, 2788, 2774,
    2759, 2744, 2730, 2716, 2702, 2688, 2674, 2661, 2647, 2634, 2621, 2608, 2595, 2582, 2570, 2557,
    2545, 2532, 2520, 2508, 2496, 2484, 2473, 2461, 2449, 2438, 2427, 2416, 2404, 2394, 2383, 2372,
    2361, 2351, 2340, 2330, 2319, 2309, 2299, 2289, 2279, 2269, 2259, 2250, 2240, 2231, 2221, 2212,
    2202, 2193, 2184, 2175, 2166, 2157, 2148, 2139, 2131, 2122, 2114, 2105, 2097, 2088, 2080, 2072,
    2064, 2056, 2048, 2040, 2032, 2024, 2016, 2008, 2001, 1993, 1985, 1978, 1971, 1963, 1956, 1949,
    1941, 1934, 1927, 1920, 1913, 1906, 1899, 1892, 1885, 1879, 1872, 1865, 1859, 1852, 1846, 1839,
    1833, 1826, 1820, 1814, 1807, 1801, 1795, 1789, 1783, 1777, 1771, 1765, 1759, 1753, 1747, 1741,
    1736, 1730, 1724, 1718, 1713, 1707, 1702, 1696, 1691, 1685, 1680, 1675, 1669, 1664, 1659, 1653,
    1648, 1643, 1638, 1633, 1628, 1623, 1618, 1613, 1608, 1603, 1598, 1593, 1588, 1583, 1579, 1574,
    1569, 1565, 1560, 1555, 1551, 1546, 1542, 1537, 1533, 1528, 1524, 1519, 1515, 1510, 1506, 1502,
    1497, 1493, 1489, 1485, 1481, 1476, 1472, 1468, 1464, 1460, 1456, 1452, 1448, 1444, 1440, 1436,
    1432, 1428, 1424, 1420, 1416, 1413, 1409, 1405, 1401, 1398, 1394, 1390, 1387, 1383, 1379, 1376,
    1372, 1368, 1365, 1361, 1358, 1354, 1351, 1347, 1344, 1340, 1337, 1334, 1330, 1327, 1323, 1320,
    1317, 1314, 1310, 1307, 1304, 1300, 1297, 1294, 1291, 1288, 1285, 1281, 1278, 1275, 1272, 1269,
    1266, 1263, 1260, 1257, 1254, 1251, 1248, 1245, 1242, 1239, 1236, 1233, 1230, 1227, 1224, 1222,
    1219, 1216, 1213, 1210, 1208, 1205, 1202, 1199, 1197, 1194, 1191, 1188, 1186, 1183, 1180, 1178,
    1175, 1172, 1170, 1167, 1165, 1162, 1159, 1157, 1154, 1152, 1149, 1147, 1144, 1142, 1139, 1137,
    1134, 1132, 1129, 1127, 1125, 1122, 1120, 1117, 1115, 1113, 1110, 1108, 1106, 1103, 1101, 1099,
    1096, 1094, 1092, 1089, 1087, 1085, 1083, 1081, 1078, 1076, 1074, 1072, 1069, 1067, 1065, 1063,
    1061, 1059, 1057, 1054, 1052, 1050, 1048, 1046, 1044, 1042, 1040, 1038, 1036, 1034, 1032, 1030,
    1028, 1026, 1024, 1022, 1020, 1018, 1016, 1014, 1012, 1010, 1008, 1006, 1004, 1002, 1000, 998,
    996, 994, 992, 991, 989, 987, 985, 983, 981, 979, 978, 976, 974, 972, 970, 969, 967, 965, 963,
    961, 960, 958, 956, 954, 953, 951, 949, 948, 946, 944, 942, 941, 939, 937, 936, 934, 932, 931,
    929, 927, 926, 924, 923, 921, 919, 918, 916, 914, 913, 911, 910, 908, 907, 905, 903, 902, 900,
    899, 897, 896, 894, 893, 891, 890, 888, 887, 885, 884, 882, 881, 879, 878, 876, 875, 873, 872,
    870, 869, 868, 866, 865, 863, 862, 860, 859, 858, 856, 855, 853, 852, 851, 849, 848, 846, 845,
    844, 842, 841, 840, 838, 837, 836, 834, 833, 832, 830, 829, 828, 826, 825, 824, 823, 821, 820,
    819, 817, 816, 815, 814, 812, 811, 810, 809, 807, 806, 805, 804, 802, 801, 800, 799, 798, 796,
    795, 794, 793, 791, 790, 789, 788, 787, 786, 784, 783, 782, 781, 780, 779, 777, 776, 775, 774,
    773, 772, 771, 769, 768, 767, 766, 765, 764, 763, 762, 760, 759, 758, 757, 756, 755, 754, 753,
    752, 751, 750, 748, 747, 746, 745, 744, 743, 742, 741, 740, 739, 738, 737, 736, 735, 734, 733,
    732, 731, 730, 729, 728, 727, 726, 725, 724, 723, 722, 721, 720, 719, 718, 717, 716, 715, 714,
    713, 712, 711, 710, 709, 708, 707, 706, 705, 704, 703, 702, 701, 700, 699, 699, 698, 697, 696,
    695, 694, 693, 692, 691, 690, 689, 688, 688, 687, 686, 685, 684, 683, 682, 681, 680, 680, 679,
    678, 677, 676, 675, 674, 673, 673, 672, 671, 670, 669, 668, 667, 667, 666, 665, 664, 663, 662,
    661, 661, 660, 659, 658, 657, 657, 656, 655, 654, 653, 652, 652, 651, 650, 649, 648, 648, 647,
    646, 645, 644, 644, 643, 642, 641, 640, 640, 639, 638, 637, 637, 636, 635, 634, 633, 633, 632,
    631, 630, 630, 629, 628, 627, 627, 626, 625, 624, 624, 623, 622, 621, 621, 620, 619, 618, 618,
    617, 616, 616, 615, 614, 613, 613, 612, 611, 611, 610, 609, 608, 608, 607, 606, 606, 605, 604,
    604, 603, 602, 601, 601, 600, 599, 599, 598, 597, 597, 596, 595, 595, 594, 593, 593, 592, 591,
    591, 590, 589, 589, 588, 587, 587, 586, 585, 585, 584, 583, 583, 582, 581, 581, 580, 579, 579,
    578, 578, 577, 576, 576, 575, 574, 574, 573, 572, 572, 571, 571, 570, 569, 569, 568, 568, 567,
    566, 566, 565, 564, 564, 563, 563, 562, 561, 561, 560, 560, 559, 558, 558, 557, 557, 556, 555,
    555, 554, 554, 553, 553, 552, 551, 551, 550, 550, 549, 548, 548, 547, 547, 546, 546, 545, 544,
    544, 543, 543, 542, 542, 541, 541, 540, 539, 539, 538, 538, 537, 537, 536, 536, 535, 534, 534,
    533, 533, 532, 532, 531, 531, 530, 530, 529, 529, 528, 527, 527, 526, 526, 525, 525, 524, 524,
    523, 523, 522, 522, 521, 521, 520, 520, 519, 519, 518, 518, 517, 517, 516, 516, 515, 515, 514,
    514,
];

// upstream: src/enc/picture_csp_enc.c DIVIDE_BY_ALPHA
// Note that LinearToGamma() expects the values to be premultiplied by 4,
// so we incorporate this factor 4 inside the DIVIDE_BY_ALPHA macro directly.
fn divide_by_alpha(sum: u32, a: u32) -> u32 {
    sum.wrapping_mul(INV_ALPHA[usize::try_from(a).expect("the alpha fits the table")])
        >> (ALPHA_FIX - 2)
}

// upstream: src/enc/picture_csp_enc.c LinearToGammaWeighted
fn linear_to_gamma_weighted(
    src: &[u8],
    src_pos: usize,
    a_pos: usize,
    total_a: u32,
    step: i32,
    rgb_stride: i32,
) -> i32 {
    // One weighted term per sample of the 2x2 block: alpha times linear value.
    let term = |delta: i64| -> u32 {
        let a = sample(src, offset_by(a_pos, delta));
        u32::from(a).wrapping_mul(gamma_to_linear(sample(src, offset_by(src_pos, delta))))
    };
    let sum = term(0)
        .wrapping_add(term(i64::from(step)))
        .wrapping_add(term(i64::from(rgb_stride)))
        .wrapping_add(term(i64::from(rgb_stride) + i64::from(step)));
    debug_assert!(total_a > 0 && total_a <= 4 * 0xff);
    debug_assert!(
        u64::from(sum)
            * u64::from(INV_ALPHA[usize::try_from(total_a).expect("the alpha fits the table")])
            < 1_u64 << 32
    );
    linear_to_gamma(divide_by_alpha(sum, total_a), 0)
}

// upstream: src/enc/picture_csp_enc.c ConvertRowToY
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has seven parameters, and the planes need positions"
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does; the C conversion truncates as the source does"
)]
fn convert_row_to_y(
    src: &[u8],
    r_ptr: usize,
    g_ptr: usize,
    b_ptr: usize,
    step: i32,
    dst_y: &mut [u8],
    dst_pos: usize,
    width: i32,
    mut rg: Option<&mut VP8Random>,
) {
    let mut j = 0_i64;
    for i in 0..index(width) {
        let r = i32::from(sample(src, offset_by(r_ptr, j)));
        let g = i32::from(sample(src, offset_by(g_ptr, j)));
        let b = i32::from(sample(src, offset_by(b_ptr, j)));
        dst_y[dst_pos + i] = rgb_to_y(r, g, b, rg.as_deref_mut()) as u8;
        j += i64::from(step);
    }
}

// upstream: src/enc/picture_csp_enc.c AccumulateRGBA
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has eight parameters"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the accumulated values are at most 1020, which fit uint16_t as in C"
)]
#[expect(
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    reason = "the C conversion keeps the bit pattern, as the source does; the names are those of the C code"
)]
fn accumulate_rgba(
    src: &[u8],
    r_ptr: usize,
    g_ptr: usize,
    b_ptr: usize,
    a_ptr: usize,
    rgb_stride: i32,
    dst: &mut [u16],
    width: i32,
) {
    // we loop over 2x2 blocks and produce one R/G/B/A value for each.
    let mut j = 0_i64;
    let mut d = 0_usize;
    for _ in 0..(width >> 1) {
        let a = sum4_alpha(src, offset_by(a_ptr, j), rgb_stride);
        let (r, g, b);
        if a == 4 * 0xff || a == 0 {
            r = sum4(src, offset_by(r_ptr, j), 4, rgb_stride);
            g = sum4(src, offset_by(g_ptr, j), 4, rgb_stride);
            b = sum4(src, offset_by(b_ptr, j), 4, rgb_stride);
        } else {
            r = linear_to_gamma_weighted(
                src,
                offset_by(r_ptr, j),
                offset_by(a_ptr, j),
                a,
                4,
                rgb_stride,
            );
            g = linear_to_gamma_weighted(
                src,
                offset_by(g_ptr, j),
                offset_by(a_ptr, j),
                a,
                4,
                rgb_stride,
            );
            b = linear_to_gamma_weighted(
                src,
                offset_by(b_ptr, j),
                offset_by(a_ptr, j),
                a,
                4,
                rgb_stride,
            );
        }
        dst[d] = r as u16;
        dst[d + 1] = g as u16;
        dst[d + 2] = b as u16;
        dst[d + 3] = a as u16;
        j += 2 * 4;
        d += 4;
    }
    if (width & 1) != 0 {
        let a = 2 * sum2_alpha(src, offset_by(a_ptr, j), rgb_stride);
        let (r, g, b);
        if a == 4 * 0xff || a == 0 {
            r = sum2(src, offset_by(r_ptr, j), rgb_stride);
            g = sum2(src, offset_by(g_ptr, j), rgb_stride);
            b = sum2(src, offset_by(b_ptr, j), rgb_stride);
        } else {
            r = linear_to_gamma_weighted(
                src,
                offset_by(r_ptr, j),
                offset_by(a_ptr, j),
                a,
                0,
                rgb_stride,
            );
            g = linear_to_gamma_weighted(
                src,
                offset_by(g_ptr, j),
                offset_by(a_ptr, j),
                a,
                0,
                rgb_stride,
            );
            b = linear_to_gamma_weighted(
                src,
                offset_by(b_ptr, j),
                offset_by(a_ptr, j),
                a,
                0,
                rgb_stride,
            );
        }
        dst[d] = r as u16;
        dst[d + 1] = g as u16;
        dst[d + 2] = b as u16;
        dst[d + 3] = a as u16;
    }
}

// upstream: src/enc/picture_csp_enc.c AccumulateRGB
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has seven parameters"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the accumulated values are at most 1020, which fit uint16_t as in C"
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does"
)]
fn accumulate_rgb(
    src: &[u8],
    r_ptr: usize,
    g_ptr: usize,
    b_ptr: usize,
    step: i32,
    rgb_stride: i32,
    dst: &mut [u16],
    width: i32,
) {
    let mut j = 0_i64;
    let mut d = 0_usize;
    for _ in 0..(width >> 1) {
        dst[d] = sum4(src, offset_by(r_ptr, j), step, rgb_stride) as u16;
        dst[d + 1] = sum4(src, offset_by(g_ptr, j), step, rgb_stride) as u16;
        dst[d + 2] = sum4(src, offset_by(b_ptr, j), step, rgb_stride) as u16;
        j += 2 * i64::from(step);
        d += 4;
    }
    if (width & 1) != 0 {
        dst[d] = sum2(src, offset_by(r_ptr, j), rgb_stride) as u16;
        dst[d + 1] = sum2(src, offset_by(g_ptr, j), rgb_stride) as u16;
        dst[d + 2] = sum2(src, offset_by(b_ptr, j), rgb_stride) as u16;
    }
}

// upstream: src/enc/picture_csp_enc.c ConvertRowsToUV
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the bit pattern, as the source does; the C conversion truncates as the source does"
)]
fn convert_rows_to_uv(
    rgb: &[u16],
    dst_u: &mut [u8],
    dst_v: &mut [u8],
    width: i32,
    mut rg: Option<&mut VP8Random>,
) {
    for i in 0..index(width) {
        let base = 4 * i;
        let r = i32::from(rgb[base]);
        let g = i32::from(rgb[base + 1]);
        let b = i32::from(rgb[base + 2]);
        dst_u[i] = rgb_to_u(r, g, b, rg.as_deref_mut()) as u8;
        dst_v[i] = rgb_to_v(r, g, b, rg.as_deref_mut()) as u8;
    }
}

// upstream: src/enc/picture_csp_enc.c ImportYUVAFromRGBA
/// Converts the RGB(A) source to the YUV(A) planes of `picture`, which it allocates
/// (C: `ImportYUVAFromRGBA`). The source is `src`; the first samples of the R, G, B
/// and A channels are at the positions `r_ptr`, `g_ptr`, `b_ptr` and `a_ptr`, and
/// `step` is the number of bytes per pixel.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has nine parameters"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the function follows the C control flow, with the sharp and the regular path"
)]
#[expect(
    clippy::if_not_else,
    reason = "the branches keep the order of the C code"
)]
fn import_yuva_from_rgba(
    src: &[u8],
    r_ptr: usize,
    g_ptr: usize,
    b_ptr: usize,
    a_ptr: Option<usize>,
    step: i32,
    rgb_stride: i32,
    dithering: f32,
    use_iterative_conversion: bool,
    picture: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let width = picture.width;
    let height = picture.height;
    let has_alpha = check_non_opaque(src, a_ptr, width, height, step, rgb_stride);
    let is_rgb = r_ptr < b_ptr; // otherwise it's bgr

    picture.colorspace = if has_alpha {
        WebPEncCSP::WEBP_YUV420A
    } else {
        WebPEncCSP::WEBP_YUV420
    };
    picture.use_argb = 0;

    // disable smart conversion if source is too small (overkill).
    let use_iterative_conversion = use_iterative_conversion
        && width >= MIN_DIMENSION_ITERATIVE_CONVERSION
        && height >= MIN_DIMENSION_ITERATIVE_CONVERSION;

    webp_picture_alloc_yuva(picture)?;
    if has_alpha {
        debug_assert_eq!(step, 4);
    }

    let y_stride = index(picture.y_stride);
    let uv_stride = index(picture.uv_stride);
    let a_stride = index(picture.a_stride);
    let a_stride_i32 = picture.a_stride;
    let uv_width = (width + 1) >> 1;

    if use_iterative_conversion {
        // SharpYuvInit(VP8GetCPUInfo) only assigns the dispatch pointers (WP5).
        preprocess_argb(src, r_ptr, g_ptr, b_ptr, step, rgb_stride, picture)?;
        if has_alpha {
            let a_src = a_ptr.expect("the alpha source exists when the source has alpha");
            let [_, _, _, a_plane] =
                yuva_planes_mut(&mut picture.yuva, [None, None, None, picture.a]);
            let a_plane = a_plane.expect("the alpha plane is allocated");
            webp_extract_alpha(
                &src[a_src..],
                rgb_stride,
                width,
                height,
                a_plane,
                picture.a_stride,
            );
        }
    } else {
        let mut use_dsp = step == 3; // use special function in this case
        // temporary storage for accumulated R/G/B values during conversion to U/V
        let tmp_rgb = webp_safe_malloc::<u16>(4 * u64::try_from(uv_width).unwrap_or(0), 2);

        let mut base_rg = VP8Random {
            index1: 0,
            index2: 0,
            tab: [0; VP8_RANDOM_TABLE_SIZE],
            amp: 0,
        };
        let mut rg: Option<&mut VP8Random> = None;
        if dithering > 0.0 {
            vp8_init_random(&mut base_rg, dithering);
            rg = Some(&mut base_rg);
            use_dsp = false; // can't use dsp in this case
        }
        // WebPInitConvertARGBToYUV() only assigns portable pointers (ADR 0003).
        init_gamma_tables();

        let Some(mut tmp_rgb) = tmp_rgb else {
            return Err(webp_encoding_set_error(
                picture,
                WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
            ));
        };
        tmp_rgb.resize(4 * index(uv_width), 0);

        let [y_plane, u_plane, v_plane, a_plane] = yuva_planes_mut(
            &mut picture.yuva,
            [picture.y, picture.u, picture.v, picture.a],
        );
        let y_plane = y_plane.expect("the luma plane is allocated");
        let u_plane = u_plane.expect("the U plane is allocated");
        let v_plane = v_plane.expect("the V plane is allocated");
        let mut a_plane = a_plane;

        // Downsample Y/U/V planes, two rows at a time
        let mut r_ptr = r_ptr;
        let mut g_ptr = g_ptr;
        let mut b_ptr = b_ptr;
        let mut a_ptr = a_ptr;
        let mut dst_y = 0_usize;
        let mut dst_u = 0_usize;
        let mut dst_a = 0_usize;
        for _ in 0..(height >> 1) {
            let mut rows_have_alpha = has_alpha;
            if use_dsp {
                if is_rgb {
                    webp_convert_rgb24_to_y(&src[r_ptr..], &mut y_plane[dst_y..], width);
                    webp_convert_rgb24_to_y(
                        &src[offset_by(r_ptr, i64::from(rgb_stride))..],
                        &mut y_plane[dst_y + y_stride..],
                        width,
                    );
                } else {
                    webp_convert_bgr24_to_y(&src[b_ptr..], &mut y_plane[dst_y..], width);
                    webp_convert_bgr24_to_y(
                        &src[offset_by(b_ptr, i64::from(rgb_stride))..],
                        &mut y_plane[dst_y + y_stride..],
                        width,
                    );
                }
            } else {
                convert_row_to_y(
                    src,
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    step,
                    y_plane,
                    dst_y,
                    width,
                    rg.as_deref_mut(),
                );
                convert_row_to_y(
                    src,
                    offset_by(r_ptr, i64::from(rgb_stride)),
                    offset_by(g_ptr, i64::from(rgb_stride)),
                    offset_by(b_ptr, i64::from(rgb_stride)),
                    step,
                    y_plane,
                    dst_y + y_stride,
                    width,
                    rg.as_deref_mut(),
                );
            }
            dst_y += 2 * y_stride;
            if has_alpha {
                let a_plane = a_plane.as_mut().expect("the alpha plane is allocated");
                let a_row = a_ptr.expect("the alpha source exists when the source has alpha");
                rows_have_alpha &= !webp_extract_alpha(
                    &src[a_row..],
                    rgb_stride,
                    width,
                    2,
                    &mut a_plane[dst_a..],
                    a_stride_i32,
                );
                dst_a += 2 * a_stride;
            }
            // Collect averaged R/G/B(/A)
            if !rows_have_alpha {
                accumulate_rgb(
                    src,
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    step,
                    rgb_stride,
                    &mut tmp_rgb,
                    width,
                );
            } else {
                accumulate_rgba(
                    src,
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    a_ptr.expect("the alpha source exists when the source has alpha"),
                    rgb_stride,
                    &mut tmp_rgb,
                    width,
                );
            }
            // Convert to U/V
            match &mut rg {
                None => webp_convert_rgba32_to_uv(
                    &tmp_rgb,
                    &mut u_plane[dst_u..],
                    &mut v_plane[dst_u..],
                    uv_width,
                ),
                Some(rg) => convert_rows_to_uv(
                    &tmp_rgb,
                    &mut u_plane[dst_u..],
                    &mut v_plane[dst_u..],
                    uv_width,
                    Some(&mut **rg),
                ),
            }
            dst_u += uv_stride;
            r_ptr = offset_by(r_ptr, 2 * i64::from(rgb_stride));
            b_ptr = offset_by(b_ptr, 2 * i64::from(rgb_stride));
            g_ptr = offset_by(g_ptr, 2 * i64::from(rgb_stride));
            if has_alpha {
                a_ptr = a_ptr.map(|a| offset_by(a, 2 * i64::from(rgb_stride)));
            }
        }
        if (height & 1) != 0 {
            // extra last row
            let mut row_has_alpha = has_alpha;
            if use_dsp {
                if is_rgb {
                    webp_convert_rgb24_to_y(&src[r_ptr..], &mut y_plane[dst_y..], width);
                } else {
                    webp_convert_bgr24_to_y(&src[b_ptr..], &mut y_plane[dst_y..], width);
                }
            } else {
                convert_row_to_y(
                    src,
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    step,
                    y_plane,
                    dst_y,
                    width,
                    rg.as_deref_mut(),
                );
            }
            if row_has_alpha {
                let a_plane = a_plane.as_mut().expect("the alpha plane is allocated");
                let a_row = a_ptr.expect("the alpha source exists when the source has alpha");
                row_has_alpha &=
                    !webp_extract_alpha(&src[a_row..], 0, width, 1, &mut a_plane[dst_a..], 0);
            }
            // Collect averaged R/G/B(/A)
            if !row_has_alpha {
                // Collect averaged R/G/B
                accumulate_rgb(src, r_ptr, g_ptr, b_ptr, step, 0, &mut tmp_rgb, width);
            } else {
                accumulate_rgba(
                    src,
                    r_ptr,
                    g_ptr,
                    b_ptr,
                    a_ptr.expect("the alpha source exists when the source has alpha"),
                    0,
                    &mut tmp_rgb,
                    width,
                );
            }
            match &mut rg {
                None => webp_convert_rgba32_to_uv(
                    &tmp_rgb,
                    &mut u_plane[dst_u..],
                    &mut v_plane[dst_u..],
                    uv_width,
                ),
                Some(rg) => convert_rows_to_uv(
                    &tmp_rgb,
                    &mut u_plane[dst_u..],
                    &mut v_plane[dst_u..],
                    uv_width,
                    Some(&mut **rg),
                ),
            }
        }
    }
    Ok(())
}

// upstream: src/enc/picture_csp_enc.c PictureARGBToYUVA
/// Converts the ARGB samples of `picture` to YUV with the given colorspace and
/// dithering, reading the channels as bytes in native order (C:
/// `PictureARGBToYUVA`).
fn picture_argb_to_yuva(
    picture: &mut WebPPicture<'_>,
    colorspace: WebPEncCSP,
    dithering: f32,
    use_iterative_conversion: bool,
) -> Result<(), WebPEncodingError> {
    let Some(plane) = picture.argb else {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER,
        ));
    };
    if (colorspace.0 & WebPEncCSP::WEBP_CSP_UV_MASK.0) != WebPEncCSP::WEBP_YUV420.0 {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION,
        ));
    }
    picture.colorspace = WebPEncCSP::WEBP_YUV420;
    let argb = argb_bytes(argb_samples(&picture.argb_storage, plane));
    let rgb_stride = 4 * picture.argb_stride;
    import_yuva_from_rgba(
        &argb,
        channel_offset(1),
        channel_offset(2),
        channel_offset(3),
        Some(channel_offset(0)),
        4,
        rgb_stride,
        dithering,
        use_iterative_conversion,
        picture,
    )
}

// upstream: src/enc/picture_csp_enc.c WebPPictureARGBToYUVADithered
/// Converts the ARGB samples to YUV with `dithering` (C: `WebPPictureARGBToYUVADithered`).
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_argb_to_yuva_dithered(
    picture: &mut WebPPicture<'_>,
    colorspace: WebPEncCSP,
    dithering: f32,
) -> Result<(), WebPEncodingError> {
    picture_argb_to_yuva(picture, colorspace, dithering, false)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureARGBToYUVA
/// Converts the ARGB samples to YUV without dithering (C: `WebPPictureARGBToYUVA`).
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_argb_to_yuva(
    picture: &mut WebPPicture<'_>,
    colorspace: WebPEncCSP,
) -> Result<(), WebPEncodingError> {
    picture_argb_to_yuva(picture, colorspace, 0.0, false)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureSharpARGBToYUVA
/// Converts the ARGB samples to YUV with the sharp (iterative) conversion (C:
/// `WebPPictureSharpARGBToYUVA`).
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_sharp_argb_to_yuva(
    picture: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    picture_argb_to_yuva(picture, WebPEncCSP::WEBP_YUV420, 0.0, true)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureSmartARGBToYUVA
/// Kept for backward compatibility; the sharp conversion (C:
/// `WebPPictureSmartARGBToYUVA`).
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_smart_argb_to_yuva(
    picture: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    webp_picture_sharp_argb_to_yuva(picture)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureYUVAToARGB
/// Converts the YUVA planes of `picture` to ARGB, replacing its ARGB plane (C:
/// `WebPPictureYUVAToARGB`). The upsampler writes bytes in the order of the
/// converter, which the native-order words of the ARGB plane reproduce.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
///
/// # Panics
///
/// Panics on a precondition that C leaves undefined, such as a missing plane.
#[expect(
    clippy::too_many_lines,
    reason = "the function follows the C control flow"
)]
pub fn webp_picture_yuva_to_argb(picture: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    if picture.y.is_none() || picture.u.is_none() || picture.v.is_none() {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER,
        ));
    }
    if (picture.colorspace.0 & WebPEncCSP::WEBP_CSP_ALPHA_BIT.0) != 0 && picture.a.is_none() {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER,
        ));
    }
    if (picture.colorspace.0 & WebPEncCSP::WEBP_CSP_UV_MASK.0) != WebPEncCSP::WEBP_YUV420.0 {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION,
        ));
    }
    // Allocate a new argb buffer (discarding the previous one).
    webp_picture_alloc_argb(picture)?;
    picture.use_argb = 1;

    // Convert
    let width = picture.width;
    let height = picture.height;
    let argb_stride = 4 * index(picture.argb_stride); // bytes per row
    let y_stride = index(picture.y_stride);
    let uv_stride = index(picture.uv_stride);
    let mut dst = vec![0_u8; argb_stride * index(height)];
    let colorspace = picture.colorspace;
    let a_plane = picture.a;
    let y_plane = yuva_samples(
        &picture.yuva,
        picture.y.expect("the luma plane is allocated"),
    );
    let u_plane = yuva_samples(&picture.yuva, picture.u.expect("the U plane is allocated"));
    let v_plane = yuva_samples(&picture.yuva, picture.v.expect("the V plane is allocated"));
    let upsample: WebPUpsampleLinePairFunc = webp_get_line_pair_converter(ALPHA_OFFSET > 0);

    // First row, with replicated top samples.
    let mut cur_y = 0_usize;
    let mut cur_u = 0_usize;
    let mut cur_v = 0_usize;
    let mut dst_row = 0_usize;
    upsample(
        &y_plane[cur_y..],
        None,
        &u_plane[cur_u..],
        &v_plane[cur_v..],
        &u_plane[cur_u..],
        &v_plane[cur_v..],
        &mut dst[dst_row..],
        width,
    );
    cur_y += y_stride;
    dst_row += argb_stride;
    // Center rows.
    let mut y = 1_i32;
    while y + 1 < height {
        let top_u = cur_u;
        let top_v = cur_v;
        cur_u += uv_stride;
        cur_v += uv_stride;
        let (top_part, bottom_part) = dst.split_at_mut(dst_row + argb_stride);
        upsample(
            &y_plane[cur_y..],
            Some((&y_plane[cur_y + y_stride..], &mut bottom_part[..])),
            &u_plane[top_u..],
            &v_plane[top_v..],
            &u_plane[cur_u..],
            &v_plane[cur_v..],
            &mut top_part[dst_row..],
            width,
        );
        cur_y += 2 * y_stride;
        dst_row += 2 * argb_stride;
        y += 2;
    }
    // Last row (if needed), with replicated bottom samples.
    if height > 1 && (height & 1) == 0 {
        upsample(
            &y_plane[cur_y..],
            None,
            &u_plane[cur_u..],
            &v_plane[cur_v..],
            &u_plane[cur_u..],
            &v_plane[cur_v..],
            &mut dst[dst_row..],
            width,
        );
    }

    // Store the converted rows as native-order words, one per pixel.
    let stride = index(picture.argb_stride);
    let argb_plane = picture.argb.expect("the ARGB plane is allocated");
    let samples = argb_samples_mut(&mut picture.argb_storage, argb_plane);
    for row in 0..index(height) {
        for x in 0..index(width) {
            samples[row * stride + x] = word_at(&dst, row * argb_stride + 4 * x);
        }
    }

    // Insert alpha values if needed, in replacement for the default 0xff ones.
    if (colorspace.0 & WebPEncCSP::WEBP_CSP_ALPHA_BIT.0) != 0 {
        let a_samples = yuva_samples(
            &picture.yuva,
            a_plane.expect("the alpha plane is allocated"),
        );
        let a_stride = index(picture.a_stride);
        for row in 0..index(height) {
            for x in 0..index(width) {
                let alpha = u32::from(a_samples[row * a_stride + x]);
                let at = row * stride + x;
                samples[at] = (samples[at] & 0x00ff_ffff) | (alpha << 24);
            }
        }
    }
    Ok(())
}

/// The native-order word of the four bytes at `pos`.
fn word_at(bytes: &[u8], pos: usize) -> u32 {
    u32::from_ne_bytes([bytes[pos], bytes[pos + 1], bytes[pos + 2], bytes[pos + 3]])
}

// upstream: src/enc/picture_csp_enc.c Import
/// Imports `rgb` into `picture`, reading `step` bytes per pixel with the red and blue
/// channels swapped when `swap_rb` is set, and the alpha channel when `import_alpha`
/// is set (C: `Import`). Returns `false` without recording an error when the stride
/// cannot hold a row; the other failures are recorded on the picture.
fn import(
    picture: &mut WebPPicture<'_>,
    rgb: &[u8],
    rgb_stride: i32,
    step: i32,
    swap_rb: bool,
    import_alpha: bool,
) -> bool {
    // swap_rb -> b,g,r,a , !swap_rb -> r,g,b,a
    let r_ptr = if swap_rb { 2 } else { 0 };
    let g_ptr = 1;
    let b_ptr = if swap_rb { 0 } else { 2 };
    let width = picture.width;
    let height = picture.height;

    let min_row = i64::from(if import_alpha { 4 } else { 3 }) * i64::from(width);
    if i64::from(rgb_stride).abs() < min_row {
        return false;
    }

    if picture.use_argb == 0 {
        let a_ptr = if import_alpha { Some(3) } else { None };
        return import_yuva_from_rgba(
            rgb, r_ptr, g_ptr, b_ptr, a_ptr, step, rgb_stride, 0.0, // no dithering
            false, picture,
        )
        .is_ok();
    }
    if webp_picture_alloc(picture).is_err() {
        return false;
    }

    // VP8LDspInit() and WebPInitAlphaProcessing() only assign portable pointers (ADR 0003).
    let plane = picture.argb.expect("the ARGB plane is allocated");
    let stride = index(picture.argb_stride);
    let samples = argb_samples_mut(&mut picture.argb_storage, plane);
    if import_alpha {
        // dst[] byte order is {a,r,g,b} for big-endian, {b,g,r,a} for little endian
        let do_copy = ALPHA_OFFSET == 3 && swap_rb;
        for y in 0..height {
            let row = offset_by(0, i64::from(y) * i64::from(rgb_stride));
            let dst = index(y) * stride;
            let words: Vec<u32> = (0..index(width))
                .map(|x| word_at(rgb, row + 4 * x))
                .collect();
            if do_copy {
                // memcpy(dst, rgb, width * 4)
                samples[dst..dst + index(width)].copy_from_slice(&words);
            } else {
                // RGBA input order. Need to swap R and B.
                // VP8LConvertBGRAToRGBA((const uint32_t*)rgb, width, (uint8_t*)dst)
                let mut out = vec![0_u8; 4 * index(width)];
                vp8l_convert_bgra_to_rgba(&words, width, &mut out);
                for x in 0..index(width) {
                    samples[dst + x] = word_at(&out, 4 * x);
                }
            }
        }
    } else {
        for y in 0..height {
            let delta = i64::from(y) * i64::from(rgb_stride);
            let r = offset_by(r_ptr, delta);
            let g = offset_by(g_ptr, delta);
            let b = offset_by(b_ptr, delta);
            webp_pack_rgb(
                &rgb[r..],
                &rgb[g..],
                &rgb[b..],
                width,
                step,
                &mut samples[index(y) * stride..],
            );
        }
    }
    true
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportBGR
/// Imports BGR rows of `bgr_stride` bytes into `picture` (C: `WebPPictureImportBGR`).
pub fn webp_picture_import_bgr(picture: &mut WebPPicture<'_>, bgr: &[u8], bgr_stride: i32) -> bool {
    import(picture, bgr, bgr_stride, 3, true, false)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportBGRA
/// Imports BGRA rows of `bgra_stride` bytes into `picture` (C: `WebPPictureImportBGRA`).
pub fn webp_picture_import_bgra(
    picture: &mut WebPPicture<'_>,
    bgra: &[u8],
    bgra_stride: i32,
) -> bool {
    import(picture, bgra, bgra_stride, 4, true, true)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportBGRX
/// Imports BGRX rows of `bgrx_stride` bytes into `picture`, ignoring the padding byte
/// (C: `WebPPictureImportBGRX`).
pub fn webp_picture_import_bgrx(
    picture: &mut WebPPicture<'_>,
    bgrx: &[u8],
    bgrx_stride: i32,
) -> bool {
    import(picture, bgrx, bgrx_stride, 4, true, false)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportRGB
/// Imports RGB rows of `rgb_stride` bytes into `picture` (C: `WebPPictureImportRGB`).
pub fn webp_picture_import_rgb(picture: &mut WebPPicture<'_>, rgb: &[u8], rgb_stride: i32) -> bool {
    import(picture, rgb, rgb_stride, 3, false, false)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportRGBA
/// Imports RGBA rows of `rgba_stride` bytes into `picture` (C: `WebPPictureImportRGBA`).
pub fn webp_picture_import_rgba(
    picture: &mut WebPPicture<'_>,
    rgba: &[u8],
    rgba_stride: i32,
) -> bool {
    import(picture, rgba, rgba_stride, 4, false, true)
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportRGBX
/// Imports RGBX rows of `rgbx_stride` bytes into `picture`, ignoring the padding byte
/// (C: `WebPPictureImportRGBX`).
pub fn webp_picture_import_rgbx(
    picture: &mut WebPPicture<'_>,
    rgbx: &[u8],
    rgbx_stride: i32,
) -> bool {
    import(picture, rgbx, rgbx_stride, 4, false, false)
}
