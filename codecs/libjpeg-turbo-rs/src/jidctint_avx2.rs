// libjpeg-turbo 3.2.1, simd/x86_64/jidctint-avx2.asm:
//
// ;
// ; Accurate Integer Inverse DCT (64-bit AVX2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2018, 2020, 2024-2025, D. R. Commander.
// ; Copyright (C) 2018, Matthias Räncker.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
// ;
// ; This file contains a slower but more accurate integer implementation of the
// ; inverse DCT (Discrete Cosine Transform).  The following code is based
// ; directly on the IJG's original jidctint.c; see jidctint.c for more details.
//! Translation of `simd/x86_64/jidctint-avx2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernel is AVX2 code; its 128-bit instructions (`vpor`, `vpmullw`
//! `xmm`, `vpshufd` `xmm`) are the VEX forms of the SSE2 intrinsics, which
//! the AVX2 target feature selects. The `%if CONST_BITS == 13` branch of
//! the constants is taken, and `NO_ZERO_COLUMN_TEST_ISLOW_AVX2` is undefined
//! in the oracle's build, so the zero-column test of pass 1 is translated.
//! The calling convention has no `%ifdef WIN64` branch in this file;
//! `COLLECT_ARGS`, `PUSH_XMM` and the pushes and pops become the function's
//! parameters.
//!
//! `jconst_idct_islow_avx2` is a `static` of [`JconstIdctIslowAvx2`], whose
//! fields are the asm's labels; an operand `[rel LABEL]` reads `rel.LABEL`,
//! where `rel` is a reference to that table. `r10` is the multiplier table's
//! `islow_array` (16-bit `ISLOW_MULT_TYPE` with `WITH_SIMD`) and `r11` the
//! block, and an operand `XMMWORD [XMMBLOCK(m, 0, base, 2)]` or
//! `YMMWORD [YMMBLOCK(m, 0, base, 2)]` of `jdct.inc` is the 8 or 16
//! elements of that array from row `m` on. The scalar
//! `dword [DWBLOCK(m, 0, r11, 2)]` is the first two coefficients of row `m`
//! as one little-endian `u32`. An `xmm` register and the `ymm` register it
//! is the low half of are separate variables; `vinserti128 ymm4, ymm4, xmm5,
//! 1` widens `xmm4` with `_mm256_castsi128_si256`, which emits no
//! instruction (the VEX-encoded `vpunpcklwd` has zeroed the upper half).
//!
//! The macros `DOTRANSPOSE` and `DODCT` are functions of those names. Their
//! input/output registers `%1`-`%4` are `&mut` parameters `p1`-`p4`, and
//! their temp registers (`%5`-`%8` and `%5`-`%12`), whose values no caller
//! reads afterwards, are locals `p5`-`p12`. `DODCT`'s last argument `%13`,
//! the pass, only pastes the names `DESCALE_P %+ %13` and
//! `PD_DESCALE_P %+ %13`; the function takes the shift as the const
//! parameter `DESCALE_P` and the constant as `pd_descale_p`.
//!
//! The labels `.columnDCT` and `.column_end` become labeled blocks: a jump
//! to a label leaves the block that ends there. The `prefetchnta` hints for
//! the next coefficient block are left out: they change no value, and their
//! address lies past the block that the kernel borrows. So is `vzeroupper`,
//! which the compiler emits where AVX code returns to SSE code. The stores write the
//! rows through one row slice at a time, in the asm's order; each `movq`
//! writes the 8 samples of a row from `output_col` on.

#![forbid(unsafe_code)]

#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "ADR 0008: the intrinsics, shadowed by the x86 loads and stores"
    )
)]
use core::arch::x86_64::*;

use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, JCOEF, JDIMENSION};
use crate::jpeglib::{DCTSIZE, JBLOCK, JSAMPROWS_MUT};
use crate::x86::{_mm_loadu_si128, _mm_storel_epi64, _mm256_loadu_si256};

// upstream: simd/x86_64/jidctint-avx2.asm CONST_BITS
const CONST_BITS: i32 = 13;
// upstream: simd/x86_64/jidctint-avx2.asm PASS1_BITS
const PASS1_BITS: i32 = 2;

// upstream: simd/x86_64/jidctint-avx2.asm DESCALE_P1
const DESCALE_P1: i32 = CONST_BITS - PASS1_BITS;
// upstream: simd/x86_64/jidctint-avx2.asm DESCALE_P2
const DESCALE_P2: i32 = CONST_BITS + PASS1_BITS + 3;

// upstream: simd/x86_64/jidctint-avx2.asm F_0_298
const F_0_298: i16 = 2446; // FIX(0.298631336)
// upstream: simd/x86_64/jidctint-avx2.asm F_0_390
const F_0_390: i16 = 3196; // FIX(0.390180644)
// upstream: simd/x86_64/jidctint-avx2.asm F_0_541
const F_0_541: i16 = 4433; // FIX(0.541196100)
// upstream: simd/x86_64/jidctint-avx2.asm F_0_765
const F_0_765: i16 = 6270; // FIX(0.765366865)
// upstream: simd/x86_64/jidctint-avx2.asm F_0_899
const F_0_899: i16 = 7373; // FIX(0.899976223)
// upstream: simd/x86_64/jidctint-avx2.asm F_1_175
const F_1_175: i16 = 9633; // FIX(1.175875602)
// upstream: simd/x86_64/jidctint-avx2.asm F_1_501
const F_1_501: i16 = 12299; // FIX(1.501321110)
// upstream: simd/x86_64/jidctint-avx2.asm F_1_847
const F_1_847: i16 = 15137; // FIX(1.847759065)
// upstream: simd/x86_64/jidctint-avx2.asm F_1_961
const F_1_961: i16 = 16069; // FIX(1.961570560)
// upstream: simd/x86_64/jidctint-avx2.asm F_2_053
const F_2_053: i16 = 16819; // FIX(2.053119869)
// upstream: simd/x86_64/jidctint-avx2.asm F_2_562
const F_2_562: i16 = 20995; // FIX(2.562915447)
// upstream: simd/x86_64/jidctint-avx2.asm F_3_072
const F_3_072: i16 = 25172; // FIX(3.072711026)

/// `times 4 dw a, b` followed by `times 4 dw c, d`. Not an upstream item.
const fn times_4_dw(a: i16, b: i16, c: i16, d: i16) -> [i16; 16] {
    [a, b, a, b, a, b, a, b, c, d, c, d, c, d, c, d]
}

/// `CENTERJSAMPLE` as the byte that `db` stores. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    reason = "db stores CENTERJSAMPLE as a byte"
)]
const CENTERJSAMP: u8 = CENTERJSAMPLE as u8;

/// The layout of `jconst_idct_islow_avx2`: one field per label, aligned as
/// `ALIGNZ 32` aligns the table. Not an upstream item.
#[expect(non_snake_case, reason = "asm label names")]
#[repr(C, align(32))]
pub struct JconstIdctIslowAvx2 {
    PW_F130_F054_MF130_F054: [i16; 16],
    PW_MF078_F117_F078_F117: [i16; 16],
    PW_MF060_MF089_MF050_MF256: [i16; 16],
    PW_MF089_F060_MF256_F050: [i16; 16],
    PD_DESCALE_P1: [i32; 8],
    PD_DESCALE_P2: [i32; 8],
    PB_CENTERJSAMP: [u8; 32],
    PW_1_NEG1: [i16; 16],
}

// upstream: simd/x86_64/jidctint-avx2.asm jconst_idct_islow_avx2
/// The constants of `jsimd_idct_islow_avx2`.
#[expect(non_upper_case_globals, reason = "asm label name")]
pub static jconst_idct_islow_avx2: JconstIdctIslowAvx2 = JconstIdctIslowAvx2 {
    PW_F130_F054_MF130_F054: times_4_dw(F_0_541 + F_0_765, F_0_541, F_0_541 - F_1_847, F_0_541),
    PW_MF078_F117_F078_F117: times_4_dw(F_1_175 - F_1_961, F_1_175, F_1_175 - F_0_390, F_1_175),
    PW_MF060_MF089_MF050_MF256: times_4_dw(
        F_0_298 - F_0_899,
        -F_0_899,
        F_2_053 - F_2_562,
        -F_2_562,
    ),
    PW_MF089_F060_MF256_F050: times_4_dw(-F_0_899, F_1_501 - F_0_899, -F_2_562, F_3_072 - F_2_562),
    PD_DESCALE_P1: [1 << (DESCALE_P1 - 1); 8],
    PD_DESCALE_P2: [1 << (DESCALE_P2 - 1); 8],
    PB_CENTERJSAMP: [CENTERJSAMP; 32],
    PW_1_NEG1: [1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1],
};

/// The message of a block or table row that cannot fail: a block holds
/// `DCTSIZE` elements per row. Not an upstream item.
const ROW: &str = "a block holds 8 elements per row";
/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds 8 samples from output_col on";

/// The scalar operand `dword [DWBLOCK(m, 0, r11, SIZEOF_JCOEF)]`: two
/// coefficients as one little-endian doubleword. Not an upstream item.
fn dword(coefs: [JCOEF; 2]) -> u32 {
    let [b0, b1] = coefs[0].to_le_bytes();
    let [b2, b3] = coefs[1].to_le_bytes();
    u32::from_le_bytes([b0, b1, b2, b3])
}

// upstream: simd/x86_64/jidctint-avx2.asm DOTRANSPOSE
/// In-place 8x8x16-bit inverse matrix transpose using AVX2 instructions.
/// `p1`-`p4`: Input/output registers; `p5`-`p8`: Temp registers.
#[expect(non_snake_case, reason = "asm macro name")]
#[target_feature(enable = "avx2")]
#[inline]
fn DOTRANSPOSE(p1: &mut __m256i, p2: &mut __m256i, p3: &mut __m256i, p4: &mut __m256i) {
    // %5 = (00 10 20 30 40 50 60 70  01 11 21 31 41 51 61 71)
    // %6 = (03 13 23 33 43 53 63 73  02 12 22 32 42 52 62 72)
    // %7 = (04 14 24 34 44 54 64 74  05 15 25 35 45 55 65 75)
    // %8 = (07 17 27 37 47 57 67 77  06 16 26 36 46 56 66 76)

    // transpose coefficients(phase 1)
    let mut p5 = _mm256_permute4x64_epi64::<0xD8>(*p1);
    // %5 = (00 10 20 30 01 11 21 31  40 50 60 70 41 51 61 71)
    let mut p6 = _mm256_permute4x64_epi64::<0x72>(*p2);
    // %6 = (02 12 22 32 03 13 23 33  42 52 62 72 43 53 63 73)
    let mut p7 = _mm256_permute4x64_epi64::<0xD8>(*p3);
    // %7 = (04 14 24 34 05 15 25 35  44 54 64 74 45 55 65 75)
    let mut p8 = _mm256_permute4x64_epi64::<0x72>(*p4);
    // %8 = (06 16 26 36 07 17 27 37  46 56 66 76 47 57 67 77)

    // transpose coefficients(phase 2)
    *p1 = _mm256_unpacklo_epi16(p5, p6);
    // %1 = (00 02 10 12 20 22 30 32  40 42 50 52 60 62 70 72)
    *p2 = _mm256_unpackhi_epi16(p5, p6);
    // %2 = (01 03 11 13 21 23 31 33  41 43 51 53 61 63 71 73)
    *p3 = _mm256_unpacklo_epi16(p7, p8);
    // %3 = (04 06 14 16 24 26 34 36  44 46 54 56 64 66 74 76)
    *p4 = _mm256_unpackhi_epi16(p7, p8);
    // %4 = (05 07 15 17 25 27 35 37  45 47 55 57 65 67 75 77)

    // transpose coefficients(phase 3)
    p5 = _mm256_unpacklo_epi16(*p1, *p2);
    // %5 = (00 01 02 03 10 11 12 13  40 41 42 43 50 51 52 53)
    p6 = _mm256_unpacklo_epi16(*p3, *p4);
    // %6 = (04 05 06 07 14 15 16 17  44 45 46 47 54 55 56 57)
    p7 = _mm256_unpackhi_epi16(*p1, *p2);
    // %7 = (20 21 22 23 30 31 32 33  60 61 62 63 70 71 72 73)
    p8 = _mm256_unpackhi_epi16(*p3, *p4);
    // %8 = (24 25 26 27 34 35 36 37  64 65 66 67 74 75 76 77)

    // transpose coefficients(phase 4)
    *p1 = _mm256_unpacklo_epi64(p5, p6);
    // %1 = (00 01 02 03 04 05 06 07  40 41 42 43 44 45 46 47)
    *p2 = _mm256_unpackhi_epi64(p5, p6);
    // %2 = (10 11 12 13 14 15 16 17  50 51 52 53 54 55 56 57)
    *p3 = _mm256_unpacklo_epi64(p7, p8);
    // %3 = (20 21 22 23 24 25 26 27  60 61 62 63 64 65 66 67)
    *p4 = _mm256_unpackhi_epi64(p7, p8);
    // %4 = (30 31 32 33 34 35 36 37  70 71 72 73 74 75 76 77)
}

// upstream: simd/x86_64/jidctint-avx2.asm DODCT
/// In-place 8x8x16-bit accurate integer inverse DCT using AVX2 instructions.
/// `p1`-`p4`: Input/output registers; `p5`-`p12`: Temp registers;
/// `DESCALE_P` and `pd_descale_p`: `DESCALE_P1` and `PD_DESCALE_P1` in
/// pass 1, `DESCALE_P2` and `PD_DESCALE_P2` in pass 2.
#[expect(non_snake_case, reason = "asm macro name")]
#[expect(
    clippy::used_underscore_items,
    reason = "the x86 loads and stores have the names of their intrinsics"
)]
#[target_feature(enable = "avx2")]
#[inline]
fn DODCT<const DESCALE_P: i32>(
    p1: &mut __m256i,
    p2: &mut __m256i,
    p3: &mut __m256i,
    p4: &mut __m256i,
    pd_descale_p: &[i32; 8],
) {
    let rel = &jconst_idct_islow_avx2;

    // -- Even part

    // (Original)
    // z1 = (z2 + z3) * 0.541196100;
    // tmp2 = z1 + z3 * -1.847759065;
    // tmp3 = z1 + z2 * 0.765366865;
    //
    // (This implementation)
    // tmp2 = z2 * 0.541196100 + z3 * (0.541196100 - 1.847759065);
    // tmp3 = z2 * (0.541196100 + 0.765366865) + z3 * 0.541196100;

    let mut p6 = _mm256_permute2x128_si256::<0x01>(*p3, *p3); // %6 = in6_2
    let mut p5 = _mm256_unpacklo_epi16(*p3, p6); // %5 = in26_62L
    p6 = _mm256_unpackhi_epi16(*p3, p6); // %6 = in26_62H
    p5 = _mm256_madd_epi16(p5, _mm256_loadu_si256(&rel.PW_F130_F054_MF130_F054)); // %5 = tmp3_2L
    p6 = _mm256_madd_epi16(p6, _mm256_loadu_si256(&rel.PW_F130_F054_MF130_F054)); // %6 = tmp3_2H

    let mut p7 = _mm256_permute2x128_si256::<0x01>(*p1, *p1); // %7 = in4_0
    *p1 = _mm256_sign_epi16(*p1, _mm256_loadu_si256(&rel.PW_1_NEG1));
    p7 = _mm256_add_epi16(p7, *p1); // %7 = (in0 + in4)_(in0 - in4)

    *p1 = _mm256_setzero_si256();
    let mut p8 = _mm256_unpacklo_epi16(*p1, p7); // %8 = tmp0_1L
    *p1 = _mm256_unpackhi_epi16(*p1, p7); // %1 = tmp0_1H
    p8 = _mm256_srai_epi32::<{ 16 - CONST_BITS }>(p8);
    // vpsrad %8, 16 & vpslld %8, CONST_BITS
    *p1 = _mm256_srai_epi32::<{ 16 - CONST_BITS }>(*p1);
    // vpsrad %1, 16 & vpslld %1, CONST_BITS

    let p11 = _mm256_sub_epi32(p8, p5); // %11 = tmp0_1L - tmp3_2L = tmp13_12L
    let p9 = _mm256_add_epi32(p8, p5); // %9 = tmp0_1L + tmp3_2L = tmp10_11L
    let p12 = _mm256_sub_epi32(*p1, p6); // %12 = tmp0_1H - tmp3_2H = tmp13_12H
    let p10 = _mm256_add_epi32(*p1, p6); // %10 = tmp0_1H + tmp3_2H = tmp10_11H

    // -- Odd part

    *p1 = _mm256_add_epi16(*p4, *p2); // %1 = in7_5 + in3_1 = z3_4

    // (Original)
    // z5 = (z3 + z4) * 1.175875602;
    // z3 = z3 * -1.961570560;  z4 = z4 * -0.390180644;
    // z3 += z5;  z4 += z5;
    //
    // (This implementation)
    // z3 = z3 * (1.175875602 - 1.961570560) + z4 * 1.175875602;
    // z4 = z3 * 1.175875602 + z4 * (1.175875602 - 0.390180644);

    p8 = _mm256_permute2x128_si256::<0x01>(*p1, *p1); // %8 = z4_3
    p7 = _mm256_unpacklo_epi16(*p1, p8); // %7 = z34_43L
    p8 = _mm256_unpackhi_epi16(*p1, p8); // %8 = z34_43H
    p7 = _mm256_madd_epi16(p7, _mm256_loadu_si256(&rel.PW_MF078_F117_F078_F117)); // %7 = z3_4L
    p8 = _mm256_madd_epi16(p8, _mm256_loadu_si256(&rel.PW_MF078_F117_F078_F117)); // %8 = z3_4H

    // (Original)
    // z1 = tmp0 + tmp3;  z2 = tmp1 + tmp2;
    // tmp0 = tmp0 * 0.298631336;  tmp1 = tmp1 * 2.053119869;
    // tmp2 = tmp2 * 3.072711026;  tmp3 = tmp3 * 1.501321110;
    // z1 = z1 * -0.899976223;  z2 = z2 * -2.562915447;
    // tmp0 += z1 + z3;  tmp1 += z2 + z4;
    // tmp2 += z2 + z3;  tmp3 += z1 + z4;
    //
    // (This implementation)
    // tmp0 = tmp0 * (0.298631336 - 0.899976223) + tmp3 * -0.899976223;
    // tmp1 = tmp1 * (2.053119869 - 2.562915447) + tmp2 * -2.562915447;
    // tmp2 = tmp1 * -2.562915447 + tmp2 * (3.072711026 - 2.562915447);
    // tmp3 = tmp0 * -0.899976223 + tmp3 * (1.501321110 - 0.899976223);
    // tmp0 += z3;  tmp1 += z4;
    // tmp2 += z3;  tmp3 += z4;

    *p2 = _mm256_permute2x128_si256::<0x01>(*p2, *p2); // %2 = in1_3
    *p3 = _mm256_unpacklo_epi16(*p4, *p2); // %3 = in71_53L
    *p4 = _mm256_unpackhi_epi16(*p4, *p2); // %4 = in71_53H

    p5 = _mm256_madd_epi16(*p3, _mm256_loadu_si256(&rel.PW_MF060_MF089_MF050_MF256)); // %5 = tmp0_1L
    p6 = _mm256_madd_epi16(*p4, _mm256_loadu_si256(&rel.PW_MF060_MF089_MF050_MF256)); // %6 = tmp0_1H
    p5 = _mm256_add_epi32(p5, p7); // %5 = tmp0_1L + z3_4L = tmp0_1L
    p6 = _mm256_add_epi32(p6, p8); // %6 = tmp0_1H + z3_4H = tmp0_1H

    *p3 = _mm256_madd_epi16(*p3, _mm256_loadu_si256(&rel.PW_MF089_F060_MF256_F050)); // %3 = tmp3_2L
    *p4 = _mm256_madd_epi16(*p4, _mm256_loadu_si256(&rel.PW_MF089_F060_MF256_F050)); // %4 = tmp3_2H
    p7 = _mm256_permute2x128_si256::<0x01>(p7, p7); // %7 = z4_3L
    p8 = _mm256_permute2x128_si256::<0x01>(p8, p8); // %8 = z4_3H
    p7 = _mm256_add_epi32(*p3, p7); // %7 = tmp3_2L + z4_3L = tmp3_2L
    p8 = _mm256_add_epi32(*p4, p8); // %8 = tmp3_2H + z4_3H = tmp3_2H

    // -- Final output stage

    *p1 = _mm256_add_epi32(p9, p7); // %1 = tmp10_11L + tmp3_2L = data0_1L
    *p2 = _mm256_add_epi32(p10, p8); // %2 = tmp10_11H + tmp3_2H = data0_1H
    *p1 = _mm256_add_epi32(*p1, _mm256_loadu_si256(pd_descale_p));
    *p2 = _mm256_add_epi32(*p2, _mm256_loadu_si256(pd_descale_p));
    *p1 = _mm256_srai_epi32::<DESCALE_P>(*p1);
    *p2 = _mm256_srai_epi32::<DESCALE_P>(*p2);
    *p1 = _mm256_packs_epi32(*p1, *p2); // %1 = data0_1

    *p3 = _mm256_sub_epi32(p9, p7); // %3 = tmp10_11L - tmp3_2L = data7_6L
    *p4 = _mm256_sub_epi32(p10, p8); // %4 = tmp10_11H - tmp3_2H = data7_6H
    *p3 = _mm256_add_epi32(*p3, _mm256_loadu_si256(pd_descale_p));
    *p4 = _mm256_add_epi32(*p4, _mm256_loadu_si256(pd_descale_p));
    *p3 = _mm256_srai_epi32::<DESCALE_P>(*p3);
    *p4 = _mm256_srai_epi32::<DESCALE_P>(*p4);
    *p4 = _mm256_packs_epi32(*p3, *p4); // %4 = data7_6

    p7 = _mm256_add_epi32(p11, p5); // %7 = tmp13_12L + tmp0_1L = data3_2L
    p8 = _mm256_add_epi32(p12, p6); // %8 = tmp13_12H + tmp0_1H = data3_2H
    p7 = _mm256_add_epi32(p7, _mm256_loadu_si256(pd_descale_p));
    p8 = _mm256_add_epi32(p8, _mm256_loadu_si256(pd_descale_p));
    p7 = _mm256_srai_epi32::<DESCALE_P>(p7);
    p8 = _mm256_srai_epi32::<DESCALE_P>(p8);
    *p2 = _mm256_packs_epi32(p7, p8); // %2 = data3_2

    p7 = _mm256_sub_epi32(p11, p5); // %7 = tmp13_12L - tmp0_1L = data4_5L
    p8 = _mm256_sub_epi32(p12, p6); // %8 = tmp13_12H - tmp0_1H = data4_5H
    p7 = _mm256_add_epi32(p7, _mm256_loadu_si256(pd_descale_p));
    p8 = _mm256_add_epi32(p8, _mm256_loadu_si256(pd_descale_p));
    p7 = _mm256_srai_epi32::<DESCALE_P>(p7);
    p8 = _mm256_srai_epi32::<DESCALE_P>(p8);
    *p3 = _mm256_packs_epi32(p7, p8); // %3 = data4_5
}

// upstream: simd/x86_64/jidctint-avx2.asm jsimd_idct_islow_avx2
/// Perform dequantization and inverse DCT on one block of coefficients.
///
/// # Panics
///
/// When `output_buf` has fewer than 8 rows or 8 columns from `output_col`
/// on.
///
/// # Safety
///
/// AVX2 must be available; `jsimd` selects this kernel only when the CPU
/// has it.
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(
    clippy::used_underscore_items,
    reason = "the x86 loads and stores have the names of their intrinsics"
)]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "the asm writes XMMBLOCK(m, ...) row offsets with m = 0 and 1"
)]
#[target_feature(enable = "avx2")]
pub fn jsimd_idct_islow_avx2(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let rel = &jconst_idct_islow_avx2;
    let r10 = dct_table.islow_array(); // jpeg_component_info *compptr
    let r11 = coef_block; // JCOEFPTR coef_block
    let mut ymm0;
    let mut ymm1;
    let mut ymm2;
    let mut ymm3;

    // ---- Pass 1: process columns.

    // .column_end
    'column_end: {
        // %ifndef NO_ZERO_COLUMN_TEST_ISLOW_AVX2
        // .columnDCT
        'columnDCT: {
            let mut eax = dword(*r11[1 * DCTSIZE..].first_chunk().expect(ROW));
            eax |= dword(*r11[2 * DCTSIZE..].first_chunk().expect(ROW));
            if eax != 0 {
                break 'columnDCT;
            }

            let mut xmm0 = _mm_loadu_si128(r11[1 * DCTSIZE..].first_chunk::<8>().expect(ROW));
            let mut xmm1 = _mm_loadu_si128(r11[2 * DCTSIZE..].first_chunk::<8>().expect(ROW));
            xmm0 = _mm_or_si128(
                xmm0,
                _mm_loadu_si128(r11[3 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm1 = _mm_or_si128(
                xmm1,
                _mm_loadu_si128(r11[4 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm0 = _mm_or_si128(
                xmm0,
                _mm_loadu_si128(r11[5 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm1 = _mm_or_si128(
                xmm1,
                _mm_loadu_si128(r11[6 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm0 = _mm_or_si128(
                xmm0,
                _mm_loadu_si128(r11[7 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm1 = _mm_or_si128(xmm1, xmm0);
            xmm1 = _mm_packs_epi16(xmm1, xmm1);
            xmm1 = _mm_packs_epi16(xmm1, xmm1);
            let eax = _mm_cvtsi128_si32(xmm1);
            if eax != 0 {
                break 'columnDCT;
            }

            // -- AC terms all zero

            let mut xmm5 = _mm_loadu_si128(r11[0 * DCTSIZE..].first_chunk::<8>().expect(ROW));
            xmm5 = _mm_mullo_epi16(
                xmm5,
                _mm_loadu_si128(r10[0 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );

            xmm5 = _mm_slli_epi16::<PASS1_BITS>(xmm5);

            let xmm4 = _mm_unpacklo_epi16(xmm5, xmm5); // xmm4 = (00 00 01 01 02 02 03 03)
            xmm5 = _mm_unpackhi_epi16(xmm5, xmm5); // xmm5 = (04 04 05 05 06 06 07 07)
            let ymm4 = _mm256_inserti128_si256::<1>(_mm256_castsi128_si256(xmm4), xmm5);

            ymm0 = _mm256_shuffle_epi32::<0x00>(ymm4);
            // ymm0 = col0_4 = (00 00 00 00 00 00 00 00  04 04 04 04 04 04 04 04)
            ymm1 = _mm256_shuffle_epi32::<0x55>(ymm4);
            // ymm1 = col1_5 = (01 01 01 01 01 01 01 01  05 05 05 05 05 05 05 05)
            ymm2 = _mm256_shuffle_epi32::<0xAA>(ymm4);
            // ymm2 = col2_6 = (02 02 02 02 02 02 02 02  06 06 06 06 06 06 06 06)
            ymm3 = _mm256_shuffle_epi32::<0xFF>(ymm4);
            // ymm3 = col3_7 = (03 03 03 03 03 03 03 03  07 07 07 07 07 07 07 07)

            break 'column_end;
        }
        // %endif

        let mut ymm4 = _mm256_loadu_si256(r11[0 * DCTSIZE..].first_chunk::<16>().expect(ROW));
        // ymm4 = in0_1
        let mut ymm5 = _mm256_loadu_si256(r11[2 * DCTSIZE..].first_chunk::<16>().expect(ROW));
        // ymm5 = in2_3
        let mut ymm6 = _mm256_loadu_si256(r11[4 * DCTSIZE..].first_chunk::<16>().expect(ROW));
        // ymm6 = in4_5
        let mut ymm7 = _mm256_loadu_si256(r11[6 * DCTSIZE..].first_chunk::<16>().expect(ROW));
        // ymm7 = in6_7
        ymm4 = _mm256_mullo_epi16(
            ymm4,
            _mm256_loadu_si256(r10[0 * DCTSIZE..].first_chunk::<16>().expect(ROW)),
        );
        ymm5 = _mm256_mullo_epi16(
            ymm5,
            _mm256_loadu_si256(r10[2 * DCTSIZE..].first_chunk::<16>().expect(ROW)),
        );
        ymm6 = _mm256_mullo_epi16(
            ymm6,
            _mm256_loadu_si256(r10[4 * DCTSIZE..].first_chunk::<16>().expect(ROW)),
        );
        ymm7 = _mm256_mullo_epi16(
            ymm7,
            _mm256_loadu_si256(r10[6 * DCTSIZE..].first_chunk::<16>().expect(ROW)),
        );

        ymm0 = _mm256_permute2x128_si256::<0x20>(ymm4, ymm6); // ymm0 = in0_4
        ymm1 = _mm256_permute2x128_si256::<0x31>(ymm5, ymm4); // ymm1 = in3_1
        ymm2 = _mm256_permute2x128_si256::<0x20>(ymm5, ymm7); // ymm2 = in2_6
        ymm3 = _mm256_permute2x128_si256::<0x31>(ymm7, ymm6); // ymm3 = in7_5

        DODCT::<DESCALE_P1>(
            &mut ymm0,
            &mut ymm1,
            &mut ymm2,
            &mut ymm3,
            &rel.PD_DESCALE_P1,
        );
        // ymm0 = data0_1, ymm1 = data3_2, ymm2 = data4_5, ymm3 = data7_6

        DOTRANSPOSE(&mut ymm0, &mut ymm1, &mut ymm2, &mut ymm3);
        // ymm0 = data0_4, ymm1 = data1_5, ymm2 = data2_6, ymm3 = data3_7
    }

    // -- Prefetch the next coefficient block (left out; see the module
    // documentation)

    // ---- Pass 2: process rows.

    let mut ymm4 = _mm256_permute2x128_si256::<0x31>(ymm3, ymm1); // ymm3 = in7_5
    ymm1 = _mm256_permute2x128_si256::<0x20>(ymm3, ymm1); // ymm1 = in3_1

    DODCT::<DESCALE_P2>(
        &mut ymm0,
        &mut ymm1,
        &mut ymm2,
        &mut ymm4,
        &rel.PD_DESCALE_P2,
    );
    // ymm0 = data0_1, ymm1 = data3_2, ymm2 = data4_5, ymm4 = data7_6

    DOTRANSPOSE(&mut ymm0, &mut ymm1, &mut ymm2, &mut ymm4);
    // ymm0 = data0_4, ymm1 = data1_5, ymm2 = data2_6, ymm4 = data3_7

    ymm0 = _mm256_packs_epi16(ymm0, ymm1); // ymm0 = data01_45
    ymm1 = _mm256_packs_epi16(ymm2, ymm4); // ymm1 = data23_67
    ymm0 = _mm256_add_epi8(ymm0, _mm256_loadu_si256(&rel.PB_CENTERJSAMP));
    ymm1 = _mm256_add_epi8(ymm1, _mm256_loadu_si256(&rel.PB_CENTERJSAMP));

    let xmm6 = _mm256_extracti128_si256::<1>(ymm1); // xmm3 = data67
    let xmm4 = _mm256_extracti128_si256::<1>(ymm0); // xmm2 = data45
    let xmm2 = _mm256_extracti128_si256::<0>(ymm1); // xmm1 = data23
    let xmm0 = _mm256_extracti128_si256::<0>(ymm0); // xmm0 = data01

    let xmm1 = _mm_shuffle_epi32::<0x4E>(xmm0);
    // xmm1 = (10 11 12 13 14 15 16 17 00 01 02 03 04 05 06 07)
    let xmm3 = _mm_shuffle_epi32::<0x4E>(xmm2);
    // xmm3 = (30 31 32 33 34 35 36 37 20 21 22 23 24 25 26 27)
    let xmm5 = _mm_shuffle_epi32::<0x4E>(xmm4);
    // xmm5 = (50 51 52 53 54 55 56 57 40 41 42 43 44 45 46 47)
    let xmm7 = _mm_shuffle_epi32::<0x4E>(xmm6);
    // xmm7 = (70 71 72 73 74 75 76 77 60 61 62 63 64 65 66 67)

    // vzeroupper: left to the compiler (ADR 0008, decision 4)

    let eax = output_col as usize;
    let r12 = &mut output_buf;

    // rdx = row 0, rsi = row 1
    _mm_storel_epi64(
        r12.row_mut(0)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm0,
    );
    _mm_storel_epi64(
        r12.row_mut(1)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm1,
    );

    // rdx = row 2, rsi = row 3
    _mm_storel_epi64(
        r12.row_mut(2)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm2,
    );
    _mm_storel_epi64(
        r12.row_mut(3)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm3,
    );

    // rdx = row 4, rsi = row 5
    _mm_storel_epi64(
        r12.row_mut(4)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm4,
    );
    _mm_storel_epi64(
        r12.row_mut(5)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm5,
    );

    // rdx = row 6, rsi = row 7
    _mm_storel_epi64(
        r12.row_mut(6)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm6,
    );
    _mm_storel_epi64(
        r12.row_mut(7)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm7,
    );
}
