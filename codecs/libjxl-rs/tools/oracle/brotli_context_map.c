// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual pinned decoder bodies and genuine encoder-generated header fixtures.
#include <inttypes.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "brotli/encode.h"
#if defined(CONTEXT_MAP_DEFAULT)
#include "brotli/decode.h"
#else
#include "dec/decode.c"
void ContextMapFixtures(void);
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

static uint8_t* capture; static size_t capture_size;
static void Bytes(const void* bytes,size_t size) {
  if(capture){if(capture_size+size>8192)abort();memcpy(capture+capture_size,bytes,size);capture_size+=size;}
  else if(fwrite(bytes,1,size,stdout)!=size)abort();
}
static void U64(uint64_t value) {
  unsigned i;uint8_t bytes[8];for(i=0;i<8;++i)bytes[i]=(uint8_t)(value>>(8*i));Bytes(bytes,8);
}
static void HeaderBytes(const BrotliMetablockHeaderArena* h){size_t i;const uint8_t* base=(const uint8_t*)h;
 for(i=0;i<field_count;++i){if(fields[i].offset==offsetof(BrotliMetablockHeaderArena,symbol_lists)){
  if(h->symbol_lists!=h->symbols_lists_array+16)abort();U64(16);
 }else Bytes(base+fields[i].offset,fields[i].size);}
}
typedef struct {uint8_t seed;BrotliDecoderState* state;const uint8_t* input;int which,record,fail;uint8_t trace[8192];size_t trace_size,calls;} SeedContext;
static brotli_reg_t* Num(SeedContext* c){return c->which?&c->state->num_dist_htrees:&c->state->num_literal_htrees;}
static uint8_t** Map(SeedContext* c){return c->which?&c->state->dist_context_map:&c->state->context_map;}
static void* SeedAlloc(void* opaque,size_t size){SeedContext* c=opaque;void* p;
 if(c->record){BrotliDecoderState* s=c->state;BrotliMetablockHeaderArena* h=&s->arena.header;
  if(c->calls++)abort();capture=c->trace;capture_size=0;
  U64(size);U64(*Num(c));if(*Map(c)!=NULL)abort();U64(0);U64(h->context_index);
  U64(h->substate_context_map);U64(s->substate_decode_uint8);U64(s->br.val_);U64(s->br.bit_pos_);
  U64((uintptr_t)s->br.next_in-(uintptr_t)c->input);U64((uintptr_t)s->br.guard_in-(uintptr_t)c->input);U64((uintptr_t)s->br.last_in-(uintptr_t)c->input);U64(c->fail);
  HeaderBytes(h);U64(s->mtf_upper_bound);Bytes((const uint8_t*)s+offsetof(BrotliDecoderState,mtf),sizeof(s->mtf));c->trace_size=capture_size;capture=NULL;
  if(c->fail)return NULL;
 }
 p=malloc(size);if(p)memset(p,c->seed,size);return p;
}
static void SeedFree(void* opaque,void* p){(void)opaque;free(p);}
static BrotliDecoderState* NewHeader(SeedContext* c){
 const uint8_t empty[1]={0};const uint8_t* in=empty;size_t ai=0,ao=0;
 BrotliDecoderState* s=BrotliDecoderCreateInstance(SeedAlloc,SeedFree,c);if(!s)abort();c->state=s;
 s->state=BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER;
 if(BrotliDecoderDecompressStream(s,&ai,&in,&ao,NULL,NULL)!=BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT)abort();
 if(s->arena.header.substate_huffman!=BROTLI_STATE_HUFFMAN_NONE||s->arena.header.symbol_lists!=s->arena.header.symbols_lists_array+16)abort();return s;
}
static void Schema(void){size_t i;U64(sizeof(BrotliMetablockHeaderArena));U64(field_count);U64(field_bytes);
 for(i=0;i<field_count;++i){U64(fields[i].offset);U64(fields[i].size);}
 U64(sizeof(BrotliDecoderState));U64(offsetof(BrotliDecoderState,num_literal_htrees));U64(offsetof(BrotliDecoderState,context_map));U64(offsetof(BrotliDecoderState,num_dist_htrees));U64(offsetof(BrotliDecoderState,dist_context_map));U64(offsetof(BrotliDecoderState,mtf_upper_bound));U64(offsetof(BrotliDecoderState,mtf));U64(offsetof(BrotliDecoderState,substate_decode_uint8));
}
static void Observe(SeedContext* c,int result,size_t size,size_t prefix){BrotliDecoderState* s=c->state;uint8_t* map=*Map(c);
 U64((uint64_t)(int64_t)result);U64(s->br.val_);U64(s->br.bit_pos_);U64((uintptr_t)s->br.next_in-(uintptr_t)c->input);U64((uintptr_t)s->br.guard_in-(uintptr_t)c->input);U64((size_t)(s->br.last_in-s->br.next_in));
 Bytes(Num(c),sizeof(*Num(c)));U64(map?1:0);U64(map?size:0);U64(prefix);U64(c->calls);U64(c->trace_size);Bytes(c->trace,c->trace_size);
 HeaderBytes(&s->arena.header);Bytes(&s->substate_decode_uint8,sizeof(s->substate_decode_uint8));U64(s->mtf_upper_bound);Bytes((const uint8_t*)s+offsetof(BrotliDecoderState,mtf),sizeof(s->mtf));if(map)Bytes(map,size);
}
static void Run(void){unsigned size,which,mode,initial_bits,fail,steps,n,i,j;uint64_t num,prefix,code,index,bound,uint8_state,symbol,initial_val;
 while(scanf("%u %u %u %"SCNu64" %"SCNu64" %"SCNu64" %"SCNu64" %"SCNu64" %"SCNu64" %"SCNu64" %u %"SCNu64" %u %u %u",&size,&which,&mode,&num,&prefix,&code,&index,&bound,&uint8_state,&symbol,&initial_bits,&initial_val,&fail,&steps,&n)==15){
  uint8_t input[INPUT_CAPACITY];unsigned chunks[INPUT_CAPACITY];
  if(size>16384||which>1||mode>9||initial_bits>32||fail>1||steps>INPUT_CAPACITY||n>INPUT_CAPACITY||bound>63||symbol>65535)abort();
  if((mode==1||mode==2)&&(num<2||num>256||prefix>16))abort();
  if(mode==8&&(num>32||(num>7&&!fail)))abort();
  if(mode==4&&(code>32))abort();
  if((mode==3||mode==4||mode==7)&&index>size)abort();
  for(i=0;i<steps;++i)if(scanf("%u",&chunks[i])!=1)abort();for(i=0;i<n;++i){unsigned b;if(scanf("%2x",&b)!=1)abort();input[i]=(uint8_t)b;}
  for(j=0;j<2;++j){SeedContext c;BrotliDecoderState* s;BrotliMetablockHeaderArena* h;size_t off=0,known=0;
   memset(&c,0,sizeof(c));c.seed=j?0x5a:0xa5;c.which=which;c.input=input;c.fail=fail;s=NewHeader(&c);h=&s->arena.header;
   if(mode>=1&&mode<=5||mode==7){size_t k;*Map(&c)=SeedAlloc(&c,size);if(!*Map(&c))abort();h->context_index=index;*Num(&c)=num;
    if(mode==5){for(k=0;k<size;++k)(*Map(&c))[k]=(uint8_t)k;known=size;}
    else{for(k=0;k<index;++k)(*Map(&c))[k]=(uint8_t)k;known=index;}
   }
   if(mode==1)h->substate_context_map=BROTLI_STATE_CONTEXT_MAP_READ_PREFIX;
   if(mode==2){h->substate_context_map=BROTLI_STATE_CONTEXT_MAP_HUFFMAN;h->max_run_length_prefix=prefix;}
   if(mode==3||mode==4||mode==7){uint16_t values[2]={0,(uint16_t)symbol};
    h->substate_context_map=BROTLI_STATE_CONTEXT_MAP_DECODE;h->max_run_length_prefix=prefix;h->code=mode==4?code:0xffff;
    if(mode==3)values[0]=(uint16_t)symbol;BrotliBuildSimpleHuffmanTable(h->context_map_table,8,values,mode==7?1:0);
   }
   if(mode==5){h->substate_context_map=BROTLI_STATE_CONTEXT_MAP_TRANSFORM;InverseMoveToFrontTransform(NULL,0,s);s->mtf_upper_bound=bound;}
   if(mode==6)h->substate_context_map=(BrotliRunningContextMapState)UINT32_MAX;
   if(mode==8){s->substate_decode_uint8=BROTLI_STATE_DECODE_UINT8_LONG;*Num(&c)=num;}
   if(mode==9)s->substate_decode_uint8=(BrotliRunningDecodeUint8State)UINT32_MAX;
   BrotliInitBitReader(&s->br);s->br.val_=(brotli_reg_t)initial_val;s->br.bit_pos_=initial_bits;c.record=1;
   for(i=0;i<steps;++i){int result;if(chunks[i]>n-off)abort();BrotliBitReaderSetInput(&s->br,input+off,chunks[i]);
    result=(int)DecodeContextMap(size,Num(&c),Map(&c),s);
    if(*Map(&c)){
     if(result==BROTLI_DECODER_SUCCESS||(result==BROTLI_DECODER_NEEDS_MORE_INPUT&&h->substate_context_map==BROTLI_STATE_CONTEXT_MAP_TRANSFORM))known=size;
     else if(result==BROTLI_DECODER_NEEDS_MORE_INPUT&&h->substate_context_map==BROTLI_STATE_CONTEXT_MAP_DECODE){if(h->context_index>size)abort();if(h->context_index>known)known=h->context_index;}
    }
    Observe(&c,result,size,known);if(result!=BROTLI_DECODER_NEEDS_MORE_INPUT)break;if(s->br.next_in!=s->br.last_in)abort();off+=chunks[i];
   }
   c.record=0;BrotliDecoderDestroyInstance(s);
  }
 }
}
#endif
static void DecodeFile(const char* path){FILE* f=fopen(path,"rb");uint8_t* input=malloc(1u<<22),*output=malloc(1u<<24);size_t len,size=1u<<24;
 if(!f||!input||!output)abort();len=fread(input,1,1u<<22,f);if(!feof(f))abort();fclose(f);if(BrotliDecoderDecompress(len,input,&size,output)!=BROTLI_DECODER_RESULT_SUCCESS)abort();if(fwrite(output,1,size,stdout)!=size)abort();free(output);free(input);
}
static void EncodeFile(const char* path){FILE* f=fopen(path,"rb");uint8_t* input=malloc(1u<<22),*output=malloc(1u<<24);size_t len,size=1u<<24;
 if(!f||!input||!output)abort();len=fread(input,1,1u<<22,f);if(!feof(f))abort();fclose(f);if(!BrotliEncoderCompress(6,22,BROTLI_MODE_GENERIC,len,input,&size,output))abort();if(fwrite(output,1,size,stdout)!=size)abort();free(output);free(input);
}
int main(int argc,char** argv){if(argc==3&&strcmp(argv[1],"encode")==0){EncodeFile(argv[2]);return 0;}if(argc==3&&strcmp(argv[1],"decode")==0){DecodeFile(argv[2]);return 0;}
#if !defined(CONTEXT_MAP_DEFAULT)
 InitFields();if(argc==2&&strcmp(argv[1],"run-diagnostics")==0){Run();return ferror(stdin)||ferror(stdout);}if(argc==2&&strcmp(argv[1],"fixtures")==0){ContextMapFixtures();return ferror(stdout);}if(argc==2&&strcmp(argv[1],"schema")==0){Schema();return ferror(stdout);}if(argc==2&&strcmp(argv[1],"run")==0){Run();return ferror(stdin)||ferror(stdout);}
#endif
 return 2;
}
