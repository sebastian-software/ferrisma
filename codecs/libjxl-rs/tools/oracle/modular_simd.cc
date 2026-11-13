// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the modular and output-conversion kernels of port ADR 0004
// Plan item 2, observed at the oracle's single Highway target. The kernels
// are the original's exported HWY_NAMESPACE functions (ORACLE_HWY_NS), called
// on 128-byte aligned buffers as libjxl's own plane rows are (issue #252);
// this TU supplies no kernel arithmetic.
//   mx-u32 <bits> <num> <m> <m words>  FloatToU32(in, out, num,
//                                      (1 << bits) - 1, bits)
//   mx-f16 <num> <m> <m words>         FloatToF16(in, out, num)
//   mx-msum <xsize> <m> <factor> <m words> <m words>
//                                      MultiplySum(xsize, in, in_Y, factor, out)
//   mx-rgb <xsize> <m> <factor> <m words>
//                                      RgbFromSingle(xsize, in, factor, r, g, b)
//   mx-single <xsize> <m> <factor> <m words>
//                                      SingleFromSingle(xsize, in, factor, out)
//   mx-rct, mx-squeeze: InvRCT and InvSqueeze through the original's
//   HWY_DYNAMIC_DISPATCH on whole modular images (see the handler).
// `m` input words fill the input buffer (m >= RoundUpTo(num, 8), so whole
// vectors read defined words on every target); the output buffer holds m
// sentinel words (0xdeadbeef, 0xdead), and all m output words are printed.
#include <hwy/base.h>
#include <jxl/memory_manager.h>

#include "lib/jxl/modular/modular_image.h"
#include "lib/jxl/modular/transform/rct.h"
#include "lib/jxl/modular/transform/squeeze.h"
#include "lib/jxl/modular/transform/squeeze_params.h"
#include "lib/jxl/modular/transform/transform.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace jxl {
namespace ORACLE_HWY_NS {
void FloatToU32(const float* in, uint32_t* out, size_t num, float mul,
                size_t bits_per_sample);
void FloatToF16(const float* in, hwy::float16_t* out, size_t num);
void MultiplySum(size_t xsize, const int32_t* row_in, const int32_t* row_in_Y,
                 float factor, float* row_out);
void RgbFromSingle(size_t xsize, const int32_t* row_in, float factor,
                   float* out_r, float* out_g, float* out_b);
void SingleFromSingle(size_t xsize, const int32_t* row_in, float factor,
                      float* row_out);
}  // namespace ORACLE_HWY_NS
}  // namespace jxl

namespace {
// A buffer of `n` elements whose first element lies on a 128-byte boundary.
template <class T>
class MXAligned {
 public:
  explicit MXAligned(size_t n) : storage_(n + 128 / sizeof(T)) {
    const uintptr_t address = reinterpret_cast<uintptr_t>(storage_.data());
    data_ = storage_.data() + ((128 - address % 128) % 128) / sizeof(T);
  }
  T* data() { return data_; }

 private:
  std::vector<T> storage_;
  T* data_;
};

size_t MXCount(std::istringstream& in, size_t limit) {
  size_t n;
  in >> n;
  if (n > limit) throw std::runtime_error("mx: count");
  return n;
}

void MXWords(std::istringstream& in, uint32_t* words, size_t m) {
  for (size_t i = 0; i < m; ++i) in >> words[i];
}
}  // namespace

bool ModularSimdCommand(const std::string& command, std::istringstream& in,
                        JxlMemoryManager* mm) {
  if (command == "mx-u32") {
    size_t bits = MXCount(in, 31);
    size_t num = MXCount(in, 4096);
    size_t m = MXCount(in, 4096);
    if (m < num) throw std::runtime_error("mx-u32: m < num");
    MXAligned<uint32_t> input(m);
    MXWords(in, input.data(), m);
    MXAligned<uint32_t> output(m);
    for (size_t i = 0; i < m; ++i) output.data()[i] = 0xdeadbeef;
    jxl::ORACLE_HWY_NS::FloatToU32(reinterpret_cast<const float*>(input.data()),
                                   output.data(), num,
                                   static_cast<float>((uint64_t{1} << bits) - 1),
                                   bits);
    for (size_t i = 0; i < m; ++i) std::cout << (i ? " " : "") << output.data()[i];
    return true;
  }
  if (command == "mx-f16") {
    size_t num = MXCount(in, 4096);
    size_t m = MXCount(in, 4096);
    if (m < num) throw std::runtime_error("mx-f16: m < num");
    MXAligned<uint32_t> input(m);
    MXWords(in, input.data(), m);
    MXAligned<uint16_t> output(m);
    for (size_t i = 0; i < m; ++i) output.data()[i] = 0xdead;
    jxl::ORACLE_HWY_NS::FloatToF16(reinterpret_cast<const float*>(input.data()),
                                   reinterpret_cast<hwy::float16_t*>(output.data()),
                                   num);
    for (size_t i = 0; i < m; ++i) std::cout << (i ? " " : "") << output.data()[i];
    return true;
  }
  if (command == "mx-msum" || command == "mx-rgb" || command == "mx-single") {
    size_t xsize = MXCount(in, 4096);
    size_t m = MXCount(in, 4096);
    if (m < xsize) throw std::runtime_error("mx: m < xsize");
    uint32_t factor_bits;
    in >> factor_bits;
    float factor;
    std::memcpy(&factor, &factor_bits, sizeof(factor));
    const size_t inputs = command == "mx-msum" ? 2 : 1;
    const size_t outputs = command == "mx-rgb" ? 3 : 1;
    std::vector<MXAligned<uint32_t>> rows_in, rows_out;
    for (size_t i = 0; i < inputs; ++i) {
      rows_in.emplace_back(m);
      MXWords(in, rows_in.back().data(), m);
    }
    for (size_t i = 0; i < outputs; ++i) {
      rows_out.emplace_back(m);
      for (size_t x = 0; x < m; ++x) rows_out.back().data()[x] = 0xdeadbeef;
    }
    auto in_row = [&](size_t i) { return reinterpret_cast<const int32_t*>(rows_in[i].data()); };
    auto out_row = [&](size_t i) { return reinterpret_cast<float*>(rows_out[i].data()); };
    if (command == "mx-msum") {
      jxl::ORACLE_HWY_NS::MultiplySum(xsize, in_row(0), in_row(1), factor, out_row(0));
    } else if (command == "mx-rgb") {
      jxl::ORACLE_HWY_NS::RgbFromSingle(xsize, in_row(0), factor, out_row(0), out_row(1),
                                        out_row(2));
    } else {
      jxl::ORACLE_HWY_NS::SingleFromSingle(xsize, in_row(0), factor, out_row(0));
    }
    for (size_t i = 0; i < outputs; ++i)
      for (size_t x = 0; x < m; ++x) std::cout << (i || x ? " " : "") << rows_out[i].data()[x];
    return true;
  }
  if (command == "mx-rct" || command == "mx-squeeze") {
    // mx-rct <rct_type> <w> <h> <n> <n words>: three w x h channels filled
    // with words[(c * 7 + y * 13 + x) % n], then InvRCT(image, 0, rct_type).
    // mx-squeeze <mode> <w> <h> <n> <n words>: one w x h channel, squeeze
    // parameters by mode (0: default list, 1: one horizontal step, 2: one
    // vertical step, 3: horizontal then vertical), MetaSqueeze, every
    // channel filled as above, then InvSqueeze. Both print the status and
    // every channel's w, h and samples.
    const bool rct = command == "mx-rct";
    size_t kind = MXCount(in, 41);
    size_t w = MXCount(in, 4096);
    size_t h = MXCount(in, 4096);
    size_t n = MXCount(in, 4096);
    if (n == 0) throw std::runtime_error("mx: no words");
    std::vector<int32_t> words(n);
    for (int32_t& v : words) {
      uint32_t bits;
      in >> bits;
      v = static_cast<int32_t>(bits);
    }
    auto created = jxl::Image::Create(mm, w, h, 8, rct ? 3 : 1);
    if (!created.ok()) throw std::runtime_error("mx: image");
    jxl::Image image = std::move(created).value_();
    std::vector<jxl::SqueezeParams> params;
    jxl::Status status = true;
    if (!rct) {
      if (kind == 0) {
        jxl::DefaultSqueezeParameters(&params, image);
      } else {
        for (bool horizontal : {true, false}) {
          if ((kind == 1 && !horizontal) || (kind == 2 && horizontal)) continue;
          jxl::SqueezeParams p;
          p.horizontal = horizontal;
          p.in_place = true;
          p.begin_c = 0;
          p.num_c = 1;
          params.push_back(p);
        }
      }
      status = jxl::MetaSqueeze(image, &params);
    }
    if (status) {
      for (size_t c = 0; c < image.channel.size(); ++c) {
        jxl::Channel& ch = image.channel[c];
        for (size_t y = 0; y < ch.h; ++y)
          for (size_t x = 0; x < ch.w; ++x)
            ch.Row(y)[x] = words[(c * 7 + y * 13 + x) % n];
      }
      status = rct ? jxl::InvRCT(image, 0, kind, nullptr)
                   : jxl::InvSqueeze(image, params, nullptr);
    }
    std::cout << int(status.code()) << ' ' << image.channel.size();
    for (const jxl::Channel& ch : image.channel) {
      std::cout << ' ' << ch.w << ' ' << ch.h;
      for (size_t y = 0; y < ch.h; ++y)
        for (size_t x = 0; x < ch.w; ++x) std::cout << ' ' << static_cast<uint32_t>(ch.Row(y)[x]);
    }
    return true;
  }
  return false;
}
