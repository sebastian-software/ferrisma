// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Transform orchestration and profile-usability operations from pinned
//! skcms.cc:1741-1810,2482-3143. Pixel execution uses the unchanged baseline
//! four-lane stage schedule; optimized HSW/SKX targets are separate boundaries.

use super::curve::Curve;
use super::fitting::{approximate_curve, are_approximate_inverses, max_roundtrip_error};
use super::profile::{self, IccProfile, SIGNATURE_CMYK, SIGNATURE_LAB};
use super::transfer_function::{TfType, TransferFunction as Tf, classify_raw, invert_raw, powf};
use super::transform_types::{AlphaFormat, Op, PixelFormat};
use super::types::{Matrix3x3, Matrix3x4};
use std::mem::{MaybeUninit, size_of};
use std::ptr;

#[derive(Clone, Copy)]
struct OpAndArg {
    op: Op,
    arg: *const (),
}
const NOOP: OpAndArg = OpAndArg {
    op: Op::load_a8,
    arg: ptr::null(),
};
unsafe fn tf_is_gamma(tf: *const Tf) -> bool {
    // SAFETY: the caller initializes only source-reached fields; raw loads keep
    // the original left-to-right conjunction and never form a full Tf value.
    unsafe {
        (*tf).g > 0.0
            && (*tf).a == 1.0
            && (*tf).b == 0.0
            && (*tf).c == 0.0
            && (*tf).d == 0.0
            && (*tf).e == 0.0
            && (*tf).f == 0.0
    }
}

unsafe fn select_curve_op(curve: *const Curve, channel: usize) -> OpAndArg {
    use Op::*;
    const OPS: [[Op; 6]; 4] = [
        [gamma_r, tf_r, pq_r, hlg_r, hlginv_r, table_r],
        [gamma_g, tf_g, pq_g, hlg_g, hlginv_g, table_g],
        [gamma_b, tf_b, pq_b, hlg_b, hlginv_b, table_b],
        [gamma_a, tf_a, pq_a, hlg_a, hlginv_a, table_a],
    ];
    // SAFETY: internal callers supply a valid active curve and channel 0..4.
    unsafe {
        if curve.cast::<u32>().read() == 0 {
            let p = ptr::addr_of!((*curve).parametric.function);
            if tf_is_gamma(p) {
                return if (*p).g != 1.0 {
                    OpAndArg {
                        op: OPS[channel][0],
                        arg: p.cast(),
                    }
                } else {
                    NOOP
                };
            }
            let index = match classify_raw(p) {
                TfType::Invalid | TfType::Pq | TfType::Hlg => return NOOP,
                TfType::Srgbish => 1,
                TfType::Pqish => 2,
                TfType::Hlgish => 3,
                TfType::HlgInvish => 4,
            };
            return OpAndArg {
                op: OPS[channel][index],
                arg: p.cast(),
            };
        }
        OpAndArg {
            op: OPS[channel][5],
            arg: curve.cast(),
        }
    }
}
unsafe fn bytes_equal(a: *const (), b: *const (), size: usize) -> bool {
    // SAFETY: caller establishes initialized readable bytes for this original memcmp.
    unsafe { (0..size).all(|i| a.cast::<u8>().add(i).read() == b.cast::<u8>().add(i).read()) }
}
unsafe fn select_curve_ops(curves: *const Curve, channels: i32, ops: &mut [OpAndArg; 4]) -> usize {
    let mut cursor = 0;
    let mut index = channels;
    loop {
        let previous = index;
        index = index.wrapping_sub(1);
        if previous <= 0 {
            break;
        }
        let i = index as usize;
        // SAFETY: source-defined positive counts cover at most four curves.
        // Negative counts read no curve/output; INT_MIN's original signed
        // decrement is source UB and remains outside the caller's contract.
        ops[cursor] = unsafe { select_curve_op(curves.add(i), i) };
        if !ops[cursor].arg.is_null() {
            cursor += 1;
        }
    }
    if cursor >= 3 {
        use Op::*;
        const FUSABLE: [[Op; 4]; 5] = [
            [gamma_r, gamma_g, gamma_b, gamma_rgb],
            [tf_r, tf_g, tf_b, tf_rgb],
            [pq_r, pq_g, pq_b, pq_rgb],
            [hlg_r, hlg_g, hlg_b, hlg_rgb],
            [hlginv_r, hlginv_g, hlginv_b, hlginv_rgb],
        ];
        let r = cursor - 1;
        let g = cursor - 2;
        let b = cursor - 3;
        for f in FUSABLE {
            // SAFETY: only transfer op contexts reach these original seven-word memcmp calls.
            if ops[r].op == f[0]
                && ops[g].op == f[1]
                && ops[b].op == f[2]
                && unsafe {
                    bytes_equal(ops[r].arg, ops[g].arg, size_of::<Tf>())
                        && bytes_equal(ops[r].arg, ops[b].arg, size_of::<Tf>())
                }
            {
                ops[b].op = f[3];
                cursor -= 2;
                break;
            }
        }
    }
    cursor
}
/// Original format width, ignoring the RGB/BGR low bit.
pub fn bytes_per_pixel(format: PixelFormat) -> usize {
    match format.0 >> 1 {
        0 | 1 => 1,
        2..=4 => 2,
        5 => 3,
        6..=8 => 4,
        9 | 11 | 13 | 15 => 6,
        10 | 12 | 14 | 16 | 20 => 8,
        17 => 12,
        18 => 16,
        19 => 4,
        _ => 0,
    }
}

unsafe fn has_cicp(profile: *const IccProfile, transfer: u8) -> bool {
    // SAFETY: the source reads the CICP transfer byte only behind its initialized flag.
    unsafe { (*profile).has_cicp && (*profile).cicp.transfer_characteristics == transfer }
}
#[expect(
    clippy::assign_op_pattern,
    reason = "Retain original scale-first operand order."
)]
fn reference_pq() -> Tf {
    let mut tf = Tf::default();
    tf.make_pqish(
        -107.0 / 128.0,
        1.0,
        32.0 / 2523.0,
        2413.0 / 128.0,
        -2392.0 / 128.0,
        8192.0 / 1305.0,
    );
    let ws = powf(10000.0 / 203.0, 1.0 / tf.f);
    tf.a = ws * tf.a;
    tf.b = ws * tf.b;
    tf
}
#[expect(
    clippy::excessive_precision,
    reason = "Retain original HLG decimal literal."
)]
fn sdr_hlg() -> Tf {
    let mut tf = Tf::default();
    tf.make_hlgish(2.0, 2.0, 1.0 / 0.17883277, 0.28466892, 0.55991073);
    tf.f = 1.0 / 12.0 - 1.0;
    tf
}
unsafe fn prep_for_destination(
    profile: *const IccProfile,
    from_xyz: &mut Matrix3x3,
    inverse: &mut [Tf; 3],
    using_b2a: &mut bool,
    using_hlg: &mut bool,
) -> bool {
    // SAFETY: every field is reached under its original initialized source flag.
    // The internal caller's output locals are disjoint from this profile.
    unsafe {
        let has_xyz = (*profile).has_to_xyz_d50
            && (&*ptr::addr_of!((*profile).to_xyz_d50)).invert_into(from_xyz);
        *using_b2a = false;
        *using_hlg = false;
        if has_cicp(profile, 16) && has_xyz {
            let tf = reference_pq();
            for out in inverse {
                tf.invert_into(out);
            }
            return true;
        }
        if has_cicp(profile, 18) && has_xyz {
            let tf = sdr_hlg();
            for out in inverse {
                tf.invert_into(out);
            }
            *using_hlg = true;
            return true;
        }
        if (*profile).has_b2a {
            *using_b2a = true;
            return true;
        }
        if !has_xyz || !(*profile).has_trc {
            return false;
        }
        for i in 0..3 {
            if ptr::addr_of!((*profile).trc[i]).cast::<u32>().read() != 0 {
                return false;
            }
        }
        for (i, out) in inverse.iter_mut().enumerate() {
            if !invert_raw(ptr::addr_of!((*profile).trc[i].parametric.function), out) {
                return false;
            }
        }
        true
    }
}
struct Program {
    ops: [Op; 32],
    contexts: [*const (); 32],
    len: usize,
}
impl Program {
    fn new() -> Self {
        Self {
            ops: [Op::load_a8; 32],
            contexts: [ptr::null(); 32],
            len: 0,
        }
    }
    fn add(&mut self, op: Op) {
        self.add_ctx(op, ptr::null());
    }
    fn add_ctx(&mut self, op: Op, arg: *const ()) {
        self.ops[self.len] = op;
        self.contexts[self.len] = arg;
        self.len += 1;
    }
    unsafe fn curves(&mut self, curves: *const Curve, channels: i32) {
        let mut ops = [NOOP; 4];
        // SAFETY: forwarded active contiguous curve contract; source permits <=4 channels.
        let count = unsafe { select_curve_ops(curves, channels, &mut ops) };
        for v in ops.iter().take(count) {
            self.add_ctx(v.op, v.arg);
        }
    }
}
static IDENTITY_3X4: Matrix3x4 = Matrix3x4 {
    vals: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ],
};

/// Original baseline skcms_Transform, including null-profile defaults, exact
/// same-address handling, format/alpha order and reached profile branches.
///
/// # Safety
/// Source and destination cover all reached pixels with the original typed
/// alignment. Aliases satisfy the original reached memcpy/load/store contract.
/// Non-null profiles initialize every reached flag and active field, and their
/// borrowed curve/grid storage stays readable. A profile may contain unknown
/// inactive bytes; no full profile Rust value is formed. Dimensions and reached
/// index/float-to-integer arithmetic must satisfy original defined behavior.
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve original transform signature."
)]
pub unsafe fn transform(
    src: *const u8,
    src_fmt: PixelFormat,
    mut src_alpha: AlphaFormat,
    mut src_profile: *const IccProfile,
    dst: *mut u8,
    dst_fmt: PixelFormat,
    mut dst_alpha: AlphaFormat,
    mut dst_profile: *const IccProfile,
    nz: usize,
) -> bool {
    use Op::*;
    use PixelFormat as P;
    let dst_bpp = bytes_per_pixel(dst_fmt);
    let src_bpp = bytes_per_pixel(src_fmt);
    if nz.wrapping_mul(dst_bpp) > i32::MAX as usize || nz.wrapping_mul(src_bpp) > i32::MAX as usize
    {
        return false;
    }
    let n = nz as i32;
    if src_profile.is_null() {
        src_profile = profile::srgb_profile();
    }
    if dst_profile.is_null() {
        dst_profile = profile::srgb_profile();
    }
    if dst.cast_const() == src && dst_bpp != src_bpp {
        return false;
    }
    let mut program = Program::new();
    let mut src_cicp = MaybeUninit::<Tf>::uninit();
    let mut dst_curves = [Curve::from_parametric(Tf::default()); 3];
    let mut dst_from_xyz = Matrix3x3::default();
    let mut dst_from_src = MaybeUninit::<Matrix3x3>::uninit();
    let src_index = src_fmt.0 >> 1;
    let dst_index = dst_fmt.0 >> 1;
    const LOADS: [Op; 21] = [
        load_a8,
        load_g8,
        load_ga88,
        load_565,
        load_4444,
        load_888,
        load_8888,
        load_8888,
        load_1010102,
        load_161616LE,
        load_16161616LE,
        load_161616BE,
        load_16161616BE,
        load_hhh,
        load_hhhh,
        load_hhh,
        load_hhhh,
        load_fff,
        load_ffff,
        load_101010x_XR,
        load_10101010_XR,
    ];
    const STORES: [Op; 21] = [
        store_a8,
        store_g8,
        store_ga88,
        store_565,
        store_4444,
        store_888,
        store_8888,
        store_8888,
        store_1010102,
        store_161616LE,
        store_16161616LE,
        store_161616BE,
        store_16161616BE,
        store_hhh,
        store_hhhh,
        store_hhh,
        store_hhhh,
        store_fff,
        store_ffff,
        store_101010x_XR,
        store_10101010_XR,
    ];
    let Some(&load) = LOADS.get(src_index as usize) else {
        return false;
    };
    program.add(load);
    if src_index == P::RGBA_8888_sRGB.0 >> 1 {
        program.add_ctx(tf_rgb, profile::srgb_transfer_function().cast());
    }
    if matches!(src_fmt, P::RGB_hhh_Norm | P::RGBA_hhhh_Norm) {
        program.add(clamp);
    }
    if src_fmt.0 & 1 != 0 {
        program.add(swap_rb);
    }
    let mut gray_dst = MaybeUninit::<IccProfile>::uninit();
    // SAFETY: the source traverses the same initialized flags/active fields.
    // Raw address formation and MaybeUninit byte copies preserve inactive bytes.
    unsafe {
        // This switch deliberately preserves the source's literal enum cases,
        // although the selector has already shifted the format by one.
        if dst_index == P::G_8.0 || dst_index == P::GA_88.0 {
            ptr::copy_nonoverlapping(
                dst_profile.cast::<MaybeUninit<u8>>(),
                gray_dst.as_mut_ptr().cast::<MaybeUninit<u8>>(),
                size_of::<IccProfile>(),
            );
            profile::set_xyz_d50(
                gray_dst.as_mut_ptr(),
                ptr::addr_of!((*profile::xyz_d50_profile()).to_xyz_d50),
            );
            dst_profile = gray_dst.as_ptr();
        }
        if (*src_profile).data_color_space == SIGNATURE_CMYK {
            program.add(invert);
            src_alpha = AlphaFormat::Unpremul;
        }
        if src_alpha == AlphaFormat::Opaque {
            program.add(force_opaque);
        } else if src_alpha == AlphaFormat::PremulAsEncoded {
            program.add(unpremul);
        }
        if dst_profile != src_profile {
            let mut src_a2b = false;
            let mut src_hlg = false;
            let mut dst_b2a = false;
            let mut dst_hlg = false;
            let mut inverse = [Tf::default(); 3];
            if !prep_for_destination(
                dst_profile,
                &mut dst_from_xyz,
                &mut inverse,
                &mut dst_b2a,
                &mut dst_hlg,
            ) {
                return false;
            }
            for i in 0..3 {
                dst_curves[i] = Curve::from_parametric(inverse[i]);
            }
            if has_cicp(src_profile, 16) && (*src_profile).has_to_xyz_d50 {
                src_cicp.write(reference_pq());
                program.add_ctx(pq_rgb, src_cicp.as_ptr().cast());
            } else if has_cicp(src_profile, 18) && (*src_profile).has_to_xyz_d50 {
                src_hlg = true;
                src_cicp.write(sdr_hlg());
                program.add_ctx(hlg_rgb, src_cicp.as_ptr().cast());
            } else if (*src_profile).has_a2b {
                src_a2b = true;
                let p = ptr::addr_of!((*src_profile).a2b);
                if (*p).input_channels != 0 {
                    program.curves(
                        ptr::addr_of!((*p).input_curves).cast(),
                        (*p).input_channels as i32,
                    );
                    program.add(clamp);
                    program.add_ctx(clut_A2B, p.cast());
                }
                if (*p).matrix_channels == 3 {
                    program.curves(ptr::addr_of!((*p).matrix_curves).cast(), 3);
                    if !bytes_equal(
                        ptr::addr_of!(IDENTITY_3X4).cast(),
                        ptr::addr_of!((*p).matrix).cast(),
                        size_of::<Matrix3x4>(),
                    ) {
                        program.add_ctx(matrix_3x4, ptr::addr_of!((*p).matrix).cast());
                    }
                }
                if (*p).output_channels == 3 {
                    program.curves(ptr::addr_of!((*p).output_curves).cast(), 3);
                }
                if (*src_profile).pcs == SIGNATURE_LAB {
                    program.add(lab_to_xyz);
                }
            } else if (*src_profile).has_trc && (*src_profile).has_to_xyz_d50 {
                program.curves(ptr::addr_of!((*src_profile).trc).cast(), 3);
            } else {
                return false;
            }
            if dst_b2a {
                if !src_a2b {
                    program.add_ctx(matrix_3x3, ptr::addr_of!((*src_profile).to_xyz_d50).cast());
                    if src_hlg {
                        program.add(hlg_ootf_scale);
                    }
                }
                if (*dst_profile).pcs == SIGNATURE_LAB {
                    program.add(xyz_to_lab);
                }
                let p = ptr::addr_of!((*dst_profile).b2a);
                if (*p).input_channels == 3 {
                    program.curves(ptr::addr_of!((*p).input_curves).cast(), 3);
                }
                if (*p).matrix_channels == 3 {
                    if !bytes_equal(
                        ptr::addr_of!(IDENTITY_3X4).cast(),
                        ptr::addr_of!((*p).matrix).cast(),
                        size_of::<Matrix3x4>(),
                    ) {
                        program.add_ctx(matrix_3x4, ptr::addr_of!((*p).matrix).cast());
                    }
                    program.curves(ptr::addr_of!((*p).matrix_curves).cast(), 3);
                }
                if (*p).output_channels != 0 {
                    program.add(clamp);
                    program.add_ctx(clut_B2A, p.cast());
                    program.curves(
                        ptr::addr_of!((*p).output_curves).cast(),
                        (*p).output_channels as i32,
                    );
                }
            } else {
                if src_hlg != dst_hlg {
                    if !src_a2b {
                        program
                            .add_ctx(matrix_3x3, ptr::addr_of!((*src_profile).to_xyz_d50).cast());
                    }
                    if src_hlg {
                        program.add(hlg_ootf_scale);
                    }
                    if dst_hlg {
                        program.add(hlginv_ootf_scale);
                    }
                    program.add_ctx(matrix_3x3, ptr::addr_of!(dst_from_xyz).cast());
                } else if src_a2b {
                    program.add_ctx(matrix_3x3, ptr::addr_of!(dst_from_xyz).cast());
                } else {
                    let to_xyz = ptr::addr_of!((*src_profile).to_xyz_d50);
                    if !bytes_equal(
                        ptr::addr_of!((*dst_profile).to_xyz_d50).cast(),
                        to_xyz.cast(),
                        size_of::<Matrix3x3>(),
                    ) {
                        dst_from_src.write(dst_from_xyz.concat(&*to_xyz));
                        program.add_ctx(matrix_3x3, dst_from_src.as_ptr().cast());
                    }
                }
                program.curves(dst_curves.as_ptr(), 3);
            }
        }
        if (dst_fmt.0) < P::RGB_hhh.0 {
            program.add(clamp);
        }
        if (*dst_profile).data_color_space == SIGNATURE_CMYK {
            program.add(invert);
            dst_alpha = AlphaFormat::Unpremul;
        }
        if dst_alpha == AlphaFormat::Opaque {
            program.add(force_opaque);
        } else if dst_alpha == AlphaFormat::PremulAsEncoded {
            program.add(premul);
        }
        if dst_fmt.0 & 1 != 0 {
            program.add(swap_rb);
        }
        if dst_index == P::RGBA_8888_sRGB.0 >> 1 {
            program.add_ctx(tf_rgb, profile::srgb_inverse_transfer_function().cast());
        }
        let Some(&store) = STORES.get(dst_index as usize) else {
            return false;
        };
        program.add(store);
        super::transform_simd::run_program(
            &program.ops[..program.len],
            &program.contexts[..program.len],
            src,
            dst,
            n,
            src_bpp,
            dst_bpp,
        );
    }
    true
}

/// Original three-TRC inverse check, retaining its null/flag short circuit.
///
/// # Safety
/// Reached profile fields and curve backing satisfy are_approximate_inverses;
/// the inverse initializes only fields reached by source classification/evaluation.
pub unsafe fn trcs_are_approximate_inverse(profile: *const IccProfile, inverse: *const Tf) -> bool {
    // SAFETY: null returns before access; each later call has the source's active curve contract.
    unsafe {
        if profile.is_null() || !(*profile).has_trc {
            return false;
        }
        for i in 0..3 {
            if !are_approximate_inverses(ptr::addr_of!((*profile).trc[i]), inverse) {
                return false;
            }
        }
        true
    }
}

/// Original in-place conversion of active profile curves into invertible TRCs.
/// Failure retains all original profile bytes; success commits in source order.
///
/// # Safety
/// Profile has writable storage and initializes reached flags/active fields.
/// Its table backing satisfies approximate_curve's source contract. Pointees
/// must not be concurrently mutated during reached reads.
pub unsafe fn make_usable_as_destination(profile: *mut IccProfile) -> bool {
    // SAFETY: every source field is reached under its original guard. Only
    // successfully obtained functions are committed, after all three succeed.
    unsafe {
        if !(*profile).has_b2a {
            let mut from_xyz = Matrix3x3::default();
            if !(*profile).has_trc
                || !(*profile).has_to_xyz_d50
                || !(&*ptr::addr_of!((*profile).to_xyz_d50)).invert_into(&mut from_xyz)
            {
                return false;
            }
            let mut tf = [Tf::default(); 3];
            for (i, out) in tf.iter_mut().enumerate() {
                let curve = ptr::addr_of!((*profile).trc[i]);
                let mut inverse = Tf::default();
                if curve.cast::<u32>().read() == 0
                    && invert_raw(ptr::addr_of!((*curve).parametric.function), &mut inverse)
                {
                    *out = ptr::addr_of!((*curve).parametric.function).read();
                    continue;
                }
                let mut error = 0.0;
                if !approximate_curve(curve, out, &mut error) {
                    return false;
                }
            }
            for (i, tf) in tf.into_iter().enumerate() {
                let curve = ptr::addr_of_mut!((*profile).trc[i]);
                curve.cast::<u32>().write(0);
                ptr::addr_of_mut!((*curve).parametric.function).write(tf);
            }
        }
    }
    true
}

/// Original single-curve destination conversion, including whole-object byte
/// copying and error selection against the original three curves.
///
/// # Safety
/// Profile satisfies make_usable_as_destination and each reached curve's
/// max_roundtrip_error preconditions. Original inactive bytes may be unknown;
/// their representations are copied without interpreting them.
pub unsafe fn make_usable_as_destination_with_single_curve(profile: *mut IccProfile) -> bool {
    let mut result = MaybeUninit::<IccProfile>::uninit();
    let p = result.as_mut_ptr();
    // SAFETY: byte copies use MaybeUninit for every inactive source byte; raw
    // accesses only reach flags and initialized active fields, as in the source.
    unsafe {
        ptr::copy_nonoverlapping(
            profile.cast::<MaybeUninit<u8>>(),
            p.cast::<MaybeUninit<u8>>(),
            size_of::<IccProfile>(),
        );
        ptr::addr_of_mut!((*p).has_b2a).write(false);
        if !make_usable_as_destination(p) {
            return false;
        }
        let mut best = 0usize;
        let mut min_error = f32::INFINITY;
        for i in 0..3 {
            let mut inverse = Tf::default();
            if !invert_raw(ptr::addr_of!((*p).trc[i].parametric.function), &mut inverse) {
                return false;
            }
            let mut error = 0.0;
            for j in 0..3 {
                let e = max_roundtrip_error(ptr::addr_of!((*profile).trc[j]), &inverse);
                error = if error > e { error } else { e };
            }
            if min_error > error {
                min_error = error;
                best = i;
            }
        }
        for i in 0..3 {
            let tf = ptr::addr_of!((*p).trc[best].parametric.function).read();
            ptr::addr_of_mut!((*p).trc[i].parametric.function).write(tf);
        }
        ptr::copy_nonoverlapping(
            p.cast::<MaybeUninit<u8>>(),
            profile.cast::<MaybeUninit<u8>>(),
            size_of::<IccProfile>(),
        );
    }
    true
}

const RANDOM_BYTES: [u8; 252] = [
    8, 179, 128, 204, 253, 38, 134, 184, 68, 102, 32, 138, 99, 39, 169, 215, 119, 26, 3, 223, 95,
    239, 52, 132, 114, 74, 81, 234, 97, 116, 244, 205, 30, 154, 173, 12, 51, 159, 122, 153, 61,
    226, 236, 178, 229, 55, 181, 220, 191, 194, 160, 126, 168, 82, 131, 18, 180, 245, 163, 22, 246,
    69, 235, 252, 57, 108, 14, 6, 152, 240, 255, 171, 242, 20, 227, 177, 238, 96, 85, 16, 211, 70,
    200, 149, 155, 146, 127, 145, 100, 151, 109, 19, 165, 208, 195, 164, 137, 254, 182, 248, 64,
    201, 45, 209, 5, 147, 207, 210, 113, 162, 83, 225, 9, 31, 15, 231, 115, 37, 58, 53, 24, 49,
    197, 56, 120, 172, 48, 21, 214, 129, 111, 11, 50, 187, 196, 34, 60, 103, 71, 144, 47, 203, 77,
    80, 232, 140, 222, 250, 206, 166, 247, 139, 249, 221, 72, 106, 27, 199, 117, 54, 219, 135, 118,
    40, 79, 41, 251, 46, 93, 212, 92, 233, 148, 28, 121, 63, 123, 158, 105, 59, 29, 42, 143, 23, 0,
    107, 176, 87, 104, 183, 156, 193, 189, 90, 188, 65, 190, 17, 198, 7, 186, 161, 1, 124, 78, 125,
    170, 133, 174, 218, 67, 157, 75, 101, 89, 217, 62, 33, 141, 228, 25, 35, 91, 230, 4, 2, 13, 73,
    86, 167, 237, 84, 243, 44, 185, 66, 130, 110, 150, 142, 216, 88, 112, 36, 224, 136, 202, 76,
    94, 98, 175, 213,
];

/// Original profile comparison through the two actual XYZ D50 transforms.
///
/// # Safety
/// Equal pointers short-circuit before access. Otherwise both whole profile
/// object representations must be readable and initialized for the original
/// memcmp. Reached active fields/backing satisfy transform's contract.
pub unsafe fn approximately_equal_profiles(a: *const IccProfile, b: *const IccProfile) -> bool {
    // SAFETY: caller establishes original byte-comparison and active-profile
    // contracts. Output arrays are fully written only after successful transforms.
    unsafe {
        if a == b || bytes_equal(a.cast(), b.cast(), size_of::<IccProfile>()) {
            return true;
        }
        let cmyk_a = (*a).data_color_space == SIGNATURE_CMYK;
        if cmyk_a != ((*b).data_color_space == SIGNATURE_CMYK) {
            return false;
        }
        let fmt = if cmyk_a {
            PixelFormat::RGBA_8888
        } else {
            PixelFormat::RGB_888
        };
        let n = if cmyk_a { 63 } else { 84 };
        let mut dst_a = [0u8; 252];
        let mut dst_b = [0u8; 252];
        if !transform(
            RANDOM_BYTES.as_ptr(),
            fmt,
            AlphaFormat::Unpremul,
            a,
            dst_a.as_mut_ptr(),
            PixelFormat::RGB_888,
            AlphaFormat::Unpremul,
            profile::xyz_d50_profile(),
            n,
        ) {
            return false;
        }
        if !transform(
            RANDOM_BYTES.as_ptr(),
            fmt,
            AlphaFormat::Unpremul,
            b,
            dst_b.as_mut_ptr(),
            PixelFormat::RGB_888,
            AlphaFormat::Unpremul,
            profile::xyz_d50_profile(),
            n,
        ) {
            return false;
        }
        for i in 0..252 {
            if (dst_a[i] as i32 - dst_b[i] as i32).abs() > 1 {
                return false;
            }
        }
    }
    true
}

/// Data-only access to actual private curve selection for the opt-in oracle.
///
/// # Safety
/// Positive counts cover at most four initialized active curves. Counts <= 0
/// access no curve. INT_MIN is excluded by the original signed decrement UB.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_curve_ops(curves: *const Curve, channels: i32) -> Vec<(i32, isize)> {
    let mut ops = [NOOP; 4];
    // SAFETY: forwarded internal curve-selection contract, with no replacement algorithm.
    let count = unsafe { select_curve_ops(curves, channels, &mut ops) };
    ops[..count]
        .iter()
        .map(|x| {
            (
                x.op as i32,
                (x.arg as usize).wrapping_sub(curves as usize) as isize,
            )
        })
        .collect()
}
/// Data-only access to the original private destination preparation for the oracle.
///
/// # Safety
/// Profile and disjoint output locals satisfy the internal destination contract.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_prep(
    profile: *const IccProfile,
    matrix: &mut Matrix3x3,
    inverse: &mut [Tf; 3],
    b2a: &mut bool,
    hlg: &mut bool,
) -> bool {
    // SAFETY: forwarded reached-field and disjoint-output contract.
    unsafe { prep_for_destination(profile, matrix, inverse, b2a, hlg) }
}
/// Data-only access to the two actual private CICP function constructors.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub fn oracle_cicp(hlg: bool) -> Tf {
    if hlg { sdr_hlg() } else { reference_pq() }
}
/// Executes the actual baseline stage engine for the opt-in differential oracle.
///
/// # Safety
/// Every opcode is an original valid Op, the program ends with a store, and
/// context/pixel storage satisfies the original reached stage preconditions.
#[cfg(feature = "cpp-oracle")]
#[doc(hidden)]
pub unsafe fn oracle_program(
    ops: &[i32],
    contexts: &[*const ()],
    src: *const u8,
    dst: *mut u8,
    n: i32,
    src_bpp: usize,
    dst_bpp: usize,
) {
    let ops: Vec<Op> = ops
        .iter()
        .map(|&op| {
            assert!((0..=Op::store_ffff as i32).contains(&op));
            // SAFETY: this is the complete contiguous repr(i32) source opcode range.
            unsafe { std::mem::transmute::<i32, Op>(op) }
        })
        .collect();
    // SAFETY: forwarded exact program/context/pixel contract, without a second implementation.
    unsafe {
        super::transform_simd::run_program(&ops, contexts, src, dst, n, src_bpp, dst_bpp);
    }
}
