// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call the original encoder transform, Gaborish, convolution and XYB
// functions in the complete archive: Symmetric5, SlowSymmetric3,
// GaborishInverse, TransformFromPixels, DCFromLowestFrequencies, AFVDCT4x4,
// ComputePremulAbsorb, LinearRGBRowToXYB and ToXYB keep their library objects
// and flags. Owners start with recorded canary words; this adapter supplies
// no algorithm. GaborishKernel is extracted verbatim from enc_gaborish_test.cc
// (see source_fragments.json); the test's roundtrip sequence calls the
// original functions in the test's order.
#include <jxl/cms.h>
#include <jxl/color_encoding.h>
#include <jxl/memory_manager.h>

#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include <hwy/base.h>

#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/convolve.h"
#include "lib/jxl/enc_gaborish.h"
#include "lib/jxl/enc_transforms.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_ops.h"

JxlColorEncoding ReadColorEncoding(std::istringstream& in);

namespace jxl {
namespace {
#include "original_gaborish_kernel.inc"
}  // namespace
}  // namespace jxl

namespace {
constexpr float kCanary = -123.25f;

float EtxFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
uint32_t EtxBits(float v) { uint32_t bits; std::memcpy(&bits, &v, sizeof(bits)); return bits; }
void EtxWords(const float* values, size_t count) {
  std::cout << " B";
  for (size_t i = 0; i < count; ++i) std::cout << ' ' << EtxBits(values[i]);
}
jxl::Rect EtxRect(std::istringstream& in) {
  size_t x0, y0, xs, ys; in >> x0 >> y0 >> xs >> ys; return jxl::Rect(x0, y0, xs, ys);
}
jxl::ImageF EtxPlane(JxlMemoryManager* mm, size_t xsize, size_t ysize) {
  auto plane = jxl::ImageF::Create(mm, xsize, ysize);
  if (!plane.ok()) throw std::runtime_error("encoder transform plane allocation");
  return std::move(plane).value_();
}
jxl::Image3F EtxImage(JxlMemoryManager* mm, size_t xsize, size_t ysize) {
  auto image = jxl::Image3F::Create(mm, xsize, ysize);
  if (!image.ok()) throw std::runtime_error("encoder transform image allocation");
  return std::move(image).value_();
}
void EtxReadPlane(std::istringstream& in, jxl::ImageF* plane) {
  for (size_t y = 0; y < plane->ysize(); ++y)
    for (size_t x = 0; x < plane->xsize(); ++x) plane->Row(y)[x] = EtxFloat(in);
}
// Distinct canary words keep unwritten output samples observable.
void EtxCanary(jxl::ImageF* plane, uint32_t salt) {
  for (size_t y = 0; y < plane->ysize(); ++y)
    for (size_t x = 0; x < plane->xsize(); ++x) {
      uint32_t bits = 0x7fa00000u + salt * 0x10000u + uint32_t(y * 256 + x);
      std::memcpy(&plane->Row(y)[x], &bits, sizeof(bits));
    }
}
void EtxPrintPlane(const jxl::ImageF& plane) {
  std::cout << " P";
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << EtxBits(plane.ConstRow(y)[x]);
}
void EtxPrintImage(const jxl::Image3F& image) {
  for (size_t c = 0; c < 3; ++c) EtxPrintPlane(image.Plane(c));
}
jxl::WeightsSymmetric5 EtxWeights5(std::istringstream& in) {
  jxl::WeightsSymmetric5 w;
  for (float* p : {w.c, w.r, w.R, w.d, w.D, w.L}) {
    float v = EtxFloat(in);
    for (size_t i = 0; i < 4; ++i) p[i] = v;
  }
  return w;
}
}  // namespace

bool EncoderTransformsCommand(const std::string& command, std::istringstream& in,
                              JxlMemoryManager* mm) {
  if (command.rfind("etx-", 0) != 0) return false;
  if (command == "etx-pixels" || command == "etx-llf") {
    unsigned raw; size_t stride; in >> raw >> stride;
    auto acs = jxl::AcStrategy::FromRawStrategy(static_cast<uint8_t>(raw));
    const auto strategy = static_cast<jxl::AcStrategyType>(raw);
    if (command == "etx-pixels") {
      size_t rows = acs.covered_blocks_y() * 8, cols = acs.covered_blocks_x() * 8;
      std::vector<float> pixels(rows * stride + 4, kCanary);
      for (size_t i = 0; i < rows * stride; ++i) pixels[i] = EtxFloat(in);
      std::vector<float> coefficients(rows * cols + 4, kCanary);
      std::vector<float> scratch(rows * cols * 4 + 3 * (rows + cols) + 4, kCanary);
      jxl::TransformFromPixels(strategy, pixels.data(), stride, coefficients.data(),
                               scratch.data());
      std::cout << "T";
      EtxWords(pixels.data(), pixels.size());
      EtxWords(coefficients.data(), coefficients.size());
      EtxWords(scratch.data(), scratch.size());
    } else {
      size_t rows = acs.covered_blocks_y(), cols = acs.covered_blocks_x();
      std::vector<float> block(rows * cols * 64 + 4, kCanary);
      for (size_t i = 0; i < rows * cols * 64; ++i) block[i] = EtxFloat(in);
      std::vector<float> dc(rows * stride + 4, kCanary);
      std::vector<float> scratch(4 * 32 * 32 + 4, kCanary);
      jxl::DCFromLowestFrequencies(strategy, block.data(), dc.data(), stride, scratch.data());
      std::cout << "L";
      EtxWords(block.data(), block.size());
      EtxWords(dc.data(), dc.size());
      EtxWords(scratch.data(), scratch.size());
    }
    return true;
  }
  if (command == "etx-afv") {
    alignas(64) float pixels[16 + 4];
    alignas(64) float coeffs[16 + 4];
    for (size_t i = 0; i < 16; ++i) pixels[i] = EtxFloat(in);
    for (size_t i = 16; i < 20; ++i) pixels[i] = kCanary;
    for (float& v : coeffs) v = kCanary;
    jxl::AFVDCT4x4(pixels, coeffs);
    std::cout << "A";
    EtxWords(pixels, 20);
    EtxWords(coeffs, 20);
    return true;
  }
  if (command == "etx-s5") {
    size_t xsize, ysize, oxsize, oysize; in >> xsize >> ysize;
    const jxl::Rect in_rect = EtxRect(in);
    in >> oxsize >> oysize;
    const jxl::Rect out_rect = EtxRect(in);
    const auto weights = EtxWeights5(in);
    auto input = EtxPlane(mm, xsize, ysize);
    EtxReadPlane(in, &input);
    auto out = EtxPlane(mm, oxsize, oysize);
    EtxCanary(&out, 1);
    const jxl::Status status = jxl::Symmetric5(input, in_rect, weights, nullptr, &out, out_rect);
    std::cout << "S " << int(status.code());
    EtxPrintPlane(input);
    EtxPrintPlane(out);
    return true;
  }
  if (command == "etx-s3") {
    size_t xsize, ysize; in >> xsize >> ysize;
    const jxl::Rect rect = EtxRect(in);
    jxl::WeightsSymmetric3 w;
    for (float* p : {w.c, w.r, w.d}) {
      float v = EtxFloat(in);
      for (size_t i = 0; i < 4; ++i) p[i] = v;
    }
    auto input = EtxPlane(mm, xsize, ysize);
    EtxReadPlane(in, &input);
    auto out = EtxPlane(mm, xsize, ysize);
    EtxCanary(&out, 2);
    const jxl::Status status = jxl::SlowSymmetric3(input, rect, w, nullptr, &out);
    std::cout << "W " << int(status.code());
    EtxPrintPlane(out);
    return true;
  }
  if (command == "etx-gab") {
    size_t xsize, ysize; in >> xsize >> ysize;
    const jxl::Rect rect = EtxRect(in);
    float mul[3];
    for (float& m : mul) m = EtxFloat(in);
    auto image = EtxImage(mm, xsize, ysize);
    for (size_t c = 0; c < 3; ++c) EtxReadPlane(in, &image.Plane(c));
    const jxl::Status status = jxl::GaborishInverse(&image, rect, mul, nullptr);
    std::cout << "G " << int(status.code());
    EtxPrintImage(image);
    return true;
  }
  if (command == "etx-gabtest") {
    // enc_gaborish_test.cc: TestZero (0), TestFlat (1), and the
    // upstream-disabled TestDirac input (2), through TestRoundTrip.
    unsigned which; in >> which;
    auto input = EtxImage(mm, 20, 20);
    if (which == 1) {
      jxl::FillImage(1.0f, &input);
    } else {
      jxl::ZeroFillImage(&input);
      if (which == 2) input.PlaneRow(1, 10)[10] = 10.0f;
    }
    auto fwd = EtxImage(mm, input.xsize(), input.ysize());
    jxl::ThreadPool* null_pool = nullptr;
    const jxl::WeightsSymmetric3 kernel = jxl::GaborishKernel(0, 0);
    std::cout << "K " << EtxBits(kernel.c[0]) << ' ' << EtxBits(kernel.r[0]) << ' '
              << EtxBits(kernel.d[0]);
    for (size_t c = 0; c < 3; ++c) {
      // ConvolveGaborish: SameSize, then SlowSymmetric3 over Rect(in).
      const jxl::Status s = jxl::SlowSymmetric3(input.Plane(c), jxl::Rect(input.Plane(c)), kernel,
                                                null_pool, &fwd.Plane(c));
      std::cout << ' ' << int(s.code());
    }
    EtxPrintImage(fwd);
    float w = 0.92718927264540152f;
    float weights[3] = {w, w, w};
    const jxl::Status status = jxl::GaborishInverse(&fwd, jxl::Rect(fwd), weights, null_pool);
    std::cout << " G " << int(status.code());
    EtxPrintImage(fwd);
    return true;
  }
  if (command == "etx-premul") {
    const float intensity = EtxFloat(in);
    float premul[12 + 4];
    for (float& v : premul) v = kCanary;
    jxl::ComputePremulAbsorb(intensity, premul);
    std::cout << "M";
    EtxWords(premul, 16);
    return true;
  }
  if (command == "etx-row") {
    size_t xsize; in >> xsize;
    const float intensity = EtxFloat(in);
    float premul[12];
    jxl::ComputePremulAbsorb(intensity, premul);
    std::vector<float> rows[3];
    for (auto& row : rows) {
      row.assign(xsize + 4, kCanary);
      for (size_t x = 0; x < xsize; ++x) row[x] = EtxFloat(in);
    }
    jxl::LinearRGBRowToXYB(rows[0].data(), rows[1].data(), rows[2].data(), premul, xsize);
    std::cout << "R";
    for (const auto& row : rows) EtxWords(row.data(), row.size());
    return true;
  }
  if (command == "etx-xyb") {
    const JxlColorEncoding external = ReadColorEncoding(in);
    jxl::ColorEncoding c_current;
    const jxl::Status converted = c_current.FromExternal(external);
    const float intensity = EtxFloat(in);
    unsigned black_mode, linear_mode; size_t xsize, ysize;
    in >> black_mode >> linear_mode >> xsize >> ysize;
    // The original ToXYB branch predicates, in its order.
    const bool same_linear =
        jxl::ColorEncoding::LinearSRGB(c_current.IsGray()).SameColorEncoding(c_current);
    const bool is_srgb = c_current.IsSRGB();
    std::cout << "X " << int(converted.code()) << ' ' << same_linear << ' ' << is_srgb;
    auto image = EtxImage(mm, xsize, ysize);
    for (size_t c = 0; c < 3; ++c) EtxReadPlane(in, &image.Plane(c));
    if (!same_linear && !is_srgb) return true;  // ApplyColorTransform: not compared
    jxl::ImageF black;
    if (black_mode != 0) {
      black = EtxPlane(mm, xsize + (black_mode == 2), ysize);
      jxl::ZeroFillImage(&black);
    }
    jxl::Image3F linear;
    if (linear_mode != 0) {
      linear = EtxImage(mm, xsize, ysize + (linear_mode == 2));
      for (size_t c = 0; c < 3; ++c) EtxCanary(&linear.Plane(c), 3 + c);
    }
    const jxl::Status status =
        jxl::ToXYB(c_current, intensity, black_mode != 0 ? &black : nullptr, nullptr, &image,
                   *JxlGetDefaultCms(), linear_mode != 0 ? &linear : nullptr);
    std::cout << ' ' << int(status.code());
    EtxPrintImage(image);
    if (linear_mode != 0) EtxPrintImage(linear);
    return true;
  }
  throw std::runtime_error("unknown encoder transform command");
}
