// WP2 probe: drives VP8BitReader through src/utils/bit_reader_utils.c and
// src/utils/bit_reader_inl_utils.h, reading one command per line from standard
// input:
//
//   reset               zeroes the reader and releases its buffer (buf == NULL)
//   remap_null OFFSET   VP8RemapBitReader with OFFSET on a reader without a
//                       buffer, which leaves the reader unchanged
//   init [HEX]          VP8InitBitReader over the bytes HEX (no argument: empty)
//   move NEW            moves the input bytes to index NEW of the buffer and calls
//                       VP8RemapBitReader with the index delta
//   bit P               VP8GetBit with probability P
//   bit_alt P           VP8GetBitAlt with probability P
//   value N             VP8GetValue with N bits
//   get                 VP8Get, one bit at probability 128
//   signed_value N      VP8GetSignedValue with N bits
//   signed V            VP8GetSigned with the value V
//
// The buffer holds the input at index `base` and `PAD` more bytes, so that `move`
// can place the input at any index up to `PAD`. Positions are reported as indexes
// into the buffer, which is the representation of the Rust reader.
//
// After every command the probe prints `op=` with the command name, `result=`
// with the returned value (for the commands that return one), and the state:
// `value`, `range`, `bits`, `attached` (1 when buf != NULL), `buf`, `buf_end`,
// `buf_max` (indexes; 0 when not attached) and `eof`.
// tests/wp2_bit_reader.rs generates the commands and the expected output.
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

// Bytes after the input, so that `move` can shift the input by up to PAD.
#define PAD 64

static char line[1 << 16];
static char hex[1 << 16];
static uint8_t* mem = NULL;   // buffer: the input at index base, then PAD bytes
static size_t base = 0;       // index of the input in the buffer
static size_t input_size = 0; // length of the input
static VP8BitReader br;

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "vp8_bit_reader: bad hex digit %c\n", c);
  exit(2);
}

// Replaces the buffer with a fresh one holding the bytes in `text` at index 0.
// The buffer is never NULL, so an empty input still reaches the reader with a
// valid pointer.
static void SetInput(const char* text) {
  size_t i;
  size_t n = strlen(text) / 2;
  free(mem);
  mem = (uint8_t*)calloc(n + PAD, 1);
  if (mem == NULL) exit(2);
  for (i = 0; i < n; ++i) {
    mem[i] = (uint8_t)(HexValue(text[2 * i]) * 16 + HexValue(text[2 * i + 1]));
  }
  base = 0;
  input_size = n;
}

static void PrintState(void) {
  const int attached = br.buf != NULL;
  printf("value=%llu\n", (unsigned long long)br.value);
  printf("range=%u\n", (unsigned)br.range);
  printf("bits=%d\n", br.bits);
  printf("attached=%d\n", attached);
  printf("buf=%zu\n", attached ? (size_t)(br.buf - mem) : 0);
  printf("buf_end=%zu\n", attached ? (size_t)(br.buf_end - mem) : 0);
  printf("buf_max=%zu\n", attached ? (size_t)(br.buf_max - mem) : 0);
  printf("eof=%d\n", br.eof);
}

int main(int argc, char* argv[]) {
  char op[32];
  ProbeCheckArgs(argc, 0, 0, "vp8_bit_reader < commands.txt");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    int n = 0;
    long offset = 0;
    unsigned int result = 0;
    int has_result = 1;
    hex[0] = '\0';
    if (sscanf(line, "%31s", op) != 1) continue;
    if (strcmp(op, "reset") == 0) {
      free(mem);
      mem = NULL;
      base = 0;
      input_size = 0;
      memset(&br, 0, sizeof(br));
      has_result = 0;
    } else if (strcmp(op, "remap_null") == 0) {
      if (br.buf != NULL) return 2;
      if (sscanf(line, "%*s %ld", &offset) != 1) return 2;
      VP8RemapBitReader(&br, (ptrdiff_t)offset);
      has_result = 0;
    } else if (strcmp(op, "init") == 0) {
      sscanf(line, "%*s %65535s", hex);
      SetInput(hex);
      VP8InitBitReader(&br, mem, input_size);
      has_result = 0;
    } else if (strcmp(op, "move") == 0) {
      size_t target;
      if (br.buf == NULL) return 2;
      if (sscanf(line, "%*s %zu", &target) != 1 || target > PAD) return 2;
      memmove(mem + target, mem + base, input_size);
      VP8RemapBitReader(&br, (ptrdiff_t)target - (ptrdiff_t)base);
      base = target;
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
  free(mem);
  return 0;
}
