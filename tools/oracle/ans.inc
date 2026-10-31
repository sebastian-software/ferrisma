// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol adapters only; entropy algorithms execute unmodified upstream code.

#include "lib/jxl/dec_ans.h"
#include "lib/jxl/dec_context_map.h"
#include "lib/jxl/enc_ans.h"
#include "lib/jxl/inverse_mtf-inl.h"

namespace jxl {
Status DecodeANSCodes(JxlMemoryManager*, size_t, size_t, BitReader*, ANSCode*);
}

jxl::HybridUintConfig ReadUintConfigArgs(std::istringstream& in) {
  uint32_t e, m, l;
  in >> e >> m >> l;
  return jxl::HybridUintConfig(e, m, l);
}

void UintConfigObservation(const jxl::HybridUintConfig& cfg) {
  std::cout << ' ' << cfg.split_exponent << ',' << cfg.split_token << ','
            << cfg.msb_in_token << ',' << cfg.lsb_in_token;
}

void ANSCodeObservation(const jxl::ANSCode& code, bool initialized_tables) {
  std::cout << ' ' << code.use_prefix_code << ' ' << unsigned(code.log_alpha_size)
            << ' ' << code.max_num_bits << ' ' << code.lz77.enabled << ' '
            << code.lz77.min_symbol << ' ' << code.lz77.min_length << ' '
            << code.lz77.nonserialized_distance_context;
  UintConfigObservation(code.lz77.length_uint_config);
  std::cout << ' ' << code.uint_config.size();
  for (const auto& cfg : code.uint_config) UintConfigObservation(cfg);
  std::cout << ' ' << code.degenerate_symbols.size();
  for (int symbol : code.degenerate_symbols) std::cout << ' ' << symbol;
  std::cout << ' ' << code.huffman_data.size();
  for (const auto& data : code.huffman_data) HuffmanTableObservation(data.table_);
  // A failed source allocation/table build may leave entries uninitialized.
  // Only completed table arrays are valid observations.
  if (initialized_tables && !code.use_prefix_code) {
    auto* tables = code.alias_tables.address<jxl::AliasTable::Entry>();
    size_t count = code.uint_config.size() << code.log_alpha_size;
    std::cout << ' ' << count;
    for (size_t i = 0; i < count; ++i) {
      const auto& e = tables[i];
      std::cout << ' ' << unsigned(e.cutoff) << ',' << unsigned(e.right_value) << ','
                << e.freq0 << ',' << e.offsets1 << ',' << e.freq1_xor_freq0;
    }
  }
}

void ANSRead(std::istringstream& in, JxlMemoryManager* mm, const std::string& mode) {
  bool stream = mode != "ans-hist";
  size_t contexts, offset, count = 0, multiplier = 0;
  bool disallow;
  std::string hex;
  in >> contexts >> disallow >> hex >> offset;
  if (stream) in >> count >> multiplier;
  size_t checkpoint_at = 0, interval = 0;
  if (mode == "ans-checkpoint") in >> checkpoint_at >> interval;
  Input input(hex);
  BitReader br(input.bytes());
  br.SkipBits(offset);
  jxl::ANSCode code;
  code.use_prefix_code = false;
  code.log_alpha_size = 0;
  code.lz77.nonserialized_distance_context = 0;
  std::vector<uint8_t> context_map;
  Status status = jxl::DecodeHistograms(mm, &br, contexts, &code, &context_map, disallow);
  std::cout << Code(status) << ' ' << br.TotalBitsConsumed() << ' ' << context_map.size();
  for (auto ctx : context_map) std::cout << ' ' << unsigned(ctx);
  ANSCodeObservation(code, status);
  if (stream && status) {
    auto result = jxl::ANSSymbolReader::Create(&code, &br, multiplier);
    if (!result.ok()) throw std::runtime_error("ANS reader allocation");
    auto reader = std::move(result).value_();
    std::cout << ' ' << reader.UsesLZ77() << ' ' << reader.IsHuffRleOnly();
    if (mode == "ans-single") {
      uint32_t value = 123456;
      bool single = reader.IsSingleValueAndAdvance(context_map[0], &value, count);
      std::cout << ' ' << single << ',' << value;
    }
    uint32_t value = 99, run = 17;
    for (size_t i = 0; i < count;) {
      if (mode == "ans-rle") {
        reader.ReadHybridUintClusteredHuffRleOnly(context_map[0], &br, &value, &run);
        std::cout << ' ' << value << ',' << run;
        ++i;
      } else if (mode == "ans-checkpoint" && i == checkpoint_at) {
        jxl::ANSSymbolReader::Checkpoint checkpoint;
        reader.Save(&checkpoint);
        size_t pos = br.TotalBitsConsumed();
        std::cout << " C" << checkpoint.state << ',' << checkpoint.num_decoded << ','
                  << checkpoint.num_to_copy << ',' << checkpoint.copy_pos;
        std::vector<size_t> values(interval);
        for (auto& item : values) item = reader.ReadHybridUint(0, &br, context_map);
        reader.Restore(checkpoint);
        std::cout << ',' << Code(br.Close());
        br = BitReader(input.bytes()); br.SkipBits(pos);
        for (auto expected : values) {
          auto replay = reader.ReadHybridUint(0, &br, context_map);
          if (replay != expected) throw std::runtime_error("checkpoint replay mismatch");
          std::cout << ' ' << replay;
        }
        i += interval;
      } else {
        auto value = mode == "ans-nolz"
            ? reader.ReadHybridUintInlined<false>(0, &br, context_map)
            : reader.ReadHybridUint(0, &br, context_map);
        std::cout << ' ' << value;
        ++i;
      }
    }
    std::cout << ' ' << reader.CheckANSFinalState();
  }
  std::cout << ' ' << br.TotalBitsConsumed() << ' ' << Code(br.Close());
}

// This fixture adapter composes existing source entry points. Its small
// outer histogram envelope is test framing, not an image encoder.
void ANSFixture(std::istringstream& in, JxlMemoryManager* mm, int context_mtf = -1) {
  bool prefix, lz77;
  size_t log_alpha, offset, count;
  in >> prefix >> log_alpha >> lz77;
  auto cfg = ReadUintConfigArgs(in);
  in >> offset >> count;
  std::vector<jxl::Token> tokens;
  jxl::EntropyEncodingData codes;
  codes.use_prefix_code = prefix;
  codes.log_alpha_size = log_alpha;
  codes.lz77.enabled = lz77;
  codes.lz77.min_symbol = 224;
  codes.lz77.min_length = 3;
  codes.lz77.length_uint_config = jxl::HybridUintConfig(0, 0, 0);
  codes.context_map = lz77 ? std::vector<uint8_t>{0, 1} : std::vector<uint8_t>{0};
  codes.uint_config.push_back(cfg);
  if (lz77) codes.uint_config.emplace_back(0, 0, 0);
  std::vector<jxl::Histogram> histograms(codes.uint_config.size());
  for (size_t i = 0; i < count; ++i) {
    uint32_t ctx, value, length;
    in >> ctx >> value >> length;
    tokens.emplace_back(ctx, value);
    tokens.back().is_lz77_length = length;
    uint32_t token, nbits, bits;
    (length ? codes.lz77.length_uint_config : codes.uint_config[ctx]).Encode(value, &token, &nbits, &bits);
    token += length ? codes.lz77.min_symbol : 0;
    histograms[ctx].Add(token);
  }
  codes.encoding_info.resize(histograms.size());
  for (size_t i = 0; i < histograms.size(); ++i)
    codes.encoding_info[i].resize(std::max(size_t{1}, histograms[i].alphabet_size()));
  BitWriter header(mm), payload(mm);
  if (context_mtf >= 0) {
    if (!header.WithMaxBits(2, LayerType::Header, nullptr, [&] {
      header.Write(1, 0); header.Write(1, context_mtf); return true;
    })) throw std::runtime_error("context map prefix");
  } else if (!Prefix(header, offset)) throw std::runtime_error("ANS prefix");
  if (!Prefix(payload, offset)) throw std::runtime_error("ANS payload prefix");
  Status status = jxl::Bundle::Write(codes.lz77, &header, LayerType::Header, nullptr);
  if (!status) throw std::runtime_error("LZ77 fixture fields");
  status = header.WithMaxBits(32768, LayerType::Header, nullptr, [&]() -> Status {
    if (lz77) {
      jxl::EncodeUintConfigs({codes.lz77.length_uint_config}, &header, 8);
      header.Write(1, 1); header.Write(2, 1); // simple 1-bit context map
      header.Write(1, 0); header.Write(1, 1);
    }
    header.Write(1, prefix);
    if (!prefix) header.Write(2, log_alpha - 5);
    jxl::EncodeUintConfigs(codes.uint_config, &header, prefix ? 15 : log_alpha);
    if (prefix) {
      for (const auto& histogram : histograms) {
        size_t n = std::max(size_t{1}, histogram.alphabet_size()) - 1;
        if (n == 0) header.Write(1, 0);
        else {
          size_t bits = jxl::FloorLog2Nonzero(n);
          header.Write(1, 1); header.Write(4, bits);
          header.Write(bits, n - (1ULL << bits));
        }
      }
    }
    // The source helper fills encoding_info.back(), so move each destination
    // into that position during this adapter's loop.
    for (size_t i = 0; i < histograms.size(); ++i) {
      std::swap(codes.encoding_info[i], codes.encoding_info.back());
      auto cost = codes.BuildAndStoreANSEncodingData(mm,
          jxl::HistogramParams::ANSHistogramStrategy::kPrecise, histograms[i], &header);
      std::swap(codes.encoding_info[i], codes.encoding_info.back());
      if (!cost.ok()) return cost.status();
    }
    return true;
  });
  if (!status) throw std::runtime_error("ANS fixture histogram");
  size_t header_bits = header.BitsWritten();
  if (!jxl::WriteTokens(tokens, codes, 0, &payload, LayerType::Header, nullptr))
    throw std::runtime_error("ANS fixture payload");
  size_t payload_bits = payload.BitsWritten();
  if (!Pad(payload)) throw std::runtime_error("ANS fixture padding");
  if (!jxl::WriteTokens(tokens, codes, 0, &header, LayerType::Header, nullptr) || !Pad(header))
    throw std::runtime_error("ANS fixture stream");
  std::cout << header_bits << ' ' << payload_bits << ' ' << Hex(payload.GetSpan()) << ' ' << Hex(header.GetSpan());
  std::cout << ' ' << codes.encoding_info.size();
  for (const auto& histogram : codes.encoding_info) {
    std::cout << ' ' << histogram.size();
    for (const auto& symbol : histogram) {
      // Prefix fixtures do not use ANS fields and vice versa.
      std::cout << ' ' << (prefix ? 0 : symbol.freq_) << ' ' << (prefix ? 0 : symbol.ifreq_)
                << ' ' << (prefix ? unsigned(symbol.depth) : 0) << ' ' << (prefix ? symbol.bits : 0)
                << ' ' << symbol.reverse_map_.size();
      for (auto value : symbol.reverse_map_) std::cout << ' ' << value;
    }
  }
}

bool ANSCommand(const std::string& command, std::istringstream& in, JxlMemoryManager* mm) {
  if (command == "ans-hist" || command == "ans-stream" || command == "ans-checkpoint" || command == "ans-single" || command == "ans-rle" || command == "ans-nolz") ANSRead(in, mm, command);
  else if (command == "ans-fixture") ANSFixture(in, mm);
  else if (command == "context-fixture") {
    bool prefix, mtf; size_t size; in >> prefix >> mtf >> size;
    std::ostringstream text;
    text << prefix << " 8 0 4 2 0 2 " << size;
    for (size_t i = 0; i < size; ++i) {
      uint32_t value; in >> value;
      text << " 0 " << value << " 0";
    }
    std::istringstream request(text.str());
    ANSFixture(request, mm, mtf);
  }
  else if (command == "ans-wrap-fixture") {
    bool prefix; in >> prefix;
    std::ostringstream text;
    const size_t literals = jxl::kWindowSize + 10;
    text << prefix << " 8 1 4 2 0 0 " << literals + 2;
    for (size_t i = 0; i < literals; ++i) text << " 0 " << i % 7 << " 0";
    // A near-window-size copy overwrites entries that the next replay will
    // read, so omitting restoration of saved window bytes changes samples.
    text << " 0 507 1 1 " << jxl::kWindowSize - 2 << " 0";
    std::istringstream request(text.str());
    ANSFixture(request, mm);
  }
  else if (command == "hybrid") {
    auto cfg = ReadUintConfigArgs(in);
    uint32_t value, token, nbits, bits;
    size_t offset;
    in >> value >> offset;
    cfg.Encode(value, &token, &nbits, &bits);
    BitWriter writer(mm);
    if (!writer.WithMaxBits(64, LayerType::Header, nullptr, [&] {
      writer.Write(offset, (1ULL << offset) - 1); writer.Write(nbits, bits); return true;
    }) || !Pad(writer)) throw std::runtime_error("hybrid allocation");
    BitReader reader(writer.GetSpan()); reader.SkipBits(offset); reader.Refill();
    auto decoded = jxl::ANSSymbolReader::ReadHybridUintConfig(cfg, token, &reader);
    std::cout << token << ' ' << nbits << ' ' << bits << ' ' << cfg.LsbMask() << ' '
              << decoded << ' ' << reader.TotalBitsConsumed() << ' ' << Code(reader.Close());
  } else if (command == "uint-config") {
    size_t log_alpha, offset;
    in >> log_alpha >> offset;
    auto cfg = ReadUintConfigArgs(in);
    BitWriter writer(mm);
    if (!Prefix(writer, offset) || !writer.WithMaxBits(32, LayerType::Header, nullptr, [&] {
      jxl::EncodeUintConfigs({cfg}, &writer, log_alpha); return true;
    })) throw std::runtime_error("uint config allocation");
    size_t bits = writer.BitsWritten();
    if (!Pad(writer)) throw std::runtime_error("uint config padding");
    BitReader reader(writer.GetSpan()); reader.SkipBits(offset);
    std::vector<jxl::HybridUintConfig> decoded(1);
    Status status = jxl::DecodeUintConfigs(log_alpha, &decoded, &reader);
    std::cout << bits << ' ' << Hex(writer.GetSpan()) << ' ' << Code(status);
    UintConfigObservation(decoded[0]);
    std::cout << ' ' << reader.TotalBitsConsumed() << ' ' << Code(reader.Close());
  } else if (command == "uint-config-read") {
    size_t log_alpha, offset, count;
    std::string hex;
    in >> log_alpha >> hex >> offset >> count;
    Input input(hex); BitReader reader(input.bytes()); reader.SkipBits(offset);
    std::vector<jxl::HybridUintConfig> configs(count);
    Status status = jxl::DecodeUintConfigs(log_alpha, &configs, &reader);
    std::cout << Code(status);
    for (const auto& cfg : configs) UintConfigObservation(cfg);
    std::cout << ' ' << reader.TotalBitsConsumed() << ' ' << Code(reader.Close());
  } else if (command == "context-map") {
    size_t size, offset, num_htrees = 99;
    std::string hex;
    in >> size >> hex >> offset;
    Input input(hex); BitReader reader(input.bytes()); reader.SkipBits(offset);
    std::vector<uint8_t> map(size, 0xA5);
    Status status = jxl::DecodeContextMap(mm, &map, &num_htrees, &reader);
    std::cout << Code(status) << ' ' << num_htrees;
    for (auto value : map) std::cout << ' ' << unsigned(value);
    std::cout << ' ' << reader.TotalBitsConsumed() << ' ' << Code(reader.Close());
  } else if (command == "inverse-mtf") {
    std::string hex; in >> hex;
    auto bytes = Unhex(hex);
    jxl::InverseMoveToFrontTransform(bytes.data(), bytes.size());
    std::cout << Hex(jxl::Bytes(bytes.data(), bytes.size()));
  } else if (command == "special-distance") {
    size_t index; int multiplier; in >> index >> multiplier;
    std::cout << jxl::SpecialDistance(index, multiplier);
  } else return false;
  return true;
}
