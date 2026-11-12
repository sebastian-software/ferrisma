// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The `ecf-chain` request runs the original encoder from sRGB samples to the
// coefficient, DC and AC-metadata streams of a VarDCT frame at kSquirrel, for
// one serial thread: ToXYB and padding, the kSquirrel part of
// LossyFrameHeuristics (as `eqh-chain`, plus AcStrategyHeuristics::Finalize,
// the epf_sharpness fill and FindBestBlockEntropyModel), then the archive's
// InitializePassesEncoder (ComputeCoefficients, AddVarDCTDC,
// AdaptiveDCSmoothing) and ComputeACMetadata, ComputeUsedOrders and
// ComputeCoeffOrder as ComputeAllCoeffOrders calls them, TokenizeCoefficients
// per group as TokenizeAllCoefficients does, and the serialization units
// ColorCorrelationEncodeDC, EncodeBlockCtxMap, EncodeCoeffOrders,
// BuildAndEncodeHistograms for the AC tokens with
// EncodeGroupTokenizedCoefficients, and ModularFrameEncoder::ComputeTree,
// ComputeTokens, EncodeGlobalInfo and EncodeStream for the DC and AC-metadata
// streams. ComputeARHeuristics is not run: epf_sharpness keeps the value 4
// that LossyFrameHeuristics stores. The enc_frame.cc orchestration functions
// have internal linkage; this adapter repeats their call sequence and supplies
// no algorithm. Compiled with -fno-access-control to read the modular
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

#include "lib/jxl/ac_context.h"
#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/dec_modular.h"
#include "lib/jxl/enc_ac_strategy.h"
#include "lib/jxl/enc_adaptive_quantization.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_cache.h"
#include "lib/jxl/enc_chroma_from_luma.h"
#include "lib/jxl/enc_coeff_order.h"
#include "lib/jxl/enc_context_map.h"
#include "lib/jxl/enc_entropy_coder.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_gaborish.h"
#include "lib/jxl/enc_group.h"
#include "lib/jxl/enc_modular.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_quant_weights.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/fields.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/memory_manager_internal.h"
#include "lib/jxl/quant_weights.h"
#include "lib/jxl/quantizer.h"

namespace jxl {
// External-linkage definition in enc_heuristics.cc (the header declares a
// different overload).
void FindBestBlockEntropyModel(const CompressParams& cparams, const ImageI& rqf,
                               const AcStrategyImage& ac_strategy,
                               BlockCtxMap* block_ctx_map);
}  // namespace jxl

namespace {
uint32_t EcfBits(float v) { uint32_t b; std::memcpy(&b, &v, sizeof(b)); return b; }
float EcfFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
void EcfPrintPlane(const jxl::ImageF& plane) {
  std::cout << " P " << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << EcfBits(plane.ConstRow(y)[x]);
}
template <typename T>
void EcfPrintInts(const char* tag, const jxl::Plane<T>& plane) {
  std::cout << ' ' << tag << ' ' << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x) std::cout << ' ' << int64_t(plane.ConstRow(y)[x]);
}
void EcfPrintStrategy(const jxl::AcStrategyImage& acs) {
  std::cout << " S";
  for (size_t y = 0; y < acs.ysize(); ++y) {
    const jxl::AcStrategyRow row = acs.ConstRow(y);
    for (size_t x = 0; x < acs.xsize(); ++x) {
      std::cout << ' ' << int(row[x].RawStrategy()) << ':' << row[x].IsFirstBlock();
    }
  }
}
void EcfPrintQuantizer(const jxl::Quantizer& q) {
  const jxl::QuantizerParams params = q.GetParams();
  std::cout << " Q " << params.global_scale << ' ' << params.quant_dc << ' ' << EcfBits(q.Scale())
            << ' ' << EcfBits(q.InvGlobalScale()) << ' ' << EcfBits(q.inv_quant_dc());
  for (size_t c = 0; c < 3; ++c) std::cout << ' ' << EcfBits(q.MulDC()[c]);
  for (size_t c = 0; c < 3; ++c) std::cout << ' ' << EcfBits(q.InvMulDC()[c]);
}
void EcfPrintBytes(const char* tag, jxl::BitWriter& writer) {
  std::cout << ' ' << tag << ' ' << writer.BitsWritten();
  writer.ZeroPadToByte();
  const jxl::Span<const uint8_t> span = writer.GetSpan();
  for (size_t i = 0; i < span.size(); ++i) std::cout << ' ' << int(span[i]);
}
void EcfPrintTokens(const std::vector<jxl::Token>& tokens) {
  std::cout << " K " << tokens.size();
  for (const jxl::Token& t : tokens) std::cout << ' ' << t.context << ':' << t.value;
}
void EcfPrintModularImage(const jxl::Image& image) {
  std::cout << " M " << image.channel.size();
  for (const jxl::Channel& ch : image.channel) {
    std::cout << ' ' << ch.w << ' ' << ch.h << ' ' << ch.hshift << ' ' << ch.vshift;
    for (size_t y = 0; y < ch.h; ++y)
      for (size_t x = 0; x < ch.w; ++x) std::cout << ' ' << ch.plane.ConstRow(y)[x];
  }
}
std::vector<jxl::Rect> EcfTiles(size_t xsize_blocks, size_t ysize_blocks) {
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
T EcfValue(jxl::StatusOr<T>&& value) {
  if (!value.ok()) throw std::runtime_error("ecf allocation");
  return std::move(value).value_();
}
}  // namespace

bool EncoderCoefficientsCommand(const std::string& command, std::istringstream& in,
                                JxlMemoryManager* mm) {
  if (command != "ecf-chain") return false;
  const float distance = EcfFloat(in);
  size_t xsize, ysize, gray, x_qm_scale, b_qm_scale;
  in >> xsize >> ysize >> gray >> x_qm_scale >> b_qm_scale;
  const size_t pxs = jxl::RoundUpToBlockDim(xsize), pys = jxl::RoundUpToBlockDim(ysize);
  jxl::Image3F opsin = EcfValue(jxl::Image3F::Create(mm, pxs, pys));
  if (!opsin.ShrinkTo(xsize, ysize)) throw std::runtime_error("ShrinkTo");
  for (size_t c = 0; c < 3; ++c) {
    for (size_t y = 0; y < ysize; ++y)
      for (size_t x = 0; x < xsize; ++x) opsin.PlaneRow(c, y)[x] = EcfFloat(in);
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
  jxl::PassesEncoderState enc_state(mm);
  enc_state.cparams = cparams;
  jxl::PassesSharedState& shared = enc_state.shared;
  shared.metadata = &metadata;
  shared.frame_dim = frame_header.ToFrameDimensions();
  const jxl::FrameDimensions& frame_dim = shared.frame_dim;
  auto modular_or = jxl::ModularFrameEncoder::Create(mm, frame_header, cparams, false);
  if (!modular_or.ok()) throw std::runtime_error("modular");
  std::unique_ptr<jxl::ModularFrameEncoder> modular = std::move(modular_or).value_();
  shared.ac_strategy = EcfValue(
      jxl::AcStrategyImage::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.raw_quant_field =
      EcfValue(jxl::ImageI::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.epf_sharpness =
      EcfValue(jxl::ImageB::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.cmap = EcfValue(jxl::ColorCorrelationMap::Create(mm, frame_dim.xsize, frame_dim.ysize));
  shared.coeff_order_size = jxl::kCoeffOrderMaxSize;
  shared.coeff_orders.resize(frame_header.passes.num_passes * jxl::kCoeffOrderMaxSize);
  shared.quant_dc =
      EcfValue(jxl::ImageB::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.dc_storage =
      EcfValue(jxl::Image3F::Create(mm, frame_dim.xsize_blocks, frame_dim.ysize_blocks));
  shared.dc = &shared.dc_storage;
  enc_state.passes.resize(enc_state.progressive_splitter.GetNumPasses());
  for (auto& pass : enc_state.passes) pass.ac_tokens.resize(frame_dim.num_groups);

  // LossyFrameHeuristics at kSquirrel without splines and patches.
  const jxl::Rect rect(opsin);
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
  for (const jxl::Rect& r : EcfTiles(frame_dim.xsize_blocks, frame_dim.ysize_blocks)) {
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
  std::cout << " B " << shared.block_ctx_map.num_ctxs << ' '
            << shared.block_ctx_map.qf_thresholds.size();
  for (uint32_t t : shared.block_ctx_map.qf_thresholds) std::cout << ' ' << t;
  std::cout << ' ' << shared.block_ctx_map.ctx_map.size();
  for (uint8_t v : shared.block_ctx_map.ctx_map) std::cout << ' ' << int(v);

  // ComputeVarDCTEncodingData after the heuristics, without AR.
  status = jxl::InitializePassesEncoder(frame_header, opsin, rect, *JxlGetDefaultCms(), nullptr,
                                        &enc_state, modular.get(), nullptr);
  std::cout << " I " << int(status.code());
  if (!status) return true;
  status = jxl::ComputeACMetadata(nullptr, &enc_state, modular.get());
  std::cout << ' ' << int(status.code());
  if (!status) return true;
  EcfPrintQuantizer(shared.quantizer);
  std::cout << " D";
  for (size_t c = 0; c < 3; ++c) std::cout << ' ' << EcfBits(shared.matrices.DCQuants()[c]);
  EcfPrintStrategy(shared.ac_strategy);
  EcfPrintInts("R", shared.raw_quant_field);
  EcfPrintInts("C", shared.cmap.ytox_map);
  EcfPrintInts("C", shared.cmap.ytob_map);
  std::cout << ' ' << EcfBits(enc_state.x_qm_multiplier) << ' '
            << EcfBits(enc_state.b_qm_multiplier);
  for (size_t c = 0; c < 3; ++c) EcfPrintPlane(shared.dc_storage.Plane(c));
  EcfPrintInts("U", shared.quant_dc);
  for (size_t g = 0; g < frame_dim.num_groups; ++g) {
    const jxl::Rect br = frame_dim.BlockGroupRect(g);
    const size_t n = br.xsize() * br.ysize() * jxl::kDCTBlockSize;
    std::cout << " G " << g;
    for (size_t c = 0; c < 3; ++c) {
      const int32_t* row = enc_state.coeffs[0]->PlaneRow(c, g, 0).ptr32;
      for (size_t i = 0; i < n; ++i) std::cout << ' ' << row[i];
    }
  }
  for (size_t g = 0; g < frame_dim.num_dc_groups; ++g) {
    std::cout << " E " << int(modular->extra_dc_precision[g]) << ' '
              << modular->ac_metadata_size[g];
    EcfPrintModularImage(
        modular->stream_images_[jxl::ModularStreamId::VarDCTDC(g).ID(frame_dim)]);
    EcfPrintModularImage(
        modular->stream_images_[jxl::ModularStreamId::ACMetadata(g).ID(frame_dim)]);
  }

  // ComputeAllCoeffOrders.
  auto used_orders_info = jxl::ComputeUsedOrders(cparams.speed_tier, shared.ac_strategy,
                                                 jxl::Rect(shared.raw_quant_field));
  enc_state.used_orders.resize(enc_state.progressive_splitter.GetNumPasses());
  status = jxl::ComputeCoeffOrder(cparams.speed_tier, *enc_state.coeffs[0], shared.ac_strategy,
                                  frame_dim, enc_state.used_orders[0], enc_state.used_acs,
                                  used_orders_info.first, used_orders_info.second,
                                  &shared.coeff_orders[0]);
  enc_state.used_acs |= used_orders_info.first;
  std::cout << " O " << int(status.code()) << ' ' << used_orders_info.first << ' '
            << used_orders_info.second << ' ' << enc_state.used_orders[0] << ' '
            << enc_state.used_acs;
  if (!status) return true;
  for (size_t ord = 0; ord < jxl::kNumOrders; ++ord) {
    if (!(enc_state.used_acs & (1u << ord))) continue;
    using jxl::kCoeffOrderOffset;
    using jxl::kDCTBlockSize;
    for (size_t i = CoeffOrderOffset(ord, 0); i < CoeffOrderOffset(ord + 1, 0); ++i) {
      std::cout << ' ' << shared.coeff_orders[i];
    }
  }
  shared.num_histograms = 1;
  enc_state.histogram_idx.resize(frame_dim.num_groups);

  // TokenizeAllCoefficients.
  auto num_nzeroes =
      EcfValue(jxl::Image3I::Create(mm, jxl::kGroupDimInBlocks, jxl::kGroupDimInBlocks));
  for (size_t g = 0; g < frame_dim.num_groups; ++g) {
    const int32_t* JXL_RESTRICT ac_rows[3] = {
        enc_state.coeffs[0]->PlaneRow(0, g, 0).ptr32,
        enc_state.coeffs[0]->PlaneRow(1, g, 0).ptr32,
        enc_state.coeffs[0]->PlaneRow(2, g, 0).ptr32,
    };
    status = jxl::TokenizeCoefficients(&shared.coeff_orders[0], frame_dim.BlockGroupRect(g),
                                       ac_rows, shared.ac_strategy,
                                       frame_header.chroma_subsampling, &num_nzeroes,
                                       &enc_state.passes[0].ac_tokens[g], shared.quant_dc,
                                       shared.raw_quant_field, shared.block_ctx_map);
    std::cout << " T " << int(status.code());
    if (!status) return true;
    EcfPrintTokens(enc_state.passes[0].ac_tokens[g]);
  }

  // Modular tree and tokens for the DC and AC-metadata streams.
  status = modular->ComputeTree(nullptr);
  if (status) status = modular->ComputeTokens(nullptr);
  std::cout << " Z " << int(status.code());
  if (!status) return true;

  // Serialization units, each into its own writer.
  {
    jxl::BitWriter writer(mm);
    status = jxl::WriteQuantizerParams(shared.quantizer.GetParams(), &writer,
                                       jxl::LayerType::Quant, nullptr);
    if (status) status = jxl::EncodeBlockCtxMap(shared.block_ctx_map, &writer, nullptr);
    if (status) {
      status = jxl::ColorCorrelationEncodeDC(shared.cmap.base(), &writer, jxl::LayerType::Dc,
                                             nullptr);
    }
    std::cout << " W " << int(status.code());
    if (!status) return true;
    EcfPrintBytes("dcinfo", writer);
  }
  {
    jxl::BitWriter writer(mm);
    status = modular->EncodeGlobalInfo(false, &writer, nullptr);
    if (status) {
      status = modular->EncodeStream(&writer, nullptr, jxl::LayerType::ModularGlobal,
                                     jxl::ModularStreamId::Global());
    }
    std::cout << " W " << int(status.code());
    if (!status) return true;
    EcfPrintBytes("global", writer);
  }
  for (size_t g = 0; g < frame_dim.num_dc_groups; ++g) {
    jxl::BitWriter writer(mm);
    status = writer.WithMaxBits(2, jxl::LayerType::Dc, nullptr, [&] {
      writer.Write(2, modular->extra_dc_precision[g]);
      return true;
    });
    if (status) {
      status = modular->EncodeStream(&writer, nullptr, jxl::LayerType::Dc,
                                     jxl::ModularStreamId::VarDCTDC(g));
    }
    if (status) {
      status = modular->EncodeStream(&writer, nullptr, jxl::LayerType::ModularDcGroup,
                                     jxl::ModularStreamId::ModularDC(g));
    }
    const jxl::Rect& r = frame_dim.DCGroupRect(g);
    size_t nb_bits = jxl::CeilLog2Nonzero(r.xsize() * r.ysize());
    if (status && nb_bits != 0) {
      status = writer.WithMaxBits(nb_bits, jxl::LayerType::ControlFields, nullptr, [&] {
        writer.Write(nb_bits, modular->ac_metadata_size[g] - 1);
        return true;
      });
    }
    if (status) {
      status = modular->EncodeStream(&writer, nullptr, jxl::LayerType::ControlFields,
                                     jxl::ModularStreamId::ACMetadata(g));
    }
    std::cout << " W " << int(status.code());
    if (!status) return true;
    EcfPrintBytes("dcgroup", writer);
  }
  {
    // EncodeGlobalACInfo for one pass, not streaming.
    jxl::BitWriter writer(mm);
    status = jxl::DequantMatricesEncode(mm, shared.matrices, &writer, jxl::LayerType::Quant,
                                        nullptr, modular.get());
    size_t num_histo_bits = jxl::CeilLog2Nonzero(frame_dim.num_groups);
    if (status && num_histo_bits != 0) {
      status = writer.WithMaxBits(num_histo_bits, jxl::LayerType::Ac, nullptr, [&] {
        writer.Write(num_histo_bits, shared.num_histograms - 1);
        return true;
      });
    }
    size_t order_bits = 0;
    if (status) status = jxl::U32Coder::CanEncode(jxl::kOrderEnc, enc_state.used_orders[0],
                                                  &order_bits);
    if (status) {
      status = writer.WithMaxBits(order_bits, jxl::LayerType::Order, nullptr, [&] {
        return jxl::U32Coder::Write(jxl::kOrderEnc, enc_state.used_orders[0], &writer);
      });
    }
    if (status) {
      status = jxl::EncodeCoeffOrders(enc_state.used_orders[0], &shared.coeff_orders[0], &writer,
                                      jxl::LayerType::Order, nullptr);
    }
    jxl::HistogramParams hist_params(cparams.speed_tier, shared.block_ctx_map.NumACContexts());
    if (cparams.speed_tier > jxl::SpeedTier::kTortoise) {
      hist_params.lz77_method = jxl::HistogramParams::LZ77Method::kNone;
    }
    if (cparams.decoding_speed_tier >= 1) hist_params.max_histograms = 6;
    hist_params.streaming_mode = false;
    hist_params.initialize_global_state = true;
    if (status) {
      auto cost = jxl::BuildAndEncodeHistograms(
          mm, hist_params, shared.num_histograms * shared.block_ctx_map.NumACContexts(),
          enc_state.passes[0].ac_tokens, &enc_state.passes[0].codes, &writer,
          jxl::LayerType::Ac, nullptr);
      status = cost.status();
    }
    std::cout << " W " << int(status.code());
    if (!status) return true;
    EcfPrintBytes("acglobal", writer);
  }
  for (size_t g = 0; g < frame_dim.num_groups; ++g) {
    jxl::BitWriter writer(mm);
    status = jxl::EncodeGroupTokenizedCoefficients(g, 0, enc_state.histogram_idx[g], enc_state,
                                                   &writer, nullptr);
    if (status) {
      status = modular->EncodeStream(&writer, nullptr, jxl::LayerType::ModularAcGroup,
                                     jxl::ModularStreamId::ModularAC(g, 0));
    }
    std::cout << " W " << int(status.code());
    if (!status) return true;
    EcfPrintBytes("acgroup", writer);
  }
  return true;
}
