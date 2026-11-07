/* WP4 probe: the inverse DCTs and their manager (jddctmgr.c, jidctint.c,
 * jidctfst.c, jidctflt.c, jidctred.c).
 *
 * Usage: idct -method M -size N [-blocks K] [-seed S] [-qmax Q] [-extreme]
 * Creates a decompression object with one component, gives it a random
 * quantization table (values 1..Q) as jdinput.c's latch_quant_tables would,
 * sets DCT_scaled_size = N and dct_method = M (islow, ifast, float or a
 * number), builds sample_range_limit as jdmaster.c's
 * prepare_range_limit_table does, and calls jinit_inverse_dct and
 * (*cinfo->idct->start_pass). It then transforms K pseudo-random coefficient
 * blocks (seed S) with (*cinfo->idct->inverse_DCT[0]) into a 32x16 sample
 * array from alloc_sarray, at output column 0 or N in turn. -extreme adds
 * blocks of arbitrary JCOEF values; it is valid only for the methods that do
 * all their arithmetic in JLONG (islow and the scaled sizes), where such
 * blocks are defined behavior in C.
 *
 * Output: `out=` with an FNV-1a hash of the N x N output of every block, the
 * memory pools (wp2_support.c's Wp2PrintMemory) and the error.
 * tests/wp4_idct.rs runs the same sequence on the Rust port.
 */

#include "wp2_support.h"

#include <stdlib.h>
#include <string.h>

static struct jpeg_decompress_struct cinfo;
static Wp2ErrorMgr err;
static unsigned long long rng_state;

/* xorshift64 */
static unsigned int Next(void)
{
  rng_state ^= rng_state << 13;
  rng_state ^= rng_state >> 7;
  rng_state ^= rng_state << 17;
  return (unsigned int)(rng_state >> 32);
}

static int Rand(int n)
{
  return (int)(Next() % (unsigned int)n);
}

/* jdmaster.c's prepare_range_limit_table, 8-bit branch. */
static void PrepareRangeLimitTable(j_decompress_ptr cinfo)
{
  JSAMPLE *table;
  int i;

  table = (JSAMPLE *)
    (*cinfo->mem->alloc_small) ((j_common_ptr)cinfo, JPOOL_IMAGE,
                (5 * (MAXJSAMPLE + 1) + CENTERJSAMPLE) * sizeof(JSAMPLE));
  table += (MAXJSAMPLE + 1);
  cinfo->sample_range_limit = table;
  memset(table - (MAXJSAMPLE + 1), 0, (MAXJSAMPLE + 1) * sizeof(JSAMPLE));
  for (i = 0; i <= MAXJSAMPLE; i++)
    table[i] = (JSAMPLE)i;
  table += CENTERJSAMPLE;
  for (i = CENTERJSAMPLE; i < 2 * (MAXJSAMPLE + 1); i++)
    table[i] = MAXJSAMPLE;
  memset(table + (2 * (MAXJSAMPLE + 1)), 0,
         (2 * (MAXJSAMPLE + 1) - CENTERJSAMPLE) * sizeof(JSAMPLE));
  memcpy(table + (4 * (MAXJSAMPLE + 1) - CENTERJSAMPLE),
         cinfo->sample_range_limit, CENTERJSAMPLE * sizeof(JSAMPLE));
}

/* One pseudo-random block for quantization table qtbl. */
static void MakeBlock(JCOEF *block, const JQUANT_TBL *qtbl, int extreme)
{
  int k, kind = Rand(extreme ? 5 : 4);

  memset(block, 0, DCTSIZE2 * sizeof(JCOEF));
  switch (kind) {
  case 0:                       /* image-like: falling magnitudes, sparse */
    for (k = 0; k < DCTSIZE2; k++) {
      int lim = 1024 / (1 + k / DCTSIZE + k % DCTSIZE);

      if (k == 0 || Rand(4) == 0)
        block[k] = (JCOEF)((Rand(2 * lim + 1) - lim) / (int)qtbl->quantval[k]);
    }
    break;
  case 1:                       /* DC, perhaps with one AC term in row 0 */
    block[0] = (JCOEF)((Rand(2049) - 1024) / (int)qtbl->quantval[0]);
    if (Rand(2)) {
      k = 1 + Rand(DCTSIZE - 1);  /* drawn before the value, as one sequence */
      block[k] = (JCOEF)(Rand(201) - 100);
    }
    break;
  case 2:                       /* one coefficient anywhere */
    k = Rand(DCTSIZE2);
    block[k] = (JCOEF)((Rand(4097) - 2048) / (int)qtbl->quantval[k]);
    break;
  case 3:                       /* dense, moderate */
    for (k = 0; k < DCTSIZE2; k++)
      block[k] = (JCOEF)(Rand(257) - 128);
    break;
  default:                      /* arbitrary values (-extreme) */
    for (k = 0; k < DCTSIZE2; k++) {
      static const int edges[] = { -32768, 32767, -1, 0, 1 };

      block[k] = Rand(2) ? (JCOEF)(short)(Next() & 0xFFFF) :
                           (JCOEF)edges[Rand(5)];
    }
    break;
  }
}

int main(int argc, char *argv[])
{
  int i, size = DCTSIZE, blocks = 100, qmax = 255, extreme = 0, b;
  J_DCT_METHOD method = JDCT_ISLOW;
  unsigned long long seed = 1;
  JQUANT_TBL *qtbl;
  JSAMPARRAY buffer;
  JCOEF block[DCTSIZE2];

  for (i = 1; i < argc; i++) {
    if (!strcmp(argv[i], "-method") && i + 1 < argc) {
      i++;
      if (!strcmp(argv[i], "islow"))
        method = JDCT_ISLOW;
      else if (!strcmp(argv[i], "ifast"))
        method = JDCT_IFAST;
      else if (!strcmp(argv[i], "float"))
        method = JDCT_FLOAT;
      else
        method = (J_DCT_METHOD)atoi(argv[i]);
    } else if (!strcmp(argv[i], "-size") && i + 1 < argc) {
      size = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-blocks") && i + 1 < argc) {
      blocks = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-seed") && i + 1 < argc) {
      seed = strtoull(argv[++i], NULL, 10);
    } else if (!strcmp(argv[i], "-qmax") && i + 1 < argc) {
      qmax = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-extreme")) {
      extreme = 1;
    } else {
      fprintf(stderr, "unknown option %s\n", argv[i]);
      return 2;
    }
  }
  rng_state = seed * 2654435761ULL + 1;

  cinfo.err = Wp2ErrorInit(&err);
  if (setjmp(err.setjmp_buffer)) {
    printf("out=\n");
    Wp2PrintMemory(&cinfo);
    Wp2PrintError(&err);
    return 0;
  }
  jpeg_create_decompress(&cinfo);
  cinfo.data_precision = 8;
  cinfo.num_components = 1;
  cinfo.dct_method = method;
  cinfo.comp_info = (jpeg_component_info *)
    (*cinfo.mem->alloc_small) ((j_common_ptr)&cinfo, JPOOL_IMAGE,
                               sizeof(jpeg_component_info));
  memset(cinfo.comp_info, 0, sizeof(jpeg_component_info));
  cinfo.comp_info[0].component_needed = TRUE;
  cinfo.comp_info[0].DCT_scaled_size = size;
  qtbl = (JQUANT_TBL *)
    (*cinfo.mem->alloc_small) ((j_common_ptr)&cinfo, JPOOL_IMAGE,
                               sizeof(JQUANT_TBL));
  memset(qtbl, 0, sizeof(JQUANT_TBL));
  for (i = 0; i < DCTSIZE2; i++)
    qtbl->quantval[i] = (UINT16)(1 + Rand(qmax));
  cinfo.comp_info[0].quant_table = qtbl;

  PrepareRangeLimitTable(&cinfo);
  jinit_inverse_dct(&cinfo);
  buffer = (*cinfo.mem->alloc_sarray) ((j_common_ptr)&cinfo, JPOOL_IMAGE,
                                       2 * 16, 16);
  for (i = 0; i < 16; i++)
    memset(buffer[i], 0, 2 * 16);
  (*cinfo.idct->start_pass) (&cinfo);

  printf("out=");
  for (b = 0; b < blocks; b++) {
    JDIMENSION output_col = (JDIMENSION)((b & 1) * size);
    unsigned long long hash = 1469598103934665603ULL;
    int row, col;

    MakeBlock(block, qtbl, extreme);
    (*cinfo.idct->inverse_DCT[0]) (&cinfo, &cinfo.comp_info[0], block, buffer,
                                   output_col);
    for (row = 0; row < size; row++)
      for (col = 0; col < size; col++)
        hash = (hash ^ buffer[row][output_col + col]) * 1099511628211ULL;
    printf("%s%016llx", b ? "," : "", hash);
  }
  printf("\n");
  Wp2PrintMemory(&cinfo);
  Wp2PrintError(&err);
  return 0;
}
