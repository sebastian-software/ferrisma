// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Include unchanged source to call its private encoder conversion. Unused
// frame methods are discarded by the linker, without replacement definitions.
#include "lib/jxl/enc_modular.cc"
#include "lib/jxl/dec_modular.cc"

#include <iostream>
#include <sstream>
#include <string>

namespace {
template <class T, class U> T SCBits(U bits) {
  static_assert(sizeof(T) == sizeof(U), "equal bit widths");
  T value;
  memcpy(&value, &bits, sizeof(value));
  return value;
}
template <class T> std::vector<T> SCRead(std::istringstream& in, size_t n) {
  std::vector<T> values(n);
  for (auto& value : values) {
    uint32_t bits;
    in >> bits;
    value = SCBits<T>(bits);
  }
  return values;
}
template <class T> void SCWrite(const std::vector<T>& values) {
  for (auto value : values) std::cout << ' ' << SCBits<uint32_t>(value);
}
}  // namespace

bool SampleConversionCommand(const std::string& command, std::istringstream& in) {
  if (command == "sc-f2i") {
    unsigned bits, exp_bits;
    bool fp;
    uint64_t factor;
    size_t xsize, n;
    in >> bits >> exp_bits >> fp >> factor >> xsize >> n;
    const auto input = SCRead<float>(in, n);
    auto output = SCRead<int32_t>(in, n);
    auto status = jxl::float_to_int(input.data(), output.data(), xsize, bits,
                                    exp_bits, fp, SCBits<double>(factor));
    std::cout << static_cast<int>(status.code());
    SCWrite(output);
  } else if (command == "sc-i2f") {
    int bits, exp_bits;
    size_t xsize, n;
    in >> bits >> exp_bits >> xsize >> n;
    const auto input = SCRead<int32_t>(in, n);
    auto output = SCRead<float>(in, n);
    auto status = jxl::int_to_float(input.data(), output.data(), xsize, bits, exp_bits);
    std::cout << static_cast<int>(status.code());
    SCWrite(output);
  } else if (command == "sc-kernel") {
    unsigned mode;
    uint64_t double_factor;
    uint32_t float_factor;
    size_t xsize, n;
    in >> mode >> double_factor >> float_factor >> xsize >> n;
    const auto input = SCRead<int32_t>(in, n);
    const auto y = SCRead<int32_t>(in, n);
    auto r = SCRead<float>(in, n);
    auto g = SCRead<float>(in, n);
    auto b = SCRead<float>(in, n);
    const float factor = SCBits<float>(float_factor);
    if (mode == 0) {
      jxl::N_SCALAR::MultiplySum(xsize, input.data(), y.data(), factor, r.data());
    } else if (mode == 1) {
      jxl::N_SCALAR::RgbFromSingle(xsize, input.data(), factor, r.data(), g.data(), b.data());
    } else if (mode == 2) {
      jxl::N_SCALAR::SingleFromSingle(xsize, input.data(), factor, r.data());
    } else if (mode == 3) {
      jxl::SingleFromSingleAccurate(xsize, input.data(), SCBits<double>(double_factor), r.data());
    } else {
      throw std::runtime_error("unknown sample kernel");
    }
    std::cout << 'K';
    SCWrite(r);
    SCWrite(g);
    SCWrite(b);
  } else {
    return false;
  }
  return true;
}

#include "modular_frame.inc"
