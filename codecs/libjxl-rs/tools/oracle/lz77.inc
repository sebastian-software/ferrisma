// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Include the pristine source in this test translation unit so its local cost
// model can be observed directly. No algorithms are copied into the adapter.
#include "lib/jxl/enc_lz77.cc"

std::vector<std::vector<jxl::Token>> ReadTokenStreams(std::istringstream& in, bool flags) {
  size_t count; in >> count;
  std::vector<std::vector<jxl::Token>> streams(count);
  for (auto& stream : streams) {
    size_t size; in >> size;
    for (size_t i = 0; i < size; ++i) {
      uint32_t ctx, value, flag = 0; in >> ctx >> value;
      if (flags) in >> flag;
      stream.emplace_back(ctx,value); stream.back().is_lz77_length = flag;
    }
  }
  return streams;
}

bool LZ77Command(const std::string& command, std::istringstream& in) {
  if (command == "lz-static") {
    size_t len, dist; in >> len >> dist;
    std::cout << FloatBits(jxl::LenCost(len)) << ' ' << FloatBits(jxl::DistCost(dist));
  } else if (command == "lz-cost" || command == "lz-apply") {
    size_t method = 0, contexts;
    bool prefix;
    if (command == "lz-apply") in >> method;
    in >> prefix >> contexts;
    jxl::HistogramParams params; params.force_huffman = prefix;
    params.lz77_method = static_cast<jxl::HistogramParams::LZ77Method>(method);
    if (command == "lz-apply") {
      size_t size; in >> size; params.image_widths.resize(size);
      for (auto& width : params.image_widths) in >> width;
    }
    auto tokens = ReadTokenStreams(in, command == "lz-cost");
    jxl::LZ77Params lz77;
    lz77.min_symbol = prefix ? 512 : 224;
    lz77.nonserialized_distance_context = contexts;
    if (command == "lz-apply") {
      auto streams = jxl::ApplyLZ77(params,contexts,tokens,lz77);
      std::cout << streams.size();
      for (const auto& stream : streams) {
        std::cout << ' ' << stream.size();
        for (const auto& token : stream)
          std::cout << ' ' << token.context << ',' << token.value << ',' << token.is_lz77_length;
      }
    } else {
      jxl::SymbolCostEstimator model(contexts,prefix,tokens,lz77);
      std::vector<jxl::Histogram> histograms(contexts);
      size_t alphabet = 0;
      for (const auto& stream : tokens) for (const auto& token : stream) {
        uint32_t tok, nbits, bits;
        (token.is_lz77_length ? lz77.length_uint_config : jxl::HybridUintConfig()).Encode(token.value,&tok,&nbits,&bits);
        tok += token.is_lz77_length ? lz77.min_symbol : 0;
        histograms[token.context].Add(tok);
        alphabet = std::max(alphabet,histograms[token.context].counts.size());
      }
      std::cout << contexts << ' ' << alphabet;
      for (size_t ctx = 0; ctx < contexts; ++ctx) {
        std::cout << ' ' << FloatBits(model.AddSymbolCost(ctx));
        for (size_t sym = 0; sym < alphabet; ++sym)
          std::cout << ' ' << FloatBits(model.Bits(ctx,sym));
      }
    }
  } else return false;
  return true;
}
