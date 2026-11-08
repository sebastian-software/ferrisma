// libjpeg-turbo 3.2.1, simd/x86_64/jdcolext-avx2.asm:
//
// ;
// ; YCbCr-to-RGB Color Conversion (64-bit AVX2)
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
//! Translation of `simd/x86_64/jdcolext-avx2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! AVX2. `jdcolor-avx2.asm` includes this file once per RGB pixel layout,
//! with `RGB_RED`, `RGB_GREEN`, `RGB_BLUE` and `RGB_PIXELSIZE` defined and
//! the function renamed. The kernel is a function generic over those values,
//! and `jdcolor_avx2.rs` names the instances. `jcolsamp.inc` names the
//! registers of each component after its position in the pixel, `ymmA` to
//! `ymmH`; [`jcolsamp`] returns the registers in that order, and the
//! interleaving code reads and writes those names. The `%ifdef
//! RGBX_FILLER_0XFF` branch is taken, as `jsimdcfg.inc` defines it: the
//! fourth byte of a pixel is 0xFF.
//!
//! The registers are local variables named after them, and `wk(i)` is the
//! local array `wk`. In the tail, the asm uses `xmmA`, the low half of
//! `ymmA`; that is `_mm256_castsi256_si128(ymmA)`, which emits no
//! instruction, and from `.column_st15` on, where only the low half is
//! used, the local `xmmA`. The rows are slices; a pointer increment is a
//! reslice, and [`input`] and [`output`] make the arrays that the loads and
//! stores of `x86` take. The input rows extend to the end of their storage,
//! which covers the whole-vector loads past `out_width` into the
//! `alloc_sarray` padding; the stores stay within `out_width`, as the asm's
//! tail does. The asm tests the output address and stores whole vectors
//! with `vmovntdq` when it is aligned and with `vmovdqu` otherwise; both are
//! [`_mm256_storeu_si256`] (ADR 0008, decision 4), so the test is not kept.
//! The `sfence` that ends the non-temporal stores is kept; the jumps to
//! `.return` leave the block `'ret` after it. `vzeroupper` is left to the
//! compiler, which emits it where an AVX function returns (ADR 0008,
//! decision 4). The tail's `vmovd eax, xmmA` and the word and
//! byte stores from `ax` and `al` use the local `eax`, since the row counter
//! `rax` is pushed there. The row counter is `num_rows` zero-extended to 64
//! bits, as `mov eax, r14d` loads it.

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

use crate::jdcolor_avx2::{SCALEBITS, jconst_ycc_rgb_convert_avx2};
use crate::jmorecfg::{JDIMENSION, JSAMPLE};
use crate::jpeglib::{JSAMPROWS, JSAMPROWS_MUT};
use crate::x86::{_mm_storel_epi64, _mm_storeu_si128, _mm256_loadu_si256, _mm256_storeu_si256};

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
// upstream: simd/nasm/jsimdext.inc SIZEOF_YMMWORD
const SIZEOF_YMMWORD: usize = 32;

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

/// `jcolsamp.inc`'s pseudo-registers: `ymmA` to `ymmH` from `ymm0` to
/// `ymm7`, in that order. The components at pixel positions 0, 1, 2 and 3
/// are `ymmA`/`ymmB`, `ymmC`/`ymmD`, `ymmE`/`ymmF` and `ymmG`/`ymmH`; each
/// pair is `ymm0`/`ymm1` for red, `ymm2`/`ymm3` for green, `ymm4`/`ymm5` for
/// blue and `ymm6`/`ymm7` otherwise. Not an upstream item.
#[inline]
#[must_use]
pub fn jcolsamp<const RGB_RED: i32, const RGB_GREEN: i32, const RGB_BLUE: i32, T: Copy>(
    ymm: [T; 8],
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
        ymm[a],
        ymm[a + 1],
        ymm[c],
        ymm[c + 1],
        ymm[e],
        ymm[e + 1],
        ymm[g],
        ymm[g + 1],
    ]
}

// --------------------------------------------------------------------------
//
// Convert some rows of samples to the output colorspace.
//
// GLOBAL(void)
// jsimd_ycc_rgb_convert_avx2(JDIMENSION out_width, JSAMPIMAGE input_buf,
//                            JDIMENSION input_row, JSAMPARRAY output_buf,
//                            int num_rows)
//
// r10d = JDIMENSION out_width
// r11 = JSAMPIMAGE input_buf
// r12d = JDIMENSION input_row
// r13 = JSAMPARRAY output_buf
// r14d = int num_rows

// upstream: simd/x86_64/jdcolext-avx2.asm WK_NUM
const WK_NUM: usize = 2;

// upstream: simd/x86_64/jdcolext-avx2.asm jsimd_ycc_rgb_convert_avx2
/// Converts `num_rows` rows of `out_width` pixels of `input_buf` from row
/// `input_row` on from YCbCr to RGB in the pixel layout of the parameters.
///
/// # Safety
///
/// AVX2 must be available; `jsimd` selects this kernel only when the CPU
/// has it.
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
#[target_feature(enable = "avx2")]
pub fn jsimd_ycc_rgb_convert_avx2<
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
    let consts = &jconst_ycc_rgb_convert_avx2;
    let mut wk = [_mm256_setzero_si256(); WK_NUM];

    // the jumps to .return
    'ret: {
        let num_cols = out_width as usize; // num_cols
        if num_cols == 0 {
            break 'ret;
        }

        // rsi, rbx, rdx: input_buf[0], [1], [2] + input_row
        let mut row = input_row as usize;
        // rdi: output_buf
        let mut out_row = 0;
        let mut rax = i64::from(num_rows.cast_unsigned());
        if rax <= 0 {
            break 'ret;
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
                let mut ymm5 = _mm256_loadu_si256(input::<32>(inptr1));
                // ymm5 = Cb(0123456789abcdefghijklmnopqrstuv)
                let mut ymm1 = _mm256_loadu_si256(input::<32>(inptr2));
                // ymm1 = Cr(0123456789abcdefghijklmnopqrstuv)

                let mut ymm0 = _mm256_set1_epi16(-1);
                let mut ymm7 = _mm256_set1_epi16(-1);
                ymm0 = _mm256_srli_epi16::<BYTE_BIT>(ymm0); // ymm0 = { 0xFF 0x00 0xFF 0x00 .. }
                ymm7 = _mm256_slli_epi16::<7>(ymm7); // ymm7 = { 0xFF80 0xFF80 0xFF80 0xFF80 .. }

                let mut ymm4 = _mm256_and_si256(ymm0, ymm5); // ymm4 = Cb(02468acegikmoqsu) = CbE
                ymm5 = _mm256_srli_epi16::<BYTE_BIT>(ymm5); // ymm5 = Cb(13579bdfhjlnprtv) = CbO
                ymm0 = _mm256_and_si256(ymm0, ymm1); // ymm0 = Cr(02468acegikmoqsu) = CrE
                ymm1 = _mm256_srli_epi16::<BYTE_BIT>(ymm1); // ymm1 = Cr(13579bdfhjlnprtv) = CrO

                let mut ymm2 = _mm256_add_epi16(ymm4, ymm7);
                let mut ymm3 = _mm256_add_epi16(ymm5, ymm7);
                let mut ymm6 = _mm256_add_epi16(ymm0, ymm7);
                ymm7 = _mm256_add_epi16(ymm1, ymm7);

                // (Original)
                // R = Y                + 1.40200 * Cr
                // G = Y - 0.34414 * Cb - 0.71414 * Cr
                // B = Y + 1.77200 * Cb
                //
                // (This implementation)
                // R = Y                + 0.40200 * Cr + Cr
                // G = Y - 0.34414 * Cb + 0.28586 * Cr - Cr
                // B = Y - 0.22800 * Cb + Cb + Cb

                ymm4 = _mm256_add_epi16(ymm2, ymm2); // ymm4 = 2 * CbE
                ymm5 = _mm256_add_epi16(ymm3, ymm3); // ymm5 = 2 * CbO
                ymm0 = _mm256_add_epi16(ymm6, ymm6); // ymm0 = 2 * CrE
                ymm1 = _mm256_add_epi16(ymm7, ymm7); // ymm1 = 2 * CrO

                ymm4 = _mm256_mulhi_epi16(ymm4, _mm256_loadu_si256(&consts.PW_MF0228)); // ymm4 = (2 * CbE * -FIX(0.22800))
                ymm5 = _mm256_mulhi_epi16(ymm5, _mm256_loadu_si256(&consts.PW_MF0228)); // ymm5 = (2 * CbO * -FIX(0.22800))
                ymm0 = _mm256_mulhi_epi16(ymm0, _mm256_loadu_si256(&consts.PW_F0402)); // ymm0 = (2 * CrE * FIX(0.40200))
                ymm1 = _mm256_mulhi_epi16(ymm1, _mm256_loadu_si256(&consts.PW_F0402)); // ymm1 = (2 * CrO * FIX(0.40200))

                ymm4 = _mm256_add_epi16(ymm4, _mm256_loadu_si256(&consts.PW_ONE));
                ymm5 = _mm256_add_epi16(ymm5, _mm256_loadu_si256(&consts.PW_ONE));
                ymm4 = _mm256_srai_epi16::<1>(ymm4); // ymm4 = (CbE * -FIX(0.22800))
                ymm5 = _mm256_srai_epi16::<1>(ymm5); // ymm5 = (CbO * -FIX(0.22800))
                ymm0 = _mm256_add_epi16(ymm0, _mm256_loadu_si256(&consts.PW_ONE));
                ymm1 = _mm256_add_epi16(ymm1, _mm256_loadu_si256(&consts.PW_ONE));
                ymm0 = _mm256_srai_epi16::<1>(ymm0); // ymm0 = (CrE * FIX(0.40200))
                ymm1 = _mm256_srai_epi16::<1>(ymm1); // ymm1 = (CrO * FIX(0.40200))

                ymm4 = _mm256_add_epi16(ymm4, ymm2);
                ymm5 = _mm256_add_epi16(ymm5, ymm3);
                ymm4 = _mm256_add_epi16(ymm4, ymm2); // ymm4 = (CbE * FIX(1.77200)) = (B - Y)E
                ymm5 = _mm256_add_epi16(ymm5, ymm3); // ymm5 = (CbO * FIX(1.77200)) = (B - Y)O
                ymm0 = _mm256_add_epi16(ymm0, ymm6); // ymm0 = (CrE * FIX(1.40200)) = (R - Y)E
                ymm1 = _mm256_add_epi16(ymm1, ymm7); // ymm1 = (CrO * FIX(1.40200)) = (R - Y)O

                wk[0] = ymm4; // wk(0) = (B - Y)E
                wk[1] = ymm5; // wk(1) = (B - Y)O

                ymm4 = _mm256_unpackhi_epi16(ymm2, ymm6);
                ymm2 = _mm256_unpacklo_epi16(ymm2, ymm6);
                ymm2 = _mm256_madd_epi16(ymm2, _mm256_loadu_si256(&consts.PW_MF0344_F0285));
                ymm4 = _mm256_madd_epi16(ymm4, _mm256_loadu_si256(&consts.PW_MF0344_F0285));
                ymm5 = _mm256_unpackhi_epi16(ymm3, ymm7);
                ymm3 = _mm256_unpacklo_epi16(ymm3, ymm7);
                ymm3 = _mm256_madd_epi16(ymm3, _mm256_loadu_si256(&consts.PW_MF0344_F0285));
                ymm5 = _mm256_madd_epi16(ymm5, _mm256_loadu_si256(&consts.PW_MF0344_F0285));

                ymm2 = _mm256_add_epi32(ymm2, _mm256_loadu_si256(&consts.PD_ONEHALF));
                ymm4 = _mm256_add_epi32(ymm4, _mm256_loadu_si256(&consts.PD_ONEHALF));
                ymm2 = _mm256_srai_epi32::<SCALEBITS>(ymm2);
                ymm4 = _mm256_srai_epi32::<SCALEBITS>(ymm4);
                ymm3 = _mm256_add_epi32(ymm3, _mm256_loadu_si256(&consts.PD_ONEHALF));
                ymm5 = _mm256_add_epi32(ymm5, _mm256_loadu_si256(&consts.PD_ONEHALF));
                ymm3 = _mm256_srai_epi32::<SCALEBITS>(ymm3);
                ymm5 = _mm256_srai_epi32::<SCALEBITS>(ymm5);

                ymm2 = _mm256_packs_epi32(ymm2, ymm4);
                // ymm2 = CbE * -FIX(0.344) + CrE * FIX(0.285)
                ymm3 = _mm256_packs_epi32(ymm3, ymm5);
                // ymm3 = CbO * -FIX(0.344) + CrO * FIX(0.285)
                ymm2 = _mm256_sub_epi16(ymm2, ymm6);
                // ymm2 = CbE * -FIX(0.344) + CrE * -FIX(0.714) = (G - Y)E
                ymm3 = _mm256_sub_epi16(ymm3, ymm7);
                // ymm3 = CbO * -FIX(0.344) + CrO * -FIX(0.714) = (G - Y)O

                ymm5 = _mm256_loadu_si256(input::<32>(inptr0));
                // ymm5 = Y(0123456789abcdefghijklmnopqrstuv)

                ymm4 = _mm256_set1_epi16(-1);
                ymm4 = _mm256_srli_epi16::<BYTE_BIT>(ymm4); // ymm4 = { 0xFF 0x00 0xFF 0x00 .. }
                ymm4 = _mm256_and_si256(ymm4, ymm5); // ymm4 = Y(02468acegikmoqsu) = YE
                ymm5 = _mm256_srli_epi16::<BYTE_BIT>(ymm5); // ymm5 = Y(13579bdfhjlnprtv) = YO

                ymm0 = _mm256_add_epi16(ymm0, ymm4);
                // ymm0 = ((R - Y)E + YE) = RE = R(02468acegikmoqsu)
                ymm1 = _mm256_add_epi16(ymm1, ymm5);
                // ymm1 = ((R - Y)O + YO) = RO = R(13579bdfhjlnprtv)
                ymm0 = _mm256_packus_epi16(ymm0, ymm0);
                // ymm0 = R(02468ace********gikmoqsu********)
                ymm1 = _mm256_packus_epi16(ymm1, ymm1);
                // ymm1 = R(13579bdf********hjlnprtv********)

                ymm2 = _mm256_add_epi16(ymm2, ymm4);
                // ymm2 = ((G - Y)E + YE) = GE = G(02468acegikmoqsu)
                ymm3 = _mm256_add_epi16(ymm3, ymm5);
                // ymm3 = ((G - Y)O + YO) = GO = G(13579bdfhjlnprtv)
                ymm2 = _mm256_packus_epi16(ymm2, ymm2);
                // ymm2 = G(02468ace********gikmoqsu********)
                ymm3 = _mm256_packus_epi16(ymm3, ymm3);
                // ymm3 = G(13579bdf********hjlnprtv********)

                ymm4 = _mm256_add_epi16(ymm4, wk[0]);
                // ymm4 = (YE + (B - Y)E) = BE = B(02468acegikmoqsu)
                ymm5 = _mm256_add_epi16(ymm5, wk[1]);
                // ymm5 = (YO + (B - Y)O) = BO = B(13579bdfhjlnprtv)
                ymm4 = _mm256_packus_epi16(ymm4, ymm4);
                // ymm4 = B(02468ace********gikmoqsu********)
                ymm5 = _mm256_packus_epi16(ymm5, ymm5);
                // ymm5 = B(13579bdf********hjlnprtv********)

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
                    let [
                        mut ymmA,
                        mut ymmB,
                        mut ymmC,
                        mut ymmD,
                        mut ymmE,
                        mut ymmF,
                        _,
                        _,
                    ] = jcolsamp::<RGB_RED, RGB_GREEN, RGB_BLUE, _>([
                        ymm0, ymm1, ymm2, ymm3, ymm4, ymm5, ymm6, ymm7,
                    ]);

                    ymmA = _mm256_unpacklo_epi8(ymmA, ymmC);
                    // ymmA = (A0 B0 A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be
                    //         Ag Bg Ai Bi Ak Bk Am Bm Ao Bo Aq Bq As Bs Au Bu)
                    ymmE = _mm256_unpacklo_epi8(ymmE, ymmB);
                    // ymmE = (C0 A1 C2 A3 C4 A5 C6 A7 C8 A9 Ca Ab Cc Ad Ce Af
                    //         Cg Ah Ci Aj Ck Al Cm An Co Ap Cq Ar Cs At Cu Av)
                    ymmD = _mm256_unpacklo_epi8(ymmD, ymmF);
                    // ymmD = (B1 C1 B3 C3 B5 C5 B7 C7 B9 C9 Bb Cb Bd Cd Bf Cf
                    //         Bh Ch Bj Cj Bl Cl Bn Cn Bp Cp Br Cr Bt Ct Bv Cv)

                    let mut ymmH = _mm256_srli_si256::<2>(ymmA);
                    // ymmH = (A2 B2 A4 B4 A6 B6 A8 B8 Aa Ba Ac Bc Ae Be Ag Bg
                    //         Ai Bi Ak Bk Am Bm Ao Bo Aq Bq As Bs Au Bu -- --)
                    let mut ymmG = _mm256_unpackhi_epi16(ymmA, ymmE);
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

                    if rcx < SIZEOF_YMMWORD {
                        // .column_st64
                        rcx += rcx * 2; // imul ecx, RGB_PIXELSIZE
                        if rcx >= 2 * SIZEOF_YMMWORD {
                            _mm256_storeu_si256(output::<32>(outptr, 0), ymmA);
                            _mm256_storeu_si256(output::<32>(outptr, SIZEOF_YMMWORD), ymmD);
                            outptr = &mut outptr[2 * SIZEOF_YMMWORD..]; // outptr
                            ymmA = ymmF;
                            rcx -= 2 * SIZEOF_YMMWORD;
                        } else if rcx >= SIZEOF_YMMWORD {
                            // .column_st32
                            _mm256_storeu_si256(output::<32>(outptr, 0), ymmA);
                            outptr = &mut outptr[SIZEOF_YMMWORD..]; // outptr
                            ymmA = ymmD;
                            rcx -= SIZEOF_YMMWORD;
                        }
                        // .column_st31
                        if rcx >= SIZEOF_XMMWORD {
                            _mm_storeu_si128(output::<16>(outptr, 0), _mm256_castsi256_si128(ymmA));
                            outptr = &mut outptr[SIZEOF_XMMWORD..]; // outptr
                            ymmA = _mm256_permute2x128_si256::<1>(ymmA, ymmA);
                            rcx -= SIZEOF_XMMWORD;
                        }
                        let mut xmmA = _mm256_castsi256_si128(ymmA);
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

                    _mm256_storeu_si256(output::<32>(outptr, 0), ymmA);
                    _mm256_storeu_si256(output::<32>(outptr, SIZEOF_YMMWORD), ymmD);
                    _mm256_storeu_si256(output::<32>(outptr, 2 * SIZEOF_YMMWORD), ymmF);
                } else {
                    // RGB_PIXELSIZE == 4 ; -----------

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
                    let [
                        mut ymmA,
                        mut ymmB,
                        mut ymmC,
                        mut ymmD,
                        mut ymmE,
                        mut ymmF,
                        mut ymmG,
                        mut ymmH,
                    ] = jcolsamp::<RGB_RED, RGB_GREEN, RGB_BLUE, _>([
                        ymm0, ymm1, ymm2, ymm3, ymm4, ymm5, ymm6, ymm7,
                    ]);

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

                    if rcx < SIZEOF_YMMWORD {
                        // .column_st64
                        if rcx >= SIZEOF_YMMWORD / 2 {
                            _mm256_storeu_si256(output::<32>(outptr, 0), ymmA);
                            _mm256_storeu_si256(output::<32>(outptr, SIZEOF_YMMWORD), ymmD);
                            outptr = &mut outptr[2 * SIZEOF_YMMWORD..]; // outptr
                            ymmA = ymmC;
                            ymmD = ymmH;
                            rcx -= SIZEOF_YMMWORD / 2;
                        }
                        // .column_st32
                        if rcx >= SIZEOF_YMMWORD / 4 {
                            _mm256_storeu_si256(output::<32>(outptr, 0), ymmA);
                            outptr = &mut outptr[SIZEOF_YMMWORD..]; // outptr
                            ymmA = ymmD;
                            rcx -= SIZEOF_YMMWORD / 4;
                        }
                        // .column_st16
                        if rcx >= SIZEOF_YMMWORD / 8 {
                            _mm_storeu_si128(output::<16>(outptr, 0), _mm256_castsi256_si128(ymmA));
                            ymmA = _mm256_permute2x128_si256::<1>(ymmA, ymmA);
                            outptr = &mut outptr[SIZEOF_XMMWORD..]; // outptr
                            rcx -= SIZEOF_YMMWORD / 8;
                        }
                        let mut xmmA = _mm256_castsi256_si128(ymmA);
                        // .column_st15
                        // Store two pixels (8 bytes) of ymmA to the output when it has enough
                        // space.
                        if rcx >= SIZEOF_YMMWORD / 16 {
                            _mm_storel_epi64(output::<8>(outptr, 0), xmmA);
                            outptr = &mut outptr[SIZEOF_YMMWORD / 16 * 4..];
                            rcx -= SIZEOF_YMMWORD / 16;
                            xmmA = _mm_srli_si128::<8>(xmmA); // SIZEOF_YMMWORD / 16 * 4
                        }
                        // .column_st7
                        // Store one pixel (4 bytes) of ymmA to the output when it has enough
                        // space.
                        if rcx != 0 {
                            *output::<4>(outptr, 0) = _mm_cvtsi128_si32(xmmA).to_le_bytes();
                        }
                        break;
                    }

                    _mm256_storeu_si256(output::<32>(outptr, 0), ymmA);
                    _mm256_storeu_si256(output::<32>(outptr, SIZEOF_YMMWORD), ymmD);
                    _mm256_storeu_si256(output::<32>(outptr, 2 * SIZEOF_YMMWORD), ymmC);
                    _mm256_storeu_si256(output::<32>(outptr, 3 * SIZEOF_YMMWORD), ymmH);
                } // RGB_PIXELSIZE ; ---------------

                // .out0
                outptr = &mut outptr[RGB_PIXELSIZE as usize * SIZEOF_YMMWORD..]; // outptr
                rcx -= SIZEOF_YMMWORD;
                if rcx == 0 {
                    break;
                }

                inptr0 = &inptr0[SIZEOF_YMMWORD..]; // inptr0
                inptr1 = &inptr1[SIZEOF_YMMWORD..]; // inptr1
                inptr2 = &inptr2[SIZEOF_YMMWORD..]; // inptr2
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

    // .return
}
