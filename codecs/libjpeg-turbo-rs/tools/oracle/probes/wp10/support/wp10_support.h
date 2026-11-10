/* WP10 probe support: the inputs and outputs of the kernel probes
 * (ADR 0004, ADR 0008).
 *
 * A kernel probe tools/oracle/probes/wp10/<family>.c calls x86-64 SIMD
 * kernels of the oracle's libjpeg directly, with inputs from the seeded
 * xorshift64 generator below, and prints FNV-1a hashes of the outputs as
 * `key=value` lines. tests/support/kernels.rs generates the same inputs, in
 * the same order, for the port's kernels, and tests/wp10_<family>.rs
 * compares the hashes.
 *
 * Sample rows follow alloc_sarray with WITH_SIMD (ALIGN_SIZE 32): one
 * contiguous block aligned to 32 bytes, each row padded to a multiple of 64
 * samples. The kernels read into that padding and the upsamplers write into
 * it, as they do in a decode.
 */

#ifndef LIBJPEG_TURBO_RS_WP10_SUPPORT_H
#define LIBJPEG_TURBO_RS_WP10_SUPPORT_H

#include <stddef.h>

#include "probe_support.h"

/* Seeds the generator; seed 0 counts as 1, since xorshift64 stays at 0. */
void Wp10Seed(unsigned long long seed);

/* The next 32 random bits: the upper half of the xorshift64 state, as
 * tools/oracle/probes/wp4/idct.c draws them. */
unsigned int Wp10Next(void);

/* A random integer in 0..n-1: Wp10Next() % n. */
int Wp10Rand(int n);

/* A random sample: the top 8 bits of Wp10Next(). */
JSAMPLE Wp10Sample(void);

/* The row stride of alloc_sarray: samplesperrow rounded up to 64. */
JDIMENSION Wp10Stride(JDIMENSION samplesperrow);

/* numrows rows of Wp10Stride(samplesperrow) samples in one zeroed block
 * aligned to 32 bytes; free with Wp10FreeRows. Exits on allocation
 * failure. */
JSAMPARRAY Wp10Rows(int numrows, JDIMENSION samplesperrow);
void Wp10FreeRows(JSAMPARRAY rows);

/* Fills the whole block of Wp10Rows(numrows, samplesperrow), padding
 * included, with Wp10Sample(), row by row. */
void Wp10FillRows(JSAMPARRAY rows, int numrows, JDIMENSION samplesperrow);

/* The FNV-1a hash of len bytes, as tools/oracle/probes/wp6/api.c hashes. */
unsigned long long Wp10Hash(const void *data, size_t len);

/* Prints `key=` and the hashes of `len` bytes of each row, as 16 hex digits
 * each, separated by commas. */
void Wp10PrintRows(const char *key, JSAMPARRAY rows, int numrows, size_t len);

#endif
