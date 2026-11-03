/* Copyright 2015 Google Inc. All Rights Reserved.
   Distributed under the MIT license in licenses/BROTLI-MIT.txt. */
/* Perform actual assignments/reads of the pinned source's declared bitfields. */
#include "c/dec/state.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void Assign(BrotliDecoderState* s, unsigned field, unsigned value) {
  switch (field) {
    case 0: s->is_last_metablock = value; break;
    case 1: s->is_uncompressed = value; break;
    case 2: s->is_metadata = value; break;
    case 3: s->should_wrap_ringbuffer = value; break;
    case 4: s->canny_ringbuffer_allocation = value; break;
    case 5: s->large_window = value; break;
    case 6: s->window_bits = value; break;
    case 7: s->size_nibbles = value; break;
    default: abort();
  }
}
static unsigned Read(const BrotliDecoderState* s, unsigned field) {
  switch (field) {
    case 0: return s->is_last_metablock;
    case 1: return s->is_uncompressed;
    case 2: return s->is_metadata;
    case 3: return s->should_wrap_ringbuffer;
    case 4: return s->canny_ringbuffer_allocation;
    case 5: return s->large_window;
    case 6: return s->window_bits;
    case 7: return s->size_nibbles;
    default: abort();
  }
}
int main(int argc, char** argv) {
  unsigned seed, count;
  const int initialized = !(argc == 2 && strcmp(argv[1], "--uninitialized") == 0);
  while (scanf("%u %u", &seed, &count) == 2) {
    BrotliDecoderState state;
    unsigned step;
    /* Every byte is initialized by the caller before actual source fields
       are assigned. Preserve and compare all other object bytes as well. */
    if (initialized) memset(&state, seed, sizeof(state));
    printf("%zu %zu", sizeof(state), offsetof(BrotliDecoderState, new_ringbuffer_size) + sizeof(int));
    for (step = 0; step < count; ++step) {
      unsigned field, value;
      size_t i;
      const unsigned char* bytes = (const unsigned char*)&state;
      if (scanf("%u %u", &field, &value) != 2) return 2;
      Assign(&state, field, value);
      printf(" %u:", Read(&state, field));
      for (i = 0; initialized && i < sizeof(state); ++i) {
        const size_t offset = offsetof(BrotliDecoderState, new_ringbuffer_size) + sizeof(int);
        if (i >= offset && i < offset + 4) printf("%02x", bytes[i]);
        else if (bytes[i] != (unsigned char)seed) {
          fputs("source bitfield touched a byte outside the observed word\n", stderr);
          return 4;
        }
      }
    }
    putchar('\n');
  }
  return ferror(stdin) ? 3 : 0;
}
