// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The `ear-chain` request runs the original encoder's adaptive-reconstruction
// (AR) heuristics with the decoder in the loop. From sRGB samples it repeats
// the `ecf-chain` sequence (encoder_coefficients.cc): ToXYB and padding, the
// pre-Gaborish copy that ComputeVarDCTEncodingData keeps for the AR heuristics,
// the kSquirrel part of LossyFrameHeuristics, and the archive's
// InitializePassesEncoder. The loop filter comes from the internal
// LoopFilterFromParams (through enc_frame.inc) or, for unit isolation, from
// explicit Gaborish and EPF-iteration values. It then reports, for each EPF
// sharpness candidate that ComputeARHeuristics tries at this distance, the
// archive's ReconstructImage output (with the whole epf_sharpness plane set to
// the candidate, as ComputeARHeuristics does) and the per-block
// ComputeBlockL2Distance image, then runs the archive's ComputeARHeuristics
// itself and reports the chosen epf_sharpness plane, and finally runs
// ComputeACMetadata and reports the AC-metadata modular images that carry it.
// ReconstructImage and ComputeBlockL2Distance have external linkage without a
// header declaration. Compiled with -fno-access-control to read the modular
// encoder's stream images.
#include <jxl/cms.h>
#include <jxl/memory_manager.h>

#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/dec_modular.h"
#include "lib/jxl/enc_ac_strategy.h"
#include "lib/jxl/enc_adaptive_quantization.h"
#include "lib/jxl/enc_cache.h"
#include "lib/jxl/enc_chroma_from_luma.h"
#include "lib/jxl/enc_gaborish.h"
#include "lib/jxl/enc_heuristics.h"
#include "lib/jxl/enc_modular.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/quantizer.h"

namespace jxl {
// External-linkage definitions in enc_heuristics.cc.
void FindBestBlockEntropyModel(const CompressParams& cparams, const ImageI& rqf,
                               const AcStrategyImage& ac_strategy,
                               BlockCtxMap* block_ctx_map);
StatusOr<Image3F> ReconstructImage(const FrameHeader& orig_frame_header,
                                   const PassesSharedState& shared,
                                   const std::vector<std::unique_ptr<ACImage>>& coeffs,
                                   ThreadPool* pool);
float ComputeBlockL2Distance(const Image3F& a, const Image3F& b, const ImageF& mask1x1,
                             size_t by, size_t bx);
}  // namespace jxl

// enc_frame.inc (compiled in main.cc) exposes the internal function.
jxl::Status EFLoopFilterFromParams(const jxl::CompressParams& p, bool streaming,
                                   jxl::FrameHeader* header);

namespace {
uint32_t EarBits(float v) { uint32_t b; std::memcpy(&b, &v, sizeof(b)); return b; }
float EarFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
void EarPrintPlane(const jxl::ImageF& plane) {
  std::cout << " P " << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << EarBits(plane.ConstRow(y)[x]);
}
void EarPrintModularImage(const jxl::Image& image) {
  std::cout << " M " << image.channel.size();
  for (const jxl::Channel& ch : image.channel) {
    std::cout << ' ' << ch.w << ' ' << ch.h << ' ' << ch.hshift << ' ' << ch.vshift;
    for (size_t y = 0; y < ch.h; ++y)
      for (size_t x = 0; x < ch.w; ++x) std::cout << ' ' << ch.plane.ConstRow(y)[x];
  }
}
std::vector<jxl::Rect> EarTiles(size_t xsize_blocks, size_t ysize_blocks) {
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
template <typename T>
T EarValue(jxl::StatusOr<T>&& value) {
  if (!value.ok()) throw std::runtime_error("ear allocation");
  return std::move(value).value_();
}
}  // namespace

bool EncoderArCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command != "ear-chain") return false;
  const float distance = EarFloat(in);
  size_t xsize, ysize, gray, x_qm_scale, b_qm_scale, lf_explicit, gab, epf_iters;
  in >> xsize >> ysize >> gray >> x_qm_scale >> b_qm_scale >> lf_explicit >> gab >> epf_iters;
  const size_t pxs = jxl::RoundUpToBlockDim(xsize), pys = jxl::RoundUpToBlockDim(ysize);
  jxl::Image3F opsin = EarValue(jxl::Image3F::Create(mm, pxs, pys));
  if (!opsin.ShrinkTo(xsize, ysize)) throw std::runtime_error("ShrinkTo");
  for (size_t c = 0; c < 3; ++c) {
    for (size_t y = 0; y < ysize; ++y)
      for (size_t x = 0; x < xsize; ++x) opsin.PlaneRow(c, y)[x] = EarFloat(in);
  }
  jxl::Status status = jxl::ToXYB(jxl::ColorEncoding::SRGB(gray != 0), 255.0f, nullptr, nullptr,
                                  &opsin, *JxlGetDefaultCms(), nullptr);
  if (status) status = jxl::PadImageToBlockMultipleInPlace(&opsin);
  std::cout << "X " << int(status.code());
  if (!status) return true;

  jxl::CodecMetadata metadata;
  if (!metadata.size.Set(xsize, ysize)) throw std::runtime_error("size");
  metadata.m.color_encoding = jxl::ColorEncoding::SRGB(gray != 0);
  jxl::FrameHeader frame_header(&metadata);
  frame_header.encoding = jxl::FrameEncoding::kVarDCT;
  frame_header.color_transform = jxl::ColorTransform::kXYB;
  frame_header.x_qm_scale = x_qm_scale;
  frame_header.b_qm_scale = b_qm_scale;

  jxl::CompressParams cparams;
  cparams.butteraugli_distance = distance;
  if (lf_explicit != 0) {
    frame_header.loop_filter.gab = gab != 0;
    frame_header.loop_filter.epf_iters = epf_iters;
  } else {
    status = EFLoopFilterFromParams(cparams, false, &frame_header);
  }
  std::cout << " L " << int(status.code()) << ' ' << int(frame_header.loop_filter.gab) << ' '
            << frame_header.loop_filter.epf_iters;
  if (!status) return true;

  jxl::PassesEncoderState enc_state(mm);
  enc_state.cparams = cparams;
  jxl::PassesSharedState& shared = enc_state.shared;
  shared.metadata = &metadata;
  shared.frame_dim = frame_header.ToFrameDimensions();
  const jxl::FrameDimensions& frame_dim = shared.frame_dim;
  auto modular_or = jxl::ModularFrameEncoder::Create(mm, frame_header, cparams, false);
  if (!modular_or.ok()) throw std::runtime_error("modular");
  std::unique_ptr<jxl::ModularFrameEncoder> modular = std::move(modular_or).value_();
  shared.ac_strategy = EarValue(
      jxl::AcStrategyImage::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.raw_quant_field =
      EarValue(jxl::ImageI::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.epf_sharpness =
      EarValue(jxl::ImageB::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.cmap = EarValue(jxl::ColorCorrelationMap::Create(mm, frame_dim.xsize, frame_dim.ysize));
  shared.coeff_order_size = jxl::kCoeffOrderMaxSize;
  shared.coeff_orders.resize(frame_header.passes.num_passes * jxl::kCoeffOrderMaxSize);
  shared.quant_dc =
      EarValue(jxl::ImageB::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.dc_storage =
      EarValue(jxl::Image3F::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.dc = &shared.dc_storage;
  enc_state.passes.resize(enc_state.progressive_splitter.GetNumPasses());
  for (auto& pass : enc_state.passes) pass.ac_tokens.resize(frame_dim.num_groups);

  // ComputeVarDCTEncodingData: the pre-Gaborish copy for the AR heuristics.
  const jxl::Rect rect(opsin);
  jxl::Image3F orig_opsin = EarValue(jxl::Image3F::Create(mm, rect.xsize(), rect.ysize()));
  status = jxl::CopyImageTo(rect, opsin, jxl::Rect(orig_opsin), &orig_opsin);
  if (status) status = orig_opsin.ShrinkTo(frame_dim.xsize, frame_dim.ysize);
  if (!status) throw std::runtime_error("orig_opsin");

  // LossyFrameHeuristics at kSquirrel without splines and patches.
  const float quant_dc = jxl::InitialQuantDC(cparams.butteraugli_distance);
  jxl::AcStrategyHeuristics acs_heuristics(mm, cparams);
  jxl::CfLHeuristics cfl_heuristics(mm);
  jxl::ImageF initial_quant_masking;
  auto iqf = jxl::InitialQuantField(cparams.butteraugli_distance, opsin, rect, nullptr, 1.0f,
                                    &initial_quant_masking, &enc_state.initial_quant_masking1x1);
  std::cout << ' ' << int(iqf.status().code());
  if (!iqf.ok()) return true;
  jxl::ImageF initial_quant_field = std::move(iqf).value_();
  float q = 0.39 / cparams.butteraugli_distance;
  shared.quantizer.ComputeGlobalScaleAndQuant(quant_dc, q, 0);
  float weight[3] = {1.0f, 1.0f, 1.0f};
  status = jxl::GaborishInverse(&opsin, rect, weight, nullptr);
  if (status) status = cfl_heuristics.Init(rect);
  if (status) {
    status = acs_heuristics.Init(opsin, rect, initial_quant_field, initial_quant_masking,
                                 enc_state.initial_quant_masking1x1, &shared.matrices);
  }
  if (status) status = acs_heuristics.PrepareForThreads(1);
  if (status) status = cfl_heuristics.PrepareForThreads(1);
  for (const jxl::Rect& r : EarTiles(frame_dim.xsize_blocks, frame_dim.ysize_blocks)) {
    if (!status) break;
    status = cfl_heuristics.ComputeTile(r, opsin, rect, shared.matrices, nullptr, nullptr,
                                        nullptr, false, 0, &shared.cmap);
    if (status) status = acs_heuristics.ProcessRect(r, shared.cmap, &shared.ac_strategy, 0);
    if (status) {
      status = jxl::AdjustQuantField(shared.ac_strategy, r, cparams.butteraugli_distance,
                                     &initial_quant_field);
    }
    if (!status) break;
    shared.quantizer.SetQuantFieldRect(initial_quant_field, r, &shared.raw_quant_field);
    status = cfl_heuristics.ComputeTile(r, opsin, rect, shared.matrices, &shared.ac_strategy,
                                        &shared.raw_quant_field, &shared.quantizer, false, 0,
                                        &shared.cmap);
  }
  if (status) status = acs_heuristics.Finalize(frame_dim, shared.ac_strategy, nullptr);
  std::cout << " H " << int(status.code());
  if (!status) return true;
  jxl::FillPlane(static_cast<uint8_t>(4), &shared.epf_sharpness, jxl::Rect(shared.epf_sharpness));
  jxl::FindBestBlockEntropyModel(cparams, shared.raw_quant_field, shared.ac_strategy,
                                 &shared.block_ctx_map);
  status = jxl::InitializePassesEncoder(frame_header, opsin, rect, *JxlGetDefaultCms(), nullptr,
                                        &enc_state, modular.get(), nullptr);
  std::cout << " I " << int(status.code());
  if (!status) return true;

  // The candidates ComputeARHeuristics reconstructs at this distance.
  std::vector<uint8_t> epf_steps;
  if (cparams.butteraugli_distance > 4.5f) {
    epf_steps = {0, 4};
  } else {
    epf_steps = {0, 2, 7};
  }
  for (uint8_t val : epf_steps) {
    jxl::FillPlane(val, &shared.epf_sharpness, jxl::Rect(shared.epf_sharpness));
    auto decoded_or = jxl::ReconstructImage(frame_header, shared, enc_state.coeffs, nullptr);
    std::cout << " V " << int(val) << ' ' << int(decoded_or.status().code());
    if (!decoded_or.ok()) return true;
    jxl::Image3F decoded = std::move(decoded_or).value_();
    for (size_t c = 0; c < 3; ++c) EarPrintPlane(decoded.Plane(c));
    jxl::ImageF error =
        EarValue(jxl::ImageF::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
    for (size_t by = 0; by < frame_dim.ysize_blocks; ++by) {
      for (size_t bx = 0; bx < frame_dim.xsize_blocks; ++bx) {
        error.Row(by)[bx] = jxl::ComputeBlockL2Distance(orig_opsin, decoded,
                                                        enc_state.initial_quant_masking1x1, by, bx);
      }
    }
    std::cout << " e";
    EarPrintPlane(error);
  }

  // The archive's ComputeARHeuristics, from the value LossyFrameHeuristics
  // stores.
  jxl::FillPlane(static_cast<uint8_t>(4), &shared.epf_sharpness, jxl::Rect(shared.epf_sharpness));
  status = jxl::ComputeARHeuristics(frame_header, &enc_state, orig_opsin, rect, nullptr);
  std::cout << " A " << int(status.code());
  if (!status) return true;
  std::cout << " F " << shared.epf_sharpness.xsize() << ' ' << shared.epf_sharpness.ysize();
  for (size_t y = 0; y < shared.epf_sharpness.ysize(); ++y)
    for (size_t x = 0; x < shared.epf_sharpness.xsize(); ++x)
      std::cout << ' ' << int(shared.epf_sharpness.ConstRow(y)[x]);

  status = jxl::ComputeACMetadata(nullptr, &enc_state, modular.get());
  std::cout << " C " << int(status.code());
  if (!status) return true;
  for (size_t g = 0; g < frame_dim.num_dc_groups; ++g) {
    std::cout << " E " << modular->ac_metadata_size[g];
    EarPrintModularImage(
        modular->stream_images_[jxl::ModularStreamId::ACMetadata(g).ID(frame_dim)]);
  }
  return true;
}
