// WP5 probe support: SplitMix64, see wp5_rng.h.

#include "wp5_rng.h"

uint64_t Wp5RngNext(Wp5Rng* rng) {
  uint64_t z;
  rng->state += 0x9e3779b97f4a7c15ULL;
  z = rng->state;
  z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
  z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
  return z ^ (z >> 31);
}

uint64_t Wp5RngBelow(Wp5Rng* rng, uint64_t bound) {
  return Wp5RngNext(rng) % bound;
}
