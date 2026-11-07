// WP11 probe: the VP8L encoder, VP8LEncodeImage (src/enc/vp8l_enc.c), which
// writes the complete RIFF file through the picture's writer (ADR 0002).
//
// Standard input is a sequence of binary records, all fields little-endian
// 32-bit values:
//
//   width height stride quality_bits method exact near_lossless
//   use_delta_palette thread_level image_hint abort_at fail_write_at
//   with_stats with_extra_info null_argb
//   then stride * height ARGB words
//
// quality_bits is the IEEE 754 pattern of the float quality. abort_at < 0
// installs no progress hook; otherwise the hook answers percent < abort_at.
// fail_write_at >= 0 makes the writer fail at that call (0-based). The probe
// prints, per record:
//
//   ret=        the return value of VP8LEncodeImage
//   error=      picture.error_code
//   progress=   the percents passed to the hook, comma-separated, or -
//   writes=     the sizes passed to the writer, comma-separated, or -
//   out=        the bytes the writer accepted, in hex
//   stats=      (with_stats) the WebPAuxStats fields, floats as bit patterns
//   extra=      (with_extra_info) the extra_info bytes after the call
//
// tests/wp11_encode.rs generates the records and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/enc/vp8li_enc.h"
#include "src/webp/encode.h"

typedef struct {
  uint8_t* data;
  size_t size;
  size_t capacity;
  int calls;
  int fail_at;
  char sizes[1 << 16];
  size_t sizes_len;
} Output;

static int g_abort_at = -1;
static char g_progress[1 << 16];
static size_t g_progress_len = 0;

static void Append(char* buffer, size_t* len, size_t capacity, long value) {
  const int n = snprintf(buffer + *len, capacity - *len, "%s%ld",
                         (*len == 0) ? "" : ",", value);
  if (n > 0 && *len + (size_t)n < capacity) *len += (size_t)n;
}

static int Writer(const uint8_t* data, size_t data_size,
                  const WebPPicture* picture) {
  Output* const out = (Output*)picture->custom_ptr;
  const int call = out->calls++;
  Append(out->sizes, &out->sizes_len, sizeof(out->sizes), (long)data_size);
  if (call == out->fail_at) return 0;
  if (out->size + data_size > out->capacity) {
    size_t capacity = out->capacity * 2 + data_size + 64;
    uint8_t* const grown = (uint8_t*)realloc(out->data, capacity);
    if (grown == NULL) exit(2);
    out->data = grown;
    out->capacity = capacity;
  }
  if (data_size > 0) memcpy(out->data + out->size, data, data_size);
  out->size += data_size;
  return 1;
}

static int Progress(int percent, const WebPPicture* picture) {
  (void)picture;
  Append(g_progress, &g_progress_len, sizeof(g_progress), percent);
  return percent < g_abort_at;
}

static int ReadI32(int32_t* value) {
  uint8_t b[4];
  if (fread(b, 1, 4, stdin) != 4) return 0;
  *value = (int32_t)((uint32_t)b[0] | ((uint32_t)b[1] << 8) |
                     ((uint32_t)b[2] << 16) | ((uint32_t)b[3] << 24));
  return 1;
}

static int32_t MustReadI32(void) {
  int32_t value;
  if (!ReadI32(&value)) exit(2);
  return value;
}

static void PrintFloat(float f) {
  uint32_t bits;
  memcpy(&bits, &f, 4);
  printf(",%08x", bits);
}

int main(int argc, char* argv[]) {
  int32_t width;
  ProbeCheckArgs(argc, 0, 0, "encode < records.bin");
  (void)argv;
  while (ReadI32(&width)) {
    const int32_t height = MustReadI32();
    const int32_t stride = MustReadI32();
    const int32_t quality_bits = MustReadI32();
    const int32_t method = MustReadI32();
    const int32_t exact = MustReadI32();
    const int32_t near_lossless = MustReadI32();
    const int32_t use_delta_palette = MustReadI32();
    const int32_t thread_level = MustReadI32();
    const int32_t image_hint = MustReadI32();
    const int32_t abort_at = MustReadI32();
    const int32_t fail_write_at = MustReadI32();
    const int32_t with_stats = MustReadI32();
    const int32_t with_extra_info = MustReadI32();
    const int32_t null_argb = MustReadI32();
    const size_t count = (size_t)stride * (size_t)height;
    uint32_t* const argb = (uint32_t*)malloc(count * 4 + 4);
    WebPConfig config;
    WebPPicture picture;
    WebPAuxStats stats;
    Output out;
    uint8_t* extra = NULL;
    size_t extra_size = 0;
    float quality;
    size_t i;
    int ret;
    if (argb == NULL) return 2;
    for (i = 0; i < count; ++i) argb[i] = (uint32_t)MustReadI32();
    memcpy(&quality, &quality_bits, 4);

    if (!WebPConfigInit(&config)) return 2;
    config.lossless = 1;
    config.quality = quality;
    config.method = method;
    config.exact = exact;
    config.near_lossless = near_lossless;
    config.use_delta_palette = use_delta_palette;
    config.thread_level = thread_level;
    config.image_hint = (WebPImageHint)image_hint;

    if (!WebPPictureInit(&picture)) return 2;
    picture.use_argb = 1;
    picture.width = width;
    picture.height = height;
    picture.argb = null_argb ? NULL : argb;
    picture.argb_stride = stride;
    memset(&out, 0, sizeof(out));
    out.fail_at = fail_write_at;
    picture.writer = Writer;
    picture.custom_ptr = &out;
    g_abort_at = abort_at;
    g_progress_len = 0;
    g_progress[0] = '\0';
    if (abort_at >= 0) picture.progress_hook = Progress;
    memset(&stats, 0xa5, sizeof(stats));
    if (with_stats) picture.stats = &stats;
    if (with_extra_info) {
      extra_size = (size_t)(((width + 15) >> 4) * ((height + 15) >> 4));
      extra = (uint8_t*)malloc(extra_size + 1);
      if (extra == NULL) return 2;
      memset(extra, 0xaa, extra_size + 1);
      picture.extra_info = extra;
    }

    ret = VP8LEncodeImage(&config, &picture);

    printf("ret=%d\nerror=%d\n", ret, (int)picture.error_code);
    printf("progress=%s\n", (g_progress_len == 0) ? "-" : g_progress);
    printf("writes=%s\n", (out.sizes_len == 0) ? "-" : out.sizes);
    ProbePrintHex("out", out.data, out.size);
    if (with_stats) {
      int k;
      printf("stats=%d", stats.coded_size);
      for (k = 0; k < 5; ++k) PrintFloat(stats.PSNR[k]);
      for (k = 0; k < 3; ++k) printf(",%d", stats.block_count[k]);
      for (k = 0; k < 2; ++k) printf(",%d", stats.header_bytes[k]);
      for (k = 0; k < 12; ++k) printf(",%d", stats.residual_bytes[k / 4][k % 4]);
      for (k = 0; k < 4; ++k) printf(",%d", stats.segment_size[k]);
      for (k = 0; k < 4; ++k) printf(",%d", stats.segment_quant[k]);
      for (k = 0; k < 4; ++k) printf(",%d", stats.segment_level[k]);
      printf(",%d,%d,%u,%d,%d,%d,%d,%d,%d,%d,%d,%u\n", stats.alpha_data_size,
             stats.layer_data_size, stats.lossless_features,
             stats.histogram_bits, stats.transform_bits, stats.cache_bits,
             stats.palette_size, stats.lossless_size, stats.lossless_hdr_size,
             stats.lossless_data_size, stats.cross_color_transform_bits,
             stats.pad[0]);
    }
    if (with_extra_info) ProbePrintHex("extra", extra, extra_size + 1);
    fflush(stdout);
    free(out.data);
    free(extra);
    free(argb);
  }
  return 0;
}
