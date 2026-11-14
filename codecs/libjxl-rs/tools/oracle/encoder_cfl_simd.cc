// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the encoder's chroma-from-luma SIMD kernels (port ADR 0004,
// "Encoder path"), observed at the oracle's single Highway target. The
// kernels are reached through the original's public, dispatching
// CfLHeuristics (HWY_DYNAMIC_DISPATCH reaches the one compiled target); this
// TU supplies no kernel arithmetic.
//   ecf-init <xsize> <ysize>
//       CfLHeuristics::Init(Rect(0, 0, xsize, ysize)) (InitDCStorage).
//       Prints the status, MaxVectorSize() / sizeof(float) (the lanes of
//       HWY_FULL(float)), the size of dc_values and, per row, the last
//       min(xsize, lanes) samples, the ones InitDCStorage zero-fills (the
//       others are uninitialized).
#include <jxl/memory_manager.h>

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>

#include "lib/jxl/base/rect.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/enc_chroma_from_luma.h"
#include "lib/jxl/image.h"
#include "lib/jxl/simd_util.h"

namespace {
void EcfWord(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  std::cout << ' ' << bits;
}
}  // namespace

bool EncoderCflSimdCommand(const std::string& command, std::istringstream& in,
                           JxlMemoryManager* mm) {
  if (command.rfind("ecf-", 0) != 0) return false;
  if (command == "ecf-init") {
    size_t xsize, ysize;
    in >> xsize >> ysize;
    jxl::CfLHeuristics cfl(mm);
    const jxl::Status status = cfl.Init(jxl::Rect(0, 0, xsize, ysize));
    const size_t lanes = jxl::MaxVectorSize() / sizeof(float);
    std::cout << "I " << int(status.code()) << ' ' << lanes;
    if (!status) return true;
    const jxl::ImageF& dc = cfl.dc_values;
    std::cout << ' ' << dc.xsize() << ' ' << dc.ysize();
    for (size_t y = 0; y < dc.ysize(); ++y) {
      std::cout << " R";
      for (size_t x = dc.xsize() - std::min(dc.xsize(), lanes); x < dc.xsize(); ++x)
        EcfWord(dc.ConstRow(y)[x]);
    }
    return true;
  }
  throw std::runtime_error("unknown encoder CfL SIMD command");
}
