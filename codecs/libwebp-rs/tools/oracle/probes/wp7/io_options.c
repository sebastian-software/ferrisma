// WP7 probe: WebPIoInitFromOptions and WebPCheckCropDimensions of
// src/dec/webp_dec.c (ADR 0002). Commands on standard input:
//
//   init FILL WIDTH HEIGHT SRC_MODE OPT USE_CROP LEFT TOP CROP_W CROP_H
//        USE_SCALE SCALED_W SCALED_H BYPASS NO_FANCY
//                     prepares an io whose integer fields are FILL and whose
//                     flags are FILL != 0, sets its dimensions, and calls
//                     WebPIoInitFromOptions with the options (NULL when OPT is
//                     0); prints init=0|1 and the io fields
//   crop W H X Y CW CH
//                     prints crop=0|1 of WebPCheckCropDimensions
//
// tests/wp7_io_options.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/webpi_dec.h"
#include "src/webp/decode.h"
#include "probe_support.h"
#include "wp7_io.h"
#include "wp7_state.h"

static void Init(void) {
  const int fill = (int)Wp7ReadLong();
  const int width = (int)Wp7ReadLong();
  const int height = (int)Wp7ReadLong();
  const WEBP_CSP_MODE mode = (WEBP_CSP_MODE)Wp7ReadLong();
  const int use_options = (int)Wp7ReadLong();
  const int flag = (fill != 0);
  WebPDecoderOptions options;
  VP8Io io;
  int ok;

  memset(&options, 0, sizeof(options));
  options.use_cropping = (int)Wp7ReadLong();
  options.crop_left = (int)Wp7ReadLong();
  options.crop_top = (int)Wp7ReadLong();
  options.crop_width = (int)Wp7ReadLong();
  options.crop_height = (int)Wp7ReadLong();
  options.use_scaling = (int)Wp7ReadLong();
  options.scaled_width = (int)Wp7ReadLong();
  options.scaled_height = (int)Wp7ReadLong();
  options.bypass_filtering = (int)Wp7ReadLong();
  options.no_fancy_upsampling = (int)Wp7ReadLong();

  memset(&io, 0, sizeof(io));
  io.width = width;
  io.height = height;
  io.mb_y = io.mb_w = io.mb_h = fill;
  io.y_stride = io.uv_stride = fill;
  io.crop_left = io.crop_right = io.crop_top = io.crop_bottom = fill;
  io.scaled_width = io.scaled_height = fill;
  io.fancy_upsampling = io.bypass_filtering = flag;
  io.use_cropping = io.use_scaling = flag;

  ok = WebPIoInitFromOptions(use_options ? &options : NULL, &io, mode);
  printf("init=%d\n", ok);
  Wp7PrintIo(&io);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "io_options < commands");
  (void)argv;
  while ((command = Wp7ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      Init();
    } else if (strcmp(command, "crop") == 0) {
      const int w = (int)Wp7ReadLong();
      const int h = (int)Wp7ReadLong();
      const int x = (int)Wp7ReadLong();
      const int y = (int)Wp7ReadLong();
      const int cw = (int)Wp7ReadLong();
      const int ch = (int)Wp7ReadLong();
      printf("crop=%d\n", WebPCheckCropDimensions(w, h, x, y, cw, ch));
    } else {
      fprintf(stderr, "io_options: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
