// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Whole decodes through the original public decoder (port ADR 0004,
// "Decoder path" condition (b)): `dps-decode <type> <file>` decodes the whole
// file with the original JxlDecoder, as the Ferrisma engine calls the port's
// (one call with the whole input, closed input, no parallel runner, no CMS,
// default coalescing and orientation), and prints the image info and every
// output byte. tests/simd_public.rs mirrors this sequence token for token on
// the port. It calls only the public API declared by the pinned
// lib/include/jxl headers and implements no decoder algorithm.
#include <jxl/codestream_header.h>
#include <jxl/color_encoding.h>
#include <jxl/decode.h>
#include <jxl/types.h>

#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "decoder_public_driver.h"

namespace {

using decoder_public::Bytes;
using decoder_public::F32;
using decoder_public::F64;
using decoder_public::Hex;
using decoder_public::Unhex;

void BasicInfo(JxlDecoder* dec, std::ostream& out) {
  JxlBasicInfo info;
  memset(&info, 0, sizeof(info));
  JxlDecoderStatus status = JxlDecoderGetBasicInfo(dec, &info);
  out << " b:" << status;
  if (status != JXL_DEC_SUCCESS) return;
  out << ',' << info.have_container << ',' << info.xsize << ',' << info.ysize << ','
      << info.bits_per_sample << ',' << info.exponent_bits_per_sample << ','
      << F32(info.intensity_target) << ',' << F32(info.min_nits) << ','
      << info.relative_to_max_display << ',' << F32(info.linear_below) << ','
      << info.uses_original_profile << ',' << info.have_preview << ',' << info.have_animation
      << ',' << info.orientation << ',' << info.num_color_channels << ','
      << info.num_extra_channels << ',' << info.alpha_bits << ',' << info.alpha_exponent_bits
      << ',' << info.alpha_premultiplied << ',' << info.preview.xsize << ','
      << info.preview.ysize << ',' << info.animation.tps_numerator << ','
      << info.animation.tps_denominator << ',' << info.animation.num_loops << ','
      << info.animation.have_timecodes << ',' << info.intrinsic_xsize << ','
      << info.intrinsic_ysize;
  for (uint32_t i = 0; i < info.num_extra_channels; ++i) {
    JxlExtraChannelInfo ec;
    memset(&ec, 0, sizeof(ec));
    status = JxlDecoderGetExtraChannelInfo(dec, i, &ec);
    out << " e:" << status;
    if (status != JXL_DEC_SUCCESS) continue;
    out << ',' << ec.type << ',' << ec.bits_per_sample << ',' << ec.exponent_bits_per_sample
        << ',' << ec.dim_shift << ',' << ec.name_length << ',' << ec.alpha_premultiplied;
    for (float v : ec.spot_color) out << ',' << F32(v);
    out << ',' << ec.cfa_channel;
  }
}

void Color(JxlDecoder* dec, std::ostream& out) {
  for (int target = 0; target < 2; ++target) {
    size_t size = 0;
    JxlDecoderStatus status =
        JxlDecoderGetICCProfileSize(dec, JxlColorProfileTarget(target), &size);
    out << " c:" << status << ',' << size;
    if (status == JXL_DEC_SUCCESS) {
      std::vector<uint8_t> icc(size);
      status = JxlDecoderGetColorAsICCProfile(dec, JxlColorProfileTarget(target), icc.data(),
                                              size);
      out << ',' << status << ',' << Hex(Bytes(icc.data(), icc.size()));
    }
    JxlColorEncoding color;
    memset(&color, 0, sizeof(color));
    status = JxlDecoderGetColorAsEncodedProfile(dec, JxlColorProfileTarget(target), &color);
    out << " C:" << status;
    if (status != JXL_DEC_SUCCESS) continue;
    out << ',' << color.color_space << ',' << color.white_point << ','
        << F64(color.white_point_xy[0]) << ',' << F64(color.white_point_xy[1]) << ','
        << color.primaries << ',' << F64(color.primaries_red_xy[0]) << ','
        << F64(color.primaries_red_xy[1]) << ',' << F64(color.primaries_green_xy[0]) << ','
        << F64(color.primaries_green_xy[1]) << ',' << F64(color.primaries_blue_xy[0]) << ','
        << F64(color.primaries_blue_xy[1]) << ',' << color.transfer_function << ','
        << F64(color.gamma) << ',' << color.rendering_intent;
  }
}

void FrameHeader(JxlDecoder* dec, std::ostream& out) {
  JxlFrameHeader header;
  memset(&header, 0, sizeof(header));
  JxlDecoderStatus status = JxlDecoderGetFrameHeader(dec, &header);
  out << " h:" << status;
  if (status != JXL_DEC_SUCCESS) return;
  const JxlLayerInfo& l = header.layer_info;
  out << ',' << header.duration << ',' << header.timecode << ',' << header.name_length << ','
      << header.is_last << ',' << l.have_crop << ',' << l.crop_x0 << ',' << l.crop_y0 << ','
      << l.xsize << ',' << l.ysize << ',' << l.blend_info.blendmode << ','
      << l.blend_info.source << ',' << l.blend_info.alpha << ',' << l.blend_info.clamp << ','
      << l.save_as_reference;
}

void Decode(uint32_t type, const std::vector<uint8_t>& file, std::ostream& out) {
  JxlDecoder* dec = JxlDecoderCreate(nullptr);
  out << "create:" << (dec ? 1 : 0);
  if (!dec) return;
  int events = JXL_DEC_BASIC_INFO | JXL_DEC_COLOR_ENCODING | JXL_DEC_FRAME | JXL_DEC_FULL_IMAGE;
  out << " s:" << JxlDecoderSubscribeEvents(dec, events);
  out << " I:" << JxlDecoderSetInput(dec, file.data(), file.size());
  JxlDecoderCloseInput(dec);
  JxlPixelFormat format{0, JxlDataType(type), JXL_NATIVE_ENDIAN, 0};
  JxlPixelFormat ec_format{1, JxlDataType(type), JXL_NATIVE_ENDIAN, 0};
  uint32_t extras = 0;
  std::vector<uint8_t> image;
  std::vector<std::vector<uint8_t>> extra;
  for (;;) {
    JxlDecoderStatus status = JxlDecoderProcessInput(dec);
    out << " P" << status;
    if (status == JXL_DEC_BASIC_INFO) {
      BasicInfo(dec, out);
      JxlBasicInfo info;
      if (JxlDecoderGetBasicInfo(dec, &info) == JXL_DEC_SUCCESS) {
        format.num_channels = info.num_color_channels + (info.alpha_bits ? 1 : 0);
        extras = info.num_extra_channels;
      }
    } else if (status == JXL_DEC_COLOR_ENCODING) {
      Color(dec, out);
    } else if (status == JXL_DEC_FRAME) {
      FrameHeader(dec, out);
    } else if (status == JXL_DEC_NEED_IMAGE_OUT_BUFFER) {
      size_t size = 0;
      JxlDecoderStatus s = JxlDecoderImageOutBufferSize(dec, &format, &size);
      out << " q:" << s << ',' << size;
      if (s != JXL_DEC_SUCCESS) break;
      image.assign(size, 0);
      out << " Q:" << JxlDecoderSetImageOutBuffer(dec, &format, image.data(), size);
      extra.assign(extras, {});
      for (uint32_t i = 0; i < extras; ++i) {
        s = JxlDecoderExtraChannelBufferSize(dec, &ec_format, &size, i);
        out << " v:" << s << ',' << size;
        if (s != JXL_DEC_SUCCESS) continue;
        extra[i].assign(size, 0);
        out << ',' << JxlDecoderSetExtraChannelBuffer(dec, &ec_format, extra[i].data(), size, i);
      }
    } else if (status == JXL_DEC_FULL_IMAGE) {
      out << " o:" << Hex(Bytes(image.data(), image.size()));
      for (const auto& e : extra) out << " x:" << Hex(Bytes(e.data(), e.size()));
    } else {
      break;
    }
  }
  JxlDecoderDestroy(dec);
}

}  // namespace

bool DecoderPublicSimdCommand(const std::string& command, std::istringstream& in,
                              JxlMemoryManager*) {
  if (command != "dps-decode") return false;
  uint32_t type;
  std::string hex;
  in >> type >> hex;
  if (!in) throw std::runtime_error("dps-decode arguments");
  Decode(type, Unhex(hex), std::cout);
  return true;
}
