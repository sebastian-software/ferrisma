// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Kernel weight owners from lib/jxl/convolve.h. Symmetric5 is translated in
//! `convolve_symmetric5`, SlowSymmetric3 in `convolve_slow`; SlowSeparable5
//! and Separable5 remain unported.

/// The symmetric 3x3 kernel: d r d / r c r / d r d, each replicated 4x.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WeightsSymmetric3 {
    pub c: [f32; 4],
    pub r: [f32; 4],
    pub d: [f32; 4],
}

/// The lower-right quadrant of the symmetric 5x5 kernel, each replicated 4x:
/// c r R / r d L / R L D, in the source's field order.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[expect(
    non_snake_case,
    reason = "Retain the source's distinct R, D and L field names."
)]
pub struct WeightsSymmetric5 {
    pub c: [f32; 4],
    pub r: [f32; 4],
    pub R: [f32; 4],
    pub d: [f32; 4],
    pub D: [f32; 4],
    pub L: [f32; 4],
}
