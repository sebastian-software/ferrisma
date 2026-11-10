/* WP10 probe: the input generator and the hashes of the kernel probes.
 *
 * Usage: rng -seed S -count N -rows R -width W
 * Prints `next=` with the first N values of Wp10Next() after Wp10Seed(S),
 * then fills Wp10Rows(R, W) with Wp10FillRows and prints `rows=` with
 * Wp10PrintRows over each row's whole stride, and `stride=`.
 * tests/wp10_support.rs checks that tests/support/kernels.rs draws and hashes
 * the same values, which every kernel test relies on.
 */

#include "wp10_support.h"

#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv)
{
  unsigned long long seed = 1;
  int count = 0, numrows = 1, argi, i;
  JDIMENSION width = 1;
  JSAMPARRAY rows;

  for (argi = 1; argi + 1 < argc; argi += 2) {
    if (!strcmp(argv[argi], "-seed"))
      seed = strtoull(argv[argi + 1], NULL, 10);
    else if (!strcmp(argv[argi], "-count"))
      count = atoi(argv[argi + 1]);
    else if (!strcmp(argv[argi], "-rows"))
      numrows = atoi(argv[argi + 1]);
    else if (!strcmp(argv[argi], "-width"))
      width = (JDIMENSION)atoi(argv[argi + 1]);
    else {
      fprintf(stderr, "unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  Wp10Seed(seed);
  printf("next=");
  for (i = 0; i < count; i++)
    printf("%s%u", i ? "," : "", Wp10Next());
  printf("\n");
  rows = Wp10Rows(numrows, width);
  Wp10FillRows(rows, numrows, width);
  printf("stride=%u\n", Wp10Stride(width));
  Wp10PrintRows("rows", rows, numrows, Wp10Stride(width));
  Wp10FreeRows(rows);
  return 0;
}
