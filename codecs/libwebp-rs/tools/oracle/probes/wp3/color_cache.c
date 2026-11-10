// WP3 probe: the color cache of src/utils/color_cache_utils.c and .h
// (ADR 0002). Two caches, 0 and 1, start cleared. It reads commands from
// standard input and prints what the library computes:
//
//   init C BITS        VP8LColorCacheInit; prints init=0|1, shift= and bits=
//   insert C ARGB      VP8LColorCacheInsert
//   set C KEY ARGB     VP8LColorCacheSet
//   lookup C KEY       VP8LColorCacheLookup; prints lookup=
//   index C ARGB       VP8LColorCacheGetIndex; prints index=
//   contains C ARGB    VP8LColorCacheContains; prints contains=
//   copy FROM TO       VP8LColorCacheCopy
//   dump C             prints colors= (the 1 << hash_bits colors)
//   clear C            VP8LColorCacheClear; prints null=0|1
//   hash ARGB SHIFT    VP8LHashPix; prints hash=
//
// Values are decimal. tests/wp3_color_cache.rs generates the commands and the
// expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/color_cache_utils.h"
#include "wp3_io.h"

static uint32_t ReadU32(void) { return (uint32_t)Wp3ReadLong(); }

int main(int argc, char* argv[]) {
  VP8LColorCache caches[2];
  char* command;
  ProbeCheckArgs(argc, 0, 0, "color_cache < commands.txt");
  memset(caches, 0, sizeof(caches));
  while ((command = Wp3ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      const int bits = (int)Wp3ReadLong();
      VP8LColorCacheClear(cc);
      printf("init=%d\n", VP8LColorCacheInit(cc, bits));
      printf("shift=%d\nbits=%d\n", cc->hash_shift, cc->hash_bits);
    } else if (strcmp(command, "insert") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      VP8LColorCacheInsert(cc, ReadU32());
    } else if (strcmp(command, "set") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      const uint32_t key = ReadU32();
      VP8LColorCacheSet(cc, key, ReadU32());
    } else if (strcmp(command, "lookup") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      printf("lookup=%u\n", VP8LColorCacheLookup(cc, ReadU32()));
    } else if (strcmp(command, "index") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      printf("index=%d\n", VP8LColorCacheGetIndex(cc, ReadU32()));
    } else if (strcmp(command, "contains") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      printf("contains=%d\n", VP8LColorCacheContains(cc, ReadU32()));
    } else if (strcmp(command, "copy") == 0) {
      VP8LColorCache* const src = &caches[Wp3ReadLong()];
      VP8LColorCacheCopy(src, &caches[Wp3ReadLong()]);
    } else if (strcmp(command, "dump") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      Wp3PrintU32s("colors", cc->colors, (size_t)1 << cc->hash_bits);
    } else if (strcmp(command, "clear") == 0) {
      VP8LColorCache* const cc = &caches[Wp3ReadLong()];
      VP8LColorCacheClear(cc);
      printf("null=%d\n", cc->colors == NULL);
    } else if (strcmp(command, "hash") == 0) {
      const uint32_t argb = ReadU32();
      printf("hash=%d\n", VP8LHashPix(argb, (int)Wp3ReadLong()));
    } else {
      fprintf(stderr, "color_cache: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  VP8LColorCacheClear(&caches[0]);
  VP8LColorCacheClear(&caches[1]);
  return 0;
}
