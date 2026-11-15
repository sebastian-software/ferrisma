// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Requests for the encoder's smaller SIMD exports (port ADR 0004, "Encoder
// path", package `encoder entropy`), observed at the oracle's single
// Highway target. The kernels are the original's public, dispatching
// functions (HWY_DYNAMIC_DISPATCH reaches the one compiled target) or, where
// the original exports none, its target-namespace functions
// (ORACLE_HWY_NS); this TU supplies no kernel arithmetic. Integer words are
// printed as signed decimals, float words as their bits.
//   ene-rct <mode> <rct_type> <w> <h> <3 * h * RoundUpTo(w, 8) words>
//       mode 0: FwdRct(image, 0, rct_type, nullptr) on a 3-channel w x h
//       modular image whose rows hold the request's words in their first
//       RoundUpTo(w, 8) samples (the row padding included: the SIMD rows
//       load and store whole vectors up to RoundUpTo(w, Lanes)). Mode 1:
//       FwdRct({in}, {out}, rct_type, nullptr) from such an image into a
//       second one filled with sentinels. Prints the status and the output
//       channels' first RoundUpTo(w, 8) samples of every row.
// A histogram <h> is `<total_count> <entropy bits> <n> <n counts>` (the
// request keeps n a multiple of 8, as Histogram::Add and EnsureCapacity do,
// so the original's whole-vector loads stay inside the counts); a printed
// histogram is ` h <size> <counts> <total_count> <entropy bits>`.
//   ene-hcond <h>   Histogram::Condition(); prints the histogram.
//   ene-hent <h>    Histogram::ShannonEntropy(); prints it and the histogram.
//   ene-hdist <h> <h>
//       HistogramDistance(a, b) and HistogramKLDivergence(a, b) of the target
//       namespace (not exports).
//   ene-fast <max_histograms> <prefix_codes> <n> <n h> <m> <m h>
//       FastClusterHistograms of the target namespace (a local export) from
//       n histograms into m previous ones.
//   ene-cluster <clustering> <force_huffman> <params.max_histograms>
//               <max_histograms> <n> <n h> <m> <m h>
//       ClusterHistograms (clustering 0: kFastest, 1: kFast, 2: kBest).
//   Both print the status, the symbols (three sentinel 7s before the call),
//   the input histograms' entropy caches and the output histograms.
#include <jxl/memory_manager.h>

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

#include "lib/jxl/base/status.h"
#include "lib/jxl/ac_context.h"
#include "lib/jxl/ac_strategy.h"
#include "lib/jxl/base/rect.h"
#include "lib/jxl/coeff_order.h"
#include "lib/jxl/coeff_order_fwd.h"
#include "lib/jxl/dec_ans.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/enc_ans_params.h"
#include "lib/jxl/enc_ans_simd.h"
#include "lib/jxl/enc_entropy_coder.h"
#include "lib/jxl/enc_modular_simd.h"
#include "lib/jxl/frame_header.h"
#include "lib/jxl/image.h"
#include "lib/jxl/memory_manager_internal.h"
#include "lib/jxl/enc_cluster.h"
#include "lib/jxl/modular/modular_image.h"
#include "lib/jxl/modular/transform/enc_rct.h"
#include "lib/jxl/modular/transform/transform.h"

namespace jxl {
namespace ORACLE_HWY_NS {
float HistogramDistance(const Histogram& a, const Histogram& b);
float HistogramKLDivergence(const Histogram& actual, const Histogram& coding);
Status FastClusterHistograms(const std::vector<Histogram>& in, size_t max_histograms,
                             bool prefix_codes, std::vector<Histogram>* out,
                             std::vector<uint32_t>* histogram_symbols);
}  // namespace ORACLE_HWY_NS
}  // namespace jxl

namespace {
constexpr size_t kMaxLanes = 8;

size_t EneRoundUp8(size_t x) { return (x + kMaxLanes - 1) / kMaxLanes * kMaxLanes; }

size_t EneCount(std::istringstream& in, size_t limit) {
  size_t n;
  in >> n;
  if (in.fail() || n > limit) throw std::runtime_error("encoder entropy: count");
  return n;
}
int32_t EneInt(std::istringstream& in) {
  int64_t v;
  in >> v;
  return static_cast<int32_t>(v);
}
// Distinct sentinels keep unwritten samples observable.
int32_t EneSentinel(uint32_t salt, size_t i) {
  return static_cast<int32_t>(0x5a000000u + salt * 0x100000u + uint32_t(i & 0xfffff));
}

jxl::Image EneModularImage(JxlMemoryManager* mm, size_t w, size_t h) {
  auto created = jxl::Image::Create(mm, w, h, 16, 3);
  if (!created.ok()) throw std::runtime_error("encoder entropy: image");
  jxl::Image image = std::move(created).value_();
  for (const jxl::Channel& ch : image.channel)
    if (h > 0 && size_t(ch.plane.PixelsPerRow()) < EneRoundUp8(w))
      throw std::runtime_error("encoder entropy: row padding below 8 lanes");
  return image;
}
void EnePrintChannels(const jxl::Image& image, size_t padded) {
  for (const jxl::Channel& ch : image.channel) {
    std::cout << " C";
    for (size_t y = 0; y < ch.h; ++y)
      for (size_t x = 0; x < padded; ++x) std::cout << ' ' << ch.Row(y)[x];
  }
}
uint32_t EneBits(float v) {
  uint32_t bits;
  std::memcpy(&bits, &v, sizeof(bits));
  return bits;
}
jxl::Histogram EneHistogram(std::istringstream& in) {
  jxl::Histogram h;
  uint64_t total;
  uint32_t entropy;
  in >> total >> entropy;
  h.total_count = total;
  std::memcpy(&h.entropy, &entropy, sizeof(float));
  const size_t n = EneCount(in, 1 << 16);
  if (n % kMaxLanes != 0) throw std::runtime_error("encoder entropy: histogram length");
  h.counts.resize(n);
  for (int32_t& c : h.counts) c = EneInt(in);
  return h;
}
void EnePrintHistogram(const jxl::Histogram& h) {
  std::cout << " h " << h.counts.size();
  for (int32_t c : h.counts) std::cout << ' ' << c;
  std::cout << ' ' << uint64_t(h.total_count) << ' ' << EneBits(h.entropy);
}
}  // namespace

bool EncoderEntropySimdCommand(const std::string& command, std::istringstream& in,
                               JxlMemoryManager* mm) {
  if (command.rfind("ene-", 0) != 0) return false;
  if (command == "ene-rct") {
    const size_t mode = EneCount(in, 1);
    const size_t rct_type = EneCount(in, 41);
    const size_t w = EneCount(in, 4096);
    const size_t h = EneCount(in, 64);
    const size_t padded = EneRoundUp8(w);
    jxl::Image from = EneModularImage(mm, w, h);
    for (jxl::Channel& ch : from.channel)
      for (size_t y = 0; y < h; ++y)
        for (size_t x = 0; x < padded; ++x) ch.Row(y)[x] = EneInt(in);
    jxl::Status status = true;
    if (mode == 0) {
      status = jxl::FwdRct(from, 0, rct_type, nullptr);
      std::cout << "S " << int(status.code());
      EnePrintChannels(from, padded);
    } else {
      jxl::Image to = EneModularImage(mm, w, h);
      for (size_t c = 0; c < 3; ++c)
        for (size_t y = 0; y < h; ++y)
          for (size_t x = 0; x < padded; ++x)
            to.channel[c].Row(y)[x] = EneSentinel(c, y * padded + x);
      status = jxl::FwdRct({&from.channel[0], &from.channel[1], &from.channel[2]},
                           {&to.channel[0], &to.channel[1], &to.channel[2]}, rct_type, nullptr);
      std::cout << "S " << int(status.code());
      EnePrintChannels(to, padded);
    }
    return true;
  }
  if (command == "ene-hcond" || command == "ene-hent") {
    jxl::Histogram h = EneHistogram(in);
    if (command == "ene-hcond") {
      h.Condition();
      std::cout << "H";
      EnePrintHistogram(h);
    } else {
      const float entropy = h.ShannonEntropy();
      std::cout << "E " << EneBits(entropy);
      EnePrintHistogram(h);
    }
    return true;
  }
  if (command == "ene-hdist") {
    const jxl::Histogram a = EneHistogram(in);
    const jxl::Histogram b = EneHistogram(in);
    std::cout << "D " << EneBits(jxl::ORACLE_HWY_NS::HistogramDistance(a, b)) << ' '
              << EneBits(jxl::ORACLE_HWY_NS::HistogramKLDivergence(a, b));
    return true;
  }
  if (command == "ene-fast" || command == "ene-cluster") {
    jxl::HistogramParams params;
    size_t max_histograms;
    bool prefix_codes = false;
    if (command == "ene-fast") {
      max_histograms = EneCount(in, 1 << 20);
      prefix_codes = EneCount(in, 1) != 0;
    } else {
      const size_t clustering = EneCount(in, 2);
      params.clustering = clustering == 0   ? jxl::HistogramParams::ClusteringType::kFastest
                          : clustering == 1 ? jxl::HistogramParams::ClusteringType::kFast
                                            : jxl::HistogramParams::ClusteringType::kBest;
      params.force_huffman = EneCount(in, 1) != 0;
      params.max_histograms = EneCount(in, 1 << 20);
      max_histograms = EneCount(in, 1 << 20);
    }
    std::vector<jxl::Histogram> histograms(EneCount(in, 4096));
    for (jxl::Histogram& h : histograms) h = EneHistogram(in);
    std::vector<jxl::Histogram> out(EneCount(in, 4096));
    for (jxl::Histogram& h : out) h = EneHistogram(in);
    std::vector<uint32_t> symbols = {7, 7, 7};
    const jxl::Status status =
        command == "ene-fast"
            ? jxl::ORACLE_HWY_NS::FastClusterHistograms(histograms, max_histograms, prefix_codes,
                                                        &out, &symbols)
            : jxl::ClusterHistograms(params, histograms, max_histograms, &out, &symbols);
    std::cout << "S " << int(status.code()) << " Y " << symbols.size();
    for (uint32_t s : symbols) std::cout << ' ' << s;
    std::cout << " I";
    for (const jxl::Histogram& h : histograms) std::cout << ' ' << EneBits(h.entropy);
    std::cout << " O " << out.size();
    for (const jxl::Histogram& h : out) EnePrintHistogram(h);
    return true;
  }
  throw std::runtime_error("unknown encoder entropy SIMD command");
}
