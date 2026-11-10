// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// This observer calls only the actual archived, pinned Brotli transforms.
// Unknown malloc tails are never printed or loaded by this adapter.

#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include "common/dictionary.h"
#include "common/transform.h"

static constexpr size_t kDestinationSize = 64;

static void Write(const void* data, size_t size) {
  if (std::fwrite(data, 1, size, stdout) != size) std::abort();
}

static void Length(int value) {
  static_assert(sizeof(int) == 4, "The protocol observes a 32-bit source int");
  const auto bits = static_cast<uint32_t>(value);
  const uint8_t encoded[4] = {static_cast<uint8_t>(bits),
      static_cast<uint8_t>(bits >> 8), static_cast<uint8_t>(bits >> 16),
      static_cast<uint8_t>(bits >> 24)};
  Write(encoded, sizeof(encoded));
}

// Check fixture backing preconditions, without supplying a transform routine.
// The source dictionary/transform metadata remains the genuine immutable owner.
static void CheckBuiltinDomain(const BrotliDictionary* dictionary,
    const BrotliTransforms* transforms) {
  if (!dictionary || !dictionary->data || dictionary->data_size != 122784 ||
      !transforms || transforms->num_transforms != 121 || transforms->params) {
    std::abort();
  }
  size_t words = 0;
  for (int length = 4; length <= 24; ++length) {
    const auto bits = dictionary->size_bits_by_length[length];
    if (bits >= 32) std::abort();
    const size_t count = size_t{1} << bits;
    words += count;
    const size_t start = dictionary->offsets_by_length[length];
    if (start > dictionary->data_size ||
        count * length > dictionary->data_size - start) std::abort();
    for (unsigned transform = 0; transform < transforms->num_transforms; ++transform) {
      const auto* prefix = BROTLI_TRANSFORM_PREFIX(transforms, transform);
      const auto* suffix = BROTLI_TRANSFORM_SUFFIX(transforms, transform);
      const auto type = BROTLI_TRANSFORM_TYPE(transforms, transform);
      if (type > BROTLI_TRANSFORM_OMIT_FIRST_9) std::abort();
      // Two spare bytes cover every simplified-uppercase XOR, even for an
      // incomplete final UTF8 sequence. Omission only reduces the copied word.
      const size_t extent = size_t{prefix[0]} + length + suffix[0] + 2;
      if (extent > kDestinationSize) std::abort();
      if (type >= BROTLI_TRANSFORM_OMIT_FIRST_1) {
        const size_t skip = type - (BROTLI_TRANSFORM_OMIT_FIRST_1 - 1);
        // The source advances its word pointer even if the copy loop is empty.
        const size_t last_word_start = start + (count - 1) * length;
        if (skip > dictionary->data_size - last_word_start) std::abort();
      }
    }
  }
  if (words != 13504) std::abort();
}

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  const bool seeded = std::strcmp(argv[1], "seeded") == 0;
  const bool unknown = std::strcmp(argv[1], "unknown") == 0;
  if (!seeded && !unknown) return 2;
  static char output_buffer[1 << 20];
  if (std::setvbuf(stdout, output_buffer, _IOFBF, sizeof(output_buffer))) return 2;
  const auto* dictionary = BrotliGetDictionary();
  const auto* transforms = BrotliGetTransforms();
  CheckBuiltinDomain(dictionary, transforms);
  for (int length = 4; length <= 24; ++length) {
    for (unsigned index = 0; index < (1u << dictionary->size_bits_by_length[length]); ++index) {
      const auto* word = dictionary->data + dictionary->offsets_by_length[length] + index * length;
      for (unsigned transform = 0; transform < transforms->num_transforms; ++transform) {
        // Each unknown case starts with a new malloc lifetime. No observer
        // initialization, whole-buffer copy, or tail-byte read occurs here.
        auto* dst = static_cast<uint8_t*>(std::malloc(kDestinationSize));
        if (!dst) return 3;
        if (seeded) {
          const unsigned seed = length * 11u + index * 3u + transform * 17u;
          for (unsigned i = 0; i < kDestinationSize; ++i) {
            dst[i] = static_cast<uint8_t>(seed + i * 37);
          }
        }
        const int result = BrotliTransformDictionaryWord(dst, word, length, transforms, transform);
        if (result < 0 || static_cast<size_t>(result) > kDestinationSize) std::abort();
        Length(result);
        // The original writes its complete returned prefix. Unknown bytes
        // beyond that prefix are neither printed nor converted to scalars.
        Write(dst, seeded ? kDestinationSize : static_cast<size_t>(result));
        std::free(dst);
      }
    }
  }
  if (std::fflush(stdout)) return 4;
  return 0;
}
