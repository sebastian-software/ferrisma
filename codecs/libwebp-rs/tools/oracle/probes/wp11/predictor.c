// WP11 probe: the image transforms of the lossless encoder
// (src/enc/predictor_enc.c, src/enc/near_lossless_enc.c; ADR 0002). Commands
// on standard input:
//
//   residual W H MIN_BITS MAX_BITS LOW_EFFORT NEAR_LOSSLESS EXACT USED_SG ARGB
//       VP8LResidualImage on the W by H pixels; the scratch rows are sized as
//       AllocateTransformBuffer sizes them and filled with 0xa5 bytes
//   cross W H BITS QUALITY ARGB
//       VP8LColorSpaceTransform
//   sampling FULL_W FULL_H BITS MAX_BITS IMAGE
//       VP8LOptimizeSampling on the subsampled image
//   near W H STRIDE QUALITY ARGB
//       VP8ApplyNearLossless on a picture of the W by H pixels, rows STRIDE
//       pixels apart
//
// Each prints the return value, the error code and progress where they exist,
// the best bits, and the buffers the function writes. tests/wp11_predictor.rs
// generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/lossless.h"
#include "src/dsp/lossless_common.h"
#include "src/enc/vp8li_enc.h"
#include "src/webp/encode.h"
#include "wp11_io.h"

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "predictor < commands.txt");
  (void)argv;
  VP8LEncDspInit();
  while ((command = Wp11ReadToken()) != NULL) {
    if (strcmp(command, "residual") == 0) {
      const int width = (int)Wp11ReadLong();
      const int height = (int)Wp11ReadLong();
      const int min_bits = (int)Wp11ReadLong();
      const int max_bits = (int)Wp11ReadLong();
      const int low_effort = (int)Wp11ReadLong();
      const int near_lossless = (int)Wp11ReadLong();
      const int exact = (int)Wp11ReadLong();
      const int used_subtract_green = (int)Wp11ReadLong();
      size_t n;
      uint32_t* const argb = Wp11ReadU32s(&n);
      const size_t scratch_size =
          (size_t)(width + 1) * 2 + ((size_t)width * 2 + 3) / 4;
      const size_t image_size =
          (size_t)VP8LSubSampleSize(width, 2) * VP8LSubSampleSize(height, 2);
      uint32_t* const scratch = (uint32_t*)malloc(scratch_size * 4 + 4);
      uint32_t* const image = (uint32_t*)malloc(image_size * 4 + 4);
      WebPPicture pic;
      int percent = 0, best_bits = -1, ret;
      if (scratch == NULL || image == NULL || !WebPPictureInit(&pic)) return 2;
      if (n != (size_t)width * height) return 2;
      memset(scratch, 0xa5, scratch_size * 4 + 4);
      memset(image, 0xa5, image_size * 4 + 4);
      ret = VP8LResidualImage(width, height, min_bits, max_bits, low_effort,
                              argb, scratch, image, near_lossless, exact,
                              used_subtract_green, &pic, 40, &percent,
                              &best_bits);
      printf("ret=%d\nerror=%d\npercent=%d\nbest_bits=%d\n", ret,
             (int)pic.error_code, percent, best_bits);
      Wp11PrintU32s("argb", argb, n);
      Wp11PrintU32s("image",
                    image, (size_t)VP8LSubSampleSize(width, best_bits) *
                               VP8LSubSampleSize(height, best_bits));
      free(argb);
      free(scratch);
      free(image);
    } else if (strcmp(command, "cross") == 0) {
      const int width = (int)Wp11ReadLong();
      const int height = (int)Wp11ReadLong();
      const int bits = (int)Wp11ReadLong();
      const int quality = (int)Wp11ReadLong();
      size_t n;
      uint32_t* const argb = Wp11ReadU32s(&n);
      const size_t image_size =
          (size_t)VP8LSubSampleSize(width, bits) * VP8LSubSampleSize(height, bits);
      uint32_t* const image = (uint32_t*)malloc(image_size * 4 + 4);
      WebPPicture pic;
      int percent = 0, best_bits = -1, ret;
      if (image == NULL || !WebPPictureInit(&pic)) return 2;
      memset(image, 0xa5, image_size * 4 + 4);
      ret = VP8LColorSpaceTransform(width, height, bits, quality, argb, image,
                                    &pic, 40, &percent, &best_bits);
      printf("ret=%d\nerror=%d\npercent=%d\nbest_bits=%d\n", ret,
             (int)pic.error_code, percent, best_bits);
      Wp11PrintU32s("argb", argb, n);
      Wp11PrintU32s("image",
                    image, (size_t)VP8LSubSampleSize(width, best_bits) *
                               VP8LSubSampleSize(height, best_bits));
      free(argb);
      free(image);
    } else if (strcmp(command, "sampling") == 0) {
      const int full_width = (int)Wp11ReadLong();
      const int full_height = (int)Wp11ReadLong();
      const int bits = (int)Wp11ReadLong();
      const int max_bits = (int)Wp11ReadLong();
      size_t n;
      uint32_t* const image = Wp11ReadU32s(&n);
      int best_bits = -1;
      VP8LOptimizeSampling(image, full_width, full_height, bits, max_bits,
                           &best_bits);
      printf("best_bits=%d\n", best_bits);
      Wp11PrintU32s("image", image, n);
      free(image);
    } else if (strcmp(command, "near") == 0) {
      const int width = (int)Wp11ReadLong();
      const int height = (int)Wp11ReadLong();
      const int stride = (int)Wp11ReadLong();
      const int quality = (int)Wp11ReadLong();
      size_t n;
      uint32_t* const argb = Wp11ReadU32s(&n);
      uint32_t* const dst = (uint32_t*)malloc((size_t)width * height * 4 + 4);
      WebPPicture pic;
      int ret;
      if (dst == NULL || !WebPPictureInit(&pic)) return 2;
      memset(dst, 0xa5, (size_t)width * height * 4 + 4);
      pic.use_argb = 1;
      pic.width = width;
      pic.height = height;
      pic.argb = argb;
      pic.argb_stride = stride;
      ret = VP8ApplyNearLossless(&pic, quality, dst);
      printf("ret=%d\n", ret);
      Wp11PrintU32s("dst", dst, (size_t)width * height);
      free(argb);
      free(dst);
    } else {
      fprintf(stderr, "predictor: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  return 0;
}
