// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile as C: conversion from any int32_t to the selected signed C enum
// retains its representable value, without a C++ invalid-enum cast/load.
#include <inttypes.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "brotli/decode.h"
#include "common/version.h"
#include "dec/state.h"

_Static_assert(sizeof(BrotliDecoderErrorCode) == sizeof(int32_t), "32-bit errors");
_Static_assert((BrotliDecoderErrorCode)-1 < 0, "signed errors");
_Static_assert(sizeof(BrotliDecoderResult) == sizeof(uint32_t), "32-bit results");
_Static_assert((BrotliDecoderResult)-1 > 0, "unsigned results");

static const char* ErrorStringFromInt32(int32_t value) {
  return BrotliDecoderErrorString((BrotliDecoderErrorCode)value);
}

static void Declarations(void) {
  printf("version %" PRIu32 " %u %u %u %u %u %u\n", BrotliDecoderVersion(),
      BROTLI_VERSION_MAJOR, BROTLI_VERSION_MINOR, BROTLI_VERSION_PATCH,
      BROTLI_ABI_CURRENT, BROTLI_ABI_REVISION, BROTLI_ABI_AGE);
  printf("layout BrotliDecoderResult %zu %zu 0\n", sizeof(BrotliDecoderResult), _Alignof(BrotliDecoderResult));
  printf("layout BrotliDecoderErrorCode %zu %zu 1\n", sizeof(BrotliDecoderErrorCode), _Alignof(BrotliDecoderErrorCode));
  printf("result BROTLI_DECODER_RESULT_ERROR %u\n", (unsigned)BROTLI_DECODER_RESULT_ERROR);
  printf("result BROTLI_DECODER_RESULT_SUCCESS %u\n", (unsigned)BROTLI_DECODER_RESULT_SUCCESS);
  printf("result BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT %u\n", (unsigned)BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT);
  printf("result BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT %u\n", (unsigned)BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT);
#define PRINT_CODE(PREFIX, NAME, CODE) \
  printf("code BROTLI_DECODER%s%s %d %s\n", #PREFIX, #NAME, \
      (int)(BROTLI_DECODER ## PREFIX ## NAME), \
      BrotliDecoderErrorString(BROTLI_DECODER ## PREFIX ## NAME));
#define NO_SEPARATOR
  BROTLI_DECODER_ERROR_CODES_LIST(PRINT_CODE, NO_SEPARATOR)
#undef PRINT_CODE
#undef NO_SEPARATOR
  printf("alias BROTLI_LAST_ERROR_CODE %d\n", (int)BROTLI_LAST_ERROR_CODE);
  _Static_assert(sizeof(BrotliRunningState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningState %zu %zu 0\n", sizeof(BrotliRunningState), _Alignof(BrotliRunningState));
  printf("running BrotliRunningState BROTLI_STATE_UNINITED %u\n", (unsigned)BROTLI_STATE_UNINITED);
  printf("running BrotliRunningState BROTLI_STATE_LARGE_WINDOW_BITS %u\n", (unsigned)BROTLI_STATE_LARGE_WINDOW_BITS);
  printf("running BrotliRunningState BROTLI_STATE_INITIALIZE %u\n", (unsigned)BROTLI_STATE_INITIALIZE);
  printf("running BrotliRunningState BROTLI_STATE_METABLOCK_BEGIN %u\n", (unsigned)BROTLI_STATE_METABLOCK_BEGIN);
  printf("running BrotliRunningState BROTLI_STATE_METABLOCK_HEADER %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER);
  printf("running BrotliRunningState BROTLI_STATE_METABLOCK_HEADER_2 %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_2);
  printf("running BrotliRunningState BROTLI_STATE_CONTEXT_MODES %u\n", (unsigned)BROTLI_STATE_CONTEXT_MODES);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_BEGIN %u\n", (unsigned)BROTLI_STATE_COMMAND_BEGIN);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_INNER %u\n", (unsigned)BROTLI_STATE_COMMAND_INNER);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_POST_DECODE_LITERALS %u\n", (unsigned)BROTLI_STATE_COMMAND_POST_DECODE_LITERALS);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_POST_WRAP_COPY %u\n", (unsigned)BROTLI_STATE_COMMAND_POST_WRAP_COPY);
  printf("running BrotliRunningState BROTLI_STATE_UNCOMPRESSED %u\n", (unsigned)BROTLI_STATE_UNCOMPRESSED);
  printf("running BrotliRunningState BROTLI_STATE_METADATA %u\n", (unsigned)BROTLI_STATE_METADATA);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_INNER_WRITE %u\n", (unsigned)BROTLI_STATE_COMMAND_INNER_WRITE);
  printf("running BrotliRunningState BROTLI_STATE_METABLOCK_DONE %u\n", (unsigned)BROTLI_STATE_METABLOCK_DONE);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_POST_WRITE_1 %u\n", (unsigned)BROTLI_STATE_COMMAND_POST_WRITE_1);
  printf("running BrotliRunningState BROTLI_STATE_COMMAND_POST_WRITE_2 %u\n", (unsigned)BROTLI_STATE_COMMAND_POST_WRITE_2);
  printf("running BrotliRunningState BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER %u\n", (unsigned)BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER);
  printf("running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_0 %u\n", (unsigned)BROTLI_STATE_HUFFMAN_CODE_0);
  printf("running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_1 %u\n", (unsigned)BROTLI_STATE_HUFFMAN_CODE_1);
  printf("running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_2 %u\n", (unsigned)BROTLI_STATE_HUFFMAN_CODE_2);
  printf("running BrotliRunningState BROTLI_STATE_HUFFMAN_CODE_3 %u\n", (unsigned)BROTLI_STATE_HUFFMAN_CODE_3);
  printf("running BrotliRunningState BROTLI_STATE_CONTEXT_MAP_1 %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_1);
  printf("running BrotliRunningState BROTLI_STATE_CONTEXT_MAP_2 %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_2);
  printf("running BrotliRunningState BROTLI_STATE_TREE_GROUP %u\n", (unsigned)BROTLI_STATE_TREE_GROUP);
  printf("running BrotliRunningState BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY %u\n", (unsigned)BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY);
  printf("running BrotliRunningState BROTLI_STATE_DONE %u\n", (unsigned)BROTLI_STATE_DONE);
  _Static_assert(sizeof(BrotliRunningMetablockHeaderState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningMetablockHeaderState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningMetablockHeaderState %zu %zu 0\n", sizeof(BrotliRunningMetablockHeaderState), _Alignof(BrotliRunningMetablockHeaderState));
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_NONE %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_NONE);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_EMPTY %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_EMPTY);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_NIBBLES %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_NIBBLES);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_SIZE %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_SIZE);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_RESERVED %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_RESERVED);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_BYTES %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_BYTES);
  printf("running BrotliRunningMetablockHeaderState BROTLI_STATE_METABLOCK_HEADER_METADATA %u\n", (unsigned)BROTLI_STATE_METABLOCK_HEADER_METADATA);
  _Static_assert(sizeof(BrotliRunningUncompressedState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningUncompressedState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningUncompressedState %zu %zu 0\n", sizeof(BrotliRunningUncompressedState), _Alignof(BrotliRunningUncompressedState));
  printf("running BrotliRunningUncompressedState BROTLI_STATE_UNCOMPRESSED_NONE %u\n", (unsigned)BROTLI_STATE_UNCOMPRESSED_NONE);
  printf("running BrotliRunningUncompressedState BROTLI_STATE_UNCOMPRESSED_WRITE %u\n", (unsigned)BROTLI_STATE_UNCOMPRESSED_WRITE);
  _Static_assert(sizeof(BrotliRunningTreeGroupState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningTreeGroupState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningTreeGroupState %zu %zu 0\n", sizeof(BrotliRunningTreeGroupState), _Alignof(BrotliRunningTreeGroupState));
  printf("running BrotliRunningTreeGroupState BROTLI_STATE_TREE_GROUP_NONE %u\n", (unsigned)BROTLI_STATE_TREE_GROUP_NONE);
  printf("running BrotliRunningTreeGroupState BROTLI_STATE_TREE_GROUP_LOOP %u\n", (unsigned)BROTLI_STATE_TREE_GROUP_LOOP);
  _Static_assert(sizeof(BrotliRunningContextMapState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningContextMapState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningContextMapState %zu %zu 0\n", sizeof(BrotliRunningContextMapState), _Alignof(BrotliRunningContextMapState));
  printf("running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_NONE %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_NONE);
  printf("running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_READ_PREFIX %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_READ_PREFIX);
  printf("running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_HUFFMAN %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_HUFFMAN);
  printf("running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_DECODE %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_DECODE);
  printf("running BrotliRunningContextMapState BROTLI_STATE_CONTEXT_MAP_TRANSFORM %u\n", (unsigned)BROTLI_STATE_CONTEXT_MAP_TRANSFORM);
  _Static_assert(sizeof(BrotliRunningHuffmanState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningHuffmanState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningHuffmanState %zu %zu 0\n", sizeof(BrotliRunningHuffmanState), _Alignof(BrotliRunningHuffmanState));
  printf("running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_NONE %u\n", (unsigned)BROTLI_STATE_HUFFMAN_NONE);
  printf("running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_SIMPLE_SIZE %u\n", (unsigned)BROTLI_STATE_HUFFMAN_SIMPLE_SIZE);
  printf("running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_SIMPLE_READ %u\n", (unsigned)BROTLI_STATE_HUFFMAN_SIMPLE_READ);
  printf("running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_SIMPLE_BUILD %u\n", (unsigned)BROTLI_STATE_HUFFMAN_SIMPLE_BUILD);
  printf("running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_COMPLEX %u\n", (unsigned)BROTLI_STATE_HUFFMAN_COMPLEX);
  printf("running BrotliRunningHuffmanState BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS %u\n", (unsigned)BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS);
  _Static_assert(sizeof(BrotliRunningDecodeUint8State) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningDecodeUint8State)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningDecodeUint8State %zu %zu 0\n", sizeof(BrotliRunningDecodeUint8State), _Alignof(BrotliRunningDecodeUint8State));
  printf("running BrotliRunningDecodeUint8State BROTLI_STATE_DECODE_UINT8_NONE %u\n", (unsigned)BROTLI_STATE_DECODE_UINT8_NONE);
  printf("running BrotliRunningDecodeUint8State BROTLI_STATE_DECODE_UINT8_SHORT %u\n", (unsigned)BROTLI_STATE_DECODE_UINT8_SHORT);
  printf("running BrotliRunningDecodeUint8State BROTLI_STATE_DECODE_UINT8_LONG %u\n", (unsigned)BROTLI_STATE_DECODE_UINT8_LONG);
  _Static_assert(sizeof(BrotliRunningReadBlockLengthState) == sizeof(uint32_t), "32-bit running state");
  _Static_assert((BrotliRunningReadBlockLengthState)-1 > 0, "unsigned running state");
  printf("layout BrotliRunningReadBlockLengthState %zu %zu 0\n", sizeof(BrotliRunningReadBlockLengthState), _Alignof(BrotliRunningReadBlockLengthState));
  printf("running BrotliRunningReadBlockLengthState BROTLI_STATE_READ_BLOCK_LENGTH_NONE %u\n", (unsigned)BROTLI_STATE_READ_BLOCK_LENGTH_NONE);
  printf("running BrotliRunningReadBlockLengthState BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX %u\n", (unsigned)BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX);
}

int main(int argc, char** argv) {
  if (argc != 2) return 2;
  if (!strcmp(argv[1], "declarations")) {
    Declarations();
  } else if (!strcmp(argv[1], "strings")) {
    int32_t value;
    while (scanf("%" SCNd32, &value) == 1) {
      printf("%" PRId32 " %s\n", value, ErrorStringFromInt32(value));
    }
    if (!feof(stdin)) return 2;
  } else {
    return 2;
  }
  return ferror(stdout) ? 3 : 0;
}
