// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Calls real pinned lifecycle/configuration API symbols from the archive.
#include <brotli/decode.h>
#include "dec/state.h"
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// The public routine is C. Keep unknown integral parameters in a C caller,
// avoiding an adapter-created invalid C++ enum load under UBSan.
extern "C" int BrotliInstanceSetParameter(BrotliDecoderState*, uint32_t, uint32_t);

namespace {
struct Block { void* pointer; bool live; };
struct Context {
  size_t calls = 0, fail = 0;
  unsigned seed = 0;
  std::vector<Block> blocks;
  std::ostringstream trace;
  ~Context() { for (const auto& b : blocks) std::free(b.pointer); }
};
void* Allocate(void* opaque, size_t size) {
  auto& c = *static_cast<Context*>(opaque);
  c.trace << " A" << ++c.calls << ':' << size;
  if (c.calls == c.fail) { c.trace << ":0"; return nullptr; }
  void* p = std::malloc(size ? size : 1);
  if (!p) std::abort();
  std::memset(p, static_cast<int>(c.seed), size);
  c.blocks.push_back({p, true});
  c.trace << ":1";
  return p;
}
void Free(void* opaque, void* pointer) {
  auto& c = *static_cast<Context*>(opaque);
  if (!pointer) { c.trace << " F0"; return; }
  for (size_t i = 0; i < c.blocks.size(); ++i) {
    auto& b = c.blocks[i];
    if (b.pointer == pointer) {
      if (!b.live) std::abort();
      c.trace << " F" << i + 1;
      b.live = false;
      return;
    }
  }
  std::abort();
}
void Start(void*, size_t) {}
void Chunk(void*, const uint8_t*, size_t) {}
}  // namespace

int main() {
  std::string line;
  while (std::getline(std::cin, line)) {
    std::istringstream in(line);
    std::string command;
    unsigned pair, seed, metadata;
    size_t fail, count;
    in >> command >> pair >> fail >> seed >> metadata >> count;
    if (command != "bi-create") return 2;
    Context c;
    c.fail = fail;
    c.seed = seed;
    auto* s = BrotliDecoderCreateInstance(
        (pair & 1) ? Allocate : nullptr, (pair & 2) ? Free : nullptr, &c);
    c.trace << " R" << (s != nullptr);
    for (size_t i = 0; i < count; ++i) {
      unsigned parameter, value;
      in >> parameter >> value;
      if (s) {
        c.trace << " P" << BrotliInstanceSetParameter(s, parameter, value);
        c.trace << ':' << s->canny_ringbuffer_allocation << ':' << s->large_window;
      }
    }
    if (s) {
      c.trace << " O" << BrotliDecoderIsUsed(s) << ':' << BrotliDecoderGetErrorCode(s);
      c.trace << ':' << (s->memory_manager_opaque == (pair == 3 ? &c : nullptr));
      const auto start = (metadata & 1) ? Start : nullptr;
      const auto chunk = (metadata & 2) ? Chunk : nullptr;
      BrotliDecoderSetMetadataCallbacks(s, start, chunk, &c);
      c.trace << " M" << (s->metadata_start_func == start) << ':'
              << (s->metadata_chunk_func == chunk) << ':'
              << (s->metadata_callback_opaque == &c);
    }
    BrotliDecoderDestroyInstance(s);
    std::cout << c.trace.str() << '\n';
  }
}
