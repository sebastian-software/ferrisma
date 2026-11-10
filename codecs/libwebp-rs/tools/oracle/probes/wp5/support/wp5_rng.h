// WP5 probe support: SplitMix64, the generator that the probes and
// tests/wp5_*.rs share. Both sides draw the same values in the same order, so the
// generated inputs need not be stored anywhere.

#ifndef LIBWEBP_RS_WP5_RNG_H_
#define LIBWEBP_RS_WP5_RNG_H_

#include <stdint.h>

typedef struct {
  uint64_t state;
} Wp5Rng;

// Returns the next 64-bit value.
uint64_t Wp5RngNext(Wp5Rng* rng);

// Returns a value in [0, bound); bound must be positive.
uint64_t Wp5RngBelow(Wp5Rng* rng, uint64_t bound);

#endif  // LIBWEBP_RS_WP5_RNG_H_
