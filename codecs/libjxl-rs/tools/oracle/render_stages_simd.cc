// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Row requests for the render stages of port ADR 0004 Plan item 1, observed
// through the original's public factories (HWY_DYNAMIC_DISPATCH selects the
// oracle's single Highway target). This TU supplies no stage arithmetic.
//   rs-xyb <custom> <params> <space> <width> <origin> <n> <n words>
//   rs-fl <tf> <have_gamma> <gamma> <orig_it> <desired_it> <inverse_gamma>
//         <lum0> <lum1> <lum2> <width> <origin> <n> <n words>
// Each in-place request fills five channel rows with words[(c * 7 + x) % n],
// runs ProcessRow(rows, {}, 0, 0, width, 19, 23, 0) and prints the status
// followed by all five rows (len = origin + kRenderPipelineXOffset + width +
// 16 samples each).
#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/dec_xyb.h"
#include "lib/jxl/image.h"
#include "lib/jxl/render_pipeline/render_pipeline_stage.h"
#include "lib/jxl/render_pipeline/stage_from_linear.h"
#include "lib/jxl/render_pipeline/stage_xyb.h"

namespace {
template <class T, class U> T RSBits(U value) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T result;
  std::memcpy(&result, &value, sizeof(result));
  return result;
}
float RSFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  return RSBits<float>(bits);
}
std::vector<float> RSWords(std::istringstream& in) {
  size_t n;
  in >> n;
  if (n == 0 || n > 4096) throw std::runtime_error("rs: word count");
  std::vector<float> words(n);
  for (float& w : words) w = RSFloat(in);
  return words;
}

// Five channel rows; the stage's row pointer (sample `origin`) lies on a
// 128-byte boundary, as libjxl's own pipeline rows do (issue #252). A stage
// processes the samples from GetInputRow = row + kRenderPipelineXOffset on,
// whole vectors to RoundUpTo(width, Lanes(d)), so the row holds
// kRenderPipelineXOffset + width + 16 samples after `origin`, as libjxl's
// pipeline rows hold the offset, the group and vector padding.
void RSInPlace(jxl::RenderPipelineStage& stage, size_t width, size_t origin,
               const std::vector<float>& words, JxlMemoryManager* mm) {
  const size_t len = origin + jxl::kRenderPipelineXOffset + width + 16;
  const size_t row_align = 128 / sizeof(float);
  const size_t shift = (row_align - origin % row_align) % row_align;
  std::vector<jxl::ImageF> planes;
  jxl::RenderPipelineStage::RowInfo rows(5), unused;
  for (size_t c = 0; c < 5; ++c) {
    auto made = jxl::ImageF::Create(mm, len + row_align, 1);
    if (!made.ok()) throw std::runtime_error("rs: row allocation");
    planes.push_back(std::move(made).value_());
    float* row = planes.back().Row(0) + shift;
    for (size_t x = 0; x < len; ++x) row[x] = words[(c * 7 + x) % words.size()];
    rows[c].push_back(row + origin);
  }
  auto status = stage.ProcessRow(rows, unused, 0, 0, width, 19, 23, 0);
  std::cout << int(status.code());
  for (const auto& p : planes)
    for (size_t x = 0; x < len; ++x) std::cout << ' ' << RSBits<uint32_t>((p.Row(0) + shift)[x]);
}

jxl::OpsinParams RSOpsin(std::istringstream& in) {
  unsigned custom;
  in >> custom;
  jxl::OpsinParams params{};
  if (!custom) {
    params.Init(RSFloat(in));
  } else {
    for (auto& v : params.inverse_opsin_matrix) v = RSFloat(in);
    for (auto& v : params.opsin_biases) v = RSFloat(in);
    for (auto& v : params.opsin_biases_cbrt) v = RSFloat(in);
    for (auto& v : params.quant_biases) v = RSFloat(in);
  }
  return params;
}
}  // namespace

bool RenderStagesSimdCommand(const std::string& command, std::istringstream& in,
                             JxlMemoryManager* mm) {
  if (command.rfind("rs-", 0) != 0) return false;
  std::unique_ptr<jxl::RenderPipelineStage> stage;
  jxl::OutputEncodingInfo info{};
  if (command == "rs-xyb") {
    info.opsin_params = RSOpsin(in);
    unsigned space;
    in >> space;
    info.color_encoding.SetColorSpace(static_cast<jxl::ColorSpace>(space));
    stage = jxl::GetXYBStage(info);
  } else if (command == "rs-fl") {
    unsigned tf, have_gamma;
    uint32_t gamma;
    in >> tf >> have_gamma >> gamma;
    info.color_encoding.Tf().SetTransferFunction(static_cast<jxl::TransferFunction>(tf));
    info.color_encoding.Tf().have_gamma = have_gamma;
    info.color_encoding.Tf().gamma = gamma;
    info.orig_intensity_target = RSFloat(in);
    info.desired_intensity_target = RSFloat(in);
    info.inverse_gamma = RSFloat(in);
    for (auto& l : info.luminances) l = RSFloat(in);
    stage = jxl::GetFromLinearStage(info);
  } else {
    throw std::runtime_error("rs: unknown command " + command);
  }
  size_t width, origin;
  in >> width >> origin;
  const auto words = RSWords(in);
  if (!stage) throw std::runtime_error("rs: no stage");
  RSInPlace(*stage, width, origin, words, mm);
  return true;
}
