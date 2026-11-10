// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call the original quantization heuristics in the complete archive:
// Quantizer::ComputeGlobalScaleAndQuant, SetQuantField, SetQuantFieldRect and
// SetQuant; InitialQuantDC, InitialQuantField and AdjustQuantField;
// CfLHeuristics with the external N_SCALAR::FindBestMultiplier; and
// AcStrategyHeuristics with the external N_SCALAR::EstimateEntropy and
// FindBest8x8Transform. The `eqh-chain` request runs these original
// functions, with ToXYB, PadImageToBlockMultipleInPlace and GaborishInverse,
// in LossyFrameHeuristics' kSquirrel order for one serial thread. This
// adapter supplies no algorithm; owners start with recorded canary words.
#include <jxl/cms.h>
#include <jxl/memory_manager.h>

#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/span.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/enc_ac_strategy.h"
#include "lib/jxl/enc_adaptive_quantization.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_chroma_from_luma.h"
#include "lib/jxl/enc_gaborish.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/memory_manager_internal.h"
#include "lib/jxl/quant_weights.h"
#include "lib/jxl/quantizer.h"
// GetSomeTestImage is the extracted original helper in decode_test_inputs.cc.
#include "lib/jxl/test_image.h"

namespace jxl {
namespace N_SCALAR {
// External-linkage HWY_SCALAR definitions in enc_ac_strategy.cc and
// enc_chroma_from_luma.cc.
Status EstimateEntropy(const AcStrategy& acs, float entropy_mul, size_t x, size_t y,
                       const ACSConfig& config, const float* JXL_RESTRICT cmap_factors,
                       float* block, float* full_scratch_space, uint32_t* quantized,
                       float& entropy);
Status FindBest8x8Transform(size_t x, size_t y, int encoding_speed_tier,
                            float butteraugli_target, const ACSConfig& config,
                            const float* JXL_RESTRICT cmap_factors,
                            AcStrategyImage* JXL_RESTRICT ac_strategy, float* block,
                            float* scratch_space, uint32_t* quantized, float* entropy_out,
                            AcStrategyType& best_tx);
int32_t FindBestMultiplier(const float* values_m, const float* values_s, size_t num,
                           float base, float distance_mul, bool fast);
}  // namespace N_SCALAR
}  // namespace jxl

namespace {
uint32_t EqhBits(float v) { uint32_t b; std::memcpy(&b, &v, sizeof(b)); return b; }
float EqhFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
jxl::Rect EqhRect(std::istringstream& in) {
  size_t x0, y0, xs, ys; in >> x0 >> y0 >> xs >> ys; return jxl::Rect(x0, y0, xs, ys);
}
jxl::ImageF EqhPlane(JxlMemoryManager* mm, size_t xsize, size_t ysize) {
  auto plane = jxl::ImageF::Create(mm, xsize, ysize);
  if (!plane.ok()) throw std::runtime_error("heuristics plane allocation");
  return std::move(plane).value_();
}
void EqhReadPlane(std::istringstream& in, jxl::ImageF* plane) {
  for (size_t y = 0; y < plane->ysize(); ++y)
    for (size_t x = 0; x < plane->xsize(); ++x) plane->Row(y)[x] = EqhFloat(in);
}
jxl::Image3F EqhImage(JxlMemoryManager* mm, std::istringstream& in, size_t xsize, size_t ysize) {
  auto image = jxl::Image3F::Create(mm, xsize, ysize);
  if (!image.ok()) throw std::runtime_error("heuristics image allocation");
  jxl::Image3F result = std::move(image).value_();
  for (size_t c = 0; c < 3; ++c) EqhReadPlane(in, &result.Plane(c));
  return result;
}
void EqhPrintPlane(const jxl::ImageF& plane) {
  std::cout << " P " << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << EqhBits(plane.ConstRow(y)[x]);
}
void EqhPrintInts(const jxl::ImageI& plane) {
  std::cout << " I";
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << plane.ConstRow(y)[x];
}
void EqhPrintMap(const jxl::ImageSB& plane) {
  std::cout << " C";
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << int(plane.ConstRow(y)[x]);
}
void EqhPrintStrategy(const jxl::AcStrategyImage& acs) {
  std::cout << " S";
  for (size_t y = 0; y < acs.ysize(); ++y) {
    const jxl::AcStrategyRow row = acs.ConstRow(y);
    for (size_t x = 0; x < acs.xsize(); ++x) {
      std::cout << ' ' << int(row[x].RawStrategy()) << ':' << row[x].IsFirstBlock();
    }
  }
}
void EqhPrintQuantizer(const jxl::Quantizer& q) {
  const jxl::QuantizerParams params = q.GetParams();
  std::cout << " Q " << params.global_scale << ' ' << params.quant_dc << ' ' << EqhBits(q.Scale())
            << ' ' << EqhBits(q.InvGlobalScale()) << ' ' << EqhBits(q.inv_quant_dc());
  for (size_t c = 0; c < 3; ++c) std::cout << ' ' << EqhBits(q.MulDC()[c]);
  for (size_t c = 0; c < 3; ++c) std::cout << ' ' << EqhBits(q.InvMulDC()[c]);
}
jxl::ImageI EqhInts(JxlMemoryManager* mm, size_t xsize, size_t ysize) {
  auto plane = jxl::ImageI::Create(mm, xsize, ysize);
  if (!plane.ok()) throw std::runtime_error("heuristics int plane allocation");
  jxl::ImageI result = std::move(plane).value_();
  for (size_t y = 0; y < ysize; ++y)
    for (size_t x = 0; x < xsize; ++x) result.Row(y)[x] = -7000 - int(y * 64 + x);
  return result;
}
// "n (x y raw)*" placements over a DCT8-filled strategy image.
jxl::AcStrategyImage EqhStrategy(JxlMemoryManager* mm, std::istringstream& in, size_t xsize,
                                 size_t ysize) {
  auto image = jxl::AcStrategyImage::Create(mm, xsize, ysize);
  if (!image.ok()) throw std::runtime_error("strategy image allocation");
  jxl::AcStrategyImage acs = std::move(image).value_();
  acs.FillDCT8();
  size_t n; in >> n;
  for (size_t i = 0; i < n; ++i) {
    size_t x, y; unsigned raw; in >> x >> y >> raw;
    if (!acs.Set(x, y, static_cast<jxl::AcStrategyType>(raw))) throw std::runtime_error("Set");
  }
  return acs;
}
void EqhReadMap(std::istringstream& in, jxl::ImageSB* map) {
  for (size_t y = 0; y < map->ysize(); ++y)
    for (size_t x = 0; x < map->xsize(); ++x) { int v; in >> v; map->Row(y)[x] = int8_t(v); }
}
// cparams: distance, speed tier, decoding speed tier.
void EqhParams(std::istringstream& in, jxl::CompressParams* cparams) {
  cparams->butteraugli_distance = EqhFloat(in);
  int tier; size_t decoding; in >> tier >> decoding;
  cparams->speed_tier = static_cast<jxl::SpeedTier>(tier);
  cparams->decoding_speed_tier = decoding;
}
void EqhPrintConfig(const jxl::ACSConfig& config) {
  std::cout << " K " << EqhBits(config.info_loss_multiplier) << ' ' << EqhBits(config.cost_delta)
            << ' ' << EqhBits(config.zeros_mul) << ' ' << config.mask1x1_xsize;
}
// Serial RunOnPool order of the 64x64 encoder tiles of `blocks`.
std::vector<jxl::Rect> EqhTiles(size_t xsize_blocks, size_t ysize_blocks) {
  std::vector<jxl::Rect> tiles;
  const size_t n = jxl::DivCeil(xsize_blocks, jxl::kEncTileDimInBlocks);
  const size_t count = n * jxl::DivCeil(ysize_blocks, jxl::kEncTileDimInBlocks);
  for (size_t tid = 0; tid < count; ++tid) {
    size_t tx = tid % n, ty = tid / n;
    size_t by0 = ty * jxl::kEncTileDimInBlocks;
    size_t by1 = std::min((ty + 1) * jxl::kEncTileDimInBlocks, ysize_blocks);
    size_t bx0 = tx * jxl::kEncTileDimInBlocks;
    size_t bx1 = std::min((tx + 1) * jxl::kEncTileDimInBlocks, xsize_blocks);
    tiles.emplace_back(bx0, by0, bx1 - bx0, by1 - by0);
  }
  return tiles;
}
jxl::DequantMatrices EqhMatrices(JxlMemoryManager* mm) {
  jxl::DequantMatrices matrices;
  uint32_t acs_mask = 0;
  for (size_t i = 0; i < static_cast<size_t>(jxl::AcStrategyType::DCT128X128); i++) {
    acs_mask |= (1 << i);
  }
  if (!matrices.EnsureComputed(mm, acs_mask)) throw std::runtime_error("EnsureComputed");
  return matrices;
}
}  // namespace

bool EncoderHeuristicsCommand(const std::string& command, std::istringstream& in,
                              JxlMemoryManager* mm) {
  if (command.rfind("eqh-", 0) != 0) return false;
  if (command == "eqh-gsq" || command == "eqh-sq") {
    jxl::DequantMatrices dequant;
    jxl::Quantizer quantizer(dequant);
    const float quant_dc = EqhFloat(in);
    const float a = EqhFloat(in);
    if (command == "eqh-gsq") {
      const float absd = EqhFloat(in);
      quantizer.ComputeGlobalScaleAndQuant(quant_dc, a, absd);
      std::cout << "G";
      EqhPrintQuantizer(quantizer);
    } else {
      size_t xsize, ysize; in >> xsize >> ysize;
      jxl::ImageI raw = EqhInts(mm, xsize, ysize);
      quantizer.SetQuant(quant_dc, a, &raw);
      std::cout << "U";
      EqhPrintQuantizer(quantizer);
      EqhPrintInts(raw);
    }
    return true;
  }
  if (command == "eqh-sqf" || command == "eqh-sqfr") {
    jxl::DequantMatrices dequant;
    jxl::Quantizer quantizer(dequant);
    const float quant_dc = EqhFloat(in);
    size_t xsize, ysize; unsigned with_raw;
    jxl::Rect rect;
    float median = 0;
    if (command == "eqh-sqf") {
      in >> with_raw >> xsize >> ysize;
    } else {
      median = EqhFloat(in);
      in >> xsize >> ysize;
      rect = EqhRect(in);
    }
    jxl::ImageF qf = EqhPlane(mm, xsize, ysize);
    EqhReadPlane(in, &qf);
    jxl::ImageI raw = EqhInts(mm, xsize, ysize);
    if (command == "eqh-sqf") {
      const jxl::Status status =
          quantizer.SetQuantField(quant_dc, qf, with_raw != 0 ? &raw : nullptr);
      std::cout << "F " << int(status.code());
    } else {
      quantizer.ComputeGlobalScaleAndQuant(quant_dc, median, 0);
      quantizer.SetQuantFieldRect(qf, rect, &raw);
      std::cout << "R";
    }
    EqhPrintQuantizer(quantizer);
    EqhPrintInts(raw);
    return true;
  }
  if (command == "eqh-some") {
    // GetSomeTestImage(W, H, C, seed): 16-bit big-endian samples.
    size_t xsize, ysize, channels; unsigned seed; in >> xsize >> ysize >> channels >> seed;
    const std::vector<uint8_t> pixels =
        jxl::test::GetSomeTestImage(xsize, ysize, channels, static_cast<uint16_t>(seed));
    std::cout << "V";
    for (size_t i = 0; i + 1 < pixels.size(); i += 2) {
      std::cout << ' ' << ((unsigned(pixels[i]) << 8) | pixels[i + 1]);
    }
    return true;
  }
  if (command == "eqh-iqdc") {
    std::cout << "D " << EqhBits(jxl::InitialQuantDC(EqhFloat(in)));
    return true;
  }
  if (command == "eqh-iqf") {
    const float distance = EqhFloat(in);
    const float rescale = EqhFloat(in);
    size_t xsize, ysize; in >> xsize >> ysize;
    const jxl::Rect rect = EqhRect(in);
    jxl::Image3F opsin = EqhImage(mm, in, xsize, ysize);
    jxl::ImageF mask, mask1x1;
    auto qf = jxl::InitialQuantField(distance, opsin, rect, nullptr, rescale, &mask, &mask1x1);
    std::cout << "Q " << int(qf.status().code());
    if (!qf.ok()) return true;
    const jxl::ImageF field = std::move(qf).value_();
    EqhPrintPlane(field);
    EqhPrintPlane(mask);
    EqhPrintPlane(mask1x1);
    return true;
  }
  if (command == "eqh-adj") {
    const float distance = EqhFloat(in);
    size_t xsize, ysize; in >> xsize >> ysize;
    const jxl::Rect rect = EqhRect(in);
    jxl::AcStrategyImage acs = EqhStrategy(mm, in, xsize, ysize);
    jxl::ImageF qf = EqhPlane(mm, xsize, ysize);
    EqhReadPlane(in, &qf);
    const jxl::Status status = jxl::AdjustQuantField(acs, rect, distance, &qf);
    std::cout << "A " << int(status.code());
    EqhPrintPlane(qf);
    return true;
  }
  if (command == "eqh-mult") {
    size_t num; in >> num;
    const float base = EqhFloat(in);
    const float distance_mul = EqhFloat(in);
    unsigned fast; in >> fast;
    std::vector<float> m(num + 1), s(num + 1);
    for (size_t i = 0; i < num; ++i) m[i] = EqhFloat(in);
    for (size_t i = 0; i < num; ++i) s[i] = EqhFloat(in);
    std::cout << "M "
              << jxl::N_SCALAR::FindBestMultiplier(m.data(), s.data(), num, base, distance_mul,
                                                   fast != 0);
    return true;
  }
  if (command == "eqh-cfl") {
    // mode 0: the DCT8 pass; mode 1: the strategy/quant-field pass.
    unsigned mode, fast; size_t xsize, ysize; in >> mode >> fast >> xsize >> ysize;
    jxl::Image3F opsin = EqhImage(mm, in, xsize, ysize);
    const jxl::Rect rect(0, 0, xsize, ysize);
    const size_t bxs = xsize / 8, bys = ysize / 8;
    jxl::DequantMatrices matrices = EqhMatrices(mm);
    jxl::AcStrategyImage acs;
    jxl::ImageI raw;
    jxl::Quantizer quantizer(matrices);
    if (mode == 1) {
      acs = EqhStrategy(mm, in, bxs, bys);
      raw = EqhInts(mm, bxs, bys);
      for (size_t y = 0; y < bys; ++y)
        for (size_t x = 0; x < bxs; ++x) in >> raw.Row(y)[x];
      const float quant_dc = EqhFloat(in);
      const float median = EqhFloat(in);
      quantizer.ComputeGlobalScaleAndQuant(quant_dc, median, 0);
    }
    auto cmap_or = jxl::ColorCorrelationMap::Create(mm, xsize, ysize);
    if (!cmap_or.ok()) throw std::runtime_error("cmap");
    jxl::ColorCorrelationMap cmap = std::move(cmap_or).value_();
    jxl::CfLHeuristics cfl(mm);
    jxl::Status status = cfl.Init(rect);
    if (status) status = cfl.PrepareForThreads(1);
    for (const jxl::Rect& r : EqhTiles(bxs, bys)) {
      if (!status) break;
      status = cfl.ComputeTile(r, opsin, rect, matrices, mode == 1 ? &acs : nullptr,
                               mode == 1 ? &raw : nullptr, mode == 1 ? &quantizer : nullptr,
                               fast != 0, 0, &cmap);
    }
    std::cout << "L " << int(status.code());
    EqhPrintMap(cmap.ytox_map);
    EqhPrintMap(cmap.ytob_map);
    EqhPrintPlane(cfl.dc_values);
    return true;
  }
  if (command == "eqh-acs" || command == "eqh-entropy" || command == "eqh-best8") {
    jxl::CompressParams cparams;
    EqhParams(in, &cparams);
    size_t xsize, ysize; in >> xsize >> ysize;
    const size_t bxs = xsize / 8, bys = ysize / 8;
    jxl::Image3F opsin = EqhImage(mm, in, xsize, ysize);
    const jxl::Rect rect(0, 0, xsize, ysize);
    jxl::ImageF qf = EqhPlane(mm, bxs, bys), mask = EqhPlane(mm, bxs, bys);
    jxl::ImageF mask1x1 = EqhPlane(mm, xsize, ysize);
    EqhReadPlane(in, &qf);
    EqhReadPlane(in, &mask);
    EqhReadPlane(in, &mask1x1);
    auto cmap_or = jxl::ColorCorrelationMap::Create(mm, xsize, ysize);
    if (!cmap_or.ok()) throw std::runtime_error("cmap");
    jxl::ColorCorrelationMap cmap = std::move(cmap_or).value_();
    EqhReadMap(in, &cmap.ytox_map);
    EqhReadMap(in, &cmap.ytob_map);
    jxl::AcStrategyImage acs = EqhStrategy(mm, in, bxs, bys);
    jxl::DequantMatrices matrices;
    jxl::AcStrategyHeuristics heuristics(mm, cparams);
    jxl::Status status = heuristics.Init(opsin, rect, qf, mask, mask1x1, &matrices);
    if (status) status = heuristics.PrepareForThreads(1);
    std::cout << "H " << int(status.code());
    EqhPrintConfig(heuristics.config);
    if (!status) return true;
    if (command == "eqh-acs") {
      for (const jxl::Rect& r : EqhTiles(bxs, bys)) {
        if (!status) break;
        status = heuristics.ProcessRect(r, cmap, &acs, 0);
      }
      jxl::AuxOut aux;
      jxl::FrameDimensions frame_dim;
      if (status) status = heuristics.Finalize(frame_dim, acs, &aux);
      std::cout << ' ' << int(status.code());
      EqhPrintStrategy(acs);
      std::cout << " N " << aux.num_small_blocks << ' ' << aux.num_dct4x8_blocks << ' '
                << aux.num_afv_blocks << ' ' << aux.num_dct8_blocks << ' '
                << aux.num_dct8x16_blocks << ' ' << aux.num_dct8x32_blocks << ' '
                << aux.num_dct16_blocks << ' ' << aux.num_dct16x32_blocks << ' '
                << aux.num_dct32_blocks << ' ' << aux.num_dct32x64_blocks << ' '
                << aux.num_dct64_blocks;
      return true;
    }
    // A single EstimateEntropy or FindBest8x8Transform call in thread 0's
    // memory, as ProcessRectACS splits it.
    size_t x, y; in >> x >> y;
    float cmap_factors[3];
    for (float& f : cmap_factors) f = EqhFloat(in);
    float* block = heuristics.mem.address<float>();
    float* scratch = block + 3 * jxl::AcStrategy::kMaxCoeffArea;
    uint32_t* quantized = heuristics.qmem.address<uint32_t>();
    const size_t words = heuristics.mem_per_thread;
    for (size_t i = 0; i < words; ++i) {
      uint32_t bits = 0x7fb00000u + uint32_t(i % 4096);
      std::memcpy(block + i, &bits, sizeof(bits));
    }
    if (command == "eqh-entropy") {
      unsigned raw; in >> raw;
      const float entropy_mul = EqhFloat(in);
      const jxl::AcStrategy strategy =
          jxl::AcStrategy::FromRawStrategy(static_cast<jxl::AcStrategyType>(raw));
      float entropy = -1;
      status = jxl::N_SCALAR::EstimateEntropy(strategy, entropy_mul, x, y, heuristics.config,
                                              cmap_factors, block, scratch, quantized, entropy);
      std::cout << " E " << int(status.code()) << ' ' << EqhBits(entropy);
      const size_t size = (1 << strategy.log2_covered_blocks()) * jxl::kDCTBlockSize;
      std::cout << " B";
      for (size_t i = 0; i < 3 * size; ++i) std::cout << ' ' << EqhBits(block[i]);
      std::cout << " W";
      for (size_t i = 0; i < size; ++i) std::cout << ' ' << EqhBits(scratch[i]);
    } else {
      float entropy = -1;
      jxl::AcStrategyType best = jxl::AcStrategyType::DCT128X256;
      status = jxl::N_SCALAR::FindBest8x8Transform(
          x, y, static_cast<int>(cparams.speed_tier), cparams.butteraugli_distance,
          heuristics.config, cmap_factors, &acs, block, scratch, quantized, &entropy, best);
      std::cout << " T " << int(status.code()) << ' ' << EqhBits(entropy) << ' '
                << static_cast<unsigned>(best);
    }
    return true;
  }
  if (command == "eqh-chain") {
    // LossyFrameHeuristics at kSquirrel without splines, patches,
    // dequant-matrix search or FindBestQuantizer (a no-op at kSquirrel).
    const float distance = EqhFloat(in);
    size_t xsize, ysize; in >> xsize >> ysize;
    const size_t pxs = jxl::RoundUpToBlockDim(xsize), pys = jxl::RoundUpToBlockDim(ysize);
    auto color_or = jxl::Image3F::Create(mm, pxs, pys);
    if (!color_or.ok()) throw std::runtime_error("chain image");
    jxl::Image3F opsin = std::move(color_or).value_();
    if (!opsin.ShrinkTo(xsize, ysize)) throw std::runtime_error("ShrinkTo");
    for (size_t c = 0; c < 3; ++c) EqhReadPlane(in, &opsin.Plane(c));
    jxl::Status status = jxl::ToXYB(jxl::ColorEncoding::SRGB(false), 255.0f, nullptr, nullptr,
                                    &opsin, *JxlGetDefaultCms(), nullptr);
    if (status) status = jxl::PadImageToBlockMultipleInPlace(&opsin);
    std::cout << "X " << int(status.code());
    if (!status) return true;
    for (size_t c = 0; c < 3; ++c) EqhPrintPlane(opsin.Plane(c));
    const jxl::Rect rect(opsin);
    const size_t bxs = pxs / 8, bys = pys / 8;
    jxl::CompressParams cparams;
    cparams.butteraugli_distance = distance;
    jxl::DequantMatrices matrices;
    jxl::Quantizer quantizer(matrices);
    auto raw_or = jxl::ImageI::Create(mm, bxs, bys);
    auto cmap_or = jxl::ColorCorrelationMap::Create(mm, pxs, pys);
    auto acs_or = jxl::AcStrategyImage::Create(mm, bxs, bys);
    if (!raw_or.ok() || !cmap_or.ok() || !acs_or.ok()) throw std::runtime_error("chain owners");
    jxl::ImageI raw_quant_field = std::move(raw_or).value_();
    jxl::ColorCorrelationMap cmap = std::move(cmap_or).value_();
    jxl::AcStrategyImage ac_strategy = std::move(acs_or).value_();
    const float quant_dc = jxl::InitialQuantDC(cparams.butteraugli_distance);
    jxl::AcStrategyHeuristics acs_heuristics(mm, cparams);
    jxl::CfLHeuristics cfl_heuristics(mm);
    jxl::ImageF initial_quant_masking, initial_quant_masking1x1;
    auto iqf = jxl::InitialQuantField(cparams.butteraugli_distance, opsin, rect, nullptr, 1.0f,
                                      &initial_quant_masking, &initial_quant_masking1x1);
    std::cout << ' ' << int(iqf.status().code());
    if (!iqf.ok()) return true;
    jxl::ImageF initial_quant_field = std::move(iqf).value_();
    EqhPrintPlane(initial_quant_field);
    EqhPrintPlane(initial_quant_masking);
    EqhPrintPlane(initial_quant_masking1x1);
    float q = 0.39 / cparams.butteraugli_distance;
    quantizer.ComputeGlobalScaleAndQuant(quant_dc, q, 0);
    EqhPrintQuantizer(quantizer);
    float weight[3] = {1.0f, 1.0f, 1.0f};
    status = jxl::GaborishInverse(&opsin, rect, weight, nullptr);
    if (status) status = cfl_heuristics.Init(rect);
    if (status) {
      status = acs_heuristics.Init(opsin, rect, initial_quant_field, initial_quant_masking,
                                   initial_quant_masking1x1, &matrices);
    }
    if (status) status = acs_heuristics.PrepareForThreads(1);
    if (status) status = cfl_heuristics.PrepareForThreads(1);
    for (const jxl::Rect& r : EqhTiles(bxs, bys)) {
      if (!status) break;
      status = cfl_heuristics.ComputeTile(r, opsin, rect, matrices, nullptr, nullptr, nullptr,
                                          false, 0, &cmap);
      if (status) status = acs_heuristics.ProcessRect(r, cmap, &ac_strategy, 0);
      if (status) {
        status = jxl::AdjustQuantField(ac_strategy, r, cparams.butteraugli_distance,
                                       &initial_quant_field);
      }
      if (!status) break;
      quantizer.SetQuantFieldRect(initial_quant_field, r, &raw_quant_field);
      status = cfl_heuristics.ComputeTile(r, opsin, rect, matrices, &ac_strategy,
                                          &raw_quant_field, &quantizer, false, 0, &cmap);
    }
    std::cout << " T " << int(status.code());
    EqhPrintStrategy(ac_strategy);
    EqhPrintPlane(initial_quant_field);
    EqhPrintInts(raw_quant_field);
    EqhPrintMap(cmap.ytox_map);
    EqhPrintMap(cmap.ytob_map);
    EqhPrintPlane(cfl_heuristics.dc_values);
    return true;
  }
  throw std::runtime_error("unknown encoder heuristics command");
}
