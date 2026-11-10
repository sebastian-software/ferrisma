// WP11 probe: the constants, enumerators and C type sizes of the lossless
// encoder (ADR 0002, ADR 0003). The sizes are the `size` arguments of the
// allocations of these types. The types that src/enc/*.c defines privately are
// repeated here from the pinned source, unchanged, so that the compiler of the
// oracle lays them out. It prints one name=value line per item;
// tests/wp11_constants.rs compares them.

#include <stdio.h>

#include "probe_support.h"
#include "src/dsp/lossless.h"
#include "src/dsp/lossless_common.h"
#include "src/enc/backward_references_enc.h"
#include "src/enc/histogram_enc.h"
#include "src/enc/vp8li_enc.h"

// From src/enc/backward_references_enc.c.
struct PixOrCopyBlock {
  PixOrCopyBlock* next;   // next block (or NULL)
  PixOrCopy* start;       // data start
  int size;               // currently used size
};

// From src/enc/histogram_enc.c.
typedef struct {
  int idx1;
  int idx2;
  int64_t cost_diff;
  uint64_t cost_combo;
  uint64_t costs[5];
} HistogramPair;

// From src/enc/backward_references_cost_enc.c.
#define VALUES_IN_BYTE 256
typedef struct {
  uint32_t alpha[VALUES_IN_BYTE];
  uint32_t red[VALUES_IN_BYTE];
  uint32_t blue[VALUES_IN_BYTE];
  uint32_t distance[NUM_DISTANCE_CODES];
  uint32_t* literal;
} CostModel;

typedef struct CostInterval CostInterval;
struct CostInterval {
  int64_t cost;
  int start;
  int end;
  int index;
  CostInterval* previous;
  CostInterval* next;
};

typedef struct {
  int64_t cost;
  int start;
  int end;       // Exclusive.
} CostCacheInterval;

// From src/enc/vp8l_enc.c.
#define NUM_BUCKETS 256
typedef uint32_t HistogramBuckets[NUM_BUCKETS];
typedef struct {
  HistogramBuckets category[13];
} Histograms;

int main(int argc, char* argv[]) {
  ProbeCheckArgs(argc, 0, 0, "constants");
  (void)argv;
  printf("sizeof_VP8LHistogram=%zu\n", sizeof(VP8LHistogram));
  printf("sizeof_VP8LHistogramSet=%zu\n", sizeof(VP8LHistogramSet));
  printf("sizeof_PixOrCopy=%zu\n", sizeof(PixOrCopy));
  printf("sizeof_PixOrCopyBlock=%zu\n", sizeof(struct PixOrCopyBlock));
  printf("sizeof_HistogramPair=%zu\n", sizeof(HistogramPair));
  printf("sizeof_CostModel=%zu\n", sizeof(CostModel));
  printf("sizeof_CostInterval=%zu\n", sizeof(CostInterval));
  printf("sizeof_CostCacheInterval=%zu\n", sizeof(CostCacheInterval));
  printf("sizeof_Histograms=%zu\n", sizeof(Histograms));
  printf("MAX_COLOR_CACHE_BITS=%d\n", MAX_COLOR_CACHE_BITS);
  printf("HASH_BITS=%d\n", HASH_BITS);
  printf("HASH_SIZE=%d\n", HASH_SIZE);
  printf("MAX_LENGTH_BITS=%d\n", MAX_LENGTH_BITS);
  printf("WINDOW_SIZE_BITS=%d\n", WINDOW_SIZE_BITS);
  printf("MAX_LENGTH=%d\n", MAX_LENGTH);
  printf("MAX_REFS_BLOCK_PER_IMAGE=%d\n", MAX_REFS_BLOCK_PER_IMAGE);
  printf("kLiteral=%d\nkCacheIdx=%d\nkCopy=%d\nkNone=%d\n", (int)kLiteral,
         (int)kCacheIdx, (int)kCopy, (int)kNone);
  printf("kLZ77Standard=%d\nkLZ77RLE=%d\nkLZ77Box=%d\n", (int)kLZ77Standard,
         (int)kLZ77RLE, (int)kLZ77Box);
  printf("VP8L_NON_TRIVIAL_SYM=%d\n", (int)VP8L_NON_TRIVIAL_SYM);
  printf("MAX_TRANSFORM_BITS=%d\n", MAX_TRANSFORM_BITS);
  printf("WEBP_NEAR_LOSSLESS=%d\n", WEBP_NEAR_LOSSLESS);
  printf("kEncoderNone=%d\nkEncoderARGB=%d\nkEncoderNearLossless=%d\n"
         "kEncoderPalette=%d\n",
         (int)kEncoderNone, (int)kEncoderARGB, (int)kEncoderNearLossless,
         (int)kEncoderPalette);
  printf("PREFIX_LOOKUP_IDX_MAX=%d\n", PREFIX_LOOKUP_IDX_MAX);
  return 0;
}
