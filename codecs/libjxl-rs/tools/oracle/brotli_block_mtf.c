// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#if defined(BLOCK_MTF_DEFAULT)
#include "brotli/decode.h"
#else
#include "dec/decode.c"
#if defined(BROTLI_HUFFMAN_CODE_FAST_LOAD)
#error Native separate-field Huffman representation required
#endif

static void U64(uint64_t value) {
  unsigned i; uint8_t bytes[8];
  for (i = 0; i < 8; ++i) bytes[i] = (uint8_t)(value >> (8*i));
  if (fwrite(bytes,1,8,stdout) != 8) abort();
}
static void Bytes(const void* ptr,size_t size) {
  if (fwrite(ptr,1,size,stdout) != size) abort();
}
static void* SeedAlloc(void* opaque,size_t size) {
  void* ptr = malloc(size); if (ptr) memset(ptr,*(uint8_t*)opaque,size);return ptr;
}
static void SeedFree(void* opaque,void* ptr) { (void)opaque;free(ptr); }

static void Table(HuffmanCode* table,unsigned family,unsigned code) {
  unsigned i,n = 26;uint8_t lengths[26];uint16_t links[42],count[16] = {0};int next[16];
  uint16_t symbols[4] = {(uint16_t)code,(uint16_t)((code+1)%26),0,0};
  for (i = 0; i < 512; ++i) table[i] = ConstructHuffmanCode(165,48879);
  if (family <= 1) { BrotliBuildSimpleHuffmanTable(table,8,symbols,family);return; }
  if (family == 2) {
    for (i = 0; i < 26; ++i) lengths[i] = i < 6 ? 4 : 5;
  } else {
    n = 16;for (i = 0; i < 14; ++i) lengths[i] = (uint8_t)(i+1);
    lengths[14] = lengths[15] = 15;
    while (n < 26) {
      unsigned j = 0;for (i = 1; i < n; ++i) if (lengths[i] < lengths[j]) j = i;
      ++lengths[j];lengths[n++] = lengths[j];
    }
  }
  for (i = 0; i < 16; ++i) next[i] = (int)i-16;
  for (i = 0; i < 42; ++i) links[i] = 65535;
  for (i = 0; i < 26; ++i) {
    unsigned len = lengths[i];++count[len];links[16+next[len]] = (uint16_t)i;next[len] = (int)i;
  }
  BrotliBuildHuffmanTable(table,8,links+16,count);
}

static void Blocks(void) {
  unsigned family,code,op,state,index_known,index,available,size,steps,i,j;
  uint64_t val;
  while (scanf("%u %u %u %u %u %u %u %" SCNu64 " %u %u",&family,&code,&op,
      &state,&index_known,&index,&available,&val,&size,&steps) == 10) {
    uint8_t input[128];unsigned chunks[128];HuffmanCode table[512];
    if (family > 3 || code > 25 || op > 1 || available > 63 || size > 128 || steps > 128 ||
        index > 25 || (state != 0 && !index_known) || (!op && (state != 0 || steps != 1 || size < 64))) abort();
    for (i = 0; i < steps; ++i) if (scanf("%u",&chunks[i]) != 1) abort();
    for (i = 0; i < size; ++i) { unsigned b;if (scanf("%2x",&b) != 1) abort();input[i]=(uint8_t)b; }
    Table(table,family,code);
    for (j = 0; j < 2; ++j) {
      uint8_t seed = j ? 0x5a : 0xa5;size_t offset = 0;
      BrotliDecoderState* s = BrotliDecoderCreateInstance(SeedAlloc,SeedFree,&seed);
      brotli_reg_t result = 3735928559u;
      if (!s) abort();
      s->substate_read_block_length = (BrotliRunningReadBlockLengthState)state;
      if (index_known) s->block_length_index = index;
      s->br.val_ = (brotli_reg_t)val;s->br.bit_pos_ = available;
      for (i = 0; i < steps; ++i) {
        BROTLI_BOOL ok = BROTLI_TRUE;
        if (chunks[i] > size-offset) abort();
        BrotliBitReaderSetInput(&s->br,input+offset,chunks[i]);
        if (op) ok = SafeReadBlockLength(s,&result,table,&s->br);
        else result = ReadBlockLength(table,&s->br);
        U64(ok);U64(result);U64(s->br.val_);U64(s->br.bit_pos_);
        U64((uint64_t)(s->br.next_in-input));U64((uint64_t)(s->br.guard_in-input));
        U64(BrotliBitReaderGetAvailIn(&s->br));U64(s->substate_read_block_length);
        /* Allocator-seeded actual field bytes; do not infer logical source
           initialization merely from this fixture's known physical contents. */
        Bytes(&s->block_length_index,sizeof(s->block_length_index));
        if (ok) break;
        if (s->br.next_in != s->br.last_in) abort();offset += chunks[i];
      }
      BrotliDecoderDestroyInstance(s);
    }
  }
}

static void Mtf(void) {
  int initial_bound;unsigned calls,j;
  while (scanf("%d %u",&initial_bound,&calls) == 2) {
    unsigned sizes[128],lengths[128],i,k;uint8_t* data[128];
    if (initial_bound < -1 || initial_bound > 63 || calls > 128) abort();
    for (i = 0; i < calls; ++i) {
      if (scanf("%u %u",&sizes[i],&lengths[i]) != 2 || sizes[i] > 8192 || lengths[i] > sizes[i]) abort();
      data[i] = malloc(sizes[i]+1);if (!data[i]) abort();
      for (k = 0; k < sizes[i]; ++k) {unsigned b;if(scanf("%2x",&b)!=1)abort();data[i][k]=(uint8_t)b;}
    }
    for (j = 0; j < 2; ++j) {
      uint8_t seed = j ? 0x5a : 0xa5;
      BrotliDecoderState* s = BrotliDecoderCreateInstance(SeedAlloc,SeedFree,&seed);
      if (!s) abort();if(initial_bound>=0)s->mtf_upper_bound=(brotli_reg_t)initial_bound;
      for (i = 0; i < calls; ++i) {
        uint8_t v[8192];memcpy(v,data[i],sizes[i]);
        /* Actual existing source test bridge, not an observer algorithm body. */
        BrotliInverseMoveToFrontTransformForTest(v,lengths[i],s);
        U64(s->mtf_upper_bound);Bytes(s->mtf,sizeof(s->mtf));Bytes(v,sizes[i]);
      }
      BrotliDecoderDestroyInstance(s);
    }
    for (i = 0; i < calls; ++i) free(data[i]);
  }
}
#endif

static void DecodeFile(const char* path) {
  FILE* file=fopen(path,"rb");uint8_t* input=malloc(1u<<22);uint8_t* output=malloc(1u<<24);
  size_t n,size=1u<<24;if(!file||!input||!output)abort();n=fread(input,1,1u<<22,file);
  if(!feof(file))abort();fclose(file);
  if(BrotliDecoderDecompress(n,input,&size,output)!=BROTLI_DECODER_RESULT_SUCCESS)abort();
  if(fwrite(output,1,size,stdout)!=size)abort();free(output);free(input);
}
int main(int argc,char**argv) {
  if(argc==3&&strcmp(argv[1],"decode")==0){DecodeFile(argv[2]);return 0;}
#if !defined(BLOCK_MTF_DEFAULT)
  if(argc==2&&strcmp(argv[1],"block")==0){Blocks();return ferror(stdin)||ferror(stdout);}
  if(argc==2&&strcmp(argv[1],"mtf")==0){Mtf();return ferror(stdin)||ferror(stdout);}
  if(argc==2&&strcmp(argv[1],"tables")==0){
    unsigned family,code,i;HuffmanCode table[512];
    for(family=0;family<4;++family)for(code=0;code<26;++code){
      Table(table,family,code);
      /* Explicit scalar fields only: source HuffmanCode byte1 is padding. */
      for(i=0;i<512;++i){U64(table[i].bits);U64(table[i].value);}
    }
    return ferror(stdout);
  }
#endif
  return 2;
}
