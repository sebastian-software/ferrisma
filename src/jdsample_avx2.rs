// libjpeg-turbo 3.2.1, simd/x86_64/jdsample-avx2.asm:
//
// ;
// ; Upsampling (64-bit AVX2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2024-2026, D. R. Commander.
// ; Copyright (C) 2015, Intel Corporation.
// ; Copyright (C) 2018, Matthias Räncker.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/x86_64/jdsample-avx2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernels serve `jdsample.c` through `simd/jsimd.c`'s entry points
//! (`jsimd.rs`), which store them in the `*_upsample_simd` slots of the
//! upsampler when the CPU has AVX2 and, for the fancy upsamplers, when
//! [`jconst_fancy_upsample_avx2`] is aligned to 32 bytes. Each kernel is AVX2
//! code, `#[target_feature(enable = "avx2")]`, which `jsimd` selects only
//! when `jpeg_simd_cpu_support` reports AVX2 (ADR 0008, decision 5).
//!
//! The translation follows ADR 0008, decision 3, as `jdsample_sse2.rs` does:
//! one intrinsic per vector instruction in the asm's order, a local variable
//! per register, `wk(i)` as the local array `wk`, the labels as loops and
//! blocks, and the asm's aligned moves as unaligned loads and stores. The
//! calling convention and its `%ifdef WIN64` branches of `jsimdext.inc`
//! (`PUSH_XMM` and `POP_XMM` save `xmm8` to `xmm10` only on Windows) are
//! dropped, and so is `vzeroupper`, which the compiler emits where an AVX
//! function returns. A VEX-encoded instruction on an `xmm` register zeroes
//! the upper half of its `ymm` register: where the asm then uses the `ymm`
//! register, `_mm256_zextsi128_si256`, which compiles to no instruction,
//! gives that value. The 128-bit instructions of the plain upsamplers
//! (`vmovdqu xmm0`, `vpunpcklbw xmm0`) are the SSE2 intrinsics, which the
//! compiler encodes with VEX in an AVX2 function.
//!
//! The rows are as in `jdsample_sse2.rs`: the output rows are the
//! upsampler's own padded rows, the fancy h2v2 kernel reads the context
//! rows through `JSAMPROWS_AT` and uses its output rows as scratch space, and
//! the fancy upsamplers insert their dummy sample into a copy of the 32-byte
//! vector that holds it ([`DummyRow`]), because the input rows are
//! read-only.

#![forbid(unsafe_code)]

#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "ADR 0008: the intrinsics by name; Clippy skips test builds"
    )
)]
use core::arch::x86_64::*;

use crate::jdsample_sse2::DummyRow;
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::{JSAMPROWS_AT, JSAMPROWS_MUT};
use crate::x86::{_mm_loadu_si128, _mm_storeu_si128, _mm256_loadu_si256, _mm256_storeu_si256};

/// `SIZEOF_XMMWORD` of `jsimdext.inc`: the bytes of an SSE register.
const SIZEOF_XMMWORD: usize = 16;

/// `SIZEOF_YMMWORD` of `jsimdext.inc`: the bytes of an AVX register.
const SIZEOF_YMMWORD: usize = 32;

/// `BYTE_BIT` of `jsimdext.inc`: `CHAR_BIT`.
const BYTE_BIT: i32 = 8;

// upstream: simd/x86_64/jdsample-avx2.asm jconst_fancy_upsample_avx2
/// The constants of the fancy upsamplers, with the asm's labels as fields
/// and its `ALIGNZ 32` alignment.
#[expect(non_snake_case, reason = "the asm's labels")]
#[expect(clippy::struct_field_names, reason = "the asm's labels")]
#[repr(C, align(32))]
pub struct JconstFancyUpsampleAvx2 {
    /// `PW_ONE`: `times 16 dw 1`.
    PW_ONE: [i16; 16],
    /// `PW_TWO`: `times 16 dw 2`.
    PW_TWO: [i16; 16],
    /// `PW_THREE`: `times 16 dw 3`.
    PW_THREE: [i16; 16],
    /// `PW_SEVEN`: `times 16 dw 7`.
    PW_SEVEN: [i16; 16],
    /// `PW_EIGHT`: `times 16 dw 8`.
    PW_EIGHT: [i16; 16],
}

// upstream: simd/x86_64/jdsample-avx2.asm jconst_fancy_upsample_avx2
/// `jconst_fancy_upsample_avx2`, which `jsimd.rs` checks for alignment.
#[expect(non_upper_case_globals, reason = "the asm's symbol")]
pub static jconst_fancy_upsample_avx2: JconstFancyUpsampleAvx2 = JconstFancyUpsampleAvx2 {
    PW_ONE: [1; 16],
    PW_TWO: [2; 16],
    PW_THREE: [3; 16],
    PW_SEVEN: [7; 16],
    PW_EIGHT: [8; 16],
};

/// The `XMMWORD` of `row` at `offset`: the 16 samples that a 128-bit load
/// or store at that column moves. Not an upstream item.
///
/// # Panics
///
/// When the row is shorter.
#[inline]
fn xmmword(row: &[JSAMPLE], offset: usize) -> &[JSAMPLE; SIZEOF_XMMWORD] {
    row[offset..]
        .first_chunk()
        .expect("row shorter than the vector")
}

/// The `XMMWORD` of `row` at `offset`, mutably; see [`xmmword`]. Not an
/// upstream item.
///
/// # Panics
///
/// When the row is shorter.
#[inline]
fn xmmword_mut(row: &mut [JSAMPLE], offset: usize) -> &mut [JSAMPLE; SIZEOF_XMMWORD] {
    row[offset..]
        .first_chunk_mut()
        .expect("row shorter than the vector")
}

/// The `YMMWORD` of `row` at `offset`: the 32 samples that a 256-bit load
/// or store at that column moves. Not an upstream item.
///
/// # Panics
///
/// When the row is shorter.
#[inline]
fn ymmword(row: &[JSAMPLE], offset: usize) -> &[JSAMPLE; SIZEOF_YMMWORD] {
    row[offset..]
        .first_chunk()
        .expect("row shorter than the vector")
}

/// The `YMMWORD` of `row` at `offset`, mutably; see [`ymmword`]. Not an
/// upstream item.
///
/// # Panics
///
/// When the row is shorter.
#[inline]
fn ymmword_mut(row: &mut [JSAMPLE], offset: usize) -> &mut [JSAMPLE; SIZEOF_YMMWORD] {
    row[offset..]
        .first_chunk_mut()
        .expect("row shorter than the vector")
}

// Fancy processing for the common case of 2:1 horizontal and 1:1 vertical.
//
// The upsampling algorithm is linear interpolation between component centers,
// also known as a "triangle filter".  This is a good compromise between speed
// and visual quality.  The centers of the output components are 1/4 and 3/4 of
// the way between input component centers.

// upstream: simd/x86_64/jdsample-avx2.asm jsimd_h2v1_fancy_upsample_avx2
/// Fancy h2v1 upsampling of `max_v_samp_factor` rows of
/// `downsampled_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// The CPU must have AVX2; `jsimd` selects the kernel only when
/// `jpeg_simd_cpu_support` reports it.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_YMMWORD` and `1 * SIZEOF_YMMWORD` offsets"
)]
#[expect(clippy::too_many_lines, reason = "the asm function")]
#[target_feature(enable = "avx2")]
pub fn jsimd_h2v1_fancy_upsample_avx2(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    let consts = &jconst_fancy_upsample_avx2;
    let mut ymm1: __m256i;
    let mut ymm2: __m256i;
    let mut ymm3: __m256i;
    let mut ymm4: __m256i;
    let mut ymm5: __m256i;
    let mut ymm6: __m256i;
    let mut ymm7: __m256i;
    let mut ymm8: __m256i;
    let mut xmm9: __m128i;

    let eax = downsampled_width as usize; // colctr
    if eax == 0 {
        return;
    }

    let mut rcx = i64::from(max_v_samp_factor); // rowctr
    if rcx == 0 {
        return;
    }

    let ymm0: __m256i = _mm256_setzero_si256(); // ymm0 = (all 0's)
    xmm9 = _mm_set1_epi8(-1);
    let xmm10: __m128i = _mm_srli_si128::<15>(xmm9);
    // (ff -- -- -- ... -- --) LSB is ff

    xmm9 = _mm_slli_si128::<15>(xmm9);
    let ymm9: __m256i =
        _mm256_permute2x128_si256::<1>(_mm256_zextsi128_si256(xmm9), _mm256_zextsi128_si256(xmm9));
    // (---- ---- ... ---- ---- ff) MSB is ff

    let mut row: usize = 0; // input_data, output_data
    // .rowloop:
    loop {
        let mut rax = eax; // colctr

        let mut inptr = DummyRow::<SIZEOF_YMMWORD>::new(input_data.row(row.cast_signed())); // inptr
        let outptr = output_data.row_mut(row); // outptr
        let mut incol: usize = 0;
        let mut outcol: usize = 0;

        if rax & (SIZEOF_YMMWORD - 1) != 0 {
            inptr.insert_dummy(rax);
            // insert a dummy sample
        }
        // .skip:
        ymm7 = _mm256_and_si256(
            _mm256_zextsi128_si256(xmm10),
            _mm256_loadu_si256(inptr.vector(incol + 0 * SIZEOF_YMMWORD)),
        );

        rax = (rax + (SIZEOF_YMMWORD - 1)) & !(SIZEOF_YMMWORD - 1);
        let mut columnloop_last = rax <= SIZEOF_YMMWORD;

        loop {
            if columnloop_last {
                // .columnloop_last:
                ymm6 = _mm256_and_si256(
                    ymm9,
                    _mm256_loadu_si256(inptr.vector(incol + 0 * SIZEOF_YMMWORD)),
                );
            } else {
                // .columnloop:
                ymm6 = _mm256_loadu_si256(inptr.vector(incol + 1 * SIZEOF_YMMWORD));
                ymm6 = _mm256_permute2x128_si256::<0x20>(ymm0, ymm6);
                ymm6 = _mm256_slli_si256::<15>(ymm6);
            }

            // .upsample:
            ymm1 = _mm256_loadu_si256(inptr.vector(incol + 0 * SIZEOF_YMMWORD));
            // ymm1 = ( 0  1  2 ... 29 30 31)

            ymm2 = _mm256_permute2x128_si256::<0x20>(ymm0, ymm1);
            ymm2 = _mm256_alignr_epi8::<15>(ymm1, ymm2); // ymm2 = (--  0  1 ... 28 29 30)
            ymm4 = _mm256_permute2x128_si256::<0x03>(ymm0, ymm1);
            ymm3 = _mm256_alignr_epi8::<1>(ymm4, ymm1); // ymm3 = ( 1  2  3 ... 30 31 --)

            ymm2 = _mm256_or_si256(ymm2, ymm7); // ymm2 = (-1  0  1 ... 28 29 30)
            ymm3 = _mm256_or_si256(ymm3, ymm6); // ymm3 = ( 1  2  3 ... 30 31 32)

            ymm7 = _mm256_srli_si256::<15>(ymm4);
            // ymm7 = (31 -- -- ... -- -- --)

            ymm4 = _mm256_unpackhi_epi8(ymm1, ymm0);
            // ymm4 = ( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
            ymm5 = _mm256_unpacklo_epi8(ymm1, ymm0);
            // ymm5 = ( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
            ymm1 = _mm256_permute2x128_si256::<0x20>(ymm5, ymm4);
            // ymm1 = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
            ymm4 = _mm256_permute2x128_si256::<0x31>(ymm5, ymm4);
            // ymm4 = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

            ymm5 = _mm256_unpackhi_epi8(ymm2, ymm0);
            // ymm5 = ( 7  8  9 10 11 12 13 14 23 24 25 26 27 28 29 30)
            ymm6 = _mm256_unpacklo_epi8(ymm2, ymm0);
            // ymm6 = (-1  0  1  2  3  4  5  6 15 16 17 18 19 20 21 22)
            ymm2 = _mm256_permute2x128_si256::<0x20>(ymm6, ymm5);
            // ymm2 = (-1  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14)
            ymm5 = _mm256_permute2x128_si256::<0x31>(ymm6, ymm5);
            // ymm5 = (15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)

            ymm6 = _mm256_unpackhi_epi8(ymm3, ymm0);
            // ymm6 = ( 1  2  3  4  5  6  7  8 17 18 19 20 21 22 23 24)
            ymm8 = _mm256_unpacklo_epi8(ymm3, ymm0);
            // ymm8 = ( 9 10 11 12 13 14 15 16 25 26 27 28 29 30 31 32)
            ymm3 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm6);
            // ymm3 = ( 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16)
            ymm6 = _mm256_permute2x128_si256::<0x31>(ymm8, ymm6);
            // ymm6 = (17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32)

            ymm1 = _mm256_mullo_epi16(ymm1, _mm256_loadu_si256(&consts.PW_THREE));
            ymm4 = _mm256_mullo_epi16(ymm4, _mm256_loadu_si256(&consts.PW_THREE));
            ymm2 = _mm256_add_epi16(ymm2, _mm256_loadu_si256(&consts.PW_ONE));
            ymm5 = _mm256_add_epi16(ymm5, _mm256_loadu_si256(&consts.PW_ONE));
            ymm3 = _mm256_add_epi16(ymm3, _mm256_loadu_si256(&consts.PW_TWO));
            ymm6 = _mm256_add_epi16(ymm6, _mm256_loadu_si256(&consts.PW_TWO));

            ymm2 = _mm256_add_epi16(ymm2, ymm1);
            ymm5 = _mm256_add_epi16(ymm5, ymm4);
            ymm2 = _mm256_srli_epi16::<2>(ymm2);
            // ymm2 = OutLE = ( 0  2  4  6  8 10 12 14 16 18 20 22 24 26 28 30)
            ymm5 = _mm256_srli_epi16::<2>(ymm5);
            // ymm5 = OutHE = (32 34 36 38 40 42 44 46 48 50 52 54 56 58 60 62)
            ymm3 = _mm256_add_epi16(ymm3, ymm1);
            ymm6 = _mm256_add_epi16(ymm6, ymm4);
            ymm3 = _mm256_srli_epi16::<2>(ymm3);
            // ymm3 = OutLO = ( 1  3  5  7  9 11 13 15 17 19 21 23 25 27 29 31)
            ymm6 = _mm256_srli_epi16::<2>(ymm6);
            // ymm6 = OutHO = (33 35 37 39 41 43 45 47 49 51 53 55 57 59 61 63)

            ymm3 = _mm256_slli_epi16::<BYTE_BIT>(ymm3);
            ymm6 = _mm256_slli_epi16::<BYTE_BIT>(ymm6);
            ymm2 = _mm256_or_si256(ymm2, ymm3); // ymm2 = OutL = ( 0  1  2 ... 29 30 31)
            ymm5 = _mm256_or_si256(ymm5, ymm6); // ymm5 = OutH = (32 33 34 ... 61 62 63)

            _mm256_storeu_si256(ymmword_mut(outptr, outcol + 0 * SIZEOF_YMMWORD), ymm2);
            _mm256_storeu_si256(ymmword_mut(outptr, outcol + 1 * SIZEOF_YMMWORD), ymm5);

            rax -= SIZEOF_YMMWORD;
            incol += 1 * SIZEOF_YMMWORD; // inptr
            outcol += 2 * SIZEOF_YMMWORD; // outptr
            if rax > SIZEOF_YMMWORD {
                columnloop_last = false;
            } else if rax != 0 {
                columnloop_last = true;
            } else {
                break;
            }
        }

        row += 1; // input_data, output_data
        rcx -= 1; // rowctr
        if rcx <= 0 {
            break;
        }
    }
    // .return:
}

// Fancy processing for the common case of 2:1 horizontal and 2:1 vertical.
// Again a triangle filter; see comments for h2v1 case, above.

// upstream: simd/x86_64/jdsample-avx2.asm jsimd_h2v2_fancy_upsample_avx2
/// Fancy h2v2 upsampling of `max_v_samp_factor` output rows from the input
/// rows and the context rows above and below them.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes, or a context row
/// lies outside the input's row list.
///
/// # Safety
///
/// The CPU must have AVX2; `jsimd` selects the kernel only when
/// `jpeg_simd_cpu_support` reports it.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_YMMWORD` and `1 * SIZEOF_YMMWORD` offsets"
)]
#[expect(clippy::too_many_lines, reason = "the asm function")]
#[target_feature(enable = "avx2")]
pub fn jsimd_h2v2_fancy_upsample_avx2(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    const WK_NUM: usize = 4;
    let mut output_data = output_data_ptr;
    let consts = &jconst_fancy_upsample_avx2;
    let mut wk: [__m256i; WK_NUM] = [_mm256_setzero_si256(); WK_NUM];
    let mut ymm0: __m256i;
    let mut ymm1: __m256i;
    let mut ymm2: __m256i;
    let mut ymm3: __m256i;
    let mut ymm4: __m256i;
    let mut ymm5: __m256i;
    let mut ymm6: __m256i;
    let mut ymm7: __m256i;
    let mut ymm8: __m256i;
    let mut xmm9: __m128i;
    let mut ymm9: __m256i;
    let mut xmm10: __m128i;

    let eax = downsampled_width as usize; // colctr
    if eax == 0 {
        return;
    }

    let mut rcx = i64::from(max_v_samp_factor); // rowctr
    if rcx == 0 {
        return;
    }

    let mut inrow: isize = 0; // input_data
    let mut outrow: usize = 0; // output_data
    // .rowloop:
    loop {
        let mut rax = eax; // colctr

        let mut inptr1_above = DummyRow::<SIZEOF_YMMWORD>::new(input_data.row(inrow - 1)); // inptr1(above)
        let mut inptr0 = DummyRow::<SIZEOF_YMMWORD>::new(input_data.row(inrow)); // inptr0
        let mut inptr1_below = DummyRow::<SIZEOF_YMMWORD>::new(input_data.row(inrow + 1)); // inptr1(below)
        let (outptr0, outptr1) = output_data.two_rows_mut(outrow); // outptr0, outptr1
        let mut incol: usize = 0;
        let mut outcol: usize = 0;

        ymm8 = _mm256_setzero_si256(); // ymm8 = (all 0's)
        xmm9 = _mm_set1_epi8(-1);
        xmm10 = _mm_srli_si128::<14>(xmm9);
        // (ffff ---- ---- ... ---- ----) LSB is ffff
        xmm9 = _mm_slli_si128::<14>(xmm9);
        ymm9 = _mm256_permute2x128_si256::<1>(
            _mm256_zextsi128_si256(xmm9),
            _mm256_zextsi128_si256(xmm9),
        );
        // (---- ---- ... ---- ---- ffff) MSB is ffff

        if rax & (SIZEOF_YMMWORD - 1) != 0 {
            inptr1_above.insert_dummy(rax);
            inptr0.insert_dummy(rax);
            inptr1_below.insert_dummy(rax);
            // insert a dummy sample
        }
        // .skip:
        // -- process the first column block

        ymm0 = _mm256_loadu_si256(inptr0.vector(incol + 0 * SIZEOF_YMMWORD)); // ymm0 = row[ 0][0]
        ymm1 = _mm256_loadu_si256(inptr1_above.vector(incol + 0 * SIZEOF_YMMWORD)); // ymm1 = row[-1][0]
        ymm2 = _mm256_loadu_si256(inptr1_below.vector(incol + 0 * SIZEOF_YMMWORD)); // ymm2 = row[+1][0]

        ymm4 = _mm256_unpackhi_epi8(ymm0, ymm8);
        // ymm4 = row[ 0]( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
        ymm5 = _mm256_unpacklo_epi8(ymm0, ymm8);
        // ymm5 = row[ 0]( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
        ymm0 = _mm256_permute2x128_si256::<0x20>(ymm5, ymm4);
        // ymm0 = row[ 0]( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
        ymm4 = _mm256_permute2x128_si256::<0x31>(ymm5, ymm4);
        // ymm4 = row[ 0](16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

        ymm5 = _mm256_unpackhi_epi8(ymm1, ymm8);
        // ymm5 = row[-1]( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
        ymm6 = _mm256_unpacklo_epi8(ymm1, ymm8);
        // ymm6 = row[-1]( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
        ymm1 = _mm256_permute2x128_si256::<0x20>(ymm6, ymm5);
        // ymm1 = row[-1]( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
        ymm5 = _mm256_permute2x128_si256::<0x31>(ymm6, ymm5);
        // ymm5 = row[-1](16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

        ymm6 = _mm256_unpackhi_epi8(ymm2, ymm8);
        // ymm6 = row[+1]( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
        ymm3 = _mm256_unpacklo_epi8(ymm2, ymm8);
        // ymm3 = row[+1]( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
        ymm2 = _mm256_permute2x128_si256::<0x20>(ymm3, ymm6);
        // ymm2 = row[+1]( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
        ymm6 = _mm256_permute2x128_si256::<0x31>(ymm3, ymm6);
        // ymm6 = row[+1](16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

        ymm0 = _mm256_mullo_epi16(ymm0, _mm256_loadu_si256(&consts.PW_THREE));
        ymm4 = _mm256_mullo_epi16(ymm4, _mm256_loadu_si256(&consts.PW_THREE));

        ymm1 = _mm256_add_epi16(ymm1, ymm0);
        // ymm1 = Int0L = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
        ymm5 = _mm256_add_epi16(ymm5, ymm4);
        // ymm5 = Int0H = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
        ymm2 = _mm256_add_epi16(ymm2, ymm0);
        // ymm2 = Int1L = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
        ymm6 = _mm256_add_epi16(ymm6, ymm4);
        // ymm6 = Int1H = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

        // temporarily save the intermediate data
        _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 0 * SIZEOF_YMMWORD), ymm1);
        _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 1 * SIZEOF_YMMWORD), ymm5);
        _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 0 * SIZEOF_YMMWORD), ymm2);
        _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 1 * SIZEOF_YMMWORD), ymm6);

        ymm1 = _mm256_and_si256(ymm1, _mm256_zextsi128_si256(xmm10));
        // ymm1 = ( 0 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --)
        ymm2 = _mm256_and_si256(ymm2, _mm256_zextsi128_si256(xmm10));
        // ymm2 = ( 0 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --)

        wk[0] = ymm1;
        wk[1] = ymm2;

        rax = (rax + (SIZEOF_YMMWORD - 1)) & !(SIZEOF_YMMWORD - 1);
        let mut columnloop_last = rax <= SIZEOF_YMMWORD;

        loop {
            if columnloop_last {
                // .columnloop_last:
                // -- process the last column block

                ymm1 = _mm256_and_si256(
                    ymm9,
                    _mm256_loadu_si256(ymmword(outptr0, outcol + 1 * SIZEOF_YMMWORD)),
                );
                ymm2 = _mm256_and_si256(
                    ymm9,
                    _mm256_loadu_si256(ymmword(outptr1, outcol + 1 * SIZEOF_YMMWORD)),
                );

                wk[2] = ymm1;
                // ymm1 = (-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 31)
                wk[3] = ymm2;
                // ymm2 = (-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 31)
            } else {
                // .columnloop:
                // -- process the next column block

                ymm0 = _mm256_loadu_si256(inptr0.vector(incol + 1 * SIZEOF_YMMWORD)); // ymm0 = row[ 0][1]
                ymm1 = _mm256_loadu_si256(inptr1_above.vector(incol + 1 * SIZEOF_YMMWORD)); // ymm1 = row[-1][1]
                ymm2 = _mm256_loadu_si256(inptr1_below.vector(incol + 1 * SIZEOF_YMMWORD)); // ymm2 = row[+1][1]

                ymm4 = _mm256_unpackhi_epi8(ymm0, ymm8);
                // ymm4 = row[ 0]( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
                ymm5 = _mm256_unpacklo_epi8(ymm0, ymm8);
                // ymm5 = row[ 0]( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
                ymm0 = _mm256_permute2x128_si256::<0x20>(ymm5, ymm4);
                // ymm0 = row[ 0]( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
                ymm4 = _mm256_permute2x128_si256::<0x31>(ymm5, ymm4);
                // ymm4 = row[ 0](16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

                ymm5 = _mm256_unpackhi_epi8(ymm1, ymm8);
                // ymm5 = row[-1]( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
                ymm6 = _mm256_unpacklo_epi8(ymm1, ymm8);
                // ymm6 = row[-1]( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
                ymm1 = _mm256_permute2x128_si256::<0x20>(ymm6, ymm5);
                // ymm1 = row[-1]( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
                ymm5 = _mm256_permute2x128_si256::<0x31>(ymm6, ymm5);
                // ymm5 = row[-1](16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

                ymm6 = _mm256_unpackhi_epi8(ymm2, ymm8);
                // ymm6 = row[+1]( 8  9 10 11 12 13 14 15 24 25 26 27 28 29 30 31)
                ymm7 = _mm256_unpacklo_epi8(ymm2, ymm8);
                // ymm7 = row[+1]( 0  1  2  3  4  5  6  7 16 17 18 19 20 21 22 23)
                ymm2 = _mm256_permute2x128_si256::<0x20>(ymm7, ymm6);
                // ymm2 = row[+1]( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
                ymm6 = _mm256_permute2x128_si256::<0x31>(ymm7, ymm6);
                // ymm6 = row[+1](16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

                ymm0 = _mm256_mullo_epi16(ymm0, _mm256_loadu_si256(&consts.PW_THREE));
                ymm4 = _mm256_mullo_epi16(ymm4, _mm256_loadu_si256(&consts.PW_THREE));

                ymm1 = _mm256_add_epi16(ymm1, ymm0);
                // ymm1 = Int0L = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
                ymm5 = _mm256_add_epi16(ymm5, ymm4);
                // ymm5 = Int0H = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
                ymm2 = _mm256_add_epi16(ymm2, ymm0);
                // ymm2 = Int1L = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
                ymm6 = _mm256_add_epi16(ymm6, ymm4);
                // ymm6 = Int1H = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

                // temporarily save the intermediate data
                _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 2 * SIZEOF_YMMWORD), ymm1);
                _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 3 * SIZEOF_YMMWORD), ymm5);
                _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 2 * SIZEOF_YMMWORD), ymm2);
                _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 3 * SIZEOF_YMMWORD), ymm6);

                ymm1 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm1);
                ymm1 = _mm256_slli_si256::<14>(ymm1);
                // ymm1 = (-- -- -- -- -- -- -- -- -- -- -- -- -- -- --  0)
                ymm2 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm2);
                ymm2 = _mm256_slli_si256::<14>(ymm2);
                // ymm2 = (-- -- -- -- -- -- -- -- -- -- -- -- -- -- --  0)

                wk[2] = ymm1;
                wk[3] = ymm2;
            }

            // .upsample:
            // -- process the upper row

            ymm7 = _mm256_loadu_si256(ymmword(outptr0, outcol + 0 * SIZEOF_YMMWORD));
            // ymm7 = Int0L = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
            ymm3 = _mm256_loadu_si256(ymmword(outptr0, outcol + 1 * SIZEOF_YMMWORD));
            // ymm3 = Int0H = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

            ymm0 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm7);
            ymm0 = _mm256_alignr_epi8::<2>(ymm0, ymm7);
            // ymm0 = ( 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 --)
            ymm4 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm3);
            ymm4 = _mm256_slli_si256::<14>(ymm4);
            // ymm4 = (-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 16)

            ymm5 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm7);
            ymm5 = _mm256_srli_si256::<14>(ymm5);
            // ymm5 = (15 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --)
            ymm6 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm3);
            ymm6 = _mm256_alignr_epi8::<14>(ymm3, ymm6);
            // ymm6 = (-- 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)

            ymm0 = _mm256_or_si256(ymm0, ymm4);
            // ymm0 = ( 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16)
            ymm5 = _mm256_or_si256(ymm5, ymm6);
            // ymm5 = (15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)

            ymm2 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm3);
            ymm2 = _mm256_alignr_epi8::<2>(ymm2, ymm3);
            // ymm2 = (17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 --)
            ymm4 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm3);
            ymm4 = _mm256_srli_si256::<14>(ymm4);
            // ymm4 = (31 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --)
            ymm1 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm7);
            ymm1 = _mm256_alignr_epi8::<14>(ymm7, ymm1);
            // ymm1 = (--  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14)

            ymm1 = _mm256_or_si256(ymm1, wk[0]);
            // ymm1 = (-1  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14)
            ymm2 = _mm256_or_si256(ymm2, wk[2]);
            // ymm2 = (17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32)

            wk[0] = ymm4;

            ymm7 = _mm256_mullo_epi16(ymm7, _mm256_loadu_si256(&consts.PW_THREE));
            ymm3 = _mm256_mullo_epi16(ymm3, _mm256_loadu_si256(&consts.PW_THREE));
            ymm1 = _mm256_add_epi16(ymm1, _mm256_loadu_si256(&consts.PW_EIGHT));
            ymm5 = _mm256_add_epi16(ymm5, _mm256_loadu_si256(&consts.PW_EIGHT));
            ymm0 = _mm256_add_epi16(ymm0, _mm256_loadu_si256(&consts.PW_SEVEN));
            ymm2 = _mm256_add_epi16(ymm2, _mm256_loadu_si256(&consts.PW_SEVEN));

            ymm1 = _mm256_add_epi16(ymm1, ymm7);
            ymm5 = _mm256_add_epi16(ymm5, ymm3);
            ymm1 = _mm256_srli_epi16::<4>(ymm1);
            // ymm1 = Out0LE = ( 0  2  4  6  8 10 12 14 16 18 20 22 24 26 28 30)
            ymm5 = _mm256_srli_epi16::<4>(ymm5);
            // ymm5 = Out0HE = (32 34 36 38 40 42 44 46 48 50 52 54 56 58 60 62)
            ymm0 = _mm256_add_epi16(ymm0, ymm7);
            ymm2 = _mm256_add_epi16(ymm2, ymm3);
            ymm0 = _mm256_srli_epi16::<4>(ymm0);
            // ymm0 = Out0LO = ( 1  3  5  7  9 11 13 15 17 19 21 23 25 27 29 31)
            ymm2 = _mm256_srli_epi16::<4>(ymm2);
            // ymm2 = Out0HO = (33 35 37 39 41 43 45 47 49 51 53 55 57 59 61 63)

            ymm0 = _mm256_slli_epi16::<BYTE_BIT>(ymm0);
            ymm2 = _mm256_slli_epi16::<BYTE_BIT>(ymm2);
            ymm1 = _mm256_or_si256(ymm1, ymm0); // ymm1 = Out0L = ( 0  1  2 ... 29 30 31)
            ymm5 = _mm256_or_si256(ymm5, ymm2); // ymm5 = Out0H = (32 33 34 ... 61 62 63)

            _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 0 * SIZEOF_YMMWORD), ymm1);
            _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 1 * SIZEOF_YMMWORD), ymm5);

            // -- process the lower row

            ymm6 = _mm256_loadu_si256(ymmword(outptr1, outcol + 0 * SIZEOF_YMMWORD));
            // ymm6 = Int1L = ( 0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15)
            ymm4 = _mm256_loadu_si256(ymmword(outptr1, outcol + 1 * SIZEOF_YMMWORD));
            // ymm4 = Int1H = (16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)

            ymm7 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm6);
            ymm7 = _mm256_alignr_epi8::<2>(ymm7, ymm6);
            // ymm7 = ( 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 --)
            ymm3 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm4);
            ymm3 = _mm256_slli_si256::<14>(ymm3);
            // ymm3 = (-- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 16)

            ymm0 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm6);
            ymm0 = _mm256_srli_si256::<14>(ymm0);
            // ymm0 = (15 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --)
            ymm2 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm4);
            ymm2 = _mm256_alignr_epi8::<14>(ymm4, ymm2);
            // ymm2 = (-- 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)

            ymm7 = _mm256_or_si256(ymm7, ymm3);
            // ymm7 = ( 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16)
            ymm0 = _mm256_or_si256(ymm0, ymm2);
            // ymm0 = (15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)

            ymm5 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm4);
            ymm5 = _mm256_alignr_epi8::<2>(ymm5, ymm4);
            // ymm5 = (17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 --)
            ymm3 = _mm256_permute2x128_si256::<0x03>(ymm8, ymm4);
            ymm3 = _mm256_srli_si256::<14>(ymm3);
            // ymm3 = (31 -- -- -- -- -- -- -- -- -- -- -- -- -- -- --)
            ymm1 = _mm256_permute2x128_si256::<0x20>(ymm8, ymm6);
            ymm1 = _mm256_alignr_epi8::<14>(ymm6, ymm1);
            // ymm1 = (--  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14)

            ymm1 = _mm256_or_si256(ymm1, wk[1]);
            // ymm1 = (-1  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14)
            ymm5 = _mm256_or_si256(ymm5, wk[3]);
            // ymm5 = (17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32)

            wk[1] = ymm3;

            ymm6 = _mm256_mullo_epi16(ymm6, _mm256_loadu_si256(&consts.PW_THREE));
            ymm4 = _mm256_mullo_epi16(ymm4, _mm256_loadu_si256(&consts.PW_THREE));
            ymm1 = _mm256_add_epi16(ymm1, _mm256_loadu_si256(&consts.PW_EIGHT));
            ymm0 = _mm256_add_epi16(ymm0, _mm256_loadu_si256(&consts.PW_EIGHT));
            ymm7 = _mm256_add_epi16(ymm7, _mm256_loadu_si256(&consts.PW_SEVEN));
            ymm5 = _mm256_add_epi16(ymm5, _mm256_loadu_si256(&consts.PW_SEVEN));

            ymm1 = _mm256_add_epi16(ymm1, ymm6);
            ymm0 = _mm256_add_epi16(ymm0, ymm4);
            ymm1 = _mm256_srli_epi16::<4>(ymm1);
            // ymm1 = Out1LE = ( 0  2  4  6  8 10 12 14 16 18 20 22 24 26 28 30)
            ymm0 = _mm256_srli_epi16::<4>(ymm0);
            // ymm0 = Out1HE = (32 34 36 38 40 42 44 46 48 50 52 54 56 58 60 62)
            ymm7 = _mm256_add_epi16(ymm7, ymm6);
            ymm5 = _mm256_add_epi16(ymm5, ymm4);
            ymm7 = _mm256_srli_epi16::<4>(ymm7);
            // ymm7 = Out1LO = ( 1  3  5  7  9 11 13 15 17 19 21 23 25 27 29 31)
            ymm5 = _mm256_srli_epi16::<4>(ymm5);
            // ymm5 = Out1HO = (33 35 37 39 41 43 45 47 49 51 53 55 57 59 61 63)

            ymm7 = _mm256_slli_epi16::<BYTE_BIT>(ymm7);
            ymm5 = _mm256_slli_epi16::<BYTE_BIT>(ymm5);
            ymm1 = _mm256_or_si256(ymm1, ymm7); // ymm1 = Out1L = ( 0  1  2 ... 29 30 31)
            ymm0 = _mm256_or_si256(ymm0, ymm5); // ymm0 = Out1H = (32 33 34 ... 61 62 63)

            _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 0 * SIZEOF_YMMWORD), ymm1);
            _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 1 * SIZEOF_YMMWORD), ymm0);

            rax -= SIZEOF_YMMWORD;
            incol += 1 * SIZEOF_YMMWORD; // inptr1(above), inptr0, inptr1(below)
            outcol += 2 * SIZEOF_YMMWORD; // outptr0, outptr1
            if rax > SIZEOF_YMMWORD {
                columnloop_last = false;
            } else if rax != 0 {
                columnloop_last = true;
            } else {
                break;
            }
        }

        inrow += 1; // input_data
        outrow += 2; // output_data
        rcx -= 2; // rowctr
        if rcx <= 0 {
            break;
        }
    }
    // .return:
}

// Fast processing for the common case of 2:1 horizontal and 1:1 vertical.
// It's still a box filter.

// upstream: simd/x86_64/jdsample-avx2.asm jsimd_h2v1_upsample_avx2
/// Plain h2v1 upsampling of `max_v_samp_factor` rows into rows of
/// `output_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// The CPU must have AVX2; `jsimd` selects the kernel only when
/// `jpeg_simd_cpu_support` reports it.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_YMMWORD` and `1 * SIZEOF_XMMWORD` offsets"
)]
#[target_feature(enable = "avx2")]
pub fn jsimd_h2v1_upsample_avx2(
    max_v_samp_factor: i32,
    output_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut ymm0: __m256i;
    let mut ymm1: __m256i;

    let mut rdx = output_width as usize;
    rdx = (rdx + (SIZEOF_YMMWORD - 1)) & !(SIZEOF_YMMWORD - 1);
    if rdx == 0 {
        return;
    }

    let mut rcx = i64::from(max_v_samp_factor); // rowctr
    if rcx == 0 {
        return;
    }

    let mut row: usize = 0; // input_data, output_data
    // .rowloop:
    loop {
        let inptr = input_data.row(row.cast_signed()); // inptr
        let outptr = output_data.row_mut(row); // outptr
        let mut incol: usize = 0;
        let mut outcol: usize = 0;
        let mut rax = rdx; // colctr
        // .columnloop:
        loop {
            if rax <= SIZEOF_YMMWORD {
                xmm0 = _mm_loadu_si128(xmmword(inptr, incol + 0 * SIZEOF_YMMWORD));
                xmm1 = _mm_unpackhi_epi8(xmm0, xmm0);
                xmm0 = _mm_unpacklo_epi8(xmm0, xmm0);

                _mm_storeu_si128(xmmword_mut(outptr, outcol + 0 * SIZEOF_XMMWORD), xmm0);
                _mm_storeu_si128(xmmword_mut(outptr, outcol + 1 * SIZEOF_XMMWORD), xmm1);

                break;
            }

            // .above_16:
            ymm0 = _mm256_loadu_si256(ymmword(inptr, incol + 0 * SIZEOF_YMMWORD));

            ymm0 = _mm256_permute4x64_epi64::<0xd8>(ymm0);
            ymm1 = _mm256_unpackhi_epi8(ymm0, ymm0);
            ymm0 = _mm256_unpacklo_epi8(ymm0, ymm0);

            _mm256_storeu_si256(ymmword_mut(outptr, outcol + 0 * SIZEOF_YMMWORD), ymm0);
            _mm256_storeu_si256(ymmword_mut(outptr, outcol + 1 * SIZEOF_YMMWORD), ymm1);

            rax -= 2 * SIZEOF_YMMWORD;
            if rax == 0 {
                break;
            }

            incol += SIZEOF_YMMWORD; // inptr
            outcol += 2 * SIZEOF_YMMWORD; // outptr
        }

        // .nextrow:
        row += 1; // input_data, output_data
        rcx -= 1; // rowctr
        if rcx <= 0 {
            break;
        }
    }
    // .return:
}

// Fast processing for the common case of 2:1 horizontal and 2:1 vertical.
// It's still a box filter.

// upstream: simd/x86_64/jdsample-avx2.asm jsimd_h2v2_upsample_avx2
/// Plain h2v2 upsampling of `max_v_samp_factor` output rows of
/// `output_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// The CPU must have AVX2; `jsimd` selects the kernel only when
/// `jpeg_simd_cpu_support` reports it.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_XMMWORD` and `1 * SIZEOF_XMMWORD` offsets"
)]
#[target_feature(enable = "avx2")]
pub fn jsimd_h2v2_upsample_avx2(
    max_v_samp_factor: i32,
    output_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut ymm0: __m256i;
    let mut ymm1: __m256i;

    let mut rdx = output_width as usize;
    rdx = (rdx + (SIZEOF_YMMWORD - 1)) & !(SIZEOF_YMMWORD - 1);
    if rdx == 0 {
        return;
    }

    let mut rcx = i64::from(max_v_samp_factor); // rowctr
    if rcx == 0 {
        return;
    }

    let mut inrow: isize = 0; // input_data
    let mut outrow: usize = 0; // output_data
    // .rowloop:
    loop {
        let inptr = input_data.row(inrow); // inptr
        let (outptr0, outptr1) = output_data.two_rows_mut(outrow); // outptr0, outptr1
        let mut incol: usize = 0;
        let mut outcol: usize = 0;
        let mut rax = rdx; // colctr
        // .columnloop:
        loop {
            if rax <= SIZEOF_YMMWORD {
                xmm0 = _mm_loadu_si128(xmmword(inptr, incol + 0 * SIZEOF_XMMWORD));
                xmm1 = _mm_unpackhi_epi8(xmm0, xmm0);
                xmm0 = _mm_unpacklo_epi8(xmm0, xmm0);

                _mm_storeu_si128(xmmword_mut(outptr0, outcol + 0 * SIZEOF_XMMWORD), xmm0);
                _mm_storeu_si128(xmmword_mut(outptr0, outcol + 1 * SIZEOF_XMMWORD), xmm1);
                _mm_storeu_si128(xmmword_mut(outptr1, outcol + 0 * SIZEOF_XMMWORD), xmm0);
                _mm_storeu_si128(xmmword_mut(outptr1, outcol + 1 * SIZEOF_XMMWORD), xmm1);

                break;
            }

            // .above_16:
            ymm0 = _mm256_loadu_si256(ymmword(inptr, incol + 0 * SIZEOF_YMMWORD));

            ymm0 = _mm256_permute4x64_epi64::<0xd8>(ymm0);
            ymm1 = _mm256_unpackhi_epi8(ymm0, ymm0);
            ymm0 = _mm256_unpacklo_epi8(ymm0, ymm0);

            _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 0 * SIZEOF_YMMWORD), ymm0);
            _mm256_storeu_si256(ymmword_mut(outptr0, outcol + 1 * SIZEOF_YMMWORD), ymm1);
            _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 0 * SIZEOF_YMMWORD), ymm0);
            _mm256_storeu_si256(ymmword_mut(outptr1, outcol + 1 * SIZEOF_YMMWORD), ymm1);

            rax -= 2 * SIZEOF_YMMWORD;
            if rax == 0 {
                break;
            }

            incol += SIZEOF_YMMWORD; // inptr
            outcol += 2 * SIZEOF_YMMWORD; // outptr0, outptr1
        }

        // .nextrow:
        inrow += 1; // input_data
        outrow += 2; // output_data
        rcx -= 2; // rowctr
        if rcx <= 0 {
            break;
        }
    }
    // .return:
}
