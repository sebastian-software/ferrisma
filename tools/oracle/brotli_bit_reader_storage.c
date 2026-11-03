// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
/* Original reader fields survive a struct copy without SetInput. Every
   operation below calls the pristine pinned header / compiled source body. */
#include "c/dec/bit_reader.h"
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

static void Observe(const BrotliBitReader* br, const uint8_t* anchor) {
  printf("%" PRIu64 " %" PRIu64 " %zu %zu %zu",
      (uint64_t)br->val_, (uint64_t)br->bit_pos_,
      (size_t)(br->next_in-anchor), (size_t)(br->guard_in-anchor),
      (size_t)(br->last_in-anchor));
}
int main(void) {
  unsigned skip, width;
  while (scanf("%u %u", &skip, &width)==2) {
    uint8_t input[32]; unsigned i; brotli_reg_t value=0xdeadbeef;
    BrotliBitReader original, resumed; int first, second;
    if(skip>32||width>32) abort();
    for(i=0;i<32;++i) input[i]=(uint8_t)i;
    BrotliInitBitReader(&original);
    BrotliBitReaderSetInput(&original,input,sizeof(input));
    BrotliDropBytes(&original,skip);
    first=BrotliSafeReadBits(&original,width,&value);
    printf("%d %" PRIu64 " ",first,(uint64_t)value);Observe(&original,input);
    /* C copies all initialized reader fields. The Rust lifetime loan is the
       representation counterpart; no source reset/SetInput checkpoint exists. */
    resumed=original;
    value=0xdeadbeef;
    second=BrotliSafeReadBits32Slow(&resumed,32,&value);
    printf(" %d %" PRIu64 " ",second,(uint64_t)value);Observe(&resumed,input);
    printf("\n");
  }
  return ferror(stdin)||ferror(stdout);
}
