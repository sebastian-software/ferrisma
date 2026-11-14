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
//   esx-pixels <raw strategy> <stride> <rows * stride words>
//       TransformFromPixels from 128-byte aligned pixels (8 sentinel words
//       after them) into 128-byte aligned coefficients (rows * cols + 8
//       sentinel words, all printed) with the scratch the encoder's callers
//       pass (2 * kMaxCoeffArea + 3 * 8 * kMaxBlockDim words, as enc_group.cc
//       for 8 lanes), printed up to its last written word plus 4.
//   esx-llf <raw strategy> <stride> <blocks * 64 words>
//       DCFromLowestFrequencies from the coefficients into a dc buffer of
//       covered_blocks_y * stride + 8 sentinel words, same scratch.
//   esx-afv <16 words>
//       AFVDCT4x4 from 16 + 8 words into 16 + 8 sentinel words.
//   esg-block <quant_dc> <quant_median> <c> <qm_multiplier> <raw strategy>
//             <xsize> <ysize> <4 threshold words> <quant>
//             <64 * xsize * ysize words>
//       QuantizeBlockAC of the target namespace (ORACLE_HWY_NS; the original
//       exports only ComputeCoefficients) with a Quantizer from
//       ComputeGlobalScaleAndQuant over the default matrices, into
//       64 * xsize * ysize + 8 sentinel ints; thresholds and ints printed.
//   esg-coeffs <xsize> <ysize> <speed_tier> <x_qm_multiplier>
//              <b_qm_multiplier> <quant_dc> <quant_median> <n> <n * (bx by
//              raw)> <ytox words> <ytob words> <raw quant field words>
//              <3 * ysize_blocks * 8 * xsize_blocks * 8 opsin words>
//       ComputeCoefficients for every group of a PassesEncoderState set up as
//       InitializePassesEncoder does (one pass, 32-bit coefficients), with
//       the request's strategies (DCT elsewhere), color correlation tiles,
//       raw quant field and block-padded opsin image. Prints the status, the
//       quant field, the DC image and each group's coefficients up to the
//       group's used length plus 8 (within the group's row).
#include <jxl/cms.h>
#include <jxl/color_encoding.h>
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
#include "lib/jxl/common.h"
#include "lib/jxl/dct_util.h"
#include "lib/jxl/enc_cache.h"
#include "lib/jxl/enc_group.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/quant_weights.h"
#include "lib/jxl/quantizer.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/enc_transforms.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/image.h"

JxlColorEncoding ReadColorEncoding(std::istringstream& in);

namespace jxl {
namespace ORACLE_HWY_NS {
void QuantizeBlockAC(const Quantizer& quantizer, const bool error_diffusion,
                     size_t c, float qm_multiplier, AcStrategyType quant_kind,
                     size_t xsize, size_t ysize, float* thresholds,
                     const float* JXL_RESTRICT block_in, const int32_t* quant,
                     int32_t* JXL_RESTRICT block_out);
}  // namespace ORACLE_HWY_NS
}  // namespace jxl

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
uint32_t EsxBitsOf(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  return bits;
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
// The scratch of the encoder's callers (enc_group.cc ComputeCoefficients:
// scratch_space = fmem + 3 * kMaxCoeffArea within 5 * kMaxCoeffArea +
// dct_scratch_size), for 8 lanes.
constexpr size_t kCallerScratch =
    2 * jxl::AcStrategy::kMaxCoeffArea + 3 * kMaxLanes * jxl::AcStrategy::kMaxBlockDim;
void EsxScratch(const float* scratch) {
  size_t used = 0;
  for (size_t i = 0; i < kCallerScratch; ++i) {
    const float sentinel = EsxSentinel(9, i);
    if (std::memcmp(&scratch[i], &sentinel, sizeof(float)) != 0) used = i + 1;
  }
  EsxWords(scratch, std::min(kCallerScratch, used + 4));
}
}  // namespace

bool EncoderSimdCommand(const std::string& command, std::istringstream& in,
                        JxlMemoryManager* mm) {
  if (command.rfind("esx-", 0) != 0 && command.rfind("esg-", 0) != 0) return false;
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
  if (command == "esx-pixels" || command == "esx-llf") {
    unsigned raw;
    size_t stride;
    in >> raw >> stride;
    const auto acs = jxl::AcStrategy::FromRawStrategy(static_cast<uint8_t>(raw));
    const auto strategy = static_cast<jxl::AcStrategyType>(raw);
    const size_t bx = acs.covered_blocks_x(), by = acs.covered_blocks_y();
    EsxAligned scratch(kCallerScratch);
    for (size_t i = 0; i < kCallerScratch; ++i) scratch.data()[i] = EsxSentinel(9, i);
    if (command == "esx-pixels") {
      const size_t rows = by * 8, cols = bx * 8;
      EsxAligned pixels(rows * stride + 8);
      for (size_t i = 0; i < rows * stride; ++i) pixels.data()[i] = EsxFloat(in);
      for (size_t i = rows * stride; i < rows * stride + 8; ++i) pixels.data()[i] = EsxSentinel(10, i);
      EsxAligned coefficients(rows * cols + 8);
      for (size_t i = 0; i < rows * cols + 8; ++i) coefficients.data()[i] = EsxSentinel(11, i);
      jxl::TransformFromPixels(strategy, pixels.data(), stride, coefficients.data(), scratch.data());
      std::cout << "T";
      EsxWords(pixels.data(), rows * stride + 8);
      EsxWords(coefficients.data(), rows * cols + 8);
    } else {
      EsxAligned block(by * bx * 64 + 8);
      for (size_t i = 0; i < by * bx * 64; ++i) block.data()[i] = EsxFloat(in);
      for (size_t i = by * bx * 64; i < by * bx * 64 + 8; ++i) block.data()[i] = EsxSentinel(12, i);
      EsxAligned dc(by * stride + 8);
      for (size_t i = 0; i < by * stride + 8; ++i) dc.data()[i] = EsxSentinel(13, i);
      jxl::DCFromLowestFrequencies(strategy, block.data(), dc.data(), stride, scratch.data());
      std::cout << "L";
      EsxWords(block.data(), by * bx * 64 + 8);
      EsxWords(dc.data(), by * stride + 8);
    }
    EsxScratch(scratch.data());
    return true;
  }
  if (command == "esx-afv") {
    EsxAligned pixels(24), coeffs(24);
    for (size_t i = 0; i < 16; ++i) pixels.data()[i] = EsxFloat(in);
    for (size_t i = 16; i < 24; ++i) pixels.data()[i] = EsxSentinel(14, i);
    for (size_t i = 0; i < 24; ++i) coeffs.data()[i] = EsxSentinel(15, i);
    jxl::AFVDCT4x4(pixels.data(), coeffs.data());
    std::cout << "A";
    EsxWords(pixels.data(), 24);
    EsxWords(coeffs.data(), 24);
    return true;
  }
  if (command == "esg-block") {
    const float quant_dc = EsxFloat(in), quant_median = EsxFloat(in);
    size_t c;
    in >> c;
    const float qm_multiplier = EsxFloat(in);
    unsigned raw;
    size_t xsize, ysize;
    in >> raw >> xsize >> ysize;
    float thresholds[4];
    for (float& t : thresholds) t = EsxFloat(in);
    int32_t quant;
    in >> quant;
    jxl::DequantMatrices matrices;
    if (!matrices.EnsureComputed(mm, ~0u)) throw std::runtime_error("matrices");
    jxl::Quantizer quantizer(matrices);
    quantizer.ComputeGlobalScaleAndQuant(quant_dc, quant_median, 0);
    const size_t n = 64 * xsize * ysize;
    EsxAligned block_in(n + 8);
    for (size_t i = 0; i < n; ++i) block_in.data()[i] = EsxFloat(in);
    for (size_t i = n; i < n + 8; ++i) block_in.data()[i] = EsxSentinel(16, i);
    std::vector<int32_t> storage(n + 8 + 32);
    int32_t* out = storage.data();
    while (reinterpret_cast<uintptr_t>(out) % 128 != 0) ++out;
    for (size_t i = 0; i < n + 8; ++i) out[i] = int32_t(0x5a5a0000 + i);
    jxl::ORACLE_HWY_NS::QuantizeBlockAC(quantizer, false, c, qm_multiplier,
                                        static_cast<jxl::AcStrategyType>(raw), xsize, ysize,
                                        thresholds, block_in.data(), &quant, out);
    std::cout << "Q " << EsxBitsOf(quantizer.Scale());
    EsxWords(thresholds, 4);
    std::cout << " I";
    for (size_t i = 0; i < n + 8; ++i) std::cout << ' ' << out[i];
    return true;
  }
  if (command == "esg-coeffs") {
    size_t xsize, ysize;
    int speed_tier;
    in >> xsize >> ysize >> speed_tier;
    const float x_qm = EsxFloat(in), b_qm = EsxFloat(in);
    const float quant_dc = EsxFloat(in), quant_median = EsxFloat(in);
    jxl::PassesEncoderState enc_state(mm);
    enc_state.cparams.speed_tier = static_cast<jxl::SpeedTier>(speed_tier);
    enc_state.x_qm_multiplier = x_qm;
    enc_state.b_qm_multiplier = b_qm;
    jxl::PassesSharedState& shared = enc_state.shared;
    shared.frame_dim.Set(xsize, ysize, 1, 0, 0, false, 1);
    const jxl::FrameDimensions& fd = shared.frame_dim;
    auto acs = jxl::AcStrategyImage::Create(mm, fd.xsize_blocks, fd.ysize_blocks);
    if (!acs.ok()) throw std::runtime_error("ac strategy");
    shared.ac_strategy = std::move(acs).value_();
    shared.ac_strategy.FillDCT8();
    size_t n;
    in >> n;
    for (size_t i = 0; i < n; ++i) {
      size_t bx, by;
      unsigned raw;
      in >> bx >> by >> raw;
      if (!shared.ac_strategy.Set(bx, by, static_cast<jxl::AcStrategyType>(raw)))
        throw std::runtime_error("strategy placement");
    }
    auto cmap = jxl::ColorCorrelationMap::Create(mm, fd.xsize, fd.ysize);
    if (!cmap.ok()) throw std::runtime_error("cmap");
    shared.cmap = std::move(cmap).value_();
    for (jxl::ImageSB* map : {&shared.cmap.ytox_map, &shared.cmap.ytob_map})
      for (size_t y = 0; y < map->ysize(); ++y)
        for (size_t x = 0; x < map->xsize(); ++x) {
          int v;
          in >> v;
          map->Row(y)[x] = static_cast<int8_t>(v);
        }
    auto rqf = jxl::ImageI::Create(mm, fd.xsize_blocks, fd.ysize_blocks);
    if (!rqf.ok()) throw std::runtime_error("quant field");
    shared.raw_quant_field = std::move(rqf).value_();
    for (size_t y = 0; y < fd.ysize_blocks; ++y)
      for (size_t x = 0; x < fd.xsize_blocks; ++x) in >> shared.raw_quant_field.Row(y)[x];
    if (!shared.matrices.EnsureComputed(mm, ~0u)) throw std::runtime_error("matrices");
    shared.quantizer.ComputeGlobalScaleAndQuant(quant_dc, quant_median, 0);
    auto opsin = EsxImage(mm, fd.xsize_blocks * 8, fd.ysize_blocks * 8);
    for (size_t c = 0; c < 3; ++c)
      for (size_t y = 0; y < opsin.ysize(); ++y)
        for (size_t x = 0; x < opsin.xsize(); ++x) opsin.PlaneRow(c, y)[x] = EsxFloat(in);
    auto dc = EsxImage(mm, fd.xsize_blocks, fd.ysize_blocks);
    for (size_t c = 0; c < 3; ++c)
      for (size_t y = 0; y < dc.ysize(); ++y)
        for (size_t x = 0; x < dc.xsize(); ++x) dc.PlaneRow(c, y)[x] = EsxSentinel(17 + c, y * 4096 + x);
    auto coeffs = jxl::ACImageT<int32_t>::Make(mm, jxl::kGroupDim * jxl::kGroupDim, fd.num_groups);
    if (!coeffs.ok()) throw std::runtime_error("coefficients");
    enc_state.coeffs.emplace_back(std::move(coeffs).value_());
    for (size_t c = 0; c < 3; ++c)
      for (size_t g = 0; g < fd.num_groups; ++g) {
        int32_t* row = enc_state.coeffs[0]->PlaneRow(c, g, 0).ptr32;
        for (size_t i = 0; i < jxl::kGroupDim * jxl::kGroupDim; ++i) row[i] = int32_t(0x5a5a0000 + i);
      }
    jxl::Status status = true;
    for (size_t g = 0; g < fd.num_groups && status; ++g)
      status = jxl::ComputeCoefficients(g, &enc_state, opsin, jxl::Rect(opsin), &dc);
    std::cout << "C " << int(status.code()) << " R";
    for (size_t y = 0; y < fd.ysize_blocks; ++y)
      for (size_t x = 0; x < fd.xsize_blocks; ++x) std::cout << ' ' << shared.raw_quant_field.Row(y)[x];
    for (size_t c = 0; c < 3; ++c) {
      std::cout << " D";
      for (size_t y = 0; y < dc.ysize(); ++y)
        for (size_t x = 0; x < dc.xsize(); ++x) EsxWord(dc.ConstPlaneRow(c, y)[x]);
    }
    for (size_t g = 0; g < fd.num_groups; ++g) {
      const size_t used = 64 * fd.BlockGroupRect(g).xsize() * fd.BlockGroupRect(g).ysize();
      for (size_t c = 0; c < 3; ++c) {
        const int32_t* row = enc_state.coeffs[0]->PlaneRow(c, g, 0).ptr32;
        std::cout << " G";
        const size_t shown = std::min(used + 8, jxl::kGroupDim * jxl::kGroupDim);
        for (size_t i = 0; i < shown; ++i) std::cout << ' ' << row[i];
      }
    }
    return true;
  }
  throw std::runtime_error("unknown encoder SIMD command");
}
