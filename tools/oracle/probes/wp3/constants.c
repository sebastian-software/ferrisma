// WP3 probe: the constants, enumerators and C type sizes of the WP3 headers
// (ADR 0002, ADR 0003). The sizes are the `size` arguments of the allocations
// of these types. Doubles print as their 64-bit patterns. It prints one
// name=value line per item; tests/wp3_constants.rs compares them.

#include <inttypes.h>
#include <stdio.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/lossless.h"
#include "src/dsp/lossless_common.h"
#include "src/utils/color_cache_utils.h"
#include "src/utils/huffman_encode_utils.h"
#include "src/utils/huffman_utils.h"
#include "src/utils/palette.h"

static void PrintDouble(const char* name, double value) {
  uint64_t bits;
  memcpy(&bits, &value, sizeof(bits));
  printf("%s=%" PRIu64 "\n", name, bits);
}

int main(int argc, char* argv[]) {
  ProbeCheckArgs(argc, 0, 0, "constants");
  printf("sizeof_HuffmanCode=%zu\n", sizeof(HuffmanCode));
  printf("sizeof_HuffmanTablesSegment=%zu\n", sizeof(HuffmanTablesSegment));
  printf("sizeof_HTreeGroup=%zu\n", sizeof(HTreeGroup));
  printf("sizeof_HuffmanTreeToken=%zu\n", sizeof(HuffmanTreeToken));
  printf("sizeof_HuffmanTreeCode=%zu\n", sizeof(HuffmanTreeCode));
  printf("sizeof_HuffmanTree=%zu\n", sizeof(HuffmanTree));
  printf("HUFFMAN_TABLE_BITS=%d\n", HUFFMAN_TABLE_BITS);
  printf("HUFFMAN_TABLE_MASK=%d\n", HUFFMAN_TABLE_MASK);
  printf("LENGTHS_TABLE_BITS=%d\n", LENGTHS_TABLE_BITS);
  printf("LENGTHS_TABLE_MASK=%d\n", LENGTHS_TABLE_MASK);
  printf("HUFFMAN_PACKED_BITS=%d\n", HUFFMAN_PACKED_BITS);
  printf("HUFFMAN_PACKED_TABLE_SIZE=%u\n", HUFFMAN_PACKED_TABLE_SIZE);
  printf("kHashMul=%u\n", kHashMul);
  printf("kSortedDefault=%d\n", (int)kSortedDefault);
  printf("kMinimizeDelta=%d\n", (int)kMinimizeDelta);
  printf("kModifiedZeng=%d\n", (int)kModifiedZeng);
  printf("kUnusedPalette=%d\n", (int)kUnusedPalette);
  printf("kPaletteSortingNum=%d\n", (int)kPaletteSortingNum);
  printf("APPROX_LOG_WITH_CORRECTION_MAX=%d\n", APPROX_LOG_WITH_CORRECTION_MAX);
  printf("APPROX_LOG_MAX=%d\n", APPROX_LOG_MAX);
  printf("LOG_2_PRECISION_BITS=%d\n", LOG_2_PRECISION_BITS);
  PrintDouble("LOG_2_RECIPROCAL", LOG_2_RECIPROCAL);
  PrintDouble("LOG_2_RECIPROCAL_FIXED_DOUBLE", LOG_2_RECIPROCAL_FIXED_DOUBLE);
  printf("LOG_2_RECIPROCAL_FIXED=%" PRIu64 "\n", (uint64_t)LOG_2_RECIPROCAL_FIXED);
  printf("LOG_LOOKUP_IDX_MAX=%d\n", LOG_LOOKUP_IDX_MAX);
  printf("WEBP_INT64_MAX=%" PRId64 "\n", (int64_t)WEBP_INT64_MAX);
  printf("WEBP_UINT64_MAX=%" PRIu64 "\n", (uint64_t)WEBP_UINT64_MAX);
  printf("PREFIX_LOOKUP_IDX_MAX=%d\n", PREFIX_LOOKUP_IDX_MAX);
  return 0;
}
