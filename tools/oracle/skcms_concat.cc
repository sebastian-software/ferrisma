// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
// Observe the hash-verified complete original body in an isolated x86 -O0 TU.
// Only its symbol is renamed; the original optimized archive stays untouched.
#include "src/skcms_public.h"
#define skcms_Matrix3x3_concat jxl_oracle_skcms_Matrix3x3_concat
extern "C" {
#include "original_skcms_concat.inc"
}
#undef skcms_Matrix3x3_concat
