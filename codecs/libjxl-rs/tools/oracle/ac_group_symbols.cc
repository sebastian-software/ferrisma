// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol framing only; the exact pinned decoder template is extracted at
// configure time, and entropy streams come from the original encoder.
#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/ac_context.h"
#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/printf_macros.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/dec_ans.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/dct_util.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/entropy_coder.h"
#include "lib/jxl/frame_dimensions.h"

int Code(jxl::Status);
std::string Hex(jxl::Span<const uint8_t>);
std::vector<uint8_t> Unhex(const std::string&);
jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status Pad(jxl::BitWriter&);

namespace jxl {
namespace {
#include "original_decode_ac_var_block.inc"
}
}
#include "ac_group_symbols.inc"
