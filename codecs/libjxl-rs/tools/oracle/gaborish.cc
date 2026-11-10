// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compile the pristine scalar stage in a dedicated observation unit. Scoped
// -O0 keeps Highway's original scalar operator boundaries stable with and
// without sanitizer instrumentation; the complete original library retains
// its normal build flags. No stage or factory algorithm is supplied here.
#include "lib/jxl/render_pipeline/stage_gaborish.cc"
