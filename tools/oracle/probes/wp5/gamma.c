// WP5 probe: the transfer functions and the gamma tables of
// sharpyuv/sharpyuv_gamma.c. SharpYuvGammaToLinear and SharpYuvLinearToGamma are
// evaluated over every valid transfer function (1 and 4 to 18; the reserved and
// unspecified values 0, 2 and 3 and the count 19 are undefined, see
// src/sharpyuv/sharpyuv_gamma.rs, "Excluded undefined behavior", item 2) and
// every bit depth of the conversion (8, 10, 12 and 16), which covers both gamma
// tables through the sRGB path.
//
// Output, one group per table, with the values as little-endian bytes in hex:
//   group=gamma-to-linear-tN-bB  hex=...   (u32 per input value)
//   group=linear-to-gamma-tN-bB  hex=...   (u16 per input value)
// Inputs up to 12 bits are swept exhaustively, and so is the sRGB table at 16 bits;
// the other 16-bit sweeps take every 257th value and the last one. The linear
// sweep covers 0 to 1 << 16 inclusive for sRGB and 0 to 65535 otherwise, with a
// stride of 257 except for sRGB.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "probe_support.h"
#include "sharpyuv/sharpyuv.h"
#include "sharpyuv/sharpyuv_gamma.h"

static const int kTransfers[16] = {1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18};
static const int kBitDepths[4] = {8, 10, 12, 16};

static void PrintGammaToLinear(int transfer, int bit_depth) {
  const uint32_t max_value = (1u << bit_depth) - 1u;
  const uint32_t step = (transfer == 13 || bit_depth <= 12) ? 1u : 257u;
  uint32_t v, count = 0;
  uint8_t* bytes;
  size_t n = 0;
  for (v = 0; v <= max_value; v += step) ++count;
  if ((max_value % step) != 0) ++count;  // the last value, when the stride misses it
  bytes = (uint8_t*)malloc((size_t)count * 4);
  if (bytes == NULL) exit(2);
  for (v = 0; v <= max_value; v += step) {
    const uint32_t value = SharpYuvGammaToLinear((uint16_t)v, bit_depth,
                                                 (SharpYuvTransferFunctionType)transfer);
    bytes[n++] = (uint8_t)(value & 0xff);
    bytes[n++] = (uint8_t)((value >> 8) & 0xff);
    bytes[n++] = (uint8_t)((value >> 16) & 0xff);
    bytes[n++] = (uint8_t)((value >> 24) & 0xff);
  }
  if ((max_value % step) != 0) {
    const uint32_t value = SharpYuvGammaToLinear((uint16_t)max_value, bit_depth,
                                                 (SharpYuvTransferFunctionType)transfer);
    bytes[n++] = (uint8_t)(value & 0xff);
    bytes[n++] = (uint8_t)((value >> 8) & 0xff);
    bytes[n++] = (uint8_t)((value >> 16) & 0xff);
    bytes[n++] = (uint8_t)((value >> 24) & 0xff);
  }
  printf("group=gamma-to-linear-t%d-b%d\n", transfer, bit_depth);
  ProbePrintHex("hex", bytes, n);
  free(bytes);
}

static void PrintLinearToGamma(int transfer, int bit_depth) {
  // Linear values reach 1 << 16 for the sRGB table, which the sweep includes. The
  // other transfer functions stop at 65535: for 65536, a value that the conversion
  // never passes them, the C code converts an out-of-range float to uint16_t, which
  // is undefined (src/sharpyuv/sharpyuv_gamma.rs, "Excluded undefined
  // behavior", item 1).
  const uint32_t top = (transfer == 13) ? (1u << 16) : 65535u;
  const uint32_t step = (transfer == 13) ? 1u : 257u;
  uint32_t v, count = 0;
  uint8_t* bytes;
  size_t n = 0;
  for (v = 0; v <= top; v += step) ++count;
  if ((top % step) != 0) ++count;
  bytes = (uint8_t*)malloc((size_t)count * 2);
  if (bytes == NULL) exit(2);
  for (v = 0; v <= top; v += step) {
    const uint16_t value = SharpYuvLinearToGamma(v, bit_depth,
                                                 (SharpYuvTransferFunctionType)transfer);
    bytes[n++] = (uint8_t)(value & 0xff);
    bytes[n++] = (uint8_t)(value >> 8);
  }
  if ((top % step) != 0) {
    const uint16_t value = SharpYuvLinearToGamma(top, bit_depth,
                                                 (SharpYuvTransferFunctionType)transfer);
    bytes[n++] = (uint8_t)(value & 0xff);
    bytes[n++] = (uint8_t)(value >> 8);
  }
  printf("group=linear-to-gamma-t%d-b%d\n", transfer, bit_depth);
  ProbePrintHex("hex", bytes, n);
  free(bytes);
}

int main(int argc, char* argv[]) {
  int t, b;
  ProbeCheckArgs(argc, 0, 0, "gamma");
  SharpYuvInitGammaTables();
  for (t = 0; t < 16; ++t) {
    for (b = 0; b < 4; ++b) {
      // The sRGB table path needs bit depths up to 16; the others stay in 8 to 12
      // bits exhaustively and sample 16 bits.
      PrintGammaToLinear(kTransfers[t], kBitDepths[b]);
      PrintLinearToGamma(kTransfers[t], kBitDepths[b]);
    }
  }
  return 0;
}
