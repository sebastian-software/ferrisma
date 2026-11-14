// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Adapter code that calls the selected target's HWY_NAMESPACE functions
// directly (port ADR 0004). The AVX2 variant is upstream's dispatch build
// (ORACLE_HWY_DISPATCH): its TUs keep the baseline x86-64 flags, so code
// that inlines N_AVX2 operations must itself carry Highway's HWY_AVX2 target
// attributes, exactly as HWY_BEFORE_NAMESPACE gives them to N_AVX2 code.
// ORACLE_TARGET_BEGIN/END push and pop those attributes; in the SCALAR and
// NEON variants, which compile one static target, they expand to nothing.
// An adapter that names N_AVX2 code of an -inl.h header without including
// a pristine .cc first defines HWY_TARGET_INCLUDE and runs foreach_target in
// the AVX2 variant, as upstream's .cc files do. foreach_target runs at most
// once per TU (its include guard): a TU that includes a pristine .cc with
// HWY code gets its targets from that .cc, and a second such .cc in the same
// TU would silently compile only the static target, so none may follow.
#ifndef ORACLE_TARGET_H_
#define ORACLE_TARGET_H_
#if defined(ORACLE_HWY_DISPATCH)
#define ORACLE_TARGET_BEGIN HWY_PUSH_ATTRIBUTES(HWY_TARGET_STR_AVX2)
#define ORACLE_TARGET_END HWY_POP_ATTRIBUTES
#else
#define ORACLE_TARGET_BEGIN
#define ORACLE_TARGET_END
#endif
#endif  // ORACLE_TARGET_H_
