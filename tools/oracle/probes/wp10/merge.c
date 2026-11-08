/* WP10 probe: the merged upsamplers of the x86-64 SIMD extensions
 * (simd/x86_64/jdmerge-sse2.asm, jdmrgext-sse2.asm, jdmerge-avx2.asm and
 * jdmrgext-avx2.asm).
 *
 * Usage: merge -isa I [-seed S]
 * Calls the oracle's h2v1 and h2v2 merged upsampling kernels of instruction
 * set I (sse2 or avx2) directly, for each pixel layout: the default instance
 * and the extrgb, extrgbx, extbgr, extbgrx, extxbgr and extxrgb instances,
 * in that order. For each kernel and layout it runs one case per output
 * width W of 1 to 66, 101, 227 and 1000, drawing from one generator seeded
 * S (Wp10Seed) in this order: the number of row groups G = 1 + Wp10Rand(3),
 * the row group in_row_group_ctr = Wp10Rand(G), then the luma rows
 * (Wp10Rows(G * V, W), V = 1 for h2v1 and 2 for h2v2), the Cb rows and the
 * Cr rows (Wp10Rows(G, (W + 1) / 2) each), filled with Wp10FillRows. The
 * output rows are V separate zeroed buffers of exactly W * pixel size
 * bytes, as an application's rows may be.
 *
 * Output: one line per kernel and layout, keyed by the kernel's name
 * without the jsimd_ prefix and the _merged_upsample_I suffix (h2v1,
 * h2v1_extrgb, ..., h2v2_extxrgb), with the hashes of the V output rows of
 * every case, in case order, separated by commas (Wp10Hash over exactly
 * W * pixel size bytes). tests/wp10_merge.rs runs the same cases on the
 * port's kernels.
 *
 * The AVX2 kernels need a CPU with AVX2; under Rosetta 2, set
 * ROSETTA_ADVERTISE_AVX=1. The kernels exist only in the x86-64 SIMD build;
 * in every other oracle variant the probe only reports that and exits with
 * status 2.
 */

#include "jconfig.h"
#include "wp10_support.h"

#include <stdlib.h>
#include <string.h>

#if defined(__x86_64__) && defined(WITH_SIMD)

/* The kernels' prototypes, as simd/jsimdint.h declares them with
 * DEFINE_SIMD_EXTRGB_MERGED_UPSAMPLERS. */
#define DEFINE_SIMD_EXTRGB_MERGED_UPSAMPLERS(samp, instrset) \
EXTERN(void) jsimd_##samp##_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf); \
EXTERN(void) jsimd_##samp##_extrgb_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf); \
EXTERN(void) jsimd_##samp##_extrgbx_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf); \
EXTERN(void) jsimd_##samp##_extbgr_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf); \
EXTERN(void) jsimd_##samp##_extbgrx_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf); \
EXTERN(void) jsimd_##samp##_extxbgr_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf); \
EXTERN(void) jsimd_##samp##_extxrgb_merged_upsample_##instrset \
  (JDIMENSION output_width, JSAMPIMAGE input_buf, \
   JDIMENSION in_row_group_ctr, JSAMPARRAY output_buf);

DEFINE_SIMD_EXTRGB_MERGED_UPSAMPLERS(h2v1, avx2)
DEFINE_SIMD_EXTRGB_MERGED_UPSAMPLERS(h2v2, avx2)

DEFINE_SIMD_EXTRGB_MERGED_UPSAMPLERS(h2v1, sse2)
DEFINE_SIMD_EXTRGB_MERGED_UPSAMPLERS(h2v2, sse2)

typedef void (*merged_upsample_ptr) (JDIMENSION output_width,
                                     JSAMPIMAGE input_buf,
                                     JDIMENSION in_row_group_ctr,
                                     JSAMPARRAY output_buf);

typedef struct {
  const char *key;
  int v_samp;                   /* output rows per row group */
  int pixelsize;
  merged_upsample_ptr sse2, avx2;
} Kernel;

#define KERNEL(samp, v, ext, key, pixelsize) \
  { key, v, pixelsize, jsimd_##samp##ext##_merged_upsample_sse2, \
    jsimd_##samp##ext##_merged_upsample_avx2 }

#define KERNELS(samp, v) \
  KERNEL(samp, v, , #samp, 3), \
  KERNEL(samp, v, _extrgb, #samp "_extrgb", 3), \
  KERNEL(samp, v, _extrgbx, #samp "_extrgbx", 4), \
  KERNEL(samp, v, _extbgr, #samp "_extbgr", 3), \
  KERNEL(samp, v, _extbgrx, #samp "_extbgrx", 4), \
  KERNEL(samp, v, _extxbgr, #samp "_extxbgr", 4), \
  KERNEL(samp, v, _extxrgb, #samp "_extxrgb", 4)

static const Kernel kernels[] = { KERNELS(h2v1, 1), KERNELS(h2v2, 2) };

/* The output widths of the cases: 1 to 66, then wider odd ones. */
static int Width(int i)
{
  static const int wide[] = { 101, 227, 1000 };

  return i < 66 ? i + 1 : wide[i - 66];
}

#define NUM_WIDTHS  (66 + 3)

int main(int argc, char **argv)
{
  unsigned long long seed = 1;
  const char *isa = NULL;
  int argi, k, i, row;

  for (argi = 1; argi + 1 < argc; argi += 2) {
    if (!strcmp(argv[argi], "-seed"))
      seed = strtoull(argv[argi + 1], NULL, 10);
    else if (!strcmp(argv[argi], "-isa"))
      isa = argv[argi + 1];
    else {
      fprintf(stderr, "unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  if (argi != argc || isa == NULL ||
      (strcmp(isa, "sse2") && strcmp(isa, "avx2"))) {
    fprintf(stderr, "usage: merge -isa sse2|avx2 [-seed S]\n");
    return 2;
  }

  Wp10Seed(seed);
  for (k = 0; k < (int)(sizeof(kernels) / sizeof(kernels[0])); k++) {
    const Kernel *kernel = &kernels[k];
    merged_upsample_ptr fn = strcmp(isa, "avx2") ? kernel->sse2 : kernel->avx2;

    printf("%s=", kernel->key);
    for (i = 0; i < NUM_WIDTHS; i++) {
      JDIMENSION width = (JDIMENSION)Width(i);
      JDIMENSION chroma_width = (width + 1) / 2;
      int groups = 1 + Wp10Rand(3);
      JDIMENSION ctr = (JDIMENSION)Wp10Rand(groups);
      int y_rows = groups * kernel->v_samp;
      size_t len = (size_t)width * kernel->pixelsize;
      JSAMPARRAY image[3];
      JSAMPROW output[2];

      image[0] = Wp10Rows(y_rows, width);
      image[1] = Wp10Rows(groups, chroma_width);
      image[2] = Wp10Rows(groups, chroma_width);
      Wp10FillRows(image[0], y_rows, width);
      Wp10FillRows(image[1], groups, chroma_width);
      Wp10FillRows(image[2], groups, chroma_width);
      for (row = 0; row < kernel->v_samp; row++) {
        output[row] = (JSAMPROW)calloc(len, 1);
        if (output[row] == NULL) {
          fprintf(stderr, "merge: out of memory\n");
          return 2;
        }
      }

      (*fn) (width, image, ctr, output);

      for (row = 0; row < kernel->v_samp; row++) {
        printf("%s%016llx", i || row ? "," : "", Wp10Hash(output[row], len));
        free(output[row]);
      }
      Wp10FreeRows(image[0]);
      Wp10FreeRows(image[1]);
      Wp10FreeRows(image[2]);
    }
    printf("\n");
  }
  return 0;
}

#else

int main(void)
{
  fprintf(stderr, "merge: the probe needs the x86-64 SIMD build\n");
  return 2;
}

#endif
