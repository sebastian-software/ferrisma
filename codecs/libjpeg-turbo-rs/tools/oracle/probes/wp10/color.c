/* WP10 probe: the YCbCr-to-RGB kernels of simd/x86_64 (jdcolor-sse2.asm,
 * jdcolext-sse2.asm, jdcolor-avx2.asm, jdcolext-avx2.asm).
 *
 * Usage: color -isa I -format F [-seed S] [-cases N]
 * Calls jsimd_ycc_<format>_convert_<I> of the oracle's libjpeg directly,
 * where I is sse2 or avx2 and F selects the instance:
 *
 *   0  jsimd_ycc_rgb_convert     (RGB_PIXELSIZE 3, the default layout)
 *   1  jsimd_ycc_extrgb_convert  (3)
 *   2  jsimd_ycc_extrgbx_convert (4)
 *   3  jsimd_ycc_extbgr_convert  (3)
 *   4  jsimd_ycc_extbgrx_convert (4)
 *   5  jsimd_ycc_extxbgr_convert (4)
 *   6  jsimd_ycc_extxrgb_convert (4)
 *
 * After Wp10Seed(S), case c (0 <= c < N, default 69) takes the width
 * widths[c % 69] (1 to 66, 101, 227 and 1000) and draws, in this order,
 * input_row = Wp10Rand(3) and num_rows = Wp10Rand(4). It allocates the three
 * components as Wp10Rows(input_row + num_rows + 1, width) and fills them
 * with Wp10FillRows, Y, Cb and Cr in turn, then allocates num_rows separate
 * output rows of exactly width * RGB_PIXELSIZE bytes, as an application's
 * rows may be, and fills each, row by row, with Wp10Sample(). It calls the
 * kernel with out_width = width.
 *
 * Output: `case<c>=` with Wp10PrintRows over the width * RGB_PIXELSIZE
 * bytes of each output row. tests/wp10_color.rs runs the same cases on the
 * port's kernels.
 *
 * The kernels exist only in the x86-64 SIMD build; elsewhere the probe
 * says so on stderr and fails.
 */

#include "wp10_support.h"

#include <stdlib.h>
#include <string.h>

#if defined(__x86_64__) && defined(WITH_SIMD)

/* The prototypes of DEFINE_SIMD_EXTRGB_COLOR_DECONVERTERS in
 * simd/jsimdint.h. */
#define DECLARE(name) \
  EXTERN(void) name(JDIMENSION out_width, JSAMPIMAGE input_buf, \
                    JDIMENSION input_row, JSAMPARRAY output_buf, int num_rows);

DECLARE(jsimd_ycc_rgb_convert_sse2)
DECLARE(jsimd_ycc_extrgb_convert_sse2)
DECLARE(jsimd_ycc_extrgbx_convert_sse2)
DECLARE(jsimd_ycc_extbgr_convert_sse2)
DECLARE(jsimd_ycc_extbgrx_convert_sse2)
DECLARE(jsimd_ycc_extxbgr_convert_sse2)
DECLARE(jsimd_ycc_extxrgb_convert_sse2)
DECLARE(jsimd_ycc_rgb_convert_avx2)
DECLARE(jsimd_ycc_extrgb_convert_avx2)
DECLARE(jsimd_ycc_extrgbx_convert_avx2)
DECLARE(jsimd_ycc_extbgr_convert_avx2)
DECLARE(jsimd_ycc_extbgrx_convert_avx2)
DECLARE(jsimd_ycc_extxbgr_convert_avx2)
DECLARE(jsimd_ycc_extxrgb_convert_avx2)

typedef void (*convert_fn) (JDIMENSION, JSAMPIMAGE, JDIMENSION, JSAMPARRAY,
                            int);

static const convert_fn kernels_sse2[7] = {
  jsimd_ycc_rgb_convert_sse2, jsimd_ycc_extrgb_convert_sse2,
  jsimd_ycc_extrgbx_convert_sse2, jsimd_ycc_extbgr_convert_sse2,
  jsimd_ycc_extbgrx_convert_sse2, jsimd_ycc_extxbgr_convert_sse2,
  jsimd_ycc_extxrgb_convert_sse2
};

static const convert_fn kernels_avx2[7] = {
  jsimd_ycc_rgb_convert_avx2, jsimd_ycc_extrgb_convert_avx2,
  jsimd_ycc_extrgbx_convert_avx2, jsimd_ycc_extbgr_convert_avx2,
  jsimd_ycc_extbgrx_convert_avx2, jsimd_ycc_extxbgr_convert_avx2,
  jsimd_ycc_extxrgb_convert_avx2
};

static const int pixelsizes[7] = { 3, 3, 4, 3, 4, 4, 4 };

#define NUM_WIDTHS  69

int main(int argc, char **argv)
{
  JDIMENSION widths[NUM_WIDTHS];
  unsigned long long seed = 1;
  const convert_fn *kernels = NULL;
  int format = -1, cases = NUM_WIDTHS, argi, c, i;

  for (argi = 1; argi + 1 < argc; argi += 2) {
    if (!strcmp(argv[argi], "-isa")) {
      if (!strcmp(argv[argi + 1], "sse2"))
        kernels = kernels_sse2;
      else if (!strcmp(argv[argi + 1], "avx2"))
        kernels = kernels_avx2;
    } else if (!strcmp(argv[argi], "-format"))
      format = atoi(argv[argi + 1]);
    else if (!strcmp(argv[argi], "-seed"))
      seed = strtoull(argv[argi + 1], NULL, 10);
    else if (!strcmp(argv[argi], "-cases"))
      cases = atoi(argv[argi + 1]);
    else {
      fprintf(stderr, "unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  if (kernels == NULL || format < 0 || format > 6) {
    fprintf(stderr, "usage: color -isa sse2|avx2 -format 0..6 [-seed S] "
            "[-cases N]\n");
    return 2;
  }

  for (i = 0; i < 66; i++)
    widths[i] = (JDIMENSION)(i + 1);
  widths[66] = 101;
  widths[67] = 227;
  widths[68] = 1000;

  Wp10Seed(seed);
  for (c = 0; c < cases; c++) {
    JDIMENSION width = widths[c % NUM_WIDTHS];
    size_t len = (size_t)width * pixelsizes[format];
    JSAMPARRAY input_buf[3], output_buf;
    JDIMENSION input_row;
    int num_rows, ci, row;
    char key[32];
    size_t col;

    input_row = (JDIMENSION)Wp10Rand(3);
    num_rows = Wp10Rand(4);
    for (ci = 0; ci < 3; ci++) {
      input_buf[ci] = Wp10Rows((int)input_row + num_rows + 1, width);
      Wp10FillRows(input_buf[ci], (int)input_row + num_rows + 1, width);
    }
    output_buf = (JSAMPARRAY)malloc((size_t)(num_rows + 1) *
                                    sizeof(JSAMPROW));
    if (output_buf == NULL) {
      fprintf(stderr, "out of memory\n");
      return 2;
    }
    for (row = 0; row < num_rows; row++) {
      output_buf[row] = (JSAMPROW)malloc(len);
      if (output_buf[row] == NULL) {
        fprintf(stderr, "out of memory\n");
        return 2;
      }
      for (col = 0; col < len; col++)
        output_buf[row][col] = Wp10Sample();
    }

    kernels[format] (width, input_buf, input_row, output_buf, num_rows);

    snprintf(key, sizeof(key), "case%d", c);
    Wp10PrintRows(key, output_buf, num_rows, len);

    for (row = 0; row < num_rows; row++)
      free(output_buf[row]);
    free(output_buf);
    for (ci = 0; ci < 3; ci++)
      Wp10FreeRows(input_buf[ci]);
  }
  return 0;
}

#else

int main(void)
{
  fprintf(stderr, "color: the probe needs the x86-64 SIMD build\n");
  return 2;
}

#endif
