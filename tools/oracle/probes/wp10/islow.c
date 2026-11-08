/* WP10 probe: the accurate integer IDCT kernels of the x86-64 SIMD build
 * (simd/x86_64/jidctint-sse2.asm and jidctint-avx2.asm).
 *
 * Usage: islow -isa I [-seed S] [-cases N]
 * Calls jsimd_idct_islow_I (I = sse2 or avx2) of the oracle's libjpeg
 * directly, N times (default 2000), after Wp10Seed(S) (default 1). Each case
 * draws, in this order:
 *   - a quantization table: qmax from {1, 255, 32767, 65535}, then
 *     quantval[i] = 1 + Wp10Rand(qmax) for each i, turned into the 16-bit
 *     ISLOW_MULT_TYPE multiplier table as jddctmgr.c's start_pass does with
 *     WITH_SIMD (a quantizer above 32767 wraps);
 *   - a coefficient block (MakeBlock below), from image-like blocks to
 *     arbitrary JCOEF values, with the cases of the zero-column test (rows
 *     1-7 all zero, or zero but for one coefficient);
 *   - an output array of Wp10Rows(16, 8 * B) with B = 1 + Wp10Rand(12),
 *     filled with Wp10FillRows;
 *   - the first output row, 0 or 8, and output_col: a multiple of 8 below
 *     8 * B, as the decoder passes, or any column up to 8 * B - 8.
 * The kernel writes 8 rows of 8 samples from the first output row and
 * output_col on.
 *
 * Output: `out=` with the FNV-1a hash (Wp10Hash) of the whole output array,
 * all 16 rows with their padding, after each case, as 16 hex digits each,
 * separated by commas. tests/wp10_islow.rs draws the same cases for the
 * port's kernels and compares the hashes.
 *
 * The kernels exist only in an x86-64 build with WITH_SIMD; elsewhere the
 * probe reports that and exits with status 2.
 */

#define JPEG_INTERNALS
#include "wp10_support.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "jdct.h"

#if defined(__x86_64__) && defined(WITH_SIMD)

/* The prototypes of simd/jsimdint.h. */
EXTERN(void) jsimd_idct_islow_avx2
  (void *dct_table, JCOEFPTR coef_block, JSAMPARRAY output_buf,
   JDIMENSION output_col);
EXTERN(void) jsimd_idct_islow_sse2
  (void *dct_table, JCOEFPTR coef_block, JSAMPARRAY output_buf,
   JDIMENSION output_col);

typedef void (*idct_kernel) (void *dct_table, JCOEFPTR coef_block,
                             JSAMPARRAY output_buf, JDIMENSION output_col);

#define NUM_ROWS  16

/* The asm reads the block and the table with aligned SSE2 memory operands,
 * so both are aligned as the decoder's are. */
static _Alignas(32) JCOEF block[DCTSIZE2];
static _Alignas(32) ISLOW_MULT_TYPE table[DCTSIZE2];

/* A random coefficient: an arbitrary 16-bit value or an edge case. */
static JCOEF Coef(void)
{
  static const int edges[] = { -32768, 32767, -1, 0, 1, 256, -256, 0x7F00 };

  return Wp10Rand(2) ? (JCOEF)(short)(Wp10Next() & 0xFFFF) :
                       (JCOEF)edges[Wp10Rand(8)];
}

/* One pseudo-random block for the quantization values quantval. */
static void MakeBlock(const unsigned int *quantval)
{
  int k, kind = Wp10Rand(8);

  memset(block, 0, sizeof(block));
  switch (kind) {
  case 0:                       /* image-like: falling magnitudes, sparse */
    for (k = 0; k < DCTSIZE2; k++) {
      int lim = 1024 / (1 + k / DCTSIZE + k % DCTSIZE);

      if (k == 0 || Wp10Rand(4) == 0)
        block[k] = (JCOEF)((Wp10Rand(2 * lim + 1) - lim) / (int)quantval[k]);
    }
    break;
  case 1:                       /* DC, perhaps with one AC term in row 0 */
    block[0] = (JCOEF)((Wp10Rand(2049) - 1024) / (int)quantval[0]);
    if (Wp10Rand(2)) {
      k = 1 + Wp10Rand(DCTSIZE - 1);  /* drawn before the value */
      block[k] = (JCOEF)(Wp10Rand(201) - 100);
    }
    break;
  case 2:                       /* one coefficient anywhere */
    k = Wp10Rand(DCTSIZE2);
    block[k] = (JCOEF)((Wp10Rand(4097) - 2048) / (int)quantval[k]);
    break;
  case 3:                       /* dense, moderate */
    for (k = 0; k < DCTSIZE2; k++)
      block[k] = (JCOEF)(Wp10Rand(257) - 128);
    break;
  case 4:                       /* arbitrary values */
    for (k = 0; k < DCTSIZE2; k++)
      block[k] = Coef();
    break;
  case 5:                       /* arbitrary row 0, rows 1-7 zero */
    for (k = 0; k < DCTSIZE; k++)
      block[k] = Coef();
    break;
  case 6:                       /* arbitrary row 0, one value in rows 1-7 */
    for (k = 0; k < DCTSIZE; k++)
      block[k] = Coef();
    k = DCTSIZE + Wp10Rand(DCTSIZE2 - DCTSIZE);
    block[k] = Coef();
    break;
  default:                      /* all zero */
    break;
  }
}

int main(int argc, char **argv)
{
  static const unsigned int qmaxes[] = { 1, 255, 32767, 65535 };
  unsigned long long seed = 1;
  int cases = 2000, argi, c, i;
  const char *isa = NULL;
  idct_kernel kernel;

  for (argi = 1; argi + 1 < argc; argi += 2) {
    if (!strcmp(argv[argi], "-isa"))
      isa = argv[argi + 1];
    else if (!strcmp(argv[argi], "-seed"))
      seed = strtoull(argv[argi + 1], NULL, 10);
    else if (!strcmp(argv[argi], "-cases"))
      cases = atoi(argv[argi + 1]);
    else {
      fprintf(stderr, "unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  if (isa && !strcmp(isa, "sse2"))
    kernel = jsimd_idct_islow_sse2;
  else if (isa && !strcmp(isa, "avx2"))
    kernel = jsimd_idct_islow_avx2;
  else {
    fprintf(stderr, "-isa must be sse2 or avx2\n");
    return 2;
  }

  Wp10Seed(seed);
  printf("out=");
  for (c = 0; c < cases; c++) {
    unsigned int quantval[DCTSIZE2], qmax = qmaxes[Wp10Rand(4)];
    JDIMENSION width, output_col;
    JSAMPARRAY rows;
    int first_row;

    for (i = 0; i < DCTSIZE2; i++) {
      quantval[i] = 1 + (unsigned int)Wp10Rand((int)qmax);
      table[i] = (ISLOW_MULT_TYPE)quantval[i];
    }
    MakeBlock(quantval);
    width = (JDIMENSION)(DCTSIZE * (1 + Wp10Rand(12)));
    rows = Wp10Rows(NUM_ROWS, width);
    Wp10FillRows(rows, NUM_ROWS, width);
    first_row = DCTSIZE * Wp10Rand(2);
    output_col = Wp10Rand(2) ?
                 (JDIMENSION)(DCTSIZE * Wp10Rand((int)(width / DCTSIZE))) :
                 (JDIMENSION)Wp10Rand((int)width - DCTSIZE + 1);
    (*kernel) (table, block, rows + first_row, output_col);
    printf("%s%016llx", c ? "," : "",
           Wp10Hash(rows[0], (size_t)NUM_ROWS * Wp10Stride(width)));
    Wp10FreeRows(rows);
  }
  printf("\n");
  return 0;
}

#else

int main(void)
{
  fprintf(stderr, "islow: the x86-64 SIMD kernels are not in this build\n");
  return 2;
}

#endif
