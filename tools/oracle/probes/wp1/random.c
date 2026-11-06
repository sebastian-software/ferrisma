// WP1 probe: drives the VP8Random generator of src/utils/random_utils.h through
// commands read from standard input, one per line, and prints what it returns.
//
//   init BITS              VP8InitRandom with the float whose bits are BITS
//   set I1 I2 AMP HEX      stores the state: indices, amplitude and the 55 table
//                          words as 220 hex bytes in memory (little-endian) order
//   bits2 NUM AMP          prints `value=` of VP8RandomBits2(rg, NUM, AMP)
//   bits NUM               prints `value=` of VP8RandomBits(rg, NUM)
//   dump                   prints index1, index2, amp and the table as hex
//
// tests/wp1_random.rs generates the commands and the expected output.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/random_utils.h"

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "random: bad hex digit %c\n", c);
  exit(2);
}

static void ParseHex(const char* text, uint8_t* out, size_t size) {
  size_t i;
  for (i = 0; i < size; ++i) {
    out[i] = (uint8_t)(HexValue(text[2 * i]) * 16 + HexValue(text[2 * i + 1]));
  }
}

static void PrintState(const VP8Random* rg) {
  printf("index1=%d\n", rg->index1);
  printf("index2=%d\n", rg->index2);
  printf("amp=%d\n", rg->amp);
  ProbePrintHex("tab", (const uint8_t*)rg->tab, sizeof(rg->tab));
}

int main(int argc, char* argv[]) {
  VP8Random rg;
  char command[16];
  ProbeCheckArgs(argc, 0, 0, "random < commands.txt");
  memset(&rg, 0, sizeof(rg));
  while (scanf("%15s", command) == 1) {
    if (strcmp(command, "init") == 0) {
      unsigned int bits;
      float dithering;
      if (scanf("%u", &bits) != 1) return 2;
      memcpy(&dithering, &bits, sizeof(dithering));
      VP8InitRandom(&rg, dithering);
    } else if (strcmp(command, "set") == 0) {
      int index1, index2, amp;
      char hex[2 * sizeof(rg.tab) + 1];
      if (scanf("%d %d %d %440s", &index1, &index2, &amp, hex) != 4) return 2;
      rg.index1 = index1;
      rg.index2 = index2;
      rg.amp = amp;
      ParseHex(hex, (uint8_t*)rg.tab, sizeof(rg.tab));
    } else if (strcmp(command, "bits2") == 0) {
      int num_bits, amp;
      if (scanf("%d %d", &num_bits, &amp) != 2) return 2;
      printf("value=%d\n", VP8RandomBits2(&rg, num_bits, amp));
    } else if (strcmp(command, "bits") == 0) {
      int num_bits;
      if (scanf("%d", &num_bits) != 1) return 2;
      printf("value=%d\n", VP8RandomBits(&rg, num_bits));
    } else if (strcmp(command, "dump") == 0) {
      PrintState(&rg);
    } else {
      fprintf(stderr, "random: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
