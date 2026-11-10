// WP12 probe: the row counters of the Ferrixel cropping measurement
// (docs/adr/0010-ferrixel-adoption.md). Decodes with the internal API that DecodeInto
// uses, with put() wrapped by a counter, and prints which rows a cropped or
// scaled decoding touches. Commands on standard input:
//
//   rows PATH MODE OPTIONS
//
//   VP8:  mb_h (macroblock rows of the image), parsed (rows that ParseFrame
//         parses: br_mb_y), filter_type, filtered (rows with in-loop
//         filtering: br_mb_y - tl_mb_y when filtering), puts (put() calls),
//         put_first and put_rows (first row and number of rows passed to
//         put(), after the crop), out (output rows, params.last_y).
//   VP8L: height, decoded (rows that DecodeImageData decoded: last_row),
//         out (output rows: last_out_row).
//
// OPTIONS is as in decode.c. tests/wp12_crop_rows.rs compares the Rust
// translation and holds the table of numbers in its documentation.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/vp8i_dec.h"
#include "src/dec/vp8li_dec.h"
#include "src/dec/webpi_dec.h"
#include "src/webp/decode.h"
#include "probe_support.h"

typedef struct {
  VP8IoPutHook put;
  int calls, first, rows;
} Counter;

static Counter g_counter;

static int CountingPut(const VP8Io* const io) {
  if (g_counter.calls == 0) g_counter.first = io->mb_y;
  ++g_counter.calls;
  g_counter.rows += io->mb_h;
  return g_counter.put(io);
}

static void ParseOptions(const char* text, WebPDecoderOptions* const options) {
  int v[14];
  if (sscanf(text, "%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d", &v[0], &v[1],
             &v[2], &v[3], &v[4], &v[5], &v[6], &v[7], &v[8], &v[9], &v[10],
             &v[11], &v[12], &v[13]) != 14) {
    exit(2);
  }
  memset(options, 0, sizeof(*options));
  options->bypass_filtering = v[0];
  options->no_fancy_upsampling = v[1];
  options->use_cropping = v[2];
  options->crop_left = v[3];
  options->crop_top = v[4];
  options->crop_width = v[5];
  options->crop_height = v[6];
  options->use_scaling = v[7];
  options->scaled_width = v[8];
  options->scaled_height = v[9];
  options->use_threads = v[10];
  options->dithering_strength = v[11];
  options->flip = v[12];
  options->alpha_dithering_strength = v[13];
}

static void Rows(const uint8_t* data, size_t size, int mode,
                 const char* option_text) {
  WebPHeaderStructure headers;
  WebPDecoderOptions options;
  WebPDecParams params;
  WebPDecBuffer output;
  VP8Io io;
  ParseOptions(option_text, &options);
  memset(&headers, 0, sizeof(headers));
  headers.data = data;
  headers.data_size = size;
  headers.have_all_data = 1;
  if (WebPParseHeaders(&headers) != VP8_STATUS_OK) exit(2);
  if (!WebPInitDecBuffer(&output) || !VP8InitIo(&io)) exit(2);
  WebPResetDecParams(&params);
  params.output = &output;
  params.options = &options;
  output.colorspace = (WEBP_CSP_MODE)mode;
  io.data = headers.data + headers.offset;
  io.data_size = headers.data_size - headers.offset;
  WebPInitCustomIo(&params, &io);
  memset(&g_counter, 0, sizeof(g_counter));
  g_counter.first = -1;
  if (!headers.is_lossless) {
    VP8Decoder* const dec = VP8New();
    int ok;
    if (dec == NULL) exit(2);
    dec->alpha_data = headers.alpha_data;
    dec->alpha_data_size = headers.alpha_data_size;
    if (!VP8GetHeaders(dec, &io)) exit(2);
    if (WebPAllocateDecBuffer(io.width, io.height, &options, &output) !=
        VP8_STATUS_OK) {
      exit(2);
    }
    dec->mt_method = 0;
    VP8InitDithering(&options, dec);
    g_counter.put = io.put;
    io.put = CountingPut;
    ok = VP8Decode(dec, &io);
    printf("vp8=%d,%d,%d,%d,%d,%d,%d,%d,%d\n", ok, dec->mb_h, dec->br_mb_y,
           dec->filter_type,
           dec->filter_type > 0 ? dec->br_mb_y - dec->tl_mb_y : 0,
           g_counter.calls, g_counter.first, g_counter.rows, params.last_y);
    VP8Delete(dec);
  } else {
    VP8LDecoder* const dec = VP8LNew();
    int ok;
    if (dec == NULL) exit(2);
    if (!VP8LDecodeHeader(dec, &io)) exit(2);
    if (WebPAllocateDecBuffer(io.width, io.height, &options, &output) !=
        VP8_STATUS_OK) {
      exit(2);
    }
    ok = VP8LDecodeImage(dec);
    printf("vp8l=%d,%d,%d,%d\n", ok, io.height, dec->last_row,
           dec->last_out_row);
    VP8LDelete(dec);
  }
  WebPFreeDecBuffer(&output);
}

int main(int argc, char* argv[]) {
  char line[4096];
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "crop_rows < commands");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    char cmd[16], path[2048], options[512];
    int mode;
    size_t size;
    uint8_t* data;
    if (sscanf(line, "%15s %2047s %d %511s", cmd, path, &mode, options) != 4 ||
        strcmp(cmd, "rows")) {
      fprintf(stderr, "bad command: %s", line);
      return 2;
    }
    printf("> %s", line);
    data = ProbeReadInput(path, &size);
    Rows(data, size, mode, options);
    free(data);
    fflush(stdout);
  }
  return 0;
}
