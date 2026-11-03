// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
// Observation only: every transfer operation calls the unchanged skcms archive.
#include "src/skcms_public.h"
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>

namespace {
float Read(std::istringstream& in) {
  uint32_t word;
  in >> word;
  float value;
  memcpy(&value, &word, sizeof(value));
  return value;
}
skcms_TransferFunction ReadTF(std::istringstream& in) {
  return {Read(in), Read(in), Read(in), Read(in), Read(in), Read(in), Read(in)};
}
void Write(float value) {
  uint32_t word;
  memcpy(&word, &value, sizeof(word));
  std::cout << ' ' << word;
}
void WriteTF(const skcms_TransferFunction& tf) {
  for (float value : {tf.g, tf.a, tf.b, tf.c, tf.d, tf.e, tf.f}) Write(value);
}
}

bool SkcmsTransferCommand(const std::string& command, std::istringstream& in) {
  if (command == "skcms-tf" || command == "skcms-tf-in-place") {
    bool eval;
    in >> eval;
    auto tf = ReadTF(in);
    const float x = Read(in);
    auto dst = ReadTF(in);
    std::cout << int(skcms_TransferFunction_getType(&tf));
    if (eval) Write(skcms_TransferFunction_eval(&tf, x));
    if (command == "skcms-tf-in-place") {
      std::cout << ' ' << skcms_TransferFunction_invert(&tf, &tf);
      WriteTF(tf);
    } else {
      std::cout << ' ' << skcms_TransferFunction_invert(&tf, &dst);
      WriteTF(dst);
    }
    return true;
  }
  if (command != "skcms-make") return false;
  unsigned kind;
  in >> kind;
  const auto params = ReadTF(in);
  auto dst = ReadTF(in);
  switch (kind) {
    case 0:
      std::cout << skcms_TransferFunction_makePQish(
          &dst, params.g, params.a, params.b, params.c, params.d, params.e);
      break;
    case 1:
      std::cout << skcms_TransferFunction_makeScaledHLGish(
          &dst, params.g, params.a, params.b, params.c, params.d, params.e);
      break;
    case 2:
      std::cout << skcms_TransferFunction_makeHLGish(
          &dst, params.g, params.a, params.b, params.c, params.d);
      break;
    case 3:
      skcms_TransferFunction_makePQ(&dst, params.g);
      std::cout << 'V';
      break;
    case 4:
      skcms_TransferFunction_makeHLG(&dst, params.g, params.a, params.b);
      std::cout << 'V';
      break;
    default:
      throw std::runtime_error("original transfer constructor domain");
  }
  WriteTF(dst);
  std::cout << ' ' << int(skcms_TransferFunction_getType(&dst)) << ' '
      << skcms_TransferFunction_isSRGBish(&dst) << ' '
      << skcms_TransferFunction_isPQish(&dst) << ' '
      << skcms_TransferFunction_isHLGish(&dst) << ' '
      << skcms_TransferFunction_isPQ(&dst) << ' '
      << skcms_TransferFunction_isHLG(&dst);
  return true;
}
