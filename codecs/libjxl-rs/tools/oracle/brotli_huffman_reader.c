// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual pinned decoder bodies and genuine encoder-generated header fixtures.
#include <inttypes.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#if defined(HEADER_DEFAULT)
#include "brotli/decode.h"
#else
#include "dec/decode.c"
#include "enc/brotli_bit_stream.h"
#if defined(BROTLI_HUFFMAN_CODE_FAST_LOAD)
#error This native observer requires separate bits/value fields
#endif

enum { OUTPUT_CODES = 1152, INPUT_CAPACITY = 16384 };
typedef struct { size_t offset, size; } Field;
static Field fields[1400];
static size_t field_count, field_bytes;

static void AddField(size_t offset, size_t size) {
  if (field_count == sizeof(fields) / sizeof(fields[0])) abort();
  fields[field_count].offset = offset; fields[field_count].size = size;
  ++field_count; field_bytes += size;
}
#define FIELD(F) AddField(offsetof(BrotliMetablockHeaderArena, F), sizeof(((BrotliMetablockHeaderArena*)0)->F))
static void InitFields(void) {
  unsigned i;
  FIELD(substate_tree_group); FIELD(substate_context_map); FIELD(substate_huffman);
  FIELD(sub_loop_counter); FIELD(repeat_code_len); FIELD(prev_code_len);
  FIELD(symbol); FIELD(repeat); FIELD(space);
  for (i = 0; i < 32; ++i) {
    AddField(offsetof(BrotliMetablockHeaderArena,table) + i * sizeof(HuffmanCode) + offsetof(HuffmanCode,bits),1);
    AddField(offsetof(BrotliMetablockHeaderArena,table) + i * sizeof(HuffmanCode) + offsetof(HuffmanCode,value),2);
  }
  FIELD(symbol_lists); FIELD(symbols_lists_array); FIELD(next_symbol);
  FIELD(code_length_code_lengths); FIELD(code_length_histo);
  FIELD(htree_index); FIELD(next); FIELD(context_index); FIELD(max_run_length_prefix); FIELD(code);
  for (i = 0; i < 646; ++i) {
    AddField(offsetof(BrotliMetablockHeaderArena,context_map_table) + i * sizeof(HuffmanCode) + offsetof(HuffmanCode,bits),1);
    AddField(offsetof(BrotliMetablockHeaderArena,context_map_table) + i * sizeof(HuffmanCode) + offsetof(HuffmanCode,value),2);
  }
}
#undef FIELD

static void U64(uint64_t value) {
  unsigned i; uint8_t bytes[8];
  for (i = 0; i < 8; ++i) bytes[i] = (uint8_t)(value >> (8 * i));
  if (fwrite(bytes,1,8,stdout) != 8) abort();
}
static void Bytes(const void* bytes,size_t size) {
  if (fwrite(bytes,1,size,stdout) != size) abort();
}
static void* SeedAlloc(void* opaque,size_t size) {
  void* block = malloc(size);
  if (block) memset(block,*(const uint8_t*)opaque,size);
  return block;
}
static void SeedFree(void* opaque,void* block) { (void)opaque; free(block); }

static BrotliDecoderState* NewHeader(uint8_t* seed) {
  const uint8_t empty[1] = {0};
  const uint8_t* in = empty;
  size_t available_in = 0, available_out = 0;
  BrotliDecoderState* state = BrotliDecoderCreateInstance(SeedAlloc,SeedFree,seed);
  if (!state) abort();
  /* Execute the real source checkpoint, rather than supply a copied body.
     StateInit supplies cleanup/stream invariants; empty input stops the very
     first DecodeVarLenUint8 before any Huffman header is consumed. */
  state->state = BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER;
  if (BrotliDecoderDecompressStream(state,&available_in,&in,&available_out,NULL,NULL)
      != BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT) abort();
  if (state->arena.header.substate_huffman != BROTLI_STATE_HUFFMAN_NONE ||
      state->arena.header.symbol_lists != state->arena.header.symbols_lists_array + 16) abort();
  return state;
}

static void HeaderBytes(const BrotliMetablockHeaderArena* header) {
  size_t i;
  const uint8_t* base = (const uint8_t*)header;
  for (i = 0; i < field_count; ++i) {
    if (fields[i].offset == offsetof(BrotliMetablockHeaderArena,symbol_lists)) {
      if (header->symbol_lists != header->symbols_lists_array + 16) abort();
      U64(16);  /* Known source-initialized owner-relative pointer only. */
    } else {
      /* Every byte here was seeded by the allocator or written by source.
         No padding is included, even inside either HuffmanCode array. */
      Bytes(base + fields[i].offset,fields[i].size);
    }
  }
}

static void Fixtures(void) {
  unsigned id;
  U64(50);
  for (id = 0; id < 50; ++id) {
    HuffmanTree tree[2 * 704 + 1];
    uint32_t histo[704] = {0};
    uint8_t depth[704] = {0}, storage[INPUT_CAPACITY] = {0};
    uint16_t bits[704] = {0};
    size_t alpha, total = 0, storage_ix = 0, max_bits = 0, i;
    unsigned mode;
    if (id < 5 || id >= 41) {
      unsigned form = id < 5 ? id : (id < 46 ? id - 41 : 4);
      alpha = id < 5 ? 16 : 2048;
      if (id >= 46) {
        static const unsigned extra_alpha[4] = {2,4,128,512};
        alpha = extra_alpha[id - 46];
        if (alpha == 2) form = 1;
        if (alpha == 4) form = 3;
      }
      for (i = 0; i <= (form < 4 ? form : 3); ++i) {
        unsigned symbol = alpha < 16 ? (unsigned)i : (unsigned)(i * 3 + 1);
        histo[symbol] = form == 4 && i == 0 ? 1000 : 1;
      }
      mode = 0;
    } else {
      static const unsigned alphabets[6] = {8,16,32,64,256,704};
      unsigned group = (id - 5) / 6;
      mode = (id - 5) % 6;
      alpha = alphabets[group];
      for (i = 0; i < alpha; ++i) {
        if (mode / 2 == 0) histo[i] = 1;
        if (mode / 2 == 1) histo[i] = (uint32_t)(1 + ((i * 1237 + 19) % 10000));
        if (mode / 2 == 2 && (i % (alpha < 32 ? 3 : alpha / 6 + 1) == 0 || i + 1 == alpha)) histo[i] = (uint32_t)(1 + (i % 17));
      }
    }
    for (i = 0; i < alpha && i < 704; ++i) total += histo[i];
    for (i = alpha - 1; i; i >>= 1) ++max_bits;
    BrotliBuildAndStoreHuffmanTreeFast(tree,histo,total,max_bits,depth,bits,&storage_ix,storage);
    if (id >= 5 && id < 41 && (mode & 1)) {
      /* Dynamic code-length alphabet emitted by the other genuine encoder. */
      storage_ix = 0; memset(storage,0,sizeof(storage));
      BrotliStoreHuffmanTree(depth,alpha,tree,&storage_ix,storage);
    }
    U64(alpha); U64(storage_ix); U64((storage_ix + 7) / 8);
    Bytes(storage,(storage_ix + 7) / 8);
  }
}

static void Schema(void) {
  size_t i;
  U64(sizeof(BrotliMetablockHeaderArena)); U64(_Alignof(BrotliMetablockHeaderArena));
  U64(sizeof(BrotliDecoderState)); U64(offsetof(BrotliDecoderState,arena));
  U64(field_bytes); U64(field_count);
  for (i = 0; i < field_count; ++i) { U64(fields[i].offset); U64(fields[i].size); }
}

static void Observe(BrotliDecoderState* state, HuffmanCode* table, const uint8_t* input,
                    int result, brotli_reg_t table_size) {
  unsigned i;
  U64((uint64_t)(int64_t)result); U64(state->br.val_); U64(state->br.bit_pos_);
  U64((uint64_t)(state->br.next_in - input)); U64((uint64_t)(state->br.guard_in - input));
  U64(BrotliBitReaderGetAvailIn(&state->br)); U64(table_size);
  HeaderBytes(&state->arena.header);
  for (i = 0; i < OUTPUT_CODES; ++i) {
    uint8_t bytes[3] = {table[i].bits,(uint8_t)table[i].value,(uint8_t)(table[i].value >> 8)};
    Bytes(bytes,3);
  }
}

static void Run(void) {
  unsigned max,limit,opt,initial_bits,steps,size,start_state;
  uint64_t initial_val;
  while (scanf("%u %u %u %u %" SCNu64 " %u %u %u",&max,&limit,&opt,
      &initial_bits,&initial_val,&steps,&size,&start_state) == 8) {
    unsigned chunks[INPUT_CAPACITY],i,j;
    uint8_t input[INPUT_CAPACITY];
    if (steps > INPUT_CAPACITY || size > INPUT_CAPACITY || initial_bits > 7 || max < 2 || max > 2048 || limit > max || limit > 704 || opt > 3) abort();
    if ((opt & 2) && limit > 272) abort();
    for (i = 0; i < steps; ++i) if (scanf("%u",&chunks[i]) != 1) abort();
    for (i = 0; i < size; ++i) {
      unsigned byte; if (scanf("%2x",&byte) != 1) abort(); input[i] = (uint8_t)byte;
    }
    for (j = 0; j < 2; ++j) {
      uint8_t seed = j ? 0x5a : 0xa5;
      BrotliDecoderState* state = NewHeader(&seed);
      HuffmanCode table[OUTPUT_CODES];
      brotli_reg_t table_size = 3735928559u;
      size_t offset = 0;
      if (start_state != 0 && start_state != UINT32_MAX) abort();
      state->arena.header.substate_huffman = (BrotliRunningHuffmanState)start_state;
      for (i = 0; i < OUTPUT_CODES; ++i) table[i] = ConstructHuffmanCode(seed,j ? 4660 : 48879);
      BrotliInitBitReader(&state->br);
      state->br.val_ = (brotli_reg_t)initial_val; state->br.bit_pos_ = initial_bits;
      for (i = 0; i < steps; ++i) {
        BrotliDecoderErrorCode result;
        if (chunks[i] > size - offset) abort();
        BrotliBitReaderSetInput(&state->br,input + offset,chunks[i]);
        result = ReadHuffmanCode(max,limit,(opt & 2) ? state->arena.header.context_map_table : table,
            (opt & 1) ? &table_size : NULL,state);
        Observe(state,table,input,(int)result,table_size);
        if (result != BROTLI_DECODER_NEEDS_MORE_INPUT) break;
        if (state->br.next_in != state->br.last_in) abort();
        offset += chunks[i];
      }
      BrotliDecoderDestroyInstance(state);
    }
  }
}
#endif

static void DecodeFile(const char* path) {
  FILE* file = fopen(path,"rb"); uint8_t* input = malloc(1u << 22);
  uint8_t* output = malloc(1u << 24); size_t size = 1u << 24,len;
  if (!file || !input || !output) abort();
  len = fread(input,1,1u << 22,file); if (!feof(file)) abort(); fclose(file);
  if (BrotliDecoderDecompress(len,input,&size,output) != BROTLI_DECODER_RESULT_SUCCESS) abort();
  if (fwrite(output,1,size,stdout) != size) abort(); free(output); free(input);
}
int main(int argc,char** argv) {
  if (argc == 3 && strcmp(argv[1],"decode") == 0) { DecodeFile(argv[2]); return 0; }
#if !defined(HEADER_DEFAULT)
  InitFields();
  if (argc == 2 && strcmp(argv[1],"fixtures") == 0) { Fixtures(); return 0; }
  if (argc == 2 && strcmp(argv[1],"schema") == 0) { Schema(); return 0; }
  if (argc == 2 && strcmp(argv[1],"run") == 0) { Run(); return ferror(stdin) || ferror(stdout); }
#endif
  return 2;
}
