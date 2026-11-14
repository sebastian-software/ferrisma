// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The per-target part of dequant.cc in the AVX2 dispatch variant (port ADR
// 0004, oracle_target.h): foreach_target compiles it for every target, with
// Highway's per-target include guard, as upstream's dec_group.cc compiles
// its DequantLane/DequantBlock. The same declarations as dequant.cc's
// single-target block; the extracted templates are unchanged.
#if defined(ORACLE_DEQUANT_TARGETS_INL_H_) == defined(HWY_TARGET_TOGGLE)
#ifdef ORACLE_DEQUANT_TARGETS_INL_H_
#undef ORACLE_DEQUANT_TARGETS_INL_H_
#else
#define ORACLE_DEQUANT_TARGETS_INL_H_
#endif
#include "lib/jxl/dec_transforms-inl.h"
#include "lib/jxl/quantizer-inl.h"
HWY_BEFORE_NAMESPACE();
namespace jxl { namespace HWY_NAMESPACE { namespace OracleDequant {
using D = HWY_FULL(float);
using DI = HWY_FULL(int32_t);
using DI16 = hwy::HWY_NAMESPACE::Rebind<int16_t, DI>;
constexpr D d;
constexpr DI di;
constexpr DI16 di16;
#include "original_dequant_block.inc"
}}}
HWY_AFTER_NAMESPACE();
#endif  // ORACLE_DEQUANT_TARGETS_INL_H_
