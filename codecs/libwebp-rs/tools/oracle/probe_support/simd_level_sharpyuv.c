// Caps upstream's SharpYUV CPU query at the level LIBWEBP_SIMD_LEVEL names,
// before main runs, with the rule of simd_level.c (ADR 0008).
//
// sharpyuv_cpu.c compiles src/dsp/cpu.c a second time with VP8GetCPUInfo
// renamed to SharpYuvGetCPUInfo, so libsharpyuv holds its own query pointer.
// The encoder replaces it with VP8GetCPUInfo (SharpYuvInit(VP8GetCPUInfo) in
// picture_csp_enc.c), which simd_level.c caps; the probes that call the
// SharpYUV functions or read the pointer directly see this cap. Only the
// executables that link upstream's libsharpyuv link this file: the consumer
// builds against the Rust library have no SharpYuvGetCPUInfo, and the port
// answers SharpYuvGetCPUInfo through its capped VP8GetCPUInfo.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sharpyuv/sharpyuv_cpu.h"

extern VP8CPUInfo SharpYuvGetCPUInfo;

static VP8CPUInfo original_query;
static int max_feature;  // the highest CPUFeature value answered as reported

static int CappedQuery(CPUFeature feature) {
  if ((int)feature > max_feature) return 0;
  if (max_feature == (int)kSSE2 && feature != kSSE2) return 0;
  return original_query(feature);
}

__attribute__((constructor)) static void InstallSharpYuvSimdLevel(void) {
  const char* const level = getenv("LIBWEBP_SIMD_LEVEL");
  if (level == NULL || level[0] == '\0' || !strcmp(level, "avx2") ||
      !strcmp(level, "neon")) {
    return;
  }
  if (!strcmp(level, "sse41")) {
    max_feature = (int)kSSE4_1;
  } else if (!strcmp(level, "sse2")) {
    max_feature = (int)kSSE2;
  } else {
    fprintf(stderr, "LIBWEBP_SIMD_LEVEL=%s: unknown level\n", level);
    abort();
  }
  if (SharpYuvGetCPUInfo == NULL) return;
  original_query = SharpYuvGetCPUInfo;
  SharpYuvGetCPUInfo = CappedQuery;
}
