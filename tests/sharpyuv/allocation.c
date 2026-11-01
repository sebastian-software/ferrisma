#include <stdio.h>
#include <stdlib.h>
#include "sharpyuv/sharpyuv.h"
#include "sharpyuv/sharpyuv_csp.h"

static int calls, fail_at, live;
void *sharp_test_malloc(size_t size) {
  ++calls;
  if (calls == fail_at) return NULL;
  void *p = malloc(size);
  if (p) ++live;
  return p;
}
void sharp_test_free(void *p) {
  if (p) --live;
  free(p);
}

int main(void) {
  unsigned char rgb[17 * 9 * 3], y[17 * 9], u[9 * 5], v[9 * 5];
  for (size_t i = 0; i < sizeof(rgb); ++i) rgb[i] = (unsigned char)(i * 73);
  int total = 0;
  printf("[");
  for (int failure = 0; failure <= total; ++failure) {
    calls = live = 0;
    fail_at = failure;
    int code = SharpYuvConvert(rgb, rgb + 1, rgb + 2, 3, 17 * 3, 8,
                              y, 17, u, 9, v, 9, 8, 17, 9,
                              SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp));
    if (live != 0 || (failure == 0 ? code != 1 : code != 0)) return 2;
    if (failure == 0) total = calls;
    printf("%s[%d,%d,%d,%d]", failure ? "," : "", failure, calls, code, live);
  }
  printf("]\n");
  return total > 0 ? 0 : 3;
}
