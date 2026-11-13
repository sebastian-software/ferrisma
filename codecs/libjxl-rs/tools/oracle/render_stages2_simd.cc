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
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/render_pipeline/render_pipeline.h"
#include "lib/jxl/render_pipeline/render_pipeline_stage.h"
#include "lib/jxl/render_pipeline/stage_upsampling.h"
#include "lib/jxl/render_pipeline/stage_write.h"

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
}  // namespace

bool RenderStages2SimdCommand(const std::string& command, std::istringstream& in,
                              JxlMemoryManager* mm) {
  if (command == "s2-ups") {
    S2Ups(in, mm);
  } else if (command == "s2-ups-pipe") {
    S2UpsPipe(in, mm);
  } else {
    return false;
  }
  return true;
}
