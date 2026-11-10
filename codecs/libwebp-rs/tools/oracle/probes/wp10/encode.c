// WP10 probe: the VP8 encoder core through WebPEncode (src/enc/webp_enc.c),
// which the Rust test replays step by step (InitVP8Encoder, VP8EncAnalyze,
// VP8EncStartAlpha, VP8EncLoop or VP8EncTokenLoop, VP8EncFinishAlpha,
// VP8EncWrite, DeleteVP8Encoder) on the YUV(A) planes this probe prints
// (ADR 0002).
//
// Standard input is a sequence of binary records, all fields little-endian
// 32-bit values:
//
//   width height import preset quality_bits
//   method segments partitions sns_strength filter_strength filter_sharpness
//   filter_type autofilter pass target_size target_psnr_bits emulate_jpeg_size
//   low_memory thread_level use_sharp_yuv preprocessing alpha_quality
//   alpha_filtering alpha_compression exact partition_limit qmin qmax
//   show_compressed
//   abort_at fail_write_at with_stats extra_info_type
//   then width * height RGBA words (R in the low byte)
//
// import 0 imports RGB into YUV, 1 imports RGBA into YUV(A), 2 imports RGBA
// into ARGB, which WebPEncode converts. preset < 0 starts from WebPConfigInit,
// otherwise from WebPConfigPreset(preset, quality). A configuration field of
// INT32_MIN keeps the preset's value; *_bits fields are float bit patterns.
// abort_at < -1 installs no progress hook; otherwise the hook fails at call
// abort_at (0-based, -1 never). fail_write_at >= 0 makes the writer fail at
// that call. extra_info_type > 0 installs an extra_info map of that type.
//
// The probe prints, per record:
//
//   valid=     WebPValidateConfig; the other keys follow only when it is 1
//   y= u= v=   the planes WebPEncode encodes: after its colorspace conversion
//   a=         and WebPCleanupTransparentArea, rows packed; a=- without alpha
//   ret=       the return value of WebPEncode
//   error=     picture.error_code
//   progress=  the percents passed to the hook, comma-separated, or -
//   writes=    the sizes passed to the writer, comma-separated, or -
//   out=       the bytes the writer accepted, in hex
//   stats=     (with_stats) the WebPAuxStats fields, floats as bit patterns
//   extra=     (extra_info_type > 0) the extra_info bytes and one guard byte
//   shown_y= shown_u= shown_v=  (show_compressed) the planes after the call
//
// tests/wp10_encode.rs generates the records and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/webp/encode.h"

#define KEEP ((int32_t)0x80000000u)

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
static int g_progress_calls = 0;
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
  const int call = g_progress_calls++;
  (void)picture;
  Append(g_progress, &g_progress_len, sizeof(g_progress), percent);
  return call != g_abort_at;
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

static float Float(int32_t bits) {
  float f;
  memcpy(&f, &bits, 4);
  return f;
}

static void PrintFloat(float f) {
  uint32_t bits;
  memcpy(&bits, &f, 4);
  printf(",%08x", bits);
}

static void Set(int* field, int32_t value) {
  if (value != KEEP) *field = value;
}

// Prints `key=` and the plane's rows of `width` samples, or "-".
static void PrintPlane(const char* key, const uint8_t* plane, int stride,
                       int width, int height) {
  int y;
  if (plane == NULL) {
    printf("%s=-\n", key);
    return;
  }
  printf("%s=", key);
  for (y = 0; y < height; ++y) {
    int x;
    for (x = 0; x < width; ++x) printf("%02x", plane[y * stride + x]);
  }
  printf("\n");
}

int main(int argc, char* argv[]) {
  int32_t width;
  ProbeCheckArgs(argc, 0, 0, "encode < records.bin");
  (void)argv;
  while (ReadI32(&width)) {
    const int32_t height = MustReadI32();
    const int32_t import = MustReadI32();
    const int32_t preset = MustReadI32();
    const float quality = Float(MustReadI32());
    int32_t fields[24];
    int32_t abort_at, fail_write_at, with_stats, extra_info_type;
    const size_t count = (size_t)width * (size_t)height;
    uint8_t* const rgba = (uint8_t*)malloc(count * 4 + 4);
    uint8_t* const rgb = (uint8_t*)malloc(count * 3 + 4);
    WebPConfig config;
    WebPPicture picture, converted;
    WebPAuxStats stats;
    Output out;
    uint8_t* extra = NULL;
    size_t extra_size = 0;
    size_t i;
    int k, ret;
    if (rgba == NULL || rgb == NULL) return 2;
    for (k = 0; k < 24; ++k) fields[k] = MustReadI32();
    abort_at = MustReadI32();
    fail_write_at = MustReadI32();
    with_stats = MustReadI32();
    extra_info_type = MustReadI32();
    for (i = 0; i < count; ++i) {
      const uint32_t word = (uint32_t)MustReadI32();
      rgba[4 * i + 0] = (uint8_t)(word >> 0);
      rgba[4 * i + 1] = (uint8_t)(word >> 8);
      rgba[4 * i + 2] = (uint8_t)(word >> 16);
      rgba[4 * i + 3] = (uint8_t)(word >> 24);
      rgb[3 * i + 0] = rgba[4 * i + 0];
      rgb[3 * i + 1] = rgba[4 * i + 1];
      rgb[3 * i + 2] = rgba[4 * i + 2];
    }

    if (preset < 0) {
      if (!WebPConfigInit(&config)) return 2;
    } else {
      if (!WebPConfigPreset(&config, (WebPPreset)preset, quality)) return 2;
    }
    config.quality = quality;
    Set(&config.method, fields[0]);
    Set(&config.segments, fields[1]);
    Set(&config.partitions, fields[2]);
    Set(&config.sns_strength, fields[3]);
    Set(&config.filter_strength, fields[4]);
    Set(&config.filter_sharpness, fields[5]);
    Set(&config.filter_type, fields[6]);
    Set(&config.autofilter, fields[7]);
    Set(&config.pass, fields[8]);
    Set(&config.target_size, fields[9]);
    if (fields[10] != KEEP) config.target_PSNR = Float(fields[10]);
    Set(&config.emulate_jpeg_size, fields[11]);
    Set(&config.low_memory, fields[12]);
    Set(&config.thread_level, fields[13]);
    Set(&config.use_sharp_yuv, fields[14]);
    Set(&config.preprocessing, fields[15]);
    Set(&config.alpha_quality, fields[16]);
    Set(&config.alpha_filtering, fields[17]);
    Set(&config.alpha_compression, fields[18]);
    Set(&config.exact, fields[19]);
    Set(&config.partition_limit, fields[20]);
    Set(&config.qmin, fields[21]);
    Set(&config.qmax, fields[22]);
    Set(&config.show_compressed, fields[23]);
    printf("valid=%d\n", WebPValidateConfig(&config));
    if (!WebPValidateConfig(&config)) {
      fflush(stdout);
      free(rgba);
      free(rgb);
      continue;
    }

    if (!WebPPictureInit(&picture)) return 2;
    picture.width = width;
    picture.height = height;
    picture.use_argb = (import == 2);
    if (import == 0) {
      if (!WebPPictureImportRGB(&picture, rgb, 3 * width)) return 2;
    } else {
      if (!WebPPictureImportRGBA(&picture, rgba, 4 * width)) return 2;
    }

    // The planes that WebPEncode hands to InitVP8Encoder.
    if (!WebPPictureInit(&converted)) return 2;
    if (!WebPPictureCopy(&picture, &converted)) return 2;
    if (converted.use_argb || converted.y == NULL || converted.u == NULL ||
        converted.v == NULL) {
      if (config.use_sharp_yuv || (config.preprocessing & 4)) {
        if (!WebPPictureSharpARGBToYUVA(&converted)) return 2;
      } else {
        float dithering = 0.f;
        if (config.preprocessing & 2) {
          const float x = config.quality / 100.f;
          const float x2 = x * x;
          dithering = 1.0f + (0.5f - 1.0f) * x2 * x2;
        }
        if (!WebPPictureARGBToYUVADithered(&converted, WEBP_YUV420,
                                           dithering)) {
          return 2;
        }
      }
    }
    if (!config.exact) WebPCleanupTransparentArea(&converted);
    PrintPlane("y", converted.y, converted.y_stride, width, height);
    PrintPlane("u", converted.u, converted.uv_stride, (width + 1) / 2,
               (height + 1) / 2);
    PrintPlane("v", converted.v, converted.uv_stride, (width + 1) / 2,
               (height + 1) / 2);
    PrintPlane("a", converted.a, converted.a_stride, width, height);
    WebPPictureFree(&converted);

    memset(&out, 0, sizeof(out));
    out.fail_at = fail_write_at;
    picture.writer = Writer;
    picture.custom_ptr = &out;
    g_abort_at = abort_at;
    g_progress_calls = 0;
    g_progress_len = 0;
    g_progress[0] = '\0';
    if (abort_at >= -1) picture.progress_hook = Progress;
    memset(&stats, 0xa5, sizeof(stats));
    if (with_stats) picture.stats = &stats;
    if (extra_info_type > 0) {
      extra_size = (size_t)(((width + 15) >> 4) * ((height + 15) >> 4));
      extra = (uint8_t*)malloc(extra_size + 1);
      if (extra == NULL) return 2;
      memset(extra, 0xaa, extra_size + 1);
      picture.extra_info = extra;
      picture.extra_info_type = extra_info_type;
    }

    ret = WebPEncode(&config, &picture);

    printf("ret=%d\nerror=%d\n", ret, (int)picture.error_code);
    printf("progress=%s\n", (g_progress_len == 0) ? "-" : g_progress);
    printf("writes=%s\n", (out.sizes_len == 0) ? "-" : out.sizes);
    ProbePrintHex("out", out.data, out.size);
    if (with_stats) {
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
    if (extra_info_type > 0) ProbePrintHex("extra", extra, extra_size + 1);
    if (config.show_compressed) {
      PrintPlane("shown_y", picture.y, picture.y_stride, width, height);
      PrintPlane("shown_u", picture.u, picture.uv_stride, (width + 1) / 2,
                 (height + 1) / 2);
      PrintPlane("shown_v", picture.v, picture.uv_stride, (width + 1) / 2,
                 (height + 1) / 2);
    }
    fflush(stdout);
    WebPPictureFree(&picture);
    free(out.data);
    free(extra);
    free(rgba);
    free(rgb);
  }
  return 0;
}
