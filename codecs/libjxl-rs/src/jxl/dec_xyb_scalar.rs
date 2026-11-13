// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! One-lane XybToRgb from lib/jxl/dec_xyb-inl.h. Highway's scalar MulAdd
//! rounds its multiplication separately, including on AArch64. The local
//! Add/Mul/MulAdd primitives keep its source operations and operand order.
//! They inline freely: which NaN payload survives is not part of the gate
//! (decision D8; port ADR 0002).
// The one-lane primitives derive from hwy/ops/scalar-inl.h and
// hwy/ops/generic_ops-inl.h: Copyright 2019 Google LLC; Copyright (c) The Highway
// Project Authors. All rights reserved. BSD alternative notice is retained in
// licenses/HIGHWAY-BSD-3-Clause.txt.
use super::dec_xyb::OpsinParams;

#[inline]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
#[inline]
fn widen(value: f32) -> f64 {
    f64::from(value)
}
#[inline]
fn mul_f64(a: f64, b: f64) -> f64 {
    a * b
}
// scalar-inl.h's floating operator* widens both operands to double and
// narrows its product to the lane type, as written here.
#[inline]
pub(super) fn mul(a: f32, b: f32) -> f32 {
    let a = widen(a);
    let b = widen(b);
    mul_f64(a, b) as f32
}
fn mul_add(a: f32, b: f32, c: f32) -> f32 {
    add(mul(a, b), c)
}

#[inline]
pub fn xyb_to_rgb(opsin_x: f32, opsin_y: f32, opsin_b: f32, params: &OpsinParams) -> [f32; 3] {
    let neg_bias_r = params.opsin_biases[0];
    let neg_bias_g = params.opsin_biases[1];
    let neg_bias_b = params.opsin_biases[2];
    let mut gamma_r = add(opsin_y, opsin_x);
    let mut gamma_g = opsin_y - opsin_x;
    let mut gamma_b = opsin_b;
    gamma_r -= params.opsin_biases_cbrt[0];
    gamma_g -= params.opsin_biases_cbrt[1];
    gamma_b -= params.opsin_biases_cbrt[2];
    let gamma_r2 = mul(gamma_r, gamma_r);
    let gamma_g2 = mul(gamma_g, gamma_g);
    let gamma_b2 = mul(gamma_b, gamma_b);
    let mixed_r = mul_add(gamma_r2, gamma_r, neg_bias_r);
    let mixed_g = mul_add(gamma_g2, gamma_g, neg_bias_g);
    let mixed_b = mul_add(gamma_b2, gamma_b, neg_bias_b);
    let matrix = &params.inverse_opsin_matrix;
    let mut linear_r = mul(matrix[0], mixed_r);
    let mut linear_g = mul(matrix[12], mixed_r);
    let mut linear_b = mul(matrix[24], mixed_r);
    linear_r = mul_add(matrix[4], mixed_g, linear_r);
    linear_g = mul_add(matrix[16], mixed_g, linear_g);
    linear_b = mul_add(matrix[28], mixed_g, linear_b);
    linear_r = mul_add(matrix[8], mixed_b, linear_r);
    linear_g = mul_add(matrix[20], mixed_b, linear_g);
    linear_b = mul_add(matrix[32], mixed_b, linear_b);
    [linear_r, linear_g, linear_b]
}
