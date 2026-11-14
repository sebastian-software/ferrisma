// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the encoder's SIMD kernels (port ADR 0004, "Encoder path"),
// observed at the oracle's single Highway target. The kernels are the
// original's public, dispatching functions (HWY_DYNAMIC_DISPATCH reaches
// the one compiled target); this TU supplies no kernel arithmetic. Buffers
// handed to the original are 128-byte aligned and sized for 8-lane vectors,
// as libjxl's own callers size them, and every word the original may write
// is printed.
//   esx-premul <intensity>
//       ComputePremulAbsorb into 12 * 8 + 4 sentinel words; all printed.
//   esx-row <xsize> <intensity> <3 * RoundUpTo(xsize, 8) words>
//       LinearRGBRowToXYB on three rows of RoundUpTo(xsize, 8) + 4 words
//       (the request fills the first RoundUpTo(xsize, 8), sentinels after),
//       with the original's ComputePremulAbsorb constants; all printed.
//   esx-xyb <color encoding> <intensity> <black_mode> <linear_mode> <xsize>
//           <ysize> <3 * ysize * RoundUpTo(xsize, 8) words>
//       ToXYB on an Image3F whose rows hold the request's words in the first
//       RoundUpTo(xsize, 8) samples (row padding included, so whole vectors
//       read defined words); black_mode/linear_mode 0: none, 1: same size,
//       2: one more column/row (the SameSize checks). The image and the
//       linear output (sentinel words) are printed over RoundUpTo(xsize, 8)
//       samples per row. Encodings outside ToXYB's linear-sRGB and sRGB
//       branches are not run (ApplyColorTransform is not translated).
#include <jxl/cms.h>
#include <jxl/color_encoding.h>
#include <jxl/memory_manager.h>

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/base/status.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/image.h"

JxlColorEncoding ReadColorEncoding(std::istringstream& in);

namespace {
constexpr size_t kMaxLanes = 8;

size_t RoundUp8(size_t x) { return (x + kMaxLanes - 1) / kMaxLanes * kMaxLanes; }

float EsxFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}
void EsxWord(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  std::cout << ' ' << bits;
}
void EsxWords(const float* values, size_t count) {
  std::cout << " B";
  for (size_t i = 0; i < count; ++i) EsxWord(values[i]);
}
// Distinct finite sentinel words keep unwritten samples observable (a NaN
// sentinel would equal any NaN the kernel writes).
float EsxSentinel(uint32_t salt, size_t i) {
  const uint32_t bits = 0x5a000000u + salt * 0x100000u + uint32_t(i & 0xfffff);
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}

// A buffer of `n` floats whose first element lies on a 128-byte boundary.
class EsxAligned {
 public:
  explicit EsxAligned(size_t n) : storage_(n + 128 / sizeof(float)) {
    const uintptr_t address = reinterpret_cast<uintptr_t>(storage_.data());
    offset_ = ((128 - address % 128) % 128) / sizeof(float);
  }
  float* data() { return storage_.data() + offset_; }

 private:
  std::vector<float> storage_;
  size_t offset_ = 0;
};

jxl::Image3F EsxImage(JxlMemoryManager* mm, size_t xsize, size_t ysize) {
  auto image = jxl::Image3F::Create(mm, xsize, ysize);
  if (!image.ok()) throw std::runtime_error("encoder SIMD image allocation");
  return std::move(image).value_();
}
void EsxCheckPadding(const jxl::Image3F& image, size_t padded) {
  if (image.PixelsPerRow() < padded) throw std::runtime_error("row padding below 8 lanes");
}
void EsxPrintImage(const jxl::Image3F& image, size_t padded) {
  for (size_t c = 0; c < 3; ++c) {
    std::cout << " P";
    for (size_t y = 0; y < image.ysize(); ++y)
      for (size_t x = 0; x < padded; ++x) EsxWord(image.ConstPlaneRow(c, y)[x]);
  }
}
}  // namespace

bool EncoderSimdCommand(const std::string& command, std::istringstream& in,
                        JxlMemoryManager* mm) {
  if (command.rfind("esx-", 0) != 0) return false;
  if (command == "esx-premul") {
    const float intensity = EsxFloat(in);
    EsxAligned premul(12 * kMaxLanes + 4);
    for (size_t i = 0; i < 12 * kMaxLanes + 4; ++i) premul.data()[i] = EsxSentinel(1, i);
    jxl::ComputePremulAbsorb(intensity, premul.data());
    std::cout << "M";
    EsxWords(premul.data(), 12 * kMaxLanes + 4);
    return true;
  }
  if (command == "esx-row") {
    size_t xsize;
    in >> xsize;
    const float intensity = EsxFloat(in);
    EsxAligned premul(12 * kMaxLanes);
    jxl::ComputePremulAbsorb(intensity, premul.data());
    const size_t padded = RoundUp8(xsize);
    std::vector<EsxAligned> rows;
    for (size_t c = 0; c < 3; ++c) {
      rows.emplace_back(padded + 4);
      float* row = rows.back().data();
      for (size_t x = 0; x < padded; ++x) row[x] = EsxFloat(in);
      for (size_t x = padded; x < padded + 4; ++x) row[x] = EsxSentinel(2 + c, x);
    }
    jxl::LinearRGBRowToXYB(rows[0].data(), rows[1].data(), rows[2].data(), premul.data(), xsize);
    std::cout << "R";
    for (auto& row : rows) EsxWords(row.data(), padded + 4);
    return true;
  }
  if (command == "esx-xyb") {
    const JxlColorEncoding external = ReadColorEncoding(in);
    jxl::ColorEncoding c_current;
    const jxl::Status converted = c_current.FromExternal(external);
    const float intensity = EsxFloat(in);
    unsigned black_mode, linear_mode;
    size_t xsize, ysize;
    in >> black_mode >> linear_mode >> xsize >> ysize;
    const bool same_linear =
        jxl::ColorEncoding::LinearSRGB(c_current.IsGray()).SameColorEncoding(c_current);
    const bool is_srgb = c_current.IsSRGB();
    std::cout << "X " << int(converted.code()) << ' ' << same_linear << ' ' << is_srgb;
    const size_t padded = RoundUp8(xsize);
    auto image = EsxImage(mm, xsize, ysize);
    EsxCheckPadding(image, padded);
    for (size_t c = 0; c < 3; ++c)
      for (size_t y = 0; y < ysize; ++y)
        for (size_t x = 0; x < padded; ++x) image.PlaneRow(c, y)[x] = EsxFloat(in);
    if (!same_linear && !is_srgb) return true;
    jxl::ImageF black;
    if (black_mode != 0) {
      auto plane = jxl::ImageF::Create(mm, xsize + (black_mode == 2), ysize);
      if (!plane.ok()) throw std::runtime_error("encoder SIMD plane allocation");
      black = std::move(plane).value_();
      for (size_t y = 0; y < black.ysize(); ++y)
        std::memset(black.Row(y), 0, black.xsize() * sizeof(float));
    }
    jxl::Image3F linear;
    if (linear_mode != 0) {
      linear = EsxImage(mm, xsize, ysize + (linear_mode == 2));
      EsxCheckPadding(linear, padded);
      for (size_t c = 0; c < 3; ++c)
        for (size_t y = 0; y < linear.ysize(); ++y)
          for (size_t x = 0; x < padded; ++x)
            linear.PlaneRow(c, y)[x] = EsxSentinel(5 + c, y * padded + x);
    }
    const jxl::Status status =
        jxl::ToXYB(c_current, intensity, black_mode != 0 ? &black : nullptr, nullptr, &image,
                   *JxlGetDefaultCms(), linear_mode != 0 ? &linear : nullptr);
    std::cout << ' ' << int(status.code());
    EsxPrintImage(image, padded);
    if (linear_mode != 0) EsxPrintImage(linear, padded);
    return true;
  }
  throw std::runtime_error("unknown encoder SIMD command");
}
