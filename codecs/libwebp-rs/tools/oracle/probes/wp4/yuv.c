// WP4 probe: the conversions of src/dsp/yuv.h and src/dsp/yuv.c, the row
// samplers WebPSamplers[mode], WebPSamplerProcessPlane and the ARGB, RGB and
// RGBA to Y and UV converters (ADR 0002). It reads commands from standard
// input, one per line, and prints what the library computes.
//
// Byte arrays are hex strings with two digits per byte, "-" for an empty array;
// 32-bit ARGB values are eight hex digits, 16-bit RGBA values four.
//
//   init                                  WebPInitSamplers, WebPInitConvertARGBToYUV
//   row MODE LEN YHEX UHEX VHEX           WebPSamplers[MODE] over LEN pixels; the
//                                         destination holds 4*LEN+8 bytes, filled
//                                         with 0xaa before the call; prints dst=HEX
//   plane MODE W H YSTRIDE UVSTRIDE DSTSTRIDE YHEX UHEX VHEX
//                                         WebPSamplerProcessPlane with WebPSamplers[MODE];
//                                         the destination holds DSTSTRIDE*H bytes; prints dst=HEX
//   argb_y W ARGBHEX                      WebPConvertARGBToY; prints y=HEX
//   argb_uv W DO_STORE ARGBHEX UHEX VHEX  WebPConvertARGBToUV on u and v that start
//                                         with the given contents; prints u=HEX v=HEX
//   rgb_y W RGBHEX                        WebPConvertRGB24ToY; prints y=HEX
//   bgr_y W BGRHEX                        WebPConvertBGR24ToY; prints y=HEX
//   rgba_uv W RGBA16HEX                   WebPConvertRGBA32ToUV; prints u=HEX v=HEX
//   yuv Y U V                             the inline YUV to RGB conversions; prints
//                                         the component values and the packed forms
//   rgb_yuv R G B ROUNDING                VP8RGBToY, VP8RGBToU and VP8RGBToV; prints
//                                         y, u and v in decimal
//
// tests/wp4_yuv.rs generates the commands and the expected output.

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"
#include "src/dsp/yuv.h"
#include "src/webp/decode.h"

#define SENTINEL 0xaa

static char* ReadToken(void) {
  size_t cap = 64, len = 0;
  int c;
  char* s = (char*)malloc(cap);
  if (s == NULL) exit(2);
  do {
    c = getchar();
  } while (c != EOF && isspace(c));
  if (c == EOF) {
    free(s);
    return NULL;
  }
  do {
    if (len + 1 >= cap) {
      cap *= 2;
      s = (char*)realloc(s, cap);
      if (s == NULL) exit(2);
    }
    s[len++] = (char)c;
    c = getchar();
  } while (c != EOF && !isspace(c));
  s[len] = '\0';
  return s;
}

static long ReadLong(void) {
  char* token = ReadToken();
  long value;
  if (token == NULL) exit(2);
  value = strtol(token, NULL, 10);
  free(token);
  return value;
}

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "yuv: bad hex digit %c\n", c);
  exit(2);
}

// Parses `digits` hex digits of `hex` into `out`, which holds `count` values of
// `width` hex digits each, big-endian, in the order the text gives them.
static void ParseHexValues(const char* hex, size_t count, size_t width,
                           uint32_t* out) {
  size_t i, k;
  for (i = 0; i < count; ++i) {
    uint32_t value = 0;
    for (k = 0; k < width; ++k) {
      value = (value << 4) | (uint32_t)HexValue(hex[i * width + k]);
    }
    out[i] = value;
  }
}

// Returns a buffer of `size` bytes (at least one) read from a hex token.
static uint8_t* ReadBytes(size_t size) {
  char* token = ReadToken();
  uint8_t* buffer = (uint8_t*)calloc(size + 1, 1);
  size_t i;
  if (token == NULL || buffer == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    if (strlen(token) != 2 * size) {
      fprintf(stderr, "yuv: expected %lu hex digits, got %lu\n",
              (unsigned long)(2 * size), (unsigned long)strlen(token));
      exit(2);
    }
    for (i = 0; i < size; ++i) {
      buffer[i] = (uint8_t)(HexValue(token[2 * i]) * 16 +
                            HexValue(token[2 * i + 1]));
    }
  }
  free(token);
  return buffer;
}

static uint32_t* ReadWords32(size_t count) {
  char* token = ReadToken();
  uint32_t* words = (uint32_t*)calloc(count + 1, sizeof(uint32_t));
  if (token == NULL || words == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    if (strlen(token) != 8 * count) exit(2);
    ParseHexValues(token, count, 8, words);
  }
  free(token);
  return words;
}

static uint16_t* ReadWords16(size_t count) {
  char* token = ReadToken();
  uint16_t* words = (uint16_t*)calloc(count + 1, sizeof(uint16_t));
  uint32_t* wide = (uint32_t*)calloc(count + 1, sizeof(uint32_t));
  size_t i;
  if (token == NULL || words == NULL || wide == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    if (strlen(token) != 4 * count) exit(2);
    ParseHexValues(token, count, 4, wide);
    for (i = 0; i < count; ++i) words[i] = (uint16_t)wide[i];
  }
  free(token);
  free(wide);
  return words;
}

static uint8_t* Filled(size_t size) {
  uint8_t* buffer = (uint8_t*)malloc(size + 1);
  if (buffer == NULL) exit(2);
  memset(buffer, SENTINEL, size + 1);
  return buffer;
}

static WebPSamplerRowFunc SamplerOf(long mode) {
  if (mode < 0 || mode >= MODE_YUV) {
    fprintf(stderr, "yuv: mode %ld has no sampler\n", mode);
    exit(2);
  }
  return WebPSamplers[mode];
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "yuv < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      WebPInitSamplers();
      WebPInitConvertARGBToYUV();
    } else if (strcmp(command, "row") == 0) {
      const long mode = ReadLong();
      const long len = ReadLong();
      const size_t n = (size_t)len;
      uint8_t* y = ReadBytes(n);
      uint8_t* u = ReadBytes((n + 1) / 2);
      uint8_t* v = ReadBytes((n + 1) / 2);
      uint8_t* dst = Filled(4 * n + 8);
      SamplerOf(mode)(y, u, v, dst, (int)len);
      ProbePrintHex("dst", dst, 4 * n + 8);
      free(y); free(u); free(v); free(dst);
    } else if (strcmp(command, "plane") == 0) {
      const long mode = ReadLong();
      const long w = ReadLong();
      const long h = ReadLong();
      const long ystride = ReadLong();
      const long uvstride = ReadLong();
      const long dststride = ReadLong();
      const size_t uv_rows = (size_t)((h + 1) / 2);
      uint8_t* y = ReadBytes((size_t)(ystride * h));
      uint8_t* u = ReadBytes((size_t)(uvstride * uv_rows));
      uint8_t* v = ReadBytes((size_t)(uvstride * uv_rows));
      uint8_t* dst = Filled((size_t)(dststride * h));
      WebPSamplerProcessPlane(y, (int)ystride, u, v, (int)uvstride, dst,
                              (int)dststride, (int)w, (int)h, SamplerOf(mode));
      ProbePrintHex("dst", dst, (size_t)(dststride * h));
      free(y); free(u); free(v); free(dst);
    } else if (strcmp(command, "argb_y") == 0) {
      const long w = ReadLong();
      const size_t n = (size_t)w;
      uint32_t* argb = ReadWords32(n);
      uint8_t* y = Filled(n);
      WebPConvertARGBToY(argb, y, (int)w);
      ProbePrintHex("y", y, n);
      free(argb); free(y);
    } else if (strcmp(command, "argb_uv") == 0) {
      const long w = ReadLong();
      const long do_store = ReadLong();
      const size_t n = (size_t)w;
      const size_t uv = (n + 1) / 2;
      uint32_t* argb = ReadWords32(n);
      uint8_t* u = ReadBytes(uv);
      uint8_t* v = ReadBytes(uv);
      WebPConvertARGBToUV(argb, u, v, (int)w, (int)do_store);
      ProbePrintHex("u", u, uv);
      ProbePrintHex("v", v, uv);
      free(argb); free(u); free(v);
    } else if (strcmp(command, "rgb_y") == 0 ||
               strcmp(command, "bgr_y") == 0) {
      const long w = ReadLong();
      const size_t n = (size_t)w;
      uint8_t* rgb = ReadBytes(3 * n);
      uint8_t* y = Filled(n);
      if (command[0] == 'r') {
        WebPConvertRGB24ToY(rgb, y, (int)w);
      } else {
        WebPConvertBGR24ToY(rgb, y, (int)w);
      }
      ProbePrintHex("y", y, n);
      free(rgb); free(y);
    } else if (strcmp(command, "rgba_uv") == 0) {
      const long w = ReadLong();
      const size_t n = (size_t)w;
      const size_t uv = n;
      uint16_t* rgb = ReadWords16(4 * n);
      uint8_t* u = Filled(uv);
      uint8_t* v = Filled(uv);
      WebPConvertRGBA32ToUV(rgb, u, v, (int)w);
      ProbePrintHex("u", u, uv);
      ProbePrintHex("v", v, uv);
      free(rgb); free(u); free(v);
    } else if (strcmp(command, "yuv") == 0) {
      const int y = (int)ReadLong();
      const int u = (int)ReadLong();
      const int v = (int)ReadLong();
      uint8_t rgb[4], bgr[4], rgb565[2], rgba4444[2], argb[4], bgra[4], rgba[4];
      memset(rgb, SENTINEL, sizeof(rgb));
      memset(bgr, SENTINEL, sizeof(bgr));
      memset(rgb565, SENTINEL, sizeof(rgb565));
      memset(rgba4444, SENTINEL, sizeof(rgba4444));
      memset(argb, SENTINEL, sizeof(argb));
      memset(bgra, SENTINEL, sizeof(bgra));
      memset(rgba, SENTINEL, sizeof(rgba));
      VP8YuvToRgb(y, u, v, rgb);
      VP8YuvToBgr(y, u, v, bgr);
      VP8YuvToRgb565(y, u, v, rgb565);
      VP8YuvToRgba4444(y, u, v, rgba4444);
      VP8YuvToArgb((uint8_t)y, (uint8_t)u, (uint8_t)v, argb);
      VP8YuvToBgra((uint8_t)y, (uint8_t)u, (uint8_t)v, bgra);
      VP8YuvToRgba((uint8_t)y, (uint8_t)u, (uint8_t)v, rgba);
      ProbePrintHex("rgb", rgb, 3);
      ProbePrintHex("bgr", bgr, 3);
      ProbePrintHex("rgb565", rgb565, 2);
      ProbePrintHex("rgba4444", rgba4444, 2);
      ProbePrintHex("argb", argb, 4);
      ProbePrintHex("bgra", bgra, 4);
      ProbePrintHex("rgba", rgba, 4);
      printf("r=%d\n", VP8YUVToR(y, v));
      printf("g=%d\n", VP8YUVToG(y, u, v));
      printf("b=%d\n", VP8YUVToB(y, u));
    } else if (strcmp(command, "rgb_yuv") == 0) {
      const int r = (int)ReadLong();
      const int g = (int)ReadLong();
      const int b = (int)ReadLong();
      const int rounding = (int)ReadLong();
      printf("y=%d\n", VP8RGBToY(r, g, b, rounding));
      printf("u=%d\n", VP8RGBToU(r, g, b, rounding));
      printf("v=%d\n", VP8RGBToV(r, g, b, rounding));
    } else {
      fprintf(stderr, "yuv: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
