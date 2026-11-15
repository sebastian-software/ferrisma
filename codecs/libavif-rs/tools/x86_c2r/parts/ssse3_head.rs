/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Translated from upstream/aom/av1/common/x86/av1_inv_txfm_ssse3.h and
// av1_inv_txfm_ssse3.c (ADR 0056, ADR 0058). Upstream compiles the .c file
// with -mssse3. The header items that the high-bitdepth inverse transforms
// use (the 1D type tables, the eob tables, and the eob helpers) were
// translated by hand first. The header's butterfly macros and
// round_shift_16bit_ssse3 were written by hand with the same expansion, and
// the .c file was transliterated with a typing-aware C-subset printer (not
// C2Rust) and reviewed against the C function by function: the same sequence
// of intrinsic and helper calls, and the same identifiers, constants, and
// operators in order, as ADR 0058 describes for the high-bitdepth transforms.
//
// Translation notes:
// - Every function of the .c file carries `ssse3`. The RTCD specialization
//   av1_inv_txfm_add_ssse3 is exported under its upstream name; the other
//   non-static functions, which av1_inv_txfm_avx2.c calls, are public Rust
//   functions.
// - The eob helpers use no intrinsics; they carry `ssse3`, the flag of the
//   header's own translation unit, and inline into SSE4.1 and AVX2 callers.
// - `int *` out-parameters and array parameters stay raw pointers
//   (`in[i]` -> `*in_.offset(i)`), so in-place calls keep the C aliasing.
//   Local arrays are Rust arrays (`x[i]` -> `x[i as usize]`) and start zeroed.
// - C integer conversions are explicit (`as`); `switch` becomes `match` on
//   the `int`-promoted value; `assert` becomes `debug_assert!`. Calling a NULL
//   `transform_1d_ssse3` entry, which C leaves undefined, panics through
//   `Option::unwrap`.
// - Immediate operands become const generics. Shift counts that are run-time
//   `int` values use the count-register intrinsics, which is what GCC and
//   Clang emit for a non-constant count.
// - The butterfly macros read `__rounding` and `cos_bit` from the caller's
//   scope in C; the Rust macros take them as trailing arguments.
// - The DECLARE_ALIGNED(16, ...) alignment of the eob tables is not needed:
//   they are only read element by element. The pointer table is `static mut`
//   because it holds raw pointers, as in the Neon translation.
#![allow(non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_parens)]
#![allow(unused_variables, dead_code, clippy::all)]

use core::arch::x86_64::*;

use crate::av1__common__av1_inv_txfm2d::av1_inv_txfm_shift_ls;
use crate::av1__common__av1_txfm::{av1_cospi_arr_data, av1_sinpi_arr_data};
use crate::av1__common__idct::{av1_inv_txfm_add_c, TxfmParam};
use crate::simd::aom_dsp__x86__transpose_sse2::{
    transpose_16bit_4x4, transpose_16bit_4x8, transpose_16bit_8x4, transpose_16bit_8x8,
};
use crate::simd::aom_dsp__x86__txfm_common_sse2::pair_set_epi16;
use crate::simd::av1__common__x86__av1_txfm_sse2::{
    btf_16_4p_sse2, btf_16_sse2, flip_buf_sse2, load_32bit_to_16bit, load_buffer_32bit_to_16bit,
    load_buffer_32bit_to_16bit_w4, round_shift_16bit,
};

