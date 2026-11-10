// WP1 probe: reports, for each case read from standard input, whether
// WebPSafeMalloc or WebPSafeCalloc returns memory. Each input line is
// `malloc NMEMB SIZE` or `calloc NMEMB SIZE`, with decimal values; each output
// line is `accepted=1` or `accepted=0`, in input order. The allocated block is
// freed at once.
//
// NMEMB and SIZE must be nonzero. The C functions assert `nmemb * size > 0` on
// the accepted path, so the probe exits with status 2 on a zero value instead of
// letting the debug oracle abort.
//
// Usage: safe_alloc < cases.txt

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/utils.h"

int main(int argc, char* argv[]) {
  char operation[16];
  unsigned long long nmemb, size;
  ProbeCheckArgs(argc, 0, 0, "safe_alloc < cases.txt");
  while (scanf("%15s %llu %llu", operation, &nmemb, &size) == 3) {
    void* ptr;
    if (nmemb == 0 || size == 0) {
      fprintf(stderr,
              "safe_alloc: NMEMB and SIZE must be nonzero; the C functions "
              "assert nmemb * size > 0\n");
      return 2;
    }
    if (strcmp(operation, "malloc") == 0) {
      ptr = WebPSafeMalloc(nmemb, (size_t)size);
    } else if (strcmp(operation, "calloc") == 0) {
      ptr = WebPSafeCalloc(nmemb, (size_t)size);
    } else {
      fprintf(stderr, "safe_alloc: unknown operation %s\n", operation);
      return 2;
    }
    printf("accepted=%d\n", ptr != NULL ? 1 : 0);
    WebPSafeFree(ptr);
  }
  return 0;
}
