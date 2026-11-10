// libwebp v1.6.0, src/dsp/rescaler_neon.c:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// NEON version of rescaling functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/rescaler_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The module holds no code: the NEON row exports of upstream are not
//! admitted (ADR 0008, decided on issue #28).
//!
//! # Deviations from upstream
//!
//! 1. **`RescalerExportRowExpand_NEON` and `RescalerExportRowShrink_NEON`
//!    are not selected.** Upstream's `WebPRescalerDspInitNEON` assigns both,
//!    and `WebPRescalerDspInit` calls it unconditionally on 64-bit ARM
//!    (`WEBP_NEON_OMIT_C_CODE`); the port keeps
//!    `webp_rescaler_export_row_expand` and `webp_rescaler_export_row_shrink`
//!    of `src/dsp/rescaler.rs` on the C functions there. The kernels multiply
//!    with `vqrdmulhq_s32` by `scale >> 1` (`MAKE_HALF_CST`), which drops the
//!    scale's lowest bit, so they are not bit-exact with the C functions for
//!    inputs the library passes: of 600 random size pairs (widths 1 to 300,
//!    heights 1 to 80, one to four channels) through the `release-simd`
//!    probe `wp4/rescaler`, 6 differ from C in one or two bytes, each one
//!    lower, and the scaled decode of `vp80-05-sharpness-1443.webp` to 640 by
//!    48 differs from the reference oracle. A faithful translation matched
//!    upstream's NEON bytes in all 600 cases; it is kept on the branch
//!    `sebastian/wp20-neon-rescaler-checkpoint` (commit 4d00d9f) for the
//!    tuning phase (issue #38). Issue #37 tracks the report to upstream.
//!
//! # Excluded undefined behavior
//!
//! None: nothing is translated.
//!
//! # Checks not applicable
//!
//! 1. **The parity test** of ADR 0008 has no function to compare; the export
//!    dispatch functions keep the C functions, which the WP4 rescaler tests
//!    and the scaled decodes of WP12 and WP16 verify against the reference
//!    oracle with `simd` on.
