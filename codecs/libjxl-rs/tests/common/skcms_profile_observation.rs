// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::skcms::curve::{Curve, TableCurve};
use jxl::skcms::profile::IccProfile;
use jxl::skcms::transfer_function::TransferFunction;
use std::fmt::Write;
use std::mem::{offset_of, size_of};
pub(super) fn pointer(out: &mut String, pointer: *const u8, base: *const u8) {
    let offset = if pointer.is_null() {
        u64::MAX
    } else {
        (pointer as usize).wrapping_sub(base as usize) as u64
    };
    write!(out, " {offset}").unwrap();
}
pub(super) fn function(out: &mut String, function: &TransferFunction) {
    for x in [
        function.g, function.a, function.b, function.c, function.d, function.e, function.f,
    ] {
        write!(out, " {}", x.to_bits()).unwrap();
    }
}
pub(super) fn curve(out: &mut String, curve: &Curve, base: *const u8) {
    // SAFETY: every observer curve is initialized by source-equivalent memset,
    // static construction or an explicit active-branch fixture. Count selects
    // exactly the initialized source branch; no padding bytes are read.
    unsafe {
        let count = curve.table.table_entries;
        write!(out, " {count}").unwrap();
        if count == 0 {
            function(out, &curve.parametric.function);
        } else {
            pointer(out, curve.table.table_8, base);
            pointer(out, curve.table.table_16, base);
            // SAFETY: fixtures initialize every union byte before source-style
            // field stores. Pointer representations are omitted and normalized
            // separately; every retained non-pointer word stays initialized.
            for offset in (4..size_of::<Curve>()).step_by(4) {
                if (offset_of!(TableCurve, table_8)
                    ..offset_of!(TableCurve, table_8) + size_of::<*const u8>())
                    .contains(&offset)
                    || (offset_of!(TableCurve, table_16)
                        ..offset_of!(TableCurve, table_16) + size_of::<*const u8>())
                        .contains(&offset)
                {
                    continue;
                }
                let word = (curve as *const Curve)
                    .cast::<u8>()
                    .add(offset)
                    .cast::<u32>()
                    .read_unaligned();
                write!(out, " {word}").unwrap();
            }
        }
    }
}
pub(super) fn curves(out: &mut String, curves: &[Curve], base: *const u8) {
    for c in curves {
        curve(out, c, base);
    }
}
pub(super) fn matrix<const N: usize>(out: &mut String, matrix: &[[f32; N]; 3]) {
    for row in matrix {
        for x in row {
            write!(out, " {}", x.to_bits()).unwrap();
        }
    }
}
pub(super) fn profile(out: &mut String, p: &IccProfile, base: *const u8) {
    write!(
        out,
        "{} {} {} {} {}",
        u8::from(p.has_trc),
        u8::from(p.has_to_xyz_d50),
        u8::from(p.has_a2b),
        u8::from(p.has_b2a),
        u8::from(p.has_cicp)
    )
    .unwrap();
    pointer(out, p.buffer, base);
    write!(
        out,
        " {} {} {} {}",
        p.size, p.data_color_space, p.pcs, p.tag_count
    )
    .unwrap();
    curves(out, &p.trc, base);
    matrix(out, &p.to_xyz_d50.vals);
    let a = &p.a2b;
    curves(out, &a.input_curves, base);
    pointer(out, a.grid_8, base);
    pointer(out, a.grid_16, base);
    write!(out, " {}", a.input_channels).unwrap();
    for x in a.grid_points {
        write!(out, " {x}").unwrap();
    }
    curves(out, &a.matrix_curves, base);
    matrix(out, &a.matrix.vals);
    write!(out, " {} {}", a.matrix_channels, a.output_channels).unwrap();
    curves(out, &a.output_curves, base);
    let b = &p.b2a;
    curves(out, &b.input_curves, base);
    write!(out, " {} {}", b.input_channels, b.matrix_channels).unwrap();
    curves(out, &b.matrix_curves, base);
    matrix(out, &b.matrix.vals);
    curves(out, &b.output_curves, base);
    pointer(out, b.grid_8, base);
    pointer(out, b.grid_16, base);
    for x in b.grid_points {
        write!(out, " {x}").unwrap();
    }
    write!(
        out,
        " {} {} {} {} {}",
        b.output_channels,
        p.cicp.color_primaries,
        p.cicp.transfer_characteristics,
        p.cicp.matrix_coefficients,
        p.cicp.video_full_range_flag
    )
    .unwrap();
}
