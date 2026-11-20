// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/looprestoration16.S` (16 bpc) with NEON intrinsics
//! (engine ADR-0044 D2, libavif-rs ADR 0062): the Wiener filters, the horizontal box sums of the
//! self-guided filter, and the file's instances of the template `looprestoration_tmpl.S`
//! (`sgr_funcs 16`; the generic functions are in `looprestoration_tmpl.rs`). The vertical box sums are
//! in `looprestoration_common.rs`, the C side of the self-guided filters and the slot assignments in
//! `simd/arm/looprestoration.rs`. The table `right_ext_mask_buf` is `RIGHT_EXT_MASK_BUF`.
//!
//! Stack buffers: the Wiener filters keep their rows of intermediate values in a local buffer, zero-initialized
//! (`[0; N]`); the self-guided kernels have none (their rows are the wrappers',
//! zeroed whole there).
//!
//! # The Wiener filters
//!
//! `wiener_filter7_16bpc_neon` and `wiener_filter5_16bpc_neon` are the entry points
//! `dav1d_wiener_filter{7,5}_16bpc_neon`; their internal functions `wiener_filter{7,5}_{h,v,hv}_16bpc_neon`
//! are the Rust functions of the same names.
//!
//! The assembly calls the internal functions with `bl` and a private register convention: the state stays
//! in registers, and a row function pushes what it changes and pops it at the end, the row pointers
//! shifted by one. Here the registers that live across those calls are the fields of `WienerRegs`, which
//! every row function takes by reference, and a row function leaves in it exactly what the assembly's
//! function leaves in the registers. The row functions are separate functions, as in the assembly (whether
//! they are inlined is the compiler's choice); each loads the coefficients and the constants derived from the bit depth it uses from
//! `WienerRegs`, where the assembly keeps them in registers across `bl`, and stores the shifted row
//! pointers at entry, where the assembly pushes them shifted and pops them at the end.
//!
//! The labels of the entry points (`L(main_7)`, `L(no_top_7)`, ...) are labeled blocks; the fall-through
//! chain `L(v3_7)` -> `L(v2_7)` -> `L(v1_7)` is a count of the vertical passes that remain. In the row
//! functions the numeric labels are comments; `3:` (the padding of the right edge) is the `if` at the
//! head of the loop, entered as the assembly enters it: without LR_HAVE_RIGHT, before every 16 pixels,
//! when fewer than 19 (5 taps: 18) are left.
//!
//! Single instructions in another form: a two-register `st1` with post-increment is two `vst1q` stores
//! (compiled to one `stp` with post-index; `vst1q_s16_x2` compiled to `st1` and a separate `add`); the
//! row loops address the source and destination by an element index (`i`) where the assembly post-increments
//! `x3` and `x0`; `ld1 {v4.d}[1], [x2]` is
//! a scalar load into lane 1 of a zero vector (the low half is not used).
//!
//! # The self-guided filter kernels
//!
//! The three row kernels (`sgr_box3_row_h_16bpc_neon`, `sgr_box5_row_h_16bpc_neon`,
//! `sgr_box35_row_h_16bpc_neon`) are one function each; the labels `0:`–`4:`,
//! `9:` are comments. General registers keep their assembly names; vector
//! registers are typed locals named after the registers (`v0`/`v1` the 16
//! pixels of the window, `v30` the padding pixel). The pixel pointer (`x3`,
//! `x5`) is a `*const u16`: the assembly's byte offsets on it are pixel
//! offsets here (`sub x3, x3, #4` is `x3.sub(2)`, `[x3, w13, sxtw #1]` is
//! `x3.offset(w13)`). A post-incremented address is the access followed by
//! the `add` of the elements accessed.
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
//! block than the assembly). `ldr h30, [x3, w13, sxtw #1]` +
//! `dup v30.8h, v30.h[0]` is the load-and-replicate `ld1r` (`vld1q_dup_u16`);
//! `ld1 {v2.d}[1], [x2]` of the `left` pixels, of which the `ext` that
//! follows reads only that half, is a load replicated to both halves
//! (`sgr_ld1_left_16bpc`), since the intrinsic for a lane load needs a
//! register to insert into. Neither of these two is in a loop.
//!
//! A detail of the assembly kept as it is: box5 and box35 take the padding
//! mask at `right_ext_mask - 1 - 2 * w` ("fuse the +1 into the buffer
//! pointer"), one byte and not one pixel before, so the mask also selects the
//! high byte of the pixel `v0.h[w]`. That pixel is the last valid one, which
//! is the padding pixel itself; the result is the same.
//!
//! Memory, for all three (`K` = 2 for box3, 3 for box5 and box35; `n` =
//! `ceil((w + 2) / 8)` blocks; pixels are `uint16_t`):
//! - written: `8 * n` elements of each `sum` row and `8 * n` elements of each
//!   `sumsq` row (at most 392 at the test's largest width 384; the wrappers'
//!   rows have 400).
//! - read: the pixels `src[-K..16 - K]` with LR_HAVE_LEFT and `left == NULL`,
//!   else `src[0..16]` and, with LR_HAVE_LEFT, `left[0][0..4]`; then 8 more
//!   pixels per further block, up to `src[8 * n + 7 - K]` (with one block the
//!   first load is all, up to `src[15]`). The sums use the pixels up to
//!   `src[w + K - 1]` with LR_HAVE_RIGHT and up to `src[w - 1]` without. The
//!   kernel therefore reads up to 15 pixels at and behind `src + w` (`16 - w`
//!   for `w <= 6`, else `8 * n + 8 - K - w`, which is 8 to 15 for box3 and 7
//!   to 14 for box5 and box35) — also without LR_HAVE_RIGHT, where it
//!   replaces what it loaded there by the padding pixel. This is the
//!   assembly's behavior, kept as it is; see the report.
//! - without LR_HAVE_RIGHT one more pixel: `src[w - 1]`, the padding pixel.
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
    x0: *mut u16,
    /// `x1`: `stride`, in bytes.
    x1: isize,
    /// `x2`: `left` (NULL for the `lpf` rows).
    x2: *const [u16; 4],
    /// `x3`: the source row (`lpf` or `p`).
    x3: *const u16,
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
    /// `v0`: the horizontal coefficients `filter[0][0..4]` in `h[0-3]`, the vertical coefficients
    /// `filter[1][0..4]` in `h[4-7]`.
    v0: int16x8_t,
    /// `v27`: `-round_bits_v`.
    v27: int32x4_t,
    /// `v28`: `bitdepth_max`.
    v28: uint16x8_t,
    /// `v29`: `-round_bits_h`.
    v29: int32x4_t,
    /// `v30`: `1 << (bitdepth + 6)`.
    v30: int32x4_t,
    /// `v31`: `1 << 13`.
    v31: uint16x8_t,
}

/// `wiener_filter7_16bpc_neon`: `void dav1d_wiener_filter7_16bpc_neon(pixel *p, const ptrdiff_t stride,
/// const pixel (*left)[4], const pixel *lpf, const int w, int h, const LooprestorationParams *const params,
/// const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// Reads `params.filter` (32 bytes). Per source row (the `h` rows of `p`, with LR_HAVE_TOP the two rows at
/// `lpf` and `lpf + stride`, with LR_HAVE_BOTTOM the two rows at `lpf + 6 * stride` and `lpf + 7 * stride`)
/// it reads the pixels `[-3, 16 * n + 5)` of the row with LR_HAVE_LEFT and `left == NULL` (the `lpf` rows),
/// else `[0, 16 * n + 5)`, where `n = (w + 15) / 16`; with LR_HAVE_LEFT, 4 pixels of `left` per row of `p`.
/// Writes the pixels `[0, 16 * n)` of each of the `h` rows of `p`. The intermediate rows are in the local,
/// zero-initialized `WienerRows`.
pub unsafe extern "C" fn dav1d_wiener_filter7_16bpc_neon(
    p: *mut u16,
    stride: isize,
    left: *const [u16; 4],
    lpf: *const u16,
    w: c_int,
    h: c_int,
    params: *const LooprestorationParams,
    edges: c_uint,
    bitdepth_max: c_int,
) {
    let x6 = params as *const i16;
    let w8 = bitdepth_max as u32; // ldr w8, [sp]
    let v0_1 = vld1q_s16_x2(x6); // ld1 {v0.8h, v1.8h}, [x6]

    let v28 = vdupq_n_u16(w8 as u16); // dup v28.8h, w8 // bitdepth_max
    let w8 = w8.leading_zeros() as i32; // clz w8, w8
    let v30 = vdupq_n_u32(1); // movi v30.4s, #1
    let w10 = w8 - 38; // -(bitdepth + 6)
    let w11 = w8 - 11; // round_bits_v
    let w8 = w8 - 25; // -round_bits_h
    let w10 = -w10; // bitdepth + 6
    let w11 = -w11; // -round_bits_v
    let v2 = vdupq_n_s32(w10);
    let v29 = vdupq_n_s32(w8); // -round_bits_h
    let v27 = vdupq_n_s32(w11); // -round_bits_v
    let v31 = vdupq_n_u16(0x20 << 8); // movi v31.8h, #0x20, lsl #8 // 1 << 13 = 8192
    let v30 = vreinterpretq_s32_u32(vshlq_u32(v30, v2)); // ushl v30.4s, v30.4s, v2.4s // 1 << (bitdepth + 6)

    // zip1 v0.2d, v0.2d, v1.2d // move vertical coeffs to v0.h[4-7], freeing up v1
    let v0 = vreinterpretq_s16_s64(vzip1q_s64(vreinterpretq_s64_s16(v0_1.0), vreinterpretq_s64_s16(v0_1.1)));
    let mut w5 = h;

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
        v0,
        v27,
        v28,
        v29,
        v30,
        v31,
    };
    let x16: *const u16;

    // The number of vertical passes that remain: 3 for L(v3_7), 2 for L(v2_7), 1 for L(v1_7).
    let v_passes = 'v: {
        'main_7: {
            // tst w7, #4 // LR_HAVE_TOP; b.eq L(no_top_7)
            if r.w7 & 4 != 0 {
                let x16_left = r.x2; // mov x16, x2 // backup left
                r.x2 = core::ptr::null(); // mov x2, #0
                wiener_filter7_h_16bpc_neon(&mut r);
                r.x3 = r.x3.byte_offset(r.x1); // lpf += stride
                r.x9 = r.x14; // t6
                r.x10 = r.x14; // t5
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_16bpc_neon(&mut r);
                r.x3 = r.x3.wrapping_byte_offset(r.x1 * 4);
                r.x3 = r.x3.wrapping_byte_offset(r.x1); // lpf += stride*5
                r.x11 = r.x14; // t4
                r.x14 = r.x14.add(384); // t1 += 384*2
                r.x2 = x16_left; // left
                x16 = r.x3; // backup lpf
                r.x3 = r.x0; // lpf = p
                wiener_filter7_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x12 = r.x14; // t3
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_7)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_16bpc_neon(&mut r);
                r.x13 = r.x14; // t2
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_7)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 3; // b.eq L(v3_7)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
            } else {
                // L(no_top_7):
                r.x3 = r.x3.wrapping_byte_offset(r.x1 * 4);
                x16 = r.x3.wrapping_byte_offset(r.x1 * 2); // lpf += stride*6, backup
                r.x3 = r.x0; // lpf = p

                wiener_filter7_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x9 = r.x14; // t6
                r.x10 = r.x14; // t5
                r.x11 = r.x14; // t4
                r.x12 = r.x14; // t3
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_7)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += p_stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_7)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += p_stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter7_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 3; // b.eq L(v3_7)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += p_stride
                r.x15 = r.x14.add(384); // t0 = t1 + 384*2
                wiener_filter7_hv_16bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 3; // b.eq L(v3_7)
                }
                r.x15 = r.x15.add(384 * 4); // t0 += 384*2*4
                wiener_filter7_hv_16bpc_neon(&mut r);
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
            wiener_filter7_hv_16bpc_neon(&mut r);
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
        wiener_filter7_hv_16bpc_neon(&mut r);
        wiener_filter7_hv_16bpc_neon(&mut r);
        1 // L(v1_7)
    };
    if v_passes >= 3 {
        // L(v3_7):
        wiener_filter7_v_16bpc_neon(&mut r);
    }
    if v_passes >= 2 {
        // L(v2_7):
        wiener_filter7_v_16bpc_neon(&mut r);
    }
    // L(v1_7):
    wiener_filter7_v_16bpc_neon(&mut r);
}

/// `wiener_filter7_h_16bpc_neon`: filters the source row `x3` horizontally into the row `x14`.
/// Leaves `x3`, `w4` and `x14` unchanged (the assembly saves and restores them); advances `x2` by one
/// row of `left` if it is not NULL.
unsafe fn wiener_filter7_h_16bpc_neon(r: &mut WienerRegs) {
    // stp x3, x4, [sp, #-32]!; str x14, [sp, #16]
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    let x14 = r.x14;
    let (v0, v29, v30, v31) = (r.v0, r.v29, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let (mut v2, mut v3);
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(3); // sub x3, x3, #6
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            v2 = v2_3.0;
            v3 = v2_3.1;
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            // ld1 {v4.d}[1], [x2], #8 (the low half of v4 is not used)
            let v4 = vsetq_lane_u64::<1>(read_unaligned(r.x2 as *const u64), vdupq_n_u64(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 3 pixels we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(3); // sub x3, x3, #6
            v3 = vextq_s16::<5>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #10
            v2 = vextq_s16::<5>(vreinterpretq_s16_u64(v4), v2_3.0); // ext v2.16b, v4.16b, v2.16b, #10
        }
    } else {
        // 1:
        let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v4 with the leftmost pixel
        // and shift v3 to have 3x the first pixel at the front.
        let v4 = vdupq_laneq_s16::<0>(v2_3.0);
        // Move x3 back to account for the last 3 pixels we loaded before,
        // which we shifted out.
        x3 = x3.sub(3); // sub x3, x3, #6
        v3 = vextq_s16::<5>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #10
        v2 = vextq_s16::<5>(v4, v2_3.0); // ext v2.16b, v4.16b, v2.16b, #10
    }

    // 2:
    let mut v4 = vld1q_s16(x3 as *const i16); // ld1 {v4.8h}, [x3], #16
    x3 = x3.add(8);

    // The element index of the 16-pixel block (`x3` and `x0` post-increment in the assembly).
    let mut i = 0;
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
            let h26 = read_unaligned(x3.add(i).offset(w17 as isize)); // ldr h26, [x3, w17, sxtw #1]
            let v26 = vreinterpretq_u8_u16(vdupq_n_u16(h26)); // dup v26.8h, v26.h[0]
            let v23_25 = vld1q_u8_x3(x6); // ld1 {v23.16b, v24.16b, v25.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v23_25.0, v26, vreinterpretq_u8_s16(v2))); // bit v2.16b, v26.16b, v23.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v23_25.1, v26, vreinterpretq_u8_s16(v3))); // bit v3.16b, v26.16b, v24.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v23_25.2, v26, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v26.16b, v25.16b
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
        let v6 = vmull_laneq_s16::<3>(vget_low_s16(v18), v0); // smull v6.4s, v18.4h, v0.h[3]
        let v6 = vmlal_laneq_s16::<2>(v6, vget_low_s16(v19), v0); // smlal v6.4s, v19.4h, v0.h[2]
        let v6 = vmlal_laneq_s16::<1>(v6, vget_low_s16(v20), v0); // smlal v6.4s, v20.4h, v0.h[1]
        let v6 = vmlal_laneq_s16::<0>(v6, vget_low_s16(v21), v0); // smlal v6.4s, v21.4h, v0.h[0]
        let v7 = vmull_high_laneq_s16::<3>(v18, v0); // smull2 v7.4s, v18.8h, v0.h[3]
        let v7 = vmlal_high_laneq_s16::<2>(v7, v19, v0); // smlal2 v7.4s, v19.8h, v0.h[2]
        let v7 = vmlal_high_laneq_s16::<1>(v7, v20, v0); // smlal2 v7.4s, v20.8h, v0.h[1]
        let v7 = vmlal_high_laneq_s16::<0>(v7, v21, v0); // smlal2 v7.4s, v21.8h, v0.h[0]

        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v20 = vextq_s16::<5>(v3, v4); // ext v20.16b, v3.16b, v4.16b, #10
        let v21 = vextq_s16::<6>(v3, v4); // ext v21.16b, v3.16b, v4.16b, #12
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6

        let v19 = vaddq_s16(v19, v17);
        let v20 = vaddq_s16(v20, v16);
        let v21 = vaddq_s16(v21, v3);
        let v16 = vmull_laneq_s16::<3>(vget_low_s16(v18), v0); // smull v16.4s, v18.4h, v0.h[3]
        let v16 = vmlal_laneq_s16::<2>(v16, vget_low_s16(v19), v0); // smlal v16.4s, v19.4h, v0.h[2]
        let v16 = vmlal_laneq_s16::<1>(v16, vget_low_s16(v20), v0); // smlal v16.4s, v20.4h, v0.h[1]
        let v16 = vmlal_laneq_s16::<0>(v16, vget_low_s16(v21), v0); // smlal v16.4s, v21.4h, v0.h[0]
        let v17 = vmull_high_laneq_s16::<3>(v18, v0); // smull2 v17.4s, v18.8h, v0.h[3]
        let v17 = vmlal_high_laneq_s16::<2>(v17, v19, v0); // smlal2 v17.4s, v19.8h, v0.h[2]
        let v17 = vmlal_high_laneq_s16::<1>(v17, v20, v0); // smlal2 v17.4s, v20.8h, v0.h[1]
        let v17 = vmlal_high_laneq_s16::<0>(v17, v21, v0); // smlal2 v17.4s, v21.8h, v0.h[0]

        let v24 = vdupq_n_u16(0x7fff); // mvni v24.8h, #0x80, lsl #8 // 0x7fff = (1 << 15) - 1
        let v6 = vaddq_s32(v6, v30);
        let v7 = vaddq_s32(v7, v30);
        let v16 = vaddq_s32(v16, v30);
        let v17 = vaddq_s32(v17, v30);
        let v6 = vrshlq_s32(v6, v29); // srshl v6.4s, v6.4s, v29.4s
        let v7 = vrshlq_s32(v7, v29); // srshl v7.4s, v7.4s, v29.4s
        let v16 = vrshlq_s32(v16, v29); // srshl v16.4s, v16.4s, v29.4s
        let v17 = vrshlq_s32(v17, v29); // srshl v17.4s, v17.4s, v29.4s
        let v6 = vqmovun_s32(v6); // sqxtun v6.4h, v6.4s
        let v6 = vqmovun_high_s32(v6, v7); // sqxtun2 v6.8h, v7.4s
        let v7 = vqmovun_s32(v16); // sqxtun v7.4h, v16.4s
        let v7 = vqmovun_high_s32(v7, v17); // sqxtun2 v7.8h, v17.4s
        let v6 = vminq_u16(v6, v24);
        let v7 = vminq_u16(v7, v24);
        let v6 = vreinterpretq_s16_u16(vsubq_u16(v6, v31));
        let v7 = vreinterpretq_s16_u16(vsubq_u16(v7, v31));

        w4 -= 16; // subs w4, w4, #16

        vst1q_s16(x14.add(i), v6); // st1 {v6.8h, v7.8h}, [x14], #32
        vst1q_s16(x14.add(i + 8), v7);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        let v3_4 = vld1q_s16_x2(x3.add(i) as *const i16); // ld1 {v3.8h, v4.8h}, [x3], #32
        i += 16;
        v3 = v3_4.0;
        v4 = v3_4.1;
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }
    // 0:
    // ldr x14, [sp, #16]; ldp x3, x4, [sp], #32
}

/// `wiener_filter7_v_16bpc_neon`: filters the rows `x9`-`x14` (t6-t1, with t1 standing in for t0)
/// vertically into the output row `x0`. Leaves the row pointers shifted (`x9` gets the value of `x10`,
/// etc; `x14` stays) and `x0` advanced by `stride`.
unsafe fn wiener_filter7_v_16bpc_neon(r: &mut WienerRegs) {
    // Backing up/restoring registers shifted, so that x9 gets the value
    // of x10, etc, afterwards.
    // stp x10, x11, [sp, #-64]!; stp x12, x13, [sp, #16]; stp x14, x14, [sp, #32]; stp x0, x4, [sp, #48]
    let (x9, x10, x11, x12, x13, x14) = (r.x9, r.x10, r.x11, r.x12, r.x13, r.x14);
    let x0 = r.x0;
    let mut w4 = r.w4;
    // The values the `ldp`s at the end leave in x9-x13 (x14 stays).
    r.x9 = x10;
    r.x10 = x11;
    r.x11 = x12;
    r.x12 = x13;
    r.x13 = x14;
    let (v0, v27, v28) = (r.v0, r.v27, r.v28);
    // The element index of the 16-pixel block (`x3` and `x0` post-increment in the assembly).
    let mut i = 0;
    loop {
        // 1:
        let v16_17 = vld1q_s16_x2(x9.add(i)); // ld1 {v16.8h, v17.8h}, [x9], #32
        let v18_19 = vld1q_s16_x2(x10.add(i)); // ld1 {v18.8h, v19.8h}, [x10], #32
        let v20_21 = vld1q_s16_x2(x11.add(i)); // ld1 {v20.8h, v21.8h}, [x11], #32
        let v22_23 = vld1q_s16_x2(x12.add(i)); // ld1 {v22.8h, v23.8h}, [x12], #32
        let v24_25 = vld1q_s16_x2(x13.add(i)); // ld1 {v24.8h, v25.8h}, [x13], #32
        let v6_7 = vld1q_s16_x2(x14.add(i)); // ld1 {v6.8h, v7.8h}, [x14], #32
        let (v16, v17) = (v16_17.0, v16_17.1);
        let (v18, v19) = (v18_19.0, v18_19.1);
        let (v20, v21) = (v20_21.0, v20_21.1);
        let (v22, v23) = (v22_23.0, v22_23.1);
        let (v24, v25) = (v24_25.0, v24_25.1);
        let (v6, v7) = (v6_7.0, v6_7.1);

        let v2 = vmull_laneq_s16::<4>(vget_low_s16(v16), v0); // smull v2.4s, v16.4h, v0.h[4]
        let v2 = vmlal_laneq_s16::<5>(v2, vget_low_s16(v18), v0); // smlal v2.4s, v18.4h, v0.h[5]
        let v2 = vmlal_laneq_s16::<6>(v2, vget_low_s16(v20), v0); // smlal v2.4s, v20.4h, v0.h[6]
        let v2 = vmlal_laneq_s16::<7>(v2, vget_low_s16(v22), v0); // smlal v2.4s, v22.4h, v0.h[7]
        let v2 = vmlal_laneq_s16::<6>(v2, vget_low_s16(v24), v0); // smlal v2.4s, v24.4h, v0.h[6]
        let v2 = vmlal_laneq_s16::<5>(v2, vget_low_s16(v6), v0); // smlal v2.4s, v6.4h, v0.h[5]
        let v2 = vmlal_laneq_s16::<4>(v2, vget_low_s16(v6), v0); // smlal v2.4s, v6.4h, v0.h[4]
        let v3 = vmull_high_laneq_s16::<4>(v16, v0); // smull2 v3.4s, v16.8h, v0.h[4]
        let v3 = vmlal_high_laneq_s16::<5>(v3, v18, v0); // smlal2 v3.4s, v18.8h, v0.h[5]
        let v3 = vmlal_high_laneq_s16::<6>(v3, v20, v0); // smlal2 v3.4s, v20.8h, v0.h[6]
        let v3 = vmlal_high_laneq_s16::<7>(v3, v22, v0); // smlal2 v3.4s, v22.8h, v0.h[7]
        let v3 = vmlal_high_laneq_s16::<6>(v3, v24, v0); // smlal2 v3.4s, v24.8h, v0.h[6]
        let v3 = vmlal_high_laneq_s16::<5>(v3, v6, v0); // smlal2 v3.4s, v6.8h, v0.h[5]
        let v3 = vmlal_high_laneq_s16::<4>(v3, v6, v0); // smlal2 v3.4s, v6.8h, v0.h[4]
        let v4 = vmull_laneq_s16::<4>(vget_low_s16(v17), v0); // smull v4.4s, v17.4h, v0.h[4]
        let v4 = vmlal_laneq_s16::<5>(v4, vget_low_s16(v19), v0); // smlal v4.4s, v19.4h, v0.h[5]
        let v4 = vmlal_laneq_s16::<6>(v4, vget_low_s16(v21), v0); // smlal v4.4s, v21.4h, v0.h[6]
        let v4 = vmlal_laneq_s16::<7>(v4, vget_low_s16(v23), v0); // smlal v4.4s, v23.4h, v0.h[7]
        let v4 = vmlal_laneq_s16::<6>(v4, vget_low_s16(v25), v0); // smlal v4.4s, v25.4h, v0.h[6]
        let v4 = vmlal_laneq_s16::<5>(v4, vget_low_s16(v7), v0); // smlal v4.4s, v7.4h, v0.h[5]
        let v4 = vmlal_laneq_s16::<4>(v4, vget_low_s16(v7), v0); // smlal v4.4s, v7.4h, v0.h[4]
        let v5 = vmull_high_laneq_s16::<4>(v17, v0); // smull2 v5.4s, v17.8h, v0.h[4]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v19, v0); // smlal2 v5.4s, v19.8h, v0.h[5]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v21, v0); // smlal2 v5.4s, v21.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<7>(v5, v23, v0); // smlal2 v5.4s, v23.8h, v0.h[7]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v25, v0); // smlal2 v5.4s, v25.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v7, v0); // smlal2 v5.4s, v7.8h, v0.h[5]
        let v5 = vmlal_high_laneq_s16::<4>(v5, v7, v0); // smlal2 v5.4s, v7.8h, v0.h[4]
        let v2 = vrshlq_s32(v2, v27); // srshl v2.4s, v2.4s, v27.4s // -round_bits_v
        let v3 = vrshlq_s32(v3, v27); // srshl v3.4s, v3.4s, v27.4s
        let v4 = vrshlq_s32(v4, v27); // srshl v4.4s, v4.4s, v27.4s
        let v5 = vrshlq_s32(v5, v27); // srshl v5.4s, v5.4s, v27.4s
        let v2 = vqmovun_s32(v2); // sqxtun v2.4h, v2.4s
        let v2 = vqmovun_high_s32(v2, v3); // sqxtun2 v2.8h, v3.4s
        let v3 = vqmovun_s32(v4); // sqxtun v3.4h, v4.4s
        let v3 = vqmovun_high_s32(v3, v5); // sqxtun2 v3.8h, v5.4s
        let v2 = vminq_u16(v2, v28); // umin v2.8h, v2.8h, v28.8h // bitdepth_max
        let v3 = vminq_u16(v3, v28); // umin v3.8h, v3.8h, v28.8h
        w4 -= 16; // subs w4, w4, #16
        vst1q_u16(x0.add(i), v2); // st1 {v2.8h, v3.8h}, [x0], #32
        vst1q_u16(x0.add(i + 8), v3);
        i += 16;
        if w4 <= 0 {
            break; // b.gt 1b
        }
    }

    // ldp x0, x4, [sp, #48]; ldp x13, x14, [sp, #32]; ldp x11, x12, [sp, #16]; ldp x9, x10, [sp], #64
    // (`r` has the shifted x9-x13 since the entry.)

    r.x0 = r.x0.wrapping_byte_offset(r.x1); // add x0, x0, x1
}

/// `wiener_filter7_hv_16bpc_neon`: filters the source row `x3` horizontally into the row `x15` (t0) and
/// the rows `x9`-`x14` with it vertically into the output row `x0`. Leaves the row pointers shifted
/// (`x9` gets the value of `x10`, etc, `x14` that of `x15`, and `x15` the former `x10`), `x3` and `x0`
/// advanced by `stride`, and `x2` advanced by one row of `left` if it is not NULL.
unsafe fn wiener_filter7_hv_16bpc_neon(r: &mut WienerRegs) {
    // Backing up/restoring registers shifted, so that x9 gets the value
    // of x10, etc, and x15==x9, afterwards.
    // stp x10, x11, [sp, #-80]!; stp x12, x13, [sp, #16]; stp x14, x15, [sp, #32]; stp x10, x0, [sp, #48]
    // stp x3, x4, [sp, #64]
    let (x9, x10, x11, x12, x13, x14, x15) = (r.x9, r.x10, r.x11, r.x12, r.x13, r.x14, r.x15);
    let x0 = r.x0;
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
    let (v0, v27, v28, v29, v30, v31) = (r.v0, r.v27, r.v28, r.v29, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let (mut v2, mut v3);
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(3); // sub x3, x3, #6
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            v2 = v2_3.0;
            v3 = v2_3.1;
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            // ld1 {v4.d}[1], [x2], #8 (the low half of v4 is not used)
            let v4 = vsetq_lane_u64::<1>(read_unaligned(r.x2 as *const u64), vdupq_n_u64(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 3 pixels we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(3); // sub x3, x3, #6
            v3 = vextq_s16::<5>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #10
            v2 = vextq_s16::<5>(vreinterpretq_s16_u64(v4), v2_3.0); // ext v2.16b, v4.16b, v2.16b, #10
        }
    } else {
        // 1:
        let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v4 with the leftmost pixel
        // and shift v3 to have 3x the first pixel at the front.
        let v4 = vdupq_laneq_s16::<0>(v2_3.0);
        // Move x3 back to account for the last 3 pixels we loaded before,
        // which we shifted out.
        x3 = x3.sub(3); // sub x3, x3, #6
        v3 = vextq_s16::<5>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #10
        v2 = vextq_s16::<5>(v4, v2_3.0); // ext v2.16b, v4.16b, v2.16b, #10
    }

    // 2:
    let mut v4 = vld1q_s16(x3 as *const i16); // ld1 {v4.8h}, [x3], #16
    x3 = x3.add(8);

    // The element index of the 16-pixel block (`x3` and `x0` post-increment in the assembly).
    let mut i = 0;
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
            let h26 = read_unaligned(x3.add(i).offset(w17 as isize)); // ldr h26, [x3, w17, sxtw #1]
            let v26 = vreinterpretq_u8_u16(vdupq_n_u16(h26)); // dup v26.8h, v26.h[0]
            let v23_25 = vld1q_u8_x3(x6); // ld1 {v23.16b, v24.16b, v25.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v23_25.0, v26, vreinterpretq_u8_s16(v2))); // bit v2.16b, v26.16b, v23.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v23_25.1, v26, vreinterpretq_u8_s16(v3))); // bit v3.16b, v26.16b, v24.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v23_25.2, v26, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v26.16b, v25.16b
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
        let v6 = vmull_laneq_s16::<3>(vget_low_s16(v18), v0); // smull v6.4s, v18.4h, v0.h[3]
        let v6 = vmlal_laneq_s16::<2>(v6, vget_low_s16(v19), v0); // smlal v6.4s, v19.4h, v0.h[2]
        let v6 = vmlal_laneq_s16::<1>(v6, vget_low_s16(v20), v0); // smlal v6.4s, v20.4h, v0.h[1]
        let v6 = vmlal_laneq_s16::<0>(v6, vget_low_s16(v21), v0); // smlal v6.4s, v21.4h, v0.h[0]
        let v7 = vmull_high_laneq_s16::<3>(v18, v0); // smull2 v7.4s, v18.8h, v0.h[3]
        let v7 = vmlal_high_laneq_s16::<2>(v7, v19, v0); // smlal2 v7.4s, v19.8h, v0.h[2]
        let v7 = vmlal_high_laneq_s16::<1>(v7, v20, v0); // smlal2 v7.4s, v20.8h, v0.h[1]
        let v7 = vmlal_high_laneq_s16::<0>(v7, v21, v0); // smlal2 v7.4s, v21.8h, v0.h[0]

        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v20 = vextq_s16::<5>(v3, v4); // ext v20.16b, v3.16b, v4.16b, #10
        let v21 = vextq_s16::<6>(v3, v4); // ext v21.16b, v3.16b, v4.16b, #12
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6

        let v19 = vaddq_s16(v19, v17);
        let v20 = vaddq_s16(v20, v16);
        let v21 = vaddq_s16(v21, v3);
        let v24 = vmull_laneq_s16::<3>(vget_low_s16(v18), v0); // smull v24.4s, v18.4h, v0.h[3]
        let v24 = vmlal_laneq_s16::<2>(v24, vget_low_s16(v19), v0); // smlal v24.4s, v19.4h, v0.h[2]
        let v24 = vmlal_laneq_s16::<1>(v24, vget_low_s16(v20), v0); // smlal v24.4s, v20.4h, v0.h[1]
        let v24 = vmlal_laneq_s16::<0>(v24, vget_low_s16(v21), v0); // smlal v24.4s, v21.4h, v0.h[0]
        let v25 = vmull_high_laneq_s16::<3>(v18, v0); // smull2 v25.4s, v18.8h, v0.h[3]
        let v25 = vmlal_high_laneq_s16::<2>(v25, v19, v0); // smlal2 v25.4s, v19.8h, v0.h[2]
        let v25 = vmlal_high_laneq_s16::<1>(v25, v20, v0); // smlal2 v25.4s, v20.8h, v0.h[1]
        let v25 = vmlal_high_laneq_s16::<0>(v25, v21, v0); // smlal2 v25.4s, v21.8h, v0.h[0]

        let v16_17 = vld1q_s16_x2(x9.add(i)); // ld1 {v16.8h, v17.8h}, [x9], #32

        let v26 = vdupq_n_u16(0x7fff); // mvni v26.8h, #0x80, lsl #8 // 0x7fff = (1 << 15) - 1
        let v6 = vaddq_s32(v6, v30);
        let v7 = vaddq_s32(v7, v30);
        let v24 = vaddq_s32(v24, v30);
        let v25 = vaddq_s32(v25, v30);
        let v18_19 = vld1q_s16_x2(x10.add(i)); // ld1 {v18.8h, v19.8h}, [x10], #32
        let v6 = vrshlq_s32(v6, v29); // srshl v6.4s, v6.4s, v29.4s
        let v7 = vrshlq_s32(v7, v29); // srshl v7.4s, v7.4s, v29.4s
        let v24 = vrshlq_s32(v24, v29); // srshl v24.4s, v24.4s, v29.4s
        let v25 = vrshlq_s32(v25, v29); // srshl v25.4s, v25.4s, v29.4s
        let v20_21 = vld1q_s16_x2(x11.add(i)); // ld1 {v20.8h, v21.8h}, [x11], #32
        let v6 = vqmovun_s32(v6); // sqxtun v6.4h, v6.4s
        let v6 = vqmovun_high_s32(v6, v7); // sqxtun2 v6.8h, v7.4s
        let v7 = vqmovun_s32(v24); // sqxtun v7.4h, v24.4s
        let v7 = vqmovun_high_s32(v7, v25); // sqxtun2 v7.8h, v25.4s
        let v22_23 = vld1q_s16_x2(x12.add(i)); // ld1 {v22.8h, v23.8h}, [x12], #32
        let v6 = vminq_u16(v6, v26);
        let v7 = vminq_u16(v7, v26);
        let v24_25 = vld1q_s16_x2(x13.add(i)); // ld1 {v24.8h, v25.8h}, [x13], #32
        let v6 = vreinterpretq_s16_u16(vsubq_u16(v6, v31));
        let v7 = vreinterpretq_s16_u16(vsubq_u16(v7, v31));

        let v8_9 = vld1q_s16_x2(x14.add(i)); // ld1 {v8.8h, v9.8h}, [x14], #32
        let (v16, v17) = (v16_17.0, v16_17.1);
        let (v18, v19) = (v18_19.0, v18_19.1);
        let (v20, v21) = (v20_21.0, v20_21.1);
        let (v22, v23) = (v22_23.0, v22_23.1);
        let (v24, v25) = (v24_25.0, v24_25.1);
        let (v8, v9) = (v8_9.0, v8_9.1);

        let v1 = vmull_laneq_s16::<4>(vget_low_s16(v16), v0); // smull v1.4s, v16.4h, v0.h[4]
        let v1 = vmlal_laneq_s16::<5>(v1, vget_low_s16(v18), v0); // smlal v1.4s, v18.4h, v0.h[5]
        let v1 = vmlal_laneq_s16::<6>(v1, vget_low_s16(v20), v0); // smlal v1.4s, v20.4h, v0.h[6]
        let v1 = vmlal_laneq_s16::<7>(v1, vget_low_s16(v22), v0); // smlal v1.4s, v22.4h, v0.h[7]
        let v1 = vmlal_laneq_s16::<6>(v1, vget_low_s16(v24), v0); // smlal v1.4s, v24.4h, v0.h[6]
        let v1 = vmlal_laneq_s16::<5>(v1, vget_low_s16(v8), v0); // smlal v1.4s, v8.4h, v0.h[5]
        let v1 = vmlal_laneq_s16::<4>(v1, vget_low_s16(v6), v0); // smlal v1.4s, v6.4h, v0.h[4]
        let v5 = vmull_high_laneq_s16::<4>(v16, v0); // smull2 v5.4s, v16.8h, v0.h[4]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v18, v0); // smlal2 v5.4s, v18.8h, v0.h[5]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v20, v0); // smlal2 v5.4s, v20.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<7>(v5, v22, v0); // smlal2 v5.4s, v22.8h, v0.h[7]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v24, v0); // smlal2 v5.4s, v24.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v8, v0); // smlal2 v5.4s, v8.8h, v0.h[5]
        let v5 = vmlal_high_laneq_s16::<4>(v5, v6, v0); // smlal2 v5.4s, v6.8h, v0.h[4]
        let v26 = vmull_laneq_s16::<4>(vget_low_s16(v17), v0); // smull v26.4s, v17.4h, v0.h[4]
        let v26 = vmlal_laneq_s16::<5>(v26, vget_low_s16(v19), v0); // smlal v26.4s, v19.4h, v0.h[5]
        let v26 = vmlal_laneq_s16::<6>(v26, vget_low_s16(v21), v0); // smlal v26.4s, v21.4h, v0.h[6]
        let v26 = vmlal_laneq_s16::<7>(v26, vget_low_s16(v23), v0); // smlal v26.4s, v23.4h, v0.h[7]
        let v26 = vmlal_laneq_s16::<6>(v26, vget_low_s16(v25), v0); // smlal v26.4s, v25.4h, v0.h[6]
        let v26 = vmlal_laneq_s16::<5>(v26, vget_low_s16(v9), v0); // smlal v26.4s, v9.4h, v0.h[5]
        let v26 = vmlal_laneq_s16::<4>(v26, vget_low_s16(v7), v0); // smlal v26.4s, v7.4h, v0.h[4]
        let v16 = vmull_high_laneq_s16::<4>(v17, v0); // smull2 v16.4s, v17.8h, v0.h[4]
        let v16 = vmlal_high_laneq_s16::<5>(v16, v19, v0); // smlal2 v16.4s, v19.8h, v0.h[5]
        let v16 = vmlal_high_laneq_s16::<6>(v16, v21, v0); // smlal2 v16.4s, v21.8h, v0.h[6]
        let v16 = vmlal_high_laneq_s16::<7>(v16, v23, v0); // smlal2 v16.4s, v23.8h, v0.h[7]
        let v16 = vmlal_high_laneq_s16::<6>(v16, v25, v0); // smlal2 v16.4s, v25.8h, v0.h[6]
        let v16 = vmlal_high_laneq_s16::<5>(v16, v9, v0); // smlal2 v16.4s, v9.8h, v0.h[5]
        let v16 = vmlal_high_laneq_s16::<4>(v16, v7, v0); // smlal2 v16.4s, v7.8h, v0.h[4]
        let v1 = vrshlq_s32(v1, v27); // srshl v1.4s, v1.4s, v27.4s // -round_bits_v
        let v5 = vrshlq_s32(v5, v27); // srshl v5.4s, v5.4s, v27.4s
        let v26 = vrshlq_s32(v26, v27); // srshl v26.4s, v26.4s, v27.4s
        let v16 = vrshlq_s32(v16, v27); // srshl v16.4s, v16.4s, v27.4s
        let v18 = vqmovun_s32(v1); // sqxtun v18.4h, v1.4s
        let v18 = vqmovun_high_s32(v18, v5); // sqxtun2 v18.8h, v5.4s
        let v19 = vqmovun_s32(v26); // sqxtun v19.4h, v26.4s
        let v19 = vqmovun_high_s32(v19, v16); // sqxtun2 v19.8h, v16.4s
        vst1q_s16(x15.add(i), v6); // st1 {v6.8h, v7.8h}, [x15], #32
        vst1q_s16(x15.add(i + 8), v7);
        let v18 = vminq_u16(v18, v28); // umin v18.8h, v18.8h, v28.8h // bitdepth_max
        let v19 = vminq_u16(v19, v28); // umin v19.8h, v19.8h, v28.8h
        w4 -= 16; // subs w4, w4, #16

        vst1q_u16(x0.add(i), v18); // st1 {v18.8h, v19.8h}, [x0], #32
        vst1q_u16(x0.add(i + 8), v19);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        let v3_4 = vld1q_s16_x2(x3.add(i) as *const i16); // ld1 {v3.8h, v4.8h}, [x3], #32
        i += 16;
        v3 = v3_4.0;
        v4 = v3_4.1;
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }

    // 0:
    // ldp x3, x4, [sp, #64]; ldp x15, x0, [sp, #48]; ldp x13, x14, [sp, #32]; ldp x11, x12, [sp, #16]
    // ldp x9, x10, [sp], #80
    // (`r` has the shifted x9-x15 since the entry.)

    r.x3 = r.x3.wrapping_byte_offset(r.x1); // add x3, x3, x1
    r.x0 = r.x0.wrapping_byte_offset(r.x1); // add x0, x0, x1
}

/// `wiener_filter5_16bpc_neon`: `void dav1d_wiener_filter5_16bpc_neon(pixel *p, const ptrdiff_t stride,
/// const pixel (*left)[4], const pixel *lpf, const int w, int h, const LooprestorationParams *const params,
/// const enum LrEdgeFlags edges HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// Reads `params.filter` (32 bytes). Per source row (the `h` rows of `p`, with LR_HAVE_TOP the two rows at
/// `lpf` and `lpf + stride`, with LR_HAVE_BOTTOM the two rows at `lpf + 6 * stride` and `lpf + 7 * stride`)
/// it reads the pixels `[-2, 16 * n + 6)` of the row with LR_HAVE_LEFT and `left == NULL` (the `lpf` rows),
/// else `[0, 16 * n + 6)`, where `n = (w + 15) / 16`; with LR_HAVE_LEFT, 4 pixels of `left` per row of `p`.
/// Writes the pixels `[0, 16 * n)` of each of the `h` rows of `p`. The intermediate rows are in the local,
/// zero-initialized `WienerRows`.
pub unsafe extern "C" fn dav1d_wiener_filter5_16bpc_neon(
    p: *mut u16,
    stride: isize,
    left: *const [u16; 4],
    lpf: *const u16,
    w: c_int,
    h: c_int,
    params: *const LooprestorationParams,
    edges: c_uint,
    bitdepth_max: c_int,
) {
    let x6 = params as *const i16;
    let w8 = bitdepth_max as u32; // ldr w8, [sp]
    let v0_1 = vld1q_s16_x2(x6); // ld1 {v0.8h, v1.8h}, [x6]

    let v28 = vdupq_n_u16(w8 as u16); // dup v28.8h, w8 // bitdepth_max
    let w8 = w8.leading_zeros() as i32; // clz w8, w8
    let v30 = vdupq_n_u32(1); // movi v30.4s, #1
    let w10 = w8 - 38; // -(bitdepth + 6)
    let w11 = w8 - 11; // round_bits_v
    let w8 = w8 - 25; // -round_bits_h
    let w10 = -w10; // bitdepth + 6
    let w11 = -w11; // -round_bits_v
    let v2 = vdupq_n_s32(w10);
    let v29 = vdupq_n_s32(w8); // -round_bits_h
    let v27 = vdupq_n_s32(w11); // -round_bits_v
    let v31 = vdupq_n_u16(0x20 << 8); // movi v31.8h, #0x20, lsl #8 // 1 << 13 = 8192
    let v30 = vreinterpretq_s32_u32(vshlq_u32(v30, v2)); // ushl v30.4s, v30.4s, v2.4s // 1 << (bitdepth + 6)

    // zip1 v0.2d, v0.2d, v1.2d // move vertical coeffs to v0.h[4-7], freeing up v1
    let v0 = vreinterpretq_s16_s64(vzip1q_s64(vreinterpretq_s64_s16(v0_1.0), vreinterpretq_s64_s16(v0_1.1)));
    let mut w5 = h;

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
        v0,
        v27,
        v28,
        v29,
        v30,
        v31,
    };
    let x16: *const u16;

    // The number of vertical passes that remain: 2 for L(v2_5), 1 for L(v1_5), 0 for L(end_5).
    let v_passes = 'v: {
        'main_5: {
            // tst w7, #4 // LR_HAVE_TOP; b.eq L(no_top_5)
            if r.w7 & 4 != 0 {
                let x16_left = r.x2; // mov x16, x2 // backup left
                r.x2 = core::ptr::null(); // mov x2, #0
                wiener_filter5_h_16bpc_neon(&mut r);
                r.x3 = r.x3.byte_offset(r.x1); // lpf += stride
                r.x11 = r.x14; // t4
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter5_h_16bpc_neon(&mut r);
                r.x3 = r.x3.wrapping_byte_offset(r.x1 * 4);
                r.x3 = r.x3.wrapping_byte_offset(r.x1); // lpf += stride*5
                r.x12 = r.x14; // t3
                r.x14 = r.x14.add(384); // t1 += 384*2
                r.x2 = x16_left; // left
                x16 = r.x3; // backup lpf
                r.x3 = r.x0; // lpf = p
                wiener_filter5_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_5)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter5_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_5)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
            } else {
                // L(no_top_5):
                r.x3 = r.x3.wrapping_byte_offset(r.x1 * 4);
                x16 = r.x3.wrapping_byte_offset(r.x1 * 2); // lpf += stride*6, backup
                r.x3 = r.x0; // lpf = p

                wiener_filter5_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                r.x11 = r.x14; // t4
                r.x12 = r.x14; // t3
                r.x13 = r.x14; // t2
                if w5 == 0 {
                    break 'v 1; // b.eq L(v1_5)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
                r.x14 = r.x14.add(384); // t1 += 384*2
                wiener_filter5_h_16bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_5)
                }
                r.x3 = r.x3.byte_offset(r.x1); // src += stride
                r.x15 = r.x14.add(384); // t0 = t1 + 384*2
                wiener_filter5_hv_16bpc_neon(&mut r);
                w5 -= 1; // h--
                if w5 == 0 {
                    break 'v 2; // b.eq L(v2_5)
                }
                r.x15 = r.x15.add(384 * 3); // t0 += 384*2*3
                wiener_filter5_hv_16bpc_neon(&mut r);
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
            wiener_filter5_hv_16bpc_neon(&mut r);
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
        wiener_filter5_hv_16bpc_neon(&mut r);
        wiener_filter5_hv_16bpc_neon(&mut r);
        0 // L(end_5)
    };
    if v_passes >= 2 {
        // L(v2_5):
        wiener_filter5_v_16bpc_neon(&mut r);
        r.x0 = r.x0.wrapping_byte_offset(r.x1); // add x0, x0, x1
        r.x11 = r.x12; // mov x11, x12
        r.x12 = r.x13; // mov x12, x13
        r.x13 = r.x14; // mov x13, x14
    }
    if v_passes >= 1 {
        // L(v1_5):
        wiener_filter5_v_16bpc_neon(&mut r);
    }
    // L(end_5):
}

/// `wiener_filter5_h_16bpc_neon`: filters the source row `x3` horizontally into the row `x14`.
/// Leaves `x3`, `w4` and `x14` unchanged (the assembly saves and restores them); advances `x2` by one
/// row of `left` if it is not NULL.
unsafe fn wiener_filter5_h_16bpc_neon(r: &mut WienerRegs) {
    // stp x3, x4, [sp, #-32]!; str x14, [sp, #16]
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    let x14 = r.x14;
    let (v0, v29, v30, v31) = (r.v0, r.v29, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let (mut v2, mut v3);
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(2); // sub x3, x3, #4
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            v2 = v2_3.0;
            v3 = v2_3.1;
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            // ld1 {v4.d}[1], [x2], #8 (the low half of v4 is not used)
            let v4 = vsetq_lane_u64::<1>(read_unaligned(r.x2 as *const u64), vdupq_n_u64(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 2 pixels we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(2); // sub x3, x3, #4
            v3 = vextq_s16::<6>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #12
            v2 = vextq_s16::<6>(vreinterpretq_s16_u64(v4), v2_3.0); // ext v2.16b, v4.16b, v2.16b, #12
        }
    } else {
        // 1:
        let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost pixel
        // and shift v3 to have 3x the first pixel at the front.
        let v4 = vdupq_laneq_s16::<0>(v2_3.0);
        // Move x3 back to account for the last 2 pixels we loaded before,
        // which we shifted out.
        x3 = x3.sub(2); // sub x3, x3, #4
        v3 = vextq_s16::<6>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #12
        v2 = vextq_s16::<6>(v4, v2_3.0); // ext v2.16b, v4.16b, v2.16b, #12
    }

    // 2:
    let mut v4 = vld1q_s16(x3 as *const i16); // ld1 {v4.8h}, [x3], #16
    x3 = x3.add(8);

    // The element index of the 16-pixel block (`x3` and `x0` post-increment in the assembly).
    let mut i = 0;
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
            let h26 = read_unaligned(x3.add(i).offset(w17 as isize)); // ldr h26, [x3, w17, sxtw #1]
            let v26 = vreinterpretq_u8_u16(vdupq_n_u16(h26)); // dup v26.8h, v26.h[0]
            let v23_25 = vld1q_u8_x3(x6); // ld1 {v23.16b, v24.16b, v25.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v23_25.0, v26, vreinterpretq_u8_s16(v2))); // bit v2.16b, v26.16b, v23.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v23_25.1, v26, vreinterpretq_u8_s16(v3))); // bit v3.16b, v26.16b, v24.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v23_25.2, v26, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v26.16b, v25.16b
        }

        // 4: // Loop horizontally
        let v16 = vextq_s16::<1>(v2, v3); // ext v16.16b, v2.16b, v3.16b, #2
        let v18 = vextq_s16::<3>(v2, v3); // ext v18.16b, v2.16b, v3.16b, #6
        let v19 = vextq_s16::<4>(v2, v3); // ext v19.16b, v2.16b, v3.16b, #8
        let v17 = vextq_s16::<2>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v2);
        let v6 = vmull_laneq_s16::<3>(vget_low_s16(v17), v0); // smull v6.4s, v17.4h, v0.h[3]
        let v6 = vmlal_laneq_s16::<2>(v6, vget_low_s16(v18), v0); // smlal v6.4s, v18.4h, v0.h[2]
        let v6 = vmlal_laneq_s16::<1>(v6, vget_low_s16(v19), v0); // smlal v6.4s, v19.4h, v0.h[1]
        let v7 = vmull_high_laneq_s16::<3>(v17, v0); // smull2 v7.4s, v17.8h, v0.h[3]
        let v7 = vmlal_high_laneq_s16::<2>(v7, v18, v0); // smlal2 v7.4s, v18.8h, v0.h[2]
        let v7 = vmlal_high_laneq_s16::<1>(v7, v19, v0); // smlal2 v7.4s, v19.8h, v0.h[1]

        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v3);
        let v16 = vmull_laneq_s16::<3>(vget_low_s16(v17), v0); // smull v16.4s, v17.4h, v0.h[3]
        let v16 = vmlal_laneq_s16::<2>(v16, vget_low_s16(v18), v0); // smlal v16.4s, v18.4h, v0.h[2]
        let v16 = vmlal_laneq_s16::<1>(v16, vget_low_s16(v19), v0); // smlal v16.4s, v19.4h, v0.h[1]
        let v17 = vmull_high_laneq_s16::<3>(v17, v0); // smull2 v17.4s, v17.8h, v0.h[3]
        let v17 = vmlal_high_laneq_s16::<2>(v17, v18, v0); // smlal2 v17.4s, v18.8h, v0.h[2]
        let v17 = vmlal_high_laneq_s16::<1>(v17, v19, v0); // smlal2 v17.4s, v19.8h, v0.h[1]

        let v24 = vdupq_n_u16(0x7fff); // mvni v24.8h, #0x80, lsl #8 // 0x7fff = (1 << 15) - 1
        let v6 = vaddq_s32(v6, v30);
        let v7 = vaddq_s32(v7, v30);
        let v16 = vaddq_s32(v16, v30);
        let v17 = vaddq_s32(v17, v30);
        let v6 = vrshlq_s32(v6, v29); // srshl v6.4s, v6.4s, v29.4s
        let v7 = vrshlq_s32(v7, v29); // srshl v7.4s, v7.4s, v29.4s
        let v16 = vrshlq_s32(v16, v29); // srshl v16.4s, v16.4s, v29.4s
        let v17 = vrshlq_s32(v17, v29); // srshl v17.4s, v17.4s, v29.4s
        let v6 = vqmovun_s32(v6); // sqxtun v6.4h, v6.4s
        let v6 = vqmovun_high_s32(v6, v7); // sqxtun2 v6.8h, v7.4s
        let v7 = vqmovun_s32(v16); // sqxtun v7.4h, v16.4s
        let v7 = vqmovun_high_s32(v7, v17); // sqxtun2 v7.8h, v17.4s
        let v6 = vminq_u16(v6, v24);
        let v7 = vminq_u16(v7, v24);
        let v6 = vreinterpretq_s16_u16(vsubq_u16(v6, v31));
        let v7 = vreinterpretq_s16_u16(vsubq_u16(v7, v31));

        w4 -= 16; // subs w4, w4, #16

        vst1q_s16(x14.add(i), v6); // st1 {v6.8h, v7.8h}, [x14], #32
        vst1q_s16(x14.add(i + 8), v7);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        let v3_4 = vld1q_s16_x2(x3.add(i) as *const i16); // ld1 {v3.8h, v4.8h}, [x3], #32
        i += 16;
        v3 = v3_4.0;
        v4 = v3_4.1;
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }
    // 0:
    // ldr x14, [sp, #16]; ldp x3, x4, [sp], #32
}

/// `wiener_filter5_v_16bpc_neon`: filters the rows `x11`-`x14` (t4-t1, with t1 standing in for t0)
/// vertically into the output row `x0`. Leaves every register unchanged (the assembly saves and
/// restores the row pointers, `x0` and `w4`).
unsafe fn wiener_filter5_v_16bpc_neon(r: &mut WienerRegs) {
    // stp x11, x12, [sp, #-48]!; stp x13, x14, [sp, #16]; stp x0, x4, [sp, #32]
    let (x11, x12, x13, x14) = (r.x11, r.x12, r.x13, r.x14);
    let x0 = r.x0;
    let mut w4 = r.w4;
    let (v0, v27, v28) = (r.v0, r.v27, r.v28);
    // The element index of the 16-pixel block (`x3` and `x0` post-increment in the assembly).
    let mut i = 0;
    loop {
        // 1:
        let v16_17 = vld1q_s16_x2(x11.add(i)); // ld1 {v16.8h, v17.8h}, [x11], #32
        let v18_19 = vld1q_s16_x2(x12.add(i)); // ld1 {v18.8h, v19.8h}, [x12], #32
        let v20_21 = vld1q_s16_x2(x13.add(i)); // ld1 {v20.8h, v21.8h}, [x13], #32
        let v22_23 = vld1q_s16_x2(x14.add(i)); // ld1 {v22.8h, v23.8h}, [x14], #32
        let (v16, v17) = (v16_17.0, v16_17.1);
        let (v18, v19) = (v18_19.0, v18_19.1);
        let (v20, v21) = (v20_21.0, v20_21.1);
        let (v22, v23) = (v22_23.0, v22_23.1);

        let v2 = vmull_laneq_s16::<5>(vget_low_s16(v16), v0); // smull v2.4s, v16.4h, v0.h[5]
        let v2 = vmlal_laneq_s16::<6>(v2, vget_low_s16(v18), v0); // smlal v2.4s, v18.4h, v0.h[6]
        let v2 = vmlal_laneq_s16::<7>(v2, vget_low_s16(v20), v0); // smlal v2.4s, v20.4h, v0.h[7]
        let v2 = vmlal_laneq_s16::<6>(v2, vget_low_s16(v22), v0); // smlal v2.4s, v22.4h, v0.h[6]
        let v2 = vmlal_laneq_s16::<5>(v2, vget_low_s16(v22), v0); // smlal v2.4s, v22.4h, v0.h[5]
        let v3 = vmull_high_laneq_s16::<5>(v16, v0); // smull2 v3.4s, v16.8h, v0.h[5]
        let v3 = vmlal_high_laneq_s16::<6>(v3, v18, v0); // smlal2 v3.4s, v18.8h, v0.h[6]
        let v3 = vmlal_high_laneq_s16::<7>(v3, v20, v0); // smlal2 v3.4s, v20.8h, v0.h[7]
        let v3 = vmlal_high_laneq_s16::<6>(v3, v22, v0); // smlal2 v3.4s, v22.8h, v0.h[6]
        let v3 = vmlal_high_laneq_s16::<5>(v3, v22, v0); // smlal2 v3.4s, v22.8h, v0.h[5]
        let v4 = vmull_laneq_s16::<5>(vget_low_s16(v17), v0); // smull v4.4s, v17.4h, v0.h[5]
        let v4 = vmlal_laneq_s16::<6>(v4, vget_low_s16(v19), v0); // smlal v4.4s, v19.4h, v0.h[6]
        let v4 = vmlal_laneq_s16::<7>(v4, vget_low_s16(v21), v0); // smlal v4.4s, v21.4h, v0.h[7]
        let v4 = vmlal_laneq_s16::<6>(v4, vget_low_s16(v23), v0); // smlal v4.4s, v23.4h, v0.h[6]
        let v4 = vmlal_laneq_s16::<5>(v4, vget_low_s16(v23), v0); // smlal v4.4s, v23.4h, v0.h[5]
        let v5 = vmull_high_laneq_s16::<5>(v17, v0); // smull2 v5.4s, v17.8h, v0.h[5]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v19, v0); // smlal2 v5.4s, v19.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<7>(v5, v21, v0); // smlal2 v5.4s, v21.8h, v0.h[7]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v23, v0); // smlal2 v5.4s, v23.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v23, v0); // smlal2 v5.4s, v23.8h, v0.h[5]
        let v2 = vrshlq_s32(v2, v27); // srshl v2.4s, v2.4s, v27.4s // -round_bits_v
        let v3 = vrshlq_s32(v3, v27); // srshl v3.4s, v3.4s, v27.4s
        let v4 = vrshlq_s32(v4, v27); // srshl v4.4s, v4.4s, v27.4s
        let v5 = vrshlq_s32(v5, v27); // srshl v5.4s, v5.4s, v27.4s
        let v2 = vqmovun_s32(v2); // sqxtun v2.4h, v2.4s
        let v2 = vqmovun_high_s32(v2, v3); // sqxtun2 v2.8h, v3.4s
        let v3 = vqmovun_s32(v4); // sqxtun v3.4h, v4.4s
        let v3 = vqmovun_high_s32(v3, v5); // sqxtun2 v3.8h, v5.4s
        let v2 = vminq_u16(v2, v28); // umin v2.8h, v2.8h, v28.8h // bitdepth_max
        let v3 = vminq_u16(v3, v28); // umin v3.8h, v3.8h, v28.8h

        w4 -= 16; // subs w4, w4, #16
        vst1q_u16(x0.add(i), v2); // st1 {v2.8h, v3.8h}, [x0], #32
        vst1q_u16(x0.add(i + 8), v3);
        i += 16;
        if w4 <= 0 {
            break; // b.gt 1b
        }
    }

    // ldp x0, x4, [sp, #32]; ldp x13, x14, [sp, #16]; ldp x11, x12, [sp], #48
}

/// `wiener_filter5_hv_16bpc_neon`: filters the source row `x3` horizontally into the row `x15` (t0) and
/// the rows `x11`-`x14` with it vertically into the output row `x0`. Leaves the row pointers shifted
/// (`x11` gets the value of `x12`, etc, `x14` that of `x15`, and `x15` the former `x12`), `x3` and `x0`
/// advanced by `stride`, and `x2` advanced by one row of `left` if it is not NULL.
unsafe fn wiener_filter5_hv_16bpc_neon(r: &mut WienerRegs) {
    // Backing up/restoring registers shifted, so that x11 gets the value
    // of x12, etc, and x15==x11, afterwards.
    // stp x12, x13, [sp, #-64]!; stp x14, x15, [sp, #16]; stp x12, x0, [sp, #32]; stp x3, x4, [sp, #48]
    let (x11, x12, x13, x14, x15) = (r.x11, r.x12, r.x13, r.x14, r.x15);
    let x0 = r.x0;
    let mut x3 = r.x3;
    let mut w4 = r.w4;
    // The values the `ldp`s at the end leave in x11-x15.
    r.x11 = x12;
    r.x12 = x13;
    r.x13 = x14;
    r.x14 = x15;
    r.x15 = x12;
    let (v0, v27, v28, v29, v30, v31) = (r.v0, r.v27, r.v28, r.v29, r.v30, r.v31);

    // Set up the src pointers to include the left edge, for LR_HAVE_LEFT, left == NULL
    let (mut v2, mut v3);
    // tst w7, #1 // LR_HAVE_LEFT; b.eq 1f
    if r.w7 & 1 != 0 {
        // LR_HAVE_LEFT
        // cbnz x2, 0f
        if r.x2.is_null() {
            // left == NULL
            x3 = x3.sub(2); // sub x3, x3, #4
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            v2 = v2_3.0;
            v3 = v2_3.1;
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
            x3 = x3.add(16);
            // ld1 {v4.d}[1], [x2], #8 (the low half of v4 is not used)
            let v4 = vsetq_lane_u64::<1>(read_unaligned(r.x2 as *const u64), vdupq_n_u64(0));
            r.x2 = r.x2.wrapping_add(1);
            // Move x3 back to account for the last 2 pixels we loaded earlier,
            // which we'll shift out.
            x3 = x3.sub(2); // sub x3, x3, #4
            v3 = vextq_s16::<6>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #12
            v2 = vextq_s16::<6>(vreinterpretq_s16_u64(v4), v2_3.0); // ext v2.16b, v4.16b, v2.16b, #12
        }
    } else {
        // 1:
        let v2_3 = vld1q_s16_x2(x3 as *const i16); // ld1 {v2.8h, v3.8h}, [x3], #32
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost pixel
        // and shift v3 to have 2x the first pixel at the front.
        let v4 = vdupq_laneq_s16::<0>(v2_3.0);
        // Move x3 back to account for the last 2 pixels we loaded before,
        // which we shifted out.
        x3 = x3.sub(2); // sub x3, x3, #4
        v3 = vextq_s16::<6>(v2_3.0, v2_3.1); // ext v3.16b, v2.16b, v3.16b, #12
        v2 = vextq_s16::<6>(v4, v2_3.0); // ext v2.16b, v4.16b, v2.16b, #12
    }

    // 2:
    let mut v4 = vld1q_s16(x3 as *const i16); // ld1 {v4.8h}, [x3], #16
    x3 = x3.add(8);

    // The element index of the 16-pixel block (`x3` and `x0` post-increment in the assembly).
    let mut i = 0;
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
            let h26 = read_unaligned(x3.add(i).offset(w17 as isize)); // ldr h26, [x3, w17, sxtw #1]
            let v26 = vreinterpretq_u8_u16(vdupq_n_u16(h26)); // dup v26.8h, v26.h[0]
            let v23_25 = vld1q_u8_x3(x6); // ld1 {v23.16b, v24.16b, v25.16b}, [x6]

            v2 = vreinterpretq_s16_u8(vbslq_u8(v23_25.0, v26, vreinterpretq_u8_s16(v2))); // bit v2.16b, v26.16b, v23.16b
            v3 = vreinterpretq_s16_u8(vbslq_u8(v23_25.1, v26, vreinterpretq_u8_s16(v3))); // bit v3.16b, v26.16b, v24.16b
            v4 = vreinterpretq_s16_u8(vbslq_u8(v23_25.2, v26, vreinterpretq_u8_s16(v4)));
            // bit v4.16b, v26.16b, v25.16b
        }

        // 4: // Loop horizontally
        let v16 = vextq_s16::<1>(v2, v3); // ext v16.16b, v2.16b, v3.16b, #2
        let v18 = vextq_s16::<3>(v2, v3); // ext v18.16b, v2.16b, v3.16b, #6
        let v19 = vextq_s16::<4>(v2, v3); // ext v19.16b, v2.16b, v3.16b, #8
        let v17 = vextq_s16::<2>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v2);
        let v6 = vmull_laneq_s16::<3>(vget_low_s16(v17), v0); // smull v6.4s, v17.4h, v0.h[3]
        let v6 = vmlal_laneq_s16::<2>(v6, vget_low_s16(v18), v0); // smlal v6.4s, v18.4h, v0.h[2]
        let v6 = vmlal_laneq_s16::<1>(v6, vget_low_s16(v19), v0); // smlal v6.4s, v19.4h, v0.h[1]
        let v7 = vmull_high_laneq_s16::<3>(v17, v0); // smull2 v7.4s, v17.8h, v0.h[3]
        let v7 = vmlal_high_laneq_s16::<2>(v7, v18, v0); // smlal2 v7.4s, v18.8h, v0.h[2]
        let v7 = vmlal_high_laneq_s16::<1>(v7, v19, v0); // smlal2 v7.4s, v19.8h, v0.h[1]

        let v16 = vextq_s16::<1>(v3, v4); // ext v16.16b, v3.16b, v4.16b, #2
        let v18 = vextq_s16::<3>(v3, v4); // ext v18.16b, v3.16b, v4.16b, #6
        let v19 = vextq_s16::<4>(v3, v4); // ext v19.16b, v3.16b, v4.16b, #8
        let v17 = vextq_s16::<2>(v3, v4); // ext v17.16b, v3.16b, v4.16b, #4
        let v18 = vaddq_s16(v18, v16);
        let v19 = vaddq_s16(v19, v3);
        let v24 = vmull_laneq_s16::<3>(vget_low_s16(v17), v0); // smull v24.4s, v17.4h, v0.h[3]
        let v24 = vmlal_laneq_s16::<2>(v24, vget_low_s16(v18), v0); // smlal v24.4s, v18.4h, v0.h[2]
        let v24 = vmlal_laneq_s16::<1>(v24, vget_low_s16(v19), v0); // smlal v24.4s, v19.4h, v0.h[1]
        let v25 = vmull_high_laneq_s16::<3>(v17, v0); // smull2 v25.4s, v17.8h, v0.h[3]
        let v25 = vmlal_high_laneq_s16::<2>(v25, v18, v0); // smlal2 v25.4s, v18.8h, v0.h[2]
        let v25 = vmlal_high_laneq_s16::<1>(v25, v19, v0); // smlal2 v25.4s, v19.8h, v0.h[1]

        let v16_17 = vld1q_s16_x2(x11.add(i)); // ld1 {v16.8h, v17.8h}, [x11], #32
        let v26 = vdupq_n_u16(0x7fff); // mvni v26.8h, #0x80, lsl #8 // 0x7fff = (1 << 15) - 1
        let v6 = vaddq_s32(v6, v30);
        let v7 = vaddq_s32(v7, v30);
        let v24 = vaddq_s32(v24, v30);
        let v25 = vaddq_s32(v25, v30);
        let v18_19 = vld1q_s16_x2(x12.add(i)); // ld1 {v18.8h, v19.8h}, [x12], #32
        let v6 = vrshlq_s32(v6, v29); // srshl v6.4s, v6.4s, v29.4s
        let v7 = vrshlq_s32(v7, v29); // srshl v7.4s, v7.4s, v29.4s
        let v24 = vrshlq_s32(v24, v29); // srshl v24.4s, v24.4s, v29.4s
        let v25 = vrshlq_s32(v25, v29); // srshl v25.4s, v25.4s, v29.4s
        let v20_21 = vld1q_s16_x2(x13.add(i)); // ld1 {v20.8h, v21.8h}, [x13], #32
        let v6 = vqmovun_s32(v6); // sqxtun v6.4h, v6.4s
        let v6 = vqmovun_high_s32(v6, v7); // sqxtun2 v6.8h, v7.4s
        let v7 = vqmovun_s32(v24); // sqxtun v7.4h, v24.4s
        let v7 = vqmovun_high_s32(v7, v25); // sqxtun2 v7.8h, v25.4s
        let v22_23 = vld1q_s16_x2(x14.add(i)); // ld1 {v22.8h, v23.8h}, [x14], #32
        let v6 = vminq_u16(v6, v26);
        let v7 = vminq_u16(v7, v26);
        let v6 = vreinterpretq_s16_u16(vsubq_u16(v6, v31));
        let v7 = vreinterpretq_s16_u16(vsubq_u16(v7, v31));
        let (v16, v17) = (v16_17.0, v16_17.1);
        let (v18, v19) = (v18_19.0, v18_19.1);
        let (v20, v21) = (v20_21.0, v20_21.1);
        let (v22, v23) = (v22_23.0, v22_23.1);

        let v8 = vmull_laneq_s16::<5>(vget_low_s16(v16), v0); // smull v8.4s, v16.4h, v0.h[5]
        let v8 = vmlal_laneq_s16::<6>(v8, vget_low_s16(v18), v0); // smlal v8.4s, v18.4h, v0.h[6]
        let v8 = vmlal_laneq_s16::<7>(v8, vget_low_s16(v20), v0); // smlal v8.4s, v20.4h, v0.h[7]
        let v8 = vmlal_laneq_s16::<6>(v8, vget_low_s16(v22), v0); // smlal v8.4s, v22.4h, v0.h[6]
        let v8 = vmlal_laneq_s16::<5>(v8, vget_low_s16(v6), v0); // smlal v8.4s, v6.4h, v0.h[5]
        let v9 = vmull_high_laneq_s16::<5>(v16, v0); // smull2 v9.4s, v16.8h, v0.h[5]
        let v9 = vmlal_high_laneq_s16::<6>(v9, v18, v0); // smlal2 v9.4s, v18.8h, v0.h[6]
        let v9 = vmlal_high_laneq_s16::<7>(v9, v20, v0); // smlal2 v9.4s, v20.8h, v0.h[7]
        let v9 = vmlal_high_laneq_s16::<6>(v9, v22, v0); // smlal2 v9.4s, v22.8h, v0.h[6]
        let v9 = vmlal_high_laneq_s16::<5>(v9, v6, v0); // smlal2 v9.4s, v6.8h, v0.h[5]
        let v1 = vmull_laneq_s16::<5>(vget_low_s16(v17), v0); // smull v1.4s, v17.4h, v0.h[5]
        let v1 = vmlal_laneq_s16::<6>(v1, vget_low_s16(v19), v0); // smlal v1.4s, v19.4h, v0.h[6]
        let v1 = vmlal_laneq_s16::<7>(v1, vget_low_s16(v21), v0); // smlal v1.4s, v21.4h, v0.h[7]
        let v1 = vmlal_laneq_s16::<6>(v1, vget_low_s16(v23), v0); // smlal v1.4s, v23.4h, v0.h[6]
        let v1 = vmlal_laneq_s16::<5>(v1, vget_low_s16(v7), v0); // smlal v1.4s, v7.4h, v0.h[5]
        let v5 = vmull_high_laneq_s16::<5>(v17, v0); // smull2 v5.4s, v17.8h, v0.h[5]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v19, v0); // smlal2 v5.4s, v19.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<7>(v5, v21, v0); // smlal2 v5.4s, v21.8h, v0.h[7]
        let v5 = vmlal_high_laneq_s16::<6>(v5, v23, v0); // smlal2 v5.4s, v23.8h, v0.h[6]
        let v5 = vmlal_high_laneq_s16::<5>(v5, v7, v0); // smlal2 v5.4s, v7.8h, v0.h[5]
        let v8 = vrshlq_s32(v8, v27); // srshl v8.4s, v8.4s, v27.4s // -round_bits_v
        let v9 = vrshlq_s32(v9, v27); // srshl v9.4s, v9.4s, v27.4s
        let v1 = vrshlq_s32(v1, v27); // srshl v1.4s, v1.4s, v27.4s
        let v5 = vrshlq_s32(v5, v27); // srshl v5.4s, v5.4s, v27.4s
        let v8 = vqmovun_s32(v8); // sqxtun v8.4h, v8.4s
        let v8 = vqmovun_high_s32(v8, v9); // sqxtun2 v8.8h, v9.4s
        let v9 = vqmovun_s32(v1); // sqxtun v9.4h, v1.4s
        let v9 = vqmovun_high_s32(v9, v5); // sqxtun2 v9.8h, v5.4s
        vst1q_s16(x15.add(i), v6); // st1 {v6.8h, v7.8h}, [x15], #32
        vst1q_s16(x15.add(i + 8), v7);
        let v8 = vminq_u16(v8, v28); // umin v8.8h, v8.8h, v28.8h // bitdepth_max
        let v9 = vminq_u16(v9, v28); // umin v9.8h, v9.8h, v28.8h

        w4 -= 16; // subs w4, w4, #16

        vst1q_u16(x0.add(i), v8); // st1 {v8.8h, v9.8h}, [x0], #32
        vst1q_u16(x0.add(i + 8), v9);

        if w4 <= 0 {
            break; // b.le 0f
        }
        v2 = v4; // mov v2.16b, v4.16b

        // tst w7, #2 // LR_HAVE_RIGHT
        let v3_4 = vld1q_s16_x2(x3.add(i) as *const i16); // ld1 {v3.8h, v4.8h}, [x3], #32
        i += 16;
        v3 = v3_4.0;
        v4 = v3_4.1;
        // b.ne 4b // If we don't need to pad, just keep filtering.
        // b 3b // If we need to pad, check how many pixels we have left.
    }

    // 0:
    // ldp x3, x4, [sp, #48]; ldp x15, x0, [sp, #32]; ldp x13, x14, [sp, #16]; ldp x11, x12, [sp], #64
    // (`r` has the shifted x11-x15 since the entry.)

    r.x3 = r.x3.wrapping_byte_offset(r.x1); // add x3, x3, x1
    r.x0 = r.x0.wrapping_byte_offset(r.x1); // add x0, x0, x1
}

/// `ld1 {v2.d}[1], [x2]`: the four pixels of `left`, here in both 64-bit halves (only the upper one is read by the
/// `ext` that follows).
#[inline(always)]
unsafe fn sgr_ld1_left_16bpc(x2: *const [u16; 4]) -> uint16x8_t {
    vreinterpretq_u16_u64(vdupq_n_u64(read_unaligned(x2 as *const u64)))
}

/// `sgr_box3_row_h_16bpc_neon`: `void dav1d_sgr_box3_row_h_16bpc_neon(int32_t *sumsq, int16_t *sum,
/// const pixel (*left)[4], const pixel *src, const int w, const enum LrEdgeFlags edges)`
///
/// # Safety
/// See the notes at the top of this part: `sumsq` and `sum` writable for `8 * ceil((w + 2) / 8)` elements; `src`
/// readable as described there; `left` null or four pixels.
pub unsafe extern "C" fn dav1d_sgr_box3_row_h_16bpc_neon(
    sumsq: *mut i32,
    sum: *mut i16,
    left: *const [u16; 4],
    src: *const u16,
    w: c_int,
    edges: c_uint,
) {
    let (mut x0, mut x1, x2, mut x3, w5) = (sumsq as *mut u32, sum as *mut u16, left, src, edges);
    let mut w4 = w + 2; // add w4, w4, #2 (w += 2)

    let (mut v0, mut v1);
    if w5 & 1 != 0 {
        // tst w5, #1 (LR_HAVE_LEFT); b.eq 1f
        if x2.is_null() {
            // cbnz x2, 0f
            // LR_HAVE_LEFT && left == NULL
            x3 = x3.sub(2); // sub x3, x3, #4
            uint16x8x2_t(v0, v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            uint16x8x2_t(v0, v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
            x3 = x3.add(16);
            let v2 = sgr_ld1_left_16bpc(x2); // ld1 {v2.d}[1], [x2]
                                             // Move x3 back to account for the last 2 pixels we loaded earlier,
                                             // which we'll shift out.
            x3 = x3.sub(2); // sub x3, x3, #4
            v1 = vextq_u16::<6>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #12
            v0 = vextq_u16::<6>(v2, v0); // ext v0.16b, v2.16b, v0.16b, #12
        }
    } else {
        // 1:
        uint16x8x2_t(v0, v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost pixel
        // and shift v0/v1 to have 2x the first pixel at the front.
        let v2 = vdupq_laneq_u16::<0>(v0); // dup v2.8h, v0.h[0]
                                           // Move x3 back to account for the last 2 pixels we loaded before,
                                           // which we shifted out.
        x3 = x3.sub(2); // sub x3, x3, #4
        v1 = vextq_u16::<6>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #12
        v0 = vextq_u16::<6>(v2, v0); // ext v0.16b, v2.16b, v0.16b, #12
    }

    // 2:
    // 4: Loop horizontally: one block of 8 sums; leaves the loop around it at `b.le 9f`.
    macro_rules! loop_horizontally {
        () => {
            let v26 = vextq_u16::<1>(v0, v1); // ext v26.16b, v0.16b, v1.16b, #2
            let v27 = vextq_u16::<2>(v0, v1); // ext v27.16b, v0.16b, v1.16b, #4

            let v6 = vaddq_u16(v0, v26); // add v6.8h, v0.8h, v26.8h
            let v22 = vmull_u16(vget_low_u16(v0), vget_low_u16(v0)); // umull v22.4s, v0.4h, v0.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v26), vget_low_u16(v26)); // umlal v22.4s, v26.4h, v26.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v27), vget_low_u16(v27)); // umlal v22.4s, v27.4h, v27.4h
            let v6 = vaddq_u16(v6, v27); // add v6.8h, v6.8h, v27.8h
            let v23 = vmull_high_u16(v0, v0); // umull2 v23.4s, v0.8h, v0.8h
            let v23 = vmlal_high_u16(v23, v26, v26); // umlal2 v23.4s, v26.8h, v26.8h
            let v23 = vmlal_high_u16(v23, v27, v27); // umlal2 v23.4s, v27.8h, v27.8h

            w4 -= 8; // subs w4, w4, #8

            vst1q_u16(x1, v6); // st1 {v6.8h}, [x1], #16
            x1 = x1.add(8);
            vst1q_u32_x2(x0, uint32x4x2_t(v22, v23)); // st1 {v22.4s, v23.4s}, [x0], #32
            x0 = x0.add(8);

            if w4 <= 0 {
                break; // b.le 9f
            }
            // tst w5, #2 (LR_HAVE_RIGHT)
            v0 = v1; // mov v0.16b, v1.16b
            v1 = vld1q_u16(x3); // ld1 {v1.8h}, [x3], #16
            x3 = x3.add(8);
        };
    }

    if w5 & 2 != 0 {
        // tst w5, #2 (LR_HAVE_RIGHT); b.ne 4f
        loop {
            loop_horizontally!();
            // b.ne 4b (If we don't need to pad, just keep summing.)
        }
    } else {
        // If we'll need to pad the right edge, load that pixel to pad with
        // here since we can find it pretty easily from here.
        let w13 = w4 - (2 + 16 - 2 + 1); // sub w13, w4, #(2 + 16 - 2 + 1)

        // Fill v30 with the right padding pixel
        // ldr h30, [x3, w13, sxtw #1]; dup v30.8h, v30.h[0]
        let v30 = vld1q_dup_u16(x3.offset(w13 as isize));
        loop {
            // 3: !LR_HAVE_RIGHT

            // Check whether we need to pad the right edge
            if w4 < 10 {
                // cmp w4, #10; b.ge 4f (If w >= 10, all used input pixels are valid)

                // 1 <= w < 10, w pixels valid in v0. For w=9, this ends up called
                // again; it's not strictly needed in those cases (we pad enough here),
                // but keeping the code as simple as possible.

                // Insert padding in v0.h[w] onwards
                let x13 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK); // movrel x13, right_ext_mask
                let x13 = x13.sub((w4 as u32 as usize) << 1); // sub x13, x13, w4, uxtw #1
                let uint8x16x2_t(v28, v29) = vld1q_u8_x2(x13); // ld1 {v28.16b, v29.16b}, [x13]

                v0 = vbslq_u16(vreinterpretq_u16_u8(v28), v30, v0); // bit v0.16b, v30.16b, v28.16b
                v1 = vbslq_u16(vreinterpretq_u16_u8(v29), v30, v1); // bit v1.16b, v30.16b, v29.16b
            }

            loop_horizontally!();
            // b 3b (If we need to pad, check how many pixels we have left.)
        }
    }
    // 9:
}

/// `sgr_box5_row_h_16bpc_neon`: `void dav1d_sgr_box5_row_h_16bpc_neon(int32_t *sumsq, int16_t *sum,
/// const pixel (*left)[4], const pixel *src, const int w, const enum LrEdgeFlags edges)`
///
/// # Safety
/// See the notes at the top of this part: `sumsq` and `sum` writable for `8 * ceil((w + 2) / 8)` elements; `src`
/// readable as described there; `left` null or four pixels.
pub unsafe extern "C" fn dav1d_sgr_box5_row_h_16bpc_neon(
    sumsq: *mut i32,
    sum: *mut i16,
    left: *const [u16; 4],
    src: *const u16,
    w: c_int,
    edges: c_uint,
) {
    let (mut x0, mut x1, x2, mut x3, w5) = (sumsq as *mut u32, sum as *mut u16, left, src, edges);
    let mut w4 = w + 2; // add w4, w4, #2 (w += 2)

    let (mut v0, mut v1);
    if w5 & 1 != 0 {
        // tst w5, #1 (LR_HAVE_LEFT); b.eq 1f
        if x2.is_null() {
            // cbnz x2, 0f
            // LR_HAVE_LEFT && left == NULL
            x3 = x3.sub(3); // sub x3, x3, #6
            uint16x8x2_t(v0, v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
            x3 = x3.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            uint16x8x2_t(v0, v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
            x3 = x3.add(16);
            let v2 = sgr_ld1_left_16bpc(x2); // ld1 {v2.d}[1], [x2], #8
                                             // Move x3 back to account for the last 3 pixels we loaded earlier,
                                             // which we'll shift out.
            x3 = x3.sub(3); // sub x3, x3, #6
            v1 = vextq_u16::<5>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #10
            v0 = vextq_u16::<5>(v2, v0); // ext v0.16b, v2.16b, v0.16b, #10
        }
    } else {
        // 1:
        uint16x8x2_t(v0, v1) = vld1q_u16_x2(x3); // ld1 {v0.8h, v1.8h}, [x3], #32
        x3 = x3.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost pixel
        // and shift v0/v1 to have 3x the first pixel at the front.
        let v2 = vdupq_laneq_u16::<0>(v0); // dup v2.8h, v0.h[0]
                                           // Move x3 back to account for the last 3 pixels we loaded before,
                                           // which we shifted out.
        x3 = x3.sub(3); // sub x3, x3, #6
        v1 = vextq_u16::<5>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #10
        v0 = vextq_u16::<5>(v2, v0); // ext v0.16b, v2.16b, v0.16b, #10
    }

    // 2:
    // 4: Loop horizontally: one block of 8 sums; leaves the loop around it at `b.le 9f`.
    macro_rules! loop_horizontally {
        () => {
            let v26 = vextq_u16::<1>(v0, v1); // ext v26.16b, v0.16b, v1.16b, #2
            let v27 = vextq_u16::<2>(v0, v1); // ext v27.16b, v0.16b, v1.16b, #4

            let v6 = vaddq_u16(v0, v26); // add v6.8h, v0.8h, v26.8h
            let v22 = vmull_u16(vget_low_u16(v0), vget_low_u16(v0)); // umull v22.4s, v0.4h, v0.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v26), vget_low_u16(v26)); // umlal v22.4s, v26.4h, v26.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v27), vget_low_u16(v27)); // umlal v22.4s, v27.4h, v27.4h
            let v6 = vaddq_u16(v6, v27); // add v6.8h, v6.8h, v27.8h
            let v23 = vmull_high_u16(v0, v0); // umull2 v23.4s, v0.8h, v0.8h
            let v23 = vmlal_high_u16(v23, v26, v26); // umlal2 v23.4s, v26.8h, v26.8h
            let v23 = vmlal_high_u16(v23, v27, v27); // umlal2 v23.4s, v27.8h, v27.8h

            let v26 = vextq_u16::<3>(v0, v1); // ext v26.16b, v0.16b, v1.16b, #6
            let v27 = vextq_u16::<4>(v0, v1); // ext v27.16b, v0.16b, v1.16b, #8

            let v6 = vaddq_u16(v6, v26); // add v6.8h, v6.8h, v26.8h
            let v22 = vmlal_u16(v22, vget_low_u16(v26), vget_low_u16(v26)); // umlal v22.4s, v26.4h, v26.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v27), vget_low_u16(v27)); // umlal v22.4s, v27.4h, v27.4h
            let v6 = vaddq_u16(v6, v27); // add v6.8h, v6.8h, v27.8h
            let v23 = vmlal_high_u16(v23, v26, v26); // umlal2 v23.4s, v26.8h, v26.8h
            let v23 = vmlal_high_u16(v23, v27, v27); // umlal2 v23.4s, v27.8h, v27.8h

            w4 -= 8; // subs w4, w4, #8

            vst1q_u16(x1, v6); // st1 {v6.8h}, [x1], #16
            x1 = x1.add(8);
            vst1q_u32_x2(x0, uint32x4x2_t(v22, v23)); // st1 {v22.4s, v23.4s}, [x0], #32
            x0 = x0.add(8);

            if w4 <= 0 {
                break; // b.le 9f
            }
            // tst w5, #2 (LR_HAVE_RIGHT)
            v0 = v1; // mov v0.16b, v1.16b
            v1 = vld1q_u16(x3); // ld1 {v1.8h}, [x3], #16
            x3 = x3.add(8);
        };
    }

    if w5 & 2 != 0 {
        // tst w5, #2 (LR_HAVE_RIGHT); b.ne 4f
        loop {
            loop_horizontally!();
            // b.ne 4b (If we don't need to pad, just keep summing.)
        }
    } else {
        // If we'll need to pad the right edge, load that pixel to pad with
        // here since we can find it pretty easily from here.
        let w13 = w4 - (2 + 16 - 3 + 1); // sub w13, w4, #(2 + 16 - 3 + 1)

        // Fill v30 with the right padding pixel
        // ldr h30, [x3, w13, sxtw #1]; dup v30.8h, v30.h[0]
        let v30 = vld1q_dup_u16(x3.offset(w13 as isize));
        loop {
            // 3: !LR_HAVE_RIGHT

            // Check whether we need to pad the right edge
            if w4 < 11 {
                // cmp w4, #11; b.ge 4f (If w >= 11, all used input pixels are valid)

                // 1 <= w < 11, w+1 pixels valid in v0. For w=9 or w=10,
                // this ends up called again; it's not strictly needed in those
                // cases (we pad enough here), but keeping the code as simple as possible.

                // Insert padding in v0.h[w+1] onwards; fuse the +1 into the
                // buffer pointer.
                let x13 = RIGHT_EXT_MASK_BUF.as_ptr().add(RIGHT_EXT_MASK - 1); // movrel x13, right_ext_mask, -1
                let x13 = x13.sub((w4 as u32 as usize) << 1); // sub x13, x13, w4, uxtw #1
                let uint8x16x2_t(v28, v29) = vld1q_u8_x2(x13); // ld1 {v28.16b, v29.16b}, [x13]

                v0 = vbslq_u16(vreinterpretq_u16_u8(v28), v30, v0); // bit v0.16b, v30.16b, v28.16b
                v1 = vbslq_u16(vreinterpretq_u16_u8(v29), v30, v1); // bit v1.16b, v30.16b, v29.16b
            }

            loop_horizontally!();
            // b 3b (If we need to pad, check how many pixels we have left.)
        }
    }
    // 9:
}

/// `sgr_box35_row_h_16bpc_neon`: `void dav1d_sgr_box35_row_h_16bpc_neon(int32_t *sumsq3, int16_t *sum3,
/// int32_t *sumsq5, int16_t *sum5, const pixel (*left)[4], const pixel *src, const int w,
/// const enum LrEdgeFlags edges)`
///
/// # Safety
/// See the notes at the top of this part: the four output rows writable for `8 * ceil((w + 2) / 8)` elements; `src`
/// readable as described there; `left` null or four pixels.
pub unsafe extern "C" fn dav1d_sgr_box35_row_h_16bpc_neon(
    sumsq3: *mut i32,
    sum3: *mut i16,
    sumsq5: *mut i32,
    sum5: *mut i16,
    left: *const [u16; 4],
    src: *const u16,
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

    let (mut v0, mut v1);
    if w7 & 1 != 0 {
        // tst w7, #1 (LR_HAVE_LEFT); b.eq 1f
        if x4.is_null() {
            // cbnz x4, 0f
            // LR_HAVE_LEFT && left == NULL
            x5 = x5.sub(3); // sub x5, x5, #6
            uint16x8x2_t(v0, v1) = vld1q_u16_x2(x5); // ld1 {v0.8h, v1.8h}, [x5], #32
            x5 = x5.add(16);
        } else {
            // 0:
            // LR_HAVE_LEFT, left != NULL
            uint16x8x2_t(v0, v1) = vld1q_u16_x2(x5); // ld1 {v0.8h, v1.8h}, [x5], #32
            x5 = x5.add(16);
            let v2 = sgr_ld1_left_16bpc(x4); // ld1 {v2.d}[1], [x4], #8
                                             // Move x3 back to account for the last 3 pixels we loaded earlier,
                                             // which we'll shift out.
            x5 = x5.sub(3); // sub x5, x5, #6
            v1 = vextq_u16::<5>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #10
            v0 = vextq_u16::<5>(v2, v0); // ext v0.16b, v2.16b, v0.16b, #10
        }
    } else {
        // 1:
        uint16x8x2_t(v0, v1) = vld1q_u16_x2(x5); // ld1 {v0.8h, v1.8h}, [x5], #32
        x5 = x5.add(16);
        // !LR_HAVE_LEFT, fill v2 with the leftmost pixel
        // and shift v0/v1 to have 3x the first pixel at the front.
        let v2 = vdupq_laneq_u16::<0>(v0); // dup v2.8h, v0.h[0]
                                           // Move x5 back to account for the last 3 pixels we loaded before,
                                           // which we shifted out.
        x5 = x5.sub(3); // sub x5, x5, #6
        v1 = vextq_u16::<5>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #10
        v0 = vextq_u16::<5>(v2, v0); // ext v0.16b, v2.16b, v0.16b, #10
    }

    // 2:
    // 4: Loop horizontally: one block of 8 sums; leaves the loop around it at `b.le 9f`.
    macro_rules! loop_horizontally {
        () => {
            let v16 = vextq_u16::<1>(v0, v1); // ext v16.16b, v0.16b, v1.16b, #2
            let v17 = vextq_u16::<2>(v0, v1); // ext v17.16b, v0.16b, v1.16b, #4
            let v19 = vextq_u16::<4>(v0, v1); // ext v19.16b, v0.16b, v1.16b, #8
            let v18 = vextq_u16::<3>(v0, v1); // ext v18.16b, v0.16b, v1.16b, #6

            let v20 = vaddq_u16(v16, v17); // add v20.8h, v16.8h, v17.8h
            let v21 = vaddq_u16(v0, v19); // add v21.8h, v0.8h, v19.8h
            let v20 = vaddq_u16(v20, v18); // add v20.8h, v20.8h, v18.8h

            let v22 = vmull_u16(vget_low_u16(v16), vget_low_u16(v16)); // umull v22.4s, v16.4h, v16.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v17), vget_low_u16(v17)); // umlal v22.4s, v17.4h, v17.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v18), vget_low_u16(v18)); // umlal v22.4s, v18.4h, v18.4h

            let v23 = vmull_high_u16(v16, v16); // umull2 v23.4s, v16.8h, v16.8h
            let v23 = vmlal_high_u16(v23, v17, v17); // umlal2 v23.4s, v17.8h, v17.8h
            let v23 = vmlal_high_u16(v23, v18, v18); // umlal2 v23.4s, v18.8h, v18.8h

            let v21 = vaddq_u16(v21, v20); // add v21.8h, v21.8h, v20.8h
            vst1q_u16(x1, v20); // st1 {v20.8h}, [x1], #16
            x1 = x1.add(8);
            vst1q_u32_x2(x0, uint32x4x2_t(v22, v23)); // st1 {v22.4s, v23.4s}, [x0], #32
            x0 = x0.add(8);

            let v22 = vmlal_u16(v22, vget_low_u16(v0), vget_low_u16(v0)); // umlal v22.4s, v0.4h, v0.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v19), vget_low_u16(v19)); // umlal v22.4s, v19.4h, v19.4h

            let v23 = vmlal_high_u16(v23, v0, v0); // umlal2 v23.4s, v0.8h, v0.8h
            let v23 = vmlal_high_u16(v23, v19, v19); // umlal2 v23.4s, v19.8h, v19.8h

            w6 -= 8; // subs w6, w6, #8

            vst1q_u16(x3, v21); // st1 {v21.8h}, [x3], #16
            x3 = x3.add(8);
            vst1q_u32_x2(x2, uint32x4x2_t(v22, v23)); // st1 {v22.4s, v23.4s}, [x2], #32
            x2 = x2.add(8);

            if w6 <= 0 {
                break; // b.le 9f
            }
            // tst w7, #2 (LR_HAVE_RIGHT)
            v0 = v1; // mov v0.16b, v1.16b
            v1 = vld1q_u16(x5); // ld1 {v1.8h}, [x5], #16
            x5 = x5.add(8);
        };
    }

    if w7 & 2 != 0 {
        // tst w7, #2 (LR_HAVE_RIGHT); b.ne 4f
        loop {
            loop_horizontally!();
            // b.ne 4b (If we don't need to pad, just keep summing.)
        }
    } else {
        // If we'll need to pad the right edge, load that pixel to pad with
        // here since we can find it pretty easily from here.
        let w13 = w6 - (2 + 16 - 3 + 1); // sub w13, w6, #(2 + 16 - 3 + 1)

        // Fill v30 with the right padding pixel
        // ldr h30, [x5, w13, sxtw #1]; dup v30.8h, v30.h[0]
        let v30 = vld1q_dup_u16(x5.offset(w13 as isize));
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
                let x13 = x13.sub((w6 as u32 as usize) << 1); // sub x13, x13, w6, uxtw #1
                let uint8x16x2_t(v28, v29) = vld1q_u8_x2(x13); // ld1 {v28.16b, v29.16b}, [x13]

                v0 = vbslq_u16(vreinterpretq_u16_u8(v28), v30, v0); // bit v0.16b, v30.16b, v28.16b
                v1 = vbslq_u16(vreinterpretq_u16_u8(v29), v30, v1); // bit v1.16b, v30.16b, v29.16b
            }

            loop_horizontally!();
            // b 3b (If we need to pad, check how many pixels we have left.)
        }
    }
    // 9:
}

// `sgr_funcs 16`: the instances of looprestoration_tmpl.S.

/// `sgr_finish_filter1_2rows_16bpc_neon`: `void dav1d_sgr_finish_filter1_2rows_16bpc_neon(int16_t *tmp,
/// const pixel *src, const ptrdiff_t src_stride, int32_t **A_ptrs, int16_t **B_ptrs, const int w, const int h)`
///
/// # Safety
/// As for `sgr_finish_filter1_2rows` (`looprestoration_tmpl.rs`); `src_stride` is in bytes.
pub unsafe extern "C" fn dav1d_sgr_finish_filter1_2rows_16bpc_neon(
    tmp: *mut i16,
    src: *const u16,
    src_stride: isize,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    h: c_int,
) {
    sgr_finish_filter1_2rows::<16>(tmp, src as *const u8, src_stride, a_ptrs, b_ptrs, w, h)
}

/// `sgr_finish_weighted1_16bpc_neon`: `void dav1d_sgr_finish_weighted1_16bpc_neon(pixel *dst, int32_t **A_ptrs,
/// int16_t **B_ptrs, const int w, const int w1 HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// As for `sgr_finish_weighted1` (`looprestoration_tmpl.rs`).
pub unsafe extern "C" fn dav1d_sgr_finish_weighted1_16bpc_neon(
    dst: *mut u16,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    w1: c_int,
    bitdepth_max: c_int,
) {
    sgr_finish_weighted1::<16>(dst as *mut u8, a_ptrs, b_ptrs, w, w1, bitdepth_max)
}

/// `sgr_finish_filter2_2rows_16bpc_neon`: `void dav1d_sgr_finish_filter2_2rows_16bpc_neon(int16_t *tmp,
/// const pixel *src, const ptrdiff_t src_stride, int32_t **A_ptrs, int16_t **B_ptrs, const int w, const int h)`
///
/// # Safety
/// As for `sgr_finish_filter2_2rows` (`looprestoration_tmpl.rs`); `src_stride` is in bytes.
pub unsafe extern "C" fn dav1d_sgr_finish_filter2_2rows_16bpc_neon(
    tmp: *mut i16,
    src: *const u16,
    src_stride: isize,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    h: c_int,
) {
    sgr_finish_filter2_2rows::<16>(tmp, src as *const u8, src_stride, a_ptrs, b_ptrs, w, h)
}

/// `sgr_finish_weighted2_16bpc_neon`: `void dav1d_sgr_finish_weighted2_16bpc_neon(pixel *dst,
/// const ptrdiff_t stride, int32_t **A_ptrs, int16_t **B_ptrs, const int w, const int h, const int w1
/// HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// As for `sgr_finish_weighted2` (`looprestoration_tmpl.rs`); `stride` is in bytes.
pub unsafe extern "C" fn dav1d_sgr_finish_weighted2_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    a_ptrs: *mut *mut i32,
    b_ptrs: *mut *mut i16,
    w: c_int,
    h: c_int,
    w1: c_int,
    bitdepth_max: c_int,
) {
    sgr_finish_weighted2::<16>(dst as *mut u8, stride, a_ptrs, b_ptrs, w, h, w1, bitdepth_max)
}

/// `sgr_weighted2_16bpc_neon`: `void dav1d_sgr_weighted2_16bpc_neon(pixel *dst, const ptrdiff_t dst_stride,
/// const int16_t *t1, const int16_t *t2, const int w, const int h, const int16_t wt[2] HIGHBD_DECL_SUFFIX)`
///
/// # Safety
/// As for `sgr_weighted2` (`looprestoration_tmpl.rs`); `dst_stride` is in bytes.
pub unsafe extern "C" fn dav1d_sgr_weighted2_16bpc_neon(
    dst: *mut u16,
    dst_stride: isize,
    t1: *const i16,
    t2: *const i16,
    w: c_int,
    h: c_int,
    wt: *const i16,
    bitdepth_max: c_int,
) {
    sgr_weighted2::<16>(dst as *mut u8, dst_stride, t1, t2, w, h, wt, bitdepth_max)
}
