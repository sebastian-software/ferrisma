// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the upsampling, noise, blending and splines render stages of
// port ADR 0004 Plan item 1 ("stages2"), observed through the original's
// public factories (HWY_DYNAMIC_DISPATCH selects the oracle's single Highway
// target). This TU supplies no stage arithmetic.
//
//   s2-ups <c> <shift> <nw> <nw weight words> <width> <n> <n words>
//     One UpsamplingStage::ProcessRow on fresh rows: five 128-byte aligned
//     input rows of kRenderPipelineXOffset + width + 40 samples, row iy
//     filled with words[(iy * 7 + x) % n]; N = 1 << shift output rows of
//     S2UpsOut(width, N) samples prefilled with 0xc1234567. Prints the
//     PrepareForThreads and ProcessRow statuses and every output sample.
//   s2-ups-pipe <simple> <w> <h> <shift> <c> <nw> <nw words> <n> <n words>
//     The stage in the simple (1) or low-memory (0) pipeline followed by
//     WriteToImage3F; the pipeline mirrors the image borders. Input sample
//     (c, x, y) of each group buffer is words[(c * 31 + x * 17 + y * 9) % n].
//     Prints the statuses and the output image.
// Weights: nw == 0 keeps CustomTransformData's defaults, otherwise weight i
// of the 2x/4x/8x tables is wwords[(i + 5 * table) % nw].
//   s2-noise-add <first_c> <origin> <width> <8 lut words> <x16> <b16> <n> <n words>
//     AddNoiseStage::ProcessRow on first_c + 3 single rows (row c filled
//     with words[(c * 7 + x) % n], the stage's row at sample `origin`, a
//     128-byte boundary) with a ColorCorrelation decoded from base
//     correlations x16/b16 (binary16; 65535 keeps the default). Prints the
//     status and every sample of every row.
//   s2-noise-conv <first_c> <origin> <width> <left> <right> <n> <n words>
//     ConvolveNoiseStage::ProcessRow on five input rows per channel (row y
//     of channel c filled with words[(c * 7 + y * 3 + x) % n]) and one
//     output row per channel prefilled with 0xc1234567. Prints the status
//     and every sample of every output row.
//   s2-noise-random <visible> <nonvisible> <x0> <y0> <w> <h> <rx> <ry>
//     Random3Planes of the oracle's target on three planes of rx + w + 24 by
//     ry + h + 2 samples (prefilled with -17 - c) at Rect(rx, ry, w, h).
//     Prints every sample of the three planes. Column rx lies on a 128-byte
//     boundary (the planes hold extra unprinted samples before column 0).
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/chroma_from_luma.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/noise.h"
#include "lib/jxl/render_pipeline/render_pipeline.h"
#include "lib/jxl/render_pipeline/render_pipeline_stage.h"
#include "lib/jxl/render_pipeline/stage_noise.h"
#include "lib/jxl/render_pipeline/stage_upsampling.h"
#include "lib/jxl/render_pipeline/stage_write.h"

namespace jxl {
namespace ORACLE_HWY_NS {
// Defined by noise_render.cc's inclusion of the pristine dec_noise.cc.
void Random3Planes(size_t visible_frame_index, size_t nonvisible_frame_index,
                   size_t x0, size_t y0, const std::pair<ImageF*, Rect>& plane0,
                   const std::pair<ImageF*, Rect>& plane1,
                   const std::pair<ImageF*, Rect>& plane2);
}  // namespace ORACLE_HWY_NS
}  // namespace jxl

namespace {
template <class T, class U> T S2Bits(U value) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T result;
  std::memcpy(&result, &value, sizeof(result));
  return result;
}
std::vector<float> S2Words(std::istringstream& in, bool allow_empty) {
  size_t n;
  in >> n;
  if ((n == 0 && !allow_empty) || n > 4096) throw std::runtime_error("s2: word count");
  std::vector<float> words(n);
  for (float& w : words) {
    uint32_t bits;
    in >> bits;
    w = S2Bits<float>(bits);
  }
  return words;
}
jxl::ImageF S2Plane(JxlMemoryManager* mm, size_t xsize, size_t ysize) {
  auto made = jxl::ImageF::Create(mm, xsize, ysize);
  if (!made.ok()) throw std::runtime_error("s2: allocation");
  return std::move(made).value_();
}

void S2Weights(std::istringstream& in, jxl::CustomTransformData& factors) {
  const auto w = S2Words(in, true);
  if (w.empty()) return;
  size_t table = 0;
  for (auto [p, n] : {std::pair<float*, size_t>{factors.upsampling2_weights, 15},
                      {factors.upsampling4_weights, 55},
                      {factors.upsampling8_weights, 210}}) {
    for (size_t i = 0; i < n; ++i) p[i] = w[(i + 5 * table) % w.size()];
    ++table;
  }
}

// Whole 8-lane vectors of N outputs each past the row, plus slack.
size_t S2UpsOut(size_t width, size_t n) {
  return jxl::kRenderPipelineXOffset + (width + 8) / 8 * 8 * n + 16;
}

void S2Ups(std::istringstream& in, JxlMemoryManager* mm) {
  size_t c, shift;
  in >> c >> shift;
  jxl::CustomTransformData factors;
  S2Weights(in, factors);
  size_t width;
  in >> width;
  const auto words = S2Words(in, false);
  auto stage = jxl::GetUpsamplingStage(mm, factors, c, shift);
  if (!stage) throw std::runtime_error("s2: no stage");
  const size_t n = size_t{1} << shift;
  const size_t in_len = jxl::kRenderPipelineXOffset + width + 40;
  const size_t out_len = S2UpsOut(width, n);
  // ImageF rows are 128-byte aligned, as libjxl's pipeline rows.
  jxl::ImageF input = S2Plane(mm, in_len, 5);
  jxl::ImageF output = S2Plane(mm, out_len, n);
  for (size_t y = 0; y < 5; ++y)
    for (size_t x = 0; x < in_len; ++x) input.Row(y)[x] = words[(y * 7 + x) % words.size()];
  for (size_t y = 0; y < n; ++y)
    for (size_t x = 0; x < out_len; ++x) output.Row(y)[x] = S2Bits<float>(uint32_t{0xc1234567});
  jxl::RenderPipelineStage::RowInfo input_rows(c + 1), output_rows(c + 1);
  for (size_t y = 0; y < 5; ++y) input_rows[c].push_back(input.Row(y));
  for (size_t y = 0; y < n; ++y) output_rows[c].push_back(output.Row(y));
  auto prepared = stage->PrepareForThreads(1);
  std::cout << int(prepared.code());
  auto status = stage->ProcessRow(input_rows, output_rows, 0, 0, width, 3, 5, 0);
  std::cout << ' ' << int(status.code());
  for (size_t y = 0; y < n; ++y)
    for (size_t x = 0; x < out_len; ++x) std::cout << ' ' << S2Bits<uint32_t>(output.Row(y)[x]);
}

void S2UpsPipe(std::istringstream& in, JxlMemoryManager* mm) {
  unsigned simple;
  size_t w, h, shift, c;
  in >> simple >> w >> h >> shift >> c;
  jxl::CustomTransformData factors;
  S2Weights(in, factors);
  const auto words = S2Words(in, false);
  jxl::FrameDimensions dim;
  dim.Set(w, h, 0, 0, 0, true, 1);
  jxl::Image3F output;
  {
    jxl::RenderPipeline::Builder builder(mm, 3);
    if (simple) builder.UseSimpleImplementation();
    if (!builder.AddStage(jxl::GetUpsamplingStage(mm, factors, c, shift)))
      throw std::runtime_error("s2: upsampling stage");
    if (!builder.AddStage(jxl::GetWriteToImage3FStage(mm, &output)))
      throw std::runtime_error("s2: write stage");
    auto made = std::move(builder).Finalize(dim);
    if (!made.ok()) throw std::runtime_error("s2: pipeline");
    auto pipeline = std::move(made).value_();
    auto status = pipeline->PrepareForThreads(1, true);
    std::cout << int(status.code());
    for (size_t g = 0; status && g < dim.num_groups; ++g) {
      auto input = pipeline->GetInputBuffers(g, 0);
      for (size_t cc = 0; cc < 3; ++cc) {
        auto [plane, r] = input.GetBuffer(cc);
        for (size_t y = 0; y < r.ysize(); ++y)
          for (size_t x = 0; x < r.xsize(); ++x)
            r.Row(plane, y)[x] = words[(cc * 31 + x * 17 + y * 9) % words.size()];
      }
      status = input.Done();
      std::cout << ' ' << int(status.code());
    }
  }
  std::cout << " O " << output.xsize() << ' ' << output.ysize();
  for (size_t cc = 0; cc < 3; ++cc)
    for (size_t y = 0; y < output.ysize(); ++y)
      for (size_t x = 0; x < output.xsize(); ++x)
        std::cout << ' ' << S2Bits<uint32_t>(output.PlaneRow(cc, y)[x]);
}
// A row of `len` samples whose sample `origin` lies on a 128-byte boundary.
struct S2Row {
  jxl::ImageF plane;
  float* row;
};
S2Row S2AlignedRow(JxlMemoryManager* mm, size_t len, size_t origin) {
  const size_t row_align = 128 / sizeof(float);
  const size_t shift = (row_align - origin % row_align) % row_align;
  S2Row r{S2Plane(mm, len + row_align, 1), nullptr};
  r.row = r.plane.Row(0) + shift;
  return r;
}

// ColorCorrelation::DecodeDC of: not all default, color factor 84, base
// correlations x16/b16 (binary16), ytox_dc = ytob_dc = 0.
void S2Correlation(std::istringstream& in, jxl::ColorCorrelation& cc) {
  uint32_t x16, b16;
  in >> x16 >> b16;
  if (x16 == 65535) return;
  const uint64_t bits = (uint64_t{x16} << 3) | (uint64_t{b16} << 19) |
                        (uint64_t{128} << 35) | (uint64_t{128} << 43);
  uint8_t bytes[8];
  for (size_t i = 0; i < 8; ++i) bytes[i] = uint8_t(bits >> (8 * i));
  jxl::BitReader br(jxl::Span<const uint8_t>(bytes, 8));
  auto status = cc.DecodeDC(&br);
  auto closed = br.Close();
  if (!status || !closed) throw std::runtime_error("s2: correlation");
}

void S2NoiseAdd(std::istringstream& in, JxlMemoryManager* mm) {
  size_t first_c, origin, width;
  in >> first_c >> origin >> width;
  jxl::NoiseParams params;
  for (float& v : params.lut) {
    uint32_t bits;
    in >> bits;
    v = S2Bits<float>(bits);
  }
  jxl::ColorCorrelation correlation;
  S2Correlation(in, correlation);
  const auto words = S2Words(in, false);
  const size_t channels = first_c + 3;
  const size_t len = origin + jxl::kRenderPipelineXOffset + width + 16;
  std::vector<S2Row> rows;
  jxl::RenderPipelineStage::RowInfo input_rows(channels), unused;
  for (size_t c = 0; c < channels; ++c) {
    rows.push_back(S2AlignedRow(mm, len, origin));
    for (size_t x = 0; x < len; ++x) rows[c].row[x] = words[(c * 7 + x) % words.size()];
    input_rows[c].push_back(rows[c].row + origin);
  }
  auto stage = jxl::GetAddNoiseStage(params, correlation, first_c);
  auto status = stage->ProcessRow(input_rows, unused, 0, 0, width, 19, 23, 0);
  std::cout << int(status.code());
  for (const auto& r : rows)
    for (size_t x = 0; x < len; ++x) std::cout << ' ' << S2Bits<uint32_t>(r.row[x]);
}

void S2NoiseConv(std::istringstream& in, JxlMemoryManager* mm) {
  size_t first_c, origin, width, left, right;
  in >> first_c >> origin >> width >> left >> right;
  const auto words = S2Words(in, false);
  const size_t channels = first_c + 3;
  const size_t len = origin + jxl::kRenderPipelineXOffset + width + right + 16;
  std::vector<S2Row> rows, outs;
  jxl::RenderPipelineStage::RowInfo input_rows(channels), output_rows(channels);
  for (size_t c = first_c; c < channels; ++c) {
    for (size_t y = 0; y < 5; ++y) {
      rows.push_back(S2AlignedRow(mm, len, origin));
      for (size_t x = 0; x < len; ++x)
        rows.back().row[x] = words[(c * 7 + y * 3 + x) % words.size()];
      input_rows[c].push_back(rows.back().row + origin);
    }
    outs.push_back(S2AlignedRow(mm, len, origin));
    for (size_t x = 0; x < len; ++x) outs.back().row[x] = S2Bits<float>(uint32_t{0xc1234567});
    output_rows[c].push_back(outs.back().row + origin);
  }
  auto stage = jxl::GetConvolveNoiseStage(first_c);
  auto status = stage->ProcessRow(input_rows, output_rows, left, right, width, 19, 23, 0);
  std::cout << int(status.code());
  for (const auto& r : outs)
    for (size_t x = 0; x < len; ++x) std::cout << ' ' << S2Bits<uint32_t>(r.row[x]);
}

void S2NoiseRandom(std::istringstream& in, JxlMemoryManager* mm) {
  size_t visible, nonvisible, x0, y0, w, h, rx, ry;
  in >> visible >> nonvisible >> x0 >> y0 >> w >> h >> rx >> ry;
  // RandomImage stores whole vectors with the aligned Store (vmovdqa on
  // AVX2) from the rect's first sample on, as libjxl's noise input rects
  // start at kRenderPipelineXOffset of 128-byte aligned rows. Column `rx` of
  // the observed planes therefore lies on a 128-byte boundary: each plane
  // holds `shift` samples before the observed rx + w + 24 columns (ImageF
  // rows start 128-byte aligned), and only the observed columns are printed.
  const size_t row_align = 128 / sizeof(float);
  const size_t shift = (row_align - rx % row_align) % row_align;
  const size_t xsize = rx + w + 24;
  std::vector<jxl::ImageF> planes;
  for (size_t c = 0; c < 3; ++c) {
    planes.push_back(S2Plane(mm, shift + xsize, ry + h + 2));
    jxl::FillImage(-17.0f - float(c), &planes.back());
  }
  const jxl::Rect r(shift + rx, ry, w, h);
  jxl::ORACLE_HWY_NS::Random3Planes(visible, nonvisible, x0, y0, {&planes[0], r},
                                    {&planes[1], r}, {&planes[2], r});
  for (const auto& p : planes)
    for (size_t y = 0; y < p.ysize(); ++y)
      for (size_t x = 0; x < xsize; ++x) std::cout << ' ' << S2Bits<uint32_t>(p.Row(y)[shift + x]);
}
}  // namespace

bool RenderStages2SimdCommand(const std::string& command, std::istringstream& in,
                              JxlMemoryManager* mm) {
  if (command == "s2-ups") {
    S2Ups(in, mm);
  } else if (command == "s2-ups-pipe") {
    S2UpsPipe(in, mm);
  } else if (command == "s2-noise-add") {
    S2NoiseAdd(in, mm);
  } else if (command == "s2-noise-conv") {
    S2NoiseConv(in, mm);
  } else if (command == "s2-noise-random") {
    S2NoiseRandom(in, mm);
  } else {
    return false;
  }
  return true;
}
