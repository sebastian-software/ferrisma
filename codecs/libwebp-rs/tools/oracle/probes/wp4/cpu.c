// WP4 probe: VP8GetCPUInfo of src/dsp/cpu.c, the feature query, and on x86 the
// CPU model table of its static helper CheckSlowModel (ADR 0002, ADR 0003).
//
// The probe includes the upstream source on x86 to reach CheckSlowModel, which
// only x86 builds compile. Elsewhere the reference configuration leaves
// VP8GetCPUInfo NULL, and the probe says so.
//
// Usage: cpu
//
// Output, one `key=value` line each:
//   null=1                     VP8GetCPUInfo is NULL; nothing else follows
//                              for the feature query
//   null=0                     VP8GetCPUInfo is set
//   feature<N>=<0|1>           VP8GetCPUInfo(N) for the ten CPUFeature values
//   slow_<INFO>=<0|1>          CheckSlowModel(INFO) on x86, for the EAX values
//                              of cpuid leaf 1 that tests/wp4_cpu.rs generates
//
// tests/wp4_cpu.rs generates the same grid and compares the answers.

#include <stdio.h>

#include "probe_support.h"
#include "src/dsp/cpu.h"

#if defined(__i386__) || defined(__x86_64__)
#include "src/dsp/cpu.c"
#else
extern VP8CPUInfo VP8GetCPUInfo;
#endif

#define FEATURE_COUNT 10

int main(int argc, char* argv[]) {
  int feature;
  ProbeCheckArgs(argc, 0, 0, "cpu");
  if (VP8GetCPUInfo == NULL) {
    printf("null=1\n");
  } else {
    printf("null=0\n");
    for (feature = 0; feature < FEATURE_COUNT; ++feature) {
      printf("feature%d=%d\n", feature,
             VP8GetCPUInfo((CPUFeature)feature) ? 1 : 0);
    }
  }
#if defined(__i386__) || defined(__x86_64__)
  {
    int family, ext, low;
    for (family = 0; family < 16; family += 3) {
      for (ext = 0; ext < 4; ++ext) {
        for (low = 0; low < 16; ++low) {
          const int info = (ext << 16) | (family << 8) | (low << 4);
          printf("slow_%d=%d\n", info, CheckSlowModel(info) ? 1 : 0);
        }
      }
    }
  }
#endif
  return 0;
}
