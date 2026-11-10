// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
/* Dedicated fixture visibility only: invoke actual pristine encoder bodies. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include "enc/brotli_bit_stream.c"
static void Word(uint64_t x){unsigned i;for(i=0;i<8;++i)if(putchar((int)((x>>(8*i))&255))==EOF)abort();}
static void Emit(size_t size,size_t clusters,size_t bits,const uint8_t* data,const uint32_t* map){
  size_t i,n=(bits+7)/8;Word(size);Word(clusters);Word(bits);Word(n);
  if(fwrite(data,1,n,stdout)!=n)abort();for(i=0;i<size;++i)if(putchar((int)map[i])==EOF)abort();
}
void ContextMapFixtures(void){
  static const unsigned sizes[7]={1,2,4,16,64,256,1024};
  static const unsigned clusters[9]={1,2,3,4,8,16,64,128,256};
  static const unsigned types[6]={1,2,3,8,64,256};
  MemoryManager memory;unsigned a,b,style;Word(7*9*4+6*2+1);
  BrotliInitMemoryManager(&memory,NULL,NULL,NULL);
  {
    EncodeContextMapArena arena;HuffmanTree tree[2*BROTLI_MAX_CONTEXT_MAP_SYMBOLS+1];
    uint8_t storage[65536]={0};uint32_t map[1]={0};size_t bits=0;
    EncodeContextMap(&memory,&arena,map,0,1,tree,&bits,storage);Emit(0,1,bits,storage,map);
  }
  for(a=0;a<7;++a)for(b=0;b<9;++b)for(style=0;style<4;++style){
    EncodeContextMapArena arena;HuffmanTree tree[2*BROTLI_MAX_CONTEXT_MAP_SYMBOLS+1];
    uint8_t storage[65536]={0};uint32_t map[1024];size_t i,bits=0,n=sizes[a],c=clusters[b];
    for(i=0;i<n;++i)map[i]=style==0?0:style==1?(uint32_t)(i%c):style==2?(uint32_t)((i/13)%c):(uint32_t)((i*31337+i/7)%c);
    EncodeContextMap(&memory,&arena,map,n,c,tree,&bits,storage);if(bits+64>sizeof(storage)*8)abort();
    Emit(n,c,bits,storage,map);
  }
  for(a=0;a<6;++a)for(b=0;b<2;++b){
    EncodeContextMapArena arena;HuffmanTree tree[2*BROTLI_MAX_CONTEXT_MAP_SYMBOLS+1];
    uint8_t storage[65536]={0};uint32_t map[16384];size_t i,bits=0,c=types[a],cb=b?6:2,n=c*((size_t)1<<cb);
    for(i=0;i<n;++i)map[i]=(uint32_t)(i>>cb);
    StoreTrivialContextMap(&arena,c,cb,tree,&bits,storage);if(bits+64>sizeof(storage)*8)abort();Emit(n,c,bits,storage,map);
  }
  BrotliWipeOutMemoryManager(&memory);
}
