// WP2 probe: drives VP8LBitReader through src/utils/bit_reader_utils.c and the
// inline functions of src/utils/bit_reader_utils.h, reading one command per line
// from standard input:
//
//   init [HEX]       VP8LInitBitReader over the bytes HEX (no argument: empty)
//   set [HEX]        VP8LBitReaderSetBuffer over the bytes HEX; keeps the
//                    position, the window and the end-of-stream state
//   read N           VP8LReadBits with N bits
//   prefetch         VP8LPrefetchBits
//   set_bit_pos V    VP8LSetBitPos to V
//   fill             VP8LFillBitWindow
//   do_fill          VP8LDoFillBitWindow (the caller guarantees bit_pos >= 32)
//   eos              VP8LIsEndOfStream
//
// After every command the probe prints `op=`, `result=` (not for init, set,
// set_bit_pos, fill and do_fill), and the state: `val` (the prefetched window as
// an unsigned integer), `pos`, `len`, `bit_pos` and `eos`.
// tests/wp2_vp8l_bit_reader.rs generates the commands and the expected output.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/bit_reader_utils.h"

static char line[1 << 16];
static char hex[1 << 16];
static uint8_t* data = NULL;
static size_t data_size = 0;
static VP8LBitReader br;

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "vp8l_bit_reader: bad hex digit %c\n", c);
  exit(2);
}

// Replaces the buffer with a fresh copy of the bytes in `text`; never NULL.
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
  printf("val=%llu\n", (unsigned long long)br.val);
  printf("pos=%zu\n", br.pos);
  printf("len=%zu\n", br.len);
  printf("bit_pos=%d\n", br.bit_pos);
  printf("eos=%d\n", br.eos);
}

int main(int argc, char* argv[]) {
  char op[32];
  ProbeCheckArgs(argc, 0, 0, "vp8l_bit_reader < commands.txt");
  while (fgets(line, sizeof(line), stdin) != NULL) {
    int n = 0;
    unsigned int result = 0;
    int has_result = 0;
    hex[0] = '\0';
    if (sscanf(line, "%31s", op) != 1) continue;
    if (strcmp(op, "init") == 0 || strcmp(op, "set") == 0) {
      sscanf(line, "%*s %65535s", hex);
      SetData(hex);
      if (strcmp(op, "init") == 0) {
        VP8LInitBitReader(&br, data, data_size);
      } else {
        VP8LBitReaderSetBuffer(&br, data, data_size);
      }
    } else if (strcmp(op, "read") == 0) {
      if (sscanf(line, "%*s %d", &n) != 1) return 2;
      result = VP8LReadBits(&br, n);
      has_result = 1;
    } else if (strcmp(op, "prefetch") == 0) {
      result = VP8LPrefetchBits(&br);
      has_result = 1;
    } else if (strcmp(op, "set_bit_pos") == 0) {
      if (sscanf(line, "%*s %d", &n) != 1) return 2;
      VP8LSetBitPos(&br, n);
    } else if (strcmp(op, "fill") == 0) {
      VP8LFillBitWindow(&br);
    } else if (strcmp(op, "do_fill") == 0) {
      VP8LDoFillBitWindow(&br);
    } else if (strcmp(op, "eos") == 0) {
      result = (unsigned int)VP8LIsEndOfStream(&br);
      has_result = 1;
    } else {
      fprintf(stderr, "vp8l_bit_reader: unknown command %s\n", op);
      return 2;
    }
    printf("op=%s\n", op);
    if (has_result) printf("result=%u\n", result);
    PrintState();
  }
  free(data);
  return 0;
}
