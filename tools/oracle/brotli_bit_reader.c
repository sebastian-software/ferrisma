/* Copyright 2013 Google Inc. All Rights Reserved.
   Distributed under the MIT license in licenses/BROTLI-MIT.txt. */
/* Observe the actual pinned header routines and compiled bit_reader.c symbols. */
#include "c/dec/bit_reader.h"
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* These wrappers select the source's dynamic argument branch. Literal sites
   below pass actual literals to the same pristine header routines. */
static BROTLI_NOINLINE void DynamicFill(BrotliBitReader* br, brotli_reg_t n) {
  BrotliFillBitWindow(br, n);
}
static BROTLI_NOINLINE brotli_reg_t DynamicGet(BrotliBitReader* br, brotli_reg_t n) {
  return BrotliGetBits(br, n);
}
#define LITERAL_CASE(N, F) case N: F(br, N); break
static void LiteralFill(BrotliBitReader* br, unsigned n) {
  switch (n) {
    LITERAL_CASE(1, BrotliFillBitWindow); LITERAL_CASE(8, BrotliFillBitWindow);
    LITERAL_CASE(16, BrotliFillBitWindow); LITERAL_CASE(24, BrotliFillBitWindow);
    LITERAL_CASE(32, BrotliFillBitWindow); default: abort();
  }
}
#undef LITERAL_CASE
static brotli_reg_t LiteralGet(BrotliBitReader* br, unsigned n) {
  switch (n) {
    case 1: return BrotliGetBits(br, 1); case 8: return BrotliGetBits(br, 8);
    case 16: return BrotliGetBits(br, 16); case 24: return BrotliGetBits(br, 24);
    case 32: return BrotliGetBits(br, 32); default: abort();
  }
}
static brotli_reg_t LiteralRead24(BrotliBitReader* br, unsigned n) {
  switch (n) {
    case 1: return BrotliReadBits24(br, 1); case 8: return BrotliReadBits24(br, 8);
    case 16: return BrotliReadBits24(br, 16); case 24: return BrotliReadBits24(br, 24);
    default: abort();
  }
}
static brotli_reg_t LiteralRead32(BrotliBitReader* br, unsigned n) {
  switch (n) {
    case 1: return BrotliReadBits32(br, 1); case 8: return BrotliReadBits32(br, 8);
    case 16: return BrotliReadBits32(br, 16); case 24: return BrotliReadBits32(br, 24);
    case 32: return BrotliReadBits32(br, 32); default: abort();
  }
}

int main(void) {
  unsigned op, bits, n;
  uint64_t accumulator;
  char hex[8192];
  while (scanf("%u %" SCNu64 " %u %u %8191s", &op, &accumulator, &bits, &n, hex) == 5) {
    unsigned char input[4096], output[4104];
    size_t size = strcmp(hex, "-") == 0 ? 0 : strlen(hex)/2;
    size_t i;
    int ok = 1;
    brotli_reg_t value = 0xdeadbeef;
    BrotliBitReader br;
    if (strlen(hex) % 2 && strcmp(hex, "-") != 0) return 2;
    for (i=0;i<size;++i) {
      char pair[3] = {hex[2*i],hex[2*i+1],0};
      input[i] = (unsigned char)strtoul(pair, NULL, 16);
    }
    memset(output, 0xcc, sizeof(output));
    BrotliInitBitReader(&br);
    BrotliBitReaderSetInput(&br, input, size);
    br.val_ = (brotli_reg_t)accumulator;
    br.bit_pos_ = bits;
    switch (op) {
      case 0: ok = BrotliWarmupBitReader(&br); break;
      case 1: ok = BrotliPullByte(&br); break;
      case 2: ok = BrotliSafeGetBits(&br, n, &value); break;
      case 3: ok = BrotliSafeReadBits(&br, n, &value); break;
      case 4: ok = BrotliSafeReadBits32(&br, n, &value); break;
      case 5: ok = BrotliSafeReadBits32Slow(&br, n, &value); break;
      case 6: DynamicFill(&br, n); break;
      case 7: LiteralFill(&br, n); break;
      case 8: value = BrotliGet16BitsUnmasked(&br); break;
      case 9: value = DynamicGet(&br, n); break;
      case 10: value = LiteralGet(&br, n); break;
      case 11: BrotliBitReaderNormalize(&br); break;
      case 12:
        ok = BrotliSafeGetBits(&br, n, &value);
        BrotliBitReaderUnload(&br); break;
      case 13: ok = BrotliJumpToByteBoundary(&br); break;
      case 14: BrotliCopyBytes(output, &br, n); break;
      case 15: {
        BrotliBitReaderState saved;
        BrotliBitReaderSaveState(&br, &saved);
        ok = BrotliSafeReadBits32(&br, n, &value);
        BrotliBitReaderRestoreState(&br, &saved);
        break;
      }
      case 16: value = BrotliReadBits24(&br, n); break;
      case 17: value = BrotliReadBits32(&br, n); break;
      case 18: BrotliTakeBits(&br, n, &value); break;
      case 19: BrotliDropBits(&br, n); break;
      case 20: BrotliFillBitWindow16(&br); break;
      case 21: {
        BrotliBitReaderState saved;
        BrotliSafeReadBits(&br, 3, &value);
        BrotliBitReaderSaveState(&br, &saved);
        ok = BrotliSafeReadBits32(&br, n, &value);
        BrotliBitReaderRestoreState(&br, &saved);
        break;
      }
      case 23: value = LiteralRead24(&br, n); break;
      case 24: value = LiteralRead32(&br, n); break;
      default: abort();
    }
    printf("%d %" PRIu64 " %" PRIu64 " %" PRIu64 " %zu %zu %zu %zu %d", ok,
        (uint64_t)value, (uint64_t)br.val_, (uint64_t)br.bit_pos_,
        (size_t)(br.next_in-input), (size_t)(br.guard_in-input),
        BrotliBitReaderGetAvailIn(&br), BrotliGetRemainingBytes(&br), BrotliCheckInputAmount(&br));
    if (op==14) { putchar(' '); for (i=0;i<n+8;++i) printf("%02x",output[i]); }
    putchar('\n');
  }
  return ferror(stdin) ? 3 : 0;
}
