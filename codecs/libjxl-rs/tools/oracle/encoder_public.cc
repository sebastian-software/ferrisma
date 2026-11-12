// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observation driver for the original public encoder API in the complete
// archive. It replays caller histories on the original JxlEncoder through the
// public functions of the pinned lib/include/jxl/encode.h and records every
// status, error and output chunk. It implements no encoder algorithm. The
// Rust driver in tests/support/encoder_public.rs mirrors the epub-run
// protocol token for token.
//
// Two observations read the original's internal state, compiled with
// -fno-access-control as the other internal fixtures: the 'S' checkpoint
// (stored metadata, serialized through the original WriteCodestreamHeaders,
// frame-settings values and queued inputs) and the epub-probe command, which
// reports at each AddImageFrame and ProcessOutput call whether the original
// took its fast-lossless branch or called ProcessOneEnqueuedInput. The latter
// is the evidence for every admission result of the Rust encoder (ADR 0001).
// epub-wrap drives the original JxlEncoderOutputProcessorWrapper in buffer
// mode.
#include <jxl/cms.h>
#include <jxl/codestream_header.h>
#include <jxl/color_encoding.h>
#include <jxl/encode.h>
#include <jxl/memory_manager.h>
#include <jxl/types.h>

#include <algorithm>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <map>
#include <ostream>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "decoder_public_driver.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_frame.h"
#include "lib/jxl/encode_internal.h"

namespace {

using decoder_public::Bytes;
using decoder_public::F32;
using decoder_public::F64;
using decoder_public::Hex;
using decoder_public::Unhex;

std::vector<std::string> Split(const std::string& arg) {
  std::vector<std::string> fields;
  std::istringstream in(arg);
  std::string field;
  while (std::getline(in, field, ',')) fields.push_back(field);
  return fields;
}

float FloatBits(const std::string& field) {
  uint32_t bits = uint32_t(std::stoul(field));
  float value;
  memcpy(&value, &bits, 4);
  return value;
}

uint64_t Digest(const uint8_t* data, size_t size) {
  uint64_t h = 14695981039346656037ull;
  for (size_t i = 0; i < size; ++i) {
    h ^= data[i];
    h *= 1099511628211ull;
  }
  return h;
}

// Every manager allocation in these histories is an object whose size is a
// property of the language layout (the encoder, frame settings, queued frames
// and boxes), so the log records order and identity, not sizes.
struct Allocator {
  size_t calls = 0, fail = 0, next = 0;
  std::map<void*, size_t> live;
  std::vector<std::string> events;
  static void* Alloc(void* opaque, size_t size) {
    auto& a = *static_cast<Allocator*>(opaque);
    ++a.calls;
    a.events.push_back("A");
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

std::string Blend(const JxlBlendInfo& b) {
  std::ostringstream o;
  o << b.blendmode << '.' << b.source << '.' << b.alpha << '.' << b.clamp;
  return o.str();
}

std::string Values(const jxl::JxlEncoderFrameSettingsValues& v) {
  std::ostringstream o;
  const jxl::CompressParams& c = v.cparams;
  o << v.lossless << ',' << F32(c.butteraugli_distance) << ',' << c.ec_distance.size();
  for (float d : c.ec_distance) o << ':' << F32(d);
  o << ',' << c.max_error_mode << ',' << c.disable_perceptual_optimizations << ','
    << static_cast<int>(c.speed_tier) << ',' << c.brotli_effort << ',' << c.decoding_speed_tier
    << ',' << static_cast<uint32_t>(c.color_transform) << ',' << c.modular_mode << ','
    << c.modular_group_size_shift << ',' << static_cast<int>(c.preview) << ','
    << static_cast<int>(c.noise) << ',' << static_cast<int>(c.dots) << ','
    << static_cast<int>(c.patches) << ',' << static_cast<int>(c.gaborish) << ',' << c.epf << ','
    << static_cast<int>(c.progressive_mode) << ',' << static_cast<int>(c.qprogressive_mode)
    << ',' << c.centerfirst << ',' << c.center_x << ',' << c.center_y << ','
    << c.progressive_dc << ',' << static_cast<int>(c.keep_invisible) << ',' << c.cms_set << ','
    << c.force_cfl_jpeg_recompression << ',' << c.force_lfs_jpeg_recompression << ','
    << c.jpeg_compress_boxes << ',' << c.jpeg_keep_exif << ',' << c.jpeg_keep_xmp << ','
    << c.jpeg_keep_jumbf << ',' << F32(c.photon_noise_iso) << ',' << F32(c.options.nb_repeats)
    << ',' << c.options.max_properties << ',' << static_cast<uint32_t>(c.options.predictor)
    << ',' << c.responsive << ',' << c.colorspace << ','
    << F32(c.channel_colors_pre_transform_percent) << ',' << F32(c.channel_colors_percent)
    << ',' << c.palette_colors << ',' << c.lossy_palette << ',' << c.resampling << ','
    << c.ec_resampling << ',' << c.already_downsampled << ',' << c.level << ',' << c.buffering
    << ',' << c.output_mode << ',' << c.use_full_image_heuristics << ','
    << (c.debug_image != nullptr);
  const JxlFrameHeader& h = v.header;
  o << ",h" << h.duration << ',' << h.timecode << ',' << h.name_length << ',' << h.is_last << ','
    << h.layer_info.have_crop << ',' << h.layer_info.crop_x0 << ',' << h.layer_info.crop_y0 << ','
    << h.layer_info.xsize << ',' << h.layer_info.ysize << ',' << Blend(h.layer_info.blend_info)
    << ',' << h.layer_info.save_as_reference;
  o << ",b" << v.extra_channel_blend_info.size();
  for (const auto& b : v.extra_channel_blend_info) o << ':' << Blend(b);
  o << ",n" << Hex(Bytes(reinterpret_cast<const uint8_t*>(v.frame_name.data()), v.frame_name.size()))
    << ",d" << v.image_bit_depth.type << ':' << v.image_bit_depth.bits_per_sample << ':'
    << v.image_bit_depth.exponent_bits_per_sample << ",i" << v.frame_index_box << ",x"
    << (v.aux_out != nullptr) << ",a" << v.strip_alpha;
  return o.str();
}

// The original JxlEncoder has a user-provided constructor, so its basic_info
// member is default-initialized and JxlEncoderInitBasicInfo leaves `padding`
// unset: those bytes are indeterminate until JxlEncoderSetBasicInfo copies a
// caller's struct. A checkpoint prints them only while basic_info_set holds.
std::string BasicInfo(const JxlBasicInfo& info, bool padding_defined) {
  std::ostringstream o;
  o << info.have_container << ',' << info.xsize << ',' << info.ysize << ','
    << info.bits_per_sample << ',' << info.exponent_bits_per_sample << ','
    << F32(info.intensity_target) << ',' << F32(info.min_nits) << ','
    << info.relative_to_max_display << ',' << F32(info.linear_below) << ','
    << info.uses_original_profile << ',' << info.have_preview << ',' << info.have_animation
    << ',' << info.orientation << ',' << info.num_color_channels << ','
    << info.num_extra_channels << ',' << info.alpha_bits << ',' << info.alpha_exponent_bits
    << ',' << info.alpha_premultiplied << ',' << info.preview.xsize << ',' << info.preview.ysize
    << ',' << info.animation.tps_numerator << ',' << info.animation.tps_denominator << ','
    << info.animation.num_loops << ',' << info.animation.have_timecodes << ','
    << info.intrinsic_xsize << ',' << info.intrinsic_ysize << ','
    << (padding_defined ? Hex(Bytes(info.padding, sizeof(info.padding))) : "~");
  return o.str();
}

std::string Encoding(const JxlColorEncoding& c) {
  std::ostringstream o;
  o << c.color_space << '.' << c.white_point << '.' << F64(c.white_point_xy[0]) << '.'
    << F64(c.white_point_xy[1]) << '.' << c.primaries << '.' << F64(c.primaries_red_xy[0]) << '.'
    << F64(c.primaries_red_xy[1]) << '.' << F64(c.primaries_green_xy[0]) << '.'
    << F64(c.primaries_green_xy[1]) << '.' << F64(c.primaries_blue_xy[0]) << '.'
    << F64(c.primaries_blue_xy[1]) << '.' << c.transfer_function << '.' << F64(c.gamma) << '.'
    << c.rendering_intent;
  return o.str();
}

JxlPixelFormat Format(const std::vector<std::string>& f, size_t at) {
  return JxlPixelFormat{uint32_t(std::stoul(f[at])), JxlDataType(std::stoul(f[at + 1])),
                        JxlEndianness(std::stoul(f[at + 2])), size_t(std::stoul(f[at + 3]))};
}

class Driver {
 public:
  Driver(std::ostream& out, JxlMemoryManager* mm, bool probe)
      : out_(out), mm_(mm), probe_(probe) {}
  ~Driver() {
    if (enc_) JxlEncoderDestroy(enc_);
  }

  // mm 0: default manager; 1: custom manager with its event log; 2: custom
  // manager without the log.
  bool Create(int mm, size_t fail) {
    allocator_.fail = fail;
    print_events_ = mm == 1;
    JxlMemoryManager manager{&allocator_, Allocator::Alloc, Allocator::Free};
    enc_ = JxlEncoderCreate(mm ? &manager : nullptr);
    out_ << "create:" << (enc_ ? 1 : 0);
    return enc_ != nullptr;
  }

  void Destroy() {
    JxlEncoderDestroy(enc_);
    enc_ = nullptr;
    out_ << " E";
    if (print_events_)
      for (const auto& e : allocator_.events) out_ << ' ' << e;
  }

  void Op(const std::string& op) {
    char kind = op[0];
    std::string arg = op.substr(1);
    std::vector<std::string> f = Split(arg);
    out_ << ' ' << kind << ':';
    if (probe_) {
      Probe(kind, arg, f);
      return;
    }
    out_ << Result(kind, arg, f) << '/' << JxlEncoderGetError(enc_);
  }

 private:
  JxlEncoderFrameSettings* Settings(const std::string& index) {
    return settings_.at(std::stoul(index));
  }

  // The original's internal state at the admission checkpoints, observed
  // around the same public call.
  void Probe(char kind, const std::string& arg, const std::vector<std::string>& f) {
    if (kind == 'p') {
      out_ << enc_->input_queue.size() << ',' << f[0] << ','
           << enc_->output_processor.OutputProcessorSet();
      FrameEvidence();
      Result(kind, arg, f);
      return;
    }
    Result(kind, arg, f);
    if (kind == 'a') {
      out_ << enc_->input_queue.size() << ',';
      if (enc_->input_queue.empty()) {
        out_ << '-';
      } else {
        const auto& back = enc_->input_queue.back();
        out_ << (back.fast_lossless_frame ? 'L' : back.frame ? 'F' : 'B');
      }
      return;
    }
    out_ << '-';
  }

  // The front queued frame's parameters as the original's EncodeFrame
  // resolves them (speed-tier switches, then the original ParamsPostInit) and
  // its frame and metadata state: the evidence for the frame encoder's
  // admission results. Fields: distance bits, speed tier, buffering, epf,
  // decoding speed tier, modular mode, frame xsize and ysize, xyb_encoded,
  // extra channels, resampling.
  void FrameEvidence() {
    if (enc_->input_queue.empty() || !enc_->input_queue[0].frame) return;
    const jxl::JxlEncoderQueuedFrame& frame = *enc_->input_queue[0].frame;
    jxl::CompressParams c = frame.option_values.cparams;
    if (c.speed_tier == jxl::SpeedTier::kTectonicPlate && !c.IsLossless()) {
      c.speed_tier = jxl::SpeedTier::kGlacier;
    }
    if (c.speed_tier == jxl::SpeedTier::kLightning) c.speed_tier = jxl::SpeedTier::kThunder;
    bool ok = static_cast<bool>(jxl::ParamsPostInit(&c));
    out_ << ";" << ok << ',' << F32(c.butteraugli_distance) << ','
         << static_cast<int>(c.speed_tier) << ',' << c.buffering << ',' << c.epf << ','
         << c.decoding_speed_tier << ',' << c.modular_mode << ',' << frame.frame_data.xsize
         << ',' << frame.frame_data.ysize << ',' << enc_->metadata.m.xyb_encoded << ','
         << enc_->metadata.m.num_extra_channels << ',' << c.resampling;
  }

  std::string Result(char kind, const std::string& arg, const std::vector<std::string>& f) {
    std::ostringstream o;
    switch (kind) {
      case 'V': o << JxlEncoderVersion(); break;
      case 'x': {
        JxlEncoderReset(enc_);
        settings_.clear();
        ec_set_.clear();
        o << 0;
        break;
      }
      case 'e': o << JxlEncoderGetError(enc_); break;
      case 'B': {
        JxlBasicInfo info;
        memset(&info, 0, sizeof(info));
        JxlEncoderInitBasicInfo(&info);
        for (const auto& field : f) {
          auto colon = field.find(':');
          std::string key = field.substr(0, colon);
          uint32_t v = uint32_t(std::stoul(field.substr(colon + 1)));
          float fv;
          memcpy(&fv, &v, 4);
          if (key == "w") info.xsize = v;
          else if (key == "h") info.ysize = v;
          else if (key == "bits") info.bits_per_sample = v;
          else if (key == "exp") info.exponent_bits_per_sample = v;
          else if (key == "it") info.intensity_target = fv;
          else if (key == "mn") info.min_nits = fv;
          else if (key == "rel") info.relative_to_max_display = int(v);
          else if (key == "lb") info.linear_below = fv;
          else if (key == "orig") info.uses_original_profile = int(v);
          else if (key == "prev") info.have_preview = int(v);
          else if (key == "anim") info.have_animation = int(v);
          else if (key == "or") info.orientation = JxlOrientation(v);
          else if (key == "c") info.num_color_channels = v;
          else if (key == "ec") info.num_extra_channels = v;
          else if (key == "ab") info.alpha_bits = v;
          else if (key == "aexp") info.alpha_exponent_bits = v;
          else if (key == "apre") info.alpha_premultiplied = int(v);
          else if (key == "px") info.preview.xsize = v;
          else if (key == "py") info.preview.ysize = v;
          else if (key == "ix") info.intrinsic_xsize = v;
          else if (key == "iy") info.intrinsic_ysize = v;
          else if (key == "tn") info.animation.tps_numerator = v;
          else if (key == "td") info.animation.tps_denominator = v;
          else if (key == "nl") info.animation.num_loops = v;
          else if (key == "tc") info.animation.have_timecodes = int(v);
          else if (key == "cont") info.have_container = int(v);
          else if (key == "pad") info.padding[v % sizeof(info.padding)] = uint8_t(v >> 8);
          else throw std::runtime_error("basic info key");
        }
        o << JxlEncoderSetBasicInfo(enc_, &info);
        break;
      }
      case 'I': {
        JxlBasicInfo info;
        memset(&info, 0xA5, sizeof(info));
        JxlEncoderInitBasicInfo(&info);
        JxlFrameHeader header;
        memset(&header, 0xA5, sizeof(header));
        JxlEncoderInitFrameHeader(&header);
        JxlBlendInfo blend;
        memset(&blend, 0xA5, sizeof(blend));
        JxlEncoderInitBlendInfo(&blend);
        JxlExtraChannelInfo ec;
        memset(&ec, 0xA5, sizeof(ec));
        JxlEncoderInitExtraChannelInfo(JxlExtraChannelType(std::stoul(arg)), &ec);
        o << 0 << ';' << BasicInfo(info, true) << ';' << header.duration << ',' << header.timecode << ','
          << header.name_length << ',' << header.is_last << ',' << header.layer_info.have_crop
          << ',' << header.layer_info.crop_x0 << ',' << header.layer_info.crop_y0 << ','
          << header.layer_info.xsize << ',' << header.layer_info.ysize << ','
          << Blend(header.layer_info.blend_info) << ',' << header.layer_info.save_as_reference
          << ';' << Blend(blend) << ';' << ec.type << ',' << ec.bits_per_sample << ','
          << ec.exponent_bits_per_sample << ',' << ec.dim_shift << ',' << ec.name_length << ','
          << ec.alpha_premultiplied;
        for (float v : ec.spot_color) o << ',' << F32(v);
        o << ',' << ec.cfa_channel;
        break;
      }
      case 'K': {
        JxlExtraChannelInfo ec;
        memset(&ec, 0, sizeof(ec));
        ec.type = JxlExtraChannelType(std::stoul(f[1]));
        ec.bits_per_sample = uint32_t(std::stoul(f[2]));
        ec.exponent_bits_per_sample = uint32_t(std::stoul(f[3]));
        ec.dim_shift = uint32_t(std::stoul(f[4]));
        ec.alpha_premultiplied = int(std::stoul(f[5]));
        ec.cfa_channel = uint32_t(std::stoul(f[6]));
        for (int i = 0; i < 4; ++i) ec.spot_color[i] = FloatBits(f[7 + i]);
        o << JxlEncoderSetExtraChannelInfo(enc_, std::stoul(f[0]), &ec);
        break;
      }
      case 'n': {
        std::vector<uint8_t> name = Unhex(f[1]);
        name.push_back(0);
        o << JxlEncoderSetExtraChannelName(enc_, std::stoul(f[0]),
                                           reinterpret_cast<const char*>(name.data()),
                                           name.size() - 1);
        break;
      }
      case 'C': {
        JxlColorEncoding color = decoder_public::ParseEncoding(arg);
        o << JxlEncoderSetColorEncoding(enc_, &color);
        break;
      }
      case 'G':
      case 'g': {
        JxlColorEncoding color;
        memset(&color, 0, sizeof(color));
        if (kind == 'G') JxlColorEncodingSetToSRGB(&color, int(std::stoul(arg)));
        else JxlColorEncodingSetToLinearSRGB(&color, int(std::stoul(arg)));
        o << JxlEncoderSetColorEncoding(enc_, &color);
        break;
      }
      case 'T': {
        JxlColorEncoding color;
        memset(&color, 0xA5, sizeof(color));
        JxlColorEncodingSetToSRGB(&color, int(std::stoul(arg)));
        o << 0 << ';' << Encoding(color);
        memset(&color, 0xA5, sizeof(color));
        JxlColorEncodingSetToLinearSRGB(&color, int(std::stoul(arg)));
        o << ';' << Encoding(color);
        break;
      }
      case 'Q': {
        std::vector<uint8_t> icc = Unhex(arg);
        o << JxlEncoderSetICCProfile(enc_, icc.empty() ? nullptr : icc.data(), icc.size());
        break;
      }
      case 'L': o << JxlEncoderSetCodestreamLevel(enc_, std::stoi(arg)); break;
      case 'l': o << JxlEncoderGetRequiredCodestreamLevel(enc_); break;
      case 'U': o << JxlEncoderUseContainer(enc_, std::stoi(arg)); break;
      case 'J': o << JxlEncoderStoreJPEGMetadata(enc_, std::stoi(arg)); break;
      case 'X': o << JxlEncoderUseBoxes(enc_); break;
      case 'A': JxlEncoderAllowExpertOptions(enc_); o << 0; break;
      case 'M': JxlEncoderSetCms(enc_, *JxlGetDefaultCms()); o << 0; break;
      case 'u':
        o << JxlEncoderSetUpsamplingMode(enc_, std::stoll(f[0]), std::stoll(f[1]));
        break;
      case 's': {
        const JxlEncoderFrameSettings* source = arg == "-" ? nullptr : Settings(arg);
        JxlEncoderFrameSettings* created = JxlEncoderFrameSettingsCreate(enc_, source);
        if (created) {
          o << settings_.size();
          settings_.push_back(created);
        } else {
          o << '-';
        }
        break;
      }
      case 'o':
        o << JxlEncoderFrameSettingsSetOption(Settings(f[0]),
                                              JxlEncoderFrameSettingId(std::stoul(f[1])),
                                              std::stoll(f[2]));
        break;
      case 'f':
        o << JxlEncoderFrameSettingsSetFloatOption(
            Settings(f[0]), JxlEncoderFrameSettingId(std::stoul(f[1])), FloatBits(f[2]));
        break;
      case 'd': o << JxlEncoderSetFrameDistance(Settings(f[0]), FloatBits(f[1])); break;
      case 'D':
        o << JxlEncoderSetExtraChannelDistance(Settings(f[0]), std::stoul(f[1]), FloatBits(f[2]));
        break;
      case 'q': o << F32(JxlEncoderDistanceFromQuality(FloatBits(arg))); break;
      case 'y': o << JxlEncoderSetFrameLossless(Settings(f[0]), std::stoi(f[1])); break;
      case 'h': {
        JxlFrameHeader h;
        memset(&h, 0, sizeof(h));
        JxlEncoderInitFrameHeader(&h);
        std::vector<int64_t> v;
        for (size_t i = 1; i < f.size(); ++i) v.push_back(std::stoll(f[i]));
        if (v.size() != 14) throw std::runtime_error("frame header fields");
        h.duration = uint32_t(v[0]);
        h.timecode = uint32_t(v[1]);
        h.name_length = uint32_t(v[2]);
        h.is_last = int(v[3]);
        h.layer_info.have_crop = int(v[4]);
        h.layer_info.crop_x0 = int32_t(v[5]);
        h.layer_info.crop_y0 = int32_t(v[6]);
        h.layer_info.xsize = uint32_t(v[7]);
        h.layer_info.ysize = uint32_t(v[8]);
        h.layer_info.blend_info.blendmode = JxlBlendMode(v[9]);
        h.layer_info.blend_info.source = uint32_t(v[10]);
        h.layer_info.blend_info.alpha = uint32_t(v[11]);
        h.layer_info.blend_info.clamp = int(v[12]);
        h.layer_info.save_as_reference = uint32_t(v[13]);
        o << JxlEncoderSetFrameHeader(Settings(f[0]), &h);
        break;
      }
      case 't': {
        JxlBlendInfo b{JxlBlendMode(std::stoul(f[2])), uint32_t(std::stoul(f[3])),
                       uint32_t(std::stoul(f[4])), int(std::stoul(f[5]))};
        o << JxlEncoderSetExtraChannelBlendInfo(Settings(f[0]), std::stoul(f[1]), &b);
        break;
      }
      case 'N': {
        if (f[1] == "~") {
          o << JxlEncoderSetFrameName(Settings(f[0]), nullptr);
        } else {
          std::vector<uint8_t> name = Unhex(f[1]);
          name.push_back(0);
          o << JxlEncoderSetFrameName(Settings(f[0]), reinterpret_cast<const char*>(name.data()));
        }
        break;
      }
      case 'W': {
        JxlBitDepth depth{JxlBitDepthType(std::stoul(f[1])), uint32_t(std::stoul(f[2])),
                          uint32_t(std::stoul(f[3]))};
        o << JxlEncoderSetFrameBitDepth(Settings(f[0]), &depth);
        break;
      }
      case 'a': {
        JxlPixelFormat format = Format(f, 1);
        bool null = f[5] == "~";
        size_t size = null ? 0 : std::stoul(f[5]);
        // Optional image content: a7,...,size,content,xsize,ysize.
        size_t alpha_c = format.num_channels == 2 || format.num_channels == 4
                             ? format.num_channels - 1
                             : kNoAlpha;
        std::vector<uint8_t> pixels =
            f.size() > 6 ? Content(size, format, std::stoul(f[6]), std::stoul(f[7]),
                                   std::stoul(f[8]), alpha_c)
                         : Pattern(size, 7, FloatFormat(format));
        o << JxlEncoderAddImageFrame(Settings(f[0]), &format, null ? nullptr : pixels.data(),
                                     size);
        break;
      }
      case 'E': {
        JxlPixelFormat format = Format(f, 1);
        bool null = f[5] == "~";
        size_t size = null ? 0 : std::stoul(f[5]);
        uint32_t index = uint32_t(std::stoul(f[6]));
        // Optional content: E0,...,size,index,content,xsize,ysize; its single
        // channel takes the alpha mode of the content.
        std::vector<uint8_t> pixels =
            f.size() > 7 ? Content(size, format, std::stoul(f[7]), std::stoul(f[8]),
                                   std::stoul(f[9]), 0)
                         : Pattern(size, 11 + index, FloatFormat(format));
        JxlEncoderStatus status = JxlEncoderSetExtraChannelBuffer(
            Settings(f[0]), &format, null ? nullptr : pixels.data(), size, index);
        if (status == JXL_ENC_SUCCESS) {
          ec_set_[enc_->input_queue.back().frame.get()].insert(index);
        }
        o << status;
        break;
      }
      case 'O': {
        std::vector<uint8_t> type = Unhex(f[0]);
        if (type.size() != 4) throw std::runtime_error("box type");
        std::vector<uint8_t> contents = Pattern(std::stoul(f[1]), 5);
        o << JxlEncoderAddBox(enc_, reinterpret_cast<const char*>(type.data()), contents.data(), std::stoul(f[1]),
                              std::stoi(f[2]));
        break;
      }
      case 'F': JxlEncoderCloseFrames(enc_); o << 0; break;
      case 'Y': JxlEncoderCloseBoxes(enc_); o << 0; break;
      case 'Z': JxlEncoderCloseInput(enc_); o << 0; break;
      case 'p': {
        size_t avail = std::stoul(f[0]);
        output_.assign(avail + 1, 0xA5);
        next_out_ = output_.data();
        avail_out_ = avail;
        JxlEncoderStatus status = JxlEncoderProcessOutput(enc_, &next_out_, &avail_out_);
        o << status << ',' << (next_out_ - output_.data()) << ',' << avail_out_ << ','
          << Hex(Bytes(output_.data(), avail));
        break;
      }
      case 'S': o << 0 << ';' << State(); break;
      default: throw std::runtime_error("encoder public op");
    }
    return o.str();
  }

  // Caller sample bytes. One extra byte keeps a nonnull pointer for zero
  // sizes. Float and float16 formats use bytes 0x30-0x3f only: every 4- or
  // 2-byte word of them, at any offset and byte order, is a finite positive
  // sample below 2, so the original's float-to-integer conversions stay
  // defined; arbitrary bytes would include NaN and out-of-range values.
  static std::vector<uint8_t> Pattern(size_t size, uint32_t seed, bool floats = false) {
    std::vector<uint8_t> data(size + 1);
    for (size_t i = 0; i < data.size(); ++i) {
      uint8_t v = uint8_t((i * 13 + seed * 29 + 7) & 0xff);
      data[i] = floats ? uint8_t(0x30 | (v & 0x0f)) : v;
    }
    return data;
  }
  // Image content for the lossy encoder gate. Each sample is q/1024 for an
  // integer q in [0, 1024], from integer arithmetic only, stored exactly in
  // the requested type (uint8 and uint16 round q * max / 1024 to nearest),
  // byte order and row alignment. Content 1 is flat, 2 a gradient, 3 noise
  // and 4 a photograph-like mix of smooth ramps, edges and fine texture,
  // 5 text-like content: dark glyphs, repeated from a small set, on a flat
  // light background. Content + 16 * m selects alpha mode m for the alpha
  // channel `alpha_c`: 0 the color content, 1 opaque, 2 a ramp with fully
  // transparent stripes, 3 a ramp without zeros.
  static constexpr size_t kNoAlpha = ~size_t(0);
  static uint32_t ContentHash(uint32_t x, uint32_t y, uint32_t c) {
    uint32_t h = x * 0x9E3779B1u ^ (y + 0x7F4A7C15u) * 0x85EBCA77u ^ (c + 1) * 0xC2B2AE3Du;
    h ^= h >> 15;
    h *= 0x2C1B3C6Du;
    h ^= h >> 12;
    return h;
  }
  static uint32_t ContentQ(uint32_t content, uint32_t x, uint32_t y, uint32_t c, uint32_t xsize,
                           uint32_t ysize) {
    auto tri = [](uint32_t t) { uint32_t m = t % 512; return m < 256 ? m : 511 - m; };
    switch (content) {
      case 1: return 300 + 200 * c;
      case 2: return (x * 700) / std::max(xsize, 1u) + (y * 300) / std::max(ysize, 1u) + c * 10;
      case 3: return ContentHash(x, y, c) % 1025;
      case 5: {
        uint32_t gx = x % 10, gy = y % 14;
        if (x < 4 || y < 4 || gx >= 7 || gy >= 10) return 960;
        uint32_t glyph = (x / 10 * 7 + y / 14 * 3) % 5;
        return ContentHash(gx, gy, glyph + 100) % 3 != 0 ? 80 + 5 * c : 960;
      }
      default: {
        uint32_t q = 150 + tri(x * 3 + y * 2 + c * 40) + tri(x * 11 / 7 + 5 * c) / 2;
        if ((x / 13 + y / 9) % 5 == 0) q += 120;
        q += ContentHash(x, y, c) % 48;
        return std::min(q, 1024u);
      }
    }
  }
  static uint32_t AlphaQ(uint32_t mode, uint32_t x, uint32_t y, uint32_t xsize) {
    switch (mode) {
      case 1: return 1024;
      case 2:
        if ((x + 2 * y) % 23 < 6) return 0;
        return std::min(1024u, 40 + (x * 1024) / std::max(xsize, 1u) + ContentHash(x, y, 9) % 32);
      default: return 200 + (x * 800) / std::max(xsize, 1u) + (y % 7) * 3;
    }
  }
  static std::vector<uint8_t> Content(size_t size, const JxlPixelFormat& format,
                                      uint32_t content, uint32_t xsize, uint32_t ysize,
                                      size_t alpha_c) {
    uint32_t alpha_mode = content / 16;
    content %= 16;
    std::vector<uint8_t> data(size + 1, 0);
    size_t bytes = format.data_type == JXL_TYPE_UINT8 ? 1
                   : format.data_type == JXL_TYPE_FLOAT ? 4
                                                        : 2;
    size_t row = size_t(xsize) * format.num_channels * bytes;
    size_t stride = format.align > 1 && row % format.align != 0
                        ? row + format.align - row % format.align
                        : row;
    bool big = format.endianness == JXL_BIG_ENDIAN;
    for (uint32_t y = 0; y < ysize; ++y) {
      for (uint32_t x = 0; x < xsize; ++x) {
        for (uint32_t c = 0; c < format.num_channels; ++c) {
          size_t at = y * stride + (size_t(x) * format.num_channels + c) * bytes;
          if (at + bytes > size) continue;
          uint32_t q = alpha_mode != 0 && c == alpha_c
                           ? AlphaQ(alpha_mode, x, y, xsize)
                           : ContentQ(content, x, y, c, xsize, ysize);
          uint32_t word;
          if (format.data_type == JXL_TYPE_UINT8) {
            word = (q * 255 + 512) / 1024;
          } else if (format.data_type == JXL_TYPE_UINT16) {
            word = (q * 65535 + 512) / 1024;
          } else if (format.data_type == JXL_TYPE_FLOAT) {
            float v = float(q) / 1024.0f;
            memcpy(&word, &v, 4);
          } else {
            // float16: q/1024 exactly; q < 2^11 fits the 10-bit mantissa.
            if (q == 0) {
              word = 0;
            } else {
              uint32_t e = 0;
              while ((q >> (e + 1)) != 0) ++e;
              word = ((e + 5) << 10) | (((q << 10) >> e) & 0x3ff);
            }
          }
          for (size_t b = 0; b < bytes; ++b) {
            size_t shift = big ? 8 * (bytes - 1 - b) : 8 * b;
            data[at + b] = uint8_t(word >> shift);
          }
        }
      }
    }
    return data;
  }
  static bool FloatFormat(const JxlPixelFormat& format) {
    return format.data_type == JXL_TYPE_FLOAT || format.data_type == JXL_TYPE_FLOAT16;
  }

  std::string Channel(const jxl::JxlEncoderChunkedFrameAdapter& adapter, size_t index) {
    const auto& c = adapter.channels_[index];
    std::ostringstream o;
    o << 'c' << index << '.' << c.format_.num_channels << '.' << c.format_.data_type << '.'
      << c.format_.endianness << '.' << c.format_.align << '.' << c.xsize_ << '.' << c.ysize_
      << '.' << c.bytes_per_pixel_ << '.' << c.stride_ << '.' << c.buffer_size_ << '.'
      << c.copy_.size() << '.' << Digest(c.copy_.data(), c.copy_.size());
    return o.str();
  }

  std::string State() {
    std::ostringstream o;
    const JxlEncoder& e = *enc_;
    o << e.cms_set << e.use_container << e.use_boxes << e.store_jpeg_metadata << e.wrote_bytes
      << e.frames_closed << e.boxes_closed << e.basic_info_set << e.color_encoding_set
      << e.intensity_target_set << e.allow_expert_options << ',' << e.codestream_level << ','
      << e.brotli_effort << ',' << e.container_ftyp_version << ',' << e.num_queued_frames << ','
      << e.num_queued_boxes << ',' << e.codestream_bytes_written_end_of_frame << ','
      << e.jxlp_counter << ',' << e.error << ',' << e.frame_index_box.index_box_requested_through_api
      << ',' << e.frame_index_box.TNUM << ',' << e.frame_index_box.TDEN << ','
      << e.frame_index_box.entries.size() << ',' << e.jpeg_metadata.size() << ','
      << (e.thread_pool != nullptr) << ',' << e.output_processor.OutputProcessorSet() << ','
      << e.output_processor.HasAvailOut() << ',' << e.output_processor.CurrentPosition() << ','
      << e.output_processor.HasOutputToWrite();
    // The stored metadata, serialized by the original on a copy.
    jxl::CodecMetadata copy = e.metadata;
    jxl::BitWriter writer(mm_);
    jxl::Status status = jxl::WriteCodestreamHeaders(&copy, &writer, nullptr);
    if (status) {
      status = writer.WithMaxBits(8, jxl::LayerType::Header, nullptr, [&] {
        writer.ZeroPadToByte();
        return true;
      });
    }
    o << ";H" << (status ? 0 : 1) << ':';
    if (status) {
      jxl::Span<const uint8_t> span = writer.GetSpan();
      o << Hex(Bytes(span.data(), span.size()));
    } else {
      o << '-';
    }
    const auto& icc = e.metadata.m.color_encoding.ICC();
    o << ";I" << e.metadata.m.color_encoding.WantICC() << ':' << Hex(Bytes(icc.data(), icc.size()));
    o << ";B" << BasicInfo(e.basic_info, e.basic_info_set);
    for (size_t i = 0; i < e.encoder_options.size(); ++i) {
      o << ";s" << i << ':' << Values(e.encoder_options[i]->values) << ','
        << (e.encoder_options[i]->enc == enc_);
    }
    o << ";q" << e.input_queue.size();
    for (const auto& input : e.input_queue) {
      if (input.frame) {
        const jxl::JxlEncoderQueuedFrame& frame = *input.frame;
        o << ";F" << input.output_mode << ':' << Values(frame.option_values) << ":ec";
        for (uint8_t v : frame.ec_initialized) o << int(v);
        o << ':' << frame.frame_data.xsize << '.' << frame.frame_data.ysize << '.'
          << frame.frame_data.channels_.size() << ':' << Channel(frame.frame_data, 0);
        auto set = ec_set_.find(input.frame.get());
        if (set != ec_set_.end())
          for (size_t index : set->second) o << ':' << Channel(frame.frame_data, 1 + index);
      } else if (input.box) {
        const jxl::JxlEncoderQueuedBox& box = *input.box;
        o << ";X" << input.output_mode << ':' << Hex(Bytes(box.type.data(), 4)) << ':'
          << box.compress_box << ':' << box.contents.size() << ':'
          << Digest(box.contents.data(), box.contents.size());
      } else {
        o << ";L" << input.output_mode;
      }
    }
    return o.str();
  }

  std::ostream& out_;
  JxlMemoryManager* mm_;
  bool probe_;
  JxlEncoder* enc_ = nullptr;
  Allocator allocator_;
  bool print_events_ = false;
  std::vector<JxlEncoderFrameSettings*> settings_;
  // Extra channels of each queued frame with a successful buffer call.
  std::map<const jxl::JxlEncoderQueuedFrame*, std::set<size_t>> ec_set_;
  // ProcessOutput's caller variables; the original retains their addresses.
  std::vector<uint8_t> output_;
  uint8_t* next_out_ = nullptr;
  size_t avail_out_ = 0;
};

void RunHistory(std::istringstream& in, std::ostream& out, JxlMemoryManager* mm, bool probe) {
  int manager;
  size_t fail;
  in >> manager >> fail;
  Driver driver(out, mm, probe);
  std::string op;
  if (driver.Create(manager, fail)) {
    while (in >> op) driver.Op(op);
    driver.Destroy();
  } else {
    while (in >> op) {
    }
  }
  if (!in.eof()) throw std::runtime_error("encoder public parse");
  in.clear();
}

// The buffer mode of the original JxlEncoderOutputProcessorWrapper.
void RunWrapper(std::istringstream& in, std::ostream& out, JxlMemoryManager* mm) {
  JxlEncoderOutputProcessorWrapper wrapper(mm);
  std::vector<uint8_t> caller;
  uint8_t* next = nullptr;
  size_t avail = 0;
  size_t caller_size = 0;
  std::string op;
  bool first = true;
  while (in >> op) {
    char kind = op[0];
    std::vector<std::string> f = Split(op.substr(1));
    out << (first ? "" : " ") << kind << ':';
    first = false;
    auto pattern = [](size_t count, size_t seed) {
      std::vector<uint8_t> data(count);
      for (size_t i = 0; i < count; ++i) data[i] = uint8_t((i * 7 + seed * 31 + 3) & 0xff);
      return data;
    };
    switch (kind) {
      case 'A': {
        caller_size = std::stoul(f[0]);
        caller.assign(caller_size + 1, 0xA5);
        next = caller.data();
        avail = caller_size;
        out << (wrapper.SetAvailOut(&next, &avail) ? 0 : 1);
        break;
      }
      case 'G':
      case 'R': {
        auto result = wrapper.GetBuffer(std::stoul(f[0]), std::stoul(f[1]));
        if (!result.ok()) {
          out << 1 << ',' << static_cast<int>(result.status().code());
          break;
        }
        JxlOutputProcessorBuffer buffer = std::move(result).value_();
        // Appends and advances stay within the window, as encoder callers do.
        size_t count = std::min<size_t>(std::stoul(f[2]), buffer.size());
        out << 0 << ',' << buffer.size();
        // One extra byte keeps a nonnull source pointer for empty appends.
        std::vector<uint8_t> data = pattern(count + 1, std::stoul(f[3]));
        out << ',' << (buffer.append(data.data(), count) ? 0 : 1);
        if (kind == 'R') out << ',' << (buffer.release() ? 0 : 1);
        break;
      }
      case 'K': out << (wrapper.Seek(std::stoul(f[0])) ? 0 : 1); break;
      case 'k': out << (wrapper.Seek(wrapper.CurrentPosition() + std::stoul(f[0])) ? 0 : 1); break;
      case 'H': {
        // The AppendBox order: reserve the header, write the contents, write
        // the header into the reserved bytes and return to the end.
        size_t header = std::stoul(f[0]);
        size_t start = wrapper.CurrentPosition();
        out << (wrapper.Seek(start + header) ? 0 : 1);
        std::vector<uint8_t> contents = pattern(std::stoul(f[1]), std::stoul(f[2]));
        out << (AppendData(wrapper, contents) ? 0 : 1);
        size_t end = wrapper.CurrentPosition();
        out << (wrapper.Seek(start) ? 0 : 1);
        std::vector<uint8_t> bytes = pattern(header, std::stoul(f[2]) + 1);
        out << (AppendData(wrapper, bytes) ? 0 : 1);
        out << (wrapper.Seek(end) ? 0 : 1);
        break;
      }
      case 'F': out << (wrapper.SetFinalizedPosition() ? 0 : 1); break;
      case 'D': {
        std::vector<uint8_t> data = pattern(std::stoul(f[0]), std::stoul(f[1]));
        out << (AppendData(wrapper, data) ? 0 : 1);
        break;
      }
      case 'P':
        out << wrapper.CurrentPosition() << ',' << wrapper.HasOutputToWrite() << ','
            << wrapper.WasStopRequested() << ',' << wrapper.HasAvailOut() << ','
            << wrapper.OutputProcessorSet();
        break;
      default: throw std::runtime_error("output wrapper op");
    }
    if (!caller.empty()) {
      out << '|' << (next - caller.data()) << ',' << avail << ','
          << Hex(Bytes(caller.data(), caller_size));
    }
  }
  if (!in.eof()) throw std::runtime_error("output wrapper parse");
  in.clear();
}

}  // namespace

bool EncoderPublicCommand(const std::string& command, std::istringstream& in,
                          JxlMemoryManager* mm) {
  if (command == "epub-run" || command == "epub-probe") {
    RunHistory(in, std::cout, mm, command == "epub-probe");
    return true;
  }
  if (command == "epub-wrap") {
    RunWrapper(in, std::cout, mm);
    return true;
  }
  if (command == "epub-icc") {
    // An original profile generated by the original ColorEncoding code:
    // P3 primaries with a gamma curve (gray: gamma only).
    uint32_t gray;
    in >> gray;
    JxlColorEncoding external;
    JxlColorEncodingSetToSRGB(&external, gray ? JXL_TRUE : JXL_FALSE);
    if (!gray) external.primaries = JXL_PRIMARIES_P3;
    external.transfer_function = JXL_TRANSFER_FUNCTION_GAMMA;
    external.gamma = 0.4;
    jxl::ColorEncoding c;
    if (!c.FromExternal(external)) throw std::runtime_error("icc encoding");
    if (c.ICC().empty() && !c.CreateICC()) throw std::runtime_error("icc profile");
    std::cout << Hex(Bytes(c.ICC().data(), c.ICC().size()));
    return true;
  }
  return false;
}
