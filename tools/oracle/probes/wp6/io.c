// WP6 probe: the output functions of src/dec/io_dec.c through
// WebPInitCustomIo (CustomSetup, CustomPut with the emit functions, the
// rescalers and the alpha emitters, CustomTeardown), fed with the row chunks
// that FinishRow of src/dec/frame_dec.c passes to put() (ADR 0002). Command on
// standard input, repeated:
//
//   case MODE W H EXTRA HAS_OPTIONS [BYPASS NOFANCY USECROP LEFT TOP CW CH
//        USESCALE SW SH] Y U V A OUTW OUTH REGIONS PLANES... DUMP
//
// MODE is the WEBP_CSP_MODE of the output. Y, U, V and A are the source
// planes of a W by H picture in hex (A is "-" without alpha), with strides W,
// (W + 1) / 2 and W. EXTRA is the number of rows that the in-loop filter holds
// back (0, 2 or 8). REGIONS lists the sizes of the output allocations, each
// filled with 0xa5. PLANES are REGION OFFSET STRIDE SIZE quadruples: one for
// the RGBA modes, four (Y, U, V, A; REGION -1 for no A) for the YUV modes. A
// negative stride with an offset at the last row is a flipped buffer. The
// probe runs setup, then put() on each row chunk as FinishRow computes it from
// the io after setup, stopping at the first failing put, then teardown. It
// prints the result of each call, the io after setup, `last_y`, and the output
// regions (in hex when DUMP is 1, as digests otherwise).
//
// tests/wp6_io.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/webpi_dec.h"
#include "src/webp/decode.h"
#include "probe_support.h"
#include "wp6_io.h"
#include "wp6_state.h"

#define MAX_REGIONS 4

static void PrintRegions(uint8_t* regions[], const long* sizes, size_t count,
                         int dump) {
  size_t r;
  for (r = 0; r < count; ++r) {
    char key[16];
    snprintf(key, sizeof(key), "region%zu", r);
    if (dump) {
      ProbePrintHex(key, regions[r], (size_t)sizes[r]);
    } else {
      Wp6PrintDigest(key, regions[r], (size_t)sizes[r]);
    }
  }
}

static uint8_t* ReadPlane(long* region, uint8_t* regions[], size_t count,
                          int* stride, size_t* size) {
  const long r = Wp6ReadLong();
  const long offset = Wp6ReadLong();
  *stride = (int)Wp6ReadLong();
  *size = (size_t)Wp6ReadLong();
  *region = r;
  if (r < 0) return NULL;
  if ((size_t)r >= count) exit(2);
  return regions[r] + offset;
}

static void Case(void) {
  const WEBP_CSP_MODE mode = (WEBP_CSP_MODE)Wp6ReadLong();
  const int width = (int)Wp6ReadLong();
  const int height = (int)Wp6ReadLong();
  const int extra = (int)Wp6ReadLong();
  const int has_options = (int)Wp6ReadLong();
  WebPDecoderOptions options;
  size_t y_size, u_size, v_size, a_size, num_regions, r;
  uint8_t *y, *u, *v, *a;
  uint8_t* regions[MAX_REGIONS] = { NULL, NULL, NULL, NULL };
  long* sizes;
  int out_w, out_h, dump, ok, mb_y, mb_h, br_mb_y;
  WebPDecBuffer output;
  WebPDecParams params;
  VP8Io io;

  memset(&options, 0, sizeof(options));
  if (has_options) {
    options.bypass_filtering = (int)Wp6ReadLong();
    options.no_fancy_upsampling = (int)Wp6ReadLong();
    options.use_cropping = (int)Wp6ReadLong();
    options.crop_left = (int)Wp6ReadLong();
    options.crop_top = (int)Wp6ReadLong();
    options.crop_width = (int)Wp6ReadLong();
    options.crop_height = (int)Wp6ReadLong();
    options.use_scaling = (int)Wp6ReadLong();
    options.scaled_width = (int)Wp6ReadLong();
    options.scaled_height = (int)Wp6ReadLong();
  }
  y = Wp6ReadBytes(&y_size);
  u = Wp6ReadBytes(&u_size);
  v = Wp6ReadBytes(&v_size);
  a = Wp6ReadBytes(&a_size);
  out_w = (int)Wp6ReadLong();
  out_h = (int)Wp6ReadLong();
  sizes = Wp6ReadList(&num_regions);
  if (num_regions == 0 || num_regions > MAX_REGIONS) exit(2);
  for (r = 0; r < num_regions; ++r) {
    regions[r] = (uint8_t*)malloc((size_t)sizes[r] + 1);
    if (regions[r] == NULL) exit(2);
    memset(regions[r], 0xa5, (size_t)sizes[r]);
  }

  if (!WebPInitDecBuffer(&output)) exit(2);
  output.colorspace = mode;
  output.width = out_w;
  output.height = out_h;
  output.is_external_memory = 1;
  if (WebPIsRGBMode(mode)) {
    long region;
    WebPRGBABuffer* const buf = &output.u.RGBA;
    buf->rgba = ReadPlane(&region, regions, num_regions, &buf->stride,
                          &buf->size);
  } else {
    long region;
    WebPYUVABuffer* const buf = &output.u.YUVA;
    buf->y = ReadPlane(&region, regions, num_regions, &buf->y_stride,
                       &buf->y_size);
    buf->u = ReadPlane(&region, regions, num_regions, &buf->u_stride,
                       &buf->u_size);
    buf->v = ReadPlane(&region, regions, num_regions, &buf->v_stride,
                       &buf->v_size);
    buf->a = ReadPlane(&region, regions, num_regions, &buf->a_stride,
                       &buf->a_size);
  }
  dump = (int)Wp6ReadLong();

  memset(&params, 0, sizeof(params));
  params.output = &output;
  params.options = has_options ? &options : NULL;
  if (!VP8InitIo(&io)) exit(2);
  WebPInitCustomIo(&params, &io);
  // As VP8GetHeaders sets them up.
  io.width = width;
  io.height = height;
  io.use_cropping = 0;
  io.crop_top = 0;
  io.crop_left = 0;
  io.crop_right = width;
  io.crop_bottom = height;
  io.use_scaling = 0;
  io.scaled_width = width;
  io.scaled_height = height;
  io.mb_w = width;
  io.mb_h = height;
  // As InitIo of frame_dec.c sets them up, with the source planes as cache.
  io.y_stride = width;
  io.uv_stride = (width + 1) / 2;

  ok = io.setup(&io);
  printf("setup=%d\n", ok);
  Wp6PrintIo(&io);
  if (ok) {
    // The row chunks of FinishRow.
    mb_h = (height + 15) >> 4;
    br_mb_y = (io.crop_bottom + 15 + extra) >> 4;
    if (br_mb_y > mb_h) br_mb_y = mb_h;
    for (mb_y = 0; mb_y < br_mb_y && ok; ++mb_y) {
      const int is_first_row = (mb_y == 0);
      const int is_last_row = (mb_y >= br_mb_y - 1);
      int y_start = mb_y * 16;
      int y_end = (mb_y + 1) * 16;
      if (!is_first_row) y_start -= extra;
      if (!is_last_row) y_end -= extra;
      if (y_end > io.crop_bottom) y_end = io.crop_bottom;
      io.y = y + (size_t)y_start * io.y_stride;
      io.u = u + (size_t)(y_start >> 1) * io.uv_stride;
      io.v = v + (size_t)(y_start >> 1) * io.uv_stride;
      io.a = NULL;
      if (a_size > 0 && y_start < y_end) {
        io.a = a + (size_t)y_start * width;
      }
      if (y_start < io.crop_top) {
        const int delta_y = io.crop_top - y_start;
        y_start = io.crop_top;
        io.y += io.y_stride * delta_y;
        io.u += io.uv_stride * (delta_y >> 1);
        io.v += io.uv_stride * (delta_y >> 1);
        if (io.a != NULL) io.a += io.width * delta_y;
      }
      if (y_start < y_end) {
        io.y += io.crop_left;
        io.u += io.crop_left >> 1;
        io.v += io.crop_left >> 1;
        if (io.a != NULL) io.a += io.crop_left;
        io.mb_y = y_start - io.crop_top;
        io.mb_w = io.crop_right - io.crop_left;
        io.mb_h = y_end - y_start;
        ok = io.put(&io);
        printf("put=%d,%d,%d,%d last_y=%d\n", ok, io.mb_y, io.mb_w, io.mb_h,
               params.last_y);
        PrintRegions(regions, sizes, num_regions, dump);
      }
    }
    io.teardown(&io);
  }
  PrintRegions(regions, sizes, num_regions, 1);
  for (r = 0; r < num_regions; ++r) free(regions[r]);
  free(sizes);
  free(y);
  free(u);
  free(v);
  free(a);
}

int main(int argc, char* argv[]) {
  char* command;
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "io < commands");
  while ((command = Wp6ReadToken()) != NULL) {
    if (!strcmp(command, "case")) {
      Case();
    } else {
      fprintf(stderr, "wp6: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
