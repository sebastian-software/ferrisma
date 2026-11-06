// WP2 probe: drives VP8BitWriter through src/utils/bit_writer_utils.c and the
// inline functions of src/utils/bit_writer_utils.h, reading one command per line
// from standard input:
//
//   init N           VP8BitWriterInit with N expected bytes
//   bit B P          VP8PutBit of the bit B (0 or 1) with probability P
//   uniform B        VP8PutBitUniform of the bit B
//   bits V N         VP8PutBits of the value V with N bits (0 < N < 32)
//   signed V N       VP8PutSignedBits of the value V with N bits
//   append HEX       VP8BitWriterAppend of the bytes HEX
//   finish           VP8BitWriterFinish; prints `size=`
//   bytes            the first VP8BitWriterSize bytes of VP8BitWriterBuf
//   wipe             VP8BitWriterWipeOut
//
// After every command the probe prints `op=`, `result=` for the commands that
// return a value (init, bit, uniform and append), and the state: `range`,
// `value`, `run`, `nb_bits`, `pos`, `max_pos` (the allocated size), `error`, and
// `pos_bits` (VP8BitWriterPos). The bytes are printed only by `bytes`.
// tests/wp2_bit_writer.rs generates the commands and the expected output.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/bit_writer_utils.h"

static char line[1 << 16];
static char hex[1 << 16];
static uint8_t* payload = NULL;
static VP8BitWriter bw;

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "vp8_bit_writer: bad hex digit %c\n", c);
  exit(2);
}

static size_t DecodeHex(const char* text) {
  size_t i;
  size_t n = strlen(text) / 2;
  free(payload);
  payload = (uint8_t*)malloc(n > 0 ? n : 1);
  if (payload == NULL) exit(2);
  for (i = 0; i < n; ++i) {
    payload[i] = (uint8_t)(HexValue(text[2 * i]) * 16 + HexValue(text[2 * i + 1]));
  }
  return n;
}

static void PrintState(void) {
  printf("range=%d\n", bw.range);
  printf("value=%d\n", bw.value);
  printf("run=%d\n", bw.run);
  printf("nb_bits=%d\n", bw.nb_bits);
  printf("pos=%zu\n", bw.pos);
  printf("max_pos=%zu\n", bw.max_pos);
  printf("error=%d\n", bw.error);
  printf("pos_bits=%llu\n", (unsigned long long)VP8BitWriterPos(&bw));
}

int main(int argc, char* argv[]) {
  char op[32];
  ProbeCheckArgs(argc, 0, 0, "vp8_bit_writer < commands.txt");
  memset(&bw, 0, sizeof(bw));
  while (fgets(line, sizeof(line), stdin) != NULL) {
    int a = 0;
    int b = 0;
    int result = 0;
    int has_result = 0;
    hex[0] = '\0';
    if (sscanf(line, "%31s", op) != 1) continue;
    if (strcmp(op, "init") == 0) {
      unsigned long n;
      if (sscanf(line, "%*s %lu", &n) != 1) return 2;
      result = VP8BitWriterInit(&bw, (size_t)n);
      has_result = 1;
    } else if (strcmp(op, "bit") == 0) {
      if (sscanf(line, "%*s %d %d", &a, &b) != 2) return 2;
      result = VP8PutBit(&bw, a, b);
      has_result = 1;
    } else if (strcmp(op, "uniform") == 0) {
      if (sscanf(line, "%*s %d", &a) != 1) return 2;
      result = VP8PutBitUniform(&bw, a);
      has_result = 1;
    } else if (strcmp(op, "bits") == 0) {
      unsigned int v;
      if (sscanf(line, "%*s %u %d", &v, &a) != 2) return 2;
      VP8PutBits(&bw, v, a);
    } else if (strcmp(op, "signed") == 0) {
      if (sscanf(line, "%*s %d %d", &a, &b) != 2) return 2;
      VP8PutSignedBits(&bw, a, b);
    } else if (strcmp(op, "append") == 0) {
      size_t n;
      sscanf(line, "%*s %65535s", hex);
      n = DecodeHex(hex);
      result = VP8BitWriterAppend(&bw, payload, n);
      has_result = 1;
    } else if (strcmp(op, "finish") == 0) {
      VP8BitWriterFinish(&bw);
      printf("op=%s\n", op);
      printf("size=%zu\n", VP8BitWriterSize(&bw));
      PrintState();
      continue;
    } else if (strcmp(op, "bytes") == 0) {
      printf("op=%s\n", op);
      ProbePrintHex("bytes", VP8BitWriterBuf(&bw), VP8BitWriterSize(&bw));
      PrintState();
      continue;
    } else if (strcmp(op, "wipe") == 0) {
      VP8BitWriterWipeOut(&bw);
    } else {
      fprintf(stderr, "vp8_bit_writer: unknown command %s\n", op);
      return 2;
    }
    printf("op=%s\n", op);
    if (has_result) printf("result=%d\n", result);
    PrintState();
  }
  free(payload);
  return 0;
}
