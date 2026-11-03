// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The scoped observer includes the pristine source, without rewriting any body.
// BROTLI_TEST adds visibility bridges only. The default target links the
// untouched archive without that definition for complete-stream comparison.
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#if defined(SYMBOLS_DEFAULT)
#include "brotli/decode.h"
#else
#include "dec/decode.c"
#if defined(BROTLI_HUFFMAN_CODE_FAST_LOAD)
#error This observer requires the native separate-field representation
#endif

static HuffmanCode tables[35][33200];
static uint32_t sizes[35];

static void InitTables(void) {
  unsigned id;
  for (id = 0; id < 35; ++id) {
    uint16_t values[4] = {65535, 0, 257, 17};
    unsigned i;
    for (i = 0; i < 33200; ++i) tables[id][i] = ConstructHuffmanCode(165, 48879);
    if (id < 5) {
      sizes[id] = BrotliBuildSimpleHuffmanTable(tables[id], 8, values, id);
    } else {
      uint16_t links[32768 + 16];
      uint16_t count[16] = {0};
      int next[16];
      unsigned depth = id < 20 ? id - 4 : (id == 34 ? 1 : id - 18);
      unsigned n = id < 20 ? 1u << depth : (id == 34 ? 4 : depth + 1);
      for (i = 0; i < 16; ++i) next[i] = (int)i - 16;
      for (i = 0; i < n + 16; ++i) links[i] = 65535;
      for (i = 0; i < n; ++i) {
        unsigned len = id == 34 ? (i & 1) : (id < 20 ? depth : (i < depth - 1 ? i + 1 : depth));
        if (!len) continue;
        ++count[len];
        links[16 + next[len]] = (uint16_t)i;
        next[len] = (int)i;
      }
      sizes[id] = BrotliBuildHuffmanTable(tables[id], 8, links + 16, count);
    }
  }
}

static void U64(uint64_t value) {
  unsigned i;
  for (i = 0; i < 8; ++i) putchar((int)((value >> (8 * i)) & 255));
}

static void Run(void) {
  unsigned op, id, available, input_len;
  uint64_t val;
  int pos, limit, safe;
  while (scanf("%u %u %u %" SCNu64 " %u %d %d %d", &op, &id,
      &available, &val, &input_len, &pos, &limit, &safe) == 8) {
    uint8_t input[128], destination[64];
    BrotliBitReader br;
    brotli_reg_t bits = 5, value = 4660, result = 3735928559u;
    int status = 1;
    unsigned i;
    if (id >= 35 || available > 63 || input_len > 128 || pos < 0 || pos > 64 ||
        limit > 64 - pos || limit < -16 || op > 7) abort();
    for (i = 0; i < 128; ++i) input[i] = (uint8_t)(i * 73 + 19);
    for (i = 0; i < 64; ++i) destination[i] = (uint8_t)(i * 37 + 165);
    BrotliInitBitReader(&br);
    BrotliBitReaderSetInput(&br, input, input_len);
    br.val_ = (brotli_reg_t)val;
    br.bit_pos_ = available;
    switch (op) {
      case 0: result = DecodeSymbol((brotli_reg_t)val, tables[id], &br); break;
      case 1: status = SafeDecodeSymbol(tables[id], &br, &result); break;
      case 2: status = SafeReadSymbol(tables[id], &br, &result); break;
      case 3: result = ReadSymbol(tables[id], &br); break;
      case 4: PreloadSymbol(safe, tables[id], &br, &bits, &value); break;
      case 5:
        PreloadSymbol(0, tables[id], &br, &bits, &value);
        result = ReadPreloadedSymbol(tables[id], &br, &bits, &value);
        break;
      case 6:
        PreloadSymbol(0, tables[id], &br, &bits, &value);
        status = BrotliCopyPreloadedSymbolsToU8(tables[id], &br, &bits,
            &value, destination, pos, limit);
        break;
      case 7: status = BrotliSafeReadSymbolForTest(tables[id], &br, &result); break;
    }
    U64((uint64_t)(int64_t)status);
    U64(result); U64(br.val_); U64(br.bit_pos_);
    U64((uint64_t)(br.next_in - input));
    U64((uint64_t)(br.guard_in - input));
    U64(BrotliBitReaderGetAvailIn(&br)); U64(bits); U64(value);
    if (fwrite(destination, 1, 64, stdout) != 64) abort();
  }
}
#endif

static void DecodeFile(const char* path) {
  FILE* file = fopen(path, "rb");
  uint8_t* input = malloc(1u << 22);
  uint8_t* output = malloc(1u << 24);
  size_t len, size = 1u << 24;
  BrotliDecoderResult result;
  if (!file || !input || !output) abort();
  len = fread(input, 1, 1u << 22, file);
  if (!feof(file)) abort();
  fclose(file);
  result = BrotliDecoderDecompress(len, input, &size, output);
  if (result != BROTLI_DECODER_RESULT_SUCCESS || size > (1u << 24)) abort();
  if (fwrite(output, 1, size, stdout) != size) abort();
  free(output); free(input);
}

int main(int argc, char** argv) {
  if (argc == 3 && strcmp(argv[1], "decode") == 0) {
    DecodeFile(argv[2]);
    return 0;
  }
#if !defined(SYMBOLS_DEFAULT)
  InitTables();
  if (argc == 2 && strcmp(argv[1], "run") == 0) {
    Run();
    return ferror(stdin) || ferror(stdout);
  }
  if (argc == 2 && strcmp(argv[1], "tables") == 0) {
    unsigned id, i;
    for (id = 0; id < 35; ++id) {
      U64(sizes[id]);
      for (i = 0; i < sizes[id]; ++i) {
        U64(tables[id][i].bits); U64(tables[id][i].value);
      }
    }
    return ferror(stdout);
  }
#endif
  return 2;
}
