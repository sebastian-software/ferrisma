// WP7 probe: the lossless decoder of src/dec/vp8l_dec.c, driven through
// VP8LDecodeHeader and VP8LDecodeImage as src/dec/webp_dec.c and
// src/dec/idec_dec.c drive it (ADR 0002). Commands on standard input:
//
//   data HEX                    sets the input to the bytes
//   file PATH OFFSET SIZE       sets the input to SIZE bytes of a file
//   decode HEADER_SIZE MODE OPT USE_CROP LEFT TOP WIDTH HEIGHT USE_SCALE
//          SCALED_W SCALED_H BYPASS NO_FANCY OUT_W OUT_H S0 S1 S2 S3 FLIP
//          STEPS DUMP
//
// `decode` creates a decoder, an io (VP8InitIo, WebPInitCustomIo) and the
// parameters (WebPResetDecParams), with the options when OPT is 1 and NULL
// otherwise. It decodes the header from the first HEADER_SIZE input bytes and
// prints the state. The output buffer is external memory of OUT_W by OUT_H
// pixels in colorspace MODE, filled with 0xa5: one plane with stride S0 for
// the RGB modes, or Y, U, V and A planes with strides S0 to S3 for the YUV
// modes, without an A plane when S3 is 0. FLIP negates the strides and starts
// each plane at its last row, as WebPFlipBuffer does. STEPS lists pairs
// SIZE,INCREMENTAL: each step sets the reader to the first SIZE bytes
// (VP8LBitReaderSetBuffer, as the incremental decoder does), sets
// dec->incremental, and calls VP8LDecodeImage. The steps stop after a failure
// or when the status is no longer VP8_STATUS_SUSPENDED. After each step the
// probe prints the state and each plane, in hex when DUMP is 1 and as a digest
// otherwise.
//
// tests/wp7_decode.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/vp8li_dec.h"
#include "src/dec/webpi_dec.h"
#include "src/utils/bit_reader_utils.h"
#include "src/webp/decode.h"
#include "probe_support.h"
#include "wp7_io.h"
#include "wp7_state.h"

static uint8_t* input = NULL;
static size_t input_size = 0;

typedef struct {
  uint8_t* memory;
  size_t size;
  int stride;
} Plane;

static void AllocatePlane(Plane* plane, int stride, int rows) {
  plane->stride = stride;
  plane->size = (size_t)stride * (size_t)rows;
  plane->memory = (uint8_t*)malloc(plane->size + 1);
  if (plane->memory == NULL) exit(2);
  memset(plane->memory, 0xa5, plane->size);
}

// The plane pointer and stride for the buffer, flipped as WebPFlipBuffer does.
static uint8_t* PlaneStart(const Plane* plane, int rows, int flip,
                           int* stride) {
  if (flip) {
    *stride = -plane->stride;
    return plane->memory + (size_t)(rows - 1) * plane->stride;
  }
  *stride = plane->stride;
  return plane->memory;
}

static void PrintPlane(const char* key, const Plane* plane, int dump) {
  if (plane->memory == NULL) return;
  if (dump) {
    ProbePrintHex(key, plane->memory, plane->size);
  } else {
    Wp7PrintDigest(key, plane->memory, plane->size);
  }
}

static void Decode(void) {
  const long header_size = Wp7ReadLong();
  const WEBP_CSP_MODE mode = (WEBP_CSP_MODE)Wp7ReadLong();
  const int use_options = (int)Wp7ReadLong();
  WebPDecoderOptions options;
  int out_w, out_h, strides[4], flip, dump;
  long* steps;
  size_t num_steps, step;
  Plane planes[4];
  VP8LDecoder* dec;
  VP8Io io;
  WebPDecParams params;
  WebPDecBuffer output;
  int ok, i;

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
  out_w = (int)Wp7ReadLong();
  out_h = (int)Wp7ReadLong();
  for (i = 0; i < 4; ++i) strides[i] = (int)Wp7ReadLong();
  flip = (int)Wp7ReadLong();
  steps = Wp7ReadList(&num_steps);
  dump = (int)Wp7ReadLong();
  if (header_size < 0 || (size_t)header_size > input_size ||
      num_steps % 2 != 0) {
    exit(2);
  }

  memset(planes, 0, sizeof(planes));
  WebPInitDecBuffer(&output);
  output.colorspace = mode;
  output.width = out_w;
  output.height = out_h;
  output.is_external_memory = 1;
  if (WebPIsRGBMode(mode)) {
    WebPRGBABuffer* const buf = &output.u.RGBA;
    AllocatePlane(&planes[0], strides[0], out_h);
    buf->rgba = PlaneStart(&planes[0], out_h, flip, &buf->stride);
    buf->size = planes[0].size;
  } else {
    WebPYUVABuffer* const buf = &output.u.YUVA;
    const int uv_h = (out_h + 1) / 2;
    AllocatePlane(&planes[0], strides[0], out_h);
    AllocatePlane(&planes[1], strides[1], uv_h);
    AllocatePlane(&planes[2], strides[2], uv_h);
    buf->y = PlaneStart(&planes[0], out_h, flip, &buf->y_stride);
    buf->u = PlaneStart(&planes[1], uv_h, flip, &buf->u_stride);
    buf->v = PlaneStart(&planes[2], uv_h, flip, &buf->v_stride);
    buf->y_size = planes[0].size;
    buf->u_size = planes[1].size;
    buf->v_size = planes[2].size;
    if (strides[3] > 0) {
      AllocatePlane(&planes[3], strides[3], out_h);
      buf->a = PlaneStart(&planes[3], out_h, flip, &buf->a_stride);
      buf->a_size = planes[3].size;
    }
  }

  dec = VP8LNew();
  if (dec == NULL) exit(2);
  if (!VP8InitIo(&io)) exit(2);
  WebPResetDecParams(&params);
  params.output = &output;
  params.options = use_options ? &options : NULL;
  WebPInitCustomIo(&params, &io);
  io.data = input;
  io.data_size = (size_t)header_size;

  ok = VP8LDecodeHeader(dec, &io);
  printf("header=%d\n", ok);
  Wp7PrintIo(&io);
  Wp7PrintDecoderFull(dec);

  for (step = 0; ok && step < num_steps; step += 2) {
    const long size = steps[step];
    if (size < header_size || (size_t)size > input_size) exit(2);
    VP8LBitReaderSetBuffer(&dec->br, input, (size_t)size);
    dec->incremental = (int)steps[step + 1];
    ok = VP8LDecodeImage(dec);
    printf("step=%ld\n", size);
    printf("image=%d\n", ok);
    printf("last_y=%d\n", params.last_y);
    Wp7PrintIo(&io);
    Wp7PrintDecoder(dec);
    PrintPlane("plane0", &planes[0], dump);
    PrintPlane("plane1", &planes[1], dump);
    PrintPlane("plane2", &planes[2], dump);
    PrintPlane("plane3", &planes[3], dump);
    if (dec->status != VP8_STATUS_SUSPENDED) break;
  }

  VP8LDelete(dec);
  free(steps);
  for (i = 0; i < 4; ++i) free(planes[i].memory);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "decode < commands");
  (void)argv;
  while ((command = Wp7ReadToken()) != NULL) {
    if (strcmp(command, "data") == 0) {
      free(input);
      input = Wp7ReadBytes(&input_size);
    } else if (strcmp(command, "file") == 0) {
      char* const path = Wp7ReadToken();
      const long offset = Wp7ReadLong();
      const long size = Wp7ReadLong();
      if (path == NULL) exit(2);
      free(input);
      input = Wp7ReadFile(path, offset, size);
      input_size = (size_t)size;
      free(path);
    } else if (strcmp(command, "decode") == 0) {
      Decode();
    } else {
      fprintf(stderr, "decode: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  free(input);
  return 0;
}
