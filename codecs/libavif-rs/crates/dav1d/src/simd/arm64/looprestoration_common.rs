// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/looprestoration_common.S` with
//! NEON intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062): the vertical
//! box sums of the self-guided filter with the `a`/`b` computation, shared
//! by both bit depths (`bitdepth_max` is a parameter).
//!
//! - `x_by_x_tables` is `X_BY_X_TABLES` (`RangeMins`, `DiffMasks`, `RefLo`,
//!   80 bytes, in the assembly's order).
//! - `sgr_box3_vert_neon` is `dav1d_sgr_box3_vert_neon`: 16 columns per
//!   iteration of the loop `1:`.
//! - `sgr_box5_vert_neon` is `dav1d_sgr_box5_vert_neon`: 8 columns per
//!   iteration of the loop `1:`.
//!
//! Correspondence with the assembly: general registers keep their names
//! (`x5`, `w4`); a post-incremented address is the access followed by the
//! `add` of the bytes accessed. Vector registers are typed locals named
//! after the registers. The sums of squares and the `a`/`b` values are
//! `uint32x4_t`/`uint16x8_t` (the arithmetic is `add`, `mul`, `umull`,
//! `uqsub`, `uqshrn`; the two signed instructions `srshl` and `srshr` are
//! the helpers `srshl_4s`, `srshl_8h`, `srshr_4s`). Where a register holds
//! a value of another lane size than the rows it otherwise holds in the
//! function, the local carries the arrangement as a suffix (`v1b` is
//! `v1.16b`, `v9h` is `v9.8h`, `v2s` is `v2.4s`). The order of the loads
//! and stores is the assembly's; the saves of `d8`–`d15` are the
//! compiler's.
//!
//! Deviations: none in the arithmetic, the loads or the stores. (The
//! comments `// RangeMins` and `// DiffMasks` at the two `tbl` lines of the
//! assembly are swapped there; the registers are what counts: `v24`/`v25`
//! hold `RangeMins`, `v26`/`v27` hold `DiffMasks`.)
//!
//! Safety (the callers are the wrappers `sgr_box3_vert_neon` and
//! `sgr_box5_vert_neon` of `simd/arm/looprestoration.rs`, which pass rows
//! of their zero-initialized buffers: `ARM_BUF_STRIDE` = 400 elements per
//! row, 16 spare elements behind the last row): with `n = ceil((w + 2) /
//! 16)` (box3) the kernel reads `16 * (n + 1)` elements of `sumsq[0..3]`
//! and of `sum[0..2]`, `16 * n` elements of `sum[2]`, and writes `16 * n`
//! elements of `AA` and of `BB`; at the test's largest width (384) that is
//! 416 elements read, which is the row plus the 16 spare elements when the
//! row is the last of its buffer (`2 * 400 + 416 = 3 * 400 + 16`), and 400
//! written (one row). With `n = ceil((w + 2) / 8)` (box5) it reads
//! `8 * (n + 1)` elements of all five `sumsq` and `sum` rows and writes
//! `8 * n` elements of `AA` and `BB`: at most 400 read and 392 written,
//! inside one row. The elements read beyond those the row kernels wrote
//! are the zeros the wrappers initialize the buffers with (or older sums);
//! they only reach output columns nobody reads.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::c_int;

/// `x_by_x_tables`: a series of LUTs for efficiently computing sgr's
/// `1 - x/(x+1)` table (`RefTable` is `dav1d_sgr_x_by_x`).
///
/// - bytes 0..32, `RangeMins`: `Min(RefTable[i*8:i*8+8])`, the first two
///   values zeroed; lookup with `RangeMins[x >> 3]`.
/// - bytes 32..64, `DiffMasks`: per range of 8 `RefTable` entries one byte
///   whose bits say at which index the value of `RefTable` changes; the
///   popcount of the byte shifted left by `x & 7` integrates the changes
///   up to `x`.
/// - bytes 64..80, `RefLo`: `RefTable[0:16]` (`tbl` yields 0 for `x > 15`).
#[repr(C, align(16))]
struct XByXTables([u8; 80]);
static X_BY_X_TABLES: XByXTables = XByXTables([
    // RangeMins
    0, 0, 11, 8, 6, 5, 5, 4, 4, 3, 3, 3, 2, 2, 2, 2, //
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, //
    // DiffMasks
    0x00, 0x00, 0xD4, 0x44, //
    0x42, 0x04, 0x00, 0x00, //
    0x00, 0x80, 0x00, 0x00, //
    0x04, 0x00, 0x00, 0x00, //
    0x00, 0x00, 0x00, 0x00, //
    0x00, 0x40, 0x00, 0x00, //
    0x00, 0x00, 0x00, 0x00, //
    0x00, 0x00, 0x00, 0x02, //
    // RefLo
    255, 128, 85, 64, 51, 43, 37, 32, 28, 26, 23, 21, 20, 18, 17, 16,
]);

// --- instructions ---------------------------------------------------------------
// SAFETY of the value intrinsics below: NEON is baseline on AArch64.

/// `srshl vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn srshl_4s(a: uint32x4_t, b: int32x4_t) -> uint32x4_t {
    unsafe { vreinterpretq_u32_s32(vrshlq_s32(vreinterpretq_s32_u32(a), b)) }
}

/// `srshl vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn srshl_8h(a: uint16x8_t, b: int16x8_t) -> uint16x8_t {
    unsafe { vreinterpretq_u16_s16(vrshlq_s16(vreinterpretq_s16_u16(a), b)) }
}

/// `srshr vd.4s, vn.4s, #N`
#[inline(always)]
fn srshr_4s<const N: i32>(a: uint32x4_t) -> uint32x4_t {
    unsafe { vreinterpretq_u32_s32(vrshrq_n_s32::<N>(vreinterpretq_s32_u32(a))) }
}

/// `sgr_box3_vert_neon`: `void dav1d_sgr_box3_vert_neon(int32_t **sumsq, int16_t **sum, int32_t *AA, int16_t *BB,
/// const int w, const int s, const int bitdepth_max)`
///
/// # Safety
/// `sumsq[0..3]` and `sum[0..3]` are rows readable for `16 * (ceil((w + 2) / 16) + 1)` elements (`sum[2]` for one
/// block of 16 less), `AA` and `BB` rows writable for `16 * ceil((w + 2) / 16)` elements, not overlapping the
/// rows read. See the module documentation for how the wrappers' buffers provide that.
pub unsafe extern "C" fn dav1d_sgr_box3_vert_neon(
    sumsq: *mut *mut i32,
    sum: *mut *mut i16,
    aa: *mut i32,
    bb: *mut i16,
    w: c_int,
    s: c_int,
    bitdepth_max: c_int,
) {
    let (mut x2, mut x3) = (aa as *mut u32, bb as *mut u16);
    let mut w4 = w + 2; // add w4, w4, #2
    let w9 = (bitdepth_max as u32).leading_zeros() as i32; // clz w9, w6 (bitdepth_max)
    let v28 = vdupq_n_u32(s as u32); // dup v28.4s, w5 (strength)

    let mut x5 = *sumsq as *const u32; // ldp x5, x6, [x0]
    let mut x6 = *sumsq.add(1) as *const u32;
    let mut x0 = *sumsq.add(2) as *const u32; // ldr x0, [x0, #16]
    let mut x7 = *sum as *const u16; // ldp x7, x8, [x1]
    let mut x8 = *sum.add(1) as *const u16;
    let mut x1 = *sum.add(2) as *const u16; // ldr x1, [x1, #16]

    let v31 = vdupq_n_u32(9); // movi v31.4s, #9 (n)

    let w9 = w9 - 24; // sub w9, w9, #24 (-bitdepth_min_8)
    let x12 = X_BY_X_TABLES.0.as_ptr(); // movrel x12, x_by_x_tables
    let w13 = 455u32; // mov w13, #455 (one_by_x)
    let uint8x16x4_t(v24, v25, v26, v27) = vld1q_u8_x4(x12); // ld1 {v24.16b, v25.16b, v26.16b, v27.16b}, [x12] (RangeMins, DiffMasks)
    let v22 = vdupq_n_u8(0x7); // movi v22.16b, #0x7
    let v23 = vld1q_u8(x12.add(64)); // ldr q23, [x12, #64] (RefLo)
    let v6 = vdupq_n_s16(w9 as i16); // dup v6.8h, w9 (-bitdepth_min_8)
    let v7 = vaddl_s16(vget_low_s16(v6), vget_low_s16(v6)); // saddl v7.4s, v6.4h, v6.4h (-2*bitdepth_min_8)
    let v30 = vdupq_n_u32(w13); // dup v30.4s, w13 (one_by_x)

    let uint32x4x4_t(mut v8, mut v9, mut v10, mut v11) = vld1q_u32_x4(x5); // ld1 {v8.4s, v9.4s, v10.4s, v11.4s}, [x5], #64
    x5 = x5.add(16);
    let uint32x4x4_t(mut v12, mut v13, mut v14, mut v15) = vld1q_u32_x4(x6); // ld1 {v12.4s, v13.4s, v14.4s, v15.4s}, [x6], #64
    x6 = x6.add(16);
    let uint32x4x4_t(mut v16, mut v17, mut v18, mut v19) = vld1q_u32_x4(x0); // ld1 {v16.4s, v17.4s, v18.4s, v19.4s}, [x0], #64
    x0 = x0.add(16);
    let uint16x8x2_t(mut v20, mut v21) = vld1q_u16_x2(x8); // ld1 {v20.8h, v21.8h}, [x8], #32
    x8 = x8.add(16);
    let uint16x8x2_t(mut v0, mut v1) = vld1q_u16_x2(x7); // ld1 {v0.8h, v1.8h}, [x7], #32
    x7 = x7.add(16);
    loop {
        // 1:
        let uint16x8x2_t(v2, v3) = vld1q_u16_x2(x1); // ld1 {v2.8h, v3.8h}, [x1], #32
        x1 = x1.add(16);
        v8 = vaddq_u32(v8, v12); // add v8.4s, v8.4s, v12.4s
        v9 = vaddq_u32(v9, v13); // add v9.4s, v9.4s, v13.4s
        v10 = vaddq_u32(v10, v14); // add v10.4s, v10.4s, v14.4s
        v11 = vaddq_u32(v11, v15); // add v11.4s, v11.4s, v15.4s
        v0 = vaddq_u16(v0, v20); // add v0.8h, v0.8h, v20.8h
        v1 = vaddq_u16(v1, v21); // add v1.8h, v1.8h, v21.8h

        v16 = vaddq_u32(v16, v8); // add v16.4s, v16.4s, v8.4s
        v17 = vaddq_u32(v17, v9); // add v17.4s, v17.4s, v9.4s
        v18 = vaddq_u32(v18, v10); // add v18.4s, v18.4s, v10.4s
        v19 = vaddq_u32(v19, v11); // add v19.4s, v19.4s, v11.4s
        let v4 = vaddq_u16(v2, v0); // add v4.8h, v2.8h, v0.8h
        let v5 = vaddq_u16(v3, v1); // add v5.8h, v3.8h, v1.8h

        v16 = srshl_4s(v16, v7); // srshl v16.4s, v16.4s, v7.4s
        v17 = srshl_4s(v17, v7); // srshl v17.4s, v17.4s, v7.4s
        v18 = srshl_4s(v18, v7); // srshl v18.4s, v18.4s, v7.4s
        v19 = srshl_4s(v19, v7); // srshl v19.4s, v19.4s, v7.4s
        let v9h = srshl_8h(v4, v6); // srshl v9.8h, v4.8h, v6.8h
        let v13h = srshl_8h(v5, v6); // srshl v13.8h, v5.8h, v6.8h
        v16 = vmulq_u32(v16, v31); // mul v16.4s, v16.4s, v31.4s (a * n)
        v17 = vmulq_u32(v17, v31); // mul v17.4s, v17.4s, v31.4s (a * n)
        v18 = vmulq_u32(v18, v31); // mul v18.4s, v18.4s, v31.4s (a * n)
        v19 = vmulq_u32(v19, v31); // mul v19.4s, v19.4s, v31.4s (a * n)
        v8 = vmull_u16(vget_low_u16(v9h), vget_low_u16(v9h)); // umull v8.4s, v9.4h, v9.4h (b * b)
        v9 = vmull_high_u16(v9h, v9h); // umull2 v9.4s, v9.8h, v9.8h (b * b)
        v12 = vmull_u16(vget_low_u16(v13h), vget_low_u16(v13h)); // umull v12.4s, v13.4h, v13.4h (b * b)
        v13 = vmull_high_u16(v13h, v13h); // umull2 v13.4s, v13.8h, v13.8h (b * b)
        v16 = vqsubq_u32(v16, v8); // uqsub v16.4s, v16.4s, v8.4s (imax(a * n - b * b, 0))
        v17 = vqsubq_u32(v17, v9); // uqsub v17.4s, v17.4s, v9.4s (imax(a * n - b * b, 0))
        v18 = vqsubq_u32(v18, v12); // uqsub v18.4s, v18.4s, v12.4s (imax(a * n - b * b, 0))
        v19 = vqsubq_u32(v19, v13); // uqsub v19.4s, v19.4s, v13.4s (imax(a * n - b * b, 0))
        v16 = vmulq_u32(v16, v28); // mul v16.4s, v16.4s, v28.4s (p * s)
        v17 = vmulq_u32(v17, v28); // mul v17.4s, v17.4s, v28.4s (p * s)
        v18 = vmulq_u32(v18, v28); // mul v18.4s, v18.4s, v28.4s (p * s)
        v19 = vmulq_u32(v19, v28); // mul v19.4s, v19.4s, v28.4s (p * s)
        let v16h = vqshrn_high_n_u32::<16>(vqshrn_n_u32::<16>(v16), v17); // uqshrn v16.4h, v16.4s, #16; uqshrn2 v16.8h, v17.4s, #16
        let v18h = vqshrn_high_n_u32::<16>(vqshrn_n_u32::<16>(v18), v19); // uqshrn v18.4h, v18.4s, #16; uqshrn2 v18.8h, v19.4s, #16
        let v1b = vqrshrn_high_n_u16::<4>(vqrshrn_n_u16::<4>(v16h), v18h); // uqrshrn v1.8b, v16.8h, #4; uqrshrn2 v1.16b, v18.8h, #4 (imin(z, 255))

        uint32x4x2_t(v16, v17) = vld1q_u32_x2(x0); // ld1 {v16.4s, v17.4s}, [x0], #32
        x0 = x0.add(8);
        w4 -= 16; // subs w4, w4, #16

        let v0b = vshrq_n_u8::<3>(v1b); // ushr v0.16b, v1.16b, #3
        uint32x4x2_t(v8, v9) = vld1q_u32_x2(x5); // ld1 {v8.4s, v9.4s}, [x5], #32
        x5 = x5.add(8);
        let v2b = vqtbl2q_u8(uint8x16x2_t(v26, v27), v0b); // tbl v2.16b, {v26.16b, v27.16b}, v0.16b (DiffMasks)
        let v0b = vqtbl2q_u8(uint8x16x2_t(v24, v25), v0b); // tbl v0.16b, {v24.16b, v25.16b}, v0.16b (RangeMins)
        let v3b = vqtbl1q_u8(v23, v1b); // tbl v3.16b, {v23.16b}, v1.16b (RefLo)
        let v1b = vandq_u8(v1b, v22); // and v1.16b, v1.16b, v22.16b
        uint32x4x2_t(v12, v13) = vld1q_u32_x2(x6); // ld1 {v12.4s, v13.4s}, [x6], #32
        x6 = x6.add(8);
        let v1b = vshlq_u8(v2b, vreinterpretq_s8_u8(v1b)); // ushl v1.16b, v2.16b, v1.16b
        uint16x8x2_t(v20, v21) = vld1q_u16_x2(x8); // ld1 {v20.8h, v21.8h}, [x8], #32
        x8 = x8.add(16);
        let v3b = vaddq_u8(v3b, v0b); // add v3.16b, v3.16b, v0.16b
        let v1b = vcntq_u8(v1b); // cnt v1.16b, v1.16b
        uint32x4x2_t(v18, v19) = vld1q_u32_x2(x0); // ld1 {v18.4s, v19.4s}, [x0], #32
        x0 = x0.add(8);
        let v3b = vaddq_u8(v3b, v1b); // add v3.16b, v3.16b, v1.16b
        uint32x4x2_t(v10, v11) = vld1q_u32_x2(x5); // ld1 {v10.4s, v11.4s}, [x5], #32
        x5 = x5.add(8);
        v0 = vmovl_u8(vget_low_u8(v3b)); // uxtl v0.8h, v3.8b (x)
        v1 = vmovl_high_u8(v3b); // uxtl2 v1.8h, v3.16b (x)

        uint32x4x2_t(v14, v15) = vld1q_u32_x2(x6); // ld1 {v14.4s, v15.4s}, [x6], #32
        x6 = x6.add(8);

        let v2s = vmull_u16(vget_low_u16(v0), vget_low_u16(v4)); // umull v2.4s, v0.4h, v4.4h (x * BB[i])
        let v3s = vmull_high_u16(v0, v4); // umull2 v3.4s, v0.8h, v4.8h (x * BB[i])
        let v4s = vmull_u16(vget_low_u16(v1), vget_low_u16(v5)); // umull v4.4s, v1.4h, v5.4h (x * BB[i])
        let v5s = vmull_high_u16(v1, v5); // umull2 v5.4s, v1.8h, v5.8h (x * BB[i])
        let v2s = vmulq_u32(v2s, v30); // mul v2.4s, v2.4s, v30.4s (x * BB[i] * sgr_one_by_x)
        let v3s = vmulq_u32(v3s, v30); // mul v3.4s, v3.4s, v30.4s (x * BB[i] * sgr_one_by_x)
        let v4s = vmulq_u32(v4s, v30); // mul v4.4s, v4.4s, v30.4s (x * BB[i] * sgr_one_by_x)
        let v5s = vmulq_u32(v5s, v30); // mul v5.4s, v5.4s, v30.4s (x * BB[i] * sgr_one_by_x)
        vst1q_u16_x2(x3, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x3], #32
        x3 = x3.add(16);
        uint16x8x2_t(v0, v1) = vld1q_u16_x2(x7); // ld1 {v0.8h, v1.8h}, [x7], #32
        x7 = x7.add(16);
        let v2s = srshr_4s::<12>(v2s); // srshr v2.4s, v2.4s, #12 (AA[i])
        let v3s = srshr_4s::<12>(v3s); // srshr v3.4s, v3.4s, #12 (AA[i])
        let v4s = srshr_4s::<12>(v4s); // srshr v4.4s, v4.4s, #12 (AA[i])
        let v5s = srshr_4s::<12>(v5s); // srshr v5.4s, v5.4s, #12 (AA[i])

        vst1q_u32_x4(x2, uint32x4x4_t(v2s, v3s, v4s, v5s)); // st1 {v2.4s, v3.4s, v4.4s, v5.4s}, [x2], #64
        x2 = x2.add(16);
        if w4 <= 0 {
            break; // b.gt 1b
        }
    }
}

/// `sgr_box5_vert_neon`: `void dav1d_sgr_box5_vert_neon(int32_t **sumsq, int16_t **sum, int32_t *AA, int16_t *BB,
/// const int w, const int s, const int bitdepth_max)`
///
/// # Safety
/// `sumsq[0..5]` and `sum[0..5]` are rows readable for `8 * (ceil((w + 2) / 8) + 1)` elements, `AA` and `BB` rows
/// writable for `8 * ceil((w + 2) / 8)` elements, not overlapping the rows read. See the module documentation for
/// how the wrappers' buffers provide that.
pub unsafe extern "C" fn dav1d_sgr_box5_vert_neon(
    sumsq: *mut *mut i32,
    sum: *mut *mut i16,
    aa: *mut i32,
    bb: *mut i16,
    w: c_int,
    s: c_int,
    bitdepth_max: c_int,
) {
    let (mut x2, mut x3) = (aa as *mut u32, bb as *mut u16);
    let mut w4 = w + 2; // add w4, w4, #2
    let w15 = (bitdepth_max as u32).leading_zeros() as i32; // clz w15, w6 (bitdepth_max)
    let v28 = vdupq_n_u32(s as u32); // dup v28.4s, w5 (strength)

    let mut x5 = *sumsq as *const u32; // ldp x5, x6, [x0]
    let mut x6 = *sumsq.add(1) as *const u32;
    let mut x7 = *sumsq.add(2) as *const u32; // ldp x7, x8, [x0, #16]
    let mut x8 = *sumsq.add(3) as *const u32;
    let mut x0 = *sumsq.add(4) as *const u32; // ldr x0, [x0, #32]
    let mut x9 = *sum as *const u16; // ldp x9, x10, [x1]
    let mut x10 = *sum.add(1) as *const u16;
    let mut x11 = *sum.add(2) as *const u16; // ldp x11, x12, [x1, #16]
    let mut x12 = *sum.add(3) as *const u16;
    let mut x1 = *sum.add(4) as *const u16; // ldr x1, [x1, #32]

    let v31 = vdupq_n_u32(25); // movi v31.4s, #25 (n)

    let w15 = w15 - 24; // sub w15, w15, #24 (-bitdepth_min_8)
    let x13 = X_BY_X_TABLES.0.as_ptr(); // movrel x13, x_by_x_tables
    let v30 = vdupq_n_u32(164); // movi v30.4s, #164
    let uint8x16x4_t(v24, v25, v26, v27) = vld1q_u8_x4(x13); // ld1 {v24.16b, v25.16b, v26.16b, v27.16b}, [x13] (RangeMins, DiffMasks)
    let v6 = vdupq_n_s16(w15 as i16); // dup v6.8h, w15 (-bitdepth_min_8)
    let v19 = vdup_n_u8(0x7); // movi v19.8b, #0x7
    let v18 = vld1q_u8(x13.add(64)); // ldr q18, [x13, #64] (RefLo)
    let v7 = vaddl_s16(vget_low_s16(v6), vget_low_s16(v6)); // saddl v7.4s, v6.4h, v6.4h (-2*bitdepth_min_8)

    let uint32x4x2_t(mut v8, mut v9) = vld1q_u32_x2(x5); // ld1 {v8.4s, v9.4s}, [x5], #32
    x5 = x5.add(8);
    let uint32x4x2_t(mut v10, mut v11) = vld1q_u32_x2(x6); // ld1 {v10.4s, v11.4s}, [x6], #32
    x6 = x6.add(8);
    let uint32x4x2_t(mut v12, mut v13) = vld1q_u32_x2(x7); // ld1 {v12.4s, v13.4s}, [x7], #32
    x7 = x7.add(8);
    let uint32x4x2_t(mut v16, mut v17) = vld1q_u32_x2(x8); // ld1 {v16.4s, v17.4s}, [x8], #32
    x8 = x8.add(8);
    let mut v20 = vld1q_u16(x9); // ld1 {v20.8h}, [x9], #16
    x9 = x9.add(8);
    let mut v21 = vld1q_u16(x10); // ld1 {v21.8h}, [x10], #16
    x10 = x10.add(8);
    let mut v22 = vld1q_u16(x11); // ld1 {v22.8h}, [x11], #16
    x11 = x11.add(8);
    let mut v23 = vld1q_u16(x12); // ld1 {v23.8h}, [x12], #16
    x12 = x12.add(8);
    let uint32x4x2_t(mut v0, mut v1) = vld1q_u32_x2(x0); // ld1 {v0.4s, v1.4s}, [x0], #32
    x0 = x0.add(8);
    let mut v2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], #16
    x1 = x1.add(8);

    loop {
        // 1:
        v8 = vaddq_u32(v8, v10); // add v8.4s, v8.4s, v10.4s
        v9 = vaddq_u32(v9, v11); // add v9.4s, v9.4s, v11.4s
        v12 = vaddq_u32(v12, v16); // add v12.4s, v12.4s, v16.4s
        v13 = vaddq_u32(v13, v17); // add v13.4s, v13.4s, v17.4s

        v20 = vaddq_u16(v20, v21); // add v20.8h, v20.8h, v21.8h
        v22 = vaddq_u16(v22, v23); // add v22.8h, v22.8h, v23.8h

        v0 = vaddq_u32(v0, v8); // add v0.4s, v0.4s, v8.4s
        v1 = vaddq_u32(v1, v9); // add v1.4s, v1.4s, v9.4s
        v2 = vaddq_u16(v2, v20); // add v2.8h, v2.8h, v20.8h

        v0 = vaddq_u32(v0, v12); // add v0.4s, v0.4s, v12.4s
        v1 = vaddq_u32(v1, v13); // add v1.4s, v1.4s, v13.4s
        v2 = vaddq_u16(v2, v22); // add v2.8h, v2.8h, v22.8h

        w4 -= 8; // subs w4, w4, #8

        v0 = srshl_4s(v0, v7); // srshl v0.4s, v0.4s, v7.4s
        v1 = srshl_4s(v1, v7); // srshl v1.4s, v1.4s, v7.4s
        let v4h = srshl_8h(v2, v6); // srshl v4.8h, v2.8h, v6.8h
        v0 = vmulq_u32(v0, v31); // mul v0.4s, v0.4s, v31.4s (a * n)
        v1 = vmulq_u32(v1, v31); // mul v1.4s, v1.4s, v31.4s (a * n)
        let v3 = vmull_u16(vget_low_u16(v4h), vget_low_u16(v4h)); // umull v3.4s, v4.4h, v4.4h (b * b)
        let v4 = vmull_high_u16(v4h, v4h); // umull2 v4.4s, v4.8h, v4.8h (b * b)
        v0 = vqsubq_u32(v0, v3); // uqsub v0.4s, v0.4s, v3.4s (imax(a * n - b * b, 0))
        v1 = vqsubq_u32(v1, v4); // uqsub v1.4s, v1.4s, v4.4s (imax(a * n - b * b, 0))
        v0 = vmulq_u32(v0, v28); // mul v0.4s, v0.4s, v28.4s (p * s)
        v1 = vmulq_u32(v1, v28); // mul v1.4s, v1.4s, v28.4s (p * s)
        uint32x4x2_t(v8, v9) = vld1q_u32_x2(x5); // ld1 {v8.4s, v9.4s}, [x5], #32
        x5 = x5.add(8);
        let v0h = vqshrn_high_n_u32::<16>(vqshrn_n_u32::<16>(v0), v1); // uqshrn v0.4h, v0.4s, #16; uqshrn2 v0.8h, v1.4s, #16
        uint32x4x2_t(v10, v11) = vld1q_u32_x2(x6); // ld1 {v10.4s, v11.4s}, [x6], #32
        x6 = x6.add(8);
        let v0b = vqrshrn_n_u16::<4>(v0h); // uqrshrn v0.8b, v0.8h, #4 (imin(z, 255))

        uint32x4x2_t(v12, v13) = vld1q_u32_x2(x7); // ld1 {v12.4s, v13.4s}, [x7], #32
        x7 = x7.add(8);

        let v1b = vshr_n_u8::<3>(v0b); // ushr v1.8b, v0.8b, #3
        uint32x4x2_t(v16, v17) = vld1q_u32_x2(x8); // ld1 {v16.4s, v17.4s}, [x8], #32
        x8 = x8.add(8);
        let v5b = vqtbl2_u8(uint8x16x2_t(v26, v27), v1b); // tbl v5.8b, {v26.16b, v27.16b}, v1.8b (DiffMasks)
        let v1b = vqtbl2_u8(uint8x16x2_t(v24, v25), v1b); // tbl v1.8b, {v24.16b, v25.16b}, v1.8b (RangeMins)
        let v4b = vqtbl1_u8(v18, v0b); // tbl v4.8b, {v18.16b}, v0.8b (RefLo)
        let v0b = vand_u8(v0b, v19); // and v0.8b, v0.8b, v19.8b
        v20 = vld1q_u16(x9); // ld1 {v20.8h}, [x9], #16
        x9 = x9.add(8);
        let v5b = vshl_u8(v5b, vreinterpret_s8_u8(v0b)); // ushl v5.8b, v5.8b, v0.8b
        let v4b = vadd_u8(v4b, v1b); // add v4.8b, v4.8b, v1.8b
        v21 = vld1q_u16(x10); // ld1 {v21.8h}, [x10], #16
        x10 = x10.add(8);
        let v5b = vcnt_u8(v5b); // cnt v5.8b, v5.8b
        v22 = vld1q_u16(x11); // ld1 {v22.8h}, [x11], #16
        x11 = x11.add(8);
        let v5b = vadd_u8(v4b, v5b); // add v5.8b, v4.8b, v5.8b
        v23 = vld1q_u16(x12); // ld1 {v23.8h}, [x12], #16
        x12 = x12.add(8);
        let v5 = vmovl_u8(v5b); // uxtl v5.8h, v5.8b (x)

        uint32x4x2_t(v0, v1) = vld1q_u32_x2(x0); // ld1 {v0.4s, v1.4s}, [x0], #32
        x0 = x0.add(8);
        let v3 = vmull_u16(vget_low_u16(v5), vget_low_u16(v2)); // umull v3.4s, v5.4h, v2.4h (x * BB[i])
        let v4 = vmull_high_u16(v5, v2); // umull2 v4.4s, v5.8h, v2.8h (x * BB[i])
        let v3 = vmulq_u32(v3, v30); // mul v3.4s, v3.4s, v30.4s (x * BB[i] * sgr_one_by_x)
        let v4 = vmulq_u32(v4, v30); // mul v4.4s, v4.4s, v30.4s (x * BB[i] * sgr_one_by_x)
        let v3 = srshr_4s::<12>(v3); // srshr v3.4s, v3.4s, #12 (AA[i])
        let v4 = srshr_4s::<12>(v4); // srshr v4.4s, v4.4s, #12 (AA[i])
        v2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], #16
        x1 = x1.add(8);

        vst1q_u32_x2(x2, uint32x4x2_t(v3, v4)); // st1 {v3.4s, v4.4s}, [x2], #32
        x2 = x2.add(8);
        vst1q_u16(x3, v5); // st1 {v5.8h}, [x3], #16
        x3 = x3.add(8);
        if w4 <= 0 {
            break; // b.gt 1b
        }
    }
}
