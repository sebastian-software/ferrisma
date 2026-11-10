// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Mechanically translated source boundaries of the pinned skcms dependency.
pub mod curve;
pub mod fitting;
pub mod matrix;
pub mod profile;
pub mod transfer_function;
pub mod transform;
mod transform_simd;
pub mod transform_types;
#[cfg(target_arch = "x86_64")]
mod transform_x86;
pub mod types;
