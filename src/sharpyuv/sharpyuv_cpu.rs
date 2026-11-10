// libwebp v1.6.0, sharpyuv/sharpyuv_cpu.c:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//

// libwebp v1.6.0, sharpyuv/sharpyuv_cpu.h:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//

//! Translation of `sharpyuv/sharpyuv_cpu.c` and `sharpyuv/sharpyuv_cpu.h` from libwebp v1.6.0.
//!
//! Work package: WP5 (`SharpYUV`).
//!
//! The C file compiles `src/dsp/cpu.c` a second time, with `VP8GetCPUInfo` renamed
//! to `SharpYuvGetCPUInfo` by the header, so the two names hold the same query.
//! The rename has no Rust counterpart. `SharpYuvGetCPUInfo` is a function with the
//! `VP8CPUInfo` signature that answers with WP4's `vp8_get_cpu_info` (ADR 0003).
//! Where the C pointer is NULL, the query answers `false` (`src/dsp/cpu.rs`).
//! `SharpYuvInit` replaces the pointer and is not translated here; see
//! `sharpyuv.rs`.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **NULL CPU-information pointer.** Outside x86 the reference configuration
//!    sets `SharpYuvGetCPUInfo` to NULL, and calling it would crash. The Rust
//!    query answers `false` for every feature there, as `src/dsp/cpu.rs`
//!    records for `VP8GetCPUInfo` (ADR 0003). On x86 it answers as the C
//!    pointer does.

use crate::dsp::cpu::{CPUFeature, vp8_get_cpu_info};

// upstream: sharpyuv/sharpyuv_cpu.c SharpYuvGetCPUInfo
/// Reports whether the running CPU supports `feature`. The C pointer that
/// `SharpYuvInit` can replace is not represented; its default value is this query.
#[must_use]
pub fn sharp_yuv_get_cpu_info(feature: CPUFeature) -> bool {
    vp8_get_cpu_info(feature)
}
