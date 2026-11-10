// Copyright 2020 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this source code is governed by the BSD license in
// licenses/HIGHWAY-BSD-3-Clause.txt. Highway offers this license as an
// alternative to Apache-2.0; see its pinned README.md.
// Modified: translated the prefetch operation and its enablement into Rust.

//! Ported from Highway's `hwy/cache_control.h` for the Clang reference build.
//! Its top-level guard disables cache control without SSE2, and also on 32-bit
//! x86 with Clang. In particular, the original AArch64 operation is disabled.

pub(crate) fn prefetch<T>(value: &T) {
    let _ = value; // The source also permits a disabled cache hint.
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    // SAFETY: SSE2 (and hence SSE) is enabled by the compiler target. The live
    // shared borrow supplies an initialized address for the same T0 read hint
    // as the source. This hint neither writes nor retains the pointer.
    unsafe {
        std::arch::x86_64::_mm_prefetch((value as *const T).cast(), std::arch::x86_64::_MM_HINT_T0);
    }
}
