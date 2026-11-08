/* WP10 probe: the x86-64 upsampling kernels (simd/x86_64/jdsample-sse2.asm,
 * simd/x86_64/jdsample-avx2.asm).
 *
 * Usage: sample -kernel K [-seed S]
 * K is one of h2v1_fancy, h2v2_fancy, h2v1 and h2v2, followed by _sse2 or
 * _avx2: the kernel jsimd_<K>_upsample... that simd/jsimd.c stores in the
 * upsampler, called directly by its symbol. After Wp10Seed(S), the probe
 * draws RANDOM_WIDTHS widths of 1..1000 with Wp10Rand and runs one case for
 * each width of 1..66, 101, 227 and 1000 and then of the random widths, and
 * for each max_v_samp_factor the decoder passes to the kernel (1..4 for
 * h2v1, 2 and 4 for h2v2). The width is downsampled_width for the fancy
 * kernels and output_width for the plain ones.
 *
 * Each case allocates its input rows with Wp10Rows (the rows the kernel
 * reads; for h2v2_fancy also the context rows above and below, and in every
 * third case the row above, or in every third case from the second on the
 * row below, is the first or last input row itself, as jdmainct.c's pointer
 * lists repeat rows at the image edges) and max_v_samp_factor output rows,
 * fills both with Wp10FillRows (input first), calls the kernel and prints
 * `out.<width>.<max_v_samp_factor>=` with Wp10PrintRows over the whole
 * stride of each output row. The fancy kernels write a dummy sample into
 * their input rows, so every case draws new inputs.
 * tests/wp10_sample.rs runs the same cases on the port's kernels.
 *
 * The kernels exist only in the x86-64 SIMD build; elsewhere the probe
 * reports that on stderr and exits with status 2.
 */

#include "wp10_support.h"         /* jpeglib.h, and through it jconfig.h */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#if defined(__x86_64__) && defined(WITH_SIMD)

/* The kernels, as simd/jsimdint.h declares them. */
EXTERN(void) jsimd_h2v1_upsample_avx2
  (int max_v_samp_factor, JDIMENSION output_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);
EXTERN(void) jsimd_h2v2_upsample_avx2
  (int max_v_samp_factor, JDIMENSION output_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);

EXTERN(void) jsimd_h2v1_upsample_sse2
  (int max_v_samp_factor, JDIMENSION output_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);
EXTERN(void) jsimd_h2v2_upsample_sse2
  (int max_v_samp_factor, JDIMENSION output_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);

EXTERN(void) jsimd_h2v1_fancy_upsample_avx2
  (int max_v_samp_factor, JDIMENSION downsampled_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);
EXTERN(void) jsimd_h2v2_fancy_upsample_avx2
  (int max_v_samp_factor, JDIMENSION downsampled_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);

EXTERN(void) jsimd_h2v1_fancy_upsample_sse2
  (int max_v_samp_factor, JDIMENSION downsampled_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);
EXTERN(void) jsimd_h2v2_fancy_upsample_sse2
  (int max_v_samp_factor, JDIMENSION downsampled_width, JSAMPARRAY input_data,
   JSAMPARRAY *output_data_ptr);

typedef void (*upsample_ptr) (int, JDIMENSION, JSAMPARRAY, JSAMPARRAY *);

#define RANDOM_WIDTHS  12

static const struct {
  const char *name;
  upsample_ptr kernel;
  int fancy;                    /* fancy (TRUE) or plain (FALSE) */
  int v2;                       /* h2v2 (TRUE) or h2v1 (FALSE) */
} kernels[] = {
  { "h2v1_fancy_sse2", jsimd_h2v1_fancy_upsample_sse2, TRUE, FALSE },
  { "h2v2_fancy_sse2", jsimd_h2v2_fancy_upsample_sse2, TRUE, TRUE },
  { "h2v1_sse2", jsimd_h2v1_upsample_sse2, FALSE, FALSE },
  { "h2v2_sse2", jsimd_h2v2_upsample_sse2, FALSE, TRUE },
  { "h2v1_fancy_avx2", jsimd_h2v1_fancy_upsample_avx2, TRUE, FALSE },
  { "h2v2_fancy_avx2", jsimd_h2v2_fancy_upsample_avx2, TRUE, TRUE },
  { "h2v1_avx2", jsimd_h2v1_upsample_avx2, FALSE, FALSE },
  { "h2v2_avx2", jsimd_h2v2_upsample_avx2, FALSE, TRUE },
};

/* One case: see the usage comment. */
static void RunCase(int k, JDIMENSION width, int max_v_samp_factor, int index)
{
  int fancy = kernels[k].fancy, v2 = kernels[k].v2;
  int in_rows = v2 ? max_v_samp_factor / 2 : max_v_samp_factor;
  int context = fancy && v2;
  int numrows = in_rows + (context ? 2 : 0), row;
  JDIMENSION in_width = fancy ? width : (width + 1) / 2;
  JDIMENSION out_width = fancy ? 2 * width : width;
  JSAMPARRAY input = Wp10Rows(numrows, in_width);
  JSAMPARRAY output = Wp10Rows(max_v_samp_factor, out_width);
  JSAMPROW list[6];
  char key[64];

  Wp10FillRows(input, numrows, in_width);
  Wp10FillRows(output, max_v_samp_factor, out_width);
  for (row = 0; row < numrows; row++)
    list[row] = input[row];
  if (context) {
    if (index % 3 == 1)
      list[0] = list[1];
    else if (index % 3 == 2)
      list[numrows - 1] = list[numrows - 2];
  }
  (*kernels[k].kernel) (max_v_samp_factor, width, list + (context ? 1 : 0),
                        &output);
  snprintf(key, sizeof(key), "out.%u.%d", width, max_v_samp_factor);
  Wp10PrintRows(key, output, max_v_samp_factor, Wp10Stride(out_width));
  Wp10FreeRows(output);
  Wp10FreeRows(input);
}

int main(int argc, char **argv)
{
  unsigned long long seed = 1;
  const char *name = NULL;
  JDIMENSION widths[66 + 3 + RANDOM_WIDTHS];
  int argi, k = -1, i, num_widths = 0, index = 0, max_v;

  for (argi = 1; argi + 1 < argc; argi += 2) {
    if (!strcmp(argv[argi], "-kernel"))
      name = argv[argi + 1];
    else if (!strcmp(argv[argi], "-seed"))
      seed = strtoull(argv[argi + 1], NULL, 10);
    else {
      fprintf(stderr, "unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  for (i = 0; name && i < (int)(sizeof(kernels) / sizeof(kernels[0])); i++)
    if (!strcmp(name, kernels[i].name))
      k = i;
  if (k < 0) {
    fprintf(stderr, "unknown kernel %s\n", name ? name : "(none)");
    return 2;
  }

  Wp10Seed(seed);
  for (i = 1; i <= 66; i++)
    widths[num_widths++] = (JDIMENSION)i;
  widths[num_widths++] = 101;
  widths[num_widths++] = 227;
  widths[num_widths++] = 1000;
  for (i = 0; i < RANDOM_WIDTHS; i++)
    widths[num_widths++] = (JDIMENSION)(Wp10Rand(1000) + 1);

  for (i = 0; i < num_widths; i++) {
    for (max_v = kernels[k].v2 ? 2 : 1; max_v <= 4;
         max_v += kernels[k].v2 ? 2 : 1)
      RunCase(k, widths[i], max_v, index++);
  }
  return 0;
}

#else

/* Only the x86-64 SIMD build has the asm kernels. */
int main(void)
{
  fprintf(stderr, "sample: this probe needs the x86-64 SIMD build\n");
  return 2;
}

#endif
