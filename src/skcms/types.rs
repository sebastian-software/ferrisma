// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Shared matrix representations from src/skcms_public.h.

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Matrix3x3 {
    pub vals: [[f32; 3]; 3],
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Matrix3x4 {
    pub vals: [[f32; 4]; 3],
}
