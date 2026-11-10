// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
namespace {
jxl::BlockCtxMap ACGroupMap(bool custom) {
  jxl::BlockCtxMap map;
  if (custom) {
    map.num_ctxs = 3;
    map.num_dc_ctxs = 2;
    map.qf_thresholds = {3, 8};
    map.ctx_map.resize(3 * jxl::kNumOrders * 3 * 2);
    for (size_t i = 0; i < map.ctx_map.size(); ++i)
      map.ctx_map[i] = (i * 7 + 1) % 3;
  }
  return map;
}
void ACGroupCheckpoint(jxl::ANSSymbolReader& reader, jxl::BitReader& br) {
  jxl::ANSSymbolReader::Checkpoint cp;
  reader.Save(&cp);
  // The unread LZ77 window is source-indeterminate; observe initialized scalar
  // checkpoints only, without exposing its copied object representation.
  std::cout << ' ' << br.TotalBitsConsumed() << ',' << cp.state << ','
            << cp.num_decoded << ',' << cp.num_to_copy << ',' << cp.copy_pos;
}
template<class T> void ACGroupArray(const std::vector<T>& values) {
  std::cout << ' ' << values.size();
  for (auto value : values) std::cout << ' ' << +value;
}
void ACGroupFixture(std::istringstream& in, JxlMemoryManager* mm) {
  bool prefix;
  size_t lz, offset, contexts, count;
  in >> prefix >> lz >> offset >> contexts >> count;
  std::vector<std::vector<jxl::Token>> tokens(1);
  for (size_t i = 0; i < count; ++i) {
    uint32_t context, value;
    in >> context >> value;
    tokens[0].emplace_back(context, value);
  }
  jxl::HistogramParams params;
  params.force_huffman = prefix;
  // Preserve an explicitly selected original global entropy mode; otherwise
  // the original small-stream heuristic always chooses prefix coding.
  params.initialize_global_state = false;
  params.lz77_method = static_cast<jxl::HistogramParams::LZ77Method>(lz);
  params.clustering = jxl::HistogramParams::ClusteringType::kFastest;
  params.uint_method = jxl::HistogramParams::HybridUintMethod::kNone;
  jxl::EntropyEncodingData codes;
  codes.use_prefix_code = prefix;
  jxl::BitWriter writer(mm);
  if (!Prefix(writer, offset)) throw std::runtime_error("AC group fixture prefix");
  auto cost = jxl::BuildAndEncodeHistograms(mm, params, contexts, tokens, &codes,
                                           &writer, jxl::LayerType::Ac, nullptr);
  if (!cost.ok() || !jxl::WriteTokens(tokens[0], codes, 0, &writer,
                                     jxl::LayerType::Ac, nullptr))
    throw std::runtime_error("AC group fixture encoding");
  size_t bits = writer.BitsWritten();
  if (!Pad(writer)) throw std::runtime_error("AC group fixture padding");
  std::cout << bits << ' ' << Hex(writer.GetSpan()) << ' ' << codes.lz77.enabled
            << ' ' << codes.use_prefix_code;
}
void ACGroupDecode(std::istringstream& in, JxlMemoryManager* mm) {
  size_t width, strategy, c, bx, lbx, by, stride, seed, order_mode, ctx_offset;
  bool top, custom;
  int32_t initial, qf;
  unsigned qdc;
  std::string hex;
  size_t offset, calls, warmup_count;
  in >> width >> strategy >> c >> bx >> lbx >> by >> stride >> top >> seed
     >> initial >> order_mode >> custom >> ctx_offset >> qf >> qdc >> hex
     >> offset >> calls >> warmup_count;
  auto acs = jxl::AcStrategy::FromRawStrategy(strategy);
  size_t covered = size_t{1} << acs.log2_covered_blocks();
  size_t size = covered * jxl::kDCTBlockSize;
  size_t order_offset = jxl::kCoeffOrderOffset[3 * jxl::kStrategyOrder[strategy] + c] * jxl::kDCTBlockSize;
  std::vector<jxl::coeff_order_t> order(order_offset + size);
  acs.ComputeNaturalCoeffOrder(order.data() + order_offset);
  if (order_mode) std::reverse(order.begin() + order_offset + covered, order.end());
  std::vector<int32_t> row(stride * acs.covered_blocks_y() + 16), row_top(stride);
  for (size_t i = 0; i < row.size(); ++i) row[i] = (i * 5 + seed) % 64;
  for (size_t i = 0; i < row_top.size(); ++i) row_top[i] = (i * 3 + seed) % 64;
  std::vector<uint8_t> qdc_row(lbx + 1, qdc);
  std::vector<int32_t> qf_row(row.size(), qf);
  auto map = ACGroupMap(custom);
  std::vector<int16_t> block16(size);
  std::vector<int32_t> block32(size);
  for (size_t i = 0; i < size; ++i) {
    block16[i] = static_cast<int16_t>(initial + static_cast<int32_t>(i % 13) - 6);
    block32[i] = initial + static_cast<int32_t>(i % 13) - 6;
  }
  auto bytes = Unhex(hex);
  // Preserve source span length with real storage before/after even for short
  // inputs, keeping its end-minus-eight reader pointer inside the allocation.
  std::vector<uint8_t> backing(bytes.size() + 16);
  std::copy(bytes.begin(), bytes.end(), backing.begin() + 8);
  jxl::BitReader br(jxl::Bytes(backing.data() + 8, bytes.size()));
  br.SkipBits(offset);
  jxl::ANSCode code;
  std::vector<uint8_t> context_map;
  auto status = jxl::DecodeHistograms(mm, &br,
      ctx_offset + map.NumACContexts() + 16, &code, &context_map, false);
  std::cout << Code(status) << ' ' << br.TotalBitsConsumed();
  std::vector<size_t> shifts(calls);
  for (auto& shift : shifts) in >> shift;
  if (status) {
    auto result = jxl::ANSSymbolReader::Create(&code, &br, 0);
    if (!result.ok()) throw std::runtime_error("AC group reader allocation");
    auto decoder = std::move(result).value_();
    std::cout << ' ' << decoder.UsesLZ77() << ' ' << code.use_prefix_code;
    ACGroupCheckpoint(decoder, br);
    std::vector<size_t> warmup(warmup_count);
    for (auto& value : warmup) value = decoder.ReadHybridUint(0, &br, context_map);
    ACGroupArray(warmup);
    ACGroupCheckpoint(decoder, br);
    for (size_t shift : shifts) {
      jxl::ACPtr block;
      block.ptr16 = block16.data();
      if (width == 32) block.ptr32 = block32.data();
#define AC_GROUP_DECODE(W, L) jxl::DecodeACVarBlock<jxl::ACType::k##W, L>( \
      ctx_offset, acs.log2_covered_blocks(), row.data(), \
      top ? row_top.data() : nullptr, stride, c, bx, by, lbx, acs, order.data(), \
      &br, &decoder, context_map, qdc_row.data(), qf_row.data(), map, block, shift)
      if (width == 16) status = decoder.UsesLZ77() ? AC_GROUP_DECODE(16, true) : AC_GROUP_DECODE(16, false);
      else status = decoder.UsesLZ77() ? AC_GROUP_DECODE(32, true) : AC_GROUP_DECODE(32, false);
#undef AC_GROUP_DECODE
      std::cout << ' ' << Code(status);
      ACGroupCheckpoint(decoder, br);
      ACGroupArray(row);
      if (width == 16) ACGroupArray(block16); else ACGroupArray(block32);
    }
    std::cout << ' ' << decoder.CheckANSFinalState();
  }
  std::cout << ' ' << br.TotalBitsConsumed() << ' ' << Code(br.Close());
}
}
bool ACGroupSymbolsCommand(const std::string& command, std::istringstream& in,
                           JxlMemoryManager* mm) {
  if (command == "acg-fixture") ACGroupFixture(in, mm);
  else if (command == "acg-decode") ACGroupDecode(in, mm);
  else return false;
  return true;
}
