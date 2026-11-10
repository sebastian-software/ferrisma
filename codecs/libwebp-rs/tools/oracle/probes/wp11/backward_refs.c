// WP11 probe: hash chains, backward references and the histogram image
// (src/enc/backward_references_enc.c, backward_references_cost_enc.c and
// histogram_enc.c; ADR 0002). Commands on standard input, one per line:
//
//   refs W H QUALITY LOW_EFFORT LZ77 CACHE_BITS DO_NO_CACHE HISTO_BITS ARGB
//
// runs VP8LHashChainFill and VP8LGetBackwardReferences on the W by H pixels
// ARGB, then VP8LGetHistoImageSymbols on the best references with HISTO_BITS
// and the chosen cache bits. It prints the hash chain, the return values, the
// error code, the chosen cache bits, the best references (and the best ones
// without a cache when DO_NO_CACHE is set) as mode, length and value per entry,
// and the histogram image: its size, the symbols and the cost fields of every
// histogram.
//
// tests/wp11_backward_refs.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/lossless.h"
#include "src/dsp/lossless_common.h"
#include "src/enc/backward_references_enc.h"
#include "src/enc/histogram_enc.h"
#include "src/enc/vp8li_enc.h"
#include "wp11_io.h"

static void PrintRefs(const char* key, const VP8LBackwardRefs* const refs) {
  VP8LRefsCursor c = VP8LRefsCursorInit(refs);
  int count = 0;
  printf("%s=", key);
  while (VP8LRefsCursorOk(&c)) {
    printf("%02x%04x%08x", c.cur_pos->mode, c.cur_pos->len,
           c.cur_pos->argb_or_distance);
    ++count;
    VP8LRefsCursorNext(&c);
  }
  if (count == 0) printf("-");
  printf("\n");
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "backward_refs < commands.txt");
  (void)argv;
  VP8LEncDspInit();
  while ((command = Wp11ReadToken()) != NULL) {
    if (strcmp(command, "refs") == 0) {
      const int width = (int)Wp11ReadLong();
      const int height = (int)Wp11ReadLong();
      const int quality = (int)Wp11ReadLong();
      const int low_effort = (int)Wp11ReadLong();
      const int lz77 = (int)Wp11ReadLong();
      const int cache_bits = (int)Wp11ReadLong();
      const int do_no_cache = (int)Wp11ReadLong();
      const int histo_bits = (int)Wp11ReadLong();
      size_t count;
      uint32_t* const argb = Wp11ReadU32s(&count);
      const int size = width * height;
      VP8LHashChain chain;
      VP8LBackwardRefs refs[3];
      WebPPicture pic;
      int cache_bits_best = -1;
      int percent = 0;
      int ret, i;
      if ((int)count != size) return 2;
      if (!WebPPictureInit(&pic)) return 2;
      memset(&chain, 0, sizeof(chain));
      if (!VP8LHashChainInit(&chain, size)) return 2;
      for (i = 0; i < 3; ++i) {
        VP8LBackwardRefsInit(&refs[i], (size - 1) / MAX_REFS_BLOCK_PER_IMAGE + 1);
      }
      ret = VP8LHashChainFill(&chain, quality, argb, width, height, low_effort,
                              &pic, 30, &percent);
      printf("fill=%d\n", ret);
      Wp11PrintU32s("chain", chain.offset_length, (size_t)size);
      ret = VP8LGetBackwardReferences(width, height, argb, quality, low_effort,
                                      lz77, cache_bits, do_no_cache, &chain,
                                      refs, &cache_bits_best, &pic, 30,
                                      &percent);
      printf("refs_ret=%d\nerror=%d\ncache_bits=%d\npercent=%d\n", ret,
             (int)pic.error_code, cache_bits_best, percent);
      PrintRefs("refs0", &refs[0]);
      if (do_no_cache) PrintRefs("refs1", &refs[1]);
      {
        const int histo_xsize = VP8LSubSampleSize(width, histo_bits);
        const int histo_ysize = VP8LSubSampleSize(height, histo_bits);
        const int xysize = histo_xsize * histo_ysize;
        VP8LHistogramSet* const set =
            VP8LAllocateHistogramSet(xysize, cache_bits_best);
        VP8LHistogram* const tmp = VP8LAllocateHistogram(cache_bits_best);
        uint32_t* const symbols =
            (uint32_t*)calloc((size_t)xysize + 1, sizeof(*symbols));
        uint64_t costs[6];
        if (set == NULL || tmp == NULL || symbols == NULL) return 2;
        ret = VP8LGetHistoImageSymbols(width, height, &refs[0], quality,
                                       low_effort, histo_bits, cache_bits_best,
                                       set, tmp, symbols, &pic, 30, &percent);
        printf("histo_ret=%d\nhisto_size=%d\n", ret, set->size);
        Wp11PrintU32s("symbols", symbols, (size_t)xysize);
        for (i = 0; i < set->size; ++i) {
          const VP8LHistogram* const h = set->histograms[i];
          int k;
          costs[0] = h->bit_cost;
          for (k = 0; k < 5; ++k) costs[k + 1] = h->costs[k];
          Wp11PrintU64s("costs", costs, 6);
          printf("estimate=%016llx\n",
                 (unsigned long long)VP8LHistogramEstimateBits(h));
        }
        VP8LFreeHistogramSet(set);
        VP8LFreeHistogram(tmp);
        free(symbols);
      }
      for (i = 0; i < 3; ++i) VP8LBackwardRefsClear(&refs[i]);
      VP8LHashChainClear(&chain);
      free(argb);
    } else {
      fprintf(stderr, "backward_refs: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  return 0;
}
