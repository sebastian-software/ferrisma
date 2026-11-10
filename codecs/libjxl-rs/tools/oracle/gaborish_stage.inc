// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Call the actual original scalar stage, including its factory and pipeline.
#include "lib/jxl/render_pipeline/stage_gaborish.h"
jxl::LoopFilter GabReadFilter(std::istringstream& in) {
  jxl::LoopFilter lf;
  unsigned enabled; in >> enabled; lf.gab = enabled != 0;
  for (float* p : {&lf.gab_x_weight1, &lf.gab_x_weight2,
                   &lf.gab_y_weight1, &lf.gab_y_weight2,
                   &lf.gab_b_weight1, &lf.gab_b_weight2}) {
    uint32_t bits; in >> bits; *p = SCBits<float>(bits);
  }
  return lf;
}
bool GaborishStageCommand(const std::string& command, std::istringstream& in,
                          JxlMemoryManager* mm) {
  if (command.rfind("gab-", 0) != 0) return false;
  const auto lf = GabReadFilter(in);
  auto stage = jxl::GetGaborishStage(lf);
  if (command == "gab-factory") {
    std::cout << bool(stage);
    if (stage) {
      auto s = stage->settings_;
      std::cout << ' ' << s.shift_x << ' ' << s.shift_y << ' '
                << s.border_x << ' ' << s.border_y << ' ' << stage->GetName();
      for (size_t c : {size_t{0}, size_t{1}, size_t{2}, size_t{3}, size_t{4}, SIZE_MAX})
        std::cout << ' ' << int(stage->GetChannelMode(c));
      for (size_t n : {size_t{0}, size_t{2}, size_t{1}})
        std::cout << ' ' << int(stage->PrepareForThreads(n).code());
      std::cout << ' ' << int(stage->IsInitialized().code());
    }
    return true;
  }
  if (command == "gab-row") {
    size_t left, right, width, origin;
    in >> left >> right >> width >> origin;
    auto values = SCRead<float>(in, 9);
    const size_t len = origin + jxl::kRenderPipelineXOffset + width + right + 3;
    std::vector<jxl::ImageF> input, output;
    for (size_t c = 0; c < 5; ++c) {
      auto ai = jxl::ImageF::Create(mm, len, 7);
      auto ao = jxl::ImageF::Create(mm, len, 4);
      if (!ai.ok() || !ao.ok()) throw std::runtime_error("Gaborish row allocation");
      input.push_back(std::move(ai).value_()); output.push_back(std::move(ao).value_());
      for (size_t y = 0; y < 7; ++y)
        for (size_t x = 0; x < len; ++x) input[c].Row(y)[x] = values[(c * 7 + y * 3 + x) % 9];
      for (size_t y = 0; y < 4; ++y)
        for (size_t x = 0; x < len; ++x)
          output[c].Row(y)[x] = SCBits<float>(uint32_t(0x7fa00000 + c * 256 + y * 64 + x));
    }
    jxl::RenderPipelineStage::RowInfo ir(5), orows(5);
    for (size_t c = 0; c < 5; ++c) {
      for (size_t y : {size_t{1}, size_t{3}, size_t{5}}) ir[c].push_back(input[c].Row(y) + origin);
      orows[c].push_back(output[c].Row(2) + origin);
    }
    auto status = stage->ProcessRow(ir, orows, left, right, width, 23, 41, 17);
    std::cout << int(status.code());
    for (const auto& plane : output)
      for (size_t y = 0; y < 4; ++y)
        for (size_t x = 0; x < len; ++x) std::cout << ' ' << SCBits<uint32_t>(plane.Row(y)[x]);
    return true;
  }
  if (command != "gab-simple") throw std::runtime_error("unknown Gaborish command");
  size_t w, h, threads, passes; unsigned mode; bool reverse;
  in >> w >> h >> threads >> mode >> reverse >> passes;
  jxl::FrameDimensions dim; dim.Set(w, h, 0, 0, 0, true, 1); jxl::Image3F output;
  {
    jxl::RenderPipeline::Builder builder(mm, 3); builder.UseSimpleImplementation();
    auto added = builder.AddStage(std::move(stage)); std::cout << " B " << int(added.code());
    if (!added) return true;
    if (!builder.AddStage(jxl::GetWriteToImage3FStage(mm, &output))) throw std::runtime_error("Gaborish writer");
    auto made = std::move(builder).Finalize(dim); if (!made.ok()) throw std::runtime_error("Gaborish pipeline");
    auto base = std::move(made).value_(); auto& pipeline = *static_cast<jxl::SimpleRenderPipeline*>(base.get());
    auto status = pipeline.PrepareForThreads(threads, true); std::cout << " T " << int(status.code());
    if (status) for (size_t pass = 0; pass < passes; ++pass) {
      for (size_t i = 0; i < dim.num_groups; ++i) {
        size_t g = reverse ? dim.num_groups - 1 - i : i; auto input = pipeline.GetInputBuffers(g, pass % threads);
        for (size_t c = 0; c < 3; ++c) {
          auto [plane, r] = input.GetBuffer(c);
          for (size_t y = 0; y < r.ysize(); ++y)
            for (size_t x = 0; x < r.xsize(); ++x)
              r.Row(plane, y)[x] = RPValue(c, r.x0() + x - jxl::kRenderPipelineXOffset, r.y0() + y - jxl::kRenderPipelineXOffset, pass, mode);
        }
        status = input.Done(); std::cout << " D " << int(status.code()); if (!status) break;
      }
      if (!status) break;
      status = pipeline.PrepareForThreads(threads, true); std::cout << " A " << int(status.code()); if (!status) break;
    }
    RPPipeline(pipeline);
  }
  MGOutput(output); return true;
}
