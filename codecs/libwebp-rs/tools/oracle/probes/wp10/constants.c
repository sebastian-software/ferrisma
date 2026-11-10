// WP10 probe: the C sizes and the exported tables of the VP8 encoder core
// (src/enc/vp8i_enc.h, cost_enc.c, tree_enc.c, frame_enc.c, quant_enc.c,
// src/dsp/cost.c and enc.c), the encoder version, and the filter strengths of
// VP8FilterStrengthFromDelta (ADR 0002).
//
// The probe takes no input and prints `key=value` lines; tables are printed
// as comma-separated decimal values. tests/wp10_constants.rs compares them.

#include <stdio.h>

#include "probe_support.h"
#include "src/enc/cost_enc.h"
#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"

static void PrintU8s(const char* key, const uint8_t* v, int n) {
  int i;
  printf("%s=", key);
  for (i = 0; i < n; ++i) printf("%s%u", i ? "," : "", v[i]);
  printf("\n");
}

static void PrintU16s(const char* key, const uint16_t* v, int n) {
  int i;
  printf("%s=", key);
  for (i = 0; i < n; ++i) printf("%s%u", i ? "," : "", v[i]);
  printf("\n");
}

static void PrintInts(const char* key, const int* v, int n) {
  int i;
  printf("%s=", key);
  for (i = 0; i < n; ++i) printf("%s%d", i ? "," : "", v[i]);
  printf("\n");
}

int main(int argc, char* argv[]) {
  int strengths[8 * 80];
  int s, d;
  ProbeCheckArgs(argc, 0, 0, "constants");
  (void)argv;
  printf("sizeof_VP8Encoder=%zu\n", sizeof(VP8Encoder));
  printf("sizeof_VP8MBInfo=%zu\n", sizeof(VP8MBInfo));
  printf("sizeof_DError=%zu\n", sizeof(DError));
  printf("sizeof_LFStats=%zu\n", sizeof(LFStats));
  printf("sizeof_VP8Tokens_header=%zu\n", sizeof(void*));
  printf("version=%d\n", WebPGetEncoderVersion());
  printf("MAX_LF_LEVELS=%d\nMAX_VARIABLE_LEVEL=%d\nMAX_LEVEL=%d\n",
         MAX_LF_LEVELS, MAX_VARIABLE_LEVEL, MAX_LEVEL);
  printf("YUV_SIZE_ENC=%d\nPRED_SIZE_ENC=%d\n", YUV_SIZE_ENC, PRED_SIZE_ENC);
  printf("Y_OFF_ENC=%d\nU_OFF_ENC=%d\nV_OFF_ENC=%d\n", Y_OFF_ENC, U_OFF_ENC,
         V_OFF_ENC);
  printf("I16=%d,%d,%d,%d\n", I16DC16, I16TM16, I16VE16, I16HE16);
  printf("C8=%d,%d,%d,%d\n", C8DC8, C8TM8, C8VE8, C8HE8);
  printf("I4=%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\n", I4DC4, I4TM4, I4VE4, I4HE4,
         I4RD4, I4VR4, I4LD4, I4VL4, I4HD4, I4HU4, I4TMP);
  printf("MAX_COST=%lld\nQFIX=%d\nERROR_DIFFUSION_QUALITY=%d\n",
         (long long)MAX_COST, QFIX, ERROR_DIFFUSION_QUALITY);
  PrintU16s("VP8EntropyCost", VP8EntropyCost, 256);
  PrintU16s("VP8LevelFixedCosts", VP8LevelFixedCosts, MAX_LEVEL + 1);
  PrintU8s("VP8EncBands", VP8EncBands, 17);
  PrintU16s("VP8FixedCostsUV", VP8FixedCostsUV, 4);
  PrintU16s("VP8FixedCostsI16", VP8FixedCostsI16, 4);
  PrintU16s("VP8FixedCostsI4", &VP8FixedCostsI4[0][0][0],
            NUM_BMODES * NUM_BMODES * NUM_BMODES);
  PrintU8s("VP8CoeffsProba0", &VP8CoeffsProba0[0][0][0][0],
           NUM_TYPES * NUM_BANDS * NUM_CTX * NUM_PROBAS);
  PrintU8s("VP8CoeffsUpdateProba", &VP8CoeffsUpdateProba[0][0][0][0],
           NUM_TYPES * NUM_BANDS * NUM_CTX * NUM_PROBAS);
  PrintU8s("VP8Cat3", VP8Cat3, 3);
  PrintU8s("VP8Cat4", VP8Cat4, 4);
  PrintU8s("VP8Cat5", VP8Cat5, 5);
  PrintU8s("VP8Cat6", VP8Cat6, 11);
  PrintU16s("VP8Scan", VP8Scan, 16);
  PrintU16s("VP8I16ModeOffsets", VP8I16ModeOffsets, 4);
  PrintU16s("VP8UVModeOffsets", VP8UVModeOffsets, 4);
  PrintInts("VP8DspScan", VP8DspScan, 16 + 4 + 4);
  for (s = 0; s < 8; ++s) {
    for (d = 0; d < 80; ++d) {
      strengths[s * 80 + d] = VP8FilterStrengthFromDelta(s, d);
    }
  }
  PrintInts("VP8FilterStrengthFromDelta", strengths, 8 * 80);
  return 0;
}
