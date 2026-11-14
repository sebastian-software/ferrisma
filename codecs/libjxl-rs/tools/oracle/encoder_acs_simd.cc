// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the encoder's AC strategy search (enc_ac_strategy.cc, the
// ProcessRectACS export; port ADR 0004, "Encoder path"), observed at the
// oracle's single Highway target. The functions are the target namespace's
// (ORACLE_HWY_NS, external linkage in the original) and, for the search
// itself, the original's AcStrategyHeuristics, whose ProcessRect reaches the
// one compiled target through HWY_DYNAMIC_DISPATCH; this TU supplies no
// kernel arithmetic. Buffers handed to the original are 128-byte aligned
// and sized as PrepareForThreads sizes them for 8-lane vectors, filled with
// finite sentinel words, and every word the original may write is printed.
//
// Every request carries the same input description, `<inputs>`:
//   <xsize> <ysize> (multiples of 8) <3 * ysize * xsize source words>
//   <ysize/8 * xsize/8 quant field words> <ysize * xsize mask1x1 words>
//   <info_loss_multiplier> <cost_delta> <zeros_mul> <mask1x1_xsize>
// from which an ACSConfig is set up as AcStrategyHeuristics::Init sets it
// (row pointers and strides of the planes; the three multipliers and
// mask1x1_xsize as given), with the dequant matrices of EnsureComputed for
// every strategy below DCT128X128 (the target's ComputeQuantTable).
//   esa-entropy <inputs> <raw strategy> <entropy_mul> <x> <y> <3 cmap words>
//       EstimateEntropy in block (3 * kMaxCoeffArea + 8 words) and the full
//       scratch (3 * kMaxCoeffArea + 3 * 8 * kMaxBlockDim words). Prints the
//       status, the entropy, block up to 3 * size + 8 words and the scratch
//       up to its last written word plus 4.
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
#include "lib/jxl/base/status.h"
#include "lib/jxl/enc_ac_strategy.h"
#include "lib/jxl/image.h"
#include "lib/jxl/quant_weights.h"

namespace jxl {
namespace ORACLE_HWY_NS {
// External-linkage definitions of the target namespace in enc_ac_strategy.cc.
Status EstimateEntropy(const AcStrategy& acs, float entropy_mul, size_t x, size_t y,
                       const ACSConfig& config, const float* JXL_RESTRICT cmap_factors,
                       float* block, float* full_scratch_space, uint32_t* quantized,
                       float& entropy);
}  // namespace ORACLE_HWY_NS
}  // namespace jxl

namespace {
constexpr size_t kMaxLanes = 8;
constexpr size_t kBlockWords = 3 * jxl::AcStrategy::kMaxCoeffArea;
// PrepareForThreads' mem_per_thread past the block, for 8 lanes.
constexpr size_t kScratchWords =
    3 * jxl::AcStrategy::kMaxCoeffArea + 3 * kMaxLanes * jxl::AcStrategy::kMaxBlockDim;

float EsaFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}
uint32_t EsaBits(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  return bits;
}
// Distinct finite sentinel words keep unwritten words observable.
float EsaSentinel(uint32_t salt, size_t i) {
  const uint32_t bits = 0x5a000000u + salt * 0x100000u + uint32_t(i & 0xfffff);
  float v;
  std::memcpy(&v, &bits, sizeof(v));
  return v;
}

// A buffer of `n` floats whose first element lies on a 128-byte boundary,
// filled with sentinel words.
class EsaAligned {
 public:
  EsaAligned(size_t n, uint32_t salt) : storage_(n + 128 / sizeof(float)) {
    const uintptr_t address = reinterpret_cast<uintptr_t>(storage_.data());
    offset_ = ((128 - address % 128) % 128) / sizeof(float);
    for (size_t i = 0; i < n; ++i) data()[i] = EsaSentinel(salt, i);
  }
  float* data() { return storage_.data() + offset_; }

 private:
  std::vector<float> storage_;
  size_t offset_ = 0;
};

void EsaWords(const char* tag, const float* values, size_t count) {
  std::cout << ' ' << tag;
  for (size_t i = 0; i < count; ++i) std::cout << ' ' << EsaBits(values[i]);
}
// The scratch up to its last written word plus 4.
void EsaScratch(const float* scratch, uint32_t salt) {
  size_t used = 0;
  for (size_t i = 0; i < kScratchWords; ++i) {
    const float sentinel = EsaSentinel(salt, i);
    if (std::memcmp(&scratch[i], &sentinel, sizeof(float)) != 0) used = i + 1;
  }
  EsaWords("W", scratch, std::min(kScratchWords, used + 4));
}

jxl::ImageF EsaPlane(JxlMemoryManager* mm, std::istringstream& in, size_t xsize, size_t ysize) {
  auto plane = jxl::ImageF::Create(mm, xsize, ysize);
  if (!plane.ok()) throw std::runtime_error("acs plane allocation");
  jxl::ImageF result = std::move(plane).value_();
  for (size_t y = 0; y < ysize; ++y)
    for (size_t x = 0; x < xsize; ++x) result.Row(y)[x] = EsaFloat(in);
  return result;
}

// The planes and matrices an ACSConfig points to.
struct EsaInputs {
  size_t xsize = 0, ysize = 0;
  std::vector<jxl::ImageF> src;
  jxl::ImageF qf, mask1x1;
  jxl::DequantMatrices matrices;
  jxl::ACSConfig config = {};
};

void EsaReadInputs(JxlMemoryManager* mm, std::istringstream& in, EsaInputs* inputs) {
  in >> inputs->xsize >> inputs->ysize;
  const size_t xsize = inputs->xsize, ysize = inputs->ysize;
  if (xsize % 8 != 0 || ysize % 8 != 0) throw std::runtime_error("acs image size");
  for (size_t c = 0; c < 3; ++c) inputs->src.push_back(EsaPlane(mm, in, xsize, ysize));
  inputs->qf = EsaPlane(mm, in, xsize / 8, ysize / 8);
  inputs->mask1x1 = EsaPlane(mm, in, xsize, ysize);
  jxl::ACSConfig& config = inputs->config;
  config.info_loss_multiplier = EsaFloat(in);
  config.cost_delta = EsaFloat(in);
  config.zeros_mul = EsaFloat(in);
  in >> config.mask1x1_xsize;
  if (config.mask1x1_xsize > xsize) throw std::runtime_error("mask1x1_xsize");
  uint32_t acs_mask = 0;
  for (size_t i = 0; i < static_cast<size_t>(jxl::AcStrategyType::DCT128X128); i++) {
    acs_mask |= (1 << i);
  }
  if (!inputs->matrices.EnsureComputed(mm, acs_mask)) throw std::runtime_error("EnsureComputed");
  config.dequant = &inputs->matrices;
  config.quant_field_row = inputs->qf.Row(0);
  config.quant_field_stride = inputs->qf.PixelsPerRow();
  config.masking1x1_field_row = inputs->mask1x1.Row(0);
  config.masking1x1_field_stride = inputs->mask1x1.PixelsPerRow();
  for (size_t c = 0; c < 3; ++c) config.src_rows[c] = inputs->src[c].ConstRow(0);
  config.src_stride = inputs->src[0].PixelsPerRow();
}
}  // namespace

bool EncoderAcsSimdCommand(const std::string& command, std::istringstream& in,
                           JxlMemoryManager* mm) {
  if (command.rfind("esa-", 0) != 0) return false;
  if (command == "esa-entropy") {
    EsaInputs inputs;
    EsaReadInputs(mm, in, &inputs);
    unsigned raw;
    size_t x, y;
    in >> raw;
    const float entropy_mul = EsaFloat(in);
    in >> x >> y;
    float cmap_factors[3];
    for (float& f : cmap_factors) f = EsaFloat(in);
    if (raw >= static_cast<unsigned>(jxl::AcStrategyType::DCT128X128)) {
      throw std::runtime_error("strategy without matrices");
    }
    const jxl::AcStrategy acs =
        jxl::AcStrategy::FromRawStrategy(static_cast<jxl::AcStrategyType>(raw));
    if (x % 8 != 0 || y % 8 != 0 || x + 8 * acs.covered_blocks_x() > inputs.xsize ||
        y + 8 * acs.covered_blocks_y() > inputs.ysize) {
      throw std::runtime_error("strategy outside the image");
    }
    EsaAligned block(kBlockWords + kMaxLanes, 1);
    EsaAligned scratch(kScratchWords, 2);
    std::vector<uint32_t> quantized(jxl::AcStrategy::kMaxCoeffArea, 0x5a5a5a5au);
    float entropy = -1;
    const jxl::Status status = jxl::ORACLE_HWY_NS::EstimateEntropy(
        acs, entropy_mul, x, y, inputs.config, cmap_factors, block.data(), scratch.data(),
        quantized.data(), entropy);
    const size_t size = (1 << acs.log2_covered_blocks()) * jxl::kDCTBlockSize;
    std::cout << "E " << int(status.code()) << ' ' << EsaBits(entropy);
    EsaWords("B", block.data(), 3 * size + kMaxLanes);
    EsaScratch(scratch.data(), 2);
    return true;
  }
  throw std::runtime_error("unknown encoder acs command");
}
