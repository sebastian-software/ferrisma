// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/looprestoration_tmpl.S` with
//! NEON intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062): the macro
//! `sgr_funcs bpc` as functions generic over the bit depth (const parameter
//! `BPC`, 8 or 16), instantiated by the named entry points
//! `dav1d_sgr_*_8bpc_neon` in `looprestoration.rs` and
//! `dav1d_sgr_*_16bpc_neon` in `looprestoration16.rs`, where the assembly
//! files invoke `sgr_funcs 8` and `sgr_funcs 16`.
//!
//! - `sgr_finish_filter1_2rows_\bpc\()bpc_neon` is `sgr_finish_filter1_2rows::<BPC>`
//! - `sgr_finish_weighted1_\bpc\()bpc_neon` is `sgr_finish_weighted1::<BPC>`
//! - `sgr_finish_filter2_2rows_\bpc\()bpc_neon` is `sgr_finish_filter2_2rows::<BPC>`
//! - `sgr_finish_weighted2_\bpc\()bpc_neon` is `sgr_finish_weighted2::<BPC>`
//! - `sgr_weighted2_\bpc\()bpc_neon` is `sgr_weighted2::<BPC>`
//!
//! Correspondence with the assembly: general registers keep their names;
//! pixel pointers are byte pointers (`*mut u8`), as the assembly's address
//! arithmetic, and a post-incremented address is the access followed by
//! the `add` of the bytes accessed (eight pixels are `BPC` bytes). Vector
//! registers are typed locals named after the registers: the `b` rows
//! (`int16_t`) are `uint16x8_t`, the `a` rows (`int32_t`) are `uint32x4_t`
//! (the arithmetic on them is `add`, `shl`, `mul`, `mla`, `umlsl`). Where a
//! register holds a value of another lane size than the row it otherwise
//! holds in the function, the local carries the arrangement as a suffix
//! (`v25h` is `v25.8h`). The `.if \bpc == 8` branches are `if BPC == 8`,
//! constant after monomorphization; the two that touch pixels are the
//! helpers `sgr_ld1_px` (`ld1` of eight pixels, with the `uxtl` the 8 bpc
//! branch does a few instructions later) and `sgr_st1_px` (`sqxtun` or
//! `umin`, then `st1`). The registers a loop carries from one block to the
//! next (`mov v0.16b, v1.16b` and so on) are the `mut` locals; the saves of
//! `d8`–`d15` are the compiler's.
//!
//! Instruction sequences the assembly repeats are written once:
//! `sgr_finish_filter2_block` is the block `2:` of
//! `sgr_finish_filter2_2rows` up to its two `rshrn` pairs, which
//! `sgr_finish_weighted2` repeats instruction for instruction with the same
//! registers (the two pixel loads in the middle of it are done by the
//! caller just before; there is no store in between);
//! `sgr_weighted2_block` is one row of a block of `sgr_weighted2` (its loop
//! `1:` does two rows with interleaved instructions, its loop `2:` one);
//! `sgr_smull_rshrn11` is the `smull`/`smull2`/`rshrn`/`rshrn2` of the
//! "weighted1" step, `sgr_rshrn_u32` an `rshrn`/`rshrn2` pair.
//!
//! Deviations: `dup v30.8h, w5` of `sgr_finish_weighted1` and
//! `dup v15.8h, w7` of `sgr_finish_weighted2` (`bitdepth_max`) run at
//! 16 bpc only; the assembly runs them at 8 bpc too, on a register the
//! 8 bpc caller does not set, and never reads the result there (one
//! instruction less per call). The multiplications by a weight
//! (`smull`/`smull2` by `w1` in the two `weighted*` functions,
//! `smull`/`smlal` and `smull2`/`smlal2` by `wt[0]`/`wt[1]` in
//! `sgr_weighted2`) are written with the by-element intrinsics on lane 0 of
//! the weight register (`vmull_laneq_s16`, `vmlal_high_laneq_s16`, …): the
//! same instructions with the weight as `v.h[0]` instead of `v.4h`/`v.8h`.
//! In the plain vector form the compiler takes the upper half of the
//! weight register out of the loop and then extracts the upper half of the
//! other operand with an extra instruction. Nothing else.
//!
//! Safety (the callers are the wrappers of `simd/arm/looprestoration.rs`;
//! their `A`/`B` rows have `ARM_BUF_STRIDE` = 400 elements, zero-initialized
//! whole; the two `tmp` buffers have `2 * FILTER_OUT_STRIDE` elements). With
//! `n = ceil(w / 8)` blocks of 8 pixels, every `a` row is read for
//! `8 * n + 4` elements (at most 388); the `b` rows are read for
//! `8 * n + 8` elements by the two `filter*_2rows` functions (at most 392)
//! and for `8 * n + 16` elements by the two `weighted*` functions, which
//! load the next block before they test the count (at most 400, the whole
//! row). `dst`, `src` are read and `dst`, `tmp` written for `8 * n` pixels
//! or elements per row: past the right edge up to the next multiple of 8
//! pixels, which the contract of `src/looprestoration.h` allows (at most
//! 384 elements of the 384 of a `tmp` row). See the functions for the
//! second row with `h <= 1`.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::c_int;

/// `FILTER_OUT_STRIDE`
pub(super) const FILTER_OUT_STRIDE: usize = 384;

/// `ld1 {v.8b}, [x]` and (later in the assembly) `uxtl v.8h, v.8b` at 8 bpc, `ld1 {v.8h}, [x]` at 16 bpc: eight
/// pixels as 16-bit lanes.
#[inline(always)]
unsafe fn sgr_ld1_px<const BPC: u32>(x: *const u8) -> uint16x8_t {
    if BPC == 8 {
        vmovl_u8(vld1_u8(x))
    } else {
        vld1q_u16(x as *const u16)
    }
}

/// `sqxtun vt.8b, v.8h` + `st1 {vt.8b}, [x]` at 8 bpc, `umin vt.8h, v.8h, max.8h` + `st1 {vt.8h}, [x]` at 16 bpc.
#[inline(always)]
unsafe fn sgr_st1_px<const BPC: u32>(x: *mut u8, v: uint16x8_t, max: uint16x8_t) {
    if BPC == 8 {
        vst1_u8(x, vqmovun_s16(vreinterpretq_s16_u16(v)));
    } else {
        vst1q_u16(x as *mut u16, vminq_u16(v, max));
    }
}

/// `rshrn vd.4h, a.4s, #N` + `rshrn2 vd.8h, b.4s, #N`
#[inline(always)]
unsafe fn sgr_rshrn_u32<const N: i32>(a: uint32x4_t, b: uint32x4_t) -> uint16x8_t {
    vrshrn_high_n_u32::<N>(vrshrn_n_u32::<N>(a), b)
}

/// `smull vt.4s, a.4h, w.4h` + `smull2 vu.4s, a.8h, w.8h` + `rshrn vd.4h, vt.4s, #11` + `rshrn2 vd.8h, vu.4s, #11`:
/// `v = t * w`, rounded (the "weighted1" step of the two `weighted*` functions).
#[inline(always)]
unsafe fn sgr_smull_rshrn11(a: uint16x8_t, w: int16x8_t) -> int16x8_t {
    let a = vreinterpretq_s16_u16(a);
    let lo = vmull_laneq_s16::<0>(vget_low_s16(a), w);
    let hi = vmull_high_laneq_s16::<0>(a, w);
    vrshrn_high_n_s32::<11>(vrshrn_n_s32::<11>(lo), hi)
}

/// `sgr_finish_filter1_2rows_\bpc\()bpc_neon`: `void dav1d_sgr_finish_filter1_2rows_Xbpc_neon(int16_t *tmp,
/// const pixel *src, const ptrdiff_t src_stride, const int32_t **a, const int16_t **b, const int w, const int h)`
///
/// # Safety
/// With `n = ceil(w / 8)`: `a[0..4]` are rows readable for `8 * n + 4` elements, `b[0..4]` for `8 * n + 8`; `src`
/// is readable for `8 * n` pixels in its row and, with `h > 1`, in the next (`src + src_stride`; with `h <= 1` the
/// second row reads the first again); `tmp` is writable for `8 * n` elements at `tmp` and at
/// `tmp + FILTER_OUT_STRIDE` (both rows are written whatever `h` is).
#[inline(always)]
pub(super) unsafe fn sgr_finish_filter1_2rows<const BPC: u32>(
    tmp: *mut i16,
    src: *const u8,
    src_stride: isize,
    a: *mut *mut i32,
    b: *mut *mut i16,
    w: c_int,
    h: c_int,
) {
    let (mut x0, mut x1, mut w5, w6) = (tmp as *mut u16, src, w, h);
    let mut x7 = *a as *const u32; // ldp x7, x8, [x3]
    let mut x8 = *a.add(1) as *const u32;
    let mut x9 = *a.add(2) as *const u32; // ldp x9, x3, [x3, #16]
    let mut x3 = *a.add(3) as *const u32;
    let mut x10 = *b as *const u16; // ldp x10, x11, [x4]
    let mut x11 = *b.add(1) as *const u16;
    let mut x12 = *b.add(2) as *const u16; // ldp x12, x4, [x4, #16]
    let mut x4 = *b.add(3) as *const u16;

    // mov x13, #FILTER_OUT_STRIDE; cmp w6, #1
    let mut x2 = x1.wrapping_offset(src_stride); // add x2, x1, x2 (src + stride)
    if w6 <= 1 {
        x2 = x1; // csel x2, x1, x2, le (if (h <= 1) x2 = x1)
    }
    let mut x13 = x0.add(FILTER_OUT_STRIDE); // add x13, x0, x13, lsl #1

    let v30 = vdupq_n_u16(3); // movi v30.8h, #3
    let v31 = vdupq_n_u32(3); // movi v31.4s, #3
                              // 1:
    let uint16x8x2_t(mut v0, mut v1) = vld1q_u16_x2(x10); // ld1 {v0.8h, v1.8h}, [x10], #32
    x10 = x10.add(16);
    let uint16x8x2_t(mut v2, mut v3) = vld1q_u16_x2(x11); // ld1 {v2.8h, v3.8h}, [x11], #32
    x11 = x11.add(16);
    let uint16x8x2_t(mut v4, mut v5) = vld1q_u16_x2(x12); // ld1 {v4.8h, v5.8h}, [x12], #32
    x12 = x12.add(16);
    let uint16x8x2_t(mut v6, mut v7) = vld1q_u16_x2(x4); // ld1 {v6.8h, v7.8h}, [x4], #32
    x4 = x4.add(16);
    let uint32x4x3_t(mut v16, mut v17, mut v18) = vld1q_u32_x3(x7); // ld1 {v16.4s, v17.4s, v18.4s}, [x7], #48
    x7 = x7.add(12);
    let uint32x4x3_t(mut v19, mut v20, mut v21) = vld1q_u32_x3(x8); // ld1 {v19.4s, v20.4s, v21.4s}, [x8], #48
    x8 = x8.add(12);
    let uint32x4x3_t(mut v22, mut v23, mut v24) = vld1q_u32_x3(x9); // ld1 {v22.4s, v23.4s, v24.4s}, [x9], #48
    x9 = x9.add(12);
    let uint32x4x3_t(mut v25, mut v26, mut v27) = vld1q_u32_x3(x3); // ld1 {v25.4s, v26.4s, v27.4s}, [x3], #48
    x3 = x3.add(12);

    loop {
        // 2:
        let v8 = vextq_u16::<1>(v0, v1); // ext v8.16b, v0.16b, v1.16b, #2 ([0][1])
        let v9 = vextq_u16::<1>(v2, v3); // ext v9.16b, v2.16b, v3.16b, #2 ([1][1])
        let v10 = vextq_u16::<1>(v4, v5); // ext v10.16b, v4.16b, v5.16b, #2 ([2][1])
        let v11 = vextq_u16::<2>(v0, v1); // ext v11.16b, v0.16b, v1.16b, #4 ([0][2])
        let v12 = vextq_u16::<2>(v2, v3); // ext v12.16b, v2.16b, v3.16b, #4 ([1][2])
        let v13 = vextq_u16::<2>(v4, v5); // ext v13.16b, v4.16b, v5.16b, #4 ([2][2])

        let v14 = vaddq_u16(v2, v8); // add v14.8h, v2.8h, v8.8h ([1][0] + [0][1])
        let v15 = vaddq_u16(v9, v10); // add v15.8h, v9.8h, v10.8h ([1][1] + [2][1])

        let v28 = vaddq_u16(v0, v11); // add v28.8h, v0.8h, v11.8h ([0][0] + [0][2])
        let v14 = vaddq_u16(v14, v12); // add v14.8h, v14.8h, v12.8h (() + [1][2])
        let v29 = vaddq_u16(v4, v13); // add v29.8h, v4.8h, v13.8h ([2][0] + [2][2])

        let v8 = vextq_u16::<1>(v6, v7); // ext v8.16b, v6.16b, v7.16b, #2 ([3][1])
        let v11 = vextq_u16::<2>(v6, v7); // ext v11.16b, v6.16b, v7.16b, #4 ([3][2])

        let v14 = vaddq_u16(v14, v15); // add v14.8h, v14.8h, v15.8h (mid)
        let v15 = vaddq_u16(v28, v29); // add v15.8h, v28.8h, v29.8h (corners)

        let v28 = vaddq_u16(v4, v9); // add v28.8h, v4.8h, v9.8h ([2][0] + [1][1])
        let v29 = vaddq_u16(v10, v8); // add v29.8h, v10.8h, v8.8h ([2][1] + [3][1])

        v2 = vaddq_u16(v2, v12); // add v2.8h, v2.8h, v12.8h ([1][0] + [1][2])
        let v28 = vaddq_u16(v28, v13); // add v28.8h, v28.8h, v13.8h (() + [2][2])
        v4 = vaddq_u16(v6, v11); // add v4.8h, v6.8h, v11.8h ([3][0] + [3][2])

        v0 = vaddq_u16(v28, v29); // add v0.8h, v28.8h, v29.8h (mid)
        v2 = vaddq_u16(v2, v4); // add v2.8h, v2.8h, v4.8h (corners)

        v4 = vshlq_n_u16::<2>(v14); // shl v4.8h, v14.8h, #2
        v4 = vmlaq_u16(v4, v15, v30); // mla v4.8h, v15.8h, v30.8h (* 3 -> a)

        v0 = vshlq_n_u16::<2>(v0); // shl v0.8h, v0.8h, #2
        v0 = vmlaq_u16(v0, v2, v30); // mla v0.8h, v2.8h, v30.8h (* 3 -> a)

        let v8 = vextq_u32::<1>(v16, v17); // ext v8.16b, v16.16b, v17.16b, #4 ([0][1])
        let v9 = vextq_u32::<1>(v17, v18); // ext v9.16b, v17.16b, v18.16b, #4
        let v10 = vextq_u32::<2>(v16, v17); // ext v10.16b, v16.16b, v17.16b, #8 ([0][2])
        let v11 = vextq_u32::<2>(v17, v18); // ext v11.16b, v17.16b, v18.16b, #8
        let v12 = vextq_u32::<1>(v19, v20); // ext v12.16b, v19.16b, v20.16b, #4 ([1][1])
        let v13 = vextq_u32::<1>(v20, v21); // ext v13.16b, v20.16b, v21.16b, #4
        let v8 = vaddq_u32(v8, v19); // add v8.4s, v8.4s, v19.4s ([0][1] + [1][0])
        let v9 = vaddq_u32(v9, v20); // add v9.4s, v9.4s, v20.4s
        v16 = vaddq_u32(v16, v10); // add v16.4s, v16.4s, v10.4s ([0][0] + [0][2])
        v17 = vaddq_u32(v17, v11); // add v17.4s, v17.4s, v11.4s
        let v14 = vextq_u32::<2>(v19, v20); // ext v14.16b, v19.16b, v20.16b, #8 ([1][2])
        let v15 = vextq_u32::<2>(v20, v21); // ext v15.16b, v20.16b, v21.16b, #8
        v16 = vaddq_u32(v16, v22); // add v16.4s, v16.4s, v22.4s (() + [2][0])
        v17 = vaddq_u32(v17, v23); // add v17.4s, v17.4s, v23.4s
        let v28 = vaddq_u32(v12, v14); // add v28.4s, v12.4s, v14.4s ([1][1] + [1][2])
        let v29 = vaddq_u32(v13, v15); // add v29.4s, v13.4s, v15.4s
        let v10 = vextq_u32::<1>(v22, v23); // ext v10.16b, v22.16b, v23.16b, #4 ([2][1])
        let v11 = vextq_u32::<1>(v23, v24); // ext v11.16b, v23.16b, v24.16b, #4
        let v8 = vaddq_u32(v8, v28); // add v8.4s, v8.4s, v28.4s (mid (incomplete))
        let v9 = vaddq_u32(v9, v29); // add v9.4s, v9.4s, v29.4s

        v19 = vaddq_u32(v19, v14); // add v19.4s, v19.4s, v14.4s ([1][0] + [1][2])
        v20 = vaddq_u32(v20, v15); // add v20.4s, v20.4s, v15.4s
        let v14 = vaddq_u32(v22, v12); // add v14.4s, v22.4s, v12.4s ([2][0] + [1][1])
        let v15 = vaddq_u32(v23, v13); // add v15.4s, v23.4s, v13.4s

        let v12 = vextq_u32::<2>(v22, v23); // ext v12.16b, v22.16b, v23.16b, #8 ([2][2])
        let v13 = vextq_u32::<2>(v23, v24); // ext v13.16b, v23.16b, v24.16b, #8
        let v28 = vextq_u32::<1>(v25, v26); // ext v28.16b, v25.16b, v26.16b, #4 ([3][1])
        let v29 = vextq_u32::<1>(v26, v27); // ext v29.16b, v26.16b, v27.16b, #4
        let v8 = vaddq_u32(v8, v10); // add v8.4s, v8.4s, v10.4s (() + [2][1] = mid)
        let v9 = vaddq_u32(v9, v11); // add v9.4s, v9.4s, v11.4s
        let v14 = vaddq_u32(v14, v10); // add v14.4s, v14.4s, v10.4s (() + [2][1])
        let v15 = vaddq_u32(v15, v11); // add v15.4s, v15.4s, v11.4s
        let v10 = vextq_u32::<2>(v25, v26); // ext v10.16b, v25.16b, v26.16b, #8 ([3][2])
        let v11 = vextq_u32::<2>(v26, v27); // ext v11.16b, v26.16b, v27.16b, #8
        v16 = vaddq_u32(v16, v12); // add v16.4s, v16.4s, v12.4s (() + [2][2] = corner)
        v17 = vaddq_u32(v17, v13); // add v17.4s, v17.4s, v13.4s

        let v12 = vaddq_u32(v12, v28); // add v12.4s, v12.4s, v28.4s ([2][2] + [3][1])
        let v13 = vaddq_u32(v13, v29); // add v13.4s, v13.4s, v29.4s
        v25 = vaddq_u32(v25, v10); // add v25.4s, v25.4s, v10.4s ([3][0] + [3][2])
        v26 = vaddq_u32(v26, v11); // add v26.4s, v26.4s, v11.4s

        let v14 = vaddq_u32(v14, v12); // add v14.4s, v14.4s, v12.4s (mid)
        let v15 = vaddq_u32(v15, v13); // add v15.4s, v15.4s, v13.4s
        v19 = vaddq_u32(v19, v25); // add v19.4s, v19.4s, v25.4s (corner)
        v20 = vaddq_u32(v20, v26); // add v20.4s, v20.4s, v26.4s

        // .if \bpc == 8: ld1 {v25.8b}, [x1], #8; ld1 {v26.8b}, [x2], #8 (uxtl v25.8h, v25.8b; uxtl v26.8h, v26.8b below)
        // .else: ld1 {v25.8h}, [x1], #16; ld1 {v26.8h}, [x2], #16
        let v25h = sgr_ld1_px::<BPC>(x1); // src
        x1 = x1.add(BPC as usize);
        let v26h = sgr_ld1_px::<BPC>(x2);
        x2 = x2.add(BPC as usize);

        let v8 = vshlq_n_u32::<2>(v8); // shl v8.4s, v8.4s, #2
        let v9 = vshlq_n_u32::<2>(v9); // shl v9.4s, v9.4s, #2
        let v8 = vmlaq_u32(v8, v16, v31); // mla v8.4s, v16.4s, v31.4s (* 3 -> b)
        let v9 = vmlaq_u32(v9, v17, v31); // mla v9.4s, v17.4s, v31.4s

        let v14 = vshlq_n_u32::<2>(v14); // shl v14.4s, v14.4s, #2
        let v15 = vshlq_n_u32::<2>(v15); // shl v15.4s, v15.4s, #2
        let v14 = vmlaq_u32(v14, v19, v31); // mla v14.4s, v19.4s, v31.4s (* 3 -> b)
        let v15 = vmlaq_u32(v15, v20, v31); // mla v15.4s, v20.4s, v31.4s

        let v8 = vmlsl_u16(v8, vget_low_u16(v4), vget_low_u16(v25h)); // umlsl v8.4s, v4.4h, v25.4h (b - a * src)
        let v9 = vmlsl_high_u16(v9, v4, v25h); // umlsl2 v9.4s, v4.8h, v25.8h
        let v14 = vmlsl_u16(v14, vget_low_u16(v0), vget_low_u16(v26h)); // umlsl v14.4s, v0.4h, v26.4h (b - a * src)
        let v15 = vmlsl_high_u16(v15, v0, v26h); // umlsl2 v15.4s, v0.8h, v26.8h
        v0 = v1; // mov v0.16b, v1.16b
        let v8h = sgr_rshrn_u32::<9>(v8, v9); // rshrn v8.4h, v8.4s, #9; rshrn2 v8.8h, v9.4s, #9
        v2 = v3; // mov v2.16b, v3.16b
        let v14h = sgr_rshrn_u32::<9>(v14, v15); // rshrn v14.4h, v14.4s, #9; rshrn2 v14.8h, v15.4s, #9
        w5 -= 8; // subs w5, w5, #8
        v4 = v5; // mov v4.16b, v5.16b
        vst1q_u16(x0, v8h); // st1 {v8.8h}, [x0], #16
        x0 = x0.add(8);
        v6 = v7; // mov v6.16b, v7.16b
        vst1q_u16(x13, v14h); // st1 {v14.8h}, [x13], #16
        x13 = x13.add(8);

        if w5 <= 0 {
            break; // b.le 3f
        }
        v16 = v18; // mov v16.16b, v18.16b
        v19 = v21; // mov v19.16b, v21.16b
        v22 = v24; // mov v22.16b, v24.16b
        v25 = v27; // mov v25.16b, v27.16b
        v1 = vld1q_u16(x10); // ld1 {v1.8h}, [x10], #16
        x10 = x10.add(8);
        v3 = vld1q_u16(x11); // ld1 {v3.8h}, [x11], #16
        x11 = x11.add(8);
        v5 = vld1q_u16(x12); // ld1 {v5.8h}, [x12], #16
        x12 = x12.add(8);
        v7 = vld1q_u16(x4); // ld1 {v7.8h}, [x4], #16
        x4 = x4.add(8);
        uint32x4x2_t(v17, v18) = vld1q_u32_x2(x7); // ld1 {v17.4s, v18.4s}, [x7], #32
        x7 = x7.add(8);
        uint32x4x2_t(v20, v21) = vld1q_u32_x2(x8); // ld1 {v20.4s, v21.4s}, [x8], #32
        x8 = x8.add(8);
        uint32x4x2_t(v23, v24) = vld1q_u32_x2(x9); // ld1 {v23.4s, v24.4s}, [x9], #32
        x9 = x9.add(8);
        uint32x4x2_t(v26, v27) = vld1q_u32_x2(x3); // ld1 {v26.4s, v27.4s}, [x3], #32
        x3 = x3.add(8);
        // b 2b
    }
    // 3:
}

/// `sgr_finish_weighted1_\bpc\()bpc_neon`: `void dav1d_sgr_finish_weighted1_Xbpc_neon(pixel *dst,
/// const int32_t **a, const int16_t **b, const int w, const int w1, const int bitdepth_max)`
///
/// # Safety
/// With `n = ceil(w / 8)`: `a[0..3]` are rows readable for `8 * n + 4` elements, `b[0..3]` for `8 * n + 16` (the
/// next block of `b` is loaded before the count is tested); `dst` is readable and writable for `8 * n` pixels.
#[inline(always)]
pub(super) unsafe fn sgr_finish_weighted1<const BPC: u32>(
    dst: *mut u8,
    a: *mut *mut i32,
    b: *mut *mut i16,
    w: c_int,
    w1: c_int,
    bitdepth_max: c_int,
) {
    let (mut x0, mut w3) = (dst, w);
    let mut x7 = *a as *const u32; // ldp x7, x8, [x1]
    let mut x8 = *a.add(1) as *const u32;
    let mut x1 = *a.add(2) as *const u32; // ldr x1, [x1, #16]
    let mut x9 = *b as *const u16; // ldp x9, x10, [x2]
    let mut x10 = *b.add(1) as *const u16;
    let mut x2 = *b.add(2) as *const u16; // ldr x2, [x2, #16]

    let v31 = vdupq_n_s16(w1 as i16); // dup v31.8h, w4
                                      // dup v30.8h, w5 (bitdepth_max; read at 16 bpc only, see the module documentation)
    let v30 = if BPC == 16 {
        vdupq_n_u16(bitdepth_max as u16)
    } else {
        vdupq_n_u16(0)
    };

    let v6 = vdupq_n_u16(3); // movi v6.8h, #3
    let v7 = vdupq_n_u32(3); // movi v7.4s, #3
                             // 1:
    let uint16x8x2_t(mut v0, mut v1) = vld1q_u16_x2(x9); // ld1 {v0.8h, v1.8h}, [x9], #32
    x9 = x9.add(16);
    let uint16x8x2_t(mut v2, mut v3) = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10], #32
    x10 = x10.add(16);
    let uint16x8x2_t(mut v4, mut v5) = vld1q_u16_x2(x2); // ld1 {v4.8h, v5.8h}, [x2], #32
    x2 = x2.add(16);
    let uint32x4x3_t(mut v16, mut v17, mut v18) = vld1q_u32_x3(x7); // ld1 {v16.4s, v17.4s, v18.4s}, [x7], #48
    x7 = x7.add(12);
    let uint32x4x3_t(mut v19, mut v20, mut v21) = vld1q_u32_x3(x8); // ld1 {v19.4s, v20.4s, v21.4s}, [x8], #48
    x8 = x8.add(12);
    let uint32x4x3_t(mut v22, mut v23, mut v24) = vld1q_u32_x3(x1); // ld1 {v22.4s, v23.4s, v24.4s}, [x1], #48
    x1 = x1.add(12);

    loop {
        // 2:
        let v25h = vextq_u16::<1>(v0, v1); // ext v25.16b, v0.16b, v1.16b, #2 (-stride)
        let v26h = vextq_u16::<1>(v2, v3); // ext v26.16b, v2.16b, v3.16b, #2 (0)
        let v27h = vextq_u16::<1>(v4, v5); // ext v27.16b, v4.16b, v5.16b, #2 (+stride)
        let v28h = vextq_u16::<2>(v0, v1); // ext v28.16b, v0.16b, v1.16b, #4 (+1-stride)
        let v29h = vextq_u16::<2>(v2, v3); // ext v29.16b, v2.16b, v3.16b, #4 (+1)
        v2 = vaddq_u16(v2, v25h); // add v2.8h, v2.8h, v25.8h (-1, -stride)
        let v25h = vextq_u16::<2>(v4, v5); // ext v25.16b, v4.16b, v5.16b, #4 (+1+stride)
        let v26h = vaddq_u16(v26h, v27h); // add v26.8h, v26.8h, v27.8h (0, +stride)
        v0 = vaddq_u16(v0, v28h); // add v0.8h, v0.8h, v28.8h (-1-stride, +1-stride)
        v2 = vaddq_u16(v2, v26h); // add v2.8h, v2.8h, v26.8h
        v4 = vaddq_u16(v4, v25h); // add v4.8h, v4.8h, v25.8h (-1+stride, +1+stride)
        v2 = vaddq_u16(v2, v29h); // add v2.8h, v2.8h, v29.8h (+1)
        v0 = vaddq_u16(v0, v4); // add v0.8h, v0.8h, v4.8h

        let v25 = vextq_u32::<1>(v16, v17); // ext v25.16b, v16.16b, v17.16b, #4 (-stride)
        let v26 = vextq_u32::<1>(v17, v18); // ext v26.16b, v17.16b, v18.16b, #4
        v2 = vshlq_n_u16::<2>(v2); // shl v2.8h, v2.8h, #2
        let v27 = vextq_u32::<2>(v16, v17); // ext v27.16b, v16.16b, v17.16b, #8 (+1-stride)
        let v28 = vextq_u32::<2>(v17, v18); // ext v28.16b, v17.16b, v18.16b, #8
        let v29 = vextq_u32::<1>(v19, v20); // ext v29.16b, v19.16b, v20.16b, #4 (0)
        let v4s = vextq_u32::<1>(v20, v21); // ext v4.16b, v20.16b, v21.16b, #4
        v2 = vmlaq_u16(v2, v0, v6); // mla v2.8h, v0.8h, v6.8h (* 3 -> a)
        let v25 = vaddq_u32(v25, v19); // add v25.4s, v25.4s, v19.4s (-stride, -1)
        let v26 = vaddq_u32(v26, v20); // add v26.4s, v26.4s, v20.4s
        v16 = vaddq_u32(v16, v27); // add v16.4s, v16.4s, v27.4s (-1-stride, +1-stride)
        v17 = vaddq_u32(v17, v28); // add v17.4s, v17.4s, v28.4s
        let v27 = vextq_u32::<2>(v19, v20); // ext v27.16b, v19.16b, v20.16b, #8 (+1)
        let v28 = vextq_u32::<2>(v20, v21); // ext v28.16b, v20.16b, v21.16b, #8
        v16 = vaddq_u32(v16, v22); // add v16.4s, v16.4s, v22.4s (-1+stride)
        v17 = vaddq_u32(v17, v23); // add v17.4s, v17.4s, v23.4s
        let v29 = vaddq_u32(v29, v27); // add v29.4s, v29.4s, v27.4s (0, +1)
        let v4s = vaddq_u32(v4s, v28); // add v4.4s, v4.4s, v28.4s
        let v25 = vaddq_u32(v25, v29); // add v25.4s, v25.4s, v29.4s
        let v26 = vaddq_u32(v26, v4s); // add v26.4s, v26.4s, v4.4s
        let v27 = vextq_u32::<1>(v22, v23); // ext v27.16b, v22.16b, v23.16b, #4 (+stride)
        let v28 = vextq_u32::<1>(v23, v24); // ext v28.16b, v23.16b, v24.16b, #4
        let v29 = vextq_u32::<2>(v22, v23); // ext v29.16b, v22.16b, v23.16b, #8 (+1+stride)
        let v4s = vextq_u32::<2>(v23, v24); // ext v4.16b, v23.16b, v24.16b, #8
                                            // .if \bpc == 8: ld1 {v19.8b}, [x0] (uxtl v19.8h, v19.8b below); .else: ld1 {v19.8h}, [x0]
        let v19h = sgr_ld1_px::<BPC>(x0); // src
        let v25 = vaddq_u32(v25, v27); // add v25.4s, v25.4s, v27.4s (+stride)
        let v26 = vaddq_u32(v26, v28); // add v26.4s, v26.4s, v28.4s
        v16 = vaddq_u32(v16, v29); // add v16.4s, v16.4s, v29.4s (+1+stride)
        v17 = vaddq_u32(v17, v4s); // add v17.4s, v17.4s, v4.4s
        let v25 = vshlq_n_u32::<2>(v25); // shl v25.4s, v25.4s, #2
        let v26 = vshlq_n_u32::<2>(v26); // shl v26.4s, v26.4s, #2
        let v25 = vmlaq_u32(v25, v16, v7); // mla v25.4s, v16.4s, v7.4s (* 3 -> b)
        let v26 = vmlaq_u32(v26, v17, v7); // mla v26.4s, v17.4s, v7.4s
        v0 = v1; // mov v0.16b, v1.16b
        let v25 = vmlsl_u16(v25, vget_low_u16(v2), vget_low_u16(v19h)); // umlsl v25.4s, v2.4h, v19.4h (b - a * src)
        let v26 = vmlsl_high_u16(v26, v2, v19h); // umlsl2 v26.4s, v2.8h, v19.8h
        v2 = v3; // mov v2.16b, v3.16b
        let v25h = sgr_rshrn_u32::<9>(v25, v26); // rshrn v25.4h, v25.4s, #9; rshrn2 v25.8h, v26.4s, #9

        w3 -= 8; // subs w3, w3, #8

        // weighted1
        v4 = v5; // mov v4.16b, v5.16b

        v1 = vld1q_u16(x9); // ld1 {v1.8h}, [x9], #16
        x9 = x9.add(8);
        v3 = vld1q_u16(x10); // ld1 {v3.8h}, [x10], #16
        x10 = x10.add(8);
        // smull v26.4s, v25.4h, v31.4h (v = t1 * w1); smull2 v27.4s, v25.8h, v31.8h;
        // rshrn v26.4h, v26.4s, #11; rshrn2 v26.8h, v27.4s, #11
        let v26h = sgr_smull_rshrn11(v25h, v31);
        v5 = vld1q_u16(x2); // ld1 {v5.8h}, [x2], #16
        x2 = x2.add(8);
        let v19h = vsqaddq_u16(v19h, v26h); // usqadd v19.8h, v26.8h
        v16 = v18; // mov v16.16b, v18.16b
        v19 = v21; // mov v19.16b, v21.16b
        v22 = v24; // mov v22.16b, v24.16b
                   // .if \bpc == 8: sqxtun v26.8b, v19.8h; st1 {v26.8b}, [x0], #8
                   // .else: umin v26.8h, v19.8h, v30.8h; st1 {v26.8h}, [x0], #16
        sgr_st1_px::<BPC>(x0, v19h, v30);
        x0 = x0.add(BPC as usize);

        if w3 <= 0 {
            break; // b.le 3f
        }
        uint32x4x2_t(v17, v18) = vld1q_u32_x2(x7); // ld1 {v17.4s, v18.4s}, [x7], #32
        x7 = x7.add(8);
        uint32x4x2_t(v20, v21) = vld1q_u32_x2(x8); // ld1 {v20.4s, v21.4s}, [x8], #32
        x8 = x8.add(8);
        uint32x4x2_t(v23, v24) = vld1q_u32_x2(x1); // ld1 {v23.4s, v24.4s}, [x1], #32
        x1 = x1.add(8);
        // b 2b
    }
    // 3:
}

/// The block `2:` up to the two `rshrn` pairs, which `sgr_finish_filter2_2rows` and `sgr_finish_weighted2` have in
/// common instruction for instruction (the assembly repeats the text; the registers are the same in both): from
/// the `b` rows `v0`/`v1` (-stride) and `v2`/`v3` (+stride), the `a` rows `v16`–`v18` and `v19`–`v21` and the
/// pixels `v31` (first row) and `v30` (second row) to the filtered rows `v16.8h` and `v9.8h`.
#[inline(always)]
unsafe fn sgr_finish_filter2_block(
    v0: uint16x8_t,
    v1: uint16x8_t,
    v2: uint16x8_t,
    v3: uint16x8_t,
    v16: uint32x4_t,
    v17: uint32x4_t,
    v18: uint32x4_t,
    v19: uint32x4_t,
    v20: uint32x4_t,
    v21: uint32x4_t,
    v31: uint16x8_t,
    v30: uint16x8_t,
) -> (uint16x8_t, uint16x8_t) {
    let v4 = vdupq_n_u16(5); // movi v4.8h, #5 (before the loop)
    let v5 = vdupq_n_u32(5); // movi v5.4s, #5
    let v6 = vdupq_n_u16(6); // movi v6.8h, #6
    let v7 = vdupq_n_u32(6); // movi v7.4s, #6

    let v24 = vextq_u16::<2>(v0, v1); // ext v24.16b, v0.16b, v1.16b, #4 (+1-stride)
    let v25 = vextq_u16::<2>(v2, v3); // ext v25.16b, v2.16b, v3.16b, #4 (+1+stride)
    let v22 = vextq_u16::<1>(v0, v1); // ext v22.16b, v0.16b, v1.16b, #2 (-stride)
    let v23 = vextq_u16::<1>(v2, v3); // ext v23.16b, v2.16b, v3.16b, #2 (+stride)
    let v0 = vaddq_u16(v0, v24); // add v0.8h, v0.8h, v24.8h (-1-stride, +1-stride)
    let v25 = vaddq_u16(v2, v25); // add v25.8h, v2.8h, v25.8h (-1+stride, +1+stride)
    let v2 = vaddq_u16(v22, v23); // add v2.8h, v22.8h, v23.8h (-stride, +stride)
    let v0 = vaddq_u16(v0, v25); // add v0.8h, v0.8h, v25.8h

    let v8 = vmulq_u16(v25, v4); // mul v8.8h, v25.8h, v4.8h (* 5)
    let v8 = vmlaq_u16(v8, v23, v6); // mla v8.8h, v23.8h, v6.8h (* 6)

    let v22 = vextq_u32::<1>(v16, v17); // ext v22.16b, v16.16b, v17.16b, #4 (-stride)
    let v23 = vextq_u32::<1>(v17, v18); // ext v23.16b, v17.16b, v18.16b, #4
    let v24 = vextq_u32::<1>(v19, v20); // ext v24.16b, v19.16b, v20.16b, #4 (+stride)
    let v25 = vextq_u32::<1>(v20, v21); // ext v25.16b, v20.16b, v21.16b, #4
    let v26 = vextq_u32::<2>(v16, v17); // ext v26.16b, v16.16b, v17.16b, #8 (+1-stride)
    let v27 = vextq_u32::<2>(v17, v18); // ext v27.16b, v17.16b, v18.16b, #8
    let v28 = vextq_u32::<2>(v19, v20); // ext v28.16b, v19.16b, v20.16b, #8 (+1+stride)
    let v29 = vextq_u32::<2>(v20, v21); // ext v29.16b, v20.16b, v21.16b, #8
    let v0 = vmulq_u16(v0, v4); // mul v0.8h, v0.8h, v4.8h (* 5)
    let v0 = vmlaq_u16(v0, v2, v6); // mla v0.8h, v2.8h, v6.8h (* 6)
                                    // (here: the loads of v31 and v30, done by the caller)
    let v16 = vaddq_u32(v16, v26); // add v16.4s, v16.4s, v26.4s (-1-stride, +1-stride)
    let v17 = vaddq_u32(v17, v27); // add v17.4s, v17.4s, v27.4s
    let v19 = vaddq_u32(v19, v28); // add v19.4s, v19.4s, v28.4s (-1+stride, +1+stride)
    let v20 = vaddq_u32(v20, v29); // add v20.4s, v20.4s, v29.4s
    let v16 = vaddq_u32(v16, v19); // add v16.4s, v16.4s, v19.4s
    let v17 = vaddq_u32(v17, v20); // add v17.4s, v17.4s, v20.4s

    let v9 = vmulq_u32(v19, v5); // mul v9.4s, v19.4s, v5.4s (* 5)
    let v9 = vmlaq_u32(v9, v24, v7); // mla v9.4s, v24.4s, v7.4s (* 6)
    let v10 = vmulq_u32(v20, v5); // mul v10.4s, v20.4s, v5.4s (* 5)
    let v10 = vmlaq_u32(v10, v25, v7); // mla v10.4s, v25.4s, v7.4s (* 6)

    let v22 = vaddq_u32(v22, v24); // add v22.4s, v22.4s, v24.4s (-stride, +stride)
    let v23 = vaddq_u32(v23, v25); // add v23.4s, v23.4s, v25.4s
                                   // This is, surprisingly, faster than other variants where the
                                   // mul+mla pairs are further apart, on Cortex A53.
    let v16 = vmulq_u32(v16, v5); // mul v16.4s, v16.4s, v5.4s (* 5)
    let v16 = vmlaq_u32(v16, v22, v7); // mla v16.4s, v22.4s, v7.4s (* 6)
    let v17 = vmulq_u32(v17, v5); // mul v17.4s, v17.4s, v5.4s (* 5)
    let v17 = vmlaq_u32(v17, v23, v7); // mla v17.4s, v23.4s, v7.4s (* 6)

    // (.if \bpc == 8: uxtl v31.8h, v31.8b; uxtl v30.8h, v30.8b: in sgr_ld1_px)
    let v16 = vmlsl_u16(v16, vget_low_u16(v0), vget_low_u16(v31)); // umlsl v16.4s, v0.4h, v31.4h (b - a * src)
    let v17 = vmlsl_high_u16(v17, v0, v31); // umlsl2 v17.4s, v0.8h, v31.8h
    let v9 = vmlsl_u16(v9, vget_low_u16(v8), vget_low_u16(v30)); // umlsl v9.4s, v8.4h, v30.4h (b - a * src)
    let v10 = vmlsl_high_u16(v10, v8, v30); // umlsl2 v10.4s, v8.8h, v30.8h
    let v16h = sgr_rshrn_u32::<9>(v16, v17); // rshrn v16.4h, v16.4s, #9; rshrn2 v16.8h, v17.4s, #9
    let v9h = sgr_rshrn_u32::<8>(v9, v10); // rshrn v9.4h, v9.4s, #8; rshrn2 v9.8h, v10.4s, #8
    (v16h, v9h)
}

/// `sgr_finish_filter2_2rows_\bpc\()bpc_neon`: `void dav1d_sgr_finish_filter2_2rows_Xbpc_neon(int16_t *tmp,
/// const pixel *src, const ptrdiff_t stride, const int32_t **a, const int16_t **b, const int w, const int h)`
///
/// # Safety
/// With `n = ceil(w / 8)`: `a[0..2]` are rows readable for `8 * n + 4` elements, `b[0..2]` for `8 * n + 8`; `src`
/// is readable for `8 * n` pixels in its row and, with `h > 1`, in the next (with `h <= 1` the second row reads the
/// first again); `tmp` is writable for `8 * n` elements at `tmp` and at `tmp + FILTER_OUT_STRIDE` (both rows are
/// written whatever `h` is).
#[inline(always)]
pub(super) unsafe fn sgr_finish_filter2_2rows<const BPC: u32>(
    tmp: *mut i16,
    src: *const u8,
    stride: isize,
    a: *mut *mut i32,
    b: *mut *mut i16,
    w: c_int,
    h: c_int,
) {
    let (mut x0, mut x1, mut w5, w6) = (tmp as *mut u16, src, w, h);
    let mut x3 = *a as *const u32; // ldp x3, x7, [x3]
    let mut x7 = *a.add(1) as *const u32;
    let mut x4 = *b as *const u16; // ldp x4, x8, [x4]
    let mut x8 = *b.add(1) as *const u16;
    // mov x10, #FILTER_OUT_STRIDE; cmp w6, #1
    let mut x2 = x1.wrapping_offset(stride); // add x2, x1, x2 (src + stride)
    if w6 <= 1 {
        x2 = x1; // csel x2, x1, x2, le (if (h <= 1) x2 = x1)
    }
    let mut x10 = x0.add(FILTER_OUT_STRIDE); // add x10, x0, x10, lsl #1
                                             // movi v4.8h, #5; movi v5.4s, #5; movi v6.8h, #6; movi v7.4s, #6: in sgr_finish_filter2_block
                                             // 1:
    let uint16x8x2_t(mut v0, mut v1) = vld1q_u16_x2(x4); // ld1 {v0.8h, v1.8h}, [x4], #32
    x4 = x4.add(16);
    let uint16x8x2_t(mut v2, mut v3) = vld1q_u16_x2(x8); // ld1 {v2.8h, v3.8h}, [x8], #32
    x8 = x8.add(16);
    let uint32x4x3_t(mut v16, mut v17, mut v18) = vld1q_u32_x3(x3); // ld1 {v16.4s, v17.4s, v18.4s}, [x3], #48
    x3 = x3.add(12);
    let uint32x4x3_t(mut v19, mut v20, mut v21) = vld1q_u32_x3(x7); // ld1 {v19.4s, v20.4s, v21.4s}, [x7], #48
    x7 = x7.add(12);

    loop {
        // 2:
        // .if \bpc == 8: ld1 {v31.8b}, [x1], #8; ld1 {v30.8b}, [x2], #8
        // .else: ld1 {v31.8h}, [x1], #16; ld1 {v30.8h}, [x2], #16
        let v31 = sgr_ld1_px::<BPC>(x1);
        x1 = x1.add(BPC as usize);
        let v30 = sgr_ld1_px::<BPC>(x2);
        x2 = x2.add(BPC as usize);
        let (v16h, v9h) = sgr_finish_filter2_block(v0, v1, v2, v3, v16, v17, v18, v19, v20, v21, v31, v30);
        v0 = v1; // mov v0.16b, v1.16b
        w5 -= 8; // subs w5, w5, #8
        v2 = v3; // mov v2.16b, v3.16b
        vst1q_u16(x0, v16h); // st1 {v16.8h}, [x0], #16
        x0 = x0.add(8);
        vst1q_u16(x10, v9h); // st1 {v9.8h}, [x10], #16
        x10 = x10.add(8);

        if w5 <= 0 {
            break; // b.le 9f
        }
        v16 = v18; // mov v16.16b, v18.16b
        v19 = v21; // mov v19.16b, v21.16b
        v1 = vld1q_u16(x4); // ld1 {v1.8h}, [x4], #16
        x4 = x4.add(8);
        v3 = vld1q_u16(x8); // ld1 {v3.8h}, [x8], #16
        x8 = x8.add(8);
        uint32x4x2_t(v17, v18) = vld1q_u32_x2(x3); // ld1 {v17.4s, v18.4s}, [x3], #32
        x3 = x3.add(8);
        uint32x4x2_t(v20, v21) = vld1q_u32_x2(x7); // ld1 {v20.4s, v21.4s}, [x7], #32
        x7 = x7.add(8);
        // b 2b
    }
    // 9:
}

/// `sgr_finish_weighted2_\bpc\()bpc_neon`: `void dav1d_sgr_finish_weighted2_Xbpc_neon(pixel *dst,
/// const ptrdiff_t stride, const int32_t **a, const int16_t **b, const int w, const int h, const int w1,
/// const int bitdepth_max)`
///
/// # Safety
/// With `n = ceil(w / 8)`: `a[0..2]` are rows readable for `8 * n + 4` elements, `b[0..2]` for `8 * n + 16` (the
/// next block of `b` is loaded before the count is tested); `dst` is readable and writable for `8 * n` pixels in
/// its row and, with `h > 1`, in the next. With `h <= 1` the assembly takes the row `a[0]` as the second row ("any
/// dummy buffer we can clobber"): it reads and writes `8 * n` pixels (at most 384 bytes at 8 bpc, 768 at 16 bpc)
/// at the start of that row, always behind the position up to which the same iteration has loaded the row's
/// `int32_t` values (`48 + 32 * i` bytes loaded against `8 * (i + 1)` or `16 * (i + 1)` bytes written), so `a[0]`
/// must be writable for that many bytes and its contents are lost; the wrapper's row has 1600 bytes and is not
/// read again after such a call.
#[inline(always)]
pub(super) unsafe fn sgr_finish_weighted2<const BPC: u32>(
    dst: *mut u8,
    stride: isize,
    a: *mut *mut i32,
    b: *mut *mut i16,
    w: c_int,
    h: c_int,
    w1: c_int,
    bitdepth_max: c_int,
) {
    let (mut x0, mut w4, w5) = (dst, w, h);
    let v14 = vdupq_n_s16(w1 as i16); // dup v14.8h, w6
                                      // dup v15.8h, w7 (bitdepth_max; read at 16 bpc only, see the module documentation)
    let v15 = if BPC == 16 {
        vdupq_n_u16(bitdepth_max as u16)
    } else {
        vdupq_n_u16(0)
    };

    let mut x2 = *a as *const u32; // ldp x2, x7, [x2]
    let mut x7 = *a.add(1) as *const u32;
    let mut x3 = *b as *const u16; // ldp x3, x8, [x3]
    let mut x8 = *b.add(1) as *const u16;
    // cmp w5, #1
    let mut x1 = x0.wrapping_offset(stride); // add x1, x0, x1 (src + stride)
                                             // if (h <= 1), set the pointer to the second row to any dummy buffer
                                             // we can clobber (x2 in this case)
    if w5 <= 1 {
        x1 = *a as *mut u8; // csel x1, x2, x1, le
    }
    // movi v4.8h, #5; movi v5.4s, #5; movi v6.8h, #6; movi v7.4s, #6: in sgr_finish_filter2_block
    // 1:
    let uint16x8x2_t(mut v0, mut v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
    x3 = x3.add(16);
    let uint16x8x2_t(mut v2, mut v3) = vld1q_u16_x2(x8); // ld1 {v2.8h, v3.8h}, [x8], #32
    x8 = x8.add(16);
    let uint32x4x3_t(mut v16, mut v17, mut v18) = vld1q_u32_x3(x2); // ld1 {v16.4s, v17.4s, v18.4s}, [x2], #48
    x2 = x2.add(12);
    let uint32x4x3_t(mut v19, mut v20, mut v21) = vld1q_u32_x3(x7); // ld1 {v19.4s, v20.4s, v21.4s}, [x7], #48
    x7 = x7.add(12);

    loop {
        // 2:
        // .if \bpc == 8: ld1 {v31.8b}, [x0]; ld1 {v30.8b}, [x1]; .else: ld1 {v31.8h}, [x0]; ld1 {v30.8h}, [x1]
        let v31 = sgr_ld1_px::<BPC>(x0);
        let v30 = sgr_ld1_px::<BPC>(x1);
        let (v16h, v9h) = sgr_finish_filter2_block(v0, v1, v2, v3, v16, v17, v18, v19, v20, v21, v31, v30);
        v0 = v1; // mov v0.16b, v1.16b

        w4 -= 8; // subs w4, w4, #8

        // weighted1
        v2 = v3; // mov v2.16b, v3.16b

        v1 = vld1q_u16(x3); // ld1 {v1.8h}, [x3], #16
        x3 = x3.add(8);
        v3 = vld1q_u16(x8); // ld1 {v3.8h}, [x8], #16
        x8 = x8.add(8);
        // smull v22.4s, v16.4h, v14.4h (v); smull2 v23.4s, v16.8h, v14.8h; rshrn v22.4h, v22.4s, #11;
        // rshrn2 v22.8h, v23.4s, #11
        let v22 = sgr_smull_rshrn11(v16h, v14);
        v16 = v18; // mov v16.16b, v18.16b
                   // smull v24.4s, v9.4h, v14.4h; smull2 v25.4s, v9.8h, v14.8h; rshrn v23.4h, v24.4s, #11;
                   // rshrn2 v23.8h, v25.4s, #11
        let v23 = sgr_smull_rshrn11(v9h, v14);
        v19 = v21; // mov v19.16b, v21.16b
        let v31 = vsqaddq_u16(v31, v22); // usqadd v31.8h, v22.8h
        let v30 = vsqaddq_u16(v30, v23); // usqadd v30.8h, v23.8h
                                         // .if \bpc == 8: sqxtun v22.8b, v31.8h; sqxtun v23.8b, v30.8h; st1 {v22.8b}, [x0], #8; st1 {v23.8b}, [x1], #8
                                         // .else: umin v22.8h, v31.8h, v15.8h; umin v23.8h, v30.8h, v15.8h; st1 {v22.8h}, [x0], #16;
                                         // st1 {v23.8h}, [x1], #16
        sgr_st1_px::<BPC>(x0, v31, v15);
        x0 = x0.add(BPC as usize);
        sgr_st1_px::<BPC>(x1, v30, v15);
        x1 = x1.add(BPC as usize);

        if w4 <= 0 {
            break; // b.le 3f
        }
        uint32x4x2_t(v17, v18) = vld1q_u32_x2(x2); // ld1 {v17.4s, v18.4s}, [x2], #32
        x2 = x2.add(8);
        uint32x4x2_t(v20, v21) = vld1q_u32_x2(x7); // ld1 {v20.4s, v21.4s}, [x7], #32
        x7 = x7.add(8);
        // b 2b
    }
    // 3:
}

/// The block of one row of `sgr_weighted2` (the loop `2:`, and each of the two rows of the loop `1:`): eight
/// pixels at `dst`, `t1` and `t2` to eight pixels at `dst`.
#[inline(always)]
unsafe fn sgr_weighted2_block<const BPC: u32>(
    dst: *mut u8,
    v1: int16x8_t,
    v2: int16x8_t,
    v0: uint16x8_t,
    v30: int16x8_t,
    v31: int16x8_t,
    v29: uint16x8_t,
) {
    let v3 = vmull_laneq_s16::<0>(vget_low_s16(v1), v30); // smull v3.4s, v1.4h, v30.4h (wt[0] * t1)
    let v3 = vmlal_laneq_s16::<0>(v3, vget_low_s16(v2), v31); // smlal v3.4s, v2.4h, v31.4h (wt[1] * t2)
    let v4 = vmull_high_laneq_s16::<0>(v1, v30); // smull2 v4.4s, v1.8h, v30.8h (wt[0] * t1)
    let v4 = vmlal_high_laneq_s16::<0>(v4, v2, v31); // smlal2 v4.4s, v2.8h, v31.8h (wt[1] * t2)
    let v3 = vrshrn_high_n_s32::<11>(vrshrn_n_s32::<11>(v3), v4); // rshrn v3.4h, v3.4s, #11; rshrn2 v3.8h, v4.4s, #11
    let v0 = vsqaddq_u16(v0, v3); // usqadd v0.8h, v3.8h
                                  // .if \bpc == 8: sqxtun v3.8b, v0.8h; st1 {v3.8b}, [x0], #8
                                  // .else: umin v3.8h, v0.8h, v29.8h; st1 {v3.8h}, [x0], #16
    sgr_st1_px::<BPC>(dst, v0, v29);
}

/// `sgr_weighted2_\bpc\()bpc_neon`: `void dav1d_sgr_weighted2_Xbpc_neon(pixel *dst, const ptrdiff_t stride,
/// const int16_t *t1, const int16_t *t2, const int w, const int h, const int16_t wt[2], const int bitdepth_max)`
///
/// # Safety
/// With `n = ceil(w / 8)`: `dst` is readable and writable for `8 * n` pixels in each of its `h` rows; `t1` and `t2`
/// are readable for `8 * n` elements in each of `h` rows of `FILTER_OUT_STRIDE` elements; `wt` has two elements.
#[inline(always)]
pub(super) unsafe fn sgr_weighted2<const BPC: u32>(
    dst: *mut u8,
    stride: isize,
    t1: *const i16,
    t2: *const i16,
    w: c_int,
    h: c_int,
    wt: *const i16,
    bitdepth_max: c_int,
) {
    let (mut x0, mut x2, mut x3, mut w4, mut w5) = (dst, t1, t2, w, h);
    let lt = w5 < 2; // cmp w5, #2
    let mut x10 = x0.wrapping_offset(stride); // add x10, x0, x1
    let mut x12 = x2.wrapping_add(FILTER_OUT_STRIDE); // add x12, x2, #2*FILTER_OUT_STRIDE
    let mut x13 = x3.wrapping_add(FILTER_OUT_STRIDE); // add x13, x3, #2*FILTER_OUT_STRIDE
    let int16x8x2_t(v30, v31) = vld2q_dup_s16(wt); // ld2r {v30.8h, v31.8h}, [x6] (wt[0], wt[1])
                                                   // .if \bpc == 16: dup v29.8h, w7
    let v29 = if BPC == 16 {
        vdupq_n_u16(bitdepth_max as u16)
    } else {
        vdupq_n_u16(0)
    };
    let x8 = 4 * FILTER_OUT_STRIDE as isize; // mov x8, #4*FILTER_OUT_STRIDE
    let x1 = stride << 1; // lsl x1, x1, #1
    let x9 = ((w4 + 7) as u32 as usize & !7) as isize; // add w9, w4, #7; bic x9, x9, #7 (Aligned width)
    let x1 = if BPC == 8 { x1 - x9 } else { x1 - (x9 << 1) }; // sub x1, x1, x9 / sub x1, x1, x9, lsl #1
    let x8 = x8 - (x9 << 1); // sub x8, x8, x9, lsl #1 (bytes)
    let w9 = w4; // mov w9, w4
                 // b.lt 2f
    if !lt {
        loop {
            loop {
                // 1:
                // .if \bpc == 8: ld1 {v0.8b}, [x0]; ld1 {v16.8b}, [x10] (uxtl v0.8h, v0.8b; uxtl v16.8h, v16.8b below)
                // .else: ld1 {v0.8h}, [x0]; ld1 {v16.8h}, [x10]
                let v0 = sgr_ld1_px::<BPC>(x0);
                let v16 = sgr_ld1_px::<BPC>(x10);
                let v1 = vld1q_s16(x2); // ld1 {v1.8h}, [x2], #16
                x2 = x2.add(8);
                let v17 = vld1q_s16(x12); // ld1 {v17.8h}, [x12], #16
                x12 = x12.add(8);
                let v2 = vld1q_s16(x3); // ld1 {v2.8h}, [x3], #16
                x3 = x3.add(8);
                let v18 = vld1q_s16(x13); // ld1 {v18.8h}, [x13], #16
                x13 = x13.add(8);
                w4 -= 8; // subs w4, w4, #8
                sgr_weighted2_block::<BPC>(x0, v1, v2, v0, v30, v31, v29); // (v3, v4; st1 [x0])
                x0 = x0.add(BPC as usize);
                sgr_weighted2_block::<BPC>(x10, v17, v18, v16, v30, v31, v29); // (v19, v20; st1 [x10])
                x10 = x10.add(BPC as usize);
                if w4 <= 0 {
                    break; // b.gt 1b
                }
            }

            w5 -= 2; // subs w5, w5, #2
            if w5 < 1 {
                return; // cmp w5, #1; b.lt 0f
            }
            w4 = w9; // mov w4, w9
            x0 = x0.wrapping_offset(x1); // add x0, x0, x1
            x10 = x10.wrapping_offset(x1); // add x10, x10, x1
            x2 = x2.wrapping_byte_offset(x8); // add x2, x2, x8
            x12 = x12.wrapping_byte_offset(x8); // add x12, x12, x8
            x3 = x3.wrapping_byte_offset(x8); // add x3, x3, x8
            x13 = x13.wrapping_byte_offset(x8); // add x13, x13, x8
            if w5 == 1 {
                break; // b.eq 2f
            }
            // b 1b
        }
    }

    loop {
        // 2:
        // .if \bpc == 8: ld1 {v0.8b}, [x0] (uxtl v0.8h, v0.8b below); .else: ld1 {v0.8h}, [x0]
        let v0 = sgr_ld1_px::<BPC>(x0);
        let v1 = vld1q_s16(x2); // ld1 {v1.8h}, [x2], #16
        x2 = x2.add(8);
        let v2 = vld1q_s16(x3); // ld1 {v2.8h}, [x3], #16
        x3 = x3.add(8);
        w4 -= 8; // subs w4, w4, #8
        sgr_weighted2_block::<BPC>(x0, v1, v2, v0, v30, v31, v29); // (v3, v4; st1 [x0])
        x0 = x0.add(BPC as usize);
        if w4 <= 0 {
            break; // b.gt 2b
        }
    }
    // 0:
}
