// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// This observer calls the unmodified pinned Brotli implementation.
#include <array>
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "common/constants.h"
#include "dec/huffman.h"

template <typename T> T Read(std::istringstream& in) {
  uint64_t value;
  if (!(in >> value)) std::abort();
  return static_cast<T>(value);
}

static void PrintTable(const std::vector<HuffmanCode>& table) {
  for (const auto& code : table) {
    // Only initialized fields are observed; never inspect struct padding.
    std::cout << ' ' << unsigned(code.bits) << ' ' << code.value;
  }
}

int main() {
  std::string line;
  while (std::getline(std::cin, line)) {
    std::istringstream in(line);
    std::string kind;
    in >> kind;
    if (kind == "simple") {
      const int root = Read<int>(in);
      const uint32_t num = Read<uint32_t>(in);
      std::array<uint16_t, 4> val;
      for (auto& v : val) v = Read<uint16_t>(in);
      std::vector<HuffmanCode> table((size_t{1} << root) + 8,
          ConstructHuffmanCode(165, 48879));
      const auto size = BrotliBuildSimpleHuffmanTable(table.data(), root, val.data(), num);
      std::cout << size;
      for (const auto v : val) std::cout << ' ' << v;
      PrintTable(table);
    } else if (kind == "lengths" || kind == "general" || kind == "general-accessor") {
      const bool is_general = kind == "general" || kind == "general-accessor";
      const int root = is_general ? Read<int>(in) : 5;
      const size_t n = is_general ? Read<size_t>(in) : 18;
      std::vector<uint8_t> lengths(n);
      std::array<uint16_t, 16> count{};
      std::vector<uint16_t> links(n + 16, 65535);
      std::array<int, 16> next;
      for (int i = 0; i < 16; ++i) next[i] = i - 16;
      for (size_t i = 0; i < n; ++i) {
        const auto len = Read<uint8_t>(in);
        lengths[i] = len;
        if (len) {
          ++count[len];
          links[16 + next[len]] = static_cast<uint16_t>(i);
          next[len] = static_cast<int>(i);
        }
      }
      count[0] = 4660;  // Unused count slot must remain untouched.
      const size_t capacity = is_general ? n + 376 + 8 : 40;
      std::vector<HuffmanCode> table(capacity, ConstructHuffmanCode(165, 48879));
      uint32_t size = 32;
      if (is_general) {
        size = BrotliBuildHuffmanTable(table.data(), root, links.data() + 16, count.data());
      } else {
        BrotliBuildCodeLengthsHuffmanTable(table.data(), lengths.data(), count.data());
      }
      std::cout << size;
      for (const auto v : count) std::cout << ' ' << v;
      for (const auto v : links) std::cout << ' ' << v;
      PrintTable(table);
    } else if (kind == "distance") {
      const auto distance = Read<uint32_t>(in);
      const auto postfix = Read<uint32_t>(in);
      const auto direct = Read<uint32_t>(in);
      const auto limit = BrotliCalculateDistanceCodeLimit(distance, postfix, direct);
      std::cout << limit.max_alphabet_size << ' ' << limit.max_distance;
    } else if (kind == "alphabet") {
      const uint32_t postfix = Read<uint32_t>(in);
      const uint32_t direct = Read<uint32_t>(in);
      const uint32_t bits = Read<uint32_t>(in);
      std::cout << BROTLI_DISTANCE_ALPHABET_SIZE(postfix, direct, bits);
    } else if (kind == "constants") {
      std::cout << sizeof(HuffmanCode) << ' ' << alignof(HuffmanCode);
#if defined(BROTLI_RBIT)
      std::cout << " 1";
#else
      std::cout << " 0";
#endif
      for (const auto range : _kBrotliPrefixCodeRanges) {
        std::cout << ' ' << range.offset << ' ' << unsigned(range.nbits);
      }
      for (unsigned p = 0; p <= 3; ++p) {
        for (unsigned d = 0; d <= (15u << p); d += 1u << p) {
          std::cout << ' ' << BROTLI_DISTANCE_ALPHABET_SIZE(p, d, 24u);
          std::cout << ' ' << BROTLI_DISTANCE_ALPHABET_SIZE(p, d, 62u);
        }
      }
      for (unsigned w = 10; w <= 30; ++w) {
        std::cout << ' ' << BROTLI_MAX_BACKWARD_LIMIT(w);
      }
    } else {
      return 2;
    }
    std::cout << '\n';
  }
}
