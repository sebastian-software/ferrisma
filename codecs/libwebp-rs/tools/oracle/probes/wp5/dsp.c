// WP5 probe: the three SharpYUV dispatch functions of sharpyuv/sharpyuv_dsp.c,
// SharpYuvUpdateY, SharpYuvUpdateRGB and SharpYuvFilterRow, called through the
// pointers that SharpYuvInitDsp assigns, on 200 seeded inputs each.
//
// The inputs of case K come from the generator of wp5_rng.h, seeded with 0x5eed
// plus K, and are drawn in the order of the Rust test. Lengths are at most 40
// (or 20 for the filter), sample ranges follow the conversion (10, 12 or 14 bits
// for the unsigned samples; int16_t chroma differences within +-16384), and each
// output array is printed whole so that writes beyond `len` are visible.
//
// Output:
//   group=update-y-K   diff=D   dst=HEX
//   group=update-rgb-K dst=HEX
//   group=filter-row-K out=HEX

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "probe_support.h"
#include "sharpyuv/sharpyuv_dsp.h"
#include "wp5_rng.h"

#define kMaxLen 40
#define kCases 200

static void PrintU16Hex(const char* key, const uint16_t* data, int count) {
  uint8_t bytes[2 * (2 * kMaxLen + 2)];
  int i;
  for (i = 0; i < count; ++i) {
    bytes[2 * i] = (uint8_t)(data[i] & 0xff);
    bytes[2 * i + 1] = (uint8_t)(data[i] >> 8);
  }
  ProbePrintHex(key, bytes, 2 * (size_t)count);
}

static void PrintI16Hex(const char* key, const int16_t* data, int count) {
  uint8_t bytes[2 * (2 * kMaxLen + 2)];
  int i;
  for (i = 0; i < count; ++i) {
    bytes[2 * i] = (uint8_t)((uint16_t)data[i] & 0xff);
    bytes[2 * i + 1] = (uint8_t)((uint16_t)data[i] >> 8);
  }
  ProbePrintHex(key, bytes, 2 * (size_t)count);
}

static int16_t RandomI16(Wp5Rng* rng) {
  return (int16_t)((int)Wp5RngBelow(rng, 32768) - 16384);
}

static uint16_t RandomSample(Wp5Rng* rng, int bit_depth) {
  return (uint16_t)Wp5RngBelow(rng, (uint64_t)1 << bit_depth);
}

int main(int argc, char* argv[]) {
  int k, i, len, bit_depth;
  uint64_t diff;
  ProbeCheckArgs(argc, 0, 0, "dsp");
  SharpYuvInitDsp();
  for (k = 0; k < kCases; ++k) {
    // One generator stream per case, drawn in the order of the Rust test.
    Wp5Rng rng;
    uint16_t src[kMaxLen], ref[kMaxLen], dst[kMaxLen];
    int16_t src16[kMaxLen], ref16[kMaxLen], dst16[kMaxLen];
    int16_t a[kMaxLen + 1], b[kMaxLen + 1];
    uint16_t best_y[2 * kMaxLen + 2], out[2 * kMaxLen + 2];

    rng.state = 0x5eedULL + (uint64_t)k;
    bit_depth = 10 + 2 * (int)Wp5RngBelow(&rng, 3);

    // update-y: the sample ranges follow the bit depth
    len = 1 + (int)Wp5RngBelow(&rng, kMaxLen);
    for (i = 0; i < kMaxLen; ++i) src[i] = RandomSample(&rng, bit_depth);
    for (i = 0; i < kMaxLen; ++i) ref[i] = RandomSample(&rng, bit_depth);
    for (i = 0; i < kMaxLen; ++i) dst[i] = RandomSample(&rng, bit_depth);
    diff = SharpYuvUpdateY(src, ref, dst, len, bit_depth);
    printf("group=update-y-%d\n", k);
    printf("diff=%llu\n", (unsigned long long)diff);
    PrintU16Hex("dst", dst, kMaxLen);

    // update-rgb: int16_t differences
    len = 1 + (int)Wp5RngBelow(&rng, kMaxLen);
    for (i = 0; i < kMaxLen; ++i) src16[i] = RandomI16(&rng);
    for (i = 0; i < kMaxLen; ++i) ref16[i] = RandomI16(&rng);
    for (i = 0; i < kMaxLen; ++i) dst16[i] = RandomI16(&rng);
    SharpYuvUpdateRGB(src16, ref16, dst16, len);
    printf("group=update-rgb-%d\n", k);
    PrintI16Hex("dst", dst16, kMaxLen);

    // filter-row: two chroma rows, one luma row and the output
    len = 1 + (int)Wp5RngBelow(&rng, 20);
    for (i = 0; i < kMaxLen + 1; ++i) a[i] = RandomI16(&rng);
    for (i = 0; i < kMaxLen + 1; ++i) b[i] = RandomI16(&rng);
    for (i = 0; i < 2 * kMaxLen + 2; ++i) best_y[i] = RandomSample(&rng, bit_depth);
    memset(out, 0, sizeof(out));
    SharpYuvFilterRow(a, b, len, best_y, out, bit_depth);
    printf("group=filter-row-%d\n", k);
    PrintU16Hex("out", out, 2 * kMaxLen + 2);
  }
  return 0;
}
