// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/base/matrix_ops.h`. Array borrows replace template
//! matrix references. Intermediate widths and source mutation order remain.

use super::float::mul_add_f64;
use super::status::{Status, StatusCode};

pub type Vector3 = [f32; 3];
pub type Vector3d = [f64; 3];
pub type Matrix3x3 = [Vector3; 3];
pub type Matrix3x3d = [Vector3d; 3];

pub trait MatrixScalar: Copy + Default {
    fn to_f64(self) -> f64;
    fn from_f64(value: f64) -> Self;
}
impl MatrixScalar for f32 {
    fn to_f64(self) -> f64 {
        f64::from(self)
    }
    fn from_f64(value: f64) -> Self {
        value as f32
    }
}
impl MatrixScalar for f64 {
    fn to_f64(self) -> f64 {
        self
    }
    fn from_f64(value: f64) -> Self {
        value
    }
}

pub fn mul3x3_matrix<T: MatrixScalar>(a: &[[T; 3]; 3], b: &[[T; 3]; 3], c: &mut [[T; 3]; 3]) {
    #[repr(align(16))]
    struct AlignedVector(Vector3d);
    for x in 0..3 {
        let temp = AlignedVector([b[0][x].to_f64(), b[1][x].to_f64(), b[2][x].to_f64()]);
        for y in 0..3 {
            c[y][x] = T::from_f64(mul_add_f64(
                a[y][2].to_f64(),
                temp.0[2],
                mul_add_f64(a[y][0].to_f64(), temp.0[0], a[y][1].to_f64() * temp.0[1]),
            ));
        }
    }
}
pub fn mul3x3_vector<M: MatrixScalar, V: MatrixScalar>(
    a: &[[M; 3]; 3],
    b: &[V; 3],
    c: &mut [V; 3],
) {
    for y in 0..3 {
        let mut e = 0.0;
        for x in 0..3 {
            e = mul_add_f64(a[y][x].to_f64(), b[x].to_f64(), e);
        }
        c[y] = V::from_f64(e);
    }
}
pub fn inv3x3_matrix<T: MatrixScalar>(matrix: &mut [[T; 3]; 3]) -> Status {
    let m = &*matrix;
    let mut temp = [[0.0f64; 3]; 3];
    temp[0][0] = mul_add_f64(
        m[1][1].to_f64(),
        m[2][2].to_f64(),
        -m[1][2].to_f64() * m[2][1].to_f64(),
    );
    temp[0][1] = mul_add_f64(
        m[0][2].to_f64(),
        m[2][1].to_f64(),
        -m[0][1].to_f64() * m[2][2].to_f64(),
    );
    temp[0][2] = mul_add_f64(
        m[0][1].to_f64(),
        m[1][2].to_f64(),
        -m[0][2].to_f64() * m[1][1].to_f64(),
    );
    temp[1][0] = mul_add_f64(
        m[1][2].to_f64(),
        m[2][0].to_f64(),
        -m[1][0].to_f64() * m[2][2].to_f64(),
    );
    temp[1][1] = mul_add_f64(
        m[0][0].to_f64(),
        m[2][2].to_f64(),
        -m[0][2].to_f64() * m[2][0].to_f64(),
    );
    temp[1][2] = mul_add_f64(
        m[0][2].to_f64(),
        m[1][0].to_f64(),
        -m[0][0].to_f64() * m[1][2].to_f64(),
    );
    temp[2][0] = mul_add_f64(
        m[1][0].to_f64(),
        m[2][1].to_f64(),
        -m[1][1].to_f64() * m[2][0].to_f64(),
    );
    temp[2][1] = mul_add_f64(
        m[0][1].to_f64(),
        m[2][0].to_f64(),
        -m[0][0].to_f64() * m[2][1].to_f64(),
    );
    temp[2][2] = mul_add_f64(
        m[0][0].to_f64(),
        m[1][1].to_f64(),
        -m[0][1].to_f64() * m[1][0].to_f64(),
    );
    let det = mul_add_f64(
        m[0][2].to_f64(),
        temp[2][0],
        mul_add_f64(m[0][0].to_f64(), temp[0][0], m[0][1].to_f64() * temp[1][0]),
    );
    if det.abs() < 1e-10 {
        return Err(StatusCode::GenericError);
    }
    let idet = 1.0 / det;
    for j in 0..3 {
        for i in 0..3 {
            matrix[j][i] = T::from_f64(temp[j][i] * idet);
        }
    }
    Ok(())
}
