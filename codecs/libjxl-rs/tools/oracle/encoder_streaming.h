// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observation helpers of encoder step 2 for the epub-run and epub-wrap
// drivers (encoder_public.cc): a recording JxlEncoderOutputProcessor and a
// recording JxlChunkedFrameInputSource. They implement no encoder algorithm;
// they log every callback the original makes, in order, so that the Rust
// driver (tests/support/encoder_public.rs, block "encoder streaming") can be
// compared with it call for call. Both drivers implement the same schedules
// and the same lossless run-length notation of the log.
#ifndef LIBJXL_RS_ORACLE_ENCODER_STREAMING_H_
#define LIBJXL_RS_ORACLE_ENCODER_STREAMING_H_

#include <jxl/encode.h>

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

namespace encoder_streaming {

// Run-length notation of a callback log: a repeated unit of one to three
// events is written "[a,b,c]*k" ("a*k" for one event), choosing at each
// point the unit length that covers the most events (the shortest on a
// tie); events are separated by '.', so that a log is one word of the
// space-separated observation. Lossless; event tokens contain none of the
// characters ".[],*".
inline std::string Compress(const std::vector<std::string>& events) {
  std::string out;
  size_t i = 0;
  while (i < events.size()) {
    size_t best_len = 1, best_k = 1;
    for (size_t len = 1; len <= 3; ++len) {
      if (i + len > events.size()) break;
      size_t k = 1;
      while (i + (k + 1) * len <= events.size() &&
             std::equal(events.begin() + i, events.begin() + i + len,
                        events.begin() + i + k * len)) {
        ++k;
      }
      if (k >= 2 && k * len > best_k * best_len) {
        best_len = len;
        best_k = k;
      }
    }
    if (!out.empty()) out += '.';
    if (best_k == 1) {
      out += events[i];
    } else if (best_len == 1) {
      out += events[i] + "*" + std::to_string(best_k);
    } else {
      out += "[";
      for (size_t j = 0; j < best_len; ++j) out += (j ? "," : "") + events[i + j];
      out += "]*" + std::to_string(best_k);
    }
    i += best_len * best_k;
  }
  return out.empty() ? "-" : out;
}

// A recording output processor. `mask` leaves callbacks out (1 get_buffer,
// 2 release_buffer, 4 seek, 8 set_finalized_position). Buffer schedules: 0
// returns every byte from the position to the end of the grown file (as
// encode_test's JxlStreamingAdapter with large buffers), 1 one byte (its
// small buffers), 2 the suggested size, 3 at most 3, 64 and 4096 bytes in
// turn. The `stop_at`-th get_buffer call (1-based, 0 never) returns a stop
// request: a null pointer (`stop_kind` 0) or a size of 0 (1). Log tokens:
// "g<suggested>><given>" ("g=" when the suggestion is the previous one less
// the previous given size), "r<n>", "k<pos>" and "f<pos>" ("f=" at the
// processor's position).
struct Processor {
  std::vector<uint8_t> file;
  size_t position = 0;
  size_t finalized = 0;
  uint32_t schedule = 0;
  size_t stop_at = 0;
  uint32_t stop_kind = 0;
  size_t calls = 0;
  size_t last_suggested = 0, last_given = 0;
  std::vector<std::string> events;

  static void* GetBuffer(void* opaque, size_t* size) {
    auto& p = *static_cast<Processor*>(opaque);
    ++p.calls;
    std::string e = p.calls > 1 && *size + p.last_given == p.last_suggested
                        ? std::string("g=>")
                        : "g" + std::to_string(*size) + ">";
    p.last_suggested = *size;
    p.last_given = 0;
    if (p.stop_at != 0 && p.calls == p.stop_at) {
      if (p.stop_kind == 0) {
        p.events.push_back(e + "~");
        return nullptr;
      }
      *size = 0;
      p.events.push_back(e + "0");
      if (p.file.size() < p.position + 1) p.file.resize(p.position + 1, 0xDA);
      return p.file.data() + p.position;
    }
    if (p.schedule == 1) {
      *size = 1;
    } else if (p.schedule == 3) {
      static const size_t kCycle[3] = {3, 64, 4096};
      *size = std::min(*size, kCycle[p.calls % 3]);
    }
    if (p.position + *size > p.file.size()) p.file.resize(p.position + *size, 0xDA);
    if (p.schedule == 0) *size = p.file.size() - p.position;
    p.last_given = *size;
    p.events.push_back(e + std::to_string(*size));
    return p.file.data() + p.position;
  }
  static void ReleaseBuffer(void* opaque, size_t written_bytes) {
    auto& p = *static_cast<Processor*>(opaque);
    p.events.push_back("r" + std::to_string(written_bytes));
    p.position += written_bytes;
  }
  static void Seek(void* opaque, uint64_t position) {
    auto& p = *static_cast<Processor*>(opaque);
    p.events.push_back("k" + std::to_string(position));
    p.position = size_t(position);
  }
  static void SetFinalizedPosition(void* opaque, uint64_t finalized_position) {
    auto& p = *static_cast<Processor*>(opaque);
    p.events.push_back(finalized_position == p.position ? std::string("f=")
                                                         : "f" + std::to_string(finalized_position));
    p.finalized = size_t(finalized_position);
  }

  JxlEncoderOutputProcessor Callbacks(uint32_t mask) {
    JxlEncoderOutputProcessor o;
    o.opaque = this;
    o.get_buffer = (mask & 1) ? nullptr : &GetBuffer;
    o.release_buffer = (mask & 2) ? nullptr : &ReleaseBuffer;
    o.seek = (mask & 4) ? nullptr : &Seek;
    o.set_finalized_position = (mask & 8) ? nullptr : &SetFinalizedPosition;
    return o;
  }

  // The log since the last report, the positions and the whole file.
  template <typename HexFn>
  std::string Report(HexFn hex) {
    std::string out = Compress(events) + ";" + std::to_string(position) + "," +
                      std::to_string(finalized) + "," + std::to_string(file.size()) + ";" +
                      hex(file.data(), file.size());
    events.clear();
    return out;
  }
};

// A recording chunked frame input source over whole-image buffers whose
// rows are `pad` bytes longer than the pixels. Every callback is logged:
// "F" color format, "E<ec>" extra-channel format, "c<x>,<y>,<xs>,<ys>" and
// "e<ec>,<x>,<y>,<xs>,<ys>" data requests, "R<tag><offset>" releases (tag c
// for color, e<ec> for an extra channel, offset from that buffer's start).
// The `fail_at`-th data request (1-based, 0 never) returns null, as does a
// request outside the image ("!") or for a missing extra channel.
struct Source {
  JxlPixelFormat color_format;
  JxlPixelFormat ec_format;
  size_t xsize = 0, ysize = 0;
  size_t color_stride = 0, ec_stride = 0;
  std::vector<uint8_t> color;
  std::vector<std::vector<uint8_t>> ec;
  size_t fail_at = 0;
  size_t requests = 0;
  std::vector<std::string> events;

  static size_t BytesPerPixel(const JxlPixelFormat& f) {
    size_t bytes = f.data_type == JXL_TYPE_UINT8 ? 1 : f.data_type == JXL_TYPE_FLOAT ? 4 : 2;
    return f.num_channels * bytes;
  }
  // Rows of `row` bytes from contiguous `data`, each followed by `pad` bytes
  // of 0xEE.
  static std::vector<uint8_t> Padded(const std::vector<uint8_t>& data, size_t row, size_t ysize,
                                     size_t pad) {
    std::vector<uint8_t> out((row + pad) * ysize + 1, 0xEE);
    for (size_t y = 0; y < ysize; ++y) {
      std::copy(data.begin() + y * row, data.begin() + (y + 1) * row,
                out.begin() + y * (row + pad));
    }
    return out;
  }
  bool Inside(size_t x, size_t y, size_t xs, size_t ys) const {
    return x <= xsize && xs <= xsize - x && y <= ysize && ys <= ysize - y;
  }
  const void* Request(std::string e, const std::vector<uint8_t>* buffer, size_t stride,
                      size_t bpp, size_t x, size_t y, size_t xs, size_t ys, size_t* row_offset) {
    ++requests;
    if (fail_at != 0 && requests == fail_at) {
      events.push_back(e + "~");
      return nullptr;
    }
    if (buffer == nullptr || !Inside(x, y, xs, ys)) {
      events.push_back(e + "!");
      return nullptr;
    }
    events.push_back(e);
    *row_offset = stride;
    return buffer->data() + y * stride + x * bpp;
  }
  static void ColorFormat(void* opaque, JxlPixelFormat* pixel_format) {
    auto& s = *static_cast<Source*>(opaque);
    s.events.push_back("F");
    *pixel_format = s.color_format;
  }
  static const void* ColorAt(void* opaque, size_t x, size_t y, size_t xs, size_t ys,
                             size_t* row_offset) {
    auto& s = *static_cast<Source*>(opaque);
    std::string e = "c" + std::to_string(x) + "," + std::to_string(y) + "," +
                    std::to_string(xs) + "," + std::to_string(ys);
    return s.Request(e, &s.color, s.color_stride, BytesPerPixel(s.color_format), x, y, xs, ys,
                     row_offset);
  }
  static void ExtraFormat(void* opaque, size_t ec_index, JxlPixelFormat* pixel_format) {
    auto& s = *static_cast<Source*>(opaque);
    s.events.push_back("E" + std::to_string(ec_index));
    *pixel_format = s.ec_format;
  }
  static const void* ExtraAt(void* opaque, size_t ec_index, size_t x, size_t y, size_t xs,
                             size_t ys, size_t* row_offset) {
    auto& s = *static_cast<Source*>(opaque);
    std::string e = "e" + std::to_string(ec_index) + "," + std::to_string(x) + "," +
                    std::to_string(y) + "," + std::to_string(xs) + "," + std::to_string(ys);
    return s.Request(e, ec_index < s.ec.size() ? &s.ec[ec_index] : nullptr, s.ec_stride,
                     BytesPerPixel(s.ec_format), x, y, xs, ys, row_offset);
  }
  static void Release(void* opaque, const void* buf) {
    auto& s = *static_cast<Source*>(opaque);
    const uint8_t* p = static_cast<const uint8_t*>(buf);
    std::string e = "R?";
    if (p >= s.color.data() && p < s.color.data() + s.color.size()) {
      e = "Rc" + std::to_string(p - s.color.data());
    }
    for (size_t i = 0; i < s.ec.size(); ++i) {
      if (p >= s.ec[i].data() && p < s.ec[i].data() + s.ec[i].size()) {
        e = "Re" + std::to_string(i) + ":" + std::to_string(p - s.ec[i].data());
      }
    }
    s.events.push_back(e);
  }
  JxlChunkedFrameInputSource Callbacks() {
    return JxlChunkedFrameInputSource{this, &ColorFormat, &ColorAt, &ExtraFormat, &ExtraAt,
                                      &Release};
  }
};

}  // namespace encoder_streaming

#endif  // LIBJXL_RS_ORACLE_ENCODER_STREAMING_H_
