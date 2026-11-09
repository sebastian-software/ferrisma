// WP15 probe: WebPAnimEncoderRefineRect() of src/mux/anim_encode.c, which the
// internal header src/mux/animi.h declares (no public header does, so the C
// ABI does not export it and the probe builds against the C library only).
// Commands on standard input, one per line:
//
//   refine PREV CURR W H ARGB LOSSLESS QUALITY X Y WIDTH HEIGHT
//       imports the W x H RGBA files PREV and CURR into two pictures (ARGB 1:
//       use_argb, 0: YUV) and prints the result and the rectangle of
//       WebPAnimEncoderRefineRect() from the given one; QUALITY is a float
//
// tests/wp15_refine_rect.rs reproduces the output with the translation.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/mux/animi.h"
#include "src/webp/encode.h"
#include "probe_support.h"

static void Import(const char* path, int width, int height, int argb,
                   WebPPicture* const pic) {
  size_t size;
  uint8_t* const rgba = ProbeReadInput(path, &size);
  if (!WebPPictureInit(pic)) abort();
  pic->width = width;
  pic->height = height;
  pic->use_argb = argb;
  if (size != (size_t)width * height * 4 ||
      !WebPPictureImportRGBA(pic, rgba, width * 4)) {
    fprintf(stderr, "refine_rect probe: bad picture %s\n", path);
    exit(2);
  }
  free(rgba);
}

int main(int argc, char* argv[]) {
  char line[4096];
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "refine_rect < commands");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    char prev_path[1024], curr_path[1024];
    int width, height, argb, lossless, x, y, w, h, ok;
    float quality;
    WebPPicture prev, curr;
    if (sscanf(line, "refine %1023s %1023s %d %d %d %d %f %d %d %d %d",
               prev_path, curr_path, &width, &height, &argb, &lossless,
               &quality, &x, &y, &w, &h) != 11) {
      fprintf(stderr, "refine_rect probe: bad command %s", line);
      return 2;
    }
    Import(prev_path, width, height, argb, &prev);
    Import(curr_path, width, height, 1, &curr);
    ok = WebPAnimEncoderRefineRect(&prev, &curr, lossless, quality, &x, &y, &w,
                                   &h);
    printf("refine=%d,%d,%d,%d,%d\n", ok, x, y, w, h);
    WebPPictureFree(&prev);
    WebPPictureFree(&curr);
    fflush(stdout);
  }
  return 0;
}
