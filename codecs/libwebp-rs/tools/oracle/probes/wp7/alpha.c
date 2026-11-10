// WP7 probe: the alpha path of the lossless decoder in src/dec/vp8l_dec.c,
// VP8LDecodeAlphaHeader and VP8LDecodeAlphaImageStream, set up as ALPHInit and
// ALPHDecode of src/dec/alpha_dec.c set them up (ADR 0002). Commands on
// standard input:
//
//   data HEX                    sets the input to the bytes
//   file PATH OFFSET SIZE       sets the input to SIZE bytes of a file
//   alpha SIZE WIDTH HEIGHT USE_CROP LEFT RIGHT TOP BOTTOM ROWS DUMP
//
// The input is the payload of an ALPH chunk. `alpha` takes the method, filter
// and pre-processing of its first byte, as ALPHInit does, and decodes the
// lossless stream that follows in the first SIZE bytes. The alpha plane is
// WIDTH by HEIGHT bytes, filled with 0xa5. The io carries the dimensions and
// the crop window. ROWS lists the `last_row` arguments of the successive
// VP8LDecodeAlphaImageStream calls; they stop after a failure. After the header
// and each call the probe prints the state and the plane, in hex when DUMP is
// 1 and as a digest otherwise.
//
// tests/wp7_alpha.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/alphai_dec.h"
#include "src/dec/vp8_dec.h"
#include "src/dec/vp8li_dec.h"
#include "src/dec/webpi_dec.h"
#include "src/dsp/dsp.h"
#include "probe_support.h"
#include "wp7_io.h"
#include "wp7_state.h"

static uint8_t* input = NULL;
static size_t input_size = 0;

static void PrintAlpha(const ALPHDecoder* alph_dec, const uint8_t* plane,
                       size_t plane_size, int dump) {
  printf("use_8b_decode=%d\n", alph_dec->use_8b_decode);
  printf("prev_line=%ld\n", alph_dec->prev_line == NULL
                                ? -1L
                                : (long)(alph_dec->prev_line - plane));
  Wp7PrintIo(&alph_dec->io);
  if (dump) {
    ProbePrintHex("plane", plane, plane_size);
  } else {
    Wp7PrintDigest("plane", plane, plane_size);
  }
}

static void Alpha(void) {
  const long size = Wp7ReadLong();
  const int width = (int)Wp7ReadLong();
  const int height = (int)Wp7ReadLong();
  const int use_cropping = (int)Wp7ReadLong();
  const int crop_left = (int)Wp7ReadLong();
  const int crop_right = (int)Wp7ReadLong();
  const int crop_top = (int)Wp7ReadLong();
  const int crop_bottom = (int)Wp7ReadLong();
  size_t num_rows, i;
  long* const rows = Wp7ReadList(&num_rows);
  const int dump = (int)Wp7ReadLong();
  const size_t plane_size = (size_t)width * (size_t)height;
  uint8_t* const plane = (uint8_t*)malloc(plane_size + 1);
  ALPHDecoder* const alph_dec = (ALPHDecoder*)calloc(1, sizeof(*alph_dec));
  VP8Io* const io = &alph_dec->io;
  int ok;

  if (plane == NULL || alph_dec == NULL) exit(2);
  if (size < 1 || (size_t)size > input_size) exit(2);
  memset(plane, 0xa5, plane_size);

  // ALPHInit, without the checks of the header byte and the method.
  VP8FiltersInit();
  alph_dec->output = plane;
  alph_dec->width = width;
  alph_dec->height = height;
  alph_dec->method = (input[0] >> 0) & 0x03;
  alph_dec->filter = (WEBP_FILTER_TYPE)((input[0] >> 2) & 0x03);
  alph_dec->pre_processing = (input[0] >> 4) & 0x03;
  if (!VP8InitIo(io)) exit(2);
  WebPInitCustomIo(NULL, io);
  io->opaque = alph_dec;
  io->width = width;
  io->height = height;
  io->use_cropping = use_cropping;
  io->crop_left = crop_left;
  io->crop_right = crop_right;
  io->crop_top = crop_top;
  io->crop_bottom = crop_bottom;

  ok = VP8LDecodeAlphaHeader(alph_dec, input + 1, (size_t)size - 1);
  printf("header=%d\n", ok);
  printf("method=%d\nfilter=%d\n", alph_dec->method, alph_dec->filter);
  printf("has_decoder=%d\n", alph_dec->vp8l_dec != NULL);
  if (alph_dec->vp8l_dec != NULL) Wp7PrintDecoderFull(alph_dec->vp8l_dec);
  PrintAlpha(alph_dec, plane, plane_size, dump);

  for (i = 0; ok && i < num_rows; ++i) {
    VP8LDecoder* const dec = alph_dec->vp8l_dec;
    ok = VP8LDecodeAlphaImageStream(alph_dec, (int)rows[i]);
    printf("rows=%ld\n", rows[i]);
    printf("image=%d\n", ok);
    Wp7PrintDecoder(dec);
    PrintAlpha(alph_dec, plane, plane_size, dump);
  }

  VP8LDelete(alph_dec->vp8l_dec);
  free(alph_dec);
  free(plane);
  free(rows);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "alpha < commands");
  (void)argv;
  while ((command = Wp7ReadToken()) != NULL) {
    if (strcmp(command, "data") == 0) {
      free(input);
      input = Wp7ReadBytes(&input_size);
    } else if (strcmp(command, "file") == 0) {
      char* const path = Wp7ReadToken();
      const long offset = Wp7ReadLong();
      const long file_size = Wp7ReadLong();
      if (path == NULL) exit(2);
      free(input);
      input = Wp7ReadFile(path, offset, file_size);
      input_size = (size_t)file_size;
      free(path);
    } else if (strcmp(command, "alpha") == 0) {
      Alpha();
    } else {
      fprintf(stderr, "alpha: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  free(input);
  return 0;
}
