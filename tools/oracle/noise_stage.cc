// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile the pristine scalar noise stage at scoped -O0. Native ARM O3 ordinary
// and ASan/UBSan objects select different competing NaN operands. This unit
// supplies the real original factories/stages with every sanitizer retained;
// the codec archive and generation/parsing observer retain their build flags.
#include "lib/jxl/render_pipeline/stage_noise.cc"
