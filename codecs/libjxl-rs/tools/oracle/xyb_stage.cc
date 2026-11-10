// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// A dedicated reference translation unit keeps its numeric configuration local.
#include <array>
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
#include "lib/jxl/memory_manager_internal.h"
#include "lib/jxl/render_pipeline/simple_render_pipeline.h"
#include "lib/jxl/render_pipeline/stage_write.h"
namespace {
template <class T, class U> T SCBits(U value) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T result; std::memcpy(&result, &value, sizeof(result)); return result;
}
}
float RPValue(size_t, size_t, size_t, size_t, unsigned);
void RPPipeline(jxl::SimpleRenderPipeline&);
void MGOutput(const jxl::Image3F&);
#include "xyb_stage.inc"
