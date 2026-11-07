// WP12 probe: the decoding API of src/dec/webp_dec.c, buffer_dec.c and
// idec_dec.c through the public functions of src/webp/decode.h (ADR 0002).
// Commands on standard input, one per line:
//
//   features PATH LEN
//   featureshex HEX
//
//     WebPGetFeatures and WebPGetInfo on the first LEN bytes of the file PATH
//     (-1: all of it), or on the bytes in HEX ("-" for none).
//
//   decode PATH LEN MODE OPTIONS GEOMETRY
//
//     WebPDecode with config.output.colorspace = MODE and the options; the
//     output memory is allocated by the library when GEOMETRY is "-", and is a
//     caller buffer with guard bytes otherwise (see ParseGeometry). Prints the
//     status, the features, the output buffer and the digests of its memory.
//
//   validate MODE OPTIONS WIDTH HEIGHT
//
//     WebPValidateDecoderConfig with the features' width and height set.
//
//   simple PATH LEN FUNC
//
//     WebPDecodeRGB, RGBA, ARGB, BGR, BGRA or YUV (FUNC).
//
//   into PATH LEN FUNC GEOMETRY
//
//     WebPDecode{RGB,RGBA,ARGB,BGR,BGRA}Into or WebPDecodeYUVInto (FUNC) into
//     caller buffers with guard bytes.
//
//   incr PATH LEN CHUNK APPEND KIND MODE OPTIONS GEOMETRY
//
//     The incremental decoder: KIND "config" (WebPIDecode with a configuration
//     of MODE and OPTIONS, the output allocated by the library or GEOMETRY),
//     "new" (WebPINewDecoder(NULL)), "rgb" (WebPINewRGB(MODE, ...)), "yuva"
//     (WebPINewYUVA) or "yuv" (WebPINewYUV), with GEOMETRY "-" for library
//     memory. The input arrives in CHUNK-byte steps through WebPIAppend
//     (APPEND 1) or WebPIUpdate (APPEND 0, each time in a fresh copy of the
//     data so far); APPEND 2 appends the first step and updates the next. After each step it prints the status, WebPIDecodedArea and
//     WebPIDecGetRGB or WebPIDecGetYUVA with a digest of the decoded rows.
//
// OPTIONS is 14 comma-separated integers: bypass_filtering,
// no_fancy_upsampling, use_cropping, crop_left, crop_top, crop_width,
// crop_height, use_scaling, scaled_width, scaled_height, use_threads,
// dithering_strength, flip, alpha_dithering_strength.
//
// tests/wp12_decode.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/webp/decode.h"
#include "probe_support.h"

#define GUARD 64

static uint64_t Digest(const uint8_t* data, size_t size) {
  uint64_t hash = 0xcbf29ce484222325ull;
  size_t i;
  for (i = 0; i < size; ++i) {
    hash ^= data[i];
    hash *= 0x100000001b3ull;
  }
  return hash;
}

static void PrintDigest(const char* key, const uint8_t* data, size_t size) {
  printf("%s=%016llx\n", key, (unsigned long long)Digest(data, size));
}

// The digest of `height` rows of `width` bytes, `stride` apart from `rows`.
static void PrintRowsDigest(const char* key, const uint8_t* rows, int stride,
                            int width, int height) {
  uint64_t hash = 0xcbf29ce484222325ull;
  int y, x;
  for (y = 0; y < height; ++y) {
    const uint8_t* const row = rows + (ptrdiff_t)y * stride;
    for (x = 0; x < width; ++x) {
      hash ^= row[x];
      hash *= 0x100000001b3ull;
    }
  }
  printf("%s=%016llx\n", key, (unsigned long long)hash);
}

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  exit(2);
}

static uint8_t* ParseHex(const char* hex, size_t* size) {
  const size_t len = (strcmp(hex, "-") == 0) ? 0 : strlen(hex);
  uint8_t* const data = (uint8_t*)malloc(len / 2 + 1);
  size_t i;
  if (data == NULL || (len & 1)) exit(2);
  for (i = 0; i < len / 2; ++i) {
    data[i] = (uint8_t)(HexValue(hex[2 * i]) * 16 + HexValue(hex[2 * i + 1]));
  }
  *size = len / 2;
  return data;
}

// Reads the first LEN bytes of PATH (-1: the whole file) into a buffer of
// exactly that size (at least one byte allocated).
static uint8_t* ReadPrefix(const char* path, long len, size_t* size) {
  size_t file_size;
  uint8_t* const file = ProbeReadInput(path, &file_size);
  if (len >= 0 && (size_t)len < file_size) file_size = (size_t)len;
  *size = file_size;
  return file;
}

static void ParseOptions(const char* text, WebPDecoderOptions* const options) {
  int v[14];
  if (sscanf(text, "%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d", &v[0], &v[1],
             &v[2], &v[3], &v[4], &v[5], &v[6], &v[7], &v[8], &v[9], &v[10],
             &v[11], &v[12], &v[13]) != 14) {
    exit(2);
  }
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

// Caller memory: up to four planes, each with GUARD bytes on both sides
// filled with a pattern.
typedef struct {
  int count;
  int stride[4];
  size_t size[4];
  uint8_t* block[4];
} Geometry;

// GEOMETRY is "-" (no caller memory), "STRIDE,SIZE" (one plane) or
// "YSTRIDE,YSIZE,USTRIDE,USIZE,VSTRIDE,VSIZE[,ASTRIDE,ASIZE]".
static void ParseGeometry(const char* text, Geometry* const geometry) {
  long long v[8];
  int i;
  memset(geometry, 0, sizeof(*geometry));
  if (strcmp(text, "-") == 0) return;
  geometry->count = sscanf(text, "%lld,%lld,%lld,%lld,%lld,%lld,%lld,%lld",
                           &v[0], &v[1], &v[2], &v[3], &v[4], &v[5], &v[6],
                           &v[7]) / 2;
  if (geometry->count < 1) exit(2);
  for (i = 0; i < geometry->count; ++i) {
    size_t j;
    geometry->stride[i] = (int)v[2 * i];
    geometry->size[i] = (size_t)v[2 * i + 1];
    geometry->block[i] = (uint8_t*)malloc(geometry->size[i] + 2 * GUARD);
    if (geometry->block[i] == NULL) exit(2);
    for (j = 0; j < geometry->size[i] + 2 * GUARD; ++j) {
      geometry->block[i][j] = (uint8_t)(j * 31 + 7);
    }
  }
}

static uint8_t* Plane(const Geometry* const geometry, int i) {
  return (i < geometry->count) ? geometry->block[i] + GUARD : NULL;
}

static void PrintGeometry(const Geometry* const geometry) {
  int i;
  for (i = 0; i < geometry->count; ++i) {
    char key[32];
    snprintf(key, sizeof(key), "ext%d", i);
    PrintDigest(key, geometry->block[i], geometry->size[i] + 2 * GUARD);
  }
}

static void FreeGeometry(Geometry* const geometry) {
  int i;
  for (i = 0; i < geometry->count; ++i) free(geometry->block[i]);
}

static const int kModeBpp[MODE_LAST] = {3, 4, 3, 4, 4, 2, 2, 4, 4, 4, 2, 1, 1};

static void PrintFeatures(const char* prefix,
                          const WebPBitstreamFeatures* const f) {
  printf("%s=%d,%d,%d,%d,%d\n", prefix, f->width, f->height, f->has_alpha,
         f->has_animation, f->format);
}

// The position of a plane in the memory that holds it: the caller's plane or
// the library's private memory.
static long long Offset(const uint8_t* plane, const uint8_t* base) {
  return (plane == NULL || base == NULL) ? -1 : (long long)(plane - base);
}

// Set while an incremental decoding that did not finish is printed: rows that
// it did not decode are uninitialized library memory, which is not compared.
static int g_partial = 0;

static void PrintBuffer(const WebPDecBuffer* const buf,
                        const Geometry* const geometry, size_t private_size) {
  printf("out=%d,%d,%d,%d\n", (int)buf->colorspace, buf->width, buf->height,
         buf->is_external_memory);
  if (WebPIsRGBMode(buf->colorspace)) {
    const WebPRGBABuffer* const rgba = &buf->u.RGBA;
    const uint8_t* const base =
        geometry->count ? Plane(geometry, 0) : buf->private_memory;
    printf("rgba=%lld,%d,%llu\n", Offset(rgba->rgba, base), rgba->stride,
           (unsigned long long)rgba->size);
  } else {
    const WebPYUVABuffer* const yuva = &buf->u.YUVA;
    const uint8_t* const base = buf->private_memory;
    const int ext = geometry->count;
    printf("yuva=%lld,%lld,%lld,%lld,%d,%d,%d,%d,%llu,%llu,%llu,%llu\n",
           Offset(yuva->y, ext ? Plane(geometry, 0) : base),
           Offset(yuva->u, ext ? Plane(geometry, 1) : base),
           Offset(yuva->v, ext ? Plane(geometry, 2) : base),
           Offset(yuva->a, ext ? Plane(geometry, 3) : base), yuva->y_stride,
           yuva->u_stride, yuva->v_stride, yuva->a_stride,
           (unsigned long long)yuva->y_size, (unsigned long long)yuva->u_size,
           (unsigned long long)yuva->v_size, (unsigned long long)yuva->a_size);
  }
  if (geometry->count) {
    PrintGeometry(geometry);
  } else if (buf->private_memory != NULL && g_partial) {
    printf("mem=partial\n");
  } else if (buf->private_memory != NULL) {
    PrintDigest("mem", buf->private_memory, private_size);
  } else {
    printf("mem=null\n");
  }
}

// The size of the memory that AllocateBuffer allocates for `buf`.
static size_t PrivateSize(const WebPDecBuffer* const buf) {
  const int w = buf->width, h = buf->height;
  const WEBP_CSP_MODE mode = buf->colorspace;
  size_t total = (size_t)w * kModeBpp[mode] * h;
  if (!WebPIsRGBMode(mode)) {
    total += 2 * (size_t)((w + 1) / 2) * ((h + 1) / 2);
    if (mode == MODE_YUVA) total += (size_t)w * h;
  }
  return total;
}

static void SetGeometry(WebPDecBuffer* const buf,
                        const Geometry* const geometry) {
  if (geometry->count == 0) return;
  buf->is_external_memory = 1;
  if (WebPIsRGBMode(buf->colorspace)) {
    buf->u.RGBA.rgba = Plane(geometry, 0);
    buf->u.RGBA.stride = geometry->stride[0];
    buf->u.RGBA.size = geometry->size[0];
  } else {
    buf->u.YUVA.y = Plane(geometry, 0);
    buf->u.YUVA.y_stride = geometry->stride[0];
    buf->u.YUVA.y_size = geometry->size[0];
    buf->u.YUVA.u = Plane(geometry, 1);
    buf->u.YUVA.u_stride = geometry->stride[1];
    buf->u.YUVA.u_size = geometry->size[1];
    buf->u.YUVA.v = Plane(geometry, 2);
    buf->u.YUVA.v_stride = geometry->stride[2];
    buf->u.YUVA.v_size = geometry->size[2];
    buf->u.YUVA.a = Plane(geometry, 3);
    buf->u.YUVA.a_stride = geometry->stride[3];
    buf->u.YUVA.a_size = geometry->size[3];
  }
}

static void Features(const uint8_t* data, size_t size) {
  WebPBitstreamFeatures features;
  int width = -1, height = -1;
  int ok;
  memset(&features, 0x5a, sizeof(features));
  printf("status=%d\n", (int)WebPGetFeatures(data, size, &features));
  PrintFeatures("features", &features);
  ok = WebPGetInfo(data, size, &width, &height);
  printf("info=%d,%d,%d\n", ok, width, height);
}

static void Decode(const uint8_t* data, size_t size, int mode,
                   const char* options, const char* geometry_text) {
  WebPDecoderConfig config;
  Geometry geometry;
  VP8StatusCode status;
  if (!WebPInitDecoderConfig(&config)) exit(2);
  ParseOptions(options, &config.options);
  config.output.colorspace = (WEBP_CSP_MODE)mode;
  ParseGeometry(geometry_text, &geometry);
  SetGeometry(&config.output, &geometry);
  printf("valid=%d\n", WebPValidateDecoderConfig(&config));
  status = WebPDecode(data, size, &config);
  printf("status=%d\n", (int)status);
  PrintFeatures("input", &config.input);
  PrintBuffer(&config.output, &geometry,
              config.output.private_memory ? PrivateSize(&config.output) : 0);
  WebPFreeDecBuffer(&config.output);
  FreeGeometry(&geometry);
}

static void Validate(int mode, const char* options, int width, int height) {
  WebPDecoderConfig config;
  if (!WebPInitDecoderConfig(&config)) exit(2);
  ParseOptions(options, &config.options);
  config.output.colorspace = (WEBP_CSP_MODE)mode;
  config.input.width = width;
  config.input.height = height;
  printf("valid=%d\n", WebPValidateDecoderConfig(&config));
}

static void Simple(const uint8_t* data, size_t size, const char* func) {
  int width = -1, height = -1;
  uint8_t* out = NULL;
  int bpp = 0;
  if (!strcmp(func, "RGB")) {
    out = WebPDecodeRGB(data, size, &width, &height), bpp = 3;
  } else if (!strcmp(func, "RGBA")) {
    out = WebPDecodeRGBA(data, size, &width, &height), bpp = 4;
  } else if (!strcmp(func, "ARGB")) {
    out = WebPDecodeARGB(data, size, &width, &height), bpp = 4;
  } else if (!strcmp(func, "BGR")) {
    out = WebPDecodeBGR(data, size, &width, &height), bpp = 3;
  } else if (!strcmp(func, "BGRA")) {
    out = WebPDecodeBGRA(data, size, &width, &height), bpp = 4;
  } else if (!strcmp(func, "YUV")) {
    uint8_t* u = NULL;
    uint8_t* v = NULL;
    int stride = -1, uv_stride = -1;
    out = WebPDecodeYUV(data, size, &width, &height, &u, &v, &stride,
                        &uv_stride);
    printf("dims=%d,%d\n", width, height);
    if (out == NULL) {
      printf("out=null\n");
    } else {
      const size_t uv_size = (size_t)uv_stride * ((height + 1) / 2);
      printf("yuv=%lld,%lld,%d,%d\n", (long long)(u - out),
             (long long)(v - out), stride, uv_stride);
      PrintDigest("y", out, (size_t)stride * height);
      PrintDigest("u", u, uv_size);
      PrintDigest("v", v, uv_size);
      WebPFree(out);
    }
    return;
  } else {
    exit(2);
  }
  printf("dims=%d,%d\n", width, height);
  if (out == NULL) {
    printf("out=null\n");
  } else {
    PrintDigest("out", out, (size_t)width * height * bpp);
    WebPFree(out);
  }
}

static void Into(const uint8_t* data, size_t size, const char* func,
                 const char* geometry_text) {
  Geometry geometry;
  uint8_t* out = NULL;
  ParseGeometry(geometry_text, &geometry);
  if (!strcmp(func, "YUV")) {
    out = WebPDecodeYUVInto(data, size, Plane(&geometry, 0), geometry.size[0],
                            geometry.stride[0], Plane(&geometry, 1),
                            geometry.size[1], geometry.stride[1],
                            Plane(&geometry, 2), geometry.size[2],
                            geometry.stride[2]);
  } else {
    uint8_t* const buf = Plane(&geometry, 0);
    const size_t buf_size = geometry.size[0];
    const int stride = geometry.stride[0];
    if (!strcmp(func, "RGB")) {
      out = WebPDecodeRGBInto(data, size, buf, buf_size, stride);
    } else if (!strcmp(func, "RGBA")) {
      out = WebPDecodeRGBAInto(data, size, buf, buf_size, stride);
    } else if (!strcmp(func, "ARGB")) {
      out = WebPDecodeARGBInto(data, size, buf, buf_size, stride);
    } else if (!strcmp(func, "BGR")) {
      out = WebPDecodeBGRInto(data, size, buf, buf_size, stride);
    } else if (!strcmp(func, "BGRA")) {
      out = WebPDecodeBGRAInto(data, size, buf, buf_size, stride);
    } else {
      exit(2);
    }
  }
  printf("result=%s\n", out == NULL ? "null" : (out == Plane(&geometry, 0))
                                                   ? "buffer" : "other");
  PrintGeometry(&geometry);
  FreeGeometry(&geometry);
}

// The state of the incremental decoder after one step. The decoded rows are
// digested when `last_y` changed since the previous step (`*prev_last_y`).
static void PrintIncremental(const WebPIDecoder* const idec, int yuv,
                             const Geometry* const geometry,
                             int* const prev_last_y) {
  int left = -1, top = -1, width = -1, height = -1;
  const WebPDecBuffer* const area =
      WebPIDecodedArea(idec, &left, &top, &width, &height);
  const uint8_t* const base =
      (area != NULL) ? area->private_memory : NULL;
  printf("area=%d,%d,%d,%d,%d\n", area != NULL, left, top, width, height);
  if (!yuv) {
    int last_y = -1, w = -1, h = -1, stride = 0;
    const uint8_t* const rgb = WebPIDecGetRGB(idec, &last_y, &w, &h, &stride);
    if (rgb == NULL) {
      printf("rgb=null,%d,%d,%d,%d\n", last_y, w, h, stride);
    } else {
      const uint8_t* const origin = geometry->count ? Plane(geometry, 0) : base;
      printf("rgb=%lld,%d,%d,%d,%d\n", Offset(rgb, origin), last_y, w, h,
             stride);
      if (last_y != *prev_last_y) {
        PrintRowsDigest("rows", rgb, stride, w * kModeBpp[area->colorspace],
                        last_y);
        *prev_last_y = last_y;
      }
    }
  } else {
    int last_y = -1, w = -1, h = -1, stride = 0, uv_stride = 0, a_stride = 0;
    uint8_t* u = NULL;
    uint8_t* v = NULL;
    uint8_t* a = NULL;
    const uint8_t* const y = WebPIDecGetYUVA(idec, &last_y, &u, &v, &a, &w, &h,
                                             &stride, &uv_stride, &a_stride);
    if (y == NULL) {
      printf("yuva=null,%d,%d,%d\n", last_y, w, h);
    } else {
      const int ext = geometry->count;
      printf("yuva=%lld,%lld,%lld,%lld,%d,%d,%d,%d,%d,%d\n",
             Offset(y, ext ? Plane(geometry, 0) : base),
             Offset(u, ext ? Plane(geometry, 1) : base),
             Offset(v, ext ? Plane(geometry, 2) : base),
             Offset(a, ext ? Plane(geometry, 3) : base), last_y, w, h, stride,
             uv_stride, a_stride);
      if (last_y != *prev_last_y) {
        // `last_y` counts luma rows. The U, V and alpha rows of a rescaled
        // output can lag behind, so in library memory, which is not
        // initialized, only caller memory has defined bytes there.
        PrintRowsDigest("y", y, stride, w, last_y);
        if (ext) {
          PrintRowsDigest("u", u, uv_stride, (w + 1) / 2, (last_y + 1) / 2);
          PrintRowsDigest("v", v, uv_stride, (w + 1) / 2, (last_y + 1) / 2);
          if (a != NULL) PrintRowsDigest("a", a, a_stride, w, last_y);
        }
        *prev_last_y = last_y;
      }
    }
  }
}

static void Incremental(const uint8_t* data, size_t size, size_t chunk,
                        int append, const char* kind, int mode,
                        const char* options, const char* geometry_text) {
  WebPDecoderConfig config;
  Geometry geometry;
  WebPIDecoder* idec = NULL;
  uint8_t* copy = NULL;
  size_t pos = 0;
  int yuv;
  if (!WebPInitDecoderConfig(&config)) exit(2);
  ParseOptions(options, &config.options);
  ParseGeometry(geometry_text, &geometry);
  if (!strcmp(kind, "config")) {
    config.output.colorspace = (WEBP_CSP_MODE)mode;
    SetGeometry(&config.output, &geometry);
    idec = WebPIDecode(NULL, 0, &config);
  } else if (!strcmp(kind, "new")) {
    mode = MODE_RGB;
    idec = WebPINewDecoder(NULL);
  } else if (!strcmp(kind, "rgb")) {
    idec = WebPINewRGB((WEBP_CSP_MODE)mode, Plane(&geometry, 0),
                       geometry.size[0], geometry.stride[0]);
  } else if (!strcmp(kind, "yuva") || !strcmp(kind, "yuv")) {
    mode = (geometry.count == 3 || !strcmp(kind, "yuv")) && geometry.count
               ? MODE_YUV
               : MODE_YUVA;
    if (!strcmp(kind, "yuv")) {
      idec = WebPINewYUV(Plane(&geometry, 0), geometry.size[0],
                         geometry.stride[0], Plane(&geometry, 1),
                         geometry.size[1], geometry.stride[1],
                         Plane(&geometry, 2), geometry.size[2],
                         geometry.stride[2]);
    } else {
      idec = WebPINewYUVA(Plane(&geometry, 0), geometry.size[0],
                          geometry.stride[0], Plane(&geometry, 1),
                          geometry.size[1], geometry.stride[1],
                          Plane(&geometry, 2), geometry.size[2],
                          geometry.stride[2], Plane(&geometry, 3),
                          geometry.size[3], geometry.stride[3]);
    }
  } else {
    exit(2);
  }
  printf("idec=%d\n", idec != NULL);
  if (idec == NULL) {
    FreeGeometry(&geometry);
    return;
  }
  yuv = !WebPIsRGBMode((WEBP_CSP_MODE)mode);
  {
  int prev_last_y = -1;
  g_partial = 1;
  while (1) {
    const size_t next = (size - pos < chunk) ? size : pos + chunk;
    VP8StatusCode status;
    // APPEND 2 alternates the two functions, which mixes the memory modes.
    if (append == 1 || (append == 2 && pos == 0)) {
      status = WebPIAppend(idec, data + pos, next - pos);
    } else {
      uint8_t* const fresh = (uint8_t*)malloc(next + 1);
      if (fresh == NULL) exit(2);
      memcpy(fresh, data, next);
      status = WebPIUpdate(idec, fresh, next);
      free(copy);
      copy = fresh;
    }
    pos = next;
    printf("step=%llu,%d\n", (unsigned long long)pos, (int)status);
    PrintIncremental(idec, yuv, &geometry, &prev_last_y);
    g_partial = (status != VP8_STATUS_OK);
    if (status != VP8_STATUS_SUSPENDED || pos == size) break;
  }
  }
  {
    const WebPDecBuffer* const area = WebPIDecodedArea(idec, NULL, NULL, NULL,
                                                       NULL);
    if (!strcmp(kind, "config")) {
      PrintBuffer(&config.output, &geometry,
                  config.output.private_memory ? PrivateSize(&config.output)
                                               : 0);
    } else if (area != NULL) {
      PrintBuffer(area, &geometry,
                  area->private_memory ? PrivateSize(area) : 0);
    } else {
      PrintGeometry(&geometry);
    }
  }
  g_partial = 0;
  WebPIDelete(idec);
  free(copy);
  WebPFreeDecBuffer(&config.output);
  FreeGeometry(&geometry);
}

int main(int argc, char* argv[]) {
  static char line[1 << 17];
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "decode < commands");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    static char a[1 << 16];
    char cmd[32], b[256], c[256], d[256], e[256], f[256], g[256], h[256];
    const int n = sscanf(line,
                         "%31s %65535s %255s %255s %255s %255s %255s %255s %255s",
                         cmd, a, b, c, d, e, f, g, h);
    size_t size;
    uint8_t* data;
    if (n < 1) continue;
    printf("> %s", line);
    if (!strcmp(cmd, "features") && n == 3) {
      data = ReadPrefix(a, atol(b), &size);
      Features(data, size);
    } else if (!strcmp(cmd, "featureshex") && n == 2) {
      data = ParseHex(a, &size);
      Features(data, size);
    } else if (!strcmp(cmd, "decode") && n == 6) {
      data = ReadPrefix(a, atol(b), &size);
      Decode(data, size, atoi(c), d, e);
    } else if (!strcmp(cmd, "validate") && n == 5) {
      Validate(atoi(a), b, atoi(c), atoi(d));
      continue;
    } else if (!strcmp(cmd, "simple") && n == 4) {
      data = ReadPrefix(a, atol(b), &size);
      Simple(data, size, c);
    } else if (!strcmp(cmd, "into") && n == 5) {
      data = ReadPrefix(a, atol(b), &size);
      Into(data, size, c, d);
    } else if (!strcmp(cmd, "incr") && n == 9) {
      data = ReadPrefix(a, atol(b), &size);
      Incremental(data, size, (size_t)atol(c), atoi(d), e, atoi(f), g, h);
    } else {
      fprintf(stderr, "bad command: %s", line);
      return 2;
    }
    free(data);
    fflush(stdout);
  }
  return 0;
}
