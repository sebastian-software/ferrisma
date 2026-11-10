// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Only original exported Brotli dictionary/transform routines supply results.
#include <array>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "common/dictionary.h"
#include "common/transform.h"

static void U32(uint32_t value) {
  const uint8_t bytes[4] = {static_cast<uint8_t>(value),
      static_cast<uint8_t>(value >> 8), static_cast<uint8_t>(value >> 16),
      static_cast<uint8_t>(value >> 24)};
  if (std::fwrite(bytes, 1, 4, stdout) != 4) std::abort();
}

template <size_t N>
static void Record(int len, const std::array<uint8_t, N>& dst) {
  U32(static_cast<uint32_t>(len));
  if (std::fwrite(dst.data(), 1, dst.size(), stdout) != dst.size()) std::abort();
}

template <size_t N = 64>
static std::array<uint8_t, N> Destination(unsigned seed) {
  std::array<uint8_t, N> dst;
  for (unsigned i = 0; i < dst.size(); ++i) dst[i] = static_cast<uint8_t>(seed + i * 37);
  return dst;
}

static std::vector<uint8_t> Unhex(const std::string& text) {
  if (text == "-") return {};
  if (text.size() % 2) std::abort();
  std::vector<uint8_t> bytes;
  for (size_t i = 0; i < text.size(); i += 2) {
    bytes.push_back(static_cast<uint8_t>(std::stoul(text.substr(i, 2), nullptr, 16)));
  }
  return bytes;
}

static void Word(int length, unsigned index, const BrotliDictionary* dictionary,
    const BrotliTransforms* transforms) {
  const auto* word = dictionary->data + dictionary->offsets_by_length[length] + index * length;
  for (unsigned transform = 0; transform < transforms->num_transforms; ++transform) {
    auto dst = Destination(length * 11u + index * 3u + transform * 17u);
    const int len = BrotliTransformDictionaryWord(dst.data(), word, length, transforms, transform);
    Record(len, dst);
  }
}

template <size_t N = 64>
static void Custom(unsigned type, unsigned parameter, int len, unsigned seed,
    const std::vector<uint8_t>& prefix, const std::vector<uint8_t>& suffix,
    std::vector<uint8_t> word) {
  word.resize(word.size() + 16, 0xD3);
  std::vector<uint8_t> prefix_suffix;
  prefix_suffix.push_back(static_cast<uint8_t>(prefix.size()));
  prefix_suffix.insert(prefix_suffix.end(), prefix.begin(), prefix.end());
  prefix_suffix.push_back(static_cast<uint8_t>(suffix.size()));
  prefix_suffix.insert(prefix_suffix.end(), suffix.begin(), suffix.end());
  prefix_suffix.push_back(0);
  const uint16_t map[2] = {0, static_cast<uint16_t>(prefix.size() + 1)};
  const uint8_t triplet[3] = {0, static_cast<uint8_t>(type), 1};
  const uint8_t params[2] = {static_cast<uint8_t>(parameter), static_cast<uint8_t>(parameter >> 8)};
  BrotliTransforms custom{};
  custom.prefix_suffix_size = static_cast<uint16_t>(prefix_suffix.size());
  custom.prefix_suffix = prefix_suffix.data();
  custom.prefix_suffix_map = map;
  custom.num_transforms = 1;
  custom.transforms = triplet;
  custom.params = (type == 21 || type == 22) ? params : nullptr;
  for (auto& cutoff : custom.cutOffTransforms) cutoff = -1;
  auto dst = Destination<N>(seed);
  Record(BrotliTransformDictionaryWord(dst.data(), word.data(), len, &custom, 0), dst);
}

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  static char output_buffer[1 << 20];
  std::setvbuf(stdout, output_buffer, _IOFBF, sizeof(output_buffer));
  const auto* dictionary = BrotliGetDictionary();
  const auto* transforms = BrotliGetTransforms();
  const std::string mode = argv[1];
  if (mode == "builtin") {
    for (int length = 4; length <= 24; ++length) {
      for (unsigned index = 0; index < (1u << dictionary->size_bits_by_length[length]); ++index) {
        Word(length, index, dictionary, transforms);
      }
    }
  } else if (mode == "words") {
    int length;
    unsigned index;
    while (std::cin >> length >> index) Word(length, index, dictionary, transforms);
  } else if (mode == "metadata") {
    const std::array<uint8_t, 16> alternate{};
    BrotliSetDictionaryData(nullptr);
    BrotliSetDictionaryData(alternate.data());
    const auto* after = BrotliGetDictionary();
    U32(dictionary == after);
    U32(dictionary->data == after->data);
    for (const auto bits : dictionary->size_bits_by_length) U32(bits);
    for (const auto offset : dictionary->offsets_by_length) U32(offset);
    U32(static_cast<uint32_t>(dictionary->data_size));
    std::fwrite(dictionary->data, 1, dictionary->data_size, stdout);
    U32(transforms->prefix_suffix_size);
    U32(transforms->num_transforms);
    U32(transforms->params == nullptr);
    for (const auto cutoff : transforms->cutOffTransforms) U32(static_cast<uint32_t>(cutoff));
    std::fwrite(transforms->prefix_suffix, 1, transforms->prefix_suffix_size, stdout);
    for (unsigned i = 0; i < 50; ++i) U32(transforms->prefix_suffix_map[i]);
    std::fwrite(transforms->transforms, 1, transforms->num_transforms * 3, stdout);
  } else if (mode == "custom-corpus") {
    for (unsigned type = 0; type <= 255; ++type) {
      for (const int len : {-3, -1, 0, 1, 2, 3, 4, 5, 8, 12}) {
        const std::vector<uint8_t> prefix = type % 2 == 0 ? std::vector<uint8_t>{'p','r','e'} : std::vector<uint8_t>{};
        const std::vector<uint8_t> suffix = type % 3 == 0 ? std::vector<uint8_t>{'s','u','f','f','i','x'} : std::vector<uint8_t>{};
        Custom(type, 0x9234, len, type, prefix, suffix, {'a','B','c','D','x','y','Z','1','2','3','4','5'});
      }
    }
    for (unsigned first = 0; first <= 255; ++first) {
      for (int len = 0; len <= 8; ++len) {
        for (const unsigned type : {10, 11, 21, 22}) {
          const unsigned parameters[] = {0, 1, 0x7FFF, 0x8000, 0xFFFF};
          for (unsigned i = 0; i < (type >= 21 ? 5u : 1u); ++i) {
            Custom(type, parameters[i], len, first, {'<'}, {'>'}, {static_cast<uint8_t>(first),0x81,0xB2,0xC3,0xE5,0xA9,'z','q'});
          }
        }
      }
    }
    const std::vector<std::vector<uint8_t>> scalar_words = {{'a'}, {0xC3,0xA9}, {0xE2,0x82,0xAC}, {0xF0,0x90,0x8D,0x88}};
    for (unsigned parameter = 0; parameter <= 65535; ++parameter) {
      for (const auto& word : scalar_words) {
        for (const unsigned type : {21, 22}) {
          Custom(type, parameter, static_cast<int>(word.size()), parameter, {}, {}, word);
        }
      }
    }
  } else if (mode == "wide") {
    const std::vector<uint8_t> word = {'a','B','c','D','x','y','Z','1','2','3','4','5','q','R','s','T','u','V','w','X','y','Z','0','1'};
    for (unsigned prefix_len : {0, 1, 127, 128, 254, 255}) {
      for (unsigned suffix_len : {0, 1, 127, 128, 254, 255}) {
        const std::vector<uint8_t> prefix(prefix_len, 'p');
        const std::vector<uint8_t> suffix(suffix_len, 's');
        for (unsigned type = 0; type <= 23; ++type) {
          for (int len : {-3, 0, 1, 4, 24}) {
            Custom<1024>(type, 0x8000, len, prefix_len + suffix_len * 3 + type * 7, prefix, suffix, word);
          }
        }
      }
    }
  } else if (mode == "custom") {
    std::string line;
    while (std::getline(std::cin, line)) {
      std::istringstream in(line);
      unsigned type, parameter, seed;
      int len;
      std::string prefix_hex, suffix_hex, word_hex;
      if (!(in >> type >> parameter >> len >> seed >> prefix_hex >> suffix_hex >> word_hex)) return 2;
      const auto prefix = Unhex(prefix_hex);
      const auto suffix = Unhex(suffix_hex);
      auto word = Unhex(word_hex);
      Custom(type, parameter, len, seed, prefix, suffix, word);
    }
  } else {
    return 2;
  }
  return 0;
}
