// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe pristine scalar templates and strategy dispatch, including mutable
// coefficient/scratch storage. This adapter supplies no transform algorithm.
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_transforms-inl.h"
#include "aligned_vector.h"

namespace {
float ReadFloat(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
template <class V>
void Words(const V& values, bool first = false) {
  std::cout << (first ? "B" : " B");
  for (float v : values) { uint32_t bits; std::memcpy(&bits, &v, sizeof(bits)); std::cout << ' ' << bits; }
}
template <size_t R, size_t C>
void Scaled(bool inverse, float* in, size_t stride, float* out, float* scratch) {
  if (inverse) jxl::ORACLE_HWY_NS::ComputeScaledIDCT<R,C>()(in, jxl::ORACLE_HWY_NS::DCTTo(out, stride), scratch);
  else jxl::ORACLE_HWY_NS::ComputeScaledDCT<R,C>()(jxl::ORACLE_HWY_NS::DCTFrom(in, stride), out, scratch);
}
template <size_t R>
void Columns(size_t cols, bool inverse, float* in, size_t stride, float* out, float* scratch) {
#define COLUMN(C) case C: return Scaled<R,C>(inverse,in,stride,out,scratch)
  switch(cols) { COLUMN(1); COLUMN(2); COLUMN(4); COLUMN(8); COLUMN(16); COLUMN(32); COLUMN(64); COLUMN(128); COLUMN(256); }
#undef COLUMN
  throw std::runtime_error("DCT columns");
}
void Rows(size_t rows, size_t cols, bool inverse, float* in, size_t stride, float* out, float* scratch) {
#define ROW(R) case R: return Columns<R>(cols,inverse,in,stride,out,scratch)
  switch(rows) { ROW(1); ROW(2); ROW(4); ROW(8); ROW(16); ROW(32); ROW(64); ROW(128); ROW(256); }
#undef ROW
  throw std::runtime_error("DCT rows");
}
}
bool DctCommand(const std::string& command, std::istringstream& in) {
  if (command == "dct-tables") {
    std::vector<float> values{jxl::kSqrt2, jxl::kSqrt0_5};
    for (float v : jxl::DCTResampleScales<8,1>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<1,8>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<16,2>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<2,16>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<32,4>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<4,32>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<64,8>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<8,64>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<128,16>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<16,128>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<256,32>::kScales) values.push_back(v);
    for (float v : jxl::DCTResampleScales<32,256>::kScales) values.push_back(v);
    for (float v : jxl::WcMultipliers<4>::kMultipliers) values.push_back(v);
    for (float v : jxl::WcMultipliers<8>::kMultipliers) values.push_back(v);
    for (float v : jxl::WcMultipliers<16>::kMultipliers) values.push_back(v);
    for (float v : jxl::WcMultipliers<32>::kMultipliers) values.push_back(v);
    for (float v : jxl::WcMultipliers<64>::kMultipliers) values.push_back(v);
    for (float v : jxl::WcMultipliers<128>::kMultipliers) values.push_back(v);
    for (float v : jxl::WcMultipliers<256>::kMultipliers) values.push_back(v);
    Words(values, true); return true;
  }
  if (command == "dct-strategy") {
    unsigned raw; in >> raw;
    auto acs = jxl::AcStrategy::FromRawStrategy(static_cast<uint8_t>(raw));
    std::cout << unsigned(acs.RawStrategy()) << ' ' << unsigned(acs.Strategy()) << ' ' << acs.IsFirstBlock() << ' ' << acs.IsMultiblock()
              << ' ' << acs.covered_blocks_x() << ' ' << acs.covered_blocks_y() << ' ' << acs.log2_covered_blocks()
              << ' ' << jxl::AcStrategy::TypeBit(acs.Strategy());
    size_t size = acs.covered_blocks_x() * acs.covered_blocks_y() * 64;
    std::vector<jxl::coeff_order_t> order(size + 3, 0x31415926), lut(size + 3, 0x31415926);
    acs.ComputeNaturalCoeffOrder(order.data()); acs.ComputeNaturalCoeffOrderLut(lut.data());
    for(auto v : order) std::cout << ' ' << v;
    for(auto v : lut) std::cout << ' ' << v;
    return true;
  }
  if (command != "dct-scaled" && command != "dct-pixels" && command != "dct-llf") return false;
  size_t rows, cols, stride; unsigned option; in >> rows >> cols >> stride >> option;
  bool inverse = command != "dct-scaled" || option != 0;
  size_t count = command == "dct-llf" ? rows * stride : inverse ? rows * cols : rows * stride;
  // Owners are separate and retain a tail canary. All observed storage starts
  // initialized; the source's local uninitialized AFV/warm arrays are omitted.
  // vardct kernels: coefficients, pixels and scratch are aligned as
  // libjxl's decoder allocates them (dec_group_block, the pipeline rows and
  // scratch_space); the SIMD transforms Load/Store the scratch aligned.
  OracleAlignedVector<float> input(count + 4, -123.25f);
  for (size_t i = 0; i < count; ++i) input[i] = ReadFloat(in);
  OracleAlignedVector<float> output((inverse ? rows * stride : rows * cols) + 4, -123.25f);
  if(command == "dct-llf") output.resize(rows * cols * 64 + 4, -123.25f);
  // dct-llf: rows/cols count blocks; LowestFrequenciesFromDC's caller hands
  // it the group cache's scratch_space of 4 * max_block_area floats
  // (dec_cache.cc), which the 8-lane transform temporaries need.
  size_t scratch_size = command == "dct-llf" ? rows * cols * 64 * 4 + 4 : rows * cols * 4 + 3 * (rows + cols) + 4;
  OracleAlignedVector<float> scratch(scratch_size, -123.25f);
  if(command == "dct-scaled") Rows(rows, cols, inverse, input.data(), stride, output.data(), scratch.data());
  else if(command == "dct-pixels") jxl::ORACLE_HWY_NS::TransformToPixels(static_cast<jxl::AcStrategyType>(option), input.data(), output.data(), stride, scratch.data());
  else jxl::ORACLE_HWY_NS::LowestFrequenciesFromDC(static_cast<jxl::AcStrategyType>(option), input.data(), stride, output.data(), scratch.data());
  Words(input, true); Words(output); Words(scratch); return true;
}
