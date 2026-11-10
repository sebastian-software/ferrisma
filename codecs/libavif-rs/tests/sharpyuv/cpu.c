#include <stdio.h>
#include "sharpyuv/sharpyuv_cpu.h"

/* Internal original symbol, intentionally absent from the public API header. */
extern VP8CPUInfo SharpYuvGetCPUInfo;

int main(void) {
  printf("[%d", SharpYuvGetCPUInfo != NULL);
  for (unsigned feature = 0; feature < 10; ++feature) {
    printf(",%d", SharpYuvGetCPUInfo ? SharpYuvGetCPUInfo((CPUFeature)feature) : 0);
  }
  printf("]\n");
  return 0;
}
