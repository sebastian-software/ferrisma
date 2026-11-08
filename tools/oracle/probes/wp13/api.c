// WP13 probe: the encode.h API apart from WebPEncode's matrix, through the
// public header only, so that it also links against the Rust library
// (probes/wp13/api_rs, oracle.cmake). tests/wp13_abi.rs compares api with
// api_rs; tests/wp13_api.rs compares the simple API and the configuration
// with the Rust translation (ADR 0002).
//
// Standard input holds one command per line. Pictures are P[0] to P[3]; input
// samples come from a xorshift32 generator (Fill), which the Rust tests
// reproduce. Commands and what they print:
//
//   version                         version=
//   null                            the result of every function with NULL
//                                   arguments (null.<name>=)
//   config preset quality_bits ver  WebPConfigInitInternal on 0xa5 bytes: ret=, config=
//   lpreset level                   WebPConfigLosslessPreset: ret=, config=
//   validate field value            WebPValidateConfig of the default config
//                                   with field (0-based int index) set: valid=
//   writer size...                  WebPMemoryWrite of chunks of these sizes:
//                                   write=ret,size,max_size,digest per chunk,
//                                   then clear=size,max_size,mem_is_null
//   simple f w h stride q_bits seed amode flags
//                                   WebPEncode{RGB,BGR,RGBA,BGRA} (f 0-3) and
//                                   WebPEncodeLossless* (f 4-7) on generated
//                                   rows; flags bit 0 passes NULL output, bit 1
//                                   NULL input: size=, output= (untouched,
//                                   null or set), out=
//   init i                          WebPPictureInit: ret=
//   set i use_argb colorspace w h   sets the fields
//   alloc i | free i                WebPPictureAlloc (ret=, error=) | WebPPictureFree
//   import i kind stride seed amode WebPPictureImport{RGB,RGBA,RGBX,BGR,BGRA,BGRX}
//                                   (kind 0-5) on generated rows: ret=, error=
//   caller i pad                    moves P[i]'s planes into caller memory with
//                                   rows pad samples longer
//   copy i j | view i l t w h j | crop i l t w h | rescale i w h
//   argb2yuva i cs dith_bits (dith_bits < 0: WebPPictureARGBToYUVA)
//   sharp i | smart i | yuva2argb i | cleanup i | blend i rgb
//                                   ret= (where the function returns one),
//                                   error=, then pic= of the target
//   transparent i | isview i        ret=
//   distortion i j type             ret=, result= (float bit patterns)
//   planedist w h stride xstep type seed1 seed2
//                                   WebPPlaneDistortion: ret=, result=
//   print i                         pic=
//
// pic= prints use_argb, colorspace, width, height, the four strides,
// WebPPictureIsView, and the FNV-1a digests of the visible samples of the y,
// u, v, a and argb planes (- for NULL).

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/webp/encode.h"

#define NUM_PICTURES 4

static WebPPicture P[NUM_PICTURES];
static void* g_caller[256];
static int g_num_caller = 0;

static uint32_t Next(uint32_t* s) {
  *s ^= *s << 13;
  *s ^= *s >> 17;
  *s ^= *s << 5;
  return *s;
}

// Fills `size` bytes; with 4-byte pixels, amode 1 makes the fourth byte 0xff
// and amode 2 makes it 0 or 0xff.
static void Fill(uint8_t* data, size_t size, uint32_t seed, int amode) {
  uint32_t s = seed | 1;
  size_t i;
  for (i = 0; i < size; ++i) {
    data[i] = (uint8_t)Next(&s);
    if ((i & 3) == 3) {
      if (amode == 1) data[i] = 0xff;
      if (amode == 2) data[i] = (data[i] & 0x10) ? 0xff : 0;
    }
  }
}

static float Float(int32_t bits) {
  float f;
  memcpy(&f, &bits, 4);
  return f;
}

static uint32_t Bits(float f) {
  uint32_t bits;
  memcpy(&bits, &f, 4);
  return bits;
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

static void PrintPicture(const WebPPicture* pic) {
  const int w = (pic->width > 0) ? pic->width : 0;
  const int h = (pic->height > 0) ? pic->height : 0;
  printf("pic=%d,%d,%d,%d,%d,%d,%d,%d,%d", pic->use_argb, (int)pic->colorspace,
         pic->width, pic->height, pic->y_stride, pic->uv_stride, pic->a_stride,
         pic->argb_stride, WebPPictureIsView(pic));
  PrintPlane(pic->y, pic->y_stride, w, h);
  PrintPlane(pic->u, pic->uv_stride, (w + 1) >> 1, (h + 1) >> 1);
  PrintPlane(pic->v, pic->uv_stride, (w + 1) >> 1, (h + 1) >> 1);
  PrintPlane(pic->a, pic->a_stride, w, h);
  PrintArgb(pic->argb, pic->argb_stride, w, h);
  printf("\n");
}

static void* Keep(void* p) {
  if (p == NULL || g_num_caller == 256) exit(2);
  g_caller[g_num_caller++] = p;
  return p;
}

static uint8_t* CopyPlane(const uint8_t* src, int src_stride, int width,
                          int height, int stride) {
  uint8_t* const dst = (uint8_t*)Keep(malloc((size_t)stride * height + 1));
  int y;
  memset(dst, 0x5a, (size_t)stride * height + 1);
  for (y = 0; y < height; ++y) {
    memcpy(dst + (size_t)y * stride, src + (ptrdiff_t)y * src_stride,
           (size_t)width);
  }
  return dst;
}

// Moves the planes of `pic` into caller memory (rows `pad` samples longer).
static void ToCaller(WebPPicture* pic, int pad) {
  WebPPicture out = *pic;
  out.memory_ = NULL;
  out.memory_argb_ = NULL;
  if (pic->use_argb) {
    const int stride = pic->width + pad;
    uint32_t* const argb = (uint32_t*)Keep(
        malloc(((size_t)stride * pic->height + 1) * sizeof(uint32_t)));
    int y;
    memset(argb, 0x5a, ((size_t)stride * pic->height + 1) * sizeof(uint32_t));
    for (y = 0; y < pic->height; ++y) {
      memcpy(argb + (size_t)y * stride, pic->argb + (size_t)y * pic->argb_stride,
             (size_t)pic->width * 4);
    }
    out.argb = argb;
    out.argb_stride = stride;
    out.y = out.u = out.v = out.a = NULL;
  } else {
    const int uv_w = (pic->width + 1) >> 1, uv_h = (pic->height + 1) >> 1;
    out.y = CopyPlane(pic->y, pic->y_stride, pic->width, pic->height,
                      pic->width + pad);
    out.u = CopyPlane(pic->u, pic->uv_stride, uv_w, uv_h, uv_w + pad);
    out.v = CopyPlane(pic->v, pic->uv_stride, uv_w, uv_h, uv_w + pad);
    out.y_stride = pic->width + pad;
    out.uv_stride = uv_w + pad;
    if (pic->a != NULL) {
      out.a = CopyPlane(pic->a, pic->a_stride, pic->width, pic->height,
                        pic->width + pad);
      out.a_stride = pic->width + pad;
    }
    out.argb = NULL;
  }
  WebPPictureFree(pic);
  *pic = out;
}

static void PrintConfig(const WebPConfig* config) {
  ProbePrintHex("config", (const uint8_t*)config, sizeof(*config));
}

static void Null(void) {
  WebPPicture pic;
  WebPConfig config;
  float f[5];
  uint8_t* out = (uint8_t*)1;
  uint8_t data[16] = {0};
  if (!WebPPictureInit(&pic) || !WebPConfigInit(&config)) exit(2);
  pic.width = pic.height = 1;
  printf("null.WebPEncodeRGB=%zu\n", WebPEncodeRGB(data, 1, 1, 3, 75.f, NULL));
  printf("null.WebPEncodeLosslessBGRA=%zu\n",
         WebPEncodeLosslessBGRA(data, 1, 1, 4, NULL));
  printf("null.WebPConfigInitInternal=%d\n",
         WebPConfigInitInternal(NULL, WEBP_PRESET_DEFAULT, 75.f,
                                WEBP_ENCODER_ABI_VERSION));
  printf("null.WebPConfigLosslessPreset=%d\n", WebPConfigLosslessPreset(NULL, 3));
  printf("null.WebPValidateConfig=%d\n", WebPValidateConfig(NULL));
  WebPMemoryWriterClear(NULL);
  printf("null.WebPPictureInitInternal=%d\n",
         WebPPictureInitInternal(NULL, WEBP_ENCODER_ABI_VERSION));
  printf("null.WebPPictureInitInternal.bad=%d\n",
         WebPPictureInitInternal(&pic, WEBP_ENCODER_ABI_VERSION + 0x100));
  printf("null.WebPPictureAlloc=%d\n", WebPPictureAlloc(NULL));
  WebPPictureFree(NULL);
  printf("null.WebPPictureCopy=%d,%d\n", WebPPictureCopy(NULL, &pic),
         WebPPictureCopy(&pic, NULL));
  printf("null.WebPPictureCopy.same=%d\n", WebPPictureCopy(&pic, &pic));
  printf("null.WebPPlaneDistortion=%d,%d,%d,%d\n",
         WebPPlaneDistortion(NULL, 1, data, 1, 1, 1, 1, 0, f, f + 1),
         WebPPlaneDistortion(data, 1, NULL, 1, 1, 1, 1, 0, f, f + 1),
         WebPPlaneDistortion(data, 1, data, 1, 1, 1, 1, 0, NULL, f + 1),
         WebPPlaneDistortion(data, 1, data, 1, 1, 1, 1, 0, f, NULL));
  printf("null.WebPPictureDistortion=%d,%d,%d\n",
         WebPPictureDistortion(NULL, &pic, 0, f),
         WebPPictureDistortion(&pic, NULL, 0, f),
         WebPPictureDistortion(&pic, &pic, 0, NULL));
  printf("null.WebPPictureCrop=%d\n", WebPPictureCrop(NULL, 0, 0, 1, 1));
  printf("null.WebPPictureView=%d,%d\n",
         WebPPictureView(NULL, 0, 0, 1, 1, &pic),
         WebPPictureView(&pic, 0, 0, 1, 1, NULL));
  printf("null.WebPPictureIsView=%d\n", WebPPictureIsView(NULL));
  printf("null.WebPPictureRescale=%d\n", WebPPictureRescale(NULL, 1, 1));
  printf("null.WebPPictureImport=%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\n",
         WebPPictureImportRGB(NULL, data, 3), WebPPictureImportRGB(&pic, NULL, 3),
         WebPPictureImportRGBA(NULL, data, 4),
         WebPPictureImportRGBA(&pic, NULL, 4),
         WebPPictureImportRGBX(NULL, data, 4),
         WebPPictureImportRGBX(&pic, NULL, 4),
         WebPPictureImportBGR(NULL, data, 3), WebPPictureImportBGR(&pic, NULL, 3),
         WebPPictureImportBGRA(NULL, data, 4),
         WebPPictureImportBGRA(&pic, NULL, 4),
         WebPPictureImportBGRX(NULL, data, 4),
         WebPPictureImportBGRX(&pic, NULL, 4));
  printf("null.WebPPictureARGBToYUVA=%d,%d,%d,%d\n",
         WebPPictureARGBToYUVA(NULL, WEBP_YUV420),
         WebPPictureARGBToYUVADithered(NULL, WEBP_YUV420, 1.f),
         WebPPictureSharpARGBToYUVA(NULL), WebPPictureSmartARGBToYUVA(NULL));
  printf("null.WebPPictureYUVAToARGB=%d\n", WebPPictureYUVAToARGB(NULL));
  WebPCleanupTransparentArea(NULL);
  printf("null.WebPPictureHasTransparency=%d\n", WebPPictureHasTransparency(NULL));
  WebPBlendAlpha(NULL, 0);
  printf("null.WebPEncode=%d\n", WebPEncode(&config, NULL));
  pic.error_code = VP8_ENC_ERROR_BAD_WRITE;
  printf("null.WebPEncode.config=%d,%d\n", WebPEncode(NULL, &pic),
         (int)pic.error_code);
  // The picture functions on a picture without planes.
  pic.error_code = VP8_ENC_OK;
  printf("empty.WebPPictureARGBToYUVA=%d,%d\n",
         WebPPictureARGBToYUVA(&pic, WEBP_YUV420), (int)pic.error_code);
  pic.error_code = VP8_ENC_OK;
  printf("empty.WebPPictureYUVAToARGB=%d,%d\n", WebPPictureYUVAToARGB(&pic),
         (int)pic.error_code);
  pic.error_code = VP8_ENC_OK;
  printf("empty.WebPPictureHasTransparency=%d\n", WebPPictureHasTransparency(&pic));
  printf("empty.WebPPictureIsView=%d\n", WebPPictureIsView(&pic));
  (void)out;
  WebPPictureFree(&pic);
}

static void Simple(int f, int w, int h, int stride, float q, uint32_t seed,
                   int amode, int flags) {
  static size_t (*const lossy[4])(const uint8_t*, int, int, int, float,
                                  uint8_t**) = {
      WebPEncodeRGB, WebPEncodeBGR, WebPEncodeRGBA, WebPEncodeBGRA};
  static size_t (*const lossless[4])(const uint8_t*, int, int, int,
                                     uint8_t**) = {
      WebPEncodeLosslessRGB, WebPEncodeLosslessBGR, WebPEncodeLosslessRGBA,
      WebPEncodeLosslessBGRA};
  const size_t rows = (size_t)(h > 0 ? h : 1);
  const size_t row = (size_t)(stride < 0 ? -stride : stride);
  const size_t size = rows * row + 64;
  uint8_t* const buffer = (uint8_t*)malloc(size);
  uint8_t* const sentinel = (uint8_t*)&g_num_caller;
  const uint8_t* input = buffer;
  uint8_t* output = sentinel;
  size_t result;
  if (buffer == NULL) exit(2);
  Fill(buffer, size, seed, amode);
  if (stride < 0) input = buffer + (rows - 1) * row;
  if (flags & 2) input = NULL;
  if (f < 4) {
    result = lossy[f](input, w, h, stride, q, (flags & 1) ? NULL : &output);
  } else {
    result = lossless[f - 4](input, w, h, stride, (flags & 1) ? NULL : &output);
  }
  printf("size=%zu\n", result);
  printf("output=%s\n", (output == sentinel) ? "untouched"
                        : (output == NULL)   ? "null"
                                             : "set");
  if (output != sentinel && output != NULL) {
    ProbePrintHex("out", output, result);
    WebPFree(output);
  }
  free(buffer);
}

static void Writer(int count, const int* sizes) {
  WebPMemoryWriter writer;
  WebPPicture pic;
  int i;
  if (!WebPPictureInit(&pic)) exit(2);
  memset(&writer, 0xa5, sizeof(writer));
  WebPMemoryWriterInit(&writer);
  pic.custom_ptr = &writer;
  for (i = 0; i < count; ++i) {
    uint8_t* const data = (uint8_t*)malloc((size_t)sizes[i] + 1);
    int ret;
    if (data == NULL) exit(2);
    Fill(data, (size_t)sizes[i] + 1, (uint32_t)(i + 7), 0);
    ret = WebPMemoryWrite(data, (size_t)sizes[i], &pic);
    printf("write=%d,%zu,%zu,%016llx,%u\n", ret, writer.size, writer.max_size,
           (unsigned long long)Fnv(FNV_START, writer.mem, writer.size),
           writer.pad[0]);
    free(data);
  }
  WebPMemoryWriterClear(&writer);
  printf("clear=%zu,%zu,%d\n", writer.size, writer.max_size, writer.mem == NULL);
  pic.custom_ptr = NULL;
  printf("write.null=%d\n", WebPMemoryWrite((const uint8_t*)"x", 1, &pic));
}

static int Import(WebPPicture* pic, int kind, int stride, uint32_t seed,
                  int amode) {
  static int (*const importers[6])(WebPPicture*, const uint8_t*, int) = {
      WebPPictureImportRGB, WebPPictureImportRGBA, WebPPictureImportRGBX,
      WebPPictureImportBGR, WebPPictureImportBGRA, WebPPictureImportBGRX};
  const size_t rows = (size_t)(pic->height > 0 ? pic->height : 1);
  const size_t row = (size_t)(stride < 0 ? -stride : stride);
  const size_t size = rows * row + 64;
  uint8_t* const buffer = (uint8_t*)malloc(size);
  const uint8_t* input = buffer;
  int ret;
  if (buffer == NULL) exit(2);
  Fill(buffer, size, seed, amode);
  if (stride < 0) input = buffer + (rows - 1) * row;
  ret = importers[kind](pic, input, stride);
  free(buffer);
  return ret;
}

static void Result(int ret, const WebPPicture* pic) {
  printf("ret=%d\nerror=%d\n", ret, (int)pic->error_code);
  PrintPicture(pic);
}

int main(int argc, char* argv[]) {
  char line[4096];
  int i;
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "api < commands.txt");
  for (i = 0; i < NUM_PICTURES; ++i) {
    if (!WebPPictureInit(&P[i])) return 2;
  }
  while (fgets(line, sizeof(line), stdin) != NULL) {
    char cmd[32];
    int a[8] = {0};
    const int n = sscanf(line, "%31s %d %d %d %d %d %d %d %d", cmd, &a[0], &a[1],
                         &a[2], &a[3], &a[4], &a[5], &a[6], &a[7]);
    if (n < 1) continue;
    printf("cmd=%s\n", cmd);
    if (!strcmp(cmd, "version")) {
      printf("version=%d\n", WebPGetEncoderVersion());
    } else if (!strcmp(cmd, "null")) {
      Null();
    } else if (!strcmp(cmd, "config")) {
      WebPConfig config;
      int ret;
      memset(&config, 0xa5, sizeof(config));
      ret = WebPConfigInitInternal(&config, (WebPPreset)a[0], Float(a[1]), a[2]);
      printf("ret=%d\n", ret);
      PrintConfig(&config);
    } else if (!strcmp(cmd, "lpreset")) {
      WebPConfig config;
      int ret;
      memset(&config, 0xa5, sizeof(config));
      if (!WebPConfigInit(&config)) return 2;
      ret = WebPConfigLosslessPreset(&config, a[0]);
      printf("ret=%d\n", ret);
      PrintConfig(&config);
    } else if (!strcmp(cmd, "validate")) {
      WebPConfig config;
      if (!WebPConfigInit(&config)) return 2;
      ((int*)&config)[a[0]] = a[1];
      printf("valid=%d\n", WebPValidateConfig(&config));
    } else if (!strcmp(cmd, "writer")) {
      Writer(n - 1, a);
    } else if (!strcmp(cmd, "simple")) {
      Simple(a[0], a[1], a[2], a[3], Float(a[4]), (uint32_t)a[5], a[6], a[7]);
    } else if (!strcmp(cmd, "init")) {
      printf("ret=%d\n", WebPPictureInit(&P[a[0]]));
    } else if (!strcmp(cmd, "set")) {
      P[a[0]].use_argb = a[1];
      P[a[0]].colorspace = (WebPEncCSP)a[2];
      P[a[0]].width = a[3];
      P[a[0]].height = a[4];
    } else if (!strcmp(cmd, "alloc")) {
      Result(WebPPictureAlloc(&P[a[0]]), &P[a[0]]);
    } else if (!strcmp(cmd, "free")) {
      WebPPictureFree(&P[a[0]]);
      PrintPicture(&P[a[0]]);
    } else if (!strcmp(cmd, "import")) {
      Result(Import(&P[a[0]], a[1], a[2], (uint32_t)a[3], a[4]), &P[a[0]]);
    } else if (!strcmp(cmd, "caller")) {
      ToCaller(&P[a[0]], a[1]);
      PrintPicture(&P[a[0]]);
    } else if (!strcmp(cmd, "copy")) {
      Result(WebPPictureCopy(&P[a[0]], &P[a[1]]), &P[a[1]]);
    } else if (!strcmp(cmd, "view")) {
      Result(WebPPictureView(&P[a[0]], a[1], a[2], a[3], a[4], &P[a[5]]),
             &P[a[5]]);
    } else if (!strcmp(cmd, "crop")) {
      Result(WebPPictureCrop(&P[a[0]], a[1], a[2], a[3], a[4]), &P[a[0]]);
    } else if (!strcmp(cmd, "rescale")) {
      Result(WebPPictureRescale(&P[a[0]], a[1], a[2]), &P[a[0]]);
    } else if (!strcmp(cmd, "argb2yuva")) {
      const int ret =
          (a[2] < 0) ? WebPPictureARGBToYUVA(&P[a[0]], (WebPEncCSP)a[1])
                     : WebPPictureARGBToYUVADithered(&P[a[0]], (WebPEncCSP)a[1],
                                                     Float(a[2]));
      Result(ret, &P[a[0]]);
    } else if (!strcmp(cmd, "sharp")) {
      Result(WebPPictureSharpARGBToYUVA(&P[a[0]]), &P[a[0]]);
    } else if (!strcmp(cmd, "smart")) {
      Result(WebPPictureSmartARGBToYUVA(&P[a[0]]), &P[a[0]]);
    } else if (!strcmp(cmd, "yuva2argb")) {
      Result(WebPPictureYUVAToARGB(&P[a[0]]), &P[a[0]]);
    } else if (!strcmp(cmd, "cleanup")) {
      WebPCleanupTransparentArea(&P[a[0]]);
      PrintPicture(&P[a[0]]);
    } else if (!strcmp(cmd, "blend")) {
      WebPBlendAlpha(&P[a[0]], (uint32_t)a[1]);
      PrintPicture(&P[a[0]]);
    } else if (!strcmp(cmd, "transparent")) {
      printf("ret=%d\n", WebPPictureHasTransparency(&P[a[0]]));
    } else if (!strcmp(cmd, "isview")) {
      printf("ret=%d\n", WebPPictureIsView(&P[a[0]]));
    } else if (!strcmp(cmd, "distortion")) {
      float result[5];
      int k;
      const int ret = WebPPictureDistortion(&P[a[0]], &P[a[1]], a[2], result);
      printf("ret=%d\n", ret);
      if (ret) {
        printf("result=");
        for (k = 0; k < 5; ++k) printf("%s%08x", k ? "," : "", Bits(result[k]));
        printf("\n");
      }
    } else if (!strcmp(cmd, "planedist")) {
      const int w = a[0], h = a[1], stride = a[2];
      const size_t size = (size_t)(stride * (h > 0 ? h : 1)) + 64;
      uint8_t* const src = (uint8_t*)malloc(size);
      uint8_t* const ref = (uint8_t*)malloc(size);
      float distortion = -1.f, result = -1.f;
      int ret;
      if (src == NULL || ref == NULL) return 2;
      Fill(src, size, (uint32_t)a[5], 0);
      Fill(ref, size, (uint32_t)a[6], 0);
      ret = WebPPlaneDistortion(src, (size_t)stride, ref, (size_t)stride, w, h,
                                (size_t)a[3], a[4], &distortion, &result);
      printf("ret=%d\nresult=%08x,%08x\n", ret, Bits(distortion), Bits(result));
      free(src);
      free(ref);
    } else if (!strcmp(cmd, "print")) {
      PrintPicture(&P[a[0]]);
    } else {
      fprintf(stderr, "unknown command %s\n", cmd);
      return 2;
    }
    fflush(stdout);
  }
  for (i = 0; i < NUM_PICTURES; ++i) WebPPictureFree(&P[i]);
  for (i = 0; i < g_num_caller; ++i) free(g_caller[i]);
  return 0;
}
