// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Diagnostic only: observe primitive instructions and the pristine helper.
#include <array>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include "lib/jxl/cms/tone_mapping-inl.h"

float FromBits(uint32_t bits) { float value; std::memcpy(&value, &bits, sizeof(value)); return value; }
uint32_t Bits(float value) { uint32_t bits; std::memcpy(&bits, &value, sizeof(bits)); return bits; }
extern "C" __attribute__((noinline)) float AddProbe(float first, float second) { return first + second; }
extern "C" __attribute__((noinline)) float MulProbe(float first, float second) { return first * second; }
extern "C" __attribute__((noinline)) void OriginalGamutProbe(float* rgb, const float* primaries, float preserve) {
  const hwy::N_SCALAR::ScalableTag<float> d;
  auto r = hwy::N_SCALAR::Set(d, rgb[0]);
  auto g = hwy::N_SCALAR::Set(d, rgb[1]);
  auto b = hwy::N_SCALAR::Set(d, rgb[2]);
  jxl::Vector3 p{primaries[0], primaries[1], primaries[2]};
  jxl::N_SCALAR::GamutMap(&r, &g, &b, p, preserve);
  rgb[0] = hwy::N_SCALAR::GetLane(r);
  rgb[1] = hwy::N_SCALAR::GetLane(g);
  rgb[2] = hwy::N_SCALAR::GetLane(b);
}
int main(int argc, char** argv) {
  if (argc != 8) return 2;
  std::array<uint32_t, 7> input;
  for (size_t i = 0; i < input.size(); ++i) input[i] = std::strtoul(argv[i + 1], nullptr, 10);
  const std::array<uint32_t, 4> pairs{input[1], input[1] | 0x00400000u, input[2], input[2] ^ 0x80000000u};
  for (const uint32_t first : pairs) for (const uint32_t second : pairs) {
    std::cout << "add " << first << ' ' << second << ' ' << Bits(AddProbe(FromBits(first), FromBits(second))) << '\n';
    std::cout << "mul " << first << ' ' << second << ' ' << Bits(MulProbe(FromBits(first), FromBits(second))) << '\n';
  }
  float rgb[3]{FromBits(input[0]), FromBits(input[1]), FromBits(input[2])};
  const float p[3]{FromBits(input[3]), FromBits(input[4]), FromBits(input[5])};
  for (size_t i = 0; i < 3; ++i) std::cout << "product " << i << ' ' << Bits(MulProbe(p[i], rgb[i])) << '\n';
  OriginalGamutProbe(rgb, p, FromBits(input[6]));
  std::cout << "gamut " << Bits(rgb[0]) << ' ' << Bits(rgb[1]) << ' ' << Bits(rgb[2]) << '\n';
}
