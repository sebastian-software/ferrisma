// libjpeg-turbo 3.2.1, simd/x86_64/jdcolext-sse2.asm:
//
// ;
// ; YCbCr-to-RGB Color Conversion (64-bit SSE2)
// ;
// ; Copyright 2009, 2012 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2012, 2016, 2024-2025, D. R. Commander.
// ; Copyright (C) 2018, Matthias Räncker.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/x86_64/jdcolext-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! SSE2. `jdcolor-sse2.asm` includes this file once per RGB pixel layout,
//! with `RGB_RED`, `RGB_GREEN`, `RGB_BLUE` and `RGB_PIXELSIZE` defined and
//! the function renamed. The kernel is a function generic over those values,
//! and `jdcolor_sse2.rs` names the instances. `jcolsamp.inc` names the
//! registers of each component after its position in the pixel, `xmmA` to
//! `xmmH`; [`jcolsamp`] returns the registers in that order, and the
//! interleaving code reads and writes those names. The `%ifdef
//! RGBX_FILLER_0XFF` branch is taken, as `jsimdcfg.inc` defines it: the
//! fourth byte of a pixel is 0xFF.
//!
//! The registers are local variables named after them, and `wk(i)` is the
//! local array `wk`. The rows are slices; a pointer increment is a reslice,
//! and [`input`] and [`output`] make the arrays that the loads and stores of
//! `x86` take. The input rows extend to the end of their storage, which
//! covers the whole-vector loads past `out_width` into the `alloc_sarray`
//! padding; the stores stay within `out_width`, as the asm's tail does. The
//! asm tests the output address and stores whole vectors with `movntdq` when
//! it is aligned and with `movdqu` otherwise; both are
//! [`_mm_storeu_si128`] (ADR 0008, decision 4), so the test is not kept.
//! The `sfence` that ends the non-temporal stores is kept. The tail's
//! `movd eax, xmmA` and the word and byte stores from `ax` and `al` use the
//! local `eax`, since the row counter `rax` is pushed there. The row counter
//! is `num_rows` zero-extended to 64 bits, as `mov eax, r14d` loads it.

#![forbid(unsafe_code)]

// Clippy skips wildcard imports in test builds.
#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "the asm's instructions as intrinsics (ADR 0008)"
    )
)]
use core::arch::x86_64::*;

use crate::jdcolor_sse2::{SCALEBITS, jconst_ycc_rgb_convert_sse2};
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::{JSAMPROWS, JSAMPROWS_MUT};
use crate::x86::{_mm_loadu_si128, _mm_storel_epi64, _mm_storeu_si128};

// upstream: simd/nasm/jsimdext.inc BYTE_BIT
const BYTE_BIT: i32 = 8;
// upstream: simd/nasm/jsimdext.inc SIZEOF_WORD
const SIZEOF_WORD: usize = 2;
// upstream: simd/nasm/jsimdext.inc SIZEOF_DWORD
const SIZEOF_DWORD: usize = 4;
// upstream: simd/nasm/jsimdext.inc SIZEOF_MMWORD
const SIZEOF_MMWORD: usize = 8;
// upstream: simd/nasm/jsimdext.inc SIZEOF_XMMWORD
const SIZEOF_XMMWORD: usize = 16;

/// The `N` samples from `inptr` on that a load reads. Not an upstream item.
///
/// # Panics
///
/// When the row's storage ends before them, where the asm would read past
/// the allocation.
#[inline]
#[must_use]
pub fn input<const N: usize>(inptr: &[JSAMPLE]) -> &[JSAMPLE; N] {
    inptr
        .first_chunk()
        .expect("input row shorter than the load reads")
}

/// The `N` samples from `outptr + offset` on that a store writes. Not an
/// upstream item.
///
/// # Panics
///
/// When the row ends before them, where the asm would write past the row.
#[inline]
pub fn output<const N: usize>(outptr: &mut [JSAMPLE], offset: usize) -> &mut [JSAMPLE; N] {
    outptr[offset..]
        .first_chunk_mut()
        .expect("output row shorter than the store writes")
}

/// `jcolsamp.inc`'s pseudo-registers: `xmmA` to `xmmH` from `xmm0` to
/// `xmm7`, in that order. The components at pixel positions 0, 1, 2 and 3
/// are `xmmA`/`xmmB`, `xmmC`/`xmmD`, `xmmE`/`xmmF` and `xmmG`/`xmmH`; each
/// pair is `xmm0`/`xmm1` for red, `xmm2`/`xmm3` for green, `xmm4`/`xmm5` for
/// blue and `xmm6`/`xmm7` otherwise. Not an upstream item.
#[inline]
#[must_use]
pub fn jcolsamp<const RGB_RED: i32, const RGB_GREEN: i32, const RGB_BLUE: i32, T: Copy>(
    xmm: [T; 8],
) -> [T; 8] {
    let pair = |position: i32| -> usize {
        if RGB_RED == position {
            0
        } else if RGB_GREEN == position {
            2
        } else if RGB_BLUE == position {
            4
        } else {
            6
        }
    };
    let (a, c, e, g) = (pair(0), pair(1), pair(2), pair(3));
    [
        xmm[a],
        xmm[a + 1],
        xmm[c],
        xmm[c + 1],
        xmm[e],
        xmm[e + 1],
        xmm[g],
        xmm[g + 1],
    ]
}

// --------------------------------------------------------------------------
//
// Convert some rows of samples to the output colorspace.
//
// GLOBAL(void)
// jsimd_ycc_rgb_convert_sse2(JDIMENSION out_width, JSAMPIMAGE input_buf,
//                            JDIMENSION input_row, JSAMPARRAY output_buf,
//                            int num_rows)
//
// r10d = JDIMENSION out_width
// r11 = JSAMPIMAGE input_buf
// r12d = JDIMENSION input_row
// r13 = JSAMPARRAY output_buf
// r14d = int num_rows

// upstream: simd/x86_64/jdcolext-sse2.asm WK_NUM
const WK_NUM: usize = 2;

// upstream: simd/x86_64/jdcolext-sse2.asm jsimd_ycc_rgb_convert_sse2
/// Converts `num_rows` rows of `out_width` pixels of `input_buf` from row
/// `input_row` on from YCbCr to RGB in the pixel layout of the parameters.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
///
/// # Panics
///
/// When a row is shorter than the asm reads or writes.
#[expect(
    clippy::cast_sign_loss,
    reason = "the asm's RGB_PIXELSIZE as an address offset"
)]
#[expect(non_snake_case, reason = "the asm's register names")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(
    clippy::used_underscore_items,
    reason = "the loads and stores of x86, named after their intrinsics"
)]
#[target_feature(enable = "sse2")]
pub fn jsimd_ycc_rgb_convert_sse2<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_PIXELSIZE: i32,
>(
    out_width: JDIMENSION,
    input_buf: &[JSAMPROWS<'_>],
    input_row: JDIMENSION,
    mut output_buf: JSAMPROWS_MUT<'_, '_>,
    num_rows: i32,
) {
    let consts = &jconst_ycc_rgb_convert_sse2;
    let mut wk = [_mm_setzero_si128(); WK_NUM];

    let num_cols = out_width as usize; // num_cols
    if num_cols == 0 {
        return;
    }

    // rsi, rbx, rdx: input_buf[0], [1], [2] + input_row
    let mut row = input_row as usize;
    // rdi: output_buf
    let mut out_row = 0;
    let mut rax = i64::from(num_rows.cast_unsigned());
    if rax <= 0 {
        return;
    }
    // .rowloop
    loop {
        let mut rcx = num_cols; // col

        let mut inptr0: &[JSAMPLE] = input_buf[0].row(row); // inptr0
        let mut inptr1: &[JSAMPLE] = input_buf[1].row(row); // inptr1
        let mut inptr2: &[JSAMPLE] = input_buf[2].row(row); // inptr2
        let mut outptr: &mut [JSAMPLE] = output_buf.row_mut(out_row); // outptr
        // .columnloop
        loop {
            let mut xmm5 = _mm_loadu_si128(input::<16>(inptr1)); // xmm5 = Cb(0123456789abcdef)
            let mut xmm1 = _mm_loadu_si128(input::<16>(inptr2)); // xmm1 = Cr(0123456789abcdef)

            let mut xmm4 = _mm_set1_epi16(-1);
            let mut xmm7 = _mm_set1_epi16(-1);
            xmm4 = _mm_srli_epi16::<BYTE_BIT>(xmm4);
            xmm7 = _mm_slli_epi16::<7>(xmm7); // xmm7 = { 0xFF80 0xFF80 0xFF80 0xFF80 .. }
            let mut xmm0 = xmm4; // xmm0 = xmm4 = { 0xFF 0x00 0xFF 0x00 .. }

            xmm4 = _mm_and_si128(xmm4, xmm5); // xmm4 = Cb(02468ace) = CbE
            xmm5 = _mm_srli_epi16::<BYTE_BIT>(xmm5); // xmm5 = Cb(13579bdf) = CbO
            xmm0 = _mm_and_si128(xmm0, xmm1); // xmm0 = Cr(02468ace) = CrE
            xmm1 = _mm_srli_epi16::<BYTE_BIT>(xmm1); // xmm1 = Cr(13579bdf) = CrO

            xmm4 = _mm_add_epi16(xmm4, xmm7);
            xmm5 = _mm_add_epi16(xmm5, xmm7);
            xmm0 = _mm_add_epi16(xmm0, xmm7);
            xmm1 = _mm_add_epi16(xmm1, xmm7);

            // (Original)
            // R = Y                + 1.40200 * Cr
            // G = Y - 0.34414 * Cb - 0.71414 * Cr
            // B = Y + 1.77200 * Cb
            //
            // (This implementation)
            // R = Y                + 0.40200 * Cr + Cr
            // G = Y - 0.34414 * Cb + 0.28586 * Cr - Cr
            // B = Y - 0.22800 * Cb + Cb + Cb

            let mut xmm2 = xmm4; // xmm2 = CbE
            let mut xmm3 = xmm5; // xmm3 = CbO
            xmm4 = _mm_add_epi16(xmm4, xmm4); // xmm4 = 2 * CbE
            xmm5 = _mm_add_epi16(xmm5, xmm5); // xmm5 = 2 * CbO
            let mut xmm6 = xmm0; // xmm6 = CrE
            xmm7 = xmm1; // xmm7 = CrO
            xmm0 = _mm_add_epi16(xmm0, xmm0); // xmm0 = 2 * CrE
            xmm1 = _mm_add_epi16(xmm1, xmm1); // xmm1 = 2 * CrO

            xmm4 = _mm_mulhi_epi16(xmm4, _mm_loadu_si128(&consts.PW_MF0228)); // xmm4 = (2 * CbE * -FIX(0.22800))
            xmm5 = _mm_mulhi_epi16(xmm5, _mm_loadu_si128(&consts.PW_MF0228)); // xmm5 = (2 * CbO * -FIX(0.22800))
            xmm0 = _mm_mulhi_epi16(xmm0, _mm_loadu_si128(&consts.PW_F0402)); // xmm0 = (2 * CrE * FIX(0.40200))
            xmm1 = _mm_mulhi_epi16(xmm1, _mm_loadu_si128(&consts.PW_F0402)); // xmm1 = (2 * CrO * FIX(0.40200))

            xmm4 = _mm_add_epi16(xmm4, _mm_loadu_si128(&consts.PW_ONE));
            xmm5 = _mm_add_epi16(xmm5, _mm_loadu_si128(&consts.PW_ONE));
            xmm4 = _mm_srai_epi16::<1>(xmm4); // xmm4 = (CbE * -FIX(0.22800))
            xmm5 = _mm_srai_epi16::<1>(xmm5); // xmm5 = (CbO * -FIX(0.22800))
            xmm0 = _mm_add_epi16(xmm0, _mm_loadu_si128(&consts.PW_ONE));
            xmm1 = _mm_add_epi16(xmm1, _mm_loadu_si128(&consts.PW_ONE));
            xmm0 = _mm_srai_epi16::<1>(xmm0); // xmm0 = (CrE * FIX(0.40200))
            xmm1 = _mm_srai_epi16::<1>(xmm1); // xmm1 = (CrO * FIX(0.40200))

            xmm4 = _mm_add_epi16(xmm4, xmm2);
            xmm5 = _mm_add_epi16(xmm5, xmm3);
            xmm4 = _mm_add_epi16(xmm4, xmm2); // xmm4 = (CbE * FIX(1.77200)) = (B - Y)E
            xmm5 = _mm_add_epi16(xmm5, xmm3); // xmm5 = (CbO * FIX(1.77200)) = (B - Y)O
            xmm0 = _mm_add_epi16(xmm0, xmm6); // xmm0 = (CrE * FIX(1.40200)) = (R - Y)E
            xmm1 = _mm_add_epi16(xmm1, xmm7); // xmm1 = (CrO * FIX(1.40200)) = (R - Y)O

            wk[0] = xmm4; // wk(0) = (B - Y)E
            wk[1] = xmm5; // wk(1) = (B - Y)O

            xmm4 = xmm2;
            xmm5 = xmm3;
            xmm2 = _mm_unpacklo_epi16(xmm2, xmm6);
            xmm4 = _mm_unpackhi_epi16(xmm4, xmm6);
            xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&consts.PW_MF0344_F0285));
            xmm4 = _mm_madd_epi16(xmm4, _mm_loadu_si128(&consts.PW_MF0344_F0285));
            xmm3 = _mm_unpacklo_epi16(xmm3, xmm7);
            xmm5 = _mm_unpackhi_epi16(xmm5, xmm7);
            xmm3 = _mm_madd_epi16(xmm3, _mm_loadu_si128(&consts.PW_MF0344_F0285));
            xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&consts.PW_MF0344_F0285));

            xmm2 = _mm_add_epi32(xmm2, _mm_loadu_si128(&consts.PD_ONEHALF));
            xmm4 = _mm_add_epi32(xmm4, _mm_loadu_si128(&consts.PD_ONEHALF));
            xmm2 = _mm_srai_epi32::<SCALEBITS>(xmm2);
            xmm4 = _mm_srai_epi32::<SCALEBITS>(xmm4);
            xmm3 = _mm_add_epi32(xmm3, _mm_loadu_si128(&consts.PD_ONEHALF));
            xmm5 = _mm_add_epi32(xmm5, _mm_loadu_si128(&consts.PD_ONEHALF));
            xmm3 = _mm_srai_epi32::<SCALEBITS>(xmm3);
            xmm5 = _mm_srai_epi32::<SCALEBITS>(xmm5);

            xmm2 = _mm_packs_epi32(xmm2, xmm4);
            // xmm2 = CbE * -FIX(0.344) + CrE * FIX(0.285)
            xmm3 = _mm_packs_epi32(xmm3, xmm5);
            // xmm3 = CbO * -FIX(0.344) + CrO * FIX(0.285)
            xmm2 = _mm_sub_epi16(xmm2, xmm6);
            // xmm2 = CbE * -FIX(0.344) + CrE * -FIX(0.714) = (G - Y)E
            xmm3 = _mm_sub_epi16(xmm3, xmm7);
            // xmm3 = CbO * -FIX(0.344) + CrO * -FIX(0.714) = (G - Y)O

            xmm5 = _mm_loadu_si128(input::<16>(inptr0)); // xmm5 = Y(0123456789abcdef)

            xmm4 = _mm_set1_epi16(-1);
            xmm4 = _mm_srli_epi16::<BYTE_BIT>(xmm4); // xmm4 = { 0xFF 0x00 0xFF 0x00 .. }
            xmm4 = _mm_and_si128(xmm4, xmm5); // xmm4 = Y(02468ace) = YE
            xmm5 = _mm_srli_epi16::<BYTE_BIT>(xmm5); // xmm5 = Y(13579bdf) = YO

            xmm0 = _mm_add_epi16(xmm0, xmm4); // xmm0 = ((R - Y)E + YE) = RE = R(02468ace)
            xmm1 = _mm_add_epi16(xmm1, xmm5); // xmm1 = ((R - Y)O + YO) = RO = R(13579bdf)
            xmm0 = _mm_packus_epi16(xmm0, xmm0); // xmm0 = R(02468ace********)
            xmm1 = _mm_packus_epi16(xmm1, xmm1); // xmm1 = R(13579bdf********)

            xmm2 = _mm_add_epi16(xmm2, xmm4); // xmm2 = ((G - Y)E + YE) = GE = G(02468ace)
            xmm3 = _mm_add_epi16(xmm3, xmm5); // xmm3 = ((G - Y)O + YO) = GO = G(13579bdf)
            xmm2 = _mm_packus_epi16(xmm2, xmm2); // xmm2 = G(02468ace********)
            xmm3 = _mm_packus_epi16(xmm3, xmm3); // xmm3 = G(13579bdf********)

            xmm4 = _mm_add_epi16(xmm4, wk[0]);
            // xmm4 = (YE + (B - Y)E) = BE = B(02468ace)
            xmm5 = _mm_add_epi16(xmm5, wk[1]);
            // xmm5 = (YO + (B - Y)O) = BO = B(13579bdf)
            xmm4 = _mm_packus_epi16(xmm4, xmm4); // xmm4 = B(02468ace********)
            xmm5 = _mm_packus_epi16(xmm5, xmm5); // xmm5 = B(13579bdf********)

            if RGB_PIXELSIZE == 3 {
                // ---------------

                // NOTE: The values of RGB_RED, RGB_GREEN, and RGB_BLUE determine the
                // mapping of components A, B, and C to red, green, and blue.
                //
                // xmmA = (A0 A2 A4 A6 A8 Aa Ac Ae) = AE
                // xmmB = (A1 A3 A5 A7 A9 Ab Ad Af) = AO
                // xmmC = (B0 B2 B4 B6 B8 Ba Bc Be) = BE
                // xmmD = (B1 B3 B5 B7 B9 Bb Bd Bf) = BO
                // xmmE = (C0 C2 C4 C6 C8 Ca Cc Ce) = CE
                // xmmF = (C1 C3 C5 C7 C9 Cb Cd Cf) = CO
                // xmmG = (** ** ** ** ** ** ** **)
                // xmmH = (** ** ** ** ** ** ** **)
                let [
                    mut xmmA,
                    mut xmmB,
                    mut xmmC,
                    mut xmmD,
                    mut xmmE,
                    mut xmmF,
                    _,
                    _,
                ] = jcolsamp::<RGB_RED, RGB_GREEN, RGB_BLUE, _>([
                    xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7,
                ]);

                xmmA = _mm_unpacklo_epi8(xmmA, xmmC);
                // xmmA = (A0 B0 A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be)
                xmmE = _mm_unpacklo_epi8(xmmE, xmmB);
                // xmmE = (C0 A1 C2 A3 C4 A5 C6 A7 C8 A9 Ca Ab Cc Ad Ce Af)
                xmmD = _mm_unpacklo_epi8(xmmD, xmmF);
                // xmmD = (B1 C1 B3 C3 B5 C5 B7 C7 B9 C9 Bb Cb Bd Cd Bf Cf)

                let mut xmmG = xmmA;
                let mut xmmH = xmmA;
                xmmA = _mm_unpacklo_epi16(xmmA, xmmE);
                // xmmA = (A0 B0 C0 A1 A2 B2 C2 A3 A4 B4 C4 A5 A6 B6 C6 A7)
                xmmG = _mm_unpackhi_epi16(xmmG, xmmE);
                // xmmG = (A8 B8 C8 A9 Aa Ba Ca Ab Ac Bc Cc Ad Ae Be Ce Af)

                xmmH = _mm_srli_si128::<2>(xmmH);
                // xmmH = (A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be -- --)
                xmmE = _mm_srli_si128::<2>(xmmE);
                // xmmE = (C2 A3 C4 A5 C6 A7 C8 A9 Ca Ab Cc Ad Ce Af -- --)

                xmmC = xmmD;
                xmmB = xmmD;
                xmmD = _mm_unpacklo_epi16(xmmD, xmmH);
                // xmmD = (B1 C1 A2 B2 B3 C3 A4 B4 B5 C5 A6 B6 B7 C7 A8 B8)
                xmmC = _mm_unpackhi_epi16(xmmC, xmmH);
                // xmmC = (B9 C9 Aa Ba Bb Cb Ac Bc Bd Cd Ae Be Bf Cf -- --)

                xmmB = _mm_srli_si128::<2>(xmmB);
                // xmmB = (B3 C3 B5 C5 B7 C7 B9 C9 Bb Cb Bd Cd Bf Cf -- --)

                xmmF = xmmE;
                xmmE = _mm_unpacklo_epi16(xmmE, xmmB);
                // xmmE = (C2 A3 B3 C3 C4 A5 B5 C5 C6 A7 B7 C7 C8 A9 B9 C9)
                xmmF = _mm_unpackhi_epi16(xmmF, xmmB);
                // xmmF = (Ca Ab Bb Cb Cc Ad Bd Cd Ce Af Bf Cf -- -- -- --)

                xmmH = _mm_shuffle_epi32::<0x4E>(xmmA);
                // xmmH = (A4 B4 C4 A5 A6 B6 C6 A7 A0 B0 C0 A1 A2 B2 C2 A3)
                xmmB = xmmE;
                xmmA = _mm_unpacklo_epi32(xmmA, xmmD);
                // xmmA = (A0 B0 C0 A1 B1 C1 A2 B2 A2 B2 C2 A3 B3 C3 A4 B4)
                xmmE = _mm_unpacklo_epi32(xmmE, xmmH);
                // xmmE = (C2 A3 B3 C3 A4 B4 C4 A5 C4 A5 B5 C5 A6 B6 C6 A7)
                xmmD = _mm_unpackhi_epi32(xmmD, xmmB);
                // xmmD = (B5 C5 A6 B6 C6 A7 B7 C7 B7 C7 A8 B8 C8 A9 B9 C9)

                xmmH = _mm_shuffle_epi32::<0x4E>(xmmG);
                // xmmH = (Ac Bc Cc Ad Ae Be Ce Af A8 B8 C8 A9 Aa Ba Ca Ab)
                xmmB = xmmF;
                xmmG = _mm_unpacklo_epi32(xmmG, xmmC);
                // xmmG = (A8 B8 C8 A9 B9 C9 Aa Ba Aa Ba Ca Ab Bb Cb Ac Bc)
                xmmF = _mm_unpacklo_epi32(xmmF, xmmH);
                // xmmF = (Ca Ab Bb Cb Ac Bc Cc Ad Cc Ad Bd Cd Ae Be Ce Af)
                xmmC = _mm_unpackhi_epi32(xmmC, xmmB);
                // xmmC = (Bd Cd Ae Be Ce Af Bf Cf Bf Cf -- -- -- -- -- --)

                xmmA = _mm_unpacklo_epi64(xmmA, xmmE);
                // xmmA = (A0 B0 C0 A1 B1 C1 A2 B2 C2 A3 B3 C3 A4 B4 C4 A5)
                xmmD = _mm_unpacklo_epi64(xmmD, xmmG);
                // xmmD = (B5 C5 A6 B6 C6 A7 B7 C7 A8 B8 C8 A9 B9 C9 Aa Ba)
                xmmF = _mm_unpacklo_epi64(xmmF, xmmC);
                // xmmF = (Ca Ab Bb Cb Ac Bc Cc Ad Bd Cd Ae Be Ce Af Bf Cf)

                if rcx < SIZEOF_XMMWORD {
                    // .column_st32
                    rcx += rcx * 2; // imul ecx, RGB_PIXELSIZE
                    if rcx >= 2 * SIZEOF_XMMWORD {
                        _mm_storeu_si128(output::<16>(outptr, 0), xmmA);
                        _mm_storeu_si128(output::<16>(outptr, SIZEOF_XMMWORD), xmmD);
                        outptr = &mut outptr[2 * SIZEOF_XMMWORD..]; // outptr
                        xmmA = xmmF;
                        rcx -= 2 * SIZEOF_XMMWORD;
                    } else if rcx >= SIZEOF_XMMWORD {
                        // .column_st16
                        _mm_storeu_si128(output::<16>(outptr, 0), xmmA);
                        outptr = &mut outptr[SIZEOF_XMMWORD..]; // outptr
                        xmmA = xmmD;
                        rcx -= SIZEOF_XMMWORD;
                    }
                    // .column_st15
                    // Store the lower 8 bytes of xmmA to the output when it has enough
                    // space.
                    if rcx >= SIZEOF_MMWORD {
                        _mm_storel_epi64(output::<8>(outptr, 0), xmmA);
                        outptr = &mut outptr[SIZEOF_MMWORD..];
                        rcx -= SIZEOF_MMWORD;
                        xmmA = _mm_srli_si128::<8>(xmmA); // SIZEOF_MMWORD
                    }
                    // .column_st7
                    // Store the lower 4 bytes of xmmA to the output when it has enough
                    // space.
                    if rcx >= SIZEOF_DWORD {
                        *output::<4>(outptr, 0) = _mm_cvtsi128_si32(xmmA).to_le_bytes();
                        outptr = &mut outptr[SIZEOF_DWORD..];
                        rcx -= SIZEOF_DWORD;
                        xmmA = _mm_srli_si128::<4>(xmmA); // SIZEOF_DWORD
                    }
                    // .column_st3
                    // Store the lower 2 bytes of rax to the output when it has enough
                    // space.
                    let mut eax = _mm_cvtsi128_si32(xmmA).cast_unsigned();
                    if rcx >= SIZEOF_WORD {
                        let [al, ah, _, _] = eax.to_le_bytes();
                        *output::<2>(outptr, 0) = [al, ah];
                        outptr = &mut outptr[SIZEOF_WORD..];
                        rcx -= SIZEOF_WORD;
                        eax >>= 16;
                    }
                    // .column_st1
                    // Store the lower 1 byte of rax to the output when it has enough
                    // space.
                    if rcx != 0 {
                        outptr[0] = eax.to_le_bytes()[0];
                    }
                    break;
                }

                _mm_storeu_si128(output::<16>(outptr, 0), xmmA);
                _mm_storeu_si128(output::<16>(outptr, SIZEOF_XMMWORD), xmmD);
                _mm_storeu_si128(output::<16>(outptr, 2 * SIZEOF_XMMWORD), xmmF);
            } else {
                // RGB_PIXELSIZE == 4 ; -----------

                xmm6 = _mm_set1_epi8(-1); // xmm6 = XE = X(02468ace********)
                xmm7 = _mm_set1_epi8(-1); // xmm7 = XO = X(13579bdf********)
                // NOTE: The values of RGB_RED, RGB_GREEN, and RGB_BLUE determine the
                // mapping of components A, B, C, and D to red, green, and blue.
                //
                // xmmA = (A0 A2 A4 A6 A8 Aa Ac Ae) = AE
                // xmmB = (A1 A3 A5 A7 A9 Ab Ad Af) = AO
                // xmmC = (B0 B2 B4 B6 B8 Ba Bc Be) = BE
                // xmmD = (B1 B3 B5 B7 B9 Bb Bd Bf) = BO
                // xmmE = (C0 C2 C4 C6 C8 Ca Cc Ce) = CE
                // xmmF = (C1 C3 C5 C7 C9 Cb Cd Cf) = CO
                // xmmG = (D0 D2 D4 D6 D8 Da Dc De) = DE
                // xmmH = (D1 D3 D5 D7 D9 Db Dd Df) = DO
                let [
                    mut xmmA,
                    mut xmmB,
                    mut xmmC,
                    mut xmmD,
                    mut xmmE,
                    mut xmmF,
                    mut xmmG,
                    mut xmmH,
                ] = jcolsamp::<RGB_RED, RGB_GREEN, RGB_BLUE, _>([
                    xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7,
                ]);

                xmmA = _mm_unpacklo_epi8(xmmA, xmmC);
                // xmmA = (A0 B0 A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be)
                xmmE = _mm_unpacklo_epi8(xmmE, xmmG);
                // xmmE = (C0 D0 C2 D2 C4 D4 C6 D6 C8 D8 Ca Da Cc Dc Ce De)
                xmmB = _mm_unpacklo_epi8(xmmB, xmmD);
                // xmmB = (A1 B1 A3 B3 A5 B5 A7 B7 A9 B9 Ab Bb Ad Bd Af Bf)
                xmmF = _mm_unpacklo_epi8(xmmF, xmmH);
                // xmmF = (C1 D1 C3 D3 C5 D5 C7 D7 C9 D9 Cb Db Cd Dd Cf Df)

                xmmC = xmmA;
                xmmA = _mm_unpacklo_epi16(xmmA, xmmE);
                // xmmA = (A0 B0 C0 D0 A2 B2 C2 D2 A4 B4 C4 D4 A6 B6 C6 D6)
                xmmC = _mm_unpackhi_epi16(xmmC, xmmE);
                // xmmC = (A8 B8 C8 D8 Aa Ba Ca Da Ac Bc Cc Dc Ae Be Ce De)
                xmmG = xmmB;
                xmmB = _mm_unpacklo_epi16(xmmB, xmmF);
                // xmmB = (A1 B1 C1 D1 A3 B3 C3 D3 A5 B5 C5 D5 A7 B7 C7 D7)
                xmmG = _mm_unpackhi_epi16(xmmG, xmmF);
                // xmmG = (A9 B9 C9 D9 Ab Bb Cb Db Ad Bd Cd Dd Af Bf Cf Df)

                xmmD = xmmA;
                xmmA = _mm_unpacklo_epi32(xmmA, xmmB);
                // xmmA = (A0 B0 C0 D0 A1 B1 C1 D1 A2 B2 C2 D2 A3 B3 C3 D3)
                xmmD = _mm_unpackhi_epi32(xmmD, xmmB);
                // xmmD = (A4 B4 C4 D4 A5 B5 C5 D5 A6 B6 C6 D6 A7 B7 C7 D7)
                xmmH = xmmC;
                xmmC = _mm_unpacklo_epi32(xmmC, xmmG);
                // xmmC = (A8 B8 C8 D8 A9 B9 C9 D9 Aa Ba Ca Da Ab Bb Cb Db)
                xmmH = _mm_unpackhi_epi32(xmmH, xmmG);
                // xmmH = (Ac Bc Cc Dc Ad Bd Cd Dd Ae Be Ce De Af Bf Cf Df)

                if rcx < SIZEOF_XMMWORD {
                    // .column_st32
                    if rcx >= SIZEOF_XMMWORD / 2 {
                        _mm_storeu_si128(output::<16>(outptr, 0), xmmA);
                        _mm_storeu_si128(output::<16>(outptr, SIZEOF_XMMWORD), xmmD);
                        outptr = &mut outptr[2 * SIZEOF_XMMWORD..]; // outptr
                        xmmA = xmmC;
                        xmmD = xmmH;
                        rcx -= SIZEOF_XMMWORD / 2;
                    }
                    // .column_st16
                    if rcx >= SIZEOF_XMMWORD / 4 {
                        _mm_storeu_si128(output::<16>(outptr, 0), xmmA);
                        outptr = &mut outptr[SIZEOF_XMMWORD..]; // outptr
                        xmmA = xmmD;
                        rcx -= SIZEOF_XMMWORD / 4;
                    }
                    // .column_st15
                    // Store two pixels (8 bytes) of xmmA to the output when it has enough
                    // space.
                    if rcx >= SIZEOF_XMMWORD / 8 {
                        _mm_storel_epi64(output::<8>(outptr, 0), xmmA);
                        outptr = &mut outptr[SIZEOF_XMMWORD / 8 * 4..];
                        rcx -= SIZEOF_XMMWORD / 8;
                        xmmA = _mm_srli_si128::<8>(xmmA); // SIZEOF_XMMWORD / 8 * 4
                    }
                    // .column_st7
                    // Store one pixel (4 bytes) of xmmA to the output when it has enough
                    // space.
                    if rcx != 0 {
                        *output::<4>(outptr, 0) = _mm_cvtsi128_si32(xmmA).to_le_bytes();
                    }
                    break;
                }

                _mm_storeu_si128(output::<16>(outptr, 0), xmmA);
                _mm_storeu_si128(output::<16>(outptr, SIZEOF_XMMWORD), xmmD);
                _mm_storeu_si128(output::<16>(outptr, 2 * SIZEOF_XMMWORD), xmmC);
                _mm_storeu_si128(output::<16>(outptr, 3 * SIZEOF_XMMWORD), xmmH);
            } // RGB_PIXELSIZE ; ---------------

            // .out0
            outptr = &mut outptr[RGB_PIXELSIZE as usize * SIZEOF_XMMWORD..]; // outptr
            rcx -= SIZEOF_XMMWORD;
            if rcx == 0 {
                break;
            }

            inptr0 = &inptr0[SIZEOF_XMMWORD..]; // inptr0
            inptr1 = &inptr1[SIZEOF_XMMWORD..]; // inptr1
            inptr2 = &inptr2[SIZEOF_XMMWORD..]; // inptr2
        }

        // .nextrow
        row += 1;
        out_row += 1; // output_buf
        rax -= 1; // num_rows
        if rax <= 0 {
            break;
        }
    }

    _mm_sfence(); // flush the write buffer
}
