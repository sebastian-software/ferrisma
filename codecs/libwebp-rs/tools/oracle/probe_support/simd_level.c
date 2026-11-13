// Caps upstream's CPU query at the instruction-set level that the environment
// variable LIBWEBP_SIMD_LEVEL names, before main runs (ADR 0008).
//
// libwebp has no option of its own to restrict its SIMD selection to a lower
// x86 level; its mechanism is the writable query pointer VP8GetCPUInfo, which
// every Init function consults (the tools' -noasm sets it to NULL). This file
// is linked into every executable of the oracle, the probes, upstream's tools
// and the consumer builds against the Rust library alike, and wraps the query
// that the pointer holds at start-up:
//
//   unset, avx2, neon   no change (the level the host's query reports)
//   sse41               kSSE2, kSSE3, kSlowSSSE3 and kSSE4_1 as reported;
//                       kAVX, kAVX2 and every other feature answer 0
//   sse2                kSSE2 as reported; every other feature answers 0
//
// The port applies the same cap to its own query with its `oracle` feature
// (src/dsp/cpu.rs), so that both sides of a comparison select the same
// variants. An unknown value aborts, so a misspelled mode cannot pass as the
// uncapped one. A NULL pointer (the scalar reference build on 64-bit ARM)
// stays NULL.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dsp/cpu.h"

// Declared per file, as upstream's own users (rescaler.c, upsampling.c) do.
extern VP8CPUInfo VP8GetCPUInfo;

static VP8CPUInfo original_query;
static int max_feature;  // the highest CPUFeature value answered as reported

static int CappedQuery(CPUFeature feature) {
  if ((int)feature > max_feature) return 0;
  if (max_feature == (int)kSSE2 && feature != kSSE2) return 0;
  return original_query(feature);
}

__attribute__((constructor)) static void InstallSimdLevel(void) {
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
  if (VP8GetCPUInfo == NULL) return;
  original_query = VP8GetCPUInfo;
  VP8GetCPUInfo = CappedQuery;
}
