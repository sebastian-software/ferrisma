// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Instantiate the pristine scalar TransformFromPixels and AFVDCT4x4 from
// enc_transforms-inl.h in a dedicated observation unit for competing NaN
// payloads only. Their selection is a register-allocation choice: in the
// 8-point and larger DCTs (native AArch64) and in the AFV basis sum (native
// x86-64), the ordinary O3 archive object and its ASan counterpart pick
// different operands, so neither is a stable reference. Source-scoped
// -fno-unroll-loops, as for dct.cc, keeps one loop shape in both graphs;
// default contraction, O3 and all sanitizers stay active. All other requests
// call the archive's own objects. This adapter supplies no transform
// algorithm.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "lib/jxl/enc_transforms-inl.h"

bool EncoderTransformsUnitCommand(const std::string& command, std::istringstream& in) {
  if (command != "etx-unit-pixels" && command != "etx-unit-afv") return false;
  const auto words = [](const std::vector<float>& values) {
    std::cout << " B";
    for (float v : values) { uint32_t b; std::memcpy(&b, &v, sizeof(b)); std::cout << ' ' << b; }
  };
  constexpr float kCanary = -123.25f;
  if (command == "etx-unit-afv") {
    std::vector<float> pixels(20, kCanary), coeffs(20, kCanary);
    for (size_t i = 0; i < 16; ++i) {
      uint32_t bits; in >> bits; std::memcpy(&pixels[i], &bits, sizeof(bits));
    }
    jxl::N_SCALAR::AFVDCT4x4(pixels.data(), coeffs.data());
    std::cout << "A"; words(pixels); words(coeffs);
    return true;
  }
  unsigned raw; size_t stride; in >> raw >> stride;
  auto acs = jxl::AcStrategy::FromRawStrategy(static_cast<uint8_t>(raw));
  size_t rows = acs.covered_blocks_y() * 8, cols = acs.covered_blocks_x() * 8;
  std::vector<float> pixels(rows * stride + 4, kCanary);
  for (size_t i = 0; i < rows * stride; ++i) {
    uint32_t bits; in >> bits; std::memcpy(&pixels[i], &bits, sizeof(bits));
  }
  std::vector<float> coefficients(rows * cols + 4, kCanary);
  std::vector<float> scratch(rows * cols * 4 + 3 * (rows + cols) + 4, kCanary);
  jxl::N_SCALAR::TransformFromPixels(static_cast<jxl::AcStrategyType>(raw), pixels.data(), stride,
                                     coefficients.data(), scratch.data());
  std::cout << "T"; words(pixels); words(coefficients); words(scratch);
  return true;
}
