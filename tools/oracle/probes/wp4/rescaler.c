// WP4 probe: the rescaler of src/dsp/rescaler.c and src/utils/rescaler_utils.c
// (ADR 0002). It reads commands from standard input, one per line.
//
//   dims SRCW SRCH SW SH           WebPRescalerGetScaledDimensions with the given
//                                  scaled width and height; prints ret=N sw=W sh=H
//   rescale SRCW SRCH DSTW DSTH CHANNELS DSTSTRIDE SRCSTRIDE CHUNK SRCHEX
//                                  drives a rescaler the way the encoder and the
//                                  decoder do: WebPRescaleNeededLines, then
//                                  WebPRescalerImport and WebPRescalerExport, one
//                                  step at a time, and prints each step, then dst=HEX
//                                  of DSTSTRIDE*DSTH bytes filled with 0xaa at first.
//                                  SRCHEX holds SRCSTRIDE*SRCH bytes; "-" when empty.
//
// tests/wp4_rescaler.rs generates the commands and the expected output.

#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"
#include "src/utils/rescaler_utils.h"

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
  fprintf(stderr, "rescaler: bad hex digit %c\n", c);
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

static void Dims(void) {
  const long src_w = ReadLong();
  const long src_h = ReadLong();
  int sw = (int)ReadLong();
  int sh = (int)ReadLong();
  const int ret = WebPRescalerGetScaledDimensions((int)src_w, (int)src_h, &sw,
                                                  &sh);
  printf("ret=%d sw=%d sh=%d\n", ret, sw, sh);
}

static void Rescale(void) {
  const long src_w = ReadLong();
  const long src_h = ReadLong();
  const long dst_w = ReadLong();
  const long dst_h = ReadLong();
  const long channels = ReadLong();
  const long dst_stride = ReadLong();
  const long src_stride = ReadLong();
  const long chunk = ReadLong();
  uint8_t* src = ReadBytes((size_t)(src_stride * src_h));
  uint8_t* dst = (uint8_t*)malloc((size_t)(dst_stride * dst_h) + 1);
  rescaler_t* work = (rescaler_t*)calloc((size_t)(2 * dst_w * channels) + 1,
                                         sizeof(rescaler_t));
  WebPRescaler wrk;
  long src_row = 0;
  int steps = 0;
  if (dst == NULL || work == NULL) exit(2);
  memset(dst, SENTINEL, (size_t)(dst_stride * dst_h));
  printf("init=%d\n",
         WebPRescalerInit(&wrk, (int)src_w, (int)src_h, dst, (int)dst_w,
                          (int)dst_h, (int)dst_stride, (int)channels, work));
  while (!WebPRescalerInputDone(&wrk)) {
    const int needed = WebPRescaleNeededLines(&wrk, (int)chunk);
    long num = chunk;
    int imported, exported;
    if (src_h - src_row < num) num = src_h - src_row;
    imported = WebPRescalerImport(&wrk, (int)num, src + src_row * src_stride,
                                  (int)src_stride);
    exported = WebPRescalerExport(&wrk);
    printf("step needed=%d imported=%d exported=%d src_y=%d dst_y=%d y_accum=%d\n",
           needed, imported, exported, wrk.src_y, wrk.dst_y, wrk.y_accum);
    src_row += imported;
    if (++steps > 100000) break;
    if (imported == 0 && exported == 0) break;
  }
  ProbePrintHex("dst", dst, (size_t)(dst_stride * dst_h));
  free(src);
  free(dst);
  free(work);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "rescaler < commands.txt");
  while ((command = ReadToken()) != NULL) {
    if (strcmp(command, "dims") == 0) {
      Dims();
    } else if (strcmp(command, "rescale") == 0) {
      Rescale();
    } else {
      fprintf(stderr, "rescaler: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
