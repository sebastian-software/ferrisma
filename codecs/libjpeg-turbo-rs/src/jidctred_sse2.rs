// libjpeg-turbo 3.2.1, simd/x86_64/jidctred-sse2.asm:
//
// ;
// ; Scaled Integer Inverse DCT (64-bit SSE2)
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
// ; This file contains inverse DCT routines that produce reduced-size output:
// ; either 4x4 or 2x2 pixels from an 8x8 DCT block.  The following code is based
// ; directly on the IJG's original jidctred.c; see jidctred.c for more details.
//! Translation of `simd/x86_64/jidctred-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernels are SSE2 code, `#[target_feature(enable = "sse2")]`, which
//! every x86-64 target enables. Each vector instruction is the intrinsic that
//! compiles to it, in the asm's order, on local variables named after the
//! registers (ADR 0008, decision 3); a register move is an assignment and
//! the stack slots `wk(i)` are the local array `wk`. The calling convention
//! (`COLLECT_ARGS`, the pushes and pops, `ENDBR64`) and the `%ifdef WIN64`
//! branches of `jsimdext.inc` that it expands to have no translation.
//!
//! `ISLOW_MULT_TYPE` is 16-bit with `WITH_SIMD`, and the multiplier table is
//! the component's `islow_array`. `CONST_BITS` is 13, which keeps the
//! `%if CONST_BITS == 13` branch of the constants.
//! `NO_ZERO_COLUMN_TEST_4X4_SSE2` is undefined, as in the oracle's build, so
//! the test for blocks whose AC terms are all zero is translated.
//!
//! `XMMBLOCK(m, 0, rsi, SIZEOF_JCOEF)` of `jdct.inc` is row `m` of the
//! coefficient block and `XMMBLOCK(m, 0, rdx, SIZEOF_ISLOW_MULT_TYPE)` row
//! `m` of the table, which the kernels take from `as_chunks`;
//! `DWBLOCK(m, 0, rsi, SIZEOF_JCOEF)` is the 32-bit value of the first two
//! coefficients of row `m`. An output row `output_buf[i] + output_col` is
//! the row's slice from `output_col` on. The partial stores `movd` (4
//! samples) and `mov word` (2 samples, after `pextrw`) take the value out of
//! the register and write its bytes in safe code, so that they write the
//! samples of the block's row and nothing past them. The kernels take each
//! row pointer when they store, as `output_buf` lends one row at a time. The
//! `prefetchnta` hints for the next coefficient block are not translated:
//! they change no result, and they would need a pointer past `coef_block`,
//! which the module's safe code cannot form.

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

use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, JDIMENSION};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT};
use crate::x86::_mm_loadu_si128;

// upstream: simd/x86_64/jidctred-sse2.asm CONST_BITS
const CONST_BITS: i32 = 13;
// upstream: simd/x86_64/jidctred-sse2.asm PASS1_BITS
const PASS1_BITS: i32 = 2;

// upstream: simd/x86_64/jidctred-sse2.asm DESCALE_P1_4
const DESCALE_P1_4: i32 = CONST_BITS - PASS1_BITS + 1;
// upstream: simd/x86_64/jidctred-sse2.asm DESCALE_P2_4
const DESCALE_P2_4: i32 = CONST_BITS + PASS1_BITS + 3 + 1;
// upstream: simd/x86_64/jidctred-sse2.asm DESCALE_P1_2
const DESCALE_P1_2: i32 = CONST_BITS - PASS1_BITS + 2;
// upstream: simd/x86_64/jidctred-sse2.asm DESCALE_P2_2
const DESCALE_P2_2: i32 = CONST_BITS + PASS1_BITS + 3 + 2;

// upstream: simd/x86_64/jidctred-sse2.asm F_0_211
const F_0_211: i16 = 1730; /* FIX(0.211164243) */
// upstream: simd/x86_64/jidctred-sse2.asm F_0_509
const F_0_509: i16 = 4176; /* FIX(0.509795579) */
// upstream: simd/x86_64/jidctred-sse2.asm F_0_601
const F_0_601: i16 = 4926; /* FIX(0.601344887) */
// upstream: simd/x86_64/jidctred-sse2.asm F_0_720
const F_0_720: i16 = 5906; /* FIX(0.720959822) */
// upstream: simd/x86_64/jidctred-sse2.asm F_0_765
const F_0_765: i16 = 6270; /* FIX(0.765366865) */
// upstream: simd/x86_64/jidctred-sse2.asm F_0_850
const F_0_850: i16 = 6967; /* FIX(0.850430095) */
// upstream: simd/x86_64/jidctred-sse2.asm F_0_899
const F_0_899: i16 = 7373; /* FIX(0.899976223) */
// upstream: simd/x86_64/jidctred-sse2.asm F_1_061
const F_1_061: i16 = 8697; /* FIX(1.061594337) */
// upstream: simd/x86_64/jidctred-sse2.asm F_1_272
const F_1_272: i16 = 10426; /* FIX(1.272758580) */
// upstream: simd/x86_64/jidctred-sse2.asm F_1_451
const F_1_451: i16 = 11893; /* FIX(1.451774981) */
// upstream: simd/x86_64/jidctred-sse2.asm F_1_847
const F_1_847: i16 = 15137; /* FIX(1.847759065) */
// upstream: simd/x86_64/jidctred-sse2.asm F_2_172
const F_2_172: i16 = 17799; /* FIX(2.172734803) */
// upstream: simd/x86_64/jidctred-sse2.asm F_2_562
const F_2_562: i16 = 20995; /* FIX(2.562915447) */
// upstream: simd/x86_64/jidctred-sse2.asm F_3_624
const F_3_624: i16 = 29692; /* FIX(3.624509785) */

// upstream: simd/nasm/jsimdext.inc WORD_BIT
const WORD_BIT: i32 = 16; /* sizeof(word) * BYTE_BIT */

/// `times 4 dw a, b`: the pair `a, b` four times. Not an upstream item.
const fn times4(a: i16, b: i16) -> [i16; 8] {
    [a, b, a, b, a, b, a, b]
}

/// The layout of `jconst_idct_red_sse2`: the asm's labels as fields,
/// aligned as `ALIGNZ 32` aligns the table. Not an upstream item.
#[expect(non_snake_case, reason = "the asm's labels")]
#[repr(C, align(32))]
pub struct JconstIdctRedSse2 {
    PW_F184_MF076: [i16; 8],
    PW_F256_F089: [i16; 8],
    PW_F106_MF217: [i16; 8],
    PW_MF060_MF050: [i16; 8],
    PW_F145_MF021: [i16; 8],
    PW_F362_MF127: [i16; 8],
    PW_F085_MF072: [i16; 8],
    PD_DESCALE_P1_4: [i32; 4],
    PD_DESCALE_P2_4: [i32; 4],
    PD_DESCALE_P1_2: [i32; 4],
    PD_DESCALE_P2_2: [i32; 4],
    PB_CENTERJSAMP: [u8; 16],
}

// upstream: simd/x86_64/jidctred-sse2.asm jconst_idct_red_sse2
#[expect(non_upper_case_globals, reason = "the asm's label")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the asm's db CENTERJSAMPLE"
)]
pub static jconst_idct_red_sse2: JconstIdctRedSse2 = JconstIdctRedSse2 {
    PW_F184_MF076: times4(F_1_847, -F_0_765),
    PW_F256_F089: times4(F_2_562, F_0_899),
    PW_F106_MF217: times4(F_1_061, -F_2_172),
    PW_MF060_MF050: times4(-F_0_601, -F_0_509),
    PW_F145_MF021: times4(F_1_451, -F_0_211),
    PW_F362_MF127: times4(F_3_624, -F_1_272),
    PW_F085_MF072: times4(F_0_850, -F_0_720),
    PD_DESCALE_P1_4: [1 << (DESCALE_P1_4 - 1); 4],
    PD_DESCALE_P2_4: [1 << (DESCALE_P2_4 - 1); 4],
    PD_DESCALE_P1_2: [1 << (DESCALE_P1_2 - 1); 4],
    PD_DESCALE_P2_2: [1 << (DESCALE_P2_2 - 1); 4],
    PB_CENTERJSAMP: [CENTERJSAMPLE as u8; 16],
};

// upstream: simd/x86_64/jidctred-sse2.asm WK_NUM
const WK_NUM: usize = 2;

/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds the block's samples from output_col on";

// upstream: simd/x86_64/jidctred-sse2.asm jsimd_idct_4x4_sse2
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 4x4 output block.
///
/// # Panics
///
/// When `output_buf` has fewer than 4 rows or 4 columns from `output_col`
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
pub fn jsimd_idct_4x4_sse2(
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

    let rdx = dct_table.islow_array().as_chunks::<DCTSIZE>().0; // quantptr
    let rsi = coef_block.as_chunks::<DCTSIZE>().0; // inptr

    'column_end: {
        // %ifndef NO_ZERO_COLUMN_TEST_4X4_SSE2
        let mut eax =
            u32::from(rsi[1][0].cast_unsigned()) | u32::from(rsi[1][1].cast_unsigned()) << 16;
        eax |= u32::from(rsi[2][0].cast_unsigned()) | u32::from(rsi[2][1].cast_unsigned()) << 16;
        if eax == 0 {
            xmm0 = _mm_loadu_si128(&rsi[1]);
            xmm1 = _mm_loadu_si128(&rsi[2]);
            xmm0 = _mm_or_si128(xmm0, _mm_loadu_si128(&rsi[3]));
            xmm1 = _mm_or_si128(xmm1, _mm_loadu_si128(&rsi[5]));
            xmm0 = _mm_or_si128(xmm0, _mm_loadu_si128(&rsi[6]));
            xmm1 = _mm_or_si128(xmm1, _mm_loadu_si128(&rsi[7]));
            xmm0 = _mm_or_si128(xmm0, xmm1);
            xmm0 = _mm_packs_epi16(xmm0, xmm0);
            xmm0 = _mm_packs_epi16(xmm0, xmm0);
            if _mm_cvtsi128_si32(xmm0) == 0 {
                // -- AC terms all zero

                xmm0 = _mm_loadu_si128(&rsi[0]);
                xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&rdx[0]));

                xmm0 = _mm_slli_epi16::<PASS1_BITS>(xmm0);

                xmm3 = xmm0; // xmm0 = in0 = (00 01 02 03 04 05 06 07)
                xmm0 = _mm_unpacklo_epi16(xmm0, xmm0); // xmm0 = (00 00 01 01 02 02 03 03)
                xmm3 = _mm_unpackhi_epi16(xmm3, xmm3); // xmm3 = (04 04 05 05 06 06 07 07)

                xmm1 = _mm_shuffle_epi32::<0x50>(xmm0);
                // xmm1 = [col0 col1] = (00 00 00 00 01 01 01 01)
                xmm0 = _mm_shuffle_epi32::<0xFA>(xmm0);
                // xmm0 = [col2 col3] = (02 02 02 02 03 03 03 03)
                xmm6 = _mm_shuffle_epi32::<0x50>(xmm3);
                // xmm6 = [col4 col5] = (04 04 04 04 05 05 05 05)
                xmm3 = _mm_shuffle_epi32::<0xFA>(xmm3);
                // xmm3 = [col6 col7] = (06 06 06 06 07 07 07 07)

                break 'column_end;
            }
        }
        // %endif
        // .columnDCT:

        // -- Odd part

        xmm0 = _mm_loadu_si128(&rsi[1]);
        xmm1 = _mm_loadu_si128(&rsi[3]);
        xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&rdx[1]));
        xmm1 = _mm_mullo_epi16(xmm1, _mm_loadu_si128(&rdx[3]));
        xmm2 = _mm_loadu_si128(&rsi[5]);
        xmm3 = _mm_loadu_si128(&rsi[7]);
        xmm2 = _mm_mullo_epi16(xmm2, _mm_loadu_si128(&rdx[5]));
        xmm3 = _mm_mullo_epi16(xmm3, _mm_loadu_si128(&rdx[7]));

        xmm4 = xmm0;
        xmm5 = xmm0;
        xmm4 = _mm_unpacklo_epi16(xmm4, xmm1);
        xmm5 = _mm_unpackhi_epi16(xmm5, xmm1);
        xmm0 = xmm4;
        xmm1 = xmm5;
        xmm4 = _mm_madd_epi16(xmm4, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F256_F089)); // xmm4 = (tmp2L)
        xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F256_F089)); // xmm5 = (tmp2H)
        xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F106_MF217)); // xmm0 = (tmp0L)
        xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F106_MF217)); // xmm1 = (tmp0H)

        xmm6 = xmm2;
        xmm7 = xmm2;
        xmm6 = _mm_unpacklo_epi16(xmm6, xmm3);
        xmm7 = _mm_unpackhi_epi16(xmm7, xmm3);
        xmm2 = xmm6;
        xmm3 = xmm7;
        xmm6 = _mm_madd_epi16(xmm6, _mm_loadu_si128(&jconst_idct_red_sse2.PW_MF060_MF050)); // xmm6 = (tmp2L)
        xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&jconst_idct_red_sse2.PW_MF060_MF050)); // xmm7 = (tmp2H)
        xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F145_MF021)); // xmm2 = (tmp0L)
        xmm3 = _mm_madd_epi16(xmm3, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F145_MF021)); // xmm3 = (tmp0H)

        xmm6 = _mm_add_epi32(xmm6, xmm4); // xmm6 = tmp2L
        xmm7 = _mm_add_epi32(xmm7, xmm5); // xmm7 = tmp2H
        xmm2 = _mm_add_epi32(xmm2, xmm0); // xmm2 = tmp0L
        xmm3 = _mm_add_epi32(xmm3, xmm1); // xmm3 = tmp0H

        wk[0] = xmm2; // wk(0) = tmp0L
        wk[1] = xmm3; // wk(1) = tmp0H

        // -- Even part

        xmm4 = _mm_loadu_si128(&rsi[0]);
        xmm5 = _mm_loadu_si128(&rsi[2]);
        xmm0 = _mm_loadu_si128(&rsi[6]);
        xmm4 = _mm_mullo_epi16(xmm4, _mm_loadu_si128(&rdx[0]));
        xmm5 = _mm_mullo_epi16(xmm5, _mm_loadu_si128(&rdx[2]));
        xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&rdx[6]));

        xmm1 = _mm_setzero_si128();
        xmm2 = _mm_setzero_si128();
        xmm1 = _mm_unpacklo_epi16(xmm1, xmm4); // xmm1 = tmp0L
        xmm2 = _mm_unpackhi_epi16(xmm2, xmm4); // xmm2 = tmp0H
        xmm1 = _mm_srai_epi32::<{ 16 - CONST_BITS - 1 }>(xmm1);
        // psrad xmm1, 16 & pslld xmm1, CONST_BITS + 1
        xmm2 = _mm_srai_epi32::<{ 16 - CONST_BITS - 1 }>(xmm2);
        // psrad xmm2, 16 & pslld xmm2, CONST_BITS + 1

        xmm3 = xmm5; // xmm5 = in2 = z2
        xmm5 = _mm_unpacklo_epi16(xmm5, xmm0); // xmm0 = in6 = z3
        xmm3 = _mm_unpackhi_epi16(xmm3, xmm0);
        xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F184_MF076)); // xmm5 = tmp2L
        xmm3 = _mm_madd_epi16(xmm3, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F184_MF076)); // xmm3 = tmp2H

        xmm4 = xmm1;
        xmm0 = xmm2;
        xmm1 = _mm_add_epi32(xmm1, xmm5); // xmm1 = tmp10L
        xmm2 = _mm_add_epi32(xmm2, xmm3); // xmm2 = tmp10H
        xmm4 = _mm_sub_epi32(xmm4, xmm5); // xmm4 = tmp12L
        xmm0 = _mm_sub_epi32(xmm0, xmm3); // xmm0 = tmp12H

        // -- Final output stage

        xmm5 = xmm1;
        xmm3 = xmm2;
        xmm1 = _mm_add_epi32(xmm1, xmm6); // xmm1 = data0L
        xmm2 = _mm_add_epi32(xmm2, xmm7); // xmm2 = data0H
        xmm5 = _mm_sub_epi32(xmm5, xmm6); // xmm5 = data3L
        xmm3 = _mm_sub_epi32(xmm3, xmm7); // xmm3 = data3H

        xmm6 = _mm_loadu_si128(&jconst_idct_red_sse2.PD_DESCALE_P1_4); // xmm6 = [rel PD_DESCALE_P1_4]

        xmm1 = _mm_add_epi32(xmm1, xmm6);
        xmm2 = _mm_add_epi32(xmm2, xmm6);
        xmm1 = _mm_srai_epi32::<DESCALE_P1_4>(xmm1);
        xmm2 = _mm_srai_epi32::<DESCALE_P1_4>(xmm2);
        xmm5 = _mm_add_epi32(xmm5, xmm6);
        xmm3 = _mm_add_epi32(xmm3, xmm6);
        xmm5 = _mm_srai_epi32::<DESCALE_P1_4>(xmm5);
        xmm3 = _mm_srai_epi32::<DESCALE_P1_4>(xmm3);

        xmm1 = _mm_packs_epi32(xmm1, xmm2); // xmm1 = data0 = (00 01 02 03 04 05 06 07)
        xmm5 = _mm_packs_epi32(xmm5, xmm3); // xmm5 = data3 = (30 31 32 33 34 35 36 37)

        xmm7 = wk[0]; // xmm7 = tmp0L
        xmm6 = wk[1]; // xmm6 = tmp0H

        xmm2 = xmm4;
        xmm3 = xmm0;
        xmm4 = _mm_add_epi32(xmm4, xmm7); // xmm4 = data1L
        xmm0 = _mm_add_epi32(xmm0, xmm6); // xmm0 = data1H
        xmm2 = _mm_sub_epi32(xmm2, xmm7); // xmm2 = data2L
        xmm3 = _mm_sub_epi32(xmm3, xmm6); // xmm3 = data2H

        xmm7 = _mm_loadu_si128(&jconst_idct_red_sse2.PD_DESCALE_P1_4); // xmm7 = [rel PD_DESCALE_P1_4]

        xmm4 = _mm_add_epi32(xmm4, xmm7);
        xmm0 = _mm_add_epi32(xmm0, xmm7);
        xmm4 = _mm_srai_epi32::<DESCALE_P1_4>(xmm4);
        xmm0 = _mm_srai_epi32::<DESCALE_P1_4>(xmm0);
        xmm2 = _mm_add_epi32(xmm2, xmm7);
        xmm3 = _mm_add_epi32(xmm3, xmm7);
        xmm2 = _mm_srai_epi32::<DESCALE_P1_4>(xmm2);
        xmm3 = _mm_srai_epi32::<DESCALE_P1_4>(xmm3);

        xmm4 = _mm_packs_epi32(xmm4, xmm0); // xmm4 = data1 = (10 11 12 13 14 15 16 17)
        xmm2 = _mm_packs_epi32(xmm2, xmm3); // xmm2 = data2 = (20 21 22 23 24 25 26 27)

        xmm6 = xmm1; // transpose coefficients(phase 1)
        xmm1 = _mm_unpacklo_epi16(xmm1, xmm4); // xmm1 = (00 10 01 11 02 12 03 13)
        xmm6 = _mm_unpackhi_epi16(xmm6, xmm4); // xmm6 = (04 14 05 15 06 16 07 17)
        xmm7 = xmm2; // transpose coefficients(phase 1)
        xmm2 = _mm_unpacklo_epi16(xmm2, xmm5); // xmm2 = (20 30 21 31 22 32 23 33)
        xmm7 = _mm_unpackhi_epi16(xmm7, xmm5); // xmm7 = (24 34 25 35 26 36 27 37)

        xmm0 = xmm1; // transpose coefficients(phase 2)
        xmm1 = _mm_unpacklo_epi32(xmm1, xmm2); // xmm1 = [col0 col1] = (00 10 20 30 01 11 21 31)
        xmm0 = _mm_unpackhi_epi32(xmm0, xmm2); // xmm0 = [col2 col3] = (02 12 22 32 03 13 23 33)
        xmm3 = xmm6; // transpose coefficients(phase 2)
        xmm6 = _mm_unpacklo_epi32(xmm6, xmm7); // xmm6 = [col4 col5] = (04 14 24 34 05 15 25 35)
        xmm3 = _mm_unpackhi_epi32(xmm3, xmm7); // xmm3 = [col6 col7] = (06 16 26 36 07 17 27 37)
    }
    // .column_end:

    // -- Prefetch the next coefficient block
    // (prefetchnta: not translated; see the module documentation)

    // ---- Pass 2: process rows, store into output array.

    let rdi = &mut output_buf; // (JSAMPROW *)
    let eax = output_col as usize;

    // -- Even part

    xmm4 = _mm_setzero_si128();
    xmm4 = _mm_unpacklo_epi16(xmm4, xmm1); // xmm4 = tmp0
    xmm4 = _mm_srai_epi32::<{ 16 - CONST_BITS - 1 }>(xmm4);
    // psrad xmm4, 16 & pslld xmm4, CONST_BITS + 1

    // -- Odd part

    xmm1 = _mm_unpackhi_epi16(xmm1, xmm0);
    xmm6 = _mm_unpackhi_epi16(xmm6, xmm3);
    xmm5 = xmm1;
    xmm2 = xmm6;
    xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F256_F089)); // xmm1 = (tmp2)
    xmm6 = _mm_madd_epi16(xmm6, _mm_loadu_si128(&jconst_idct_red_sse2.PW_MF060_MF050)); // xmm6 = (tmp2)
    xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F106_MF217)); // xmm5 = (tmp0)
    xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F145_MF021)); // xmm2 = (tmp0)

    xmm6 = _mm_add_epi32(xmm6, xmm1); // xmm6 = tmp2
    xmm2 = _mm_add_epi32(xmm2, xmm5); // xmm2 = tmp0

    // -- Even part

    xmm0 = _mm_unpacklo_epi16(xmm0, xmm3);
    xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F184_MF076)); // xmm0 = tmp2

    xmm7 = xmm4;
    xmm4 = _mm_add_epi32(xmm4, xmm0); // xmm4 = tmp10
    xmm7 = _mm_sub_epi32(xmm7, xmm0); // xmm7 = tmp12

    // -- Final output stage

    xmm1 = _mm_loadu_si128(&jconst_idct_red_sse2.PD_DESCALE_P2_4); // xmm1 = [rel PD_DESCALE_P2_4]

    xmm5 = xmm4;
    xmm3 = xmm7;
    xmm4 = _mm_add_epi32(xmm4, xmm6); // xmm4 = data0 = (00 10 20 30)
    xmm7 = _mm_add_epi32(xmm7, xmm2); // xmm7 = data1 = (01 11 21 31)
    xmm5 = _mm_sub_epi32(xmm5, xmm6); // xmm5 = data3 = (03 13 23 33)
    xmm3 = _mm_sub_epi32(xmm3, xmm2); // xmm3 = data2 = (02 12 22 32)

    xmm4 = _mm_add_epi32(xmm4, xmm1);
    xmm7 = _mm_add_epi32(xmm7, xmm1);
    xmm4 = _mm_srai_epi32::<DESCALE_P2_4>(xmm4);
    xmm7 = _mm_srai_epi32::<DESCALE_P2_4>(xmm7);
    xmm5 = _mm_add_epi32(xmm5, xmm1);
    xmm3 = _mm_add_epi32(xmm3, xmm1);
    xmm5 = _mm_srai_epi32::<DESCALE_P2_4>(xmm5);
    xmm3 = _mm_srai_epi32::<DESCALE_P2_4>(xmm3);

    xmm4 = _mm_packs_epi32(xmm4, xmm3); // xmm4 = (00 10 20 30 02 12 22 32)
    xmm7 = _mm_packs_epi32(xmm7, xmm5); // xmm7 = (01 11 21 31 03 13 23 33)

    xmm0 = xmm4; // transpose coefficients(phase 1)
    xmm4 = _mm_unpacklo_epi16(xmm4, xmm7); // xmm4 = (00 01 10 11 20 21 30 31)
    xmm0 = _mm_unpackhi_epi16(xmm0, xmm7); // xmm0 = (02 03 12 13 22 23 32 33)

    xmm6 = xmm4; // transpose coefficients(phase 2)
    xmm4 = _mm_unpacklo_epi32(xmm4, xmm0); // xmm4 = (00 01 02 03 10 11 12 13)
    xmm6 = _mm_unpackhi_epi32(xmm6, xmm0); // xmm6 = (20 21 22 23 30 31 32 33)

    xmm4 = _mm_packs_epi16(xmm4, xmm6); // xmm4 = (00 01 02 03 10 11 12 13 20 ..)
    xmm4 = _mm_add_epi8(xmm4, _mm_loadu_si128(&jconst_idct_red_sse2.PB_CENTERJSAMP));

    xmm2 = _mm_shuffle_epi32::<0x39>(xmm4); // xmm2 = (10 11 12 13 20 21 22 23 30 ..)
    xmm1 = _mm_shuffle_epi32::<0x4E>(xmm4); // xmm1 = (20 21 22 23 30 31 32 33 00 ..)
    xmm3 = _mm_shuffle_epi32::<0x93>(xmm4); // xmm3 = (30 31 32 33 00 01 02 03 10 ..)

    let rdx = rdi.row_mut(0);
    *rdx[eax..].first_chunk_mut::<4>().expect(OUTPUT) = _mm_cvtsi128_si32(xmm4).to_le_bytes();
    let rsi = rdi.row_mut(1);
    *rsi[eax..].first_chunk_mut::<4>().expect(OUTPUT) = _mm_cvtsi128_si32(xmm2).to_le_bytes();
    let rdx = rdi.row_mut(2);
    *rdx[eax..].first_chunk_mut::<4>().expect(OUTPUT) = _mm_cvtsi128_si32(xmm1).to_le_bytes();
    let rsi = rdi.row_mut(3);
    *rsi[eax..].first_chunk_mut::<4>().expect(OUTPUT) = _mm_cvtsi128_si32(xmm3).to_le_bytes();
}

// upstream: simd/x86_64/jidctred-sse2.asm jsimd_idct_2x2_sse2
/// Perform dequantization and inverse DCT on one block of coefficients,
/// producing a reduced-size 2x2 output block.
///
/// # Panics
///
/// When `output_buf` has fewer than 2 rows or 2 columns from `output_col`
/// on.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[expect(
    clippy::used_underscore_items,
    reason = "the x86 loads and stores have the intrinsics' names"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_idct_2x2_sse2(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut xmm2: __m128i;
    let mut xmm3: __m128i;
    let mut xmm4: __m128i;
    let mut xmm5: __m128i;
    let mut xmm6: __m128i;
    let mut xmm7: __m128i;

    // ---- Pass 1: process columns from input.

    let rdx = dct_table.islow_array().as_chunks::<DCTSIZE>().0; // quantptr
    let rsi = coef_block.as_chunks::<DCTSIZE>().0; // inptr

    // | input:                  | result:        |
    // | 00 01 ** 03 ** 05 ** 07 |                |
    // | 10 11 ** 13 ** 15 ** 17 |                |
    // | ** ** ** ** ** ** ** ** |                |
    // | 30 31 ** 33 ** 35 ** 37 | A0 A1 A3 A5 A7 |
    // | ** ** ** ** ** ** ** ** | B0 B1 B3 B5 B7 |
    // | 50 51 ** 53 ** 55 ** 57 |                |
    // | ** ** ** ** ** ** ** ** |                |
    // | 70 71 ** 73 ** 75 ** 77 |                |

    // -- Odd part

    xmm0 = _mm_loadu_si128(&rsi[1]);
    xmm1 = _mm_loadu_si128(&rsi[3]);
    xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&rdx[1]));
    xmm1 = _mm_mullo_epi16(xmm1, _mm_loadu_si128(&rdx[3]));
    xmm2 = _mm_loadu_si128(&rsi[5]);
    xmm3 = _mm_loadu_si128(&rsi[7]);
    xmm2 = _mm_mullo_epi16(xmm2, _mm_loadu_si128(&rdx[5]));
    xmm3 = _mm_mullo_epi16(xmm3, _mm_loadu_si128(&rdx[7]));

    // xmm0 = (10 11 ** 13 ** 15 ** 17)
    // xmm1 = (30 31 ** 33 ** 35 ** 37)
    // xmm2 = (50 51 ** 53 ** 55 ** 57)
    // xmm3 = (70 71 ** 73 ** 75 ** 77)

    xmm7 = _mm_set1_epi32(-1);
    xmm7 = _mm_slli_epi32::<WORD_BIT>(xmm7); // xmm7 = { 0x0000 0xFFFF 0x0000 0xFFFF .. }

    xmm4 = xmm0; // xmm4 = (10 11 ** 13 ** 15 ** 17)
    xmm5 = xmm2; // xmm5 = (50 51 ** 53 ** 55 ** 57)
    xmm4 = _mm_unpacklo_epi16(xmm4, xmm1); // xmm4 = (10 30 11 31 ** ** 13 33)
    xmm5 = _mm_unpacklo_epi16(xmm5, xmm3); // xmm5 = (50 70 51 71 ** ** 53 73)
    xmm4 = _mm_madd_epi16(xmm4, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F362_MF127));
    xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F085_MF072));

    xmm0 = _mm_srli_epi32::<WORD_BIT>(xmm0); // xmm0 = (11 -- 13 -- 15 -- 17 --)
    xmm1 = _mm_and_si128(xmm1, xmm7); // xmm1 = (-- 31 -- 33 -- 35 -- 37)
    xmm2 = _mm_srli_epi32::<WORD_BIT>(xmm2); // xmm2 = (51 -- 53 -- 55 -- 57 --)
    xmm3 = _mm_and_si128(xmm3, xmm7); // xmm3 = (-- 71 -- 73 -- 75 -- 77)
    xmm0 = _mm_or_si128(xmm0, xmm1); // xmm0 = (11 31 13 33 15 35 17 37)
    xmm2 = _mm_or_si128(xmm2, xmm3); // xmm2 = (51 71 53 73 55 75 57 77)
    xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F362_MF127));
    xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F085_MF072));

    xmm4 = _mm_add_epi32(xmm4, xmm5); // xmm4 = tmp0[col0 col1 **** col3]
    xmm0 = _mm_add_epi32(xmm0, xmm2); // xmm0 = tmp0[col1 col3 col5 col7]

    // -- Even part

    xmm6 = _mm_loadu_si128(&rsi[0]);
    xmm6 = _mm_mullo_epi16(xmm6, _mm_loadu_si128(&rdx[0]));

    // xmm6 = (00 01 ** 03 ** 05 ** 07)

    xmm1 = xmm6; // xmm1 = (00 01 ** 03 ** 05 ** 07)
    xmm6 = _mm_slli_epi32::<WORD_BIT>(xmm6); // xmm6 = (-- 00 -- ** -- ** -- **)
    xmm1 = _mm_and_si128(xmm1, xmm7); // xmm1 = (-- 01 -- 03 -- 05 -- 07)
    xmm6 = _mm_srai_epi32::<{ WORD_BIT - CONST_BITS - 2 }>(xmm6);
    // xmm6 = tmp10[col0 **** **** ****]
    xmm1 = _mm_srai_epi32::<{ WORD_BIT - CONST_BITS - 2 }>(xmm1);
    // xmm1 = tmp10[col1 col3 col5 col7]

    // -- Final output stage

    xmm3 = xmm6;
    xmm5 = xmm1;
    xmm6 = _mm_add_epi32(xmm6, xmm4); // xmm6 = data0[col0 **** **** ****] = (A0 ** ** **)
    xmm1 = _mm_add_epi32(xmm1, xmm0); // xmm1 = data0[col1 col3 col5 col7] = (A1 A3 A5 A7)
    xmm3 = _mm_sub_epi32(xmm3, xmm4); // xmm3 = data1[col0 **** **** ****] = (B0 ** ** **)
    xmm5 = _mm_sub_epi32(xmm5, xmm0); // xmm5 = data1[col1 col3 col5 col7] = (B1 B3 B5 B7)

    xmm2 = _mm_loadu_si128(&jconst_idct_red_sse2.PD_DESCALE_P1_2); // xmm2 = [rel PD_DESCALE_P1_2]

    xmm6 = _mm_unpacklo_epi32(xmm6, xmm3); // xmm6 = (A0 B0 ** **)

    xmm7 = xmm1;
    xmm1 = _mm_unpacklo_epi64(xmm1, xmm5); // xmm1 = (A1 A3 B1 B3)
    xmm7 = _mm_unpackhi_epi64(xmm7, xmm5); // xmm7 = (A5 A7 B5 B7)

    xmm6 = _mm_add_epi32(xmm6, xmm2);
    xmm6 = _mm_srai_epi32::<DESCALE_P1_2>(xmm6);

    xmm1 = _mm_add_epi32(xmm1, xmm2);
    xmm7 = _mm_add_epi32(xmm7, xmm2);
    xmm1 = _mm_srai_epi32::<DESCALE_P1_2>(xmm1);
    xmm7 = _mm_srai_epi32::<DESCALE_P1_2>(xmm7);

    // -- Prefetch the next coefficient block
    // (prefetchnta: not translated; see the module documentation)

    // ---- Pass 2: process rows, store into output array.

    let rdi = &mut output_buf; // (JSAMPROW *)
    let eax = output_col as usize;

    // | input:| result:|
    // | A0 B0 |        |
    // | A1 B1 | C0 C1  |
    // | A3 B3 | D0 D1  |
    // | A5 B5 |        |
    // | A7 B7 |        |

    // -- Odd part

    xmm1 = _mm_packs_epi32(xmm1, xmm1); // xmm1 = (A1 A3 B1 B3 A1 A3 B1 B3)
    xmm7 = _mm_packs_epi32(xmm7, xmm7); // xmm7 = (A5 A7 B5 B7 A5 A7 B5 B7)
    xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F362_MF127));
    xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&jconst_idct_red_sse2.PW_F085_MF072));

    xmm1 = _mm_add_epi32(xmm1, xmm7); // xmm1 = tmp0[row0 row1 row0 row1]

    // -- Even part

    xmm6 = _mm_slli_epi32::<{ CONST_BITS + 2 }>(xmm6); // xmm6 = tmp10[row0 row1 **** ****]

    // -- Final output stage

    xmm4 = xmm6;
    xmm6 = _mm_add_epi32(xmm6, xmm1); // xmm6 = data0[row0 row1 **** ****] = (C0 C1 ** **)
    xmm4 = _mm_sub_epi32(xmm4, xmm1); // xmm4 = data1[row0 row1 **** ****] = (D0 D1 ** **)

    xmm6 = _mm_unpacklo_epi32(xmm6, xmm4); // xmm6 = (C0 D0 C1 D1)

    xmm6 = _mm_add_epi32(xmm6, _mm_loadu_si128(&jconst_idct_red_sse2.PD_DESCALE_P2_2));
    xmm6 = _mm_srai_epi32::<DESCALE_P2_2>(xmm6);

    xmm6 = _mm_packs_epi32(xmm6, xmm6); // xmm6 = (C0 D0 C1 D1 C0 D0 C1 D1)
    xmm6 = _mm_packs_epi16(xmm6, xmm6); // xmm6 = (C0 D0 C1 D1 C0 D0 C1 D1 ..)
    xmm6 = _mm_add_epi8(xmm6, _mm_loadu_si128(&jconst_idct_red_sse2.PB_CENTERJSAMP));

    let ebx = _mm_extract_epi16::<0x00>(xmm6); // ebx = (C0 D0 -- --)
    let ecx = _mm_extract_epi16::<0x01>(xmm6); // ecx = (C1 D1 -- --)

    let rdx = rdi.row_mut(0);
    rdx[eax..]
        .first_chunk_mut::<2>()
        .expect(OUTPUT)
        .copy_from_slice(&ebx.to_le_bytes()[..2]);
    let rsi = rdi.row_mut(1);
    rsi[eax..]
        .first_chunk_mut::<2>()
        .expect(OUTPUT)
        .copy_from_slice(&ecx.to_le_bytes()[..2]);
}
