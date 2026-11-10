// libjpeg-turbo 3.2.1, simd/x86_64/jidctint-sse2.asm:
//
// ;
// ; Accurate Integer Inverse DCT (64-bit SSE2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2020, 2024-2025, D. R. Commander.
// ; Copyright (C) 2018, Matthias Räncker.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
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
//! Translation of `simd/x86_64/jidctint-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernel is SSE2 code. The `%if CONST_BITS == 13` branch of the
//! constants is taken, and `NO_ZERO_COLUMN_TEST_ISLOW_SSE2` is undefined in
//! the oracle's build, so the zero-column test of pass 1 is translated. The
//! calling convention has no `%ifdef WIN64` branch in this file;
//! `COLLECT_ARGS`, the pushes and pops and the stack alignment of `wk` become
//! the function's parameters and the local array `wk`.
//!
//! `jconst_idct_islow_sse2` is a `static` of [`JconstIdctIslowSse2`], whose
//! fields are the asm's labels; an operand `[rel LABEL]` reads `rel.LABEL`,
//! where `rel` is a reference to that table. `quantptr` (`rdx`) is the
//! multiplier table's `islow_array` (16-bit `ISLOW_MULT_TYPE` with
//! `WITH_SIMD`) and `inptr` (`rsi`) the block, and an operand
//! `XMMWORD [XMMBLOCK(m, 0, base, 2)]` of `jdct.inc` is the 8 elements of
//! row `m` of that array. The scalar `dword [DWBLOCK(m, 0, rsi, 2)]` is the
//! first two coefficients of row `m` as one little-endian `u32`.
//!
//! The labels `.columnDCT` and `.column_end` become labeled blocks: a jump
//! to a label leaves the block that ends there. The `prefetchnta` hints for
//! the next coefficient block are left out: they change no value, and their
//! address lies past the block that the kernel borrows. Pass 2 stores the
//! rows through one row slice at a time, in the asm's order of stores; each
//! `movq` writes the 8 samples of a row from `output_col` on.

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
use crate::x86::{_mm_loadu_si128, _mm_storel_epi64};

// upstream: simd/x86_64/jidctint-sse2.asm CONST_BITS
const CONST_BITS: i32 = 13;
// upstream: simd/x86_64/jidctint-sse2.asm PASS1_BITS
const PASS1_BITS: i32 = 2;

// upstream: simd/x86_64/jidctint-sse2.asm DESCALE_P1
const DESCALE_P1: i32 = CONST_BITS - PASS1_BITS;
// upstream: simd/x86_64/jidctint-sse2.asm DESCALE_P2
const DESCALE_P2: i32 = CONST_BITS + PASS1_BITS + 3;

// upstream: simd/x86_64/jidctint-sse2.asm F_0_298
const F_0_298: i16 = 2446; // FIX(0.298631336)
// upstream: simd/x86_64/jidctint-sse2.asm F_0_390
const F_0_390: i16 = 3196; // FIX(0.390180644)
// upstream: simd/x86_64/jidctint-sse2.asm F_0_541
const F_0_541: i16 = 4433; // FIX(0.541196100)
// upstream: simd/x86_64/jidctint-sse2.asm F_0_765
const F_0_765: i16 = 6270; // FIX(0.765366865)
// upstream: simd/x86_64/jidctint-sse2.asm F_0_899
const F_0_899: i16 = 7373; // FIX(0.899976223)
// upstream: simd/x86_64/jidctint-sse2.asm F_1_175
const F_1_175: i16 = 9633; // FIX(1.175875602)
// upstream: simd/x86_64/jidctint-sse2.asm F_1_501
const F_1_501: i16 = 12299; // FIX(1.501321110)
// upstream: simd/x86_64/jidctint-sse2.asm F_1_847
const F_1_847: i16 = 15137; // FIX(1.847759065)
// upstream: simd/x86_64/jidctint-sse2.asm F_1_961
const F_1_961: i16 = 16069; // FIX(1.961570560)
// upstream: simd/x86_64/jidctint-sse2.asm F_2_053
const F_2_053: i16 = 16819; // FIX(2.053119869)
// upstream: simd/x86_64/jidctint-sse2.asm F_2_562
const F_2_562: i16 = 20995; // FIX(2.562915447)
// upstream: simd/x86_64/jidctint-sse2.asm F_3_072
const F_3_072: i16 = 25172; // FIX(3.072711026)

/// `times 4 dw a, b`: the word pair `a, b` four times. Not an upstream item.
const fn times_4_dw(a: i16, b: i16) -> [i16; 8] {
    [a, b, a, b, a, b, a, b]
}

/// `CENTERJSAMPLE` as the byte that `db` stores. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    reason = "db stores CENTERJSAMPLE as a byte"
)]
const CENTERJSAMP: u8 = CENTERJSAMPLE as u8;

/// The layout of `jconst_idct_islow_sse2`: one field per label, aligned as
/// `ALIGNZ 32` aligns the table. Not an upstream item.
#[expect(non_snake_case, reason = "asm label names")]
#[repr(C, align(32))]
pub struct JconstIdctIslowSse2 {
    PW_F130_F054: [i16; 8],
    PW_F054_MF130: [i16; 8],
    PW_MF078_F117: [i16; 8],
    PW_F117_F078: [i16; 8],
    PW_MF060_MF089: [i16; 8],
    PW_MF089_F060: [i16; 8],
    PW_MF050_MF256: [i16; 8],
    PW_MF256_F050: [i16; 8],
    PD_DESCALE_P1: [i32; 4],
    PD_DESCALE_P2: [i32; 4],
    PB_CENTERJSAMP: [u8; 16],
}

// upstream: simd/x86_64/jidctint-sse2.asm jconst_idct_islow_sse2
/// The constants of `jsimd_idct_islow_sse2`.
#[expect(non_upper_case_globals, reason = "asm label name")]
pub static jconst_idct_islow_sse2: JconstIdctIslowSse2 = JconstIdctIslowSse2 {
    PW_F130_F054: times_4_dw(F_0_541 + F_0_765, F_0_541),
    PW_F054_MF130: times_4_dw(F_0_541, F_0_541 - F_1_847),
    PW_MF078_F117: times_4_dw(F_1_175 - F_1_961, F_1_175),
    PW_F117_F078: times_4_dw(F_1_175, F_1_175 - F_0_390),
    PW_MF060_MF089: times_4_dw(F_0_298 - F_0_899, -F_0_899),
    PW_MF089_F060: times_4_dw(-F_0_899, F_1_501 - F_0_899),
    PW_MF050_MF256: times_4_dw(F_2_053 - F_2_562, -F_2_562),
    PW_MF256_F050: times_4_dw(-F_2_562, F_3_072 - F_2_562),
    PD_DESCALE_P1: [1 << (DESCALE_P1 - 1); 4],
    PD_DESCALE_P2: [1 << (DESCALE_P2 - 1); 4],
    PB_CENTERJSAMP: [CENTERJSAMP; 16],
};

/// The message of a block or table row that cannot fail: a row holds
/// `DCTSIZE` elements. Not an upstream item.
const ROW: &str = "a block row holds 8 elements";
/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds 8 samples from output_col on";

/// The scalar operand `dword [DWBLOCK(m, 0, rsi, SIZEOF_JCOEF)]`: two
/// coefficients as one little-endian doubleword. Not an upstream item.
fn dword(coefs: [JCOEF; 2]) -> u32 {
    let [b0, b1] = coefs[0].to_le_bytes();
    let [b2, b3] = coefs[1].to_le_bytes();
    u32::from_le_bytes([b0, b1, b2, b3])
}

// upstream: simd/x86_64/jidctint-sse2.asm WK_NUM
const WK_NUM: usize = 12;

// upstream: simd/x86_64/jidctint-sse2.asm jsimd_idct_islow_sse2
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
    reason = "the x86 loads and stores have the names of their intrinsics"
)]
#[expect(
    clippy::identity_op,
    clippy::erasing_op,
    reason = "the asm writes XMMBLOCK(m, ...) row offsets with m = 0 and 1"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_idct_islow_sse2(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let rel = &jconst_idct_islow_sse2;
    let mut wk = [_mm_setzero_si128(); WK_NUM];
    let mut xmm0;
    let mut xmm1;
    let mut xmm2;
    let mut xmm3;
    let mut xmm4;
    let mut xmm5;
    let mut xmm6;
    let mut xmm7;

    // ---- Pass 1: process columns from input.

    let rdx = dct_table.islow_array(); // quantptr
    let rsi = coef_block; // inptr

    // .column_end
    'column_end: {
        // %ifndef NO_ZERO_COLUMN_TEST_ISLOW_SSE2
        // .columnDCT
        'columnDCT: {
            let mut eax = dword(*rsi[1 * DCTSIZE..].first_chunk().expect(ROW));
            eax |= dword(*rsi[2 * DCTSIZE..].first_chunk().expect(ROW));
            if eax != 0 {
                break 'columnDCT;
            }

            xmm0 = _mm_loadu_si128(rsi[1 * DCTSIZE..].first_chunk::<8>().expect(ROW));
            xmm1 = _mm_loadu_si128(rsi[2 * DCTSIZE..].first_chunk::<8>().expect(ROW));
            xmm0 = _mm_or_si128(
                xmm0,
                _mm_loadu_si128(rsi[3 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm1 = _mm_or_si128(
                xmm1,
                _mm_loadu_si128(rsi[4 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm0 = _mm_or_si128(
                xmm0,
                _mm_loadu_si128(rsi[5 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm1 = _mm_or_si128(
                xmm1,
                _mm_loadu_si128(rsi[6 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm0 = _mm_or_si128(
                xmm0,
                _mm_loadu_si128(rsi[7 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );
            xmm1 = _mm_or_si128(xmm1, xmm0);
            xmm1 = _mm_packs_epi16(xmm1, xmm1);
            xmm1 = _mm_packs_epi16(xmm1, xmm1);
            let eax = _mm_cvtsi128_si32(xmm1);
            if eax != 0 {
                break 'columnDCT;
            }

            // -- AC terms all zero

            xmm5 = _mm_loadu_si128(rsi[0 * DCTSIZE..].first_chunk::<8>().expect(ROW));
            xmm5 = _mm_mullo_epi16(
                xmm5,
                _mm_loadu_si128(rdx[0 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
            );

            xmm5 = _mm_slli_epi16::<PASS1_BITS>(xmm5);

            xmm4 = xmm5; // xmm5 = in0 = (00 01 02 03 04 05 06 07)
            xmm5 = _mm_unpacklo_epi16(xmm5, xmm5); // xmm5 = (00 00 01 01 02 02 03 03)
            xmm4 = _mm_unpackhi_epi16(xmm4, xmm4); // xmm4 = (04 04 05 05 06 06 07 07)

            xmm7 = _mm_shuffle_epi32::<0x00>(xmm5); // xmm7 = col0 = (00 00 00 00 00 00 00 00)
            xmm6 = _mm_shuffle_epi32::<0x55>(xmm5); // xmm6 = col1 = (01 01 01 01 01 01 01 01)
            xmm1 = _mm_shuffle_epi32::<0xAA>(xmm5); // xmm1 = col2 = (02 02 02 02 02 02 02 02)
            xmm5 = _mm_shuffle_epi32::<0xFF>(xmm5); // xmm5 = col3 = (03 03 03 03 03 03 03 03)
            xmm0 = _mm_shuffle_epi32::<0x00>(xmm4); // xmm0 = col4 = (04 04 04 04 04 04 04 04)
            xmm3 = _mm_shuffle_epi32::<0x55>(xmm4); // xmm3 = col5 = (05 05 05 05 05 05 05 05)
            xmm2 = _mm_shuffle_epi32::<0xAA>(xmm4); // xmm2 = col6 = (06 06 06 06 06 06 06 06)
            xmm4 = _mm_shuffle_epi32::<0xFF>(xmm4); // xmm4 = col7 = (07 07 07 07 07 07 07 07)

            wk[8] = xmm6; // wk(8) = col1
            wk[9] = xmm5; // wk(9) = col3
            wk[10] = xmm3; // wk(10) = col5
            wk[11] = xmm4; // wk(11) = col7
            break 'column_end;
        }
        // %endif

        // -- Even part

        xmm0 = _mm_loadu_si128(rsi[0 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm1 = _mm_loadu_si128(rsi[2 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm0 = _mm_mullo_epi16(
            xmm0,
            _mm_loadu_si128(rdx[0 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );
        xmm1 = _mm_mullo_epi16(
            xmm1,
            _mm_loadu_si128(rdx[2 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );
        xmm2 = _mm_loadu_si128(rsi[4 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm3 = _mm_loadu_si128(rsi[6 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm2 = _mm_mullo_epi16(
            xmm2,
            _mm_loadu_si128(rdx[4 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );
        xmm3 = _mm_mullo_epi16(
            xmm3,
            _mm_loadu_si128(rdx[6 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );

        // (Original)
        // z1 = (z2 + z3) * 0.541196100;
        // tmp2 = z1 + z3 * -1.847759065;
        // tmp3 = z1 + z2 * 0.765366865;
        //
        // (This implementation)
        // tmp2 = z2 * 0.541196100 + z3 * (0.541196100 - 1.847759065);
        // tmp3 = z2 * (0.541196100 + 0.765366865) + z3 * 0.541196100;

        xmm4 = xmm1; // xmm1 = in2 = z2
        xmm5 = xmm1;
        xmm4 = _mm_unpacklo_epi16(xmm4, xmm3); // xmm3 = in6 = z3
        xmm5 = _mm_unpackhi_epi16(xmm5, xmm3);
        xmm1 = xmm4;
        xmm3 = xmm5;
        xmm4 = _mm_madd_epi16(xmm4, _mm_loadu_si128(&rel.PW_F130_F054)); // xmm4 = tmp3L
        xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&rel.PW_F130_F054)); // xmm5 = tmp3H
        xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&rel.PW_F054_MF130)); // xmm1 = tmp2L
        xmm3 = _mm_madd_epi16(xmm3, _mm_loadu_si128(&rel.PW_F054_MF130)); // xmm3 = tmp2H

        xmm6 = xmm0;
        xmm0 = _mm_add_epi16(xmm0, xmm2); // xmm0 = in0 + in4
        xmm6 = _mm_sub_epi16(xmm6, xmm2); // xmm6 = in0 - in4

        xmm7 = _mm_setzero_si128();
        xmm2 = _mm_setzero_si128();
        xmm7 = _mm_unpacklo_epi16(xmm7, xmm0); // xmm7 = tmp0L
        xmm2 = _mm_unpackhi_epi16(xmm2, xmm0); // xmm2 = tmp0H
        xmm7 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm7);
        // psrad xmm7, 16 & pslld xmm7, CONST_BITS
        xmm2 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm2);
        // psrad xmm2, 16 & pslld xmm2, CONST_BITS

        xmm0 = xmm7;
        xmm7 = _mm_add_epi32(xmm7, xmm4); // xmm7 = tmp10L
        xmm0 = _mm_sub_epi32(xmm0, xmm4); // xmm0 = tmp13L
        xmm4 = xmm2;
        xmm2 = _mm_add_epi32(xmm2, xmm5); // xmm2 = tmp10H
        xmm4 = _mm_sub_epi32(xmm4, xmm5); // xmm4 = tmp13H

        wk[0] = xmm7; // wk(0) = tmp10L
        wk[1] = xmm2; // wk(1) = tmp10H
        wk[2] = xmm0; // wk(2) = tmp13L
        wk[3] = xmm4; // wk(3) = tmp13H

        xmm5 = _mm_setzero_si128();
        xmm7 = _mm_setzero_si128();
        xmm5 = _mm_unpacklo_epi16(xmm5, xmm6); // xmm5 = tmp1L
        xmm7 = _mm_unpackhi_epi16(xmm7, xmm6); // xmm7 = tmp1H
        xmm5 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm5);
        // psrad xmm5, 16 & pslld xmm5, CONST_BITS
        xmm7 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm7);
        // psrad xmm7, 16 & pslld xmm7, CONST_BITS

        xmm2 = xmm5;
        xmm5 = _mm_add_epi32(xmm5, xmm1); // xmm5 = tmp11L
        xmm2 = _mm_sub_epi32(xmm2, xmm1); // xmm2 = tmp12L
        xmm0 = xmm7;
        xmm7 = _mm_add_epi32(xmm7, xmm3); // xmm7 = tmp11H
        xmm0 = _mm_sub_epi32(xmm0, xmm3); // xmm0 = tmp12H

        wk[4] = xmm5; // wk(4) = tmp11L
        wk[5] = xmm7; // wk(5) = tmp11H
        wk[6] = xmm2; // wk(6) = tmp12L
        wk[7] = xmm0; // wk(7) = tmp12H

        // -- Odd part

        xmm4 = _mm_loadu_si128(rsi[1 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm6 = _mm_loadu_si128(rsi[3 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm4 = _mm_mullo_epi16(
            xmm4,
            _mm_loadu_si128(rdx[1 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );
        xmm6 = _mm_mullo_epi16(
            xmm6,
            _mm_loadu_si128(rdx[3 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );
        xmm1 = _mm_loadu_si128(rsi[5 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm3 = _mm_loadu_si128(rsi[7 * DCTSIZE..].first_chunk::<8>().expect(ROW));
        xmm1 = _mm_mullo_epi16(
            xmm1,
            _mm_loadu_si128(rdx[5 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );
        xmm3 = _mm_mullo_epi16(
            xmm3,
            _mm_loadu_si128(rdx[7 * DCTSIZE..].first_chunk::<8>().expect(ROW)),
        );

        xmm5 = xmm6;
        xmm7 = xmm4;
        xmm5 = _mm_add_epi16(xmm5, xmm3); // xmm5 = z3
        xmm7 = _mm_add_epi16(xmm7, xmm1); // xmm7 = z4

        // (Original)
        // z5 = (z3 + z4) * 1.175875602;
        // z3 = z3 * -1.961570560;  z4 = z4 * -0.390180644;
        // z3 += z5;  z4 += z5;
        //
        // (This implementation)
        // z3 = z3 * (1.175875602 - 1.961570560) + z4 * 1.175875602;
        // z4 = z3 * 1.175875602 + z4 * (1.175875602 - 0.390180644);

        xmm2 = xmm5;
        xmm0 = xmm5;
        xmm2 = _mm_unpacklo_epi16(xmm2, xmm7);
        xmm0 = _mm_unpackhi_epi16(xmm0, xmm7);
        xmm5 = xmm2;
        xmm7 = xmm0;
        xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&rel.PW_MF078_F117)); // xmm2 = z3L
        xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&rel.PW_MF078_F117)); // xmm0 = z3H
        xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&rel.PW_F117_F078)); // xmm5 = z4L
        xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&rel.PW_F117_F078)); // xmm7 = z4H

        wk[10] = xmm2; // wk(10) = z3L
        wk[11] = xmm0; // wk(11) = z3H

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

        xmm2 = xmm3;
        xmm0 = xmm3;
        xmm2 = _mm_unpacklo_epi16(xmm2, xmm4);
        xmm0 = _mm_unpackhi_epi16(xmm0, xmm4);
        xmm3 = xmm2;
        xmm4 = xmm0;
        xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&rel.PW_MF060_MF089)); // xmm2 = tmp0L
        xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&rel.PW_MF060_MF089)); // xmm0 = tmp0H
        xmm3 = _mm_madd_epi16(xmm3, _mm_loadu_si128(&rel.PW_MF089_F060)); // xmm3 = tmp3L
        xmm4 = _mm_madd_epi16(xmm4, _mm_loadu_si128(&rel.PW_MF089_F060)); // xmm4 = tmp3H

        xmm2 = _mm_add_epi32(xmm2, wk[10]); // xmm2 = tmp0L
        xmm0 = _mm_add_epi32(xmm0, wk[11]); // xmm0 = tmp0H
        xmm3 = _mm_add_epi32(xmm3, xmm5); // xmm3 = tmp3L
        xmm4 = _mm_add_epi32(xmm4, xmm7); // xmm4 = tmp3H

        wk[8] = xmm2; // wk(8) = tmp0L
        wk[9] = xmm0; // wk(9) = tmp0H

        xmm2 = xmm1;
        xmm0 = xmm1;
        xmm2 = _mm_unpacklo_epi16(xmm2, xmm6);
        xmm0 = _mm_unpackhi_epi16(xmm0, xmm6);
        xmm1 = xmm2;
        xmm6 = xmm0;
        xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&rel.PW_MF050_MF256)); // xmm2 = tmp1L
        xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&rel.PW_MF050_MF256)); // xmm0 = tmp1H
        xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&rel.PW_MF256_F050)); // xmm1 = tmp2L
        xmm6 = _mm_madd_epi16(xmm6, _mm_loadu_si128(&rel.PW_MF256_F050)); // xmm6 = tmp2H

        xmm2 = _mm_add_epi32(xmm2, xmm5); // xmm2 = tmp1L
        xmm0 = _mm_add_epi32(xmm0, xmm7); // xmm0 = tmp1H
        xmm1 = _mm_add_epi32(xmm1, wk[10]); // xmm1 = tmp2L
        xmm6 = _mm_add_epi32(xmm6, wk[11]); // xmm6 = tmp2H

        wk[10] = xmm2; // wk(10) = tmp1L
        wk[11] = xmm0; // wk(11) = tmp1H

        // -- Final output stage

        xmm5 = wk[0]; // xmm5 = tmp10L
        xmm7 = wk[1]; // xmm7 = tmp10H

        xmm2 = xmm5;
        xmm0 = xmm7;
        xmm5 = _mm_add_epi32(xmm5, xmm3); // xmm5 = data0L
        xmm7 = _mm_add_epi32(xmm7, xmm4); // xmm7 = data0H
        xmm2 = _mm_sub_epi32(xmm2, xmm3); // xmm2 = data7L
        xmm0 = _mm_sub_epi32(xmm0, xmm4); // xmm0 = data7H

        xmm3 = _mm_loadu_si128(&rel.PD_DESCALE_P1); // xmm3 = [rel PD_DESCALE_P1]

        xmm5 = _mm_add_epi32(xmm5, xmm3);
        xmm7 = _mm_add_epi32(xmm7, xmm3);
        xmm5 = _mm_srai_epi32::<DESCALE_P1>(xmm5);
        xmm7 = _mm_srai_epi32::<DESCALE_P1>(xmm7);
        xmm2 = _mm_add_epi32(xmm2, xmm3);
        xmm0 = _mm_add_epi32(xmm0, xmm3);
        xmm2 = _mm_srai_epi32::<DESCALE_P1>(xmm2);
        xmm0 = _mm_srai_epi32::<DESCALE_P1>(xmm0);

        xmm5 = _mm_packs_epi32(xmm5, xmm7); // xmm5 = data0 = (00 01 02 03 04 05 06 07)
        xmm2 = _mm_packs_epi32(xmm2, xmm0); // xmm2 = data7 = (70 71 72 73 74 75 76 77)

        xmm4 = wk[4]; // xmm4 = tmp11L
        xmm3 = wk[5]; // xmm3 = tmp11H

        xmm7 = xmm4;
        xmm0 = xmm3;
        xmm4 = _mm_add_epi32(xmm4, xmm1); // xmm4 = data1L
        xmm3 = _mm_add_epi32(xmm3, xmm6); // xmm3 = data1H
        xmm7 = _mm_sub_epi32(xmm7, xmm1); // xmm7 = data6L
        xmm0 = _mm_sub_epi32(xmm0, xmm6); // xmm0 = data6H

        xmm1 = _mm_loadu_si128(&rel.PD_DESCALE_P1); // xmm1 = [rel PD_DESCALE_P1]

        xmm4 = _mm_add_epi32(xmm4, xmm1);
        xmm3 = _mm_add_epi32(xmm3, xmm1);
        xmm4 = _mm_srai_epi32::<DESCALE_P1>(xmm4);
        xmm3 = _mm_srai_epi32::<DESCALE_P1>(xmm3);
        xmm7 = _mm_add_epi32(xmm7, xmm1);
        xmm0 = _mm_add_epi32(xmm0, xmm1);
        xmm7 = _mm_srai_epi32::<DESCALE_P1>(xmm7);
        xmm0 = _mm_srai_epi32::<DESCALE_P1>(xmm0);

        xmm4 = _mm_packs_epi32(xmm4, xmm3); // xmm4 = data1 = (10 11 12 13 14 15 16 17)
        xmm7 = _mm_packs_epi32(xmm7, xmm0); // xmm7 = data6 = (60 61 62 63 64 65 66 67)

        xmm6 = xmm5; // transpose coefficients(phase 1)
        xmm5 = _mm_unpacklo_epi16(xmm5, xmm4); // xmm5 = (00 10 01 11 02 12 03 13)
        xmm6 = _mm_unpackhi_epi16(xmm6, xmm4); // xmm6 = (04 14 05 15 06 16 07 17)
        xmm1 = xmm7; // transpose coefficients(phase 1)
        xmm7 = _mm_unpacklo_epi16(xmm7, xmm2); // xmm7 = (60 70 61 71 62 72 63 73)
        xmm1 = _mm_unpackhi_epi16(xmm1, xmm2); // xmm1 = (64 74 65 75 66 76 67 77)

        xmm3 = wk[6]; // xmm3 = tmp12L
        xmm0 = wk[7]; // xmm0 = tmp12H
        xmm4 = wk[10]; // xmm4 = tmp1L
        xmm2 = wk[11]; // xmm2 = tmp1H

        wk[0] = xmm5; // wk(0) = (00 10 01 11 02 12 03 13)
        wk[1] = xmm6; // wk(1) = (04 14 05 15 06 16 07 17)
        wk[4] = xmm7; // wk(4) = (60 70 61 71 62 72 63 73)
        wk[5] = xmm1; // wk(5) = (64 74 65 75 66 76 67 77)

        xmm5 = xmm3;
        xmm6 = xmm0;
        xmm3 = _mm_add_epi32(xmm3, xmm4); // xmm3 = data2L
        xmm0 = _mm_add_epi32(xmm0, xmm2); // xmm0 = data2H
        xmm5 = _mm_sub_epi32(xmm5, xmm4); // xmm5 = data5L
        xmm6 = _mm_sub_epi32(xmm6, xmm2); // xmm6 = data5H

        xmm7 = _mm_loadu_si128(&rel.PD_DESCALE_P1); // xmm7 = [rel PD_DESCALE_P1]

        xmm3 = _mm_add_epi32(xmm3, xmm7);
        xmm0 = _mm_add_epi32(xmm0, xmm7);
        xmm3 = _mm_srai_epi32::<DESCALE_P1>(xmm3);
        xmm0 = _mm_srai_epi32::<DESCALE_P1>(xmm0);
        xmm5 = _mm_add_epi32(xmm5, xmm7);
        xmm6 = _mm_add_epi32(xmm6, xmm7);
        xmm5 = _mm_srai_epi32::<DESCALE_P1>(xmm5);
        xmm6 = _mm_srai_epi32::<DESCALE_P1>(xmm6);

        xmm3 = _mm_packs_epi32(xmm3, xmm0); // xmm3 = data2 = (20 21 22 23 24 25 26 27)
        xmm5 = _mm_packs_epi32(xmm5, xmm6); // xmm5 = data5 = (50 51 52 53 54 55 56 57)

        xmm1 = wk[2]; // xmm1 = tmp13L
        xmm4 = wk[3]; // xmm4 = tmp13H
        xmm2 = wk[8]; // xmm2 = tmp0L
        xmm7 = wk[9]; // xmm7 = tmp0H

        xmm0 = xmm1;
        xmm6 = xmm4;
        xmm1 = _mm_add_epi32(xmm1, xmm2); // xmm1 = data3L
        xmm4 = _mm_add_epi32(xmm4, xmm7); // xmm4 = data3H
        xmm0 = _mm_sub_epi32(xmm0, xmm2); // xmm0 = data4L
        xmm6 = _mm_sub_epi32(xmm6, xmm7); // xmm6 = data4H

        xmm2 = _mm_loadu_si128(&rel.PD_DESCALE_P1); // xmm2 = [rel PD_DESCALE_P1]

        xmm1 = _mm_add_epi32(xmm1, xmm2);
        xmm4 = _mm_add_epi32(xmm4, xmm2);
        xmm1 = _mm_srai_epi32::<DESCALE_P1>(xmm1);
        xmm4 = _mm_srai_epi32::<DESCALE_P1>(xmm4);
        xmm0 = _mm_add_epi32(xmm0, xmm2);
        xmm6 = _mm_add_epi32(xmm6, xmm2);
        xmm0 = _mm_srai_epi32::<DESCALE_P1>(xmm0);
        xmm6 = _mm_srai_epi32::<DESCALE_P1>(xmm6);

        xmm1 = _mm_packs_epi32(xmm1, xmm4); // xmm1 = data3 = (30 31 32 33 34 35 36 37)
        xmm0 = _mm_packs_epi32(xmm0, xmm6); // xmm0 = data4 = (40 41 42 43 44 45 46 47)

        xmm7 = wk[0]; // xmm7 = (00 10 01 11 02 12 03 13)
        xmm2 = wk[1]; // xmm2 = (04 14 05 15 06 16 07 17)

        xmm4 = xmm3; // transpose coefficients(phase 1)
        xmm3 = _mm_unpacklo_epi16(xmm3, xmm1); // xmm3 = (20 30 21 31 22 32 23 33)
        xmm4 = _mm_unpackhi_epi16(xmm4, xmm1); // xmm4 = (24 34 25 35 26 36 27 37)
        xmm6 = xmm0; // transpose coefficients(phase 1)
        xmm0 = _mm_unpacklo_epi16(xmm0, xmm5); // xmm0 = (40 50 41 51 42 52 43 53)
        xmm6 = _mm_unpackhi_epi16(xmm6, xmm5); // xmm6 = (44 54 45 55 46 56 47 57)

        xmm1 = xmm7; // transpose coefficients(phase 2)
        xmm7 = _mm_unpacklo_epi32(xmm7, xmm3); // xmm7 = (00 10 20 30 01 11 21 31)
        xmm1 = _mm_unpackhi_epi32(xmm1, xmm3); // xmm1 = (02 12 22 32 03 13 23 33)
        xmm5 = xmm2; // transpose coefficients(phase 2)
        xmm2 = _mm_unpacklo_epi32(xmm2, xmm4); // xmm2 = (04 14 24 34 05 15 25 35)
        xmm5 = _mm_unpackhi_epi32(xmm5, xmm4); // xmm5 = (06 16 26 36 07 17 27 37)

        xmm3 = wk[4]; // xmm3 = (60 70 61 71 62 72 63 73)
        xmm4 = wk[5]; // xmm4 = (64 74 65 75 66 76 67 77)

        wk[6] = xmm2; // wk(6) = (04 14 24 34 05 15 25 35)
        wk[7] = xmm5; // wk(7) = (06 16 26 36 07 17 27 37)

        xmm2 = xmm0; // transpose coefficients(phase 2)
        xmm0 = _mm_unpacklo_epi32(xmm0, xmm3); // xmm0 = (40 50 60 70 41 51 61 71)
        xmm2 = _mm_unpackhi_epi32(xmm2, xmm3); // xmm2 = (42 52 62 72 43 53 63 73)
        xmm5 = xmm6; // transpose coefficients(phase 2)
        xmm6 = _mm_unpacklo_epi32(xmm6, xmm4); // xmm6 = (44 54 64 74 45 55 65 75)
        xmm5 = _mm_unpackhi_epi32(xmm5, xmm4); // xmm5 = (46 56 66 76 47 57 67 77)

        xmm3 = xmm7; // transpose coefficients(phase 3)
        xmm7 = _mm_unpacklo_epi64(xmm7, xmm0); // xmm7 = col0 = (00 10 20 30 40 50 60 70)
        xmm3 = _mm_unpackhi_epi64(xmm3, xmm0); // xmm3 = col1 = (01 11 21 31 41 51 61 71)
        xmm4 = xmm1; // transpose coefficients(phase 3)
        xmm1 = _mm_unpacklo_epi64(xmm1, xmm2); // xmm1 = col2 = (02 12 22 32 42 52 62 72)
        xmm4 = _mm_unpackhi_epi64(xmm4, xmm2); // xmm4 = col3 = (03 13 23 33 43 53 63 73)

        xmm0 = wk[6]; // xmm0 = (04 14 24 34 05 15 25 35)
        xmm2 = wk[7]; // xmm2 = (06 16 26 36 07 17 27 37)

        wk[8] = xmm3; // wk(8) = col1
        wk[9] = xmm4; // wk(9) = col3

        xmm3 = xmm0; // transpose coefficients(phase 3)
        xmm0 = _mm_unpacklo_epi64(xmm0, xmm6); // xmm0 = col4 = (04 14 24 34 44 54 64 74)
        xmm3 = _mm_unpackhi_epi64(xmm3, xmm6); // xmm3 = col5 = (05 15 25 35 45 55 65 75)
        xmm4 = xmm2; // transpose coefficients(phase 3)
        xmm2 = _mm_unpacklo_epi64(xmm2, xmm5); // xmm2 = col6 = (06 16 26 36 46 56 66 76)
        xmm4 = _mm_unpackhi_epi64(xmm4, xmm5); // xmm4 = col7 = (07 17 27 37 47 57 67 77)

        wk[10] = xmm3; // wk(10) = col5
        wk[11] = xmm4; // wk(11) = col7
    }

    // -- Prefetch the next coefficient block (left out; see the module
    // documentation)

    // ---- Pass 2: process rows from work array, store into output array.

    let rdi = &mut output_buf; // (JSAMPROW *)
    let eax = output_col as usize;

    // -- Even part

    // xmm7 = col0, xmm1 = col2, xmm0 = col4, xmm2 = col6

    // (Original)
    // z1 = (z2 + z3) * 0.541196100;
    // tmp2 = z1 + z3 * -1.847759065;
    // tmp3 = z1 + z2 * 0.765366865;
    //
    // (This implementation)
    // tmp2 = z2 * 0.541196100 + z3 * (0.541196100 - 1.847759065);
    // tmp3 = z2 * (0.541196100 + 0.765366865) + z3 * 0.541196100;

    xmm6 = xmm1; // xmm1 = in2 = z2
    xmm5 = xmm1;
    xmm6 = _mm_unpacklo_epi16(xmm6, xmm2); // xmm2 = in6 = z3
    xmm5 = _mm_unpackhi_epi16(xmm5, xmm2);
    xmm1 = xmm6;
    xmm2 = xmm5;
    xmm6 = _mm_madd_epi16(xmm6, _mm_loadu_si128(&rel.PW_F130_F054)); // xmm6 = tmp3L
    xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&rel.PW_F130_F054)); // xmm5 = tmp3H
    xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&rel.PW_F054_MF130)); // xmm1 = tmp2L
    xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&rel.PW_F054_MF130)); // xmm2 = tmp2H

    xmm3 = xmm7;
    xmm7 = _mm_add_epi16(xmm7, xmm0); // xmm7 = in0 + in4
    xmm3 = _mm_sub_epi16(xmm3, xmm0); // xmm3 = in0 - in4

    xmm4 = _mm_setzero_si128();
    xmm0 = _mm_setzero_si128();
    xmm4 = _mm_unpacklo_epi16(xmm4, xmm7); // xmm4 = tmp0L
    xmm0 = _mm_unpackhi_epi16(xmm0, xmm7); // xmm0 = tmp0H
    xmm4 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm4);
    // psrad xmm4, 16 & pslld xmm4, CONST_BITS
    xmm0 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm0);
    // psrad xmm0, 16 & pslld xmm0, CONST_BITS

    xmm7 = xmm4;
    xmm4 = _mm_add_epi32(xmm4, xmm6); // xmm4 = tmp10L
    xmm7 = _mm_sub_epi32(xmm7, xmm6); // xmm7 = tmp13L
    xmm6 = xmm0;
    xmm0 = _mm_add_epi32(xmm0, xmm5); // xmm0 = tmp10H
    xmm6 = _mm_sub_epi32(xmm6, xmm5); // xmm6 = tmp13H

    wk[0] = xmm4; // wk(0) = tmp10L
    wk[1] = xmm0; // wk(1) = tmp10H
    wk[2] = xmm7; // wk(2) = tmp13L
    wk[3] = xmm6; // wk(3) = tmp13H

    xmm5 = _mm_setzero_si128();
    xmm4 = _mm_setzero_si128();
    xmm5 = _mm_unpacklo_epi16(xmm5, xmm3); // xmm5 = tmp1L
    xmm4 = _mm_unpackhi_epi16(xmm4, xmm3); // xmm4 = tmp1H
    xmm5 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm5);
    // psrad xmm5, 16 & pslld xmm5, CONST_BITS
    xmm4 = _mm_srai_epi32::<{ 16 - CONST_BITS }>(xmm4);
    // psrad xmm4, 16 & pslld xmm4, CONST_BITS

    xmm0 = xmm5;
    xmm5 = _mm_add_epi32(xmm5, xmm1); // xmm5 = tmp11L
    xmm0 = _mm_sub_epi32(xmm0, xmm1); // xmm0 = tmp12L
    xmm7 = xmm4;
    xmm4 = _mm_add_epi32(xmm4, xmm2); // xmm4 = tmp11H
    xmm7 = _mm_sub_epi32(xmm7, xmm2); // xmm7 = tmp12H

    wk[4] = xmm5; // wk(4) = tmp11L
    wk[5] = xmm4; // wk(5) = tmp11H
    wk[6] = xmm0; // wk(6) = tmp12L
    wk[7] = xmm7; // wk(7) = tmp12H

    // -- Odd part

    xmm6 = wk[9]; // xmm6 = col3
    xmm3 = wk[8]; // xmm3 = col1
    xmm1 = wk[11]; // xmm1 = col7
    xmm2 = wk[10]; // xmm2 = col5

    xmm5 = xmm6;
    xmm4 = xmm3;
    xmm5 = _mm_add_epi16(xmm5, xmm1); // xmm5 = z3
    xmm4 = _mm_add_epi16(xmm4, xmm2); // xmm4 = z4

    // (Original)
    // z5 = (z3 + z4) * 1.175875602;
    // z3 = z3 * -1.961570560;  z4 = z4 * -0.390180644;
    // z3 += z5;  z4 += z5;
    //
    // (This implementation)
    // z3 = z3 * (1.175875602 - 1.961570560) + z4 * 1.175875602;
    // z4 = z3 * 1.175875602 + z4 * (1.175875602 - 0.390180644);

    xmm0 = xmm5;
    xmm7 = xmm5;
    xmm0 = _mm_unpacklo_epi16(xmm0, xmm4);
    xmm7 = _mm_unpackhi_epi16(xmm7, xmm4);
    xmm5 = xmm0;
    xmm4 = xmm7;
    xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&rel.PW_MF078_F117)); // xmm0 = z3L
    xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&rel.PW_MF078_F117)); // xmm7 = z3H
    xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&rel.PW_F117_F078)); // xmm5 = z4L
    xmm4 = _mm_madd_epi16(xmm4, _mm_loadu_si128(&rel.PW_F117_F078)); // xmm4 = z4H

    wk[10] = xmm0; // wk(10) = z3L
    wk[11] = xmm7; // wk(11) = z3H

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

    xmm0 = xmm1;
    xmm7 = xmm1;
    xmm0 = _mm_unpacklo_epi16(xmm0, xmm3);
    xmm7 = _mm_unpackhi_epi16(xmm7, xmm3);
    xmm1 = xmm0;
    xmm3 = xmm7;
    xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&rel.PW_MF060_MF089)); // xmm0 = tmp0L
    xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&rel.PW_MF060_MF089)); // xmm7 = tmp0H
    xmm1 = _mm_madd_epi16(xmm1, _mm_loadu_si128(&rel.PW_MF089_F060)); // xmm1 = tmp3L
    xmm3 = _mm_madd_epi16(xmm3, _mm_loadu_si128(&rel.PW_MF089_F060)); // xmm3 = tmp3H

    xmm0 = _mm_add_epi32(xmm0, wk[10]); // xmm0 = tmp0L
    xmm7 = _mm_add_epi32(xmm7, wk[11]); // xmm7 = tmp0H
    xmm1 = _mm_add_epi32(xmm1, xmm5); // xmm1 = tmp3L
    xmm3 = _mm_add_epi32(xmm3, xmm4); // xmm3 = tmp3H

    wk[8] = xmm0; // wk(8) = tmp0L
    wk[9] = xmm7; // wk(9) = tmp0H

    xmm0 = xmm2;
    xmm7 = xmm2;
    xmm0 = _mm_unpacklo_epi16(xmm0, xmm6);
    xmm7 = _mm_unpackhi_epi16(xmm7, xmm6);
    xmm2 = xmm0;
    xmm6 = xmm7;
    xmm0 = _mm_madd_epi16(xmm0, _mm_loadu_si128(&rel.PW_MF050_MF256)); // xmm0 = tmp1L
    xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&rel.PW_MF050_MF256)); // xmm7 = tmp1H
    xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&rel.PW_MF256_F050)); // xmm2 = tmp2L
    xmm6 = _mm_madd_epi16(xmm6, _mm_loadu_si128(&rel.PW_MF256_F050)); // xmm6 = tmp2H

    xmm0 = _mm_add_epi32(xmm0, xmm5); // xmm0 = tmp1L
    xmm7 = _mm_add_epi32(xmm7, xmm4); // xmm7 = tmp1H
    xmm2 = _mm_add_epi32(xmm2, wk[10]); // xmm2 = tmp2L
    xmm6 = _mm_add_epi32(xmm6, wk[11]); // xmm6 = tmp2H

    wk[10] = xmm0; // wk(10) = tmp1L
    wk[11] = xmm7; // wk(11) = tmp1H

    // -- Final output stage

    xmm5 = wk[0]; // xmm5 = tmp10L
    xmm4 = wk[1]; // xmm4 = tmp10H

    xmm0 = xmm5;
    xmm7 = xmm4;
    xmm5 = _mm_add_epi32(xmm5, xmm1); // xmm5 = data0L
    xmm4 = _mm_add_epi32(xmm4, xmm3); // xmm4 = data0H
    xmm0 = _mm_sub_epi32(xmm0, xmm1); // xmm0 = data7L
    xmm7 = _mm_sub_epi32(xmm7, xmm3); // xmm7 = data7H

    xmm1 = _mm_loadu_si128(&rel.PD_DESCALE_P2); // xmm1 = [rel PD_DESCALE_P2]

    xmm5 = _mm_add_epi32(xmm5, xmm1);
    xmm4 = _mm_add_epi32(xmm4, xmm1);
    xmm5 = _mm_srai_epi32::<DESCALE_P2>(xmm5);
    xmm4 = _mm_srai_epi32::<DESCALE_P2>(xmm4);
    xmm0 = _mm_add_epi32(xmm0, xmm1);
    xmm7 = _mm_add_epi32(xmm7, xmm1);
    xmm0 = _mm_srai_epi32::<DESCALE_P2>(xmm0);
    xmm7 = _mm_srai_epi32::<DESCALE_P2>(xmm7);

    xmm5 = _mm_packs_epi32(xmm5, xmm4); // xmm5 = data0 = (00 10 20 30 40 50 60 70)
    xmm0 = _mm_packs_epi32(xmm0, xmm7); // xmm0 = data7 = (07 17 27 37 47 57 67 77)

    xmm3 = wk[4]; // xmm3 = tmp11L
    xmm1 = wk[5]; // xmm1 = tmp11H

    xmm4 = xmm3;
    xmm7 = xmm1;
    xmm3 = _mm_add_epi32(xmm3, xmm2); // xmm3 = data1L
    xmm1 = _mm_add_epi32(xmm1, xmm6); // xmm1 = data1H
    xmm4 = _mm_sub_epi32(xmm4, xmm2); // xmm4 = data6L
    xmm7 = _mm_sub_epi32(xmm7, xmm6); // xmm7 = data6H

    xmm2 = _mm_loadu_si128(&rel.PD_DESCALE_P2); // xmm2 = [rel PD_DESCALE_P2]

    xmm3 = _mm_add_epi32(xmm3, xmm2);
    xmm1 = _mm_add_epi32(xmm1, xmm2);
    xmm3 = _mm_srai_epi32::<DESCALE_P2>(xmm3);
    xmm1 = _mm_srai_epi32::<DESCALE_P2>(xmm1);
    xmm4 = _mm_add_epi32(xmm4, xmm2);
    xmm7 = _mm_add_epi32(xmm7, xmm2);
    xmm4 = _mm_srai_epi32::<DESCALE_P2>(xmm4);
    xmm7 = _mm_srai_epi32::<DESCALE_P2>(xmm7);

    xmm3 = _mm_packs_epi32(xmm3, xmm1); // xmm3 = data1 = (01 11 21 31 41 51 61 71)
    xmm4 = _mm_packs_epi32(xmm4, xmm7); // xmm4 = data6 = (06 16 26 36 46 56 66 76)

    xmm5 = _mm_packs_epi16(xmm5, xmm4);
    // xmm5 = (00 10 20 30 40 50 60 70 06 16 26 36 46 56 66 76)
    xmm3 = _mm_packs_epi16(xmm3, xmm0);
    // xmm3 = (01 11 21 31 41 51 61 71 07 17 27 37 47 57 67 77)

    xmm6 = wk[6]; // xmm6 = tmp12L
    xmm2 = wk[7]; // xmm2 = tmp12H
    xmm1 = wk[10]; // xmm1 = tmp1L
    xmm7 = wk[11]; // xmm7 = tmp1H

    wk[0] = xmm5;
    // wk(0) = (00 10 20 30 40 50 60 70 06 16 26 36 46 56 66 76)
    wk[1] = xmm3;
    // wk(1) = (01 11 21 31 41 51 61 71 07 17 27 37 47 57 67 77)

    xmm4 = xmm6;
    xmm0 = xmm2;
    xmm6 = _mm_add_epi32(xmm6, xmm1); // xmm6 = data2L
    xmm2 = _mm_add_epi32(xmm2, xmm7); // xmm2 = data2H
    xmm4 = _mm_sub_epi32(xmm4, xmm1); // xmm4 = data5L
    xmm0 = _mm_sub_epi32(xmm0, xmm7); // xmm0 = data5H

    xmm5 = _mm_loadu_si128(&rel.PD_DESCALE_P2); // xmm5 = [rel PD_DESCALE_P2]

    xmm6 = _mm_add_epi32(xmm6, xmm5);
    xmm2 = _mm_add_epi32(xmm2, xmm5);
    xmm6 = _mm_srai_epi32::<DESCALE_P2>(xmm6);
    xmm2 = _mm_srai_epi32::<DESCALE_P2>(xmm2);
    xmm4 = _mm_add_epi32(xmm4, xmm5);
    xmm0 = _mm_add_epi32(xmm0, xmm5);
    xmm4 = _mm_srai_epi32::<DESCALE_P2>(xmm4);
    xmm0 = _mm_srai_epi32::<DESCALE_P2>(xmm0);

    xmm6 = _mm_packs_epi32(xmm6, xmm2); // xmm6 = data2 = (02 12 22 32 42 52 62 72)
    xmm4 = _mm_packs_epi32(xmm4, xmm0); // xmm4 = data5 = (05 15 25 35 45 55 65 75)

    xmm3 = wk[2]; // xmm3 = tmp13L
    xmm1 = wk[3]; // xmm1 = tmp13H
    xmm7 = wk[8]; // xmm7 = tmp0L
    xmm5 = wk[9]; // xmm5 = tmp0H

    xmm2 = xmm3;
    xmm0 = xmm1;
    xmm3 = _mm_add_epi32(xmm3, xmm7); // xmm3 = data3L
    xmm1 = _mm_add_epi32(xmm1, xmm5); // xmm1 = data3H
    xmm2 = _mm_sub_epi32(xmm2, xmm7); // xmm2 = data4L
    xmm0 = _mm_sub_epi32(xmm0, xmm5); // xmm0 = data4H

    xmm7 = _mm_loadu_si128(&rel.PD_DESCALE_P2); // xmm7 = [rel PD_DESCALE_P2]

    xmm3 = _mm_add_epi32(xmm3, xmm7);
    xmm1 = _mm_add_epi32(xmm1, xmm7);
    xmm3 = _mm_srai_epi32::<DESCALE_P2>(xmm3);
    xmm1 = _mm_srai_epi32::<DESCALE_P2>(xmm1);
    xmm2 = _mm_add_epi32(xmm2, xmm7);
    xmm0 = _mm_add_epi32(xmm0, xmm7);
    xmm2 = _mm_srai_epi32::<DESCALE_P2>(xmm2);
    xmm0 = _mm_srai_epi32::<DESCALE_P2>(xmm0);

    xmm5 = _mm_loadu_si128(&rel.PB_CENTERJSAMP); // xmm5 = [rel PB_CENTERJSAMP]

    xmm3 = _mm_packs_epi32(xmm3, xmm1); // xmm3 = data3 = (03 13 23 33 43 53 63 73)
    xmm2 = _mm_packs_epi32(xmm2, xmm0); // xmm2 = data4 = (04 14 24 34 44 54 64 74)

    xmm7 = wk[0];
    // xmm7 = (00 10 20 30 40 50 60 70 06 16 26 36 46 56 66 76)
    xmm1 = wk[1];
    // xmm1 = (01 11 21 31 41 51 61 71 07 17 27 37 47 57 67 77)

    xmm6 = _mm_packs_epi16(xmm6, xmm2);
    // xmm6 = (02 12 22 32 42 52 62 72 04 14 24 34 44 54 64 74)
    xmm3 = _mm_packs_epi16(xmm3, xmm4);
    // xmm3 = (03 13 23 33 43 53 63 73 05 15 25 35 45 55 65 75)

    xmm7 = _mm_add_epi8(xmm7, xmm5);
    xmm1 = _mm_add_epi8(xmm1, xmm5);
    xmm6 = _mm_add_epi8(xmm6, xmm5);
    xmm3 = _mm_add_epi8(xmm3, xmm5);

    // transpose coefficients(phase 1)
    xmm0 = xmm7;
    xmm7 = _mm_unpacklo_epi8(xmm7, xmm1);
    // xmm7 = (00 01 10 11 20 21 30 31 40 41 50 51 60 61 70 71)
    xmm0 = _mm_unpackhi_epi8(xmm0, xmm1);
    // xmm0 = (06 07 16 17 26 27 36 37 46 47 56 57 66 67 76 77)
    xmm2 = xmm6;
    xmm6 = _mm_unpacklo_epi8(xmm6, xmm3);
    // xmm6 = (02 03 12 13 22 23 32 33 42 43 52 53 62 63 72 73)
    xmm2 = _mm_unpackhi_epi8(xmm2, xmm3);
    // xmm2 = (04 05 14 15 24 25 34 35 44 45 54 55 64 65 74 75)

    // transpose coefficients(phase 2)
    xmm4 = xmm7;
    xmm7 = _mm_unpacklo_epi16(xmm7, xmm6);
    // xmm7 = (00 01 02 03 10 11 12 13 20 21 22 23 30 31 32 33)
    xmm4 = _mm_unpackhi_epi16(xmm4, xmm6);
    // xmm4 = (40 41 42 43 50 51 52 53 60 61 62 63 70 71 72 73)
    xmm5 = xmm2;
    xmm2 = _mm_unpacklo_epi16(xmm2, xmm0);
    // xmm2 = (04 05 06 07 14 15 16 17 24 25 26 27 34 35 36 37)
    xmm5 = _mm_unpackhi_epi16(xmm5, xmm0);
    // xmm5 = (44 45 46 47 54 55 56 57 64 65 66 67 74 75 76 77)

    // transpose coefficients(phase 3)
    xmm1 = xmm7;
    xmm7 = _mm_unpacklo_epi32(xmm7, xmm2);
    // xmm7 = (00 01 02 03 04 05 06 07 10 11 12 13 14 15 16 17)
    xmm1 = _mm_unpackhi_epi32(xmm1, xmm2);
    // xmm1 = (20 21 22 23 24 25 26 27 30 31 32 33 34 35 36 37)
    xmm3 = xmm4;
    xmm4 = _mm_unpacklo_epi32(xmm4, xmm5);
    // xmm4 = (40 41 42 43 44 45 46 47 50 51 52 53 54 55 56 57)
    xmm3 = _mm_unpackhi_epi32(xmm3, xmm5);
    // xmm3 = (60 61 62 63 64 65 66 67 70 71 72 73 74 75 76 77)

    xmm6 = _mm_shuffle_epi32::<0x4E>(xmm7);
    // xmm6 = (10 11 12 13 14 15 16 17 00 01 02 03 04 05 06 07)
    xmm0 = _mm_shuffle_epi32::<0x4E>(xmm1);
    // xmm0 = (30 31 32 33 34 35 36 37 20 21 22 23 24 25 26 27)
    xmm2 = _mm_shuffle_epi32::<0x4E>(xmm4);
    // xmm2 = (50 51 52 53 54 55 56 57 40 41 42 43 44 45 46 47)
    xmm5 = _mm_shuffle_epi32::<0x4E>(xmm3);
    // xmm5 = (70 71 72 73 74 75 76 77 60 61 62 63 64 65 66 67)

    // rdx = row 0, rsi = row 2
    _mm_storel_epi64(
        rdi.row_mut(0)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm7,
    );
    _mm_storel_epi64(
        rdi.row_mut(2)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm1,
    );
    // rdx = row 4, rsi = row 6
    _mm_storel_epi64(
        rdi.row_mut(4)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm4,
    );
    _mm_storel_epi64(
        rdi.row_mut(6)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm3,
    );

    // rdx = row 1, rsi = row 3
    _mm_storel_epi64(
        rdi.row_mut(1)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm6,
    );
    _mm_storel_epi64(
        rdi.row_mut(3)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm0,
    );
    // rdx = row 5, rsi = row 7
    _mm_storel_epi64(
        rdi.row_mut(5)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm2,
    );
    _mm_storel_epi64(
        rdi.row_mut(7)[eax..].first_chunk_mut::<8>().expect(OUTPUT),
        xmm5,
    );
}
