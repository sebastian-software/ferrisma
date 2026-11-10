// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol adapters only; all algorithms execute unmodified upstream code.

#include "lib/jxl/dec_huffman.h"
#include "lib/jxl/enc_huffman.h"
#include "lib/jxl/enc_huffman_tree.h"
#include "lib/jxl/huffman_table.h"

void HuffmanTableObservation(const std::vector<jxl::HuffmanCode>& table) {
  std::cout << ' ' << table.size();
  for (const auto& entry : table)
    std::cout << ' ' << unsigned(entry.bits) << ',' << entry.value;
}

void HuffmanReadObservation(jxl::HuffmanDecodingData& data, size_t alphabet,
                            BitReader& reader, size_t num_symbols) {
  bool ok = data.ReadFromBitStream(alphabet, &reader);
  std::cout << ' ' << ok << ' ' << reader.TotalBitsConsumed();
  HuffmanTableObservation(data.table_);
  if (ok) {
    for (size_t i = 0; i < num_symbols; ++i) {
      reader.Refill();
      std::cout << ' ' << data.ReadSymbol(&reader);
    }
  }
  std::cout << ' ' << reader.TotalBitsConsumed() << ' ' << Code(reader.Close());
}

void HuffmanRoundTrip(std::istringstream& in, JxlMemoryManager* mm) {
  size_t length, offset, num_symbols;
  in >> length >> offset;
  std::vector<uint32_t> histogram(length);
  for (auto& count : histogram) in >> count;
  in >> num_symbols;
  std::vector<uint16_t> symbols(num_symbols);
  for (auto& symbol : symbols) in >> symbol;
  std::vector<uint8_t> depth(length);
  std::vector<uint16_t> bits(length);
  BitWriter writer(mm);
  if (!Prefix(writer, offset)) throw std::runtime_error("Huffman prefix");
  size_t tree_bits = 0;
  Status status = writer.WithMaxBits(32 * length + 256 + 15 * num_symbols,
      LayerType::Header, nullptr, [&]() -> Status {
    JXL_RETURN_IF_ERROR(jxl::BuildAndStoreHuffmanTree(histogram.data(), length,
                         depth.data(), bits.data(), &writer));
    tree_bits = writer.BitsWritten();
    for (auto symbol : symbols) writer.Write(depth[symbol], bits[symbol]);
    return true;
  });
  size_t total_bits = writer.BitsWritten();
  if (!Pad(writer)) throw std::runtime_error("Huffman padding");
  std::cout << Code(status) << ' ' << tree_bits << ' ' << total_bits << ' '
            << Hex(writer.GetSpan()) << ' ' << length;
  for (size_t i = 0; i < length; ++i)
    std::cout << ' ' << unsigned(depth[i]) << ',' << bits[i];
  jxl::HuffmanDecodingData data;
  BitReader reader(writer.GetSpan());
  reader.SkipBits(offset);
  HuffmanReadObservation(data, length, reader, num_symbols);
}

bool HuffmanCommand(const std::string& command, std::istringstream& in,
                     JxlMemoryManager* mm) {
  if (command == "huffman") HuffmanRoundTrip(in, mm);
  else if (command == "huffman-read") {
    size_t alphabet, offset, seed_size, num_symbols;
    std::string hex;
    in >> alphabet >> hex >> offset >> seed_size >> num_symbols;
    Input input(hex);
    jxl::HuffmanDecodingData data;
    data.table_.resize(seed_size);
    for (size_t i = 0; i < seed_size; ++i)
      data.table_[i] = {3, static_cast<uint16_t>(i * 17)};
    BitReader reader(input.bytes());
    reader.SkipBits(offset);
    HuffmanReadObservation(data, alphabet, reader, num_symbols);
  } else if (command == "huffman-rle") {
    size_t length, initial;
    in >> length >> initial;
    std::vector<uint8_t> depth(length), tree(length + initial, 0xA5), extra(length + initial, 0x5A);
    for (auto& d : depth) { unsigned value; in >> value; d = value; }
    size_t size = initial;
    jxl::WriteHuffmanTree(depth.data(), length, &size, tree.data(), extra.data());
    std::cout << size;
    for (size_t i = 0; i < size; ++i)
      std::cout << ' ' << unsigned(tree[i]) << ',' << unsigned(extra[i]);
  } else if (command == "huffman-tree") {
    size_t length;
    int limit;
    in >> length >> limit;
    std::vector<uint32_t> histogram(length);
    std::vector<uint8_t> depth(length);
    std::vector<uint16_t> bits(length);
    for (auto& count : histogram) in >> count;
    jxl::CreateHuffmanTree(histogram.data(), length, limit, depth.data());
    jxl::ConvertBitDepthsToSymbols(depth.data(), length, bits.data());
    std::cout << length;
    for (size_t i = 0; i < length; ++i)
      std::cout << ' ' << unsigned(depth[i]) << ',' << bits[i];
  } else if (command == "huffman-table") {
    size_t length;
    int root_bits;
    in >> root_bits >> length;
    std::vector<uint8_t> depth(length);
    uint16_t counts[16] = {};
    for (auto& d : depth) { unsigned value; in >> value; d = value; ++counts[d]; }
    std::vector<jxl::HuffmanCode> table(1u << 16);
    auto size = jxl::BuildHuffmanTable(table.data(), root_bits, depth.data(), length, counts);
    std::cout << size;
    for (auto count : counts) std::cout << ' ' << count;
    table.resize(size);
    HuffmanTableObservation(table);
  } else return false;
  return true;
}
