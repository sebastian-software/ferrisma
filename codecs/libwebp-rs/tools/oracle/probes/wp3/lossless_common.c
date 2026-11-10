// WP3 probe: the inline helpers of src/dsp/lossless_common.h (ADR 0002). It
// reads commands from standard input and prints what the library computes:
//
//   subsample SIZE BITS   VP8LSubSampleSize; prints sub=
//   near QUALITY          VP8LNearLosslessBits; prints near=
//   rsr V SHIFT           RightShiftRound; prints rsr=
//   divround A B          DivRound; prints div=
//   prefix DISTANCE       VP8LPrefixEncodeBitsNoLUT and VP8LPrefixEncodeNoLUT;
//                         prints bits=CODE,EXTRA and full=CODE,EXTRA,VALUE
//   pixels A B            VP8LAddPixels and VP8LSubPixels; prints add=, sub=
//   index V               VP8GetARGBIndex, VP8GetAlphaIndex, VP8GetARGBValue
//                         and VP8GetAlphaValue; prints index=
//
// Values are decimal; 64-bit values are read as unsigned or signed as the
// function takes them. tests/wp3_lossless_common.rs generates the commands.

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/lossless_common.h"
#include "wp3_io.h"

static uint64_t ReadU64(void) {
  char* token = Wp3ReadToken();
  uint64_t value;
  if (token == NULL) exit(2);
  value = strtoull(token, NULL, 10);
  free(token);
  return value;
}

static int64_t ReadI64(void) {
  char* token = Wp3ReadToken();
  int64_t value;
  if (token == NULL) exit(2);
  value = strtoll(token, NULL, 10);
  free(token);
  return value;
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "lossless_common < commands.txt");
  while ((command = Wp3ReadToken()) != NULL) {
    if (strcmp(command, "subsample") == 0) {
      const uint32_t size = (uint32_t)ReadU64();
      printf("sub=%u\n", VP8LSubSampleSize(size, (uint32_t)ReadU64()));
    } else if (strcmp(command, "near") == 0) {
      printf("near=%d\n", VP8LNearLosslessBits((int)Wp3ReadLong()));
    } else if (strcmp(command, "rsr") == 0) {
      const uint64_t v = ReadU64();
      printf("rsr=%" PRIu64 "\n", RightShiftRound(v, (uint32_t)ReadU64()));
    } else if (strcmp(command, "divround") == 0) {
      const int64_t a = ReadI64();
      printf("div=%" PRId64 "\n", DivRound(a, ReadI64()));
    } else if (strcmp(command, "prefix") == 0) {
      const int distance = (int)Wp3ReadLong();
      int code, extra_bits, extra_bits_value;
      VP8LPrefixEncodeBitsNoLUT(distance, &code, &extra_bits);
      printf("bits=%d,%d\n", code, extra_bits);
      VP8LPrefixEncodeNoLUT(distance, &code, &extra_bits, &extra_bits_value);
      printf("full=%d,%d,%d\n", code, extra_bits, extra_bits_value);
    } else if (strcmp(command, "pixels") == 0) {
      const uint32_t a = (uint32_t)ReadU64();
      const uint32_t b = (uint32_t)ReadU64();
      printf("add=%u\nsub=%u\n", VP8LAddPixels(a, b), VP8LSubPixels(a, b));
    } else if (strcmp(command, "index") == 0) {
      const uint32_t v = (uint32_t)ReadU64();
      printf("index=%u,%u,%u,%u\n", VP8GetARGBIndex(v),
             VP8GetAlphaIndex((uint8_t)v), VP8GetARGBValue(v),
             VP8GetAlphaValue(v));
    } else {
      fprintf(stderr, "lossless_common: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
