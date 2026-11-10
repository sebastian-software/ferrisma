// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Matrix, tone-map pixel, and table-curve helpers from
//! `lib/jxl/cms/jxl_cms_internal.h`. Preserve mixed float/double conversions.

use super::super::base::float::{clamp_f32, clamp_f64, mul_add_f32};
use super::super::base::matrix_ops::{Matrix3x3, inv3x3_matrix, mul3x3_matrix, mul3x3_vector};
use super::super::base::status::{Status, StatusCode};
use super::super::color_encoding::{JxlColorEncoding, TransferFunction};
use super::tone_mapping::{Color, HlgOotfBase, Rec2408ToneMapperBase, gamut_map_scalar};
use super::transfer_functions::{TfHlgBase, TfPqBase};

#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the source matrix boundary and coordinate order."
)]
pub fn primaries_to_xyz(
    rx: f32,
    ry: f32,
    gx: f32,
    gy: f32,
    bx: f32,
    by: f32,
    wx: f32,
    wy: f32,
    matrix: &mut Matrix3x3,
) -> Status {
    if !((0.0..=1.0).contains(&wx) && wy > 0.0 && wy <= 1.0) {
        return Err(StatusCode::GenericError);
    }
    let primaries = [
        [rx, gx, bx],
        [ry, gy, by],
        [1.0 - rx - ry, 1.0 - gx - gy, 1.0 - bx - by],
    ];
    let mut inverse = primaries;
    inv3x3_matrix(&mut inverse)?;
    let w = [wx / wy, 1.0, (1.0 - wx - wy) / wy];
    if !w[0].is_finite() || !w[2].is_finite() {
        return Err(StatusCode::GenericError);
    }
    let mut xyz = [0.0; 3];
    mul3x3_vector(&inverse, &w, &mut xyz);
    let a = [[xyz[0], 0.0, 0.0], [0.0, xyz[1], 0.0], [0.0, 0.0, xyz[2]]];
    mul3x3_matrix(&primaries, &a, matrix);
    Ok(())
}
const BRADFORD: Matrix3x3 = [
    [0.8951, 0.2664, -0.1614],
    [-0.7502, 1.7135, 0.0367],
    [0.0389, -0.0685, 1.0296],
];
const BRADFORD_INV: Matrix3x3 = [
    [0.9869929, -0.1470543, 0.1599627],
    [0.4323053, 0.5183603, 0.0492912],
    [-0.0085287, 0.0400428, 0.9684867],
];
pub fn adapt_to_xyz_d50(wx: f32, wy: f32, matrix: &mut Matrix3x3) -> Status {
    if !((0.0..=1.0).contains(&wx) && wy > 0.0 && wy <= 1.0) {
        return Err(StatusCode::GenericError);
    }
    let w = [wx / wy, 1.0, (1.0 - wx - wy) / wy];
    if !w[0].is_finite() || !w[2].is_finite() {
        return Err(StatusCode::GenericError);
    }
    let w50 = [0.96422, 1.0, 0.82521];
    let mut lms = [0.0; 3];
    let mut lms50 = [0.0; 3];
    mul3x3_vector(&BRADFORD, &w, &mut lms);
    mul3x3_vector(&BRADFORD, &w50, &mut lms50);
    if lms.contains(&0.0) {
        return Err(StatusCode::GenericError);
    }
    let a = [
        [lms50[0] / lms[0], 0.0, 0.0],
        [0.0, lms50[1] / lms[1], 0.0],
        [0.0, 0.0, lms50[2] / lms[2]],
    ];
    if !a[0][0].is_finite() || !a[1][1].is_finite() || !a[2][2].is_finite() {
        return Err(StatusCode::GenericError);
    }
    let mut b = [[0.0; 3]; 3];
    mul3x3_matrix(&a, &BRADFORD, &mut b);
    mul3x3_matrix(&BRADFORD_INV, &b, matrix);
    Ok(())
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the source matrix boundary and coordinate order."
)]
pub fn primaries_to_xyz_d50(
    rx: f32,
    ry: f32,
    gx: f32,
    gy: f32,
    bx: f32,
    by: f32,
    wx: f32,
    wy: f32,
    matrix: &mut Matrix3x3,
) -> Status {
    let mut to_xyz = [[0.0; 3]; 3];
    primaries_to_xyz(rx, ry, gx, gy, bx, by, wx, wy, &mut to_xyz)?;
    let mut d50 = [[0.0; 3]; 3];
    adapt_to_xyz_d50(wx, wy, &mut d50)?;
    mul3x3_matrix(&d50, &to_xyz, matrix);
    Ok(())
}
pub fn cie_xyz_from_white_cie_xy(wx: f64, wy: f64, xyz: &mut Color) -> Status {
    if wy.abs() < 1e-12 {
        return Err(StatusCode::GenericError);
    }
    let factor = (1.0 / wy) as f32;
    xyz[0] = (wx * f64::from(factor)) as f32;
    xyz[1] = 1.0;
    xyz[2] = ((1.0 - wx - wy) * f64::from(factor)) as f32;
    Ok(())
}
pub fn tone_map_pixel(c: &JxlColorEncoding, input: [f32; 3], out: &mut [u8; 3]) -> Status {
    let mut primaries = [[0.0; 3]; 3];
    primaries_to_xyz(
        c.primaries_red_xy[0] as f32,
        c.primaries_red_xy[1] as f32,
        c.primaries_green_xy[0] as f32,
        c.primaries_green_xy[1] as f32,
        c.primaries_blue_xy[0] as f32,
        c.primaries_blue_xy[1] as f32,
        c.white_point_xy[0] as f32,
        c.white_point_xy[1] as f32,
        &mut primaries,
    )?;
    let luminances = primaries[1];
    let mut linear = [0.0; 3];
    let pq = c.transfer_function == TransferFunction::PQ;
    for i in 0..3 {
        linear[i] = if pq {
            TfPqBase::display_from_encoded(10000.0, f64::from(input[i]))
        } else {
            TfHlgBase::display_from_encoded(f64::from(input[i]))
        } as f32;
    }
    if pq {
        Rec2408ToneMapperBase::new([0.0, 10000.0], [0.0, 250.0], luminances).tone_map(&mut linear);
    } else {
        HlgOotfBase::new(300.0, 80.0, luminances).apply(&mut linear);
    }
    gamut_map_scalar(&mut linear, luminances, 0.3);
    let mut chad = [[0.0; 3]; 3];
    adapt_to_xyz_d50(
        c.white_point_xy[0] as f32,
        c.white_point_xy[1] as f32,
        &mut chad,
    )?;
    let mut to_xyzd50 = [[0.0; 3]; 3];
    mul3x3_matrix(&chad, &primaries, &mut to_xyzd50);
    let mut xyz = [0.0; 3];
    for xyz_c in 0..3 {
        for rgb_c in 0..3 {
            xyz[xyz_c] = mul_add_f32(linear[rgb_c], to_xyzd50[xyz_c][rgb_c], xyz[xyz_c]);
        }
    }
    fn lab_f(x: f32) -> f32 {
        const DELTA: f32 = (6.0f64 / 29.0) as f32;
        if x <= DELTA * DELTA * DELTA {
            mul_add_f32(x, 1.0 / (3.0 * DELTA * DELTA), 4.0 / 29.0)
        } else {
            x.cbrt()
        }
    }
    let f_x = lab_f(xyz[0] / 0.964212);
    let f_y = lab_f(xyz[1]);
    let f_z = lab_f(xyz[2] / 0.825188);
    out[0] = (255.0 * clamp_f32(mul_add_f32(1.16, f_y, -0.16), 0.0, 1.0)).round() as u8;
    out[1] = (128.0 + clamp_f32(500.0 * (f_x - f_y), -128.0, 127.0)).round() as u8;
    out[2] = (128.0 + clamp_f32(200.0 * (f_y - f_z), -128.0, 127.0)).round() as u8;
    Ok(())
}
pub fn create_table_curve<const N: usize>(pq: bool, tone_map: bool) -> Vec<u16> {
    assert!(N <= 4096);
    let tone_mapper = Rec2408ToneMapperBase::new([0.0, 10000.0], [0.0, 255.0], [1.0 / 3.0; 3]);
    let mut table = vec![0; N];
    for (i, value) in table.iter_mut().enumerate() {
        let x = i as f32 / (N - 1) as f32;
        let mut y = if pq {
            TfPqBase::display_from_encoded(10000.0, f64::from(x))
        } else {
            TfHlgBase::display_from_encoded(f64::from(x))
        };
        if tone_map && pq {
            let l = (y * 10000.0 / 10000.0) as f32;
            let mut gray = [l; 3];
            tone_mapper.tone_map(&mut gray);
            y = f64::from(gray[0]);
        }
        debug_assert!(y >= 0.0);
        y = clamp_f64(y, 0.0, 1.0);
        *value = ((y * 65535.0) as f32).round() as u16;
    }
    table
}
