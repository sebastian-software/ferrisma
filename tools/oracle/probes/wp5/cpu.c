// WP5 probe: SharpYuvGetCPUInfo of sharpyuv/sharpyuv_cpu.c, the CPU feature query
// that the C file compiles from src/dsp/cpu.c under that name (ADR 0002, ADR 0003).
//
// The oracle is built without SIMD. On x86 the pointer is x86CPUInfo, and the
// probe answers for each feature on the running CPU. On other targets the pointer
// is NULL, and the probe says so and does not call it; the Rust query then answers
// `false` for every feature, which tests/wp5_cpu.rs checks.
//
// Usage: cpu
//
// Output, one `key=value` line each:
//   null=1                  SharpYuvGetCPUInfo is NULL; no feature line follows
//   null=0                  SharpYuvGetCPUInfo is set
//   feature<N>=<0|1>        SharpYuvGetCPUInfo(N) for N = 0 to 10: the ten
//                           CPUFeature values, then 10, which is not a feature
//
// tests/wp5_cpu.rs compares the answers with sharp_yuv_get_cpu_info.

#include <stdio.h>

#include "probe_support.h"
#include "sharpyuv/sharpyuv_cpu.h"

// The header renames the query of src/dsp/cpu.c but does not declare the pointer,
// so the probe declares it as sharpyuv/sharpyuv.c does.
extern VP8CPUInfo SharpYuvGetCPUInfo;

#define FEATURE_COUNT 11

int main(int argc, char* argv[]) {
  int feature;
  ProbeCheckArgs(argc, 0, 0, "cpu");
  if (SharpYuvGetCPUInfo == NULL) {
    printf("null=1\n");
    return 0;
  }
  printf("null=0\n");
  for (feature = 0; feature < FEATURE_COUNT; ++feature) {
    printf("feature%d=%d\n", feature,
           SharpYuvGetCPUInfo((CPUFeature)feature) ? 1 : 0);
  }
  return 0;
}
