// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder step 1: the upstream test cases of the step's gate in differential
// form. Each `es1-*` request runs one upstream test case's inputs and settings
// through the original archive and prints what the test observes, so that the
// Rust mirror in tests/parity/encoder_step1.rs compares every output exactly
// instead of the test's tolerance or size expectation. The inputs are the
// tests' own, built with the original's own helpers (Rng); this adapter
// supplies no encoder algorithm.
//
// es1-perm KIND LEN: coeff_order_test.cc TestPermutation (encode half):
//   the permutation of KIND (0 identity, 1 few swaps, 2 few slides,
//   3 random) of LEN entries from Rng(0), EncodePermutation's status and
//   bytes after ZeroPadToByte, and the DecodePermutation roundtrip.
// es1-qw-dc: quant_weights_test.cc DC: DequantMatricesSetCustomDC with
//   {1e5, 1e3, 1e1} (the encode, then the decode roundtrip inside it).
// es1-qw-default: quant_weights_test.cc AllDefault: DequantMatricesSetCustom
//   with kNumQuantTables Library<0> encodings, the roundtripped encodings,
//   and DequantMatricesEncode's bytes of the result.
// es1-rt TYPE CHANNELS LOSSLESS CONTAINER: roundtrip_test.cc
//   VerifyRoundtripCompression's encoder half at 63x129 for one interleaved
//   frame of JxlDataType TYPE (GetTestImage<T> samples), every status the
//   test compares and the compressed bytes. The decoder half runs through
//   the dpub-run histories of both decoders (tests/parity/decoder_public.rs).
#include <jxl/color_encoding.h>
#include <jxl/encode.h>
#include <jxl/memory_manager.h>

#include <algorithm>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iterator>
#include <iomanip>
#include <iostream>
#include <memory>
#include <numeric>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "lib/jxl/base/random.h"
#include "lib/jxl/base/span.h"
#include "lib/jxl/base/status.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/coeff_order_fwd.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_coeff_order.h"
#include "lib/jxl/enc_modular.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_quant_weights.h"
#include "lib/jxl/encode_internal.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/quant_weights.h"

namespace {
uint32_t Es1Bits(float v) { uint32_t b; std::memcpy(&b, &v, sizeof(b)); return b; }
float Es1Float(std::istringstream& in) {
  uint32_t bits; in >> bits; float v; std::memcpy(&v, &bits, sizeof(v)); return v;
}
void Es1Hex(jxl::Bytes bytes) {
  std::cout << ' ';
  if (bytes.size() == 0) std::cout << '-';
  for (size_t i = 0; i < bytes.size(); ++i) {
    std::cout << std::hex << std::setw(2) << std::setfill('0') << unsigned(bytes[i]);
  }
  std::cout << std::dec << std::setfill(' ');
}

// coeff_order_test.cc TestPermutation up to RoundtripPermutation, with the
// test's Rng(0) and kSwaps.
void Es1Permutation(JxlMemoryManager* mm, int kind, size_t len) {
  constexpr size_t kSwaps = 32;
  std::vector<jxl::coeff_order_t> perm(len);
  std::iota(perm.begin(), perm.end(), 0);
  jxl::Rng rng(0);
  if (kind == 1) {
    for (size_t i = 0; i < kSwaps; i++) {
      size_t a = rng.UniformU(0, len - 1);
      size_t b = rng.UniformU(0, len - 1);
      std::swap(perm[a], perm[b]);
    }
  }
  if (kind == 2) {
    for (size_t i = 0; i < kSwaps; i++) {
      size_t a = rng.UniformU(0, len - 1);
      size_t b = rng.UniformU(0, len - 1);
      size_t from = std::min(a, b);
      size_t to = std::max(a, b);
      size_t start = perm[from];
      for (size_t j = from; j < to; j++) {
        perm[j] = perm[j + 1];
      }
      perm[to] = start;
    }
  }
  if (kind == 3) {
    rng.Shuffle(perm.data(), perm.size());
  }
  std::cout << "P " << len;
  for (jxl::coeff_order_t v : perm) std::cout << ' ' << v;
  // RoundtripPermutation.
  jxl::BitWriter writer{mm};
  jxl::Status status =
      jxl::EncodePermutation(perm.data(), 0, len, &writer, jxl::LayerType::Header, nullptr);
  std::cout << " W " << int(status.code());
  if (!status) return;
  writer.ZeroPadToByte();
  jxl::Bytes bytes = writer.GetSpan();
  Es1Hex(bytes);
  std::cout << ' ' << bytes.size();
  std::vector<jxl::coeff_order_t> out(len);
  jxl::Status read = true;
  jxl::Status close = true;
  {
    jxl::BitReader reader(bytes);
    read = jxl::DecodePermutation(mm, 0, len, out.data(), &reader);
    close = reader.Close();
  }
  std::cout << " R " << int(read.code()) << ' ' << int(close.code());
  for (jxl::coeff_order_t v : out) std::cout << ' ' << v;
}

void Es1PrintEncoding(const jxl::QuantEncoding& e) {
  std::cout << ' ' << int(e.mode) << ',' << int(e.predefined) << ','
            << e.dct_params.num_distance_bands;
}
}  // namespace

// ---------------------------------------------------------------------------
// roundtrip_test.cc and the EncodeImageJXL cases of jxl_test.cc and
// speed_tier_test.cc.

namespace jxl {
namespace test {
// The extracted original in decode_test_inputs.cc (original_check_impl.inc).
void CheckImpl(bool ok, const char* condition, const char* file, int line);
}  // namespace test
}  // namespace jxl

namespace es1_rt {
// The test's EXPECT_EQ inside the extracted EncodeWithEncoder: recorded and
// printed instead of reported to a test framework.
bool expectations_held = true;
#define EXPECT_EQ(a, b) (es1_rt::expectations_held &= ((a) == (b)))
#define Check(OK) ::jxl::test::CheckImpl((OK), #OK, __FILE__, __LINE__)
// test_utils.cc JxlBasicInfoSetFromPixelFormat.
#include "original_basic_info_from_pixel_format.inc"
// roundtrip_test.cc ConvertTestPixel, GetTestImage and EncodeWithEncoder.
#include "original_roundtrip_test_helpers.inc"
#undef Check
#undef EXPECT_EQ

// VerifyRoundtripCompression's encoder half for the admitted instances: one
// interleaved color frame without extra channels, no resampling, no
// container, lossy. Prints every status the test compares, the queue check
// and the compressed bytes.
template <typename T>
void EncodeHalf(size_t xsize, size_t ysize, const JxlPixelFormat& input_pixel_format,
                bool lossless, bool use_container) {
  const std::vector<uint8_t> original_bytes =
      GetTestImage<T>(xsize, ysize, input_pixel_format);
  JxlEncoder* enc = JxlEncoderCreate(nullptr);
  std::cout << "R " << (enc != nullptr);
  std::cout << ' ' << JxlEncoderSetCodestreamLevel(enc, 10);
  std::cout << ' ' << JxlEncoderUseContainer(enc, use_container);
  JxlBasicInfo basic_info;
  JxlBasicInfoSetFromPixelFormat(&basic_info, &input_pixel_format);
  basic_info.xsize = xsize;
  basic_info.ysize = ysize;
  basic_info.uses_original_profile = lossless;
  uint32_t num_channels = input_pixel_format.num_channels;
  size_t has_interleaved_alpha = num_channels == 2 || num_channels == 4;
  basic_info.num_extra_channels = 0 + has_interleaved_alpha;
  std::cout << ' ' << JxlEncoderSetBasicInfo(enc, &basic_info);
  std::cout << ' ' << enc->metadata.m.num_extra_channels;
  JxlColorEncoding color_encoding;
  if (input_pixel_format.data_type == JXL_TYPE_FLOAT) {
    JxlColorEncodingSetToLinearSRGB(&color_encoding,
                                    /*is_gray=*/input_pixel_format.num_channels < 3);
  } else {
    JxlColorEncodingSetToSRGB(&color_encoding,
                              /*is_gray=*/input_pixel_format.num_channels < 3);
  }
  std::cout << ' ' << JxlEncoderSetColorEncoding(enc, &color_encoding);
  std::cout << ' ' << JxlEncoderSetUpsamplingMode(enc, 1, -1);
  JxlEncoderFrameSettings* frame_settings = JxlEncoderFrameSettingsCreate(enc, nullptr);
  std::cout << ' ' << (frame_settings != nullptr);
  std::cout << ' ' << JxlEncoderSetFrameLossless(frame_settings, lossless);
  std::cout << ' '
            << JxlEncoderAddImageFrame(frame_settings, &input_pixel_format,
                                       static_cast<const void*>(original_bytes.data()),
                                       original_bytes.size());
  std::cout << ' ' << frame_settings->enc->input_queue.empty();
  JxlEncoderCloseInput(enc);
  std::vector<uint8_t> compressed;
  expectations_held = true;
  EncodeWithEncoder(enc, &compressed);
  JxlEncoderDestroy(enc);
  std::cout << ' ' << expectations_held;
  Es1Hex(jxl::Bytes(compressed.data(), compressed.size()));
}

}  // namespace es1_rt

bool EncoderStep1Command(const std::string& command, std::istringstream& in,
                         JxlMemoryManager* mm) {
  if (command.rfind("es1-", 0) != 0) return false;
  if (command == "es1-perm") {
    int kind; size_t len; in >> kind >> len;
    Es1Permutation(mm, kind, len);
    return true;
  }
  if (command == "es1-qw-dc") {
    // quant_weights_test.cc DC.
    jxl::DequantMatrices mat;
    float dc_quant[3] = {1e+5, 1e+3, 1e+1};
    const jxl::Status status = jxl::DequantMatricesSetCustomDC(mm, &mat, dc_quant);
    std::cout << "D " << int(status.code());
    for (size_t c = 0; c < 3; c++) {
      std::cout << ' ' << Es1Bits(mat.InvDCQuant(c)) << ' ' << Es1Bits(mat.DCQuant(c));
    }
    return true;
  }
  if (command == "es1-qw-default") {
    // quant_weights_test.cc AllDefault: RoundtripMatrices' encode half.
    std::vector<jxl::QuantEncoding> encodings(jxl::kNumQuantTables,
                                              jxl::QuantEncoding::Library<0>());
    auto mat = jxl::make_unique<jxl::DequantMatrices>();
    auto metadata = jxl::make_unique<jxl::CodecMetadata>();
    jxl::FrameHeader frame_header(metadata.get());
    auto encoder_or = jxl::ModularFrameEncoder::Create(mm, frame_header, jxl::CompressParams{},
                                                       false);
    if (!encoder_or.ok()) throw std::runtime_error("ModularFrameEncoder");
    auto encoder = std::move(encoder_or).value_();
    const jxl::Status status = jxl::DequantMatricesSetCustom(mat.get(), encodings, encoder.get());
    std::cout << "S " << int(status.code()) << ' ' << mat->encodings().size();
    for (const jxl::QuantEncoding& e : mat->encodings()) Es1PrintEncoding(e);
    jxl::BitWriter writer{mm};
    const jxl::Status encoded =
        jxl::DequantMatricesEncode(mm, *mat, &writer, jxl::LayerType::Header, nullptr, nullptr);
    writer.ZeroPadToByte();
    std::cout << " W " << int(encoded.code());
    Es1Hex(writer.GetSpan());
    return true;
  }
  if (command == "es1-rt") {
    unsigned type, channels, lossless, container;
    in >> type >> channels >> lossless >> container;
    const JxlPixelFormat format = {channels, JxlDataType(type), JXL_NATIVE_ENDIAN, 0};
    if (type == JXL_TYPE_FLOAT) {
      es1_rt::EncodeHalf<float>(63, 129, format, lossless != 0, container != 0);
    } else if (type == JXL_TYPE_UINT16) {
      es1_rt::EncodeHalf<uint16_t>(63, 129, format, lossless != 0, container != 0);
    } else {
      es1_rt::EncodeHalf<uint8_t>(63, 129, format, lossless != 0, container != 0);
    }
    return true;
  }
  throw std::runtime_error("unknown encoder step 1 command");
}
