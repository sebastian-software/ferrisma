// WP4 probe: the SSIM and the squared-error functions of src/dsp/ssim.c and
// the statistics functions of src/dsp/dsp.h (ADR 0002). It reads commands from
// standard input, one per line. Byte planes are hex strings, two digits per
// byte. Doubles are printed as their 64-bit patterns in hex, so that the
// comparison is exact.
//
//   init                                   VP8SSIMDspInit
//   from_stats W XM YM XXM XYM YYM         VP8SSIMFromStats and VP8SSIMFromStatsClipped on
//                                          the statistics (w is the sum of the weights);
//                                          prints full=BITS clipped=BITS
//   get_clipped W H XO YO STRIDE1 STRIDE2 SRC1 SRC2
//                                          VP8SSIMGetClipped; prints value=BITS
//   get STRIDE1 STRIDE2 SRC1 SRC2          VP8SSIMGet over the 7 by 7 kernel; prints value=BITS
//   accumulate LEN SRC1 SRC2               VP8AccumulateSSE; prints sse=N
//
// tests/wp4_ssim.rs generates the commands and the expected output.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"

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
  fprintf(stderr, "ssim: bad hex digit %c\n", c);
  exit(2);
}

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

static void PrintDouble(const char* key, double value) {
  uint64_t bits;
  memcpy(&bits, &value, sizeof(bits));
  printf("%s=%016llx\n", key, (unsigned long long)bits);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "ssim < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      VP8SSIMDspInit();
    } else if (strcmp(command, "from_stats") == 0) {
      VP8DistoStats stats;
      stats.w = (uint32_t)ReadLong();
      stats.xm = (uint32_t)ReadLong();
      stats.ym = (uint32_t)ReadLong();
      stats.xxm = (uint32_t)ReadLong();
      stats.xym = (uint32_t)ReadLong();
      stats.yym = (uint32_t)ReadLong();
      PrintDouble("full", VP8SSIMFromStats(&stats));
      PrintDouble("clipped", VP8SSIMFromStatsClipped(&stats));
    } else if (strcmp(command, "get_clipped") == 0) {
      const long w = ReadLong();
      const long h = ReadLong();
      const long xo = ReadLong();
      const long yo = ReadLong();
      const long stride1 = ReadLong();
      const long stride2 = ReadLong();
      uint8_t* src1 = ReadBytes((size_t)(stride1 * h));
      uint8_t* src2 = ReadBytes((size_t)(stride2 * h));
      PrintDouble("value", VP8SSIMGetClipped(src1, (int)stride1, src2,
                                             (int)stride2, (int)xo, (int)yo,
                                             (int)w, (int)h));
      free(src1);
      free(src2);
    } else if (strcmp(command, "get") == 0) {
      const long stride1 = ReadLong();
      const long stride2 = ReadLong();
      uint8_t* src1 = ReadBytes((size_t)(stride1 * 7));
      uint8_t* src2 = ReadBytes((size_t)(stride2 * 7));
      PrintDouble("value", VP8SSIMGet(src1, (int)stride1, src2, (int)stride2));
      free(src1);
      free(src2);
    } else if (strcmp(command, "accumulate") == 0) {
      const long len = ReadLong();
      uint8_t* src1 = ReadBytes((size_t)len);
      uint8_t* src2 = ReadBytes((size_t)len);
      printf("sse=%u\n", (unsigned)VP8AccumulateSSE(src1, src2, (int)len));
      free(src1);
      free(src2);
    } else {
      fprintf(stderr, "ssim: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
