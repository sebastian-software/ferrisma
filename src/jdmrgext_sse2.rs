// libjpeg-turbo 3.2.1, simd/x86_64/jdmrgext-sse2.asm:
//
// ;
// ; Merged Upsampling/Color Conversion (64-bit SSE2)
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
//! Translation of `simd/x86_64/jdmrgext-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! SSE2. `jdmerge-sse2.asm` includes this file once per RGB pixel layout,
//! with `RGB_RED`, `RGB_GREEN`, `RGB_BLUE` and `RGB_PIXELSIZE` defined and
//! the functions renamed. Each kernel is a function generic over those
//! values, and `jdmerge_sse2.rs` names the instances as the asm does
//! (ADR 0008, decision 2). `jcolsamp.inc` names the registers of a pixel
//! layout: [`jcolsamp`] gives the pair of registers that holds the component
//! at a byte of the pixel, and where the asm starts to use the names
//! `xmmA` to `xmmH`, the kernel binds them to those registers' values. From
//! there to the end of the loop body the asm uses only those names, and the
//! next pass recomputes every register, so the locals hold what the
//! registers hold. `jsimdcfg.inc` defines `RGBX_FILLER_0XFF`, so the filler
//! bytes of a 4-byte pixel are 0xFF (`pcmpeqb`).
//!
//! The arguments are the slot's (`COLLECT_ARGS`, and the `%ifdef WIN64`
//! argument moves of the h2v2 kernel, become Rust parameters); the
//! general-purpose registers are locals named after the asm's comments:
//! `col`, `inptr0`, `inptr1`, `inptr2`, `outptr` and the `Yctr` in `al`.
//! An input pointer increment is a reslice, `outptr` is an index into the
//! output row, and [`input`] and [`output`] make the arrays that the loads
//! and stores of `x86` take. An input row extends to
//! the end of its storage, which covers the asm's reads of whole vectors
//! past the last column into the `alloc_sarray` padding. The output row may
//! be the application's: the whole-vector stores write only while 16 pixels
//! remain, and the tail stores (`movq`, `movd`, the word and byte stores)
//! end at `output_width`, as in the asm.
//!
//! The asm stores whole vectors with `movntdq` where `outptr` is aligned to
//! 16 bytes and with `movdqu` elsewhere; both are the unaligned store of
//! `x86` (ADR 0008, decision 4), so the alignment test is gone, and the
//! stores after the `.out1` label serve both cases. The `sfence` at
//! `.endcolumn` stays. The `.out0` code that both pixel sizes repeat
//! follows their branches once. The h2v2 kernel calls the h2v1 kernel of
//! its layout twice, as the asm does, with a three-row `JSAMPIMAGE` whose
//! luma list starts at the row group's luma row: the 4-entry stack array of
//! the asm is a 3-entry array, as the fourth entry only aligns the stack.

#![forbid(unsafe_code)]

#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "ADR 0008: the intrinsics by their asm names; the x86 import shadows the loads and stores"
    )
)]
use core::arch::x86_64::*;

use crate::jdmerge_sse2::{SCALEBITS, jconst_merged_upsample_sse2};
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::JSAMPROWS_AT;
use crate::x86::{_mm_loadu_si128, _mm_storel_epi64, _mm_storeu_si128};

/// `SIZEOF_MMWORD` of `jsimdext.inc`.
const SIZEOF_MMWORD: usize = 8;
/// `SIZEOF_XMMWORD` of `jsimdext.inc`.
const SIZEOF_XMMWORD: usize = 16;
/// `SIZEOF_DWORD` of `jsimdext.inc`.
const SIZEOF_DWORD: usize = 4;
/// `SIZEOF_WORD` of `jsimdext.inc`.
const SIZEOF_WORD: usize = 2;
/// `BYTE_BIT` of `jsimdext.inc`.
const BYTE_BIT: i32 = 8;

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
#[must_use]
pub fn output<const N: usize>(outptr: &mut [JSAMPLE], offset: usize) -> &mut [JSAMPLE; N] {
    outptr[offset..]
        .first_chunk_mut()
        .expect("output row shorter than the store writes")
}

// upstream: simd/nasm/jcolsamp.inc mmA
/// `jcolsamp.inc`'s pseudo-registers: of the registers `regs`
/// (`xmm0` to `xmm7` or `ymm0` to `ymm7`), the pair that holds the component
/// at byte `position` of a pixel: `xmmA`/`xmmB` for position 0, `xmmC`/`xmmD`
/// for 1, `xmmE`/`xmmF` for 2 and `xmmG`/`xmmH` for 3. Red is in registers 0
/// and 1, green in 2 and 3, blue in 4 and 5, and the filler in 6 and 7.
#[inline]
#[must_use]
pub fn jcolsamp<T: Copy, const RGB_RED: i32, const RGB_GREEN: i32, const RGB_BLUE: i32>(
    regs: [T; 8],
    position: i32,
) -> [T; 2] {
    let reg = if RGB_RED == position {
        0
    } else if RGB_GREEN == position {
        2
    } else if RGB_BLUE == position {
        4
    } else {
        6
    };
    [regs[reg], regs[reg + 1]]
}

// --------------------------------------------------------------------------
//
// Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
//
// GLOBAL(void)
// jsimd_h2v1_merged_upsample_sse2(JDIMENSION output_width,
//                                 JSAMPIMAGE input_buf,
//                                 JDIMENSION in_row_group_ctr,
//                                 JSAMPARRAY output_buf)
//
// r10d = JDIMENSION output_width
// r11 = JSAMPIMAGE input_buf
// r12d = JDIMENSION in_row_group_ctr
// r13 = JSAMPARRAY output_buf

// upstream: simd/x86_64/jdmrgext-sse2.asm WK_NUM
const WK_NUM: usize = 3;

// upstream: simd/x86_64/jdmrgext-sse2.asm jsimd_h2v1_merged_upsample_sse2
/// Upsamples and color-converts the row group `in_row_group_ctr` of
/// `input_buf` into the row `output_buf[0]` of `output_width` pixels in the
/// pixel layout of the parameters.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
///
/// # Panics
///
/// When a row is shorter than the asm reads or writes.
#[expect(non_snake_case, reason = "jcolsamp.inc register names")]
#[expect(
    clippy::used_underscore_items,
    reason = "the x86 loads and stores have the intrinsics' names"
)]
#[expect(clippy::cast_possible_wrap, reason = "the row index of the JSAMPARRAY")]
#[expect(clippy::too_many_lines, reason = "translated as one function")]
#[expect(clippy::cast_sign_loss, reason = "the asm's RGB_PIXELSIZE operands")]
#[target_feature(enable = "sse2")]
pub fn jsimd_h2v1_merged_upsample_sse2<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_PIXELSIZE: i32,
>(
    output_width: JDIMENSION,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    let consts = &jconst_merged_upsample_sse2;
    let mut wk = [_mm_setzero_si128(); WK_NUM]; // xmmword wk[WK_NUM]

    let mut col = output_width as usize; // col
    if col == 0 {
        return; // .return
    }

    let mut inptr0 = input_buf[0].row(in_row_group_ctr as isize); // inptr0
    let mut inptr1 = input_buf[1].row(in_row_group_ctr as isize); // inptr1
    let mut inptr2 = input_buf[2].row(in_row_group_ctr as isize); // inptr2
    let outrow: &mut [JSAMPLE] = &mut *output_buf[0];
    let mut outptr = 0; // outptr

    'columnloop: loop {
        // .columnloop
        let mut xmm6 = _mm_loadu_si128(input::<16>(inptr1)); // xmm6 = Cb(0123456789abcdef)
        let mut xmm7 = _mm_loadu_si128(input::<16>(inptr2)); // xmm7 = Cr(0123456789abcdef)

        let mut xmm1 = _mm_setzero_si128(); // xmm1 = (all 0's)
        let mut xmm3 = _mm_set1_epi16(-1);
        xmm3 = _mm_slli_epi16::<7>(xmm3); // xmm3 = { 0xFF80 0xFF80 0xFF80 0xFF80 .. }

        let mut xmm4 = xmm6;
        xmm6 = _mm_unpackhi_epi8(xmm6, xmm1); // xmm6 = Cb(89abcdef) = CbH
        xmm4 = _mm_unpacklo_epi8(xmm4, xmm1); // xmm4 = Cb(01234567) = CbL
        let mut xmm0 = xmm7;
        xmm7 = _mm_unpackhi_epi8(xmm7, xmm1); // xmm7 = Cr(89abcdef) = CrH
        xmm0 = _mm_unpacklo_epi8(xmm0, xmm1); // xmm0 = Cr(01234567) = CrL

        xmm6 = _mm_add_epi16(xmm6, xmm3);
        xmm4 = _mm_add_epi16(xmm4, xmm3);
        xmm7 = _mm_add_epi16(xmm7, xmm3);
        xmm0 = _mm_add_epi16(xmm0, xmm3);

        // (Original)
        // R = Y                + 1.40200 * Cr
        // G = Y - 0.34414 * Cb - 0.71414 * Cr
        // B = Y + 1.77200 * Cb
        //
        // (This implementation)
        // R = Y                + 0.40200 * Cr + Cr
        // G = Y - 0.34414 * Cb + 0.28586 * Cr - Cr
        // B = Y - 0.22800 * Cb + Cb + Cb

        let mut xmm5 = xmm6; // xmm5 = CbH
        let mut xmm2 = xmm4; // xmm2 = CbL
        xmm6 = _mm_add_epi16(xmm6, xmm6); // xmm6 = 2 * CbH
        xmm4 = _mm_add_epi16(xmm4, xmm4); // xmm4 = 2 * CbL
        xmm1 = xmm7; // xmm1 = CrH
        xmm3 = xmm0; // xmm3 = CrL
        xmm7 = _mm_add_epi16(xmm7, xmm7); // xmm7 = 2 * CrH
        xmm0 = _mm_add_epi16(xmm0, xmm0); // xmm0 = 2 * CrL

        xmm6 = _mm_mulhi_epi16(xmm6, _mm_loadu_si128(&consts.PW_MF0228)); // xmm6 = (2 * CbH * -FIX(0.22800))
        xmm4 = _mm_mulhi_epi16(xmm4, _mm_loadu_si128(&consts.PW_MF0228)); // xmm4 = (2 * CbL * -FIX(0.22800))
        xmm7 = _mm_mulhi_epi16(xmm7, _mm_loadu_si128(&consts.PW_F0402)); // xmm7 = (2 * CrH * FIX(0.40200))
        xmm0 = _mm_mulhi_epi16(xmm0, _mm_loadu_si128(&consts.PW_F0402)); // xmm0 = (2 * CrL * FIX(0.40200))

        xmm6 = _mm_add_epi16(xmm6, _mm_loadu_si128(&consts.PW_ONE));
        xmm4 = _mm_add_epi16(xmm4, _mm_loadu_si128(&consts.PW_ONE));
        xmm6 = _mm_srai_epi16::<1>(xmm6); // xmm6 = (CbH * -FIX(0.22800))
        xmm4 = _mm_srai_epi16::<1>(xmm4); // xmm4 = (CbL * -FIX(0.22800))
        xmm7 = _mm_add_epi16(xmm7, _mm_loadu_si128(&consts.PW_ONE));
        xmm0 = _mm_add_epi16(xmm0, _mm_loadu_si128(&consts.PW_ONE));
        xmm7 = _mm_srai_epi16::<1>(xmm7); // xmm7 = (CrH * FIX(0.40200))
        xmm0 = _mm_srai_epi16::<1>(xmm0); // xmm0 = (CrL * FIX(0.40200))

        xmm6 = _mm_add_epi16(xmm6, xmm5);
        xmm4 = _mm_add_epi16(xmm4, xmm2);
        xmm6 = _mm_add_epi16(xmm6, xmm5); // xmm6 = (CbH * FIX(1.77200)) = (B - Y)H
        xmm4 = _mm_add_epi16(xmm4, xmm2); // xmm4 = (CbL * FIX(1.77200)) = (B - Y)L
        xmm7 = _mm_add_epi16(xmm7, xmm1); // xmm7 = (CrH * FIX(1.40200)) = (R - Y)H
        xmm0 = _mm_add_epi16(xmm0, xmm3); // xmm0 = (CrL * FIX(1.40200)) = (R - Y)L

        wk[0] = xmm6; // wk(0) = (B - Y)H
        wk[1] = xmm7; // wk(1) = (R - Y)H

        xmm6 = xmm5;
        xmm7 = xmm2;
        xmm5 = _mm_unpacklo_epi16(xmm5, xmm1);
        xmm6 = _mm_unpackhi_epi16(xmm6, xmm1);
        xmm5 = _mm_madd_epi16(xmm5, _mm_loadu_si128(&consts.PW_MF0344_F0285));
        xmm6 = _mm_madd_epi16(xmm6, _mm_loadu_si128(&consts.PW_MF0344_F0285));
        xmm2 = _mm_unpacklo_epi16(xmm2, xmm3);
        xmm7 = _mm_unpackhi_epi16(xmm7, xmm3);
        xmm2 = _mm_madd_epi16(xmm2, _mm_loadu_si128(&consts.PW_MF0344_F0285));
        xmm7 = _mm_madd_epi16(xmm7, _mm_loadu_si128(&consts.PW_MF0344_F0285));

        xmm5 = _mm_add_epi32(xmm5, _mm_loadu_si128(&consts.PD_ONEHALF));
        xmm6 = _mm_add_epi32(xmm6, _mm_loadu_si128(&consts.PD_ONEHALF));
        xmm5 = _mm_srai_epi32::<SCALEBITS>(xmm5);
        xmm6 = _mm_srai_epi32::<SCALEBITS>(xmm6);
        xmm2 = _mm_add_epi32(xmm2, _mm_loadu_si128(&consts.PD_ONEHALF));
        xmm7 = _mm_add_epi32(xmm7, _mm_loadu_si128(&consts.PD_ONEHALF));
        xmm2 = _mm_srai_epi32::<SCALEBITS>(xmm2);
        xmm7 = _mm_srai_epi32::<SCALEBITS>(xmm7);

        xmm5 = _mm_packs_epi32(xmm5, xmm6);
        // xmm5 = CbH * -FIX(0.344) + CrH * FIX(0.285)
        xmm2 = _mm_packs_epi32(xmm2, xmm7);
        // xmm2 = CbL * -FIX(0.344) + CrL * FIX(0.285)
        xmm5 = _mm_sub_epi16(xmm5, xmm1);
        // xmm5 = CbH * -FIX(0.344) + CrH * -FIX(0.714) = (G - Y)H
        xmm2 = _mm_sub_epi16(xmm2, xmm3);
        // xmm2 = CbL * -FIX(0.344) + CrL * -FIX(0.714) = (G - Y)L

        wk[2] = xmm5; // wk(2) = (G - Y)H

        let mut yctr = 2; // Yctr
        // jmp short .Yloop_1st

        loop {
            // .Yloop_1st
            xmm7 = _mm_loadu_si128(input::<16>(inptr0)); // xmm7 = Y(0123456789abcdef)

            xmm6 = _mm_set1_epi16(-1);
            xmm6 = _mm_srli_epi16::<BYTE_BIT>(xmm6); // xmm6 = { 0xFF 0x00 0xFF 0x00 .. }
            xmm6 = _mm_and_si128(xmm6, xmm7); // xmm6 = Y(02468ace) = YE
            xmm7 = _mm_srli_epi16::<BYTE_BIT>(xmm7); // xmm7 = Y(13579bdf) = YO

            xmm1 = xmm0; // xmm1 = xmm0 = (R - Y)(L / H)
            xmm3 = xmm2; // xmm3 = xmm2 = (G - Y)(L / H)
            xmm5 = xmm4; // xmm5 = xmm4 = (B - Y)(L / H)

            xmm0 = _mm_add_epi16(xmm0, xmm6); // xmm0 = ((R - Y) + YE) = RE = R(02468ace)
            xmm1 = _mm_add_epi16(xmm1, xmm7); // xmm1 = ((R - Y) + YO) = RO = R(13579bdf)
            xmm0 = _mm_packus_epi16(xmm0, xmm0); // xmm0 = R(02468ace********)
            xmm1 = _mm_packus_epi16(xmm1, xmm1); // xmm1 = R(13579bdf********)

            xmm2 = _mm_add_epi16(xmm2, xmm6); // xmm2 = ((G - Y) + YE) = GE = G(02468ace)
            xmm3 = _mm_add_epi16(xmm3, xmm7); // xmm3 = ((G - Y) + YO) = GO = G(13579bdf)
            xmm2 = _mm_packus_epi16(xmm2, xmm2); // xmm2 = G(02468ace********)
            xmm3 = _mm_packus_epi16(xmm3, xmm3); // xmm3 = G(13579bdf********)

            xmm4 = _mm_add_epi16(xmm4, xmm6); // xmm4 = ((B - Y) + YE) = BE = B(02468ace)
            xmm5 = _mm_add_epi16(xmm5, xmm7); // xmm5 = ((B - Y) + YO) = BO = B(13579bdf)
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
                let xmm = [xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7];
                let [mut xmmA, mut xmmB] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 0);
                let [mut xmmC, mut xmmD] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 1);
                let [mut xmmE, mut xmmF] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 2);
                let (mut xmmG, mut xmmH);

                xmmA = _mm_unpacklo_epi8(xmmA, xmmC);
                // xmmA = (A0 B0 A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be)
                xmmE = _mm_unpacklo_epi8(xmmE, xmmB);
                // xmmE = (C0 A1 C2 A3 C4 A5 C6 A7 C8 A9 Ca Ab Cc Ad Ce Af)
                xmmD = _mm_unpacklo_epi8(xmmD, xmmF);
                // xmmD = (B1 C1 B3 C3 B5 C5 B7 C7 B9 C9 Bb Cb Bd Cd Bf Cf)

                xmmG = xmmA;
                xmmH = xmmA;
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

                if col < SIZEOF_XMMWORD {
                    // .column_st32
                    col *= 3; // imul ecx, RGB_PIXELSIZE
                    if col >= 2 * SIZEOF_XMMWORD {
                        _mm_storeu_si128(output::<16>(outrow, outptr), xmmA);
                        _mm_storeu_si128(output::<16>(outrow, outptr + SIZEOF_XMMWORD), xmmD);
                        outptr += 2 * SIZEOF_XMMWORD; // outptr
                        xmmA = xmmF;
                        col -= 2 * SIZEOF_XMMWORD;
                        // jmp short .column_st15
                    } else if col >= SIZEOF_XMMWORD {
                        // .column_st16
                        _mm_storeu_si128(output::<16>(outrow, outptr), xmmA);
                        outptr += SIZEOF_XMMWORD; // outptr
                        xmmA = xmmD;
                        col -= SIZEOF_XMMWORD;
                    }
                    // .column_st15
                    // Store the lower 8 bytes of xmmA to the output when it has enough
                    // space.
                    if col >= SIZEOF_MMWORD {
                        _mm_storel_epi64(output::<8>(outrow, outptr), xmmA);
                        outptr += SIZEOF_MMWORD;
                        col -= SIZEOF_MMWORD;
                        xmmA = _mm_srli_si128::<8>(xmmA);
                    }
                    // .column_st7
                    // Store the lower 4 bytes of xmmA to the output when it has enough
                    // space.
                    if col >= SIZEOF_DWORD {
                        *output::<4>(outrow, outptr) = _mm_cvtsi128_si32(xmmA).to_le_bytes();
                        outptr += SIZEOF_DWORD;
                        col -= SIZEOF_DWORD;
                        xmmA = _mm_srli_si128::<4>(xmmA);
                    }
                    // .column_st3
                    // Store the lower 2 bytes of rax to the output when it has enough
                    // space.
                    let mut rax = u64::from(_mm_cvtsi128_si32(xmmA).cast_unsigned());
                    if col >= SIZEOF_WORD {
                        output::<2>(outrow, outptr).copy_from_slice(&rax.to_le_bytes()[..2]);
                        outptr += SIZEOF_WORD;
                        col -= SIZEOF_WORD;
                        rax >>= 16;
                    }
                    // .column_st1
                    // Store the lower 1 byte of rax to the output when it has enough
                    // space.
                    if col != 0 {
                        outrow[outptr] = rax.to_le_bytes()[0];
                    }
                    break 'columnloop;
                }

                // --(unaligned)-----------------
                _mm_storeu_si128(output::<16>(outrow, outptr), xmmA);
                _mm_storeu_si128(output::<16>(outrow, outptr + SIZEOF_XMMWORD), xmmD);
                _mm_storeu_si128(output::<16>(outrow, outptr + 2 * SIZEOF_XMMWORD), xmmF);
            } else {
                // RGB_PIXELSIZE == 4 ; -----------

                // RGBX_FILLER_0XFF
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
                let xmm = [xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7];
                let [mut xmmA, mut xmmB] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 0);
                let [mut xmmC, mut xmmD] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 1);
                let [mut xmmE, mut xmmF] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 2);
                let [mut xmmG, mut xmmH] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(xmm, 3);

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

                if col < SIZEOF_XMMWORD {
                    // .column_st32
                    if col >= SIZEOF_XMMWORD / 2 {
                        _mm_storeu_si128(output::<16>(outrow, outptr), xmmA);
                        _mm_storeu_si128(output::<16>(outrow, outptr + SIZEOF_XMMWORD), xmmD);
                        outptr += 2 * SIZEOF_XMMWORD; // outptr
                        xmmA = xmmC;
                        xmmD = xmmH;
                        col -= SIZEOF_XMMWORD / 2;
                    }
                    // .column_st16
                    if col >= SIZEOF_XMMWORD / 4 {
                        _mm_storeu_si128(output::<16>(outrow, outptr), xmmA);
                        outptr += SIZEOF_XMMWORD; // outptr
                        xmmA = xmmD;
                        col -= SIZEOF_XMMWORD / 4;
                    }
                    // .column_st15
                    // Store two pixels (8 bytes) of xmmA to the output when it has enough
                    // space.
                    if col >= SIZEOF_XMMWORD / 8 {
                        _mm_storel_epi64(output::<8>(outrow, outptr), xmmA);
                        outptr += SIZEOF_XMMWORD / 8 * 4;
                        col -= SIZEOF_XMMWORD / 8;
                        xmmA = _mm_srli_si128::<8>(xmmA); // SIZEOF_XMMWORD / 8 * 4
                    }
                    // .column_st7
                    // Store one pixel (4 bytes) of xmmA to the output when it has enough
                    // space.
                    if col != 0 {
                        *output::<4>(outrow, outptr) = _mm_cvtsi128_si32(xmmA).to_le_bytes();
                    }
                    break 'columnloop;
                }

                // --(unaligned)-----------------
                _mm_storeu_si128(output::<16>(outrow, outptr), xmmA);
                _mm_storeu_si128(output::<16>(outrow, outptr + SIZEOF_XMMWORD), xmmD);
                _mm_storeu_si128(output::<16>(outrow, outptr + 2 * SIZEOF_XMMWORD), xmmC);
                _mm_storeu_si128(output::<16>(outrow, outptr + 3 * SIZEOF_XMMWORD), xmmH);
            } // RGB_PIXELSIZE ; ---------------

            // .out0
            outptr += RGB_PIXELSIZE as usize * SIZEOF_XMMWORD; // outptr
            col -= SIZEOF_XMMWORD;
            if col == 0 {
                break 'columnloop; // .endcolumn
            }

            inptr0 = &inptr0[SIZEOF_XMMWORD..]; // inptr0
            yctr -= 1; // Yctr
            if yctr == 0 {
                break;
            }

            // .Yloop_2nd
            xmm0 = wk[1]; // xmm0 = (R - Y)H
            xmm2 = wk[2]; // xmm2 = (G - Y)H
            xmm4 = wk[0]; // xmm4 = (B - Y)H
        }

        inptr1 = &inptr1[SIZEOF_XMMWORD..]; // inptr1
        inptr2 = &inptr2[SIZEOF_XMMWORD..]; // inptr2
    }

    // .endcolumn
    _mm_sfence(); // flush the write buffer
}

// --------------------------------------------------------------------------
//
// Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
//
// GLOBAL(void)
// jsimd_h2v2_merged_upsample_sse2(JDIMENSION output_width,
//                                 JSAMPIMAGE input_buf,
//                                 JDIMENSION in_row_group_ctr,
//                                 JSAMPARRAY output_buf)
//
// r10d = JDIMENSION output_width
// r11 = JSAMPIMAGE input_buf
// r12d = JDIMENSION in_row_group_ctr
// r13 = JSAMPARRAY output_buf

// upstream: simd/x86_64/jdmrgext-sse2.asm jsimd_h2v2_merged_upsample_sse2
/// Upsamples and color-converts the row group `in_row_group_ctr` of
/// `input_buf` into the rows `output_buf[0]` and `output_buf[1]` of
/// `output_width` pixels in the pixel layout of the parameters, with two
/// calls of the h2v1 kernel.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
///
/// # Panics
///
/// When a row is shorter than the asm reads or writes.
#[target_feature(enable = "sse2")]
pub fn jsimd_h2v2_merged_upsample_sse2<
    const RGB_RED: i32,
    const RGB_GREEN: i32,
    const RGB_BLUE: i32,
    const RGB_PIXELSIZE: i32,
>(
    output_width: JDIMENSION,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    let mut inptr00 = input_buf[0].offset(in_row_group_ctr as usize);
    let inptr1 = input_buf[1];
    let inptr2 = input_buf[2];

    let mut image = [inptr00, inptr1, inptr2]; // intpr00, intpr1, intpr2

    jsimd_h2v1_merged_upsample_sse2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>(
        output_width,
        &image,
        in_row_group_ctr,
        output_buf,
    );

    let outptr1 = &mut output_buf[1..]; // outptr1
    inptr00 = inptr00.offset(1); // inptr01

    image = [inptr00, inptr1, inptr2]; // intpr00, intpr1, intpr2

    jsimd_h2v1_merged_upsample_sse2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>(
        output_width,
        &image,
        in_row_group_ctr,
        outptr1,
    );
}
