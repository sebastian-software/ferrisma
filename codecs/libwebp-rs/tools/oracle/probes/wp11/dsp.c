// WP11 probe: the lossless encoder DSP functions of src/dsp/lossless_enc.c and
// the four inline functions of src/dsp/lossless_common.h that read its tables
// (ADR 0002). Commands on standard input:
//
//   tables                      prints kLog2Table, kSLog2Table,
//                               kPrefixEncodeCode and kPrefixEncodeExtraBitsValue
//   logs VALUES                 VP8LFastLog2 and VP8LFastSLog2 of each value
//   prefix VALUES               VP8LPrefixEncodeBits and VP8LPrefixEncode
//   subgreen ARGB               VP8LSubtractGreenFromBlueAndRed in place
//   transform G2R G2B R2B ARGB  VP8LTransformColor in place
//   collect STRIDE W H G2R G2B R2B ARGB
//                               VP8LCollectColorRedTransforms and
//                               VP8LCollectColorBlueTransforms
//   mismatch LEN A B            VP8LVectorMismatch
//   bundle W XBITS ROW          VP8LBundleColorMap (ROW holds one index per
//                               32-bit value)
//   extra LEN POPULATION        VP8LExtraCost
//   addvec A B                  VP8LAddVector and VP8LAddVectorEq
//   entropy N X Y               VP8LCombinedShannonEntropy, VP8LShannonEntropy,
//                               VP8LBitsEntropyUnrefined, VP8LBitsEntropy,
//                               VP8LGetEntropyUnrefined and
//                               VP8LGetCombinedEntropyUnrefined on N entries
//   predsub N UPPER CURRENT     every VP8LPredictorsSub and VP8LPredictorsSub_C
//                               mode on N pixels, with UPPER holding N + 2 and
//                               CURRENT N + 1 pixels (the first of each is the
//                               pixel to the left)
//
// Arrays are hex strings of 32-bit values. tests/wp11_lossless_enc.rs generates
// the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/lossless.h"
#include "src/dsp/lossless_common.h"
#include "src/enc/histogram_enc.h"
#include "wp11_io.h"

static void PrintEntropy(const char* key, const VP8LBitEntropy* e) {
  printf("%s=%016llx,%u,%d,%u,%u\n", key, (unsigned long long)e->entropy,
         e->sum, e->nonzeros, e->max_val, e->nonzero_code);
}

static void PrintStreaks(const char* key, const VP8LStreaks* s) {
  printf("%s=%d,%d,%d,%d,%d,%d\n", key, s->counts[0], s->counts[1],
         s->streaks[0][0], s->streaks[0][1], s->streaks[1][0],
         s->streaks[1][1]);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "dsp < commands.txt");
  (void)argv;
  VP8LEncDspInit();
  while ((command = Wp11ReadToken()) != NULL) {
    if (strcmp(command, "tables") == 0) {
      uint32_t codes[PREFIX_LOOKUP_IDX_MAX];
      uint32_t extra[PREFIX_LOOKUP_IDX_MAX];
      int i;
      Wp11PrintU32s("log2", kLog2Table, LOG_LOOKUP_IDX_MAX);
      Wp11PrintU64s("slog2", kSLog2Table, LOG_LOOKUP_IDX_MAX);
      for (i = 0; i < PREFIX_LOOKUP_IDX_MAX; ++i) {
        codes[i] = ((uint32_t)(uint8_t)kPrefixEncodeCode[i].code << 8) |
                   (uint8_t)kPrefixEncodeCode[i].extra_bits;
        extra[i] = kPrefixEncodeExtraBitsValue[i];
      }
      Wp11PrintU32s("prefix_code", codes, PREFIX_LOOKUP_IDX_MAX);
      Wp11PrintU32s("prefix_extra", extra, PREFIX_LOOKUP_IDX_MAX);
    } else if (strcmp(command, "logs") == 0) {
      size_t n, i;
      uint32_t* const values = Wp11ReadU32s(&n);
      uint32_t* const log2 = (uint32_t*)calloc(n + 1, sizeof(*log2));
      uint64_t* const slog2 = (uint64_t*)calloc(n + 1, sizeof(*slog2));
      if (log2 == NULL || slog2 == NULL) return 2;
      for (i = 0; i < n; ++i) {
        log2[i] = VP8LFastLog2(values[i]);
        slog2[i] = VP8LFastSLog2(values[i]);
      }
      Wp11PrintU32s("fast_log2", log2, n);
      Wp11PrintU64s("fast_slog2", slog2, n);
      free(values);
      free(log2);
      free(slog2);
    } else if (strcmp(command, "prefix") == 0) {
      size_t n, i;
      uint32_t* const values = Wp11ReadU32s(&n);
      uint32_t* const out = (uint32_t*)calloc(5 * n + 1, sizeof(*out));
      if (out == NULL) return 2;
      for (i = 0; i < n; ++i) {
        int code, extra_bits, extra_bits_value;
        VP8LPrefixEncodeBits((int)values[i], &code, &extra_bits);
        out[5 * i + 0] = (uint32_t)code;
        out[5 * i + 1] = (uint32_t)extra_bits;
        VP8LPrefixEncode((int)values[i], &code, &extra_bits, &extra_bits_value);
        out[5 * i + 2] = (uint32_t)code;
        out[5 * i + 3] = (uint32_t)extra_bits;
        out[5 * i + 4] = (uint32_t)extra_bits_value;
      }
      Wp11PrintU32s("prefix", out, 5 * n);
      free(values);
      free(out);
    } else if (strcmp(command, "subgreen") == 0) {
      size_t n;
      uint32_t* const argb = Wp11ReadU32s(&n);
      VP8LSubtractGreenFromBlueAndRed(argb, (int)n);
      Wp11PrintU32s("argb", argb, n);
      free(argb);
    } else if (strcmp(command, "transform") == 0) {
      VP8LMultipliers m;
      size_t n;
      uint32_t* argb;
      m.green_to_red = (uint8_t)Wp11ReadLong();
      m.green_to_blue = (uint8_t)Wp11ReadLong();
      m.red_to_blue = (uint8_t)Wp11ReadLong();
      argb = Wp11ReadU32s(&n);
      VP8LTransformColor(&m, argb, (int)n);
      Wp11PrintU32s("argb", argb, n);
      free(argb);
    } else if (strcmp(command, "collect") == 0) {
      const int stride = (int)Wp11ReadLong();
      const int width = (int)Wp11ReadLong();
      const int height = (int)Wp11ReadLong();
      const int g2r = (int)Wp11ReadLong();
      const int g2b = (int)Wp11ReadLong();
      const int r2b = (int)Wp11ReadLong();
      size_t n;
      uint32_t* const argb = Wp11ReadU32s(&n);
      uint32_t red[256], blue[256];
      memset(red, 0, sizeof(red));
      memset(blue, 0, sizeof(blue));
      VP8LCollectColorRedTransforms(argb, stride, width, height, g2r, red);
      VP8LCollectColorBlueTransforms(argb, stride, width, height, g2b, r2b,
                                     blue);
      Wp11PrintU32s("red", red, 256);
      Wp11PrintU32s("blue", blue, 256);
      free(argb);
    } else if (strcmp(command, "mismatch") == 0) {
      const int length = (int)Wp11ReadLong();
      size_t na, nb;
      uint32_t* const a = Wp11ReadU32s(&na);
      uint32_t* const b = Wp11ReadU32s(&nb);
      printf("mismatch=%d\n", VP8LVectorMismatch(a, b, length));
      free(a);
      free(b);
    } else if (strcmp(command, "bundle") == 0) {
      const int width = (int)Wp11ReadLong();
      const int xbits = (int)Wp11ReadLong();
      size_t n, i;
      uint32_t* const row32 = Wp11ReadU32s(&n);
      uint8_t* const row = (uint8_t*)calloc(n + 1, 1);
      const size_t dst_size = (size_t)VP8LSubSampleSize(width, xbits);
      uint32_t* const dst = (uint32_t*)calloc(dst_size + 1, sizeof(*dst));
      if (row == NULL || dst == NULL) return 2;
      for (i = 0; i < n; ++i) row[i] = (uint8_t)row32[i];
      VP8LBundleColorMap(row, width, xbits, dst);
      Wp11PrintU32s("bundle", dst, dst_size);
      free(row32);
      free(row);
      free(dst);
    } else if (strcmp(command, "extra") == 0) {
      const int length = (int)Wp11ReadLong();
      size_t n;
      uint32_t* const population = Wp11ReadU32s(&n);
      printf("extra=%u\n", VP8LExtraCost(population, length));
      free(population);
    } else if (strcmp(command, "addvec") == 0) {
      size_t na, nb;
      uint32_t* const a = Wp11ReadU32s(&na);
      uint32_t* const b = Wp11ReadU32s(&nb);
      uint32_t* const out = (uint32_t*)calloc(na + 1, sizeof(*out));
      if (out == NULL) return 2;
      VP8LAddVector(a, b, out, (int)na);
      Wp11PrintU32s("sum", out, na);
      VP8LAddVectorEq(a, b, (int)na);
      Wp11PrintU32s("sum_eq", b, na);
      free(a);
      free(b);
      free(out);
    } else if (strcmp(command, "entropy") == 0) {
      const int length = (int)Wp11ReadLong();
      size_t nx, ny;
      uint32_t* const x = Wp11ReadU32s(&nx);
      uint32_t* const y = Wp11ReadU32s(&ny);
      VP8LBitEntropy e;
      VP8LStreaks s;
      printf("combined_shannon=%016llx\n",
             (unsigned long long)VP8LCombinedShannonEntropy(x, y));
      printf("shannon=%016llx\n",
             (unsigned long long)VP8LShannonEntropy(x, length));
      VP8LBitsEntropyUnrefined(x, length, &e);
      PrintEntropy("bits_unrefined", &e);
      printf("bits_entropy=%016llx\n",
             (unsigned long long)VP8LBitsEntropy(x, length));
      VP8LGetEntropyUnrefined(x, length, &e, &s);
      PrintEntropy("entropy_unrefined", &e);
      PrintStreaks("entropy_streaks", &s);
      VP8LGetCombinedEntropyUnrefined(x, y, length, &e, &s);
      PrintEntropy("combined_unrefined", &e);
      PrintStreaks("combined_streaks", &s);
      free(x);
      free(y);
    } else if (strcmp(command, "predsub") == 0) {
      const int num = (int)Wp11ReadLong();
      size_t nu, nc;
      uint32_t* const upper = Wp11ReadU32s(&nu);
      uint32_t* const current = Wp11ReadU32s(&nc);
      uint32_t* const out = (uint32_t*)calloc((size_t)num + 1, sizeof(*out));
      int mode;
      if (out == NULL || nu != (size_t)num + 2 || nc != (size_t)num + 1) {
        return 2;
      }
      for (mode = 0; mode < 16; ++mode) {
        VP8LPredictorsSub[mode](current + 1, upper + 1, num, out);
        Wp11PrintU32s("sub", out, (size_t)num);
        memset(out, 0, (size_t)num * sizeof(*out));
        VP8LPredictorsSub_C[mode](current + 1, upper + 1, num, out);
        Wp11PrintU32s("sub_c", out, (size_t)num);
      }
      free(upper);
      free(current);
      free(out);
    } else {
      fprintf(stderr, "dsp: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  return 0;
}
