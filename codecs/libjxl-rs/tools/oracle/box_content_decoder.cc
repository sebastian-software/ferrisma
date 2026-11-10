// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe the actual compiled pinned class and Brotli APIs. No codec body is
// copied or recreated here; private access is limited to this observer TU.
#include "lib/jxl/box_content_decoder.h"
#include <brotli/encode.h>
#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

static std::vector<uint8_t> Bytes(const std::string& h) {
  std::vector<uint8_t> out;
  if (h == "-") return out;
  if (h.size() % 2) std::abort();
  for (size_t i = 0; i < h.size(); i += 2)
    out.push_back(static_cast<uint8_t>(std::stoul(h.substr(i, 2), nullptr, 16)));
  return out;
}
static std::string Hex(const uint8_t* p, size_t n) {
  if (!n) return "-";
  static const char* digits = "0123456789abcdef";
  std::string out;
  for (size_t i = 0; i < n; ++i) {
    out += digits[p[i] >> 4]; out += digits[p[i] & 15];
  }
  return out;
}
static void State(const jxl::JxlBoxContentDecoder& s) {
  // Only called after actual StartBox writes every scalar. The decoder pointer
  // is observed as presence, never an invented stable numeric address.
  std::cout << " S" << s.header_done_ << ':' << s.brob_decode_ << ':'
      << s.box_until_eof_ << ':' << s.remaining_ << ':' << s.pos_ << ':'
      << (s.brotli_dec != nullptr);
  if (s.brotli_dec) {
    std::cout << ':' << BrotliDecoderGetErrorCode(s.brotli_dec) << ':'
        << BrotliDecoderIsUsed(s.brotli_dec) << ':'
        << BrotliDecoderIsFinished(s.brotli_dec) << ':'
        << BrotliDecoderHasMoreOutput(s.brotli_dec);
  }
}
int main() {
  std::string line;
  while (std::getline(std::cin, line)) {
    std::istringstream in(line); std::string kind;
    if (!(in >> kind)) std::abort();
    if (kind == "encode") {
      int quality, window, mode; std::string h;
      if (!(in >> quality >> window >> mode >> h)) std::abort();
      auto data = Bytes(h);
      std::vector<uint8_t> output(BrotliEncoderMaxCompressedSize(data.size()));
      size_t n = output.size();
      const int ok = BrotliEncoderCompress(quality, window,
          static_cast<BrotliEncoderMode>(mode), data.size(), data.data(), &n, output.data());
      std::cout << ok << ' ' << Hex(output.data(), n) << '\n';
      continue;
    }
    if (kind != "box") std::abort();
    std::string h; size_t count;
    if (!(in >> h >> count)) std::abort();
    auto data = Bytes(h);
    // Both input and output allocations have live non-NULL backing at zero
    // extent, so a reached original memcpy(0) keeps its actual caller domain.
    const size_t input_size = data.size();
    if (data.empty()) data.push_back(0xcc);
    jxl::JxlBoxContentDecoder decoder;
    bool started = false;
    for (size_t i = 0; i < count; ++i) {
      char action; if (!(in >> action)) std::abort();
      if (action == 'S') {
        bool brob, eof; size_t n;
        if (!(in >> brob >> eof >> n)) std::abort();
        decoder.StartBox(brob, eof, n); started = true;
        std::cout << " B"; State(decoder);
      } else if (action == 'P') {
        size_t base, avail, pos, out_size;
        if (!(in >> base >> avail >> pos >> out_size) || !started ||
            base > input_size || avail > input_size - base) std::abort();
        std::vector<uint8_t> output(std::max<size_t>(out_size, 1), 0xcc);
        uint8_t* next_out = output.data(); size_t available_out = out_size;
        const auto status = decoder.Process(data.data() + base, avail, pos,
                                            &next_out, &available_out);
        const size_t produced = static_cast<size_t>(next_out - output.data());
        std::cout << " R" << static_cast<unsigned>(status) << ':' << available_out
            << ':' << produced << ':' << Hex(output.data(), produced) << ':'
            << Hex(output.data() + produced, output.size() - produced);
        State(decoder);
      } else std::abort();
    }
    std::string extra; if (in >> extra) std::abort();
    std::cout << '\n';
  }
  return std::cin.bad() || std::cout.bad();
}
