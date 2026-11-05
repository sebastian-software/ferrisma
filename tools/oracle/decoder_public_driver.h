// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The public decoder history driver of the dpub-run protocol. It calls only the
// public API declared by the pinned lib/include/jxl headers, so the same driver
// replays histories on the original JxlDecoder (decoder_public.cc) and through
// the exported C ABI of either the original or the Rust library
// (decoder_abi.cc). It implements no decoder or container algorithm. The Rust
// driver in tests/support/decoder_public.rs mirrors this protocol token for
// token.
//
// The parallel runner operations '@' and '#' call the runner functions that
// the pinned thread_parallel_runner.h and resizable_parallel_runner.h declare.
#ifndef TOOLS_ORACLE_DECODER_PUBLIC_DRIVER_H_
#define TOOLS_ORACLE_DECODER_PUBLIC_DRIVER_H_

#include <jxl/cms.h>
#include <jxl/codestream_header.h>
#include <jxl/color_encoding.h>
#include <jxl/decode.h>
#include <jxl/resizable_parallel_runner.h>
#include <jxl/thread_parallel_runner.h>
#include <jxl/types.h>

#include <algorithm>
#include <atomic>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <map>
#include <ostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

namespace decoder_public {

// The hexadecimal protocol encoding of main.cc, with "-" for no bytes.
struct ByteSpan {
  const uint8_t* data;
  size_t size;
};
inline ByteSpan Bytes(const uint8_t* data, size_t size) { return {data, size}; }
inline std::string Hex(ByteSpan bytes) {
  static constexpr char kDigits[] = "0123456789abcdef";
  if (bytes.size == 0) return "-";
  std::string result;
  for (size_t i = 0; i < bytes.size; ++i) {
    result += kDigits[bytes.data[i] >> 4];
    result += kDigits[bytes.data[i] & 15];
  }
  return result;
}
inline std::vector<uint8_t> Unhex(const std::string& hex) {
  if (hex == "-") return {};
  if (hex.size() % 2 != 0) throw std::runtime_error("odd hex length");
  std::vector<uint8_t> bytes;
  for (size_t i = 0; i < hex.size(); i += 2) {
    bytes.push_back(static_cast<uint8_t>(std::stoul(hex.substr(i, 2), nullptr, 16)));
  }
  return bytes;
}

inline uint64_t Next(uint64_t& seed) {
  seed ^= seed << 13;
  seed ^= seed >> 7;
  seed ^= seed << 17;
  return seed;
}

inline std::string F32(float value) {
  uint32_t bits;
  memcpy(&bits, &value, 4);
  return std::to_string(bits);
}

inline std::string F64(double value) {
  uint64_t bits;
  memcpy(&bits, &value, 8);
  return std::to_string(bits);
}


// ---------------------------------------------------------------------------
// Public decoder histories.

struct Allocator {
  size_t calls = 0, fail = 0, next = 0;
  std::map<void*, size_t> live;
  std::vector<std::string> events;
  static void* Alloc(void* opaque, size_t size) {
    auto& a = *static_cast<Allocator*>(opaque);
    ++a.calls;
    // The first request is the decoder object, whose size is a property of
    // the language layout rather than of the decoder algorithm.
    a.events.push_back(a.calls == 1 ? std::string("Adec") : "A" + std::to_string(size));
    if (a.calls == a.fail) return nullptr;
    void* p = std::malloc(size);
    if (p) a.live[p] = ++a.next;
    return p;
  }
  static void Free(void* opaque, void* p) {
    auto& a = *static_cast<Allocator*>(opaque);
    a.events.push_back("F" + std::to_string(p ? a.live.at(p) : 0));
    a.live.erase(p);
    std::free(p);
  }
};

// cs.wp.prim.tf.gamma-bits.ri, with fixed custom white point and primaries,
// or x:cs:wp:wx:wy:prim:rx:ry:gx:gy:bx:by:tf:gamma:ri with every double as
// its bit pattern.
inline JxlColorEncoding ParseEncoding(const std::string& token) {
  if (token.rfind("x:", 0) == 0) {
    std::vector<uint64_t> f;
    std::istringstream fields(token.substr(2));
    std::string field;
    while (std::getline(fields, field, ':')) f.push_back(std::stoull(field));
    if (f.size() != 14) throw std::runtime_error("color encoding token");
    auto d = [](uint64_t bits) {
      double v;
      memcpy(&v, &bits, 8);
      return v;
    };
    JxlColorEncoding color;
    memset(&color, 0, sizeof(color));
    color.color_space = JxlColorSpace(f[0]);
    color.white_point = JxlWhitePoint(f[1]);
    color.white_point_xy[0] = d(f[2]);
    color.white_point_xy[1] = d(f[3]);
    color.primaries = JxlPrimaries(f[4]);
    color.primaries_red_xy[0] = d(f[5]);
    color.primaries_red_xy[1] = d(f[6]);
    color.primaries_green_xy[0] = d(f[7]);
    color.primaries_green_xy[1] = d(f[8]);
    color.primaries_blue_xy[0] = d(f[9]);
    color.primaries_blue_xy[1] = d(f[10]);
    color.transfer_function = JxlTransferFunction(f[11]);
    color.gamma = d(f[12]);
    color.rendering_intent = JxlRenderingIntent(f[13]);
    return color;
  }
  std::istringstream fields(token);
  uint32_t cs, wp, prim, tf, ri;
  uint64_t gamma_bits;
  char dot;
  fields >> cs >> dot >> wp >> dot >> prim >> dot >> tf >> dot >> gamma_bits >> dot >> ri;
  if (!fields) throw std::runtime_error("color encoding token");
  JxlColorEncoding color;
  memset(&color, 0, sizeof(color));
  color.color_space = JxlColorSpace(cs);
  color.white_point = JxlWhitePoint(wp);
  color.white_point_xy[0] = 0.3127;
  color.white_point_xy[1] = 0.329;
  color.primaries = JxlPrimaries(prim);
  color.primaries_red_xy[0] = 0.68;
  color.primaries_red_xy[1] = 0.32;
  color.primaries_green_xy[0] = 0.265;
  color.primaries_green_xy[1] = 0.69;
  color.primaries_blue_xy[0] = 0.15;
  color.primaries_blue_xy[1] = 0.06;
  color.transfer_function = JxlTransferFunction(tf);
  memcpy(&color.gamma, &gamma_bits, 8);
  color.rendering_intent = JxlRenderingIntent(ri);
  return color;
}

class Driver {
 public:
  Driver(std::vector<uint8_t> file, std::ostream& out) : file_(std::move(file)), out_(out) {}
  ~Driver() {
    if (dec_) JxlDecoderDestroy(dec_);
    DestroyRunner();
  }

  // mm 0: default manager; 1: custom manager with its event log; 2: custom
  // manager without the log, for owners whose allocation paths differ.
  bool Create(int mm, size_t fail) {
    allocator_.fail = fail;
    print_events_ = mm == 1;
    JxlMemoryManager manager{&allocator_, Allocator::Alloc, Allocator::Free};
    dec_ = JxlDecoderCreate(mm ? &manager : nullptr);
    out_ << " create:" << (dec_ ? 1 : 0);
    return dec_ != nullptr;
  }

  // std::map destroys its nodes in an unspecified, standard-library-dependent
  // order (libc++ 21 post-order, libc++ 22 key order). Releases within one
  // destruction checkpoint are therefore compared in ascending allocation
  // order; the decoder object's own release stays last.
  void SortReleases(size_t begin, size_t end) {
    std::sort(allocator_.events.begin() + begin, allocator_.events.begin() + end,
              [](const std::string& a, const std::string& b) {
                return std::stoul(a.substr(1)) < std::stoul(b.substr(1));
              });
  }

  void Destroy() {
    size_t begin = allocator_.events.size();
    JxlDecoderDestroy(dec_);
    if (allocator_.events.size() > begin) SortReleases(begin, allocator_.events.size() - 1);
    dec_ = nullptr;
    out_ << " E";
    if (print_events_)
      for (const auto& e : allocator_.events) out_ << ' ' << e;
  }

  void Op(const std::string& op) {
    char kind = op[0];
    std::string arg = op.substr(1);
    out_ << ' ' << kind << ':';
    switch (kind) {
      case 's': out_ << JxlDecoderSubscribeEvents(dec_, std::stoi(arg)); break;
      case 'k': out_ << JxlDecoderSetKeepOrientation(dec_, std::stoi(arg)); break;
      case 'u': out_ << JxlDecoderSetUnpremultiplyAlpha(dec_, std::stoi(arg)); break;
      case 'r': out_ << JxlDecoderSetRenderSpotcolors(dec_, std::stoi(arg)); break;
      case 'g': out_ << JxlDecoderSetCoalescing(dec_, std::stoi(arg)); break;
      case 'd': out_ << JxlDecoderSetDecompressBoxes(dec_, std::stoi(arg)); break;
      case 'I': SetInput(arg); break;
      case 'i': out_ << Release(); break;
      case 'c': JxlDecoderCloseInput(dec_); closed_ = true; out_ << 0; break;
      case 'p': {
        // p1: ProcessInputIgnoreBoxes of decode_test.cc.
        JxlDecoderStatus status = JxlDecoderProcessInput(dec_);
        out_ << status;
        while (arg == "1" && status == JXL_DEC_BOX) {
          status = JxlDecoderProcessInput(dec_);
          out_ << ',' << status;
        }
        break;
      }
      case 'h': out_ << JxlDecoderSizeHintBasicInfo(dec_); break;
      case 'b': BasicInfo(arg != "0"); break;
      case 'e': ExtraInfo(std::stoul(arg)); break;
      case 'n': ExtraName(arg); break;
      case 'E': Encoded(arg); break;
      case 'z': IccSize(arg); break;
      case 'Z': Icc(arg); break;
      case 't': BoxType(std::stoi(arg)); break;
      case 'y': BoxSize(false, arg != "0"); break;
      case 'Y': BoxSize(true, arg != "0"); break;
      case 'O': SetBox(std::stoul(arg)); break;
      case 'o': ReleaseBox(); break;
      case 'w': {
        size_t begin = allocator_.events.size();
        JxlDecoderRewind(dec_);
        SortReleases(begin, allocator_.events.size());
        Restart();
        out_ << 0;
        break;
      }
      case 'x': {
        size_t begin = allocator_.events.size();
        JxlDecoderReset(dec_);
        SortReleases(begin, allocator_.events.size());
        Restart();
        out_ << 0;
        break;
      }
      case 'C': out_ << JxlDecoderSetCms(dec_, *JxlGetDefaultCms()); break;
      case 'T': {
        uint32_t bits = uint32_t(std::stoul(arg));
        float target;
        memcpy(&target, &bits, 4);
        out_ << JxlDecoderSetDesiredIntensityTarget(dec_, target);
        break;
      }
      case 'P': {
        if (arg == "-") {
          out_ << JxlDecoderSetPreferredColorProfile(dec_, nullptr);
        } else {
          JxlColorEncoding color = ParseEncoding(arg);
          out_ << JxlDecoderSetPreferredColorProfile(dec_, &color);
        }
        break;
      }
      case 'Q': {
        if (arg == "~") {
          out_ << JxlDecoderSetOutputColorProfile(dec_, nullptr, nullptr, 0);
          break;
        }
        std::vector<uint8_t> icc = Unhex(arg);
        icc.push_back(0);  // A non-null pointer for an empty profile.
        out_ << JxlDecoderSetOutputColorProfile(dec_, nullptr, icc.data(), icc.size() - 1);
        break;
      }
      case 'R': {
        auto comma = arg.find(',');
        JxlColorEncoding color = ParseEncoding(arg.substr(0, comma));
        if (arg.substr(comma + 1) == "~") {
          out_ << JxlDecoderSetOutputColorProfile(dec_, &color, nullptr, 0);
          break;
        }
        std::vector<uint8_t> icc = Unhex(arg.substr(comma + 1));
        icc.push_back(0);
        out_ << JxlDecoderSetOutputColorProfile(dec_, &color, icc.data(), icc.size() - 1);
        break;
      }
      case 'J': SetJpeg(std::stoul(arg)); break;
      case 'j': ReleaseJpeg(); break;
      case 'G': out_ << 0; Info(); Color(); break;
      case 'F': SetFormat(arg); break;
      case 'V': extras_ = uint32_t(std::stoul(arg)); out_ << 0; break;
      case 'M': policy_ = std::stoi(arg); out_ << 0; break;
      case 'S': JxlDecoderSkipFrames(dec_, std::stoul(arg)); out_ << 0; break;
      case 'D': out_ << JxlDecoderSetProgressiveDetail(dec_, JxlProgressiveDetail(std::stoi(arg))); break;
      case 'W': SetDepth(arg); break;
      case 'K': out_ << JxlDecoderSkipCurrentFrame(dec_); break;
      case 'f': out_ << JxlDecoderFlushImage(dec_); Outputs(); break;
      case 'L': Loop(arg); break;
      // Operations of the decode_test.cc histories.
      case 'q': BufferSize(arg); break;
      case 'U': out_ << 0; SetOutputs(false); break;
      case 'H': out_ << 0; SetOutputs(true); break;
      case 'X': out_ << 0; Outputs(); break;
      case 'N': out_ << 0; FrameInfo(); break;
      case 'm': out_ << JxlDecoderGetIntendedDownsamplingRatio(dec_); break;
      case 'l': ReleaseAndContinue(arg); break;
      case '!': out_ << JxlSignatureCheck(file_.data(), std::stoul(arg)); break;
      case 'a': GrowJpegOp(arg); break;
      case 'v': GrowBoxOp(arg); break;
      case '~': {
        out_ << retired_.size();
        for (const auto& buffer : retired_) out_ << ',' << Hex(Bytes(buffer.data(), buffer.size()));
        break;
      }
      case '@': out_ << SetRunner(arg); break;
      case '#': out_ << ResizeRunner(); break;
      case 'A': StreamLoop(arg); break;
      case 'B': ProgressionLoop(arg); break;
      default: throw std::runtime_error("decoder public op");
    }
  }

 private:
  void Restart() {
    pos_ = 0;
    set_len_ = 0;
    closed_ = false;
    if (!box_.empty() || box_set_) {
      out_ << Hex(Bytes(box_.data(), box_.size())) << ',';
      box_.clear();
      box_set_ = false;
    }
    // Rewind and Reset forget the JPEG output buffer. The caller keeps it
    // alive; '~' prints whether a later decode wrote to it.
    if (!jpeg_.empty()) {
      out_ << Hex(Bytes(jpeg_.data(), jpeg_.size())) << ',';
      retired_.push_back(std::move(jpeg_));
      jpeg_.clear();
    }
    a_avail_ = 0;
    a_total_ = 0;
  }

  void SetInput(const std::string& arg) {
    if (arg == "-") {
      out_ << JxlDecoderSetInput(dec_, nullptr, 0);
      return;
    }
    if (arg == "+" || arg == "h" || arg[0] == '@') {
      // From the current position: the rest of the file, the basic-info size
      // hint, or up to an absolute end.
      size_t len = file_.size() - pos_;
      if (arg == "h") len = std::min(len, JxlDecoderSizeHintBasicInfo(dec_));
      if (arg[0] == '@') {
        size_t end = std::stoul(arg.substr(1));
        if (end < pos_ || end > file_.size()) throw std::runtime_error("input end");
        len = end - pos_;
      }
      JxlDecoderStatus status = JxlDecoderSetInput(dec_, file_.data() + pos_, len);
      if (status == JXL_DEC_SUCCESS) set_len_ = len;
      out_ << status;
      return;
    }
    auto comma = arg.find(',');
    size_t off = std::stoul(arg.substr(0, comma)), len = std::stoul(arg.substr(comma + 1));
    if (off > file_.size() || len > file_.size() - off) throw std::runtime_error("input range");
    JxlDecoderStatus status = JxlDecoderSetInput(dec_, file_.data() + off, len);
    if (status == JXL_DEC_SUCCESS) {
      pos_ = off;
      set_len_ = len;
    }
    out_ << status;
  }

  size_t Release() {
    size_t remaining = JxlDecoderReleaseInput(dec_);
    pos_ = pos_ + set_len_ - remaining;
    set_len_ = 0;
    return remaining;
  }

  void BasicInfo(bool present) {
    JxlBasicInfo info;
    memset(&info, 0xA5, sizeof(info));
    JxlDecoderStatus status = JxlDecoderGetBasicInfo(dec_, present ? &info : nullptr);
    out_ << status;
    if (status != JXL_DEC_SUCCESS || !present) return;
    out_ << ',' << info.have_container << ',' << info.xsize << ',' << info.ysize << ','
         << info.bits_per_sample << ',' << info.exponent_bits_per_sample << ','
         << F32(info.intensity_target) << ',' << F32(info.min_nits) << ','
         << info.relative_to_max_display << ',' << F32(info.linear_below) << ','
         << info.uses_original_profile << ',' << info.have_preview << ','
         << info.have_animation << ',' << info.orientation << ','
         << info.num_color_channels << ',' << info.num_extra_channels << ','
         << info.alpha_bits << ',' << info.alpha_exponent_bits << ','
         << info.alpha_premultiplied << ',' << info.preview.xsize << ','
         << info.preview.ysize << ',' << info.animation.tps_numerator << ','
         << info.animation.tps_denominator << ',' << info.animation.num_loops << ','
         << info.animation.have_timecodes << ',' << info.intrinsic_xsize << ','
         << info.intrinsic_ysize << ',' << Hex(Bytes(info.padding, sizeof(info.padding)));
  }

  void ExtraInfo(size_t index) {
    JxlExtraChannelInfo info;
    memset(&info, 0xA5, sizeof(info));
    JxlDecoderStatus status = JxlDecoderGetExtraChannelInfo(dec_, index, &info);
    out_ << status;
    if (status != JXL_DEC_SUCCESS) return;
    out_ << ',' << info.type << ',' << info.bits_per_sample << ','
         << info.exponent_bits_per_sample << ',' << info.dim_shift << ',' << info.name_length
         << ',' << info.alpha_premultiplied;
    for (float v : info.spot_color) out_ << ',' << F32(v);
    out_ << ',' << info.cfa_channel;
  }

  void ExtraName(const std::string& arg) {
    auto comma = arg.find(',');
    size_t index = std::stoul(arg.substr(0, comma)), size = std::stoul(arg.substr(comma + 1));
    std::vector<char> name(size + 1, char(0xA5));
    JxlDecoderStatus status = JxlDecoderGetExtraChannelName(dec_, index, name.data(), size);
    out_ << status << ',' << Hex(Bytes(reinterpret_cast<uint8_t*>(name.data()), size));
  }

  void Encoded(const std::string& arg) {
    bool present = arg.size() < 2;
    JxlColorEncoding color;
    memset(&color, 0xA5, sizeof(color));
    JxlDecoderStatus status = JxlDecoderGetColorAsEncodedProfile(
        dec_, JxlColorProfileTarget(arg[0] - '0'), present ? &color : nullptr);
    out_ << status;
    if (status != JXL_DEC_SUCCESS || !present) return;
    out_ << ',' << color.color_space << ',' << color.white_point << ','
         << F64(color.white_point_xy[0]) << ',' << F64(color.white_point_xy[1]) << ','
         << color.primaries << ',' << F64(color.primaries_red_xy[0]) << ','
         << F64(color.primaries_red_xy[1]) << ',' << F64(color.primaries_green_xy[0]) << ','
         << F64(color.primaries_green_xy[1]) << ',' << F64(color.primaries_blue_xy[0]) << ','
         << F64(color.primaries_blue_xy[1]) << ',' << color.transfer_function << ','
         << F64(color.gamma) << ',' << color.rendering_intent;
  }

  void IccSize(const std::string& arg) {
    bool present = arg.size() < 2;
    size_t size = 0xA5A5;
    JxlDecoderStatus status = JxlDecoderGetICCProfileSize(
        dec_, JxlColorProfileTarget(arg[0] - '0'), present ? &size : nullptr);
    out_ << status << ',' << size;
  }

  void Icc(const std::string& arg) {
    auto comma = arg.find(',');
    int target = std::stoi(arg.substr(0, comma));
    size_t size = 0;
    if (arg.substr(comma + 1) == "?") {
      JxlDecoderGetICCProfileSize(dec_, JxlColorProfileTarget(target), &size);
    } else {
      size = std::stoul(arg.substr(comma + 1));
    }
    std::vector<uint8_t> icc(size + 1, 0xA5);
    JxlDecoderStatus status =
        JxlDecoderGetColorAsICCProfile(dec_, JxlColorProfileTarget(target), icc.data(), size);
    out_ << status << ',' << Hex(Bytes(icc.data(), icc.size()));
  }

  void BoxType(int decompressed) {
    JxlBoxType type = {'?', '?', '?', '?'};
    JxlDecoderStatus status = JxlDecoderGetBoxType(dec_, type, decompressed);
    out_ << status << ',' << Hex(Bytes(reinterpret_cast<uint8_t*>(type), 4));
  }

  void BoxSize(bool contents, bool present) {
    uint64_t size = 0xA5A5;
    JxlDecoderStatus status = contents
                                  ? JxlDecoderGetBoxSizeContents(dec_, present ? &size : nullptr)
                                  : JxlDecoderGetBoxSizeRaw(dec_, present ? &size : nullptr);
    out_ << status << ',' << size;
  }

  void SetBox(size_t size) {
    // A pending caller buffer keeps its bytes; a new one replaces it only if
    // the decoder accepts it, as a caller would retain its own buffer.
    std::vector<uint8_t> buffer(size + 1, 0xA5);
    JxlDecoderStatus status = JxlDecoderSetBoxBuffer(dec_, buffer.data(), size);
    out_ << status;
    if (status == JXL_DEC_SUCCESS) {
      box_ = std::move(buffer);
      box_set_ = true;
    }
  }

  void SetJpeg(size_t size) {
    std::vector<uint8_t> buffer(size + 1, 0xA5);
    JxlDecoderStatus status = JxlDecoderSetJPEGBuffer(dec_, buffer.data(), size);
    out_ << status;
    if (status == JXL_DEC_SUCCESS) {
      jpeg_ = std::move(buffer);
    }
  }

  void ReleaseJpeg() {
    size_t remaining = JxlDecoderReleaseJPEGBuffer(dec_);
    out_ << remaining << ',' << Hex(Bytes(jpeg_.data(), jpeg_.size()));
    jpeg_.clear();
  }

  void ReleaseBox() {
    size_t remaining = JxlDecoderReleaseBoxBuffer(dec_);
    out_ << remaining << ',' << Hex(Bytes(box_.data(), box_.size()));
    box_.clear();
    box_set_ = false;
  }

  void Info() {
    out_ << ' ';
    BasicInfo(true);
    JxlBasicInfo info;
    if (JxlDecoderGetBasicInfo(dec_, &info) != JXL_DEC_SUCCESS) return;
    for (uint32_t i = 0; i < info.num_extra_channels; ++i) {
      out_ << " e:";
      ExtraInfo(i);
      JxlExtraChannelInfo ec;
      if (JxlDecoderGetExtraChannelInfo(dec_, i, &ec) == JXL_DEC_SUCCESS) {
        out_ << " n:";
        ExtraName(std::to_string(i) + "," + std::to_string(ec.name_length + 1));
      }
    }
  }

  void Color() {
    for (int target = 0; target < 2; ++target) {
      out_ << " E:";
      Encoded(std::to_string(target));
      out_ << " z:";
      IccSize(std::to_string(target));
      size_t size = 0;
      if (JxlDecoderGetICCProfileSize(dec_, JxlColorProfileTarget(target), &size) ==
          JXL_DEC_SUCCESS) {
        out_ << " Z:";
        Icc(std::to_string(target) + "," + std::to_string(size));
      }
    }
  }

  // ---- Pixel output observation. Pixel data and callback sequences are
  // compared through a 64-bit FNV-1a digest over little-endian words.
  static void Fnv(uint64_t& h, const uint8_t* p, size_t n) {
    for (size_t i = 0; i < n; ++i) {
      h ^= p[i];
      h *= 1099511628211ull;
    }
  }
  static void FnvWord(uint64_t& h, uint64_t v) {
    uint8_t bytes[8];
    for (int i = 0; i < 8; ++i) bytes[i] = uint8_t(v >> (8 * i));
    Fnv(h, bytes, 8);
  }
  static uint64_t Digest(const std::vector<uint8_t>& v) {
    uint64_t h = 14695981039346656037ull;
    Fnv(h, v.data(), v.size());
    return h;
  }
  size_t PixelBytes() const {
    size_t bpc = format_.data_type == JXL_TYPE_UINT8   ? 1
                 : format_.data_type == JXL_TYPE_FLOAT ? 4
                                                       : 2;
    return format_.num_channels * bpc;
  }
  void Record(size_t thread, size_t x, size_t y, size_t n, const void* pixels) {
    ++cb_calls_;
    FnvWord(cb_hash_, thread);
    FnvWord(cb_hash_, x);
    FnvWord(cb_hash_, y);
    FnvWord(cb_hash_, n);
    Fnv(cb_hash_, static_cast<const uint8_t*>(pixels), n * PixelBytes());
  }
  static void SimpleCallback(void* opaque, size_t x, size_t y, size_t n, const void* pixels) {
    static_cast<Driver*>(opaque)->Record(0, x, y, n, pixels);
  }
  static void* InitCallback(void* opaque, size_t threads, size_t pixels) {
    auto* d = static_cast<Driver*>(opaque);
    ++d->cb_inits_;
    FnvWord(d->cb_hash_, threads);
    FnvWord(d->cb_hash_, pixels);
    return opaque;
  }
  static void RunCallback(void* opaque, size_t thread, size_t x, size_t y, size_t n,
                          const void* pixels) {
    static_cast<Driver*>(opaque)->Record(thread, x, y, n, pixels);
  }
  static void DestroyCallback(void* opaque) { ++static_cast<Driver*>(opaque)->cb_destroys_; }

  void SetFormat(const std::string& arg) {
    std::istringstream fields(arg);
    uint32_t channels, type, endian, mode;
    size_t align;
    char dot;
    fields >> channels >> dot >> type >> dot >> endian >> dot >> align >> dot >> mode;
    if (!fields) throw std::runtime_error("format token");
    format_ = {channels, JxlDataType(type), JxlEndianness(endian), align};
    out_mode_ = int(mode);
    out_ << 0;
  }

  void SetDepth(const std::string& arg) {
    std::istringstream fields(arg);
    uint32_t type, bits, exponent;
    char dot;
    fields >> type >> dot >> bits >> dot >> exponent;
    if (!fields) throw std::runtime_error("bit depth token");
    depth_ = {JxlBitDepthType(type), bits, exponent};
    depth_set_ = true;
    out_ << 0;
  }

  bool SetOutputs(bool preview) {
    if (out_mode_ == 3) return false;
    size_t size = 0;
    JxlDecoderStatus s = preview ? JxlDecoderPreviewOutBufferSize(dec_, &format_, &size)
                                 : JxlDecoderImageOutBufferSize(dec_, &format_, &size);
    out_ << " q:" << s << ',' << size;
    if (s != JXL_DEC_SUCCESS) return false;
    if (!preview && out_mode_ == 4) {
      // The pixel callback of decode_test.cc: rows land in a caller buffer
      // at their stride, so the result does not depend on the call order.
      JxlBasicInfo info;
      if (JxlDecoderGetBasicInfo(dec_, &info) != JXL_DEC_SUCCESS) return false;
      test_bpp_ = PixelBytes();
      test_stride_ = test_bpp_ * info.xsize;
      if (format_.align > 1) {
        test_stride_ = (test_stride_ + format_.align - 1) / format_.align * format_.align;
      }
      image_.assign(size, 0);
      s = JxlDecoderSetImageOutCallback(dec_, &format_, TestCallback, this);
    } else if (!preview && out_mode_ == 5) {
      // The multithreaded callback under a runner: rows land in the caller
      // buffer as in mode 4, so the result does not depend on the call order.
      // Init arguments enter the hash in call order (init never overlaps a
      // run call), run calls are counted, and a thread id outside the latest
      // init's thread count is recorded.
      JxlBasicInfo info;
      if (JxlDecoderGetBasicInfo(dec_, &info) != JXL_DEC_SUCCESS) return false;
      test_bpp_ = PixelBytes();
      test_stride_ = test_bpp_ * info.xsize;
      if (format_.align > 1) {
        test_stride_ = (test_stride_ + format_.align - 1) / format_.align * format_.align;
      }
      image_.assign(size, 0);
      s = JxlDecoderSetMultithreadedImageOutCallback(dec_, &format_, BufferInitCallback,
                                                     BufferRunCallback, DestroyCallback, this);
    } else if (preview || out_mode_ == 0) {
      image_.assign(size + 1, 0xA5);
      s = preview ? JxlDecoderSetPreviewOutBuffer(dec_, &format_, image_.data(), size)
                  : JxlDecoderSetImageOutBuffer(dec_, &format_, image_.data(), size);
    } else if (out_mode_ == 1) {
      image_.clear();
      s = JxlDecoderSetImageOutCallback(dec_, &format_, SimpleCallback, this);
    } else {
      image_.clear();
      s = JxlDecoderSetMultithreadedImageOutCallback(dec_, &format_, InitCallback, RunCallback,
                                                     DestroyCallback, this);
    }
    out_ << " Q:" << s;
    if (preview) return true;
    extra_.resize(std::max<size_t>(extra_.size(), extras_));
    for (uint32_t i = 0; i < extras_; ++i) {
      size_t es = 0;
      s = JxlDecoderExtraChannelBufferSize(dec_, &format_, &es, i);
      out_ << " v:" << s << ',' << es;
      if (s != JXL_DEC_SUCCESS) continue;
      extra_[i].assign(es + 1, 0xA5);
      out_ << ',' << JxlDecoderSetExtraChannelBuffer(dec_, &format_, extra_[i].data(), es, i);
    }
    if (depth_set_) out_ << " W:" << JxlDecoderSetImageOutBitDepth(dec_, &depth_);
    return true;
  }

  void Outputs() {
    out_ << " o:" << image_.size() << ',' << Digest(image_);
    if (test_overflow_) out_ << ",overflow";
    for (uint32_t i = 0; i < extras_ && i < extra_.size(); ++i) {
      out_ << ',' << extra_[i].size() << ',' << Digest(extra_[i]);
    }
    out_ << ",cb" << cb_calls_ << ',' << cb_inits_ << ',' << cb_destroys_ << ',' << cb_hash_;
    if (out_mode_ == 5) out_ << ",mt" << mt_calls_.load() << ',' << int(mt_bad_thread_.load());
  }

  void FrameInfo() {
    JxlFrameHeader header;
    memset(&header, 0xA5, sizeof(header));
    JxlDecoderStatus s = JxlDecoderGetFrameHeader(dec_, &header);
    out_ << " h:" << s;
    if (s != JXL_DEC_SUCCESS) return;
    out_ << ',' << header.duration << ',' << header.timecode << ',' << header.name_length << ','
         << header.is_last << ',' << header.layer_info.have_crop << ','
         << header.layer_info.crop_x0 << ',' << header.layer_info.crop_y0 << ','
         << header.layer_info.xsize << ',' << header.layer_info.ysize << ','
         << header.layer_info.blend_info.blendmode << ','
         << header.layer_info.blend_info.source << ',' << header.layer_info.blend_info.alpha
         << ',' << header.layer_info.blend_info.clamp << ','
         << header.layer_info.save_as_reference;
    std::vector<char> name(header.name_length + 2, char(0xA5));
    s = JxlDecoderGetFrameName(dec_, name.data(), header.name_length + 1);
    out_ << " N:" << s << ','
         << Hex(Bytes(reinterpret_cast<uint8_t*>(name.data()), name.size()));
    JxlBasicInfo info;
    if (JxlDecoderGetBasicInfo(dec_, &info) != JXL_DEC_SUCCESS) return;
    for (uint32_t i = 0; i <= info.num_extra_channels; ++i) {
      JxlBlendInfo blend;
      memset(&blend, 0xA5, sizeof(blend));
      s = JxlDecoderGetExtraChannelBlendInfo(dec_, i, &blend);
      out_ << " B:" << s;
      if (s == JXL_DEC_SUCCESS)
        out_ << ',' << blend.blendmode << ',' << blend.source << ',' << blend.alpha << ','
             << blend.clamp;
    }
  }

  // L<seed>,<max>,<boxbuf>,<steps>,<close>: the caller loop of a typical
  // incremental application with deterministic chunk sizes.
  void Loop(const std::string& arg) {
    std::istringstream fields(arg);
    uint64_t seed;
    size_t max, boxbuf, steps, close;
    char comma;
    fields >> seed >> comma >> max >> comma >> boxbuf >> comma >> steps >> comma >> close;
    if (!fields || max == 0) throw std::runtime_error("loop arguments");
    out_ << 0;
    for (size_t step = 0; step < steps; ++step) {
      JxlDecoderStatus status = JxlDecoderProcessInput(dec_);
      out_ << " P" << status;
      if (status == JXL_DEC_NEED_MORE_INPUT) {
        size_t remaining = Release();
        out_ << ',' << remaining;
        size_t start = pos_;
        if (start >= file_.size()) {
          if (close && !closed_) {
            JxlDecoderCloseInput(dec_);
            closed_ = true;
            out_ << ",c";
            continue;
          }
          break;
        }
        size_t chunk = 1 + Next(seed) % max;
        size_t len = std::min(file_.size() - start, remaining + chunk);
        JxlDecoderStatus set = JxlDecoderSetInput(dec_, file_.data() + start, len);
        if (set == JXL_DEC_SUCCESS) set_len_ = len;
        out_ << ',' << set << ',' << len;
      } else if (status == JXL_DEC_BOX) {
        out_ << ' ';
        BoxType(0);
        out_ << ' ';
        BoxType(1);
        out_ << ' ';
        BoxSize(false, true);
        out_ << ' ';
        BoxSize(true, true);
        if (boxbuf) {
          if (box_set_) {
            out_ << " o:";
            ReleaseBox();
          }
          out_ << " O:";
          SetBox(boxbuf);
        }
      } else if (status == JXL_DEC_BOX_NEED_MORE_OUTPUT) {
        out_ << " o:";
        ReleaseBox();
        out_ << " O:";
        SetBox(boxbuf ? boxbuf : 1);
      } else if (status == JXL_DEC_BOX_COMPLETE) {
        if (box_set_) {
          out_ << " o:";
          ReleaseBox();
        }
      } else if (status == JXL_DEC_BASIC_INFO) {
        Info();
      } else if (status == JXL_DEC_COLOR_ENCODING) {
        Color();
      } else if (status == JXL_DEC_JPEG_RECONSTRUCTION) {
        // The event carries no getter; decoding continues.
      } else if (status == JXL_DEC_FRAME) {
        ++frames_;
        FrameInfo();
      } else if (status == JXL_DEC_NEED_IMAGE_OUT_BUFFER) {
        if ((policy_ & 1) && frames_ % 2 == 0) {
          out_ << " K:" << JxlDecoderSkipCurrentFrame(dec_);
          continue;
        }
        if (!SetOutputs(false)) break;
      } else if (status == JXL_DEC_NEED_PREVIEW_OUT_BUFFER) {
        if (!SetOutputs(true)) break;
      } else if (status == JXL_DEC_FULL_IMAGE || status == JXL_DEC_PREVIEW_IMAGE) {
        Outputs();
      } else if (status == JXL_DEC_FRAME_PROGRESSION) {
        out_ << " r:" << JxlDecoderGetIntendedDownsamplingRatio(dec_);
        if (policy_ & 2) {
          out_ << " f:" << JxlDecoderFlushImage(dec_);
          Outputs();
        }
        if ((policy_ & 4) && progressions_++ == 0) {
          out_ << " K:" << JxlDecoderSkipCurrentFrame(dec_);
        }
      } else if (status == JXL_DEC_JPEG_NEED_MORE_OUTPUT) {
        out_ << " j:";
        size_t size = jpeg_.empty() ? 1 : (jpeg_.size() - 1);
        ReleaseJpeg();
        out_ << " J:";
        SetJpeg(size);
      } else {
        if (status == JXL_DEC_SUCCESS && box_set_) {
          out_ << " o:";
          ReleaseBox();
        }
        break;
      }
    }
  }

  // ---- decode_test.cc operations.

  // q: image out buffer size; q1: preview; q2,<i>: extra channel i.
  void BufferSize(const std::string& arg) {
    size_t size = 0xA5A5;
    JxlDecoderStatus s;
    if (arg.empty()) {
      s = JxlDecoderImageOutBufferSize(dec_, &format_, &size);
    } else if (arg == "1") {
      s = JxlDecoderPreviewOutBufferSize(dec_, &format_, &size);
    } else {
      s = JxlDecoderExtraChannelBufferSize(dec_, &format_, &size,
                                           uint32_t(std::stoul(arg.substr(2))));
    }
    out_ << s << ',' << size;
  }

  // l[<end>]: release the input, then continue from the new position up to
  // an absolute end, by default the end of the file.
  void ReleaseAndContinue(const std::string& arg) {
    size_t remaining = Release();
    size_t end = arg.empty() ? file_.size() : std::stoul(arg);
    if (end < pos_ || end > file_.size()) throw std::runtime_error("input end");
    JxlDecoderStatus status = JxlDecoderSetInput(dec_, file_.data() + pos_, end - pos_);
    if (status == JXL_DEC_SUCCESS) set_len_ = end - pos_;
    out_ << remaining << ',' << status;
  }

  void SetGrowJpeg(size_t size) {
    grow_jpeg_.assign(size, 0);
    JxlDecoderStatus status = JxlDecoderSetJPEGBuffer(dec_, grow_jpeg_.data(), size);
    grow_jpeg_set_ = status == JXL_DEC_SUCCESS;
    out_ << status;
  }
  void GrowJpeg() {
    size_t remaining = JxlDecoderReleaseJPEGBuffer(dec_);
    size_t used = grow_jpeg_.size() - remaining;
    grow_jpeg_.resize(grow_jpeg_.size() * 2);
    JxlDecoderStatus status =
        JxlDecoderSetJPEGBuffer(dec_, grow_jpeg_.data() + used, grow_jpeg_.size() - used);
    grow_jpeg_set_ = status == JXL_DEC_SUCCESS;
    out_ << remaining << ',' << status << ',' << grow_jpeg_.size();
  }
  void ReleaseGrowJpeg() {
    size_t remaining = JxlDecoderReleaseJPEGBuffer(dec_);
    size_t used = grow_jpeg_.size() - remaining;
    grow_jpeg_set_ = false;
    out_ << remaining << ',' << Hex(Bytes(grow_jpeg_.data(), used));
  }
  // a<size>: set a new growing JPEG buffer; a+: extend it; a-: release it.
  void GrowJpegOp(const std::string& arg) {
    if (arg == "+") GrowJpeg();
    else if (arg == "-") ReleaseGrowJpeg();
    else SetGrowJpeg(std::stoul(arg));
  }

  void SetGrowBox(size_t size) {
    grow_box_.resize(size);
    out_ << JxlDecoderSetBoxBuffer(dec_, grow_box_.data(), grow_box_.size());
  }
  void GrowBox() {
    size_t remaining = JxlDecoderReleaseBoxBuffer(dec_);
    size_t used = grow_box_.size() - remaining;
    grow_box_.resize(grow_box_.size() * 2);
    out_ << remaining << ','
         << JxlDecoderSetBoxBuffer(dec_, grow_box_.data() + used, grow_box_.size() - used);
  }
  void ReleaseGrowBox() {
    size_t remaining = JxlDecoderReleaseBoxBuffer(dec_);
    size_t used = grow_box_.size() - remaining;
    out_ << remaining << ',' << Hex(Bytes(grow_box_.data(), used));
    grow_box_.clear();
  }
  // v<size>: set a growing box buffer; v+: extend it; v-: release it.
  void GrowBoxOp(const std::string& arg) {
    if (arg == "+") GrowBox();
    else if (arg == "-") ReleaseGrowBox();
    else SetGrowBox(std::stoul(arg));
  }

  // @0: JxlDecoderSetParallelRunner(dec, nullptr, nullptr); @1: the thread
  // runner with min(hardware, 16) workers, as DecodeWithAPI; @2: the
  // resizable runner; @3: the thread runner with the default worker count;
  // @t<n>: the thread runner with n workers; @r<n>: the resizable runner
  // after JxlResizableParallelRunnerSetThreads(n). The Rust driver uses the
  // translated runners with the same arguments.
  JxlDecoderStatus SetRunner(const std::string& arg) {
    DestroyRunner();
    if (arg[0] == 't' || arg[0] == 'r') {
      size_t threads = std::stoul(arg.substr(1));
      if (arg[0] == 'r') {
        runner_kind_ = 2;
        runner_ = JxlResizableParallelRunnerCreate(nullptr);
        JxlResizableParallelRunnerSetThreads(runner_, threads);
        return JxlDecoderSetParallelRunner(dec_, JxlResizableParallelRunner, runner_);
      }
      runner_kind_ = 1;
      runner_ = JxlThreadParallelRunnerCreate(nullptr, threads);
      return JxlDecoderSetParallelRunner(dec_, JxlThreadParallelRunner, runner_);
    }
    int kind = std::stoi(arg);
    runner_kind_ = kind;
    if (kind == 0) return JxlDecoderSetParallelRunner(dec_, nullptr, nullptr);
    if (kind == 2) {
      runner_ = JxlResizableParallelRunnerCreate(nullptr);
      return JxlDecoderSetParallelRunner(dec_, JxlResizableParallelRunner, runner_);
    }
    size_t threads = JxlThreadParallelRunnerDefaultNumWorkerThreads();
    if (kind == 1) threads = std::min<size_t>(threads, 16);
    runner_ = JxlThreadParallelRunnerCreate(nullptr, threads);
    return JxlDecoderSetParallelRunner(dec_, JxlThreadParallelRunner, runner_);
  }
  // #: size the resizable runner for the image, as DecodeWithAPI does.
  int ResizeRunner() {
    JxlBasicInfo info;
    if (runner_kind_ != 2 || JxlDecoderGetBasicInfo(dec_, &info) != JXL_DEC_SUCCESS) return 1;
    JxlResizableParallelRunnerSetThreads(
        runner_, JxlResizableParallelRunnerSuggestThreads(info.xsize, info.ysize));
    return 0;
  }
  void DestroyRunner() {
    if (!runner_) return;
    if (runner_kind_ == 2) JxlResizableParallelRunnerDestroy(runner_);
    else JxlThreadParallelRunnerDestroy(runner_);
    runner_ = nullptr;
  }

  static void TestCallback(void* opaque, size_t x, size_t y, size_t n, const void* pixels) {
    auto* d = static_cast<Driver*>(opaque);
    size_t offset = d->test_stride_ * y + d->test_bpp_ * x, bytes = n * d->test_bpp_;
    if (offset > d->image_.size() || bytes > d->image_.size() - offset) {
      d->test_overflow_ = true;
      return;
    }
    memcpy(d->image_.data() + offset, pixels, bytes);
  }
  static void* BufferInitCallback(void* opaque, size_t threads, size_t pixels) {
    static_cast<Driver*>(opaque)->mt_threads_ = threads;
    return InitCallback(opaque, threads, pixels);
  }
  static void BufferRunCallback(void* opaque, size_t thread, size_t x, size_t y, size_t n,
                                const void* pixels) {
    auto* d = static_cast<Driver*>(opaque);
    d->mt_calls_.fetch_add(1);
    if (thread >= d->mt_threads_) d->mt_bad_thread_ = true;
    TestCallback(opaque, x, y, n, pixels);
  }

  // Event handling shared by the decode_test.cc streaming loops. Returns false
  // to stop the loop.
  bool HandleEvent(JxlDecoderStatus status, int flags, const std::string& box, size_t jpeg0) {
    switch (status) {
      case JXL_DEC_BASIC_INFO: Info(); return true;
      case JXL_DEC_COLOR_ENCODING: Color(); return true;
      case JXL_DEC_FRAME:
        ++frames_;
        FrameInfo();
        if (flags & 4) out_ << " D:" << JxlDecoderSetProgressiveDetail(dec_, kDC);
        return true;
      case JXL_DEC_NEED_IMAGE_OUT_BUFFER: return SetOutputs(false);
      case JXL_DEC_NEED_PREVIEW_OUT_BUFFER: return SetOutputs(true);
      case JXL_DEC_PREVIEW_IMAGE: Outputs(); return true;
      case JXL_DEC_FULL_IMAGE:
        Outputs();
        if (grow_jpeg_set_) {
          out_ << " a-:";
          ReleaseGrowJpeg();
        }
        return !(flags & 8);
      case JXL_DEC_FRAME_PROGRESSION:
        out_ << " r:" << JxlDecoderGetIntendedDownsamplingRatio(dec_);
        if (policy_ & 2) {
          out_ << " f:" << JxlDecoderFlushImage(dec_);
          Outputs();
        }
        return true;
      case JXL_DEC_JPEG_RECONSTRUCTION:
        if (jpeg0) {
          out_ << " a:";
          SetGrowJpeg(jpeg0);
        }
        return true;
      case JXL_DEC_JPEG_NEED_MORE_OUTPUT:
        out_ << " a+:";
        GrowJpeg();
        return true;
      case JXL_DEC_BOX: {
        if (!grow_box_.empty()) {
          out_ << " v-:";
          ReleaseGrowBox();
        }
        out_ << ' ';
        BoxType(0);
        out_ << ' ';
        BoxType(1);
        out_ << ' ';
        BoxSize(false, true);
        out_ << ' ';
        BoxSize(true, true);
        JxlBoxType type;
        if (box != "-" &&
            JxlDecoderGetBoxType(dec_, type, (flags & 16) ? JXL_TRUE : JXL_FALSE) ==
                JXL_DEC_SUCCESS &&
            memcmp(type, box.data(), 4) == 0) {
          out_ << " v:";
          SetGrowBox(8);
        }
        return true;
      }
      case JXL_DEC_BOX_NEED_MORE_OUTPUT:
        out_ << " v+:";
        GrowBox();
        return true;
      case JXL_DEC_SUCCESS:
        if (!grow_box_.empty()) {
          out_ << " v-:";
          ReleaseGrowBox();
        }
        return false;
      default: return false;
    }
  }

  // A<inc>,<flags>,<box>,<jpeg0>,<steps>: the input loops of decode_test.cc.
  // flags 1: set the input before and release it after every ProcessInput
  // (else only on NEED_MORE_INPUT); 2: close the input once all of it is set;
  // 4: SetProgressiveDetail(kDC) at each frame; 8: stop after FULL_IMAGE;
  // 16: match the decompressed box type; 64: grow the window by inc after
  // every call without subtracting consumed bytes (InputHandlingTestStreaming);
  // 128: return at the first status other than NEED_MORE_INPUT. Each
  // NEED_MORE_INPUT adds inc more bytes. Contents of boxes of type <box> go
  // to a growing buffer; JPEG reconstruction uses a growing buffer of jpeg0
  // bytes.
  void StreamLoop(const std::string& arg) {
    std::istringstream fields(arg);
    size_t inc, flags, jpeg0, steps;
    std::string box;
    char comma;
    fields >> inc >> comma >> flags >> comma;
    std::getline(fields, box, ',');
    fields >> jpeg0 >> comma >> steps;
    if (!fields || (box != "-" && box.size() != 4)) throw std::runtime_error("stream loop");
    out_ << 0;
    for (size_t step = 0; step < steps; ++step) {
      JxlDecoderStatus status;
      if (flags & (1 | 64)) {
        JxlDecoderStatus set = JxlDecoderSetInput(dec_, file_.data() + pos_, a_avail_);
        if (set == JXL_DEC_SUCCESS) set_len_ = a_avail_;
        status = JxlDecoderProcessInput(dec_);
        size_t remaining = Release();
        out_ << " A" << set << ':' << status << ',' << remaining;
        if (flags & 64) {
          a_avail_ = std::min(a_avail_ + inc, file_.size() - pos_);
          if (status == JXL_DEC_NEED_MORE_INPUT) {
            if (pos_ == file_.size()) break;
            continue;
          }
        } else {
          a_avail_ = remaining;
          if (status == JXL_DEC_NEED_MORE_INPUT) {
            if (a_total_ >= file_.size()) break;
            size_t amount = std::min(inc, file_.size() - a_total_);
            a_total_ += amount;
            a_avail_ += amount;
            continue;
          }
        }
      } else {
        status = JxlDecoderProcessInput(dec_);
        out_ << " A" << status;
        if (status == JXL_DEC_NEED_MORE_INPUT) {
          if (inc == 0) break;
          size_t remaining = Release();
          size_t amount = std::min(inc, file_.size() - a_total_);
          a_total_ += amount;
          a_avail_ = remaining + amount;
          JxlDecoderStatus set = JxlDecoderSetInput(dec_, file_.data() + pos_, a_avail_);
          if (set == JXL_DEC_SUCCESS) set_len_ = a_avail_;
          out_ << ',' << remaining << ',' << set << ',' << a_avail_;
          if ((flags & 2) && a_total_ == file_.size() && !closed_) {
            JxlDecoderCloseInput(dec_);
            closed_ = true;
            out_ << ",c";
          }
          continue;
        }
      }
      if (flags & 128) break;
      if (!HandleEvent(status, int(flags), box, jpeg0)) break;
    }
  }

  // B<end>:<flush>/...: VerifyProgression of decode_test.cc. The input ends at
  // each breakpoint in turn; flush 0 expects no flush, 1 the same and 2 a new
  // progressive image.
  void ProgressionLoop(const std::string& arg) {
    std::vector<std::pair<size_t, int>> breakpoints;
    std::istringstream fields(arg);
    std::string item;
    while (std::getline(fields, item, '/')) {
      auto colon = item.find(':');
      breakpoints.push_back({std::stoul(item.substr(0, colon)), std::stoi(item.substr(colon + 1))});
    }
    if (breakpoints.empty() || breakpoints[0].first > file_.size()) {
      throw std::runtime_error("progression breakpoints");
    }
    size_t bp = 0;
    JxlDecoderStatus set = JxlDecoderSetInput(dec_, file_.data(), breakpoints[0].first);
    if (set == JXL_DEC_SUCCESS) {
      pos_ = 0;
      set_len_ = breakpoints[0].first;
    }
    out_ << set;
    for (size_t step = 0; step < 10000; ++step) {
      JxlDecoderStatus status = JxlDecoderProcessInput(dec_);
      out_ << " B" << status;
      if (status == JXL_DEC_BASIC_INFO) {
        out_ << ' ';
        BasicInfo(true);
        out_ << " f:" << JxlDecoderFlushImage(dec_);
        Outputs();
        SetOutputs(false);
      } else if (status == JXL_DEC_FRAME) {
      } else if (status == JXL_DEC_SUCCESS) {
        break;
      } else if (status == JXL_DEC_NEED_MORE_INPUT || status == JXL_DEC_FULL_IMAGE) {
        if (breakpoints[bp].second == 0 || status != JXL_DEC_FULL_IMAGE) {
          out_ << " f:" << JxlDecoderFlushImage(dec_);
        }
        Outputs();
        if (status == JXL_DEC_FULL_IMAGE) continue;
        if (++bp >= breakpoints.size()) break;
        size_t remaining = Release();
        size_t end = breakpoints[bp].first;
        if (end < pos_ || end > file_.size()) throw std::runtime_error("progression input");
        set = JxlDecoderSetInput(dec_, file_.data() + pos_, end - pos_);
        if (set == JXL_DEC_SUCCESS) set_len_ = end - pos_;
        out_ << ',' << remaining << ',' << set;
      } else {
        break;
      }
    }
  }

  std::vector<uint8_t> file_;
  std::ostream& out_;
  Allocator allocator_;
  JxlDecoder* dec_ = nullptr;
  size_t pos_ = 0, set_len_ = 0;
  bool closed_ = false, box_set_ = false, print_events_ = false;
  std::vector<uint8_t> box_;
  JxlPixelFormat format_{4, JXL_TYPE_UINT8, JXL_NATIVE_ENDIAN, 0};
  int out_mode_ = 0, policy_ = 0;
  uint32_t extras_ = 0;
  size_t frames_ = 0, progressions_ = 0;
  bool depth_set_ = false;
  JxlBitDepth depth_{};
  std::vector<uint8_t> image_;
  std::vector<std::vector<uint8_t>> extra_;
  uint64_t cb_hash_ = 14695981039346656037ull;
  size_t cb_calls_ = 0, cb_inits_ = 0, cb_destroys_ = 0;
  // A JPEG buffer stays alive until released, as a caller retains it; the
  // decoder writes it only during ProcessInput.
  std::vector<uint8_t> jpeg_;
  // JPEG buffers forgotten by Rewind/Reset, still owned by the caller.
  std::vector<std::vector<uint8_t>> retired_;
  // Growing caller buffers of decode_test.cc: the decoder writes behind the
  // used prefix, and each extension doubles the buffer.
  std::vector<uint8_t> grow_jpeg_, grow_box_;
  bool grow_jpeg_set_ = false;
  // Streaming loop input window and the total bytes made available.
  size_t a_avail_ = 0, a_total_ = 0;
  size_t test_bpp_ = 0, test_stride_ = 0;
  std::atomic<bool> test_overflow_{false};
  // Mode 5: the latest init's thread count, run calls and thread-id checks.
  size_t mt_threads_ = 0;
  std::atomic<size_t> mt_calls_{0};
  std::atomic<bool> mt_bad_thread_{false};
  int runner_kind_ = 0;
  void* runner_ = nullptr;
};


// dpub-run <mm> <fail> <file> <op>...: one public decoder history.
inline void RunHistory(std::istringstream& in, std::ostream& out) {
  int mm;
  size_t fail;
  std::string hex;
  in >> mm >> fail >> hex;
  Driver driver(Unhex(hex), out);
  std::string op;
  if (driver.Create(mm, fail)) {
    while (in >> op) driver.Op(op);
    driver.Destroy();
  } else {
    while (in >> op) {
    }
  }
  if (!in.eof()) throw std::runtime_error("decoder public parse");
  in.clear();
}

}  // namespace decoder_public

#endif  // TOOLS_ORACLE_DECODER_PUBLIC_DRIVER_H_
