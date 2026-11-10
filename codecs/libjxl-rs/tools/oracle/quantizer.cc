// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe the pristine Quantizer and parameter bundle through their public APIs.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/span.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/fields.h"
#include "lib/jxl/quantizer.h"

namespace {
template <class T, class U> T QZBits(U value) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T result;
  std::memcpy(&result, &value, sizeof(result));
  return result;
}
float QZFloat(std::istringstream& in) {
  uint32_t bits;
  in >> bits;
  return QZBits<float>(bits);
}
struct QZInput {
  explicit QZInput(const std::string& hex) {
    size = hex == "-" ? 0 : hex.size() / 2;
    // Keep the source reader's end-minus-eight pointer within real storage,
    // including for a non-null zero-length input span.
    storage.resize(size + 16);
    for (size_t i = 0; i < size; ++i) {
      storage[8 + i] = static_cast<uint8_t>(std::stoul(hex.substr(i * 2, 2), nullptr, 16));
    }
  }
  jxl::Bytes bytes() const { return jxl::Bytes(storage.data() + 8, size); }
  std::vector<uint8_t> storage;
  size_t size;
};
}  // namespace

jxl::Status Prefix(jxl::BitWriter&, size_t);
jxl::Status Pad(jxl::BitWriter&);
std::string Hex(jxl::Bytes);
void MFSOutput(const jxl::BitWriter&, const jxl::AuxOut&, JxlMemoryManager*);
void SSQuant(const jxl::Quantizer&, const jxl::DequantMatrices&, size_t);

#include "quantizer.inc"
