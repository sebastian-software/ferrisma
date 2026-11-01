// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Fixtures call the original encoder and decoder from the pinned library.
#include <algorithm>
#include "lib/jxl/ac_strategy.h"

namespace {
constexpr uint32_t kOrderSentinel = 0x31415926;
void CoeffOrdersObservation(const std::vector<jxl::coeff_order_t>& order) {
  // This lossless sparse representation observes every output word: omitted
  // positions retain the known sentinel, including the trailing guard words.
  size_t count = 0;
  for (auto value : order) count += value != kOrderSentinel;
  std::cout << ' ' << order.size() << ' ' << count;
  for (size_t i = 0; i < order.size(); ++i) {
    if (order[i] != kOrderSentinel) std::cout << ' ' << i << ':' << order[i];
  }
}
}

bool CoeffOrdersCommand(const std::string& command, std::istringstream& in,
                        JxlMemoryManager* mm) {
  if (command == "co-orders-generate") {
    uint16_t used; unsigned variant; size_t prefix;
    in >> used >> variant >> prefix;
    std::vector<jxl::coeff_order_t> orders(jxl::kCoeffOrderMaxSize);
    uint16_t computed = 0;
    for (uint8_t raw = 0; raw < jxl::AcStrategy::kNumValidStrategies; ++raw) {
      const auto ord = jxl::kStrategyOrder[raw];
      if (computed & (1 << ord)) continue;
      computed |= 1 << ord;
      auto acs = jxl::AcStrategy::FromRawStrategy(raw);
      size_t llf = acs.covered_blocks_x() * acs.covered_blocks_y();
      size_t size = jxl::kDCTBlockSize * llf;
      for (size_t c = 0; c < 3; ++c) {
        auto* first = orders.data() + (jxl::kCoeffOrderOffset[3 * ord + c] * jxl::kDCTBlockSize);
        acs.ComputeNaturalCoeffOrder(first);
        // Keep the LLF prefix intact; vary only valid permutation fixtures.
        if (variant == 1) std::reverse(first + llf, first + size);
        if (variant == 2) {
          size_t shift = (c + ord + 1) % (size - llf);
          std::rotate(first + llf, first + llf + shift, first + size);
        }
        if (variant == 3) {
          for (size_t i = llf; i + 1 < size; i += 2) std::swap(first[i], first[i + 1]);
        }
      }
    }
    BitWriter writer(mm);
    if (!Prefix(writer, prefix)) throw std::runtime_error("coefficient prefix");
    Status status = jxl::EncodeCoeffOrders(used, orders.data(), &writer,
                                          LayerType::Header, nullptr);
    std::cout << Code(status) << ' ' << writer.BitsWritten();
    if (!Pad(writer)) throw std::runtime_error("coefficient padding");
    std::cout << ' ' << Hex(writer.GetSpan());
  } else if (command == "co-orders-read") {
    size_t actions; in >> actions;
    std::vector<jxl::coeff_order_t> orders(jxl::kCoeffOrderMaxSize + 4, kOrderSentinel);
    for (size_t a = 0; a < actions; ++a) {
      uint16_t used; uint32_t acs; size_t prefix; std::string hex;
      in >> used >> acs >> prefix >> hex;
      auto bytes = Unhex(hex);
      const size_t size = bytes.size();
      // Keep the original reader's pre-start sentinel and overread backing
      // inside the allocation even for empty/truncated logical inputs.
      std::vector<uint8_t> backing(size + 16, 0);
      std::copy(bytes.begin(), bytes.end(), backing.begin() + 8);
      BitReader br(jxl::Bytes(backing.data() + 8, size));
      br.SkipBits(prefix);
      Status status = jxl::DecodeCoeffOrders(mm, used, acs, orders.data(), &br);
      std::cout << (a == 0 ? "" : " ") << Code(status) << ' '
                << br.TotalBitsConsumed() << ' ' << Code(br.Close());
      CoeffOrdersObservation(orders);
    }
  } else return false;
  return true;
}
