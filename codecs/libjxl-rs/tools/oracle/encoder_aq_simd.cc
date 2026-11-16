// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the encoder's SIMD AdaptiveQuantizationMap (port ADR 0004,
// "Encoder path"), observed at the oracle's single Highway target. The
// export is reached through the original's public InitialQuantField, whose
// HWY_DYNAMIC_DISPATCH reaches the one compiled target; this TU supplies no
// kernel arithmetic.
//   eaq-map <distance> <rescale> <xsize> <ysize> <x0> <y0> <rect xsize>
//           <rect ysize> <pad kind> <3 * ysize * xsize words>
//       InitialQuantField(distance, opsin, rect, nullptr, rescale, &mask,
//       &mask1x1) on an Image3F (rows 128-byte aligned, padded for 8-lane
//       vectors by Image3F::Create) whose samples are the request's words
//       and whose row padding, up to PixelsPerRow, holds the pad kind's word
//       (0: +0.0, 1: a quiet NaN, 2: +inf, 3: -1e30): HfModulation's last
//       vector reads one sample past each block, masked out. Prints the
//       status and the three planes the original writes (aq_map, mask,
//       mask1x1) with their sizes, every sample. The row stride is not
//       printed: the original's follows its target's MaxVectorSize, the
//       port's encoder planes follow the scalar one (ADR 0004, "Encoder
//       path"); both leave at least one padding sample.
#include <jxl/memory_manager.h>

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>

#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/enc_adaptive_quantization.h"
#include "lib/jxl/image.h"

namespace {
float EaqFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}
uint32_t EaqBits(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  return bits;
}
float EaqPad(unsigned kind) {
  const uint32_t words[4] = {0x00000000u, 0x7fc00000u, 0x7f800000u, 0xf149f2cau};
  if (kind >= 4) throw std::runtime_error("encoder aq pad kind");
  float v;
  std::memcpy(&v, &words[kind], sizeof(v));
  return v;
}
void EaqPrintPlane(const jxl::ImageF& plane) {
  std::cout << " P " << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << EaqBits(plane.ConstRow(y)[x]);
}
}  // namespace

bool EncoderAqSimdCommand(const std::string& command, std::istringstream& in,
                          JxlMemoryManager* mm) {
  if (command.rfind("eaq-", 0) != 0) return false;
  if (command == "eaq-map") {
    const float distance = EaqFloat(in);
    const float rescale = EaqFloat(in);
    size_t xsize, ysize, x0, y0, rxs, rys;
    unsigned pad_kind;
    in >> xsize >> ysize >> x0 >> y0 >> rxs >> rys >> pad_kind;
    const float pad = EaqPad(pad_kind);
    auto created = jxl::Image3F::Create(mm, xsize, ysize);
    if (!created.ok()) throw std::runtime_error("encoder aq image allocation");
    jxl::Image3F opsin = std::move(created).value_();
    const size_t stride = opsin.PixelsPerRow();
    if (stride < xsize + 1) throw std::runtime_error("no row padding");
    for (size_t c = 0; c < 3; ++c)
      for (size_t y = 0; y < ysize; ++y) {
        float* row = opsin.PlaneRow(c, y);
        for (size_t x = 0; x < xsize; ++x) row[x] = EaqFloat(in);
        for (size_t x = xsize; x < stride; ++x) row[x] = pad;
      }
    jxl::ImageF mask, mask1x1;
    auto qf = jxl::InitialQuantField(distance, opsin, jxl::Rect(x0, y0, rxs, rys), nullptr,
                                     rescale, &mask, &mask1x1);
    std::cout << "Q " << int(qf.status().code());
    if (!qf.ok()) return true;
    const jxl::ImageF field = std::move(qf).value_();
    EaqPrintPlane(field);
    EaqPrintPlane(mask);
    EaqPrintPlane(mask1x1);
    return true;
  }
  throw std::runtime_error("unknown encoder aq command");
}
