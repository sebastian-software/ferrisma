/* Copyright 2013 Google Inc. All Rights Reserved.
 * Distributed under the MIT license in licenses/BROTLI-MIT.txt. */
#include <brotli/decode.h>
/* Use the source C enum conversion domain, not an out-of-range C++ enum cast. */
int BrotliStreamAPIAttach(BrotliDecoderState* state, uint32_t type,
    size_t size, const uint8_t* data) {
  return BrotliDecoderAttachDictionary(state, (BrotliSharedDictionaryType)type,
      size, data);
}
