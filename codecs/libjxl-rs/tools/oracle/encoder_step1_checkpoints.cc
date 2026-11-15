// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder step 1 (#118): checkpoint fixtures of the frame encoder's
// heuristics. This TU compiles the unmodified lib/jxl/enc_frame.cc a second
// time. Before it is included, every header it includes is included, so the
// macros below rename only tokens of enc_frame.cc itself: its external
// definitions (ParamsPostInit, both EncodeFrame overloads and the three
// kTectonicPlate setting lists) get Es1 names, so they do not collide with
// the copy that main.cc compiles through enc_frame.inc, and its calls of
// LossyFrameHeuristics, InitializePassesEncoder and ComputeARHeuristics from
// ComputeVarDCTEncodingData go to the Es1 wrappers below. Each wrapper calls
// the archive's original function and then records the encoder state that
// the call produced. ComputeACMetadata and every other callee stay the
// archive's. No algorithm is replaced.
//
// es1-ckpt XSIZE YSIZE GRAY LINEAR CONTENT DISTANCE INTENSITY: one frame of
//   the public encoder histories of tests/parity/encoder_public.rs (float32
//   samples q/1024 of generated CONTENT, enum sRGB or linear sRGB, gray or
//   RGB) through Es1EncodeFrame with the CompressParams, CodecMetadata and
//   FrameInfo that JxlEncoderStruct::ProcessOneEnqueuedInput passes for such
//   a history. Prints the checkpoints in call order and then the codestream
//   headers (WriteCodestreamHeaders) followed by the frame bytes, which the
//   test requires to equal the original public encoder's output.
#include <jxl/cms.h>
#include <jxl/cms_interface.h>
#include <jxl/encode.h>
#include <jxl/memory_manager.h>
#include <jxl/types.h>

#include <algorithm>
#include <array>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

// Every include of the pinned enc_frame.cc, in its order.
#include "lib/jxl/enc_frame.h"
#include "lib/jxl/ac_context.h"
#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/bits.h"
#include "lib/jxl/base/byte_order.h"
#include "lib/jxl/base/common.h"
#include "lib/jxl/base/compiler_specific.h"
#include "lib/jxl/base/data_parallel.h"
#include "lib/jxl/base/override.h"
#include "lib/jxl/base/printf_macros.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/span.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/coeff_order_fwd.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/common.h"
#include "lib/jxl/dct_util.h"
#include "lib/jxl/dec_external_image.h"
#include "lib/jxl/dec_modular.h"
#include "lib/jxl/enc_ac_strategy.h"
#include "lib/jxl/enc_adaptive_quantization.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_ans_params.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_cache.h"
#include "lib/jxl/enc_chroma_from_luma.h"
#include "lib/jxl/enc_coeff_order.h"
#include "lib/jxl/enc_context_map.h"
#include "lib/jxl/enc_entropy_coder.h"
#include "lib/jxl/enc_external_image.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_group.h"
#include "lib/jxl/enc_heuristics.h"
#include "lib/jxl/enc_modular.h"
#include "lib/jxl/enc_noise.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_patch_dictionary.h"
#include "lib/jxl/enc_photon_noise.h"
#include "lib/jxl/enc_progressive_split.h"
#include "lib/jxl/enc_quant_weights.h"
#include "lib/jxl/enc_splines.h"
#include "lib/jxl/enc_toc.h"
#include "lib/jxl/enc_xyb.h"
#include "lib/jxl/encode_internal.h"
#include "lib/jxl/fields.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_bundle.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/jpeg/enc_jpeg_data.h"
#include "lib/jxl/jpeg/jpeg_data.h"
#include "lib/jxl/loop_filter.h"
#include "lib/jxl/modular/options.h"
#include "lib/jxl/noise.h"
#include "lib/jxl/padded_bytes.h"
#include "lib/jxl/passes_state.h"
#include "lib/jxl/quant_weights.h"
#include "lib/jxl/quantizer.h"
#include "lib/jxl/splines.h"
#include "lib/jxl/toc.h"

namespace jxl {
Status Es1LossyFrameHeuristics(const FrameHeader& frame_header, PassesEncoderState* enc_state,
                               ModularFrameEncoder* modular_frame_encoder,
                               const Image3F* linear, Image3F* opsin, const Rect& rect,
                               const JxlCmsInterface& cms, ThreadPool* pool, AuxOut* aux_out);
Status Es1InitializePassesEncoder(const FrameHeader& frame_header, const Image3F& opsin,
                                  const Rect& rect, const JxlCmsInterface& cms, ThreadPool* pool,
                                  PassesEncoderState* passes_enc_state,
                                  ModularFrameEncoder* modular_frame_encoder, AuxOut* aux_out);
Status Es1ComputeARHeuristics(const FrameHeader& frame_header, PassesEncoderState* enc_state,
                              const Image3F& orig_opsin, const Rect& rect, ThreadPool* pool);
}  // namespace jxl

#define LossyFrameHeuristics Es1LossyFrameHeuristics
#define InitializePassesEncoder Es1InitializePassesEncoder
#define ComputeARHeuristics Es1ComputeARHeuristics
#define ParamsPostInit Es1ParamsPostInit
#define EncodeFrame Es1EncodeFrame
#define TectonicPlateSettingsLessPalette Es1TectonicPlateSettingsLessPalette
#define TectonicPlateSettingsMorePalette Es1TectonicPlateSettingsMorePalette
#include "lib/jxl/enc_frame.cc"
#undef LossyFrameHeuristics
#undef InitializePassesEncoder
#undef ComputeARHeuristics
#undef ParamsPostInit
#undef EncodeFrame
#undef TectonicPlateSettingsLessPalette
#undef TectonicPlateSettingsMorePalette

namespace {
std::ostringstream es1_checkpoints;

uint32_t CkBits(float v) { uint32_t b; std::memcpy(&b, &v, sizeof(b)); return b; }
void CkPlane(const jxl::ImageF& plane) {
  es1_checkpoints << " P " << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x)
      es1_checkpoints << ' ' << CkBits(plane.ConstRow(y)[x]);
}
template <typename T>
void CkInts(const jxl::Plane<T>& plane) {
  es1_checkpoints << " I " << plane.xsize() << ' ' << plane.ysize();
  for (size_t y = 0; y < plane.ysize(); ++y)
    for (size_t x = 0; x < plane.xsize(); ++x)
      es1_checkpoints << ' ' << int64_t(plane.ConstRow(y)[x]);
}
void CkStrategy(const jxl::AcStrategyImage& acs) {
  es1_checkpoints << " S";
  for (size_t y = 0; y < acs.ysize(); ++y) {
    const jxl::AcStrategyRow row = acs.ConstRow(y);
    for (size_t x = 0; x < acs.xsize(); ++x) {
      es1_checkpoints << ' ' << int(row[x].RawStrategy()) << ':' << row[x].IsFirstBlock();
    }
  }
}
void CkQuantizer(const jxl::Quantizer& q) {
  const jxl::QuantizerParams params = q.GetParams();
  es1_checkpoints << " Q " << params.global_scale << ' ' << params.quant_dc << ' '
                  << CkBits(q.Scale()) << ' ' << CkBits(q.InvGlobalScale()) << ' '
                  << CkBits(q.inv_quant_dc());
  for (size_t c = 0; c < 3; ++c) es1_checkpoints << ' ' << CkBits(q.MulDC()[c]);
  for (size_t c = 0; c < 3; ++c) es1_checkpoints << ' ' << CkBits(q.InvMulDC()[c]);
}
void CkColorCorrelation(const jxl::ColorCorrelationMap& cmap) {
  const jxl::ColorCorrelation& base = cmap.base();
  es1_checkpoints << " B " << base.ytox_dc_ << ' ' << base.ytob_dc_ << ' ' << base.color_factor_
                  << ' ' << CkBits(base.color_scale_) << ' ' << CkBits(base.base_correlation_x_)
                  << ' ' << CkBits(base.base_correlation_b_);
  for (size_t i = 0; i < 4; ++i) es1_checkpoints << ' ' << CkBits(base.dc_factors_[i]);
  CkInts(cmap.ytox_map);
  CkInts(cmap.ytob_map);
}
void CkBlockCtxMap(const jxl::BlockCtxMap& map) {
  es1_checkpoints << " K " << map.num_ctxs << ' ' << map.num_dc_ctxs;
  for (const auto& t : map.dc_thresholds) {
    es1_checkpoints << ' ' << t.size();
    for (int v : t) es1_checkpoints << ' ' << v;
  }
  es1_checkpoints << ' ' << map.qf_thresholds.size();
  for (uint32_t v : map.qf_thresholds) es1_checkpoints << ' ' << v;
  es1_checkpoints << ' ' << map.ctx_map.size();
  for (uint8_t v : map.ctx_map) es1_checkpoints << ' ' << int(v);
}
void CkMatrices(const jxl::DequantMatrices& matrices) {
  es1_checkpoints << " M";
  for (const jxl::QuantEncoding& e : matrices.encodings()) {
    es1_checkpoints << ' ' << int(e.mode) << ',' << int(e.predefined);
  }
  for (size_t c = 0; c < 3; ++c) es1_checkpoints << ' ' << CkBits(matrices.DCQuant(c));
}
}  // namespace

namespace jxl {
// Checkpoint H: LossyFrameHeuristics' outputs (enc_heuristics.cc): the
// Gaborished opsin, the masking image AR uses, the AC strategy, the raw
// quant field (after AdjustQuantField and SetQuantFieldRect), the quantizer,
// the CfL maps and DC correlation, the EPF sharpness it fills, the block
// context map (FindBestBlockEntropyModel), the dequant matrices
// (FindBestDequantMatrices) and whether splines or patches were found.
Status Es1LossyFrameHeuristics(const FrameHeader& frame_header, PassesEncoderState* enc_state,
                               ModularFrameEncoder* modular_frame_encoder,
                               const Image3F* linear, Image3F* opsin, const Rect& rect,
                               const JxlCmsInterface& cms, ThreadPool* pool, AuxOut* aux_out) {
  const Status status = LossyFrameHeuristics(frame_header, enc_state, modular_frame_encoder,
                                             linear, opsin, rect, cms, pool, aux_out);
  es1_checkpoints << " H " << int(status.code());
  if (!status) return status;
  const PassesSharedState& shared = enc_state->shared;
  es1_checkpoints << " F " << int(frame_header.loop_filter.gab) << ' '
                  << frame_header.loop_filter.epf_iters << ' ' << frame_header.x_qm_scale << ' '
                  << frame_header.b_qm_scale;
  for (size_t c = 0; c < 3; ++c) CkPlane(opsin->Plane(c));
  CkPlane(enc_state->initial_quant_masking1x1);
  CkStrategy(shared.ac_strategy);
  CkInts(shared.raw_quant_field);
  CkQuantizer(shared.quantizer);
  CkColorCorrelation(shared.cmap);
  CkInts(shared.epf_sharpness);
  CkBlockCtxMap(shared.block_ctx_map);
  CkMatrices(shared.matrices);
  es1_checkpoints << " X " << shared.image_features.splines.HasAny() << ' '
                  << shared.image_features.patches.HasAny();
  return status;
}

// Checkpoint I: InitializePassesEncoder's DC image, quantized DC and AC
// coefficients of each pass.
Status Es1InitializePassesEncoder(const FrameHeader& frame_header, const Image3F& opsin,
                                  const Rect& rect, const JxlCmsInterface& cms, ThreadPool* pool,
                                  PassesEncoderState* enc_state,
                                  ModularFrameEncoder* modular_frame_encoder, AuxOut* aux_out) {
  const Status status = InitializePassesEncoder(frame_header, opsin, rect, cms, pool, enc_state,
                                                modular_frame_encoder, aux_out);
  es1_checkpoints << " I " << int(status.code());
  if (!status) return status;
  const PassesSharedState& shared = enc_state->shared;
  for (size_t c = 0; c < 3; ++c) CkPlane(shared.dc_storage.Plane(c));
  CkInts(shared.quant_dc);
  // Each pass holds one row per group; ComputeCoefficients writes the
  // first 64 words per block of the group (the rest is never written).
  const FrameDimensions& frame_dim = shared.frame_dim;
  es1_checkpoints << " C " << enc_state->coeffs.size();
  for (const auto& coeffs : enc_state->coeffs) {
    const auto& image = static_cast<const ACImageT<int32_t>&>(*coeffs).img_;
    for (size_t g = 0; g < frame_dim.num_groups; ++g) {
      const Rect r = frame_dim.BlockGroupRect(g);
      const size_t n = r.xsize() * r.ysize() * kDCTBlockSize;
      es1_checkpoints << " G " << g << ' ' << n;
      for (size_t c = 0; c < 3; ++c) {
        const int32_t* row = image.ConstPlaneRow(c, g);
        for (size_t i = 0; i < n; ++i) es1_checkpoints << ' ' << row[i];
      }
    }
  }
  return status;
}

// Checkpoint A: ComputeARHeuristics' EPF sharpness field.
Status Es1ComputeARHeuristics(const FrameHeader& frame_header, PassesEncoderState* enc_state,
                              const Image3F& orig_opsin, const Rect& rect, ThreadPool* pool) {
  const Status status = ComputeARHeuristics(frame_header, enc_state, orig_opsin, rect, pool);
  es1_checkpoints << " A " << int(status.code());
  if (!status) return status;
  CkInts(enc_state->shared.epf_sharpness);
  return status;
}
}  // namespace jxl

namespace {
uint32_t CkContentHash(uint32_t x, uint32_t y, uint32_t c) {
  uint32_t h = x * 0x9E3779B1u ^ (y + 0x7F4A7C15u) * 0x85EBCA77u ^ (c + 1) * 0xC2B2AE3Du;
  h ^= h >> 15;
  h *= 0x2C1B3C6Du;
  h ^= h >> 12;
  return h;
}
// The generated content of encoder_public.cc's Content (ContentQ).
uint32_t CkContentQ(uint32_t content, uint32_t x, uint32_t y, uint32_t c, uint32_t xsize,
                    uint32_t ysize) {
  auto tri = [](uint32_t t) { uint32_t m = t % 512; return m < 256 ? m : 511 - m; };
  switch (content) {
    case 1: return 300 + 200 * c;
    case 2: return (x * 700) / std::max(xsize, 1u) + (y * 300) / std::max(ysize, 1u) + c * 10;
    case 3: return CkContentHash(x, y, c) % 1025;
    default: {
      uint32_t q = 150 + tri(x * 3 + y * 2 + c * 40) + tri(x * 11 / 7 + 5 * c) / 2;
      if ((x / 13 + y / 9) % 5 == 0) q += 120;
      q += CkContentHash(x, y, c) % 48;
      return std::min(q, 1024u);
    }
  }
}
float CkFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
}  // namespace

bool EncoderStep1CheckpointsCommand(const std::string& command, std::istringstream& in,
                                    JxlMemoryManager* mm) {
  if (command != "es1-ckpt") return false;
  size_t xsize, ysize; unsigned gray, linear, content;
  in >> xsize >> ysize >> gray >> linear >> content;
  const float distance = CkFloat(in);
  const float intensity = CkFloat(in);
  const size_t channels = gray ? 1 : 3;
  std::vector<float> pixels(xsize * ysize * channels);
  for (size_t y = 0; y < ysize; ++y)
    for (size_t x = 0; x < xsize; ++x)
      for (size_t c = 0; c < channels; ++c)
        pixels[(y * xsize + x) * channels + c] =
            float(CkContentQ(content, x, y, c, xsize, ysize)) / 1024.0f;

  // The metadata and frame settings of the original public encoder after
  // JxlEncoderSetBasicInfo (bits 32, exponent 8, uses_original_profile
  // false), JxlEncoderSetColorEncoding (ColorEncodingSetToSRGB or
  // ColorEncodingSetToLinearSRGB), JxlEncoderFrameSettingsCreate and
  // JxlEncoderSetFrameDistance, read from the encoder's own state.
  JxlEncoder* enc = JxlEncoderCreate(nullptr);
  if (!enc) throw std::runtime_error("ckpt encoder");
  JxlBasicInfo info;
  JxlEncoderInitBasicInfo(&info);
  info.xsize = xsize;
  info.ysize = ysize;
  info.bits_per_sample = 32;
  info.exponent_bits_per_sample = 8;
  info.num_color_channels = channels;
  info.intensity_target = intensity;
  JxlColorEncoding external;
  if (linear) {
    JxlColorEncodingSetToLinearSRGB(&external, gray);
  } else {
    JxlColorEncodingSetToSRGB(&external, gray);
  }
  JxlEncoderFrameSettings* settings = nullptr;
  if (JxlEncoderSetBasicInfo(enc, &info) != JXL_ENC_SUCCESS ||
      JxlEncoderSetColorEncoding(enc, &external) != JXL_ENC_SUCCESS ||
      !(settings = JxlEncoderFrameSettingsCreate(enc, nullptr)) ||
      JxlEncoderSetFrameDistance(settings, distance) != JXL_ENC_SUCCESS) {
    throw std::runtime_error("ckpt public setup");
  }
  jxl::CodecMetadata metadata = enc->metadata;
  jxl::CompressParams cparams = settings->values.cparams;
  JxlEncoderDestroy(enc);
  // ProcessOneEnqueuedInput's FrameInfo for the last, regular frame.
  jxl::FrameInfo frame_info;
  frame_info.is_last = true;
  jxl::JxlEncoderChunkedFrameAdapter input(xsize, ysize, 0);
  const JxlPixelFormat format{static_cast<uint32_t>(channels), JXL_TYPE_FLOAT, JXL_NATIVE_ENDIAN, 0};
  if (!input.SetFromBuffer(0, reinterpret_cast<const uint8_t*>(pixels.data()),
                           pixels.size() * sizeof(float), format)) {
    throw std::runtime_error("ckpt input");
  }
  jxl::BitWriter writer(mm);
  jxl::Status status = jxl::WriteCodestreamHeaders(&metadata, &writer, nullptr);
  if (status) {
    status = writer.WithMaxBits(8, jxl::LayerType::Header, nullptr, [&] {
      writer.ZeroPadToByte();
      return true;
    });
  }
  std::vector<uint8_t> bytes;
  if (status) {
    auto header = std::move(writer).TakeBytes();
    bytes.assign(header.begin(), header.end());
  }
  es1_checkpoints.str("");
  JxlEncoderOutputProcessorWrapper output(mm);
  if (status) {
    status = jxl::Es1EncodeFrame(mm, cparams, frame_info, &metadata, input, *JxlGetDefaultCms(),
                                 nullptr, &output, nullptr, nullptr);
  }
  std::vector<uint8_t> frame;
  if (status) status = output.SetFinalizedPosition();
  if (status) status = output.CopyOutput(frame);
  std::cout << "E " << int(status.code()) << es1_checkpoints.str() << " W ";
  bytes.insert(bytes.end(), frame.begin(), frame.end());
  if (!status || bytes.empty()) {
    std::cout << '-';
    return true;
  }
  static const char kDigits[] = "0123456789abcdef";
  for (uint8_t b : bytes) std::cout << kDigits[b >> 4] << kDigits[b & 15];
  return true;
}
