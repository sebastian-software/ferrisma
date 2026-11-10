// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The C ABI driver: replays dpub-run histories through the decoder API that
// the pinned lib/include/jxl/decode.h declares, with the shared driver of
// decoder_public_driver.h, and reports the C layout measurements of
// decoder_abi_layout.c. It includes only public headers. The same objects are
// linked twice: jxl_abi_original against the original library archive and
// jxl_abi_rust against the Rust static library. Both link the original
// default CMS library for JxlGetDefaultCms, which decode.h does not declare.
// Parallel runner operations call JxlDecoderSetParallelRunner and the runners
// of thread_parallel_runner.h and resizable_parallel_runner.h: the original
// driver links the original jxl_threads library, the Rust driver the runners
// that the Rust library exports.
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>

#include "decoder_public_driver.h"

extern "C" const char* DecoderAbiLayout(void);

int main() {
  try {
    std::string line;
    while (std::getline(std::cin, line)) {
      std::istringstream in(line);
      std::string command;
      in >> command;
      if (command == "dpub-run") {
        decoder_public::RunHistory(in, std::cout);
      } else if (command == "abi-layout") {
        std::cout << DecoderAbiLayout();
      } else {
        throw std::runtime_error("unknown command");
      }
      if (in.fail()) throw std::runtime_error("invalid request");
      std::cout << '\n';
    }
    return 0;
  } catch (const std::exception& error) {
    std::cerr << error.what() << '\n';
    return 2;
  }
}
