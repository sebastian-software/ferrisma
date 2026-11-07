/* WP1 probe: the C sizes that the port's memory accounting uses (ADR 0005,
 * "Exact accounting") and the results of jutils.c.
 *
 * Usage: sizes
 * Prints `sizeof_<type>=<bytes>` for the public structs and pointer types
 * whose size an allocation request names, ALIGN_SIZE as jmemmgr.c defines
 * it, MAX_ALLOC_CHUNK, the jpeg_natural_order table, jdiv_round_up and
 * jround_up for a set of operands, and the effect of jcopy_sample_rows,
 * jcopy_block_row and jzero_far on small arrays. tests/wp1_utils.rs compares
 * them with the Rust constants and functions.
 */

#define JPEG_INTERNALS
#include "probe_support.h"
#include "jmemsys.h"

#include <string.h>

#define ALIGN_SIZE  MAX(sizeof(void *), sizeof(double))

static const long operands[][2] = {
  { 0, 1 }, { 1, 1 }, { 1, 8 }, { 7, 8 }, { 8, 8 }, { 9, 8 }, { 227, 16 },
  { 149, 16 }, { 65500, 8 }, { 65500, 7 }, { 1000000000L, 3 },
  { 4294967295L, 2 }, { 123456789012L, 1000 }
};

int main(void)
{
  JSAMPLE storage[4][16];
  JSAMPROW rows[4];
  JBLOCK blocks[2][3];
  size_t i;

  printf("sizeof_JQUANT_TBL=%lu\n", (unsigned long)sizeof(JQUANT_TBL));
  printf("sizeof_JHUFF_TBL=%lu\n", (unsigned long)sizeof(JHUFF_TBL));
  printf("sizeof_jpeg_component_info=%lu\n",
         (unsigned long)sizeof(jpeg_component_info));
  printf("sizeof_jpeg_error_mgr=%lu\n",
         (unsigned long)sizeof(struct jpeg_error_mgr));
  printf("sizeof_JBLOCK=%lu\n", (unsigned long)sizeof(JBLOCK));
  printf("sizeof_JSAMPROW=%lu\n", (unsigned long)sizeof(JSAMPROW));
  printf("sizeof_JBLOCKROW=%lu\n", (unsigned long)sizeof(JBLOCKROW));
  printf("ALIGN_SIZE=%lu\n", (unsigned long)ALIGN_SIZE);
  printf("MAX_ALLOC_CHUNK=%ld\n", (long)MAX_ALLOC_CHUNK);

  printf("jpeg_natural_order=");
  for (i = 0; i < DCTSIZE2 + 16; i++)
    printf(i ? ",%d" : "%d", jpeg_natural_order[i]);
  printf("\n");

  for (i = 0; i < sizeof(operands) / sizeof(operands[0]); i++)
    printf("round%lu=%ld,%ld\n", (unsigned long)i,
           jdiv_round_up(operands[i][0], operands[i][1]),
           jround_up(operands[i][0], operands[i][1]));

  for (i = 0; i < 4; i++) {
    size_t j;

    for (j = 0; j < 16; j++)
      storage[i][j] = (JSAMPLE)(i * 16 + j);
    rows[i] = storage[i];
  }
  /* rows 0 and 1 onto rows 2 and 3, five samples each */
  jcopy_sample_rows(rows, 0, rows, 2, 2, 5);
  ProbePrintHex("copy_sample_rows", &storage[0][0], sizeof(storage));

  for (i = 0; i < 3; i++) {
    size_t j;

    for (j = 0; j < DCTSIZE2; j++) {
      blocks[0][i][j] = (JCOEF)(i * 100 + j);
      blocks[1][i][j] = (JCOEF)-1;
    }
  }
  jcopy_block_row(blocks[0], blocks[1], 2);
  printf("copy_block_row=");
  for (i = 0; i < 3; i++)
    printf("%s%d,%d", i ? ";" : "", blocks[1][i][0], blocks[1][i][63]);
  printf("\n");
  jzero_far((void *)blocks[0], sizeof(JBLOCK));
  printf("zero_far_blocks=%d,%d,%d\n", blocks[0][0][0], blocks[0][0][63],
         blocks[0][1][0]);
  jzero_far((void *)storage[1], 5);
  ProbePrintHex("zero_far_samples", storage[1], 16);
  return 0;
}
