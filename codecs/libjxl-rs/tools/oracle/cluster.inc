// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol adapters; all math and clustering execute original source functions.

#include "lib/jxl/base/fast_math-inl.h"
#include "lib/jxl/enc_cluster.h"

namespace jxl { namespace HWY_NAMESPACE {
float HistogramDistance(const Histogram&, const Histogram&);
float HistogramKLDivergence(const Histogram&, const Histogram&);
}}

uint32_t FloatBits(float value) { uint32_t bits; std::memcpy(&bits, &value, 4); return bits; }

jxl::Histogram ReadHistogramArgs(std::istringstream& in, float cache) {
  size_t size; in >> size;
  jxl::Histogram result;
  result.counts.resize(size); result.entropy = cache;
  for (auto& count : result.counts) { in >> count; result.total_count += count; }
  return result;
}

void HistogramObservation(const jxl::Histogram& histogram) {
  std::cout << ' ' << histogram.counts.size() << ',' << histogram.total_count << ',' << FloatBits(histogram.entropy);
  for (int count : histogram.counts) std::cout << ',' << count;
}

bool ClusterCommand(const std::string& command, std::istringstream& in) {
  if (command == "fast-log") {
    uint32_t bits; in >> bits; float value; std::memcpy(&value, &bits, 4);
    std::cout << FloatBits(jxl::FastLog2f(value));
  } else if (command == "hist-measures") {
    auto a = ReadHistogramArgs(in, 1.25f), b = ReadHistogramArgs(in, 1.25f);
    a.Condition(); b.Condition();
    HistogramObservation(a); HistogramObservation(b);
    std::cout << ' ' << FloatBits(a.ShannonEntropy()) << ' ' << FloatBits(b.ShannonEntropy());
    std::cout << ' ' << FloatBits(jxl::HWY_NAMESPACE::HistogramDistance(a,b))
              << ' ' << FloatBits(jxl::HWY_NAMESPACE::HistogramKLDivergence(a,b))
              << ' ' << FloatBits(jxl::HWY_NAMESPACE::HistogramKLDivergence(b,a));
  } else if (command == "cluster") {
    size_t clustering, max_histograms, max_param, n, previous;
    bool prefix; in >> clustering >> max_histograms >> max_param >> prefix >> n >> previous;
    jxl::HistogramParams params;
    params.clustering = static_cast<jxl::HistogramParams::ClusteringType>(clustering);
    params.max_histograms = max_param; params.force_huffman = prefix;
    std::vector<jxl::Histogram> input, out;
    for (size_t i = 0; i < n; ++i) input.push_back(ReadHistogramArgs(in, 0.25f));
    for (size_t i = 0; i < previous; ++i) out.push_back(ReadHistogramArgs(in, 0.75f));
    std::vector<uint32_t> symbols;
    Status status = jxl::ClusterHistograms(params,input,max_histograms,&out,&symbols);
    std::cout << Code(status) << ' ' << symbols.size();
    for (auto symbol : symbols) std::cout << ' ' << symbol;
    std::cout << ' ' << input.size();
    for (const auto& histogram : input) HistogramObservation(histogram);
    std::cout << ' ' << out.size();
    for (const auto& histogram : out) HistogramObservation(histogram);
  } else if (command == "hist-params") {
    int tier; size_t method; in >> tier >> method;
    jxl::HistogramParams params(static_cast<jxl::SpeedTier>(tier),17);
    std::cout << int(params.clustering) << ' ' << int(params.uint_method) << ' '
              << int(params.lz77_method) << ' ' << int(params.ans_histogram_strategy);
    params.uint_method = static_cast<jxl::HistogramParams::HybridUintMethod>(method);
    UintConfigObservation(params.UintConfig());
  } else return false;
  return true;
}
