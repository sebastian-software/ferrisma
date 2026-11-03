// Copyright 2009, 2010, 2013-2016 by the Brotli Authors.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
// Observe the real pinned Brotli API. No decoder implementation is copied here.
#include <brotli/decode.h>
#include <brotli/encode.h>
#include "c/common/shared_dictionary_internal.h"
#include "c/common/context.h"
#include "c/common/dictionary.h"
#include "c/common/transform.h"
#include "c/dec/prefix.h"
#include "c/dec/state.h"
#include <cstddef>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

extern "C" int BrotliObserveSetParameter(BrotliDecoderState*, uint32_t, uint32_t);

namespace {
struct Block { void* ptr; size_t size; size_t id; };
struct Allocations {
  size_t calls = 0, fail = 0;
  uint8_t fill = 0xa5;
  std::vector<Block> blocks;
  std::ostringstream events;
};
void* Allocate(void* opaque, size_t size) {
  auto& a = *static_cast<Allocations*>(opaque);
  const size_t id = ++a.calls;
  a.events << " A" << id << ':' << size;
  if (id == a.fail) { a.events << ":0"; return nullptr; }
  void* p = std::malloc(size ? size : 1);
  if (!p) { std::cerr << "observer host allocation failed\n"; std::abort(); }
  std::memset(p, a.fill, size);
  a.blocks.push_back({p, size, id});
  a.events << ":1";
  return p;
}
void Free(void* opaque, void* ptr) {
  auto& a = *static_cast<Allocations*>(opaque);
  if (!ptr) { a.events << " F0"; return; }
  for (auto& b : a.blocks) if (b.ptr == ptr) {
    a.events << " F" << b.id;
    b.ptr = nullptr;
    std::free(ptr);
    return;
  }
  std::cerr << "source freed an unknown observer allocation\n";
  std::abort();
}
std::vector<uint8_t> Bytes(const std::string& hex) {
  std::vector<uint8_t> result;
  if (hex == "-") return result;
  if (hex.size() % 2) throw std::runtime_error("odd hex input");
  for (size_t i = 0; i < hex.size(); i += 2)
    result.push_back(static_cast<uint8_t>(std::stoul(hex.substr(i, 2), nullptr, 16)));
  return result;
}
void Hex(const uint8_t* data, size_t size) {
  static constexpr char digits[] = "0123456789abcdef";
  if (!size) { std::cout << '-'; return; }
  for (size_t i = 0; i < size; ++i)
    std::cout << digits[data[i] >> 4] << digits[data[i] & 15];
}
void State(const BrotliDecoderState* s) {
  // Only fields assigned by StateInit or the reached decoder path are read.
  std::cout << " S" << int(s->state) << ':' << s->error_code << ':' << s->pos
            << ':' << s->buffer_length << ':' << s->br.val_ << ':'
            << s->br.bit_pos_ << ':' << s->ringbuffer_size << ':'
            << s->new_ringbuffer_size << ':' << s->partial_pos_out << ':'
            << s->used_input << ':' << s->window_bits << ':' << s->large_window
            << ':' << s->canny_ringbuffer_allocation << ':'
            << BrotliDecoderIsUsed(s) << ':' << BrotliDecoderIsFinished(s)
            << ':' << BrotliDecoderHasMoreOutput(s);
}
}  // namespace

bool BrotliDecoderCommand(const std::string& command, std::istringstream& in) {
  if (command.rfind("br-", 0) != 0) return false;
  if (command == "br-layout") {
#define LAYOUT(T) std::cout << ' ' << #T << ':' << sizeof(T) << ':' << alignof(T)
#define OFFSET(T, F) std::cout << ' ' << #T "." #F << ':' << offsetof(T, F)
    LAYOUT(BrotliDecoderState); LAYOUT(BrotliBitReader); LAYOUT(HuffmanCode);
    LAYOUT(HuffmanTreeGroup); LAYOUT(BrotliSharedDictionaryInternal);
    LAYOUT(BrotliDecoderCompoundDictionary); LAYOUT(BrotliMetablockHeaderArena);
    LAYOUT(BrotliMetablockBodyArena);
    OFFSET(BrotliDecoderState, br); OFFSET(BrotliDecoderState, alloc_func);
    OFFSET(BrotliDecoderState, buffer); OFFSET(BrotliDecoderState, ringbuffer);
    OFFSET(BrotliDecoderState, literal_hgroup); OFFSET(BrotliDecoderState, mtf);
    OFFSET(BrotliDecoderState, arena); OFFSET(BrotliDecoderState, dictionary);
    OFFSET(BrotliSharedDictionaryInternal, prefix_size);
    OFFSET(BrotliSharedDictionaryInternal, words);
    OFFSET(BrotliSharedDictionaryInternal, transforms_instances);
    OFFSET(HuffmanTreeGroup, codes); OFFSET(HuffmanCode, value);
    std::cout << " VERSION:" << BrotliDecoderVersion();
#undef LAYOUT
#undef OFFSET
    return true;
  }
  if (command == "br-tables") {
    const auto* d = BrotliGetDictionary();
    const auto* t = BrotliGetTransforms();
    Hex(d->data, d->data_size); std::cout << ' ';
    Hex(_kBrotliContextLookupTable, 2048); std::cout << ' ';
    Hex(t->prefix_suffix, t->prefix_suffix_size); std::cout << ' ';
    Hex(t->transforms, 3 * t->num_transforms);
    for (size_t i=0;i<50;++i) std::cout << ' ' << t->prefix_suffix_map[i];
    for (size_t i=0;i<704;++i) {
      const auto& e=kCmdLut[i];
      std::cout << ' ' << unsigned(e.insert_len_extra_bits) << ':'
                << unsigned(e.copy_len_extra_bits) << ':' << int(e.distance_code)
                << ':' << unsigned(e.context) << ':' << e.insert_len_offset
                << ':' << e.copy_len_offset;
    }
    return true;
  }
  if (command == "br-alloc") {
    Allocations a;
    unsigned pair, action; size_t capacity; std::string hex;
    in >> a.fail >> pair >> action >> capacity >> hex;
    auto input = Bytes(hex);
    auto* s = BrotliDecoderCreateInstance(pair & 1 ? Allocate : nullptr,
        pair & 2 ? Free : nullptr, &a);
    std::cout << !!s;
    if (s) {
      State(s);
      if (action == 0) {
        // The source leaves this arena untouched until a metablock reaches it.
        const auto* arena = reinterpret_cast<const uint8_t*>(&s->arena);
        std::cout << " ARENA:";
        if (pair == 3) Hex(arena, sizeof(s->arena));
        else std::cout << '-'; // Default malloc storage is not initialized.
      } else if (action == 1) {
        for (uint32_t parameter : {0u, 1u, 2u})
          for (uint32_t value : {0u, 1u, 2u, 0xffffffffu})
            std::cout << " P" << parameter << ':' << value << ':'
              << BrotliObserveSetParameter(s, parameter, value);
        // The backing input remains alive through DestroyInstance.
        for (size_t i = 0; i < 17; ++i)
          std::cout << " D" << i << ':' << BrotliDecoderAttachDictionary(s,
              BROTLI_SHARED_DICTIONARY_RAW, input.size(), input.data());
        std::cout << " SERIAL:" << BrotliDecoderAttachDictionary(s,
            BROTLI_SHARED_DICTIONARY_SERIALIZED, input.size(), input.data());
        State(s);
      } else if (action == 2) {
        std::vector<uint8_t> output(capacity, 0xcc);
        size_t avail_in = input.size(), avail_out = output.size(), total = 999;
        const uint8_t* next_in = input.data(); uint8_t* next_out = output.data();
        const auto result = BrotliDecoderDecompressStream(s, &avail_in, &next_in,
            &avail_out, &next_out, &total);
        std::cout << " R" << int(result) << ':' << avail_in << ':' << avail_out
                  << ':' << total << ':' << int(BrotliDecoderGetErrorCode(s)) << ':';
        Hex(output.data(), output.size() - avail_out);
        // Losslessly encode the initialized untouched suffix instead of
        // printing a large run of the caller's 0xcc fill on every failure.
        for (size_t i = output.size() - avail_out; i < output.size(); ++i)
          if (output[i] != 0xcc) throw std::runtime_error("source wrote beyond returned output");
        std::cout << ":TAILcc:" << avail_out; State(s);
      } else { throw std::runtime_error("unknown allocation action"); }
      BrotliDecoderDestroyInstance(s);
    }
    for (const auto& b : a.blocks) if (b.ptr)
      throw std::runtime_error("source left a live allocation after destruction");
    std::cout << a.events.str();
    return true;
  }
  if (command == "br-flags-create") {
    unsigned seed; in >> seed;
    Allocations a; a.fill = static_cast<uint8_t>(seed);
    auto* s = BrotliDecoderCreateInstance(Allocate, Free, &a);
    if (!s) throw std::runtime_error("unexpected source constructor failure");
    const size_t offset = offsetof(BrotliDecoderState, new_ringbuffer_size) + sizeof(int);
    std::cout << sizeof(*s) << ':' << offset << ':';
    Hex(reinterpret_cast<const uint8_t*>(s) + offset, 4);
    State(s);
    BrotliDecoderDestroyInstance(s);
    std::cout << a.events.str();
    return true;
  }
  if (command == "br-compress") {
    int quality, window, mode; std::string hex;
    in >> quality >> window >> mode >> hex; auto input = Bytes(hex);
    std::vector<uint8_t> output(BrotliEncoderMaxCompressedSize(input.size()));
    size_t size = output.size();
    const auto ok = BrotliEncoderCompress(quality, window,
        static_cast<BrotliEncoderMode>(mode), input.size(), input.data(), &size, output.data());
    std::cout << ok << ' '; Hex(output.data(), ok ? size : 0);
    return true;
  }
  throw std::runtime_error("unknown Brotli command");
}

#if defined(BROTLI_OBSERVER_STANDALONE)
int main() {
  try {
    std::string line;
    while (std::getline(std::cin, line)) {
      std::istringstream in(line); std::string command; in >> command;
      if (!BrotliDecoderCommand(command, in)) throw std::runtime_error("unknown command");
      if (!in) throw std::runtime_error("malformed observer request");
      std::cout << '\n';
    }
  } catch (const std::exception& e) {
    std::cerr << e.what() << '\n'; return 1;
  }
}
#endif
