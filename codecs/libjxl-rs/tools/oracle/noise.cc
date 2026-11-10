// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Keep the original early-return loop at this numeric observation boundary.
#include "lib/jxl/noise.h"
bool SSNoiseHasAny(const jxl::NoiseParams& noise) { return noise.HasAny(); }
