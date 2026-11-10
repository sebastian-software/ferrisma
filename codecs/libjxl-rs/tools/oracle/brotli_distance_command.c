/* Copyright (c) the JPEG XL Project Authors. All rights reserved.
 * Use of this source code is governed by a BSD-style license in LICENSE. */
#if defined(DISTANCE_COMMAND_DEFAULT)
#include <brotli/decode.h>
#else
#include "c/dec/decode.c"
#if defined(BROTLI_HUFFMAN_CODE_FAST_LOAD)
#error Native separate-field Huffman representation required
#endif
#endif
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if !defined(DISTANCE_COMMAND_DEFAULT)
static void U64(uint64_t value) {
  unsigned i;uint8_t bytes[8];for(i=0;i<8;++i)bytes[i]=(uint8_t)(value>>(i*8));
  if(fwrite(bytes,1,8,stdout)!=8)abort();
}
static void Bytes(const void* ptr,size_t n) {if(fwrite(ptr,1,n,stdout)!=n)abort();}

/* Tables are made only by actual original source builders. Source tree
   fixtures are complete; all declared symbols fit the callee's actual LUT. */
static void Table(HuffmanCode* table,unsigned family,unsigned code,unsigned n) {
  unsigned i;uint16_t symbols[4]={(uint16_t)code,(uint16_t)((code+1)%n),0,0};
  uint16_t links[720],count[16]={0};int next[16];uint8_t lengths[704];
  for(i=0;i<4096;++i)table[i]=ConstructHuffmanCode(165,48879);
  if(family<2){BrotliBuildSimpleHuffmanTable(table,8,symbols,family);return;}
  if(family==2) {
    unsigned short_count=1024-n;
    for(i=0;i<n;++i)lengths[i]=(uint8_t)(i<short_count?9:10);
  } else {
    unsigned size=16;for(i=0;i<14;++i)lengths[i]=(uint8_t)(i+1);
    lengths[14]=lengths[15]=15;
    while(size<n){unsigned j=0;for(i=1;i<size;++i)if(lengths[i]<lengths[j])j=i;
      ++lengths[j];lengths[size++]=lengths[j];}
  }
  for(i=0;i<16;++i)next[i]=(int)i-16;
  for(i=0;i<720;++i)links[i]=65535;
  for(i=0;i<n;++i){unsigned len=lengths[i];++count[len];links[16+next[len]]=(uint16_t)i;next[len]=(int)i;}
  BrotliBuildHuffmanTable(table,8,links+16,count);
}

static void Run(void) {
  unsigned op,family,code,postfix,direct,limit,available,size,seed,pattern;
  int safe,rb_index,context,r0,r1,r2,r3;
  uint64_t val,block;
  HuffmanCode general[2][2][4096];
  Table(general[0][0],2,0,544);Table(general[0][1],3,0,544);
  Table(general[1][0],2,0,704);Table(general[1][1],3,0,704);
  while(scanf("%u %u %u %d %u %u %u %u %"SCNu64" %u %"SCNu64" %d %d %d %d %d %d %u %u",
      &op,&family,&code,&safe,&postfix,&direct,&limit,&available,&val,&size,&block,
      &rb_index,&context,&r0,&r1,&r2,&r3,&seed,&pattern)==19) {
    BrotliDecoderState* s=malloc(sizeof(*s));uint8_t input[128],map[4]={0,7,255,3};
    HuffmanCode table[4096];HuffmanCode* active;HuffmanCode* trees[256];brotli_reg_t output=3735928559u;
    int insert=(int)0x5a123456;BROTLI_BOOL ok=BROTLI_TRUE;unsigned i;
    if(!s||op>10||family>3||code>=704||postfix>63||direct>528||limit>65535||available>63||size>128||seed>255)abort();
    memset(s,(int)seed,sizeof(*s));
    for(i=0;i<size;++i)input[i]=(uint8_t)(i*73+pattern);
    BrotliInitBitReader(&s->br);s->br.val_=(brotli_reg_t)val;s->br.bit_pos_=available;
    BrotliBitReaderSetInput(&s->br,input,size);
    s->distance_postfix_bits=postfix;s->num_direct_distance_codes=direct;
    s->distance_hgroup.alphabet_size_limit=(uint16_t)limit;
    s->dist_rb_idx=rb_index;s->dist_rb[0]=r0;s->dist_rb[1]=r1;s->dist_rb[2]=r2;s->dist_rb[3]=r3;
    s->distance_code=op==0?(int)code:-1;s->distance_context=context;
    s->block_length[1]=(brotli_reg_t)block;s->block_length[2]=(brotli_reg_t)block;
    s->dist_htree_index=9;s->copy_length=(int)0x5a789abc;s->dist_context_map_slice=map;
    if(op==0){if(code>17)abort();TakeDistanceFromRingBuffer(s);}
    else if(op==1){if(code>24)abort();ok=SafeReadBits(&s->br,code,&output);}
    else if(op==2){if(code>32)abort();ok=SafeReadBits32(&s->br,code,&output);}
    else if(op==3){
      CalculateDistanceLut(s);
      Bytes(s->arena.body.dist_extra_bits,sizeof(s->arena.body.dist_extra_bits));
      Bytes(s->arena.body.dist_offset,sizeof(s->arena.body.dist_offset));
      free(s);continue;
    } else if(op==4||op==7||op==8) {
      if(code>=544)abort();if(family<2){Table(table,family,code,544);active=table;}else active=general[0][family-2];
      trees[9]=active;s->distance_hgroup.htrees=trees;
      CalculateDistanceLut(s);
      if(op==4)ok=ReadDistanceInternal(safe,s,&s->br);
      else if(op==7)ReadDistance(s,&s->br);
      else ok=SafeReadDistance(s,&s->br);
    } else if(op==5||op==9||op==10) {
      if(family<2){Table(table,family,code,704);active=table;}else active=general[1][family-2];
      s->htree_command=active;
      if(op==5)ok=ReadCommandInternal(safe,s,&s->br,&insert);
      else if(op==9)ReadCommand(s,&s->br,&insert);
      else ok=SafeReadCommand(s,&s->br,&insert);
    } else ok=CheckInputAmount(safe,&s->br);
    U64(ok);U64(output);U64(s->br.val_);U64(s->br.bit_pos_);
    U64((uint64_t)(s->br.next_in-input));U64((uint64_t)(s->br.guard_in-input));U64(BrotliBitReaderGetAvailIn(&s->br));
    U64((uint64_t)(int64_t)s->distance_code);U64((uint64_t)(int64_t)s->distance_context);
    U64((uint64_t)(int64_t)s->dist_rb_idx);U64(s->block_length[1]);U64(s->block_length[2]);
    U64((uint64_t)(int64_t)s->copy_length);U64(s->dist_htree_index);U64((uint64_t)(int64_t)insert);
    for(i=0;i<4;++i)U64((uint64_t)(int64_t)s->dist_rb[i]);
    U64(CheckInputAmount(0,&s->br));free(s);
  }
}
#endif
static void DecodeFile(const char* path) {
  FILE* f=fopen(path,"rb");uint8_t* in=malloc(1u<<22);uint8_t* out=malloc(1u<<24);
  size_t n,size=1u<<24;if(!f||!in||!out)abort();n=fread(in,1,1u<<22,f);
  if(!feof(f))abort();fclose(f);
  if(BrotliDecoderDecompress(n,in,&size,out)!=BROTLI_DECODER_RESULT_SUCCESS)abort();
  if(fwrite(out,1,size,stdout)!=size)abort();free(out);free(in);
}
int main(int argc,char**argv) {
  if(argc==3&&strcmp(argv[1],"decode")==0){DecodeFile(argv[2]);return 0;}
#if !defined(DISTANCE_COMMAND_DEFAULT)
  if(argc==2&&strcmp(argv[1],"run")==0){Run();return ferror(stdin)||ferror(stdout);}
  if(argc==2&&strcmp(argv[1],"commands")==0){
    unsigned i;for(i=0;i<704;++i){CmdLutElement v=kCmdLut[i];U64(v.insert_len_extra_bits);U64(v.copy_len_extra_bits);
      U64((uint64_t)(int64_t)v.distance_code);U64(v.context);U64(v.insert_len_offset);U64(v.copy_len_offset);}return 0;
  }
#endif
  return 2;
}
