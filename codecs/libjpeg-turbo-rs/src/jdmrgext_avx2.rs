// libjpeg-turbo 3.2.1, simd/x86_64/jdmrgext-avx2.asm:
//
// ;
// ; Merged Upsampling/Color Conversion (64-bit AVX2)
// ;
// ; Copyright 2009, 2012 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2012, 2016, 2024-2025, D. R. Commander.
// ; Copyright (C) 2015, Intel Corporation.
// ; Copyright (C) 2018, Matthias Räncker.
// ; Copyright (C) 2023, Aliaksiej Kandracienka.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/x86_64/jdmrgext-avx2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! AVX2. `jdmerge-avx2.asm` includes this file once per RGB pixel layout,
//! as `jdmerge-sse2.asm` includes `jdmrgext-sse2.asm`, and the translation
//! follows `jdmrgext_sse2.rs`: generic kernels that `jdmerge_avx2.rs`
//! instantiates, [`jcolsamp`] for the `ymmA` to `ymmH` names of
//! `jcolsamp.inc`, `RGBX_FILLER_0XFF` (`vpcmpeqb`), locals named after the
//! asm's comments for the general-purpose registers, `outptr` as an index
//! into the output row, and [`input`] and [`output`] for the arrays of the
//! loads and stores of `x86`. The kernels read whole vectors of 32 luma and
//! 32 chroma samples into the `alloc_sarray` padding, as the asm does, and
//! write whole vectors only while 32 pixels remain; the tail stores end at
//! `output_width`.
//!
//! `xmmA` is the low half of `ymmA`: where the tail starts to use it, the
//! kernel takes it with `_mm256_castsi256_si128`, which emits no
//! instruction. The `vmovntdq` stores of an aligned `outptr` and the
//! `vmovdqu` stores of an unaligned one are the same unaligned store
//! (ADR 0008, decision 4), as in `jdmrgext_sse2.rs`, and the `.out0` code
//! follows both pixel sizes once. The `sfence` at `.endcolumn` stays; the
//! `vzeroupper` before `.return` is the compiler's, which emits it at the
//! end of AVX code. The h2v2 kernel calls the h2v1 kernel twice, as the asm
//! does.

#![forbid(unsafe_code)]

#[cfg_attr(
    not(test),
    expect(
        clippy::wildcard_imports,
        reason = "ADR 0008: the intrinsics by their asm names; the x86 import shadows the loads and stores"
    )
)]
use core::arch::x86_64::*;

use crate::jdmerge_avx2::{SCALEBITS, jconst_merged_upsample_avx2};
use crate::jdmrgext_sse2::{input, jcolsamp, output};
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::JSAMPROWS_AT;
use crate::x86::{_mm_storel_epi64, _mm_storeu_si128, _mm256_loadu_si256, _mm256_storeu_si256};

/// `SIZEOF_MMWORD` of `jsimdext.inc`.
const SIZEOF_MMWORD: usize = 8;
/// `SIZEOF_XMMWORD` of `jsimdext.inc`.
const SIZEOF_XMMWORD: usize = 16;
/// `SIZEOF_YMMWORD` of `jsimdext.inc`.
const SIZEOF_YMMWORD: usize = 32;
/// `SIZEOF_DWORD` of `jsimdext.inc`.
const SIZEOF_DWORD: usize = 4;
/// `SIZEOF_WORD` of `jsimdext.inc`.
const SIZEOF_WORD: usize = 2;
/// `BYTE_BIT` of `jsimdext.inc`.
const BYTE_BIT: i32 = 8;

// --------------------------------------------------------------------------
//
// Upsample and color convert for the case of 2:1 horizontal and 1:1 vertical.
//
// GLOBAL(void)
// jsimd_h2v1_merged_upsample_avx2(JDIMENSION output_width,
//                                 JSAMPIMAGE input_buf,
//                                 JDIMENSION in_row_group_ctr,
//                                 JSAMPARRAY output_buf)
//
// r10d = JDIMENSION output_width
// r11 = JSAMPIMAGE input_buf
// r12d = JDIMENSION in_row_group_ctr
// r13 = JSAMPARRAY output_buf

// upstream: simd/x86_64/jdmrgext-avx2.asm WK_NUM
const WK_NUM: usize = 3;

// upstream: simd/x86_64/jdmrgext-avx2.asm jsimd_h2v1_merged_upsample_avx2
/// Upsamples and color-converts the row group `in_row_group_ctr` of
/// `input_buf` into the row `output_buf[0]` of `output_width` pixels in the
/// pixel layout of the parameters.
///
/// # Safety
///
/// AVX2 must be available; `jsimd` selects the kernel only when the CPU has
/// it.
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
#[target_feature(enable = "avx2")]
pub fn jsimd_h2v1_merged_upsample_avx2<
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
    let consts = &jconst_merged_upsample_avx2;
    let mut wk = [_mm256_setzero_si256(); WK_NUM]; // ymmword wk[WK_NUM]

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
        let mut ymm6 = _mm256_loadu_si256(input::<32>(inptr1));
        // ymm6 = Cb(0123456789abcdefghijklmnopqrstuv)
        let mut ymm7 = _mm256_loadu_si256(input::<32>(inptr2));
        // ymm7 = Cr(0123456789abcdefghijklmnopqrstuv)

        let mut ymm1 = _mm256_setzero_si256(); // ymm1 = (all 0's)
        let mut ymm3 = _mm256_set1_epi16(-1);
        ymm3 = _mm256_slli_epi16::<7>(ymm3); // ymm3 = { 0xFF80 0xFF80 0xFF80 0xFF80 .. }

        ymm6 = _mm256_permute4x64_epi64::<0xd8>(ymm6); // ymm6 = Cb(01234567ghijklmn89abcdefopqrstuv)
        ymm7 = _mm256_permute4x64_epi64::<0xd8>(ymm7); // ymm7 = Cr(01234567ghijklmn89abcdefopqrstuv)
        let mut ymm4 = _mm256_unpacklo_epi8(ymm6, ymm1); // ymm4 = Cb(0123456789abcdef) = CbL
        ymm6 = _mm256_unpackhi_epi8(ymm6, ymm1); // ymm6 = Cb(ghijklmnopqrstuv) = CbH
        let mut ymm0 = _mm256_unpacklo_epi8(ymm7, ymm1); // ymm0 = Cr(0123456789abcdef) = CrL
        ymm7 = _mm256_unpackhi_epi8(ymm7, ymm1); // ymm7 = Cr(ghijklmnopqrstuv) = CrH

        let mut ymm5 = _mm256_add_epi16(ymm6, ymm3);
        let mut ymm2 = _mm256_add_epi16(ymm4, ymm3);
        ymm1 = _mm256_add_epi16(ymm7, ymm3);
        ymm3 = _mm256_add_epi16(ymm0, ymm3);

        // (Original)
        // R = Y                + 1.40200 * Cr
        // G = Y - 0.34414 * Cb - 0.71414 * Cr
        // B = Y + 1.77200 * Cb
        //
        // (This implementation)
        // R = Y                + 0.40200 * Cr + Cr
        // G = Y - 0.34414 * Cb + 0.28586 * Cr - Cr
        // B = Y - 0.22800 * Cb + Cb + Cb

        ymm6 = _mm256_add_epi16(ymm5, ymm5); // ymm6 = 2 * CbH
        ymm4 = _mm256_add_epi16(ymm2, ymm2); // ymm4 = 2 * CbL
        ymm7 = _mm256_add_epi16(ymm1, ymm1); // ymm7 = 2 * CrH
        ymm0 = _mm256_add_epi16(ymm3, ymm3); // ymm0 = 2 * CrL

        ymm6 = _mm256_mulhi_epi16(ymm6, _mm256_loadu_si256(&consts.PW_MF0228)); // ymm6 = (2 * CbH * -FIX(0.22800))
        ymm4 = _mm256_mulhi_epi16(ymm4, _mm256_loadu_si256(&consts.PW_MF0228)); // ymm4 = (2 * CbL * -FIX(0.22800))
        ymm7 = _mm256_mulhi_epi16(ymm7, _mm256_loadu_si256(&consts.PW_F0402)); // ymm7 = (2 * CrH * FIX(0.40200))
        ymm0 = _mm256_mulhi_epi16(ymm0, _mm256_loadu_si256(&consts.PW_F0402)); // ymm0 = (2 * CrL * FIX(0.40200))

        ymm6 = _mm256_add_epi16(ymm6, _mm256_loadu_si256(&consts.PW_ONE));
        ymm4 = _mm256_add_epi16(ymm4, _mm256_loadu_si256(&consts.PW_ONE));
        ymm6 = _mm256_srai_epi16::<1>(ymm6); // ymm6 = (CbH * -FIX(0.22800))
        ymm4 = _mm256_srai_epi16::<1>(ymm4); // ymm4 = (CbL * -FIX(0.22800))
        ymm7 = _mm256_add_epi16(ymm7, _mm256_loadu_si256(&consts.PW_ONE));
        ymm0 = _mm256_add_epi16(ymm0, _mm256_loadu_si256(&consts.PW_ONE));
        ymm7 = _mm256_srai_epi16::<1>(ymm7); // ymm7 = (CrH * FIX(0.40200))
        ymm0 = _mm256_srai_epi16::<1>(ymm0); // ymm0 = (CrL * FIX(0.40200))

        ymm6 = _mm256_add_epi16(ymm6, ymm5);
        ymm4 = _mm256_add_epi16(ymm4, ymm2);
        ymm6 = _mm256_add_epi16(ymm6, ymm5); // ymm6 = (CbH * FIX(1.77200)) = (B - Y)H
        ymm4 = _mm256_add_epi16(ymm4, ymm2); // ymm4 = (CbL * FIX(1.77200)) = (B - Y)L
        ymm7 = _mm256_add_epi16(ymm7, ymm1); // ymm7 = (CrH * FIX(1.40200)) = (R - Y)H
        ymm0 = _mm256_add_epi16(ymm0, ymm3); // ymm0 = (CrL * FIX(1.40200)) = (R - Y)L

        wk[0] = ymm6; // wk(0) = (B - Y)H
        wk[1] = ymm7; // wk(1) = (R - Y)H

        ymm6 = _mm256_unpackhi_epi16(ymm5, ymm1);
        ymm5 = _mm256_unpacklo_epi16(ymm5, ymm1);
        ymm5 = _mm256_madd_epi16(ymm5, _mm256_loadu_si256(&consts.PW_MF0344_F0285));
        ymm6 = _mm256_madd_epi16(ymm6, _mm256_loadu_si256(&consts.PW_MF0344_F0285));
        ymm7 = _mm256_unpackhi_epi16(ymm2, ymm3);
        ymm2 = _mm256_unpacklo_epi16(ymm2, ymm3);
        ymm2 = _mm256_madd_epi16(ymm2, _mm256_loadu_si256(&consts.PW_MF0344_F0285));
        ymm7 = _mm256_madd_epi16(ymm7, _mm256_loadu_si256(&consts.PW_MF0344_F0285));

        ymm5 = _mm256_add_epi32(ymm5, _mm256_loadu_si256(&consts.PD_ONEHALF));
        ymm6 = _mm256_add_epi32(ymm6, _mm256_loadu_si256(&consts.PD_ONEHALF));
        ymm5 = _mm256_srai_epi32::<SCALEBITS>(ymm5);
        ymm6 = _mm256_srai_epi32::<SCALEBITS>(ymm6);
        ymm2 = _mm256_add_epi32(ymm2, _mm256_loadu_si256(&consts.PD_ONEHALF));
        ymm7 = _mm256_add_epi32(ymm7, _mm256_loadu_si256(&consts.PD_ONEHALF));
        ymm2 = _mm256_srai_epi32::<SCALEBITS>(ymm2);
        ymm7 = _mm256_srai_epi32::<SCALEBITS>(ymm7);

        ymm5 = _mm256_packs_epi32(ymm5, ymm6);
        // ymm5 = CbH * -FIX(0.344) + CrH * FIX(0.285)
        ymm2 = _mm256_packs_epi32(ymm2, ymm7);
        // ymm2 = CbL * -FIX(0.344) + CrL * FIX(0.285)
        ymm5 = _mm256_sub_epi16(ymm5, ymm1);
        // ymm5 = CbH * -FIX(0.344) + CrH * -FIX(0.714) = (G - Y)H
        ymm2 = _mm256_sub_epi16(ymm2, ymm3);
        // ymm2 = CbL * -FIX(0.344) + CrL * -FIX(0.714) = (G - Y)L

        wk[2] = ymm5; // wk(2) = (G - Y)H

        let mut yctr = 2; // Yctr
        // jmp short .Yloop_1st

        loop {
            // .Yloop_1st
            ymm7 = _mm256_loadu_si256(input::<32>(inptr0));
            // ymm7 = Y(0123456789abcdefghijklmnopqrstuv)

            ymm6 = _mm256_set1_epi16(-1);
            ymm6 = _mm256_srli_epi16::<BYTE_BIT>(ymm6); // ymm6 = { 0xFF 0x00 0xFF 0x00 .. }
            ymm6 = _mm256_and_si256(ymm6, ymm7); // ymm6 = Y(02468acegikmoqsu) = YE
            ymm7 = _mm256_srli_epi16::<BYTE_BIT>(ymm7); // ymm7 = Y(13579bdfhjlnprtv) = YO

            ymm1 = ymm0; // ymm1 = ymm0 = (R - Y)(L / H)
            ymm3 = ymm2; // ymm3 = ymm2 = (G - Y)(L / H)
            ymm5 = ymm4; // ymm5 = ymm4 = (B - Y)(L / H)

            ymm0 = _mm256_add_epi16(ymm0, ymm6);
            // ymm0 = ((R - Y) + YE) = RE = R(02468acegikmoqsu)
            ymm1 = _mm256_add_epi16(ymm1, ymm7);
            // ymm1 = ((R - Y) + YO) = RO = R(13579bdfhjlnprtv)
            ymm0 = _mm256_packus_epi16(ymm0, ymm0); // ymm0 = R(02468ace********gikmoqsu********)
            ymm1 = _mm256_packus_epi16(ymm1, ymm1); // ymm1 = R(13579bdf********hjlnprtv********)

            ymm2 = _mm256_add_epi16(ymm2, ymm6);
            // ymm2 = ((G - Y) + YE) = GE = G(02468acegikmoqsu)
            ymm3 = _mm256_add_epi16(ymm3, ymm7);
            // ymm3 = ((G - Y) + YO) = GO = G(13579bdfhjlnprtv)
            ymm2 = _mm256_packus_epi16(ymm2, ymm2); // ymm2 = G(02468ace********gikmoqsu********)
            ymm3 = _mm256_packus_epi16(ymm3, ymm3); // ymm3 = G(13579bdf********hjlnprtv********)

            ymm4 = _mm256_add_epi16(ymm4, ymm6);
            // ymm4 = ((B - Y) + YE) = BE = B(02468acegikmoqsu)
            ymm5 = _mm256_add_epi16(ymm5, ymm7);
            // ymm5 = ((B - Y) + YO) = BO = B(13579bdfhjlnprtv)
            ymm4 = _mm256_packus_epi16(ymm4, ymm4); // ymm4 = B(02468ace********gikmoqsu********)
            ymm5 = _mm256_packus_epi16(ymm5, ymm5); // ymm5 = B(13579bdf********hjlnprtv********)

            if RGB_PIXELSIZE == 3 {
                // ---------------

                // NOTE: The values of RGB_RED, RGB_GREEN, and RGB_BLUE determine the
                // mapping of components A, B, and C to red, green, and blue.
                //
                // ymmA = (A0 A2 A4 A6 A8 Aa Ac Ae Ag Ai Ak Am Ao Aq As Au) = AE
                // ymmB = (A1 A3 A5 A7 A9 Ab Ad Af Ah Aj Al An Ap Ar At Av) = AO
                // ymmC = (B0 B2 B4 B6 B8 Ba Bc Be Bg Bi Bk Bm Bo Bq Bs Bu) = BE
                // ymmD = (B1 B3 B5 B7 B9 Bb Bd Bf Bh Bj Bl Bn Bp Br Bt Bv) = BO
                // ymmE = (C0 C2 C4 C6 C8 Ca Cc Ce Cg Ci Ck Cm Co Cq Cs Cu) = CE
                // ymmF = (C1 C3 C5 C7 C9 Cb Cd Cf Ch Cj Cl Cn Cp Cr Ct Cv) = CO
                // ymmG = (** ** ** ** ** ** ** ** ** ** ** ** ** ** ** **)
                // ymmH = (** ** ** ** ** ** ** ** ** ** ** ** ** ** ** **)
                let ymm = [ymm0, ymm1, ymm2, ymm3, ymm4, ymm5, ymm6, ymm7];
                let [mut ymmA, mut ymmB] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 0);
                let [mut ymmC, mut ymmD] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 1);
                let [mut ymmE, mut ymmF] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 2);
                let (mut ymmG, mut ymmH);

                ymmA = _mm256_unpacklo_epi8(ymmA, ymmC);
                // ymmA = (A0 B0 A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be
                //         Ag Bg Ai Bi Ak Bk Am Bm Ao Bo Aq Bq As Bs Au Bu)
                ymmE = _mm256_unpacklo_epi8(ymmE, ymmB);
                // ymmE = (C0 A1 C2 A3 C4 A5 C6 A7 C8 A9 Ca Ab Cc Ad Ce Af
                //         Cg Ah Ci Aj Ck Al Cm An Co Ap Cq Ar Cs At Cu Av)
                ymmD = _mm256_unpacklo_epi8(ymmD, ymmF);
                // ymmD = (B1 C1 B3 C3 B5 C5 B7 C7 B9 C9 Bb Cb Bd Cd Bf Cf
                //         Bh Ch Bj Cj Bl Cl Bn Cn Bp Cp Br Cr Bt Ct Bv Cv)

                ymmH = _mm256_srli_si256::<2>(ymmA);
                // ymmH = (A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be Ag Bg
                //         Ai Bi Ak Bk Am Bm Ao Bo Aq Bq As Bs Au Bu -- --)
                ymmG = _mm256_unpackhi_epi16(ymmA, ymmE);
                // ymmG = (A8 B8 C8 A9 Aa Ba Ca Ab Ac Bc Cc Ad Ae Be Ce Af
                //         Ao Bo Co Ap Aq Bq Cq Ar As Bs Cs At Au Bu Cu Av)
                ymmA = _mm256_unpacklo_epi16(ymmA, ymmE);
                // ymmA = (A0 B0 C0 A1 A2 B2 C2 A3 A4 B4 C4 A5 A6 B6 C6 A7
                //         Ag Bg Cg Ah Ai Bi Ci Aj Ak Bk Ck Al Am Bm Cm An)

                ymmE = _mm256_srli_si256::<2>(ymmE);
                // ymmE = (C2 A3 C4 A5 C6 A7 C8 A9 Ca Ab Cc Ad Ce Af Cg Ah
                //         Ci Aj Ck Al Cm An Co Ap Cq Ar Cs At Cu Av -- --)

                ymmB = _mm256_srli_si256::<2>(ymmD);
                // ymmB = (B3 C3 B5 C5 B7 C7 B9 C9 Bb Cb Bd Cd Bf Cf Bh Ch
                //         Bj Cj Bl Cl Bn Cn Bp Cp Br Cr Bt Ct Bv Cv -- --)
                ymmC = _mm256_unpackhi_epi16(ymmD, ymmH);
                // ymmC = (B9 C9 Aa Ba Bb Cb Ac Bc Bd Cd Ae Be Bf Cf Ag Bg
                //         Bp Cp Aq Bq Br Cr As Bs Bt Ct Au Bu Bv Cv -- --)
                ymmD = _mm256_unpacklo_epi16(ymmD, ymmH);
                // ymmD = (B1 C1 A2 B2 B3 C3 A4 B4 B5 C5 A6 B6 B7 C7 A8 B8
                //         Bh Ch Ai Bi Bj Cj Ak Bk Bl Cl Am Bm Bn Cn Ao Bo)

                ymmF = _mm256_unpackhi_epi16(ymmE, ymmB);
                // ymmF = (Ca Ab Bb Cb Cc Ad Bd Cd Ce Af Bf Cf Cg Ah Bh Ch
                //         Cq Ar Br Cr Cs At Bt Ct Cu Av Bv Cv -- -- -- --)
                ymmE = _mm256_unpacklo_epi16(ymmE, ymmB);
                // ymmE = (C2 A3 B3 C3 C4 A5 B5 C5 C6 A7 B7 C7 C8 A9 B9 C9
                //         Ci Aj Bj Cj Ck Al Bl Cl Cm An Bn Cn Co Ap Bp Cp)

                ymmH = _mm256_shuffle_epi32::<0x4E>(ymmA);
                // ymmH = (A4 B4 C4 A5 A6 B6 C6 A7 A0 B0 C0 A1 A2 B2 C2 A3
                //         Ak Bk Ck Al Am Bm Cm An Ag Bg Cg Ah Ai Bi Ci Aj)
                ymmA = _mm256_unpacklo_epi32(ymmA, ymmD);
                // ymmA = (A0 B0 C0 A1 B1 C1 A2 B2 A2 B2 C2 A3 B3 C3 A4 B4
                //         Ag Bg Cg Ah Bh Ch Ai Bi Ai Bi Ci Aj Bj Cj Ak Bk)
                ymmD = _mm256_unpackhi_epi32(ymmD, ymmE);
                // ymmD = (B5 C5 A6 B6 C6 A7 B7 C7 B7 C7 A8 B8 C8 A9 B9 C9
                //         Bl Cl Am Bm Cm An Bn Cn Bn Cn Ao Bo Co Ap Bp Cp)
                ymmE = _mm256_unpacklo_epi32(ymmE, ymmH);
                // ymmE = (C2 A3 B3 C3 A4 B4 C4 A5 C4 A5 B5 C5 A6 B6 C6 A7
                //         Ci Aj Bj Cj Ak Bk Ck Al Ck Al Bl Cl Am Bm Cm An)

                ymmH = _mm256_shuffle_epi32::<0x4E>(ymmG);
                // ymmH = (Ac Bc Cc Ad Ae Be Ce Af A8 B8 C8 A9 Aa Ba Ca Ab
                //         As Bs Cs At Au Bu Cu Av Ao Bo Co Ap Aq Bq Cq Ar)
                ymmG = _mm256_unpacklo_epi32(ymmG, ymmC);
                // ymmG = (A8 B8 C8 A9 B9 C9 Aa Ba Aa Ba Ca Ab Bb Cb Ac Bc
                //         Ao Bo Co Ap Bp Cp Aq Bq Aq Bq Cq Ar Br Cr As Bs)
                ymmC = _mm256_unpackhi_epi32(ymmC, ymmF);
                // ymmC = (Bd Cd Ae Be Ce Af Bf Cf Bf Cf Ag Bg Cg Ah Bh Ch
                //         Bt Ct Au Bu Cu Av Bv Cv Bv Cv -- -- -- -- -- --)
                ymmF = _mm256_unpacklo_epi32(ymmF, ymmH);
                // ymmF = (Ca Ab Bb Cb Ac Bc Cc Ad Cc Ad Bd Cd Ae Be Ce Af
                //         Cq Ar Br Cr As Bs Cs At Cs At Bt Ct Au Bu Cu Av)

                ymmH = _mm256_unpacklo_epi64(ymmA, ymmE);
                // ymmH = (A0 B0 C0 A1 B1 C1 A2 B2 C2 A3 B3 C3 A4 B4 C4 A5
                //         Ag Bg Cg Ah Bh Ch Ai Bi Ci Aj Bj Cj Ak Bk Ck Al)
                ymmG = _mm256_unpacklo_epi64(ymmD, ymmG);
                // ymmG = (B5 C5 A6 B6 C6 A7 B7 C7 A8 B8 C8 A9 B9 C9 Aa Ba
                //         Bl Cl Am Bm Cm An Bn Cn Ao Bo Co Ap Bp Cp Aq Bq)
                ymmC = _mm256_unpacklo_epi64(ymmF, ymmC);
                // ymmC = (Ca Ab Bb Cb Ac Bc Cc Ad Bd Cd Ae Be Ce Af Bf Cf
                //         Cq Ar Br Cr As Bs Cs At Bt Ct Au Bu Cu Av Bv Cv)

                ymmA = _mm256_permute2x128_si256::<0x20>(ymmH, ymmG);
                // ymmA = (A0 B0 C0 A1 B1 C1 A2 B2 C2 A3 B3 C3 A4 B4 C4 A5
                //         B5 C5 A6 B6 C6 A7 B7 C7 A8 B8 C8 A9 B9 C9 Aa Ba)
                ymmD = _mm256_permute2x128_si256::<0x30>(ymmC, ymmH);
                // ymmD = (Ca Ab Bb Cb Ac Bc Cc Ad Bd Cd Ae Be Ce Af Bf Cf
                //         Ag Bg Cg Ah Bh Ch Ai Bi Ci Aj Bj Cj Ak Bk Ck Al)
                ymmF = _mm256_permute2x128_si256::<0x31>(ymmG, ymmC);
                // ymmF = (Bl Cl Am Bm Cm An Bn Cn Ao Bo Co Ap Bp Cp Aq Bq
                //         Cq Ar Br Cr As Bs Cs At Bt Ct Au Bu Cu Av Bv Cv)

                if col < SIZEOF_YMMWORD {
                    // .column_st64
                    col *= 3; // imul ecx, RGB_PIXELSIZE
                    if col >= 2 * SIZEOF_YMMWORD {
                        _mm256_storeu_si256(output::<32>(outrow, outptr), ymmA);
                        _mm256_storeu_si256(output::<32>(outrow, outptr + SIZEOF_YMMWORD), ymmD);
                        outptr += 2 * SIZEOF_YMMWORD; // outptr
                        ymmA = ymmF;
                        col -= 2 * SIZEOF_YMMWORD;
                        // jmp short .column_st31
                    } else if col >= SIZEOF_YMMWORD {
                        // .column_st32
                        _mm256_storeu_si256(output::<32>(outrow, outptr), ymmA);
                        outptr += SIZEOF_YMMWORD; // outptr
                        ymmA = ymmD;
                        col -= SIZEOF_YMMWORD;
                        // jmp short .column_st31
                    }
                    // .column_st31
                    if col >= SIZEOF_XMMWORD {
                        _mm_storeu_si128(
                            output::<16>(outrow, outptr),
                            _mm256_castsi256_si128(ymmA),
                        );
                        outptr += SIZEOF_XMMWORD; // outptr
                        ymmA = _mm256_permute2x128_si256::<1>(ymmA, ymmA);
                        col -= SIZEOF_XMMWORD;
                    }
                    let mut xmmA = _mm256_castsi256_si128(ymmA);
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
                _mm256_storeu_si256(output::<32>(outrow, outptr), ymmA);
                _mm256_storeu_si256(output::<32>(outrow, outptr + SIZEOF_YMMWORD), ymmD);
                _mm256_storeu_si256(output::<32>(outrow, outptr + 2 * SIZEOF_YMMWORD), ymmF);
            } else {
                // RGB_PIXELSIZE == 4 ; -----------

                // RGBX_FILLER_0XFF
                ymm6 = _mm256_set1_epi8(-1);
                // ymm6 = XE = X(02468ace********gikmoqsu********)
                ymm7 = _mm256_set1_epi8(-1);
                // ymm7 = XO = X(13579bdf********hjlnprtv********)
                // NOTE: The values of RGB_RED, RGB_GREEN, and RGB_BLUE determine the
                // mapping of components A, B, C, and D to red, green, and blue.
                //
                // ymmA = (A0 A2 A4 A6 A8 Aa Ac Ae Ag Ai Ak Am Ao Aq As Au) = AE
                // ymmB = (A1 A3 A5 A7 A9 Ab Ad Af Ah Aj Al An Ap Ar At Av) = AO
                // ymmC = (B0 B2 B4 B6 B8 Ba Bc Be Bg Bi Bk Bm Bo Bq Bs Bu) = BE
                // ymmD = (B1 B3 B5 B7 B9 Bb Bd Bf Bh Bj Bl Bn Bp Br Bt Bv) = BO
                // ymmE = (C0 C2 C4 C6 C8 Ca Cc Ce Cg Ci Ck Cm Co Cq Cs Cu) = CE
                // ymmF = (C1 C3 C5 C7 C9 Cb Cd Cf Ch Cj Cl Cn Cp Cr Ct Cv) = CO
                // ymmG = (D0 D2 D4 D6 D8 Da Dc De Dg Di Dk Dm Do Dq Ds Du) = DE
                // ymmH = (D1 D3 D5 D7 D9 Db Dd Df Dh Dj Dl Dn Dp Dr Dt Dv) = DO
                let ymm = [ymm0, ymm1, ymm2, ymm3, ymm4, ymm5, ymm6, ymm7];
                let [mut ymmA, mut ymmB] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 0);
                let [mut ymmC, mut ymmD] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 1);
                let [mut ymmE, mut ymmF] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 2);
                let [mut ymmG, mut ymmH] = jcolsamp::<_, RGB_RED, RGB_GREEN, RGB_BLUE>(ymm, 3);

                ymmA = _mm256_unpacklo_epi8(ymmA, ymmC);
                // ymmA = (A0 B0 A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be
                //         Ag Bg Ai Bi Ak Bk Am Bm Ao Bo Aq Bq As Bs Au Bu)
                ymmE = _mm256_unpacklo_epi8(ymmE, ymmG);
                // ymmE = (C0 D0 C2 D2 C4 D4 C6 D6 C8 D8 Ca Da Cc Dc Ce De
                //         Cg Dg Ci Di Ck Dk Cm Dm Co Do Cq Dq Cs Ds Cu Du)
                ymmB = _mm256_unpacklo_epi8(ymmB, ymmD);
                // ymmB = (A1 B1 A3 B3 A5 B5 A7 B7 A9 B9 Ab Bb Ad Bd Af Bf
                //         Ah Bh Aj Bj Al Bl An Bn Ap Bp Ar Br At Bt Av Bv)
                ymmF = _mm256_unpacklo_epi8(ymmF, ymmH);
                // ymmF = (C1 D1 C3 D3 C5 D5 C7 D7 C9 D9 Cb Db Cd Dd Cf Df
                //         Ch Dh Cj Dj Cl Dl Cn Dn Cp Dp Cr Dr Ct Dt Cv Dv)

                ymmC = _mm256_unpackhi_epi16(ymmA, ymmE);
                // ymmC = (A8 B8 C8 D8 Aa Ba Ca Da Ac Bc Cc Dc Ae Be Ce De
                //         Ao Bo Co Do Aq Bq Cq Dq As Bs Cs Ds Au Bu Cu Du)
                ymmA = _mm256_unpacklo_epi16(ymmA, ymmE);
                // ymmA = (A0 B0 C0 D0 A2 B2 C2 D2 A4 B4 C4 D4 A6 B6 C6 D6
                //         Ag Bg Cg Dg Ai Bi Ci Di Ak Bk Ck Dk Am Bm Cm Dm)
                ymmG = _mm256_unpackhi_epi16(ymmB, ymmF);
                // ymmG = (A9 B9 C9 D9 Ab Bb Cb Db Ad Bd Cd Dd Af Bf Cf Df
                //         Ap Bp Cp Dp Ar Br Cr Dr At Bt Ct Dt Av Bv Cv Dv)
                ymmB = _mm256_unpacklo_epi16(ymmB, ymmF);
                // ymmB = (A1 B1 C1 D1 A3 B3 C3 D3 A5 B5 C5 D5 A7 B7 C7 D7
                //         Ah Bh Ch Dh Aj Bj Cj Dj Al Bl Cl Dl An Bn Cn Dn)

                ymmE = _mm256_unpackhi_epi32(ymmA, ymmB);
                // ymmE = (A4 B4 C4 D4 A5 B5 C5 D5 A6 B6 C6 D6 A7 B7 C7 D7
                //         Ak Bk Ck Dk Al Bl Cl Dl Am Bm Cm Dm An Bn Cn Dn)
                ymmB = _mm256_unpacklo_epi32(ymmA, ymmB);
                // ymmB = (A0 B0 C0 D0 A1 B1 C1 D1 A2 B2 C2 D2 A3 B3 C3 D3
                //         Ag Bg Cg Dg Ah Bh Ch Dh Ai Bi Ci Di Aj Bj Cj Dj)
                ymmF = _mm256_unpackhi_epi32(ymmC, ymmG);
                // ymmF = (Ac Bc Cc Dc Ad Bd Cd Dd Ae Be Ce De Af Bf Cf Df
                //         As Bs Cs Ds At Bt Ct Dt Au Bu Cu Du Av Bv Cv Dv)
                ymmG = _mm256_unpacklo_epi32(ymmC, ymmG);
                // ymmG = (A8 B8 C8 D8 A9 B9 C9 D9 Aa Ba Ca Da Ab Bb Cb Db
                //         Ao Bo Co Do Ap Bp Cp Dp Aq Bq Cq Dq Ar Br Cr Dr)

                ymmA = _mm256_permute2x128_si256::<0x20>(ymmB, ymmE);
                // ymmA = (A0 B0 C0 D0 A1 B1 C1 D1 A2 B2 C2 D2 A3 B3 C3 D3
                //         A4 B4 C4 D4 A5 B5 C5 D5 A6 B6 C6 D6 A7 B7 C7 D7)
                ymmD = _mm256_permute2x128_si256::<0x20>(ymmG, ymmF);
                // ymmD = (A8 B8 C8 D8 A9 B9 C9 D9 Aa Ba Ca Da Ab Bb Cb Db
                //         Ac Bc Cc Dc Ad Bd Cd Dd Ae Be Ce De Af Bf Cf Df)
                ymmC = _mm256_permute2x128_si256::<0x31>(ymmB, ymmE);
                // ymmC = (Ag Bg Cg Dg Ah Bh Ch Dh Ai Bi Ci Di Aj Bj Cj Dj
                //         Ak Bk Ck Dk Al Bl Cl Dl Am Bm Cm Dm An Bn Cn Dn)
                ymmH = _mm256_permute2x128_si256::<0x31>(ymmG, ymmF);
                // ymmH = (Ao Bo Co Do Ap Bp Cp Dp Aq Bq Cq Dq Ar Br Cr Dr
                //         As Bs Cs Ds At Bt Ct Dt Au Bu Cu Du Av Bv Cv Dv)

                if col < SIZEOF_YMMWORD {
                    // .column_st64
                    if col >= SIZEOF_YMMWORD / 2 {
                        _mm256_storeu_si256(output::<32>(outrow, outptr), ymmA);
                        _mm256_storeu_si256(output::<32>(outrow, outptr + SIZEOF_YMMWORD), ymmD);
                        outptr += 2 * SIZEOF_YMMWORD; // outptr
                        ymmA = ymmC;
                        ymmD = ymmH;
                        col -= SIZEOF_YMMWORD / 2;
                    }
                    // .column_st32
                    if col >= SIZEOF_YMMWORD / 4 {
                        _mm256_storeu_si256(output::<32>(outrow, outptr), ymmA);
                        outptr += SIZEOF_YMMWORD; // outptr
                        ymmA = ymmD;
                        col -= SIZEOF_YMMWORD / 4;
                    }
                    // .column_st16
                    if col >= SIZEOF_YMMWORD / 8 {
                        _mm_storeu_si128(
                            output::<16>(outrow, outptr),
                            _mm256_castsi256_si128(ymmA),
                        );
                        outptr += SIZEOF_XMMWORD; // outptr
                        ymmA = _mm256_permute2x128_si256::<1>(ymmA, ymmA);
                        col -= SIZEOF_YMMWORD / 8;
                    }
                    let mut xmmA = _mm256_castsi256_si128(ymmA);
                    // .column_st15
                    // Store two pixels (8 bytes) of ymmA to the output when it has enough
                    // space.
                    if col >= SIZEOF_YMMWORD / 16 {
                        _mm_storel_epi64(output::<8>(outrow, outptr), xmmA);
                        outptr += SIZEOF_YMMWORD / 16 * 4;
                        col -= SIZEOF_YMMWORD / 16;
                        xmmA = _mm_srli_si128::<8>(xmmA); // SIZEOF_YMMWORD / 16 * 4
                    }
                    // .column_st7
                    // Store one pixel (4 bytes) of ymmA to the output when it has enough
                    // space.
                    if col != 0 {
                        *output::<4>(outrow, outptr) = _mm_cvtsi128_si32(xmmA).to_le_bytes();
                    }
                    break 'columnloop;
                }

                // --(unaligned)-----------------
                _mm256_storeu_si256(output::<32>(outrow, outptr), ymmA);
                _mm256_storeu_si256(output::<32>(outrow, outptr + SIZEOF_YMMWORD), ymmD);
                _mm256_storeu_si256(output::<32>(outrow, outptr + 2 * SIZEOF_YMMWORD), ymmC);
                _mm256_storeu_si256(output::<32>(outrow, outptr + 3 * SIZEOF_YMMWORD), ymmH);
            } // RGB_PIXELSIZE ; ---------------

            // .out0
            outptr += RGB_PIXELSIZE as usize * SIZEOF_YMMWORD; // outptr
            col -= SIZEOF_YMMWORD;
            if col == 0 {
                break 'columnloop; // .endcolumn
            }

            inptr0 = &inptr0[SIZEOF_YMMWORD..]; // inptr0
            yctr -= 1;
            if yctr == 0 {
                break;
            }

            // .Yloop_2nd
            ymm0 = wk[1]; // ymm0 = (R - Y)H
            ymm2 = wk[2]; // ymm2 = (G - Y)H
            ymm4 = wk[0]; // ymm4 = (B - Y)H
        }

        inptr1 = &inptr1[SIZEOF_YMMWORD..]; // inptr1
        inptr2 = &inptr2[SIZEOF_YMMWORD..]; // inptr2
    }

    // .endcolumn
    _mm_sfence(); // flush the write buffer
}

// --------------------------------------------------------------------------
//
// Upsample and color convert for the case of 2:1 horizontal and 2:1 vertical.
//
// GLOBAL(void)
// jsimd_h2v2_merged_upsample_avx2(JDIMENSION output_width,
//                                 JSAMPIMAGE input_buf,
//                                 JDIMENSION in_row_group_ctr,
//                                 JSAMPARRAY output_buf)
//
// r10d = JDIMENSION output_width
// r11 = JSAMPIMAGE input_buf
// r12d = JDIMENSION in_row_group_ctr
// r13 = JSAMPARRAY output_buf

// upstream: simd/x86_64/jdmrgext-avx2.asm jsimd_h2v2_merged_upsample_avx2
/// Upsamples and color-converts the row group `in_row_group_ctr` of
/// `input_buf` into the rows `output_buf[0]` and `output_buf[1]` of
/// `output_width` pixels in the pixel layout of the parameters, with two
/// calls of the h2v1 kernel.
///
/// # Safety
///
/// AVX2 must be available; `jsimd` selects the kernel only when the CPU has
/// it.
///
/// # Panics
///
/// When a row is shorter than the asm reads or writes.
#[target_feature(enable = "avx2")]
pub fn jsimd_h2v2_merged_upsample_avx2<
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

    jsimd_h2v1_merged_upsample_avx2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>(
        output_width,
        &image,
        in_row_group_ctr,
        output_buf,
    );

    let outptr1 = &mut output_buf[1..]; // outptr1
    inptr00 = inptr00.offset(1); // inptr01

    image = [inptr00, inptr1, inptr2]; // intpr00, intpr1, intpr2

    jsimd_h2v1_merged_upsample_avx2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>(
        output_width,
        &image,
        in_row_group_ctr,
        outptr1,
    );
}
