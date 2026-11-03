/* Copyright 2009, 2010, 2013-2016 by the Brotli Authors.
   Distributed under the MIT license in licenses/BROTLI-MIT.txt. */
#include <brotli/decode.h>

/* The public API is C: unknown integer parameters are defined switch defaults.
   Keep this conversion in C rather than constructing an out-of-range C++ enum. */
int BrotliObserveSetParameter(BrotliDecoderState* state, uint32_t parameter,
                            uint32_t value) {
  return BrotliDecoderSetParameter(state, (BrotliDecoderParameter)parameter, value);
}
