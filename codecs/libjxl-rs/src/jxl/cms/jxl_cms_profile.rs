// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The selected skcms profile-identification branch of pinned
//! `lib/jxl/cms/jxl_cms.cc`:334-339,396-403,422-716,1209-1374.
//! Raw profiles retain source short-circuit reads of initialized active fields.

use super::color_encoding_cms::{
    CieXy, ColorEncoding, ColorSpace, Primaries, PrimariesCieXy, RenderingIntent, TransferFunction,
    WhitePoint,
};
use super::icc::{cie_xyz_from_white_cie_xy, maybe_create_profile};
use crate::jxl::base::float::mul_add_f32;
use crate::jxl::base::matrix_ops::inv3x3_matrix;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::color_encoding::JxlColorEncoding;
use crate::skcms::fitting::approximate_curve;
use crate::skcms::profile::{
    self, IccProfile, SIGNATURE_CMYK, SIGNATURE_GRAY, SIGNATURE_RGB, SIGNATURE_XYZ,
};
use crate::skcms::transfer_function::TransferFunction as SkTransferFunction;
use crate::skcms::transform::{approximately_equal_profiles, transform};
use crate::skcms::transform_types::{AlphaFormat, PixelFormat};
use crate::skcms::types::Matrix3x3;
use std::ffi::{c_int, c_void};
use std::mem::MaybeUninit;
use std::ptr;

pub(super) fn cie_xy_from_xyz(xyz: [f32; 3]) -> CieXy {
    let factor = 1.0 / f64::from(xyz[0] + xyz[1] + xyz[2]);
    #[cfg(target_arch = "aarch64")]
    {
        let x: f64;
        let y: f64;
        // SAFETY: original scalar double FMULs select factor as their first
        // operand. Preserve that order for competing NaN payloads; the block
        // accesses no memory and reads initialized float register operands.
        unsafe {
            std::arch::asm!(
                "fmul {out_x:d}, {factor:d}, {x:d}",
                "fmul {out_y:d}, {factor:d}, {y:d}",
                factor = in(vreg) factor,
                x = in(vreg) f64::from(xyz[0]),
                y = in(vreg) f64::from(xyz[1]),
                out_x = out(vreg) x,
                out_y = out(vreg) y,
                options(pure, nomem, nostack),
            );
        }
        CieXy { x, y }
    }
    #[cfg(not(target_arch = "aarch64"))]
    CieXy {
        x: f64::from(xyz[0]) * factor,
        y: f64::from(xyz[1]) * factor,
    }
}

/// Source assignment/accumulation order, including aliased input/output.
/// The caller provides valid matrix/input/output storage for all reached words.
pub(super) unsafe fn matrix_product(matrix: *const Matrix3x3, input: *const f32, output: *mut f32) {
    // SAFETY: caller supplies all nine matrix words and three input/output words.
    // Raw accesses preserve the original write-before-read order under aliasing.
    unsafe {
        for i in 0..3 {
            output.add(i).write(0.0);
            for j in 0..3 {
                let value = mul_add_f32(
                    (*matrix).vals[i][j],
                    input.add(j).read(),
                    output.add(i).read(),
                );
                output.add(i).write(value);
            }
        }
    }
}

pub(super) unsafe fn decode_profile(
    bytes: *const u8,
    size: usize,
    output: *mut IccProfile,
) -> bool {
    // SAFETY: forwards the original parser's reached-byte/output contract.
    unsafe { profile::parse(bytes, size, output) }
}

pub(super) unsafe fn color_space_from_profile(p: *const IccProfile) -> ColorSpace {
    // SAFETY: only the source-reached data-color-space word must be initialized.
    match unsafe { ptr::addr_of!((*p).data_color_space).read() } {
        SIGNATURE_RGB | SIGNATURE_CMYK => ColorSpace::RGB,
        SIGNATURE_GRAY => ColorSpace::GRAY,
        _ => ColorSpace::UNKNOWN,
    }
}

pub(super) fn is_acceptable_chad(chad: &Matrix3x3) -> bool {
    let mut double = [[0.0; 3]; 3];
    for (i, row) in double.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            *value = f64::from(chad.vals[i][j]);
        }
    }
    inv3x3_matrix(&mut double).is_ok()
}

pub(super) unsafe fn has_icc_version4_display_class(p: *const IccProfile) -> bool {
    // SAFETY: caller initializes source-reached buffer/size and valid buffer
    // bytes. Later fields/bytes are read only after each original prefix guard.
    unsafe {
        let buffer = ptr::addr_of!((*p).buffer).read();
        !buffer.is_null()
            && (*p).size >= 128
            && *buffer.add(8) == 4
            && *buffer.add(12) == b'm'
            && *buffer.add(13) == b'n'
            && *buffer.add(14) == b't'
            && *buffer.add(15) == b'r'
            && *buffer.add(36) == b'a'
            && *buffer.add(37) == b'c'
            && *buffer.add(38) == b's'
            && *buffer.add(39) == b'p'
    }
}

pub(super) unsafe fn has_tag(p: *const IccProfile, signature: *const u8) -> bool {
    // SAFETY: source-reached header and bounded tag bytes are readable. The
    // signature's four bytes are needed only when an actual tag is compared.
    unsafe {
        let buffer = ptr::addr_of!((*p).buffer).read();
        if buffer.is_null() || (*p).size < 132 {
            return false;
        }
        let count = (u32::from(*buffer.add(128)) << 24)
            | (u32::from(*buffer.add(129)) << 16)
            | (u32::from(*buffer.add(130)) << 8)
            | u32::from(*buffer.add(131));
        if count as usize > ((*p).size as usize - 132) / 12 {
            return false;
        }
        for i in 0..count as usize {
            let offset = 132 + 12 * i;
            let mut same = true;
            for j in 0..4 {
                same &= *buffer.add(offset + j) == *signature.add(j);
            }
            if same {
                return true;
            }
        }
        false
    }
}

pub(super) unsafe fn has_lut_tags(p: *const IccProfile) -> bool {
    for signature in [
        b"A2B0", b"A2B1", b"A2B2", b"B2A0", b"B2A1", b"B2A2", b"D2B0", b"D2B1", b"D2B2", b"D2B3",
        b"B2D0", b"B2D1", b"B2D2", b"B2D3",
    ] {
        // SAFETY: inherits has_tag's raw profile contract; literals have four bytes.
        if unsafe { has_tag(p, signature.as_ptr()) } {
            return true;
        }
    }
    false
}

pub(super) unsafe fn has_parametric_trcs(p: *const IccProfile) -> bool {
    // SAFETY: only the three source-reached union count prefixes are read.
    unsafe {
        (*p).trc[0].table.table_entries == 0
            && (*p).trc[1].table.table_entries == 0
            && (*p).trc[2].table.table_entries == 0
    }
}

#[expect(
    clippy::excessive_precision,
    reason = "Retain the pinned source D50 literals."
)]
pub(super) unsafe fn is_approximately_d50(xyz: *const f32) -> bool {
    // SAFETY: only the source-reached prefix is required initialized. Do not
    // materialize later words after an earlier ordered comparison fails.
    unsafe {
        (*xyz - 0.96420288_f32).abs() < 1e-3
            && (*xyz.add(1) - 1.0).abs() < 1e-3
            && (*xyz.add(2) - 0.82490540_f32).abs() < 1e-3
    }
}

pub(super) unsafe fn get_nonconforming_matrix_profile_white_point(
    p: *const IccProfile,
    media: *const f32,
    adapted: *mut f32,
) -> bool {
    // SAFETY: all fields/backing reached after each source guard are valid;
    // output requires three writable words only on the successful branch.
    unsafe {
        if !has_icc_version4_display_class(p)
            || (*p).data_color_space != SIGNATURE_RGB
            || (*p).pcs != SIGNATURE_XYZ
            || !(*p).has_trc
            || !(*p).has_to_xyz_d50
            || !has_tag(p, b"chad".as_ptr())
            || has_lut_tags(p)
            || is_approximately_d50(media)
        {
            return false;
        }
        let mut neutral = [0.0; 3];
        for (i, v) in neutral.iter_mut().enumerate() {
            *v = (*p).to_xyz_d50.vals[i][0]
                + (*p).to_xyz_d50.vals[i][1]
                + (*p).to_xyz_d50.vals[i][2];
        }
        if !is_approximately_d50(neutral.as_ptr()) {
            return false;
        }
        ptr::copy_nonoverlapping(neutral.as_ptr(), adapted, 3);
        true
    }
}

pub(super) unsafe fn recover_matrix_profile_white_point(
    p: *const IccProfile,
    media: *const f32,
    adapted: *mut f32,
) -> bool {
    let mut chad = MaybeUninit::<Matrix3x3>::uninit();
    // SAFETY: inherited reached-profile contract; get_chad initializes its
    // complete matrix before the final predicate. Failure retains adapted writes.
    unsafe {
        has_parametric_trcs(p)
            && get_nonconforming_matrix_profile_white_point(p, media, adapted)
            && profile::get_chad(p, chad.as_mut_ptr())
            && is_acceptable_chad(&chad.assume_init())
    }
}

pub(super) unsafe fn unadapted_white_point(p: *const IccProfile, output: *mut CieXy) -> Status {
    let mut media = [0.0; 3];
    let mut chad = Matrix3x3::default();
    // SAFETY: inherited profile/tag backing contract. Locals are disjoint, and
    // the output is written only after every original successful prerequisite.
    unsafe {
        if !profile::get_wtpt(p, media.as_mut_ptr()) {
            return Err(StatusCode::GenericError);
        }
        if !profile::get_chad(p, &mut chad) {
            output.write(cie_xy_from_xyz(media));
            return Ok(());
        }
        let mut adapted = [0.0; 3];
        if recover_matrix_profile_white_point(p, media.as_ptr(), adapted.as_mut_ptr()) {
            media = adapted;
        }
        let mut inverse = Matrix3x3::default();
        if !chad.invert_into(&mut inverse) {
            return Err(StatusCode::GenericError);
        }
        let mut unadapted = [0.0; 3];
        matrix_product(&inverse, media.as_ptr(), unadapted.as_mut_ptr());
        output.write(cie_xy_from_xyz(unadapted));
        Ok(())
    }
}

#[expect(
    clippy::excessive_precision,
    reason = "Retain the pinned source D50 literals."
)]
pub(super) unsafe fn identify_primaries(
    p: *const IccProfile,
    white: CieXy,
    c: &mut ColorEncoding,
) -> Status {
    if !c.has_primaries() {
        return Ok(());
    }
    let mut chad = Matrix3x3::default();
    let mut inverse = Matrix3x3::default();
    // SAFETY: inherited source profile contract. The source requires all
    // XYZ words to have been initialized before each following matrix read; a
    // failed transform retains any words initialized by an earlier iteration.
    unsafe {
        if profile::get_chad(p, &mut chad) {
            if !chad.invert_into(&mut inverse) {
                return Err(StatusCode::GenericError);
            }
        } else {
            const LMS_FROM_XYZ: Matrix3x3 = Matrix3x3 {
                vals: [
                    [0.8951, 0.2664, -0.1614],
                    [-0.7502, 1.7135, 0.0367],
                    [0.0389, -0.0685, 1.0296],
                ],
            };
            const XYZ_FROM_LMS: Matrix3x3 = Matrix3x3 {
                vals: [
                    [0.9869929, -0.1470543, 0.1599627],
                    [0.4323053, 0.5183603, 0.0492912],
                    [-0.0085287, 0.0400428, 0.9684867],
                ],
            };
            let d50 = [0.96420288_f32, 1.0, 0.82490540_f32];
            let mut xyz = [0.0; 3];
            cie_xyz_from_white_cie_xy(white.x, white.y, &mut xyz)?;
            let mut d50_lms = [0.0; 3];
            let mut white_lms = [0.0; 3];
            matrix_product(&LMS_FROM_XYZ, d50.as_ptr(), d50_lms.as_mut_ptr());
            matrix_product(&LMS_FROM_XYZ, xyz.as_ptr(), white_lms.as_mut_ptr());
            inverse.vals = [
                [white_lms[0] / d50_lms[0], 0.0, 0.0],
                [0.0, white_lms[1] / d50_lms[1], 0.0],
                [0.0, 0.0, white_lms[2] / d50_lms[2]],
            ];
            inverse = XYZ_FROM_LMS.concat(&inverse);
            inverse = inverse.concat(&LMS_FROM_XYZ);
        }
        let mut xyz = MaybeUninit::<[f32; 3]>::uninit();
        let mut points = [CieXy::default(); 3];
        for i in 0..3 {
            let mut rgb = [0.0_f32; 3];
            rgb[i] = 1.0;
            let _ = transform(
                rgb.as_ptr().cast(),
                PixelFormat::RGB_fff,
                AlphaFormat::Opaque,
                p,
                xyz.as_mut_ptr().cast(),
                PixelFormat::RGB_fff,
                AlphaFormat::Opaque,
                profile::xyz_d50_profile(),
                1,
            );
            let mut unadapted = [0.0; 3];
            matrix_product(&inverse, xyz.as_ptr().cast(), unadapted.as_mut_ptr());
            points[i] = cie_xy_from_xyz(unadapted);
        }
        c.set_primaries(PrimariesCieXy {
            r: points[0],
            g: points[1],
            b: points[2],
        })
    }
}

pub(super) unsafe fn is_approximately_equal(p: *const IccProfile, c: &ColorEncoding) -> bool {
    let mut bytes = Vec::new();
    if maybe_create_profile(&c.to_external(), &mut bytes).is_err() {
        return false;
    }
    let mut test = MaybeUninit::<IccProfile>::uninit();
    // SAFETY: bytes live through the comparison and parser initializes the
    // complete raw representation. Caller supplies the original profile's bytes.
    unsafe {
        decode_profile(bytes.as_ptr(), bytes.len(), test.as_mut_ptr())
            && approximately_equal_profiles(test.as_ptr(), p)
    }
}

unsafe fn is_gamma(tf: *const SkTransferFunction) -> bool {
    // SAFETY: preserve the original a,b,d,e,f short-circuit sequence; c and g
    // are not needed by this predicate, nor later fields after an earlier miss.
    unsafe {
        (*tf).a == 1.0 && (*tf).b == 0.0 && (*tf).d == 0.0 && (*tf).e == 0.0 && (*tf).f == 0.0
    }
}

pub(super) unsafe fn detect_transfer_function(
    p: *const IccProfile,
    c: &mut ColorEncoding,
) -> Status {
    if c.color_space == ColorSpace::XYB {
        return Err(StatusCode::GenericError);
    }
    // SAFETY: inherited reached-profile/curve/backing contract; temporary
    // approximations are read only after successful complete initialization.
    unsafe {
        let mut media = [0.0; 3];
        let mut adapted = [0.0; 3];
        if !has_parametric_trcs(p)
            && profile::get_wtpt(p, media.as_mut_ptr())
            && get_nonconforming_matrix_profile_white_point(p, media.as_ptr(), adapted.as_mut_ptr())
        {
            c.tf.set_transfer_function(TransferFunction::UNKNOWN);
            return Ok(());
        }
        let mut gamma = [0.0_f32; 3];
        if (*p).has_trc {
            for (i, g) in gamma.iter_mut().enumerate() {
                let curve = ptr::addr_of!((*p).trc)
                    .cast::<crate::skcms::curve::Curve>()
                    .add(i);
                // The actual source uses the first curve's function here.
                let first = ptr::addr_of!((*p).trc[0].parametric.function);
                if (*curve).table.table_entries == 0 && is_gamma(first) {
                    *g = 1.0 / (*first).g;
                } else {
                    let mut approx = MaybeUninit::<SkTransferFunction>::uninit();
                    let mut error = MaybeUninit::<f32>::uninit();
                    if approximate_curve(curve, approx.as_mut_ptr(), error.as_mut_ptr())
                        && is_gamma(approx.as_ptr())
                    {
                        *g = 1.0 / (*approx.as_ptr()).g;
                    }
                }
            }
        }
        if gamma[0] != 0.0
            && (gamma[0] - gamma[1]).abs() < 1e-4
            && (gamma[1] - gamma[2]).abs() < 1e-4
            && c.tf.set_gamma(f64::from(gamma[0])).is_ok()
            && is_approximately_equal(p, c)
        {
            return Ok(());
        }
        // Original Values<TransferFunction> order follows its numeric bit mask.
        for tf in [
            TransferFunction::BT709,
            TransferFunction::LINEAR,
            TransferFunction::SRGB,
            TransferFunction::PQ,
            TransferFunction::DCI,
            TransferFunction::HLG,
        ] {
            c.tf.set_transfer_function(tf);
            if is_approximately_equal(p, c) {
                return Ok(());
            }
        }
        c.tf.set_transfer_function(TransferFunction::UNKNOWN);
        Ok(())
    }
}

pub(super) fn is_known_transfer_function(tf: TransferFunction) -> bool {
    matches!(
        tf,
        TransferFunction::BT709
            | TransferFunction::LINEAR
            | TransferFunction::SRGB
            | TransferFunction::PQ
            | TransferFunction::DCI
            | TransferFunction::HLG
    )
}
pub(super) fn is_known_color_primaries(p: u8) -> bool {
    p == 12
        || matches!(
            Primaries(u32::from(p)),
            Primaries::SRGB | Primaries::BT2100 | Primaries::P3
        )
}
pub(super) unsafe fn apply_cicp(
    primaries: u8,
    transfer: u8,
    matrix: u8,
    range: u8,
    c: *mut ColorEncoding,
) -> bool {
    if matrix != 0 || range != 1 {
        return false;
    }
    let p = Primaries(u32::from(primaries));
    let tf = TransferFunction(u32::from(transfer));
    if !is_known_transfer_function(tf) || !is_known_color_primaries(primaries) {
        return false;
    }
    // SAFETY: only successful guards require the selected writable output
    // fields. The source writes without reading the old encoding or TF gamma;
    // a failed guard permits even a null output with no access.
    unsafe {
        ptr::addr_of_mut!((*c).color_space).write(ColorSpace::RGB);
        ptr::addr_of_mut!((*c).tf.have_gamma).write(false);
        ptr::addr_of_mut!((*c).tf.transfer_function).write(tf);
        if p == Primaries::P3 {
            ptr::addr_of_mut!((*c).white_point).write(WhitePoint::DCI);
            ptr::addr_of_mut!((*c).primaries).write(Primaries::P3);
        } else if primaries == 12 {
            ptr::addr_of_mut!((*c).white_point).write(WhitePoint::D65);
            ptr::addr_of_mut!((*c).primaries).write(Primaries::P3);
        } else {
            ptr::addr_of_mut!((*c).white_point).write(WhitePoint::D65);
            ptr::addr_of_mut!((*c).primaries).write(p);
        }
    }
    true
}

/// The original default CMS ICC-field callback, including partial failure writes.
///
/// # Safety
/// Non-null outputs permit the original reached writes. Nonzero ICC sizes have
/// readable backing for the parser's reached bytes, live throughout the call.
/// Every XYZ word consumed after an ignored-status basis transform must have
/// been initialized by a current or earlier iteration, as in the source.
pub(super) unsafe extern "C" fn set_fields_from_icc(
    _data: *mut c_void,
    bytes: *const u8,
    size: usize,
    c: *mut JxlColorEncoding,
    cmyk: *mut c_int,
) -> c_int {
    if c.is_null() || cmyk.is_null() {
        return 0;
    }
    // SAFETY: caller supplies reached output fields and ICC backing. No whole
    // external encoding is read before its successful complete assignment.
    unsafe {
        cmyk.write(0);
        ptr::addr_of_mut!((*c).color_space).write(ColorSpace::UNKNOWN);
        ptr::addr_of_mut!((*c).transfer_function).write(TransferFunction::UNKNOWN);
        if size == 0 {
            return 0;
        }
        let mut encoding = ColorEncoding::default();
        if size < 128 {
            return 0;
        }
        let mut profile = MaybeUninit::<IccProfile>::uninit();
        if !profile::parse(bytes, size, profile.as_mut_ptr()) {
            return 0;
        }
        let p = profile.as_ptr();
        let be = u32::from(*bytes.add(67))
            + (u32::from(*bytes.add(66)) << 8)
            + (u32::from(*bytes.add(65)) << 16)
            + (u32::from(*bytes.add(64)) << 24);
        let le = (u32::from(*bytes.add(67)) << 24)
            + (u32::from(*bytes.add(66)) << 16)
            + (u32::from(*bytes.add(65)) << 8)
            + u32::from(*bytes.add(64));
        let intent = be.min(le);
        if intent > 3 {
            return 0;
        }
        encoding.rendering_intent = RenderingIntent(intent);
        if (*p).has_cicp {
            let cicp = ptr::addr_of!((*p).cicp).read();
            if apply_cicp(
                cicp.color_primaries,
                cicp.transfer_characteristics,
                cicp.matrix_coefficients,
                cicp.video_full_range_flag,
                &mut encoding,
            ) {
                c.write(encoding.to_external());
                return 1;
            }
        }
        encoding.color_space = color_space_from_profile(p);
        cmyk.write(c_int::from((*p).data_color_space == SIGNATURE_CMYK));
        let mut white = CieXy::default();
        if unadapted_white_point(p, &mut white).is_err()
            || encoding.set_white_point(white).is_err()
            || identify_primaries(p, white, &mut encoding).is_err()
            || detect_transfer_function(p, &mut encoding).is_err()
        {
            return 0;
        }
        c.write(encoding.to_external());
        1
    }
}
