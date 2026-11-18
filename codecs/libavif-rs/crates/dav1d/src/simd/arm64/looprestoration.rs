// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/looprestoration.S` (8 bpc) with NEON intrinsics
//! (engine ADR-0044 D2, libavif-rs ADR 0062): the Wiener filters, the horizontal box sums of the
//! self-guided filter, and the file's instances of the template `looprestoration_tmpl.S`
//! (`sgr_funcs 8`; the generic functions are in `looprestoration_tmpl.rs`). The vertical box sums are
//! in `looprestoration_common.rs`, the C side of the self-guided filters and the slot assignments in
//! `simd/arm/looprestoration.rs`. The table `right_ext_mask_buf` is `RIGHT_EXT_MASK_BUF`.
//!
//! Stack buffers: the Wiener filters keep their rows of intermediate values in a local buffer, zero-initialized
//! (`[0; N]`); the self-guided kernels have none (their rows are the wrappers',
//! zeroed whole there).
//!
//! # The Wiener filters
//!
//! `wiener_filter7_8bpc_neon` and `wiener_filter5_8bpc_neon` are the entry points
//! `dav1d_wiener_filter{7,5}_8bpc_neon`; their internal functions `wiener_filter{7,5}_{h,v,hv}_8bpc_neon`
//! are the Rust functions of the same names.
//!
//! The assembly calls the internal functions with `bl` and a private register convention: the state stays
//! in registers, and a row function pushes what it changes and pops it at the end, the row pointers
//! shifted by one. Here the registers that live across those calls are the fields of `WienerRegs`, which
//! every row function takes by reference, and a row function leaves in it exactly what the assembly's
//! function leaves in the registers. The row functions are separate functions (`#[inline(never)]`), as in
//! the assembly; each loads the coefficients and the two rounding constants it uses from
//! `WienerRegs`, where the assembly keeps them in registers across `bl`, and stores the shifted row
//! pointers at entry, where the assembly pushes them shifted and pops them at the end. (Inlined into the
//! entry points, the copies of the row loops shared their widened coefficients, and LLVM compiled the
//! `smull`/`smlal` chains of the main loop as `sshll` + `mul`/`mla`.)
//!
//! The labels of the entry points (`L(main_7)`, `L(no_top_7)`, ...) are labeled blocks; the fall-through
//! chain `L(v3_7)` -> `L(v2_7)` -> `L(v1_7)` is a count of the vertical passes that remain. In the row
//! functions the numeric labels are comments; `3:` (the padding of the right edge) is the `if` at the
//! head of the loop, entered as the assembly enters it: without LR_HAVE_RIGHT, before every 16 pixels,
//! when fewer than 19 (5 taps: 18) are left.
//!
//! Single instructions in another form: `st1 {v6.8h, v7.8h}, [xN], #32` is two `vst1q_s16` stores (compiled
//! to one `stp` with post-index; `vst1q_s16_x2` compiled to `st1` and a separate `add`); `ld1 {v2.s}[3], [x2]`
//! is a scalar load into lane 3 of a zero vector (the other lanes are not used).
//!
//! # The self-guided filter kernels
//!
//! The three row kernels (`sgr_box3_row_h_8bpc_neon`, `sgr_box5_row_h_8bpc_neon`,
//! `sgr_box35_row_h_8bpc_neon`) are one function each; the labels `0:`–`4:`,
//! `9:` are comments. General registers keep their assembly names; vector
//! registers are typed locals named after the registers (`v0` the 16 pixels
//! of the window, `v1`/`v2` their squares, `v30` the padding pixel). A
//! post-incremented address is the access followed by the `add` of the bytes
//! accessed.
//!
//! The loop: the assembly has one loop body, `4:` ("Loop horizontally"), which
//! ends with `tst w5, #2` and goes back to `4:` with LR_HAVE_RIGHT (`b.ne 4b`)
//! and to `3:`, the check for padding in front of `4:`, without (`b 3b`). Here
//! `4:` is the function-local macro `loop_horizontally!` (the text of the
//! block once, `break` at `b.le 9f`), used by two loops: with LR_HAVE_RIGHT
//! the loop of `4:` alone, without it the loop of `3:` and `4:`. The flag is
//! tested once, where the assembly tests it first (`b.ne 4f`), not after
//! every block; the padding pixel is loaded on the path that uses it, as in
//! the assembly.
//!
//! Deviations: the flag test per block, as said (two instructions less per
//! block than the assembly; written as one loop the compiler tested the flag
//! with five instructions per block). `ldr b30, [x3, w13, sxtw]` +
//! `dup v30.16b, v30.b[0]` is the load-and-replicate `ld1r` (`vld1q_dup_u8`);
//! `ld1 {v1.s}[3], [x2]` of the `left` pixels, of which the `ext` that
//! follows reads only that lane, is a load replicated to all lanes
//! (`sgr_ld1_left_8bpc`), since the intrinsic for a lane load needs a
//! register to insert into. Neither of these two is in a loop.
//!
//! Memory, for all three (`K` = 2 for box3, 3 for box5 and box35; `n` =
//! `ceil((w + 2) / 8)` blocks):
//! - written: `8 * n` elements of each `sum` row and `8 * n` elements of each
//!   `sumsq` row (at most 392 at the test's largest width 384; the wrappers'
//!   rows have 400).
//! - read: `src[-K..16 - K]` with LR_HAVE_LEFT and `left == NULL`, else
//!   `src[0..16]` and, with LR_HAVE_LEFT, `left[0][0..4]`; then 8 more bytes
//!   per further block, up to `src[8 * n + 7 - K]` (with one block the first
//!   load is all, up to `src[15]`). The sums use the pixels up to
//!   `src[w + K - 1]` with LR_HAVE_RIGHT and up to `src[w - 1]` without. The
//!   kernel therefore reads up to 15 bytes at and behind `src + w` (`16 - w`
//!   for `w <= 6`, else `8 * n + 8 - K - w`, which is 8 to 15 for box3 and 7
//!   to 14 for box5 and box35) — also without LR_HAVE_RIGHT, where it
//!   replaces what it loaded there by the padding pixel. This is the
//!   assembly's behavior, kept as it is; see the report.
//! - without LR_HAVE_RIGHT one more byte: `src[w - 1]`, the padding pixel.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::{c_int, c_uint};
use core::ptr::read_unaligned;

use super::super::arm::looprestoration::LooprestorationParams;
use super::looprestoration_tmpl::*;

/// `right_ext_mask_buf` with the label `right_ext_mask` at offset 48: 48 bytes
/// of 0x00, then 48 bytes of 0xff (shared by the Wiener and the box-sum
/// kernels; `movrel xN, right_ext_mask, -K` is `RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - K)`).
static RIGHT_EXT_MASK_BUF: [u8; 96] = {
    let mut t = [0u8; 96];
    let mut i = 48;
    while i < 96 {
        t[i] = 0xff;
        i += 1;
    }
    t
};
/// Offset of the label `right_ext_mask` in `RIGHT_EXT_MASK_BUF`.
const RIGHT_EXT_MASK: usize = 48;

/// `sub_sp 384*2*6` / `sub_sp 384*2*4`: the rows of 16-bit intermediate values (384 each) on the stack of
/// an exported Wiener filter. The assembly's stack pointer is 16-byte aligned.
#[repr(C, align(16))]
struct WienerRows<const N: usize>([i16; N]);

/// The registers the Wiener filters keep across their `bl` calls.
struct WienerRegs {
    /// `x0`: the output row in `p`.
    x0: *mut u8,
    /// `x1`: `stride`.
    x1: isize,
    /// `x2`: `left` (NULL for the `lpf` rows).
    x2: *const [u8; 4],
    /// `x3`: the source row (`lpf` or `p`).
    x3: *const u8,
    /// `w4`: `w`.
    w4: c_int,
    /// `w7`: `edges`.
    w7: c_uint,
    /// `x9`: t6 (7 taps only).
    x9: *mut i16,
    /// `x10`: t5 (7 taps only).
    x10: *mut i16,
    /// `x11`: t4.
    x11: *mut i16,
    /// `x12`: t3.
    x12: *mut i16,
    /// `x13`: t2.
    x13: *mut i16,
    /// `x14`: t1.
    x14: *mut i16,
    /// `x15`: t0.
    x15: *mut i16,
    /// `v0`: `filter[0]`, the horizontal coefficients.
    v0: int16x8_t,
    /// `v1`: `filter[1]`, the vertical coefficients.
    v1: int16x8_t,
    /// `v30`: `(1 << 14) - (1 << 2)`.
    v30: int16x8_t,
    /// `v31`: `1 << 11`.
    v31: int16x8_t,
}

/// `wiener_filter7_8bpc_neon`: `void dav1d_wiener_filter7_8bpc_neon(pixel *p, const ptrdiff_t stride,
/// const pixel (*left)[4], const pixel *lpf, const int w, int h, const LooprestorationParams *const params,
/// const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// Reads `params.filter` (32 bytes). Per source row (the `h` rows of `p`, with LR_HAVE_TOP the two rows at
/// `lpf` and `lpf + stride`, with LR_HAVE_BOTTOM the two rows at `lpf + 6 * stride` and `lpf + 7 * stride`)
/// it reads the bytes `[-3, 16 * n + 5)` of the row with LR_HAVE_LEFT and `left == NULL` (the `lpf` rows),
/// else `[0, 16 * n + 5)`, where `n = (w + 15) / 16`; with LR_HAVE_LEFT, 4 bytes of `left` per row of `p`.
/// Writes the bytes `[0, 16 * n)` of each of the `h` rows of `p`. The intermediate rows are in the local,
/// zero-initialized `WienerRows`.
pub unsafe extern "C" fn dav1d_wiener_filter7_8bpc_neon(
    p: *mut u8,
    stride: isize,
    left: *const [u8; 4],
    lpf: *const u8,
    w: c_int,
    h: c_int,
    params: *const LooprestorationParams,
    edges: c_uint,
) {
    let x6 = params as *const i16;
    let mut w5 = h;
    let v0_1 = vld1q_s16_x2(x6); // ld1 {v0.8h, v1.8h}, [x6]

    // sub_sp 384*2*6. Zero-initialized whole: which of the six rows a vertical pass loads depends on the
    // edge flags and the height, so that every loaded element was stored before is not obvious here.
    let mut rows = WienerRows([0; 384 * 6]);
    let sp = rows.0.as_mut_ptr();

    // x9  - t6
    // x10 - t5
    // x11 - t4
    // x12 - t3
    // x13 - t2
    // x14 - t1
    // x15 - t0
    // The assembly leaves x9-x13 and x15 unset until the code below assigns them; here they start at the
    // first row.
    let mut r = WienerRegs {
        x0: p,
        x1: stride,
        x2: left,
        x3: lpf,
        w4: w,
        w7: edges,
        x9: sp,
        x10: sp,
        x11: sp,
        x12: sp,
        x13: sp,
        x14: sp, // mov x14, sp // t1
        x15: sp,
        v0: v0_1.0,
        v1: v0_1.1,
        v30: vdupq_n_s16((1 << 14) - (1 << 2)), // mov w17, #(1 << 14) - (1 << 2); dup v30.8h, w17
        v31: vdupq_n_s16(8 << 8),               // movi v31.8h, #8, lsl #8
    };
    let x16: *const u8;

    // The number of vertical passes that remain: 3 for L(v3_7), 2 for L(v2_7), 1 for L(v1_7).
    let v_passes = 'v: {
        'main_7: {
            // tst w7, #4 // LR_HAVE_TOP; b.eq L(no_top_7)
            if r.w7 & 4 != 0 {
                let x16_left = r.x2; // mov x16, x2 // backup left
                r.x2 = core::ptr::null(); // mov x2, #0
                wiener_filter7_h_8bpc_neon(&mut r);
                r.x3 = r.x3.offset(r.x1); // lpf += stride
                r.x9 = r.x14; // t6
                r.x10 = r.x14; // t5
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_8bpc_neon(&mut r);
                r.x3 = r.x3.wrapping_offset(r.x1 * 4);
                r.x3 = r.x3.wrapping_offset(r.x1); // lpf += stride*5
                r.x11 = r.x14; // t4
                r.x14 = r.x14.add(384); // t1 += 384*2
                r.x2 = x16_left; // left
                x16 = r.x3; // backup lpf
                r.x3 = r.x0; // lpf = p
                wiener_filter7_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x12 = r.x14; // t3
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_7)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_8bpc_neon(&mut r);
                r.x13 = r.x14; // t2
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_7)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 3; // b.eq L(v3_7)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
            } else {
                // L(no_top_7):
                r.x3 = r.x3.wrapping_offset(r.x1 * 4);
                x16 = r.x3.wrapping_offset(r.x1 * 2); // lpf += stride*6, backup
                r.x3 = r.x0; // lpf = p

                wiener_filter7_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x9 = r.x14; // t6
                r.x10 = r.x14; // t5
                r.x11 = r.x14; // t4
                r.x12 = r.x14; // t3
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_7)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_7)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 3; // b.eq L(v3_7)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x15 = r.x14.add(384); // t0 = t1 + 384*2
                wiener_filter7_hv_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 3; // b.eq L(v3_7)
                }
                r.x15 = r.x15.add(384 * 4); // t0 += 384*2*4
                wiener_filter7_hv_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 != 0 {
                    break 'main_7; // b.ne L(main_7)
                }
                break 'v 3; // L(v3_7)
            }
        }
        // L(main_7):
        r.x15 = r.x14.add(384); // t0 = t1 + 384*2
        loop {
            // L(main_loop_7):
            wiener_filter7_hv_8bpc_neon(&mut r);
            w5 -= 1; // h--
            if w5 == 0 {
                break; // b.ne L(main_loop_7)
            }
        }
        // tst w7, #8 // LR_HAVE_BOTTOM; b.eq L(v3_7)
        if r.w7 & 8 == 0 {
            break 'v 3;
        }

        r.x3 = x16; // restore lpf
        r.x2 = core::ptr::null(); // left = NULL
        wiener_filter7_hv_8bpc_neon(&mut r);
        wiener_filter7_hv_8bpc_neon(&mut r);
        1 // L(v1_7)
    };
    if v_passes >= 3 {
        // L(v3_7):
        wiener_filter7_v_8bpc_neon(&mut r);
    }
    if v_passes >= 2 {
        // L(v2_7):
        wiener_filter7_v_8bpc_neon(&mut r);
    }
    // L(v1_7):
    wiener_filter7_v_8bpc_neon(&mut r);
}

/// `wiener_filter7_h_8bpc_neon`: filters the source row `x3` horizontally into the row `x14`.
/// Leaves `x3`, `w4` and `x14` unchanged (the assembly saves and restores them); advances `x2` by one
/// row of `left` if it is not NULL.
#[inline(never)]
unsafe fn wiener_filter7_h_8bpc_neon(r: &mut WienerRegs) {
    // stp x3, x4, [sp, #-32]!; str x14, [sp, #16]
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    let mut x14 = r.x14;
    let (v0, v30, v31) = (r.v0, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let v3;
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(3);
            v3 = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
            // ld1 {v2.s}[3], [x2], #4 (the other lanes of v2 are not used)
            let v2 = vsetq_lane_u32::<3>(read_unaligned(r.x2 as *const u32), vdupq_n_u32(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 3 bytes we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(3);
            v3 = vextq_u8::<13>(vreinterpretq_u8_u32(v2), v3_);
        }
    } else {
        // 1:
        let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost byte
        // and shift v3 to have 3x the first byte at the front.
        let v2 = vdupq_laneq_u8::<0>(v3_);
        // Move x3 back to account for the last 3 bytes we loaded before,
        // which we shifted out.
        x3 = x3.sub(3);
        v3 = vextq_u8::<13>(v2, v3_);
    }

    // 2:
    let v4 = vld1_u8(x3); // ld1 {v4.8b}, [x3], #8
    x3 = x3.add(8);
    let mut v2 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v3))); // uxtl v2.8h, v3.8b
    let mut v3 = vreinterpretq_s16_u16(vmovl_high_u8(v3)); // uxtl2 v3.8h, v3.16b
    let mut v4 = vreinterpretq_s16_u16(vmovl_u8(v4)); // uxtl v4.8h, v4.8b

    loop {
        // tst w7, #2 // LR_HAVE_RIGHT; b.ne 4f
        // 3: // !LR_HAVE_RIGHT
        // Check whether we need to pad the right edge
        // cmp w4, #19; b.ge 4f // If w >= 19, all used input pixels are valid
        if r.w7 & 2 == 0 && w4 < 19 {
            // 1 <= w < 19, w+3 pixels valid in v2-v4. For w>=9,
            // this ends up called again; it's not strictly needed in those
            // cases (we pad enough here), but keeping the code as simple as possible.

            // The padding pixel is v2/3/4.h[w+2]. x3 points at the next input, ie
            // v2/3/4.h[24]. Thus read from x3[w-22] to find the padding pixel.
            let w17 = w4 - 22;
            // Insert padding in v2/3/4.h[w+3] onwards; fuse the +3 (*2) into the
            // buffer pointer.
            // movrel x6, right_ext_mask, -6; sub x6, x6, w4, uxtw #1
            let x6 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 6 - 2 * w4 as usize);
            let b28 = *x3.offset(w17 as isize); // ldr b28, [x3, w17, sxtw]
            let v28 = vreinterpretq_u8_u16(vdupq_n_u16(b28 as u16)); // dup v28.8h, v28.h[0]
            let v25_27 = vld1q_u8_x3(x6); // ld1 {v25.16b, v26.16b, v27.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v25_27.0, v28, vreinterpretq_u8_s16(v2))); // bit v2.16b, v28.16b, v25.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v25_27.1, v28, vreinterpretq_u8_s16(v3))); // bit v3.16b, v28.16b, v26.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v25_27.2, v28, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v28.16b, v27.16b
        }

        // 4: // Loop horizontally
        let v17 = vextq_s16::<2>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #4
        let v19 = vextq_s16::<4>(v2, v3); // ext v19.16b, v2.16b, v3.16b, #8
        let v16 = vextq_s16::<1>(v2, v3); // ext v16.16b, v2.16b, v3.16b, #2
        let v20 = vextq_s16::<5>(v2, v3); // ext v20.16b, v2.16b, v3.16b, #10
        let v21 = vextq_s16::<6>(v2, v3); // ext v21.16b, v2.16b, v3.16b, #12
        let v18 = vextq_s16::<3>(v2, v3); // ext v18.16b, v2.16b, v3.16b, #6
        let v19 = vaddq_s16(v19, v17);
        let v20 = vaddq_s16(v20, v16);
        let v21 = vaddq_s16(v21, v2);
        let v22 = vshlq_n_s16::<7>(v18);
        let v6 = vmulq_laneq_s16::<3>(v18, v0); // mul v6.8h, v18.8h, v0.h[3]
        let v6 = vmlaq_laneq_s16::<4>(v6, v19, v0); // mla v6.8h, v19.8h, v0.h[4]
        let v6 = vmlaq_laneq_s16::<5>(v6, v20, v0); // mla v6.8h, v20.8h, v0.h[5]
        let v6 = vmlaq_laneq_s16::<6>(v6, v21, v0); // mla v6.8h, v21.8h, v0.h[6]

        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v20 = vextq_s16::<5>(v3, v4); // ext v20.16b, v3.16b, v4.16b, #10
        let v21 = vextq_s16::<6>(v3, v4); // ext v21.16b, v3.16b, v4.16b, #12
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6

        let v19 = vaddq_s16(v19, v17);
        let v20 = vaddq_s16(v20, v16);
        let v21 = vaddq_s16(v21, v3);
        let v23 = vshlq_n_s16::<7>(v18);
        let v7 = vmulq_laneq_s16::<3>(v18, v0); // mul v7.8h, v18.8h, v0.h[3]
        let v7 = vmlaq_laneq_s16::<4>(v7, v19, v0); // mla v7.8h, v19.8h, v0.h[4]
        let v7 = vmlaq_laneq_s16::<5>(v7, v20, v0); // mla v7.8h, v20.8h, v0.h[5]
        let v7 = vmlaq_laneq_s16::<6>(v7, v21, v0); // mla v7.8h, v21.8h, v0.h[6]

        let v22 = vsubq_s16(v22, v30);
        let v23 = vsubq_s16(v23, v30);
        let v6 = vqaddq_s16(v6, v22);
        let v7 = vqaddq_s16(v7, v23);
        let v6 = vshrq_n_s16::<3>(v6);
        let v7 = vshrq_n_s16::<3>(v7);
        let v6 = vaddq_s16(v6, v31);
        let v7 = vaddq_s16(v7, v31);

        w4 -= 16; // subs w4, w4, #16

        vst1q_s16(x14, v6); // st1 {v6.8h, v7.8h}, [x14], #32
        vst1q_s16(x14.add(8), v7);
        x14 = x14.wrapping_add(16);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b
        let v4_ = vld1q_u8(x3); // ld1 {v4.16b}, [x3], #16
        x3 = x3.add(16);
        v3 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v4_))); // uxtl v3.8h, v4.8b
        v4 = vreinterpretq_s16_u16(vmovl_high_u8(v4_)); // uxtl2 v4.8h, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }
    // 0:
    // ldr x14, [sp, #16]; ldp x3, x4, [sp], #32
}

/// `wiener_filter7_v_8bpc_neon`: filters the rows `x9`-`x14` (t6-t1, with t1 standing in for t0)
/// vertically into the output row `x0`. Leaves the row pointers shifted (`x9` gets the value of `x10`,
/// etc; `x14` stays) and `x0` advanced by `stride`.
#[inline(never)]
unsafe fn wiener_filter7_v_8bpc_neon(r: &mut WienerRegs) {
    // Backing up/restoring registers shifted, so that x9 gets the value
    // of x10, etc, afterwards.
    // stp x10, x11, [sp, #-64]!; stp x12, x13, [sp, #16]; stp x14, x14, [sp, #32]; stp x0, x4, [sp, #48]
    let (mut x9, mut x10, mut x11, mut x12, mut x13, mut x14) = (r.x9, r.x10, r.x11, r.x12, r.x13, r.x14);
    let mut x0 = r.x0;
    let mut w4 = r.w4;
    // The values the `ldp`s at the end leave in x9-x13 (x14 stays).
    r.x9 = x10;
    r.x10 = x11;
    r.x11 = x12;
    r.x12 = x13;
    r.x13 = x14;
    let v1 = r.v1;
    loop {
        // 1:
        let v20_21 = vld1q_s16_x2(x11); // ld1 {v20.8h, v21.8h}, [x11], #32
        x11 = x11.wrapping_add(16);
        let v24_25 = vld1q_s16_x2(x13); // ld1 {v24.8h, v25.8h}, [x13], #32
        x13 = x13.wrapping_add(16);

        let v18_19 = vld1q_s16_x2(x10); // ld1 {v18.8h, v19.8h}, [x10], #32
        x10 = x10.wrapping_add(16);
        let v24 = vaddq_s16(v24_25.0, v20_21.0);
        let v26_27 = vld1q_s16_x2(x14); // ld1 {v26.8h, v27.8h}, [x14], #32
        x14 = x14.wrapping_add(16);

        let v16_17 = vld1q_s16_x2(x9); // ld1 {v16.8h, v17.8h}, [x9], #32
        x9 = x9.wrapping_add(16);
        let v28 = vaddq_s16(v26_27.0, v18_19.0);
        let v22_23 = vld1q_s16_x2(x12); // ld1 {v22.8h, v23.8h}, [x12], #32
        x12 = x12.wrapping_add(16);

        let v16 = vaddq_s16(v26_27.0, v16_17.0);
        let v25 = vaddq_s16(v24_25.1, v20_21.1);

        let v2 = vmull_laneq_s16::<3>(vget_low_s16(v22_23.0), v1); // smull v2.4s, v22.4h, v1.h[3]
        let v2 = vmlal_laneq_s16::<4>(v2, vget_low_s16(v24), v1); // smlal v2.4s, v24.4h, v1.h[4]
        let v2 = vmlal_laneq_s16::<5>(v2, vget_low_s16(v28), v1); // smlal v2.4s, v28.4h, v1.h[5]
        let v2 = vmlal_laneq_s16::<6>(v2, vget_low_s16(v16), v1); // smlal v2.4s, v16.4h, v1.h[6]
        let v29 = vaddq_s16(v26_27.1, v18_19.1);
        let v3 = vmull_high_laneq_s16::<3>(v22_23.0, v1); // smull2 v3.4s, v22.8h, v1.h[3]
        let v3 = vmlal_high_laneq_s16::<4>(v3, v24, v1); // smlal2 v3.4s, v24.8h, v1.h[4]
        let v3 = vmlal_high_laneq_s16::<5>(v3, v28, v1); // smlal2 v3.4s, v28.8h, v1.h[5]
        let v3 = vmlal_high_laneq_s16::<6>(v3, v16, v1); // smlal2 v3.4s, v16.8h, v1.h[6]
        let v17 = vaddq_s16(v26_27.1, v16_17.1);
        let v4 = vmull_laneq_s16::<3>(vget_low_s16(v22_23.1), v1); // smull v4.4s, v23.4h, v1.h[3]
        let v4 = vmlal_laneq_s16::<4>(v4, vget_low_s16(v25), v1); // smlal v4.4s, v25.4h, v1.h[4]
        let v4 = vmlal_laneq_s16::<5>(v4, vget_low_s16(v29), v1); // smlal v4.4s, v29.4h, v1.h[5]
        let v4 = vmlal_laneq_s16::<6>(v4, vget_low_s16(v17), v1); // smlal v4.4s, v17.4h, v1.h[6]
        let v5 = vmull_high_laneq_s16::<3>(v22_23.1, v1); // smull2 v5.4s, v23.8h, v1.h[3]
        let v5 = vmlal_high_laneq_s16::<4>(v5, v25, v1); // smlal2 v5.4s, v25.8h, v1.h[4]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v29, v1); // smlal2 v5.4s, v29.8h, v1.h[5]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v17, v1); // smlal2 v5.4s, v17.8h, v1.h[6]
        let v2 = vqrshrun_n_s32::<11>(v2); // sqrshrun v2.4h, v2.4s, #11
        let v2 = vqrshrun_high_n_s32::<11>(v2, v3); // sqrshrun2 v2.8h, v3.4s, #11
        let v3 = vqrshrun_n_s32::<11>(v4); // sqrshrun v3.4h, v4.4s, #11
        let v3 = vqrshrun_high_n_s32::<11>(v3, v5); // sqrshrun2 v3.8h, v5.4s, #11
        let v2 = vqmovun_s16(vreinterpretq_s16_u16(v2)); // sqxtun v2.8b, v2.8h
        let v2 = vqmovun_high_s16(v2, vreinterpretq_s16_u16(v3)); // sqxtun2 v2.16b, v3.8h
        w4 -= 16; // subs w4, w4, #16
        vst1q_u8(x0, v2); // st1 {v2.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w4 <= 0 {
            break; // b.gt 1b
        }
    }

    // ldp x0, x4, [sp, #48]; ldp x13, x14, [sp, #32]; ldp x11, x12, [sp, #16]; ldp x9, x10, [sp], #64
    // (`r` has the shifted x9-x13 since the entry.)

    r.x0 = r.x0.wrapping_offset(r.x1); // add x0, x0, x1
}

/// `wiener_filter7_hv_8bpc_neon`: filters the source row `x3` horizontally into the row `x15` (t0) and
/// the rows `x9`-`x14` with it vertically into the output row `x0`. Leaves the row pointers shifted
/// (`x9` gets the value of `x10`, etc, `x14` that of `x15`, and `x15` the former `x10`), `x3` and `x0`
/// advanced by `stride`, and `x2` advanced by one row of `left` if it is not NULL.
#[inline(never)]
unsafe fn wiener_filter7_hv_8bpc_neon(r: &mut WienerRegs) {
    // Backing up/restoring registers shifted, so that x9 gets the value
    // of x10, etc, and x15==x9, afterwards.
    // stp x10, x11, [sp, #-80]!; stp x12, x13, [sp, #16]; stp x14, x15, [sp, #32]; stp x10, x0, [sp, #48]
    // stp x3, x4, [sp, #64]
    let (mut x9, mut x10, mut x11, mut x12, mut x13, mut x14, mut x15) =
        (r.x9, r.x10, r.x11, r.x12, r.x13, r.x14, r.x15);
    let mut x0 = r.x0;
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    // The values the `ldp`s at the end leave in x9-x15.
    r.x9 = x10;
    r.x10 = x11;
    r.x11 = x12;
    r.x12 = x13;
    r.x13 = x14;
    r.x14 = x15;
    r.x15 = x10;
    let (v0, v1, v30, v31) = (r.v0, r.v1, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let v3;
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(3);
            v3 = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
            // ld1 {v2.s}[3], [x2], #4 (the other lanes of v2 are not used)
            let v2 = vsetq_lane_u32::<3>(read_unaligned(r.x2 as *const u32), vdupq_n_u32(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 3 bytes we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(3);
            v3 = vextq_u8::<13>(vreinterpretq_u8_u32(v2), v3_);
        }
    } else {
        // 1:
        let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost byte
        // and shift v3 to have 3x the first byte at the front.
        let v2 = vdupq_laneq_u8::<0>(v3_);
        // Move x3 back to account for the last 3 bytes we loaded before,
        // which we shifted out.
        x3 = x3.sub(3);
        v3 = vextq_u8::<13>(v2, v3_);
    }

    // 2:
    let v4 = vld1_u8(x3); // ld1 {v4.8b}, [x3], #8
    x3 = x3.add(8);
    let mut v2 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v3))); // uxtl v2.8h, v3.8b
    let mut v3 = vreinterpretq_s16_u16(vmovl_high_u8(v3)); // uxtl2 v3.8h, v3.16b
    let mut v4 = vreinterpretq_s16_u16(vmovl_u8(v4)); // uxtl v4.8h, v4.8b

    loop {
        // tst w7, #2 // LR_HAVE_RIGHT; b.ne 4f
        // 3: // !LR_HAVE_RIGHT
        // Check whether we need to pad the right edge
        // cmp w4, #19; b.ge 4f // If w >= 19, all used input pixels are valid
        if r.w7 & 2 == 0 && w4 < 19 {
            // 1 <= w < 19, w+3 pixels valid in v2-v4. For w>=9,
            // this ends up called again; it's not strictly needed in those
            // cases (we pad enough here), but keeping the code as simple as possible.

            // The padding pixel is v2/3/4.h[w+2]. x3 points at the next input, ie
            // v2/3/4.h[24]. Thus read from x3[w-22] to find the padding pixel.
            let w17 = w4 - 22;
            // Insert padding in v2/3/4.h[w+3] onwards; fuse the +3 (*2) into the
            // buffer pointer.
            // movrel x6, right_ext_mask, -6; sub x6, x6, w4, uxtw #1
            let x6 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 6 - 2 * w4 as usize);
            let b28 = *x3.offset(w17 as isize); // ldr b28, [x3, w17, sxtw]
            let v28 = vreinterpretq_u8_u16(vdupq_n_u16(b28 as u16)); // dup v28.8h, v28.h[0]
            let v25_27 = vld1q_u8_x3(x6); // ld1 {v25.16b, v26.16b, v27.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v25_27.0, v28, vreinterpretq_u8_s16(v2))); // bit v2.16b, v28.16b, v25.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v25_27.1, v28, vreinterpretq_u8_s16(v3))); // bit v3.16b, v28.16b, v26.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v25_27.2, v28, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v28.16b, v27.16b
        }

        // 4: // Loop horizontally
        let v17 = vextq_s16::<2>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #4
        let v19 = vextq_s16::<4>(v2, v3); // ext v19.16b, v2.16b, v3.16b, #8
        let v16 = vextq_s16::<1>(v2, v3); // ext v16.16b, v2.16b, v3.16b, #2
        let v20 = vextq_s16::<5>(v2, v3); // ext v20.16b, v2.16b, v3.16b, #10
        let v21 = vextq_s16::<6>(v2, v3); // ext v21.16b, v2.16b, v3.16b, #12
        let v18 = vextq_s16::<3>(v2, v3); // ext v18.16b, v2.16b, v3.16b, #6
        let v19 = vaddq_s16(v19, v17);
        let v20 = vaddq_s16(v20, v16);
        let v21 = vaddq_s16(v21, v2);
        let v22 = vshlq_n_s16::<7>(v18);
        let v6 = vmulq_laneq_s16::<3>(v18, v0); // mul v6.8h, v18.8h, v0.h[3]
        let v6 = vmlaq_laneq_s16::<4>(v6, v19, v0); // mla v6.8h, v19.8h, v0.h[4]
        let v6 = vmlaq_laneq_s16::<5>(v6, v20, v0); // mla v6.8h, v20.8h, v0.h[5]
        let v6 = vmlaq_laneq_s16::<6>(v6, v21, v0); // mla v6.8h, v21.8h, v0.h[6]

        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v20 = vextq_s16::<5>(v3, v4); // ext v20.16b, v3.16b, v4.16b, #10
        let v21 = vextq_s16::<6>(v3, v4); // ext v21.16b, v3.16b, v4.16b, #12
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6

        let v19 = vaddq_s16(v19, v17);
        let v20 = vaddq_s16(v20, v16);
        let v21 = vaddq_s16(v21, v3);
        let v23 = vshlq_n_s16::<7>(v18);
        let v7 = vmulq_laneq_s16::<3>(v18, v0); // mul v7.8h, v18.8h, v0.h[3]
        let v7 = vmlaq_laneq_s16::<4>(v7, v19, v0); // mla v7.8h, v19.8h, v0.h[4]
        let v7 = vmlaq_laneq_s16::<5>(v7, v20, v0); // mla v7.8h, v20.8h, v0.h[5]
        let v7 = vmlaq_laneq_s16::<6>(v7, v21, v0); // mla v7.8h, v21.8h, v0.h[6]

        let v20_21 = vld1q_s16_x2(x11); // ld1 {v20.8h, v21.8h}, [x11], #32
        x11 = x11.wrapping_add(16);

        let v22 = vsubq_s16(v22, v30);
        let v23 = vsubq_s16(v23, v30);
        let v26_27 = vld1q_s16_x2(x13); // ld1 {v26.8h, v27.8h}, [x13], #32
        x13 = x13.wrapping_add(16);
        let v6 = vqaddq_s16(v6, v22);
        let v7 = vqaddq_s16(v7, v23);
        let v18_19 = vld1q_s16_x2(x10); // ld1 {v18.8h, v19.8h}, [x10], #32
        x10 = x10.wrapping_add(16);
        let v6 = vshrq_n_s16::<3>(v6);
        let v7 = vshrq_n_s16::<3>(v7);
        let v28_29 = vld1q_s16_x2(x14); // ld1 {v28.8h, v29.8h}, [x14], #32
        x14 = x14.wrapping_add(16);
        let v6 = vaddq_s16(v6, v31);
        let v7 = vaddq_s16(v7, v31);

        let v16_17 = vld1q_s16_x2(x9); // ld1 {v16.8h, v17.8h}, [x9], #32
        x9 = x9.wrapping_add(16);
        let v26 = vaddq_s16(v20_21.0, v26_27.0);

        let v24_25 = vld1q_s16_x2(x12); // ld1 {v24.8h, v25.8h}, [x12], #32
        x12 = x12.wrapping_add(16);
        let v28 = vaddq_s16(v18_19.0, v28_29.0);

        let v16 = vaddq_s16(v16_17.0, v6);
        let v27 = vaddq_s16(v20_21.1, v26_27.1);

        let v18 = vmull_laneq_s16::<3>(vget_low_s16(v24_25.0), v1); // smull v18.4s, v24.4h, v1.h[3]
        let v18 = vmlal_laneq_s16::<4>(v18, vget_low_s16(v26), v1); // smlal v18.4s, v26.4h, v1.h[4]
        let v18 = vmlal_laneq_s16::<5>(v18, vget_low_s16(v28), v1); // smlal v18.4s, v28.4h, v1.h[5]
        let v18 = vmlal_laneq_s16::<6>(v18, vget_low_s16(v16), v1); // smlal v18.4s, v16.4h, v1.h[6]
        let v29 = vaddq_s16(v18_19.1, v28_29.1);
        let v19 = vmull_high_laneq_s16::<3>(v24_25.0, v1); // smull2 v19.4s, v24.8h, v1.h[3]
        let v19 = vmlal_high_laneq_s16::<4>(v19, v26, v1); // smlal2 v19.4s, v26.8h, v1.h[4]
        let v19 = vmlal_high_laneq_s16::<5>(v19, v28, v1); // smlal2 v19.4s, v28.8h, v1.h[5]
        let v19 = vmlal_high_laneq_s16::<6>(v19, v16, v1); // smlal2 v19.4s, v16.8h, v1.h[6]
        let v17 = vaddq_s16(v16_17.1, v7);
        let v20 = vmull_laneq_s16::<3>(vget_low_s16(v24_25.1), v1); // smull v20.4s, v25.4h, v1.h[3]
        let v20 = vmlal_laneq_s16::<4>(v20, vget_low_s16(v27), v1); // smlal v20.4s, v27.4h, v1.h[4]
        let v20 = vmlal_laneq_s16::<5>(v20, vget_low_s16(v29), v1); // smlal v20.4s, v29.4h, v1.h[5]
        let v20 = vmlal_laneq_s16::<6>(v20, vget_low_s16(v17), v1); // smlal v20.4s, v17.4h, v1.h[6]
        let v21 = vmull_high_laneq_s16::<3>(v24_25.1, v1); // smull2 v21.4s, v25.8h, v1.h[3]
        let v21 = vmlal_high_laneq_s16::<4>(v21, v27, v1); // smlal2 v21.4s, v27.8h, v1.h[4]
        let v21 = vmlal_high_laneq_s16::<5>(v21, v29, v1); // smlal2 v21.4s, v29.8h, v1.h[5]
        let v21 = vmlal_high_laneq_s16::<6>(v21, v17, v1); // smlal2 v21.4s, v17.8h, v1.h[6]
        let v18 = vqrshrun_n_s32::<11>(v18); // sqrshrun v18.4h, v18.4s, #11
        let v18 = vqrshrun_high_n_s32::<11>(v18, v19); // sqrshrun2 v18.8h, v19.4s, #11
        let v19 = vqrshrun_n_s32::<11>(v20); // sqrshrun v19.4h, v20.4s, #11
        let v19 = vqrshrun_high_n_s32::<11>(v19, v21); // sqrshrun2 v19.8h, v21.4s, #11
        vst1q_s16(x15, v6); // st1 {v6.8h, v7.8h}, [x15], #32
        vst1q_s16(x15.add(8), v7);
        x15 = x15.wrapping_add(16);
        let v18 = vqmovun_s16(vreinterpretq_s16_u16(v18)); // sqxtun v18.8b, v18.8h
        let v18 = vqmovun_high_s16(v18, vreinterpretq_s16_u16(v19)); // sqxtun2 v18.16b, v19.8h
        w4 -= 16; // subs w4, w4, #16

        vst1q_u8(x0, v18); // st1 {v18.16b}, [x0], #16
        x0 = x0.wrapping_add(16);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b
        let v4_ = vld1q_u8(x3); // ld1 {v4.16b}, [x3], #16
        x3 = x3.add(16);
        v3 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v4_))); // uxtl v3.8h, v4.8b
        v4 = vreinterpretq_s16_u16(vmovl_high_u8(v4_)); // uxtl2 v4.8h, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }

    // 0:
    // ldp x3, x4, [sp, #64]; ldp x15, x0, [sp, #48]; ldp x13, x14, [sp, #32]; ldp x11, x12, [sp, #16]
    // ldp x9, x10, [sp], #80
    // (`r` has the shifted x9-x15 since the entry.)

    r.x3 = r.x3.wrapping_offset(r.x1); // add x3, x3, x1
    r.x0 = r.x0.wrapping_offset(r.x1); // add x0, x0, x1
}

/// `wiener_filter5_8bpc_neon`: `void dav1d_wiener_filter5_8bpc_neon(pixel *p, const ptrdiff_t stride,
/// const pixel (*left)[4], const pixel *lpf, const int w, int h, const LooprestorationParams *const params,
/// const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// Reads `params.filter` (32 bytes). Per source row (the `h` rows of `p`, with LR_HAVE_TOP the two rows at
/// `lpf` and `lpf + stride`, with LR_HAVE_BOTTOM the two rows at `lpf + 6 * stride` and `lpf + 7 * stride`)
/// it reads the bytes `[-2, 16 * n + 6)` of the row with LR_HAVE_LEFT and `left == NULL` (the `lpf` rows),
/// else `[0, 16 * n + 6)`, where `n = (w + 15) / 16`; with LR_HAVE_LEFT, 4 bytes of `left` per row of `p`.
/// Writes the bytes `[0, 16 * n)` of each of the `h` rows of `p`. The intermediate rows are in the local,
/// zero-initialized `WienerRows`.
pub unsafe extern "C" fn dav1d_wiener_filter5_8bpc_neon(
    p: *mut u8,
    stride: isize,
    left: *const [u8; 4],
    lpf: *const u8,
    w: c_int,
    h: c_int,
    params: *const LooprestorationParams,
    edges: c_uint,
) {
    let x6 = params as *const i16;
    let mut w5 = h;
    let v0_1 = vld1q_s16_x2(x6); // ld1 {v0.8h, v1.8h}, [x6]

    // sub_sp 384*2*4. Zero-initialized whole: which of the four rows a vertical pass loads depends on the
    // edge flags and the height, so that every loaded element was stored before is not obvious here.
    let mut rows = WienerRows([0; 384 * 4]);
    let sp = rows.0.as_mut_ptr();

    // x11 - t4
    // x12 - t3
    // x13 - t2
    // x14 - t1
    // x15 - t0
    // The assembly leaves x11-x13 and x15 unset until the code below assigns them (and does not use x9
    // and x10); here they start at the first row.
    let mut r = WienerRegs {
        x0: p,
        x1: stride,
        x2: left,
        x3: lpf,
        w4: w,
        w7: edges,
        x9: sp,
        x10: sp,
        x11: sp,
        x12: sp,
        x13: sp,
        x14: sp, // mov x14, sp // t1
        x15: sp,
        v0: v0_1.0,
        v1: v0_1.1,
        v30: vdupq_n_s16((1 << 14) - (1 << 2)), // mov w17, #(1 << 14) - (1 << 2); dup v30.8h, w17
        v31: vdupq_n_s16(8 << 8),               // movi v31.8h, #8, lsl #8
    };
    let x16: *const u8;

    // The number of vertical passes that remain: 2 for L(v2_5), 1 for L(v1_5), 0 for L(end_5).
    let v_passes = 'v: {
        'main_5: {
            // tst w7, #4 // LR_HAVE_TOP; b.eq L(no_top_5)
            if r.w7 & 4 != 0 {
                let x16_left = r.x2; // mov x16, x2 // backup left
                r.x2 = core::ptr::null(); // mov x2, #0
                wiener_filter5_h_8bpc_neon(&mut r);
                r.x3 = r.x3.offset(r.x1); // lpf += stride
                r.x11 = r.x14; // t4
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter5_h_8bpc_neon(&mut r);
                r.x3 = r.x3.wrapping_offset(r.x1 * 4);
                r.x3 = r.x3.wrapping_offset(r.x1); // lpf += stride*5
                r.x12 = r.x14; // t3
                r.x14 = r.x14.add(384); // t1 += 384*2
                r.x2 = x16_left; // left
                x16 = r.x3; // backup lpf
                r.x3 = r.x0; // lpf = p
                wiener_filter5_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_5)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter5_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_5)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
            } else {
                // L(no_top_5):
                r.x3 = r.x3.wrapping_offset(r.x1 * 4);
                x16 = r.x3.wrapping_offset(r.x1 * 2); // lpf += stride*6, backup
                r.x3 = r.x0; // lpf = p

                wiener_filter5_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x11 = r.x14; // t4
                r.x12 = r.x14; // t3
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_5)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter5_h_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_5)
                }
                r.x3 = r.x3.offset(r.x1); // src += stride
                r.x15 = r.x14.add(384); // t0 = t1 + 384*2
                wiener_filter5_hv_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_5)
                }
                r.x15 = r.x15.add(384 * 3); // t0 += 384*2*3
                wiener_filter5_hv_8bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 != 0 {
                    break 'main_5; // b.ne L(main_5)
                }
                break 'v 2; // L(v2_5)
            }
        }
        // L(main_5):
        r.x15 = r.x11; // t0 = t4
        loop {
            // L(main_loop_5):
            wiener_filter5_hv_8bpc_neon(&mut r);
            w5 -= 1; // h--
            if w5 == 0 {
                break; // b.ne L(main_loop_5)
            }
        }
        // tst w7, #8 // LR_HAVE_BOTTOM; b.eq L(v2_5)
        if r.w7 & 8 == 0 {
            break 'v 2;
        }

        r.x3 = x16; // restore lpf
        r.x2 = core::ptr::null(); // left = NULL
        wiener_filter5_hv_8bpc_neon(&mut r);
        wiener_filter5_hv_8bpc_neon(&mut r);
        0 // L(end_5)
    };
    if v_passes >= 2 {
        // L(v2_5):
        wiener_filter5_v_8bpc_neon(&mut r);
        r.x0 = r.x0.wrapping_offset(r.x1); // add x0, x0, x1
        r.x11 = r.x12; // mov x11, x12
        r.x12 = r.x13; // mov x12, x13
        r.x13 = r.x14; // mov x13, x14
    }
    if v_passes >= 1 {
        // L(v1_5):
        wiener_filter5_v_8bpc_neon(&mut r);
    }
    // L(end_5):
}

/// `wiener_filter5_h_8bpc_neon`: filters the source row `x3` horizontally into the row `x14`.
/// Leaves `x3`, `w4` and `x14` unchanged (the assembly saves and restores them); advances `x2` by one
/// row of `left` if it is not NULL.
#[inline(never)]
unsafe fn wiener_filter5_h_8bpc_neon(r: &mut WienerRegs) {
    // stp x3, x4, [sp, #-32]!; str x14, [sp, #16]
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    let mut x14 = r.x14;
    let (v0, v30, v31) = (r.v0, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let v3;
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(2);
            v3 = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
            // ld1 {v2.s}[3], [x2], #4 (the other lanes of v2 are not used)
            let v2 = vsetq_lane_u32::<3>(read_unaligned(r.x2 as *const u32), vdupq_n_u32(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 2 bytes we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(2);
            v3 = vextq_u8::<14>(vreinterpretq_u8_u32(v2), v3_);
        }
    } else {
        // 1:
        let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost byte
        // and shift v3 to have 3x the first byte at the front.
        let v2 = vdupq_laneq_u8::<0>(v3_);
        // Move x3 back to account for the last 2 bytes we loaded before,
        // which we shifted out.
        x3 = x3.sub(2);
        v3 = vextq_u8::<14>(v2, v3_);
    }

    // 2:
    let v4 = vld1_u8(x3); // ld1 {v4.8b}, [x3], #8
    x3 = x3.add(8);
    let mut v2 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v3))); // uxtl v2.8h, v3.8b
    let mut v3 = vreinterpretq_s16_u16(vmovl_high_u8(v3)); // uxtl2 v3.8h, v3.16b
    let mut v4 = vreinterpretq_s16_u16(vmovl_u8(v4)); // uxtl v4.8h, v4.8b

    loop {
        // tst w7, #2 // LR_HAVE_RIGHT; b.ne 4f
        // 3: // !LR_HAVE_RIGHT
        // Check whether we need to pad the right edge
        // cmp w4, #18; b.ge 4f // If w >= 18, all used input pixels are valid
        if r.w7 & 2 == 0 && w4 < 18 {
            // 1 <= w < 18, w+2 pixels valid in v2-v4. For w>=9,
            // this ends up called again; it's not strictly needed in those
            // cases (we pad enough here), but keeping the code as simple as possible.

            // The padding pixel is v2/3/4.h[w+2]. x3 points at the next input, ie
            // v2/3/4.h[24]. Thus read from x3[w-23] to find the padding pixel.
            let w17 = w4 - 23;
            // Insert padding in v2/3/4.h[w+2] onwards; fuse the +2 (*2) into the
            // buffer pointer.
            // movrel x6, right_ext_mask, -4; sub x6, x6, w4, uxtw #1
            let x6 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 4 - 2 * w4 as usize);
            let b28 = *x3.offset(w17 as isize); // ldr b28, [x3, w17, sxtw]
            let v28 = vreinterpretq_u8_u16(vdupq_n_u16(b28 as u16)); // dup v28.8h, v28.h[0]
            let v25_27 = vld1q_u8_x3(x6); // ld1 {v25.16b, v26.16b, v27.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v25_27.0, v28, vreinterpretq_u8_s16(v2))); // bit v2.16b, v28.16b, v25.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v25_27.1, v28, vreinterpretq_u8_s16(v3))); // bit v3.16b, v28.16b, v26.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v25_27.2, v28, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v28.16b, v27.16b
        }

        // 4: // Loop horizontally
        let v16 = vextq_s16::<1>(v2, v3); // ext v16.16b, v2.16b, v3.16b, #2
        let v18 = vextq_s16::<3>(v2, v3); // ext v18.16b, v2.16b, v3.16b, #6
        let v19 = vextq_s16::<4>(v2, v3); // ext v19.16b, v2.16b, v3.16b, #8
        let v17 = vextq_s16::<2>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v2);
        let v22 = vshlq_n_s16::<7>(v17);
        let v6 = vmulq_laneq_s16::<3>(v17, v0); // mul v6.8h, v17.8h, v0.h[3]
        let v6 = vmlaq_laneq_s16::<4>(v6, v18, v0); // mla v6.8h, v18.8h, v0.h[4]
        let v6 = vmlaq_laneq_s16::<5>(v6, v19, v0); // mla v6.8h, v19.8h, v0.h[5]

        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v3);
        let v23 = vshlq_n_s16::<7>(v17);
        let v7 = vmulq_laneq_s16::<3>(v17, v0); // mul v7.8h, v17.8h, v0.h[3]
        let v7 = vmlaq_laneq_s16::<4>(v7, v18, v0); // mla v7.8h, v18.8h, v0.h[4]
        let v7 = vmlaq_laneq_s16::<5>(v7, v19, v0); // mla v7.8h, v19.8h, v0.h[5]

        let v22 = vsubq_s16(v22, v30);
        let v23 = vsubq_s16(v23, v30);
        let v6 = vqaddq_s16(v6, v22);
        let v7 = vqaddq_s16(v7, v23);
        let v6 = vshrq_n_s16::<3>(v6);
        let v7 = vshrq_n_s16::<3>(v7);
        let v6 = vaddq_s16(v6, v31);
        let v7 = vaddq_s16(v7, v31);

        w4 -= 16; // subs w4, w4, #16

        vst1q_s16(x14, v6); // st1 {v6.8h, v7.8h}, [x14], #32
        vst1q_s16(x14.add(8), v7);
        x14 = x14.wrapping_add(16);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b
        let v4_ = vld1q_u8(x3); // ld1 {v4.16b}, [x3], #16
        x3 = x3.add(16);
        v3 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v4_))); // uxtl v3.8h, v4.8b
        v4 = vreinterpretq_s16_u16(vmovl_high_u8(v4_)); // uxtl2 v4.8h, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }
    // 0:
    // ldr x14, [sp, #16]; ldp x3, x4, [sp], #32
}

/// `wiener_filter5_v_8bpc_neon`: filters the rows `x11`-`x14` (t4-t1, with t1 standing in for t0)
/// vertically into the output row `x0`. Leaves every register unchanged (the assembly saves and
/// restores the row pointers, `x0` and `w4`).
#[inline(never)]
unsafe fn wiener_filter5_v_8bpc_neon(r: &mut WienerRegs) {
    // stp x11, x12, [sp, #-48]!; stp x13, x14, [sp, #16]; stp x0, x4, [sp, #32]
    let (mut x11, mut x12, mut x13, mut x14) = (r.x11, r.x12, r.x13, r.x14);
    let mut x0 = r.x0;
    let mut w4 = r.w4;
    let v1 = r.v1;
    loop {
        // 1:
        let v18_19 = vld1q_s16_x2(x12); // ld1 {v18.8h, v19.8h}, [x12], #32
        x12 = x12.wrapping_add(16);
        let v22_23 = vld1q_s16_x2(x14); // ld1 {v22.8h, v23.8h}, [x14], #32
        x14 = x14.wrapping_add(16);
        let v16_17 = vld1q_s16_x2(x11); // ld1 {v16.8h, v17.8h}, [x11], #32
        x11 = x11.wrapping_add(16);

        let v24 = vaddq_s16(v22_23.0, v18_19.0);
        let v20_21 = vld1q_s16_x2(x13); // ld1 {v20.8h, v21.8h}, [x13], #32
        x13 = x13.wrapping_add(16);
        let v16 = vaddq_s16(v22_23.0, v16_17.0);
        let v25 = vaddq_s16(v22_23.1, v18_19.1);

        let v2 = vmull_laneq_s16::<3>(vget_low_s16(v20_21.0), v1); // smull v2.4s, v20.4h, v1.h[3]
        let v2 = vmlal_laneq_s16::<4>(v2, vget_low_s16(v24), v1); // smlal v2.4s, v24.4h, v1.h[4]
        let v2 = vmlal_laneq_s16::<5>(v2, vget_low_s16(v16), v1); // smlal v2.4s, v16.4h, v1.h[5]
        let v17 = vaddq_s16(v22_23.1, v16_17.1);
        let v3 = vmull_high_laneq_s16::<3>(v20_21.0, v1); // smull2 v3.4s, v20.8h, v1.h[3]
        let v3 = vmlal_high_laneq_s16::<4>(v3, v24, v1); // smlal2 v3.4s, v24.8h, v1.h[4]
        let v3 = vmlal_high_laneq_s16::<5>(v3, v16, v1); // smlal2 v3.4s, v16.8h, v1.h[5]
        let v4 = vmull_laneq_s16::<3>(vget_low_s16(v20_21.1), v1); // smull v4.4s, v21.4h, v1.h[3]
        let v4 = vmlal_laneq_s16::<4>(v4, vget_low_s16(v25), v1); // smlal v4.4s, v25.4h, v1.h[4]
        let v4 = vmlal_laneq_s16::<5>(v4, vget_low_s16(v17), v1); // smlal v4.4s, v17.4h, v1.h[5]
        let v5 = vmull_high_laneq_s16::<3>(v20_21.1, v1); // smull2 v5.4s, v21.8h, v1.h[3]
        let v5 = vmlal_high_laneq_s16::<4>(v5, v25, v1); // smlal2 v5.4s, v25.8h, v1.h[4]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v17, v1); // smlal2 v5.4s, v17.8h, v1.h[5]
        let v2 = vqrshrun_n_s32::<11>(v2); // sqrshrun v2.4h, v2.4s, #11
        let v2 = vqrshrun_high_n_s32::<11>(v2, v3); // sqrshrun2 v2.8h, v3.4s, #11
        let v3 = vqrshrun_n_s32::<11>(v4); // sqrshrun v3.4h, v4.4s, #11
        let v3 = vqrshrun_high_n_s32::<11>(v3, v5); // sqrshrun2 v3.8h, v5.4s, #11
        let v2 = vqmovun_s16(vreinterpretq_s16_u16(v2)); // sqxtun v2.8b, v2.8h
        let v2 = vqmovun_high_s16(v2, vreinterpretq_s16_u16(v3)); // sqxtun2 v2.16b, v3.8h
        w4 -= 16; // subs w4, w4, #16
        vst1q_u8(x0, v2); // st1 {v2.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w4 <= 0 {
            break; // b.gt 1b
        }
    }

    // ldp x0, x4, [sp, #32]; ldp x13, x14, [sp, #16]; ldp x11, x12, [sp], #48
}

/// `wiener_filter5_hv_8bpc_neon`: filters the source row `x3` horizontally into the row `x15` (t0) and
/// the rows `x11`-`x14` with it vertically into the output row `x0`. Leaves the row pointers shifted
/// (`x11` gets the value of `x12`, etc, `x14` that of `x15`, and `x15` the former `x12`), `x3` and `x0`
/// advanced by `stride`, and `x2` advanced by one row of `left` if it is not NULL.
#[inline(never)]
unsafe fn wiener_filter5_hv_8bpc_neon(r: &mut WienerRegs) {
    // Backing up/restoring registers shifted, so that x11 gets the value
    // of x12, etc, and x15==x11, afterwards.
    // stp x12, x13, [sp, #-64]!; stp x14, x15, [sp, #16]; stp x12, x0, [sp, #32]; stp x3, x4, [sp, #48]
    let (mut x11, mut x12, mut x13, mut x14, mut x15) = (r.x11, r.x12, r.x13, r.x14, r.x15);
    let mut x0 = r.x0;
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    // The values the `ldp`s at the end leave in x11-x15.
    r.x11 = x12;
    r.x12 = x13;
    r.x13 = x14;
    r.x14 = x15;
    r.x15 = x12;
    let (v0, v1, v30, v31) = (r.v0, r.v1, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let v3;
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(2);
            v3 = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
            x3 = x3.add(16);
            // ld1 {v2.s}[3], [x2], #4 (the other lanes of v2 are not used)
            let v2 = vsetq_lane_u32::<3>(read_unaligned(r.x2 as *const u32), vdupq_n_u32(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 2 bytes we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(2);
            v3 = vextq_u8::<14>(vreinterpretq_u8_u32(v2), v3_);
        }
    } else {
        // 1:
        let v3_ = vld1q_u8(x3); // ld1 {v3.16b}, [x3], #16
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost byte
        // and shift v3 to have 2x the first byte at the front.
        let v2 = vdupq_laneq_u8::<0>(v3_);
        // Move x3 back to account for the last 2 bytes we loaded before,
        // which we shifted out.
        x3 = x3.sub(2);
        v3 = vextq_u8::<14>(v2, v3_);
    }

    // 2:
    let v4 = vld1_u8(x3); // ld1 {v4.8b}, [x3], #8
    x3 = x3.add(8);
    let mut v2 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v3))); // uxtl v2.8h, v3.8b
    let mut v3 = vreinterpretq_s16_u16(vmovl_high_u8(v3)); // uxtl2 v3.8h, v3.16b
    let mut v4 = vreinterpretq_s16_u16(vmovl_u8(v4)); // uxtl v4.8h, v4.8b

    loop {
        // tst w7, #2 // LR_HAVE_RIGHT; b.ne 4f
        // 3: // !LR_HAVE_RIGHT
        // Check whether we need to pad the right edge
        // cmp w4, #18; b.ge 4f // If w >= 18, all used input pixels are valid
        if r.w7 & 2 == 0 && w4 < 18 {
            // 1 <= w < 18, w+2 pixels valid in v2-v4. For w>=9,
            // this ends up called again; it's not strictly needed in those
            // cases (we pad enough here), but keeping the code as simple as possible.

            // The padding pixel is v2/3/4.h[w+1]. x3 points at the next input, ie
            // v2/3/4.h[24]. Thus read from x3[w-23] to find the padding pixel.
            let w17 = w4 - 23;
            // Insert padding in v2/3/4.h[w+2] onwards; fuse the +2 (*2) into the
            // buffer pointer.
            // movrel x6, right_ext_mask, -4; sub x6, x6, w4, uxtw #1
            let x6 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 4 - 2 * w4 as usize);
            let b28 = *x3.offset(w17 as isize); // ldr b28, [x3, w17, sxtw]
            let v28 = vreinterpretq_u8_u16(vdupq_n_u16(b28 as u16)); // dup v28.8h, v28.h[0]
            let v25_27 = vld1q_u8_x3(x6); // ld1 {v25.16b, v26.16b, v27.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v25_27.0, v28, vreinterpretq_u8_s16(v2))); // bit v2.16b, v28.16b, v25.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v25_27.1, v28, vreinterpretq_u8_s16(v3))); // bit v3.16b, v28.16b, v26.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v25_27.2, v28, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v28.16b, v27.16b
        }

        // 4: // Loop horizontally
        let v16 = vextq_s16::<1>(v2, v3); // ext v16.16b, v2.16b, v3.16b, #2
        let v18 = vextq_s16::<3>(v2, v3); // ext v18.16b, v2.16b, v3.16b, #6
        let v19 = vextq_s16::<4>(v2, v3); // ext v19.16b, v2.16b, v3.16b, #8
        let v17 = vextq_s16::<2>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v2);
        let v22 = vshlq_n_s16::<7>(v17);
        let v6 = vmulq_laneq_s16::<3>(v17, v0); // mul v6.8h, v17.8h, v0.h[3]
        let v6 = vmlaq_laneq_s16::<4>(v6, v18, v0); // mla v6.8h, v18.8h, v0.h[4]
        let v6 = vmlaq_laneq_s16::<5>(v6, v19, v0); // mla v6.8h, v19.8h, v0.h[5]

        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v3);
        let v23 = vshlq_n_s16::<7>(v17);
        let v7 = vmulq_laneq_s16::<3>(v17, v0); // mul v7.8h, v17.8h, v0.h[3]
        let v7 = vmlaq_laneq_s16::<4>(v7, v18, v0); // mla v7.8h, v18.8h, v0.h[4]
        let v7 = vmlaq_laneq_s16::<5>(v7, v19, v0); // mla v7.8h, v19.8h, v0.h[5]

        let v18_19 = vld1q_s16_x2(x12); // ld1 {v18.8h, v19.8h}, [x12], #32
        x12 = x12.wrapping_add(16);

        let v22 = vsubq_s16(v22, v30);
        let v23 = vsubq_s16(v23, v30);
        let v24_25 = vld1q_s16_x2(x14); // ld1 {v24.8h, v25.8h}, [x14], #32
        x14 = x14.wrapping_add(16);
        let v6 = vqaddq_s16(v6, v22);
        let v7 = vqaddq_s16(v7, v23);
        let v16_17 = vld1q_s16_x2(x11); // ld1 {v16.8h, v17.8h}, [x11], #32
        x11 = x11.wrapping_add(16);
        let v6 = vshrq_n_s16::<3>(v6);
        let v7 = vshrq_n_s16::<3>(v7);
        let v20_21 = vld1q_s16_x2(x13); // ld1 {v20.8h, v21.8h}, [x13], #32
        x13 = x13.wrapping_add(16);
        let v6 = vaddq_s16(v6, v31);
        let v7 = vaddq_s16(v7, v31);

        let v24 = vaddq_s16(v24_25.0, v18_19.0);
        let v16 = vaddq_s16(v16_17.0, v6);

        let v18 = vmull_laneq_s16::<3>(vget_low_s16(v20_21.0), v1); // smull v18.4s, v20.4h, v1.h[3]
        let v18 = vmlal_laneq_s16::<4>(v18, vget_low_s16(v24), v1); // smlal v18.4s, v24.4h, v1.h[4]
        let v18 = vmlal_laneq_s16::<5>(v18, vget_low_s16(v16), v1); // smlal v18.4s, v16.4h, v1.h[5]
        let v25 = vaddq_s16(v24_25.1, v18_19.1);
        let v19 = vmull_high_laneq_s16::<3>(v20_21.0, v1); // smull2 v19.4s, v20.8h, v1.h[3]
        let v19 = vmlal_high_laneq_s16::<4>(v19, v24, v1); // smlal2 v19.4s, v24.8h, v1.h[4]
        let v19 = vmlal_high_laneq_s16::<5>(v19, v16, v1); // smlal2 v19.4s, v16.8h, v1.h[5]
        let v17 = vaddq_s16(v16_17.1, v7);
        let v20 = vmull_laneq_s16::<3>(vget_low_s16(v20_21.1), v1); // smull v20.4s, v21.4h, v1.h[3]
        let v20 = vmlal_laneq_s16::<4>(v20, vget_low_s16(v25), v1); // smlal v20.4s, v25.4h, v1.h[4]
        let v20 = vmlal_laneq_s16::<5>(v20, vget_low_s16(v17), v1); // smlal v20.4s, v17.4h, v1.h[5]
        let v21 = vmull_high_laneq_s16::<3>(v20_21.1, v1); // smull2 v21.4s, v21.8h, v1.h[3]
        let v21 = vmlal_high_laneq_s16::<4>(v21, v25, v1); // smlal2 v21.4s, v25.8h, v1.h[4]
        let v21 = vmlal_high_laneq_s16::<5>(v21, v17, v1); // smlal2 v21.4s, v17.8h, v1.h[5]
        let v18 = vqrshrun_n_s32::<11>(v18); // sqrshrun v18.4h, v18.4s, #11
        let v18 = vqrshrun_high_n_s32::<11>(v18, v19); // sqrshrun2 v18.8h, v19.4s, #11
        let v19 = vqrshrun_n_s32::<11>(v20); // sqrshrun v19.4h, v20.4s, #11
        let v19 = vqrshrun_high_n_s32::<11>(v19, v21); // sqrshrun2 v19.8h, v21.4s, #11
        vst1q_s16(x15, v6); // st1 {v6.8h, v7.8h}, [x15], #32
        vst1q_s16(x15.add(8), v7);
        x15 = x15.wrapping_add(16);
        let v18 = vqmovun_s16(vreinterpretq_s16_u16(v18)); // sqxtun v18.8b, v18.8h
        let v18 = vqmovun_high_s16(v18, vreinterpretq_s16_u16(v19)); // sqxtun2 v18.16b, v19.8h
        w4 -= 16; // subs w4, w4, #16

        vst1q_u8(x0, v18); // st1 {v18.16b}, [x0], #16
        x0 = x0.wrapping_add(16);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b
        let v4_ = vld1q_u8(x3); // ld1 {v4.16b}, [x3], #16
        x3 = x3.add(16);
        v3 = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v4_))); // uxtl v3.8h, v4.8b
        v4 = vreinterpretq_s16_u16(vmovl_high_u8(v4_)); // uxtl2 v4.8h, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }

    // 0:
    // ldp x3, x4, [sp, #48]; ldp x15, x0, [sp, #32]; ldp x13, x14, [sp, #16]; ldp x11, x12, [sp], #64
    // (`r` has the shifted x11-x15 since the entry.)

    r.x3 = r.x3.wrapping_offset(r.x1); // add x3, x3, x1
    r.x0 = r.x0.wrapping_offset(r.x1); // add x0, x0, x1
}

/// `ld1 {v1.s}[3], [x2]`: the four pixels of `left`, here in every 32-bit lane (only lane 3 is read by the `ext`
/// that follows).
#[inline(always)]
unsafe fn sgr_ld1_left_8bpc(x2: *const [u8; 4]) -> uint8x16_t {
    vreinterpretq_u8_u32(vdupq_n_u32(read_unaligned(x2 as *const u32)))
}

/// `sgr_box3_row_h_8bpc_neon`: `void dav1d_sgr_box3_row_h_8bpc_neon(int32_t *sumsq, int16_t *sum,
/// const pixel (*left)[4], const pixel *src, const int w, const enum LrEdgeFlags edges)`
///
/// # Safety
/// See the notes at the top of this part: `sumsq` and `sum` writable for `8 * ceil((w + 2) / 8)` elements; `src`
/// readable as described there; `left` null or four pixels.
pub unsafe extern "C" fn dav1d_sgr_box3_row_h_8bpc_neon(
    sumsq: *mut i32,
    sum: *mut i16,
    left: *const [u8; 4],
    src: *const u8,
    w: c_int,
    edges: c_uint,
) {
    let (mut x0, mut x1, x2, mut x3, w5) = (sumsq as *mut u32, sum as *mut u16, left, src, edges);
    let mut w4 = w + 2; // add w4, w4, #2 (w += 2)

    let mut v0;
    if w5 & 1 != 0 {
        // tst w5, #1 (LR_HAVE_LEFT); b.eq 1f
        if x2.is_null() {
            // cbnz x2, 0f
            // LR_HAVE_LEFT && left == NULL
            x3 = x3.sub(2); // sub x3, x3, #2
            v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3], #16
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3], #16
            x3 = x3.add(16);
            let v1 = sgr_ld1_left_8bpc(x2); // ld1 {v1.s}[3], [x2]
                                            // Move x3 back to account for the last 2 bytes we loaded earlier,
                                            // which we'll shift out.
            x3 = x3.sub(2); // sub x3, x3, #2
            v0 = vextq_u8::<14>(v1, v0); // ext v0.16b, v1.16b, v0.16b, #14
        }
    } else {
        // 1:
        v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3], #16
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v1 with the leftmost byte
        // and shift v0 to have 2x the first byte at the front.
        let v1 = vdupq_laneq_u8::<0>(v0); // dup v1.16b, v0.b[0]
                                          // Move x3 back to account for the last 2 bytes we loaded before,
                                          // which we shifted out.
        x3 = x3.sub(2); // sub x3, x3, #2
        v0 = vextq_u8::<14>(v1, v0); // ext v0.16b, v1.16b, v0.16b, #14
    }

    // 2:
    let mut v1 = vmull_u8(vget_low_u8(v0), vget_low_u8(v0)); // umull v1.8h, v0.8b, v0.8b
    let mut v2 = vmull_high_u8(v0, v0); // umull2 v2.8h, v0.16b, v0.16b

    // 4: Loop horizontally: one block of 8 sums; leaves the loop around it at `b.le 9f`.
    macro_rules! loop_horizontally {
        () => {
            let v16 = vextq_u8::<1>(v0, v0); // ext v16.16b, v0.16b, v0.16b, #1
            let v17 = vextq_u8::<2>(v0, v0); // ext v17.16b, v0.16b, v0.16b, #2
            let v3 = vaddl_u8(vget_low_u8(v0), vget_low_u8(v16)); // uaddl v3.8h, v0.8b, v16.8b
            let v20 = vextq_u16::<1>(v1, v2); // ext v20.16b, v1.16b, v2.16b, #2
            let v3 = vaddw_u8(v3, vget_low_u8(v17)); // uaddw v3.8h, v3.8h, v17.8b

            let v21 = vextq_u16::<2>(v1, v2); // ext v21.16b, v1.16b, v2.16b, #4

            let v26 = vaddl_u16(vget_low_u16(v1), vget_low_u16(v20)); // uaddl v26.4s, v1.4h, v20.4h
            let v27 = vaddl_high_u16(v1, v20); // uaddl2 v27.4s, v1.8h, v20.8h
            let v26 = vaddw_u16(v26, vget_low_u16(v21)); // uaddw v26.4s, v26.4s, v21.4h
            let v27 = vaddw_high_u16(v27, v21); // uaddw2 v27.4s, v27.4s, v21.8h

            w4 -= 8; // subs w4, w4, #8

            vst1q_u16(x1, v3); // st1 {v3.8h}, [x1], #16
            x1 = x1.add(8);
            vst1q_u32_x2(x0, uint32x4x2_t(v26, v27)); // st1 {v26.4s, v27.4s}, [x0], #32
            x0 = x0.add(8);

            if w4 <= 0 {
                break; // b.le 9f
            }
            // tst w5, #2 (LR_HAVE_RIGHT)
            let v3 = vld1_u8(x3); // ld1 {v3.8b}, [x3], #8
            x3 = x3.add(8);
            v1 = v2; // mov v1.16b, v2.16b
            v0 = vcombine_u8(vget_high_u8(v0), v3); // ext v0.16b, v0.16b, v3.16b, #8
            v2 = vmull_u8(v3, v3); // umull v2.8h, v3.8b, v3.8b
        };
    }

    if w5 & 2 != 0 {
        // tst w5, #2 (LR_HAVE_RIGHT); b.ne 4f
        loop {
            loop_horizontally!();
            // b.ne 4b (If we don't need to pad, just keep summing.)
        }
    } else {
        // If we'll need to pad the right edge, load that byte to pad with
        // here since we can find it pretty easily from here.
        let w13 = w4 - (2 + 16 - 2 + 1); // sub w13, w4, #(2 + 16 - 2 + 1)

        // Fill v30 with the right padding pixel
        // ldr b30, [x3, w13, sxtw]; dup v30.16b, v30.b[0]
        let v30 = vld1q_dup_u8(x3.offset(w13 as isize));
        loop {
            // 3: !LR_HAVE_RIGHT

            // Check whether we need to pad the right edge
            if w4 < 10 {
                // cmp w4, #10; b.ge 4f (If w >= 10, all used input pixels are valid)

                // 1 <= w < 10, w pixels valid in v0. For w=9, this ends up called
                // again; it's not strictly needed in those cases (we pad enough here),
                // but keeping the code as simple as possible.

                // Insert padding in v0.b[w] onwards
                let x13 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK); // movrel x13, right_ext_mask
                let x13 = x13.sub(w4 as u32 as usize); // sub x13, x13, w4, uxtw
                let v29 = vld1q_u8(x13); // ld1 {v29.16b}, [x13]

                v0 = vbslq_u8(v29, v30, v0); // bit v0.16b, v30.16b, v29.16b

                // Update the precalculated squares
                v1 = vmull_u8(vget_low_u8(v0), vget_low_u8(v0)); // umull v1.8h, v0.8b, v0.8b
                v2 = vmull_high_u8(v0, v0); // umull2 v2.8h, v0.16b, v0.16b
            }

            loop_horizontally!();
            // b 3b (If we need to pad, check how many pixels we have left.)
        }
    }
    // 9:
}

/// `sgr_box5_row_h_8bpc_neon`: `void dav1d_sgr_box5_row_h_8bpc_neon(int32_t *sumsq, int16_t *sum,
/// const pixel (*left)[4], const pixel *src, const int w, const enum LrEdgeFlags edges)`
///
/// # Safety
/// See the notes at the top of this part: `sumsq` and `sum` writable for `8 * ceil((w + 2) / 8)` elements; `src`
/// readable as described there; `left` null or four pixels.
pub unsafe extern "C" fn dav1d_sgr_box5_row_h_8bpc_neon(
    sumsq: *mut i32,
    sum: *mut i16,
    left: *const [u8; 4],
    src: *const u8,
    w: c_int,
    edges: c_uint,
) {
    let (mut x0, mut x1, x2, mut x3, w5) = (sumsq as *mut u32, sum as *mut u16, left, src, edges);
    let mut w4 = w + 2; // add w4, w4, #2 (w += 2)

    let mut v0;
    if w5 & 1 != 0 {
        // tst w5, #1 (LR_HAVE_LEFT); b.eq 1f
        if x2.is_null() {
            // cbnz x2, 0f
            // LR_HAVE_LEFT && left == NULL
            x3 = x3.sub(3); // sub x3, x3, #3
            v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3], #16
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3], #16
            x3 = x3.add(16);
            let v1 = sgr_ld1_left_8bpc(x2); // ld1 {v1.s}[3], [x2], #4
                                            // Move x3 back to account for the last 3 bytes we loaded earlier,
                                            // which we'll shift out.
            x3 = x3.sub(3); // sub x3, x3, #3
            v0 = vextq_u8::<13>(v1, v0); // ext v0.16b, v1.16b, v0.16b, #13
        }
    } else {
        // 1:
        v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3], #16
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v1 with the leftmost byte
        // and shift v0 to have 3x the first byte at the front.
        let v1 = vdupq_laneq_u8::<0>(v0); // dup v1.16b, v0.b[0]
                                          // Move x3 back to account for the last 3 bytes we loaded before,
                                          // which we shifted out.
        x3 = x3.sub(3); // sub x3, x3, #3
        v0 = vextq_u8::<13>(v1, v0); // ext v0.16b, v1.16b, v0.16b, #13
    }

    // 2:
    let mut v1 = vmull_u8(vget_low_u8(v0), vget_low_u8(v0)); // umull v1.8h, v0.8b, v0.8b
    let mut v2 = vmull_high_u8(v0, v0); // umull2 v2.8h, v0.16b, v0.16b

    // 4: Loop horizontally: one block of 8 sums; leaves the loop around it at `b.le 9f`.
    macro_rules! loop_horizontally {
        () => {
            let v16 = vextq_u8::<1>(v0, v0); // ext v16.16b, v0.16b, v0.16b, #1
            let v17 = vextq_u8::<2>(v0, v0); // ext v17.16b, v0.16b, v0.16b, #2
            let v18 = vextq_u8::<3>(v0, v0); // ext v18.16b, v0.16b, v0.16b, #3
            let v19 = vextq_u8::<4>(v0, v0); // ext v19.16b, v0.16b, v0.16b, #4
            let v3 = vaddl_u8(vget_low_u8(v0), vget_low_u8(v16)); // uaddl v3.8h, v0.8b, v16.8b
            let v24 = vaddl_u8(vget_low_u8(v17), vget_low_u8(v18)); // uaddl v24.8h, v17.8b, v18.8b
            let v3 = vaddw_u8(v3, vget_low_u8(v19)); // uaddw v3.8h, v3.8h, v19.8b
            let v3 = vaddq_u16(v3, v24); // add v3.8h, v3.8h, v24.8h

            let v16 = vextq_u16::<1>(v1, v2); // ext v16.16b, v1.16b, v2.16b, #2
            let v17 = vextq_u16::<2>(v1, v2); // ext v17.16b, v1.16b, v2.16b, #4
            let v18 = vextq_u16::<3>(v1, v2); // ext v18.16b, v1.16b, v2.16b, #6
            let v19 = vextq_u16::<4>(v1, v2); // ext v19.16b, v1.16b, v2.16b, #8

            let v26 = vaddl_u16(vget_low_u16(v1), vget_low_u16(v16)); // uaddl v26.4s, v1.4h, v16.4h
            let v27 = vaddl_high_u16(v1, v16); // uaddl2 v27.4s, v1.8h, v16.8h
            let v16s = vaddl_u16(vget_low_u16(v17), vget_low_u16(v18)); // uaddl v16.4s, v17.4h, v18.4h
            let v17s = vaddl_high_u16(v17, v18); // uaddl2 v17.4s, v17.8h, v18.8h
            let v26 = vaddw_u16(v26, vget_low_u16(v19)); // uaddw v26.4s, v26.4s, v19.4h
            let v27 = vaddw_high_u16(v27, v19); // uaddw2 v27.4s, v27.4s, v19.8h
            let v26 = vaddq_u32(v26, v16s); // add v26.4s, v26.4s, v16.4s
            let v27 = vaddq_u32(v27, v17s); // add v27.4s, v27.4s, v17.4s

            w4 -= 8; // subs w4, w4, #8

            vst1q_u16(x1, v3); // st1 {v3.8h}, [x1], #16
            x1 = x1.add(8);
            vst1q_u32_x2(x0, uint32x4x2_t(v26, v27)); // st1 {v26.4s, v27.4s}, [x0], #32
            x0 = x0.add(8);

            if w4 <= 0 {
                break; // b.le 9f
            }
            // tst w5, #2 (LR_HAVE_RIGHT)
            let v3 = vld1_u8(x3); // ld1 {v3.8b}, [x3], #8
            x3 = x3.add(8);
            v1 = v2; // mov v1.16b, v2.16b
            v0 = vcombine_u8(vget_high_u8(v0), v3); // ext v0.16b, v0.16b, v3.16b, #8
            v2 = vmull_u8(v3, v3); // umull v2.8h, v3.8b, v3.8b
        };
    }

    if w5 & 2 != 0 {
        // tst w5, #2 (LR_HAVE_RIGHT); b.ne 4f
        loop {
            loop_horizontally!();
            // b.ne 4b (If we don't need to pad, just keep summing.)
        }
    } else {
        // If we'll need to pad the right edge, load that byte to pad with
        // here since we can find it pretty easily from here.
        let w13 = w4 - (2 + 16 - 3 + 1); // sub w13, w4, #(2 + 16 - 3 + 1)

        // Fill v30 with the right padding pixel
        // ldr b30, [x3, w13, sxtw]; dup v30.16b, v30.b[0]
        let v30 = vld1q_dup_u8(x3.offset(w13 as isize));
        loop {
            // 3: !LR_HAVE_RIGHT

            // Check whether we need to pad the right edge
            if w4 < 11 {
                // cmp w4, #11; b.ge 4f (If w >= 11, all used input pixels are valid)

                // 1 <= w < 11, w+1 pixels valid in v0. For w=9 or w=10,
                // this ends up called again; it's not strictly needed in those
                // cases (we pad enough here), but keeping the code as simple as possible.

                // Insert padding in v0.b[w+1] onwards; fuse the +1 into the
                // buffer pointer.
                let x13 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 1); // movrel x13, right_ext_mask, -1
                let x13 = x13.sub(w4 as u32 as usize); // sub x13, x13, w4, uxtw
                let v29 = vld1q_u8(x13); // ld1 {v29.16b}, [x13]

                v0 = vbslq_u8(v29, v30, v0); // bit v0.16b, v30.16b, v29.16b

                // Update the precalculated squares
                v1 = vmull_u8(vget_low_u8(v0), vget_low_u8(v0)); // umull v1.8h, v0.8b, v0.8b
                v2 = vmull_high_u8(v0, v0); // umull2 v2.8h, v0.16b, v0.16b
            }

            loop_horizontally!();
            // b 3b (If we need to pad, check how many pixels we have left.)
        }
    }
    // 9:
}

/// `sgr_box35_row_h_8bpc_neon`: `void dav1d_sgr_box35_row_h_8bpc_neon(int32_t *sumsq3, int16_t *sum3,
/// int32_t *sumsq5, int16_t *sum5, const pixel (*left)[4], const pixel *src, const int w,
/// const enum LrEdgeFlags edges)`
///
/// # Safety
/// See the notes at the top of this part: the four output rows writable for `8 * ceil((w + 2) / 8)` elements; `src`
/// readable as described there; `left` null or four pixels.
pub unsafe extern "C" fn dav1d_sgr_box35_row_h_8bpc_neon(
    sumsq3: *mut i32,
    sum3: *mut i16,
    sumsq5: *mut i32,
    sum5: *mut i16,
    left: *const [u8; 4],
    src: *const u8,
    w: c_int,
    edges: c_uint,
) {
    let (mut x0, mut x1, mut x2, mut x3) = (
        sumsq3 as *mut u32,
        sum3 as *mut u16,
        sumsq5 as *mut u32,
        sum5 as *mut u16,
    );
    let (x4, mut x5, w7) = (left, src, edges);
    let mut w6 = w + 2; // add w6, w6, #2 (w += 2)

    let mut v0;
    if w7 & 1 != 0 {
        // tst w7, #1 (LR_HAVE_LEFT); b.eq 1f
        if x4.is_null() {
            // cbnz x4, 0f
            // LR_HAVE_LEFT && left == NULL
            x5 = x5.sub(3); // sub x5, x5, #3
            v0 = vld1q_u8(x5); // ld1 {v0.16b}, [x5], #16
            x5 = x5.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            v0 = vld1q_u8(x5); // ld1 {v0.16b}, [x5], #16
            x5 = x5.add(16);
            let v1 = sgr_ld1_left_8bpc(x4); // ld1 {v1.s}[3], [x4], #4
                                            // Move x3 back to account for the last 3 bytes we loaded earlier,
                                            // which we'll shift out.
            x5 = x5.sub(3); // sub x5, x5, #3
            v0 = vextq_u8::<13>(v1, v0); // ext v0.16b, v1.16b, v0.16b, #13
        }
    } else {
        // 1:
        v0 = vld1q_u8(x5); // ld1 {v0.16b}, [x5], #16
        x5 = x5.add(16);
        // !LR_HAVE_LEFT, fill v1 with the leftmost byte
        // and shift v0 to have 3x the first byte at the front.
        let v1 = vdupq_laneq_u8::<0>(v0); // dup v1.16b, v0.b[0]
                                          // Move x3 back to account for the last 3 bytes we loaded before,
                                          // which we shifted out.
        x5 = x5.sub(3); // sub x5, x5, #3
        v0 = vextq_u8::<13>(v1, v0); // ext v0.16b, v1.16b, v0.16b, #13
    }

    // 2:
    let mut v1 = vmull_u8(vget_low_u8(v0), vget_low_u8(v0)); // umull v1.8h, v0.8b, v0.8b
    let mut v2 = vmull_high_u8(v0, v0); // umull2 v2.8h, v0.16b, v0.16b

    // 4: Loop horizontally: one block of 8 sums; leaves the loop around it at `b.le 9f`.
    macro_rules! loop_horizontally {
        () => {
            let v16 = vextq_u8::<1>(v0, v0); // ext v16.16b, v0.16b, v0.16b, #1
            let v17 = vextq_u8::<2>(v0, v0); // ext v17.16b, v0.16b, v0.16b, #2
            let v19 = vextq_u8::<4>(v0, v0); // ext v19.16b, v0.16b, v0.16b, #4
            let v18 = vextq_u8::<3>(v0, v0); // ext v18.16b, v0.16b, v0.16b, #3
            let v3 = vaddl_u8(vget_low_u8(v16), vget_low_u8(v17)); // uaddl v3.8h, v16.8b, v17.8b
            let v24 = vaddl_u8(vget_low_u8(v0), vget_low_u8(v19)); // uaddl v24.8h, v0.8b, v19.8b
            let v3 = vaddw_u8(v3, vget_low_u8(v18)); // uaddw v3.8h, v3.8h, v18.8b

            let v16 = vextq_u16::<1>(v1, v2); // ext v16.16b, v1.16b, v2.16b, #2
            let v17 = vextq_u16::<2>(v1, v2); // ext v17.16b, v1.16b, v2.16b, #4
            let v19 = vextq_u16::<4>(v1, v2); // ext v19.16b, v1.16b, v2.16b, #8
            let v18 = vextq_u16::<3>(v1, v2); // ext v18.16b, v1.16b, v2.16b, #6

            vst1q_u16(x1, v3); // st1 {v3.8h}, [x1], #16
            x1 = x1.add(8);
            let v3 = vaddq_u16(v3, v24); // add v3.8h, v3.8h, v24.8h

            let v26 = vaddl_u16(vget_low_u16(v16), vget_low_u16(v17)); // uaddl v26.4s, v16.4h, v17.4h
            let v27 = vaddl_high_u16(v16, v17); // uaddl2 v27.4s, v16.8h, v17.8h
            let v16s = vaddl_u16(vget_low_u16(v1), vget_low_u16(v19)); // uaddl v16.4s, v1.4h, v19.4h
            let v17s = vaddl_high_u16(v1, v19); // uaddl2 v17.4s, v1.8h, v19.8h
            let v26 = vaddw_u16(v26, vget_low_u16(v18)); // uaddw v26.4s, v26.4s, v18.4h
            let v27 = vaddw_high_u16(v27, v18); // uaddw2 v27.4s, v27.4s, v18.8h

            vst1q_u32_x2(x0, uint32x4x2_t(v26, v27)); // st1 {v26.4s, v27.4s}, [x0], #32
            x0 = x0.add(8);
            let v26 = vaddq_u32(v26, v16s); // add v26.4s, v26.4s, v16.4s
            let v27 = vaddq_u32(v27, v17s); // add v27.4s, v27.4s, v17.4s

            w6 -= 8; // subs w6, w6, #8

            vst1q_u16(x3, v3); // st1 {v3.8h}, [x3], #16
            x3 = x3.add(8);
            vst1q_u32_x2(x2, uint32x4x2_t(v26, v27)); // st1 {v26.4s, v27.4s}, [x2], #32
            x2 = x2.add(8);

            if w6 <= 0 {
                break; // b.le 9f
            }
            // tst w7, #2 (LR_HAVE_RIGHT)
            let v3 = vld1_u8(x5); // ld1 {v3.8b}, [x5], #8
            x5 = x5.add(8);
            v1 = v2; // mov v1.16b, v2.16b
            v0 = vcombine_u8(vget_high_u8(v0), v3); // ext v0.16b, v0.16b, v3.16b, #8
            v2 = vmull_u8(v3, v3); // umull v2.8h, v3.8b, v3.8b
        };
    }

    if w7 & 2 != 0 {
        // tst w7, #2 (LR_HAVE_RIGHT); b.ne 4f
        loop {
            loop_horizontally!();
            // b.ne 4b (If we don't need to pad, just keep summing.)
        }
    } else {
        // If we'll need to pad the right edge, load that byte to pad with
        // here since we can find it pretty easily from here.
        let w13 = w6 - (2 + 16 - 3 + 1); // sub w13, w6, #(2 + 16 - 3 + 1)

        // Fill v30 with the right padding pixel
        // ldr b30, [x5, w13, sxtw]; dup v30.16b, v30.b[0]
        let v30 = vld1q_dup_u8(x5.offset(w13 as isize));
        loop {
            // 3: !LR_HAVE_RIGHT

            // Check whether we need to pad the right edge
            if w6 < 11 {
                // cmp w6, #11; b.ge 4f (If w >= 11, all used input pixels are valid)

                // 1 <= w < 11, w+1 pixels valid in v0. For w=9 or w=10,
                // this ends up called again; it's not strictly needed in those
                // cases (we pad enough here), but keeping the code as simple as possible.

                // Insert padding in v0.b[w+1] onwards; fuse the +1 into the
                // buffer pointer.
                let x13 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 1); // movrel x13, right_ext_mask, -1
                let x13 = x13.sub(w6 as u32 as usize); // sub x13, x13, w6, uxtw
                let v29 = vld1q_u8(x13); // ld1 {v29.16b}, [x13]

                v0 = vbslq_u8(v29, v30, v0); // bit v0.16b, v30.16b, v29.16b

                // Update the precalculated squares
                v1 = vmull_u8(vget_low_u8(v0), vget_low_u8(v0)); // umull v1.8h, v0.8b, v0.8b
                v2 = vmull_high_u8(v0, v0); // umull2 v2.8h, v0.16b, v0.16b
            }

            loop_horizontally!();
            // b 3b (If we need to pad, check how many pixels we have left.)
        }
    }
    // 9:
}

// `sgr_funcs 8`: the instances of looprestoration_tmpl.S.

/// `sgr_finish_filter1_2rows_8bpc_neon`: `void dav1d_sgr_finish_filter1_2rows_8bpc_neon(int16_t *tmp,
/// const pixel *src, const ptrdiff_t src_stride, int32_t **A_ptrs, int16_t **B_ptrs, const int w, const int h)`
///
/// # Safety
/// As for `sgr_finish_filter1_2rows` (`looprestoration_tmpl.rs`).
pub unsafe extern "C" fn dav1d_sgr_finish_filter1_2rows_8bpc_neon(
    tmp: *mut i16,
    src: *const u8,
    src_stride: isize,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    h: c_int,
) {
    sgr_finish_filter1_2rows::<8>(tmp, src, src_stride, a_ptrs, b_ptrs, w, h)
}

/// `sgr_finish_weighted1_8bpc_neon`: `void dav1d_sgr_finish_weighted1_8bpc_neon(pixel *dst, int32_t **A_ptrs,
/// int16_t **B_ptrs, const int w, const int w1 HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// As for `sgr_finish_weighted1` (`looprestoration_tmpl.rs`).
pub unsafe extern "C" fn dav1d_sgr_finish_weighted1_8bpc_neon(
    dst: *mut u8,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    w1: c_int,
) {
    sgr_finish_weighted1::<8>(dst, a_ptrs, b_ptrs, w, w1, 0)
}

/// `sgr_finish_filter2_2rows_8bpc_neon`: `void dav1d_sgr_finish_filter2_2rows_8bpc_neon(int16_t *tmp,
/// const pixel *src, const ptrdiff_t src_stride, int32_t **A_ptrs, int16_t **B_ptrs, const int w, const int h)`
///
/// # Safety
/// As for `sgr_finish_filter2_2rows` (`looprestoration_tmpl.rs`).
pub unsafe extern "C" fn dav1d_sgr_finish_filter2_2rows_8bpc_neon(
    tmp: *mut i16,
    src: *const u8,
    src_stride: isize,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    h: c_int,
) {
    sgr_finish_filter2_2rows::<8>(tmp, src, src_stride, a_ptrs, b_ptrs, w, h)
}

/// `sgr_finish_weighted2_8bpc_neon`: `void dav1d_sgr_finish_weighted2_8bpc_neon(pixel *dst,
/// const ptrdiff_t stride, int32_t **A_ptrs, int16_t **B_ptrs, const int w, const int h, const int w1
/// HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// As for `sgr_finish_weighted2` (`looprestoration_tmpl.rs`).
pub unsafe extern "C" fn dav1d_sgr_finish_weighted2_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    h: c_int,
    w1: c_int,
) {
    sgr_finish_weighted2::<8>(dst, stride, a_ptrs, b_ptrs, w, h, w1, 0)
}

/// `sgr_weighted2_8bpc_neon`: `void dav1d_sgr_weighted2_8bpc_neon(pixel *dst, const ptrdiff_t dst_stride,
/// const int16_t *t1, const int16_t *t2, const int w, const int h, const int16_t wt[2] HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// As for `sgr_weighted2` (`looprestoration_tmpl.rs`).
pub unsafe extern "C" fn dav1d_sgr_weighted2_8bpc_neon(
    dst: *mut u8,
    dst_stride: isize,
    t1: *const i16,
    t2: *const i16,
    w: c_int,
    h: c_int,
    wt: *const i16,
) {
    sgr_weighted2::<8>(dst, dst_stride, t1, t2, w, h, wt, 0)
}
