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
// Upstream's dynamic-dispatch build (ADR-0041 "SIMD reference build at level
// L"; port ADR 0004): every translation unit keeps the baseline x86-64 flags
// and compiles its HWY_NAMESPACE code once per target through foreach_target
// and Highway's target attributes. Upstream's own CMake default disables
// HWY_SSSE3 (and the RVV/SVE targets of other architectures); every target
// above HWY_AVX2 is disabled in addition, so that run-time dispatch selects
// HWY_AVX2 on an AVX2 CPU (decision D6 excludes AVX-512). hwy_ops.cc
// refuses to run where it selects another target.
#define HWY_DISABLED_TARGETS                                              \
  (HWY_SSSE3 | HWY_AVX3 | HWY_AVX3_DL | HWY_AVX3_ZEN4 | HWY_AVX3_SPR |   \
   HWY_RVV | HWY_SVE | HWY_SVE_256 | HWY_SVE2 | HWY_SVE2_128)
// Upstream's CMake turns JPEGXL_ENABLE_HWY_AVX3=OFF into
// -DFJXL_ENABLE_AVX512=0 as well, but only when it computes the list itself;
// with the list forced here, the same switch keeps the fast-lossless
// encoder's own AVX-512 path out.
#define FJXL_ENABLE_AVX512 0
#else
#error "Define ORACLE_HWY_TARGET_NEON or ORACLE_HWY_TARGET_AVX2"
#endif
#endif  // ORACLE_HWY_TARGET_H_
