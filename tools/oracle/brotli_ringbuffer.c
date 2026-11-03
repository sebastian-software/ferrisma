/* Independent helper observer; codec algorithm bodies are included pristine. */
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "c/dec/decode.c"

typedef struct {
  void* pointer;
  size_t size;
  int live;
} Block;
typedef struct {
  Block blocks[8];
  size_t count;
  unsigned calls;
  unsigned fail;
  unsigned seed;
  BrotliDecoderState* state;
} Context;
static unsigned Token(Context* c, const void* p) {
  size_t i;
  if (!p) return 0;
  for (i = 0; i < c->count; i++) if (p == c->blocks[i].pointer) return (unsigned)i + 1;
  abort();
}
static void* Allocate(void* opaque, size_t size) {
  Context* c = (Context*)opaque;
  void* p;
  ++c->calls;
  printf(" A%u:%zu", c->calls, size);
  if (c->state) printf(":%u:%d:%d", Token(c, c->state->ringbuffer), c->state->ringbuffer_size, c->state->new_ringbuffer_size);
  if (c->calls == c->fail) { printf(":0"); return NULL; }
  p = malloc(size ? size : 1);
  if (!p || c->count == 8) abort();
  memset(p, (int)c->seed, size);
  c->blocks[c->count].pointer = p;
  c->blocks[c->count].size = size;
  c->blocks[c->count].live = 1;
  ++c->count;
  printf(":%u", Token(c, p));
  return p;
}
static void Free(void* opaque, void* p) {
  Context* c = (Context*)opaque;
  unsigned token = Token(c, p);
  printf(" F%u", token);
  if (c->state) printf(":%u:%d:%d", Token(c, c->state->ringbuffer), c->state->ringbuffer_size, c->state->new_ringbuffer_size);
  if (token) {
    if (!c->blocks[token - 1].live) abort();
    c->blocks[token - 1].live = 0;
  }
}
static void Bytes(const uint8_t* p, size_t n) {
  size_t i;
  for (i = 0; i < n; ++i) printf("%02x", p[i]);
}
int main(void) {
  unsigned op, seed, fail, window, canny, wrap, state;
  int old_size, new_size, pos, remaining, force, target, error;
  size_t rounds, partial, capacity, consumed;
  while (scanf("%u %u %u %u %u %d %d %d %d %zu %zu %zu %d %d %u %d %u %zu", &op, &seed, &fail, &window, &canny, &old_size, &new_size, &pos, &remaining, &rounds, &partial, &capacity, &force, &target, &state, &error, &wrap, &consumed) == 18) {
    Context c;
    BrotliDecoderState* s;
    size_t i, available = capacity, total = 777, output_size = capacity;
    int result = 777;
    uint8_t *output, *next, *reference = NULL;
    const uint8_t* taken = NULL;
    memset(&c, 0, sizeof(c)); c.seed = seed; c.fail = fail;
    s = BrotliDecoderCreateInstance(Allocate, Free, &c);
    if (!s) { puts(" X"); goto release; }
    c.state = s;
    if (old_size) {
      s->new_ringbuffer_size = old_size;
      if (!BrotliEnsureRingBuffer(s)) { printf(" X"); c.state = NULL; BrotliDecoderDestroyInstance(s); puts(""); goto release; }
    }
    s->new_ringbuffer_size = new_size;
    s->pos = pos; s->meta_block_remaining_len = remaining;
    s->rb_roundtrips = rounds; s->partial_pos_out = partial;
    s->window_bits = window; s->canny_ringbuffer_allocation = canny;
    s->should_wrap_ringbuffer = wrap; if (op == 0) s->is_metadata = force; s->state = (BrotliRunningState)state;
    s->error_code = error;
    s->used_input = UINT64_MAX - 1;
    /* SaveErrorCode reads these only when its internal buffer is nonempty. */
    s->buffer_length = wrap;
    s->br.next_in = (const uint8_t*)s->buffer.u8;
    s->br.last_in = s->br.next_in + (target != 0);
    output = (uint8_t*)malloc(capacity ? capacity : 1);
    if (!output) abort();
    memset(output, (int)seed, capacity); next = output;
    switch (op) {
      case 0: BrotliCalculateRingBufferSize(s); break;
      case 1: result = BrotliEnsureRingBuffer(s); break;
      case 2: total = UnwrittenBytes(s, wrap); break;
      case 3:
        result = WriteRingBuffer(s, &available, target == 0 ? NULL : target == 1 ? &reference : &next, &total, force);
        break;
      case 4: WrapRingBuffer(s); break;
      case 5: taken = BrotliDecoderTakeOutput(s, &output_size); break;
      case 6: result = BrotliDecoderHasMoreOutput(s); total = (size_t)BrotliDecoderIsFinished(s); break;
      case 7: result = (int)SaveErrorCode(s, error, consumed); break;
      default: abort();
    }
    printf(" R%d:%zu:%zu:%zu:%zu:%d:%d:%d:%zu:%zu:%u:%d:%" PRIu64 ":%" PRIu64, result, available, total, output_size, (size_t)(next - output), s->ringbuffer_size, s->new_ringbuffer_size, s->pos, s->rb_roundtrips, s->partial_pos_out, s->should_wrap_ringbuffer, s->error_code, s->used_input, (uint64_t)s->buffer_length);
    printf(" P%u:", Token(&c, s->ringbuffer));
    if (s->ringbuffer) printf("%zu", (size_t)(s->ringbuffer_end - s->ringbuffer)); else printf("-");
    printf(" N"); if (reference) printf("%zu", (size_t)(reference - s->ringbuffer)); else printf("-");
    printf(" T"); if (taken) { printf("%zu:", (size_t)(taken - s->ringbuffer)); Bytes(taken, output_size); } else printf("-");
    printf(" O"); Bytes(output, capacity);
    printf(" B"); if (s->ringbuffer) Bytes(s->ringbuffer, (size_t)s->ringbuffer_size + kRingBufferWriteAheadSlack);
    free(output);
    /* No callback may inspect the already released outer block; quarantine
       keeps all allocation identities unique until fixture cleanup. */
    c.state = NULL;
    BrotliDecoderDestroyInstance(s);
    puts("");
release:
    for (i = 0; i < c.count; ++i) { if (c.blocks[i].live) abort(); free(c.blocks[i].pointer); }
  }
  return 0;
}
