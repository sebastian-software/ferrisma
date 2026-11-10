// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Input construction for the lib/jxl/decode_test.cc inventory. The helpers
// that build test files (CreateTestJXLCodestream, GetTestHeader,
// AnalyzeCodestream, GetSomeTestImage, test::EncodeFile and friends) are
// extracted verbatim from the pinned test sources and hash checked at
// configure time; see source_fragments.json. This adapter supplies only what
// the gtest-free oracle lacks:
// - gtest expectations throw, so a violated construction step fails loudly;
// - GetIccTestProfile returns the pinned testdata profile passed in the
//   request instead of reading the testdata checkout;
// - CreateTestJXLCodestream's JPEG input comes from a genuine JPEG file passed
//   in the request, because the extras libjpeg encoder is not part of the
//   oracle. The pinned code after the encoder call is unchanged.
// The bodies of the frame-sequence tests (AnimationTest, SkipFrameTest and
// others) are copied from decode_test.cc up to their EncodeFile call. No
// decoder or container algorithm is implemented here.
#include <jxl/cms.h>
#include <jxl/codestream_header.h>
#include <jxl/color_encoding.h>
#include <jxl/decode.h>
#include <jxl/memory_manager.h>
#include <jxl/types.h>

#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <memory>
#include <sstream>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include "lib/extras/codec_in_out.h"
#include "lib/extras/dec/color_description.h"
#include "lib/jxl/base/byte_order.h"
#include "lib/jxl/base/common.h"
#include "lib/jxl/base/override.h"
#include "lib/jxl/base/random.h"
#include "lib/jxl/base/span.h"
#include "lib/jxl/cms/color_encoding_cms.h"
#include "lib/jxl/color_encoding_internal.h"
#include "lib/jxl/common.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_external_image.h"
#include "lib/jxl/enc_fields.h"
#include "lib/jxl/enc_frame.h"
#include "lib/jxl/enc_icc_codec.h"
#include "lib/jxl/enc_params.h"
#include "lib/jxl/enc_progressive_split.h"
#include "lib/jxl/encode_internal.h"
#include "lib/jxl/fields.h"
#include "lib/jxl/frame_dimensions.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/headers.h"
#include "lib/jxl/icc_codec.h"
#include "lib/jxl/image.h"
#include "lib/jxl/image_bundle.h"
#include "lib/jxl/image_metadata.h"
#include "lib/jxl/image_ops.h"
#include "lib/jxl/jpeg/enc_jpeg_data.h"
#include "lib/jxl/jpeg/jpeg_data.h"
#include "lib/jxl/luminance.h"
#include "lib/jxl/padded_bytes.h"
#include "lib/jxl/test_image.h"
#include "lib/jxl/test_memory_manager.h"
#include "lib/jxl/test_utils.h"
#include "lib/jxl/toc.h"

std::string Hex(jxl::Span<const uint8_t> bytes);
std::vector<uint8_t> Unhex(const std::string& hex);

// gtest-free expectations: every construction step must hold.
#define JXL_DTI_EXPECT(condition)                                               \
  do {                                                                          \
    if (!(condition)) throw std::runtime_error("decode_test input: " #condition); \
  } while (0)
#define EXPECT_TRUE(condition) JXL_DTI_EXPECT(condition)
#define ASSERT_TRUE(condition) JXL_DTI_EXPECT(condition)
#define EXPECT_EQ(a, b) JXL_DTI_EXPECT((a) == (b))
#define ASSERT_EQ(a, b) JXL_DTI_EXPECT((a) == (b))
#define ASSERT_LE(a, b) JXL_DTI_EXPECT((a) <= (b))

using ::jxl::test::GetIccTestProfile;

namespace {
#include "original_dt_constants.inc"
}  // namespace

namespace jxl {
namespace test {
#include "original_check_impl.inc"
#include "original_encoding_from_descriptor.inc"
#include "original_all_encodings.inc"
#include "original_encode_file.inc"
#include "original_jpeg_data_to_codec_in_out.inc"
#include "original_some_test_image.inc"

// The pinned testdata profile external/Compact-ICC-Profiles/profiles/scRGB-v2.icc,
// passed by the request.
IccBytes g_icc_test_profile;
IccBytes GetIccTestProfile() { return g_icc_test_profile; }
}  // namespace test

namespace {
// A genuine JPEG file replacing the extras libjpeg encoder output.
std::vector<uint8_t> g_jpeg_input;

#include "original_dt_codestream_head.inc"
    {
      std::vector<uint8_t> jpeg_bytes = g_jpeg_input;
#include "original_dt_codestream_jpeg.inc"
    }
  }
#include "original_dt_codestream_tail.inc"
}  // namespace
}  // namespace jxl

namespace {
#include "original_dt_test_header.inc"
#include "original_dt_analyze.inc"


std::string Token(const JxlColorEncoding& c) {
  auto bits = [](double v) {
    uint64_t b;
    memcpy(&b, &v, 8);
    return std::to_string(b);
  };
  std::ostringstream out;
  out << "x:" << c.color_space << ':' << c.white_point << ':' << bits(c.white_point_xy[0]) << ':'
      << bits(c.white_point_xy[1]) << ':' << c.primaries << ':' << bits(c.primaries_red_xy[0])
      << ':' << bits(c.primaries_red_xy[1]) << ':' << bits(c.primaries_green_xy[0]) << ':'
      << bits(c.primaries_green_xy[1]) << ':' << bits(c.primaries_blue_xy[0]) << ':'
      << bits(c.primaries_blue_xy[1]) << ':' << c.transfer_function << ':' << bits(c.gamma)
      << ':' << c.rendering_intent;
  return out.str();
}

// Progressive pass sets used by SkipCurrentFrameTest and ProgressiveEventTest.
const jxl::PassDefinition kSkipPasses[] = {{2, 0, 4}, {4, 0, 4}, {8, 2, 2}, {8, 0, 1}};
const jxl::PassDefinition kEventPasses[] = {{2, 0, 4}, {4, 0, 4}, {8, 2, 2}, {8, 1, 2}, {8, 0, 1}};
const jxl::ProgressiveMode kSkipProgressiveMode{kSkipPasses};
const jxl::ProgressiveMode kEventProgressiveMode{kEventPasses};

// dti-codestream W H CHANNELS SEED key=value...: CreateTestJXLCodestream on
// GetSomeTestImage(W, H, CHANNELS, SEED).
void Codestream(std::istringstream& in) {
  size_t xsize, ysize, channels, seed;
  in >> xsize >> ysize >> channels >> seed;
  jxl::TestCodestreamParams params;
  std::vector<uint8_t> jpeg_codestream;
  std::string token;
  while (in >> token) {
    auto eq = token.find('=');
    if (eq == std::string::npos) throw std::runtime_error("dti-codestream option");
    std::string key = token.substr(0, eq), value = token.substr(eq + 1);
    if (key == "box") params.box_format = CodeStreamBoxFormat(std::stoi(value));
    else if (key == "orient") params.orientation = JxlOrientation(std::stoi(value));
    else if (key == "preview") params.preview_mode = jxl::PreviewMode(std::stoi(value));
    else if (key == "intrinsic") params.add_intrinsic_size = value != "0";
    else if (key == "icc") params.add_icc_profile = value != "0";
    else if (key == "intensity") params.intensity_target = std::stof(value);
    else if (key == "cs") params.color_space = value;
    else if (key == "jpeg") {
      jxl::g_jpeg_input = Unhex(value);
      params.jpeg_codestream = &jpeg_codestream;
    } else if (key == "lossless") params.cparams.SetLossless();
    else if (key == "tier") params.cparams.speed_tier = jxl::SpeedTier(std::stoi(value));
    else if (key == "resampling") params.cparams.resampling = std::stoi(value);
    else if (key == "ec_resampling") params.cparams.ec_resampling = std::stoi(value);
    else if (key == "noise") params.cparams.noise = jxl::Override(std::stoi(value));
    else if (key == "pdc") params.cparams.progressive_dc = std::stoi(value);
    else if (key == "responsive") params.cparams.responsive = std::stoi(value);
    else if (key == "gshift") params.cparams.modular_group_size_shift = std::stoi(value);
    else if (key == "distance") params.cparams.butteraugli_distance = std::stof(value);
    else if (key == "ct") params.cparams.color_transform = jxl::ColorTransform(std::stoi(value));
    else if (key == "passes") {
      params.cparams.custom_progressive_mode =
          value == "skip" ? &kSkipProgressiveMode : &kEventProgressiveMode;
    } else throw std::runtime_error("dti-codestream key");
  }
  in.clear();
  std::vector<uint8_t> pixels = jxl::test::GetSomeTestImage(xsize, ysize, channels, seed);
  std::vector<uint8_t> compressed = jxl::CreateTestJXLCodestream(
      jxl::Bytes(pixels.data(), pixels.size()), xsize, ysize, channels, params);
  std::cout << Hex(jxl::Bytes(compressed));
  if (params.jpeg_codestream) std::cout << ' ' << Hex(jxl::Bytes(jpeg_codestream));
}

// Frame sequences copied from the decode_test.cc test bodies.
std::vector<uint8_t> Frames(const std::string& name, std::istringstream& in) {
  JxlMemoryManager* memory_manager = jxl::test::MemoryManager();
  auto io = jxl::make_unique<jxl::CodecInOut>(memory_manager);
  jxl::CompressParams cparams;
  if (name == "animation" || name == "skip" || name == "skip-current") {
    // AnimationTest, AnimationTestStreaming, SkipFrameTest, SkipCurrentFrameTest.
    size_t xsize = name == "animation" ? 123 : 90;
    size_t ysize = name == "animation" ? 77 : 120;
    size_t num_frames = name == "animation" ? 2 : name == "skip" ? 16 : 7;
    JxlPixelFormat format = {3, JXL_TYPE_UINT16, JXL_BIG_ENDIAN, 0};
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    io->metadata.m.SetUintSamples(16);
    io->metadata.m.color_encoding = jxl::ColorEncoding::SRGB(false);
    io->metadata.m.have_animation = true;
    io->frames.clear();
    io->frames.reserve(num_frames);
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    for (size_t i = 0; i < num_frames; ++i) {
      std::vector<uint8_t> frame = jxl::test::GetSomeTestImage(xsize, ysize, 3, i);
      jxl::ImageBundle bundle(memory_manager, &io->metadata.m);
      if (name != "animation" && (i & 1)) bundle.use_for_next_frame = true;
      EXPECT_TRUE(ConvertFromExternal(jxl::Bytes(frame.data(), frame.size()), xsize, ysize,
                                      jxl::ColorEncoding::SRGB(/*is_gray=*/false),
                                      /*bits_per_sample=*/16, format,
                                      /*pool=*/nullptr, &bundle));
      bundle.duration = 5 + i;
      io->frames.push_back(std::move(bundle));
    }
    if (name == "skip-current") {
      cparams.speed_tier = jxl::SpeedTier::kThunder;
      cparams.custom_progressive_mode = &kSkipProgressiveMode;
    } else {
      cparams.SetLossless();
      cparams.speed_tier = jxl::SpeedTier::kThunder;
    }
  } else if (name == "skip-current-ec") {
    // SkipCurrentFrameForgetsExtraChannelBuffers.
    size_t xsize = 90, ysize = 120, num_frames = 2;
    JxlPixelFormat format = {4, JXL_TYPE_UINT16, JXL_BIG_ENDIAN, 0};
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    io->metadata.m.SetUintSamples(16);
    io->metadata.m.SetAlphaBits(16);
    io->metadata.m.color_encoding = jxl::ColorEncoding::SRGB(false);
    io->metadata.m.have_animation = true;
    io->frames.clear();
    for (size_t i = 0; i < num_frames; ++i) {
      std::vector<uint8_t> pixels = jxl::test::GetSomeTestImage(xsize, ysize, 4, i);
      jxl::ImageBundle bundle(memory_manager, &io->metadata.m);
      EXPECT_TRUE(ConvertFromExternal(jxl::Bytes(pixels.data(), pixels.size()), xsize, ysize,
                                      jxl::ColorEncoding::SRGB(/*is_gray=*/false),
                                      /*bits_per_sample=*/16, format,
                                      /*pool=*/nullptr, &bundle, /*set_alpha=*/true));
      bundle.duration = 1;
      io->frames.push_back(std::move(bundle));
    }
    cparams.speed_tier = jxl::SpeedTier::kThunder;
  } else if (name == "skip-blend") {
    // SkipFrameWithBlendingTest.
    size_t xsize = 90, ysize = 120, num_frames = 16;
    JxlPixelFormat format = {3, JXL_TYPE_UINT16, JXL_BIG_ENDIAN, 0};
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    io->metadata.m.SetUintSamples(16);
    io->metadata.m.color_encoding = jxl::ColorEncoding::SRGB(false);
    io->metadata.m.have_animation = true;
    io->frames.clear();
    io->frames.reserve(num_frames);
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    for (size_t i = 0; i < num_frames; ++i) {
      if (i < 5) {
        std::vector<uint8_t> frame_internal =
            jxl::test::GetSomeTestImage(xsize, ysize, 3, i * 2 + 1);
        jxl::ImageBundle bundle_internal(memory_manager, &io->metadata.m);
        EXPECT_TRUE(ConvertFromExternal(
            jxl::Bytes(frame_internal.data(), frame_internal.size()), xsize, ysize,
            jxl::ColorEncoding::SRGB(/*is_gray=*/false),
            /*bits_per_sample=*/16, format,
            /*pool=*/nullptr, &bundle_internal));
        bundle_internal.duration = 0;
        bundle_internal.use_for_next_frame = true;
        io->frames.push_back(std::move(bundle_internal));
      }
      std::vector<uint8_t> frame = jxl::test::GetSomeTestImage(xsize, ysize, 3, i * 2);
      jxl::ImageBundle bundle(memory_manager, &io->metadata.m);
      EXPECT_TRUE(ConvertFromExternal(jxl::Bytes(frame.data(), frame.size()), xsize, ysize,
                                      jxl::ColorEncoding::SRGB(/*is_gray=*/false),
                                      /*bits_per_sample=*/16, format,
                                      /*pool=*/nullptr, &bundle));
      bundle.duration = 5 + i;
      if (i != 3 && i != 9 && i != 10) bundle.use_for_next_frame = true;
      if (i != 12) {
        bundle.blend = true;
        bundle.blendmode = jxl::BlendMode::kMul;
      }
      io->frames.push_back(std::move(bundle));
    }
    cparams.SetLossless();
    cparams.speed_tier = jxl::SpeedTier::kThunder;
  } else if (name == "skip-alpha-blend") {
    // SkipFrameWithAlphaBlendingTest.
    size_t xsize = 90, ysize = 120, num_frames = 16;
    JxlPixelFormat format = {4, JXL_TYPE_UINT16, JXL_BIG_ENDIAN, 0};
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    io->metadata.m.SetUintSamples(16);
    io->metadata.m.color_encoding = jxl::ColorEncoding::SRGB(false);
    io->metadata.m.have_animation = true;
    io->frames.clear();
    io->frames.reserve(num_frames + 5);
    JXL_DTI_EXPECT(io->SetSize(xsize, ysize));
    for (size_t i = 0; i < num_frames; ++i) {
      size_t cropxsize = 1 + xsize * 2 / (i + 1);
      size_t cropysize = 1 + ysize * 3 / (i + 2);
      int cropx0 = i * 3 - 8;
      int cropy0 = i * 4 - 7;
      if (i < 5) {
        std::vector<uint8_t> frame_internal =
            jxl::test::GetSomeTestImage(xsize / 2, ysize / 2, 4, i * 2 + 1);
        jxl::ImageBundle bundle_internal(memory_manager, &io->metadata.m);
        EXPECT_TRUE(ConvertFromExternal(
            jxl::Bytes(frame_internal.data(), frame_internal.size()), xsize / 2, ysize / 2,
            jxl::ColorEncoding::SRGB(/*is_gray=*/false),
            /*bits_per_sample=*/16, format,
            /*pool=*/nullptr, &bundle_internal));
        bundle_internal.duration = 0;
        bundle_internal.use_for_next_frame = true;
        bundle_internal.origin = {13, 17};
        io->frames.push_back(std::move(bundle_internal));
      }
      std::vector<uint8_t> frame = jxl::test::GetSomeTestImage(cropxsize, cropysize, 4, i * 2);
      jxl::ImageBundle bundle(memory_manager, &io->metadata.m);
      EXPECT_TRUE(ConvertFromExternal(jxl::Bytes(frame.data(), frame.size()), cropxsize,
                                      cropysize, jxl::ColorEncoding::SRGB(/*is_gray=*/false),
                                      /*bits_per_sample=*/16, format,
                                      /*pool=*/nullptr, &bundle));
      bundle.duration = 5 + i;
      bundle.origin = {cropx0, cropy0};
      if (i != 3 && i != 9 && i != 10) bundle.use_for_next_frame = true;
      if (i != 12) {
        bundle.blend = true;
        bundle.blendmode = jxl::BlendMode::kBlend;
      }
      io->frames.push_back(std::move(bundle));
    }
    cparams.SetLossless();
    cparams.speed_tier = jxl::SpeedTier::kThunder;
  } else if (name == "oriented-crop") {
    // OrientedCroppedFrameTest with ORIENTATION RESAMPLING.
    uint32_t orientation, resampling;
    in >> orientation >> resampling;
    size_t xsize = 90, ysize = 120;
    JxlPixelFormat format = {4, JXL_TYPE_UINT16, JXL_BIG_ENDIAN, 0};
    EXPECT_TRUE(io->SetSize(xsize, ysize));
    io->metadata.m.SetUintSamples(16);
    io->metadata.m.color_encoding = jxl::ColorEncoding::SRGB(false);
    io->metadata.m.orientation = orientation;
    io->frames.clear();
    EXPECT_TRUE(io->SetSize(xsize, ysize));
    for (size_t i = 0; i < 3; ++i) {
      size_t cropxsize = 1 + xsize * 2 / (i + 1);
      size_t cropysize = 1 + ysize * 3 / (i + 2);
      int cropx0 = i * 3 - 8;
      int cropy0 = i * 4 - 7;
      std::vector<uint8_t> frame = jxl::test::GetSomeTestImage(cropxsize, cropysize, 4, i * 2);
      jxl::ImageBundle bundle(memory_manager, &io->metadata.m);
      EXPECT_TRUE(ConvertFromExternal(jxl::Bytes(frame.data(), frame.size()), cropxsize,
                                      cropysize, jxl::ColorEncoding::SRGB(/*is_gray=*/false),
                                      /*bits_per_sample=*/16, format,
                                      /*pool=*/nullptr, &bundle));
      bundle.origin = {cropx0, cropy0};
      bundle.use_for_next_frame = true;
      io->frames.push_back(std::move(bundle));
    }
    cparams.SetLossless();
    cparams.speed_tier = jxl::SpeedTier::kThunder;
    cparams.resampling = resampling;
  } else if (name == "spot") {
    // SpotColorTest.
    size_t xsize = 55, ysize = 257;
    io->metadata.m.color_encoding = jxl::ColorEncoding::LinearSRGB();
    JXL_TEST_ASSIGN_OR_DIE(jxl::Image3F main, jxl::Image3F::Create(memory_manager, xsize, ysize));
    JXL_TEST_ASSIGN_OR_DIE(jxl::ImageF spot, jxl::ImageF::Create(memory_manager, xsize, ysize));
    jxl::ZeroFillImage(&main);
    jxl::ZeroFillImage(&spot);
    for (size_t y = 0; y < ysize; y++) {
      float* JXL_RESTRICT rowm = main.PlaneRow(1, y);
      float* JXL_RESTRICT rows = spot.Row(y);
      for (size_t x = 0; x < xsize; x++) {
        rowm[x] = (x + y) * (1.f / 255.f);
        rows[x] = ((x ^ y) & 255) * (1.f / 255.f);
      }
    }
    ASSERT_TRUE(io->SetFromImage(std::move(main), jxl::ColorEncoding::LinearSRGB()));
    jxl::ExtraChannelInfo info;
    info.bit_depth.bits_per_sample = 8;
    info.dim_shift = 0;
    info.type = jxl::ExtraChannel::kSpotColor;
    info.spot_color[0] = 0.5f;
    info.spot_color[1] = 0.2f;
    info.spot_color[2] = 1.f;
    info.spot_color[3] = 0.5f;
    io->metadata.m.extra_channel_info.push_back(info);
    std::vector<jxl::ImageF> ec;
    ec.push_back(std::move(spot));
    ASSERT_TRUE(io->frames[0].SetExtraChannels(std::move(ec)));
    cparams.speed_tier = jxl::SpeedTier::kLightning;
    cparams.modular_mode = true;
    cparams.color_transform = jxl::ColorTransform::kNone;
    cparams.butteraugli_distance = 0.f;
  } else if (name == "jpeg-recon") {
    // JPEGReconstructionTest: a container built from a JPEG file.
    std::string hex;
    in >> hex;
    const std::vector<uint8_t> orig = Unhex(hex);
    auto orig_io = jxl::make_unique<jxl::CodecInOut>(memory_manager);
    JXL_TEST_ASSIGN_OR_DIE(std::unique_ptr<jxl::jpeg::JPEGData> jpeg_data,
                           jxl::jpeg::ParseJPG(memory_manager, jxl::Bytes(orig)));
    ASSERT_TRUE(jxl::test::JpegDataToCodecInOut(std::move(jpeg_data), orig_io.get()));
    jxl::jpeg::JPEGData jpeg_data_copy = *orig_io->Main().jpeg_data;
    orig_io->metadata.m.xyb_encoded = false;
    jxl::BitWriter writer{memory_manager};
    ASSERT_TRUE(WriteCodestreamHeaders(&orig_io->metadata, &writer, nullptr));
    writer.ZeroPadToByte();
    cparams.color_transform = jxl::ColorTransform::kNone;
    ASSERT_TRUE(jxl::EncodeFrame(memory_manager, cparams, jxl::FrameInfo{}, &orig_io->metadata,
                                 orig_io->Main(), *JxlGetDefaultCms(),
                                 /*pool=*/nullptr, &writer,
                                 /*aux_out=*/nullptr));
    std::vector<uint8_t> encoded_jpeg_data;
    ASSERT_TRUE(EncodeJPEGData(memory_manager, jpeg_data_copy, &encoded_jpeg_data, cparams));
    std::vector<uint8_t> container = jxl::MakeContainerHeader(0);
    jxl::AppendBoxHeader(jxl::MakeBoxType("jbrd"), encoded_jpeg_data.size(), false, &container);
    jxl::Bytes(encoded_jpeg_data).AppendTo(container);
    jxl::AppendBoxHeader(jxl::MakeBoxType("jxlc"), 0, true, &container);
    jxl::PaddedBytes codestream = std::move(writer).TakeBytes();
    jxl::Bytes(codestream).AppendTo(container);
    return container;
  } else {
    throw std::runtime_error("dti-frames name");
  }
  std::vector<uint8_t> compressed;
  EXPECT_TRUE(jxl::test::EncodeFile(cparams, io.get(), &compressed));
  return compressed;
}

void Analyze(const std::vector<uint8_t>& data) {
  StreamPositions streampos;
  AnalyzeCodestream(data, &streampos);
  std::cout << streampos.codestream_start << ' ' << streampos.codestream_end << ' '
            << streampos.basic_info << ' ' << streampos.jbrd_end << ' '
            << streampos.box_start.size();
  for (size_t b : streampos.box_start) std::cout << ' ' << b;
  std::cout << ' ' << streampos.frames.size();
  for (const auto& f : streampos.frames) {
    std::cout << ' ' << f.frame_start << ' ' << f.header_end << ' ' << f.toc_end << ' '
              << f.section_end.size();
    for (size_t s : f.section_end) std::cout << ' ' << s;
  }
}

// One encoding per line item: description, filter properties, the external
// encoding, ParseDescription(Description) and the two ICC profiles the tests
// create from it.
void Encoding(const jxl::test::ColorEncodingDescriptor& desc) {
  jxl::ColorEncoding c = jxl::test::ColorEncodingFromDescriptor(desc);
  std::string description = Description(c);
  JxlColorEncoding parsed;
  JXL_DTI_EXPECT(jxl::ParseDescription(description, &parsed));
  jxl::ColorEncoding internal_parsed;
  JXL_DTI_EXPECT(internal_parsed.FromExternal(parsed));
  JXL_DTI_EXPECT(internal_parsed.CreateICC());
  std::cout << description << ' ' << int(c.GetColorSpace()) << ' '
            << int(c.GetWhitePointType()) << ' ' << int(c.GetPrimariesType()) << ' '
            << int(c.Tf().IsPQ()) << ' ' << int(c.Tf().IsHLG()) << ' '
            << int(c.GetRenderingIntent()) << ' ' << c.Channels() << ' '
            << Token(c.ToExternal()) << ' ' << Token(parsed) << ' '
            << Hex(jxl::Bytes(c.ICC())) << ' ' << Hex(jxl::Bytes(internal_parsed.ICC()));
}

}  // namespace

bool DecodeTestInputsCommand(const std::string& command, std::istringstream& in,
                             JxlMemoryManager*) {
  if (command.rfind("dti-", 0) != 0) return false;
  std::string icc;
  if (command != "dti-analyze" && command != "dti-encodings" && command != "dti-external") {
    in >> icc;
    jxl::test::g_icc_test_profile = Unhex(icc);
  }
  if (command == "dti-codestream") {
    Codestream(in);
  } else if (command == "dti-header") {
    size_t xsize, ysize, bits, orientation, alpha_bits;
    int xyb, container, metadata_default, insert_box, with_icc;
    in >> xsize >> ysize >> bits >> orientation >> alpha_bits >> xyb >> container >>
        metadata_default >> insert_box >> with_icc;
    jxl::IccBytes icc_profile;
    if (with_icc) icc_profile = jxl::test::GetIccTestProfile();
    std::cout << Hex(jxl::Bytes(GetTestHeader(xsize, ysize, bits, orientation, alpha_bits,
                                              xyb != 0, container != 0, metadata_default != 0,
                                              insert_box != 0, icc_profile)));
  } else if (command == "dti-frames") {
    std::string name;
    in >> name;
    std::cout << Hex(jxl::Bytes(Frames(name, in)));
  } else if (command == "dti-analyze") {
    std::string hex;
    in >> hex;
    Analyze(Unhex(hex));
  } else if (command == "dti-encodings") {
    // AllEncodings, then the gray source of SetPreferredColorProfileTestFromGray.
    std::vector<jxl::test::ColorEncodingDescriptor> all = jxl::test::AllEncodings();
    all.push_back({jxl::ColorSpace::kGray, jxl::WhitePoint::kD65, jxl::Primaries::kSRGB,
                   jxl::TransferFunction::kSRGB, jxl::RenderingIntent::kRelative});
    for (size_t i = 0; i < all.size(); ++i) {
      if (i) std::cout << '|';
      Encoding(all[i]);
    }
  } else if (command == "dti-external") {
    // The preferred profiles of IccProfileTestXybEncoded.
    std::string which;
    in >> which;
    jxl::ColorEncoding c = which == "srgb" ? jxl::ColorEncoding::SRGB(false)
                                           : jxl::ColorEncoding::LinearSRGB(false);
    std::cout << Token(c.ToExternal());
  } else {
    throw std::runtime_error("dti command");
  }
  return true;
}
