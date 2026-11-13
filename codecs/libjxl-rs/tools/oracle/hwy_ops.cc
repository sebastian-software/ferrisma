// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe single pinned Highway operations at the oracle's selected target
// (port ADR 0004). Requests:
//   hwy-target                     -> "<target name> <float lanes>"
//   hwy-op <op> <n> <n u32 words>  -> "<float lanes> <result u32 words>"
// The input words fill consecutive full float vectors a, b, c, d (missing
// words are zero). This TU compiles per target through foreach_target and
// is called through HWY_DYNAMIC_DISPATCH, as the original's own kernels are;
// it supplies no operation of its own, only the Highway call.
#undef HWY_TARGET_INCLUDE
#define HWY_TARGET_INCLUDE "hwy_ops.cc"
#include <hwy/foreach_target.h>  // IWYU pragma: keep
#include <hwy/highway.h>

#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

HWY_BEFORE_NAMESPACE();
namespace oracle_hwy {
namespace HWY_NAMESPACE {
namespace hn = hwy::HWY_NAMESPACE;

int64_t Target() { return HWY_TARGET; }
size_t FloatLanes() { return hn::Lanes(hn::ScalableTag<float>()); }

template <int kBits, class V>
V ShiftLeftBy(V v) { return hn::ShiftLeft<kBits>(v); }
template <int kBits, class V>
V ShiftRightBy(V v) { return hn::ShiftRight<kBits>(v); }

// Returns the number of output words, or 0 for an unknown operation.
size_t RunOp(const std::string& op, const uint32_t* in, uint32_t* out) {
  const hn::ScalableTag<float> d;
  const hn::RebindToSigned<decltype(d)> di;
  const hn::RebindToUnsigned<decltype(d)> du;
  const size_t n = hn::Lanes(d);
  float* fo = reinterpret_cast<float*>(out);
  const auto a = hn::BitCast(d, hn::LoadU(du, in));
  const auto b = hn::BitCast(d, hn::LoadU(du, in + n));
  const auto c = hn::BitCast(d, hn::LoadU(du, in + 2 * n));
  const auto e = hn::BitCast(d, hn::LoadU(du, in + 3 * n));
  const auto ai = hn::BitCast(di, a);
  const auto bi = hn::BitCast(di, b);
  auto put = [&](decltype(a) v) { hn::StoreU(hn::BitCast(du, v), du, out); return n; };
  auto puti = [&](decltype(ai) v) { hn::StoreU(hn::BitCast(du, v), du, out); return n; };
  // Arithmetic.
  if (op == "add") return put(hn::Add(a, b));
  if (op == "sub") return put(hn::Sub(a, b));
  if (op == "mul") return put(hn::Mul(a, b));
  if (op == "div") return put(hn::Div(a, b));
  if (op == "mul_add") return put(hn::MulAdd(a, b, c));
  if (op == "neg_mul_add") return put(hn::NegMulAdd(a, b, c));
  if (op == "mul_sub") return put(hn::MulSub(a, b, c));
  if (op == "neg_mul_sub") return put(hn::NegMulSub(a, b, c));
  if (op == "min") return put(hn::Min(a, b));
  if (op == "max") return put(hn::Max(a, b));
  if (op == "clamp") return put(hn::Clamp(a, b, c));
  if (op == "abs") return put(hn::Abs(a));
  if (op == "neg") return put(hn::Neg(a));
  if (op == "abs_diff") return put(hn::AbsDiff(a, b));
  if (op == "sqrt") return put(hn::Sqrt(a));
  if (op == "approximate_reciprocal") return put(hn::ApproximateReciprocal(a));
  if (op == "approximate_reciprocal_sqrt") return put(hn::ApproximateReciprocalSqrt(a));
  if (op == "floor") return put(hn::Floor(a));
  if (op == "ceil") return put(hn::Ceil(a));
  if (op == "round") return put(hn::Round(a));
  if (op == "trunc") return put(hn::Trunc(a));
  if (op == "zero_if_negative") return put(hn::ZeroIfNegative(a));
  if (op == "copy_sign") return put(hn::CopySign(a, b));
  if (op == "copy_sign_to_abs") return put(hn::CopySignToAbs(a, b));
  // Bitwise on float vectors.
  if (op == "and") return put(hn::And(a, b));
  if (op == "or") return put(hn::Or(a, b));
  if (op == "xor") return put(hn::Xor(a, b));
  if (op == "and_not") return put(hn::AndNot(a, b));
  // Comparisons, as VecFromMask, and selections.
  if (op == "lt") return put(hn::VecFromMask(d, hn::Lt(a, b)));
  if (op == "le") return put(hn::VecFromMask(d, hn::Le(a, b)));
  if (op == "gt") return put(hn::VecFromMask(d, hn::Gt(a, b)));
  if (op == "ge") return put(hn::VecFromMask(d, hn::Ge(a, b)));
  if (op == "eq") return put(hn::VecFromMask(d, hn::Eq(a, b)));
  if (op == "ne") return put(hn::VecFromMask(d, hn::Ne(a, b)));
  if (op == "if_lt_then_else") return put(hn::IfThenElse(hn::Lt(a, b), c, e));
  if (op == "if_lt_then_zero_else") return put(hn::IfThenZeroElse(hn::Lt(a, b), c));
  if (op == "if_lt_then_else_zero") return put(hn::IfThenElseZero(hn::Lt(a, b), c));
  // Conversions.
  if (op == "convert_to_i32") return puti(hn::ConvertTo(di, a));
  if (op == "convert_to_f32") return put(hn::ConvertTo(d, ai));
  if (op == "nearest_int") return puti(hn::NearestInt(a));
  // Signed 32-bit integer lanes.
  if (op == "add_i32") return puti(hn::Add(ai, bi));
  if (op == "sub_i32") return puti(hn::Sub(ai, bi));
  if (op == "min_i32") return puti(hn::Min(ai, bi));
  if (op == "max_i32") return puti(hn::Max(ai, bi));
  if (op == "lt_i32") return puti(hn::VecFromMask(di, hn::Lt(ai, bi)));
  if (op == "eq_i32") return puti(hn::VecFromMask(di, hn::Eq(ai, bi)));
  if (op == "gt_i32") return puti(hn::VecFromMask(di, hn::Gt(ai, bi)));
  if (op == "shl_i32_1") return puti(ShiftLeftBy<1>(ai));
  if (op == "shl_i32_23") return puti(ShiftLeftBy<23>(ai));
  if (op == "shr_i32_1") return puti(ShiftRightBy<1>(ai));
  if (op == "shr_i32_23") return puti(ShiftRightBy<23>(ai));
  if (op == "shr_i32_31") return puti(ShiftRightBy<31>(ai));
  if (op == "shr_u32_1") { hn::StoreU(ShiftRightBy<1>(hn::BitCast(du, a)), du, out); return n; }
  if (op == "shr_u32_23") { hn::StoreU(ShiftRightBy<23>(hn::BitCast(du, a)), du, out); return n; }
  // Reductions (the result vector broadcasts the reduction).
  if (op == "sum_of_lanes") return put(hn::SumOfLanes(d, a));
  if (op == "max_of_lanes") return put(hn::MaxOfLanes(d, a));
  if (op == "min_of_lanes") return put(hn::MinOfLanes(d, a));
  if (op == "reduce_sum") { fo[0] = hn::ReduceSum(d, a); return 1; }
  if (op == "get_lane") { fo[0] = hn::GetLane(a); return 1; }
  // Lane movement (not defined for the one-lane HWY_SCALAR target).
#if HWY_TARGET != HWY_SCALAR
  if (op == "interleave_lower") return put(hn::InterleaveLower(d, a, b));
  if (op == "interleave_upper") return put(hn::InterleaveUpper(d, a, b));
  if (op == "concat_lower_lower") return put(hn::ConcatLowerLower(d, a, b));
  if (op == "concat_upper_upper") return put(hn::ConcatUpperUpper(d, a, b));
  if (op == "concat_lower_upper") return put(hn::ConcatLowerUpper(d, a, b));
  if (op == "concat_upper_lower") return put(hn::ConcatUpperLower(d, a, b));
  if (op == "odd_even") return put(hn::OddEven(a, b));
  if (op == "dup_even") return put(hn::DupEven(a));
  if (op == "dup_odd") return put(hn::DupOdd(a));
  if (op == "reverse") return put(hn::Reverse(d, a));
  if (op == "broadcast_0") return put(hn::Broadcast<0>(a));
  if (op == "broadcast_3") return put(hn::Broadcast<3>(a));
  if (op == "load_dup128") {
    HWY_ALIGN float block[4];
    std::memcpy(block, in, sizeof(block));
    return put(hn::LoadDup128(d, block));
  }
#endif
  if (op == "iota") return put(hn::Iota(d, hwy::BitCastScalar<float>(in[0])));
  if (op == "store_interleaved2") { hn::StoreInterleaved2(a, b, d, fo); return 2 * n; }
  if (op == "store_interleaved3") { hn::StoreInterleaved3(a, b, c, d, fo); return 3 * n; }
  if (op == "store_interleaved4") { hn::StoreInterleaved4(a, b, c, e, d, fo); return 4 * n; }
  // render stages: the binary16 words of DemoteTo(df16, a), one per output word.
  if (op == "demote_f16") {
    const hn::Rebind<hwy::float16_t, decltype(d)> df16;
    const hn::Rebind<uint16_t, decltype(d)> du16;
    uint16_t half[64];
    hn::StoreU(hn::BitCast(du16, hn::DemoteTo(df16, a)), du16, half);
    for (size_t i = 0; i < n; ++i) out[i] = half[i];
    return n;
  }
  // end render stages
  // modular: signed 32-bit operations of the modular kernels.
#if HWY_TARGET != HWY_SCALAR
  {
    const auto ci = hn::BitCast(di, c);
    const auto ei = hn::BitCast(di, e);
    if (op == "set_i32") return puti(hn::Set(di, static_cast<int32_t>(in[0])));
    if (op == "load_i32") return puti(hn::LoadU(di, reinterpret_cast<const int32_t*>(in)));
    if (op == "and_i32") return puti(hn::And(ai, bi));
    if (op == "xor_i32") return puti(hn::Xor(ai, bi));
    if (op == "abs_i32") return puti(hn::Abs(ai));
    if (op == "neg_i32") return puti(hn::Neg(ai));
    if (op == "ne_i32") return puti(hn::VecFromMask(di, hn::Ne(ai, bi)));
    if (op == "if_i32_lt_then_else") return puti(hn::IfThenElse(hn::Lt(ai, bi), ci, ei));
    if (op == "if_i32_lt_then_zero_else") return puti(hn::IfThenZeroElse(hn::Lt(ai, bi), ci));
    if (op == "mul_even_i32") return puti(hn::BitCast(di, hn::MulEven(ai, bi)));
    if (op == "mul_odd_i32") return puti(hn::BitCast(di, hn::MulOdd(ai, bi)));
    if (op == "interleave_odd_i32") return puti(hn::InterleaveOdd(di, ai, bi));
  }
#endif
  // end modular
  // stages2
  if (op == "mul_i32") return puti(hn::Mul(ai, bi));
#if HWY_TARGET != HWY_SCALAR
  if (op == "table_lookup_bytes") return puti(hn::TableLookupBytes(ai, bi));
#endif
  // end stages2
  return 0;
}

// NOLINTNEXTLINE(google-readability-namespace-comments)
}  // namespace HWY_NAMESPACE
}  // namespace oracle_hwy
HWY_AFTER_NAMESPACE();

#if HWY_ONCE
namespace oracle_hwy {
HWY_EXPORT(Target);
HWY_EXPORT(FloatLanes);
HWY_EXPORT(RunOp);

// With a single compiled target HWY_DYNAMIC_DISPATCH names the target
// namespace directly, so it is only used inside this namespace.
int64_t DispatchedTarget() { return HWY_DYNAMIC_DISPATCH(Target)(); }
size_t DispatchedLanes() { return HWY_DYNAMIC_DISPATCH(FloatLanes)(); }
size_t DispatchedOp(const std::string& op, const uint32_t* in, uint32_t* out) {
  return HWY_DYNAMIC_DISPATCH(RunOp)(op, in, out);
}

#if defined(ORACLE_HWY_REQUIRED)
// The selected SIMD variant compiles no target above the required one, and
// refuses to run where run-time dispatch would select a different target
// (for example x86-64 under Rosetta 2 without ROSETTA_ADVERTISE_AVX=1).
static_assert((HWY_TARGETS & ORACLE_HWY_REQUIRED) != 0, "required target not compiled");
static_assert((HWY_TARGETS & (ORACLE_HWY_REQUIRED - 1)) == 0, "a higher target is compiled");
const bool kTargetChecked = [] {
  // A static-only build calls its one target unconditionally; check the
  // CPU's run-time support as well as the dispatched target.
  const int64_t chosen = DispatchedTarget();
  if (chosen != ORACLE_HWY_REQUIRED || (hwy::SupportedTargets() & ORACLE_HWY_REQUIRED) == 0) {
    std::cerr << "jxl_oracle: built for " << hwy::TargetName(ORACLE_HWY_REQUIRED)
              << ", but dispatch selects " << hwy::TargetName(chosen)
              << " or the CPU does not report support; on x86-64 under Rosetta 2 set"
                 " ROSETTA_ADVERTISE_AVX=1\n";
    std::abort();
  }
  return true;
}();
#endif
}  // namespace oracle_hwy

bool HwyOpsCommand(const std::string& command, std::istringstream& in) {
  if (command == "hwy-target") {
    std::cout << hwy::TargetName(oracle_hwy::DispatchedTarget()) << ' '
              << oracle_hwy::DispatchedLanes();
    return true;
  }
  if (command != "hwy-op") return false;
  std::string op;
  size_t count;
  in >> op >> count;
  const size_t lanes = oracle_hwy::DispatchedLanes();
  if (count > 4 * lanes) throw std::runtime_error("hwy-op: too many input words");
  std::vector<uint32_t> input(4 * lanes, 0), output(4 * lanes, 0);
  for (size_t i = 0; i < count; ++i) in >> input[i];
  const size_t written = oracle_hwy::DispatchedOp(op, input.data(), output.data());
  if (written == 0) throw std::runtime_error("hwy-op: unknown operation " + op);
  std::cout << lanes;
  for (size_t i = 0; i < written; ++i) std::cout << ' ' << output[i];
  return true;
}
#endif  // HWY_ONCE
