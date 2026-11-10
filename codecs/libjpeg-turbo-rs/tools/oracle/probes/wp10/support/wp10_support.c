/* WP10 probe support; see wp10_support.h. */

#include "wp10_support.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned long long rng_state = 1;

void Wp10Seed(unsigned long long seed)
{
  rng_state = seed ? seed : 1;
}

/* xorshift64 */
unsigned int Wp10Next(void)
{
  rng_state ^= rng_state << 13;
  rng_state ^= rng_state >> 7;
  rng_state ^= rng_state << 17;
  return (unsigned int)(rng_state >> 32);
}

int Wp10Rand(int n)
{
  return (int)(Wp10Next() % (unsigned int)n);
}

JSAMPLE Wp10Sample(void)
{
  return (JSAMPLE)(Wp10Next() >> 24);
}

JDIMENSION Wp10Stride(JDIMENSION samplesperrow)
{
  return (samplesperrow + 63) & ~(JDIMENSION)63;
}

JSAMPARRAY Wp10Rows(int numrows, JDIMENSION samplesperrow)
{
  JDIMENSION stride = Wp10Stride(samplesperrow);
  JSAMPARRAY rows = (JSAMPARRAY)malloc((size_t)numrows * sizeof(JSAMPROW));
  void *block = NULL;
  int row;

  if (rows == NULL ||
      posix_memalign(&block, 32, (size_t)numrows * stride) != 0) {
    fprintf(stderr, "Wp10Rows: out of memory\n");
    exit(2);
  }
  memset(block, 0, (size_t)numrows * stride);
  for (row = 0; row < numrows; row++)
    rows[row] = (JSAMPROW)block + (size_t)row * stride;
  return rows;
}

void Wp10FreeRows(JSAMPARRAY rows)
{
  free(rows[0]);
  free(rows);
}

void Wp10FillRows(JSAMPARRAY rows, int numrows, JDIMENSION samplesperrow)
{
  JDIMENSION stride = Wp10Stride(samplesperrow), col;
  int row;

  for (row = 0; row < numrows; row++)
    for (col = 0; col < stride; col++)
      rows[row][col] = Wp10Sample();
}

unsigned long long Wp10Hash(const void *data, size_t len)
{
  const unsigned char *bytes = (const unsigned char *)data;
  unsigned long long hash = 1469598103934665603ULL;
  size_t i;

  for (i = 0; i < len; i++) {
    hash ^= bytes[i];
    hash *= 1099511628211ULL;
  }
  return hash;
}

void Wp10PrintRows(const char *key, JSAMPARRAY rows, int numrows, size_t len)
{
  int row;

  printf("%s=", key);
  for (row = 0; row < numrows; row++)
    printf("%s%016llx", row ? "," : "", Wp10Hash(rows[row], len));
  printf("\n");
}
