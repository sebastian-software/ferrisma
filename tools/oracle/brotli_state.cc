// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual pinned source state/default-allocator routines; no copied bodies.
#include <brotli/decode.h>
extern "C" {
#include "common/platform.h"
}
#include "common/shared_dictionary_internal.h"
#include "dec/state.h"
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace {
struct Block { void* pointer; size_t size; bool live; };
struct Context {
  size_t calls = 0, fail = 0;
  uint8_t seed = 0xa5;
  std::vector<Block> blocks;
  std::ostringstream trace;
  ~Context() { for (const auto& block : blocks) std::free(block.pointer); }
};
void* Allocate(void*, size_t);
void Free(void*, void*);

uintptr_t Pointer(const Context& c, uintptr_t value) {
  if (!value) return 0;
  if (value == reinterpret_cast<uintptr_t>(Allocate)) return 0x1001;
  if (value == reinterpret_cast<uintptr_t>(Free)) return 0x1002;
  if (value == reinterpret_cast<uintptr_t>(&c)) return 0x1003;
  if (value == reinterpret_cast<uintptr_t>(BrotliDefaultAllocFunc)) return 0x1004;
  if (value == reinterpret_cast<uintptr_t>(BrotliDefaultFreeFunc)) return 0x1005;
  if (value == reinterpret_cast<uintptr_t>(BrotliGetDictionary())) return 0x1010;
  if (value == reinterpret_cast<uintptr_t>(BrotliGetTransforms())) return 0x1011;
  if (value == reinterpret_cast<uintptr_t>(BrotliGetDictionary()->data)) return 0x1012;
  if (value == reinterpret_cast<uintptr_t>(BrotliGetTransforms()->prefix_suffix)) return 0x1013;
  if (value == reinterpret_cast<uintptr_t>(BrotliGetTransforms()->prefix_suffix_map)) return 0x1014;
  if (value == reinterpret_cast<uintptr_t>(BrotliGetTransforms()->transforms)) return 0x1015;
  for (size_t i = 0; i < c.blocks.size(); ++i) {
    const auto start = reinterpret_cast<uintptr_t>(c.blocks[i].pointer);
    if (start <= value && value <= start + c.blocks[i].size) {
      return uintptr_t{0x10000000} + (i + 1) * uintptr_t{0x100000} + value - start;
    }
  }
  return value;
}
void Normalize(const Context& c, std::vector<uint8_t>& bytes, size_t offset) {
  uintptr_t value;
  std::memcpy(&value, bytes.data() + offset, sizeof(value));
  value = Pointer(c, value);
  std::memcpy(bytes.data() + offset, &value, sizeof(value));
}
void Hex(std::ostream& out, const uint8_t* bytes, size_t size) {
  constexpr char digit[] = "0123456789abcdef";
  if (!size) out << '-';
  for (size_t i = 0; i < size; ++i) out << digit[bytes[i] >> 4] << digit[bytes[i] & 15];
}
void Snapshot(Context& c) {
  for (size_t id = 0; id < c.blocks.size(); ++id) {
    const auto& block = c.blocks[id];
    if (!block.live) continue;
    if (block.size != sizeof(BrotliDecoderState) &&
        block.size != sizeof(BrotliSharedDictionaryInternal)) continue;
    std::vector<uint8_t> bytes(block.size);
    std::memcpy(bytes.data(), block.pointer, block.size);
    if (id == 0 && block.size == sizeof(BrotliDecoderState)) {
#define P(F) Normalize(c, bytes, offsetof(BrotliDecoderState, F))
      P(alloc_func); P(free_func); P(memory_manager_opaque);
      P(br.next_in); P(br.guard_in); P(br.last_in);
      P(ringbuffer); P(ringbuffer_end); P(htree_command); P(context_lookup);
      P(context_map_slice); P(dist_context_map_slice);
      P(literal_hgroup.htrees); P(literal_hgroup.codes);
      P(insert_copy_hgroup.htrees); P(insert_copy_hgroup.codes);
      P(distance_hgroup.htrees); P(distance_hgroup.codes);
      P(block_type_trees); P(block_len_trees); P(dist_context_map); P(literal_htree);
      P(metadata_start_func); P(metadata_chunk_func); P(metadata_callback_opaque);
      P(context_map); P(context_modes); P(dictionary); P(compound_dictionary);
      P(arena.header.symbol_lists); P(arena.header.next);
#undef P
    } else if (block.size == sizeof(BrotliSharedDictionaryInternal)) {
#define P(F) Normalize(c, bytes, offsetof(BrotliSharedDictionaryInternal, F))
      for (size_t i = 0; i < 15; ++i) P(prefix[i]);
      for (size_t i = 0; i < 64; ++i) { P(words[i]); P(transforms[i]); }
      P(words_instances); P(transforms_instances); P(prefix_suffix_maps);
      P(alloc_func); P(free_func); P(memory_manager_opaque);
#undef P
    }
    c.trace << " B" << id + 1 << ':';
    Hex(c.trace, bytes.data(), bytes.size());
  }
}
void* Allocate(void* opaque, size_t size) {
  auto& c = *static_cast<Context*>(opaque);
  c.trace << " A" << ++c.calls << ':' << size;
  Snapshot(c); // Observe outer field writes BEFORE this allocation returns.
  if (c.calls == c.fail) { c.trace << ":0"; return nullptr; }
  auto* p = std::malloc(size ? size : 1);
  if (!p) std::abort();
  std::memset(p, c.seed, size);
  c.blocks.push_back({p, size, true});
  c.trace << ":1";
  return p;
}
void Free(void* opaque, void* pointer) {
  auto& c = *static_cast<Context*>(opaque);
  if (!pointer) {
    c.trace << " F0";
    Snapshot(c);
    return;
  }
  for (size_t i = 0; i < c.blocks.size(); ++i) {
    if (c.blocks[i].pointer != pointer) continue;
    if (!c.blocks[i].live) std::abort();
    c.trace << " F" << i + 1;
    Snapshot(c); // Exact prior pointer bytes remain visible inside free.
    c.blocks[i].live = false;
    // Quarantine fixture blocks until Context destruction so pointer identity
    // is unambiguous even for source-defined dangling pointer representations.
    return;
  }
  std::abort();
}
}

int main() {
  std::string line;
  while (std::getline(std::cin, line)) {
    std::istringstream in(line);
    std::string command;
    in >> command;
    if (command == "bs-state") {
      Context c;
      unsigned seed, action, group;
      brotli_reg_t maximum, limit, ntrees;
      in >> c.fail >> seed >> action >> group >> maximum >> limit >> ntrees;
      if (!in || seed > 255 || action > 3 || group > 2) return 2;
      c.seed = static_cast<uint8_t>(seed);
      auto* s = BrotliDecoderCreateInstance(Allocate, Free, &c);
      c.trace << " C" << !!s;
      if (s) {
        Snapshot(c);
        if (action == 1) BrotliDecoderStateMetablockBegin(s);
        if (action == 2) {
          HuffmanTreeGroup* g = group == 0 ? &s->literal_hgroup :
              group == 1 ? &s->insert_copy_hgroup : &s->distance_hgroup;
          const auto success = BrotliDecoderHuffmanTreeGroupInit(s, g, maximum, limit, ntrees);
          c.trace << " H" << !!success;
        }
        if (action == 3) BrotliDecoderStateCleanupAfterMetablock(s);
        Snapshot(c);
        BrotliDecoderDestroyInstance(s);
      }
      for (const auto& b : c.blocks) if (b.live) std::abort();
      std::cout << c.trace.str() << '\n';
    } else if (command == "bs-layout") {
      Context c;
      std::cout << alignof(max_align_t) << ' '
          << sizeof(BrotliDictionary) << ':' << alignof(BrotliDictionary) << ' '
          << sizeof(BrotliTransforms) << ':' << alignof(BrotliTransforms) << ' ';
      std::vector<uint8_t> dictionary(sizeof(BrotliDictionary));
      std::memcpy(dictionary.data(), BrotliGetDictionary(), dictionary.size());
      Normalize(c, dictionary, offsetof(BrotliDictionary, data));
      Hex(std::cout, dictionary.data(), dictionary.size());
      std::cout << ' ';
      std::vector<uint8_t> transforms(sizeof(BrotliTransforms));
      std::memcpy(transforms.data(), BrotliGetTransforms(), transforms.size());
      Normalize(c, transforms, offsetof(BrotliTransforms, prefix_suffix));
      Normalize(c, transforms, offsetof(BrotliTransforms, prefix_suffix_map));
      Normalize(c, transforms, offsetof(BrotliTransforms, transforms));
      Normalize(c, transforms, offsetof(BrotliTransforms, params));
      Hex(std::cout, transforms.data(), transforms.size());
      std::cout << '\n';
    } else if (command == "bs-default") {
      size_t size;
      unsigned fill, stored;
      in >> size >> fill >> stored;
      if (!in || fill > 255 || stored > 1) return 2;
      void* opaque = &size;
      auto alloc = BrotliDefaultAllocFunc;
      auto free = BrotliDefaultFreeFunc;
      BrotliDecoderState* s = nullptr;
      if (stored) {
        s = BrotliDecoderCreateInstance(nullptr, nullptr, opaque);
        if (!s) return 3;
        std::cout << (s->alloc_func == alloc) << ':' << (s->free_func == free)
            << ':' << (s->memory_manager_opaque == nullptr)
            << ':' << (s->dictionary->alloc_func == alloc)
            << ':' << (s->dictionary->free_func == free)
            << ':' << (s->dictionary->memory_manager_opaque == opaque) << ' ';
        alloc = s->alloc_func;
        free = s->free_func;
      }
      void* p = alloc(opaque, size);
      std::cout << !!p;
      if (p) {
        if (size > 4096) std::abort();
        std::cout << ':' << (reinterpret_cast<uintptr_t>(p) % alignof(max_align_t));
        std::memset(p, fill, size);
        std::cout << ':';
        Hex(std::cout, static_cast<uint8_t*>(p), size);
        free(opaque, p);
      }
      free(opaque, nullptr);
      BrotliDecoderDestroyInstance(s);
      std::cout << '\n';
    } else {
      return 2;
    }
  }
  return 0;
}
