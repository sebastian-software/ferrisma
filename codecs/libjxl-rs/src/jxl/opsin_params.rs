// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete default-matrix selection and lane expansion from opsin_params.cc.
//! The pinned INVERSE_OPSIN_FROM_SPEC branch is enabled in the source.
use super::base::matrix_ops::Matrix3x3;
pub fn get_opsin_absorbance_inverse_matrix() -> &'static Matrix3x3 {
    super::cms::opsin_params::default_inverse_opsin_absorbance_matrix()
}
pub fn init_simd_inverse_matrix(inverse: &Matrix3x3, simd: &mut [f32; 36], intensity_target: f32) {
    for (j, row) in inverse.iter().enumerate() {
        for (i, &entry) in row.iter().enumerate() {
            let idx = (j * 3 + i) * 4;
            let value = entry * (255.0f32 / intensity_target);
            simd[idx] = value;
            simd[idx + 1] = value;
            simd[idx + 2] = value;
            simd[idx + 3] = value;
        }
    }
}
