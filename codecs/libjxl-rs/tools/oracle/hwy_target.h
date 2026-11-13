// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Forced include (-include) of the SIMD oracle variants (port ADR 0004). It
// defines Highway's own HWY_DISABLED_TARGETS knob before any source, so that
// upstream's CMake detects it as forced and adds no default list of its own,
// and every translation unit (original and adapter) compiles the same target
// set. It changes no source file and supplies no algorithm.
#ifndef ORACLE_HWY_TARGET_H_
#define ORACLE_HWY_TARGET_H_
#if defined(ORACLE_HWY_TARGET_NEON)
// Apple Clang's default AArch64 CPU makes HWY_NEON (with AES) the baseline;
// exclude every higher Arm target so that HWY_TARGETS is HWY_NEON alone.
#define HWY_DISABLED_TARGETS \
  (HWY_NEON_BF16 | HWY_SVE | HWY_SVE2 | HWY_SVE_256 | HWY_SVE2_128)
#elif defined(ORACLE_HWY_TARGET_AVX2)
// Compile only the static target, which the AVX2 group flags of
// CMakeLists.txt make HWY_AVX2: every translation unit then compiles exactly
// one target, as the adapters require (see port ADR 0004 for the difference
// to upstream's dispatch build). AVX-512 is excluded (decision D6).
#define HWY_COMPILE_ONLY_STATIC 1
#define HWY_DISABLED_TARGETS \
  (HWY_AVX3 | HWY_AVX3_DL | HWY_AVX3_ZEN4 | HWY_AVX3_SPR)
#else
#error "Define ORACLE_HWY_TARGET_NEON or ORACLE_HWY_TARGET_AVX2"
#endif
#endif  // ORACLE_HWY_TARGET_H_
