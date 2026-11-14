// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the encoder's chroma-from-luma SIMD kernels (port ADR 0004,
// "Encoder path"), observed at the oracle's single Highway target. The
// kernels are reached through the original's public, dispatching
// CfLHeuristics (HWY_DYNAMIC_DISPATCH reaches the one compiled target); this
// TU supplies no kernel arithmetic.
//   ecf-init <xsize> <ysize>
//       CfLHeuristics::Init(Rect(0, 0, xsize, ysize)) (InitDCStorage).
//       Prints the status, MaxVectorSize() / sizeof(float) (the lanes of
//       HWY_FULL(float)), the size of dc_values and, per row, the last
//       min(xsize, lanes) samples, the ones InitDCStorage zero-fills (the
//       others are uninitialized).
//   ecf-best <num> <base> <distance_mul> <fast> <num words m> <num words s>
//       FindBestMultiplier of the target namespace (ORACLE_HWY_NS; not an
//       export, external linkage) on 128-byte aligned copies; num is a
//       multiple of 8. Prints the multiplier.
//   ecf-tile <xsize_blocks> <ysize_blocks> <x0> <y0> <pass> <fast>
//            <quant_dc> <quant_median> <n> <n * (bx by raw)>
//            <raw quant field words> <3 * ysize * xsize opsin words>
//       The CfL loop of enc_heuristics.cc over every 8x8-block tile of an
//       opsin image of xsize = x0 + 8 * xsize_blocks + 3 columns and ysize =
//       y0 + 8 * ysize_blocks rows, with opsin_rect at (x0, y0) of
//       8 * xsize_blocks x 8 * ysize_blocks: CfLHeuristics::Init,
//       PrepareForThreads(1), then ComputeTile per tile, pass 0 the DCT8
//       pass (no strategy, quant field or quantizer, fast = false) and pass 1
//       the second pass with the request's strategies (DCT elsewhere), raw
//       quant field and a Quantizer from ComputeGlobalScaleAndQuant over the
//       default matrices. dc_values is filled with sentinel words after Init
//       and the color maps with a fixed pattern. Prints the statuses, both
//       maps and every word of the four dc_values rows.
#include <jxl/memory_manager.h>

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/enc_chroma_from_luma.h"
#include "lib/jxl/image.h"
#include "lib/jxl/quant_weights.h"
#include "lib/jxl/quantizer.h"
#include "lib/jxl/simd_util.h"

namespace jxl {
namespace ORACLE_HWY_NS {
int32_t FindBestMultiplier(const float* values_m, const float* values_s, size_t num,
                           float base, float distance_mul, bool fast);
}  // namespace ORACLE_HWY_NS
}  // namespace jxl

namespace {
void EcfWord(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  std::cout << ' ' << bits;
}
float EcfFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}
// Distinct finite sentinel words keep unwritten samples observable.
float EcfSentinel(uint32_t salt, size_t i) {
  const uint32_t bits = 0x5a000000u + salt * 0x100000u + uint32_t(i & 0xfffff);
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}
// A buffer of `n` floats whose first element lies on a 128-byte boundary.
class EcfAligned {
 public:
  explicit EcfAligned(size_t n) : storage_(n + 128 / sizeof(float)) {
    const uintptr_t address = reinterpret_cast<uintptr_t>(storage_.data());
    offset_ = ((128 - address % 128) % 128) / sizeof(float);
  }
  float* data() { return storage_.data() + offset_; }

 private:
  std::vector<float> storage_;
  size_t offset_ = 0;
};
}  // namespace

bool EncoderCflSimdCommand(const std::string& command, std::istringstream& in,
                           JxlMemoryManager* mm) {
  if (command.rfind("ecf-", 0) != 0) return false;
  if (command == "ecf-init") {
    size_t xsize, ysize;
    in >> xsize >> ysize;
    jxl::CfLHeuristics cfl(mm);
    const jxl::Status status = cfl.Init(jxl::Rect(0, 0, xsize, ysize));
    const size_t lanes = jxl::MaxVectorSize() / sizeof(float);
    std::cout << "I " << int(status.code()) << ' ' << lanes;
    if (!status) return true;
    const jxl::ImageF& dc = cfl.dc_values;
    std::cout << ' ' << dc.xsize() << ' ' << dc.ysize();
    for (size_t y = 0; y < dc.ysize(); ++y) {
      std::cout << " R";
      for (size_t x = dc.xsize() - std::min(dc.xsize(), lanes); x < dc.xsize(); ++x)
        EcfWord(dc.ConstRow(y)[x]);
    }
    return true;
  }
  if (command == "ecf-best") {
    size_t num;
    in >> num;
    const float base = EcfFloat(in), distance_mul = EcfFloat(in);
    int fast;
    in >> fast;
    EcfAligned m(num + 8), s(num + 8);
    for (size_t i = 0; i < num; ++i) m.data()[i] = EcfFloat(in);
    for (size_t i = 0; i < num; ++i) s.data()[i] = EcfFloat(in);
    std::cout << "M "
              << jxl::ORACLE_HWY_NS::FindBestMultiplier(m.data(), s.data(), num, base,
                                                        distance_mul, fast != 0);
    return true;
  }
  if (command == "ecf-tile") {
    size_t xb, yb, x0, y0;
    int pass, fast;
    in >> xb >> yb >> x0 >> y0 >> pass >> fast;
    const float quant_dc = EcfFloat(in), quant_median = EcfFloat(in);
    const size_t xsize = x0 + 8 * xb + 3, ysize = y0 + 8 * yb;
    auto acs_or = jxl::AcStrategyImage::Create(mm, xb, yb);
    if (!acs_or.ok()) throw std::runtime_error("ac strategy");
    jxl::AcStrategyImage acs = std::move(acs_or).value_();
    acs.FillDCT8();
    size_t n;
    in >> n;
    for (size_t i = 0; i < n; ++i) {
      size_t bx, by;
      unsigned raw;
      in >> bx >> by >> raw;
      if (!acs.Set(bx, by, static_cast<jxl::AcStrategyType>(raw)))
        throw std::runtime_error("strategy placement");
    }
    auto rqf_or = jxl::ImageI::Create(mm, xb, yb);
    if (!rqf_or.ok()) throw std::runtime_error("quant field");
    jxl::ImageI rqf = std::move(rqf_or).value_();
    for (size_t y = 0; y < yb; ++y)
      for (size_t x = 0; x < xb; ++x) in >> rqf.Row(y)[x];
    jxl::DequantMatrices matrices;
    if (!matrices.EnsureComputed(mm, ~0u)) throw std::runtime_error("matrices");
    jxl::Quantizer quantizer(matrices);
    quantizer.ComputeGlobalScaleAndQuant(quant_dc, quant_median, 0);
    auto opsin_or = jxl::Image3F::Create(mm, xsize, ysize);
    if (!opsin_or.ok()) throw std::runtime_error("opsin");
    jxl::Image3F opsin = std::move(opsin_or).value_();
    for (size_t c = 0; c < 3; ++c)
      for (size_t y = 0; y < ysize; ++y)
        for (size_t x = 0; x < xsize; ++x) opsin.PlaneRow(c, y)[x] = EcfFloat(in);
    const jxl::Rect opsin_rect(x0, y0, 8 * xb, 8 * yb);
    auto cmap_or = jxl::ColorCorrelationMap::Create(mm, 8 * xb, 8 * yb);
    if (!cmap_or.ok()) throw std::runtime_error("cmap");
    jxl::ColorCorrelationMap cmap = std::move(cmap_or).value_();
    for (jxl::ImageSB* map : {&cmap.ytox_map, &cmap.ytob_map})
      for (size_t y = 0; y < map->ysize(); ++y)
        for (size_t x = 0; x < map->xsize(); ++x)
          map->Row(y)[x] = static_cast<int8_t>(int(x * 7 + y * 3) % 51 - 25);
    jxl::CfLHeuristics cfl(mm);
    jxl::Status status = cfl.Init(opsin_rect);
    std::cout << "T " << int(status.code());
    if (!status) return true;
    for (size_t y = 0; y < 4; ++y)
      for (size_t x = 0; x < cfl.dc_values.xsize(); ++x)
        cfl.dc_values.Row(y)[x] = EcfSentinel(1 + y, x);
    const size_t tiles_x = (xb + 7) / 8, tiles_y = (yb + 7) / 8;
    status = cfl.PrepareForThreads(1);
    std::cout << ' ' << int(status.code());
    if (!status) return true;
    for (size_t t = 0; t < tiles_x * tiles_y && status; ++t) {
      const size_t tx = t % tiles_x, ty = t / tiles_x;
      const size_t bx0 = tx * 8, by0 = ty * 8;
      const size_t bx1 = std::min(bx0 + 8, xb), by1 = std::min(by0 + 8, yb);
      const jxl::Rect r(bx0, by0, bx1 - bx0, by1 - by0);
      if (pass == 0) {
        status = cfl.ComputeTile(r, opsin, opsin_rect, matrices, nullptr, nullptr, nullptr,
                                 false, 0, &cmap);
      } else {
        status = cfl.ComputeTile(r, opsin, opsin_rect, matrices, &acs, &rqf, &quantizer,
                                 fast != 0, 0, &cmap);
      }
    }
    std::cout << ' ' << int(status.code());
    for (const jxl::ImageSB* map : {&cmap.ytox_map, &cmap.ytob_map}) {
      std::cout << " C";
      for (size_t y = 0; y < map->ysize(); ++y)
        for (size_t x = 0; x < map->xsize(); ++x) std::cout << ' ' << int(map->ConstRow(y)[x]);
    }
    for (size_t y = 0; y < 4; ++y) {
      std::cout << " D";
      for (size_t x = 0; x < cfl.dc_values.xsize(); ++x) EcfWord(cfl.dc_values.ConstRow(y)[x]);
    }
    return true;
  }
  throw std::runtime_error("unknown encoder CfL SIMD command");
}
