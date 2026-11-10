// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Genuine encoder streams; actual public decoder calls, including a public
// callback setter invoked from a callback on the same stable decoder state.
#include <brotli/decode.h>
#include <brotli/encode.h>
#include "dec/state.h"
#include <cstdint>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

namespace {
struct Control;
struct Context { Control* control; unsigned id; };
struct Control {
  BrotliDecoderState* state = nullptr;
  unsigned mode = 0;
  bool changed = false;
  Context a{this, 0}, b{this, 1};
  std::ostringstream trace;
};
void StartA(void*, size_t);
void StartB(void*, size_t);
void ChunkA(void*, const uint8_t*, size_t);
void ChunkB(void*, const uint8_t*, size_t);

void Change(Control& c, bool start, bool chunk) {
  const auto start_fn = start ? StartB : nullptr;
  const auto chunk_fn = chunk ? ChunkB : nullptr;
  BrotliDecoderSetMetadataCallbacks(c.state, start_fn, chunk_fn, &c.b);
  c.changed = true;
  c.trace << " S" << (c.state->metadata_start_func == start_fn)
          << (c.state->metadata_chunk_func == chunk_fn)
          << (c.state->metadata_callback_opaque == &c.b);
}
void Start(void* opaque, size_t size) {
  auto& ctx = *static_cast<Context*>(opaque);
  auto& c = *ctx.control;
  c.trace << " A" << ctx.id << ':' << size << ':' << c.state->meta_block_remaining_len
          << ':' << c.state->br.bit_pos_ << ':' << BrotliDecoderGetErrorCode(c.state);
  if (ctx.id == 0 && !c.changed) {
    if (c.mode == 1) Change(c, true, true);
    if (c.mode == 3) Change(c, false, false);
  }
}
void Chunk(void* opaque, const uint8_t* data, size_t size) {
  auto& ctx = *static_cast<Context*>(opaque);
  auto& c = *ctx.control;
  c.trace << " C" << ctx.id << ':' << size << ':' << c.state->meta_block_remaining_len
          << ':' << c.state->br.bit_pos_ << ':' << BrotliDecoderGetErrorCode(c.state) << ':';
  static constexpr char digit[] = "0123456789abcdef";
  for (size_t i = 0; i < size; ++i) c.trace << digit[data[i] >> 4] << digit[data[i] & 15];
  if (ctx.id == 0 && !c.changed) {
    if (c.mode == 2) Change(c, true, true);
    if (c.mode == 4) Change(c, true, false);
    if (c.mode == 5) Change(c, false, true);
  }
}
void StartA(void* opaque, size_t size) { Start(opaque, size); }
void StartB(void* opaque, size_t size) { Start(opaque, size); }
void ChunkA(void* opaque, const uint8_t* data, size_t size) { Chunk(opaque, data, size); }
void ChunkB(void* opaque, const uint8_t* data, size_t size) { Chunk(opaque, data, size); }

std::vector<uint8_t> Encode(size_t first, size_t second) {
  auto* state = BrotliEncoderCreateInstance(nullptr, nullptr, nullptr);
  if (!state) std::abort();
  if (!BrotliEncoderSetParameter(state, BROTLI_PARAM_QUALITY, 4) ||
      !BrotliEncoderSetParameter(state, BROTLI_PARAM_LGWIN, 16)) std::abort();
  std::vector<uint8_t> output(131072);
  uint8_t* next_out = output.data();
  size_t available_out = output.size();
  for (const auto length : {first, second}) {
    std::vector<uint8_t> metadata(length ? length : 1);
    for (size_t i = 0; i < length; ++i) metadata[i] = static_cast<uint8_t>(i * 97 + length);
    const uint8_t* next_in = metadata.data();
    size_t available_in = length;
    do {
      if (!BrotliEncoderCompressStream(state, BROTLI_OPERATION_EMIT_METADATA,
              &available_in, &next_in, &available_out, &next_out, nullptr)) std::abort();
      if (!available_out) std::abort();
    } while (available_in || BrotliEncoderHasMoreOutput(state));
  }
  const uint8_t empty_input = 0;
  const uint8_t* next_in = &empty_input;
  size_t available_in = 0;
  while (!BrotliEncoderIsFinished(state)) {
    if (!BrotliEncoderCompressStream(state, BROTLI_OPERATION_FINISH,
            &available_in, &next_in, &available_out, &next_out, nullptr)) std::abort();
    if (!available_out) std::abort();
  }
  output.resize(static_cast<size_t>(next_out - output.data()));
  BrotliEncoderDestroyInstance(state);
  return output;
}
}  // namespace

int main() {
  unsigned mode;
  size_t chunk, first, second;
  while (std::cin >> mode >> chunk >> first >> second) {
    if (mode > 5 || first > 16384 || second > 16384) return 2;
    const auto encoded = Encode(first, second);
    Control c;
    c.mode = mode;
    c.state = BrotliDecoderCreateInstance(nullptr, nullptr, nullptr);
    if (!c.state) return 3;
    BrotliDecoderSetMetadataCallbacks(c.state, StartA, ChunkA, &c.a);
    size_t offset = 0, total_out = 123;
    BrotliDecoderResult result;
    do {
      size_t extent = encoded.size() - offset;
      if (chunk && extent > chunk) extent = chunk;
      size_t available_in = extent, available_out = 0;
      const uint8_t* next_in = encoded.data() + offset;
      uint8_t* next_out = nullptr;
      result = BrotliDecoderDecompressStream(c.state, &available_in, &next_in,
          &available_out, &next_out, &total_out);
      const auto consumed = extent - available_in;
      c.trace << " R" << static_cast<unsigned>(result) << ':' << consumed << ':'
              << available_in << ':' << total_out << ':' << c.state->br.bit_pos_ << ':'
              << BrotliDecoderGetErrorCode(c.state) << ':' << BrotliDecoderIsFinished(c.state);
      offset += consumed;
      if (result != BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT) break;
      if (available_in != 0 || offset == encoded.size()) return 4;
    } while (true);
    c.trace << " Z" << (result == BROTLI_DECODER_RESULT_SUCCESS) << ':'
            << offset << ':' << encoded.size() << ':' << total_out;
    BrotliDecoderDestroyInstance(c.state);
    std::cout << c.trace.str() << '\n';
  }
}
