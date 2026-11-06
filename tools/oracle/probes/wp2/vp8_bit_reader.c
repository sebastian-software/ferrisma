// WP2 probe: drives VP8BitReader through src/utils/bit_reader_utils.c and
// src/utils/bit_reader_inl_utils.h, reading one command per line from standard
// input:
//
//   init [HEX]       VP8InitBitReader over the bytes HEX (no argument: empty)
//   bit P            VP8GetBit with probability P
//   bit_alt P        VP8GetBitAlt with probability P
//   value N          VP8GetValue with N bits
//   get              VP8Get, one bit at probability 128
//   signed_value N   VP8GetSignedValue with N bits
//   signed V         VP8GetSigned with the value V
//
// After every command the probe prints `op=` with the command name, `result=`
// with the returned value (not for init), and the reader state: `value` (the
// register, as an unsigned integer), `range`, `bits`, `buf` and `buf_max` as byte
// offsets from the start of the buffer, and `eof`. tests/wp2_bit_reader.rs
// generates the commands and the expected output.
//
// Labels are removed by the macros of the reference configuration (BITTRACE 0),
// so the probe passes 0 for them.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/bit_reader_inl_utils.h"
#include "src/utils/bit_reader_utils.h"

static char line[1 << 16];
static char hex[1 << 16];
static uint8_t* data = NULL;
static size_t data_size = 0;
static VP8BitReader br;

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "vp8_bit_reader: bad hex digit %c\n", c);
  exit(2);
}

// Replaces the buffer with a fresh copy of the bytes in `text`. The copy is
// never NULL, so an empty input still reaches the reader with a valid pointer.
static void SetData(const char* text) {
  size_t i;
  size_t n = strlen(text) / 2;
  free(data);
  data = (uint8_t*)malloc(n > 0 ? n : 1);
  if (data == NULL) exit(2);
  for (i = 0; i < n; ++i) {
    data[i] = (uint8_t)(HexValue(text[2 * i]) * 16 + HexValue(text[2 * i + 1]));
  }
  data_size = n;
}

static void PrintState(void) {
  printf("value=%llu\n", (unsigned long long)br.value);
  printf("range=%u\n", (unsigned)br.range);
  printf("bits=%d\n", br.bits);
  printf("buf=%zu\n", (size_t)(br.buf - data));
  printf("buf_max=%zu\n", (size_t)(br.buf_max - data));
  printf("eof=%d\n", br.eof);
}

int main(int argc, char* argv[]) {
  char op[32];
  ProbeCheckArgs(argc, 0, 0, "vp8_bit_reader < commands.txt");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    int n = 0;
    unsigned int result = 0;
    int has_result = 1;
    hex[0] = '\0';
    if (sscanf(line, "%31s", op) != 1) continue;
    if (strcmp(op, "init") == 0) {
      sscanf(line, "%*s %65535s", hex);
      SetData(hex);
      VP8InitBitReader(&br, data, data_size);
      has_result = 0;
    } else if (strcmp(op, "bit") == 0 || strcmp(op, "bit_alt") == 0) {
      if (sscanf(line, "%*s %d", &n) != 1) return 2;
      if (strcmp(op, "bit") == 0) {
        result = (unsigned int)VP8GetBit(&br, n, 0);
      } else {
        result = (unsigned int)VP8GetBitAlt(&br, n, 0);
      }
    } else if (strcmp(op, "value") == 0) {
      if (sscanf(line, "%*s %d", &n) != 1) return 2;
      result = VP8GetValue(&br, n, 0);
    } else if (strcmp(op, "get") == 0) {
      result = VP8Get(&br, 0);
    } else if (strcmp(op, "signed_value") == 0) {
      if (sscanf(line, "%*s %d", &n) != 1) return 2;
      result = (unsigned int)VP8GetSignedValue(&br, n, 0);
    } else if (strcmp(op, "signed") == 0) {
      if (sscanf(line, "%*s %d", &n) != 1) return 2;
      result = (unsigned int)VP8GetSigned(&br, n, 0);
    } else {
      fprintf(stderr, "vp8_bit_reader: unknown command %s\n", op);
      return 2;
    }
    printf("op=%s\n", op);
    if (has_result) printf("result=%u\n", result);
    PrintState();
  }
  free(data);
  return 0;
}
