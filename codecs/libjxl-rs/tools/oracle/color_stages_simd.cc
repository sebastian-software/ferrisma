// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Row requests for the color stages of port ADR 0004 Plan item 1, observed
// through the original's public factories (HWY_DYNAMIC_DISPATCH selects the
// oracle's single Highway target). This TU supplies no stage arithmetic.
//   cl-tl <tf> <have_gamma> <gamma> <orig_it> <desired_it> <inverse_gamma>
//         <lum0> <lum1> <lum2> <width> <origin> <n> <n words>
//   cl-tm <orig_tf> <orig_have_gamma> <orig_gamma> <tf> <have_gamma> <gamma>
//         <orig_it> <desired_it> <lum0> <lum1> <lum2> <width> <origin> <n>
//         <n words>
//   cl-cms <src cs wp primaries tf> <dst cs wp primaries tf> <desired_it>
//         <width> <origin> <n> <n words>
// cl-tm and cl-cms print "null" when the factory returns no stage. cl-cms
// runs GetCmsStage(info, false) with the original's default CMS
// (JxlGetDefaultCms) after SetInputSizes(3 x {width, 1}) and
// PrepareForThreads(1); both encodings come from their enum fields through
// CreateICC.
// Each in-place request fills five channel rows with words[(c * 7 + x) % n],
// runs ProcessRow(rows, {}, 0, 0, width, 19, 23, 0) and prints the status
// followed by all five rows (len = origin + kRenderPipelineXOffset + width +
// 16 samples each), as the rs-* requests of render_stages_simd.cc do.
#include <jxl/cms.h>

#include <cstdint>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/dec_xyb.h"
#include "lib/jxl/image.h"
#include "lib/jxl/render_pipeline/render_pipeline_stage.h"
#include "lib/jxl/render_pipeline/stage_cms.h"
#include "lib/jxl/render_pipeline/stage_to_linear.h"
#include "lib/jxl/render_pipeline/stage_tone_mapping.h"

namespace {
template <class T, class U> T CLBits(U value) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T result;
  std::memcpy(&result, &value, sizeof(result));
  return result;
}
float CLFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  return CLBits<float>(bits);
}
std::vector<float> CLWords(std::istringstream& in) {
  size_t n;
  in >> n;
  if (n == 0 || n > 4096) throw std::runtime_error("cl: word count");
  std::vector<float> words(n);
  for (float& w : words) w = CLFloat(in);
  return words;
}

// Five channel rows; the stage's row pointer (sample `origin`) lies on a
// 128-byte boundary, as libjxl's own pipeline rows do (issue #252), and
// holds kRenderPipelineXOffset + width + 16 samples after `origin`.
void CLInPlace(jxl::RenderPipelineStage& stage, size_t width, size_t origin,
               const std::vector<float>& words, JxlMemoryManager* mm) {
  const size_t len = origin + jxl::kRenderPipelineXOffset + width + 16;
  const size_t row_align = 128 / sizeof(float);
  const size_t shift = (row_align - origin % row_align) % row_align;
  std::vector<jxl::ImageF> planes;
  jxl::RenderPipelineStage::RowInfo rows(5), unused;
  for (size_t c = 0; c < 5; ++c) {
    auto made = jxl::ImageF::Create(mm, len + row_align, 1);
    if (!made.ok()) throw std::runtime_error("cl: row allocation");
    planes.push_back(std::move(made).value_());
    float* row = planes.back().Row(0) + shift;
    for (size_t x = 0; x < len; ++x) row[x] = words[(c * 7 + x) % words.size()];
    rows[c].push_back(row + origin);
  }
  auto status = stage.ProcessRow(rows, unused, 0, 0, width, 19, 23, 0);
  std::cout << int(status.code());
  for (const auto& p : planes)
    for (size_t x = 0; x < len; ++x) std::cout << ' ' << CLBits<uint32_t>((p.Row(0) + shift)[x]);
}

void CLTransfer(std::istringstream& in, jxl::ColorEncoding& encoding) {
  unsigned tf, have_gamma;
  uint32_t gamma;
  in >> tf >> have_gamma >> gamma;
  encoding.Tf().SetTransferFunction(static_cast<jxl::TransferFunction>(tf));
  encoding.Tf().have_gamma = have_gamma;
  encoding.Tf().gamma = gamma;
}

jxl::ColorEncoding CLEncoding(std::istringstream& in) {
  unsigned cs, wp, primaries, tf;
  in >> cs >> wp >> primaries >> tf;
  jxl::ColorEncoding c;
  c.SetColorSpace(static_cast<jxl::ColorSpace>(cs));
  if (!c.SetWhitePointType(static_cast<jxl::WhitePoint>(wp)))
    throw std::runtime_error("cl: white point");
  if (c.HasPrimaries() && !c.SetPrimariesType(static_cast<jxl::Primaries>(primaries)))
    throw std::runtime_error("cl: primaries");
  c.Tf().SetTransferFunction(static_cast<jxl::TransferFunction>(tf));
  if (!c.CreateICC()) throw std::runtime_error("cl: CreateICC");
  return c;
}
}  // namespace

bool ColorStagesSimdCommand(const std::string& command, std::istringstream& in,
                            JxlMemoryManager* mm) {
  if (command.rfind("cl-", 0) != 0) return false;
  std::unique_ptr<jxl::RenderPipelineStage> stage;
  jxl::OutputEncodingInfo info{};
  if (command == "cl-tl") {
    CLTransfer(in, info.color_encoding);
    info.orig_intensity_target = CLFloat(in);
    info.desired_intensity_target = CLFloat(in);
    info.inverse_gamma = CLFloat(in);
    for (auto& l : info.luminances) l = CLFloat(in);
    stage = jxl::GetToLinearStage(info);
  } else if (command == "cl-tm") {
    CLTransfer(in, info.orig_color_encoding);
    CLTransfer(in, info.color_encoding);
    info.orig_intensity_target = CLFloat(in);
    info.desired_intensity_target = CLFloat(in);
    for (auto& l : info.luminances) l = CLFloat(in);
    stage = jxl::GetToneMappingStage(info);
    if (!stage) {
      size_t width, origin;
      in >> width >> origin;
      CLWords(in);
      std::cout << "null";
      return true;
    }
  } else if (command == "cl-cms") {
    info.orig_color_encoding = CLEncoding(in);
    info.color_encoding = CLEncoding(in);
    info.desired_intensity_target = CLFloat(in);
    info.cms_set = true;
    info.color_management_system = *JxlGetDefaultCms();
    stage = jxl::GetCmsStage(info, /*linear=*/false);
    size_t width, origin;
    in >> width >> origin;
    const auto words = CLWords(in);
    if (!stage) {
      std::cout << "null";
      return true;
    }
    std::vector<std::pair<size_t, size_t>> sizes(3, {width, 1});
    if (!stage->SetInputSizes(sizes)) throw std::runtime_error("cl: SetInputSizes");
    if (!stage->PrepareForThreads(1)) throw std::runtime_error("cl: PrepareForThreads");
    CLInPlace(*stage, width, origin, words, mm);
    return true;
  } else {
    throw std::runtime_error("cl: unknown command " + command);
  }
  size_t width, origin;
  in >> width >> origin;
  const auto words = CLWords(in);
  if (!stage) throw std::runtime_error("cl: no stage");
  CLInPlace(*stage, width, origin, words, mm);
  return true;
}
