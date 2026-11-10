// WP6 probe: the VP8 decoder of src/dec/vp8_dec.c and frame_dec.c, driven
// through VP8GetHeaders and VP8Decode with VP8Io hooks that print the rows
// that put() receives, and the alpha decoding of src/dec/alpha_dec.c through
// VP8DecompressAlphaRows (ADR 0002). Commands on standard input:
//
//   decode PATH OFFSET SIZE ALPHA_OFFSET ALPHA_SIZE MT DITHER ALPHA_DITHER
//          BYPASS NOFANCY USECROP LEFT TOP CW CH SETUP_FAIL PUT_FAIL DUMP
//
//     Decodes SIZE bytes at OFFSET of the file PATH as io->data, with the
//     ALPH payload of ALPHA_SIZE bytes at ALPHA_OFFSET as dec->alpha_data
//     (none when ALPHA_SIZE is -1). After VP8GetHeaders it sets
//     dec->mt_method = MT and calls VP8InitDithering with the dithering
//     strengths; then VP8Decode. The setup hook applies the options with
//     WebPIoInitFromOptions (MODE_YUV) and fails when SETUP_FAIL is 1. The
//     put hook prints its io and the visible rows (mb_w by mb_h of y, u, v
//     and a), in hex when DUMP is 1 and as digests otherwise, and fails at the
//     call numbered PUT_FAIL (-1: never). The teardown hook prints a line.
//
//   decodehex DATA ALPHA ALPHA_SIZE MT ...
//
//     The same with the VP8 data and the ALPH payload in hex (ALPHA_SIZE -1:
//     no alpha, ALPHA is then "-").
//
//   alpha PATH ALPHA_OFFSET ALPHA_SIZE WIDTH HEIGHT USECROP LEFT RIGHT TOP
//         BOTTOM ALPHA_DITHER ROWS
//
//     Calls VP8DecompressAlphaRows on a new decoder with the ALPH payload and
//     an io of the given size and crop window, for the (row, num_rows) pairs
//     of ROWS (a list of 2 * n integers), and prints the result, the status
//     and the alpha plane after each call.
//
// tests/wp6_decode.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/vp8i_dec.h"
#include "src/dec/webpi_dec.h"
#include "src/webp/decode.h"
#include "probe_support.h"
#include "wp6_io.h"
#include "wp6_state.h"

typedef struct {
  WebPDecoderOptions options;
  int setup_fail;
  int put_fail;
  int put_count;
  int dump;
} HookState;

static void PrintRows(const char* key, const uint8_t* rows, int stride,
                      int width, int height, int dump) {
  const size_t size = (size_t)width * (size_t)height;
  uint8_t* const visible = (uint8_t*)malloc(size + 1);
  int j;
  if (visible == NULL) exit(2);
  for (j = 0; j < height; ++j) {
    memcpy(visible + (size_t)j * width, rows + (size_t)j * stride, width);
  }
  if (dump) {
    ProbePrintHex(key, visible, size);
  } else {
    Wp6PrintDigest(key, visible, size);
  }
  free(visible);
}

static int Setup(VP8Io* const io) {
  HookState* const state = (HookState*)io->opaque;
  const int ok = WebPIoInitFromOptions(&state->options, io, MODE_YUV);
  printf("setup=%d\n", ok);
  Wp6PrintIo(io);
  return ok && !state->setup_fail;
}

static int Put(const VP8Io* const io) {
  HookState* const state = (HookState*)io->opaque;
  const int uv_w = (io->mb_w + 1) / 2;
  const int uv_h = (io->mb_h + 1) / 2;
  const int ok = (state->put_count != state->put_fail);
  printf("put=%d mb=%d,%d,%d a=%d\n", state->put_count, io->mb_y, io->mb_w,
         io->mb_h, io->a != NULL);
  PrintRows("y", io->y, io->y_stride, io->mb_w, io->mb_h, state->dump);
  PrintRows("u", io->u, io->uv_stride, uv_w, uv_h, state->dump);
  PrintRows("v", io->v, io->uv_stride, uv_w, uv_h, state->dump);
  if (io->a != NULL) {
    PrintRows("a", io->a, io->width, io->mb_w, io->mb_h, state->dump);
  }
  ++state->put_count;
  return ok;
}

static void Teardown(const VP8Io* const io) {
  const HookState* const state = (const HookState*)io->opaque;
  printf("teardown puts=%d\n", state->put_count);
}

static void Decode(int from_file) {
  char* const path = from_file ? Wp6ReadToken() : NULL;
  long offset = 0, size = 0, alpha_offset = 0, alpha_size = -1;
  uint8_t* hex_data = NULL;
  uint8_t* hex_alpha = NULL;
  int mt;
  HookState state;
  uint8_t *data, *alpha = NULL;
  VP8Decoder* dec;
  VP8Io io;
  int ok;

  if (from_file) {
    offset = Wp6ReadLong();
    size = Wp6ReadLong();
    alpha_offset = Wp6ReadLong();
    alpha_size = Wp6ReadLong();
  } else {
    size_t data_size, a_size;
    hex_data = Wp6ReadBytes(&data_size);
    hex_alpha = Wp6ReadBytes(&a_size);
    size = (long)data_size;
    alpha_size = (long)Wp6ReadLong();
    if (alpha_size >= 0 && (size_t)alpha_size != a_size) exit(2);
  }
  mt = (int)Wp6ReadLong();
  memset(&state, 0, sizeof(state));
  state.options.dithering_strength = (int)Wp6ReadLong();
  state.options.alpha_dithering_strength = (int)Wp6ReadLong();
  state.options.bypass_filtering = (int)Wp6ReadLong();
  state.options.no_fancy_upsampling = (int)Wp6ReadLong();
  state.options.use_cropping = (int)Wp6ReadLong();
  state.options.crop_left = (int)Wp6ReadLong();
  state.options.crop_top = (int)Wp6ReadLong();
  state.options.crop_width = (int)Wp6ReadLong();
  state.options.crop_height = (int)Wp6ReadLong();
  state.setup_fail = (int)Wp6ReadLong();
  state.put_fail = (int)Wp6ReadLong();
  state.dump = (int)Wp6ReadLong();
  if (from_file) {
    if (path == NULL) exit(2);
    data = Wp6ReadFile(path, offset, size);
    if (alpha_size >= 0) alpha = Wp6ReadFile(path, alpha_offset, alpha_size);
  } else {
    data = hex_data;
    alpha = (alpha_size >= 0) ? hex_alpha : NULL;
    if (alpha == NULL) free(hex_alpha);
  }

  dec = VP8New();
  if (dec == NULL || !VP8InitIo(&io)) exit(2);
  io.data = data;
  io.data_size = (size_t)size;
  io.setup = Setup;
  io.put = Put;
  io.teardown = Teardown;
  io.opaque = &state;
  dec->alpha_data = alpha;
  dec->alpha_data_size = alpha_size >= 0 ? (size_t)alpha_size : 0;

  ok = VP8GetHeaders(dec, &io);
  printf("headers=%d\n", ok);
  Wp6PrintHeaders(dec, data);
  Wp6PrintIo(&io);
  if (ok) {
    dec->mt_method = mt;
    VP8InitDithering(&state.options, dec);
    printf("dither=%d alpha_dithering=%d\n", dec->dither,
           dec->alpha_dithering);
    ok = VP8Decode(dec, &io);
    printf("decode=%d status=%d error=%s\n", ok, dec->status,
           dec->error_msg == NULL ? "(null)" : dec->error_msg);
    printf("ready=%d is_alpha_decoded=%d\n", dec->ready,
           dec->is_alpha_decoded);
    Wp6PrintIo(&io);
  }
  VP8Delete(dec);
  free(alpha);
  free(data);
  free(path);
}

static void Alpha(void) {
  char* const path = Wp6ReadToken();
  const long alpha_offset = Wp6ReadLong();
  const long alpha_size = Wp6ReadLong();
  size_t num_rows, i;
  uint8_t* alpha;
  long* rows;
  VP8Decoder* dec;
  VP8Io io;

  if (path == NULL || !VP8InitIo(&io)) exit(2);
  io.width = (int)Wp6ReadLong();
  io.height = (int)Wp6ReadLong();
  io.use_cropping = (int)Wp6ReadLong();
  io.crop_left = (int)Wp6ReadLong();
  io.crop_right = (int)Wp6ReadLong();
  io.crop_top = (int)Wp6ReadLong();
  io.crop_bottom = (int)Wp6ReadLong();
  dec = VP8New();
  if (dec == NULL) exit(2);
  dec->alpha_dithering = (int)Wp6ReadLong();
  rows = Wp6ReadList(&num_rows);
  alpha = Wp6ReadFile(path, alpha_offset, alpha_size);
  dec->alpha_data = alpha;
  dec->alpha_data_size = (size_t)alpha_size;
  for (i = 0; i + 1 < num_rows; i += 2) {
    const uint8_t* const out =
        VP8DecompressAlphaRows(dec, &io, (int)rows[i], (int)rows[i + 1]);
    printf("rows=%ld,%ld out=%ld status=%d decoded=%d dithering=%d\n",
           rows[i], rows[i + 1],
           out == NULL ? -1L : (long)(out - dec->alpha_plane), dec->status,
           dec->is_alpha_decoded, dec->alpha_dithering);
    if (dec->alpha_plane != NULL) {
      // The plane is malloc'ed: print only the rows that the calls decoded,
      // from the crop top on (the lossless path may skip the rows above it).
      const long end = dec->is_alpha_decoded ? io.crop_bottom
                                             : rows[i] + rows[i + 1];
      const long start = io.crop_top < end ? io.crop_top : end;
      Wp6PrintDigest("plane", dec->alpha_plane + (size_t)start * io.width,
                     (size_t)(end - start) * (size_t)io.width);
    } else {
      printf("plane=null\n");
    }
    if (out == NULL) break;
  }
  VP8Delete(dec);
  free(alpha);
  free(rows);
  free(path);
}

int main(int argc, char* argv[]) {
  char* command;
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "decode < commands");
  while ((command = Wp6ReadToken()) != NULL) {
    if (!strcmp(command, "decode")) {
      Decode(1);
    } else if (!strcmp(command, "decodehex")) {
      Decode(0);
    } else if (!strcmp(command, "alpha")) {
      Alpha();
    } else {
      fprintf(stderr, "wp6: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  return 0;
}
