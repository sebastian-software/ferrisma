// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
// Observation only. Public operations bind to the original CMS archive.
// SKCMS_FIT_PRIVATE selects a separate executable including pristine skcms.cc
// for its internal helpers; that executable must not link archive skcms.cc.o.
#ifdef SKCMS_FIT_PRIVATE
#include "skcms.cc"
#else
#include "src/skcms_public.h"
#include "src/skcms_internals.h"
#endif
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <cstdlib>
#include <string>
#include <vector>

namespace {
float Read(std::istringstream& in) {
  uint32_t word;
  if (!(in >> word)) std::abort();
  float value;
  memcpy(&value, &word, 4);
  return value;
}
skcms_TransferFunction ReadTF(std::istringstream& in) {
  return {Read(in), Read(in), Read(in), Read(in), Read(in), Read(in), Read(in)};
}
void Write(float value) {
  uint32_t word;
  memcpy(&word, &value, 4);
  std::cout << ' ' << word;
}
void WriteTF(const skcms_TransferFunction& tf) {
  for (float value : {tf.g, tf.a, tf.b, tf.c, tf.d, tf.e, tf.f}) Write(value);
}
float* Field(skcms_TransferFunction* tf, int index) {
  switch (index) {
    case 0: return &tf->g;
    case 1: return &tf->a;
    case 2: return &tf->b;
    case 3: return &tf->c;
    case 4: return &tf->d;
    case 5: return &tf->e;
    case 6: return &tf->f;
    default: std::abort();
  }
}
}

bool SkcmsFittingCommand(const std::string& command, std::istringstream& in) {
  if (command.compare(0, 10, "skcms-fit-") != 0) return false;
  unsigned mode, physical, flags;
  uint32_t count;
  int alias, l, n;
  in >> mode >> count >> physical >> flags >> alias;
  float x0 = Read(in), dx = Read(in);
  in >> l >> n;
  float error = Read(in);
  auto tf = ReadTF(in);
  skcms_Curve curve = {};
  std::vector<uint8_t> table;
  if (mode == 0) {
    curve.parametric = ReadTF(in);
  } else if (mode == 8 || mode == 16) {
    table.resize(physical * (mode == 8 ? 1 : 2));
    for (unsigned i = 0; i < physical; i++) {
      unsigned value;
      in >> value;
      if (mode == 8) table[i] = uint8_t(value);
      else {
        table[2*i] = uint8_t(value >> 8);
        table[2*i+1] = uint8_t(value);
      }
    }
    curve.table_8 = mode == 8 ? table.data() : nullptr;
    curve.table_16 = mode == 16 ? table.data() : nullptr;
  } else std::abort();
  curve.table_entries = count;
  float* max_error = alias < 0 ? &error : Field(&tf, alias);
  if (command == "skcms-fit-approx") {
    const bool ok = skcms_ApproximateCurve(flags & 1 ? nullptr : &curve,
                                         flags & 2 ? nullptr : &tf,
                                         flags & 4 ? nullptr : max_error);
    std::cout << ok;
    Write(*max_error);
    WriteTF(tf);
    return true;
  }
  if (command == "skcms-fit-roundtrip") {
    const float result = skcms_MaxRoundtripError(&curve, &tf);
    std::cout << skcms_AreApproximateInverses(&curve, &tf);
    Write(result);
    WriteTF(tf);
    return true;
  }
#ifdef SKCMS_FIT_PRIVATE
  if (command == "skcms-fit-gradient") {
    float gradient[3] = {error, x0, dx};
    const float result = rg_nonlinear(x0, &curve, &tf, gradient);
    std::cout << 'V';
    Write(result);
    WriteTF(tf);
    for (float value : gradient) Write(value);
    return true;
  }
  if (command == "skcms-fit-step" || command == "skcms-fit-nonlinear") {
    const bool ok = command == "skcms-fit-step"
        ? gauss_newton_step(&curve, &tf, x0, dx, n)
        : fit_nonlinear(&curve, l, n, &tf);
    std::cout << ok;
    Write(error);
    WriteTF(tf);
    return true;
  }
  if (command == "skcms-fit-checked") {
    std::cout << 'V';
    Write(max_roundtrip_error_checked(&curve, &tf));
    WriteTF(tf);
    return true;
  }
#endif
  std::abort();
}

#ifdef SKCMS_FIT_STANDALONE
int main() {
  std::string line;
  while (std::getline(std::cin, line)) {
    std::istringstream in(line);
    std::string command;
    in >> command;
    if (!SkcmsFittingCommand(command, in)) return 2;
    if (!in) std::abort();
    std::cout << '\n';
  }
}
#endif
