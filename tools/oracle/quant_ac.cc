// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe the actual pinned quantization methods and their owning storage.
#include <array>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "lib/jxl/quant_weights.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/dec_modular.h"
#include "lib/jxl/enc_modular.h"
#include "lib/jxl/enc_quant_weights.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image_metadata.h"
namespace {
template <class T, class U> T QACBits(U value) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T result; std::memcpy(&result, &value, sizeof(result)); return result;
}
}
jxl::QuantEncoding QSRead(std::istringstream&, std::vector<int>&);
void QSEncoding(const jxl::QuantEncoding&);
void QSParams(const jxl::DctQuantWeightParams&);
#include "quant_ac.inc"
