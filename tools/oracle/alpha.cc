// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile the pristine original alpha operations as actual public symbols.
// The scoped no-inline option retains std::max's comparison boundary with
// ordinary and instrumented builds. No source body or sanitizer is replaced.
#include "lib/jxl/alpha.cc"
