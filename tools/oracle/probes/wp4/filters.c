// WP4 probe: the spatial prediction filters of src/dsp/filters.c, their
// unfilters, and the filter estimate of src/utils/filters_utils.c (ADR 0002).
// It reads commands from standard input, one per line, and prints what the
// library computes. Byte arrays are hex strings, two digits per byte, "-" when
// empty. Destinations start filled with 0xaa, except where a command gives
// their contents.
//
//   init                                       VP8FiltersInit
//   forward FILTER W H STRIDE IN               WebPFilters[FILTER]; prints out=HEX (STRIDE*H bytes)
//   unfilter FILTER W HASPREV ALIAS PREV IN OUT
//                                              WebPUnfilters[FILTER] over W bytes with
//                                              ALIAS 0: distinct buffers; 1: in == out (in-place);
//                                              2: prev == out (the previous row is the output);
//                                              PREV is "-" when HASPREV is 0; prints out=HEX
//   estimate W H STRIDE DATA                   WebPEstimateBestFilter; prints filter=N
//
// tests/wp4_filters.rs generates the commands and the expected output.

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"
#include "src/utils/filters_utils.h"

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
  fprintf(stderr, "filters: bad hex digit %c\n", c);
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

static uint8_t* Filled(size_t size) {
  uint8_t* buffer = (uint8_t*)malloc(size + 1);
  if (buffer == NULL) exit(2);
  memset(buffer, SENTINEL, size + 1);
  return buffer;
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "filters < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      VP8FiltersInit();
    } else if (strcmp(command, "forward") == 0) {
      const long filter = ReadLong();
      const long w = ReadLong();
      const long h = ReadLong();
      const long stride = ReadLong();
      uint8_t* in = ReadBytes((size_t)(stride * h));
      uint8_t* out = Filled((size_t)(stride * h));
      if (filter <= 0 || filter >= WEBP_FILTER_LAST) return 2;
      WebPFilters[filter]((const uint8_t*)in, (int)w, (int)h, (int)stride, out);
      ProbePrintHex("out", out, (size_t)(stride * h));
      free(in);
      free(out);
    } else if (strcmp(command, "unfilter") == 0) {
      const long filter = ReadLong();
      const long w = ReadLong();
      const long hasprev = ReadLong();
      const long alias = ReadLong();
      const size_t n = (size_t)w;
      uint8_t* prev = ReadBytes(n);
      uint8_t* in = ReadBytes(n);
      uint8_t* out = ReadBytes(n);
      uint8_t* cur_prev = hasprev ? prev : NULL;
      if (filter < 0 || filter >= WEBP_FILTER_LAST) return 2;
      if (alias == 1) {
        // In place: the input and the output are the same buffer.
        WebPUnfilters[filter](cur_prev, out, out, (int)w);
      } else if (alias == 2) {
        // The previous row is the output: prev and out alias.
        if (!hasprev) return 2;
        WebPUnfilters[filter](out, in, out, (int)w);
      } else {
        WebPUnfilters[filter](cur_prev, in, out, (int)w);
      }
      ProbePrintHex("out", out, n);
      free(prev);
      free(in);
      free(out);
    } else if (strcmp(command, "estimate") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long stride = ReadLong();
      uint8_t* data = ReadBytes((size_t)(stride * h));
      printf("filter=%d\n", (int)WebPEstimateBestFilter(data, (int)w, (int)h,
                                                         (int)stride));
      free(data);
    } else {
      fprintf(stderr, "filters: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
