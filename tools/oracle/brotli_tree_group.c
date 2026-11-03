// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual pinned decoder bodies and genuine encoder-generated header fixtures.
#include <inttypes.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#if defined(TREE_GROUP_DEFAULT)
#include "brotli/decode.h"
#else
#include "dec/decode.c"
#include "enc/brotli_bit_stream.h"
#if defined(BROTLI_HUFFMAN_CODE_FAST_LOAD)
#error This native observer requires separate bits/value fields
#endif

enum { INPUT_CAPACITY = 16384 };
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
typedef struct { uint8_t seed; void* last; size_t last_size; } SeedContext;
static void* SeedAlloc(void* opaque,size_t size) {
  void* block = malloc(size);
  SeedContext* context = (SeedContext*)opaque;
  if (block) memset(block,context->seed,size);
  context->last=block;context->last_size=size;return block;
}
static void SeedFree(void* opaque,void* block) { (void)opaque; free(block); }

static BrotliDecoderState* NewHeader(SeedContext* context) {
  const uint8_t empty[1] = {0};
  const uint8_t* in = empty;
  size_t available_in = 0, available_out = 0;
  BrotliDecoderState* state = BrotliDecoderCreateInstance(SeedAlloc,SeedFree,context);
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

static void HeaderBytes(const BrotliMetablockHeaderArena* header, const HuffmanCode* code_base, size_t capacity, int next_known) {
  size_t i;
  const uint8_t* base = (const uint8_t*)header;
  for (i = 0; i < field_count; ++i) {
    if (fields[i].offset == offsetof(BrotliMetablockHeaderArena,symbol_lists)) {
      if (header->symbol_lists != header->symbols_lists_array + 16) abort();
      U64(16);  /* Known source-initialized owner-relative pointer only. */
    } else if (fields[i].offset == offsetof(BrotliMetablockHeaderArena,next) && next_known) {
      if (header->next == NULL) U64(UINT64_MAX);
      else {
        uintptr_t offset = (uintptr_t)header->next - (uintptr_t)code_base;
        if ((uintptr_t)header->next < (uintptr_t)code_base || offset % sizeof(HuffmanCode) || offset / sizeof(HuffmanCode) > capacity) abort();
        U64(offset / sizeof(HuffmanCode));
      }
    } else {
      /* Every byte here was seeded by the allocator or written by source.
         No padding is included, even inside either HuffmanCode array. */
      Bytes(base + fields[i].offset,fields[i].size);
    }
  }
}

static void Fixtures(void) {
  static const unsigned alphabets[11]={2,4,8,16,32,64,128,256,512,704,2048};
  static const unsigned tree_counts[4]={1,2,3,8};
  unsigned a,c,style,t;U64(133);
  U64(16);U64(16);U64(0);U64(0);U64(0);
  for(a=0;a<11;++a)for(c=0;c<4;++c)for(style=0;style<3;++style) {
    uint8_t storage[INPUT_CAPACITY]={0};size_t storage_ix=0;
    unsigned alpha=alphabets[a],limit=alpha<704?alpha:704,ntrees=tree_counts[c];
    if(alpha==2048)limit=16;
    for(t=0;t<ntrees;++t) {
      HuffmanTree tree[2*704+1];uint32_t histo[704]={0};
      uint8_t depth[704]={0},scratch[INPUT_CAPACITY]={0};uint16_t bits[704]={0};
      size_t i,total=0,max_bits=0,scratch_ix=0;
      if(style==0||alpha==2048) {
        unsigned symbols=1+(t%4);if(symbols>limit)symbols=limit;
        for(i=0;i<symbols;++i)histo[alpha<16?i:i*3+1]=(uint32_t)(i==0?1000:1);
      } else {
        for(i=0;i<alpha;++i)histo[i]=style==1?1:(uint32_t)(1+(i*1237+t*19)%10000);
      }
      for(i=0;i<704;++i)total+=histo[i];
      for(i=alpha-1;i;i>>=1)++max_bits;
      if(style!=0&&alpha<=704&&(t&1)) {
        /* Get genuine depths from the actual fast encoder into independent
           scratch, then append the actual dynamic encoder's header. No bit
           stream is manually rewritten or invented by this observer. */
        BrotliBuildAndStoreHuffmanTreeFast(tree,histo,total,max_bits,depth,bits,&scratch_ix,scratch);
        BrotliStoreHuffmanTree(depth,alpha,tree,&storage_ix,storage);
      } else {
        BrotliBuildAndStoreHuffmanTreeFast(tree,histo,total,max_bits,depth,bits,&storage_ix,storage);
      }
    }
    if(storage_ix+64>8*sizeof(storage))abort();
    U64(alpha);U64(limit);U64(ntrees);U64(storage_ix);U64((storage_ix+7)/8);
    Bytes(storage,(storage_ix+7)/8);
  }
}

static void Schema(void) {
  size_t i;U64(sizeof(BrotliMetablockHeaderArena));U64(_Alignof(BrotliMetablockHeaderArena));
  U64(sizeof(BrotliDecoderState));U64(offsetof(BrotliDecoderState,arena));
  U64(field_bytes);U64(field_count);
  for(i=0;i<field_count;++i){U64(fields[i].offset);U64(fields[i].size);}
  U64(sizeof(HuffmanTreeGroup));U64(_Alignof(HuffmanTreeGroup));
  U64(offsetof(HuffmanTreeGroup,htrees));U64(offsetof(HuffmanTreeGroup,codes));
  U64(offsetof(HuffmanTreeGroup,alphabet_size_max));U64(offsetof(HuffmanTreeGroup,alphabet_size_limit));
  U64(offsetof(HuffmanTreeGroup,num_htrees));U64(sizeof(HuffmanCode));U64(sizeof(HuffmanCode*));
}

static void Observe(BrotliDecoderState* s,HuffmanTreeGroup* group,HuffmanCode* code_base,
    size_t capacity,size_t allocation_size,const uint8_t* input,int result,int next_known,
    const uint8_t* initialized) {
  unsigned i;U64((uint64_t)(int64_t)result);U64(s->br.val_);U64(s->br.bit_pos_);
  U64((uint64_t)(s->br.next_in-input));U64((uint64_t)(s->br.guard_in-input));
  U64(BrotliBitReaderGetAvailIn(&s->br));U64(next_known);
  U64(allocation_size);U64(group->num_htrees*sizeof(HuffmanCode*));U64(capacity);
  HeaderBytes(&s->arena.header,code_base,capacity,next_known);
  for(i=0;i<group->num_htrees;++i) {
    U64(initialized[i]);
    if(initialized[i]) {
      uintptr_t offset=(uintptr_t)group->htrees[i]-(uintptr_t)code_base;
      if((uintptr_t)group->htrees[i]<(uintptr_t)code_base||offset%sizeof(HuffmanCode)||offset/sizeof(HuffmanCode)>capacity)abort();
      U64(offset/sizeof(HuffmanCode));
    } else {
      /* Physically seeded pointer slot bytes, never an unknown typed pointer. */
      Bytes((const uint8_t*)group->htrees+i*sizeof(HuffmanCode*),sizeof(HuffmanCode*));
    }
  }
  for(i=0;i<capacity;++i) {
    uint8_t record[3]={code_base[i].bits,(uint8_t)code_base[i].value,(uint8_t)(code_base[i].value>>8)};
    Bytes(record,3);  /* Explicit fields only, never source padding byte1. */
  }
}

static void Run(void) {
  unsigned max,limit,ntrees,mode,initial_bits,size,steps;uint64_t initial_val;
  while(scanf("%u %u %u %u %u %"SCNu64" %u %u",&max,&limit,&ntrees,&mode,
      &initial_bits,&initial_val,&steps,&size)==8) {
    uint8_t input[INPUT_CAPACITY];unsigned chunks[INPUT_CAPACITY],i,j;
    if(ntrees>8||mode>6||steps>INPUT_CAPACITY||size>INPUT_CAPACITY||initial_bits>7||
       max>2048||limit>704||limit>max||(ntrees&&mode!=1&&mode!=6&&max<2))abort();
    for(i=0;i<steps;++i)if(scanf("%u",&chunks[i])!=1)abort();
    for(i=0;i<size;++i){unsigned b;if(scanf("%2x",&b)!=1)abort();input[i]=(uint8_t)b;}
    for(j=0;j<2;++j) {
      SeedContext context={(uint8_t)(j?0x5a:0xa5),NULL,0};size_t offset=0;
      BrotliDecoderState* s=NewHeader(&context);HuffmanTreeGroup* group=&s->literal_hgroup;
      HuffmanCode* code_base;size_t capacity=(limit+376)*ntrees,allocation_size;
      uint8_t initialized[8]={0};int next_known=1;
      if(!BrotliDecoderHuffmanTreeGroupInit(s,group,max,limit,ntrees))abort();
      allocation_size=context.last_size;code_base=group->codes;
      if(group->htrees!=context.last||allocation_size!=ntrees*(sizeof(HuffmanCode*)+(limit+376)*sizeof(HuffmanCode))||
         (uintptr_t)code_base!=(uintptr_t)context.last+ntrees*sizeof(HuffmanCode*))abort();
      if(mode==1||mode==6) {
        s->arena.header.substate_tree_group=BROTLI_STATE_TREE_GROUP_LOOP;
        s->arena.header.htree_index=mode==1?(int)ntrees:INT32_MAX;
        next_known=0;  /* next remains allocator-seeded, never typed-read. */
      } else if(mode==2) {
        uint8_t empty[1]={0};BrotliBitReaderSetInput(&s->br,empty,0);
        if(HuffmanTreeGroupDecode(group,s)!=BROTLI_DECODER_NEEDS_MORE_INPUT)abort();
      } else if(mode==3) {
        s->arena.header.substate_tree_group=(BrotliRunningTreeGroupState)UINT32_MAX;
      } else if(mode==4) {
        s->arena.header.substate_tree_group=BROTLI_STATE_TREE_GROUP_LOOP;
        s->arena.header.htree_index=-1;s->arena.header.next=code_base;
        s->arena.header.substate_huffman=(BrotliRunningHuffmanState)UINT32_MAX;
      } else if(mode==5) {
        if(ntrees)abort();group->codes=NULL;
      }
      BrotliInitBitReader(&s->br);s->br.val_=(brotli_reg_t)initial_val;s->br.bit_pos_=initial_bits;
      for(i=0;i<steps;++i) {
        int before=(s->arena.header.substate_tree_group==BROTLI_STATE_TREE_GROUP_LOOP)?s->arena.header.htree_index:0;
        int result,k;if(chunks[i]>size-offset)abort();
        BrotliBitReaderSetInput(&s->br,input+offset,chunks[i]);
        result=(int)HuffmanTreeGroupDecode(group,s);
        for(k=before;k<s->arena.header.htree_index;++k){if(k<0||(unsigned)k>=ntrees)abort();initialized[k]=1;}
        Observe(s,group,code_base,capacity,allocation_size,input,result,next_known,initialized);
        if(result!=BROTLI_DECODER_NEEDS_MORE_INPUT)break;
        if(s->br.next_in!=s->br.last_in)abort();offset+=chunks[i];
      }
      BrotliDecoderDestroyInstance(s);
    }
  }
}
#endif
static void DecodeFile(const char* path) {
  FILE* f=fopen(path,"rb");uint8_t* input=malloc(1u<<22),*output=malloc(1u<<24);size_t len,size=1u<<24;
  if(!f||!input||!output)abort();len=fread(input,1,1u<<22,f);if(!feof(f))abort();fclose(f);
  if(BrotliDecoderDecompress(len,input,&size,output)!=BROTLI_DECODER_RESULT_SUCCESS)abort();
  if(fwrite(output,1,size,stdout)!=size)abort();free(output);free(input);
}
int main(int argc,char**argv) {
  if(argc==3&&strcmp(argv[1],"decode")==0){DecodeFile(argv[2]);return 0;}
#if !defined(TREE_GROUP_DEFAULT)
  InitFields();
  if(argc==2&&strcmp(argv[1],"fixtures")==0){Fixtures();return ferror(stdout);}
  if(argc==2&&strcmp(argv[1],"schema")==0){Schema();return ferror(stdout);}
  if(argc==2&&strcmp(argv[1],"run")==0){Run();return ferror(stdin)||ferror(stdout);}
#endif
  return 2;
}
