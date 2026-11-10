// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Protocol adapters only; all algorithms execute unmodified upstream code.

#include "lib/jxl/ans_common.h"
#include "lib/jxl/base/bits.h"
#include "lib/jxl/base/byte_order.h"

template <typename T>
void WordObservation(T value) {
  std::cout << jxl::Num0BitsAboveMS1Bit(value) << ' '
            << jxl::Num0BitsBelowLS1Bit(value);
  if (value != 0) {
    std::cout << ' ' << jxl::FloorLog2Nonzero(value) << ' '
              << jxl::CeilLog2Nonzero(value) << ' '
              << jxl::Num0BitsAboveMS1Bit_Nonzero(value) << ' '
              << jxl::Num0BitsBelowLS1Bit_Nonzero(value);
  }
}

enum class OracleEnum : uint32_t { Zero = 0, One = 1, Seven = 7, Last = 63 };
uint64_t EnumBits(OracleEnum) { return (1ULL << 0) | (1ULL << 1) | (1ULL << 7) | (1ULL << 63); }
const char* EnumName(OracleEnum) { return "OracleEnum"; }

struct EnumBundle : public jxl::Fields {
  EnumBundle() { jxl::Bundle::Init(this); }
  JXL_FIELDS_NAME(EnumBundle)
  Status VisitFields(jxl::Visitor* v) override { return v->Enum(OracleEnum::Zero, &value); }
  OracleEnum value;
};

void EnumRead(std::istringstream& in) {
  std::string hex;
  size_t offset;
  uint32_t initial;
  in >> hex >> offset >> initial;
  Input bytes(hex);
  EnumBundle fields;
  fields.value = static_cast<OracleEnum>(initial);
  BitReader reader(bytes.bytes());
  reader.SkipBits(offset);
  Status read = jxl::Bundle::Read(&reader, &fields);
  std::cout << Code(read) << ' ' << static_cast<uint32_t>(fields.value) << ' '
            << reader.TotalBitsConsumed() << ' ' << Code(reader.Close());
}

void ByteOrderObservation(std::istringstream& in) {
  uint64_t value;
  std::string hex;
  in >> value >> hex;
  Input input(hex);
  const uint8_t* p = input.data.data() + 1;
  float f = LoadLEFloat(p);
  float swapped = BSwapFloat(f);
  uint32_t float_be, float_le, float_swap;
  float fbe = LoadBEFloat(p);
  std::memcpy(&float_be, &fbe, 4);
  std::memcpy(&float_le, &f, 4);
  std::memcpy(&float_swap, &swapped, 4);
  std::cout << Load16(OrderBE{}, p) << ' ' << Load16(OrderLE{}, p) << ' '
            << Load32(OrderBE{}, p) << ' ' << Load32(OrderLE{}, p) << ' '
            << LoadBE64(p) << ' ' << LoadLE64(p) << ' ' << float_be << ' '
            << float_le << ' ' << float_swap << ' ' << IsLittleEndian();
  for (auto order : {JXL_NATIVE_ENDIAN, JXL_LITTLE_ENDIAN, JXL_BIG_ENDIAN})
    std::cout << ' ' << SwapEndianness(order);
  for (int op = 0; op < 6; ++op) {
    uint8_t bytes[10];
    std::memset(bytes, 0xA5, sizeof(bytes));
    if (op == 0) Store16(OrderBE{}, value, bytes + 1);
    else if (op == 1) Store16(OrderLE{}, value, bytes + 1);
    else if (op == 2) Store32(OrderBE{}, value, bytes + 1);
    else if (op == 3) Store32(OrderLE{}, value, bytes + 1);
    else if (op == 4) StoreBE64(value, bytes + 1);
    else StoreLE64(value, bytes + 1);
    std::cout << ' ' << Hex(jxl::Bytes(bytes, sizeof(bytes)));
  }
}

void AliasObservation(std::istringstream& in) {
  size_t log_alpha, length;
  in >> log_alpha >> length;
  std::vector<int32_t> distribution(length);
  for (auto& count : distribution) in >> count;
  std::vector<jxl::AliasTable::Entry> table(1ULL << log_alpha);
  Status status = jxl::InitAliasTable(distribution, ANS_LOG_TAB_SIZE, log_alpha, table.data());
  std::cout << Code(status) << ' ' << table.size();
  for (const auto& entry : table)
    std::cout << ' ' << unsigned(entry.cutoff) << ',' << unsigned(entry.right_value)
              << ',' << entry.freq0 << ',' << entry.offsets1 << ',' << entry.freq1_xor_freq0;
  if (!status) return;
  for (size_t i = 0; i < ANS_TAB_SIZE; ++i) {
    auto symbol = jxl::AliasTable::Lookup(table.data(), i, ANS_LOG_TAB_SIZE - log_alpha,
                                        (ANS_TAB_SIZE >> log_alpha) - 1);
    std::cout << ' ' << symbol.value << ',' << symbol.offset << ',' << symbol.freq;
  }
}

bool EntropyCommand(const std::string& command, std::istringstream& in, JxlMemoryManager*) {
  if (command == "word") {
    unsigned width;
    uint64_t value;
    in >> width >> value;
    if (width == 32) WordObservation(static_cast<uint32_t>(value));
    else WordObservation(value);
  } else if (command == "byteorder") ByteOrderObservation(in);
  else if (command == "enum-info") {
    uint32_t raw;
    in >> raw;
    std::cout << Code(jxl::EnumValid(static_cast<OracleEnum>(raw)));
    for (auto value : jxl::Values<OracleEnum>()) std::cout << ' ' << static_cast<uint32_t>(value);
  } else if (command == "enum-read") EnumRead(in);
  else if (command == "alias") AliasObservation(in);
  else if (command == "flat") {
    int length, total;
    in >> length >> total;
    for (int count : jxl::CreateFlatHistogram(length, total)) std::cout << count << ',';
  } else if (command == "population") {
    uint32_t count, shift;
    in >> count >> shift;
    std::cout << jxl::GetPopulationCountPrecision(count, shift);
  } else return false;
  return true;
}
