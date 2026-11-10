// libjpeg-turbo 3.2.1, simd/x86_64/jdsample-sse2.asm:
//
// ;
// ; Upsampling (64-bit SSE2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2024-2026, D. R. Commander.
// ; Copyright (C) 2018, Matthias Räncker.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/x86_64/jdsample-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! The kernels serve `jdsample.c` through `simd/jsimd.c`'s entry points
//! (`jsimd.rs`), which store them in the `*_upsample_simd` slots of the
//! upsampler when the CPU has SSE2 and, for the fancy upsamplers, when
//! [`jconst_fancy_upsample_sse2`] is aligned to 16 bytes. Each kernel is SSE2
//! code, `#[target_feature(enable = "sse2")]`, which every x86-64 target
//! enables.
//!
//! The translation follows ADR 0008, decision 3: one intrinsic per vector
//! instruction in the asm's order, a local variable per register, the stack
//! slots `wk(i)` as the local array `wk`, and the labels as loops and blocks
//! named in comments. The calling convention (`COLLECT_ARGS`, the pushes and
//! pops, `ENDBR64`) becomes the parameters, with its `%ifdef WIN64` branches
//! of `jsimdext.inc` dropped; the row pointers become row slices with column
//! offsets, which advance where the asm advances its pointers. `pcmpeqb x, x`
//! and `pxor x, x` are the constant intrinsics, and the asm's aligned moves
//! are unaligned loads and stores (ADR 0008, decision 4).
//!
//! As in `jdsample_neon.rs`, a kernel receives the output rows as a row list:
//! the upsampler's own rows from `alloc_sarray`, which the kernels write in
//! whole vectors up to their padding, and the h2v2 kernels take two output
//! rows at once (`two_rows_mut`). The fancy h2v2 kernel reads the context rows
//! above and below its input rows through `JSAMPROWS_AT` and uses its output
//! rows as scratch space for the column sums, as the asm does. The loads take
//! arrays from row slices with checked conversions, which would panic instead
//! of reaching past the storage (ADR 0007, decision 4).
//!
//! The fancy upsamplers' "insert a dummy sample" stores the last sample of
//! an input row once more after it, when the row's width is not a multiple
//! of the vector, so that the last vector of the row ends the row with that
//! sample. The port's input rows are read-only (`JSAMPROWS_AT`), so the
//! kernel writes the dummy sample into a copy of the vector that holds it
//! ([`DummyRow`]) and loads that vector from the copy. The loads see the same
//! samples as the asm's. The sample that the asm leaves in the row's padding
//! is not written; the decoder reads it only through these kernels, which
//! insert it again first.

#![forbid(unsafe_code)]

#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "ADR 0008: the intrinsics by name; Clippy skips test builds"
    )
)]
use core::arch::x86_64::*;

use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::{JSAMPROWS_AT, JSAMPROWS_MUT};
use crate::x86::{_mm_loadu_si128, _mm_storeu_si128};

/// `SIZEOF_XMMWORD` of `jsimdext.inc`: the bytes of an SSE register.
const SIZEOF_XMMWORD: usize = 16;

/// `BYTE_BIT` of `jsimdext.inc`: `CHAR_BIT`.
const BYTE_BIT: i32 = 8;

// upstream: simd/x86_64/jdsample-sse2.asm jconst_fancy_upsample_sse2
/// The constants of the fancy upsamplers, with the asm's labels as fields
/// and its `ALIGNZ 32` alignment.
#[expect(non_snake_case, reason = "the asm's labels")]
#[expect(clippy::struct_field_names, reason = "the asm's labels")]
#[repr(C, align(32))]
pub struct JconstFancyUpsampleSse2 {
    /// `PW_ONE`: `times 8 dw 1`.
    PW_ONE: [i16; 8],
    /// `PW_TWO`: `times 8 dw 2`.
    PW_TWO: [i16; 8],
    /// `PW_THREE`: `times 8 dw 3`.
    PW_THREE: [i16; 8],
    /// `PW_SEVEN`: `times 8 dw 7`.
    PW_SEVEN: [i16; 8],
    /// `PW_EIGHT`: `times 8 dw 8`.
    PW_EIGHT: [i16; 8],
}

// upstream: simd/x86_64/jdsample-sse2.asm jconst_fancy_upsample_sse2
/// `jconst_fancy_upsample_sse2`, which `jsimd.rs` checks for alignment.
#[expect(non_upper_case_globals, reason = "the asm's symbol")]
pub static jconst_fancy_upsample_sse2: JconstFancyUpsampleSse2 = JconstFancyUpsampleSse2 {
    PW_ONE: [1; 8],
    PW_TWO: [2; 8],
    PW_THREE: [3; 8],
    PW_SEVEN: [7; 8],
    PW_EIGHT: [8; 8],
};

/// The `XMMWORD` of `row` at `offset`: the 16 samples that a vector load or
/// store at that column moves. Not an upstream item.
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

/// An input row of a fancy upsampler into which the kernel inserts the
/// asm's dummy sample, read in vectors of `N` samples at multiples of `N`
/// (see the module documentation). It holds a copy of the vector that
/// contains the dummy sample, from which that vector loads. Not an upstream
/// item; `jdsample_avx2.rs` uses it with `N` = 32.
pub(crate) struct DummyRow<'a, const N: usize> {
    /// The input row, to the end of its storage.
    row: &'a [JSAMPLE],
    /// The column of the copied vector, or `usize::MAX` before a dummy
    /// sample is inserted.
    dummy_col: usize,
    /// The vector at `dummy_col` with the dummy sample.
    dummy: [JSAMPLE; N],
}

impl<'a, const N: usize> DummyRow<'a, N> {
    /// The row without a dummy sample.
    #[inline]
    pub(crate) fn new(row: &'a [JSAMPLE]) -> Self {
        DummyRow {
            row,
            dummy_col: usize::MAX,
            dummy: [0; N],
        }
    }

    /// `mov dl, JSAMPLE [rsi + (rax - 1) * SIZEOF_JSAMPLE]` and
    /// `mov JSAMPLE [rsi + rax * SIZEOF_JSAMPLE], dl`: sample `colctr` takes
    /// the value of sample `colctr - 1`.
    ///
    /// # Panics
    ///
    /// When `colctr` is 0 or the row is shorter than the vector that holds
    /// sample `colctr`.
    #[inline]
    pub(crate) fn insert_dummy(&mut self, colctr: usize) {
        let col = colctr - colctr % N;
        self.dummy = *self.row[col..]
            .first_chunk()
            .expect("input row shorter than the vector");
        self.dummy[colctr - col] = self.row[colctr - 1];
        self.dummy_col = col;
    }

    /// The `N` samples at column `col`, a multiple of `N`, with the dummy
    /// sample in place.
    ///
    /// # Panics
    ///
    /// When the row is shorter.
    #[inline]
    pub(crate) fn vector(&self, col: usize) -> &[JSAMPLE; N] {
        if col == self.dummy_col {
            &self.dummy
        } else {
            self.row[col..]
                .first_chunk()
                .expect("input row shorter than the vector")
        }
    }
}

// Fancy processing for the common case of 2:1 horizontal and 1:1 vertical.
//
// The upsampling algorithm is linear interpolation between component centers,
// also known as a "triangle filter".  This is a good compromise between speed
// and visual quality.  The centers of the output components are 1/4 and 3/4 of
// the way between input component centers.

// upstream: simd/x86_64/jdsample-sse2.asm jsimd_h2v1_fancy_upsample_sse2
/// Fancy h2v1 upsampling of `max_v_samp_factor` rows of
/// `downsampled_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_XMMWORD` and `1 * SIZEOF_XMMWORD` offsets"
)]
#[expect(clippy::too_many_lines, reason = "the asm function")]
#[target_feature(enable = "sse2")]
pub fn jsimd_h2v1_fancy_upsample_sse2(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    let consts = &jconst_fancy_upsample_sse2;
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut xmm2: __m128i;
    let mut xmm3: __m128i;
    let mut xmm4: __m128i;
    let mut xmm5: __m128i;
    let mut xmm6: __m128i;
    let mut xmm7: __m128i;

    let eax = downsampled_width as usize; // colctr
    if eax == 0 {
        return;
    }

    let mut rcx = i64::from(max_v_samp_factor); // rowctr
    if rcx == 0 {
        return;
    }

    let mut row: usize = 0; // input_data, output_data
    // .rowloop:
    loop {
        let mut rax = eax; // colctr

        let mut inptr = DummyRow::<SIZEOF_XMMWORD>::new(input_data.row(row.cast_signed())); // inptr
        let outptr = output_data.row_mut(row); // outptr
        let mut incol: usize = 0;
        let mut outcol: usize = 0;

        if rax & (SIZEOF_XMMWORD - 1) != 0 {
            inptr.insert_dummy(rax);
            // insert a dummy sample
        }
        // .skip:
        xmm0 = _mm_setzero_si128(); // xmm0 = (all 0's)
        xmm7 = _mm_set1_epi8(-1);
        xmm7 = _mm_srli_si128::<15>(xmm7);
        xmm7 = _mm_and_si128(
            xmm7,
            _mm_loadu_si128(inptr.vector(incol + 0 * SIZEOF_XMMWORD)),
        );

        rax = (rax + (SIZEOF_XMMWORD - 1)) & !(SIZEOF_XMMWORD - 1);
        let mut columnloop_last = rax <= SIZEOF_XMMWORD;

        loop {
            if columnloop_last {
                // .columnloop_last:
                xmm6 = _mm_set1_epi8(-1);
                xmm6 = _mm_slli_si128::<15>(xmm6);
                xmm6 = _mm_and_si128(
                    xmm6,
                    _mm_loadu_si128(inptr.vector(incol + 0 * SIZEOF_XMMWORD)),
                );
            } else {
                // .columnloop:
                xmm6 = _mm_loadu_si128(inptr.vector(incol + 1 * SIZEOF_XMMWORD));
                xmm6 = _mm_slli_si128::<15>(xmm6);
            }

            // .upsample:
            xmm1 = _mm_loadu_si128(inptr.vector(incol + 0 * SIZEOF_XMMWORD));
            xmm2 = xmm1;
            xmm3 = xmm1; // xmm1 = ( 0  1  2 ... 13 14 15)
            xmm2 = _mm_slli_si128::<1>(xmm2); // xmm2 = (--  0  1 ... 12 13 14)
            xmm3 = _mm_srli_si128::<1>(xmm3); // xmm3 = ( 1  2  3 ... 14 15 --)

            xmm2 = _mm_or_si128(xmm2, xmm7); // xmm2 = (-1  0  1 ... 12 13 14)
            xmm3 = _mm_or_si128(xmm3, xmm6); // xmm3 = ( 1  2  3 ... 14 15 16)

            xmm7 = xmm1;
            xmm7 = _mm_srli_si128::<15>(xmm7); // xmm7 = (15 -- -- ... -- -- --)

            xmm4 = xmm1;
            xmm1 = _mm_unpacklo_epi8(xmm1, xmm0); // xmm1 = ( 0  1  2  3  4  5  6  7)
            xmm4 = _mm_unpackhi_epi8(xmm4, xmm0); // xmm4 = ( 8  9 10 11 12 13 14 15)
            xmm5 = xmm2;
            xmm2 = _mm_unpacklo_epi8(xmm2, xmm0); // xmm2 = (-1  0  1  2  3  4  5  6)
            xmm5 = _mm_unpackhi_epi8(xmm5, xmm0); // xmm5 = ( 7  8  9 10 11 12 13 14)
            xmm6 = xmm3;
            xmm3 = _mm_unpacklo_epi8(xmm3, xmm0); // xmm3 = ( 1  2  3  4  5  6  7  8)
            xmm6 = _mm_unpackhi_epi8(xmm6, xmm0); // xmm6 = ( 9 10 11 12 13 14 15 16)

            xmm1 = _mm_mullo_epi16(xmm1, _mm_loadu_si128(&consts.PW_THREE));
            xmm4 = _mm_mullo_epi16(xmm4, _mm_loadu_si128(&consts.PW_THREE));
            xmm2 = _mm_add_epi16(xmm2, _mm_loadu_si128(&consts.PW_ONE));
            xmm5 = _mm_add_epi16(xmm5, _mm_loadu_si128(&consts.PW_ONE));
            xmm3 = _mm_add_epi16(xmm3, _mm_loadu_si128(&consts.PW_TWO));
            xmm6 = _mm_add_epi16(xmm6, _mm_loadu_si128(&consts.PW_TWO));

            xmm2 = _mm_add_epi16(xmm2, xmm1);
            xmm5 = _mm_add_epi16(xmm5, xmm4);
            xmm2 = _mm_srli_epi16::<2>(xmm2); // xmm2 = OutLE = ( 0  2  4  6  8 10 12 14)
            xmm5 = _mm_srli_epi16::<2>(xmm5); // xmm5 = OutHE = (16 18 20 22 24 26 28 30)
            xmm3 = _mm_add_epi16(xmm3, xmm1);
            xmm6 = _mm_add_epi16(xmm6, xmm4);
            xmm3 = _mm_srli_epi16::<2>(xmm3); // xmm3 = OutLO = ( 1  3  5  7  9 11 13 15)
            xmm6 = _mm_srli_epi16::<2>(xmm6); // xmm6 = OutHO = (17 19 21 23 25 27 29 31)

            xmm3 = _mm_slli_epi16::<BYTE_BIT>(xmm3);
            xmm6 = _mm_slli_epi16::<BYTE_BIT>(xmm6);
            xmm2 = _mm_or_si128(xmm2, xmm3); // xmm2 = OutL = ( 0  1  2 ... 13 14 15)
            xmm5 = _mm_or_si128(xmm5, xmm6); // xmm5 = OutH = (16 17 18 ... 29 30 31)

            _mm_storeu_si128(xmmword_mut(outptr, outcol + 0 * SIZEOF_XMMWORD), xmm2);
            _mm_storeu_si128(xmmword_mut(outptr, outcol + 1 * SIZEOF_XMMWORD), xmm5);

            rax -= SIZEOF_XMMWORD;
            incol += 1 * SIZEOF_XMMWORD; // inptr
            outcol += 2 * SIZEOF_XMMWORD; // outptr
            if rax > SIZEOF_XMMWORD {
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

// upstream: simd/x86_64/jdsample-sse2.asm jsimd_h2v2_fancy_upsample_sse2
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
/// SSE2 must be available, as it is on every x86-64 target.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_XMMWORD` and `1 * SIZEOF_XMMWORD` offsets"
)]
#[expect(clippy::too_many_lines, reason = "the asm function")]
#[target_feature(enable = "sse2")]
pub fn jsimd_h2v2_fancy_upsample_sse2(
    max_v_samp_factor: i32,
    downsampled_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    const WK_NUM: usize = 4;
    let mut output_data = output_data_ptr;
    let consts = &jconst_fancy_upsample_sse2;
    let mut wk: [__m128i; WK_NUM] = [_mm_setzero_si128(); WK_NUM];
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut xmm2: __m128i;
    let mut xmm3: __m128i;
    let mut xmm4: __m128i;
    let mut xmm5: __m128i;
    let mut xmm6: __m128i;
    let mut xmm7: __m128i;

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

        let mut inptr1_above = DummyRow::<SIZEOF_XMMWORD>::new(input_data.row(inrow - 1)); // inptr1(above)
        let mut inptr0 = DummyRow::<SIZEOF_XMMWORD>::new(input_data.row(inrow)); // inptr0
        let mut inptr1_below = DummyRow::<SIZEOF_XMMWORD>::new(input_data.row(inrow + 1)); // inptr1(below)
        let (outptr0, outptr1) = output_data.two_rows_mut(outrow); // outptr0, outptr1
        let mut incol: usize = 0;
        let mut outcol: usize = 0;

        if rax & (SIZEOF_XMMWORD - 1) != 0 {
            inptr1_above.insert_dummy(rax);
            inptr0.insert_dummy(rax);
            inptr1_below.insert_dummy(rax);
            // insert a dummy sample
        }
        // .skip:
        // -- process the first column block

        xmm0 = _mm_loadu_si128(inptr0.vector(incol + 0 * SIZEOF_XMMWORD)); // xmm0 = row[ 0][0]
        xmm1 = _mm_loadu_si128(inptr1_above.vector(incol + 0 * SIZEOF_XMMWORD)); // xmm1 = row[-1][0]
        xmm2 = _mm_loadu_si128(inptr1_below.vector(incol + 0 * SIZEOF_XMMWORD)); // xmm2 = row[+1][0]

        xmm3 = _mm_setzero_si128(); // xmm3 = (all 0's)
        xmm4 = xmm0;
        xmm0 = _mm_unpacklo_epi8(xmm0, xmm3); // xmm0 = row[ 0]( 0  1  2  3  4  5  6  7)
        xmm4 = _mm_unpackhi_epi8(xmm4, xmm3); // xmm4 = row[ 0]( 8  9 10 11 12 13 14 15)
        xmm5 = xmm1;
        xmm1 = _mm_unpacklo_epi8(xmm1, xmm3); // xmm1 = row[-1]( 0  1  2  3  4  5  6  7)
        xmm5 = _mm_unpackhi_epi8(xmm5, xmm3); // xmm5 = row[-1]( 8  9 10 11 12 13 14 15)
        xmm6 = xmm2;
        xmm2 = _mm_unpacklo_epi8(xmm2, xmm3); // xmm2 = row[+1]( 0  1  2  3  4  5  6  7)
        xmm6 = _mm_unpackhi_epi8(xmm6, xmm3); // xmm6 = row[+1]( 8  9 10 11 12 13 14 15)

        xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&consts.PW_THREE));
        xmm4 = _mm_mullo_epi16(xmm4, _mm_loadu_si128(&consts.PW_THREE));

        xmm7 = _mm_set1_epi8(-1);
        xmm7 = _mm_srli_si128::<14>(xmm7);

        xmm1 = _mm_add_epi16(xmm1, xmm0); // xmm1 = Int0L = ( 0  1  2  3  4  5  6  7)
        xmm5 = _mm_add_epi16(xmm5, xmm4); // xmm5 = Int0H = ( 8  9 10 11 12 13 14 15)
        xmm2 = _mm_add_epi16(xmm2, xmm0); // xmm2 = Int1L = ( 0  1  2  3  4  5  6  7)
        xmm6 = _mm_add_epi16(xmm6, xmm4); // xmm6 = Int1H = ( 8  9 10 11 12 13 14 15)

        // temporarily save the intermediate data
        _mm_storeu_si128(xmmword_mut(outptr0, outcol + 0 * SIZEOF_XMMWORD), xmm1);
        _mm_storeu_si128(xmmword_mut(outptr0, outcol + 1 * SIZEOF_XMMWORD), xmm5);
        _mm_storeu_si128(xmmword_mut(outptr1, outcol + 0 * SIZEOF_XMMWORD), xmm2);
        _mm_storeu_si128(xmmword_mut(outptr1, outcol + 1 * SIZEOF_XMMWORD), xmm6);

        xmm1 = _mm_and_si128(xmm1, xmm7); // xmm1 = ( 0 -- -- -- -- -- -- --)
        xmm2 = _mm_and_si128(xmm2, xmm7); // xmm2 = ( 0 -- -- -- -- -- -- --)

        wk[0] = xmm1;
        wk[1] = xmm2;

        rax = (rax + (SIZEOF_XMMWORD - 1)) & !(SIZEOF_XMMWORD - 1);
        let mut columnloop_last = rax <= SIZEOF_XMMWORD;

        loop {
            if columnloop_last {
                // .columnloop_last:
                // -- process the last column block

                xmm1 = _mm_set1_epi8(-1);
                xmm1 = _mm_slli_si128::<14>(xmm1);
                xmm2 = xmm1;

                xmm1 = _mm_and_si128(
                    xmm1,
                    _mm_loadu_si128(xmmword(outptr0, outcol + 1 * SIZEOF_XMMWORD)),
                );
                xmm2 = _mm_and_si128(
                    xmm2,
                    _mm_loadu_si128(xmmword(outptr1, outcol + 1 * SIZEOF_XMMWORD)),
                );

                wk[2] = xmm1; // xmm1 = (-- -- -- -- -- -- -- 15)
                wk[3] = xmm2; // xmm2 = (-- -- -- -- -- -- -- 15)
            } else {
                // .columnloop:
                // -- process the next column block

                xmm0 = _mm_loadu_si128(inptr0.vector(incol + 1 * SIZEOF_XMMWORD)); // xmm0 = row[ 0][1]
                xmm1 = _mm_loadu_si128(inptr1_above.vector(incol + 1 * SIZEOF_XMMWORD)); // xmm1 = row[-1][1]
                xmm2 = _mm_loadu_si128(inptr1_below.vector(incol + 1 * SIZEOF_XMMWORD)); // xmm2 = row[+1][1]

                xmm3 = _mm_setzero_si128(); // xmm3 = (all 0's)
                xmm4 = xmm0;
                xmm0 = _mm_unpacklo_epi8(xmm0, xmm3); // xmm0 = row[ 0]( 0  1  2  3  4  5  6  7)
                xmm4 = _mm_unpackhi_epi8(xmm4, xmm3); // xmm4 = row[ 0]( 8  9 10 11 12 13 14 15)
                xmm5 = xmm1;
                xmm1 = _mm_unpacklo_epi8(xmm1, xmm3); // xmm1 = row[-1]( 0  1  2  3  4  5  6  7)
                xmm5 = _mm_unpackhi_epi8(xmm5, xmm3); // xmm5 = row[-1]( 8  9 10 11 12 13 14 15)
                xmm6 = xmm2;
                xmm2 = _mm_unpacklo_epi8(xmm2, xmm3); // xmm2 = row[+1]( 0  1  2  3  4  5  6  7)
                xmm6 = _mm_unpackhi_epi8(xmm6, xmm3); // xmm6 = row[+1]( 8  9 10 11 12 13 14 15)

                xmm0 = _mm_mullo_epi16(xmm0, _mm_loadu_si128(&consts.PW_THREE));
                xmm4 = _mm_mullo_epi16(xmm4, _mm_loadu_si128(&consts.PW_THREE));

                xmm1 = _mm_add_epi16(xmm1, xmm0); // xmm1 = Int0L = ( 0  1  2  3  4  5  6  7)
                xmm5 = _mm_add_epi16(xmm5, xmm4); // xmm5 = Int0H = ( 8  9 10 11 12 13 14 15)
                xmm2 = _mm_add_epi16(xmm2, xmm0); // xmm2 = Int1L = ( 0  1  2  3  4  5  6  7)
                xmm6 = _mm_add_epi16(xmm6, xmm4); // xmm6 = Int1H = ( 8  9 10 11 12 13 14 15)

                // temporarily save the intermediate data
                _mm_storeu_si128(xmmword_mut(outptr0, outcol + 2 * SIZEOF_XMMWORD), xmm1);
                _mm_storeu_si128(xmmword_mut(outptr0, outcol + 3 * SIZEOF_XMMWORD), xmm5);
                _mm_storeu_si128(xmmword_mut(outptr1, outcol + 2 * SIZEOF_XMMWORD), xmm2);
                _mm_storeu_si128(xmmword_mut(outptr1, outcol + 3 * SIZEOF_XMMWORD), xmm6);

                xmm1 = _mm_slli_si128::<14>(xmm1); // xmm1 = (-- -- -- -- -- -- --  0)
                xmm2 = _mm_slli_si128::<14>(xmm2); // xmm2 = (-- -- -- -- -- -- --  0)

                wk[2] = xmm1;
                wk[3] = xmm2;
            }

            // .upsample:
            // -- process the upper row

            xmm7 = _mm_loadu_si128(xmmword(outptr0, outcol + 0 * SIZEOF_XMMWORD));
            xmm3 = _mm_loadu_si128(xmmword(outptr0, outcol + 1 * SIZEOF_XMMWORD));

            xmm0 = xmm7; // xmm7 = Int0L = ( 0  1  2  3  4  5  6  7)
            xmm4 = xmm3; // xmm3 = Int0H = ( 8  9 10 11 12 13 14 15)
            xmm0 = _mm_srli_si128::<2>(xmm0); // xmm0 = ( 1  2  3  4  5  6  7 --)
            xmm4 = _mm_slli_si128::<14>(xmm4); // xmm4 = (-- -- -- -- -- -- --  8)
            xmm5 = xmm7;
            xmm6 = xmm3;
            xmm5 = _mm_srli_si128::<14>(xmm5); // xmm5 = ( 7 -- -- -- -- -- -- --)
            xmm6 = _mm_slli_si128::<2>(xmm6); // xmm6 = (--  8  9 10 11 12 13 14)

            xmm0 = _mm_or_si128(xmm0, xmm4); // xmm0 = ( 1  2  3  4  5  6  7  8)
            xmm5 = _mm_or_si128(xmm5, xmm6); // xmm5 = ( 7  8  9 10 11 12 13 14)

            xmm1 = xmm7;
            xmm2 = xmm3;
            xmm1 = _mm_slli_si128::<2>(xmm1); // xmm1 = (--  0  1  2  3  4  5  6)
            xmm2 = _mm_srli_si128::<2>(xmm2); // xmm2 = ( 9 10 11 12 13 14 15 --)
            xmm4 = xmm3;
            xmm4 = _mm_srli_si128::<14>(xmm4); // xmm4 = (15 -- -- -- -- -- -- --)

            xmm1 = _mm_or_si128(xmm1, wk[0]); // xmm1 = (-1  0  1  2  3  4  5  6)
            xmm2 = _mm_or_si128(xmm2, wk[2]); // xmm2 = ( 9 10 11 12 13 14 15 16)

            wk[0] = xmm4;

            xmm7 = _mm_mullo_epi16(xmm7, _mm_loadu_si128(&consts.PW_THREE));
            xmm3 = _mm_mullo_epi16(xmm3, _mm_loadu_si128(&consts.PW_THREE));
            xmm1 = _mm_add_epi16(xmm1, _mm_loadu_si128(&consts.PW_EIGHT));
            xmm5 = _mm_add_epi16(xmm5, _mm_loadu_si128(&consts.PW_EIGHT));
            xmm0 = _mm_add_epi16(xmm0, _mm_loadu_si128(&consts.PW_SEVEN));
            xmm2 = _mm_add_epi16(xmm2, _mm_loadu_si128(&consts.PW_SEVEN));

            xmm1 = _mm_add_epi16(xmm1, xmm7);
            xmm5 = _mm_add_epi16(xmm5, xmm3);
            xmm1 = _mm_srli_epi16::<4>(xmm1); // xmm1 = Out0LE = ( 0  2  4  6  8 10 12 14)
            xmm5 = _mm_srli_epi16::<4>(xmm5); // xmm5 = Out0HE = (16 18 20 22 24 26 28 30)
            xmm0 = _mm_add_epi16(xmm0, xmm7);
            xmm2 = _mm_add_epi16(xmm2, xmm3);
            xmm0 = _mm_srli_epi16::<4>(xmm0); // xmm0 = Out0LO = ( 1  3  5  7  9 11 13 15)
            xmm2 = _mm_srli_epi16::<4>(xmm2); // xmm2 = Out0HO = (17 19 21 23 25 27 29 31)

            xmm0 = _mm_slli_epi16::<BYTE_BIT>(xmm0);
            xmm2 = _mm_slli_epi16::<BYTE_BIT>(xmm2);
            xmm1 = _mm_or_si128(xmm1, xmm0); // xmm1 = Out0L = ( 0  1  2 ... 13 14 15)
            xmm5 = _mm_or_si128(xmm5, xmm2); // xmm5 = Out0H = (16 17 18 ... 29 30 31)

            _mm_storeu_si128(xmmword_mut(outptr0, outcol + 0 * SIZEOF_XMMWORD), xmm1);
            _mm_storeu_si128(xmmword_mut(outptr0, outcol + 1 * SIZEOF_XMMWORD), xmm5);

            // -- process the lower row

            xmm6 = _mm_loadu_si128(xmmword(outptr1, outcol + 0 * SIZEOF_XMMWORD));
            xmm4 = _mm_loadu_si128(xmmword(outptr1, outcol + 1 * SIZEOF_XMMWORD));

            xmm7 = xmm6; // xmm6 = Int1L = ( 0  1  2  3  4  5  6  7)
            xmm3 = xmm4; // xmm4 = Int1H = ( 8  9 10 11 12 13 14 15)
            xmm7 = _mm_srli_si128::<2>(xmm7); // xmm7 = ( 1  2  3  4  5  6  7 --)
            xmm3 = _mm_slli_si128::<14>(xmm3); // xmm3 = (-- -- -- -- -- -- --  8)
            xmm0 = xmm6;
            xmm2 = xmm4;
            xmm0 = _mm_srli_si128::<14>(xmm0); // xmm0 = ( 7 -- -- -- -- -- -- --)
            xmm2 = _mm_slli_si128::<2>(xmm2); // xmm2 = (--  8  9 10 11 12 13 14)

            xmm7 = _mm_or_si128(xmm7, xmm3); // xmm7 = ( 1  2  3  4  5  6  7  8)
            xmm0 = _mm_or_si128(xmm0, xmm2); // xmm0 = ( 7  8  9 10 11 12 13 14)

            xmm1 = xmm6;
            xmm5 = xmm4;
            xmm1 = _mm_slli_si128::<2>(xmm1); // xmm1 = (--  0  1  2  3  4  5  6)
            xmm5 = _mm_srli_si128::<2>(xmm5); // xmm5 = ( 9 10 11 12 13 14 15 --)
            xmm3 = xmm4;
            xmm3 = _mm_srli_si128::<14>(xmm3); // xmm3 = (15 -- -- -- -- -- -- --)

            xmm1 = _mm_or_si128(xmm1, wk[1]); // xmm1 = (-1  0  1  2  3  4  5  6)
            xmm5 = _mm_or_si128(xmm5, wk[3]); // xmm5 = ( 9 10 11 12 13 14 15 16)

            wk[1] = xmm3;

            xmm6 = _mm_mullo_epi16(xmm6, _mm_loadu_si128(&consts.PW_THREE));
            xmm4 = _mm_mullo_epi16(xmm4, _mm_loadu_si128(&consts.PW_THREE));
            xmm1 = _mm_add_epi16(xmm1, _mm_loadu_si128(&consts.PW_EIGHT));
            xmm0 = _mm_add_epi16(xmm0, _mm_loadu_si128(&consts.PW_EIGHT));
            xmm7 = _mm_add_epi16(xmm7, _mm_loadu_si128(&consts.PW_SEVEN));
            xmm5 = _mm_add_epi16(xmm5, _mm_loadu_si128(&consts.PW_SEVEN));

            xmm1 = _mm_add_epi16(xmm1, xmm6);
            xmm0 = _mm_add_epi16(xmm0, xmm4);
            xmm1 = _mm_srli_epi16::<4>(xmm1); // xmm1 = Out1LE = ( 0  2  4  6  8 10 12 14)
            xmm0 = _mm_srli_epi16::<4>(xmm0); // xmm0 = Out1HE = (16 18 20 22 24 26 28 30)
            xmm7 = _mm_add_epi16(xmm7, xmm6);
            xmm5 = _mm_add_epi16(xmm5, xmm4);
            xmm7 = _mm_srli_epi16::<4>(xmm7); // xmm7 = Out1LO = ( 1  3  5  7  9 11 13 15)
            xmm5 = _mm_srli_epi16::<4>(xmm5); // xmm5 = Out1HO = (17 19 21 23 25 27 29 31)

            xmm7 = _mm_slli_epi16::<BYTE_BIT>(xmm7);
            xmm5 = _mm_slli_epi16::<BYTE_BIT>(xmm5);
            xmm1 = _mm_or_si128(xmm1, xmm7); // xmm1 = Out1L = ( 0  1  2 ... 13 14 15)
            xmm0 = _mm_or_si128(xmm0, xmm5); // xmm0 = Out1H = (16 17 18 ... 29 30 31)

            _mm_storeu_si128(xmmword_mut(outptr1, outcol + 0 * SIZEOF_XMMWORD), xmm1);
            _mm_storeu_si128(xmmword_mut(outptr1, outcol + 1 * SIZEOF_XMMWORD), xmm0);

            rax -= SIZEOF_XMMWORD;
            incol += 1 * SIZEOF_XMMWORD; // inptr1(above), inptr0, inptr1(below)
            outcol += 2 * SIZEOF_XMMWORD; // outptr0, outptr1
            if rax > SIZEOF_XMMWORD {
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

// upstream: simd/x86_64/jdsample-sse2.asm jsimd_h2v1_upsample_sse2
/// Plain h2v1 upsampling of `max_v_samp_factor` rows into rows of
/// `output_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_XMMWORD` and `1 * SIZEOF_XMMWORD` offsets"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_h2v1_upsample_sse2(
    max_v_samp_factor: i32,
    output_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut xmm2: __m128i;
    let mut xmm3: __m128i;

    let mut rdx = output_width as usize;
    rdx = (rdx + ((2 * SIZEOF_XMMWORD) - 1)) & !((2 * SIZEOF_XMMWORD) - 1);
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
            xmm0 = _mm_loadu_si128(xmmword(inptr, incol + 0 * SIZEOF_XMMWORD));

            xmm1 = xmm0;
            xmm0 = _mm_unpacklo_epi8(xmm0, xmm0);
            xmm1 = _mm_unpackhi_epi8(xmm1, xmm1);

            _mm_storeu_si128(xmmword_mut(outptr, outcol + 0 * SIZEOF_XMMWORD), xmm0);
            _mm_storeu_si128(xmmword_mut(outptr, outcol + 1 * SIZEOF_XMMWORD), xmm1);

            rax -= 2 * SIZEOF_XMMWORD;
            if rax == 0 {
                break;
            }

            xmm2 = _mm_loadu_si128(xmmword(inptr, incol + 1 * SIZEOF_XMMWORD));

            xmm3 = xmm2;
            xmm2 = _mm_unpacklo_epi8(xmm2, xmm2);
            xmm3 = _mm_unpackhi_epi8(xmm3, xmm3);

            _mm_storeu_si128(xmmword_mut(outptr, outcol + 2 * SIZEOF_XMMWORD), xmm2);
            _mm_storeu_si128(xmmword_mut(outptr, outcol + 3 * SIZEOF_XMMWORD), xmm3);

            rax -= 2 * SIZEOF_XMMWORD;
            if rax == 0 {
                break;
            }

            incol += 2 * SIZEOF_XMMWORD; // inptr
            outcol += 4 * SIZEOF_XMMWORD; // outptr
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

// upstream: simd/x86_64/jdsample-sse2.asm jsimd_h2v2_upsample_sse2
/// Plain h2v2 upsampling of `max_v_samp_factor` output rows of
/// `output_width` samples.
///
/// # Panics
///
/// When a row is shorter than the kernel reads or writes.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86.rs have the intrinsics' names"
)]
#[expect(
    clippy::erasing_op,
    clippy::identity_op,
    reason = "the asm's `0 * SIZEOF_XMMWORD` and `1 * SIZEOF_XMMWORD` offsets"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_h2v2_upsample_sse2(
    max_v_samp_factor: i32,
    output_width: JDIMENSION,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: JSAMPROWS_MUT<'_, '_>,
) {
    let mut output_data = output_data_ptr;
    let mut xmm0: __m128i;
    let mut xmm1: __m128i;
    let mut xmm2: __m128i;
    let mut xmm3: __m128i;

    let mut rdx = output_width as usize;
    rdx = (rdx + ((2 * SIZEOF_XMMWORD) - 1)) & !((2 * SIZEOF_XMMWORD) - 1);
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
            xmm0 = _mm_loadu_si128(xmmword(inptr, incol + 0 * SIZEOF_XMMWORD));

            xmm1 = xmm0;
            xmm0 = _mm_unpacklo_epi8(xmm0, xmm0);
            xmm1 = _mm_unpackhi_epi8(xmm1, xmm1);

            _mm_storeu_si128(xmmword_mut(outptr0, outcol + 0 * SIZEOF_XMMWORD), xmm0);
            _mm_storeu_si128(xmmword_mut(outptr0, outcol + 1 * SIZEOF_XMMWORD), xmm1);
            _mm_storeu_si128(xmmword_mut(outptr1, outcol + 0 * SIZEOF_XMMWORD), xmm0);
            _mm_storeu_si128(xmmword_mut(outptr1, outcol + 1 * SIZEOF_XMMWORD), xmm1);

            rax -= 2 * SIZEOF_XMMWORD;
            if rax == 0 {
                break;
            }

            xmm2 = _mm_loadu_si128(xmmword(inptr, incol + 1 * SIZEOF_XMMWORD));

            xmm3 = xmm2;
            xmm2 = _mm_unpacklo_epi8(xmm2, xmm2);
            xmm3 = _mm_unpackhi_epi8(xmm3, xmm3);

            _mm_storeu_si128(xmmword_mut(outptr0, outcol + 2 * SIZEOF_XMMWORD), xmm2);
            _mm_storeu_si128(xmmword_mut(outptr0, outcol + 3 * SIZEOF_XMMWORD), xmm3);
            _mm_storeu_si128(xmmword_mut(outptr1, outcol + 2 * SIZEOF_XMMWORD), xmm2);
            _mm_storeu_si128(xmmword_mut(outptr1, outcol + 3 * SIZEOF_XMMWORD), xmm3);

            rax -= 2 * SIZEOF_XMMWORD;
            if rax == 0 {
                break;
            }

            incol += 2 * SIZEOF_XMMWORD; // inptr
            outcol += 4 * SIZEOF_XMMWORD; // outptr0, outptr1
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
