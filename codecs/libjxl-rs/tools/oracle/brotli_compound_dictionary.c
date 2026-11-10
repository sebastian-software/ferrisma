// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

/* Source-only caller-domain observer. Includes the pinned decoder exactly once;
   no decoder/compound helper body is copied or modified. */
#include <stdio.h>
#include <stdlib.h>
#include "c/dec/decode.c"
static void Print(const uint8_t* data, int n) { int i; for(i=0;i<n;i++) printf("%02x",data[i]); }
int main(void) {
  int mode;
  for(mode=0;mode<3;++mode) {
    uint8_t* prefix=(uint8_t*)malloc(6);
    BrotliDecoderState* s=BrotliDecoderCreateInstance(NULL,NULL,NULL);
    int i,attached,initialized,copied;
    if(!s||!prefix) abort();
    if(mode!=1) for(i=0;i<6;++i) prefix[i]=(uint8_t)(i+1);
    /* mode1: caller memory is accessible but still uninitialized at attach. */
    attached=BrotliDecoderAttachDictionary(s,BROTLI_SHARED_DICTIONARY_RAW,6,prefix);
    if(mode==1) for(i=0;i<6;++i) prefix[i]=(uint8_t)(i+11);
    if(mode==0) { prefix[0]=91;prefix[5]=66; }
    s->new_ringbuffer_size=1024;s->window_bits=10;
    if(!BrotliEnsureRingBuffer(s)) abort();
    s->distance_code=20;s->meta_block_remaining_len=64;
    initialized=InitializeCompoundDictionaryCopy(s,0,6);
    copied=CopyFromCompoundDictionary(s,0);
    printf("M%d A%d I%d C%d P%d:%d ",mode,attached,initialized,copied,s->dictionary->prefix[0]==prefix,s->compound_dictionary->chunks[0]==prefix);
    Print(s->ringbuffer,6);
    if(mode==2) {
      /* The original retains caller storage, not a cached copy. */
      prefix[0]=91;prefix[5]=66;
      initialized=InitializeCompoundDictionaryCopy(s,0,6);
      copied=CopyFromCompoundDictionary(s,6);
      printf(" I%d C%d ",initialized,copied);Print(s->ringbuffer+6,6);
    }
    puts("");BrotliDecoderDestroyInstance(s);free(prefix);
  }
  {
    BrotliDecoderState* s=BrotliDecoderCreateInstance(NULL,NULL,NULL);
    int attached;
    if(!s) abort();
    /* Attach stores NULL/size without dereferencing either prefix. */
    attached=BrotliDecoderAttachDictionary(s,BROTLI_SHARED_DICTIONARY_RAW,6,NULL);
    printf("M3 A%d S%d P%d:%d\n",attached,GetCompoundDictionarySize(s),
        s->dictionary->prefix[0]==NULL,s->compound_dictionary->chunks[0]==NULL);
    BrotliDecoderDestroyInstance(s);
  }
  return 0;
}
