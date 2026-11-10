// WP4 probe: the fancy upsamplers WebPUpsamplers[mode], the converter that
// WebPGetLinePairConverter returns, and the YUV 4:4:4 converters
// WebPYUV444Converters[mode] of src/dsp/upsampling.c (ADR 0002). It reads
// commands from standard input, one per line, and prints what the library
// writes into its destinations, which are pre-filled with 0xaa.
//
// Byte arrays are hex strings with two digits per byte, "-" for an empty or
// absent array.
//
//   init                                   WebPInitUpsamplers, WebPInitYUV444Converters,
//                                          WebPInitSamplers
//   upsample MODE LEN BOTTOM TOPY BOTTOMY TOPU TOPV CURU CURV
//                                          WebPUpsamplers[MODE]; BOTTOM is 0 or 1; the
//                                          destinations hold 4*LEN+8 bytes; prints top=HEX
//                                          and bottom=HEX (bottom only when BOTTOM is 1)
//   getter ALPHA_IS_LAST LEN BOTTOM TOPY BOTTOMY TOPU TOPV CURU CURV
//                                          WebPGetLinePairConverter(ALPHA_IS_LAST), with the
//                                          same output as upsample
//   yuv444 MODE LEN YHEX UHEX VHEX         WebPYUV444Converters[MODE]; prints dst=HEX
//
// tests/wp4_upsampling.rs generates the commands and the expected output.

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"
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
  fprintf(stderr, "upsampling: bad hex digit %c\n", c);
  exit(2);
}

// Returns a buffer of `size` bytes (at least one) read from a hex token.
static uint8_t* ReadBytes(size_t size) {
  char* token = ReadToken();
  uint8_t* buffer = (uint8_t*)calloc(size + 1, 1);
  size_t i;
  if (token == NULL || buffer == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    if (strlen(token) != 2 * size) {
      fprintf(stderr, "upsampling: expected %lu hex digits, got %lu\n",
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

static uint8_t* Filled(size_t size) {
  uint8_t* buffer = (uint8_t*)malloc(size + 1);
  if (buffer == NULL) exit(2);
  memset(buffer, SENTINEL, size + 1);
  return buffer;
}

// Runs one upsampler with the arguments read from standard input.
static void RunUpsampler(WebPUpsampleLinePairFunc func, long len, int bottom) {
  const size_t n = (size_t)len;
  const size_t uv = (n + 1) / 2;
  uint8_t* top_y = ReadBytes(n);
  uint8_t* bottom_y = ReadBytes(n);
  uint8_t* top_u = ReadBytes(uv);
  uint8_t* top_v = ReadBytes(uv);
  uint8_t* cur_u = ReadBytes(uv);
  uint8_t* cur_v = ReadBytes(uv);
  uint8_t* top_dst = Filled(4 * n + 8);
  uint8_t* bottom_dst = Filled(4 * n + 8);
  func(top_y, bottom ? bottom_y : NULL, top_u, top_v, cur_u, cur_v, top_dst,
       bottom ? bottom_dst : NULL, (int)len);
  ProbePrintHex("top", top_dst, 4 * n + 8);
  if (bottom) ProbePrintHex("bottom", bottom_dst, 4 * n + 8);
  free(top_y); free(bottom_y); free(top_u); free(top_v); free(cur_u);
  free(cur_v); free(top_dst); free(bottom_dst);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "upsampling < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      WebPInitUpsamplers();
      WebPInitYUV444Converters();
      WebPInitSamplers();
    } else if (strcmp(command, "upsample") == 0) {
      const long mode = ReadLong();
      const long len = ReadLong();
      const int bottom = (int)ReadLong();
      if (mode < 0 || mode >= MODE_YUV) return 2;
      RunUpsampler(WebPUpsamplers[mode], len, bottom);
    } else if (strcmp(command, "getter") == 0) {
      const int alpha_is_last = (int)ReadLong();
      const long len = ReadLong();
      const int bottom = (int)ReadLong();
      RunUpsampler(WebPGetLinePairConverter(alpha_is_last), len, bottom);
    } else if (strcmp(command, "yuv444") == 0) {
      const long mode = ReadLong();
      const long len = ReadLong();
      const size_t n = (size_t)len;
      uint8_t* y = ReadBytes(n);
      uint8_t* u = ReadBytes(n);
      uint8_t* v = ReadBytes(n);
      uint8_t* dst = Filled(4 * n + 8);
      if (mode < 0 || mode >= MODE_YUV) return 2;
      WebPYUV444Converters[mode](y, u, v, dst, (int)len);
      ProbePrintHex("dst", dst, 4 * n + 8);
      free(y); free(u); free(v); free(dst);
    } else {
      fprintf(stderr, "upsampling: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
