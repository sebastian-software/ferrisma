// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
/* Calls the complete actual private helpers from one pristine decoder include. */
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "c/dec/decode.c"

typedef struct Root Root;
typedef struct {Root* root;int id;} Context;
struct Root {BrotliDecoderState* s;const uint8_t* input;int mode,calls;Context context[2];};
static void Chunk1(void*,const uint8_t*,size_t);
static void Chunk(int which,void* opaque,const uint8_t* data,size_t n) {
  Context* context=(Context*)opaque;Root* root=context->root;
  BrotliDecoderState* s=root->s;size_t i;
  printf("C%d:%d:%zu:%d:%"PRIu64":%"PRIu64":%zu:%d:",which,context->id,n,
      s->meta_block_remaining_len,(uint64_t)s->br.val_,(uint64_t)s->br.bit_pos_,
      (size_t)(s->br.next_in-root->input),data==s->br.next_in);
  for(i=0;i<n;++i)printf("%02x",data[i]);printf(" ");
  if(root->calls++==0) {
    if(root->mode==2)BrotliDecoderSetMetadataCallbacks(s,NULL,Chunk1,&root->context[1]);
    if(root->mode==3)BrotliDecoderSetMetadataCallbacks(s,NULL,NULL,&root->context[1]);
  }
}
static void Chunk0(void* opaque,const uint8_t* data,size_t n){Chunk(0,opaque,data,n);}
static void Chunk1(void* opaque,const uint8_t* data,size_t n){Chunk(1,opaque,data,n);}
int main(void) {
  unsigned op,mode,remaining,bits,n,capacity,position;
  while(scanf("%u %u %u %u %u %u %u",&op,&mode,&remaining,&bits,&n,&capacity,&position)==7) {
    uint8_t input[32],output[2048];unsigned i;size_t ao=capacity,total=1234;
    uint8_t* next_out=output;BrotliDecoderState* s;BrotliDecoderErrorCode result;Root root;
    if(op>4||mode>3||remaining>2048||bits>64||(bits&7)||n>32||capacity>2048||position>1024)abort();
    for(i=0;i<32;++i)input[i]=(uint8_t)(0x80+i);memset(output,0xcc,sizeof(output));
    s=BrotliDecoderCreateInstance(NULL,NULL,NULL);if(!s)abort();
    if(op==4) {
      BrotliDecoderStateMetablockBegin(s);
      uint8_t* expected;size_t payload=20;uint8_t* destination=output;
      s->window_bits=10;s->new_ringbuffer_size=1024;
      if(!BrotliEnsureRingBuffer(s)||sizeof(size_t)!=8||sizeof(void*)!=8)abort();
      for(i=0;i<8;++i)s->ringbuffer[i]=(uint8_t)(i+1);s->pos=8;
      ao=mode==0?3:8;total=1234;
      if(mode==0) {next_out=output;expected=output+3;}
      else if(mode==1) {memcpy(s->ringbuffer,&payload,8);next_out=(uint8_t*)&ao;expected=(uint8_t*)&ao+8;}
      else if(mode==2) {memcpy(s->ringbuffer,&payload,8);next_out=(uint8_t*)&total;expected=(uint8_t*)&total+8;}
      else {memcpy(s->ringbuffer,&destination,8);next_out=(uint8_t*)&next_out;expected=output+8;}
      result=WriteRingBuffer(s,&ao,&next_out,mode==0?&ao:&total,BROTLI_FALSE);
      printf("A%u R%d AO%zu T%zu P%d S%zu POS%d O",mode,(int)result,ao,mode==0?ao:total,next_out==expected,s->partial_pos_out,s->pos);
      for(i=0;i<8;++i)printf("%02x",output[i]);printf("\n");
      BrotliDecoderDestroyInstance(s);continue;
    }
    if(op==2) {
      brotli_reg_t value=99;int first,second;
      BrotliBitReaderSetInput(&s->br,NULL,0);
      first=BrotliSafeReadBits(&s->br,0,&value);
      second=BrotliSafeReadBits(&s->br,1,&value);
      printf("N%d %d %"PRIu64" %"PRIu64" %"PRIu64" %d %d %d\n",first,second,
          (uint64_t)value,(uint64_t)s->br.val_,(uint64_t)s->br.bit_pos_,
          s->br.next_in==NULL,s->br.guard_in==NULL,s->br.last_in==NULL);
      BrotliDecoderDestroyInstance(s);continue;
    }
    if(op==3) {
      brotli_reg_t first=99,second=99;
      input[0]=1;BrotliBitReaderSetInput(&s->br,input,6);
      if(!BrotliSafeReadBits(&s->br,8,&first))abort();input[1]=91;
      if(!BrotliSafeReadBits(&s->br,8,&second))abort();
      printf("M%"PRIu64" %"PRIu64" %"PRIu64" %"PRIu64" %zu %zu %zu\n",
          (uint64_t)first,(uint64_t)second,(uint64_t)s->br.val_,(uint64_t)s->br.bit_pos_,
          (size_t)(s->br.next_in-input),(size_t)(s->br.guard_in-input),(size_t)(s->br.last_in-input));
      BrotliDecoderDestroyInstance(s);continue;
    }
    BrotliBitReaderSetInput(&s->br,input,n);s->br.val_=(brotli_reg_t)(UINT64_C(0x0807060504030201)&(bits==64?UINT64_MAX:((UINT64_C(1)<<bits)-1)));s->br.bit_pos_=bits;
    s->meta_block_remaining_len=(int)remaining;
    memset(&root,0,sizeof(root));root.s=s;root.input=input;root.mode=mode;
    root.context[0].root=&root;root.context[1].root=&root;root.context[1].id=1;
    if(mode)BrotliDecoderSetMetadataCallbacks(s,NULL,Chunk0,&root.context[0]);
    if(op==0)result=SkipMetadataBlock(s);
    else {
      s->window_bits=10;s->max_backward_distance=1008;s->new_ringbuffer_size=1024;
      if(!BrotliEnsureRingBuffer(s))abort();
      memset(s->ringbuffer,0x33,position);s->pos=(int)position;
      result=CopyUncompressedBlockToOutput(&ao,&next_out,&total,s);
    }
    printf("R%d %d %"PRIu64" %"PRIu64" %zu %zu %zu",(int)result,s->meta_block_remaining_len,
        (uint64_t)s->br.val_,(uint64_t)s->br.bit_pos_,(size_t)(s->br.next_in-input),
        (size_t)(s->br.guard_in-input),(size_t)(s->br.last_in-input));
    if(op==1) {
      printf(" %d %zu %zu %zu %u ",s->pos,s->partial_pos_out,ao,total,(unsigned)s->substate_uncompressed);
      for(i=0;i<32;++i)printf("%02x",output[i]);
      printf(" ");for(i=0;i<(unsigned)s->pos;++i)printf("%02x",s->ringbuffer[i]);
    }
    printf("\n");BrotliDecoderDestroyInstance(s);
  }
  return ferror(stdin)||ferror(stdout);
}
