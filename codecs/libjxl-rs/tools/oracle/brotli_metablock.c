/* Copyright (c) the JPEG XL Project Authors. All rights reserved.
 * Use of this source code is governed by a BSD-style license in LICENSE. */
/* The private helper target includes the pristine source once, with no
 * algorithm/visibility/configuration edits. A second public-only target
 * instead binds the untouched archive; real complete streams compare them. */
#if defined(ORACLE_DEFAULT_ARCHIVE)
#include <brotli/decode.h>
#else
#include "c/dec/decode.c"
#endif
#include <brotli/encode.h>
#include <inttypes.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  char line[65536];
  while (fgets(line, sizeof(line), stdin)) {
    unsigned op, seed, state, bits, large;
    uint64_t accumulator;
    size_t chunk, i, size;
    char hex[32768];
    uint8_t input[16384];
    if (sscanf(line, "%u %u %u %" SCNu64 " %u %u %zu %32767s",
        &op, &seed, &state, &accumulator, &bits, &large, &chunk, hex) != 8) return 2;
    size = strcmp(hex, "-") == 0 ? 0 : strlen(hex) / 2;
    if (strcmp(hex, "-") != 0 && (strlen(hex) & 1)) return 2;
    for (i = 0; i < size; ++i) {
      char pair[3] = {hex[2*i], hex[2*i+1], 0};
      input[i] = (uint8_t)strtoul(pair, NULL, 16);
    }
    if (op == 4) {
      uint8_t encoded[131072];
      size_t capacity = sizeof(encoded);
      BROTLI_BOOL ok = BrotliEncoderCompress((int)seed, (int)state,
          BROTLI_MODE_GENERIC, size, input, &capacity, encoded);
      printf("%d %zu ", ok, capacity);
      if (ok) for (i = 0; i < capacity; ++i) printf("%02x", encoded[i]);
      putchar('\n');
      continue;
    }
    if (op == 5) {
      uint8_t encoded[131072];
      size_t available_in = size, available_out = sizeof(encoded);
      const uint8_t* next_in = input;
      uint8_t* next_out = encoded;
      BrotliEncoderOperation operation = BROTLI_OPERATION_EMIT_METADATA;
      BrotliEncoderState* encoder = BrotliEncoderCreateInstance(NULL, NULL, NULL);
      BROTLI_BOOL ok;
      if (!encoder) return 4;
      ok = BrotliEncoderSetParameter(encoder, BROTLI_PARAM_QUALITY, seed) &&
          BrotliEncoderSetParameter(encoder, BROTLI_PARAM_LGWIN, state);
      while (ok && !BrotliEncoderIsFinished(encoder)) {
        ok = BrotliEncoderCompressStream(encoder, operation, &available_in,
            &next_in, &available_out, &next_out, NULL);
        if (available_out == 0) return 4;
        if (available_in == 0 && !BrotliEncoderHasMoreOutput(encoder))
          operation = BROTLI_OPERATION_FINISH;
      }
      printf("%d %zu ", ok, (size_t)(next_out - encoded));
      if (ok) for (i = 0; i < (size_t)(next_out - encoded); ++i) printf("%02x", encoded[i]);
      BrotliEncoderDestroyInstance(encoder);
      putchar('\n');
      continue;
    }
    if (op == 3) {
      uint8_t output[131072];
      size_t capacity = sizeof(output);
      BrotliDecoderResult result = BrotliDecoderDecompress(size, input, &capacity, output);
      printf("%u %zu", (unsigned)result, capacity);
      if (result == BROTLI_DECODER_RESULT_SUCCESS) {
        putchar(' ');
        for (i = 0; i < capacity; ++i) printf("%02x", output[i]);
      }
      putchar('\n');
      continue;
    }
#if defined(ORACLE_DEFAULT_ARCHIVE)
    return 2;
#else
    {
      BrotliDecoderState s;
      brotli_reg_t value = 0xdeadbeef;
      size_t offset = 0;
      memset(&s, (int)seed, sizeof(s));
      BrotliInitBitReader(&s.br);
      s.br.val_ = (brotli_reg_t)accumulator;
      s.br.bit_pos_ = bits;
      s.large_window = large;
      s.substate_decode_uint8 = (BrotliRunningDecodeUint8State)state;
      s.substate_metablock_header = (BrotliRunningMetablockHeaderState)state;
      do {
        BrotliDecoderErrorCode result;
        size_t extent = size - offset;
        if (chunk && extent > chunk) extent = chunk;
        BrotliBitReaderSetInput(&s.br, input + offset, extent);
        if (op == 0) result = DecodeWindowBits(&s, &s.br);
        else if (op == 1) result = DecodeVarLenUint8(&s, &s.br, &value);
        else if (op == 2) result = DecodeMetaBlockLength(&s, &s.br);
        else return 2;
        printf("%d:%" PRIu64 ":%" PRIu64 ":%" PRIu64 ":%zu:%u:%u:%d:%d:",
            (int)result, (uint64_t)value, (uint64_t)s.br.val_,
            (uint64_t)s.br.bit_pos_, (size_t)(s.br.next_in - input),
            (unsigned)s.substate_decode_uint8, (unsigned)s.substate_metablock_header,
            s.loop_counter, s.meta_block_remaining_len);
        /* The full word is allocator-seeded here. Copy actual bytes instead
         * of making any assertion about indeterminate source padding. */
        {
          const uint8_t* word = (const uint8_t*)&s +
              offsetof(BrotliDecoderState, new_ringbuffer_size) +
              sizeof(s.new_ringbuffer_size);
          for (i = 0; i < 4; ++i) printf("%02x", word[i]);
        }
        putchar(' ');
        offset += extent;
        if (result != BROTLI_DECODER_NEEDS_MORE_INPUT) break;
        if (s.br.next_in != s.br.last_in) return 3;
      } while (offset < size);
      putchar('\n');
    }
#endif
  }
  return ferror(stdin) ? 3 : 0;
}
