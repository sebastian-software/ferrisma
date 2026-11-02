// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile the pristine original alpha operations as actual public symbols.
// The scoped scalar configuration retains source expressions and std::max's
// comparison across ordinary/instrumented graphs: O0, plus strict FP only on
// x86 Linux. The durable audit retains default finite words and all exceptional
// requests. No original body or sanitizer is replaced.
#include "lib/jxl/alpha.cc"
