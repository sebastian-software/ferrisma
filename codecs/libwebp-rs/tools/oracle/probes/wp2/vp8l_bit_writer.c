// WP2 probe: drives two VP8LBitWriter objects, `0` and `1`, through
// src/utils/bit_writer_utils.c and the inline functions of
// src/utils/bit_writer_utils.h, reading one command per line from standard input:
//
//   init W N         VP8LBitWriterInit of writer W with N expected bytes
//   put W V N        VP8LPutBits of the value V with N bits (0 <= N <= 32)
//   written W        prints the bytes [0, cur) of writer W, the words flushed so far
//   num_bytes W      VP8LBitWriterNumBytes
//   finish W         VP8LBitWriterFinish; prints the NumBytes bytes it returns
//   clone S D        VP8LBitWriterClone of writer S into writer D
//   reset S D        VP8LBitWriterReset of writer D to the cursor of writer S
//   swap A B         VP8LBitWriterSwap
//   wipe W           VP8LBitWriterWipeOut
//
// After every command the probe prints `op=`, `result=` for init and clone, and
// the state of both writers: `bits`, `used`, `cur` (offset of the write position),
// `alloc` (the allocated size) and `error`. tests/wp2_vp8l_bit_writer.rs generates
// the commands and the expected output.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/bit_writer_utils.h"

static char line[1 << 16];
static VP8LBitWriter w[2];

static void PrintWriter(const char* name, const VP8LBitWriter* bw) {
  size_t alloc = bw->buf != NULL ? (size_t)(bw->end - bw->buf) : 0;
  size_t cur = bw->buf != NULL ? (size_t)(bw->cur - bw->buf) : 0;
  printf("%s_bits=%llu\n", name, (unsigned long long)bw->bits);
  printf("%s_used=%d\n", name, bw->used);
  printf("%s_cur=%zu\n", name, cur);
  printf("%s_alloc=%zu\n", name, alloc);
  printf("%s_error=%d\n", name, bw->error);
}

static void PrintState(void) {
  PrintWriter("w0", &w[0]);
  PrintWriter("w1", &w[1]);
}

static int Index(const char* text) {
  if (strcmp(text, "0") == 0) return 0;
  if (strcmp(text, "1") == 0) return 1;
  fprintf(stderr, "vp8l_bit_writer: bad writer %s\n", text);
  exit(2);
}

int main(int argc, char* argv[]) {
  char op[32];
  char first[8];
  char second[8];
  ProbeCheckArgs(argc, 0, 0, "vp8l_bit_writer < commands.txt");
  memset(w, 0, sizeof(w));
  while (fgets(line, sizeof(line), stdin) != NULL) {
    int result = 0;
    int has_result = 0;
    if (sscanf(line, "%31s", op) != 1) continue;
    if (strcmp(op, "init") == 0) {
      unsigned long n;
      if (sscanf(line, "%*s %7s %lu", first, &n) != 2) return 2;
      // VP8LBitWriterInit zeroes the writer without freeing its buffer, so the
      // previous buffer of the writer is released first.
      VP8LBitWriterWipeOut(&w[Index(first)]);
      result = VP8LBitWriterInit(&w[Index(first)], (size_t)n);
      has_result = 1;
    } else if (strcmp(op, "put") == 0) {
      unsigned int v;
      int n;
      if (sscanf(line, "%*s %7s %u %d", first, &v, &n) != 3) return 2;
      VP8LPutBits(&w[Index(first)], v, n);
    } else if (strcmp(op, "written") == 0) {
      VP8LBitWriter* bw;
      size_t cur;
      if (sscanf(line, "%*s %7s", first) != 1) return 2;
      bw = &w[Index(first)];
      cur = bw->buf != NULL ? (size_t)(bw->cur - bw->buf) : 0;
      printf("op=%s\n", op);
      ProbePrintHex("written", bw->buf, cur);
      PrintState();
      continue;
    } else if (strcmp(op, "num_bytes") == 0) {
      if (sscanf(line, "%*s %7s", first) != 1) return 2;
      result = (int)VP8LBitWriterNumBytes(&w[Index(first)]);
      has_result = 1;
      printf("op=%s\n", op);
      printf("result=%d\n", result);
      PrintState();
      continue;
    } else if (strcmp(op, "finish") == 0) {
      VP8LBitWriter* bw;
      uint8_t* out;
      size_t n;
      if (sscanf(line, "%*s %7s", first) != 1) return 2;
      bw = &w[Index(first)];
      out = VP8LBitWriterFinish(bw);
      n = VP8LBitWriterNumBytes(bw);
      printf("op=%s\n", op);
      ProbePrintHex("finished", out, n);
      PrintState();
      continue;
    } else if (strcmp(op, "clone") == 0) {
      if (sscanf(line, "%*s %7s %7s", first, second) != 2) return 2;
      result = VP8LBitWriterClone(&w[Index(first)], &w[Index(second)]);
      has_result = 1;
    } else if (strcmp(op, "reset") == 0) {
      if (sscanf(line, "%*s %7s %7s", first, second) != 2) return 2;
      VP8LBitWriterReset(&w[Index(first)], &w[Index(second)]);
    } else if (strcmp(op, "swap") == 0) {
      if (sscanf(line, "%*s %7s %7s", first, second) != 2) return 2;
      VP8LBitWriterSwap(&w[Index(first)], &w[Index(second)]);
    } else if (strcmp(op, "wipe") == 0) {
      if (sscanf(line, "%*s %7s", first) != 1) return 2;
      VP8LBitWriterWipeOut(&w[Index(first)]);
    } else {
      fprintf(stderr, "vp8l_bit_writer: unknown command %s\n", op);
      return 2;
    }
    printf("op=%s\n", op);
    if (has_result) printf("result=%d\n", result);
    PrintState();
  }
  VP8LBitWriterWipeOut(&w[0]);
  VP8LBitWriterWipeOut(&w[1]);
  return 0;
}
