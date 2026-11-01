// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! NoiseParams storage/Clear/HasAny from lib/jxl/noise.h. Noise estimation,
//! rendering, and the other helpers remain unported. Source LUT storage is
//! indeterminate before assignment; safe native initialization adds zeroes.
pub const NUM_NOISE_POINTS: usize = 8;
#[derive(Default)]
pub struct NoiseParams {
    pub lut: [f32; NUM_NOISE_POINTS],
}
impl NoiseParams {
    pub fn clear(&mut self) {
        self.lut.fill(0.0);
    }
    pub fn has_any(&self) -> bool {
        for value in self.lut {
            if value.abs() > 1e-3f32 {
                return true;
            }
        }
        false
    }
}
