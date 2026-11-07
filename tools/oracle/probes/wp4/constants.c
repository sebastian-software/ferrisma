// WP4 probe: the constants, enumerators, macros and struct layouts of
// src/dsp/dsp.h, src/dsp/yuv.h, src/dsp/cpu.h and src/utils/rescaler_utils.h
// (ADR 0002). Prints one `NAME=value` line each, in decimal, so that
// tests/wp4_constants.rs can compare them with the Rust translation.
//
// Usage: constants

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include "probe_support.h"
#include "src/dsp/cpu.h"
#include "src/dsp/dsp.h"
#include "src/dsp/yuv.h"
#include "src/utils/rescaler_utils.h"

int main(int argc, char* argv[]) {
  ProbeCheckArgs(argc, 0, 0, "constants");
  printf("BPS=%d\n", BPS);
  printf("MAX_COEFF_THRESH=%d\n", MAX_COEFF_THRESH);
  printf("VP8_SSIM_KERNEL=%d\n", VP8_SSIM_KERNEL);
  printf("VP8_DITHER_DESCALE=%d\n", VP8_DITHER_DESCALE);
  printf("VP8_DITHER_DESCALE_ROUNDER=%d\n", VP8_DITHER_DESCALE_ROUNDER);
  printf("VP8_DITHER_AMP_BITS=%d\n", VP8_DITHER_AMP_BITS);
  printf("VP8_DITHER_AMP_CENTER=%d\n", VP8_DITHER_AMP_CENTER);
  printf("WEBP_TRANSFORM_AC3_C1=%d\n", WEBP_TRANSFORM_AC3_C1);
  printf("WEBP_TRANSFORM_AC3_C2=%d\n", WEBP_TRANSFORM_AC3_C2);
  printf("WEBP_TRANSFORM_AC3_MUL1_100=%d\n", WEBP_TRANSFORM_AC3_MUL1(100));
  printf("WEBP_TRANSFORM_AC3_MUL2_100=%d\n", WEBP_TRANSFORM_AC3_MUL2(100));
  printf("WEBP_TRANSFORM_AC3_MUL1_m7=%d\n", WEBP_TRANSFORM_AC3_MUL1(-7));
  printf("WEBP_TRANSFORM_AC3_MUL2_m7=%d\n", WEBP_TRANSFORM_AC3_MUL2(-7));
  printf("WEBP_FILTER_NONE=%d\n", WEBP_FILTER_NONE);
  printf("WEBP_FILTER_HORIZONTAL=%d\n", WEBP_FILTER_HORIZONTAL);
  printf("WEBP_FILTER_VERTICAL=%d\n", WEBP_FILTER_VERTICAL);
  printf("WEBP_FILTER_GRADIENT=%d\n", WEBP_FILTER_GRADIENT);
  printf("WEBP_FILTER_LAST=%d\n", WEBP_FILTER_LAST);
  printf("WEBP_FILTER_BEST=%d\n", WEBP_FILTER_BEST);
  printf("WEBP_FILTER_FAST=%d\n", WEBP_FILTER_FAST);
  printf("YUV_FIX=%d\n", YUV_FIX);
  printf("YUV_HALF=%d\n", YUV_HALF);
  printf("YUV_FIX2=%d\n", YUV_FIX2);
  printf("YUV_MASK2=%d\n", YUV_MASK2);
  printf("WEBP_RESCALER_RFIX=%d\n", WEBP_RESCALER_RFIX);
  printf("WEBP_RESCALER_ONE=%llu\n", (unsigned long long)WEBP_RESCALER_ONE);
  printf("WEBP_RESCALER_FRAC_1_3=%u\n", (unsigned)WEBP_RESCALER_FRAC(1, 3));
  printf("WEBP_RESCALER_FRAC_m5_7=%u\n", (unsigned)WEBP_RESCALER_FRAC(-5, 7));
  printf("kSSE2=%d\n", kSSE2);
  printf("kSSE3=%d\n", kSSE3);
  printf("kSlowSSSE3=%d\n", kSlowSSSE3);
  printf("kSSE4_1=%d\n", kSSE4_1);
  printf("kAVX=%d\n", kAVX);
  printf("kAVX2=%d\n", kAVX2);
  printf("kNEON=%d\n", kNEON);
  printf("kMIPS32=%d\n", kMIPS32);
  printf("kMIPSdspR2=%d\n", kMIPSdspR2);
  printf("kMSA=%d\n", kMSA);
  printf("sizeof_VP8Histogram=%d\n", (int)sizeof(VP8Histogram));
  printf("sizeof_VP8DistoStats=%d\n", (int)sizeof(VP8DistoStats));
  printf("sizeof_WebPRescaler=%d\n", (int)sizeof(WebPRescaler));
  printf("offsetof_VP8DistoStats_w=%d\n", (int)offsetof(VP8DistoStats, w));
  printf("offsetof_VP8DistoStats_xm=%d\n", (int)offsetof(VP8DistoStats, xm));
  printf("offsetof_VP8DistoStats_ym=%d\n", (int)offsetof(VP8DistoStats, ym));
  printf("offsetof_VP8DistoStats_xxm=%d\n", (int)offsetof(VP8DistoStats, xxm));
  printf("offsetof_VP8DistoStats_xym=%d\n", (int)offsetof(VP8DistoStats, xym));
  printf("offsetof_VP8DistoStats_yym=%d\n", (int)offsetof(VP8DistoStats, yym));
  printf("offsetof_VP8Histogram_max=%d\n", (int)offsetof(VP8Histogram, max_value));
  printf("offsetof_VP8Histogram_last=%d\n", (int)offsetof(VP8Histogram, last_non_zero));
  return 0;
}
