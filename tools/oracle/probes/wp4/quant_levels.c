// WP4 probe: QuantizeLevels of src/utils/quant_levels_utils.c and
// WebPDequantizeLevels of src/utils/quant_levels_dec_utils.c (ADR 0002). It reads
// commands from standard input, one per line; byte arrays are hex strings, two
// digits per byte, "-" when empty, and "null" for a NULL plane.
//
//   quantize W H LEVELS SSEFLAG DATA
//       QuantizeLevels; SSEFLAG 1 passes &sse, 0 passes NULL; prints
//       ret=N, data=HEX (W*H bytes), and sse=N when SSEFLAG is 1
//   dequant W H STRIDE STRENGTH DATA
//       WebPDequantizeLevels; prints ret=N and data=HEX (STRIDE*H bytes)
//
// tests/wp4_quant_levels.rs generates the commands and the expected output.

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/quant_levels_dec_utils.h"
#include "src/utils/quant_levels_utils.h"

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
  fprintf(stderr, "quant_levels: bad hex digit %c\n", c);
  exit(2);
}

// Returns NULL for the token "null", otherwise a buffer of `size` bytes.
static uint8_t* ReadPlane(size_t size, int* is_null) {
  char* token = ReadToken();
  uint8_t* buffer;
  size_t i;
  if (token == NULL) exit(2);
  if (strcmp(token, "null") == 0) {
    free(token);
    *is_null = 1;
    return NULL;
  }
  *is_null = 0;
  buffer = (uint8_t*)calloc(size + 1, 1);
  if (buffer == NULL) exit(2);
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

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "quant_levels < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "quantize") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long levels = ReadLong();
      const long want_sse = ReadLong();
      int is_null = 0;
      const size_t size = (w > 0 && h > 0) ? (size_t)(w * h) : 0;
      uint8_t* data = ReadPlane(size, &is_null);
      uint64_t sse = 0;
      int ret = QuantizeLevels(data, (int)w, (int)h, (int)levels,
                               want_sse ? &sse : NULL);
      printf("ret=%d\n", ret);
      if (!is_null) ProbePrintHex("data", data, size);
      if (want_sse) printf("sse=%llu\n", (unsigned long long)sse);
      free(data);
    } else if (strcmp(command, "dequant") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long stride = ReadLong();
      const long strength = ReadLong();
      int is_null = 0;
      const size_t plane = (w > 0 && h > 0 && stride > 0) ? (size_t)(stride * h) : 0;
      uint8_t* data = ReadPlane(plane, &is_null);
      const int ret = WebPDequantizeLevels(data, (int)w, (int)h, (int)stride,
                                           (int)strength);
      printf("ret=%d\n", ret);
      if (!is_null) ProbePrintHex("data", data, plane);
      free(data);
    } else {
      fprintf(stderr, "quant_levels: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
