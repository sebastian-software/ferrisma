/* Copyright (c) the JPEG XL Project Authors. All rights reserved.
 * Use of this source code is governed by a BSD-style license in LICENSE. */
#include <brotli/decode.h>

int BrotliInstanceSetParameter(BrotliDecoderState* state, uint32_t parameter,
    uint32_t value) {
  return BrotliDecoderSetParameter(state, (BrotliDecoderParameter)parameter, value);
}
