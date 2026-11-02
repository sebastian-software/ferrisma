// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Fixture/output orchestration only; all field, frame and pixel algorithms
// call actual original entry points from the pristine source archive.
#include <algorithm>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/dec_external_image.h"
#include "lib/jxl/dec_frame.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/headers.h"
#include "lib/jxl/image_metadata.h"

std::string Hex(jxl::Bytes);
std::vector<uint8_t> Unhex(const std::string&);

namespace {
void Require(jxl::Status status) {
  if (!status) throw std::runtime_error("original geometry fixture prerequisite");
}
size_t ReadHeader(const std::vector<uint8_t>& bytes, jxl::CodecMetadata& metadata) {
  jxl::BitReader reader{jxl::Bytes(bytes)};
  Require(reader.ReadFixedBits<16>() == 0x0aff);
  Require(jxl::ReadSizeHeader(&reader, &metadata.size));
  Require(jxl::ReadImageMetadata(&reader, &metadata.m));
  metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
  Require(jxl::Bundle::Read(&reader, &metadata.transform_data));
  Require(!metadata.m.color_encoding.WantICC());
  Require(reader.JumpToByteBoundary());
  Require(reader.AllReadsWithinBounds());
  size_t offset = reader.TotalBitsConsumed() / 8;
  Require(reader.Close());
  return offset;
}
std::vector<uint8_t> WriteHeader(jxl::CodecMetadata& metadata,
                               JxlMemoryManager* mm) {
  jxl::BitWriter writer(mm);
  Require(jxl::WriteCodestreamHeaders(&metadata, &writer, nullptr));
  Require(writer.WithMaxBits(8, jxl::LayerType::Header, nullptr, [&] {
    writer.ZeroPadToByte();
    return true;
  }));
  auto result = std::move(writer).TakeBytes();
  return {result.begin(), result.end()};
}
}

bool DecoderGeometryCommand(const std::string& command, std::istringstream& in,
                            JxlMemoryManager* mm) {
  if (command == "geo-rewrite") {
    std::string hex;
    unsigned orientation;
    size_t intrinsic_width, intrinsic_height;
    in >> hex >> orientation >> intrinsic_width >> intrinsic_height;
    auto bytes = Unhex(hex);
    jxl::CodecMetadata metadata;
    const size_t offset = ReadHeader(bytes, metadata);
    auto control = WriteHeader(metadata, mm);
    if (control.size() != offset ||
        !std::equal(control.begin(), control.end(), bytes.begin())) {
      throw std::runtime_error("original global-header writer byte control");
    }
    if (orientation < 1 || orientation > 8) {
      throw std::runtime_error("geometry fixture orientation domain");
    }
    metadata.m.orientation = orientation;
    metadata.m.have_intrinsic_size = intrinsic_width != 0;
    if (metadata.m.have_intrinsic_size) {
      Require(metadata.m.intrinsic_size.Set(intrinsic_width, intrinsic_height));
    } else if (intrinsic_height != 0) {
      throw std::runtime_error("geometry fixture intrinsic-size domain");
    }
    auto output = WriteHeader(metadata, mm);
    const size_t header_size = output.size();
    output.insert(output.end(), bytes.begin() + offset, bytes.end());
    std::cout << Hex(jxl::Bytes(output)) << " G " << offset << ' ' << header_size
              << " P " << Hex(jxl::Bytes(bytes.data() + offset, bytes.size() - offset));
    return true;
  }
  if (command != "geo-export") return false;
  std::string hex;
  size_t bits, channels, padding, shortage;
  bool floating, keep_orientation, unpremul;
  unsigned endianness;
  in >> hex >> bits >> floating >> channels >> keep_orientation >> unpremul
     >> padding >> shortage >> endianness;
  auto bytes = Unhex(hex);
  jxl::CodecMetadata metadata;
  size_t offset = ReadHeader(bytes, metadata);
  // Preview input has a distinct InitFrame argument. Existing fs-decode
  // observes it directly; this output adapter only receives non-preview files.
  if (metadata.m.have_preview) throw std::runtime_error("non-preview export fixture");
  jxl::PassesDecoderState state(mm);
  Require(state.output_encoding_info.SetFromMetadata(metadata));
  jxl::ImageBundle image(mm, &metadata.m);
  bool last = false;
  for (size_t frame = 0; !last; ++frame) {
    if (frame == 64) throw std::runtime_error("geometry fixture frame bound");
    jxl::FrameHeader header(&metadata);
    auto status = jxl::DecodeFrame(&state, nullptr, bytes.data() + offset,
        bytes.size() - offset, &header, &image, metadata, true);
    if (!status) {
      std::cout << "D " << int(status.code()) << ' ' << offset;
      return true;
    }
    offset += image.decoded_bytes();
    last = header.is_last;
  }
  auto orientation = keep_orientation ? jxl::Orientation::kIdentity
                                      : metadata.m.GetOrientation();
  size_t width = image.xsize(), height = image.ysize();
  if (unsigned(orientation) > 4) std::swap(width, height);
  const size_t stride = width * channels * ((bits + 7) / 8) + padding;
  std::vector<uint8_t> output(stride * height + 7, 0xa5);
  if (shortage > output.size()) throw std::runtime_error("export fixture capacity");
  auto status = jxl::ConvertToExternal(image, bits, floating, channels,
      static_cast<JxlEndianness>(endianness), stride, nullptr, output.data(),
      output.size() - shortage, {}, orientation, unpremul);
  std::cout << int(status.code()) << " M " << metadata.m.orientation << ' '
      << metadata.m.have_intrinsic_size << ' ' << metadata.m.intrinsic_size.xsize()
      << ' ' << metadata.m.intrinsic_size.ysize() << ' '
      << metadata.oriented_xsize(keep_orientation) << ' '
      << metadata.oriented_ysize(keep_orientation) << " O " << width << ' '
      << height << ' ' << image.HasAlpha() << ' ' << image.AlphaIsPremultiplied()
      << ' ' << Hex(jxl::Bytes(output));
  return true;
}
