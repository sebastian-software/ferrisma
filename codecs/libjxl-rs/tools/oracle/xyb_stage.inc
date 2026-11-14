// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Calls the unmodified scalar helper, concrete stage, and public factory.
#include "lib/jxl/render_pipeline/stage_xyb.cc"
#include "oracle_target.h"
// Calls N_AVX2 operations directly (oracle_target.h).
ORACLE_TARGET_BEGIN
float XYBFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; return SCBits<float>(bits);
}
jxl::OpsinParams XYBParams(std::istringstream& in) {
  unsigned custom; in >> custom;
  float target = XYBFloat(in);
  jxl::OpsinParams params{};
  if (!custom) params.Init(target);
  else {
    for (auto& v : params.inverse_opsin_matrix) v = XYBFloat(in);
    for (auto& v : params.opsin_biases) v = XYBFloat(in);
    for (auto& v : params.opsin_biases_cbrt) v = XYBFloat(in);
    for (auto& v : params.quant_biases) v = XYBFloat(in);
  }
  return params;
}
// Keep the original inline helper's arithmetic independent of the much larger
// protocol parser, just as the native scalar boundary is kept out of line.
HWY_NOINLINE std::array<float, 3> XYBConvert(float x, float y, float b, const jxl::OpsinParams& params) {
  const hwy::ORACLE_HWY_NS::ScalableTag<float> d;
  auto r = hwy::ORACLE_HWY_NS::Undefined(d), g = r, out_b = r;
  jxl::ORACLE_HWY_NS::XybToRgb(d, hwy::ORACLE_HWY_NS::Set(d, x), hwy::ORACLE_HWY_NS::Set(d, y),
                         hwy::ORACLE_HWY_NS::Set(d, b), params, &r, &g, &out_b);
  return {hwy::ORACLE_HWY_NS::GetLane(r), hwy::ORACLE_HWY_NS::GetLane(g), hwy::ORACLE_HWY_NS::GetLane(out_b)};
}
bool XybStageCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command != "xyb-helper" && command != "xyb-stage" && command != "xyb-simple") return false;
  auto params = XYBParams(in);
  if (command == "xyb-helper") {
    float x = XYBFloat(in), y = XYBFloat(in), b = XYBFloat(in);
    auto rgb = XYBConvert(x, y, b, params);
    std::cout << SCBits<uint32_t>(rgb[0]) << ' ' << SCBits<uint32_t>(rgb[1]) << ' ' << SCBits<uint32_t>(rgb[2]);
    return true;
  }
  unsigned space, mode; size_t w, h, left, right, offset;
  in >> space >> w >> h >> left >> right >> offset >> mode;
  jxl::OutputEncodingInfo info{};
  info.opsin_params = params;
  info.color_encoding.SetColorSpace(static_cast<jxl::ColorSpace>(space));
  auto factory = jxl::GetXYBStage(info);
  // The original stage owns copies, independent of later caller mutations.
  info.opsin_params = jxl::OpsinParams{};
  info.color_encoding.SetColorSpace(space == 2 ? jxl::ColorSpace::kRGB : jxl::ColorSpace::kXYB);
  if (command == "xyb-stage") {
    auto settings = factory->settings_;
    std::cout << factory->GetName() << ' ' << settings.shift_x << ' ' << settings.shift_y
              << ' ' << settings.border_x << ' ' << settings.border_y;
    for (size_t c = 0; c < 6; ++c) std::cout << ' ' << int(factory->GetChannelMode(c));
    std::vector<jxl::ImageF> planes;
    jxl::RenderPipelineStage::RowInfo rows(4), unused;
    for (size_t c = 0; c < 4; ++c) {
      auto made = jxl::ImageF::Create(mm, 128, 1);
      if (!made.ok()) throw std::runtime_error("XYB row");
      planes.push_back(std::move(made).value_());
      for (size_t x = 0; x < 128; ++x) planes.back().Row(0)[x] = RPValue(c, x, 0, 0, mode);
      rows[c].push_back(planes.back().Row(0) + offset);
    }
    auto status = factory->ProcessRow(rows, unused, left, right, w, 19, 23, 1);
    std::cout << " R " << int(status.code());
    for (const auto& p : planes) for (size_t x = 0; x < 128; ++x)
      std::cout << ' ' << SCBits<uint32_t>(p.Row(0)[x]);
    return true;
  }
  jxl::FrameDimensions dim; dim.Set(w, h, 0, 0, 0, true, 1);
  jxl::Image3F output;
  {
    jxl::RenderPipeline::Builder builder(mm, 3); builder.UseSimpleImplementation();
    if (!builder.AddStage(std::move(factory))) throw std::runtime_error("XYB stage");
    if (!builder.AddStage(jxl::GetWriteToImage3FStage(mm, &output))) throw std::runtime_error("XYB write");
    auto made = std::move(builder).Finalize(dim);
    if (!made.ok()) throw std::runtime_error("XYB pipeline");
    auto base = std::move(made).value_();
    auto& pipeline = *static_cast<jxl::SimpleRenderPipeline*>(base.get());
    auto status = pipeline.PrepareForThreads(2, true);
    std::cout << " T " << int(status.code());
    if (status) for (size_t i = 0; i < dim.num_groups; ++i) {
      auto input = pipeline.GetInputBuffers(dim.num_groups - 1 - i, i % 2);
      for (size_t c = 0; c < 3; ++c) {
        auto [plane, rect] = input.GetBuffer(c);
        for (size_t y = 0; y < rect.ysize(); ++y) for (size_t x = 0; x < rect.xsize(); ++x)
          rect.Row(plane, y)[x] = RPValue(c, rect.x0() + x - jxl::kRenderPipelineXOffset,
                                       rect.y0() + y - jxl::kRenderPipelineXOffset, 0, mode);
      }
      status = input.Done(); std::cout << " D " << int(status.code()); if (!status) break;
    }
    RPPipeline(pipeline);
  }
  MGOutput(output); return true;
}
ORACLE_TARGET_END
