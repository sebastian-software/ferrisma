/* WP10 probe: the x86-64 SSE2 IDCT kernels without AVX2 versions
 * (simd/x86_64/jidctfst-sse2.asm, jidctflt-sse2.asm and jidctred-sse2.asm).
 *
 * Usage: idct -kernel K [-seed S] [-tables T] [-blocks B] [-qmax Q]
 * K is ifast (jsimd_idct_ifast_sse2), float (jsimd_idct_float_sse2), 4x4
 * (jsimd_idct_4x4_sse2) or 2x2 (jsimd_idct_2x2_sse2). After Wp10Seed(S), for
 * each of T random quantization tables (values 1..Q), a decompression object
 * with one component turns the table into the kernel's multiplier table as
 * jddctmgr.c's start_pass does: dct_method ifast or float with
 * DCT_scaled_size 8, or DCT_scaled_size 4 or 2, which use the islow table.
 * Then B times: an output buffer Wp10Rows(N, W * N) for N x N output blocks,
 * W = 1 + Wp10Rand(40) blocks wide, filled with Wp10FillRows; an output
 * column of Wp10Rand(W) * N, as the decoder steps it; a random coefficient
 * block (see MakeBlock); and one call of the kernel by its symbol.
 *
 * Output: `table=` with the FNV-1a hash (Wp10Hash) of each multiplier table
 * (128 bytes, or 256 for float), and `out=` with the hash of each output
 * buffer after its call, over all N rows of the whole stride, padding
 * included. tests/wp10_idct.rs draws the same inputs for the port's kernels.
 *
 * The asm kernels exist only in the x86-64 SIMD build; in the other oracle
 * variants the probe only reports that it needs that build and exits with
 * status 2.
 */

#define JPEG_INTERNALS

#include "wp10_support.h"

#include <stdlib.h>
#include <string.h>

#include "jdct.h"

#if defined(__x86_64__) && defined(WITH_SIMD)

/* The prototypes of simd/jsimdint.h. */
EXTERN(void) jsimd_idct_ifast_sse2
  (void *dct_table, JCOEFPTR coef_block, JSAMPARRAY output_buf,
   JDIMENSION output_col);
EXTERN(void) jsimd_idct_float_sse2
  (void *dct_table, JCOEFPTR coef_block, JSAMPARRAY output_buf,
   JDIMENSION output_col);
EXTERN(void) jsimd_idct_2x2_sse2
  (void *dct_table, JCOEFPTR coef_block, JSAMPARRAY output_buf,
   JDIMENSION output_col);
EXTERN(void) jsimd_idct_4x4_sse2
  (void *dct_table, JCOEFPTR coef_block, JSAMPARRAY output_buf,
   JDIMENSION output_col);

typedef void (*IdctKernel) (void *dct_table, JCOEFPTR coef_block,
                            JSAMPARRAY output_buf, JDIMENSION output_col);

/* The widest output buffer, in blocks. */
#define MAX_BLOCKS  40

/* A JCOEF of any 16-bit value. */
static JCOEF AnyCoef(void)
{
  return (JCOEF)(short)(Wp10Next() & 0xFFFF);
}

/* One random block for quantization table qtbl. The kinds reach the 16-bit
 * wraparound and saturation of the asm and both branches of each kernel's
 * test for all-zero AC terms. */
static void MakeBlock(JCOEF *block, const JQUANT_TBL *qtbl)
{
  static const int edges[] = { -32768, 32767, -1, 0, 1 };
  int k, half;

  memset(block, 0, DCTSIZE2 * sizeof(JCOEF));
  switch (Wp10Rand(6)) {
  case 0:                       /* full range */
    for (k = 0; k < DCTSIZE2; k++)
      block[k] = AnyCoef();
    break;
  case 1:                       /* full range and edge values */
    for (k = 0; k < DCTSIZE2; k++)
      block[k] = Wp10Rand(2) ? AnyCoef() : (JCOEF)edges[Wp10Rand(5)];
    break;
  case 2:                       /* row 0 only, perhaps one more coefficient */
    for (k = 0; k < DCTSIZE; k++)
      block[k] = AnyCoef();
    if (Wp10Rand(2)) {
      k = DCTSIZE + Wp10Rand(DCTSIZE2 - DCTSIZE);  /* drawn before the value */
      block[k] = AnyCoef();
    }
    break;
  case 3:                       /* rows 1-7 zero in columns 0-3 or 4-7 */
    half = Wp10Rand(2);
    for (k = 0; k < DCTSIZE2; k++)
      if (k < DCTSIZE || (k % DCTSIZE) / 4 != half)
        block[k] = AnyCoef();
    break;
  case 4:                       /* sparse, full range */
    for (k = 0; k < DCTSIZE2; k++)
      if (Wp10Rand(8) == 0)
        block[k] = AnyCoef();
    break;
  default:                      /* image-like: falling magnitudes, sparse */
    for (k = 0; k < DCTSIZE2; k++) {
      int lim = 1024 / (1 + k / DCTSIZE + k % DCTSIZE);

      if (k == 0 || Wp10Rand(4) == 0)
        block[k] = (JCOEF)((Wp10Rand(2 * lim + 1) - lim) /
                           (int)qtbl->quantval[k]);
    }
    break;
  }
}

int main(int argc, char **argv)
{
  const char *kernel_name = NULL;
  unsigned long long seed = 1;
  int tables = 1, blocks = 1, qmax = 255, argi, t, b, i, size;
  size_t table_size;
  J_DCT_METHOD method;
  IdctKernel kernel;
  struct jpeg_decompress_struct cinfo;
  ProbeErrorMgr err;
  JQUANT_TBL qtbl;
  JCOEF block[DCTSIZE2];
  unsigned long long *table_hashes, *out_hashes;

  for (argi = 1; argi + 1 < argc; argi += 2) {
    if (!strcmp(argv[argi], "-kernel"))
      kernel_name = argv[argi + 1];
    else if (!strcmp(argv[argi], "-seed"))
      seed = strtoull(argv[argi + 1], NULL, 10);
    else if (!strcmp(argv[argi], "-tables"))
      tables = atoi(argv[argi + 1]);
    else if (!strcmp(argv[argi], "-blocks"))
      blocks = atoi(argv[argi + 1]);
    else if (!strcmp(argv[argi], "-qmax"))
      qmax = atoi(argv[argi + 1]);
    else {
      fprintf(stderr, "unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  if (argi != argc || kernel_name == NULL) {
    fprintf(stderr, "usage: idct -kernel K [-seed S] [-tables T] [-blocks B] [-qmax Q]\n");
    return 2;
  }
  if (!strcmp(kernel_name, "ifast")) {
    kernel = jsimd_idct_ifast_sse2;
    method = JDCT_IFAST;
    size = DCTSIZE;
  } else if (!strcmp(kernel_name, "float")) {
    kernel = jsimd_idct_float_sse2;
    method = JDCT_FLOAT;
    size = DCTSIZE;
  } else if (!strcmp(kernel_name, "4x4")) {
    kernel = jsimd_idct_4x4_sse2;
    method = JDCT_ISLOW;
    size = 4;
  } else if (!strcmp(kernel_name, "2x2")) {
    kernel = jsimd_idct_2x2_sse2;
    method = JDCT_ISLOW;
    size = 2;
  } else {
    fprintf(stderr, "unknown kernel %s\n", kernel_name);
    return 2;
  }
  table_size = method == JDCT_FLOAT ? DCTSIZE2 * sizeof(FLOAT_MULT_TYPE) :
                                      DCTSIZE2 * sizeof(ISLOW_MULT_TYPE);

  table_hashes = (unsigned long long *)malloc((size_t)tables *
                                              sizeof(unsigned long long));
  out_hashes = (unsigned long long *)malloc((size_t)tables * blocks *
                                            sizeof(unsigned long long));
  if (table_hashes == NULL || out_hashes == NULL) {
    fprintf(stderr, "out of memory\n");
    return 2;
  }

  Wp10Seed(seed);
  cinfo.err = ProbeErrorInit(&err);
  if (setjmp(err.setjmp_buffer)) {
    ProbePrintErrors(&err);
    return 1;
  }
  for (t = 0; t < tables; t++) {
    memset(&qtbl, 0, sizeof(qtbl));
    for (i = 0; i < DCTSIZE2; i++)
      qtbl.quantval[i] = (UINT16)(1 + Wp10Rand(qmax));

    /* jddctmgr.c's start_pass builds the multiplier table. */
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
    cinfo.comp_info[0].quant_table = &qtbl;
    jinit_inverse_dct(&cinfo);
    (*cinfo.idct->start_pass) (&cinfo);
    table_hashes[t] = Wp10Hash(cinfo.comp_info[0].dct_table, table_size);

    for (b = 0; b < blocks; b++) {
      JDIMENSION width = (JDIMENSION)((1 + Wp10Rand(MAX_BLOCKS)) * size);
      JDIMENSION output_col;
      JSAMPARRAY rows = Wp10Rows(size, width);

      Wp10FillRows(rows, size, width);
      output_col = (JDIMENSION)(Wp10Rand((int)width / size) * size);
      MakeBlock(block, &qtbl);
      (*kernel) (cinfo.comp_info[0].dct_table, block, rows, output_col);
      out_hashes[t * blocks + b] =
        Wp10Hash(rows[0], (size_t)size * Wp10Stride(width));
      Wp10FreeRows(rows);
    }
    jpeg_destroy_decompress(&cinfo);
  }

  printf("table=");
  for (t = 0; t < tables; t++)
    printf("%s%016llx", t ? "," : "", table_hashes[t]);
  printf("\nout=");
  for (i = 0; i < tables * blocks; i++)
    printf("%s%016llx", i ? "," : "", out_hashes[i]);
  printf("\n");
  free(table_hashes);
  free(out_hashes);
  return 0;
}

#else

int main(void)
{
  fprintf(stderr, "idct: the probe needs the x86-64 SIMD build\n");
  return 2;
}

#endif
