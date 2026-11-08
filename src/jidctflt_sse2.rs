// libjpeg-turbo 3.2.1, simd/x86_64/jidctflt-sse2.asm:
//
// ;
// ; Floating Point Inverse DCT (64-bit SSE & SSE2)
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
// ; This file contains a floating-point implementation of the inverse DCT
// ; (Discrete Cosine Transform).  The following code is based directly on the
// ; IJG's original jidctflt.c; see jidctflt.c for more details.
//! Translation of `simd/x86_64/jidctflt-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernel is SSE code on floats and SSE2 code on integers,
//! `#[target_feature(enable = "sse2")]`, which every x86-64 target enables.
//! Each vector instruction is the intrinsic that compiles to it, in the asm's
//! order, on local variables named after the registers (ADR 0008,
//! decision 3); a register move is an assignment and the stack slots `wk(i)`
//! are the local array `wk`, the stack array `workspace` a local array. The
//! float operations keep the asm's order, with no fused multiply-add, so the
//! results equal the asm's. The calling convention (`COLLECT_ARGS`, the
//! pushes and pops, `ENDBR64`) and the `%ifdef WIN64` branches of
//! `jsimdext.inc` that it expands to have no translation. The macros
//! `UNPCKLPS2` and `UNPCKHPS2` are functions of the same name.
//!
//! An XMM register holds integers or floats, but a Rust variable has one
//! type, `__m128i` or `__m128`. Where an instruction changes the kind of a
//! register's contents, the register gets a new binding of the same name
//! with the other type (`let`): `cvtdq2ps` converts, and an integer
//! instruction on a register that holds floats reads it through
//! `_mm_castps_si128`, which compiles to no instruction.
//!
//! `FAST_FLOAT` and `FLOAT_MULT_TYPE` are 4-byte floats, as
//! `jsimd_set_idct_float` requires, and the multiplier table is the
//! component's `float_array`, whose 4 elements from an index a `movaps`
//! load reads (`multiplier_table::float_array`).
//! `NO_ZERO_COLUMN_TEST_FLOAT_SSE` is undefined, as in the oracle's build, so
//! the test for columns whose AC terms are all zero is translated.
//!
//! The pointers of the asm are indices: `rsi` into the coefficient block,
//! `rdx` into the multiplier table, `rdi` and `rsi` into `workspace`, and
//! `rdi` the first of the 4 output rows of a pass-2 iteration.
//! `MMBLOCK(m, 0, rsi, SIZEOF_JCOEF)` of `jdct.inc` is the 4 coefficients of
//! row `m` from `rsi` on, `XMMBLOCK(m, n, rdi, SIZEOF_FAST_FLOAT)` the 4
//! floats of row `m` from `rdi + 4 * n` on, and
//! `DWBLOCK(m, 0, rsi, SIZEOF_JCOEF)` the 32-bit value of the first two
//! coefficients of row `m` from `rsi` on. An output row
//! `output_buf[i] + output_col` is the row's slice from `output_col` on, and
//! each `movq` store converts its first 8 samples to the array that
//! `_mm_storel_epi64` takes, so it writes the 8 samples of the block's row
//! and nothing past them. The kernel takes each row pointer when it stores,
//! as `output_buf` lends one row at a time. The `prefetchnta` hints for the
//! next coefficient block are not translated: they change no result, and
//! they would need a pointer past `coef_block`, which the module's safe code
//! cannot form.

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

use crate::jdct::FLOAT_MULT_TYPE;
use crate::jddctmgr::multiplier_table;
use crate::jmorecfg::{CENTERJSAMPLE, FAST_FLOAT, JDIMENSION};
use crate::jpeglib::{DCTSIZE, DCTSIZE2, JBLOCK, JSAMPROWS_MUT};
use crate::x86::{_mm_loadl_epi64, _mm_loadu_ps, _mm_loadu_si128, _mm_storel_epi64, _mm_storeu_ps};

// upstream: simd/nasm/jsimdext.inc WORD_BIT
const WORD_BIT: i32 = 16; /* sizeof(word) * BYTE_BIT */
// upstream: simd/nasm/jsimdext.inc DWORD_BIT
const DWORD_BIT: i32 = 32; /* sizeof(dword) * BYTE_BIT */

// upstream: simd/x86_64/jidctflt-sse2.asm UNPCKLPS2
/// `%1 = (0 1 2 3) / %2 = (4 5 6 7) => %1 = (0 1 4 5)`
#[expect(non_snake_case, reason = "the asm's macro name")]
#[target_feature(enable = "sse2")]
fn UNPCKLPS2(a: __m128, b: __m128) -> __m128 {
    _mm_shuffle_ps::<0x44>(a, b)
}

// upstream: simd/x86_64/jidctflt-sse2.asm UNPCKHPS2
/// `%1 = (0 1 2 3) / %2 = (4 5 6 7) => %1 = (2 3 6 7)`
#[expect(non_snake_case, reason = "the asm's macro name")]
#[target_feature(enable = "sse2")]
fn UNPCKHPS2(a: __m128, b: __m128) -> __m128 {
    _mm_shuffle_ps::<0xEE>(a, b)
}

/// The layout of `jconst_idct_float_sse2`: the asm's labels as fields,
/// aligned as `ALIGNZ 32` aligns the table. Not an upstream item.
#[expect(non_snake_case, reason = "the asm's labels")]
#[repr(C, align(32))]
pub struct JconstIdctFloatSse2 {
    PD_1_414: [f32; 4],
    PD_1_847: [f32; 4],
    PD_1_082: [f32; 4],
    PD_M2_613: [f32; 4],
    PD_RNDINT_MAGIC: [f32; 4],
    PB_CENTERJSAMP: [u8; 16],
}

// upstream: simd/x86_64/jidctflt-sse2.asm jconst_idct_float_sse2
#[expect(non_upper_case_globals, reason = "the asm's label")]
#[expect(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::unreadable_literal,
    reason = "the asm's dd literals"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the asm's db CENTERJSAMPLE"
)]
pub static jconst_idct_float_sse2: JconstIdctFloatSse2 = JconstIdctFloatSse2 {
    PD_1_414: [1.414213562373095048801689; 4],
    PD_1_847: [1.847759065022573512256366; 4],
    PD_1_082: [1.082392200292393968799446; 4],
    PD_M2_613: [-2.613125929752753055713286; 4],
    PD_RNDINT_MAGIC: [100663296.0; 4], /* (float)(0x00C00000 << 3) */
    PB_CENTERJSAMP: [CENTERJSAMPLE as u8; 16],
};

// upstream: simd/x86_64/jidctflt-sse2.asm WK_NUM
const WK_NUM: usize = 2;

/// The message of a coefficient load that cannot fail. Not an upstream item.
const COEFS: &str = "a block row holds 4 coefficients from rsi on";
/// The message of a workspace access that cannot fail. Not an upstream item.
const WORKSPACE: &str = "a workspace row holds 4 floats from rdi on";
/// The message of an output row that is too short. Not an upstream item.
const OUTPUT: &str = "an output row holds 8 samples from output_col on";

// upstream: simd/x86_64/jidctflt-sse2.asm jsimd_idct_float_sse2
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
    clippy::identity_op,
    clippy::erasing_op,
    reason = "the asm's XMMBLOCK(m, n, ...) offsets with m or n = 0 and 1"
)]
#[expect(
    clippy::used_underscore_items,
    reason = "the x86 loads and stores have the intrinsics' names"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_idct_float_sse2(
    dct_table: &multiplier_table,
    coef_block: &JBLOCK,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let mut wk = [_mm_setzero_ps(); WK_NUM]; /* xmmword wk[WK_NUM] */
    let mut workspace: [FAST_FLOAT; DCTSIZE2] = [0.0; DCTSIZE2]; /* FAST_FLOAT workspace[DCTSIZE2] */
    // XMMWORD [XMMBLOCK(m, 0, rdx, SIZEOF_FLOAT_MULT_TYPE)] for rdx + m * DCTSIZE
    let quantptr = |i: usize| -> [FLOAT_MULT_TYPE; 4] {
        core::array::from_fn(|j| dct_table.float_array(i + j))
    };
    // MMBLOCK(m, 0, rsi, SIZEOF_JCOEF) for rsi + m * DCTSIZE
    let inptr = |i: usize| -> &[i16; 4] { coef_block[i..].first_chunk::<4>().expect(COEFS) };

    // ---- Pass 1: process columns from input, store into work array.

    let mut rdx = 0; // quantptr
    let mut rsi = 0; // inptr
    let mut rdi = 0; // FAST_FLOAT *wsptr
    let mut rcx = DCTSIZE / 4; // ctr
    loop {
        // .columnloop:
        'nextcolumn: {
            // %ifndef NO_ZERO_COLUMN_TEST_FLOAT_SSE
            let mut eax = u32::from(coef_block[rsi + 1 * DCTSIZE].cast_unsigned())
                | u32::from(coef_block[rsi + 1 * DCTSIZE + 1].cast_unsigned()) << 16;
            eax |= u32::from(coef_block[rsi + 2 * DCTSIZE].cast_unsigned())
                | u32::from(coef_block[rsi + 2 * DCTSIZE + 1].cast_unsigned()) << 16;
            if eax == 0 {
                let mut xmm1 = _mm_loadl_epi64(inptr(rsi + 1 * DCTSIZE));
                let xmm2 = _mm_loadl_epi64(inptr(rsi + 2 * DCTSIZE));
                let mut xmm3 = _mm_loadl_epi64(inptr(rsi + 3 * DCTSIZE));
                let xmm4 = _mm_loadl_epi64(inptr(rsi + 4 * DCTSIZE));
                let mut xmm5 = _mm_loadl_epi64(inptr(rsi + 5 * DCTSIZE));
                let xmm6 = _mm_loadl_epi64(inptr(rsi + 6 * DCTSIZE));
                let xmm7 = _mm_loadl_epi64(inptr(rsi + 7 * DCTSIZE));
                xmm1 = _mm_or_si128(xmm1, xmm2);
                xmm3 = _mm_or_si128(xmm3, xmm4);
                xmm5 = _mm_or_si128(xmm5, xmm6);
                xmm1 = _mm_or_si128(xmm1, xmm3);
                xmm5 = _mm_or_si128(xmm5, xmm7);
                xmm1 = _mm_or_si128(xmm1, xmm5);
                xmm1 = _mm_packs_epi16(xmm1, xmm1);
                if _mm_cvtsi128_si32(xmm1) == 0 {
                    // -- AC terms all zero

                    let xmm0 = _mm_loadl_epi64(inptr(rsi + 0 * DCTSIZE));

                    let xmm0 = _mm_unpacklo_epi16(xmm0, xmm0); // xmm0 = (00 00 01 01 02 02 03 03)
                    let xmm0 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm0); // xmm0 = in0 = (00 01 02 03)
                    let mut xmm0 = _mm_cvtepi32_ps(xmm0); // xmm0 = in0 = (00 01 02 03)

                    xmm0 = _mm_mul_ps(xmm0, _mm_loadu_ps(&quantptr(rdx + 0 * DCTSIZE)));

                    let mut xmm1 = xmm0;
                    let mut xmm2 = xmm0;
                    let mut xmm3 = xmm0;

                    xmm0 = _mm_shuffle_ps::<0x00>(xmm0, xmm0); // xmm0 = (00 00 00 00)
                    xmm1 = _mm_shuffle_ps::<0x55>(xmm1, xmm1); // xmm1 = (01 01 01 01)
                    xmm2 = _mm_shuffle_ps::<0xAA>(xmm2, xmm2); // xmm2 = (02 02 02 02)
                    xmm3 = _mm_shuffle_ps::<0xFF>(xmm3, xmm3); // xmm3 = (03 03 03 03)

                    let wsptr = &mut workspace;
                    let ws = |i: usize| -> core::ops::RangeFrom<usize> { rdi + i.. };
                    _mm_storeu_ps(
                        wsptr[ws(0 * DCTSIZE + 0 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm0,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(0 * DCTSIZE + 1 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm0,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(1 * DCTSIZE + 0 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm1,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(1 * DCTSIZE + 1 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm1,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(2 * DCTSIZE + 0 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm2,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(2 * DCTSIZE + 1 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm2,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(3 * DCTSIZE + 0 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm3,
                    );
                    _mm_storeu_ps(
                        wsptr[ws(3 * DCTSIZE + 1 * 4)]
                            .first_chunk_mut::<4>()
                            .expect(WORKSPACE),
                        xmm3,
                    );
                    break 'nextcolumn;
                }
            }
            // %endif
            // .columnDCT:

            // -- Even part

            let xmm0 = _mm_loadl_epi64(inptr(rsi + 0 * DCTSIZE));
            let xmm1 = _mm_loadl_epi64(inptr(rsi + 2 * DCTSIZE));
            let xmm2 = _mm_loadl_epi64(inptr(rsi + 4 * DCTSIZE));
            let xmm3 = _mm_loadl_epi64(inptr(rsi + 6 * DCTSIZE));

            let xmm0 = _mm_unpacklo_epi16(xmm0, xmm0); // xmm0 = (00 00 01 01 02 02 03 03)
            let xmm1 = _mm_unpacklo_epi16(xmm1, xmm1); // xmm1 = (20 20 21 21 22 22 23 23)
            let xmm0 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm0); // xmm0 = in0 = (00 01 02 03)
            let xmm1 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm1); // xmm1 = in2 = (20 21 22 23)
            let mut xmm0 = _mm_cvtepi32_ps(xmm0); // xmm0 = in0 = (00 01 02 03)
            let mut xmm1 = _mm_cvtepi32_ps(xmm1); // xmm1 = in2 = (20 21 22 23)

            let xmm2 = _mm_unpacklo_epi16(xmm2, xmm2); // xmm2 = (40 40 41 41 42 42 43 43)
            let xmm3 = _mm_unpacklo_epi16(xmm3, xmm3); // xmm3 = (60 60 61 61 62 62 63 63)
            let xmm2 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm2); // xmm2 = in4 = (40 41 42 43)
            let xmm3 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm3); // xmm3 = in6 = (60 61 62 63)
            let mut xmm2 = _mm_cvtepi32_ps(xmm2); // xmm2 = in4 = (40 41 42 43)
            let mut xmm3 = _mm_cvtepi32_ps(xmm3); // xmm3 = in6 = (60 61 62 63)

            xmm0 = _mm_mul_ps(xmm0, _mm_loadu_ps(&quantptr(rdx + 0 * DCTSIZE)));
            xmm1 = _mm_mul_ps(xmm1, _mm_loadu_ps(&quantptr(rdx + 2 * DCTSIZE)));
            xmm2 = _mm_mul_ps(xmm2, _mm_loadu_ps(&quantptr(rdx + 4 * DCTSIZE)));
            xmm3 = _mm_mul_ps(xmm3, _mm_loadu_ps(&quantptr(rdx + 6 * DCTSIZE)));

            let mut xmm4 = xmm0;
            let mut xmm5 = xmm1;
            xmm0 = _mm_sub_ps(xmm0, xmm2); // xmm0 = tmp11
            xmm1 = _mm_sub_ps(xmm1, xmm3);
            xmm4 = _mm_add_ps(xmm4, xmm2); // xmm4 = tmp10
            xmm5 = _mm_add_ps(xmm5, xmm3); // xmm5 = tmp13

            xmm1 = _mm_mul_ps(xmm1, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_414));
            xmm1 = _mm_sub_ps(xmm1, xmm5); // xmm1 = tmp12

            let mut xmm6 = xmm4;
            let mut xmm7 = xmm0;
            xmm4 = _mm_sub_ps(xmm4, xmm5); // xmm4 = tmp3
            xmm0 = _mm_sub_ps(xmm0, xmm1); // xmm0 = tmp2
            xmm6 = _mm_add_ps(xmm6, xmm5); // xmm6 = tmp0
            xmm7 = _mm_add_ps(xmm7, xmm1); // xmm7 = tmp1

            wk[1] = xmm4; // tmp3
            wk[0] = xmm0; // tmp2

            // -- Odd part

            let xmm2 = _mm_loadl_epi64(inptr(rsi + 1 * DCTSIZE));
            let xmm3 = _mm_loadl_epi64(inptr(rsi + 3 * DCTSIZE));
            let xmm5 = _mm_loadl_epi64(inptr(rsi + 5 * DCTSIZE));
            let xmm1 = _mm_loadl_epi64(inptr(rsi + 7 * DCTSIZE));

            let xmm2 = _mm_unpacklo_epi16(xmm2, xmm2); // xmm2 = (10 10 11 11 12 12 13 13)
            let xmm3 = _mm_unpacklo_epi16(xmm3, xmm3); // xmm3 = (30 30 31 31 32 32 33 33)
            let xmm2 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm2); // xmm2 = in1 = (10 11 12 13)
            let xmm3 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm3); // xmm3 = in3 = (30 31 32 33)
            let mut xmm2 = _mm_cvtepi32_ps(xmm2); // xmm2 = in1 = (10 11 12 13)
            let mut xmm3 = _mm_cvtepi32_ps(xmm3); // xmm3 = in3 = (30 31 32 33)

            let xmm5 = _mm_unpacklo_epi16(xmm5, xmm5); // xmm5 = (50 50 51 51 52 52 53 53)
            let xmm1 = _mm_unpacklo_epi16(xmm1, xmm1); // xmm1 = (70 70 71 71 72 72 73 73)
            let xmm5 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm5); // xmm5 = in5 = (50 51 52 53)
            let xmm1 = _mm_srai_epi32::<{ DWORD_BIT - WORD_BIT }>(xmm1); // xmm1 = in7 = (70 71 72 73)
            let mut xmm5 = _mm_cvtepi32_ps(xmm5); // xmm5 = in5 = (50 51 52 53)
            let mut xmm1 = _mm_cvtepi32_ps(xmm1); // xmm1 = in7 = (70 71 72 73)

            xmm2 = _mm_mul_ps(xmm2, _mm_loadu_ps(&quantptr(rdx + 1 * DCTSIZE)));
            xmm3 = _mm_mul_ps(xmm3, _mm_loadu_ps(&quantptr(rdx + 3 * DCTSIZE)));
            xmm5 = _mm_mul_ps(xmm5, _mm_loadu_ps(&quantptr(rdx + 5 * DCTSIZE)));
            xmm1 = _mm_mul_ps(xmm1, _mm_loadu_ps(&quantptr(rdx + 7 * DCTSIZE)));

            xmm4 = xmm2;
            xmm0 = xmm5;
            xmm2 = _mm_add_ps(xmm2, xmm1); // xmm2 = z11
            xmm5 = _mm_add_ps(xmm5, xmm3); // xmm5 = z13
            xmm4 = _mm_sub_ps(xmm4, xmm1); // xmm4 = z12
            xmm0 = _mm_sub_ps(xmm0, xmm3); // xmm0 = z10

            xmm1 = xmm2;
            xmm2 = _mm_sub_ps(xmm2, xmm5);
            xmm1 = _mm_add_ps(xmm1, xmm5); // xmm1 = tmp7

            xmm2 = _mm_mul_ps(xmm2, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_414)); // xmm2 = tmp11

            xmm3 = xmm0;
            xmm0 = _mm_add_ps(xmm0, xmm4);
            xmm0 = _mm_mul_ps(xmm0, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_847)); // xmm0 = z5
            xmm3 = _mm_mul_ps(xmm3, _mm_loadu_ps(&jconst_idct_float_sse2.PD_M2_613)); // xmm3 = (z10 * -2.613125930)
            xmm4 = _mm_mul_ps(xmm4, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_082)); // xmm4 = (z12 * 1.082392200)
            xmm3 = _mm_add_ps(xmm3, xmm0); // xmm3 = tmp12
            xmm4 = _mm_sub_ps(xmm4, xmm0); // xmm4 = tmp10

            // -- Final output stage

            xmm3 = _mm_sub_ps(xmm3, xmm1); // xmm3 = tmp6
            xmm5 = xmm6;
            xmm0 = xmm7;
            xmm6 = _mm_add_ps(xmm6, xmm1); // xmm6 = data0 = (00 01 02 03)
            xmm7 = _mm_add_ps(xmm7, xmm3); // xmm7 = data1 = (10 11 12 13)
            xmm5 = _mm_sub_ps(xmm5, xmm1); // xmm5 = data7 = (70 71 72 73)
            xmm0 = _mm_sub_ps(xmm0, xmm3); // xmm0 = data6 = (60 61 62 63)
            xmm2 = _mm_sub_ps(xmm2, xmm3); // xmm2 = tmp5

            xmm1 = xmm6; // transpose coefficients(phase 1)
            xmm6 = _mm_unpacklo_ps(xmm6, xmm7); // xmm6 = (00 10 01 11)
            xmm1 = _mm_unpackhi_ps(xmm1, xmm7); // xmm1 = (02 12 03 13)
            xmm3 = xmm0; // transpose coefficients(phase 1)
            xmm0 = _mm_unpacklo_ps(xmm0, xmm5); // xmm0 = (60 70 61 71)
            xmm3 = _mm_unpackhi_ps(xmm3, xmm5); // xmm3 = (62 72 63 73)

            xmm7 = wk[0]; // xmm7 = tmp2
            xmm5 = wk[1]; // xmm5 = tmp3

            wk[0] = xmm0; // wk(0) = (60 70 61 71)
            wk[1] = xmm3; // wk(1) = (62 72 63 73)

            xmm4 = _mm_add_ps(xmm4, xmm2); // xmm4 = tmp4
            xmm0 = xmm7;
            xmm3 = xmm5;
            xmm7 = _mm_add_ps(xmm7, xmm2); // xmm7 = data2 = (20 21 22 23)
            xmm5 = _mm_add_ps(xmm5, xmm4); // xmm5 = data4 = (40 41 42 43)
            xmm0 = _mm_sub_ps(xmm0, xmm2); // xmm0 = data5 = (50 51 52 53)
            xmm3 = _mm_sub_ps(xmm3, xmm4); // xmm3 = data3 = (30 31 32 33)

            xmm2 = xmm7; // transpose coefficients(phase 1)
            xmm7 = _mm_unpacklo_ps(xmm7, xmm3); // xmm7 = (20 30 21 31)
            xmm2 = _mm_unpackhi_ps(xmm2, xmm3); // xmm2 = (22 32 23 33)
            xmm4 = xmm5; // transpose coefficients(phase 1)
            xmm5 = _mm_unpacklo_ps(xmm5, xmm0); // xmm5 = (40 50 41 51)
            xmm4 = _mm_unpackhi_ps(xmm4, xmm0); // xmm4 = (42 52 43 53)

            xmm3 = xmm6; // transpose coefficients(phase 2)
            xmm6 = UNPCKLPS2(xmm6, xmm7); // xmm6 = (00 10 20 30)
            xmm3 = UNPCKHPS2(xmm3, xmm7); // xmm3 = (01 11 21 31)
            xmm0 = xmm1; // transpose coefficients(phase 2)
            xmm1 = UNPCKLPS2(xmm1, xmm2); // xmm1 = (02 12 22 32)
            xmm0 = UNPCKHPS2(xmm0, xmm2); // xmm0 = (03 13 23 33)

            xmm7 = wk[0]; // xmm7 = (60 70 61 71)
            xmm2 = wk[1]; // xmm2 = (62 72 63 73)

            let wsptr = &mut workspace;
            let ws = |i: usize| -> core::ops::RangeFrom<usize> { rdi + i.. };
            _mm_storeu_ps(
                wsptr[ws(0 * DCTSIZE + 0 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm6,
            );
            _mm_storeu_ps(
                wsptr[ws(1 * DCTSIZE + 0 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm3,
            );
            _mm_storeu_ps(
                wsptr[ws(2 * DCTSIZE + 0 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm1,
            );
            _mm_storeu_ps(
                wsptr[ws(3 * DCTSIZE + 0 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm0,
            );

            xmm6 = xmm5; // transpose coefficients(phase 2)
            xmm5 = UNPCKLPS2(xmm5, xmm7); // xmm5 = (40 50 60 70)
            xmm6 = UNPCKHPS2(xmm6, xmm7); // xmm6 = (41 51 61 71)
            xmm3 = xmm4; // transpose coefficients(phase 2)
            xmm4 = UNPCKLPS2(xmm4, xmm2); // xmm4 = (42 52 62 72)
            xmm3 = UNPCKHPS2(xmm3, xmm2); // xmm3 = (43 53 63 73)

            _mm_storeu_ps(
                wsptr[ws(0 * DCTSIZE + 1 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm5,
            );
            _mm_storeu_ps(
                wsptr[ws(1 * DCTSIZE + 1 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm6,
            );
            _mm_storeu_ps(
                wsptr[ws(2 * DCTSIZE + 1 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm4,
            );
            _mm_storeu_ps(
                wsptr[ws(3 * DCTSIZE + 1 * 4)]
                    .first_chunk_mut::<4>()
                    .expect(WORKSPACE),
                xmm3,
            );
        }
        // .nextcolumn:
        rsi += 4; // coef_block
        rdx += 4; // quantptr
        rdi += 4 * DCTSIZE; // wsptr
        rcx -= 1; // ctr
        if rcx == 0 {
            break;
        }
    }

    // -- Prefetch the next coefficient block
    // (prefetchnta: not translated; see the module documentation)

    // ---- Pass 2: process rows from work array, store into output array.

    let mut rsi = 0; // FAST_FLOAT *wsptr
    let mut rdi = 0; // (JSAMPROW *)
    let eax = output_col as usize;
    let mut rcx = DCTSIZE / 4; // ctr
    loop {
        // .rowloop:
        let wsptr = |i: usize| -> &[FAST_FLOAT; 4] {
            workspace[rsi + i..].first_chunk::<4>().expect(WORKSPACE)
        };

        // -- Even part

        let mut xmm0 = _mm_loadu_ps(wsptr(0 * DCTSIZE));
        let mut xmm1 = _mm_loadu_ps(wsptr(2 * DCTSIZE));
        let mut xmm2 = _mm_loadu_ps(wsptr(4 * DCTSIZE));
        let mut xmm3 = _mm_loadu_ps(wsptr(6 * DCTSIZE));

        let mut xmm4 = xmm0;
        let mut xmm5 = xmm1;
        xmm0 = _mm_sub_ps(xmm0, xmm2); // xmm0 = tmp11
        xmm1 = _mm_sub_ps(xmm1, xmm3);
        xmm4 = _mm_add_ps(xmm4, xmm2); // xmm4 = tmp10
        xmm5 = _mm_add_ps(xmm5, xmm3); // xmm5 = tmp13

        xmm1 = _mm_mul_ps(xmm1, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_414));
        xmm1 = _mm_sub_ps(xmm1, xmm5); // xmm1 = tmp12

        let mut xmm6 = xmm4;
        let mut xmm7 = xmm0;
        xmm4 = _mm_sub_ps(xmm4, xmm5); // xmm4 = tmp3
        xmm0 = _mm_sub_ps(xmm0, xmm1); // xmm0 = tmp2
        xmm6 = _mm_add_ps(xmm6, xmm5); // xmm6 = tmp0
        xmm7 = _mm_add_ps(xmm7, xmm1); // xmm7 = tmp1

        wk[1] = xmm4; // tmp3
        wk[0] = xmm0; // tmp2

        // -- Odd part

        xmm2 = _mm_loadu_ps(wsptr(1 * DCTSIZE));
        xmm3 = _mm_loadu_ps(wsptr(3 * DCTSIZE));
        xmm5 = _mm_loadu_ps(wsptr(5 * DCTSIZE));
        xmm1 = _mm_loadu_ps(wsptr(7 * DCTSIZE));

        xmm4 = xmm2;
        xmm0 = xmm5;
        xmm2 = _mm_add_ps(xmm2, xmm1); // xmm2 = z11
        xmm5 = _mm_add_ps(xmm5, xmm3); // xmm5 = z13
        xmm4 = _mm_sub_ps(xmm4, xmm1); // xmm4 = z12
        xmm0 = _mm_sub_ps(xmm0, xmm3); // xmm0 = z10

        xmm1 = xmm2;
        xmm2 = _mm_sub_ps(xmm2, xmm5);
        xmm1 = _mm_add_ps(xmm1, xmm5); // xmm1 = tmp7

        xmm2 = _mm_mul_ps(xmm2, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_414)); // xmm2 = tmp11

        xmm3 = xmm0;
        xmm0 = _mm_add_ps(xmm0, xmm4);
        xmm0 = _mm_mul_ps(xmm0, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_847)); // xmm0 = z5
        xmm3 = _mm_mul_ps(xmm3, _mm_loadu_ps(&jconst_idct_float_sse2.PD_M2_613)); // xmm3 = (z10 * -2.613125930)
        xmm4 = _mm_mul_ps(xmm4, _mm_loadu_ps(&jconst_idct_float_sse2.PD_1_082)); // xmm4 = (z12 * 1.082392200)
        xmm3 = _mm_add_ps(xmm3, xmm0); // xmm3 = tmp12
        xmm4 = _mm_sub_ps(xmm4, xmm0); // xmm4 = tmp10

        // -- Final output stage

        xmm3 = _mm_sub_ps(xmm3, xmm1); // xmm3 = tmp6
        xmm5 = xmm6;
        xmm0 = xmm7;
        xmm6 = _mm_add_ps(xmm6, xmm1); // xmm6 = data0 = (00 10 20 30)
        xmm7 = _mm_add_ps(xmm7, xmm3); // xmm7 = data1 = (01 11 21 31)
        xmm5 = _mm_sub_ps(xmm5, xmm1); // xmm5 = data7 = (07 17 27 37)
        xmm0 = _mm_sub_ps(xmm0, xmm3); // xmm0 = data6 = (06 16 26 36)
        xmm2 = _mm_sub_ps(xmm2, xmm3); // xmm2 = tmp5

        xmm1 = _mm_loadu_ps(&jconst_idct_float_sse2.PD_RNDINT_MAGIC); // xmm1 = [rel PD_RNDINT_MAGIC]
        let xmm3 = _mm_set1_epi32(-1);
        let xmm3 = _mm_srli_epi32::<WORD_BIT>(xmm3); // xmm3 = { 0xFFFF 0x0000 0xFFFF 0x0000 .. }

        xmm6 = _mm_add_ps(xmm6, xmm1);
        // xmm6 = roundint(data0 / 8) = (00 ** 10 ** 20 ** 30 **)
        xmm7 = _mm_add_ps(xmm7, xmm1);
        // xmm7 = roundint(data1 / 8) = (01 ** 11 ** 21 ** 31 **)
        xmm0 = _mm_add_ps(xmm0, xmm1);
        // xmm0 = roundint(data6 / 8) = (06 ** 16 ** 26 ** 36 **)
        xmm5 = _mm_add_ps(xmm5, xmm1);
        // xmm5 = roundint(data7 / 8) = (07 ** 17 ** 27 ** 37 **)

        let xmm6 = _mm_and_si128(_mm_castps_si128(xmm6), xmm3); // xmm6 = (00 -- 10 -- 20 -- 30 --)
        let xmm7 = _mm_slli_epi32::<WORD_BIT>(_mm_castps_si128(xmm7)); // xmm7 = (-- 01 -- 11 -- 21 -- 31)
        let xmm0 = _mm_and_si128(_mm_castps_si128(xmm0), xmm3); // xmm0 = (06 -- 16 -- 26 -- 36 --)
        let xmm5 = _mm_slli_epi32::<WORD_BIT>(_mm_castps_si128(xmm5)); // xmm5 = (-- 07 -- 17 -- 27 -- 37)
        let mut xmm6 = _mm_or_si128(xmm6, xmm7); // xmm6 = (00 01 10 11 20 21 30 31)
        let xmm0 = _mm_or_si128(xmm0, xmm5); // xmm0 = (06 07 16 17 26 27 36 37)

        let mut xmm1 = wk[0]; // xmm1 = tmp2
        let mut xmm3 = wk[1]; // xmm3 = tmp3

        xmm4 = _mm_add_ps(xmm4, xmm2); // xmm4 = tmp4
        let mut xmm7 = xmm1;
        let mut xmm5 = xmm3;
        xmm1 = _mm_add_ps(xmm1, xmm2); // xmm1 = data2 = (02 12 22 32)
        xmm3 = _mm_add_ps(xmm3, xmm4); // xmm3 = data4 = (04 14 24 34)
        xmm7 = _mm_sub_ps(xmm7, xmm2); // xmm7 = data5 = (05 15 25 35)
        xmm5 = _mm_sub_ps(xmm5, xmm4); // xmm5 = data3 = (03 13 23 33)

        xmm2 = _mm_loadu_ps(&jconst_idct_float_sse2.PD_RNDINT_MAGIC); // xmm2 = [rel PD_RNDINT_MAGIC]
        let xmm4 = _mm_set1_epi32(-1);
        let xmm4 = _mm_srli_epi32::<WORD_BIT>(xmm4); // xmm4 = { 0xFFFF 0x0000 0xFFFF 0x0000 .. }

        xmm3 = _mm_add_ps(xmm3, xmm2);
        // xmm3 = roundint(data4 / 8) = (04 ** 14 ** 24 ** 34 **)
        xmm7 = _mm_add_ps(xmm7, xmm2);
        // xmm7 = roundint(data5 / 8) = (05 ** 15 ** 25 ** 35 **)
        xmm1 = _mm_add_ps(xmm1, xmm2);
        // xmm1 = roundint(data2 / 8) = (02 ** 12 ** 22 ** 32 **)
        xmm5 = _mm_add_ps(xmm5, xmm2);
        // xmm5 = roundint(data3 / 8) = (03 ** 13 ** 23 ** 33 **)

        let xmm3 = _mm_and_si128(_mm_castps_si128(xmm3), xmm4); // xmm3 = (04 -- 14 -- 24 -- 34 --)
        let xmm7 = _mm_slli_epi32::<WORD_BIT>(_mm_castps_si128(xmm7)); // xmm7 = (-- 05 -- 15 -- 25 -- 35)
        let xmm1 = _mm_and_si128(_mm_castps_si128(xmm1), xmm4); // xmm1 = (02 -- 12 -- 22 -- 32 --)
        let xmm5 = _mm_slli_epi32::<WORD_BIT>(_mm_castps_si128(xmm5)); // xmm5 = (-- 03 -- 13 -- 23 -- 33)
        let xmm3 = _mm_or_si128(xmm3, xmm7); // xmm3 = (04 05 14 15 24 25 34 35)
        let mut xmm1 = _mm_or_si128(xmm1, xmm5); // xmm1 = (02 03 12 13 22 23 32 33)

        let xmm2 = _mm_loadu_si128(&jconst_idct_float_sse2.PB_CENTERJSAMP); // xmm2 = [rel PB_CENTERJSAMP]

        xmm6 = _mm_packs_epi16(xmm6, xmm3);
        // xmm6 = (00 01 10 11 20 21 30 31 04 05 14 15 24 25 34 35)
        xmm1 = _mm_packs_epi16(xmm1, xmm0);
        // xmm1 = (02 03 12 13 22 23 32 33 06 07 16 17 26 27 36 37)
        xmm6 = _mm_add_epi8(xmm6, xmm2);
        xmm1 = _mm_add_epi8(xmm1, xmm2);

        // transpose coefficients(phase 2)
        let mut xmm4 = xmm6;
        xmm6 = _mm_unpacklo_epi16(xmm6, xmm1);
        // xmm6 = (00 01 02 03 10 11 12 13 20 21 22 23 30 31 32 33)
        xmm4 = _mm_unpackhi_epi16(xmm4, xmm1);
        // xmm4 = (04 05 06 07 14 15 16 17 24 25 26 27 34 35 36 37)

        // transpose coefficients(phase 3)
        let mut xmm7 = xmm6;
        xmm6 = _mm_unpacklo_epi32(xmm6, xmm4);
        // xmm6 = (00 01 02 03 04 05 06 07 10 11 12 13 14 15 16 17)
        xmm7 = _mm_unpackhi_epi32(xmm7, xmm4);
        // xmm7 = (20 21 22 23 24 25 26 27 30 31 32 33 34 35 36 37)

        let xmm5 = _mm_shuffle_epi32::<0x4E>(xmm6);
        // xmm5 = (10 11 12 13 14 15 16 17 00 01 02 03 04 05 06 07)
        let xmm3 = _mm_shuffle_epi32::<0x4E>(xmm7);
        // xmm3 = (30 31 32 33 34 35 36 37 20 21 22 23 24 25 26 27)

        let rdx = output_buf.row_mut(rdi + 0);
        _mm_storel_epi64(rdx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm6);
        let rbx = output_buf.row_mut(rdi + 2);
        _mm_storel_epi64(rbx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm7);
        let rdx = output_buf.row_mut(rdi + 1);
        _mm_storel_epi64(rdx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm5);
        let rbx = output_buf.row_mut(rdi + 3);
        _mm_storel_epi64(rbx[eax..].first_chunk_mut::<8>().expect(OUTPUT), xmm3);

        rsi += 4; // wsptr
        rdi += 4;
        rcx -= 1; // ctr
        if rcx == 0 {
            break;
        }
    }
}
