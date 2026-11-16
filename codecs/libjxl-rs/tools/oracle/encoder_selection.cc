// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The engine's lossy encode of a photograph through the original's public
// encoder API (port ADR 0004, "Encoder selection", tests/simd_encoder_public.rs).
// The TU only calls the pinned public API; no kernel arithmetic.
//   esel-ppm <path> <x0> <y0> <xsize> <ysize> <distance bits> <alpha>
//       Reads the binary 8-bit RGB PPM at <path>, takes the rectangle
//       (<x0>, <y0>, <xsize>, <ysize>) of it and encodes it as the engine
//       does (crates/ferrisma-jxl-encoder): basic info 8 bits, sRGB, effort 7
//       and patches off set explicitly, the distance, no runner, one
//       interleaved native UINT8 frame (RGB, or RGBA with <alpha> 1 and an
//       8-bit straight alpha channel whose values are a function of the
//       pixel position, see EselAlpha), CloseInput and ProcessOutput into one
//       buffer. Prints the last status and the codestream as hex.
#include <jxl/codestream_header.h>
#include <jxl/color_encoding.h>
#include <jxl/encode.h>
#include <jxl/memory_manager.h>
#include <jxl/types.h>

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace {

// The alpha value of a pixel: a ramp with transparent stripes, every value
// from 0 to 255 (mirrored by tests/simd_encoder_public.rs).
uint8_t EselAlpha(size_t x, size_t y) {
  if ((x + 2 * y) % 23 < 5) return 0;
  return static_cast<uint8_t>((x * 7 + y * 3) % 256);
}

}  // namespace

bool EncoderSelectionCommand(const std::string& command, std::istringstream& in,
                             JxlMemoryManager*) {
  if (command != "esel-ppm") return false;
  std::string path;
  size_t x0, y0, xsize, ysize;
  uint32_t distance_bits;
  int alpha;
  in >> path >> x0 >> y0 >> xsize >> ysize >> distance_bits >> alpha;
  float distance;
  std::memcpy(&distance, &distance_bits, sizeof(distance));
  std::ifstream file(path, std::ios::binary);
  std::string magic;
  size_t width, height, maxval;
  file >> magic >> width >> height >> maxval;
  file.get();
  if (!file || magic != "P6" || maxval != 255 || x0 + xsize > width || y0 + ysize > height) {
    throw std::runtime_error("esel-ppm: unreadable PPM or rectangle outside it");
  }
  std::vector<uint8_t> image(width * height * 3);
  file.read(reinterpret_cast<char*>(image.data()), static_cast<std::streamsize>(image.size()));
  if (!file) throw std::runtime_error("esel-ppm: short PPM");
  const size_t channels = alpha ? 4 : 3;
  std::vector<uint8_t> pixels(xsize * ysize * channels);
  for (size_t y = 0; y < ysize; ++y) {
    for (size_t x = 0; x < xsize; ++x) {
      const uint8_t* src = &image[((y0 + y) * width + x0 + x) * 3];
      uint8_t* dst = &pixels[(y * xsize + x) * channels];
      std::memcpy(dst, src, 3);
      if (alpha) dst[3] = EselAlpha(x, y);
    }
  }
  JxlEncoder* enc = JxlEncoderCreate(nullptr);
  JxlEncoderStatus status = JXL_ENC_ERROR;
  std::vector<uint8_t> out;
  {
    JxlBasicInfo info;
    JxlEncoderInitBasicInfo(&info);
    info.xsize = static_cast<uint32_t>(xsize);
    info.ysize = static_cast<uint32_t>(ysize);
    info.bits_per_sample = 8;
    info.num_color_channels = 3;
    info.uses_original_profile = JXL_FALSE;
    info.orientation = JXL_ORIENT_IDENTITY;
    if (alpha) {
      info.num_extra_channels = 1;
      info.alpha_bits = 8;
    }
    status = JxlEncoderSetBasicInfo(enc, &info);
    JxlColorEncoding color;
    JxlColorEncodingSetToSRGB(&color, JXL_FALSE);
    if (status == JXL_ENC_SUCCESS) status = JxlEncoderSetColorEncoding(enc, &color);
    JxlEncoderFrameSettings* settings = JxlEncoderFrameSettingsCreate(enc, nullptr);
    if (status == JXL_ENC_SUCCESS)
      status = JxlEncoderFrameSettingsSetOption(settings, JXL_ENC_FRAME_SETTING_EFFORT, 7);
    if (status == JXL_ENC_SUCCESS)
      status = JxlEncoderFrameSettingsSetOption(settings, JXL_ENC_FRAME_SETTING_PATCHES, 0);
    if (status == JXL_ENC_SUCCESS) status = JxlEncoderSetFrameDistance(settings, distance);
    JxlPixelFormat format{static_cast<uint32_t>(channels), JXL_TYPE_UINT8, JXL_NATIVE_ENDIAN, 0};
    if (status == JXL_ENC_SUCCESS)
      status = JxlEncoderAddImageFrame(settings, &format, pixels.data(), pixels.size());
    if (status == JXL_ENC_SUCCESS) {
      JxlEncoderCloseInput(enc);
      out.resize(pixels.size() + 65536);
      uint8_t* next = out.data();
      size_t avail = out.size();
      status = JxlEncoderProcessOutput(enc, &next, &avail);
      out.resize(out.size() - avail);
    }
  }
  JxlEncoderDestroy(enc);
  std::cout << static_cast<int>(status) << ' ';
  static const char* digits = "0123456789abcdef";
  std::string hex;
  hex.reserve(2 * out.size());
  for (uint8_t b : out) {
    hex.push_back(digits[b >> 4]);
    hex.push_back(digits[b & 15]);
  }
  std::cout << (hex.empty() ? std::string("-") : hex);
  return true;
}
