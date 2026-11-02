// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Pristine original stages and public factory, including whole-file callers.
// Only this dedicated TU has scoped -O0, with every sanitizer still active.
#if defined(__linux__) && defined(__x86_64__)
#include "lib/jxl/render_pipeline/stage_chroma_upsampling.cc"
#endif
