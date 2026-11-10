// WP13 probe: WebPEncode (src/enc/webp_enc.c) through the public encoder API
// only, so that the same probe also links against the Rust library
// (probes/wp13/encode_rs, oracle.cmake) and runs the matrix through the C ABI.
// tests/wp13_encode.rs replays every record with the Rust translation;
// tests/wp13_abi.rs compares this probe with encode_rs (ADR 0002).
//
// Standard input is a sequence of binary records, all fields little-endian
// 32-bit values:
//
//   width height source layout left top view_width view_height pad
//   preset quality_bits lossless_level
//   method segments partitions sns_strength filter_strength filter_sharpness
//   filter_type autofilter pass target_size target_psnr_bits
//   emulate_jpeg_size low_memory thread_level use_sharp_yuv preprocessing
//   alpha_quality alpha_filtering alpha_compression exact partition_limit
//   qmin qmax show_compressed lossless near_lossless use_delta_palette
//   image_hint
//   abort_at fail_write_at with_stats extra_info_type writer
//   then width * height RGBA words (R in the low byte)
//
// source 0 imports RGB into YUV, 1 RGBA into YUV(A), 2 RGBA into ARGB, 3 RGB
// into ARGB (WebPPictureImport*). layout 0 encodes the imported picture, 1
// copies its planes into caller memory whose rows are `pad` samples longer
// (a picture that owns no memory), 2 encodes the view of the rectangle
// left, top, view_width, view_height (WebPPictureView). preset < 0 starts
// from WebPConfigInit, otherwise from WebPConfigPreset(preset, quality); then
// lossless_level >= 0 applies WebPConfigLosslessPreset. A configuration field
// of INT32_MIN keeps its value; *_bits fields are float bit patterns.
// abort_at < -1 installs no progress hook; otherwise the hook fails at call
// abort_at (0-based, -1 never). writer 0 is a custom writer that fails at call
// fail_write_at (>= 0), writer 1 the WebPMemoryWriter. extra_info_type > 0
// installs an extra_info map of that type.
//
// The probe prints, per record:
//
//   valid=     WebPValidateConfig
//   view=      (layout 2) the result of WebPPictureView; nothing follows if 0
//   ret=       the return value of WebPEncode
//   error=     picture.error_code
//   progress=  the percents passed to the hook, comma-separated, or -
//   writes=    (writer 0) the sizes passed to the writer, comma-separated, or -
//   mem=       (writer 1) the memory writer's size and max_size
//   out=       the bytes the writer accepted, in hex
//   stats=     (with_stats) the WebPAuxStats fields, floats as bit patterns
//   extra=     (extra_info_type > 0) the extra_info bytes, zeroed before the
//              call, and one guard byte (0xaa)
//   pic=       the encoded picture afterwards: use_argb, colorspace, width,
//              height, and the FNV-1a digest of the visible samples of the
//              y, u, v, a and argb planes (- for NULL)
//   caller=    (layout 1) the digest of the caller's buffers, padding included
//   source=    (layout 2) the source picture as pic= prints it
//
// The digest (FNV-1a, 64 bits) runs over the rows of each plane: bytes for
// Y, U, V and A, little-endian words for ARGB.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/webp/encode.h"

#define KEEP ((int32_t)0x80000000u)
#define NUM_FIELDS 28

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

static uint64_t Fnv(uint64_t hash, const uint8_t* data, size_t size) {
  size_t i;
  for (i = 0; i < size; ++i) {
    hash ^= data[i];
    hash *= 0x100000001b3ull;
  }
  return hash;
}

#define FNV_START 0xcbf29ce484222325ull

// Prints the digest of `height` rows of `width` bytes, or "-".
static void PrintPlane(const uint8_t* plane, int stride, int width,
                       int height) {
  uint64_t hash = FNV_START;
  int y;
  if (plane == NULL) {
    printf(",-");
    return;
  }
  for (y = 0; y < height; ++y) {
    hash = Fnv(hash, plane + (ptrdiff_t)y * stride, (size_t)width);
  }
  printf(",%016llx", (unsigned long long)hash);
}

static void PrintArgb(const uint32_t* plane, int stride, int width,
                      int height) {
  uint64_t hash = FNV_START;
  int x, y;
  if (plane == NULL) {
    printf(",-");
    return;
  }
  for (y = 0; y < height; ++y) {
    for (x = 0; x < width; ++x) {
      const uint32_t v = plane[(ptrdiff_t)y * stride + x];
      const uint8_t b[4] = {(uint8_t)v, (uint8_t)(v >> 8), (uint8_t)(v >> 16),
                            (uint8_t)(v >> 24)};
      hash = Fnv(hash, b, 4);
    }
  }
  printf(",%016llx", (unsigned long long)hash);
}

static void PrintPicture(const char* key, const WebPPicture* pic) {
  const int uv_w = (pic->width + 1) >> 1, uv_h = (pic->height + 1) >> 1;
  printf("%s=%d,%d,%d,%d", key, pic->use_argb, (int)pic->colorspace,
         pic->width, pic->height);
  PrintPlane(pic->y, pic->y_stride, pic->width, pic->height);
  PrintPlane(pic->u, pic->uv_stride, uv_w, uv_h);
  PrintPlane(pic->v, pic->uv_stride, uv_w, uv_h);
  PrintPlane(pic->a, pic->a_stride, pic->width, pic->height);
  PrintArgb(pic->argb, pic->argb_stride, pic->width, pic->height);
  printf("\n");
}

// Caller memory: the picture's planes copied into rows `pad` samples longer.
typedef struct {
  uint8_t* planes[4];
  size_t sizes[4];
  uint32_t* argb;
  size_t argb_size;
} Caller;

static uint8_t* CopyPlane(const uint8_t* src, int src_stride, int width,
                          int height, int stride, size_t* size) {
  uint8_t* dst;
  int y;
  *size = (size_t)stride * (size_t)height;
  dst = (uint8_t*)malloc(*size + 1);
  if (dst == NULL) exit(2);
  memset(dst, 0x5a, *size + 1);
  for (y = 0; y < height; ++y) {
    memcpy(dst + (size_t)y * stride, src + (ptrdiff_t)y * src_stride,
           (size_t)width);
  }
  return dst;
}

// Moves `pic` (library memory) into caller memory: `out` gets the same
// samples in the caller's buffers and owns no memory.
static void ToCaller(WebPPicture* pic, int pad, Caller* caller,
                     WebPPicture* out) {
  memset(caller, 0, sizeof(*caller));
  if (!WebPPictureInit(out)) exit(2);
  out->use_argb = pic->use_argb;
  out->colorspace = pic->colorspace;
  out->width = pic->width;
  out->height = pic->height;
  if (pic->use_argb) {
    int y;
    const int stride = pic->width + pad;
    caller->argb_size = (size_t)stride * (size_t)pic->height;
    caller->argb = (uint32_t*)malloc(caller->argb_size * 4 + 4);
    if (caller->argb == NULL) exit(2);
    memset(caller->argb, 0x5a, caller->argb_size * 4 + 4);
    for (y = 0; y < pic->height; ++y) {
      memcpy(caller->argb + (size_t)y * stride,
             pic->argb + (size_t)y * pic->argb_stride, (size_t)pic->width * 4);
    }
    out->argb = caller->argb;
    out->argb_stride = stride;
  } else {
    const int uv_w = (pic->width + 1) >> 1, uv_h = (pic->height + 1) >> 1;
    caller->planes[0] = CopyPlane(pic->y, pic->y_stride, pic->width,
                                  pic->height, pic->width + pad,
                                  &caller->sizes[0]);
    caller->planes[1] = CopyPlane(pic->u, pic->uv_stride, uv_w, uv_h,
                                  uv_w + pad, &caller->sizes[1]);
    caller->planes[2] = CopyPlane(pic->v, pic->uv_stride, uv_w, uv_h,
                                  uv_w + pad, &caller->sizes[2]);
    out->y = caller->planes[0];
    out->u = caller->planes[1];
    out->v = caller->planes[2];
    out->y_stride = pic->width + pad;
    out->uv_stride = uv_w + pad;
    if (pic->a != NULL) {
      caller->planes[3] = CopyPlane(pic->a, pic->a_stride, pic->width,
                                    pic->height, pic->width + pad,
                                    &caller->sizes[3]);
      out->a = caller->planes[3];
      out->a_stride = pic->width + pad;
    }
  }
}

static void PrintCaller(const Caller* caller) {
  int i;
  printf("caller=");
  for (i = 0; i < 4; ++i) {
    if (caller->planes[i] == NULL) {
      printf("-,");
    } else {
      printf("%016llx,", (unsigned long long)Fnv(FNV_START, caller->planes[i],
                                                caller->sizes[i] + 1));
    }
  }
  if (caller->argb == NULL) {
    printf("-\n");
  } else {
    size_t i2;
    uint64_t hash = FNV_START;
    for (i2 = 0; i2 < caller->argb_size + 1; ++i2) {
      const uint32_t v = caller->argb[i2];
      const uint8_t b[4] = {(uint8_t)v, (uint8_t)(v >> 8), (uint8_t)(v >> 16),
                            (uint8_t)(v >> 24)};
      hash = Fnv(hash, b, 4);
    }
    printf("%016llx\n", (unsigned long long)hash);
  }
}

static void FreeCaller(Caller* caller) {
  int i;
  for (i = 0; i < 4; ++i) free(caller->planes[i]);
  free(caller->argb);
}

int main(int argc, char* argv[]) {
  int32_t width;
  ProbeCheckArgs(argc, 0, 0, "encode < records.bin");
  (void)argv;
  while (ReadI32(&width)) {
    const int32_t height = MustReadI32();
    const int32_t source = MustReadI32();
    const int32_t layout = MustReadI32();
    const int32_t left = MustReadI32();
    const int32_t top = MustReadI32();
    const int32_t view_width = MustReadI32();
    const int32_t view_height = MustReadI32();
    const int32_t pad = MustReadI32();
    const int32_t preset = MustReadI32();
    const float quality = Float(MustReadI32());
    const int32_t lossless_level = MustReadI32();
    int32_t fields[NUM_FIELDS];
    int32_t abort_at, fail_write_at, with_stats, extra_info_type, writer;
    const size_t count = (size_t)width * (size_t)height;
    uint8_t* const rgba = (uint8_t*)malloc(count * 4 + 4);
    uint8_t* const rgb = (uint8_t*)malloc(count * 3 + 4);
    WebPConfig config;
    WebPPicture source_pic, view, *picture;
    WebPAuxStats stats;
    WebPMemoryWriter memory;
    Caller caller;
    Output out;
    uint8_t* extra = NULL;
    size_t extra_size = 0;
    size_t i;
    int k, ret;
    if (rgba == NULL || rgb == NULL) return 2;
    for (k = 0; k < NUM_FIELDS; ++k) fields[k] = MustReadI32();
    abort_at = MustReadI32();
    fail_write_at = MustReadI32();
    with_stats = MustReadI32();
    extra_info_type = MustReadI32();
    writer = MustReadI32();
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
      if (!WebPConfigPreset(&config, (WebPPreset)preset, quality)) {
        // An invalid quality fails the preset's validation; keep the fields.
      }
    }
    config.quality = quality;
    if (lossless_level >= 0) {
      if (!WebPConfigLosslessPreset(&config, lossless_level)) return 2;
    }
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
    Set(&config.lossless, fields[24]);
    Set(&config.near_lossless, fields[25]);
    Set(&config.use_delta_palette, fields[26]);
    if (fields[27] != KEEP) config.image_hint = (WebPImageHint)fields[27];
    printf("valid=%d\n", WebPValidateConfig(&config));

    if (!WebPPictureInit(&source_pic)) return 2;
    source_pic.width = width;
    source_pic.height = height;
    source_pic.use_argb = (source >= 2);
    if (source == 0 || source == 3) {
      if (!WebPPictureImportRGB(&source_pic, rgb, 3 * width)) return 2;
    } else {
      if (!WebPPictureImportRGBA(&source_pic, rgba, 4 * width)) return 2;
    }
    memset(&caller, 0, sizeof(caller));
    picture = &source_pic;
    if (layout == 1) {
      ToCaller(&source_pic, pad, &caller, &view);
      WebPPictureFree(&source_pic);
      picture = &view;
    } else if (layout == 2) {
      const int ok =
          WebPPictureView(&source_pic, left, top, view_width, view_height, &view);
      printf("view=%d\n", ok);
      if (!ok) {
        fflush(stdout);
        WebPPictureFree(&source_pic);
        free(rgba);
        free(rgb);
        continue;
      }
      picture = &view;
    }

    memset(&out, 0, sizeof(out));
    out.fail_at = fail_write_at;
    if (writer == 1) {
      WebPMemoryWriterInit(&memory);
      picture->writer = WebPMemoryWrite;
      picture->custom_ptr = &memory;
    } else {
      picture->writer = Writer;
      picture->custom_ptr = &out;
    }
    g_abort_at = abort_at;
    g_progress_calls = 0;
    g_progress_len = 0;
    g_progress[0] = '\0';
    picture->progress_hook = (abort_at >= -1) ? Progress : NULL;
    memset(&stats, 0xa5, sizeof(stats));
    picture->stats = with_stats ? &stats : NULL;
    picture->extra_info = NULL;
    if (extra_info_type > 0) {
      extra_size = (size_t)(((picture->width + 15) >> 4) *
                            ((picture->height + 15) >> 4));
      extra = (uint8_t*)malloc(extra_size + 1);
      if (extra == NULL) return 2;
      // Zeros, so that the macroblocks a failed encoding did not reach read
      // the same through the C ABI, which writes zeros there; one guard byte.
      memset(extra, 0, extra_size);
      extra[extra_size] = 0xaa;
      picture->extra_info = extra;
      picture->extra_info_type = extra_info_type;
    }

    ret = WebPEncode(&config, picture);

    printf("ret=%d\nerror=%d\n", ret, (int)picture->error_code);
    printf("progress=%s\n", (g_progress_len == 0) ? "-" : g_progress);
    if (writer == 1) {
      printf("mem=%zu,%zu\n", memory.size, memory.max_size);
      ProbePrintHex("out", memory.mem, memory.size);
      WebPMemoryWriterClear(&memory);
    } else {
      printf("writes=%s\n", (out.sizes_len == 0) ? "-" : out.sizes);
      ProbePrintHex("out", out.data, out.size);
    }
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
    PrintPicture("pic", picture);
    if (layout == 1) PrintCaller(&caller);
    if (layout == 2) PrintPicture("source", &source_pic);
    fflush(stdout);
    if (picture != &source_pic) WebPPictureFree(picture);
    WebPPictureFree(&source_pic);
    FreeCaller(&caller);
    free(out.data);
    free(extra);
    extra = NULL;
    free(rgba);
    free(rgb);
  }
  return 0;
}
