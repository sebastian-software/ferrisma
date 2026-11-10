// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Test adapter for the unchanged original context methods and real codecs.
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "lib/jxl/ac_context.h"
#include "lib/jxl/dec_bit_reader.h"
#include "lib/jxl/enc_aux_out.h"
#include "lib/jxl/enc_bit_writer.h"
#include "lib/jxl/enc_context_map.h"
#include "lib/jxl/entropy_coder.h"
#include "block_context.inc"
