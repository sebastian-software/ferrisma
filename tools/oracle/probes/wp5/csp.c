// WP5 probe: the color space and matrix functions of sharpyuv/sharpyuv_csp.c and
// sharpyuv/sharpyuv.c (SharpYuvGetVersion, SharpYuvOptionsInit), run on the
// inventory of libavif-rs docs/SHARPYUV-VALIDATION.md (version, predefined and
// computed matrices, options version checks) and on 256 seeded color spaces.
//
// Output, one group per inventory item:
//   group=NAME, then the fields named in the item. Matrices print their twelve
//   coefficients as `values=a,b,...`. A NULL matrix prints `values=null`.
//
// The NULL arguments of SharpYuvOptionsInitInternal are not probed here: the
// Rust API takes references, so they exist only at the C ABI.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "probe_support.h"
#include "sharpyuv/sharpyuv.h"
#include "sharpyuv/sharpyuv_csp.h"
#include "wp5_rng.h"

static void PrintMatrix(const char* name, const SharpYuvConversionMatrix* m) {
  int i;
  printf("group=%s\n", name);
  if (m == NULL) {
    printf("values=null\n");
    return;
  }
  printf("values=%d", m->rgb_to_y[0]);
  for (i = 1; i < 4; ++i) printf(",%d", m->rgb_to_y[i]);
  for (i = 0; i < 4; ++i) printf(",%d", m->rgb_to_u[i]);
  for (i = 0; i < 4; ++i) printf(",%d", m->rgb_to_v[i]);
  printf("\n");
}

static void PrintComputed(const char* name, float kr, float kb, int depth,
                          int range) {
  SharpYuvColorSpace space;
  SharpYuvConversionMatrix matrix;
  space.kr = kr;
  space.kb = kb;
  space.bit_depth = depth;
  space.range = (SharpYuvRange)range;
  SharpYuvComputeConversionMatrix(&space, &matrix);
  PrintMatrix(name, &matrix);
}

int main(int argc, char* argv[]) {
  static const double kInventoryKr[3] = {0.299, 0.2126, 0.2627};
  static const double kInventoryKb[3] = {0.114, 0.0722, 0.0593};
  static const unsigned int kOptionVersions[6] = {0x40002, 0x40000, 0x400ff,
                                                  0x30000, 0x1000000, 0};
  static const unsigned int kMatrixTypes[8] = {0, 1, 2, 3, 4, 5, 6, 0xffffffffu};
  const SharpYuvConversionMatrix* matrix;
  char name[64];
  int i, k, depth, range, index;
  Wp5Rng rng;

  ProbeCheckArgs(argc, 0, 0, "csp");

  printf("group=version\nvalue=%d\n", SharpYuvGetVersion());

  for (i = 0; i < 5; ++i) {
    snprintf(name, sizeof(name), "predefined-matrix-%d", i);
    matrix = SharpYuvGetConversionMatrix((SharpYuvMatrixType)i);
    if (matrix == NULL) {
      fprintf(stderr, "csp: predefined matrix %d is NULL\n", i);
      return 3;
    }
    PrintMatrix(name, matrix);
  }

  // Out-of-range types, including the value of kSharpYuvMatrixNum.
  printf("group=invalid-matrix\n");
  printf("values=%d,%d,%d\n",
         SharpYuvGetConversionMatrix((SharpYuvMatrixType)5u) != NULL,
         SharpYuvGetConversionMatrix((SharpYuvMatrixType)6u) != NULL,
         SharpYuvGetConversionMatrix((SharpYuvMatrixType)0xffffffffu) != NULL);
  for (i = 0; i < 8; ++i) {
    matrix = SharpYuvGetConversionMatrix((SharpYuvMatrixType)kMatrixTypes[i]);
    if ((matrix != NULL) != (kMatrixTypes[i] < 5)) {
      fprintf(stderr, "csp: matrix type %u\n", kMatrixTypes[i]);
      return 3;
    }
  }

  index = 0;
  for (i = 0; i < 3; ++i) {
    for (depth = 8; depth <= 12; depth += 2) {
      for (range = 0; range <= 1; ++range) {
        snprintf(name, sizeof(name), "computed-matrix-%d", index++);
        PrintComputed(name, (float)kInventoryKr[i], (float)kInventoryKb[i], depth,
                      range);
      }
    }
  }

  // Options versions: the major number must be 0 and the minor number 4.
  for (i = 0; i < 6; ++i) {
    SharpYuvOptions options;
    int code;
    options.yuv_matrix = NULL;
    options.transfer_type = (SharpYuvTransferFunctionType)99;
    matrix = SharpYuvGetConversionMatrix((SharpYuvMatrixType)0);
    code = SharpYuvOptionsInitInternal(matrix, &options, (int)kOptionVersions[i]);
    snprintf(name, sizeof(name), "options-version-%u", kOptionVersions[i]);
    printf("group=%s\n", name);
    printf("code=%d\n", code);
    printf("transfer=%u\n", (unsigned int)options.transfer_type);
    printf("matrix=%d\n", options.yuv_matrix != NULL);
  }

  // NULL arguments are not printed, because the Rust API has no NULL. The C
  // inventory requires the result 0 for each of them.
  {
    SharpYuvOptions null_options;
    matrix = SharpYuvGetConversionMatrix((SharpYuvMatrixType)0);
    if (SharpYuvOptionsInitInternal(matrix, NULL, 0x40002) != 0 ||
        SharpYuvOptionsInitInternal(NULL, &null_options, 0x40002) != 0) {
      fprintf(stderr, "csp: a NULL argument was accepted\n");
      return 4;
    }
  }

  // Seeded color spaces beyond the inventory. Inside SharpYuvComputeConversionMatrix
  // every intermediate must fit int (ground rule 8): ToFixed16 converts
  // f * 65536 + 0.5 to int, so |f| must stay below 32768. Here kr and kb are drawn
  // from [0, 0.5), which keeps 1 - kr and 1 - kb at least 0.5, so every scale factor
  // is at most 1 and every coefficient is at most 1 in magnitude. The bit depth is
  // drawn from 8 to 15: at 16 bits the offset 128 << 8 times 65536 overflows int for
  // every color space, which is undefined in C (docs/status/wp5.md).
  rng.state = 0x636f6c6f72ULL;
  for (k = 0; k < 256; ++k) {
    const float kr = (float)(Wp5RngNext(&rng) >> 40) / 33554432.0f;
    const float kb = (float)(Wp5RngNext(&rng) >> 40) / 33554432.0f;
    const int bit_depth = 8 + (int)Wp5RngBelow(&rng, 8);
    const int value_range = (int)Wp5RngBelow(&rng, 2);
    snprintf(name, sizeof(name), "computed-random-%d", k);
    PrintComputed(name, kr, kb, bit_depth, value_range);
  }
  return 0;
}
