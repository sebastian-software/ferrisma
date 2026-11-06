// WP4 probe: the alpha processing of src/dsp/alpha_processing.c (ADR 0002),
// the premultiplication, the dispatch and extraction of alpha planes, the
// channel packing and the alpha tests. It reads commands from standard input,
// one per line, and prints what the library computes. Destinations start filled
// with 0xaa, and byte and 32-bit word arrays are hex strings ("-" when empty):
// two digits per byte, eight per word.
//
//   init
//   mult_argb_row W INV WORDS              WebPMultARGBRow; prints ptr=WORDS
//   mult_row W INV ALPHA PTR               WebPMultRow; prints ptr=HEX
//   mult_argb_rows W STRIDE NROWS INV WORDS
//                                          WebPMultARGBRows with STRIDE in words; prints ptr=WORDS
//   mult_rows W STRIDE ASTRIDE NROWS INV PTR ALPHA
//                                          WebPMultRows; prints ptr=HEX
//   apply W H STRIDE ALPHAFIRST RGBA       WebPApplyAlphaMultiply; prints rgba=HEX
//   apply4444 W H STRIDE RGBA4444          WebPApplyAlphaMultiply4444; prints rgba=HEX
//   dispatch W H ASTRIDE DSTSTRIDE ALPHA   WebPDispatchAlpha; prints ret=, dst=HEX
//   dispatch_green W H ASTRIDE DSTSTRIDE ALPHA
//                                          WebPDispatchAlphaToGreen; prints dst=WORDS
//   extract W H ASTRIDE ARGBSTRIDE ARGB    WebPExtractAlpha; prints ret=, alpha=HEX
//   extract_green SIZE WORDS               WebPExtractGreen; prints alpha=HEX
//   pack_rgb LEN STEP R G B                WebPPackRGB; prints out=WORDS
//   has8 LEN BYTES                         WebPHasAlpha8b; prints ret=
//   has32 LEN BYTES                        WebPHasAlpha32b; prints ret=
//   replace LEN COLOR WORDS                WebPAlphaReplace; prints src=WORDS
//
// tests/wp4_alpha.rs generates the commands and the expected output.

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"

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
  fprintf(stderr, "alpha: bad hex digit %c\n", c);
  exit(2);
}

// A buffer of `size` bytes (at least one) read from a hex token.
static uint8_t* ReadBytes(size_t size) {
  char* token = ReadToken();
  uint8_t* buffer = (uint8_t*)calloc(size + 1, 1);
  size_t i;
  if (token == NULL || buffer == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    if (strlen(token) != 2 * size) exit(2);
    for (i = 0; i < size; ++i) {
      buffer[i] = (uint8_t)(HexValue(token[2 * i]) * 16 +
                            HexValue(token[2 * i + 1]));
    }
  }
  free(token);
  return buffer;
}

// A buffer of `count` 32-bit words read from a token of eight digits per word.
static uint32_t* ReadWords(size_t count) {
  char* token = ReadToken();
  uint32_t* words = (uint32_t*)calloc(count + 1, sizeof(uint32_t));
  size_t i, k;
  if (token == NULL || words == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    if (strlen(token) != 8 * count) exit(2);
    for (i = 0; i < count; ++i) {
      uint32_t value = 0;
      for (k = 0; k < 8; ++k) {
        value = (value << 4) | (uint32_t)HexValue(token[8 * i + k]);
      }
      words[i] = value;
    }
  }
  free(token);
  return words;
}

static uint8_t* Filled(size_t size) {
  uint8_t* buffer = (uint8_t*)malloc(size + 1);
  if (buffer == NULL) exit(2);
  memset(buffer, SENTINEL, size + 1);
  return buffer;
}

static uint32_t* FilledWords(size_t count) {
  uint32_t* words = (uint32_t*)malloc((count + 1) * sizeof(uint32_t));
  size_t i;
  if (words == NULL) exit(2);
  for (i = 0; i <= count; ++i) words[i] = 0xaaaaaaaau;
  return words;
}

static void PrintWords(const char* key, const uint32_t* words, size_t count) {
  size_t i;
  printf("%s=", key);
  for (i = 0; i < count; ++i) printf("%08x", words[i]);
  printf("\n");
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "alpha_processing < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      WebPInitAlphaProcessing();
    } else if (strcmp(command, "mult_argb_row") == 0) {
      const long w = ReadLong();
      const long inv = ReadLong();
      const size_t n = (size_t)w;
      uint32_t* ptr = ReadWords(n);
      WebPMultARGBRow(ptr, (int)w, (int)inv);
      PrintWords("ptr", ptr, n);
      free(ptr);
    } else if (strcmp(command, "mult_row") == 0) {
      const long w = ReadLong();
      const long inv = ReadLong();
      const size_t n = (size_t)w;
      uint8_t* alpha = ReadBytes(n);
      uint8_t* ptr = ReadBytes(n);
      WebPMultRow(ptr, alpha, (int)w, (int)inv);
      ProbePrintHex("ptr", ptr, n);
      free(alpha);
      free(ptr);
    } else if (strcmp(command, "mult_argb_rows") == 0) {
      const long w = ReadLong();
      const long stride = ReadLong();
      const long nrows = ReadLong();
      const long inv = ReadLong();
      const size_t count = (size_t)(stride * nrows);
      uint32_t* ptr = ReadWords(count);
      WebPMultARGBRows((uint8_t*)ptr, (int)(stride * 4), (int)w, (int)nrows,
                       (int)inv);
      PrintWords("ptr", ptr, count);
      free(ptr);
    } else if (strcmp(command, "mult_rows") == 0) {
      const long w = ReadLong();
      const long stride = ReadLong();
      const long astride = ReadLong();
      const long nrows = ReadLong();
      const long inv = ReadLong();
      uint8_t* ptr = ReadBytes((size_t)(stride * nrows));
      uint8_t* alpha = ReadBytes((size_t)(astride * nrows));
      WebPMultRows(ptr, (int)stride, alpha, (int)astride, (int)w, (int)nrows,
                   (int)inv);
      ProbePrintHex("ptr", ptr, (size_t)(stride * nrows));
      free(ptr);
      free(alpha);
    } else if (strcmp(command, "apply") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long stride = ReadLong();
      const long alpha_first = ReadLong();
      uint8_t* rgba = ReadBytes((size_t)(stride * h));
      WebPApplyAlphaMultiply(rgba, (int)alpha_first, (int)w, (int)h,
                             (int)stride);
      ProbePrintHex("rgba", rgba, (size_t)(stride * h));
      free(rgba);
    } else if (strcmp(command, "apply4444") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long stride = ReadLong();
      uint8_t* rgba = ReadBytes((size_t)(stride * h));
      WebPApplyAlphaMultiply4444(rgba, (int)w, (int)h, (int)stride);
      ProbePrintHex("rgba", rgba, (size_t)(stride * h));
      free(rgba);
    } else if (strcmp(command, "dispatch") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long astride = ReadLong();
      const long dstride = ReadLong();
      uint8_t* alpha = ReadBytes((size_t)(astride * h));
      uint8_t* dst = Filled((size_t)(dstride * h));
      const int ret = WebPDispatchAlpha(alpha, (int)astride, (int)w, (int)h,
                                        dst, (int)dstride);
      printf("ret=%d\n", ret);
      ProbePrintHex("dst", dst, (size_t)(dstride * h));
      free(alpha);
      free(dst);
    } else if (strcmp(command, "dispatch_green") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long astride = ReadLong();
      const long dstride = ReadLong();
      uint8_t* alpha = ReadBytes((size_t)(astride * h));
      uint32_t* dst = FilledWords((size_t)(dstride * h));
      WebPDispatchAlphaToGreen(alpha, (int)astride, (int)w, (int)h, dst,
                               (int)dstride);
      PrintWords("dst", dst, (size_t)(dstride * h));
      free(alpha);
      free(dst);
    } else if (strcmp(command, "extract") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long astride = ReadLong();
      const long argb_stride = ReadLong();
      uint8_t* argb = ReadBytes((size_t)(argb_stride * h));
      uint8_t* alpha = Filled((size_t)(astride * h));
      const int ret = WebPExtractAlpha(argb, (int)argb_stride, (int)w, (int)h,
                                       alpha, (int)astride);
      printf("ret=%d\n", ret);
      ProbePrintHex("alpha", alpha, (size_t)(astride * h));
      free(argb);
      free(alpha);
    } else if (strcmp(command, "extract_green") == 0) {
      const long size = ReadLong();
      const size_t n = (size_t)size;
      uint32_t* argb = ReadWords(n);
      uint8_t* alpha = Filled(n);
      WebPExtractGreen(argb, alpha, (int)size);
      ProbePrintHex("alpha", alpha, n);
      free(argb);
      free(alpha);
    } else if (strcmp(command, "pack_rgb") == 0) {
      const long len = ReadLong();
      const long step = ReadLong();
      const size_t planes = (size_t)(step * len);
      uint8_t* r = ReadBytes(planes);
      uint8_t* g = ReadBytes(planes);
      uint8_t* b = ReadBytes(planes);
      uint32_t* out = FilledWords((size_t)len);
      WebPPackRGB(r, g, b, (int)len, (int)step, out);
      PrintWords("out", out, (size_t)len);
      free(r); free(g); free(b); free(out);
    } else if (strcmp(command, "has8") == 0) {
      const long len = ReadLong();
      uint8_t* src = ReadBytes((size_t)len);
      printf("ret=%d\n", WebPHasAlpha8b(src, (int)len));
      free(src);
    } else if (strcmp(command, "has32") == 0) {
      const long len = ReadLong();
      uint8_t* src = ReadBytes((size_t)(4 * len));
      printf("ret=%d\n", WebPHasAlpha32b(src, (int)len));
      free(src);
    } else if (strcmp(command, "replace") == 0) {
      const long len = ReadLong();
      const long color = ReadLong();
      uint32_t* src = ReadWords((size_t)len);
      WebPAlphaReplace(src, (int)len, (uint32_t)color);
      PrintWords("src", src, (size_t)len);
      free(src);
    } else {
      fprintf(stderr, "alpha: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
