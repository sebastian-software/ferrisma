// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
// Data-only calls to unchanged public skcms matrix operations.
#include "src/skcms_public.h"
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <string>
namespace {
float ReadMatrixFloat(std::istringstream& in) {
  uint32_t word; in >> word;
  float value; memcpy(&value, &word, 4); return value;
}
skcms_Matrix3x3 ReadMatrix(std::istringstream& in) {
  skcms_Matrix3x3 matrix;
  for (auto& row : matrix.vals) for (auto& value : row) value = ReadMatrixFloat(in);
  return matrix;
}
void WriteMatrix(const skcms_Matrix3x3& matrix) {
  for (const auto& row : matrix.vals) for (float value : row) {
    uint32_t word; memcpy(&word, &value, 4); std::cout << ' ' << word;
  }
}
}
bool SkcmsMatrixCommand(const std::string& command, std::istringstream& in) {
  if (command == "skcms-concat") {
    bool alias; in >> alias;
    const auto a = ReadMatrix(in), b = ReadMatrix(in);
    const auto result = skcms_Matrix3x3_concat(&a, alias ? &a : &b);
    std::cout << 'V'; WriteMatrix(result); return true;
  }
  if (command == "skcms-invert") {
    bool alias; in >> alias;
    auto source = ReadMatrix(in), destination = ReadMatrix(in);
    const bool result = skcms_Matrix3x3_invert(&source, alias ? &source : &destination);
    std::cout << result; WriteMatrix(alias ? source : destination); return true;
  }
  if (command == "skcms-adapt" || command == "skcms-primaries") {
    bool null_output; in >> null_output;
    float xy[8]{};
    for (int i = 0; i < (command == "skcms-adapt" ? 2 : 8); i++) xy[i] = ReadMatrixFloat(in);
    auto destination = ReadMatrix(in);
    auto* output = null_output ? nullptr : &destination;
    const bool result = command == "skcms-adapt"
      ? skcms_AdaptToXYZD50(xy[0], xy[1], output)
      : skcms_PrimariesToXYZD50(xy[0], xy[1], xy[2], xy[3], xy[4], xy[5], xy[6], xy[7], output);
    std::cout << result; WriteMatrix(destination); return true;
  }
  return false;
}
