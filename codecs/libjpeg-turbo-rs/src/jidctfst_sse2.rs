// libjpeg-turbo 3.2.1, simd/x86_64/jidctfst-sse2.asm:
//
// ;
// ; Fast Integer Inverse DCT (64-bit SSE2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2024-2025, D. R. Commander.
// ; Copyright (C) 2018, Matthias Räncker.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
// ;
// ; This file contains a fast, not so accurate integer implementation of the
// ; inverse DCT (Discrete Cosine Transform).  The following code is based
// ; directly on the IJG's original jidctfst.c; see jidctfst.c for more details.
//! Translation of `simd/x86_64/jidctfst-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernel is SSE2 code, `#[target_feature(enable = "sse2")]`, which
//! every x86-64 target enables. Each vector instruction is the intrinsic that
//! compiles to it, in the asm's order, on local variables named after the
//! registers (ADR 0008, decision 3); a register move is an assignment and
//! the stack slots `wk(i)` are the local array `wk`. The calling convention
//! (`COLLECT_ARGS`, the pushes and pops, `ENDBR64`) and the `%ifdef WIN64`
//! branches of `jsimdext.inc` that it expands to have no translation.
//!
//! `IFAST_MULT_TYPE` is 16-bit with `WITH_SIMD` and `IFAST_SCALE_BITS` is 2,
//! as `jsimd_set_idct_ifast` requires, so the `%if` on
//! `IFAST_SCALE_BITS != PASS1_BITS` is a compile-time assertion, and
//! `CONST_BITS` is 8, which keeps the `%if CONST_BITS == 8` branch of the
//! constants. `NO_ZERO_COLUMN_TEST_IFAST_SSE2` is undefined, as in the
//! oracle's build, so the test for blocks whose AC terms are all zero is
//! translated. The multiplier table is the component's `ifast_array`.
//!
//! `XMMBLOCK(m, 0, rsi, SIZEOF_JCOEF)` of `jdct.inc` is row `m` of the
//! coefficient block and `XMMBLOCK(m, 0, rdx, SIZEOF_IFAST_MULT_TYPE)` row
//! `m` of the table, which the kernel takes from `as_chunks`;
//! `DWBLOCK(m, 0, rsi, SIZEOF_JCOEF)` is the 32-bit value of the first two
//! coefficients of row `m`. An output row `output_buf[i] + output_col` is
//! the row's slice from `output_col` on, and each `movq` store converts its
//! first 8 samples to the array that `_mm_storel_epi64` takes, so it writes
//! the 8 samples of the block's row and nothing past them. The kernel takes
//! each row pointer when it stores, as `output_buf` lends one row at a time.
//! The `prefetchnta` hints for the next coefficient block are not
//! translated: they change no result, and they would need a pointer past
//! `coef_block`, which the module's safe code cannot form.

#![forbid(unsafe_code)]

// Clippy does not report wildcard imports in a test build.
#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "ADR 0008: the intrinsics by name, shadowed by the x86 loads and stores"
    )
)]
use core::arch::x86_64::*;

use crate::jdct::IFAST_SCALE_BITS;
use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, JDIMENSION};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT};
use crate::x86::{_mm_loadu_si128, _mm_storel_epi64};

// upstream: simd/x86_64/jidctfst-sse2.asm CONST_BITS
const CONST_BITS: i32 = 8; /* 14 is also OK. */
// upstream: simd/x86_64/jidctfst-sse2.asm PASS1_BITS
const PASS1_BITS: i32 = 2;

const _: () = assert!(
    IFAST_SCALE_BITS.cast_signed() == PASS1_BITS,
    "'IFAST_SCALE_BITS' must be equal to 'PASS1_BITS'."
);

// upstream: simd/x86_64/jidctfst-sse2.asm F_1_082
const F_1_082: i16 = 277; /* FIX(1.082392200) */
// upstream: simd/x86_64/jidctfst-sse2.asm F_1_414
const F_1_414: i16 = 362; /* FIX(1.414213562) */
// upstream: simd/x86_64/jidctfst-sse2.asm F_1_847
const F_1_847: i16 = 473; /* FIX(1.847759065) */
// upstream: simd/x86_64/jidctfst-sse2.asm F_2_613
const F_2_613: i16 = 669; /* FIX(2.613125930) */
// upstream: simd/x86_64/jidctfst-sse2.asm F_1_613
const F_1_613: i16 = F_2_613 - 256; /* FIX(2.613125930) - FIX(1) */

// PRE_MULTIPLY_SCALE_BITS <= 2 (to avoid overflow)
// CONST_BITS + CONST_SHIFT + PRE_MULTIPLY_SCALE_BITS == 16 (for pmulhw)

// upstream: simd/x86_64/jidctfst-sse2.asm PRE_MULTIPLY_SCALE_BITS
const PRE_MULTIPLY_SCALE_BITS: i32 = 2;
// upstream: simd/x86_64/jidctfst-sse2.asm CONST_SHIFT
const CONST_SHIFT: i32 = 16 - PRE_MULTIPLY_SCALE_BITS - CONST_BITS;

/// The layout of `jconst_idct_ifast_sse2`: the asm's labels as fields,
/// aligned as `ALIGNZ 32` aligns the table. Not an upstream item.
#[expect(non_snake_case, reason = "the asm's labels")]
#[repr(C, align(32))]
pub struct JconstIdctIfastSse2 {
    PW_F1414: [i16; 8],
    PW_F1847: [i16; 8],
    PW_MF1613: [i16; 8],
    PW_F1082: [i16; 8],
    PB_CENTERJSAMP: [u8; 16],
}

// upstream: simd/x86_64/jidctfst-sse2.asm jconst_idct_ifast_sse2
#[expect(non_upper_case_globals, reason = "the asm's label")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the asm's db CENTERJSAMPLE"
)]
pub static jconst_idct_ifast_sse2: JconstIdctIfastSse2 = JconstIdctIfastSse2 {
    PW_F1414: [F_1_414 << CONST_SHIFT; 8],
    PW_F1847: [F_1_847 << CONST_SHIFT; 8],
    PW_MF1613: [-F_1_613 << CONST_SHIFT; 8],
    PW_F1082: [F_1_082 << CONST_SHIFT; 8],
    PB_CENTERJSAMP: [CENTERJSAMPLE as u8; 16],
};

// upstream: simd/x86_64/jidctfst-sse2.asm WK_NUM
const WK_NUM: usize = 2;

/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds 8 samples from output_col on";

// upstream: simd/x86_64/jidctfst-sse2.asm jsimd_idct_ifast_sse2
/// Perform dequantization and inverse DCT on one block of coefficients.
///
/// # Panics
///
/// When `output_buf` has fewer than 8 rows or 8 columns from `output_col`
/// on.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(
    clippy::used_underscore_items,
    reason = "the x86 loads and stores have the intrinsics' names"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_idct_ifast_sse2(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut wk = [_mm_setzero_si128(); WK_NUM]; /* xmmword wk[WK_NUM] */
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut xmm2: __m128i;
    let mut xmm3: __m128i;
    let mut xmm4: __m128i;
    let mut xmm5: __m128i;
    let mut xmm6: __m128i;
    let mut xmm7: __m128i;

    // ---- Pass 1: process columns from input.

    let rdx = dct_table.ifast_array().as_chunks::<DCTSIZE>().0; // quantptr
    let rsi = coef_block.as_chunks::<DCTSIZE>().0; // inptr

    'column_end: {
        // %ifndef NO_ZERO_COLUMN_TEST_IFAST_SSE2
        let mut eax =
            u32::from(rsi[1][0].cast_unsigned()) | u32::from(rsi[1][1].cast_unsigned()) << 16;
        eax |= u32::from(rsi[2][0].cast_unsigned()) | u32::from(rsi[2][1].cast_unsigned()) << 16;
        if eax == 0 {
            xmm0 = _mm_loadu_si128(&rsi[1]);
            xmm1 = _mm_loadu_si128(&rsi[2]);
            xmm0 = _mm_or_si128(xmm0, _mm_loadu_si128(&rsi[3]));
            xmm1 = _mm_or_si128(xmm1, _mm_loadu_si128(&rsi[4]));
            xmm0 = _mm_or_si128(xmm0, _mm_loadu_si128(&rsi[5]));
            xmm1 = _mm_or_si128(xmm1, _mm_loadu_si128(&rsi[6]));
            xmm0 = _mm_or_si128(xmm0, _mm_loadu_si128(&rsi[7]));
            xmm1 = _mm_or_si128(xmm1, xmm0);
            xmm1 = _mm_packs_epi16(xmm1, xmm1);
            xmm1 = _mm_packs_epi16(xmm1, xmm1);
            if _mm_cvtsi128_si32(xmm1) == 0 {
                // -- AC terms all zero

                xmm0 = _mm_loadu_si128(&rsi[0]);
                xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&rdx[0]));

                xmm7 = xmm0; // xmm0 = in0 = (00 01 02 03 04 05 06 07)
                xmm0 = _mm_unpacklo_epi16(xmm0, xmm0); // xmm0 = (00 00 01 01 02 02 03 03)
                xmm7 = _mm_unpackhi_epi16(xmm7, xmm7); // xmm7 = (04 04 05 05 06 06 07 07)

                xmm6 = _mm_shuffle_epi32::<0x00>(xmm0); // xmm6 = col0 = (00 00 00 00 00 00 00 00)
                xmm2 = _mm_shuffle_epi32::<0x55>(xmm0); // xmm2 = col1 = (01 01 01 01 01 01 01 01)
                xmm5 = _mm_shuffle_epi32::<0xAA>(xmm0); // xmm5 = col2 = (02 02 02 02 02 02 02 02)
                xmm0 = _mm_shuffle_epi32::<0xFF>(xmm0); // xmm0 = col3 = (03 03 03 03 03 03 03 03)
                xmm1 = _mm_shuffle_epi32::<0x00>(xmm7); // xmm1 = col4 = (04 04 04 04 04 04 04 04)
                xmm4 = _mm_shuffle_epi32::<0x55>(xmm7); // xmm4 = col5 = (05 05 05 05 05 05 05 05)
                xmm3 = _mm_shuffle_epi32::<0xAA>(xmm7); // xmm3 = col6 = (06 06 06 06 06 06 06 06)
                xmm7 = _mm_shuffle_epi32::<0xFF>(xmm7); // xmm7 = col7 = (07 07 07 07 07 07 07 07)

                wk[0] = xmm2; // wk(0) = col1
                wk[1] = xmm0; // wk(1) = col3
                break 'column_end;
            }
        }
        // %endif
        // .columnDCT:

        // -- Even part

        xmm0 = _mm_loadu_si128(&rsi[0]);
        xmm1 = _mm_loadu_si128(&rsi[2]);
        xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&rdx[0]));
        xmm1 = _mm_mullo_epi16(xmm1, _mm_loadu_si128(&rdx[2]));
        xmm2 = _mm_loadu_si128(&rsi[4]);
        xmm3 = _mm_loadu_si128(&rsi[6]);
        xmm2 = _mm_mullo_epi16(xmm2, _mm_loadu_si128(&rdx[4]));
        xmm3 = _mm_mullo_epi16(xmm3, _mm_loadu_si128(&rdx[6]));

        xmm4 = xmm0;
        xmm5 = xmm1;
        xmm0 = _mm_sub_epi16(xmm0, xmm2); // xmm0 = tmp11
        xmm1 = _mm_sub_epi16(xmm1, xmm3);
        xmm4 = _mm_add_epi16(xmm4, xmm2); // xmm4 = tmp10
        xmm5 = _mm_add_epi16(xmm5, xmm3); // xmm5 = tmp13

        xmm1 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm1);
        xmm1 = _mm_mulhi_epi16(xmm1, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1414));
        xmm1 = _mm_sub_epi16(xmm1, xmm5); // xmm1 = tmp12

        xmm6 = xmm4;
        xmm7 = xmm0;
        xmm4 = _mm_sub_epi16(xmm4, xmm5); // xmm4 = tmp3
        xmm0 = _mm_sub_epi16(xmm0, xmm1); // xmm0 = tmp2
        xmm6 = _mm_add_epi16(xmm6, xmm5); // xmm6 = tmp0
        xmm7 = _mm_add_epi16(xmm7, xmm1); // xmm7 = tmp1

        wk[1] = xmm4; // wk(1) = tmp3
        wk[0] = xmm0; // wk(0) = tmp2

        // -- Odd part

        xmm2 = _mm_loadu_si128(&rsi[1]);
        xmm3 = _mm_loadu_si128(&rsi[3]);
        xmm2 = _mm_mullo_epi16(xmm2, _mm_loadu_si128(&rdx[1]));
        xmm3 = _mm_mullo_epi16(xmm3, _mm_loadu_si128(&rdx[3]));
        xmm5 = _mm_loadu_si128(&rsi[5]);
        xmm1 = _mm_loadu_si128(&rsi[7]);
        xmm5 = _mm_mullo_epi16(xmm5, _mm_loadu_si128(&rdx[5]));
        xmm1 = _mm_mullo_epi16(xmm1, _mm_loadu_si128(&rdx[7]));

        xmm4 = xmm2;
        xmm0 = xmm5;
        xmm2 = _mm_sub_epi16(xmm2, xmm1); // xmm2 = z12
        xmm5 = _mm_sub_epi16(xmm5, xmm3); // xmm5 = z10
        xmm4 = _mm_add_epi16(xmm4, xmm1); // xmm4 = z11
        xmm0 = _mm_add_epi16(xmm0, xmm3); // xmm0 = z13

        xmm1 = xmm5; // xmm1 = z10(unscaled)
        xmm2 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm2);
        xmm5 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm5);

        xmm3 = xmm4;
        xmm4 = _mm_sub_epi16(xmm4, xmm0);
        xmm3 = _mm_add_epi16(xmm3, xmm0); // xmm3 = tmp7

        xmm4 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm4);
        xmm4 = _mm_mulhi_epi16(xmm4, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1414)); // xmm4 = tmp11

        // To avoid overflow...
        //
        // (Original)
        // tmp12 = -2.613125930 * z10 + z5;
        //
        // (This implementation)
        // tmp12 = (-1.613125930 - 1) * z10 + z5;
        //       = -1.613125930 * z10 - z10 + z5;

        xmm0 = xmm5;
        xmm5 = _mm_add_epi16(xmm5, xmm2);
        xmm5 = _mm_mulhi_epi16(xmm5, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1847)); // xmm5 = z5
        xmm0 = _mm_mulhi_epi16(xmm0, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_MF1613));
        xmm2 = _mm_mulhi_epi16(xmm2, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1082));
        xmm0 = _mm_sub_epi16(xmm0, xmm1);
        xmm2 = _mm_sub_epi16(xmm2, xmm5); // xmm2 = tmp10
        xmm0 = _mm_add_epi16(xmm0, xmm5); // xmm0 = tmp12

        // -- Final output stage

        xmm0 = _mm_sub_epi16(xmm0, xmm3); // xmm0 = tmp6
        xmm1 = xmm6;
        xmm5 = xmm7;
        xmm6 = _mm_add_epi16(xmm6, xmm3); // xmm6 = data0 = (00 01 02 03 04 05 06 07)
        xmm7 = _mm_add_epi16(xmm7, xmm0); // xmm7 = data1 = (10 11 12 13 14 15 16 17)
        xmm1 = _mm_sub_epi16(xmm1, xmm3); // xmm1 = data7 = (70 71 72 73 74 75 76 77)
        xmm5 = _mm_sub_epi16(xmm5, xmm0); // xmm5 = data6 = (60 61 62 63 64 65 66 67)
        xmm4 = _mm_sub_epi16(xmm4, xmm0); // xmm4 = tmp5

        xmm3 = xmm6; // transpose coefficients(phase 1)
        xmm6 = _mm_unpacklo_epi16(xmm6, xmm7); // xmm6 = (00 10 01 11 02 12 03 13)
        xmm3 = _mm_unpackhi_epi16(xmm3, xmm7); // xmm3 = (04 14 05 15 06 16 07 17)
        xmm0 = xmm5; // transpose coefficients(phase 1)
        xmm5 = _mm_unpacklo_epi16(xmm5, xmm1); // xmm5 = (60 70 61 71 62 72 63 73)
        xmm0 = _mm_unpackhi_epi16(xmm0, xmm1); // xmm0 = (64 74 65 75 66 76 67 77)

        xmm7 = wk[0]; // xmm7 = tmp2
        xmm1 = wk[1]; // xmm1 = tmp3

        wk[0] = xmm5; // wk(0) = (60 70 61 71 62 72 63 73)
        wk[1] = xmm0; // wk(1) = (64 74 65 75 66 76 67 77)

        xmm2 = _mm_add_epi16(xmm2, xmm4); // xmm2 = tmp4
        xmm5 = xmm7;
        xmm0 = xmm1;
        xmm7 = _mm_add_epi16(xmm7, xmm4); // xmm7 = data2 = (20 21 22 23 24 25 26 27)
        xmm1 = _mm_add_epi16(xmm1, xmm2); // xmm1 = data4 = (40 41 42 43 44 45 46 47)
        xmm5 = _mm_sub_epi16(xmm5, xmm4); // xmm5 = data5 = (50 51 52 53 54 55 56 57)
        xmm0 = _mm_sub_epi16(xmm0, xmm2); // xmm0 = data3 = (30 31 32 33 34 35 36 37)

        xmm4 = xmm7; // transpose coefficients(phase 1)
        xmm7 = _mm_unpacklo_epi16(xmm7, xmm0); // xmm7 = (20 30 21 31 22 32 23 33)
        xmm4 = _mm_unpackhi_epi16(xmm4, xmm0); // xmm4 = (24 34 25 35 26 36 27 37)
        xmm2 = xmm1; // transpose coefficients(phase 1)
        xmm1 = _mm_unpacklo_epi16(xmm1, xmm5); // xmm1 = (40 50 41 51 42 52 43 53)
        xmm2 = _mm_unpackhi_epi16(xmm2, xmm5); // xmm2 = (44 54 45 55 46 56 47 57)

        xmm0 = xmm3; // transpose coefficients(phase 2)
        xmm3 = _mm_unpacklo_epi32(xmm3, xmm4); // xmm3 = (04 14 24 34 05 15 25 35)
        xmm0 = _mm_unpackhi_epi32(xmm0, xmm4); // xmm0 = (06 16 26 36 07 17 27 37)
        xmm5 = xmm6; // transpose coefficients(phase 2)
        xmm6 = _mm_unpacklo_epi32(xmm6, xmm7); // xmm6 = (00 10 20 30 01 11 21 31)
        xmm5 = _mm_unpackhi_epi32(xmm5, xmm7); // xmm5 = (02 12 22 32 03 13 23 33)

        xmm4 = wk[0]; // xmm4 = (60 70 61 71 62 72 63 73)
        xmm7 = wk[1]; // xmm7 = (64 74 65 75 66 76 67 77)

        wk[0] = xmm3; // wk(0) = (04 14 24 34 05 15 25 35)
        wk[1] = xmm0; // wk(1) = (06 16 26 36 07 17 27 37)

        xmm3 = xmm1; // transpose coefficients(phase 2)
        xmm1 = _mm_unpacklo_epi32(xmm1, xmm4); // xmm1 = (40 50 60 70 41 51 61 71)
        xmm3 = _mm_unpackhi_epi32(xmm3, xmm4); // xmm3 = (42 52 62 72 43 53 63 73)
        xmm0 = xmm2; // transpose coefficients(phase 2)
        xmm2 = _mm_unpacklo_epi32(xmm2, xmm7); // xmm2 = (44 54 64 74 45 55 65 75)
        xmm0 = _mm_unpackhi_epi32(xmm0, xmm7); // xmm0 = (46 56 66 76 47 57 67 77)

        xmm4 = xmm6; // transpose coefficients(phase 3)
        xmm6 = _mm_unpacklo_epi64(xmm6, xmm1); // xmm6 = col0 = (00 10 20 30 40 50 60 70)
        xmm4 = _mm_unpackhi_epi64(xmm4, xmm1); // xmm4 = col1 = (01 11 21 31 41 51 61 71)
        xmm7 = xmm5; // transpose coefficients(phase 3)
        xmm5 = _mm_unpacklo_epi64(xmm5, xmm3); // xmm5 = col2 = (02 12 22 32 42 52 62 72)
        xmm7 = _mm_unpackhi_epi64(xmm7, xmm3); // xmm7 = col3 = (03 13 23 33 43 53 63 73)

        xmm1 = wk[0]; // xmm1 = (04 14 24 34 05 15 25 35)
        xmm3 = wk[1]; // xmm3 = (06 16 26 36 07 17 27 37)

        wk[0] = xmm4; // wk(0) = col1
        wk[1] = xmm7; // wk(1) = col3

        xmm4 = xmm1; // transpose coefficients(phase 3)
        xmm1 = _mm_unpacklo_epi64(xmm1, xmm2); // xmm1 = col4 = (04 14 24 34 44 54 64 74)
        xmm4 = _mm_unpackhi_epi64(xmm4, xmm2); // xmm4 = col5 = (05 15 25 35 45 55 65 75)
        xmm7 = xmm3; // transpose coefficients(phase 3)
        xmm3 = _mm_unpacklo_epi64(xmm3, xmm0); // xmm3 = col6 = (06 16 26 36 46 56 66 76)
        xmm7 = _mm_unpackhi_epi64(xmm7, xmm0); // xmm7 = col7 = (07 17 27 37 47 57 67 77)
    }
    // .column_end:

    // -- Prefetch the next coefficient block
    // (prefetchnta: not translated; see the module documentation)

    // ---- Pass 2: process rows from work array, store into output array.

    let rdi = &mut output_buf; // (JSAMPROW *)
    let eax = output_col as usize;

    // -- Even part

    // xmm6 = col0, xmm5 = col2, xmm1 = col4, xmm3 = col6

    xmm2 = xmm6;
    xmm0 = xmm5;
    xmm6 = _mm_sub_epi16(xmm6, xmm1); // xmm6 = tmp11
    xmm5 = _mm_sub_epi16(xmm5, xmm3);
    xmm2 = _mm_add_epi16(xmm2, xmm1); // xmm2 = tmp10
    xmm0 = _mm_add_epi16(xmm0, xmm3); // xmm0 = tmp13

    xmm5 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm5);
    xmm5 = _mm_mulhi_epi16(xmm5, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1414));
    xmm5 = _mm_sub_epi16(xmm5, xmm0); // xmm5 = tmp12

    xmm1 = xmm2;
    xmm3 = xmm6;
    xmm2 = _mm_sub_epi16(xmm2, xmm0); // xmm2 = tmp3
    xmm6 = _mm_sub_epi16(xmm6, xmm5); // xmm6 = tmp2
    xmm1 = _mm_add_epi16(xmm1, xmm0); // xmm1 = tmp0
    xmm3 = _mm_add_epi16(xmm3, xmm5); // xmm3 = tmp1

    xmm0 = wk[0]; // xmm0 = col1
    xmm5 = wk[1]; // xmm5 = col3

    wk[0] = xmm2; // wk(0) = tmp3
    wk[1] = xmm6; // wk(1) = tmp2

    // -- Odd part

    // xmm0 = col1, xmm5 = col3, xmm4 = col5, xmm7 = col7

    xmm2 = xmm0;
    xmm6 = xmm4;
    xmm0 = _mm_sub_epi16(xmm0, xmm7); // xmm0 = z12
    xmm4 = _mm_sub_epi16(xmm4, xmm5); // xmm4 = z10
    xmm2 = _mm_add_epi16(xmm2, xmm7); // xmm2 = z11
    xmm6 = _mm_add_epi16(xmm6, xmm5); // xmm6 = z13

    xmm7 = xmm4; // xmm7 = z10(unscaled)
    xmm0 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm0);
    xmm4 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm4);

    xmm5 = xmm2;
    xmm2 = _mm_sub_epi16(xmm2, xmm6);
    xmm5 = _mm_add_epi16(xmm5, xmm6); // xmm5 = tmp7

    xmm2 = _mm_slli_epi16::<PRE_MULTIPLY_SCALE_BITS>(xmm2);
    xmm2 = _mm_mulhi_epi16(xmm2, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1414)); // xmm2 = tmp11

    // To avoid overflow...
    //
    // (Original)
    // tmp12 = -2.613125930 * z10 + z5;
    //
    // (This implementation)
    // tmp12 = (-1.613125930 - 1) * z10 + z5;
    //       = -1.613125930 * z10 - z10 + z5;

    xmm6 = xmm4;
    xmm4 = _mm_add_epi16(xmm4, xmm0);
    xmm4 = _mm_mulhi_epi16(xmm4, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1847)); // xmm4 = z5
    xmm6 = _mm_mulhi_epi16(xmm6, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_MF1613));
    xmm0 = _mm_mulhi_epi16(xmm0, _mm_loadu_si128(&jconst_idct_ifast_sse2.PW_F1082));
    xmm6 = _mm_sub_epi16(xmm6, xmm7);
    xmm0 = _mm_sub_epi16(xmm0, xmm4); // xmm0 = tmp10
    xmm6 = _mm_add_epi16(xmm6, xmm4); // xmm6 = tmp12

    // -- Final output stage

    xmm6 = _mm_sub_epi16(xmm6, xmm5); // xmm6 = tmp6
    xmm7 = xmm1;
    xmm4 = xmm3;
    xmm1 = _mm_add_epi16(xmm1, xmm5); // xmm1 = data0 = (00 10 20 30 40 50 60 70)
    xmm3 = _mm_add_epi16(xmm3, xmm6); // xmm3 = data1 = (01 11 21 31 41 51 61 71)
    xmm1 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm1); // descale
    xmm3 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm3); // descale
    xmm7 = _mm_sub_epi16(xmm7, xmm5); // xmm7 = data7 = (07 17 27 37 47 57 67 77)
    xmm4 = _mm_sub_epi16(xmm4, xmm6); // xmm4 = data6 = (06 16 26 36 46 56 66 76)
    xmm7 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm7); // descale
    xmm4 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm4); // descale
    xmm2 = _mm_sub_epi16(xmm2, xmm6); // xmm2 = tmp5

    xmm1 = _mm_packs_epi16(xmm1, xmm4);
    // xmm1 = (00 10 20 30 40 50 60 70 06 16 26 36 46 56 66 76)
    xmm3 = _mm_packs_epi16(xmm3, xmm7);
    // xmm3 = (01 11 21 31 41 51 61 71 07 17 27 37 47 57 67 77)

    xmm5 = wk[1]; // xmm5 = tmp2
    xmm6 = wk[0]; // xmm6 = tmp3

    xmm0 = _mm_add_epi16(xmm0, xmm2); // xmm0 = tmp4
    xmm4 = xmm5;
    xmm7 = xmm6;
    xmm5 = _mm_add_epi16(xmm5, xmm2); // xmm5 = data2 = (02 12 22 32 42 52 62 72)
    xmm6 = _mm_add_epi16(xmm6, xmm0); // xmm6 = data4 = (04 14 24 34 44 54 64 74)
    xmm5 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm5); // descale
    xmm6 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm6); // descale
    xmm4 = _mm_sub_epi16(xmm4, xmm2); // xmm4 = data5 = (05 15 25 35 45 55 65 75)
    xmm7 = _mm_sub_epi16(xmm7, xmm0); // xmm7 = data3 = (03 13 23 33 43 53 63 73)
    xmm4 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm4); // descale
    xmm7 = _mm_srai_epi16::<{ PASS1_BITS + 3 }>(xmm7); // descale

    xmm2 = _mm_loadu_si128(&jconst_idct_ifast_sse2.PB_CENTERJSAMP); // xmm2 = [rel PB_CENTERJSAMP]

    xmm5 = _mm_packs_epi16(xmm5, xmm6);
    // xmm5 = (02 12 22 32 42 52 62 72 04 14 24 34 44 54 64 74)
    xmm7 = _mm_packs_epi16(xmm7, xmm4);
    // xmm7 = (03 13 23 33 43 53 63 73 05 15 25 35 45 55 65 75)

    xmm1 = _mm_add_epi8(xmm1, xmm2);
    xmm3 = _mm_add_epi8(xmm3, xmm2);
    xmm5 = _mm_add_epi8(xmm5, xmm2);
    xmm7 = _mm_add_epi8(xmm7, xmm2);

    // transpose coefficients(phase 1)
    xmm0 = xmm1;
    xmm1 = _mm_unpacklo_epi8(xmm1, xmm3);
    // xmm1 = (00 01 10 11 20 21 30 31 40 41 50 51 60 61 70 71)
    xmm0 = _mm_unpackhi_epi8(xmm0, xmm3);
    // xmm0 = (06 07 16 17 26 27 36 37 46 47 56 57 66 67 76 77)
    xmm6 = xmm5;
    xmm5 = _mm_unpacklo_epi8(xmm5, xmm7);
    // xmm5 = (02 03 12 13 22 23 32 33 42 43 52 53 62 63 72 73)
    xmm6 = _mm_unpackhi_epi8(xmm6, xmm7);
    // xmm6 = (04 05 14 15 24 25 34 35 44 45 54 55 64 65 74 75)

    // transpose coefficients(phase 2)
    xmm4 = xmm1;
    xmm1 = _mm_unpacklo_epi16(xmm1, xmm5);
    // xmm1 = (00 01 02 03 10 11 12 13 20 21 22 23 30 31 32 33)
    xmm4 = _mm_unpackhi_epi16(xmm4, xmm5);
    // xmm4 = (40 41 42 43 50 51 52 53 60 61 62 63 70 71 72 73)
    xmm2 = xmm6;
    xmm6 = _mm_unpacklo_epi16(xmm6, xmm0);
    // xmm6 = (04 05 06 07 14 15 16 17 24 25 26 27 34 35 36 37)
    xmm2 = _mm_unpackhi_epi16(xmm2, xmm0);
    // xmm2 = (44 45 46 47 54 55 56 57 64 65 66 67 74 75 76 77)

    // transpose coefficients(phase 3)
    xmm3 = xmm1;
    xmm1 = _mm_unpacklo_epi32(xmm1, xmm6);
    // xmm1 = (00 01 02 03 04 05 06 07 10 11 12 13 14 15 16 17)
    xmm3 = _mm_unpackhi_epi32(xmm3, xmm6);
    // xmm3 = (20 21 22 23 24 25 26 27 30 31 32 33 34 35 36 37)
    xmm7 = xmm4;
    xmm4 = _mm_unpacklo_epi32(xmm4, xmm2);
    // xmm4 = (40 41 42 43 44 45 46 47 50 51 52 53 54 55 56 57)
    xmm7 = _mm_unpackhi_epi32(xmm7, xmm2);
    // xmm7 = (60 61 62 63 64 65 66 67 70 71 72 73 74 75 76 77)

    xmm5 = _mm_shuffle_epi32::<0x4E>(xmm1);
    // xmm5 = (10 11 12 13 14 15 16 17 00 01 02 03 04 05 06 07)
    xmm0 = _mm_shuffle_epi32::<0x4E>(xmm3);
    // xmm0 = (30 31 32 33 34 35 36 37 20 21 22 23 24 25 26 27)
    xmm6 = _mm_shuffle_epi32::<0x4E>(xmm4);
    // xmm6 = (50 51 52 53 54 55 56 57 40 41 42 43 44 45 46 47)
    xmm2 = _mm_shuffle_epi32::<0x4E>(xmm7);
    // xmm2 = (70 71 72 73 74 75 76 77 60 61 62 63 64 65 66 67)

    let rdx = rdi.row_mut(0);
    _mm_storel_epi64(rdx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm1);
    let rsi = rdi.row_mut(2);
    _mm_storel_epi64(rsi[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm3);
    let rdx = rdi.row_mut(4);
    _mm_storel_epi64(rdx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm4);
    let rsi = rdi.row_mut(6);
    _mm_storel_epi64(rsi[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm7);

    let rdx = rdi.row_mut(1);
    _mm_storel_epi64(rdx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm5);
    let rsi = rdi.row_mut(3);
    _mm_storel_epi64(rsi[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm0);
    let rdx = rdi.row_mut(5);
    _mm_storel_epi64(rdx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm6);
    let rsi = rdi.row_mut(7);
    _mm_storel_epi64(rsi[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm2);
}
