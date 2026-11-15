// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder container (encoder step 3): observations of the original's box
// header writers, which the public histories (epub-run) reach only at sizes
// a test cannot encode. Each request calls the pinned original function and
// prints what it wrote; this adapter supplies no container algorithm. The
// Rust mirror is tests/parity/encoder_container.rs.
//
// ectr-box TYPE SIZE UNBOUNDED FORCE: jxl::WriteBoxHeader (encode.cc) into a
//   16-byte buffer; prints the returned length and the written bytes.
// ectr-append PREFIX TYPE SIZE UNBOUNDED: jxl::AppendBoxHeader
//   (encode_internal.h) onto a vector holding PREFIX; prints the vector.
// ectr-ftyp VERSION: jxl::MakeContainerHeader (encode_internal.h).
// ectr-pixels XSIZE YSIZE CHANNELS SEED: jxl::test::GetSomeTestImage, the
//   pixels of the upstream encode_test cases, as input for both drivers.
// TYPE and PREFIX are hex ("-" for an empty prefix).
#include <cstdint>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/base/span.h"
#include "lib/jxl/encode_internal.h"
#include "lib/jxl/test_image.h"

std::string Hex(jxl::Span<const uint8_t> bytes);
std::vector<uint8_t> Unhex(const std::string& hex);

namespace {

jxl::BoxType ContainerBoxType(const std::string& hex) {
  std::vector<uint8_t> bytes = Unhex(hex);
  if (bytes.size() != 4) throw std::runtime_error("box type");
  jxl::BoxType type;
  for (size_t i = 0; i < 4; ++i) type[i] = static_cast<char>(bytes[i]);
  return type;
}

}  // namespace

bool EncoderContainerCommand(const std::string& command, std::istringstream& in,
                             JxlMemoryManager*) {
  if (command == "ectr-box") {
    std::string type;
    uint64_t size;
    int unbounded, force;
    in >> type >> size >> unbounded >> force;
    uint8_t output[jxl::kLargeBoxHeaderSize] = {};
    size_t n = jxl::WriteBoxHeader(ContainerBoxType(type), static_cast<size_t>(size),
                                   unbounded != 0, force != 0, output);
    std::cout << n << ' ' << Hex(jxl::Bytes(output, n));
    return true;
  }
  if (command == "ectr-append") {
    std::string prefix, type;
    uint64_t size;
    int unbounded;
    in >> prefix >> type >> size >> unbounded;
    std::vector<uint8_t> out = prefix == "-" ? std::vector<uint8_t>() : Unhex(prefix);
    jxl::AppendBoxHeader(ContainerBoxType(type), static_cast<size_t>(size), unbounded != 0,
                         &out);
    std::cout << Hex(jxl::Bytes(out.data(), out.size()));
    return true;
  }
  if (command == "ectr-ftyp") {
    int version;
    in >> version;
    std::vector<uint8_t> out = jxl::MakeContainerHeader(version);
    std::cout << Hex(jxl::Bytes(out.data(), out.size()));
    return true;
  }
  if (command == "ectr-pixels") {
    size_t xsize, ysize, channels, seed;
    in >> xsize >> ysize >> channels >> seed;
    std::vector<uint8_t> pixels = jxl::test::GetSomeTestImage(xsize, ysize, channels, seed);
    std::cout << Hex(jxl::Bytes(pixels.data(), pixels.size()));
    return true;
  }
  return false;
}
