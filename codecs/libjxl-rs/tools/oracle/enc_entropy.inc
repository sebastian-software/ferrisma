// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Exercise complete source entry points; no outer entropy framing is supplied.
#include "lib/jxl/enc_context_map.h"

bool EncodeEntropyCommand(const std::string& command, std::istringstream& in,
                          JxlMemoryManager* mm) {
  if (command == "entropy-build") {
    size_t mode, cluster, uint_method, lz_method, strategy, contexts, width, prefix;
    bool streaming, force, missing, fixed;
    in >> mode >> streaming >> force >> cluster >> uint_method >> lz_method
       >> strategy >> missing >> fixed >> contexts >> width >> prefix;
    auto tokens = ReadTokenStreams(in,false);
    jxl::HistogramParams params;
    params.streaming_mode = streaming; params.force_huffman = force;
    params.clustering = static_cast<jxl::HistogramParams::ClusteringType>(cluster);
    params.uint_method = static_cast<jxl::HistogramParams::HybridUintMethod>(uint_method);
    params.lz77_method = static_cast<jxl::HistogramParams::LZ77Method>(lz_method);
    params.ans_histogram_strategy = static_cast<jxl::HistogramParams::ANSHistogramStrategy>(strategy);
    params.add_missing_symbols = missing; params.add_fixed_histograms = fixed;
    params.image_widths.assign(tokens.size(),width);
    jxl::EntropyEncodingData codes;
    jxl::AuxOut aux;
    BitWriter writer(mm); if (!Prefix(writer,prefix)) throw std::runtime_error("entropy prefix");
    auto cost = jxl::BuildAndEncodeHistograms(mm,params,contexts,tokens,&codes,
                       mode == 1 ? &writer : nullptr,LayerType::Ac,&aux);
    Status status = cost.ok() ? Status(true) : cost.status();
    std::cout << Code(status) << ' ';
    if (cost.ok()) std::cout << std::move(cost).value_(); else std::cout << '-';
    if (status && mode == 2) status = jxl::EncodeHistograms(codes,&writer,LayerType::Ac,&aux);
    if (status && mode != 0) for (const auto& stream : tokens)
      if (!(status = jxl::WriteTokens(stream,codes,0,&writer,LayerType::Ac,&aux))) break;
    std::cout << ' ' << Code(status) << ' ' << writer.BitsWritten();
    if (!Pad(writer)) throw std::runtime_error("entropy padding");
    std::cout << ' ' << Hex(writer.GetSpan()) << ' ' << codes.use_prefix_code
              << ' ' << codes.log_alpha_size << ' ' << codes.lz77.enabled
              << ' ' << codes.lz77.min_symbol << ' ' << codes.lz77.min_length
              << ' ' << codes.lz77.nonserialized_distance_context;
    UintConfigObservation(codes.lz77.length_uint_config);
    std::cout << ' ' << codes.uint_config.size();
    for (const auto& cfg : codes.uint_config) UintConfigObservation(cfg);
    std::cout << ' ' << codes.context_map.size();
    for (auto c : codes.context_map) std::cout << ' ' << unsigned(c);
    std::cout << ' ' << codes.encoding_info.size();
    for (const auto& histogram : codes.encoding_info) {
      std::cout << ' ' << histogram.size();
      for (const auto& info : histogram)
        std::cout << ' ' << info.freq_ << ',' << unsigned(info.depth) << ',' << info.bits;
    }
    std::cout << ' ' << codes.encoded_histograms.size();
    for (auto& histo : codes.encoded_histograms) {
      std::cout << ' ' << histo.BitsWritten();
      if (!Pad(histo)) throw std::runtime_error("histogram padding");
      std::cout << ' ' << Hex(histo.GetSpan());
    }
    const auto& totals = aux.layer(LayerType::Ac);
    uint64_t entropy_bits; std::memcpy(&entropy_bits,&totals.clustered_entropy,sizeof(entropy_bits));
    std::cout << ' ' << totals.num_clustered_histograms << ' ' << totals.extra_bits
              << ' ' << totals.histogram_bits << ' ' << totals.total_bits << ' ' << entropy_bits;
    std::cout << ' ' << tokens.size();
    for (const auto& stream : tokens) {
      std::cout << ' ' << stream.size();
      for (const auto& token : stream)
        std::cout << ' ' << token.context << ',' << token.value << ',' << token.is_lz77_length;
    }
  } else if (command == "entropy-batches") {
    size_t contexts,batches; bool force,fixed;
    in >> contexts >> batches >> force >> fixed;
    jxl::EntropyEncodingData codes;
    jxl::HistogramParams params;
    params.streaming_mode = true; params.force_huffman = force;
    params.lz77_method = jxl::HistogramParams::LZ77Method::kNone;
    std::vector<std::vector<jxl::Token>> streams;
    std::vector<size_t> offsets;
    jxl::AuxOut aux;
    for (size_t batch = 0; batch < batches; ++batch) {
      params.initialize_global_state = batch == 0;
      params.add_fixed_histograms = fixed && batch == 0;
      auto tokens = ReadTokenStreams(in,false);
      offsets.push_back(codes.context_map.size());
      auto cost = jxl::BuildAndEncodeHistograms(mm,params,contexts,tokens,&codes,nullptr,LayerType::Ac,&aux);
      if (!cost.ok()) throw std::runtime_error("batch histogram status");
      std::cout << std::move(cost).value_() << ' ' << codes.encoding_info.size() << ' ';
      streams.push_back(std::move(tokens[0]));
    }
    BitWriter writer(mm);
    Status status = jxl::EncodeHistograms(codes,&writer,LayerType::Ac,&aux);
    for (size_t batch = 0; status && batch < batches; ++batch)
      status = jxl::WriteTokens(streams[batch],codes,offsets[batch],&writer,LayerType::Ac,&aux);
    std::cout << Code(status) << ' ' << writer.BitsWritten();
    if (!Pad(writer)) throw std::runtime_error("batch padding");
    std::cout << ' ' << Hex(writer.GetSpan()) << ' ' << codes.context_map.size();
    for (auto c : codes.context_map) std::cout << ' ' << unsigned(c);
    const auto& totals = aux.layer(LayerType::Ac);
    uint64_t bits; std::memcpy(&bits,&totals.clustered_entropy,sizeof(bits));
    std::cout << ' ' << totals.num_clustered_histograms << ' ' << totals.extra_bits
              << ' ' << totals.histogram_bits << ' ' << totals.total_bits << ' ' << bits;
  } else if (command == "context-write") {
    size_t histograms, prefix, size; in >> histograms >> prefix >> size;
    std::vector<uint8_t> map(size);
    for (auto& item : map) { unsigned v; in >> v; item = v; }
    BitWriter writer(mm); if (!Prefix(writer,prefix)) throw std::runtime_error("context prefix");
    jxl::AuxOut aux;
    Status status = writer.WithMaxBits(3+size*24+32768,LayerType::Ac,&aux,[&] {
      return jxl::EncodeContextMap(map,histograms,&writer,LayerType::Ac,&aux);
    });
    std::cout << Code(status) << ' ' << writer.BitsWritten();
    if (!Pad(writer)) throw std::runtime_error("context padding");
    std::cout << ' ' << Hex(writer.GetSpan());
    const auto& totals = aux.layer(LayerType::Ac);
    std::cout << ' ' << totals.num_clustered_histograms << ' ' << totals.extra_bits
              << ' ' << totals.histogram_bits << ' ' << totals.total_bits;
  } else if (command == "entropy-read") {
    size_t contexts,width,prefix; std::string hex;
    in >> contexts >> width >> prefix >> hex;
    auto streams = ReadTokenStreams(in,false);
    Input input(hex); BitReader br(input.bytes()); br.SkipBits(prefix);
    jxl::ANSCode code; std::vector<uint8_t> map;
    Status status = jxl::DecodeHistograms(mm,&br,contexts,&code,&map);
    std::cout << Code(status) << ' ' << br.TotalBitsConsumed();
    if (status) for (const auto& stream : streams) {
      auto created = jxl::ANSSymbolReader::Create(&code,&br,width);
      if (!created.ok()) throw std::runtime_error("entropy reader allocation");
      auto reader = std::move(created).value_();
      for (const auto& token : stream) std::cout << ' ' << reader.ReadHybridUint(token.context,&br,map);
      std::cout << ' ' << reader.CheckANSFinalState() << ' ' << br.TotalBitsConsumed();
    }
    std::cout << ' ' << Code(br.Close());
  } else return false;
  return true;
}
